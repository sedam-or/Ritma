//! API-08/STM-16: transactional request idempotency for opted-in HTTP handlers.

use axum::{
    Json,
    body::Body,
    extract::{FromRequest, MatchedPath, Request},
    http::{HeaderMap, HeaderValue, StatusCode, header::HeaderName},
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    db::Tx,
    error::{AppError, codes},
    http::validated::ValidatedJson,
};

/// API-08/CAN-70: parsed canonical request body and its transaction-bound idempotency guard.
pub struct Idempotent<T> {
    body: T,
    guard: IdempotencyGuard,
}

impl<T> Idempotent<T> {
    /// API-08: returns the validated body and guard for use by the handler transaction.
    pub fn into_parts(self) -> (T, IdempotencyGuard) {
        (self.body, self.guard)
    }
}

impl<T> std::fmt::Debug for Idempotent<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Idempotent")
            .field("body", &"[REDACTED]")
            .field("guard", &self.guard)
            .finish()
    }
}

impl<S, T> FromRequest<S> for Idempotent<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Serialize + Send,
{
    type Rejection = AppError;

    async fn from_request(request: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let method = request.method().as_str().to_owned();
        let route = request
            .extensions()
            .get::<MatchedPath>()
            .map(|matched| matched.as_str().to_owned())
            .ok_or_else(|| {
                AppError::Internal(anyhow::anyhow!(
                    "idempotent handler is missing its matched route template"
                ))
            })?;
        let key = parse_key(request.headers())?;
        let ValidatedJson(body) = ValidatedJson::<T>::from_request(request, state).await?;
        let request_hash = request_hash(&method, &route, &body)?;

        Ok(Self {
            body,
            guard: IdempotencyGuard {
                key,
                route,
                request_hash,
            },
        })
    }
}

/// API-08/STM-16: guard that locks, checks and stores a key inside the caller's business transaction.
pub struct IdempotencyGuard {
    key: String,
    route: String,
    request_hash: [u8; 32],
}

impl std::fmt::Debug for IdempotencyGuard {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("IdempotencyGuard")
            .field("key", &"[REDACTED]")
            .field("route", &self.route)
            .field("request_hash", &"[REDACTED]")
            .finish()
    }
}

impl IdempotencyGuard {
    /// STM-16: locks the tenant/key pair and returns its prior response, if still within the TTL.
    pub async fn check(&self, tx: &mut Tx) -> Result<Option<StoredResponse>, AppError> {
        let Some(row) = crate::idempotency::load(tx, &self.key).await? else {
            return Ok(None);
        };

        if row.route != self.route || row.request_hash.as_slice() != self.request_hash.as_slice() {
            return Err(AppError::Unprocessable {
                code: &codes::IDEMPOTENCY_MISMATCH,
                detail: "Idempotency key was already used for a different request".to_owned(),
                extra: serde_json::Map::new(),
            });
        }

        let status = u16::try_from(row.status)
            .ok()
            .and_then(|status| StatusCode::from_u16(status).ok())
            .ok_or_else(|| {
                AppError::Internal(anyhow::anyhow!(
                    "stored idempotency response has an invalid status"
                ))
            })?;

        Ok(Some(StoredResponse {
            status,
            body: row.body,
        }))
    }

    /// STM-16: stores only a successful response in the same transaction as its business change.
    pub async fn store<T: Serialize>(
        &self,
        tx: &mut Tx,
        status: StatusCode,
        body: &T,
    ) -> Result<(), AppError> {
        let body =
            serde_json::to_value(body).map_err(|error| AppError::Internal(anyhow::Error::new(error)))?;
        let status = i32::from(status.as_u16());
        crate::idempotency::store(
            tx,
            &self.key,
            &self.route,
            self.request_hash.as_slice(),
            status,
            &body,
        )
        .await?;
        Ok(())
    }
}

/// API-08: response previously committed with the idempotent operation.
pub struct StoredResponse {
    status: StatusCode,
    body: serde_json::Value,
}

impl std::fmt::Debug for StoredResponse {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("StoredResponse")
            .field("status", &self.status)
            .field("body", &"[REDACTED]")
            .finish()
    }
}

impl StoredResponse {
    /// API-08: converts the stored JSON into the replay response with its marker header.
    pub fn into_response(self) -> Response<Body> {
        let mut response = (self.status, Json(self.body)).into_response();
        response.headers_mut().insert(
            HeaderName::from_static("idempotent-replayed"),
            HeaderValue::from_static("true"),
        );
        response
    }
}

fn parse_key(headers: &HeaderMap) -> Result<String, AppError> {
    let raw = headers
        .get("Idempotency-Key")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(bad_idempotency_key)?;
    let key = Uuid::parse_str(raw).map_err(|_| bad_idempotency_key())?;
    let canonical = key.to_string();
    if raw != canonical {
        return Err(bad_idempotency_key());
    }
    Ok(canonical)
}

fn request_hash<T: Serialize>(method: &str, route: &str, body: &T) -> Result<[u8; 32], AppError> {
    let canonical_body =
        serde_jcs::to_vec(body).map_err(|error| AppError::Internal(anyhow::Error::new(error)))?;
    let mut hasher = Sha256::new();
    hasher.update(method.as_bytes());
    hasher.update(b"\n");
    hasher.update(route.as_bytes());
    hasher.update(b"\n");
    hasher.update(canonical_body);
    Ok(hasher.finalize().into())
}

fn bad_idempotency_key() -> AppError {
    AppError::BadRequest {
        code: &codes::BAD_REQUEST,
        detail: "Idempotency-Key must be a canonical lowercase UUID".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::{IdempotencyGuard, Idempotent, request_hash};

    #[test]
    fn debug_redacts_request_body() {
        let request = Idempotent {
            body: "test-secret-idempotency".to_owned(),
            guard: IdempotencyGuard {
                key: "test-secret-idempotency-key".to_owned(),
                route: "/test".to_owned(),
                request_hash: [0; 32],
            },
        };

        let debug = format!("{request:?}");
        assert!(!debug.contains("test-secret-idempotency"));
        assert!(!debug.contains("test-secret-idempotency-key"));
    }

    #[test]
    fn request_hash_uses_canonical_json() {
        let first: serde_json::Value = serde_json::from_str("{\"a\":1,\"b\":2}").expect("valid test JSON");
        let second: serde_json::Value = serde_json::from_str("{\"b\":2, \"a\":1}").expect("valid test JSON");

        assert_eq!(
            request_hash("POST", "/test", &first).expect("canonical hash"),
            request_hash("POST", "/test", &second).expect("canonical hash")
        );
    }

    #[test]
    fn idempotency_key_requires_canonical_lowercase_uuid() {
        let mut headers = axum::http::HeaderMap::new();
        assert!(super::parse_key(&headers).is_err());

        headers.insert(
            "Idempotency-Key",
            axum::http::HeaderValue::from_static("0192F3A4-1B2C-7DEF-8ABC-0123456789AB"),
        );
        assert!(super::parse_key(&headers).is_err());

        headers.insert(
            "Idempotency-Key",
            axum::http::HeaderValue::from_static("0192f3a4-1b2c-7def-8abc-0123456789ab"),
        );
        assert_eq!(
            super::parse_key(&headers).expect("canonical key"),
            "0192f3a4-1b2c-7def-8abc-0123456789ab"
        );
    }
}
