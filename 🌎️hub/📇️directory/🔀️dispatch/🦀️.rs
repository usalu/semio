//! 🔀️ HubDirectories: the closed enum that dispatches the hub directory capability traits over the sqlite, postgres and neo4j backends.

use semio_hub_directory_contract::{ACTIVE_SYNC_SESSION_READ_MAX, ADMIN_PAGE_FETCH_MAX, InviteCapability, InviteRedemptionScopeHintV1, NewDirectoryEvent, SPACE_ADMINISTRATION_PAGE_FETCH_MAX, SessionCapability, ShareCapability};
use semio_hub_directory_contract::model::{AdminDirectoryOverviewCounts, AdminEffectCommitV1, AdminOperationAuditRecord, AdminOperationEffectReceiptV1, AdminSpaceSummaryRecord, AgentDelegationRecord, AgentDelegationRow, AuthAuditRecord, AuthSessionIssue, AuthSessionRecord, CheckpointPublicationClaimV1, CheckpointPublicationCompletionV1, CredentialAuditFactV1, DirectoryCommandClaimV1, DirectoryCommandReceiptCompletion, DirectoryCommandReceiptRecord, InviteRecord, InviteRedemptionCommit, IssuedAgentDelegation, IssuedAuthSession, IssuedInvite, IssuedShareToken, NewAdminOperationAuditRecord, NewAdminOperationEffectReceiptV1, NewCheckpointPublicationClaimV1, NewDirectoryCommandReceipt, RevokedAuthSession, ShareTokenRecord, SocketSessionBindingStatus, SocketShareBindingStatus, SpaceAdministrationInviteRow, SpaceAdministrationMemberRow, SpaceRecord, SpaceRole, SyncSessionRecord, UserRecord};
use semio_hub_artifact_authority::chunk_cas::{ArtifactCasDeleteFence, ArtifactCasObjectKey, ArtifactCasOwnershipPlanV1, ArtifactCasReservation};
use semio_hub_directory::{AdminOperationDirectory, AgentDelegationDirectory, ArtifactCasDirectory, ArtifactCasSweepCandidatePage, ArtifactCreationDirectory, AuthSessionDirectory, CredentialDirectory, DirectoryAppendOutcomeV1, DocumentDirectory, EventLog, InviteDirectory, ProjectionRebuild, ProjectionRebuildControl, ReceiptDirectory, ShareTokenDirectory, SpaceDirectory, SyncSessionDirectory, UserDirectory};
use directory::os_directory::{ArtifactCheckpoint, ArtifactHash, ArtifactRetention, DirectoryActor, DirectoryEvent, DocumentDescriptor, DocumentScope, Hlc, PublishedArtifactCheckpoint};
use semio_hub_artifact_creation::{ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactV1, ArtifactCreationIntentV1, ArtifactCreationOperationV1, DocumentGenesisAppendV1, DocumentGenesisCommitV1};
use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};

//#region 🔖️Dispatch
/// 🗄️ Closed-set dispatch enum over every hub directory backend this crate can compile in.
/// Hand-written, not `dyn_enum_close!`-generated: the macro's DSL has no per-variant `#[cfg]`
/// support (verified against its parser — `DynEnumVariant::parse` never calls
/// `Attribute::parse_outer`), and each variant here is gated on its own Cargo feature — see
/// `📓️terra-dedyn-fw-hub-repo-report.md`. The shape (one `From<Backend>` impl per variant plus a
/// match-delegating capability trait impls) is otherwise identical to what the macro emits for every
/// other closed-set family in this program (R11). `bin.rs`'s `connect_directory` builds exactly
/// one variant at process startup, chosen by `OS_HUB_DIRECTORY_BACKEND`.
pub enum HubDirectories {
    Sqlite(semio_hub_directory_sqlite::SqliteDirectory),
    Postgres(semio_hub_directory_postgres::PostgresDirectory),
    Neo4j(semio_hub_directory_neo4j::Neo4jDirectory),
}

impl ::core::convert::From<semio_hub_directory_sqlite::SqliteDirectory> for HubDirectories {
    fn from(value: semio_hub_directory_sqlite::SqliteDirectory) -> Self {
        Self::Sqlite(value)
    }
}

impl ::core::convert::From<semio_hub_directory_postgres::PostgresDirectory> for HubDirectories {
    fn from(value: semio_hub_directory_postgres::PostgresDirectory) -> Self {
        Self::Postgres(value)
    }
}

impl ::core::convert::From<semio_hub_directory_neo4j::Neo4jDirectory> for HubDirectories {
    fn from(value: semio_hub_directory_neo4j::Neo4jDirectory) -> Self {
        Self::Neo4j(value)
    }
}

impl ShareTokenDirectory for HubDirectories {
    async fn issue_share_token_as(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedShareToken> {
        match self {
            Self::Sqlite(inner) => inner.issue_share_token_as(scope, ttl_secs, actor_user_id, correlation_id).await,
            Self::Postgres(inner) => inner.issue_share_token_as(scope, ttl_secs, actor_user_id, correlation_id).await,
            Self::Neo4j(inner) => inner.issue_share_token_as(scope, ttl_secs, actor_user_id, correlation_id).await,
        }
    }

    async fn issue_share_token_as_with_admin_effect(&self, scope: &DocumentScope, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedShareToken> {
        match self {
            Self::Sqlite(inner) => inner.issue_share_token_as_with_admin_effect(scope, ttl_secs, actor_user_id, correlation_id, effect).await,
            Self::Postgres(inner) => inner.issue_share_token_as_with_admin_effect(scope, ttl_secs, actor_user_id, correlation_id, effect).await,
            Self::Neo4j(inner) => inner.issue_share_token_as_with_admin_effect(scope, ttl_secs, actor_user_id, correlation_id, effect).await,
        }
    }

    async fn revoke_share_token_as(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.revoke_share_token_as(scope, share_id, reason, actor_user_id, correlation_id).await,
            Self::Postgres(inner) => inner.revoke_share_token_as(scope, share_id, reason, actor_user_id, correlation_id).await,
            Self::Neo4j(inner) => inner.revoke_share_token_as(scope, share_id, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn revoke_share_token_as_with_admin_effect(&self, scope: &DocumentScope, share_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()> {
        match self {
            Self::Sqlite(inner) => inner.revoke_share_token_as_with_admin_effect(scope, share_id, reason, actor_user_id, correlation_id, effect).await,
            Self::Postgres(inner) => inner.revoke_share_token_as_with_admin_effect(scope, share_id, reason, actor_user_id, correlation_id, effect).await,
            Self::Neo4j(inner) => inner.revoke_share_token_as_with_admin_effect(scope, share_id, reason, actor_user_id, correlation_id, effect).await,
        }
    }

    async fn authenticate_share(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<bool> {
        match self {
            Self::Sqlite(inner) => inner.authenticate_share(scope, capability).await,
            Self::Postgres(inner) => inner.authenticate_share(scope, capability).await,
            Self::Neo4j(inner) => inner.authenticate_share(scope, capability).await,
        }
    }

    async fn authenticate_share_binding(&self, scope: &DocumentScope, capability: &ShareCapability) -> DirectoryResult<Option<ShareTokenRecord>> {
        match self {
            Self::Sqlite(inner) => inner.authenticate_share_binding(scope, capability).await,
            Self::Postgres(inner) => inner.authenticate_share_binding(scope, capability).await,
            Self::Neo4j(inner) => inner.authenticate_share_binding(scope, capability).await,
        }
    }

    async fn socket_share_binding(&self, share_id: &str, selector: &str, scope: &DocumentScope, now_ms: i64) -> DirectoryResult<SocketShareBindingStatus> {
        match self {
            Self::Sqlite(inner) => inner.socket_share_binding(share_id, selector, scope, now_ms).await,
            Self::Postgres(inner) => inner.socket_share_binding(share_id, selector, scope, now_ms).await,
            Self::Neo4j(inner) => inner.socket_share_binding(share_id, selector, scope, now_ms).await,
        }
    }
}

impl InviteDirectory for HubDirectories {
    async fn issue_invite_as(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<IssuedInvite> {
        match self {
            Self::Sqlite(inner) => inner.issue_invite_as(space_id, role, ttl_secs, actor_user_id, correlation_id).await,
            Self::Postgres(inner) => inner.issue_invite_as(space_id, role, ttl_secs, actor_user_id, correlation_id).await,
            Self::Neo4j(inner) => inner.issue_invite_as(space_id, role, ttl_secs, actor_user_id, correlation_id).await,
        }
    }

    async fn issue_invite_as_with_admin_effect(&self, space_id: &str, role: SpaceRole, ttl_secs: i64, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<IssuedInvite> {
        match self {
            Self::Sqlite(inner) => inner.issue_invite_as_with_admin_effect(space_id, role, ttl_secs, actor_user_id, correlation_id, effect).await,
            Self::Postgres(inner) => inner.issue_invite_as_with_admin_effect(space_id, role, ttl_secs, actor_user_id, correlation_id, effect).await,
            Self::Neo4j(inner) => inner.issue_invite_as_with_admin_effect(space_id, role, ttl_secs, actor_user_id, correlation_id, effect).await,
        }
    }

    async fn invite_redemption_scope_hint(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str) -> DirectoryResult<InviteRedemptionScopeHintV1> {
        match self {
            Self::Sqlite(inner) => inner.invite_redemption_scope_hint(capability, actor, user_id).await,
            Self::Postgres(inner) => inner.invite_redemption_scope_hint(capability, actor, user_id).await,
            Self::Neo4j(inner) => inner.invite_redemption_scope_hint(capability, actor, user_id).await,
        }
    }

    async fn redeem_invite_atomic(&self, capability: &InviteCapability, actor: &DirectoryActor, user_id: &str, hlc: Hlc) -> DirectoryResult<InviteRedemptionCommit> {
        match self {
            Self::Sqlite(inner) => inner.redeem_invite_atomic(capability, actor, user_id, hlc).await,
            Self::Postgres(inner) => inner.redeem_invite_atomic(capability, actor, user_id, hlc).await,
            Self::Neo4j(inner) => inner.redeem_invite_atomic(capability, actor, user_id, hlc).await,
        }
    }

    async fn revoke_invite_as(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.revoke_invite_as(space_id, invite_id, reason, actor_user_id, correlation_id).await,
            Self::Postgres(inner) => inner.revoke_invite_as(space_id, invite_id, reason, actor_user_id, correlation_id).await,
            Self::Neo4j(inner) => inner.revoke_invite_as(space_id, invite_id, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn revoke_invite_as_with_admin_effect(&self, space_id: &str, invite_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<()> {
        match self {
            Self::Sqlite(inner) => inner.revoke_invite_as_with_admin_effect(space_id, invite_id, reason, actor_user_id, correlation_id, effect).await,
            Self::Postgres(inner) => inner.revoke_invite_as_with_admin_effect(space_id, invite_id, reason, actor_user_id, correlation_id, effect).await,
            Self::Neo4j(inner) => inner.revoke_invite_as_with_admin_effect(space_id, invite_id, reason, actor_user_id, correlation_id, effect).await,
        }
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        match self {
            Self::Sqlite(inner) => inner.list_invites(space_id).await,
            Self::Postgres(inner) => inner.list_invites(space_id).await,
            Self::Neo4j(inner) => inner.list_invites(space_id).await,
        }
    }

    async fn list_space_administration_invites_page(&self, space_id: &str, after: Option<(i64, &str)>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationInviteRow>> {
        if limit == 0 || limit > SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration invite page limit must be 1..={SPACE_ADMINISTRATION_PAGE_FETCH_MAX}")));
        }
        match self {
            Self::Sqlite(inner) => inner.list_space_administration_invites_page(space_id, after, limit).await,
            Self::Postgres(inner) => inner.list_space_administration_invites_page(space_id, after, limit).await,
            Self::Neo4j(inner) => inner.list_space_administration_invites_page(space_id, after, limit).await,
        }
    }
}

impl UserDirectory for HubDirectories {
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        match self {
            Self::Sqlite(inner) => inner.create_user(email, display_name, password_hash, sso_subject, sso_provider).await,
            Self::Postgres(inner) => inner.create_user(email, display_name, password_hash, sso_subject, sso_provider).await,
            Self::Neo4j(inner) => inner.create_user(email, display_name, password_hash, sso_subject, sso_provider).await,
        }
    }

    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>> {
        match self {
            Self::Sqlite(inner) => inner.get_user(user_id).await,
            Self::Postgres(inner) => inner.get_user(user_id).await,
            Self::Neo4j(inner) => inner.get_user(user_id).await,
        }
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        match self {
            Self::Sqlite(inner) => inner.get_user_by_email(email).await,
            Self::Postgres(inner) => inner.get_user_by_email(email).await,
            Self::Neo4j(inner) => inner.get_user_by_email(email).await,
        }
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        match self {
            Self::Sqlite(inner) => inner.get_user_by_sso_subject(provider, subject).await,
            Self::Postgres(inner) => inner.get_user_by_sso_subject(provider, subject).await,
            Self::Neo4j(inner) => inner.get_user_by_sso_subject(provider, subject).await,
        }
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        match self {
            Self::Sqlite(inner) => inner.list_users(limit, offset).await,
            Self::Postgres(inner) => inner.list_users(limit, offset).await,
            Self::Neo4j(inner) => inner.list_users(limit, offset).await,
        }
    }
}

impl AuthSessionDirectory for HubDirectories {
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession> {
        match self {
            Self::Sqlite(inner) => inner.issue_auth_session(issue).await,
            Self::Postgres(inner) => inner.issue_auth_session(issue).await,
            Self::Neo4j(inner) => inner.issue_auth_session(issue).await,
        }
    }

    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>> {
        match self {
            Self::Sqlite(inner) => inner.authenticate_session(capability).await,
            Self::Postgres(inner) => inner.authenticate_session(capability).await,
            Self::Neo4j(inner) => inner.authenticate_session(capability).await,
        }
    }

    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus> {
        match self {
            Self::Sqlite(inner) => inner.socket_session_binding(session_id, user_id, authorization_generation, space_id, now_ms).await,
            Self::Postgres(inner) => inner.socket_session_binding(session_id, user_id, authorization_generation, space_id, now_ms).await,
            Self::Neo4j(inner) => inner.socket_session_binding(session_id, user_id, authorization_generation, space_id, now_ms).await,
        }
    }

    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>> {
        match self {
            Self::Sqlite(inner) => inner.revoke_auth_session(id, reason, actor_user_id, correlation_id).await,
            Self::Postgres(inner) => inner.revoke_auth_session(id, reason, actor_user_id, correlation_id).await,
            Self::Neo4j(inner) => inner.revoke_auth_session(id, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        match self {
            Self::Sqlite(inner) => inner.revoke_auth_sessions_for_user(user_id, reason, actor_user_id, correlation_id).await,
            Self::Postgres(inner) => inner.revoke_auth_sessions_for_user(user_id, reason, actor_user_id, correlation_id).await,
            Self::Neo4j(inner) => inner.revoke_auth_sessions_for_user(user_id, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn revoke_auth_sessions_for_user_with_admin_effect(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>> {
        match self {
            Self::Sqlite(inner) => inner.revoke_auth_sessions_for_user_with_admin_effect(user_id, reason, actor_user_id, correlation_id, effect).await,
            Self::Postgres(inner) => inner.revoke_auth_sessions_for_user_with_admin_effect(user_id, reason, actor_user_id, correlation_id, effect).await,
            Self::Neo4j(inner) => inner.revoke_auth_sessions_for_user_with_admin_effect(user_id, reason, actor_user_id, correlation_id, effect).await,
        }
    }

    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>> {
        match self {
            Self::Sqlite(inner) => inner.revoke_auth_sessions_for_identity(provider, subject_digest, reason, actor_user_id, correlation_id).await,
            Self::Postgres(inner) => inner.revoke_auth_sessions_for_identity(provider, subject_digest, reason, actor_user_id, correlation_id).await,
            Self::Neo4j(inner) => inner.revoke_auth_sessions_for_identity(provider, subject_digest, reason, actor_user_id, correlation_id).await,
        }
    }

    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>> {
        match self {
            Self::Sqlite(inner) => inner.list_auth_audit(limit, offset).await,
            Self::Postgres(inner) => inner.list_auth_audit(limit, offset).await,
            Self::Neo4j(inner) => inner.list_auth_audit(limit, offset).await,
        }
    }
}

impl CredentialDirectory for HubDirectories {
    async fn append_credential_audit(&self, fact: &CredentialAuditFactV1) -> DirectoryResult<AuthAuditRecord> {
        match self {
            Self::Sqlite(inner) => inner.append_credential_audit(fact).await,
            Self::Postgres(inner) => inner.append_credential_audit(fact).await,
            Self::Neo4j(inner) => inner.append_credential_audit(fact).await,
        }
    }

    async fn set_password_credential(&self, user_id: &str, encoded_credential: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.set_password_credential(user_id, encoded_credential, actor_user_id, correlation_id).await,
            Self::Postgres(inner) => inner.set_password_credential(user_id, encoded_credential, actor_user_id, correlation_id).await,
            Self::Neo4j(inner) => inner.set_password_credential(user_id, encoded_credential, actor_user_id, correlation_id).await,
        }
    }
}

impl AgentDelegationDirectory for HubDirectories {
    async fn create_agent_delegation(&self, issued: &IssuedAgentDelegation, correlation_id: &str, peer_class: &str) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.create_agent_delegation(issued, correlation_id, peer_class).await,
            Self::Postgres(inner) => inner.create_agent_delegation(issued, correlation_id, peer_class).await,
            Self::Neo4j(inner) => inner.create_agent_delegation(issued, correlation_id, peer_class).await,
        }
    }

    async fn list_agent_delegations(&self, space_id: &str, delegating_user_id: &str, limit: usize) -> DirectoryResult<Vec<AgentDelegationRow>> {
        match self {
            Self::Sqlite(inner) => inner.list_agent_delegations(space_id, delegating_user_id, limit).await,
            Self::Postgres(inner) => inner.list_agent_delegations(space_id, delegating_user_id, limit).await,
            Self::Neo4j(inner) => inner.list_agent_delegations(space_id, delegating_user_id, limit).await,
        }
    }

    async fn revoke_agent_delegation(&self, delegation_id: &str, delegating_user_id: &str, reason: &str, correlation_id: &str, now_ms: i64) -> DirectoryResult<Option<Vec<RevokedAuthSession>>> {
        match self {
            Self::Sqlite(inner) => inner.revoke_agent_delegation(delegation_id, delegating_user_id, reason, correlation_id, now_ms).await,
            Self::Postgres(inner) => inner.revoke_agent_delegation(delegation_id, delegating_user_id, reason, correlation_id, now_ms).await,
            Self::Neo4j(inner) => inner.revoke_agent_delegation(delegation_id, delegating_user_id, reason, correlation_id, now_ms).await,
        }
    }

    async fn load_agent_delegation(&self, selector: &str) -> DirectoryResult<Option<AgentDelegationRecord>> {
        match self {
            Self::Sqlite(inner) => inner.load_agent_delegation(selector).await,
            Self::Postgres(inner) => inner.load_agent_delegation(selector).await,
            Self::Neo4j(inner) => inner.load_agent_delegation(selector).await,
        }
    }

    async fn agent_delegation(&self, delegation_id: &str) -> DirectoryResult<Option<AgentDelegationRecord>> {
        match self {
            Self::Sqlite(inner) => inner.agent_delegation(delegation_id).await,
            Self::Postgres(inner) => inner.agent_delegation(delegation_id).await,
            Self::Neo4j(inner) => inner.agent_delegation(delegation_id).await,
        }
    }
}

impl SpaceDirectory for HubDirectories {
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        match self {
            Self::Sqlite(inner) => inner.get_space(space_id).await,
            Self::Postgres(inner) => inner.get_space(space_id).await,
            Self::Neo4j(inner) => inner.get_space(space_id).await,
        }
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        match self {
            Self::Sqlite(inner) => inner.list_spaces_for_user(user_id).await,
            Self::Postgres(inner) => inner.list_spaces_for_user(user_id).await,
            Self::Neo4j(inner) => inner.list_spaces_for_user(user_id).await,
        }
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        match self {
            Self::Sqlite(inner) => inner.list_spaces(limit, offset).await,
            Self::Postgres(inner) => inner.list_spaces(limit, offset).await,
            Self::Neo4j(inner) => inner.list_spaces(limit, offset).await,
        }
    }

    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>> {
        if limit == 0 || limit > ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator space page limit must be 1..={ADMIN_PAGE_FETCH_MAX}")));
        }
        match self {
            Self::Sqlite(inner) => inner.list_admin_space_summaries_page(space_id, offset, limit).await,
            Self::Postgres(inner) => inner.list_admin_space_summaries_page(space_id, offset, limit).await,
            Self::Neo4j(inner) => inner.list_admin_space_summaries_page(space_id, offset, limit).await,
        }
    }

    async fn list_visible_space_summaries(&self, user_id: Option<&str>) -> DirectoryResult<Vec<(AdminSpaceSummaryRecord, Option<SpaceRole>)>> {
        match self {
            Self::Sqlite(inner) => inner.list_visible_space_summaries(user_id).await,
            Self::Postgres(inner) => inner.list_visible_space_summaries(user_id).await,
            Self::Neo4j(inner) => inner.list_visible_space_summaries(user_id).await,
        }
    }

    async fn list_admin_space_members_page(&self, space_id: &str, offset: usize, limit: usize) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        if limit == 0 || limit > ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator member page limit must be 1..={ADMIN_PAGE_FETCH_MAX}")));
        }
        match self {
            Self::Sqlite(inner) => inner.list_admin_space_members_page(space_id, offset, limit).await,
            Self::Postgres(inner) => inner.list_admin_space_members_page(space_id, offset, limit).await,
            Self::Neo4j(inner) => inner.list_admin_space_members_page(space_id, offset, limit).await,
        }
    }

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        match self {
            Self::Sqlite(inner) => inner.list_members(space_id).await,
            Self::Postgres(inner) => inner.list_members(space_id).await,
            Self::Neo4j(inner) => inner.list_members(space_id).await,
        }
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        match self {
            Self::Sqlite(inner) => inner.get_role(space_id, user_id).await,
            Self::Postgres(inner) => inner.get_role(space_id, user_id).await,
            Self::Neo4j(inner) => inner.get_role(space_id, user_id).await,
        }
    }

    async fn list_space_administration_members_page(&self, space_id: &str, after_user_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationMemberRow>> {
        if limit == 0 || limit > SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration member page limit must be 1..={SPACE_ADMINISTRATION_PAGE_FETCH_MAX}")));
        }
        match self {
            Self::Sqlite(inner) => inner.list_space_administration_members_page(space_id, after_user_id, limit).await,
            Self::Postgres(inner) => inner.list_space_administration_members_page(space_id, after_user_id, limit).await,
            Self::Neo4j(inner) => inner.list_space_administration_members_page(space_id, after_user_id, limit).await,
        }
    }
}

impl DocumentDirectory for HubDirectories {
    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>> {
        match self {
            Self::Sqlite(inner) => inner.get_document_descriptor(scope).await,
            Self::Postgres(inner) => inner.get_document_descriptor(scope).await,
            Self::Neo4j(inner) => inner.get_document_descriptor(scope).await,
        }
    }

    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>> {
        match self {
            Self::Sqlite(inner) => inner.list_document_descriptors(space_id).await,
            Self::Postgres(inner) => inner.list_document_descriptors(space_id).await,
            Self::Neo4j(inner) => inner.list_document_descriptors(space_id).await,
        }
    }

    async fn list_document_descriptors_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<DocumentDescriptor>> {
        if limit == 0 || limit > ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator document page limit must be 1..={ADMIN_PAGE_FETCH_MAX}")));
        }
        match self {
            Self::Sqlite(inner) => inner.list_document_descriptors_page(space_id, offset, limit).await,
            Self::Postgres(inner) => inner.list_document_descriptors_page(space_id, offset, limit).await,
            Self::Neo4j(inner) => inner.list_document_descriptors_page(space_id, offset, limit).await,
        }
    }

    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        match self {
            Self::Sqlite(inner) => inner.get_artifact_checkpoint(scope, checkpoint_id).await,
            Self::Postgres(inner) => inner.get_artifact_checkpoint(scope, checkpoint_id).await,
            Self::Neo4j(inner) => inner.get_artifact_checkpoint(scope, checkpoint_id).await,
        }
    }

    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>> {
        match self {
            Self::Sqlite(inner) => inner.get_verified_artifact_checkpoint(scope, checkpoint_id).await,
            Self::Postgres(inner) => inner.get_verified_artifact_checkpoint(scope, checkpoint_id).await,
            Self::Neo4j(inner) => inner.get_verified_artifact_checkpoint(scope, checkpoint_id).await,
        }
    }

    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        match self {
            Self::Sqlite(inner) => inner.get_active_artifact_checkpoint(scope).await,
            Self::Postgres(inner) => inner.get_active_artifact_checkpoint(scope).await,
            Self::Neo4j(inner) => inner.get_active_artifact_checkpoint(scope).await,
        }
    }

    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>> {
        match self {
            Self::Sqlite(inner) => inner.get_artifact_retention(scope).await,
            Self::Postgres(inner) => inner.get_artifact_retention(scope).await,
            Self::Neo4j(inner) => inner.get_artifact_retention(scope).await,
        }
    }

    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64> {
        match self {
            Self::Sqlite(inner) => inner.artifact_checkpoint_count(scope).await,
            Self::Postgres(inner) => inner.artifact_checkpoint_count(scope).await,
            Self::Neo4j(inner) => inner.artifact_checkpoint_count(scope).await,
        }
    }

    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>> {
        match self {
            Self::Sqlite(inner) => inner.list_artifact_checkpoint_lineage(scope, limit).await,
            Self::Postgres(inner) => inner.list_artifact_checkpoint_lineage(scope, limit).await,
            Self::Neo4j(inner) => inner.list_artifact_checkpoint_lineage(scope, limit).await,
        }
    }
}

impl ArtifactCreationDirectory for HubDirectories {
    async fn claim_artifact_creation(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1> {
        match self {
            Self::Sqlite(inner) => inner.claim_artifact_creation(intent).await,
            Self::Postgres(inner) => inner.claim_artifact_creation(intent).await,
            Self::Neo4j(inner) => inner.claim_artifact_creation(intent).await,
        }
    }

    async fn read_artifact_creation(&self, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
        match self {
            Self::Sqlite(inner) => inner.read_artifact_creation(actor_user_id, request_id).await,
            Self::Postgres(inner) => inner.read_artifact_creation(actor_user_id, request_id).await,
            Self::Neo4j(inner) => inner.read_artifact_creation(actor_user_id, request_id).await,
        }
    }

    async fn append_artifact_creation_fact(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1> {
        match self {
            Self::Sqlite(inner) => inner.append_artifact_creation_fact(append).await,
            Self::Postgres(inner) => inner.append_artifact_creation_fact(append).await,
            Self::Neo4j(inner) => inner.append_artifact_creation_fact(append).await,
        }
    }

    async fn append_document_genesis(&self, append: &DocumentGenesisAppendV1) -> DirectoryResult<DocumentGenesisCommitV1> {
        match self {
            Self::Sqlite(inner) => inner.append_document_genesis(append).await,
            Self::Postgres(inner) => inner.append_document_genesis(append).await,
            Self::Neo4j(inner) => inner.append_document_genesis(append).await,
        }
    }

    async fn artifact_creation_recovery_candidates(&self, now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>> {
        match self {
            Self::Sqlite(inner) => inner.artifact_creation_recovery_candidates(now_ms, limit).await,
            Self::Postgres(inner) => inner.artifact_creation_recovery_candidates(now_ms, limit).await,
            Self::Neo4j(inner) => inner.artifact_creation_recovery_candidates(now_ms, limit).await,
        }
    }

    async fn artifact_creation_terminate_uncommitted(&self, intent: &ArtifactCreationIntentV1, now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1> {
        match self {
            Self::Sqlite(inner) => inner.artifact_creation_terminate_uncommitted(intent, now_ms).await,
            Self::Postgres(inner) => inner.artifact_creation_terminate_uncommitted(intent, now_ms).await,
            Self::Neo4j(inner) => inner.artifact_creation_terminate_uncommitted(intent, now_ms).await,
        }
    }
}

impl ArtifactCasDirectory for HubDirectories {
    async fn reserve_artifact_cas(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        match self {
            Self::Sqlite(inner) => inner.reserve_artifact_cas(plan, expires_at_ms, now_ms).await,
            Self::Postgres(inner) => inner.reserve_artifact_cas(plan, expires_at_ms, now_ms).await,
            Self::Neo4j(inner) => inner.reserve_artifact_cas(plan, expires_at_ms, now_ms).await,
        }
    }

    async fn append_reserved_artifact_checkpoint(
        &self,
        event: Option<&NewDirectoryEvent>,
        checkpoint: &ArtifactCheckpoint,
        reservation: &ArtifactCasReservation,
        completion: Option<&CheckpointPublicationCompletionV1>,
        now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self {
            Self::Sqlite(inner) => inner.append_reserved_artifact_checkpoint(event, checkpoint, reservation, completion, now_ms).await,
            Self::Postgres(inner) => inner.append_reserved_artifact_checkpoint(event, checkpoint, reservation, completion, now_ms).await,
            Self::Neo4j(inner) => inner.append_reserved_artifact_checkpoint(event, checkpoint, reservation, completion, now_ms).await,
        }
    }

    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        match self {
            Self::Sqlite(inner) => inner.artifact_cas_ledger_generation().await,
            Self::Postgres(inner) => inner.artifact_cas_ledger_generation().await,
            Self::Neo4j(inner) => inner.artifact_cas_ledger_generation().await,
        }
    }

    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]> {
        match self {
            Self::Sqlite(inner) => inner.artifact_cas_coordinator_id().await,
            Self::Postgres(inner) => inner.artifact_cas_coordinator_id().await,
            Self::Neo4j(inner) => inner.artifact_cas_coordinator_id().await,
        }
    }

    async fn artifact_cas_sweep_candidates(&self, after_generation: u64, through_generation: u64, limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        match self {
            Self::Sqlite(inner) => inner.artifact_cas_sweep_candidates(after_generation, through_generation, limit).await,
            Self::Postgres(inner) => inner.artifact_cas_sweep_candidates(after_generation, through_generation, limit).await,
            Self::Neo4j(inner) => inner.artifact_cas_sweep_candidates(after_generation, through_generation, limit).await,
        }
    }

    async fn artifact_cas_delete_preview_protected(&self, key: &ArtifactCasObjectKey, observed_generation: u64, now_ms: u64) -> DirectoryResult<bool> {
        match self {
            Self::Sqlite(inner) => inner.artifact_cas_delete_preview_protected(key, observed_generation, now_ms).await,
            Self::Postgres(inner) => inner.artifact_cas_delete_preview_protected(key, observed_generation, now_ms).await,
            Self::Neo4j(inner) => inner.artifact_cas_delete_preview_protected(key, observed_generation, now_ms).await,
        }
    }

    async fn acquire_artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, lease_token: [u8; 32], now_ms: u64, expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        match self {
            Self::Sqlite(inner) => inner.acquire_artifact_cas_delete_fence(key, observed_generation, lease_token, now_ms, expires_at_ms).await,
            Self::Postgres(inner) => inner.acquire_artifact_cas_delete_fence(key, observed_generation, lease_token, now_ms, expires_at_ms).await,
            Self::Neo4j(inner) => inner.acquire_artifact_cas_delete_fence(key, observed_generation, lease_token, now_ms, expires_at_ms).await,
        }
    }

    async fn validate_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64) -> DirectoryResult<bool> {
        match self {
            Self::Sqlite(inner) => inner.validate_artifact_cas_delete_fence(fence, now_ms).await,
            Self::Postgres(inner) => inner.validate_artifact_cas_delete_fence(fence, now_ms).await,
            Self::Neo4j(inner) => inner.validate_artifact_cas_delete_fence(fence, now_ms).await,
        }
    }

    async fn renew_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64, expires_at_ms: u64) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.renew_artifact_cas_delete_fence(fence, now_ms, expires_at_ms).await,
            Self::Postgres(inner) => inner.renew_artifact_cas_delete_fence(fence, now_ms, expires_at_ms).await,
            Self::Neo4j(inner) => inner.renew_artifact_cas_delete_fence(fence, now_ms, expires_at_ms).await,
        }
    }

    async fn release_artifact_cas_delete_fence(&self, fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.release_artifact_cas_delete_fence(fence).await,
            Self::Postgres(inner) => inner.release_artifact_cas_delete_fence(fence).await,
            Self::Neo4j(inner) => inner.release_artifact_cas_delete_fence(fence).await,
        }
    }
}

impl AdminOperationDirectory for HubDirectories {
    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts> {
        match self {
            Self::Sqlite(inner) => inner.admin_overview_counts().await,
            Self::Postgres(inner) => inner.admin_overview_counts().await,
            Self::Neo4j(inner) => inner.admin_overview_counts().await,
        }
    }

    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord> {
        match self {
            Self::Sqlite(inner) => inner.append_admin_operation_audit(fact).await,
            Self::Postgres(inner) => inner.append_admin_operation_audit(fact).await,
            Self::Neo4j(inner) => inner.append_admin_operation_audit(fact).await,
        }
    }

    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        match self {
            Self::Sqlite(inner) => inner.admin_operation_audit_for_request(request_id).await,
            Self::Postgres(inner) => inner.admin_operation_audit_for_request(request_id).await,
            Self::Neo4j(inner) => inner.admin_operation_audit_for_request(request_id).await,
        }
    }

    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        match self {
            Self::Sqlite(inner) => inner.admin_operation_audit_for_operation(operation_id).await,
            Self::Postgres(inner) => inner.admin_operation_audit_for_operation(operation_id).await,
            Self::Neo4j(inner) => inner.admin_operation_audit_for_operation(operation_id).await,
        }
    }

    async fn admin_operation_effect_receipt(&self, operation_id: &str, intent_digest: &str) -> DirectoryResult<Option<AdminOperationEffectReceiptV1>> {
        match self {
            Self::Sqlite(inner) => inner.admin_operation_effect_receipt(operation_id, intent_digest).await,
            Self::Postgres(inner) => inner.admin_operation_effect_receipt(operation_id, intent_digest).await,
            Self::Neo4j(inner) => inner.admin_operation_effect_receipt(operation_id, intent_digest).await,
        }
    }

    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        match self {
            Self::Sqlite(inner) => inner.list_admin_operation_audit(after_sequence, limit).await,
            Self::Postgres(inner) => inner.list_admin_operation_audit(after_sequence, limit).await,
            Self::Neo4j(inner) => inner.list_admin_operation_audit(after_sequence, limit).await,
        }
    }
}

impl ReceiptDirectory for HubDirectories {
    async fn claim_or_read_directory_command_receipt(&self, claim: &NewDirectoryCommandReceipt) -> DirectoryResult<DirectoryCommandClaimV1> {
        match self {
            Self::Sqlite(inner) => inner.claim_or_read_directory_command_receipt(claim).await,
            Self::Postgres(inner) => inner.claim_or_read_directory_command_receipt(claim).await,
            Self::Neo4j(inner) => inner.claim_or_read_directory_command_receipt(claim).await,
        }
    }

    async fn complete_directory_command_receipt(&self, completion: &DirectoryCommandReceiptCompletion) -> DirectoryResult<DirectoryCommandReceiptRecord> {
        match self {
            Self::Sqlite(inner) => inner.complete_directory_command_receipt(completion).await,
            Self::Postgres(inner) => inner.complete_directory_command_receipt(completion).await,
            Self::Neo4j(inner) => inner.complete_directory_command_receipt(completion).await,
        }
    }

    async fn release_directory_command_receipt(&self, actor_user_id: &str, request_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.release_directory_command_receipt(actor_user_id, request_id, command_sha256).await,
            Self::Postgres(inner) => inner.release_directory_command_receipt(actor_user_id, request_id, command_sha256).await,
            Self::Neo4j(inner) => inner.release_directory_command_receipt(actor_user_id, request_id, command_sha256).await,
        }
    }

    async fn claim_or_read_checkpoint_publication(&self, claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        match self {
            Self::Sqlite(inner) => inner.claim_or_read_checkpoint_publication(claim).await,
            Self::Postgres(inner) => inner.claim_or_read_checkpoint_publication(claim).await,
            Self::Neo4j(inner) => inner.claim_or_read_checkpoint_publication(claim).await,
        }
    }

    async fn release_checkpoint_publication(&self, actor_user_id: &str, correlation_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.release_checkpoint_publication(actor_user_id, correlation_id, command_sha256).await,
            Self::Postgres(inner) => inner.release_checkpoint_publication(actor_user_id, correlation_id, command_sha256).await,
            Self::Neo4j(inner) => inner.release_checkpoint_publication(actor_user_id, correlation_id, command_sha256).await,
        }
    }
}

impl SyncSessionDirectory for HubDirectories {
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
    ) -> DirectoryResult<SyncSessionRecord> {
        match self {
            Self::Sqlite(inner) => inner.record_sync_session_open(auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label).await,
            Self::Postgres(inner) => inner.record_sync_session_open(auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label).await,
            Self::Neo4j(inner) => inner.record_sync_session_open(auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label).await,
        }
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.record_sync_session_close(sync_session_id).await,
            Self::Postgres(inner) => inner.record_sync_session_close(sync_session_id).await,
            Self::Neo4j(inner) => inner.record_sync_session_close(sync_session_id).await,
        }
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        match self {
            Self::Sqlite(inner) => inner.list_sync_sessions_for_document(document_id).await,
            Self::Postgres(inner) => inner.list_sync_sessions_for_document(document_id).await,
            Self::Neo4j(inner) => inner.list_sync_sessions_for_document(document_id).await,
        }
    }

    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        if limit == 0 || limit > ACTIVE_SYNC_SESSION_READ_MAX {
            return Err(DirectoryError::Conflict(format!("active sync-session limit must be 1..={ACTIVE_SYNC_SESSION_READ_MAX}")));
        }
        match self {
            Self::Sqlite(inner) => inner.list_active_sync_sessions_page(space_id, offset, limit).await,
            Self::Postgres(inner) => inner.list_active_sync_sessions_page(space_id, offset, limit).await,
            Self::Neo4j(inner) => inner.list_active_sync_sessions_page(space_id, offset, limit).await,
        }
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        match self {
            Self::Sqlite(inner) => inner.close_all_sync_sessions().await,
            Self::Postgres(inner) => inner.close_all_sync_sessions().await,
            Self::Neo4j(inner) => inner.close_all_sync_sessions().await,
        }
    }
}

impl EventLog for HubDirectories {
    async fn append_decided_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1> {
        match self {
            Self::Sqlite(inner) => inner.append_decided_events(events).await,
            Self::Postgres(inner) => inner.append_decided_events(events).await,
            Self::Neo4j(inner) => inner.append_decided_events(events).await,
        }
    }

    async fn append_decided_events_with_admin_effect(&self, events: &[NewDirectoryEvent], effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<DirectoryEvent>> {
        match self {
            Self::Sqlite(inner) => inner.append_decided_events_with_admin_effect(events, effect).await,
            Self::Postgres(inner) => inner.append_decided_events_with_admin_effect(events, effect).await,
            Self::Neo4j(inner) => inner.append_decided_events_with_admin_effect(events, effect).await,
        }
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self {
            Self::Sqlite(inner) => inner.events_since(since_seq, limit).await,
            Self::Postgres(inner) => inner.events_since(since_seq, limit).await,
            Self::Neo4j(inner) => inner.events_since(since_seq, limit).await,
        }
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        match self {
            Self::Sqlite(inner) => inner.head_seq().await,
            Self::Postgres(inner) => inner.head_seq().await,
            Self::Neo4j(inner) => inner.head_seq().await,
        }
    }
}

impl ProjectionRebuild for HubDirectories {
    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        match self {
            Self::Sqlite(inner) => inner.rebuild_projections().await,
            Self::Postgres(inner) => inner.rebuild_projections().await,
            Self::Neo4j(inner) => inner.rebuild_projections().await,
        }
    }

    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64> {
        match self {
            Self::Sqlite(inner) => inner.rebuild_projections_controlled(control).await,
            Self::Postgres(inner) => inner.rebuild_projections_controlled(control).await,
            Self::Neo4j(inner) => inner.rebuild_projections_controlled(control).await,
        }
    }
}
//#endregion 🔖️Dispatch
