//! 👤️ Capability traits for users, authentication sessions, credentials and agent delegations.

use super::*;

//#region UserDirectory
/// 👤️ User accounts: creation and the lookups by id, email and single-sign-on subject.
pub trait UserDirectory: Send + Sync + 'static {
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord>;
    /// 🔎️ Single-user lookup by id — the `member.upserted`/`invite.redeemed` projections
    /// resolve `MemberView.email`/`display_name` through this, not through `get_user_by_email`.
    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>>;
}
//#endregion UserDirectory

//#region AuthSessionDirectory
/// 🔐️ Authentication sessions: issue, authenticate and revoke them, bind sockets to them, and read their audit log.
pub trait AuthSessionDirectory: Send + Sync + 'static {
    async fn issue_auth_session(&self, issue: &AuthSessionIssue) -> DirectoryResult<IssuedAuthSession>;
    async fn authenticate_session(&self, capability: &SessionCapability) -> DirectoryResult<Option<AuthSessionRecord>>;
    async fn socket_session_binding(&self, session_id: &str, user_id: &str, authorization_generation: u64, space_id: Option<&str>, now_ms: i64) -> DirectoryResult<SocketSessionBindingStatus>;
    async fn revoke_auth_session(&self, id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Option<RevokedAuthSession>>;
    async fn revoke_auth_sessions_for_user(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>>;
    async fn revoke_auth_sessions_for_user_with_admin_effect(&self, user_id: &str, reason: &str, actor_user_id: Option<&str>, correlation_id: &str, effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<RevokedAuthSession>>;
    async fn revoke_auth_sessions_for_identity(&self, provider: &str, subject_digest: [u8; 32], reason: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<Vec<RevokedAuthSession>>;
    async fn list_auth_audit(&self, limit: usize, offset: usize) -> DirectoryResult<Vec<AuthAuditRecord>>;
}
//#endregion AuthSessionDirectory

//#region CredentialDirectory
/// 🔑️ Credential lifecycle: the sign-in and credential-change audit facts and the password credential itself.
pub trait CredentialDirectory: Send + Sync + 'static {
    /// 🧾️ Appends exactly one credential-lifecycle fact (`credential-sign-in`,
    /// `credential-changed`). Every sign-in attempt — admitted or refused — is journaled through
    /// this, so the authentication log is the record of what happened rather than a mutable
    /// counter beside it.
    async fn append_credential_audit(&self, fact: &CredentialAuditFactV1) -> DirectoryResult<AuthAuditRecord>;
    /// 🔑️ Writes one user's password credential and appends the `credential-changed` fact in the
    /// same transaction, so the projection column and the log can never disagree.
    async fn set_password_credential(&self, user_id: &str, encoded_credential: &str, actor_user_id: Option<&str>, correlation_id: &str) -> DirectoryResult<()>;
}
//#endregion CredentialDirectory

//#region AgentDelegationDirectory
/// 🤖️ Agent delegations: a user's capability to let an agent act in one space. A backend without delegation storage opts out with an empty impl and keeps these defaults.
pub trait AgentDelegationDirectory: Send + Sync + 'static {
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
}
//#endregion AgentDelegationDirectory
