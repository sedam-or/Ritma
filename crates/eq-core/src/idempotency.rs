//! API-08/STM-16: Core-owned transaction operations for idempotency keys.

mod repo;

use crate::{db::Tx, error::AppError};
use serde_json::Value;

pub(crate) struct StoredEntry {
    pub(crate) route: String,
    pub(crate) request_hash: Vec<u8>,
    pub(crate) status: i32,
    pub(crate) body: Value,
}

pub(crate) async fn load(tx: &mut Tx, key: &str) -> Result<Option<StoredEntry>, AppError> {
    let tenant = tx.tenant().0;
    repo::lock_and_expire(tx, tenant, key).await?;
    Ok(repo::find(tx, tenant, key).await?)
}

pub(crate) async fn store(
    tx: &mut Tx,
    key: &str,
    route: &str,
    request_hash: &[u8],
    status: i32,
    body: &Value,
) -> Result<(), AppError> {
    let tenant = tx.tenant().0;
    repo::insert(tx, tenant, key, route, request_hash, status, body).await?;
    Ok(())
}
