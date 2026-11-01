# 30 — Production readiness for v1.0

This document lists what an enterprise-grade open-source release needs beyond the features in M0–M7. Each requirement has an ID (`PRD-nn`), an owner task and a status. Values marked **proposed** are defaults for the maintainer to confirm; they are configuration, not code literals (POL-02).

| ID | Requirement | Owner | Status |
|---|---|---|---|
| PRD-01 | Service objectives and indicators are defined for the hosted demo and the reference deployment: availability (proposed 99.5 % per month), `/healthz` success rate, and the performance budget (T-711). | T-710b | draft |
| PRD-02 | Incident response: severity levels (proposed S1–S4), roles, a runbook, and a blameless postmortem template. | T-710b | draft |
| PRD-03 | Restore drill: a restore from a backup on a clean machine is run and recorded every quarter, and before each release tag. | T-710b, T-701 | draft |
| PRD-04 | Upgrade compatibility: a dump of the previous release upgrades to the current one with row counts and policy versions verified, on PostgreSQL 16. | T-710a | draft |
| PRD-05 | Vulnerability response: advisories triaged within 5 working days, critical fixes released within 14 days (both **proposed**, maintainer to confirm). | T-710c | draft, values open |
| PRD-06 | Key management: rotation schedule and break-glass procedure for the master key and tenant DEKs. | T-710c, T-705 | draft |
| PRD-07 | External security review before v1.0: scope and budget are written down; whether it is required for v1.0 is a maintainer decision. | T-710c | draft, decision open |
| PRD-08 | Privacy notice and data-processing agreement templates for school operators (UU PDP). Templates, not legal advice. | T-710d | draft |
| PRD-09 | Accessibility statement with WCAG 2.2 AA conformance and the known gaps. | T-710d, T-711 | draft |
| PRD-10 | Licence compliance at release: THIRD_PARTY_LICENSES.md generated from `Cargo.lock` and the pnpm lock; REUSE check in CI. | T-710e | draft |
| PRD-11 | Telemetry: opt-in only, documented payload (T-703). Whether it ships in v1.0 is a maintainer decision (YAGNI). | T-703 | decision open |
| PRD-12 | Community: a dedicated conduct contact (MAINTAINERS.md), and the contributor agreement (DCO or CLA, ADR-0009). | — | decision open |

## Rules

1. A requirement is `done` only when its owner task is `done`. Draft owners are promoted by the architect when their open decisions are recorded in `tasks/DECISIONS.md`.
2. Operational documents live in `docs/operations/` and legal templates in `docs/legal/`, and are linked from `docs/README.md`.
3. The release checklist (RELEASING.md) cites this document. A release is not tagged while a `v1.0` requirement is open without a recorded decision.
