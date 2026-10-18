//! 📁️ Db filesystem storage backend.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError, check_len};
use db_io_core::{CatalogStorage, DB_IO_LEDGER_ITEMS, DB_IO_MAX_READ_BYTES, DB_IO_PAGE_BYTES, DB_IO_TEXT_BYTES, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoExecutionStep, DbIoLeaseResult, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DbIoWriterReleaseStep, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage, WalWriterPermit, db_io_operation_owner_slot};
use db_io_driver::{DbStorageOpenRejected, WalWriterGuard as _, close_db_io_backend, open_db_io_backend_admitted, register_db_io_backend_prepared_with_use, retire_db_io_backend, submit_db_io_task_admitted};
use db_io_registry::{DbIoBackendKind, DbIoBackendRollbackReservation};
use pack::{ByteRange, ContentHash};
use semio_framework_async::WorkerPool;
use std::{sync::Arc, sync::Mutex, sync::atomic::Ordering, task::Context};
#[cfg(test)]
use db_io_driver::{block_on_ready, db_io_test_pool, poll_once};
#[cfg(test)]
use std::sync::atomic::AtomicU64;

//#region 🔖️Fs
/// 📁️ The zero-touch default `DbStorage`: pure files under a root directory, no new C
/// dependency (see module doc for the wasm32/native-only gating rationale). Layout under `root`:
/// `wal/<doc>/segment-<index>.bin` (+ `.sealed` marker once sealed), `snapshot/<doc>/gen-<n>.pack`,
/// `payload/<hash[0..2]>/<hash>.bin` (blake3 CAS, two-hex-char sharding), `catalog/root.bin`
/// (8-byte LE epoch prefix + opaque bytes), `index/<doc>/run-<id>.bin`, `lease/<resource>.bin`.
/// Every multi-step write (`cas_root`, lease grants, snapshot/index/payload writes) goes through
/// `pack::write_atomic` (temp file + `fsync` + `rename`) so a reader never observes a torn file.
/// Every blocking filesystem step runs as one typed task grant on the process I/O lane.
#[cfg(not(target_arch = "wasm32"))]
#[path = "💽️fs-storage/🦀️.rs"]
mod fs_storage;

#[cfg(not(target_arch = "wasm32"))]
pub use fs_storage::FsStorage;
//#endregion 🔖️Fs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
