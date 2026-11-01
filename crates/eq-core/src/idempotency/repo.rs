//! API-08/STM-16: SQL for the Core-owned idempotency_keys table.

use serde_json::Value;
use uuid::Uuid;

use super::StoredEntry;
use crate::{db::Tx, error::AppError};

pub(super) async fn lock_and_expire(tx: &mut Tx, tenant: Uuid, key: &str) -> Result<(), AppError> {
    let lock_key = format!("{tenant}{key}");
    sqlx::query!("SELECT pg_advisory_xact_lock(hashtext($1::text))", lock_key)
        .fetch_one(tx.conn())
        .await?;

    sqlx::query!(
        "DELETE FROM public.idempotency_keys \
         WHERE tenant_id = $1 AND key = $2 \
           AND created_at <= statement_timestamp() - INTERVAL '24 hours'",
        tenant,
        key
    )
    .execute(tx.conn())
    .await?;
    Ok(())
}

pub(super) async fn find(tx: &mut Tx, tenant: Uuid, key: &str) -> Result<Option<StoredEntry>, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT route, request_hash, status, body \
         FROM public.idempotency_keys \
         WHERE tenant_id = $1 AND key = $2",
        tenant,
        key
    )
    .fetch_optional(tx.conn())
    .await?;

    Ok(row.map(|row| StoredEntry {
        route: row.route,
        request_hash: row.request_hash,
        status: row.status,
        body: row.body,
    }))
}

pub(super) async fn insert(
    tx: &mut Tx,
    tenant: Uuid,
    key: &str,
    route: &str,
    request_hash: &[u8],
    status: i32,
    body: &Value,
) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO public.idempotency_keys \
         (tenant_id, key, route, request_hash, status, body) \
         VALUES ($1, $2, $3, $4, $5, $6)",
        tenant,
        key,
        route,
        request_hash,
        status,
        body
    )
    .execute(tx.conn())
    .await?;
    Ok(())
}
