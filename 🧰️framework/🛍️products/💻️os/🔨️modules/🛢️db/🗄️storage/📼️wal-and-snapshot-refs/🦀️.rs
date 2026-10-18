//! 📼️ Wal and snapshot refs: the wal storage reference and the snapshot storage reference with their storage trait implementations. See [`super`].

use super::*;

//#region 🔖️WalRef
/// 🔀️ [`DbBackend::wal`]'s return shape — the enum that replaces
/// `&dyn WalStorage` per ruling **O1**.
pub enum WalRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(not(target_arch = "wasm32"))]
    Fs(&'a FsStorage),
    Sqlite(&'a db_storage_sqlite::SqliteStorage),
    Postgres(&'a db_storage_postgres::PostgresStorage),
    Neo4j(&'a db_storage_neo4j::Neo4jStorage),
    #[cfg(test)]
    Fault(&'a crate::FaultStorage),
}

impl<'a> WalStorage for WalRef<'a> {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<WalWriterPermit, DbError> {
        match self {
            Self::Memory(s) => s.acquire_writer(document).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.acquire_writer(document).await,
            Self::Sqlite(s) => s.acquire_writer(document).await,
            Self::Postgres(s) => s.acquire_writer(document).await,
            Self::Neo4j(s) => s.acquire_writer(document).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.acquire_writer(document)).await,
        }
    }

    async fn create_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.create_segment(writer, index).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.create_segment(writer, index).await,
            Self::Sqlite(s) => s.create_segment(writer, index).await,
            Self::Postgres(s) => s.create_segment(writer, index).await,
            Self::Neo4j(s) => s.create_segment(writer, index).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.create_segment(writer, index)).await,
        }
    }

    async fn append(&self, writer: &WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.append(writer, index, bytes).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.append(writer, index, bytes).await,
            Self::Sqlite(s) => s.append(writer, index, bytes).await,
            Self::Postgres(s) => s.append(writer, index, bytes).await,
            Self::Neo4j(s) => s.append(writer, index, bytes).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.append(writer, index, bytes)).await,
        }
    }

    async fn sync(&self, writer: &WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.sync(writer, index, class).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.sync(writer, index, class).await,
            Self::Sqlite(s) => s.sync(writer, index, class).await,
            Self::Postgres(s) => s.sync(writer, index, class).await,
            Self::Neo4j(s) => s.sync(writer, index, class).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.sync(writer, index, class)).await,
        }
    }

    async fn seal(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.seal(writer, index).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.seal(writer, index).await,
            Self::Sqlite(s) => s.seal(writer, index).await,
            Self::Postgres(s) => s.seal(writer, index).await,
            Self::Neo4j(s) => s.seal(writer, index).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.seal(writer, index)).await,
        }
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.read(document, index, range).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.read(document, index, range).await,
            Self::Sqlite(s) => s.read(document, index, range).await,
            Self::Postgres(s) => s.read(document, index, range).await,
            Self::Neo4j(s) => s.read(document, index, range).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.read(document, index, range)).await,
        }
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.segment_len(document, index).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.segment_len(document, index).await,
            Self::Sqlite(s) => s.segment_len(document, index).await,
            Self::Postgres(s) => s.segment_len(document, index).await,
            Self::Neo4j(s) => s.segment_len(document, index).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.segment_len(document, index)).await,
        }
    }

    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        match self {
            Self::Memory(s) => s.segment_state(document, index).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.segment_state(document, index).await,
            Self::Sqlite(s) => s.segment_state(document, index).await,
            Self::Postgres(s) => s.segment_state(document, index).await,
            Self::Neo4j(s) => s.segment_state(document, index).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.segment_state(document, index)).await,
        }
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match self {
            Self::Memory(s) => s.list_segments(document).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.list_segments(document).await,
            Self::Sqlite(s) => s.list_segments(document).await,
            Self::Postgres(s) => s.list_segments(document).await,
            Self::Neo4j(s) => s.list_segments(document).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.list_segments(document)).await,
        }
    }

    async fn truncate_tail(&self, writer: &WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.truncate_tail(writer, index, new_len).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.truncate_tail(writer, index, new_len).await,
            Self::Sqlite(s) => s.truncate_tail(writer, index, new_len).await,
            Self::Postgres(s) => s.truncate_tail(writer, index, new_len).await,
            Self::Neo4j(s) => s.truncate_tail(writer, index, new_len).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.truncate_tail(writer, index, new_len)).await,
        }
    }

    async fn delete_segment(&self, writer: &WalWriterPermit, index: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_segment(writer, index).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.delete_segment(writer, index).await,
            Self::Sqlite(s) => s.delete_segment(writer, index).await,
            Self::Postgres(s) => s.delete_segment(writer, index).await,
            Self::Neo4j(s) => s.delete_segment(writer, index).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.delete_segment(writer, index)).await,
        }
    }
}
//#endregion 🔖️WalRef

//#region 🔖️SnapshotRef
/// 🔀️ [`DbBackend::snapshot`]'s return shape — the enum that replaces
/// `&dyn SnapshotStorage` per ruling **O1**.
pub enum SnapshotRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(not(target_arch = "wasm32"))]
    Fs(&'a FsStorage),
    Sqlite(&'a db_storage_sqlite::SqliteStorage),
    Postgres(&'a db_storage_postgres::PostgresStorage),
    Neo4j(&'a db_storage_neo4j::Neo4jStorage),
    #[cfg(test)]
    Fault(&'a crate::FaultStorage),
}

impl<'a> SnapshotStorage for SnapshotRef<'a> {
    fn publication_scope(&self) -> usize {
        match self {
            Self::Memory(s) => s.publication_scope(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.publication_scope(),
            Self::Sqlite(s) => s.publication_scope(),
            Self::Postgres(s) => s.publication_scope(),
            Self::Neo4j(s) => s.publication_scope(),
            #[cfg(test)]
            Self::Fault(s) => s.publication_scope(),
        }
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.write_generation(document, generation, bytes).await,
            Self::Sqlite(s) => s.write_generation(document, generation, bytes).await,
            Self::Postgres(s) => s.write_generation(document, generation, bytes).await,
            Self::Neo4j(s) => s.write_generation(document, generation, bytes).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.write_generation(document, generation, bytes)).await,
        }
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.read_generation(document, generation).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.read_generation(document, generation).await,
            Self::Sqlite(s) => s.read_generation(document, generation).await,
            Self::Postgres(s) => s.read_generation(document, generation).await,
            Self::Neo4j(s) => s.read_generation(document, generation).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.read_generation(document, generation)).await,
        }
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match self {
            Self::Memory(s) => s.latest_generation(document).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.latest_generation(document).await,
            Self::Sqlite(s) => s.latest_generation(document).await,
            Self::Postgres(s) => s.latest_generation(document).await,
            Self::Neo4j(s) => s.latest_generation(document).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.latest_generation(document)).await,
        }
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match self {
            Self::Memory(s) => s.list_generations(document).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.list_generations(document).await,
            Self::Sqlite(s) => s.list_generations(document).await,
            Self::Postgres(s) => s.list_generations(document).await,
            Self::Neo4j(s) => s.list_generations(document).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.list_generations(document)).await,
        }
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_generation(document, generation).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.delete_generation(document, generation).await,
            Self::Sqlite(s) => s.delete_generation(document, generation).await,
            Self::Postgres(s) => s.delete_generation(document, generation).await,
            Self::Neo4j(s) => s.delete_generation(document, generation).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.delete_generation(document, generation)).await,
        }
    }
}
//#endregion 🔖️SnapshotRef
