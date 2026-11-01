---
id: T-115ab
title: Verify transactional idempotency over HTTP
milestone: M1
status: todo
depends_on: [T-115aa]
spec_refs: [API-08, CAN-70, STM-16, SBD-57]
estimated_size: M
---

## Goal
Prove replay, mismatch, rollback and concurrent duplicate behavior through real HTTP and disposable PostgreSQL.

## Context
T-115aa owns the Core port. Use its shipped interface, T-101's existing table and a test-only route; add no production endpoint or migration.

## Scope
### In
- Add the test-only route and acceptance tests using the app role and tenant-scoped Tx.
- Create a test-only effects table inside the disposable database; drop the database after tests.
### Out
- Core implementation (T-115aa), cleanup, If-Match, rate limits, headers/cache and public URL reporting.

## Files
| Path | Action | Purpose |
|---|---|---|
| crates/eq-server/Cargo.toml | modify | Add existing workspace test dependencies. |
| crates/eq-server/tests/http_hardening.rs | create | Exercise the Core port over HTTP and PostgreSQL. |

## Interfaces
The test handler calls Idempotent::into_parts, IdempotencyGuard::check and store in one transaction.

## Business rules
No business value or production behavior is added.

## Implementation notes
1. Run requests against the app role and the task's fresh PostgreSQL database.
2. Keep side effects, idempotency storage and commit in the same transaction.

## Security
Use eq_app (NOSUPERUSER NOBYPASSRLS) for requests. Tenant scope comes from test state and db::begin_system; owner pool is for setup/assertions only. Do not log payloads.

## Acceptance criteria
- AC1. Same key/request replays status/body and does not repeat the effect → idempotent_replay.
- AC2. Changed request with same tenant/key returns 422 idempotency_mismatch and one effect → idempotency_mismatch.
- AC3. Ten concurrent duplicates return equal bodies and one effect → idempotency_concurrent.
- AC4. Handler failure rolls back both effect and idempotency record → idempotency_rollback.

## Tests to write
| File | Test names |
|---|---|
| crates/eq-server/tests/http_hardening.rs | idempotent_replay, idempotency_mismatch, idempotency_concurrent, idempotency_rollback |

## Definition of done
- [ ] Four HTTP tests pass on a fresh PostgreSQL 16 DB in ritma-db-1; drop only the task DB.
- [ ] No migration/schema change; fmt, relevant Clippy, cargo deny, guards and tasks index/lint pass.
- [ ] Report exact command, DB, pass count and unrun checks.
- [ ] Commit test(server): verify idempotency over HTTP [T-115ab], signed off.

## Report
**Plan** — Create task DB → add fixture/route → run four HTTP cases → verify checks and drop only that DB.
**Assumptions** — Existing table/RLS contract and T-115aa interface are canonical.
**Test plan** — AC1–AC4 map to the exact tests above; cover retry, mismatch, concurrent first-use and rollback.
**Summary** — Pending implementation.
**Spec read** — Pending implementation.
**Files** — Pending implementation.
**Acceptance criteria → tests** — Pending implementation.
**Checks run** — Pending implementation.
**Manual checks** — Real HTTP requests through the test-only route.
**Debug log** — None.
**Performance** — n/a.
**Security** — Pending SBD-81 review.
**Deviations** — None known.
**Dependencies affected** — T-115a awaits this child; T-115b remains gated on T-115a.
**Rollback plan** — Revert the test commit and drop only its task database.
**Follow-ups** — None known.

## Questions
None.

## Review
Pending implementation.
