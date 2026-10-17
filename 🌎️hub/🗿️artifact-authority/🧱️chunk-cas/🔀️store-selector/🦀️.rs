//! 🔀️ The runtime-selected artifact CAS enum and its storage port without trait-object futures.

use super::*;

/// 🔀️ Concrete runtime-selected artifact CAS without trait-object futures.
pub enum ArtifactChunkCasStores {
    Memory(MemoryArtifactChunkCasStorage),
    Filesystem(FsArtifactChunkCasStorage),
    Sqlite(SqliteArtifactChunkCasStorage),
    Postgres(PostgresArtifactChunkCasStorage),
    Neo4j(Neo4jArtifactChunkCasStorage),
}

impl ArtifactChunkCasStorage for ArtifactChunkCasStores {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        match self {
            Self::Memory(storage) => storage.configure_coordinator(coordinator_id, context).await,
            Self::Filesystem(storage) => storage.configure_coordinator(coordinator_id, context).await,
            Self::Sqlite(storage) => storage.configure_coordinator(coordinator_id, context).await,
            Self::Postgres(storage) => storage.configure_coordinator(coordinator_id, context).await,
            Self::Neo4j(storage) => storage.configure_coordinator(coordinator_id, context).await,
        }
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        match self {
            Self::Memory(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
            Self::Filesystem(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
            Self::Sqlite(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
            Self::Postgres(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
            Self::Neo4j(storage) => storage.advance_physical_epoch(coordinator_id, space_id, epoch, context).await,
        }
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        match self {
            Self::Memory(storage) => storage.put_if_absent(key, bytes, context).await,
            Self::Filesystem(storage) => storage.put_if_absent(key, bytes, context).await,
            Self::Sqlite(storage) => storage.put_if_absent(key, bytes, context).await,
            Self::Postgres(storage) => storage.put_if_absent(key, bytes, context).await,
            Self::Neo4j(storage) => storage.put_if_absent(key, bytes, context).await,
        }
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        match self {
            Self::Memory(storage) => storage.get(key, context).await,
            Self::Filesystem(storage) => storage.get(key, context).await,
            Self::Sqlite(storage) => storage.get(key, context).await,
            Self::Postgres(storage) => storage.get(key, context).await,
            Self::Neo4j(storage) => storage.get(key, context).await,
        }
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        match self {
            Self::Memory(storage) => storage.delete_if_unreferenced(key, fence, context).await,
            Self::Filesystem(storage) => storage.delete_if_unreferenced(key, fence, context).await,
            Self::Sqlite(storage) => storage.delete_if_unreferenced(key, fence, context).await,
            Self::Postgres(storage) => storage.delete_if_unreferenced(key, fence, context).await,
            Self::Neo4j(storage) => storage.delete_if_unreferenced(key, fence, context).await,
        }
    }
}
