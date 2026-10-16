//! 🔌️ The HubDirectory port every persistence backend implements.

use super::*;
use semio_hub_directory_contract::model::{AdminDirectoryOverviewCounts, AdminEffectCommitV1, AdminOperationAuditRecord, AdminOperationEffectReceiptV1, AdminSpaceSummaryRecord, AgentDelegationRecord, AgentDelegationRow, AuthAuditRecord, AuthSessionIssue, AuthSessionRecord, CheckpointPublicationClaimV1, CheckpointPublicationCompletionV1, CredentialAuditFactV1, DirectoryCommandClaimV1, DirectoryCommandReceiptCompletion, DirectoryCommandReceiptRecord, DirectoryPrincipalCeilingV1, DirectoryPrincipalV1, InviteRecord, InviteRedemptionCommit, IssuedAgentDelegation, IssuedAuthSession, IssuedInvite, IssuedShareToken, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1, NewCheckpointPublicationClaimV1, NewDirectoryCommandReceipt, RevokedAuthSession, ShareTokenRecord, SocketSessionBindingStatus, SocketShareBindingStatus, SpaceAdministrationInviteRow, SpaceAdministrationMemberRow, SpaceRecord, SpaceRole, SyncSessionRecord, UserRecord};
use directory::os_directory::DocumentScope;
use semio_hub_directory_contract::{InviteCapability, InviteRedemptionScopeHintV1, NewDirectoryEvent, SessionCapability, ShareCapability, now_ms};

//#region 🔖️Trait
/// 🗄️ Backend-agnostic os-hub identity/tenancy directory. Implemented once per backend
/// (sqlite/postgres/neo4j); `HubState` holds an `Arc<HubDirectories>` (see `//#region 🔖️Dispatch`
/// below) so the directory backend is a deploy-time choice, not a compile-time one — independent of
/// `db::Database`'s own storage backend choice (see `bin.rs`, `OS_HUB_DIRECTORY_BACKEND` vs
/// `OS_HUB_STORAGE_BACKEND`).
pub trait HubDirectory: Send + Sync + 'static {
    //#region ShareTokens
    async fn issue_share_token_as(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedShareToken>;
    async fn issue_share_token_as_with_admin_effect(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedShareToken>;
    async fn issue_share_token(&self, scope: &DocumentScope, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        self.issue_share_token_as(scope, ttl_secs, None, correlation_id).await
    }
    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()>;
    async fn revoke_share_token_as_with_admin_effect(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()>;
    async fn revoke_share_token(&self, scope: &DocumentScope, share_id: &str, reason: &str, correlation_id: &str) -> DirectoryResult<()> {
        self.revoke_share_token_as(scope, share_id, reason, None, correlation_id).await
    }
    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool>;
    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>>;
    async fn socket_share_binding(&self, share_id: &str, selector: &str, scope: &DocumentScope, now_ms: i64) -> DirectoryResult<SocketShareBindingStatus>;
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord>;
    /// 🔎️ Single-user lookup by id — the `member.upserted`/`invite.redeemed` projections
    /// resolve `MemberView.email`/`display_name` through this, not through `get_user_by_email`.
    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>>;
    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts>;
    //#endregion

    //#region Spaces
    /// 🔎️ Single-space lookup by id — used by the hub handler to read `kind`/`visibility`
    /// (grant compilation, public-visibility fallback) without listing every space. Also `decide`'s
    /// (`//#region 🔖️Decider`) own "does this space exist" read.
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>>;
    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>>;
    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>>;
    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>>;
    /// 🏘️ Every space `user_id` may list — each public space and each space it is a member of —
    /// with its role there and the counts of [`HubDirectory::list_admin_space_summaries_page`], ordered by
    /// id, in ONE backend round trip. `GET /directory/spaces` used to fold the whole directory event log
    /// and then query every visible space's documents (mounting each document for its frontier) and
    /// sessions one by one: 25–58 s for a member of 82 spaces on hub 7800 (ticket 26/09/23 WG8).
    async fn list_visible_space_summaries(&self, user_id: Option<&str>) -> DirectoryResult<Vec<(AdminSpaceSummaryRecord, Option<SpaceRole>)>>;
    async fn list_admin_space_members_page(&self, space_id: &str, offset: usize, limit: usize) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>>;
    /// 🧑️‍🤝️‍🧑️ The current member roster — `decide` reads this to enforce the atelier/
    /// archive laws and to compute `archive-space`'s demote-every-author events.
    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>>;
    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>>;
    /// 🧢️ THE rule every hub authorization derives a session's space roles under: what `principal`'s session caps its
    /// account's membership roles at. A human session is uncapped. An agent session holds at most its delegation's
    /// audience ([`AgentAudience::space_role`]) and only in the delegation's one space — and nothing once the delegation
    /// is revoked, expired or not its account's — whatever the delegating account may do itself.
    async fn principal_ceiling(&self, principal: DirectoryPrincipalV1<'_>) -> DirectoryResult<DirectoryPrincipalCeilingV1> {
        if !principal.session_kind.is_agent() {
            return Ok(DirectoryPrincipalCeilingV1::Account);
        }
        Ok(match self.agent_delegation(principal.device_instance_id).await? {
            Some(delegation) if delegation.delegating_user_id == principal.user_id && delegation.revoked_at.is_none() && delegation.expires_at > now_ms() => {
                DirectoryPrincipalCeilingV1::Delegation { space_id: delegation.space_id, role: delegation.audience.space_role() }
            }
            _ => DirectoryPrincipalCeilingV1::Nothing,
        })
    }
    /// 🎚️ The role `principal` holds in `space_id`: its account's membership role under its
    /// [`Self::principal_ceiling`].
    async fn principal_role(&self, space_id: &str, principal: DirectoryPrincipalV1<'_>) -> DirectoryResult<Option<SpaceRole>> {
        let ceiling = self.principal_ceiling(principal).await?;
        Ok(ceiling.cap(space_id, self.get_role(space_id, principal.user_id).await?))
    }
    /// 🏛️ One keyset-ordered bounded administration member window (`user_id ASC`), projected to
    /// display columns inside the backend so no credential-bearing record is ever constructed.
    async fn list_space_administration_members_page(&self, space_id: &str, after_user_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationMemberRow>>;
    //#endregion

    //#region Documents
    async fn claim_artifact_creation(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1>;
    async fn read_artifact_creation(&self, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>>;
    async fn append_artifact_creation_fact(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1>;
    async fn append_document_genesis(&self, append: &DocumentGenesisAppendV1) -> DirectoryResult<DocumentGenesisCommitV1>;
    async fn artifact_creation_recovery_candidates(&self, now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>>;
    async fn artifact_creation_terminate_uncommitted(&self, intent: &ArtifactCreationIntentV1, now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1>;
    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>>;
    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>>;
    async fn list_document_descriptors_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<DocumentDescriptor>>;
    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>>;
    /// 🔐️ Internal authority read model; never serialized through directory wire DTOs.
    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>>;
    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>>;
    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>>;
    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64>;
    /// 🧵️ Oldest-to-newest lineage with a fixed caller bound; zero or max+1 is rejected.
    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>>;
    /// 🎫️ Appends one exact expiring private reachability reservation before CAS writes.
    async fn reserve_artifact_cas(&self, _plan: &ArtifactCasOwnershipPlanV1, _expires_at_ms: u64, _now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// ⚛️ Consumes one live exact reservation with the public/private checkpoint commit.
    async fn append_reserved_artifact_checkpoint(
        &self,
        _event: Option<&NewDirectoryEvent>,
        _checkpoint: &ArtifactCheckpoint,
        _reservation: &ArtifactCasReservation,
        _completion: Option<&CheckpointPublicationCompletionV1>,
        _now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🧹️ Reads a bounded page of private historical object candidates for sweeping.
    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🪪 Reads the durable private identity that binds directory epochs to the selected CAS.
    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]> {
        Err(DirectoryError::Backend("artifact CAS barrier identity is unavailable for this backend".into()))
    }
    /// 🧹️ Reads a bounded historical page through one immutable sweep generation.
    async fn artifact_cas_sweep_candidates(&self, _after_generation: u64, _through_generation: u64, _limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🔭 Reads exact current reachability for a mutation-free dry-run preview.
    async fn artifact_cas_delete_preview_protected(&self, _key: &ArtifactCasObjectKey, _observed_generation: u64, _now_ms: u64) -> DirectoryResult<bool> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🛡️ Atomically acquires a durable per-space deletion lease and rechecks reachability.
    async fn acquire_artifact_cas_delete_fence(&self, _key: &ArtifactCasObjectKey, _observed_generation: u64, _lease_token: [u8; 32], _now_ms: u64, _expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🛡️ Rechecks the exact live lease, epoch, and reachability after CAS epoch activation.
    async fn validate_artifact_cas_delete_fence(&self, _fence: &ArtifactCasDeleteFence, _now_ms: u64) -> DirectoryResult<bool> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 💓 Renews a deletion lease while the physical store operation remains in flight.
    async fn renew_artifact_cas_delete_fence(&self, _fence: &ArtifactCasDeleteFence, _now_ms: u64, _expires_at_ms: u64) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    /// 🔓 Releases exactly the opaque deletion lease owned by this fence.
    async fn release_artifact_cas_delete_fence(&self, _fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("artifact CAS ledger is unavailable for this backend".into()))
    }
    //#endregion

    //#region AuthSessions
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession>;
    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>>;
    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus>;
    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>>;
    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>>;
    async fn revoke_auth_sessions_for_user_with_admin_effect(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>>;
    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>>;
    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>>;
    /// 🧾️ Appends exactly one credential-lifecycle fact (`credential-sign-in`,
    /// `credential-changed`). Every sign-in attempt — admitted or refused — is journaled through
    /// this, so the authentication log is the record of what happened rather than a mutable
    /// counter beside it.
    async fn append_credential_audit(&self, _fact: &CredentialAuditFactV1) -> DirectoryResult<AuthAuditRecord> {
        Err(DirectoryError::Backend("credential audit is unavailable for this backend".into()))
    }
    /// 🔑️ Writes one user's password credential and appends the `credential-changed` fact in the
    /// same transaction, so the projection column and the log can never disagree.
    async fn set_password_credential(&self, _user_id: &str, _encoded_credential: &str, _actor_user_id: Option<&str>, _correlation_id: &str) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("password credentials are unavailable for this backend".into()))
    }
    /// 🤖️ Writes one agent delegation and appends its `agent-delegated` fact in the same
    /// transaction. The caller has already been authenticated as the delegating user and checked
    /// for authorship in the target space.
    async fn create_agent_delegation(&self, _issued: &IssuedAgentDelegation, _correlation_id: &str, _peer_class: &str) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("agent delegation is unavailable for this backend".into()))
    }
    /// 🤖️ Every delegation a user created in a space, newest first, revoked ones included so the
    /// delegation UI can show what was withdrawn. Never returns the selector or secret digest.
    async fn list_agent_delegations(&self, _space_id: &str, _delegating_user_id: &str, _limit: usize) -> DirectoryResult<Vec<AgentDelegationRow>> {
        Err(DirectoryError::Backend("agent delegation is unavailable for this backend".into()))
    }
    /// 🤖️ Revokes one delegation and appends its `agent-delegation-revoked` fact in the same
    /// transaction, then revokes every live agent session minted from it — the same
    /// `authorization_generation` bump an ordinary sign-out performs, so open socket grants and
    /// document plans die with it. `Ok(None)` means the delegation was not this user's to revoke.
    async fn revoke_agent_delegation(&self, _delegation_id: &str, _delegating_user_id: &str, _reason: &str, _correlation_id: &str, _now_ms: i64) -> DirectoryResult<Option<Vec<RevokedAuthSession>>> {
        Err(DirectoryError::Backend("agent delegation is unavailable for this backend".into()))
    }
    /// 🤖️ Loads one delegation by the selector half of a presented capability. The secret is never
    /// compared here — [`agent_session_preflight`] owns that law.
    async fn load_agent_delegation(&self, _selector: &str) -> DirectoryResult<Option<AgentDelegationRecord>> {
        Err(DirectoryError::Backend("agent delegation is unavailable for this backend".into()))
    }
    /// 🤖️ Loads one delegation by its id — what an agent session's device instance names.
    async fn agent_delegation(&self, _delegation_id: &str) -> DirectoryResult<Option<AgentDelegationRecord>> {
        Err(DirectoryError::Backend("agent delegation is unavailable for this backend".into()))
    }
    //#endregion

    //#region AdminOperations
    /// 🧾 Appends exactly one accepted or terminal administrator operation fact.
    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord>;
    /// 🔎 Reads the at-most-two facts for one bounded idempotency request key.
    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>>;
    /// 🎯 Reads the at-most-two facts for one server-issued operation identifier.
    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>>;
    /// 🧷️ Reads an exact private writer receipt without exposing capability material.
    async fn admin_operation_effect_receipt(&self, operation_id: &str, intent_digest: &str) -> DirectoryResult<Option<AdminOperationEffectReceiptV1>>;
    /// 📄 Reads one backend-ordered bounded operation-audit page.
    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>>;
    //#endregion

    //#region CommandReceipts
    /// 🔐️ Atomically claims one `(authenticated user id, request id)` idempotency key or reads the
    /// row already under it. An equal key with an unequal command digest is `Conflict` and never
    /// executes. This is the single serialization point the command writer holds across its whole
    /// decide/append sequence, so a duplicate request can never mint a second invitation.
    async fn claim_or_read_directory_command_receipt(&self, _claim: &NewDirectoryCommandReceipt) -> DirectoryResult<DirectoryCommandClaimV1> {
        Err(DirectoryError::Backend("directory command receipts are unavailable for this backend".into()))
    }
    /// 🧾️ Records the durable completion of one claimed key. The caller publishes only after this
    /// returns, so a delivered event is always covered by a durable receipt.
    async fn complete_directory_command_receipt(&self, _completion: &DirectoryCommandReceiptCompletion) -> DirectoryResult<DirectoryCommandReceiptRecord> {
        Err(DirectoryError::Backend("directory command receipts are unavailable for this backend".into()))
    }
    /// 🧹️ Releases one claimed key whose command failed before any durable event was appended.
    async fn release_directory_command_receipt(&self, _actor_user_id: &str, _request_id: &str, _command_sha256: &str) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("directory command receipts are unavailable for this backend".into()))
    }

    /// 🆔️ Atomically claims or reads one author/correlation/exact-command checkpoint publication.
    async fn claim_or_read_checkpoint_publication(&self, _claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        Err(DirectoryError::Backend("checkpoint publication receipts are unavailable for this backend".into()))
    }
    /// 🧹️ Releases an equal still-pending claim; a substituted digest never removes its owner.
    async fn release_checkpoint_publication(&self, _actor_user_id: &str, _correlation_id: &str, _command_sha256: &str) -> DirectoryResult<()> {
        Err(DirectoryError::Backend("checkpoint publication receipts are unavailable for this backend".into()))
    }
    //#endregion

    //#region Invites
    /// 🎟️ Not event-sourced (contract's decider laws) — only redemption is (`invite.redeemed`, see
    /// `DirectoryService::redeem_invite`). `create_invite`/`revoke_invite` are called directly by
    /// `decide` as its one documented write exception (`//#region 🔖️Decider`).
    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite>;
    async fn issue_invite_as_with_admin_effect(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedInvite>;
    async fn issue_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        self.issue_invite_as(space_id, role, ttl_secs, None, correlation_id).await
    }
    /// 🧭️ Verifies the exact stored capability before selecting the server-owned serialization scope.
    async fn invite_redemption_scope_hint(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1>;
    /// 🎟️ Claims `accepted_at`, appends the derived event and applies membership in one backend transaction.
    async fn redeem_invite_atomic(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit>;
    async fn revoke_invite_as(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()>;
    async fn revoke_invite_as_with_admin_effect(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()>;
    async fn revoke_invite(&self, space_id: &str, invite_id: &str, reason: &str, correlation_id: &str) -> DirectoryResult<()> {
        self.revoke_invite_as(space_id, invite_id, reason, None, correlation_id).await
    }
    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>>;
    /// 🏛️ One keyset-ordered bounded administration invite window (`created_at DESC, id DESC`),
    /// projected to metadata inside the backend so no selector or secret digest is ever read.
    async fn list_space_administration_invites_page(&self, space_id: &str, after: Option<(i64, &str)>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationInviteRow>>;
    //#endregion

    //#region SyncSessions
    /// 🔴️ Widened over the pre-ticket signature with `space_id`/`surface` (contract's
    /// presence scope is `(space_id, document_id, surface)`).
    async fn record_sync_session_open(
        &self,
        auth_session_id: Option<&str>,
        authorization_generation: u64,
        actor_id: &str,
        space_id: &str,
        document_id: &str,
        surface: &str,
        user_id: Option<&str>,
        authenticated_email: Option<&str>,
        space_role: Option<SpaceRole>,
        client_label: &str,
    ) -> DirectoryResult<SyncSessionRecord>;
    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()>;
    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>>;
    /// 🟢️ Every still-open session, optionally scoped to one space — the admin connections
    /// view and the per-space presence roster both read this instead of iterating documents.
    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>>;
    async fn list_active_sync_sessions(&self, space_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        self.list_active_sync_sessions_page(space_id, 0, limit).await
    }
    /// 🧹️ Marks every still-open session closed — called once at hub boot, before any real
    /// connection lands, to clear crash residue from the previous process (a session that never got
    /// its `disconnected_at` because the hub was killed mid-connection).
    async fn close_all_sync_sessions(&self) -> DirectoryResult<()>;
    //#endregion

    //#region EventLog
    /// ➕️ Persists `events` in one backend transaction: each gets a dense backend-assigned
    /// `seq` (contiguous with the current head, no gaps even under concurrent callers — callers are
    /// expected to already be serialized by `DirectoryService`'s write lock, but a backend MUST NOT
    /// rely on that alone for `seq` density since `rebuild_projections`/tests may call this
    /// directly), a minted uuid v7 `id`, and `recorded_at_ms`; then applies each event's projection
    /// (`//#region 🔖️Projections`) in the same transaction before committing.
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self.append_decided_events(events).await? {
            DirectoryAppendOutcomeV1::Appended(events) => Ok(events),
            DirectoryAppendOutcomeV1::RejectedBeforeCommit(reason) => Err(reason.into_error()),
        }
    }
    /// 🔐️ Appends a decided batch or proves a semantic refusal by acknowledging rollback.
    /// Errors, including failed rollback or commit, never authorize release of an idempotency claim.
    async fn append_decided_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1>;
    /// 🧷️ Commits the exact event page and its private administrator effect proof atomically.
    async fn append_decided_events_with_admin_effect(&self, events: &[NewDirectoryEvent], effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<DirectoryEvent>>;
    /// 📜️ Every event with `seq > since_seq`, ascending, capped at `limit` — backs both
    /// `GET /directory/events?since=` and `/directory/socket/v1`'s post-subscribe replay (contract C2).
    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>>;
    /// 🔝️ The current log length (0 when empty) — `DirectoryStreamMessage::Heartbeat`'s
    /// `head_seq` and the admin overview's `headSeq` both read this.
    async fn head_seq(&self) -> DirectoryResult<u64>;
    /// 🔁️ Truncates every projection table and replays the whole log from `seq` 1 through
    /// each event's projection (`//#region 🔖️Projections`) — returns the number of events replayed
    /// (which must equal `head_seq()` afterward). `POST /admin/api/directory/rebuild` (contract C2).
    async fn rebuild_projections(&self) -> DirectoryResult<u64>;
    /// 🔁️ Bounded replay with monotonic progress and success-only transaction commit.
    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64>;
    //#endregion
}
//#endregion 🔖️Trait
