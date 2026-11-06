---
id: T-710c
title: Publish the vulnerability-response SLA, key-management runbook and external-review plan
milestone: M7
status: todo
depends_on: [T-705, T-704]
spec_refs: [PRD-05, PRD-06, PRD-07]
estimated_size: S
---

## Goal
The security process is written down: how fast advisories are handled, how keys rotate, and whether and how an external review happens before v1.0.

## Context
PRD-05 to PRD-07 (spec/30). Response times are proposed values until the maintainer confirms them. The external review decision is open.

## Scope
### In
- A 'Response times' section in SECURITY.md with the confirmed values (or the proposed ones marked as proposed).
- `docs/operations/key-management.md`: rotation and break-glass steps that link the T-705 commands.
- `docs/operations/external-review.md`: scope, budget and the decision record.
### Out
- Implementing key rotation (T-705).
- Performing the review itself.

## Files
| Path | Action | Purpose |
|---|---|---|
| `SECURITY.md` | modify | Response times section. |
| `docs/operations/key-management.md` | create | Runbook. |
| `docs/operations/external-review.md` | create | Scope and decision record. |

## Interfaces
No code interface.

## Business rules
1. Response times are configuration of the process, recorded in SECURITY.md, not in code.
2. An open external-review decision blocks the v1.0 tag until it is recorded in tasks/DECISIONS.md.

## Security
- No triggers. Documentation only; it defines the process for SBD-80 security reviews.

## Implementation notes
1. Do not invent values: write them as proposed until the maintainer confirms.
2. Link the T-705 rotation command, do not copy it.

## Acceptance criteria
- AC1. Given SECURITY.md, when read, then it contains a response-times section → `test_security_docs::test_response_times_present`

## Tests to write
| File | Test names |
|---|---|
| `scripts/tests/test_security_docs.py` | test_response_times_present |

## Definition of done
- [ ] `scripts/tasks-index.sh --check` and `python3 -I scripts/tasks-lint.py` pass
- [ ] Each test named above passes; the script or docs change is reviewed by a maintainer
- [ ] No hard-coded business value (POL-02); decisions recorded in `tasks/DECISIONS.md`
- [ ] Commit `docs(ops): publish the vulnerability-response sla, key-management runbook and external-review plan [T-710c]`, signed off

## Report
_Filled in by the implementer (WORKFLOW.md §3 Step 12)._

## Questions
_Filled in by the implementer if a decision is needed; record it as a TD (WORKFLOW.md §4)._

## Review
_Filled in by the reviewer if changes are requested (WORKFLOW.md §5)._
