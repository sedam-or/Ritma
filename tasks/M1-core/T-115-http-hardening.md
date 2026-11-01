---
id: T-115
title: Add Idempotency-Key replay, ETag/If-Match helpers, rate limits and security headers
milestone: M1
status: draft
depends_on: [T-101, T-104, T-111, T-113b]
spec_refs: [API-07, API-08, API-15, SEC-06, SEC-11, SEC-20, CAN-31, CAN-70, STM-16]
estimated_size: M
---

## Goal
The HTTP layer enforces cross-cutting API rules once, so module handlers can opt in to safe retries, optimistic concurrency, rate limits and browser security headers.

## Context
- API-08 uses the existing Core idempotency_keys table, canonical request hash, transaction-scoped response replay and a 24-hour retention window; expired keys are cleaned daily.
- API-07 uses strong version ETags and required If-Match on the three routes named in this task.
- API-15 sets the exact general, login, public and upload limits listed in the child parts, keyed by user or trusted client IP.
- SEC-20 supplies the CSP and browser headers; CAN-31/D-10 define no-store API caching with the existing meta and signed-download exceptions.
- SEC-06 requires a fixed warning and meta flag for public non-local HTTP configuration.

## Scope
### In
- Final aggregate verification and reporting after T-115a–T-115f are review: confirm that the original acceptance criteria and contract are covered by child tests, record cross-part follow-ups, and complete this task's Report.
### Out
- All implementation and tests; they are assigned to T-115a–T-115f.
- Captcha and honeypot (ENR-03, M4).

## Split
This task combines transactional idempotency, a cleanup job, conditional writes, rate limiting, response headers/cache behavior, and startup URL security reporting. Split it into six ordered implementation parts so each change has one reviewable concern under CQ-101. Keep the original acceptance names and public behavior unchanged. T-116, T-116a, and T-303 continue to depend on this aggregate.

| Part | Owns | Acceptance tests | Depends on |
|---|---|---|---|
| T-115a | Transactional idempotency parsing, JCS hash, replay, mismatch and concurrency | idempotent_replay, idempotency_mismatch, idempotency_concurrent | T-101, T-104, T-111, T-113b |
| T-115b | core.idempotency_cleanup scheduled job | new cleanup lifecycle tests | T-115a |
| T-115c | Shared ETag/If-Match helpers and the three specified conditional-write routes | new If-Match tests | T-115b |
| T-115d | Shared rate-limit layers, keys and headers | login_rate_limited, xff_untrusted_ignored | T-115c |
| T-115e | SEC-20 response headers and API cache policy | security_headers_present | T-115d |
| T-115f | SEC-06 public-URL warning and /meta/app.insecure_public_url | startup/config/meta tests | T-115e |

After all six parts are review, promote this aggregate to todo, verify the original contract and reports together, and set it to review. Do not reimplement child scope here.

## Files
| Path | Action | Purpose |
|---|---|---|
| tasks/M1-core/T-115a-*.md | create | Transactional idempotency child contract. |
| tasks/M1-core/T-115b-*.md | create | Cleanup-job child contract. |
| tasks/M1-core/T-115c-*.md | create | If-Match child contract. |
| tasks/M1-core/T-115d-*.md | create | Rate-limit child contract. |
| tasks/M1-core/T-115e-*.md | create | Response-header child contract. |
| tasks/M1-core/T-115f-*.md | create | Public-URL security child contract. |
| tasks/DECISIONS.md | modify | Record the CQ-101 split as TD-198. |
| tasks/INDEX.md | generate | Regenerate after creating child tasks. |

## Interfaces
Use only the exact domain signatures, DTOs, SQL DDL, endpoints and wire values in this aggregate's original contract and the child task files. Do not add a public contract.

## Business rules
Apply only the policy keys, defaults, ranges and state transitions in the original contract. If a value is absent, do not infer one.

## Security
- Triggers: T1, T2, T4 (SBD-30).
- Data classes touched: S1 per ERD-04/ERD-05; do not create a data class outside spec/29-erd.md §5.
- Spoofed identity, unauthorized role or tenant scope → auth mode and permission checks precede data access; scope in SQL and RLS remain mandatory (SBD-40/41/42, AUTH-05, SEC-05).
- S2/S3 disclosure or secret leakage → encrypt/hash as specified, require read_pii for sensitive reads, and never log personal data or secret values (SBD-10/21/23, SEC-02/03/21).
- Abuse criteria remain assigned to the corresponding child tests.

## Implementation notes
Use the ordered T-115a–T-115f contracts. Preserve the shipped dependency boundaries, schema, error codes, endpoints and task-defined technical limits. The aggregate performs verification and reporting only.

## Acceptance criteria
- AC1. Same key and same body replay the original response with Idempotent-Replayed: true and one business row → idempotent_replay.
- AC2. Same key and different body returns 422 idempotency_mismatch → idempotency_mismatch.
- AC3. Ten concurrent requests with one key create one business row and return identical responses → idempotency_concurrent.
- AC4. The 11th login in one minute from one IP returns 429 with Retry-After → login_rate_limited.
- AC5. SEC-20 headers appear on /api/v1/meta/app and /; HSTS is absent for HTTP and present for HTTPS → security_headers_present.
- AC6. X-Forwarded-For from an untrusted peer is ignored for rate limiting → xff_untrusted_ignored.

## Tests to write
| File | Test names |
|---|---|
| crates/eq-server/tests/http_hardening.rs | idempotent_replay, idempotency_mismatch, idempotency_concurrent, login_rate_limited, security_headers_present, xff_untrusted_ignored |

## Definition of done
- [ ] T-115a–T-115f are review and every original acceptance criterion is covered by its assigned test.
- [ ] The original HTTP behavior, security controls, schema and dependency policies remain intact.
- [ ] Run the task index and task linter before commit.
- [ ] Commit docs(tasks): finalize HTTP hardening aggregate [T-115], signed off.

## Report
**Plan** — n/a until T-115a–T-115f are review.
**Assumptions** — Child tasks preserve every contract and acceptance name; splitting introduces no endpoint, error code, permission or schema change.
**Test plan** — Verify child Reports and exact test names against AC1–AC6; run tasks-index --check and tasks-lint before the aggregate commit.
**Deviations** — RATIFY TD-063 (downstream tasks stay on the aggregate until all parts are review); TD-067 (required headings retained); TD-066 (existing spec-backed Security section retained); TD-198 (six ordered CQ-101 parts preserve the contract).
**Summary, Spec read, Files, AC → tests, Checks run, Manual checks, Debug log, Performance, Security, Dependencies affected, Rollback plan, Follow-ups** — n/a until aggregate verification.

## Questions
- Self-resolved (implementer, run 28, needs architect ratification; TD-198): T-115 combines six cross-cutting concerns under one M-sized task and omits implementation paths → split into T-115a–T-115f in dependency order, preserving all original behavior and acceptance names, because CQ-101 requires larger work to be split and child contracts follow only existing task/spec interfaces.
- Self-resolved (implementer, run 6, needs architect ratification; TD-063): this task depended on the original parent contract after its CQ-101 split → downstream tasks continue to depend on this aggregate until every child part is review.
- Self-resolved (implementer, run 6; TD-067): required task-template sections are kept and completed from the existing task contract only.
- Self-resolved (implementer, run 6; TD-066): the existing spec-backed Security section classifies the task's SBD-30 triggers.

## Review
n/a — aggregate verification starts after the child parts are review.
