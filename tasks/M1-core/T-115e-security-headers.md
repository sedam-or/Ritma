---
id: T-115e
title: Apply response security headers and cache policy
milestone: M1
status: todo
depends_on: [T-115d]
spec_refs: [SEC-20, CAN-31, D-10, SBD-20]
estimated_size: S
---

## Goal
Apply the canonical SEC-20 browser security headers and CAN-31 API cache policy to the existing server router.

## Context
- On every API and SPA response set the CSP and headers specified by SEC-20: X-Content-Type-Options nosniff, Referrer-Policy strict-origin-when-cross-origin, Permissions-Policy camera=(self), geolocation=(self), microphone=(), and Cross-Origin-Opener-Policy same-origin.
- Use the exact task CSP: default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: blob: <tile-host>; connect-src 'self'; font-src 'self'; worker-src 'self' blob:; frame-ancestors 'none'; base-uri 'self'; form-action 'self'.
- Add HSTS only when RITMA_PUBLIC_URL uses HTTPS: max-age=31536000; includeSubDomains.
- Every /api response uses Cache-Control no-store, except signed file downloads and /meta/app which use max-age=60.
- CSP tile host comes from RITMA_MAP_TILE_HOST, default tile.openstreetmap.org.

## Scope
### In
- Replace the existing security_headers_stub and enforce the listed cache policy without overwriting route-specific signed-file cache headers.
### Out
- Public URL warning and metadata field (T-115f), limiter behavior (T-115d), and new browser UX.

## Files
| Path | Action | Purpose |
|---|---|---|
| crates/eq-api/src/health.rs | modify | Install the response layer in the router. |
| crates/eq-api/src/headers.rs | create if appropriate | Keep response policy in one shared implementation. |
| crates/eq-server/tests/http_hardening.rs | modify | Verify API, meta and SPA headers/cache values. |
| crates/eq-core/src/config.rs | modify | Read/validate RITMA_MAP_TILE_HOST with its specified default. |

## Interfaces
eq_core::http::security_headers_layer; preserve response type and router composition.

## Business rules
Use only the exact CSP, HSTS and cache exceptions stated above. Do not add origins or exceptions.

## Security
- Triggers: T1, T4 (SBD-30).
- Validate the CSP tile host as a host value and serialize it safely; never derive it from an untrusted request header.
- Abuse tests: verify required headers on API and SPA paths and ensure HTTP does not receive HSTS.

## Implementation notes
Use existing matched-route data or route metadata to identify documented cache exceptions. Do not apply signed-download behavior to other endpoints.

## Acceptance criteria
- AC1. security_headers_present verifies every required header, SPA/API coverage, HSTS conditionality, and /meta/app/signed-file cache exceptions.

## Tests to write
| File | Test names |
|---|---|
| crates/eq-server/tests/http_hardening.rs | security_headers_present |

## Definition of done
- [ ] Header/cache integration tests and Rust checks pass; run web checks only if web sources change.
- [ ] Commit feat(core): apply response security headers [T-115e], signed off.

## Report
Filled by the implementer using WORKFLOW.md §3 Step 12.

## Questions
n/a.

## Review
n/a.
