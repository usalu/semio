//! 🗃️ Db in-memory storage backend.
#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as pack;

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError, check_len};
use db_io_core::{CatalogStorage, DB_IO_MAX_READ_BYTES, DB_IO_OPERATION_ITEMS, DB_IO_PAGE_BYTES, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoCredit, DbIoExecutionStep, DbIoLeaseResult, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DbIoWriterReleaseStep, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage, WalWriterPermit, db_io_backend_owner_add, db_io_backend_owner_reserve, db_io_operation_return};
use db_io_driver::{DbStorageOpenRejected, WalWriterGuard as _, close_db_io_backend, open_db_io_backend_admitted, retire_db_io_backend, submit_db_io_task_admitted};
use db_io_registry::{DbIoBackendKind, DbIoBackendRegistrationRejected, DbIoBackendRegistrationRejectedOwner, DbIoBackendRollbackReservation, register_db_io_backend_reserved_with_use};
use pack::{ByteRange, ContentHash};
use semio_framework_async::WorkerPool;
use std::{sync::Arc, sync::Mutex, sync::atomic::AtomicU64, sync::atomic::Ordering, task::Context};
#[cfg(any(test, feature = "testkit"))]
use db_ids::GenerationId;
#[cfg(any(test, feature = "testkit"))]
use db_io_core::{DB_IO_BACKEND_CONTROLS, DB_IO_LEDGER_OWNER, DB_IO_LIST_BACKING_BYTES, DB_IO_PROCESS_BYTES, DB_IO_PROCESS_WITH_WRITER_BACKING_BYTES, DB_IO_TASK_SLOTS, DB_IO_WRITER_STATIC_BACKING_BYTES, DbIoExecutorMode, DbIoFaultKind, DbIoLedgerOwner, DbIoPagePhase, WAL_WRITER_CAPACITY, db_io_backend_owner_admits, db_io_backend_parts, db_io_ledger_census, db_io_operation_ledger, db_io_operation_reserve, db_io_operation_slot, db_io_page_arena, db_io_result_lease_credit, db_io_slot_matches};
#[cfg(any(test, feature = "testkit"))]
use db_io_driver::{DB_IO_ADMISSION_WAIT_MS, block_on_ready, db_io_backend_admission_saturated, db_io_backend_admit_operation, db_io_maintenance_step, db_io_take_async_executor, db_io_test_pool, process_isolated_law, submit_db_io_task};
#[cfg(any(test, feature = "testkit"))]
use db_io_registry::{db_io_backend_close_step, db_io_backend_registry, db_io_backend_return_operation, db_io_page_maintenance_step, db_io_return_async_executor, db_io_writer_release_lane_step, register_db_io_backend};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_async::{Job, Lane, WorkerSubmitErrorKind};
#[cfg(any(test, feature = "testkit"))]
use std::{pin::Pin, task::Poll, task::Waker};

#[path = "🧩️memory-executor-types/🦀️.rs"]
mod memory_executor_types;
use memory_executor_types::{
    memory_wal_segment_credit, MemWalSegment, MemoryDbIoCursor, MemoryDbIoExecutor, MemoryLeaseOwner, MemoryPageOwner, MemoryPayloadOwner, MemoryWalOwner, DB_IO_MEMORY_OWNERS,
};

#[path = "🧰️memory-executor/🦀️.rs"]
mod memory_executor;
pub use memory_executor::lock;
use memory_executor::memory_cursor_index;

#[path = "📨️memory-task-executor/🦀️.rs"]
mod memory_task_executor;

#[path = "🪣️memory-storage-backend/🦀️.rs"]
mod memory_storage_backend;
pub use memory_storage_backend::MemoryStorage;
#[cfg(test)]
use memory_storage_backend::memory_document;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs"]
mod db_io_retained_fixtures;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
