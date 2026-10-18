use super::*;
use semio_framework_test_fixtures::fixture_text;

use db_io_core::{DB_IO_BACKEND_CONTROLS, DB_IO_LEDGER_OWNER, DB_IO_LOST_OWNERS, DB_IO_LOST_OWNER_OVERFLOW, DB_IO_LOST_OWNER_OVERFLOW_SLOTS, DB_IO_LOST_OWNER_QUARANTINE, DB_IO_LOST_OWNER_SLOTS, DB_IO_OPERATION_BYTES, DB_IO_OPERATION_ITEMS, DB_IO_OPERATION_PAGES, DB_IO_PAGE_BYTES, DB_IO_RETIREMENT_PRESSURE_FAULT, DB_IO_TASK_SLOTS, DB_IO_TOTAL_PAGES, DbIoArtifactId, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoCredit, DbIoDriverReservation, DbIoExecutionStep, DbIoExecutorMode, DbIoFault, DbIoFaultCause, DbIoFaultKind, DbIoLeaseResult, DbIoLedgerOwner, DbIoLostOwner, DbIoPageArenaState, DbIoPagePhase, DbIoPageWriter, DbIoPages, DbIoResult, DbIoResultHandback, DbIoResultLease, DbIoTask, DbIoTaskExecutor, DbIoTaskPhase, DbIoText, LeaseStorage as _, PayloadStorage as _, db_io_backend_owner_reserve, db_io_backend_parts, db_io_ledger_census, db_io_operation_reserve, db_io_operation_return, db_io_page_arena, db_io_park_lost_owner, db_io_preflight_page_checkout, db_io_slot_matches, db_io_try_park_lost_owner};
use db_io_registry::{DB_IO_PLATFORM_BUFFERS, DB_IO_RETRY_DELAY_MS, DB_IO_RETRY_LIMIT, DB_IO_RETRY_MAINTENANCE_CURSOR, DbIoBackendKind, DbIoBackendRollbackReservation, DbIoPlatformBuffer, DbIoRejectedBackendSlot, db_io_backend_close_step, db_io_backend_registry, db_io_close_platform, db_io_literal_fault, db_io_lost_owner_close_opportunity, db_io_lost_owner_close_step, db_io_page_maintenance_step, db_io_platform_maintenance_step, db_io_prepare_platform, db_io_rejected_backend_maintenance_step, db_io_rejected_backends, db_io_task_fault, db_io_write_observed_bytes, register_db_io_backend};

include!("🧪️law-executors-and-fault-fixtures/🦀️.rs");
include!("🚧️blocking-executors-and-fixture-helpers/🦀️.rs");
include!("📏️fixed-page-output-and-lease-laws/🦀️.rs");
include!("🪤️lost-owner-ring-close-interruption-and-queued-callback/🦀️.rs");
include!("🔁️retry-driver-and-backend-controls/🦀️.rs");
include!("🧯️pressure-slots-and-lost-backend-laws/🦀️.rs");
include!("🚨️registration-cancellation-drop-and-cleanup-laws/🦀️.rs");
