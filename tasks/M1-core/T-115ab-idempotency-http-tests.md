---
id: T-115ab
title: Verify transactional idempotency over HTTP
milestone: M1
status: review
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
- [x] Four HTTP tests pass on a fresh PostgreSQL 16 DB in ritma-db-1; drop only the task DB.
- [x] No migration/schema change; fmt, relevant Clippy, cargo deny, guards and tasks index/lint pass.
- [x] Report exact command, DB, pass count and unrun checks.
- [x] Commit test(server): verify idempotency over HTTP [T-115ab], signed off.

## Report
**Plan** — (1) Recheck the T-115aa guard and Tx boundary → verify the shipped `into_parts`/`check`/`store` interface. (2) Create a fresh task database in the existing `ritma-db-1` container → verify only `ritma_test_T-115ab` is added. (3) Start the test-only Axum route with separate owner and `eq_app` pools → verify request execution uses `db::begin_system`. (4) Run the four HTTP acceptance cases → verify replay, mismatch, concurrency and rollback. (5) Run formatting, Clippy, deny, guard, ERD, index and lint checks → verify all applicable checks pass. (6) Drop only the task database and restore the existing container state → verify resource accounting. (7) Complete the Report, set review and regenerate the index → verify task checks before the signed commit.
**Assumptions** — T-115aa's shipped interface and the existing idempotency table/RLS contract are canonical. The effects table is test-only and exists only in the fresh disposable database; no migration or production route is needed. The task's test suite runs against PostgreSQL 16 in the existing `ritma-db-1` container.
**Test plan** — AC1: Given a successful request, when the identical key/body is replayed, then status and body match and one effect exists → `idempotent_replay`. AC2: Given a stored key/body, when a changed body reuses that key, then the response is 422 `idempotency_mismatch` and one effect exists → `idempotency_mismatch`. AC3: Given ten concurrent identical requests, when they complete, then all bodies match and one effect exists → `idempotency_concurrent`. AC4: Given a handler error before commit, when the same key is retried successfully, then the failed effect and key are absent and exactly one retry effect exists → `idempotency_rollback`. Edge cases: first use, replay, concurrent first use and retry after rollback. Error case: mismatch problem code/status and internal rollback response. Abuse case: business writes and the idempotency record share an `eq_app` tenant-scoped transaction; request payloads are not logged.
**Summary** — Added four real HTTP acceptance tests around the Core idempotency guard using a test-only Axum route and effects table. They verify response replay, mismatch rejection, serialized concurrent duplicates and rollback/retry on a fresh PostgreSQL 16 database.
**Spec read** — `spec/07-api-conventions.md` API-08; `spec/20-state-machines.md` STM-16; `spec/23-contracts-and-canonical-forms.md` CAN-70; `spec/18-security-by-design.md` §§5.6 and 8 (SBD-57, SBD-81); `spec/16-code-quality.md` §§9 and 12; `spec/17-clean-architecture.md` §§1 and 4; `spec/19-error-handling.md` §§6, 9 and 10.
**Files** — `crates/eq-server/Cargo.toml` (test dependencies `serde`, `serde_json`, `uuid`); `Cargo.lock` (matching `eq-server` dependency entries); `crates/eq-server/tests/http_hardening.rs` (test-only route, isolated effect fixture and four acceptance tests); `tasks/M1-core/T-115ab-idempotency-http-tests.md` (plan, report and status); `tasks/INDEX.md` (generated status).
**Acceptance criteria → tests**
| AC | Test | Result |
|---|---|---|
| AC1 replay preserves status/body and one effect | `idempotent_replay` | ✅ 1 passed |
| AC2 changed request returns 422 and one effect | `idempotency_mismatch` | ✅ 1 passed |
| AC3 ten duplicates return equal bodies and one effect | `idempotency_concurrent` | ✅ 1 passed |
| AC4 failed handler rolls back effect and idempotency record | `idempotency_rollback` | ✅ 1 passed |
**Checks run** — ✅ `CARGO_TARGET_DIR="$HOME/.cache/ritma/target" SQLX_OFFLINE=true DATABASE_URL='postgres://eq_app:test-secret-app@localhost:5433/ritma_test_T-115ab' RITMA_DATABASE_OWNER_URL='postgres://eq:eq@localhost:5433/ritma_test_T-115ab' cargo test --locked -p eq-server --test http_hardening` — 4/4 passed on the fresh PostgreSQL 16 database `ritma_test_T-115ab` in `ritma-db-1`. ✅ `cargo run --locked -p eq-server -- migrate` with the same database URLs and one-off fake `RITMA_SECRET_KEY`/`RITMA_PUBLIC_URL` — migrations applied. ✅ `CARGO_TARGET_DIR="$HOME/.cache/ritma/target" cargo fmt --all --check`. ✅ `CARGO_TARGET_DIR="$HOME/.cache/ritma/target" SQLX_OFFLINE=true cargo clippy --locked --workspace --all-targets -- -D warnings`. ✅ `cargo deny check` (advisories, bans, licenses and sources passed; existing duplicate-version warnings remain). ✅ `scripts/check-deps.sh && scripts/check-literals.sh && scripts/check-web-boundaries.sh && scripts/check-layers.sh`. ✅ `scripts/erd.py --check` (no schema or migration changed). ✅ `scripts/tasks-index.sh --check` and `python3 -I scripts/tasks-lint.py`. ✅ `git diff --check`. ✅ staged `gitleaks protect --staged --redact`. Not run: `cargo test --workspace`, because unrelated database integration suites can start additional containers outside the fixed-resource rule; web lint/typecheck/test/i18n and `make api`, because no UI or production API/OpenAPI files changed; `cargo sqlx prepare`, because this task adds no compile-time SQL macros; `scripts/erd.py --migrations`, because no migration changed; browser `make e2e`, because the task exercises only its test-only HTTP route and has no browser journey.
**Manual checks** — The acceptance test process issued local HTTP requests to an ephemeral loopback Axum listener as `eq_app` (`NOSUPERUSER`, `NOBYPASSRLS`) and asserted response status/body/headers and effect counts for all four cases. The owner pool was used only for fixture setup/assertions; request operations used `db::begin_system` and the same transaction for effects and idempotency storage.
**Debug log** — The first migration invocation reached configuration validation but lacked required `RITMA_SECRET_KEY`; reran it with a one-off fake base64 key and `RITMA_PUBLIC_URL=http://localhost`, then migrations passed. The first HTTP test invocation used `eq_app:eq_app` and all four tests failed at connection authentication with SQLSTATE `28P01`; the existing Ritma task fixtures use `eq_app:test-secret-app`, and rerunning with that fake app-role credential passed 4/4. No production code or role credentials were changed.
**Performance** — n/a; the concurrency test verifies one effect under ten simultaneous requests, but no latency benchmark was in scope.
**Security** — SBD-81: (1) auth: route is confined to the test-only listener and does not enter the production router; (2) tenant: fixed test tenant, no client-supplied tenant, app role verified `NOSUPERUSER`/`NOBYPASSRLS`, and `db::begin_system` scopes the transaction; (3) data: no production fields or persistent schema added, fixture values are synthetic and payloads are not logged; (4) input: requests go through T-115aa's validated `Idempotent` extractor; (5) output: JSON response only, no HTML/CSV; (6) crypto/secrets: no new secret handling; (7) external calls: none; (8) integrity: all four abuse cases verify replay, mismatch, single effect and rollback with the guard in the business transaction; (9) security audit: no new security event is introduced; (10) abuse tests are the four acceptance tests listed above.
**Deviations** — None. Reused T-115aa's shipped guard and added no production endpoint, migration or schema change. Dependency RATIFY items TD-199–203 remain as recorded in T-115aa; this task makes no new decision.
**Dependencies affected** — T-115a can consume this HTTP verification; T-115b remains dependent on the aggregate T-115a task. No dependency interface changed.
**Rollback plan** — Revert the signed task commit. The only database was `ritma_test_T-115ab`, which has been dropped; no persistent schema or production data changed.
**Follow-ups** — T-306 already owns the `en`/`id` translation work for `idempotency_mismatch`; no new follow-up was found.
**Workflow steps 0–12** — 0: resumed T-115ab WIP on `dev`, verified dependency and repository task checks; unrelated WIP was preserved. 1: read the full task and cited specs. 2: read T-115aa's Report, deviations, follow-ups and RATIFY items and inspected its shipped Core extractor/guard and transaction boundary. 3: n/a (backend-only task). 4: recorded plan, assumptions and AC/edge/error/abuse mapping before validation. 5: completed the test-only route, fixture and four named tests. 6: exercised real local HTTP requests against PostgreSQL 16. 7: recorded and resolved the configuration and app-role authentication failures above. 8: reviewed scope, transaction use, test isolation, error responses and secret hygiene. 9: no latency optimization needed; concurrency correctness was checked by the single-effect assertion. 10: completed the SBD-81 checklist. 11: reran the applicable Rust, deny, ERD, guard and task checks; excluded unrelated DB/container suites and web/API checks for the reasons listed above. 12: set review, regenerated/checked the index, staged only T-115ab-owned changes, committed with sign-off, and pushed only `dev`.
**Disk/resource accounting** — Before the task database: fixed target `$HOME/.cache/ritma/target` was 22G; Docker reported 8 images / 14.55GB, 13 containers / 10.49MB, 10 volumes / 5.466GB, build cache 0. After tests and cleanup: target remained 22G; Docker reported the same image/container/volume counts and sizes, build cache 0. Started and stopped the pre-existing `ritma-db-1`; created and dropped only `ritma_test_T-115ab`. No container, image or volume was created or removed. All `kinevo*`, `infrastructure*`, `eqs-*` and `equalizer-stake_*` resources and unrelated WIP were left untouched.

## Questions
None.

## Review
Self-review complete. The test-only route is absent from production wiring, uses the T-115aa Core contract inside the app-role tenant transaction, and verifies the four specified replay, mismatch, concurrency and rollback cases. No migration, production schema or public API changed. The ephemeral database was dropped and the existing container returned to its original stopped state.
