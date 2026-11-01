---
id: T-115a
title: Aggregate transactional idempotency delivery
milestone: M1
status: review
depends_on: [T-101, T-104, T-111, T-113b]
spec_refs: [API-08, CAN-10, CAN-70, STM-16, ERR-21, ERR-30, SBD-57]
estimated_size: M
---

## Goal
Deliver Core idempotency replay and concurrency behavior through two reviewable child tasks.

## Context
The Core table already exists in T-101. API-08, STM-16 and CAN-70 define its canonical key, 24-hour TTL, JCS hash, tenant transaction lock, replay and mismatch response.

## Scope
### In
- Aggregate T-115aa Core implementation and T-115ab HTTP acceptance tests.
- Preserve the original acceptance criteria and keep downstream T-115b dependent on this aggregate.
### Out
- Cleanup, If-Match, rate limits, headers/cache and startup URL reporting (T-115b–T-115f).

## Files
| Path | Action | Purpose |
|---|---|---|
| tasks/M1-core/T-115a-transactional-idempotency.md | modify | Complete this aggregate's report, decision reference and status. |
| tasks/DECISIONS.md | modify | Record the child-review promotion gate. |
| tasks/INDEX.md | generate | Regenerate task index. |

## Interfaces
T-115aa exports Idempotent<T>, IdempotencyGuard and StoredResponse; T-115ab exercises them through a test-only route.

## Business rules
Use only API-08's TTL and CAN-70's canonical hash. No business value is added.

## Implementation notes
Complete T-115aa first, then T-115ab. Promote this aggregate after both children are review.

## Security
Triggers: T1, T2, T4. Use existing tenant transaction and RLS. Never log bodies, keys or responses. Child tests verify mismatch, concurrency and rollback.

## Acceptance criteria
- AC1. Same key/request replays the original response with a replay marker and one effect → idempotent_replay.
- AC2. Same tenant/key with a changed request returns 422 idempotency_mismatch without a second effect → idempotency_mismatch.
- AC3. Ten concurrent identical requests commit one effect and return equal bodies → idempotency_concurrent.

## Tests to write
| File | Test names |
|---|---|
| crates/eq-core/src/http/idempotency.rs | debug_redacts_request_body, idempotency_key_requires_canonical_lowercase_uuid, request_hash_uses_canonical_json |
| crates/eq-server/tests/http_hardening.rs | idempotent_replay, idempotency_mismatch, idempotency_concurrent, idempotency_rollback |

## Definition of done
- [x] T-115aa and T-115ab are review and the aggregate criteria are covered.
- [x] No migration or schema change was introduced.
- [x] Aggregate report and checks are complete; task index/lint pass.
- [x] Commit docs(tasks): finalize idempotency aggregate [T-115a], signed off.

## Report
**Plan** — (1) Verify the two specified children are review and honor the task's promotion note without adding forbidden forward dependencies → verify T-115aa/T-115ab statuses and the task-index guard. (2) Read both child Reports, RATIFY items and shipped test/code paths → verify the parent ACs and exact test names. (3) Complete the aggregate Report without changing implementation or public behavior → verify the original API-08/STM-16 contract is preserved. (4) Run task index/lint, diff and staged secret checks → verify all applicable checks pass.
**Assumptions** — T-115aa and T-115ab Reports and shipped code are authoritative for this aggregate; their API-08/STM-16 behavior remains unchanged. The root prerequisites remain in the dependency list for traceability. The explicit implementation note is the readiness condition for promotion because the index guard rejects forward child IDs.
**Test plan** — AC1: Given a committed key and request, when the same request is retried, then the stored response is replayed and the effect count remains one → idempotent_replay, supported by request_hash_uses_canonical_json. AC2: Given a committed key/body, when the same tenant/key is reused with a changed body, then HTTP returns 422 idempotency_mismatch without a second effect → idempotency_mismatch. AC3: Given ten simultaneous identical requests, when all complete, then their bodies match and only one effect commits → idempotency_concurrent. Edge/error/abuse cases: canonical-body hashing and redaction, concurrent first use, mismatch, and failed-transaction rollback/retry → child tests request_hash_uses_canonical_json, debug_redacts_request_body, idempotency_rollback and the three aggregate HTTP test names.
**Summary** — Verified T-115aa's Core port and T-115ab's real HTTP acceptance suite together cover the three original idempotency acceptance criteria. The aggregate adds no runtime behavior, public contract, migration or schema change. T-115a is now review, so its cleanup child T-115b can proceed.
**Spec read** — spec/07-api-conventions.md API-08; spec/23-contracts-and-canonical-forms.md CAN-10/CAN-70; spec/20-state-machines.md STM-16; spec/19-error-handling.md ERR-21/ERR-30; spec/18-security-by-design.md SBD-57/SBD-81; spec/16-code-quality.md §§9/12; spec/17-clean-architecture.md §§1/4; T-115aa and T-115ab Reports and their shipped implementation/tests.
**Files** — tasks/M1-core/T-115a-transactional-idempotency.md (corrected child review gate, status and aggregate Report); tasks/DECISIONS.md (TD-204); tasks/INDEX.md (generated status).
**Acceptance criteria → tests**
| AC | Test | Result |
|---|---|---|
| AC1 same key/request replays response with one effect | T-115aa request_hash_uses_canonical_json; T-115ab idempotent_replay | ✅ Both child Reports record passing results; T-115aa 3/3 Core tests, T-115ab HTTP case passed within 4/4. |
| AC2 changed body returns 422 mismatch without second effect | T-115ab idempotency_mismatch | ✅ Child Report records pass; response code/status and single effect asserted. |
| AC3 ten identical requests commit one effect and return equal bodies | T-115ab idempotency_concurrent | ✅ Child Report records pass; ten concurrent requests and one effect asserted. |
| Additional rollback abuse case | T-115ab idempotency_rollback | ✅ Child Report records pass; failed transaction rolled back and successful retry committed once. |
**Checks run** — Verified from the committed child Reports, not rerun in this docs-only aggregate: T-115aa commit d09ea7f5dbb1b5844fddc296ce4dda307f3128da records CARGO_TARGET_DIR=$HOME/.cache/ritma/target SQLX_OFFLINE=true cargo test -p eq-core --lib http::idempotency --locked (3/3); T-115ab commit 8637fb920e5e0b000f84bc26c82a1b6f37e5d22e records the HTTP integration command on fresh PostgreSQL 16 database ritma_test_T-115ab (4/4). In this aggregate, all passed: CARGO_TARGET_DIR="$HOME/.cache/ritma/target" cargo fmt --all --check; CARGO_TARGET_DIR="$HOME/.cache/ritma/target" SQLX_OFFLINE=true cargo clippy --locked --workspace --all-targets -- -D warnings; scripts/check-deps.sh && scripts/check-literals.sh && scripts/check-web-boundaries.sh && scripts/check-layers.sh; python3 -I scripts/erd.py --check; python3 -I scripts/erd.py --migrations; scripts/tasks-index.sh --check; python3 -I scripts/tasks-lint.py; git diff --check; staged gitleaks protect --staged --redact. Not run here: cargo test --workspace because unrelated database suites may start extra PostgreSQL containers; make dev and browser make e2e because this aggregate changes no runtime behavior and is not an integration task; web checks because no web files changed; cargo sqlx prepare because no query macro changed. Source/runtime test evidence and other environment limits are recorded in the child Reports.
**Manual checks** — Compared the exact AC/test names to the child Reports and current code. The child HTTP suite issued real loopback HTTP requests as eq_app (NOSUPERUSER, NOBYPASSRLS) against fresh PostgreSQL 16 database ritma_test_T-115ab, then dropped that database; no runtime was restarted for this aggregate.
**Debug log** — The first dependency edit added T-115aa/T-115ab to depends_on, but scripts/tasks-index.sh rejected both as forward IDs with “dependency … is not lower (cycle risk).” Per TD-204, reverted that dependency edit and kept the guard unchanged; the task's explicit “promote after both children are review” condition is satisfied now and is recorded in Questions/DECISIONS. The rerun of the index and linter passed. No other failure.
**Performance** — n/a; aggregate-only task, no implementation or query change.
**Security** — No new security boundary or data was added. Verified the child audits: T-115aa keeps key/body/response redacted, uses canonical validation and the transaction-bound guard; T-115ab uses a test-only route, a fixed tenant, eq_app (NOSUPERUSER, NOBYPASSRLS), db::begin_system, and a single Tx for effect plus idempotency write. The child tests cover mismatch, concurrency and rollback. No new security finding. The aggregate retains inherited SBD-57/SBD-81 RATIFY notes from the child Reports.
**Deviations** — RATIFY TD-202 (split the oversized idempotency work into Core and HTTP-test children) and TD-204 (promote only after both children are review while preserving the task-index guard). Inherited child RATIFY items TD-199–203 remain recorded in their respective task Reports; this aggregate adds no implementation deviation.
**Dependencies affected** — T-115b is now eligible because T-115a is review; T-115c–f remain gated by their ordered predecessors. The parent T-115 aggregate remains draft until all six parts T-115a–f are review. No code interface changed.
**Rollback plan** — Revert this signed documentation commit. It changes only task/decision/index records; no database, migration, source code or production behavior changed.
**Follow-ups** — T-115b owns cleanup of expired keys. T-306 owns the already-specified en/id translation for idempotency_mismatch. Maintainer review should ratify TD-202/204 and inherited TD-199–203.
**Workflow steps 0–12** — 0: verified dev equals origin/dev, task index/lint passed, no task was in progress, and unrelated WIP was preserved; promoted T-115a only after both named children reached review, per its own implementation note. 1: read the full task and cited spec clauses. 2: read the T-115aa/T-115ab Reports (deviations, follow-ups, RATIFY), inspect their shipped Core guard and HTTP route/tests, and review the earlier prerequisites' Reports. 3: n/a (backend/documentation aggregate). 4: recorded plan, assumptions and AC/test mapping before aggregate verification. 5: completed the aggregate Report and TD-204 without changing runtime code. 6: n/a for new behavior; actual HTTP execution is documented by T-115ab. 7: recorded and resolved the task-index forward-ID rejection above. 8: reviewed the documentation diff and confirmed only the aggregate task, decision register and generated index changed. 9: n/a. 10: reviewed the inherited SBD-81 findings; no new security surface. 11: ran fmt, workspace Clippy, all four guards, both ERD checks, task index/lint, diff and staged secret checks; did not run the workspace integration suite because it may create extra containers, and child Reports contain the focused runtime checks and explicit Not run reasons. 12: set review, regenerated/checked the index, committed one signed task change and pushed only dev.
**Disk/resource accounting** — Before this docs-only task:
```text
$ du -sh "$HOME/.cache/ritma/target"
22G /home/kepolu/.cache/ritma/target
$ docker system df
TYPE            TOTAL     ACTIVE    SIZE      RECLAIMABLE
Images          8         7         14.55GB   3.595GB (24%)
Containers      13        11        10.49MB   0B (0%)
Local Volumes   10        10        5.466GB   0B (0%)
Build Cache     0         0         0B
```
After this docs-only task:
```text
$ du -sh "$HOME/.cache/ritma/target"
22G /home/kepolu/.cache/ritma/target
$ docker system df
TYPE            TOTAL     ACTIVE    SIZE      RECLAIMABLE
Images          8         7         14.55GB   3.595GB (24%)
Containers      13        11        10.49MB   0B (0%)
Local Volumes   10        10        5.466GB   0B (0%)
Build Cache     0         0         0B
```
No database, container, image or volume was created or removed; ritma-db-1 remained stopped. All unrelated WIP and non-Ritma resources were left untouched.

## Questions
- Self-resolved (implementer, run 28; TD-202, needs architect ratification): T-115a's Core port and HTTP tests exceed CQ-101's review-size guidance together → split into ordered Core and HTTP-test children while keeping T-115a as the downstream aggregate.
- Self-resolved (implementer, run 29; TD-204, needs architect ratification): T-115a's implementation notes require both child tasks to be review before promotion but task-index rejects the longer T-115aa/T-115ab IDs as forward dependencies → enforce the explicit review-before-promotion condition manually and keep the dependency guard unchanged.

## Review
Self-review complete. The aggregate ACs map to the exact tests in the two review children; the task-index guard remains unchanged, and the promotion condition is enforced per the task implementation note. No code, schema, migration or public contract changed.
