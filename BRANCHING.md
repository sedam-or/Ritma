# Branching strategy

Ritma has **exactly three long-lived branches: `dev`, `staging` and `main`** ([ADR-0020](spec/adr/0020-three-long-lived-branches.md), which supersedes [ADR-0015](spec/adr/0015-trunk-based-branching.md)). This file is the single source of truth for branch names, what each branch is for, promotion, hotfixes and protection. Other documents link here and do not repeat the rules.

## 1. The three branches

```
dev ──promote (PR, merge commit)──► staging ──promote (PR, merge commit)──► main ──release-please──► tag vX.Y.Z
```

| Branch | Purpose | Who writes it | Gate | Tags |
|---|---|---|---|---|
| `dev` | Integration. Every finished task lands here. | Implementers (WORKFLOW.md) and maintainers push commits directly. External contributors open PRs against `dev`. | `ci.yml` on push; `ci-ok` must be green. A red `dev` stops new task work until it is fixed. | none |
| `staging` | Release candidate. Frozen while a candidate is tested. | Only promotion PRs from `dev`. Fixes go to `dev` and are promoted again. | `ci.yml` on the PR; `ci-ok`. | `vX.Y.Z-rc.N` (maintainer, by hand) |
| `main` | Released code, the line users install. | Only promotion PRs from `staging`, and the release-please release PR. | `ci.yml`; `ci-ok`, which includes `promotion-source` (the head branch must be `staging`). | `vX.Y.Z` (release-please) |

## 2. Rules

1. **Exactly three branches on `origin`: `dev`, `staging`, `main`.** No other branch is pushed to `origin`, ever. That includes `task/`, `feat/`, `fix/`, `docs/`, `ci/`, `build/`, `hotfix/`, `release/`, `backport/`, `milestone/`, `run/` and `archive/` names. A task or a fix is a commit on `dev`, not a branch. Local branches are fine for scratch work and must never be pushed.
2. **Bot branches are the only exception.** Dependabot's `dependabot/**` and the release-please release PR branch live until their PR is merged, and are deleted automatically.
3. **Forks** are used for external experiments and for security work before an advisory is published (SECURITY.md).
4. **Never force-push and never delete** `dev`, `staging` or `main`. Rulesets enforce this (§6).
5. **Every commit is signed off** (`git commit -s`, DCO). PR titles follow COMMIT_CONVENTION §9.
6. **Keep `dev` green.** Commit a task when its checks pass. If a push breaks `ci-ok`, fix it on `dev` before the next task. Never promote a red `dev`.

## 3. Promotion and release

1. **`dev` → `staging`:** open a PR with the title `release: promote dev to staging (vX.Y.Z-rc.N)`. Merge it with **Create a merge commit**, not squash, so `staging` and `main` stay reconcilable.
2. **Test the candidate on `staging`** with the release checklist in RELEASING.md. A defect is fixed on `dev` and promoted again. Nothing is committed to `staging` directly.
3. **`staging` → `main`:** open a PR with the title `release: promote staging to main (vX.Y.Z)`, merged with a merge commit. The `promotion-source` check fails unless its head branch is `staging`.
4. **Release:** release-please opens the release PR on `main`. Merging it tags `vX.Y.Z` and runs `release.yml` (RELEASING.md).
5. **Sync back:** after the release, a maintainer merges `main` into `dev` (`git switch dev && git merge --no-ff origin/main && git push origin dev`), so `dev` has the changelog and version bump. Then `staging` is promoted again at the next candidate.

## 4. Hotfixes

1. Commit the fix on `dev` with its task or issue reference, and make CI green.
2. Promote `dev` → `staging` → `main` as in §3. One maintainer approval on the promotion PR is enough. No CI check may be skipped. Then merge the release-please PR straight away to tag the patch.
3. **Security issues:** prepare the fix in a private fork from a GitHub Security Advisory (SECURITY.md). Publish it through the advisory, then promote it as above.

## 5. Supported versions and maintenance

- **Only the latest minor of the current major receives fixes.** A fix ships as the next patch release from `main`. Older minors get no fixes; the upgrade notes in `CHANGELOG.md` say how to move.
- **There are no maintenance branches.** `release/*` and `backport/*` branches do not exist. A fix goes to `dev` first, is promoted through `staging`, and is released from `main`.

## 6. Protection (a maintainer applies these in GitHub, once)

| Setting | `main` | `staging` | `dev` |
|---|---|---|---|
| Pull request required | ✅ 1 approval, code owners | ✅ 1 approval | ❌ implementers push directly; external contributors use PRs |
| Required status check | `ci-ok` (includes `promotion-source`) | `ci-ok` | `ci-ok`, run on push |
| Block force push and deletion | ✅ | ✅ | ✅ |
| Allowed merge methods | merge commit | merge commit | squash (external PRs) |
| Dismiss stale approvals | ✅ | ✅ | n/a |
| Require conversation resolution | ✅ | ✅ | n/a |
| Bypass | nobody; the release-please app for its own PR only | nobody | nobody |

Repository settings: automatically delete head branches, private vulnerability reporting on, Dependabot security updates on.

```sh
# Run once by a repository admin. Set REPO to owner/name first.
: "${REPO:?set REPO=owner/name}"
gh api -X POST "repos/$REPO/rulesets" --input - <<'JSON'
{
  "name": "protect-main",
  "target": "branch",
  "enforcement": "active",
  "conditions": { "ref_name": { "include": ["refs/heads/main"], "exclude": [] } },
  "rules": [
    { "type": "deletion" },
    { "type": "non_fast_forward" },
    { "type": "pull_request", "parameters": {
        "required_approving_review_count": 1,
        "dismiss_stale_reviews_on_push": true,
        "require_code_owner_review": true,
        "required_review_thread_resolution": true,
        "allowed_merge_methods": ["merge"] } },
    { "type": "required_status_checks", "parameters": {
        "strict_required_status_checks_policy": true,
        "required_status_checks": [ { "context": "ci-ok" } ] } }
  ]
}
JSON
```

Create the same ruleset for `staging` and `dev` with the settings in the table; `dev` omits the pull request rule. Tags `v*` get a tag ruleset that blocks updates and deletions.

## 7. CI on branches

- `ci.yml` runs on every pull request, whatever its base, and on pushes to `dev`, `staging` and `main`. `ci-ok` is the only required check, so adding a job never changes the rulesets.
- `promotion-source` runs for pull requests into `main` and fails unless the head branch is `staging`.
- `release-please.yml` runs on pushes to `main` only.
- Workflows never run with write permissions on pull requests from forks (`pull_request`, not `pull_request_target`; spec 24 OSS §6).

## 8. Quick reference

```sh
git switch dev && git pull --ff-only
git commit -s -m "feat(scheduling): add calendar holds [T-203]"
git push origin dev                                   # CI runs on the push
gh pr create --base staging --head dev \
  --title "release: promote dev to staging (v0.4.0-rc.1)"
```
