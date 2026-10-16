//! 🏘️ Capability traits for spaces, memberships, document descriptors and artifact checkpoints.

use super::*;

//#region SpaceDirectory
/// 🏘️ Spaces and memberships: the lookups, rosters and administration pages over spaces.
pub trait SpaceDirectory: Send + Sync + 'static {
    /// 🔎️ Single-space lookup by id — used by the hub handler to read `kind`/`visibility`
    /// (grant compilation, public-visibility fallback) without listing every space. Also `decide`'s
    /// (`//#region 🔖️Decider`) own "does this space exist" read.
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>>;
    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>>;
    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>>;
    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>>;
    /// 🏘️ Every space `user_id` may list — each public space and each space it is a member of —
    /// with its role there and the counts of [`SpaceDirectory::list_admin_space_summaries_page`], ordered by
    /// id, in ONE backend round trip. `GET /directory/spaces` used to fold the whole directory event log
    /// and then query every visible space's documents (mounting each document for its frontier) and
    /// sessions one by one: 25–58 s for a member of 82 spaces on hub 7800 (ticket 26/09/23 WG8).
    async fn list_visible_space_summaries(&self, user_id: Option<&str>) -> DirectoryResult<Vec<(AdminSpaceSummaryRecord, Option<SpaceRole>)>>;
    async fn list_admin_space_members_page(&self, space_id: &str, offset: usize, limit: usize) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>>;
    /// 🧑️‍🤝️‍🧑️ The current member roster — `decide` reads this to enforce the atelier/
    /// archive laws and to compute `archive-space`'s demote-every-author events.
    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>>;
    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>>;
    /// 🏛️ One keyset-ordered bounded administration member window (`user_id ASC`), projected to
    /// display columns inside the backend so no credential-bearing record is ever constructed.
    async fn list_space_administration_members_page(&self, space_id: &str, after_user_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationMemberRow>>;
}
//#endregion SpaceDirectory

//#region DocumentDirectory
/// 📄️ Document descriptors and the published artifact checkpoints of a document scope.
pub trait DocumentDirectory: Send + Sync + 'static {
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
}
//#endregion DocumentDirectory

//#region PrincipalAuthority
/// 🧢️ The space authority of one principal: its role ceiling and the role it holds. Implemented for every directory that offers [`SpaceDirectory`] and [`AgentDelegationDirectory`].
pub trait PrincipalAuthority: SpaceDirectory + AgentDelegationDirectory {
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
}

impl<T: SpaceDirectory + AgentDelegationDirectory> PrincipalAuthority for T {}
//#endregion PrincipalAuthority
