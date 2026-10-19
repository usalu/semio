//! 📇️ Db I/O kernel backend registry, close and retirement turns.
#![allow(async_fn_in_trait)]

use db_io_core::{DbIoFaultCustody, db_io_fault_custody_reserve};

use db_ids::{ArtifactId, DbError, GenerationId};
use db_io_core::{BLOCKING_QUEUE, DB_IO_BACKEND_CONTROLS, DB_IO_LOST_OWNERS, DB_IO_LOST_OWNER_OVERFLOW, DB_IO_LOST_OWNER_QUARANTINE, DB_IO_LOST_OWNER_SLOTS, DB_IO_LOST_OWNER_WAKE_REQUESTED, DB_IO_MAX_READ_BYTES, DB_IO_OPERATION_ITEMS, DB_IO_OPERATION_PAGES, DB_IO_PAGE_BYTES, DB_IO_PAGE_RETIREMENT_SLOTS, DB_IO_RETIREMENT_PRESSURE_FAULT, DB_IO_TASK_SLOTS, DB_IO_TOTAL_PAGES, DbIoBackendControl, DbIoCredit, DbIoDriverReservation, DbIoExecutionStep, DbIoExecutorMode, DbIoExternalBytes, DbIoFault, DbIoFaultCause, DbIoFaultKind, DbIoLostOwner, DbIoPagePhase, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoTaskHandle, DbIoTaskKind, DbIoTaskPhase, DbIoTaskSlot, DbIoTerminal, DbIoText, DbIoU64List, DbIoWriterReleaseStep, EMPTY_DB_IO_PAGE_SLOT, PayloadStorage as _, WAL_WRITER_CONTROLLERS, WAL_WRITER_SIGNALS, WalWriterController, cell, db_io_admission_released, db_io_backend_owner_reserve, db_io_backend_parts, db_io_error_text, db_io_operation_detach_task, db_io_operation_return, db_io_page_arena, db_io_page_credit, db_io_result_handback, db_io_slot_matches, db_io_task_arena, db_io_text_literal, defer_fault_waiters, fault_all_requested, request, request_controller};
use pack::ContentHash;
use semio_framework_async::{Lane, WorkerMaintenanceRequest, WorkerMaintenanceStep, WorkerPool, WorkerPoolUse, WorkerSubmitErrorKind};
use std::{future::Future, pin::Pin, sync::Arc, sync::Mutex, sync::atomic::AtomicU64, sync::atomic::Ordering, task::Context, task::Poll, task::Waker};
#[cfg(test)]
use db_io_core::{DB_IO_LEDGER_OWNER, DbIoLedgerOwner, db_io_ledger_census};

#[path = "📋️page-copy-hash-and-observed-writes/🦀️.rs"]
mod page_copy_hash_and_observed_writes;
pub use page_copy_hash_and_observed_writes::{
    db_io_copy_page_owner, db_io_copy_pages, db_io_hash_pages, db_io_transfer_list,
    db_io_write_observed_bytes, db_io_write_observed_bytes_range, DbIoListTransfer,
    DbIoObservedBytesWrite, DbIoPageCopy, DbIoPageHash, DbIoPageOwnerCopy,
};

#[path = "🧱️platform-buffers/🦀️.rs"]
mod platform_buffers;
pub use platform_buffers::{
    db_io_close_platform, db_io_copy_observed_text, db_io_page_maintenance_step,
    db_io_platform_maintenance_step, db_io_prepare_platform, db_io_prepare_platform_slices,
    DbIoBackendKind, DbIoPlatformBuffer, DbIoPlatformClose, DbIoPlatformCopy,
    DB_IO_PLATFORM_BUFFERS,
};

#[path = "📊️census/🦀️.rs"]
mod census;
pub use census::{
    db_io_census, db_io_census_admission_wait, DbIoCensusV1, DB_IO_CENSUS_STEPS, DB_IO_CENSUS_TASKS,
    DB_IO_CENSUS_TURNS,
};

#[path = "🗒️backend-registry-and-rejections/🦀️.rs"]
mod backend_registry_and_rejections;
use backend_registry_and_rejections::{
    db_io_park_rejected_backend, DbIoBackendAdmittedFailure, DbIoBackendRegistrySlot,
};
pub use backend_registry_and_rejections::{
    db_io_backend_registry, db_io_rejected_backend_maintenance_step, db_io_rejected_backends,
    DbIoBackendRegistrationRejected, DbIoBackendRegistrationRejectedOwner,
    DbIoBackendRollbackReservation, DbIoRejectedBackendSlot,
};

#[path = "🎛️backend-registration-and-executor/🦀️.rs"]
mod backend_registration_and_executor;
pub use backend_registration_and_executor::{
    db_io_backend_close_step, db_io_backend_control, db_io_backend_credit,
    db_io_backend_return_operation, db_io_executor_execute, db_io_executor_mode,
    db_io_request_backend_close, db_io_return_async_executor, db_io_writer_release_lane_step,
    register_db_io_backend, register_db_io_backend_reserved_with_use,
};
use backend_registration_and_executor::db_io_executor_close_operation;
pub(crate) use backend_registration_and_executor::{
    db_io_backend_retirement_turn, DbIoBackendRetirementTurn,
};

#[path = "🧹️task-faults-and-close/🦀️.rs"]
mod task_faults_and_close;
pub use task_faults_and_close::{
    db_io_cancel_requested, db_io_literal_fault, db_io_lost_owner_close_opportunity,
    db_io_lost_owner_close_step, db_io_maintenance_turn, db_io_task_close_step, db_io_task_fault,
    db_io_wake, DB_IO_BACKEND_CONTROLS_EXHAUSTED, DB_IO_BACKEND_ROLLBACK_EXHAUSTED,
    DB_IO_EXECUTOR_TURN_MICROS, DB_IO_EXECUTOR_TURN_STEPS, DB_IO_MAINTENANCE_CURSOR,
    DB_IO_RETRY_MAINTENANCE_CURSOR,
};
pub(crate) use task_faults_and_close::{
    db_io_lost_owner_maintenance_batch, db_io_task_retirement_batch,
};
#[cfg(test)]
use task_faults_and_close::DB_IO_MAINTENANCE_CLASSES;

#[path = "🎮️writer-controller/🦀️.rs"]
mod writer_controller;
pub use writer_controller::{
    controller_credit, controller_step, defer_fault_notifications, DB_IO_RETRY_DELAY_MS,
    DB_IO_RETRY_LIMIT,
};
pub(crate) use writer_controller::{
    install_controller, notify_terminal, request_retirement, retire_controller,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️db-io-retained-fixtures/🦀️.rs"]
mod db_io_retained_fixtures;
//#endregion 🧪️Tests
