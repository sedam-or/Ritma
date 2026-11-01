---
id: T-115b
title: Schedule cleanup of expired idempotency keys
milestone: M1
status: todo
depends_on: [T-115a]
spec_refs: [API-08, STM-16, ERD-01, ERD-03]
estimated_size: S
---

## Goal
Remove expired Core idempotency records through the existing job runner while preserving the API-08 24-hour replay window.

## Context
The original T-115 contract requires core.idempotency_cleanup on cron 15 4 * * *, deleting rows older than 24 hours from the existing Core-owned idempotency_keys table. The table DDL and migration are owned by T-101; do not change schema.

## Scope
### In
- Register the Core cleanup job and cron specification.
- Delete only expired idempotency keys using the injected Clock.
### Out
- Request replay/locking (T-115a) and all other HTTP hardening parts.

## Files
| Path | Action | Purpose |
|---|---|---|
| crates/eq-core/src/jobs.rs | modify | Register the cleanup job. |
| crates/eq-core/src/jobs/repo.rs | modify | Parameterized cleanup query. |
| crates/eq-core/src/module.rs | modify | Expose the Core job and exact cron. |
| crates/eq-server/tests/http_hardening.rs | modify | Verify expiry boundary and tenant isolation. |
| .sqlx/*.json | generate if changed | Offline metadata for changed query. |

## Interfaces
Job id core.idempotency_cleanup; cron 15 4 * * *; only existing idempotency_keys rows older than the 24-hour retention window may be deleted.

## Business rules
The retention interval and schedule are the exact values specified by T-115/API-08. Do not change them.

## Security
- Triggers: T1, T4 (SBD-30).
- Data classes: S1 request records and S2 tenant identifiers per ERD.
- Cleanup must preserve tenant scoping and delete no rows from another module's tables (SBD-42/CA-30).
- Abuse test: prove the job cannot delete a newer key or a row belonging to another tenant.

## Implementation notes
Use existing Core job patterns and parameterized sqlx::query!; no new migration.

## Acceptance criteria
- AC1. Job is registered under the exact id and cron.
- AC2. Expired keys are deleted and unexpired keys remain; other tenants' keys are unaffected.

## Tests to write
| File | Test names |
|---|---|
| crates/eq-server/tests/http_hardening.rs | idempotency_cleanup_deletes_expired_rows, idempotency_cleanup_keeps_live_rows |

## Definition of done
- [ ] Focused tests pass in the existing PostgreSQL 16 container using a disposable task DB.
- [ ] scripts/erd.py --migrations passes; no schema changes.
- [ ] Rust, SQLx, deny and guard checks pass or are explicitly reported Not run with reason.
- [ ] Commit feat(core): clean expired idempotency keys [T-115b], signed off.

## Report
Filled by the implementer using WORKFLOW.md §3 Step 12.

## Questions
n/a.

## Review
n/a.
