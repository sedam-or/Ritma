# Technical decisions — index

Generated from `tasks/DECISIONS.md` by `scripts/decisions-index.py`. Do not edit by hand; CI runs `--check`.

Total: 165 decisions.

| ID | Decision | Ratify |
|---|---|---|
| TD-046 | Preset patch source boundary | Yes |
| TD-047 | Resolver authorization contract | Yes |
| TD-048 | Resolver migration slot | Yes |
| TD-049 | Deny-compatible JSON Schema dependency | Yes |
| TD-050 | Typed policy audit payload | Yes |
| TD-051 | Missing-row effective timestamp | Yes |
| TD-052 | Policy cache capacity | Yes |
| TD-053 | Preserve AC1's qualified test name | Yes |
| TD-054 | Split the outbox task under CQ-101 | Yes |
| TD-055 | Preserve the shipped event type shape | Yes |
| TD-056 | Complete events with no subscribers | Yes |
| TD-057 | Preserve aggregate order across dispatchers | Yes |
| TD-058 | Classify event-handler failures per ERR-52 | Yes |
| TD-059 | Use the approved cron parser | Yes |
| TD-060 | Use typed job failures | Yes |
| TD-061 | Bound the worker claim batch | Yes |
| TD-062 | Keep tenant enumeration behind the caller | Yes |
| TD-063 | Split oversized task contracts | Yes |
| TD-064 | Record tenant job dead letters | Yes |
| TD-065 | Preserve the platform-job boundary | Yes |
| TD-066 | Add spec-backed security sections | Yes |
| TD-067 | Complete required task headings | Yes |
| TD-068 | Preserve the mapped refresh-session schema | Yes |
| TD-069 | Keep materialisation core with its earlier provider | Yes |
| TD-070 | Keep the tunnel broker dependency pointed forward | Yes |
| TD-071 | Include the ERD-05 `reason` key in audit anonymization | Yes |
| TD-072 | Preserve the checkpoint timestamp default | Yes |
| TD-073 | Inject PolicyStore into job context | Yes |
| TD-074 | Keep the first-party audit derive license exception package-scoped | Yes |
| TD-075 | Split the oversized audit task | Yes |
| TD-076 | Replace the stale SBD-21 reference | Yes |
| TD-077 | Declare audit schema test dependencies | Yes |
| TD-078 | Use isolated unittest discovery for the task gate tests | Yes |
| TD-079 | Preserve the policy validation error path | Yes |
| TD-080 | Order audit policy schema fields | Yes |
| TD-081 | Reject unmapped anonymous audit actors | Yes |
| TD-082 | Include task-owned SQLx offline metadata | Yes |
| TD-083 | Keep audit SQL in the repository adapter | Yes |
| TD-084 | Verify both named audit sequence endpoints | Yes |
| TD-085 | Keep audit-chain SQL in the repository adapter | Yes |
| TD-086 | Fail closed on an out-of-order sealed chain | Yes |
| TD-087 | Keep audit-job queries in the repository adapter | Yes |
| TD-088 | Use an idempotent audit-chain attention item | Yes |
| TD-089 | Use the core-owned attention port | Yes |
| TD-090 | Schedule recent audit verification after retention | Yes |
| TD-091 | Preserve audit context for core actions | Yes |
| TD-092 | Verify the audit sink's transaction and tenant boundary | Yes |
| TD-093 | Serialize audit writers with chain sealing | Yes |
| TD-094 | Synchronize the writer/sealer regression test | Yes |
| TD-095 | Align the identity split tracker with its children | Yes |
| TD-096 | Split T-110a into ordered implementation parts | Yes |
| TD-097 | Validate identity policy values at construction and deserialization | Yes |
| TD-098 | Split identity schema and policy work by concern | Yes |
| TD-099 | Align T-110a1 security triggers with its scope | Yes |
| TD-100 | Keep the identity task split and index consistent | Yes |
| TD-101 | Match identity policy security triggers to the scope | Yes |
| TD-102 | Record the identity policy spec references | Yes |
| TD-103 | Match identity module security triggers to scope | Yes |
| TD-104 | Record the identity module policy spec references | Yes |
| TD-105 | Clarify policy-schema DoD boundary | Yes |
| TD-106 | Match password/JWT security triggers to scope | Yes |
| TD-107 | Register `password_weak` as an Identity application code | Yes |
| TD-108 | Keep access tokens and password hashes in redacted wrappers | Yes |
| TD-109 | Preserve AC1's common-password rejection | Yes |
| TD-110 | Resume T-110a3 under the delegated check policy | Yes |
| TD-111 | Use the spec-mapped minimal TOTP verifier features | Yes |
| TD-112 | Keep login identity lookup and anonymous audit attribution within mapped contracts | Yes |
| TD-113 | Represent absent country context for context-independent login fields | Yes |
| TD-114 | Inject shared login runtime dependencies at API composition | Yes |
| TD-116 | Lazily initialize the shared login policy store | Yes |
| TD-115 | Complete the login `PersonMe` contract without adding permission grants | Yes |
| TD-117 | Split Actor authentication and cookie role switching | Yes |
| TD-118 | Register the already-specified identity write permission | Yes |
| TD-119 | Require Origin on role-switch requests | Yes |
| TD-120 | Keep split-task dependencies in index order | Yes |
| TD-121 | Keep extracted bearer tokens in secret-safe storage | Yes |
| TD-122 | Keep split Actor task dependencies in index order | Yes |
| TD-123 | Keep the Identity surface test aligned with the shipped login endpoint | Yes |
| TD-124 | Use CACHE-14 auth cache keys | Yes |
| TD-125 | Register the role-switch route in the Identity module | Yes |
| TD-126 | Use the specified session controls for cookie role switching | Yes |
| TD-127 | Split role switching to stay within CQ-101 | Yes |
| TD-128 | Check role-switch CSRF before tenant resolution | Yes |
| TD-129 | Use a documentation commit for aggregate-only T-110a | Yes |
| TD-130 | Reuse fixed build and database resources for T-110a verification | Yes |
| TD-131 | Require Origin on refresh requests | Yes |
| TD-132 | Correct T-110b's implementation file list | Yes |
| TD-133 | Use the specified refresh success response | Yes |
| TD-134 | Split the T-111 identity workflows | Yes |
| TD-135 | Carry invite token to the Notify consumer | Yes |
| TD-136 | Keep split task ids before the aggregate | Yes |
| TD-137 | Keep invite tokens out of the event payload | Yes |
| TD-138 | Expose tenant display name to the invite preview through Core | Yes. Migration `0012` and the new function are not in `spec/06` DDL; `scripts/erd.py --migrations` must be checked and, if it flags the function, the TD-001 allow-list entry applies with a reference to TD-138 |
| TD-139 | Retire the `blocked` task status | Yes |
| TD-140 | Activate invited membership through Core during acceptance | Yes |
| TD-141 | Preserve TOTP for an already enrolled invitee | Yes |
| TD-142 | Attribute invite acceptance to the invitee | Yes |
| TD-143 | Keep reset tokens out of event payloads | Yes |
| TD-144 | Use T-110e task-scoped database and SQLx metadata | Yes |
| TD-145 | Use the system actor for password reset audit | Yes |
| TD-146 | Advance the access-token password version on reset | Yes |
| TD-147 | Version the self-profile without changing the login DTO | Yes |
| TD-148 | Accept only canonical E.164 in self-profile updates | Yes |
| TD-149 | Scope self-service to the authenticated person | Yes |
| TD-150 | Do not invent an outbox event for self-profile changes | Yes |
| TD-151 | Use the operator-approved T-110d database for T-110f | Yes |
| TD-152 | Track the missing shared API rate limiter | Yes; maintainer review is required before deployment |
| TD-153 | Name the Core-owned profile audit action | Yes |
| TD-154 | Record the T-110f task-size deviation | Yes; review whether equivalent future task contracts should be split before implementation |
| TD-155 | Preserve the declared TOTP wire contract | Yes |
| TD-156 | Limit the TOTP setup secret disclosure | Yes |
| TD-157 | Reuse the operator-approved task database name for T-110g | Yes |
| TD-158 | Register the spec-defined TOTP transition error | Yes |
| TD-159 | Audit the pending TOTP enrollment transition | Yes |
| TD-160 | Record T-110g review-size deviation | Yes; review the task sizing and whether the backlog should split comparable security workflows before implementation |
| TD-161 | Place the TOTP login acceptance test with the auth adapter | Yes |
| TD-162 | Reuse configured Argon2id parameters for recovery-code verification | Yes |
| TD-163 | Align the T-111 aggregate with IDs-only identity events | Yes |
| TD-164 | Defer aggregate OpenAPI generation to T-114 | Yes |
| TD-165 | Align T-112 split status labels with task frontmatter | Yes |
| TD-166 | Add implementation paths omitted from T-112a Files | Yes |
| TD-167 | Do not invent outbox events for people role and guardian-link changes | Yes |
| TD-168 | Use a tenant-scoped read-only transaction for people reads | Yes |
| TD-169 | Disclose T-112a review-size overage | Yes; review whether the original task split and missing Files entries should be corrected for future work |
| TD-170 | Defer client error translations until the web i18n task | Yes; confirm that the T-306 translation keys close the ERR-22 follow-up |
| TD-171 | Move people invariants into the pure domain layer | Yes |
| TD-172 | Record the T-112a E2E disk-rule overrun | Yes |
| TD-173 | Add implementation paths omitted from T-112b Files | Yes |
| TD-174 | Timestamp the deduplicated PII audit with the injected Clock | Yes |
| TD-175 | Disclose T-112b review-size overage | Yes; review the task split and omitted Files entries |
| TD-176 | Do not invent person outbox event contracts | Yes |
| TD-177 | Defer API artifact generation to T-114 | Yes |
| TD-178 | Align the T-113 split tracker with child frontmatter | Yes |
| TD-179 | Run task-linter tests under Python isolation | Yes |
| TD-180 | Add the omitted T-113a implementation paths and scoped tenant writer | Yes |
| TD-181 | Use D-24's canonical policy effective-time bound | Yes |
| TD-182 | Validate bounded Core policy values at their serde boundary | Yes |
| TD-183 | Defer settings rate limiting to the shared HTTP layer | Yes; security review must confirm the shared limiter is applied before external exposure |
| TD-184 | Retain the assigned T-113a implementation and tests above CQ-101 size guidance | Yes; the maintainer should decide whether to require a follow-up split/refactor |
| TD-185 | Run T-109 audit integration tests on its fixed task database | Yes |
| TD-186 | Preserve the existing audit derive license decision ID | Yes |
| TD-187 | Add the omitted T-113b implementation paths | Yes |
| TD-188 | Bound T-113b audit pagination with technical limits | Yes |
| TD-189 | Use a direct utility for streamed CSV response chunks | Yes |
| TD-190 | Resolve the public tenant name through the Core port | Yes |
| TD-191 | Handle the spec-defined blank preset without a loader | Yes |
| TD-192 | Defer OpenAPI generation to its dependent task | Yes |
| TD-193 | Preserve T-113b's assigned scope above the review-size guide | Yes; the maintainer should decide whether a follow-up split or refactor is needed before merge |
| TD-194 | Add the implementation paths omitted from T-114's Files table | Yes |
| TD-195 | Expose each compiled crate's ERR-21 registry | Yes |
| TD-196 | Complete the shared Problem contract from ERR-30 | Yes |
| TD-197 | Preserve T-114's required contract above the review-size guide | Yes; the maintainer should decide whether a follow-up split or refactor is needed before merge |
| TD-198 | Split the cross-cutting T-115 contract under CQ-101 | Yes |
| TD-199 | Keep the idempotency lookup in the business transaction | Yes |
| TD-200 | Use the idempotency-specific mismatch status | Yes |
| TD-201 | Keep idempotency table data class at S1 | Yes |
| TD-202 | Split the idempotency implementation and acceptance suite | Yes |
| TD-203 | Use the schema-defined idempotency request hash | Yes |
| TD-204 | Gate the idempotency aggregate on its child tasks | Yes |
| TD-205 | Scope v1.0 to M0–M7; defer M8 and M9 to v1.1 | Yes. Changing the release scope is a maintainer decision; this record reflects the maintainer's approval |
| TD-206 | Three long-lived branches and no maintenance lines (governance wave A) | Yes |
| TD-207 | Add the missing public-project files (governance wave B) | Yes |
| TD-208 | Production-readiness requirements and the two-approval exception (governance wave C) | Yes |
| TD-209 | Quarantine the non-hermetic Rust tests in the CI nextest profile | Yes |
| TD-210 | Accept RUSTSEC-2026-0440 for a dev-only test dependency | Yes |
