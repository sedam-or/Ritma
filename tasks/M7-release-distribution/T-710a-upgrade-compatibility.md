---
id: T-710a
title: Test upgrades from the previous release on PostgreSQL 16
milestone: M7
status: todo
depends_on: [T-704]
spec_refs: [PRD-04, OPS-04]
estimated_size: M
---

## Goal
Every release is proven to upgrade a database written by the previous release, with no lost rows and the same policy versions, before its tag is created.

## Context
PRD-04 (spec/30): a dump of the previous tagged release is restored, the current binary runs the forward-only migrations (OPS-04), and row counts and policy versions are compared with the dump's manifest. Migrations are forward-only (RELEASING.md).

## Scope
### In
- `scripts/upgrade-test.sh <previous-tag> <dump-file>`: restores the dump into a disposable PostgreSQL 16 database, runs the current migrations, and compares row counts and policy versions with `manifest.json` in the dump.
- A CI job `upgrade` in `ci.yml` that runs it on the release candidate.
- A synthetic fixture (no real personal data) for the first upgrade test.
### Out
- Other PostgreSQL major versions (later matrix task).
- Desktop app upgrades (M8, v1.1).

## Files
| Path | Action | Purpose |
|---|---|---|
| `scripts/upgrade-test.sh` | create | Restore, migrate, verify. |
| `scripts/tests/test_upgrade_script.py` | create | Unit tests of the verification step. |
| `tests/upgrade/README.md` | create | How the fixture is produced. |
| `.github/workflows/ci.yml` | modify | Add the `upgrade` job. |

## Interfaces
`scripts/upgrade-test.sh <previous-tag> <dump-file>`: exit 0 only if every table count and every policy version matches the manifest. Exit 2 on usage errors.

## Business rules
1. Pass means row counts and policy versions match exactly; a mismatch fails the job.
2. A policy schema bump requires a migration test (RELEASING.md).

## Security
No triggers.

## Implementation notes
1. Write the script and its tests first (the failing count-mismatch test).
2. Use the existing PostgreSQL service pattern from the `rust` job; create `ritma_upgrade_<tag>`, drop it afterwards.
3. Never reuse a database between runs.

## Acceptance criteria
- AC1. Given a dump whose counts match the manifest, when upgraded, then the script exits 0 → `test_upgrade_script::test_passes_on_matching_counts`
- AC2. Given a count mismatch, when verified, then the script exits 1 and names the table → `test_upgrade_script::test_fails_on_count_mismatch`

## Tests to write
| File | Test names |
|---|---|
| `scripts/tests/test_upgrade_script.py` | test_passes_on_matching_counts, test_fails_on_count_mismatch, test_rejects_missing_dump |

## Definition of done
- [ ] `scripts/tasks-index.sh --check` and `python3 -I scripts/tasks-lint.py` pass
- [ ] Each test named above passes; the script or docs change is reviewed by a maintainer
- [ ] No hard-coded business value (POL-02); decisions recorded in `tasks/DECISIONS.md`
- [ ] Commit `docs(ops): test upgrades from the previous release on postgresql 16 [T-710a]`, signed off

## Report
_Filled in by the implementer (WORKFLOW.md §3 Step 12)._

## Questions
_Filled in by the implementer if a decision is needed; record it as a TD (WORKFLOW.md §4)._

## Review
_Filled in by the reviewer if changes are requested (WORKFLOW.md §5)._
