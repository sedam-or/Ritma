---
id: T-115c
title: Share If-Match parsing and version ETags
milestone: M1
status: todo
depends_on: [T-115b]
spec_refs: [API-07, CAN-30, ERR-10, STM-04]
estimated_size: M
---

## Goal
Use one canonical Core HTTP helper for strong version ETags and apply it to the three conditional-write routes named by T-115.

## Context
If-Match: v<positive integer> in strong quoted form is required on PATCH /me, PATCH /people/{id}, and PUT /settings/policies/{module}. Missing is 428 precondition_required, malformed is 400 bad_request, stale is 409 version_conflict; success emits the next strong ETag. version_conflict_with(current_dto) attaches the current DTO as specified by API-07.

## Scope
### In
- Add eq_core::http::IfMatch, etag(version), and version_conflict_with(current_dto) using the canonical interface in T-115.
- Replace ad-hoc parsers in the three specified routes without changing DTOs, permissions, SQL semantics or response shapes beyond the specified ETag.
### Out
- Idempotency, cleanup, rate limiting, response security headers, and URL reporting.

## Files
| Path | Action | Purpose |
|---|---|---|
| crates/eq-core/src/http.rs | modify | Export shared helpers. |
| crates/eq-core/src/http/if_match.rs | create | Canonical parsing/formatting. |
| crates/eq-identity/src/api_me.rs | modify | Apply helper to /me. |
| crates/eq-api/src/settings.rs | modify | Apply helper to policy writes. |
| crates/eq-identity/tests/api_me.rs | modify | Preserve and extend profile precondition tests. |
| crates/eq-api/tests/api_settings.rs | modify | Exercise policy preconditions and ETags. |

## Interfaces
IfMatch.expected_version: i32; etag(version) -> HeaderValue; missing/malformed/stale statuses and codes as stated above. Preserve the current DTO on stale conflict.

## Business rules
Only positive versions use the strong ETag form. No business value is introduced.

## Security
- Triggers: T1, T2, T4 (SBD-30).
- Use the authenticated tenant transaction and existing version predicate; never accept client tenant IDs or bypass RLS.
- Abuse tests: missing/stale version cannot mutate an aggregate; cross-tenant update remains rejected.

## Implementation notes
Keep the version compare-and-set in existing repositories. Update OpenAPI through existing route annotations if required by generation.

## Acceptance criteria
- AC1. All three routes share the helper and return the specified 428/400/409 behavior.
- AC2. Successful updates emit the resulting canonical ETag and stale updates change no state.

## Tests to write
| File | Test names |
|---|---|
| crates/eq-identity/tests/api_me.rs | me_patch_requires_if_match |
| crates/eq-api/tests/api_settings.rs | Existing If-Match test plus malformed/missing/stale coverage |

## Definition of done
- [ ] Focused API tests, Rust checks, OpenAPI generation and guards pass or are explicitly reported Not run.
- [ ] Commit feat(core): share If-Match version helpers [T-115c], signed off.

## Report
Filled by the implementer using WORKFLOW.md §3 Step 12.

## Questions
n/a.

## Review
n/a.
