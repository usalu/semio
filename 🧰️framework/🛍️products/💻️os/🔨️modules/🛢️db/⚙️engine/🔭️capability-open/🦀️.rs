//! 🔭️ Db engine retained capability open operation.

use db_ids::DbError;
use db_io_core::CatalogStorage as _;
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_durability::DurabilityClass;
#[cfg(test)]
use db_engine_base::test_worker_pool;

#[path = "🎟️open-admission/🦀️.rs"]
mod open_admission;
use open_admission::{
    DatabaseCapabilityOpenAdmission, DATABASE_CAPABILITY_OPEN_ACTIVE, DATABASE_CAPABILITY_OPEN_BYTES, DATABASE_CAPABILITY_OPEN_CLOSED, DATABASE_CAPABILITY_OPEN_ITEMS,
    DATABASE_CAPABILITY_OPEN_QUEUED, DATABASE_CAPABILITY_OPEN_RETRY_LIMIT, DATABASE_CAPABILITY_OPEN_RETRY_MS, DATABASE_CAPABILITY_OPEN_SLOTS,
};
#[cfg(test)]
use open_admission::{DatabaseCapabilityOpenAdmissionState, DATABASE_CAPABILITY_OPEN_ADMISSION, DATABASE_CAPABILITY_OPEN_TOTAL_BYTES, DATABASE_CAPABILITY_OPEN_TOTAL_ITEMS};

#[path = "🔑️open-results-and-rejections/🦀️.rs"]
mod open_results_and_rejections;
pub use open_results_and_rejections::{DatabaseCapabilityOpenCloseStep, DatabaseCapabilityOpenProgress, DatabaseCapabilityOpenRejected, DatabaseCapabilityOpenResult};

#[path = "🧱️open-state-and-lease/🦀️.rs"]
mod open_state_and_lease;
use open_state_and_lease::{DatabaseCapabilityOpenLease, DatabaseCapabilityOpenPhase, DatabaseCapabilityOpenState, DatabaseCapabilityOpenWake, DatabaseCapabilityOpenWork};
pub use open_state_and_lease::database_capability_open_registry;

#[path = "⚙️open-state-operations/🦀️.rs"]
mod open_state_operations;

#[path = "📬️open-future-and-terminal-handles/🦀️.rs"]
mod open_future_and_terminal_handles;
pub use open_future_and_terminal_handles::{
    take_database_capability_open_terminal, take_next_database_capability_open_terminal, DatabaseCapabilityOpenFuture, DatabaseCapabilityOpenTerminalHandle,
    DatabaseCapabilityOpenTerminalResult, DATABASE_CAPABILITY_OPEN_LIVE,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
