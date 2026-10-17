//! 🚪️ The artifact CAS storage port: put and delete outcomes, the storage trait and its shared-handle forwarding.

use super::*;

/// 🧹️ Physical deletion result; missing orphan bytes are an idempotent success.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactCasDeleteOutcome {
    Deleted,
    Missing,
}

/// ✅️ Collision-checked immutable insertion result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactCasPutOutcome {
    Inserted,
    AlreadyPresent,
}

/// 🗄️ Dedicated artifact CAS port; generic database payload storage never crosses this seam.
pub trait ArtifactChunkCasStorage: Send + Sync {
    /// 🪪 Binds a CAS namespace to one durable directory coordinator identity.
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError>;

    /// ⏩ Monotonically activates a directory fence epoch before staging or deletion.
    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError>;

    /// ➕️ Inserts an exact verified object or confirms byte-for-byte identity on collision.
    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError>;

    /// 📖️ Reads one exact object with a kind-specific admission bound.
    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError>;

    /// 🧹️ Deletes only with an exact fence minted by the directory's immediate reachability recheck.
    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError>;
}

impl<T: ArtifactChunkCasStorage> ArtifactChunkCasStorage for Arc<T> {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        self.as_ref().configure_coordinator(coordinator_id, context).await
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        self.as_ref().advance_physical_epoch(coordinator_id, space_id, epoch, context).await
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        self.as_ref().put_if_absent(key, bytes, context).await
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        self.as_ref().get(key, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        self.as_ref().delete_if_unreferenced(key, fence, context).await
    }
}
