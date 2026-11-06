---
id: T-710e
title: Generate third-party licences and run the REUSE check at release
milestone: M7
status: todo
depends_on: [T-704]
spec_refs: [PRD-10, OSS-20]
estimated_size: S
---

## Goal
Every release ships a generated THIRD_PARTY_LICENSES.md, and the REUSE check runs in CI.

## Context
PRD-10 (spec/30). `cargo deny list` (already in the stack, OSS-20) lists crate licences. pnpm lists package licences. No new tool is added. The REUSE check uses the `reuse` tool, which is added to the release job only.

## Scope
### In
- `scripts/gen-third-party.sh`: writes THIRD_PARTY_LICENSES.md from `cargo deny list` and `pnpm -C web licenses list --json`.
- A REUSE check in the `docs` job of CI.
- The generated file is attached to the release.
### Out
- Changing the licence allow list (deny.toml); any change needs its own decision.

## Files
| Path | Action | Purpose |
|---|---|---|
| `scripts/gen-third-party.sh` | create | Generator. |
| `scripts/tests/test_third_party.py` | create | Coverage tests. |
| `THIRD_PARTY_LICENSES.md` | modify | Replace the placeholder with the generated list. |
| `.github/workflows/ci.yml` | modify | REUSE step. |
| `.github/workflows/release.yml` | modify | Attach the file. |

## Interfaces
`scripts/gen-third-party.sh` writes the file; exit 1 when a package has no licence.

## Business rules
1. Every crate in `Cargo.lock` appears in the output.
2. A package without a licence fails the script.

## Security
- No triggers. It reads lock files and writes a text file.

## Implementation notes
1. Verify `cargo deny list` output format on the current lock before parsing.
2. Do not add a tool that is not in spec/24; if one is needed, record a decision first.

## Acceptance criteria
- AC1. Given Cargo.lock, when the generator runs, then every crate name appears in the output → `test_third_party::test_every_locked_crate_listed`
- AC2. Given a package with no licence, when generated, then the script exits 1 → `test_third_party::test_fails_without_licence`

## Tests to write
| File | Test names |
|---|---|
| `scripts/tests/test_third_party.py` | test_every_locked_crate_listed, test_fails_without_licence |

## Definition of done
- [ ] `scripts/tasks-index.sh --check` and `python3 -I scripts/tasks-lint.py` pass
- [ ] Each test named above passes; the script or docs change is reviewed by a maintainer
- [ ] No hard-coded business value (POL-02); decisions recorded in `tasks/DECISIONS.md`
- [ ] Commit `docs(ops): generate third-party licences and run the reuse check at release [T-710e]`, signed off

## Report
_Filled in by the implementer (WORKFLOW.md §3 Step 12)._

## Questions
_Filled in by the implementer if a decision is needed; record it as a TD (WORKFLOW.md §4)._

## Review
_Filled in by the reviewer if changes are requested (WORKFLOW.md §5)._
