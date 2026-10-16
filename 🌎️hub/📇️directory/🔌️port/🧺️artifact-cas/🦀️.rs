//! 🧺️ The capability trait of the artifact CAS ledger.

use super::*;

//#region ArtifactCasDirectory
/// 🧺️ The artifact CAS ledger: reservations, sweep candidates and delete fences over private reachability.
pub trait ArtifactCasDirectory: Send + Sync + 'static {
    /// 🎫️ Appends one exact expiring private reachability reservation before CAS writes.
    async fn reserve_artifact_cas(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation>;
    /// ⚛️ Consumes one live exact reservation with the public/private checkpoint commit.
    async fn append_reserved_artifact_checkpoint(
        &self,
        event: Option<&NewDirectoryEvent>,
        checkpoint: &ArtifactCheckpoint,
        reservation: &ArtifactCasReservation,
        completion: Option<&CheckpointPublicationCompletionV1>,
        now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>>;
    /// 🧹️ Reads a bounded page of private historical object candidates for sweeping.
    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64>;
    /// 🪪 Reads the durable private identity that binds directory epochs to the selected CAS.
    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]>;
    /// 🧹️ Reads a bounded historical page through one immutable sweep generation.
    async fn artifact_cas_sweep_candidates(&self, after_generation: u64, through_generation: u64, limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage>;
    /// 🔭 Reads exact current reachability for a mutation-free dry-run preview.
    async fn artifact_cas_delete_preview_protected(&self, key: &ArtifactCasObjectKey, observed_generation: u64, now_ms: u64) -> DirectoryResult<bool>;
    /// 🛡️ Atomically acquires a durable per-space deletion lease and rechecks reachability.
    async fn acquire_artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, lease_token: [u8; 32], now_ms: u64, expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>>;
    /// 🛡️ Rechecks the exact live lease, epoch, and reachability after CAS epoch activation.
    async fn validate_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64) -> DirectoryResult<bool>;
    /// 💓 Renews a deletion lease while the physical store operation remains in flight.
    async fn renew_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64, expires_at_ms: u64) -> DirectoryResult<()>;
    /// 🔓 Releases exactly the opaque deletion lease owned by this fence.
    async fn release_artifact_cas_delete_fence(&self, fence: ArtifactCasDeleteFence) -> DirectoryResult<()>;
}
//#endregion ArtifactCasDirectory
