---
id: T-710f
title: Remove astral-tokio-tar from the dev dependency tree and drop the RUSTSEC-2026-0440 ignore
milestone: M7
status: draft
depends_on: []
spec_refs: []
estimated_size: S
---

## Goal
The workspace no longer pulls `astral-tokio-tar` (RUSTSEC-2026-0440), so the single `ignore` entry in `deny.toml` can be removed and `cargo deny check advisories` passes with `unmaintained = "all"`.

## Context
TD-210 accepted RUSTSEC-2026-0440 because it reaches the workspace only through `testcontainers` 0.28.0, a dev-dependency, and no release of `testcontainers` without it exists yet. This task is option (b) of TD-210: replace or upgrade the path once a safe version or an alternative exists. It is a draft and must be re-checked before promotion.

## Scope
### In
- Re-check upstream: `cargo update -p testcontainers --dry-run` and `cargo tree -i astral-tokio-tar` against the newest releases.
- If a release without `astral-tokio-tar` exists, upgrade `testcontainers` and any call sites that break, with no change to test logic.
- If none exists, evaluate replacing the dev-only harness dependency (its own decision, recorded in `tasks/DECISIONS.md`) before any code change.
- Remove the `RUSTSEC-2026-0440` entry from `deny.toml` in the same change and update TD-210 to point here.
### Out
- Changing `unmaintained = "all"` or any other advisory setting.
- Changing test logic, fixtures or the nextest quarantine (TD-209).

## Files
| Path | Action | Purpose |
|---|---|---|
| `Cargo.toml`, `Cargo.lock`, affected crates' `Cargo.toml` | Modify | Upgrade or replace `testcontainers` |
| `deny.toml` | Modify | Remove the RUSTSEC-2026-0440 ignore entry |
| `tasks/DECISIONS.md` | Modify | Update TD-210 status |

## Interfaces
No public or wire interface changes.

## Business rules
No business rules. Not applicable.

## Security
No triggers.

## Implementation notes
- Keep the change to the dependency path. Do not bundle other upgrades.
- Verify with `cargo tree -i astral-tokio-tar` (expect no path) and `cargo deny check advisories`.

## Acceptance criteria
- AC1. Given the upgrade or replacement, When `cargo tree -i astral-tokio-tar` runs, Then it reports no path into the workspace.
- AC2. Given the ignore entry is removed, When `cargo deny check advisories` runs, Then it passes with `unmaintained = "all"`.
- AC3. Given the change, When the rust CI job runs, Then the existing test suite passes unchanged.

## Tests to write
| File | Test names |
|---|---|
| n/a | Verified by `cargo deny check advisories` and the rust CI job (no new tests) |

## Definition of done
- [ ] `cargo fmt --all --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace` pass
- [ ] `cargo deny check` passes with the ignore entry removed
- [ ] `tasks/DECISIONS.md` TD-210 updated
- [ ] Commit `<type>(<scope>): <title> [T-710f]`, signed off

## Report
_Filled in by the implementer using the structure in `WORKFLOW.md` §3 Step 12._

## Questions
_Filled in by the implementer if blocked (`WORKFLOW.md` §4)._

## Review
_Filled in by the reviewer if changes are requested (`WORKFLOW.md` §5)._
