//! 🧲️ Db I/O kernel core: credit ledger, page arena, task model and lost-owner parking.
#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as pack;

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError, GenerationId};
use db_policy::AdmissionWaiters;
use pack::{ByteRange, ContentHash};
use semio_framework_async::{WorkerDeferredWakeTicket, WorkerMaintenanceTicket, WorkerPool, WorkerPoolUse, WorkerSubmitErrorKind};
use std::{future::Future, pin::Pin, sync::Arc, sync::Mutex, sync::atomic::AtomicU64, sync::atomic::Ordering, task::Context, task::Poll, task::Waker};

#[path = "💳️credit-limits-and-operation-ledger/🦀️.rs"]
mod credit_limits_and_operation_ledger;
pub use credit_limits_and_operation_ledger::{
    db_io_backend_owner_add, db_io_backend_owner_admits, db_io_backend_owner_reserve, db_io_operation_add, db_io_operation_add_result_lease, db_io_operation_attach_task,
    db_io_operation_detach_task, db_io_operation_ledger, db_io_operation_mark_task, db_io_operation_owner_slot, db_io_operation_reserve, db_io_operation_return,
    db_io_operation_return_result_lease, db_io_operation_slot, DbIoCredit, BLOCKING_QUEUE, DB_IO_BACKEND_CONTROLS, DB_IO_LEDGER_ITEMS, DB_IO_LIST_BACKING_BYTES,
    DB_IO_MAX_READ_BYTES, DB_IO_OPERATION_BYTES, DB_IO_OPERATION_ITEMS, DB_IO_OPERATION_PAGES, DB_IO_PAGE_BYTES, DB_IO_PAGE_RETIREMENT_SLOTS, DB_IO_PROCESS_BYTES,
    DB_IO_PROCESS_WITH_WRITER_BACKING_BYTES, DB_IO_TOTAL_PAGES, DB_IO_WRITER_STATIC_BACKING_BYTES,
};
use credit_limits_and_operation_ledger::{db_io_operation_transfer_to_backend, DB_IO_LIST_TRANSIENT_BYTES, DB_IO_PROCESS_CONTROL_CREDIT, DB_IO_TASK_SLOT_BYTES};
#[cfg(any(test,feature="testkit"))]
pub use credit_limits_and_operation_ledger::{db_io_ledger_census, DbIoLedgerOwner, DB_IO_LEDGER_NEXT_OWNER};

#[path = "📃️page-arena-and-lease/🦀️.rs"]
mod page_arena_and_lease;
pub use page_arena_and_lease::{db_io_page_arena, db_io_page_credit, db_io_preflight_page_checkout, DbIoPageArenaState, DbIoPagePhase, EMPTY_DB_IO_PAGE_SLOT};
use page_arena_and_lease::{db_io_checkout_pages, db_io_page_shell_credit, DbIoPageLease};

#[path = "✍️page-writer/🦀️.rs"]
mod page_writer;
pub use page_writer::{DbIoPageWriter, DbIoPageWriterRejected, DbIoPageWriterSeal};

#[path = "🔑️driver-reservation-artifact-id-and-external-bytes/🦀️.rs"]
mod driver_reservation_artifact_id_and_external_bytes;
pub use driver_reservation_artifact_id_and_external_bytes::{DbIoArtifactId, DbIoDriverReservation, DbIoExternalBytes};

#[path = "📚️pages-text-and-u64-list/🦀️.rs"]
mod pages_text_and_u64_list;
pub use pages_text_and_u64_list::{DbIoPageReader, DbIoPages, DbIoText, DbIoU64List, DB_IO_LIST_ITEMS, DB_IO_TEXT_BYTES};

#[path = "📨️tasks-results-and-census/🦀️.rs"]
mod tasks_results_and_census;
pub use tasks_results_and_census::{DbIoBackendControl, DbIoLeaseResult, DbIoResult, DbIoTask, DbIoTaskKind, DbIoTaskPhase, LeaseInfo, WalSegmentState};

#[path = "🚦️executor-driver-and-faults/🦀️.rs"]
mod executor_driver_and_faults;
pub use executor_driver_and_faults::{
    db_io_backend_parts, DbIoAsyncDriverFuture, DbIoAsyncDriverOutput, DbIoAsyncDriverRuntime, DbIoExecutionStep, DbIoExecutorMode, DbIoFault, DbIoFaultCause, DbIoFaultKind,
    DbIoTaskExecutor, DbIoTerminal, DbIoWriterReleaseStep,
};

#[path = "🧺️task-slots-lost-owners-and-admission/🦀️.rs"]
mod task_slots_lost_owners_and_admission;
pub use task_slots_lost_owners_and_admission::{
    db_io_admission_released, db_io_enqueue_close, db_io_error_text, db_io_park_lost_owner, db_io_result_handback, db_io_result_lease_credit, db_io_slot_matches, db_io_task_arena,
    db_io_text_literal, db_io_try_park_lost_owner, DbIoLostOwner, DbIoResultHandback, DbIoResultLease, DbIoTaskHandle, DbIoTaskSlot, DB_IO_ADMISSION_EPOCH,
    DB_IO_ADMISSION_WAITER_TABLE, DB_IO_ADMISSION_WAITING, DB_IO_BACKEND_CREDIT_EXHAUSTED, DB_IO_LOST_OWNERS, DB_IO_LOST_OWNER_OVERFLOW, DB_IO_LOST_OWNER_OVERFLOW_SLOTS,
    DB_IO_LOST_OWNER_QUARANTINE, DB_IO_LOST_OWNER_SLOTS, DB_IO_LOST_OWNER_WAKE_REQUESTED, DB_IO_PROCESS_CREDIT_EXHAUSTED, DB_IO_PROCESS_PAGES_EXHAUSTED,
    DB_IO_RETIREMENT_PRESSURE_FAULT, DB_IO_TASK_ADMISSION_EXHAUSTED, DB_IO_TASK_SLOTS,
};

#[path = "🗃️storage-capabilities-and-traits/🦀️.rs"]
mod storage_capabilities_and_traits;
pub use storage_capabilities_and_traits::{CatalogStorage, IndexStorage, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage};

#[path = "🎫️wal-writer-permit-and-admission/🦀️.rs"]
mod wal_writer_permit_and_admission;
pub use wal_writer_permit_and_admission::{
    capacity_changed, with_admission_line, WalWriterAdmissionCensus, WalWriterAdmissionLine, WalWriterKey, WalWriterPermit, WAL_WRITER_ADMISSION_REFUSAL,
    WAL_WRITER_ADMISSION_WAITERS, WAL_WRITER_CAPACITY,
};
pub(crate) use wal_writer_permit_and_admission::WAL_WRITER_ADMISSION_BACKING_BYTES;

#[path = "📡️wal-writer-signals-release-and-controllers/🦀️.rs"]
mod wal_writer_signals_release_and_controllers;
pub use wal_writer_signals_release_and_controllers::{
    cell, defer_fault_waiters, deferred_wake_pending, fault, fault_all_requested, finish, parked_since, prepare, reclaim, request, request_controller, requested,
    WalWriterController, WalWriterRelease, WalWriterReleaseFailure, WalWriterSignalReservation, WAL_WRITER_CONTROLLERS, WAL_WRITER_SIGNALS,
};
pub(crate) use wal_writer_signals_release_and_controllers::{
    park, request_any_controller, resume, WAL_WRITER_CONTROLLER_BACKING_BYTES, WAL_WRITER_PARK_CLOCK_BACKING_BYTES, WAL_WRITER_SIGNAL_BACKING_BYTES,
};

//#region 🔖️Limits

#[cfg(any(test, feature = "testkit"))]
std::thread_local! {
    pub static DB_IO_LEDGER_OWNER: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

//#endregion 🔖️Limits

#[cfg(all(any(test, feature = "claimed-backend-laws"), not(target_arch = "wasm32")))]
#[path = "../🧪️tests/🔬️claimed-backend/🦀️.rs"]
pub mod claimed_backend;

#[cfg(any(test, feature = "testkit"))]
pub mod testkit {
    pub use super::{credit_limits_and_operation_ledger::*, page_arena_and_lease::*, page_writer::*, driver_reservation_artifact_id_and_external_bytes::*, pages_text_and_u64_list::*, tasks_results_and_census::*, executor_driver_and_faults::*, task_slots_lost_owners_and_admission::*, storage_capabilities_and_traits::*, wal_writer_permit_and_admission::*, wal_writer_signals_release_and_controllers::*, claimed_backend::*};
}
