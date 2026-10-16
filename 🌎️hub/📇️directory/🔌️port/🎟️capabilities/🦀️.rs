//! 🎟️ Capability traits for share tokens and invitations.

use super::*;

//#region ShareTokenDirectory
/// 🔗️ Document share tokens: issue, revoke and authenticate the capabilities that grant access to one document scope.
pub trait ShareTokenDirectory: Send + Sync + 'static {
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
}
//#endregion ShareTokenDirectory

//#region InviteDirectory
/// 🎟️ Space invitations: issue, redeem and revoke the capabilities that grant membership of one space.
pub trait InviteDirectory: Send + Sync + 'static {
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
}
//#endregion InviteDirectory
