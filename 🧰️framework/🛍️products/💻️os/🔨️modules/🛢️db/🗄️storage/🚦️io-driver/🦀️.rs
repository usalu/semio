//! 🚦️ Db I/O kernel driver: task submission, drive loop, WAL writer table and the storage traits.
#![allow(async_fn_in_trait)]

use db_ids::{DbError, GenerationId};
use db_io_core::{BLOCKING_QUEUE, DB_IO_ADMISSION_EPOCH, DB_IO_ADMISSION_WAITER_TABLE, DB_IO_ADMISSION_WAITING, DB_IO_BACKEND_CREDIT_EXHAUSTED, DB_IO_OPERATION_ITEMS, DB_IO_PROCESS_CREDIT_EXHAUSTED, DB_IO_PROCESS_PAGES_EXHAUSTED, DB_IO_RETIREMENT_PRESSURE_FAULT, DB_IO_TASK_ADMISSION_EXHAUSTED, DB_IO_TASK_SLOTS, DbIoAsyncDriverRuntime as _, DbIoBackendControl, DbIoCredit, DbIoExecutionStep, DbIoExecutorMode, DbIoFault, DbIoFaultCause, DbIoFaultKind, DbIoPagePhase, DbIoResult, DbIoResultHandback, DbIoResultLease, DbIoTask, DbIoTaskExecutor, DbIoTaskHandle, DbIoTaskKind, DbIoTaskPhase, DbIoTaskSlot, DbIoTerminal, DbIoText, DbIoWriterReleaseStep, PayloadStorage as _, WAL_WRITER_ADMISSION_REFUSAL, WAL_WRITER_CAPACITY, WAL_WRITER_CONTROLLERS, WalStorage as _, WalWriterAdmissionCensus, WalWriterAdmissionLine, WalWriterKey, WalWriterPermit, capacity_changed, cell, db_io_backend_owner_reserve, db_io_backend_parts, db_io_enqueue_close, db_io_operation_add, db_io_operation_add_result_lease, db_io_operation_attach_task, db_io_operation_detach_task, db_io_operation_mark_task, db_io_operation_reserve, db_io_operation_return, db_io_operation_return_result_lease, db_io_slot_matches, db_io_task_arena, with_admission_line};
use db_io_registry::{DB_IO_BACKEND_CONTROLS_EXHAUSTED, DB_IO_BACKEND_ROLLBACK_EXHAUSTED, DB_IO_CENSUS_STEPS, DB_IO_CENSUS_TASKS, DB_IO_CENSUS_TURNS, DB_IO_EXECUTOR_TURN_MICROS, DB_IO_EXECUTOR_TURN_STEPS, DB_IO_MAINTENANCE_CURSOR, DB_IO_RETRY_DELAY_MS, DB_IO_RETRY_LIMIT, DB_IO_RETRY_MAINTENANCE_CURSOR, DbIoBackendKind, DbIoBackendRegistrationRejected, DbIoBackendRegistrationRejectedOwner, DbIoBackendRollbackReservation, controller_step, db_io_backend_control, db_io_backend_credit, db_io_backend_registry, db_io_backend_return_operation, db_io_cancel_requested, db_io_census_admission_wait, db_io_executor_execute, db_io_executor_mode, db_io_literal_fault, db_io_lost_owner_close_step, db_io_maintenance_turn, db_io_page_maintenance_step, db_io_platform_maintenance_step, db_io_rejected_backend_maintenance_step, db_io_request_backend_close, db_io_return_async_executor, db_io_task_close_step, db_io_task_fault, db_io_wake, register_db_io_backend_reserved_with_use};
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Job, Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, pin::Pin, sync::Arc, sync::atomic::Ordering, task::Context, task::Poll, task::Waker};
#[cfg(any(test, feature = "testkit"))]
use db_durability::EpochFence;
#[cfg(any(test, feature = "testkit"))]
use db_io_core::{DB_IO_BACKEND_CONTROLS, DB_IO_LEDGER_OWNER, DB_IO_LOST_OWNERS, DB_IO_LOST_OWNER_OVERFLOW, DB_IO_LOST_OWNER_OVERFLOW_SLOTS, DB_IO_LOST_OWNER_QUARANTINE, DB_IO_LOST_OWNER_SLOTS, DB_IO_OPERATION_BYTES, DB_IO_OPERATION_PAGES, DB_IO_PAGE_BYTES, DB_IO_TOTAL_PAGES, DbIoArtifactId, DbIoAsyncDriverFuture, DbIoDriverReservation, DbIoLeaseResult, DbIoLedgerOwner, DbIoLostOwner, DbIoPageArenaState, DbIoPageWriter, DbIoPages, LeaseStorage as _, WAL_WRITER_ADMISSION_WAITERS, db_io_ledger_census, db_io_page_arena, db_io_park_lost_owner, db_io_preflight_page_checkout, db_io_try_park_lost_owner, deferred_wake_pending};
#[cfg(any(test, feature = "testkit"))]
use db_io_registry::{DB_IO_PLATFORM_BUFFERS, DbIoPlatformBuffer, DbIoRejectedBackendSlot, db_io_backend_close_step, db_io_close_platform, db_io_lost_owner_close_opportunity, db_io_prepare_platform, db_io_rejected_backends, db_io_write_observed_bytes, register_db_io_backend};
#[cfg(any(test, feature = "testkit"))]
use pack::{ByteRange, ContentHash};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_async::WorkerSubmitErrorKind;
#[cfg(any(test, feature = "testkit"))]
use std::{process::Command, sync::Mutex};

#[path = "🔓️open-rejection-and-backend-lifecycle/🦀️.rs"]
mod open_rejection_and_backend_lifecycle;
pub use open_rejection_and_backend_lifecycle::{
    close_db_io_backend, db_io_backend_admit_operation, db_io_take_async_executor,
    register_db_io_backend_prepared_with_use, retire_db_io_backend, DbStorageOpenRejected,
};
use open_rejection_and_backend_lifecycle::db_io_take_backend_cleanup_fault;

#[path = "🏃️task-allocation-and-async-driver/🦀️.rs"]
mod task_allocation_and_async_driver;
use task_allocation_and_async_driver::{
    db_io_allocate_task, db_io_drive_one, db_io_request_async_driver_poll, db_io_retry,
    db_io_retry_maintenance_step, db_io_submit_job,
};

#[path = "🎟️task-operation-lease-and-admission/🦀️.rs"]
mod task_operation_lease_and_admission;
pub use task_operation_lease_and_admission::{
    db_io_backend_admission_saturated, open_db_io_backend_admitted, submit_db_io_task,
    submit_db_io_task_admitted, DbIoAsyncTaskLease, DbIoTaskOperation, DB_IO_ADMISSION_WAIT_MS,
};
use task_operation_lease_and_admission::{
    db_io_async_lease_credit, DB_IO_ASYNC_NATIVE_OPERATION_BUSY, DB_IO_BACKEND_PENDING_OPERATIONS,
    DB_IO_TASK_CAPACITY_EXHAUSTED,
};

#[path = "🔁️task-operation-futures/🦀️.rs"]
mod task_operation_futures;
#[cfg(test)]
use task_operation_futures::db_io_wait_task_retirement;
pub use task_operation_futures::{db_io_maintenance_step, db_io_test_pool};
#[cfg(any(test,feature="testkit"))]
pub use task_operation_futures::{block_on_ready, poll_once, process_isolated_law};

#[path = "📝️wal-writer-table/🦀️.rs"]
mod wal_writer_table;
pub use wal_writer_table::{WalWriterGuard, WalWriterTable};
#[cfg(not(target_arch="wasm32"))]
pub use wal_writer_table::WalFileWriterGuard;

#[path = "🛂️wal-writer-admission/🦀️.rs"]
mod wal_writer_admission;
pub use wal_writer_admission::{admission_census, admitted_acquire, WAL_WRITER_ADMISSION_WAIT_MS};
pub(crate) use wal_writer_admission::{controller_pool, faulted, retirement_waker};
#[cfg(any(test,feature="testkit"))]
pub use wal_writer_admission::{
    deferred_wake_pending_for_test, restore_controller_after_refusal,
    suspend_controller_for_refusal,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs"]
mod db_io_retained_fixtures;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
