use axum::extract::State;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use eq_core::db;
use eq_core::error::AppError;
use eq_core::http::Idempotent;
use eq_core::ids::TenantId;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::OnceCell;
use tokio::task::{JoinHandle, JoinSet};
use uuid::Uuid;

type TestResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;
static TEST_SCHEMA: OnceCell<()> = OnceCell::const_new();

#[derive(Clone)]
struct RouteState {
    app_pool: PgPool,
    tenant: TenantId,
}

struct TestServer {
    address: String,
    app_pool: PgPool,
    owner_pool: PgPool,
    tenant: TenantId,
    server: JoinHandle<Result<(), std::io::Error>>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct EffectRequest {
    request_id: String,
    value: String,
}

#[expect(
    clippy::disallowed_methods,
    reason = "the integration test connects only to task-supplied disposable database URLs"
)]
async fn start_server() -> TestResult<TestServer> {
    let owner_url = std::env::var("RITMA_DATABASE_OWNER_URL")?;
    let app_url = std::env::var("DATABASE_URL")?;
    let owner_pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&owner_url)
        .await?;
    let app_pool = PgPoolOptions::new().max_connections(16).connect(&app_url).await?;

    TEST_SCHEMA
        .get_or_try_init(|| async {
            sqlx::query(
                "CREATE TABLE IF NOT EXISTS public.http_hardening_side_effects (\
                 tenant_id uuid NOT NULL REFERENCES public.tenants(id), \
                 request_id text NOT NULL, value jsonb NOT NULL, \
                 PRIMARY KEY (tenant_id, request_id))",
            )
            .execute(&owner_pool)
            .await?;
            sqlx::query("GRANT SELECT, INSERT, DELETE ON TABLE public.http_hardening_side_effects TO eq_app")
                .execute(&owner_pool)
                .await?;
            Ok::<(), sqlx::Error>(())
        })
        .await?;

    let tenant = TenantId(Uuid::now_v7());
    let slug = format!("idem{}", tenant.0.simple());
    sqlx::query(
        "INSERT INTO public.tenants \
         (id, slug, name, country, preset_id, preset_version) \
         VALUES ($1, $2, 'Idempotency Test School', 'ID', 'test', 1)",
    )
    .bind(tenant)
    .bind(slug)
    .execute(&owner_pool)
    .await?;

    let router = Router::new()
        .route("/idempotency", post(idempotency_route))
        .with_state(Arc::new(RouteState {
            app_pool: app_pool.clone(),
            tenant,
        }));
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let address = format!("http://{}", listener.local_addr()?);
    let server = tokio::spawn(async move { axum::serve(listener, router).await });

    Ok(TestServer {
        address,
        app_pool,
        owner_pool,
        tenant,
        server,
    })
}

async fn idempotency_route(
    State(state): State<Arc<RouteState>>,
    request: Idempotent<EffectRequest>,
) -> Result<Response, AppError> {
    let (body, guard) = request.into_parts();
    let mut tx = db::begin_system(&state.app_pool, state.tenant).await?;

    if let Some(replay) = guard.check(&mut tx).await? {
        tx.commit().await?;
        return Ok(replay.into_response());
    }

    let response_body = json!({
        "request_id": body.request_id,
        "value": body.value,
    });
    sqlx::query(
        "INSERT INTO public.http_hardening_side_effects (tenant_id, request_id, value) \
         VALUES ($1, $2, $3)",
    )
    .bind(state.tenant)
    .bind(&body.request_id)
    .bind(&response_body)
    .execute(tx.conn())
    .await?;
    if body.value == "test-fail-before-commit" {
        return Err(AppError::Internal(anyhow::anyhow!("test transaction rollback")));
    }
    guard.store(&mut tx, StatusCode::CREATED, &response_body).await?;
    tx.commit().await?;

    Ok((StatusCode::CREATED, Json(response_body)).into_response())
}

fn request_body(request_id: &str, value: &str) -> EffectRequest {
    EffectRequest {
        request_id: request_id.to_owned(),
        value: value.to_owned(),
    }
}

async fn post_request(
    client: &reqwest::Client,
    server: &TestServer,
    key: &str,
    body: &EffectRequest,
) -> TestResult<reqwest::Response> {
    Ok(client
        .post(format!("{}/idempotency", server.address))
        .header("Idempotency-Key", key)
        .header(CONTENT_TYPE, "application/json")
        .json(body)
        .send()
        .await?)
}

async fn effect_count(server: &TestServer, request_id: &str) -> TestResult<i64> {
    let row = sqlx::query(
        "SELECT count(*) AS count FROM public.http_hardening_side_effects \
         WHERE tenant_id = $1 AND request_id = $2",
    )
    .bind(server.tenant)
    .bind(request_id)
    .fetch_one(&server.owner_pool)
    .await?;
    Ok(row.try_get("count")?)
}

async fn stop_server(server: TestServer) {
    server.server.abort();
    server.app_pool.close().await;
    server.owner_pool.close().await;
}

#[tokio::test]
async fn idempotent_replay() -> TestResult {
    let server = start_server().await?;
    let client = reqwest::Client::new();
    let key = Uuid::now_v7().to_string();
    let body = request_body("replay", "same request");

    let first = post_request(&client, &server, &key, &body).await?;
    assert_eq!(first.status(), StatusCode::CREATED);
    assert!(first.headers().get("idempotent-replayed").is_none());
    let first_body: Value = first.json().await?;

    let replay = post_request(&client, &server, &key, &body).await?;
    assert_eq!(replay.status(), StatusCode::CREATED);
    assert_eq!(
        replay
            .headers()
            .get("idempotent-replayed")
            .and_then(|value| value.to_str().ok()),
        Some("true")
    );
    let replay_body: Value = replay.json().await?;

    assert_eq!(replay_body, first_body);
    assert_eq!(effect_count(&server, "replay").await?, 1);
    stop_server(server).await;
    Ok(())
}

#[tokio::test]
async fn idempotency_mismatch() -> TestResult {
    let server = start_server().await?;
    let client = reqwest::Client::new();
    let key = Uuid::now_v7().to_string();

    let first = post_request(&client, &server, &key, &request_body("mismatch", "first")).await?;
    assert_eq!(first.status(), StatusCode::CREATED);
    let _first_body: Value = first.json().await?;

    let mismatch = post_request(&client, &server, &key, &request_body("mismatch", "changed")).await?;
    assert_eq!(mismatch.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let mismatch_body = mismatch.text().await?;
    assert!(mismatch_body.contains("idempotency_mismatch"));
    assert_eq!(effect_count(&server, "mismatch").await?, 1);

    stop_server(server).await;
    Ok(())
}

#[tokio::test]
async fn idempotency_concurrent() -> TestResult {
    let server = start_server().await?;
    let client = reqwest::Client::new();
    let key = Uuid::now_v7().to_string();
    let body = request_body("concurrent", "one side effect");
    let mut requests = JoinSet::new();

    for _ in 0..10 {
        let client = client.clone();
        let url = format!("{}/idempotency", server.address);
        let key = key.clone();
        let body = body.clone();
        requests.spawn(async move {
            client
                .post(url)
                .header("Idempotency-Key", key)
                .json(&body)
                .send()
                .await
        });
    }

    let mut expected_body = None;
    let mut replay_count = 0;
    for _ in 0..10 {
        let response = requests
            .join_next()
            .await
            .ok_or("concurrent request task ended without a response")???;
        assert_eq!(response.status(), StatusCode::CREATED);
        if response.headers().get("idempotent-replayed").is_some() {
            replay_count += 1;
        }
        let response_body: Value = response.json().await?;
        if let Some(expected) = &expected_body {
            assert_eq!(&response_body, expected);
        } else {
            expected_body = Some(response_body);
        }
    }

    assert_eq!(replay_count, 9);
    assert_eq!(effect_count(&server, "concurrent").await?, 1);
    stop_server(server).await;
    Ok(())
}

#[tokio::test]
async fn idempotency_rollback() -> TestResult {
    let server = start_server().await?;
    let client = reqwest::Client::new();
    let key = Uuid::now_v7().to_string();

    let failed = post_request(
        &client,
        &server,
        &key,
        &request_body("rollback", "test-fail-before-commit"),
    )
    .await?;
    assert_eq!(failed.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(effect_count(&server, "rollback").await?, 0);

    let retry = post_request(
        &client,
        &server,
        &key,
        &request_body("rollback", "committed after retry"),
    )
    .await?;
    assert_eq!(retry.status(), StatusCode::CREATED);
    assert!(retry.headers().get("idempotent-replayed").is_none());
    let _retry_body: Value = retry.json().await?;
    assert_eq!(effect_count(&server, "rollback").await?, 1);

    stop_server(server).await;
    Ok(())
}
