//! 🏭️ The capability trait of artifact creation.

use super::*;

//#region ArtifactCreationDirectory
/// 🏭️ Artifact creation: the claim, fact and genesis log of one creation request and its recovery.
pub trait ArtifactCreationDirectory: Send + Sync + 'static {
    async fn claim_artifact_creation(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1>;
    async fn read_artifact_creation(&self, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>>;
    async fn append_artifact_creation_fact(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1>;
    async fn append_document_genesis(&self, append: &DocumentGenesisAppendV1) -> DirectoryResult<DocumentGenesisCommitV1>;
    async fn artifact_creation_recovery_candidates(&self, now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>>;
    async fn artifact_creation_terminate_uncommitted(&self, intent: &ArtifactCreationIntentV1, now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1>;
}
//#endregion ArtifactCreationDirectory
