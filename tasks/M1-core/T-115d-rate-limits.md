---
id: T-115d
title: Apply shared API rate limits
milestone: M1
status: todo
depends_on: [T-115c]
spec_refs: [API-15, SEC-11, AUTH-05, SBD-20, SBD-40]
estimated_size: M
---

## Goal
Apply the T-115 shared API-15 rate-limit layers using the approved tower_governor dependency and trusted client-IP resolution.

## Context
- General: 100/minute per authenticated user and 300/minute per IP.
- POST /auth/login: 10/minute per IP.
- /api/v1/public/*: 20/minute per IP.
- Upload layer: 20/minute per user, exposed as RateLimitLayer::uploads() for later routes.
- 429 uses rate_limited with Retry-After; responses include RateLimit-Limit, RateLimit-Remaining, and RateLimit-Reset.
- IP attribution uses shipped client_ip and honors only configured trusted proxies.

## Scope
### In
- Implement and wire the shared general, login, public and upload limiters.
- Verify user/IP keys and response headers follow the exact T-115 contract.
### Out
- Changing any rate, login behavior, CAPTCHA, per-tenant policy, or upload endpoint implementation.

## Files
| Path | Action | Purpose |
|---|---|---|
| crates/eq-core/src/http.rs | modify | Export the rate-limit layer contract. |
| crates/eq-core/src/http/rate_limit.rs | create | Construct keyed layers and response metadata. |
| crates/eq-api/src/health.rs | modify | Wire shared layers into the API router. |
| crates/eq-api/src/middleware.rs | modify if present | Reuse shipped trusted client-IP function. |
| Cargo.toml | modify if needed | Workspace dependency declaration if not already present. |
| crates/eq-api/Cargo.toml | modify if needed | Runtime dependency activation. |
| crates/eq-server/tests/http_hardening.rs | modify | Exact login and untrusted-XFF tests. |

## Interfaces
eq_core::http::rate_limit and RateLimitLayer::uploads(). Use exact routes, keys, statuses and headers from the task.

## Business rules
The listed request limits are the task's exact API-15 constants. Do not alter or move them to tenant policy.

## Security
- Triggers: T1, T2, T4 (SBD-30).
- Keys derive from authenticated subject and trusted client_ip; forwarded headers from untrusted peers cannot choose the key (AUTH-05/SBD-40).
- Abuse tests: login_rate_limited proves the limit is enforced; xff_untrusted_ignored proves spoofing cannot evade the IP bucket.

## Implementation notes
Reuse workspace-pinned tower_governor 0.7; do not add a cargo-deny exception. Use technical constants with // technical: comments only for library implementation bounds.

## Acceptance criteria
- AC1. The 11th login in one minute from one IP returns 429 with Retry-After and RateLimit headers → login_rate_limited.
- AC2. An untrusted peer's X-Forwarded-For does not change its rate-limit key → xff_untrusted_ignored.
- AC3. General, public and upload layers are available with the task-specified rates and keys.

## Tests to write
| File | Test names |
|---|---|
| crates/eq-server/tests/http_hardening.rs | login_rate_limited, xff_untrusted_ignored |

## Definition of done
- [ ] Exact rate-limit tests pass; Rust, deny, OpenAPI and guards pass or are explicitly reported Not run.
- [ ] Commit feat(core): apply shared API rate limits [T-115d], signed off.

## Report
Filled by the implementer using WORKFLOW.md §3 Step 12.

## Questions
n/a.

## Review
n/a.
