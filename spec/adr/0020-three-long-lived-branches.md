# ADR-0020: Three long-lived branches (dev, staging, main)

- Status: Accepted (2026-11-01). Supersedes ADR-0015.

## Context
Ritma's implementation is done task by task by agents (WORKFLOW.md) with a single integration line, while release candidates need a branch that can be frozen and tested. ADR-0015 chose trunk-based development with one long-lived branch and short-lived task branches. In practice that produced many branches, and the maintainer asked for exactly three: `main`, `staging` and `dev`.

Options considered:
1. **Trunk-based, short-lived branches (ADR-0015).** Rejected: it produces one branch per task, and there is no frozen release candidate.
2. **GitFlow (`develop`, `release/*`, `hotfix/*`).** Rejected: more long-lived branches and maintenance lines than a small project needs.
3. **Two branches (`dev`, `main`).** Rejected: a release candidate cannot be frozen while integration continues.
4. **Three branches (`dev`, `staging`, `main`).** Chosen.

## Decision
- Exactly three long-lived branches. `dev` integrates, `staging` is the release candidate, `main` is released code.
- Work is committed to `dev`. Promotion is a pull request `dev` → `staging` and then `staging` → `main`, both merged with a merge commit so the branches stay reconcilable.
- No other branch is pushed to `origin`. Bot branches are the only exception.
- No maintenance branches. Only the latest minor of the current major receives fixes, released from `main`.
- `ci-ok` is the required check on every branch, and `promotion-source` requires PRs into `main` to come from `staging`.

## Consequences
- Implementers push directly to `dev`, so CI must run on push, and a red `dev` stops new work.
- Merge commits on promotion keep history traceable, at the cost of a less linear `main`.
- Without maintenance branches, users on an older minor must upgrade. The upgrade notes in `CHANGELOG.md` are required in each release.
- BRANCHING.md, RELEASING.md, SECURITY.md, the CI workflows and the Dependabot configuration are updated in the same change (see TD-141).
