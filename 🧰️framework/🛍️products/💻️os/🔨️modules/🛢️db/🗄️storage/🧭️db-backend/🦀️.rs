//! 🧭️ Db backend: the concrete backend enum naming every storage backend and its facet accessors. See [`super`].

use super::*;

//#region 🔖️DbBackend
/// 🧰️ The umbrella storage substrate handle `db_engine`/the `db` facade hold as
/// `Arc<DbBackend>` (selected at `Database::open`, never compile-time-only per the contract).
/// Replaces the old `Arc<dyn DbStorage>` seam per ruling **O1** (drop dyn dispatch): every
/// former `&dyn WalStorage`/etc. accessor becomes a concrete facet-ref enum
/// ([`WalRef`]/[`SnapshotRef`]/[`PayloadRef`]/[`CatalogRef`]/[`IndexRef`]/[`LeaseRef`]) instead —
/// `Send`-ness is derived STRUCTURALLY at every spawn site (ruling **R3**) — never via a `+ Send`
/// bound on a trait method. No generic `R: HostAsyncRuntime` parameter (Phase 1 of
/// `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR` deleted it): `Fs`/`Sqlite`/`Fault` never used `R`
/// for anything but the removed `run_blocking` bridge, now [`FsStorage`]/`SqliteStorage`'s own
/// strong process `Arc<WorkerPool>` field.
pub enum DbBackend {
    Memory(MemoryStorage),
    #[cfg(not(target_arch = "wasm32"))]
    Fs(FsStorage),
    Sqlite(db_storage_sqlite::SqliteStorage),
    Postgres(db_storage_postgres::PostgresStorage),
    Neo4j(db_storage_neo4j::Neo4jStorage),
    #[cfg(test)]
    Fault(Box<crate::FaultStorage>),
}

impl DbBackend {
    /// 🔀️ This backend's [`WalRef`] facet — replaces the old `&dyn WalStorage`.
    pub async fn wal(&self) -> WalRef<'_> {
        match self {
            Self::Memory(s) => WalRef::Memory(s),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => WalRef::Fs(s),
            Self::Sqlite(s) => WalRef::Sqlite(s),
            Self::Postgres(s) => WalRef::Postgres(s),
            Self::Neo4j(s) => WalRef::Neo4j(s),
            #[cfg(test)]
            Self::Fault(s) => WalRef::Fault(&**s),
        }
    }

    /// 🔀️ This backend's [`SnapshotRef`] facet — replaces the old `&dyn SnapshotStorage`.
    /// 🪪️ The publication scope of this backend's snapshot facet, available without a facet.
    pub fn publication_scope(&self) -> usize {
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

    pub async fn snapshot(&self) -> SnapshotRef<'_> {
        match self {
            Self::Memory(s) => SnapshotRef::Memory(s),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => SnapshotRef::Fs(s),
            Self::Sqlite(s) => SnapshotRef::Sqlite(s),
            Self::Postgres(s) => SnapshotRef::Postgres(s),
            Self::Neo4j(s) => SnapshotRef::Neo4j(s),
            #[cfg(test)]
            Self::Fault(s) => SnapshotRef::Fault(&**s),
        }
    }

    /// 🔀️ This backend's [`PayloadRef`] facet — replaces the old `&dyn PayloadStorage`.
    pub async fn payload(&self) -> PayloadRef<'_> {
        match self {
            Self::Memory(s) => PayloadRef::Memory(s),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => PayloadRef::Fs(s),
            Self::Sqlite(s) => PayloadRef::Sqlite(s),
            Self::Postgres(s) => PayloadRef::Postgres(s),
            Self::Neo4j(s) => PayloadRef::Neo4j(s),
            #[cfg(test)]
            Self::Fault(s) => PayloadRef::Fault(&**s),
        }
    }

    /// 🔀️ This backend's [`CatalogRef`] facet — replaces the old `&dyn CatalogStorage`.
    pub async fn catalog(&self) -> CatalogRef<'_> {
        match self {
            Self::Memory(s) => CatalogRef::Memory(s),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => CatalogRef::Fs(s),
            Self::Sqlite(s) => CatalogRef::Sqlite(s),
            Self::Postgres(s) => CatalogRef::Postgres(s),
            Self::Neo4j(s) => CatalogRef::Neo4j(s),
            #[cfg(test)]
            Self::Fault(s) => CatalogRef::Fault(&**s),
        }
    }

    /// 🔀️ This backend's [`IndexRef`] facet — replaces the old `&dyn IndexStorage`.
    pub async fn index(&self) -> IndexRef<'_> {
        match self {
            Self::Memory(s) => IndexRef::Memory(s),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => IndexRef::Fs(s),
            Self::Sqlite(s) => IndexRef::Sqlite(s),
            Self::Postgres(s) => IndexRef::Postgres(s),
            Self::Neo4j(s) => IndexRef::Neo4j(s),
            #[cfg(test)]
            Self::Fault(s) => IndexRef::Fault(&**s),
        }
    }

    /// 🔀️ This backend's [`LeaseRef`] facet — replaces the old `&dyn LeaseStorage`.
    pub async fn lease(&self) -> LeaseRef<'_> {
        match self {
            Self::Memory(s) => LeaseRef::Memory(s),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => LeaseRef::Fs(s),
            Self::Sqlite(s) => LeaseRef::Sqlite(s),
            Self::Postgres(s) => LeaseRef::Postgres(s),
            Self::Neo4j(s) => LeaseRef::Neo4j(s),
            #[cfg(test)]
            Self::Fault(s) => LeaseRef::Fault(&**s),
        }
    }

    /// 🎚️ What this concrete backend actually supports.
    pub async fn capabilities(&self) -> StorageCapabilities {
        match self {
            Self::Memory(s) => s.capabilities().await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.capabilities().await,
            Self::Sqlite(s) => s.capabilities().await,
            Self::Postgres(s) => s.capabilities().await,
            Self::Neo4j(s) => s.capabilities().await,
            // 🔀️ `FaultStorage::capabilities` calls back through `self.inner: DbBackend`
            // (fault-storage tests), so this arm is mutually recursive with this very fn — `Box::pin`
            // breaks the otherwise-infinitely-sized future (E0733).
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.capabilities()).await,
        }
    }

    /// 🔚️ Closes the concrete backend: every retained WAL writer is released on the backend
    /// itself (Postgres advisory unlock, Neo4j lease release, SQLite/filesystem writer table) and the
    /// driver is torn down before this resolves, so a process that awaits it before exiting leaves no
    /// cross-process writer fence behind. Contract: `🔐️writer/🧬️schema/🔣️.json#/$defs/WalWriterFenceV1`.
    pub async fn close(&self) -> Result<(), DbError> {
        match self {
            Self::Memory(s) => s.close().await,
            #[cfg(not(target_arch = "wasm32"))]
            Self::Fs(s) => s.close().await,
            Self::Sqlite(s) => s.close().await,
            Self::Postgres(s) => s.close().await,
            Self::Neo4j(s) => s.close().await,
            #[cfg(test)]
            Self::Fault(s) => Box::pin(s.close()).await,
        }
    }
}
//#endregion 🔖️DbBackend
