---
id: T-115f
title: Report insecure public HTTP URL configuration
milestone: M1
status: todo
depends_on: [T-115e]
spec_refs: [SEC-06, SBD-20, API-14]
estimated_size: S
---

## Goal
Make insecure public HTTP configuration visible at startup and in public app metadata without exposing URL credentials or other sensitive data.

## Context
At startup, when RITMA_PUBLIC_URL is HTTP and its host is not localhost or a private/link-local IP, log a warning containing Insecure public URL and expose meta/app.insecure_public_url = true. Otherwise expose false. The public URL is already validated by T-002.

## Scope
### In
- Add computed insecure_public_url to /meta/app.
- Emit the SEC-06 startup warning for only the specified public HTTP case; do not log the URL itself.
### Out
- Changing URL parsing, private-address classification, HSTS/header behavior, or configuration defaults.

## Files
| Path | Action | Purpose |
|---|---|---|
| crates/eq-core/src/config.rs | modify if needed | Reuse validated public URL and classify the specified host classes. |
| crates/eq-server/src/main.rs | modify | Emit startup warning without sensitive URL content. |
| crates/eq-api/src/meta.rs | modify | Expose insecure_public_url in metadata. |
| crates/eq-api/tests/http_hardening.rs | modify | Verify metadata by configuration class. |

## Interfaces
GET /api/v1/meta/app includes boolean insecure_public_url. No new route or error code.

## Business rules
Use the exact localhost/private/link-local condition from SEC-06. No new default or public URL class is inferred.

## Security
- Triggers: T1, T4 (SBD-30).
- Treat public URL configuration as sensitive; emit a fixed warning only, never the origin, credentials or tenant information (SEC-02/21).
- Abuse tests: public HTTP is flagged, and credential-bearing values never appear in logs.

## Implementation notes
Use startup configuration and current metadata builder; preserve existing public response fields.

## Acceptance criteria
- AC1. Public non-local HTTP emits the fixed warning and sets metadata true.
- AC2. HTTPS, localhost, private and link-local hosts do not warn and set metadata false.

## Tests to write
| File | Test names |
|---|---|
| crates/eq-core/src/config/tests.rs | insecure_public_url_classification |
| crates/eq-api/tests/http_hardening.rs | meta_reports_insecure_public_url |

## Definition of done
- [ ] Config/meta tests, Rust checks, OpenAPI generation and guards pass or are explicitly reported Not run.
- [ ] Commit feat(core): flag insecure public URL configuration [T-115f], signed off.

## Report
Filled by the implementer using WORKFLOW.md §3 Step 12.

## Questions
n/a.

## Review
n/a.
