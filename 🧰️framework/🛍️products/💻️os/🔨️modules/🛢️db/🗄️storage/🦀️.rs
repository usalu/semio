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

#[cfg(test)]
#[path = "../../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
pub(crate) mod test_allocation;
#[cfg(test)]
#[global_allocator]
static REQUESTED_ALLOCATOR: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

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

#[path = "🧭️db-backend/🦀️.rs"]
mod db_backend;
pub use db_backend::DbBackend;

#[path = "📼️wal-and-snapshot-refs/🦀️.rs"]
mod wal_and_snapshot_refs;
pub use wal_and_snapshot_refs::{SnapshotRef, WalRef};

#[path = "🗂️payload-catalog-index-and-lease-refs/🦀️.rs"]
mod payload_catalog_index_and_lease_refs;
pub use payload_catalog_index_and_lease_refs::{CatalogRef, IndexRef, LeaseRef, PayloadRef};

#[cfg(any(test, feature = "testkit"))]
#[path = "🧯️fault-storage/🦀️.rs"]
mod fault_storage;
#[cfg(any(test,feature="testkit"))]
pub use fault_storage::{FaultScript, FaultStorage};

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
