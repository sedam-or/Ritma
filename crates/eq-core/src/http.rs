//! VAL-40: shared HTTP request extractors.

pub mod idempotency;
pub mod validated;

pub use idempotency::{IdempotencyGuard, Idempotent, StoredResponse};
pub use validated::{ValidatedJson, ValidatedQuery, parse_path_id};
