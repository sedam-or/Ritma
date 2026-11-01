---
id: T-115aa
title: Implement the Core transactional idempotency port
milestone: M1
status: review
depends_on: [T-101, T-104, T-111, T-113b]
spec_refs: [API-08, CAN-10, CAN-70, STM-16, ERR-21, ERR-30, SBD-57]
estimated_size: M
---

## Goal
Provide the Core HTTP idempotency extractor, transaction guard, stored response and canonical request hash.

## Context
Reuse T-101's existing idempotency_keys table. API-08/STM-16 define the canonical key, 24-hour TTL, transaction lock, replay and 422 mismatch; CAN-70/D-14 define SHA-256 over method, route template and JCS body.

## Scope
### In
- Implement eq_core::http::{Idempotent, IdempotencyGuard, StoredResponse}.
- Register idempotency_mismatch response metadata.
- Add Core unit tests for redaction, canonical UUID parsing and hashing.
### Out
- HTTP acceptance route/database integration tests (T-115ab), cleanup, If-Match, rate limits, headers/cache and public URL reporting.

## Files
| Path | Action | Purpose |
|---|---|---|
| crates/eq-core/Cargo.toml | modify | Enable workspace-pinned serde_jcs. |
| crates/eq-core/src/error.rs | modify | Register idempotency_mismatch. |
| crates/eq-core/src/http.rs | modify | Export idempotency types. |
| crates/eq-core/src/http/idempotency.rs | create | Parse/hash/lock/replay/store through Tx. |
| crates/eq-core/src/idempotency.rs | create | Keep Core's transaction operations behind the HTTP API. |
| crates/eq-core/src/idempotency/repo.rs | create | Own parameterized SQL for idempotency_keys. |
| .sqlx/*.json | generate if changed | Keep query! metadata current. |
| web/openapi.json and web/src/api/** | generate if changed | Regenerate shared error contract. |

## Interfaces
Parse canonical UUID, method, Axum matched route template and validated body into a value plus guard. The handler calls check(&mut Tx) before side effects and store(&mut Tx, status, body) in the same transaction. Replay returns saved status/body with Idempotent-Replayed: true.

## Business rules
Use only the specified 24-hour TTL and canonical route/body behavior.

## Implementation notes
1. Keep the lock, lookup, business writes and stored response in the same Tx.
2. Use the existing table and parameterized query macros in Core.

## Security
Triggers: T1, T2, T4. All persisted idempotency table columns are S1 per ERD-03. Tenant scope comes only from Tx; never log keys, bodies or responses. Redact Debug output and keep SQL parameterized.

## Acceptance criteria
- AC1. Canonical JSON formatting yields a stable method/route/body hash → request_hash_uses_canonical_json.
- AC2. Missing or noncanonical UUID keys are rejected → idempotency_key_requires_canonical_lowercase_uuid.
- AC3. Debug output does not reveal request body or key → debug_redacts_request_body.

## Tests to write
| File | Test names |
|---|---|
| crates/eq-core/src/http/idempotency.rs | debug_redacts_request_body, idempotency_key_requires_canonical_lowercase_uuid, request_hash_uses_canonical_json |

## Definition of done
- [x] Core unit tests, Rust fmt and Core Clippy pass.
- [x] SQLx offline metadata is current after query changes.
- [x] erd.py --migrations, cargo deny, guards, make api, task index/lint pass.
- [ ] Follow spec 16 §12, spec 17 and SBD-81; no #[allow]. Full workspace integration tests could not run within the fixed-container boundary; see Report.
- [x] Commit feat(core): add transactional idempotency guard [T-115aa], signed off.

## Report
**Plan** — (1) Check the Core-owned idempotency DDL and dependencies → verify T-101's mapped table and T-104's transaction boundary. (2) Implement canonical header/body parsing and a redacted guard → verify the three named unit tests. (3) Keep lock, expiry, lookup and storage SQL inside Core repo → verify SQLx metadata and the EXPLAIN plan. (4) Register the mismatch response and regenerate the OpenAPI contract → verify `make api` and web checks. (5) Run Rust, dependency, schema, guard, security and task checks → verify the recorded results below.
**Assumptions** — The request handler calls `check(tx)` before side effects and `store(tx, ...)` before committing that same transaction, as STM-16 requires. The error translation catalog is owned by T-306, which already names `idempotency_mismatch` for `en` and `id`.
**Test plan** — AC1: Given semantically identical JSON bodies with different key order/whitespace, when hashed for the same method/route, then hashes match → `request_hash_uses_canonical_json`. AC2: Given a missing or noncanonical UUID, when parsing the header, then return `bad_request`; a canonical lowercase UUID is accepted → `idempotency_key_requires_canonical_lowercase_uuid`. AC3: Given a body and idempotency key containing a fake secret, when formatting the request, then neither value appears → `debug_redacts_request_body`. Edge: missing, uppercase and canonical UUIDs; reordered JSON keys. Error: malformed key maps to 400; mismatched existing request maps to 422 in T-115ab. Abuse: redacted Debug, tenant scope from Tx, and lock/replay/rollback behavior in T-115ab.
**Summary** — Added the Core HTTP extractor, transaction-bound idempotency guard, stored response replay and canonical SHA-256 request hash. Registered the 422 mismatch code and regenerated OpenAPI; the dependent T-115ab task owns real HTTP replay, mismatch, concurrency and rollback acceptance tests.
**Spec read** — `spec/06-domain-model.md` §4.11; `spec/07-api-conventions.md` API-08; `spec/16-code-quality.md` §§9/12; `spec/17-clean-architecture.md` §§1/2/3; `spec/18-security-by-design.md` SBD-50/57/81; `spec/19-error-handling.md` ERR-21/22/30; `spec/20-state-machines.md` STM-16; `spec/23-contracts-and-canonical-forms.md` CAN-30/CAN-70; `spec/29-erd.md` Core `idempotency_keys` dictionary, ERD-03/06 and §7 mapping (`0003_core_policies_files.sql`, T-101).
**Files** — `crates/eq-core/Cargo.toml`; `Cargo.lock` (Core dependency hunk only); `crates/eq-core/src/error.rs`; `crates/eq-core/src/http.rs`; `crates/eq-core/src/http/idempotency.rs`; `crates/eq-core/src/idempotency.rs`; `crates/eq-core/src/idempotency/repo.rs`; `crates/eq-core/src/lib.rs`; four `.sqlx/query-*.json` entries for the Core idempotency queries; `web/openapi.json`; the T-115 task decomposition and TD-198–203 decision records required to preserve the generated task index.
**Acceptance criteria → tests** — AC1 → `request_hash_uses_canonical_json` (pass); AC2 → `idempotency_key_requires_canonical_lowercase_uuid` (pass); AC3 → `debug_redacts_request_body` (pass). HTTP replay/mismatch/concurrency/rollback are assigned to T-115ab and are not claimed here.
**Checks run** — `CARGO_TARGET_DIR=$HOME/.cache/ritma/target SQLX_OFFLINE=true cargo test -p eq-core --lib http::idempotency --locked` (3/3 before and after the fixture adjustment); `CARGO_TARGET_DIR=$HOME/.cache/ritma/target SQLX_OFFLINE=true cargo test -p eq-core --lib --locked` (117/117); `CARGO_TARGET_DIR=$HOME/.cache/ritma/target SQLX_OFFLINE=true cargo test --workspace --lib --locked` (eq-api 2/2, eq-audit 6/6, eq-core 117/117, eq-identity 43/43; other library targets had no tests); `CARGO_TARGET_DIR=$HOME/.cache/ritma/target SQLX_OFFLINE=true cargo test --workspace --no-run --locked` (all test targets compiled); `CARGO_TARGET_DIR=$HOME/.cache/ritma/target cargo fmt --all --check`; `CARGO_TARGET_DIR=$HOME/.cache/ritma/target SQLX_OFFLINE=true cargo clippy --workspace --all-targets --locked -- -D warnings`; `cargo sqlx prepare --workspace -- --all-targets` and `cargo sqlx prepare --check --workspace -- --all-targets`; `cargo deny check`; `scripts/erd.py --migrations` and `scripts/erd.py --check`; `scripts/check-deps.sh && scripts/check-literals.sh && scripts/check-web-boundaries.sh && scripts/check-layers.sh`; `CARGO_TARGET_DIR=$HOME/.cache/ritma/target SQLX_OFFLINE=true make api`; `pnpm -C web lint`, `pnpm -C web typecheck`, `pnpm -C web test` (1/1); `scripts/tasks-index.sh --check`; `python3 -I scripts/tasks-lint.py`; `git diff --cached --check`; and `gitleaks protect --staged --redact --no-banner` all passed. Not run: full `cargo test --workspace`, because DB integration tests start additional Testcontainers and the user permits only existing `ritma-db-1`; `pnpm -C web i18n:check`, because `web/package.json` has no such script yet (T-306 owns it). The initial SQLx prepare using `eq_app` could not compile a pre-existing test query that selects owner-only `public.tenants`; metadata generation and `--check` passed using the owner connection without changing app-role grants or RLS.
**Manual checks** — No production endpoint is added in this Core child, so `make dev`/curl and browser E2E are n/a; T-115ab exercises HTTP behavior. PostgreSQL 16 in the existing `ritma-db-1` was used for SQLx metadata and the query plan; lookup ran as `eq_app` and used `idempotency_keys_pkey` with the RLS tenant filter. After checks, the created `ritma_test_T-115aa` database was dropped and the container was returned to its initially stopped state.
**Debug log** — SQLx prepare using `eq_app` reported `permission denied for table tenants` while compiling an existing Core test that reads the owner-only table. Retried metadata generation with the owner connection; generation and `--check` passed. The first `DROP DATABASE` command omitted identifier quotes around the hyphenated name; the quoted command then dropped only `ritma_test_T-115aa`. Staged Gitleaks initially flagged the UUID test fixture as `generic-api-key`; replaced it with the required obvious fake `test-secret-idempotency-key`, reran the focused tests and staged scan, and both passed. No application grant, RLS rule, schema or Gitleaks rule was changed.
**Performance** — `EXPLAIN` for the tenant/key lookup returned `Index Scan using idempotency_keys_pkey`, constrained by both primary-key columns, with the RLS tenant setting as a one-time filter. No benchmark was required or run.
**Security** — SBD-81: (1) no endpoint or auth mode was added; consuming handlers retain permission checks; (2) tenant id comes only from `Tx`, is included with key in every query, and existing table RLS remains enabled; (3) no new persisted field/data class; ERD-03 marks all existing table fields S1; request key/body/response/hash are redacted from Debug and never logged; (4) request body uses `ValidatedJson`, key must be canonical lowercase UUID, and route uses `MatchedPath`; (5) response is JSON, with no HTML/CSV output; (6) no authentication token or cryptographic key is introduced; request hashes use CAN-70 SHA-256; (7) no external calls; (8) advisory transaction lock, request comparison, successful response storage and business writes share one `Tx`; a mismatch returns the spec-defined 422; (9) no new security event or independent business state change; (10) redaction, parsing and canonical hash tests pass, with real mismatch/concurrency/rollback abuse cases assigned to T-115ab. SQL is parameterized and lives in the Core repo. No `#[allow]` or secret fixture was added.
**Deviations** — RATIFY TD-199: defer lookup to handler `Tx` so it shares the business transaction; TD-200: use the specific 422 mismatch status; TD-201: use ERD S1 for all stored columns; TD-203: use schema/CAN-70 `request_hash` despite API-08's `response_body_hash` wording. Supporting T-115 decomposition TD-198/202 is recorded in the parent/aggregate Reports. No migration, table or column changed.
**Dependencies affected** — T-115ab consumes the Core extractor/guard and tests it through HTTP; T-115a remains the aggregate dependency for T-115b. T-306 already owns the future `en`/`id` translation strings for `idempotency_mismatch`.
**Rollback plan** — Revert the signed T-115aa commit; the existing schema is unchanged and no persistent data change is made.
**Follow-ups** — T-115ab: run its exact HTTP replay, 422 mismatch, concurrency and rollback tests against a fresh disposable PostgreSQL 16 task DB, then drop only that DB. T-306: add the already-specified `errors.idempotency_mismatch` strings to both locale catalogs and implement `i18n:check`. Full workspace DB integration tests remain unrun under the one-container rule.
**Workflow steps 0–12** — 0: resumed `T-115aa` in progress on `dev`; verified the current task queue and required fixed target/database resources. 1: read the full task, `AGENTS.md`, `CLAUDE.md` and cited contracts. 2: read dependency Reports/deviations/follow-ups/RATIFY and inspect T-101's Core table/migration, T-104's `Tx`, and the current Core/API boundaries. 3: n/a (backend task). 4: recorded the plan, AC mapping, edge/error/abuse cases above. 5: implemented the extractor/guard, Core query boundary, tests and mismatch registry. 6: n/a for a production route; executed unit tests and inspected the live PostgreSQL query plan; T-115ab owns HTTP runtime coverage. 7: recorded the SQLx role error and successful owner-connection retry above. 8: reviewed layer ownership, errors, no-allow hygiene, RLS and the task diff. 9: measured the lookup plan with EXPLAIN; the primary key index is used. 10: completed the SBD-81 checklist above. 11: reran applicable Rust, SQLx, ERD, dependency, API, web, guard and task checks; full workspace DB integration suite is explicitly not run because it would create extra containers. 12: set review, regenerated/checked the index, staged only T-115 work, committed signed off, and pushed only `dev`.
**Disk/resource accounting** — Run 28 began with the fixed Cargo target at 17G; required compilation grew it to 22G. No cleanup was run. `ritma-db-1` was stopped at resume, started for this task and stopped again after dropping only the task-created `ritma_test_T-115aa`. No container, image or volume was created or removed by this task. Docker system counts changed from 8 images / 10 containers / 7 volumes at the initial sample to 8 images / 13 containers / 10 volumes at the final sample; the extra three containers and volumes were not created by any command in this task, so I left them untouched. Pre-existing untracked WIP remains unchanged.

## Questions
- Self-resolved (implementer, run 28; TD-199, needs architect ratification): extraction has no handler transaction → parse into body plus guard and perform lookup/lock with the handler Tx, as STM-16 requires.
- Self-resolved (implementer, run 28; TD-200, needs architect ratification): API-08/STM-16 specify 422 while the general ERR-21/API-07 table groups the code with 409 conflicts → use the feature-specific 422 status.
- Self-resolved (implementer, run 28; TD-201, needs architect ratification): the initial draft classified tenant identifiers as S2, but ERD-03 marks all idempotency table columns S1 → follow the ERD and retain tenant RLS.
- Self-resolved (implementer, run 28; TD-203, needs architect ratification): API-08 says `response_body_hash`, while spec 06 §4.11, ERD-03/06 and CAN-70 define request-side `request_hash` → keep the existing schema-defined canonical request hash and do not add a response-hash column.

## Review
Self-review complete. The Core transaction/query boundary, error contract, schema reuse, SBD-81 controls, hygiene and generated files match the cited task/spec contracts. Maintainer review should ratify TD-199–201 and TD-203; the parent split decisions TD-198/202 are recorded in their respective task Reports.
