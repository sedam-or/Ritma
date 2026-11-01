---
id: T-710b
title: Publish SLOs, incident response and the restore-drill log
milestone: M7
status: draft
depends_on: [T-709, T-701]
spec_refs: [PRD-01, PRD-02, PRD-03]
estimated_size: S
---

## Goal
Operators have written objectives, an incident runbook, a postmortem template and a restore-drill log, all linked from the docs index.

## Context
PRD-01 to PRD-03 (spec/30). Objectives are per deployment and configured, never hard-coded (POL-02). Proposed defaults: 99.5 % monthly availability for the hosted demo; severities S1–S4.

## Scope
### In
- `docs/operations/slo.md`: indicators (from T-709 metrics) and objectives, with the proposed defaults marked.
- `docs/operations/incident-response.md`: severities, roles, first hour, communication.
- `docs/operations/postmortem-template.md`: blameless template.
- `docs/operations/restore-drill.md`: a log table (date, backup id, machine, result, time to restore).
- Links from `docs/README.md`.
### Out
- Monitoring implementation (T-709).
- Backup implementation (T-701).

## Files
| Path | Action | Purpose |
|---|---|---|
| `docs/operations/slo.md` | create | Objectives and indicators. |
| `docs/operations/incident-response.md` | create | Runbook. |
| `docs/operations/postmortem-template.md` | create | Template. |
| `docs/operations/restore-drill.md` | create | Drill log. |
| `docs/README.md` | modify | Link the operator pages. |

## Interfaces
No code interface. Each page names the metric or command it depends on (for example `/healthz`, the backup CLI from T-701).

## Business rules
1. Objectives are configuration of each deployment, not literals in code.
2. The restore drill log is updated before each release tag (RELEASING.md checklist).

## Security
No triggers.

## Implementation notes
1. Write the pages from the templates in this task.
2. Check every linked command exists in the repository before linking it.

## Acceptance criteria
- AC1. Given the docs index, when a reader follows the operator section, then every operations page exists → `test_operations_docs::test_pages_exist_and_are_linked`

## Tests to write
| File | Test names |
|---|---|
| `scripts/tests/test_operations_docs.py` | test_pages_exist_and_are_linked |

## Definition of done
- [ ] `scripts/tasks-index.sh --check` and `python3 -I scripts/tasks-lint.py` pass
- [ ] Each test named above passes; the script or docs change is reviewed by a maintainer
- [ ] No hard-coded business value (POL-02); decisions recorded in `tasks/DECISIONS.md`
- [ ] Commit `docs(ops): publish slos, incident response and the restore-drill log [T-710b]`, signed off

## Report
_Filled in by the implementer (WORKFLOW.md §3 Step 12)._

## Questions
_Filled in by the implementer if a decision is needed; record it as a TD (WORKFLOW.md §4)._

## Review
_Filled in by the reviewer if changes are requested (WORKFLOW.md §5)._
