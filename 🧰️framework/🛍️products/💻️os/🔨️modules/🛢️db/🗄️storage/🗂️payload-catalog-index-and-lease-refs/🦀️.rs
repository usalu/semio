//! 🗂️ Payload, catalog, index and lease refs: the payload, catalog, index and lease storage references with their storage trait implementations. See [`super`].

use super::*;

//#region 🔖️PayloadRef
/// 🔀️ [`DbBackend::payload`]'s return shape — the enum that replaces
/// `&dyn PayloadStorage` per ruling **O1**.
pub enum PayloadRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(not(target_arch = "wasm32"))]
    Fs(&'a FsStorage),
    Sqlite(&'a db_storage_sqlite::SqliteStorage),
    Postgres(&'a db_storage_postgres::PostgresStorage),
    Neo4j(&'a db_storage_neo4j::Neo4jStorage),
    #[cfg(test)]
    Fault(&'a crate::FaultStorage),
}

impl<'a> PayloadStorage for PayloadRef<'a> {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match self {
            Self::Memory(s) => s.put(bytes).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.put(bytes).await,
            Self::Sqlite(s) => s.put(bytes).await,
            Self::Postgres(s) => s.put(bytes).await,
            Self::Neo4j(s) => s.put(bytes).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.put(bytes)).await,
        }
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.get(hash).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.get(hash).await,
            Self::Sqlite(s) => s.get(hash).await,
            Self::Postgres(s) => s.get(hash).await,
            Self::Neo4j(s) => s.get(hash).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.get(hash)).await,
        }
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match self {
            Self::Memory(s) => s.contains(hash).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.contains(hash).await,
            Self::Sqlite(s) => s.contains(hash).await,
            Self::Postgres(s) => s.contains(hash).await,
            Self::Neo4j(s) => s.contains(hash).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.contains(hash)).await,
        }
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete(hash).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.delete(hash).await,
            Self::Sqlite(s) => s.delete(hash).await,
            Self::Postgres(s) => s.delete(hash).await,
            Self::Neo4j(s) => s.delete(hash).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.delete(hash)).await,
        }
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match self {
            Self::Memory(s) => s.len(hash).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.len(hash).await,
            Self::Sqlite(s) => s.len(hash).await,
            Self::Postgres(s) => s.len(hash).await,
            Self::Neo4j(s) => s.len(hash).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.len(hash)).await,
        }
    }
}
//#endregion 🔖️PayloadRef

//#region 🔖️CatalogRef
/// 🔀️ [`DbBackend::catalog`]'s return shape — the enum that replaces
/// `&dyn CatalogStorage` per ruling **O1**.
pub enum CatalogRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(not(target_arch = "wasm32"))]
    Fs(&'a FsStorage),
    Sqlite(&'a db_storage_sqlite::SqliteStorage),
    Postgres(&'a db_storage_postgres::PostgresStorage),
    Neo4j(&'a db_storage_neo4j::Neo4jStorage),
    #[cfg(test)]
    Fault(&'a crate::FaultStorage),
}

impl<'a> CatalogStorage for CatalogRef<'a> {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match self {
            Self::Memory(s) => s.read_root().await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.read_root().await,
            Self::Sqlite(s) => s.read_root().await,
            Self::Postgres(s) => s.read_root().await,
            Self::Neo4j(s) => s.read_root().await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.read_root()).await,
        }
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match self {
            Self::Memory(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.cas_root(expected, new_bytes).await,
            Self::Sqlite(s) => s.cas_root(expected, new_bytes).await,
            Self::Postgres(s) => s.cas_root(expected, new_bytes).await,
            Self::Neo4j(s) => s.cas_root(expected, new_bytes).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.cas_root(expected, new_bytes)).await,
        }
    }
}
//#endregion 🔖️CatalogRef

//#region 🔖️IndexRef
/// 🔀️ [`DbBackend::index`]'s return shape — the enum that replaces
/// `&dyn IndexStorage` per ruling **O1**.
pub enum IndexRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(not(target_arch = "wasm32"))]
    Fs(&'a FsStorage),
    Sqlite(&'a db_storage_sqlite::SqliteStorage),
    Postgres(&'a db_storage_postgres::PostgresStorage),
    Neo4j(&'a db_storage_neo4j::Neo4jStorage),
    #[cfg(test)]
    Fault(&'a crate::FaultStorage),
}

impl<'a> IndexStorage for IndexRef<'a> {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.write_run(document, run_id, bytes).await,
            Self::Sqlite(s) => s.write_run(document, run_id, bytes).await,
            Self::Postgres(s) => s.write_run(document, run_id, bytes).await,
            Self::Neo4j(s) => s.write_run(document, run_id, bytes).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.write_run(document, run_id, bytes)).await,
        }
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        match self {
            Self::Memory(s) => s.read_run(document, run_id).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.read_run(document, run_id).await,
            Self::Sqlite(s) => s.read_run(document, run_id).await,
            Self::Postgres(s) => s.read_run(document, run_id).await,
            Self::Neo4j(s) => s.read_run(document, run_id).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.read_run(document, run_id)).await,
        }
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        match self {
            Self::Memory(s) => s.list_runs(document).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.list_runs(document).await,
            Self::Sqlite(s) => s.list_runs(document).await,
            Self::Postgres(s) => s.list_runs(document).await,
            Self::Neo4j(s) => s.list_runs(document).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.list_runs(document)).await,
        }
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.delete_run(document, run_id).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.delete_run(document, run_id).await,
            Self::Sqlite(s) => s.delete_run(document, run_id).await,
            Self::Postgres(s) => s.delete_run(document, run_id).await,
            Self::Neo4j(s) => s.delete_run(document, run_id).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.delete_run(document, run_id)).await,
        }
    }
}
//#endregion 🔖️IndexRef

//#region 🔖️LeaseRef
/// 🔀️ [`DbBackend::lease`]'s return shape — the enum that replaces
/// `&dyn LeaseStorage` per ruling **O1**.
pub enum LeaseRef<'a> {
    Memory(&'a MemoryStorage),
    #[cfg(not(target_arch = "wasm32"))]
    Fs(&'a FsStorage),
    Sqlite(&'a db_storage_sqlite::SqliteStorage),
    Postgres(&'a db_storage_postgres::PostgresStorage),
    Neo4j(&'a db_storage_neo4j::Neo4jStorage),
    #[cfg(test)]
    Fault(&'a crate::FaultStorage),
}

impl<'a> LeaseStorage for LeaseRef<'a> {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match self {
            Self::Memory(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            Self::Sqlite(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            Self::Postgres(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            Self::Neo4j(s) => s.acquire(resource, holder, ttl_ms, now_ms).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.acquire(resource, holder, ttl_ms, now_ms)).await,
        }
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            Self::Sqlite(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            Self::Postgres(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            Self::Neo4j(s) => s.renew(resource, holder, fence, ttl_ms, now_ms).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.renew(resource, holder, fence, ttl_ms, now_ms)).await,
        }
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.release(resource, holder, fence).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.release(resource, holder, fence).await,
            Self::Sqlite(s) => s.release(resource, holder, fence).await,
            Self::Postgres(s) => s.release(resource, holder, fence).await,
            Self::Neo4j(s) => s.release(resource, holder, fence).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.release(resource, holder, fence)).await,
        }
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match self {
            Self::Memory(s) => s.current(resource, now_ms).await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.current(resource, now_ms).await,
            Self::Sqlite(s) => s.current(resource, now_ms).await,
            Self::Postgres(s) => s.current(resource, now_ms).await,
            Self::Neo4j(s) => s.current(resource, now_ms).await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.current(resource, now_ms)).await,
        }
    }
}
//#endregion 🔖️LeaseRef
