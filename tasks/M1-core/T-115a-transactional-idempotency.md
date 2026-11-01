---
id: T-115a
title: Aggregate transactional idempotency delivery
milestone: M1
status: draft
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
| tasks/M1-core/T-115aa-core-idempotency.md | create | Core port child task. |
| tasks/M1-core/T-115ab-idempotency-http-tests.md | create | HTTP acceptance child task. |
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
- [ ] T-115aa and T-115ab are review and the aggregate criteria are covered.
- [ ] No migration or schema change was introduced.
- [ ] Aggregate report and checks are complete; task index/lint pass.
- [ ] Commit docs(tasks): finalize idempotency aggregate [T-115a], signed off.

## Report
**Plan** — Verify both child Reports and test names against AC1–AC3, then summarize cross-child behavior.
**Assumptions** — Children preserve the original API-08/STM-16 contract.
**Test plan** — AC1–AC3 map to the tests above; verify edge, error and abuse cases from child Reports.
**Summary** — Pending child review.
**Spec read** — n/a until aggregate verification.
**Files** — n/a until aggregate verification.
**Acceptance criteria → tests** — Pending child review.
**Checks run** — n/a until aggregate verification.
**Manual checks** — n/a until aggregate verification.
**Debug log** — None.
**Performance** — n/a.
**Security** — Pending child review.
**Deviations** — RATIFY TD-202.
**Dependencies affected** — T-115b remains dependent on this aggregate.
**Rollback plan** — Revert the aggregate commit; no migration exists.
**Follow-ups** — None known.

## Questions
- Self-resolved (implementer, run 28; TD-202, needs architect ratification): T-115a's Core port and HTTP tests exceed CQ-101's review-size guidance together → split into ordered Core and HTTP-test children while keeping T-115a as the downstream aggregate.

## Review
Pending T-115aa and T-115ab.
