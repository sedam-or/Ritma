# Releasing

## Versioning

- **SemVer** for the app: `MAJOR.MINOR.PATCH`. `/api/v1` is stable within major 1 (API-13).
- Each module's **policy schema version** (`SCHEMA_VERSION`) is versioned separately. A bump requires `migrate_policy` and a changelog entry under "Policy changes".
- DB migrations are forward-only (OPS-04).

## Process

1. Merge to `main` with commits that follow `COMMIT_CONVENTION.md` (types map to SemVer bumps and changelog sections, §2). **release-please** keeps a release PR with the changelog.
2. Merge the release PR, which creates tag `vX.Y.Z`.
3. `release.yml` (DST-05) builds the binaries, the multi-arch image, the desktop installers, the SBOMs, the checksums and the cosign signatures, then publishes the GitHub Release and `latest.json`.
4. A bot opens PRs bumping the image tag in `deploy/*` templates.
5. Announce: GitHub Discussions (Announcements), Discord/Telegram, and the update notice in the app.

## Hotfixes and older minors

- **Hotfix:** the fix is committed to `dev`, promoted `dev` → `staging` → `main` as a patch release, and released through release-please (BRANCHING.md §4).
- **Older minors** receive no fixes. Only the latest minor of the current major is supported (BRANCHING.md §5).
- **Security fixes** follow SECURITY.md: they are prepared in a private advisory fork and published with the release.

## Checklist (release PR)

- [ ] CI green, including e2e and the backup round-trip (TST-13)
- [ ] Promoted `dev` → `staging` (release candidate tested) and `staging` → `main` (BRANCHING.md §3)
- [ ] `main` merged back into `dev` after the release (BRANCHING.md §3, step 5)
- [ ] Policy schema bumps listed, with migration tests
- [ ] Upgrade notes (manual steps should be none)
- [ ] Docs updated for user-visible changes (EN + ID)
- [ ] Translations ≥ 95% for `id`
