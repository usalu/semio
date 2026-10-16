//! 🗄️ `db_storage` — the pluggable storage substrate seam for the `db` crate family: the trait
//! family (`WalStorage`, `SnapshotStorage`, `PayloadStorage`, `CatalogStorage`,
//! `IndexStorage`, `LeaseStorage`) every backend (this crate's `MemoryStorage`/`FsStorage`, plus
//! the sibling `db_storage_sqlite`/`db_storage_postgres`/`db_storage_neo4j` modules) implements
//! identically, so `db_engine`/the `db` facade select a backend via [`DbBackend`] at
//! `Database::open` rather than at compile time. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! 🎯️ Design choice: this crate has no opinion on WAL record framing (`db_wal` reuses
//! `protocol::{SprWriter, FrameCursor, recover}` on top of `WalStorage`'s raw segment bytes) or
//! snapshot pack-file structure (`db_snapshot` builds `.spk` bytes handed to `SnapshotStorage`
//! whole) — every trait here stores/retrieves opaque byte blobs keyed by document + a small
//! integer, plus the two primitives (`CatalogStorage::cas_root`, `LeaseStorage`) that need
//! fencing semantics of their own. This keeps the trait family stable while the format built on
//! top of it (owned by `db_wal`/`db_snapshot`/`db_index`) can evolve independently.
//!
//! ⏳️ **Async-first, zero-dyn (ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet
//! `db-dedyn`, ruling **O1**)**: every sub-trait method is a plain `async fn` returning
//! `Result<T, DbError>` directly — never a boxed `dyn Future` in trait-method return position
//! (ruling **R1**; that shape, `DbFuture<'a, T> = Pin<Box<dyn Future<..> + 'a>>`, was this family's
//! PREVIOUS state and is now deleted). Because `async fn` in a trait is not `dyn`-compatible, the
//! old `Arc<dyn DbStorage>`/`&dyn WalStorage` seams are gone too: [`DbBackend`] is a concrete enum
//! naming every backend (this crate has exactly one layer, so no layering problem — see
//! `//#region 🔖️DbBackend`), and its accessors return concrete facet-ref enums
//! ([`WalRef`]/[`SnapshotRef`]/[`PayloadRef`]/[`CatalogRef`]/[`IndexRef`]/[`LeaseRef`]) instead of
//! trait objects. `Send` on a spawned future is therefore obtained STRUCTURALLY, from the concrete
//! enum variant the compiler already knows at every call site — never from a `+ Send` bound on a
//! trait method (ruling **R3**). `#![allow(async_fn_in_trait)]` at the crate root suppresses the
//! resulting "auto trait bounds" lint (ruling **R7**): that lint's suggested fix
//! (`-> impl Future<..> + Send`) is exactly the bound R3 forbids.
//!
//! Backends split two ways: genuinely-async drivers (`db_storage_postgres`'s `sqlx`,
//! `db_storage_neo4j`'s `neo4rs`) `.await` their already-async bodies **on the bounded runtime their
//! driver is bound to**, reached through the [`DbIoAsyncDriverRuntime`] interface and never polled on
//! a pool worker (a pool worker has no foreign runtime context — see that trait's doc); genuinely-blocking
//! backends (this crate's own `FsStorage`, plus the sibling `db_storage_sqlite`) cross the
//! sync/async boundary through schema-first [`DbIoTask`] owners submitted to the ONE
//! process-wide `semio_framework_async::WorkerPool` on
//! `Lane::Io` — never a private `tokio::runtime` (this crate names no `tokio` at all; see the
//! repo's "`tokio` only in `🛎️services`" rule) and never `HostAsyncRuntime::run_blocking` (removed
//! from that trait — Phase 1 of `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`; callers now submit
//! directly to a `WorkerPool`). `MemoryStorage` (no real I/O) simply resolves immediately.
//! `FsStorage`/`SqliteStorage` construction requires that process pool; no pool-less or inline
//! blocking fallback exists, including for CLI and batch entry points.
//!
//! 🧊️ `FsStorage` (this crate's zero-touch default, behind the default `fs` feature) is native-only
//! (`std::fs`) and `#[cfg(not(target_arch = "wasm32"))]`-gated, mirroring `pack`'s own `pack_io`
//! convention — it compiles to an effectively-empty module on a `wasm32-unknown-unknown` target
//! check. `MemoryStorage` has no such gate and is always available.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError};
use db_io_core::{CatalogStorage, DbIoPages, DbIoU64List, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage, WalWriterPermit};
#[cfg(not(target_arch = "wasm32"))]
use db_storage_fs::FsStorage;
use db_storage_memory::MemoryStorage;
use pack::{ByteRange, ContentHash};
use std::{sync::Arc, sync::Mutex, sync::atomic::AtomicU64, sync::atomic::Ordering};
#[cfg(any(test, feature = "testkit"))]
use db_io_core::{DB_IO_LEDGER_OWNER, DB_IO_PAGE_BYTES, DB_IO_TASK_SLOTS, DbIoBackendControl, DbIoCredit, DbIoLedgerOwner, DbIoPageWriter, db_io_backend_parts, db_io_ledger_census};
#[cfg(all(any(test, feature = "testkit"), not(target_arch = "wasm32")))]
use db_io_core::claimed_backend;
#[cfg(any(test, feature = "testkit"))]
use db_io_driver::{DbStorageOpenRejected, WalWriterGuard as _, block_on_ready, db_io_maintenance_step, db_io_test_pool, poll_once, process_isolated_law};
#[cfg(any(test, feature = "testkit"))]
use db_io_registry::{db_io_backend_control, db_io_backend_registry, db_io_copy_pages, db_io_page_maintenance_step};
#[cfg(any(test, feature = "testkit"))]
use db_storage_memory::lock;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_async::{Lane, WorkerPool};
#[cfg(any(test, feature = "testkit"))]
use std::{future::Future, process::Command, task::Context, task::Waker};

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
            // 🔀️ `FaultStorage::read_root` calls back through `self.inner: DbBackend`, so this
            // arm is mutually recursive with this very fn — `Box::pin` breaks the otherwise-
            // infinitely-sized future (E0733), same as `DbBackend::capabilities`'s `Fault` arm.
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

//#region 🔖️FaultStorage
#[cfg(any(test, feature = "testkit"))]
/// 💥️ One knob of `FaultStorage`'s injectable fault script — every field independently
/// optional/off by default, so a freshly `FaultScript::default()`ed `FaultStorage` behaves exactly
/// like its inner backend.
#[derive(Clone, Copy, Default, Debug)]
pub struct FaultScript {
    /// 🚫️ The Nth call to `WalStorage::append` (1-indexed, counting EVERY append this
    /// storage sees, including a document's own genesis header write) fails outright with
    /// `DbError::Io` before reaching the inner backend — models a crash where the physical write
    /// never lands at all.
    pub fail_nth_write: Option<u64>,
    /// ✂️ `(nth, keep_bytes)`: the Nth `append` call forwards only its first `keep_bytes`
    /// bytes to the inner backend (still succeeding, reporting the inner backend's real new
    /// length) — models a crash mid-`write(2)`, the "torn write" fault `db_wal`'s own recovery
    /// (`WalRecoveryReport.torn_tail_bytes`) exists to detect and truncate.
    pub torn_write_at: Option<(u64, u64)>,
    /// 🚫️ The Nth call to `WalStorage::sync` (1-indexed) fails before reaching the
    /// inner backend, after any preceding append has already completed.
    pub fail_nth_sync: Option<u64>,
    /// 🤥️ `WalStorage::sync` returns `Ok(())` without ever forwarding to the inner backend —
    /// models a storage device that acknowledges `fsync` without actually forcing data to physical
    /// storage (a caller relying on `DurabilityClass::Fsync` alone, without independently verifying
    /// durability, cannot tell the difference from the return value).
    pub fsync_lies: bool,
    /// ⚔️ The Nth call to `CatalogStorage::cas_root` (1-indexed) fails with a synthetic
    /// `DbError::Fenced` (as if a competing writer had already advanced the epoch) without ever
    /// touching the inner backend — models exercising a caller's CAS-retry path without needing a
    /// second real writer.
    pub cas_conflict_nth: Option<u64>,
}

#[cfg(any(test, feature = "testkit"))]
/// 💥️ Wraps a real `db_storage::DbStorage` backend with a scriptable `FaultScript`, injected
/// only at the WAL append/sync and catalog-CAS boundaries (see `FaultScript`'s fields) — every
/// other operation (segment lifecycle, snapshot/payload/index/lease storage) passes straight
/// through to `inner` untouched, since those are outside this testkit's stated crash-simulation
/// scope (fail-nth-write/sync / torn write / fsync-lie / CAS-conflict injection).
pub struct FaultStorage {
    pub inner: Arc<DbBackend>,
    pub script: Mutex<FaultScript>,
    pub append_calls: AtomicU64,
    pub sync_calls: AtomicU64,
    sync_delegated_calls: AtomicU64,
    cas_calls: AtomicU64,
}

#[cfg(any(test, feature = "testkit"))]
impl FaultStorage {
    pub async fn new(inner: Arc<DbBackend>) -> FaultStorage {
        FaultStorage { inner, script: Mutex::new(FaultScript::default()), append_calls: AtomicU64::new(0), sync_calls: AtomicU64::new(0), sync_delegated_calls: AtomicU64::new(0), cas_calls: AtomicU64::new(0) }
    }

    pub async fn set_script(&self, script: FaultScript) {
        *self.script.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = script;
    }

    pub async fn script(&self) -> FaultScript {
        *self.script.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// 🔢️ Every `WalStorage::append` call this storage has seen so far, faulted or not —
    /// `CrashHarness` uses this to discover exactly how many write boundaries a workload has.
    pub async fn append_calls(&self) -> u64 {
        self.append_calls.load(Ordering::SeqCst)
    }

    /// 🔢️ Every `WalStorage::sync` call this storage has seen, faulted or not.
    pub async fn sync_calls(&self) -> u64 {
        self.sync_calls.load(Ordering::SeqCst)
    }

    /// 🔢️ How many `WalStorage::sync` calls actually reached the inner backend (as opposed
    /// to being lied about under `FaultScript::fsync_lies`).
    pub async fn sync_delegated_calls(&self) -> u64 {
        self.sync_delegated_calls.load(Ordering::SeqCst)
    }

    pub async fn cas_calls(&self) -> u64 {
        self.cas_calls.load(Ordering::SeqCst)
    }

    /// 🎚️ Passes the inner backend's own capabilities straight through — fault injection
    /// never changes what the backend claims to support, only what it actually does on a call.
    pub async fn capabilities(&self) -> StorageCapabilities {
        self.inner.capabilities().await
    }

    /// 🔚️ Faults never script a close; it closes the wrapped backend.
    pub async fn close(&self) -> Result<(), DbError> {
        self.inner.close().await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl WalStorage for FaultStorage {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<db_io_core::WalWriterPermit, DbError> {
        self.inner.wal().await.acquire_writer(document).await
    }
    async fn create_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.wal().await.create_segment(writer, index).await
    }

    async fn append(&self, writer: &db_io_core::WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        let call = self.append_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let script = self.script().await;
        {
            if script.fail_nth_write == Some(call) {
                return Err(DbError::Io(format!("fault_storage: injected failure on wal append #{call}")));
            }
            if let Some((torn_call, keep_bytes)) = script.torn_write_at {
                if torn_call == call {
                    let keep = (keep_bytes as usize).min(bytes.len());
                    let pages = match bytes.try_prefix(keep) {
                        Ok(pages) => pages,
                        Err(_) => return Err(DbError::Internal("fault_storage torn prefix exceeded the retained DB page owner".to_string())),
                    };
                    return self.inner.wal().await.append(writer, index, pages).await;
                }
            }
            self.inner.wal().await.append(writer, index, bytes).await
        }
    }

    async fn sync(&self, writer: &db_io_core::WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        let call = self.sync_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let script = self.script().await;
        if script.fail_nth_sync == Some(call) {
            return Err(DbError::Io(format!("fault_storage: injected failure on wal sync #{call}")));
        }
        if script.fsync_lies {
            return Ok(());
        }
        self.sync_delegated_calls.fetch_add(1, Ordering::SeqCst);
        self.inner.wal().await.sync(writer, index, class).await
    }

    async fn seal(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.wal().await.seal(writer, index).await
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: pack::ByteRange) -> Result<DbIoPages, DbError> {
        self.inner.wal().await.read(document, index, range).await
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        self.inner.wal().await.segment_len(document, index).await
    }

    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        self.inner.wal().await.segment_state(document, index).await
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        self.inner.wal().await.list_segments(document).await
    }

    async fn truncate_tail(&self, writer: &db_io_core::WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        self.inner.wal().await.truncate_tail(writer, index, new_len).await
    }

    async fn delete_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.wal().await.delete_segment(writer, index).await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl SnapshotStorage for FaultStorage {
    fn publication_scope(&self) -> usize {
        self.inner.publication_scope()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        self.inner.snapshot().await.write_generation(document, generation, bytes).await
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        self.inner.snapshot().await.read_generation(document, generation).await
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        self.inner.snapshot().await.latest_generation(document).await
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        self.inner.snapshot().await.list_generations(document).await
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        self.inner.snapshot().await.delete_generation(document, generation).await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl PayloadStorage for FaultStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        self.inner.payload().await.put(bytes).await
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        self.inner.payload().await.get(hash).await
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        self.inner.payload().await.contains(hash).await
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        self.inner.payload().await.delete(hash).await
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        self.inner.payload().await.len(hash).await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl CatalogStorage for FaultStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        self.inner.catalog().await.read_root().await
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        let call = self.cas_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let conflict = self.script().await.cas_conflict_nth == Some(call);
        {
            if conflict {
                let current = self.inner.catalog().await.read_root().await?.map_or(EpochFence::INITIAL, |(_, fence)| fence);
                return Err(DbError::Fenced { expected: current.epoch, actual: expected.epoch });
            }
            self.inner.catalog().await.cas_root(expected, new_bytes).await
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
impl IndexStorage for FaultStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        self.inner.index().await.write_run(document, run_id, bytes).await
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        self.inner.index().await.read_run(document, run_id).await
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        self.inner.index().await.list_runs(document).await
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        self.inner.index().await.delete_run(document, run_id).await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl LeaseStorage for FaultStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        self.inner.lease().await.acquire(resource, holder, ttl_ms, now_ms).await
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        self.inner.lease().await.renew(resource, holder, fence, ttl_ms, now_ms).await
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        self.inner.lease().await.release(resource, holder, fence).await
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        self.inner.lease().await.current(resource, now_ms).await
    }
}
//#endregion 🔖️FaultStorage

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs"]
mod db_io_retained_fixtures;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️fence-conformance/🦀️.rs"]
mod fence_conformance;
#[cfg(test)]
#[path = "🧪️tests/🧯️fault-storage-laws/🦀️.rs"]
mod fault_storage_laws_tests;
//#endregion 🧪️Tests
