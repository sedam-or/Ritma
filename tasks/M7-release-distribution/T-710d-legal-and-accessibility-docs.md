---
id: T-710d
title: Add privacy notice and DPA templates and an accessibility statement
milestone: M7
status: todo
depends_on: [T-710]
spec_refs: [PRD-08, PRD-09]
estimated_size: S
---

## Goal
School operators get templates for a privacy notice and a data-processing agreement, and the project publishes an accessibility statement that lists its gaps.

## Context
PRD-08 and PRD-09 (spec/30). Templates are not legal advice. The statement must list known gaps, not claim full conformance.

## Scope
### In
- `docs/legal/privacy-notice.md` and `docs/legal/dpa-template.md`, marked as templates, written in Bahasa Indonesia first (T-710).
- `docs/accessibility.md`: WCAG 2.2 AA status, known gaps and the feedback route.
### Out
- Legal review (maintainer or a lawyer).
- Fixing accessibility gaps (T-711 and the UI tasks).

## Files
| Path | Action | Purpose |
|---|---|---|
| `docs/legal/privacy-notice.md` | create | Template. |
| `docs/legal/dpa-template.md` | create | Template. |
| `docs/accessibility.md` | create | Statement with gaps. |

## Interfaces
No code interface.

## Business rules
1. Each template states that it needs review by the operator's own counsel.
2. The statement lists every known gap found by the T-711 review.

## Security
- No triggers. Documentation only.

## Implementation notes
1. Keep each template short and structured by headings.
2. Take the gap list from T-711's report.

## Acceptance criteria
- AC1. Given the legal folder, when checked, then both templates contain the review notice → `test_legal_docs::test_templates_carry_review_notice`

## Tests to write
| File | Test names |
|---|---|
| `scripts/tests/test_legal_docs.py` | test_templates_carry_review_notice |

## Definition of done
- [ ] `scripts/tasks-index.sh --check` and `python3 -I scripts/tasks-lint.py` pass
- [ ] Each test named above passes; the script or docs change is reviewed by a maintainer
- [ ] No hard-coded business value (POL-02); decisions recorded in `tasks/DECISIONS.md`
- [ ] Commit `docs(ops): add privacy notice and dpa templates and an accessibility statement [T-710d]`, signed off

## Report
_Filled in by the implementer (WORKFLOW.md §3 Step 12)._

## Questions
_Filled in by the implementer if a decision is needed; record it as a TD (WORKFLOW.md §4)._

## Review
_Filled in by the reviewer if changes are requested (WORKFLOW.md §5)._
