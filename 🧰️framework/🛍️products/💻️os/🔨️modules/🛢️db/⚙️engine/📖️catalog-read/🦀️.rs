//! 📖️ Db engine retained catalog root read operation.

use db_durability::EpochFence;
use db_ids::DbError;
use db_io_core::CatalogStorage as _;
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_engine_base::test_worker_pool;

#[path = "🎟️read-admission/🦀️.rs"]
mod read_admission;
use read_admission::{
    DatabaseCatalogReadAdmission, DATABASE_CATALOG_READ_ACTIVE, DATABASE_CATALOG_READ_BYTES, DATABASE_CATALOG_READ_CLOSED, DATABASE_CATALOG_READ_QUEUED,
    DATABASE_CATALOG_READ_RETRY_LIMIT, DATABASE_CATALOG_READ_SLOTS,
};
#[cfg(test)]
use read_admission::{DatabaseCatalogReadAdmissionState, DATABASE_CATALOG_READ_ADMISSION, DATABASE_CATALOG_READ_ITEMS};

#[path = "🔑️read-results-and-rejections/🦀️.rs"]
mod read_results_and_rejections;
pub use read_results_and_rejections::{DatabaseCatalogReadCloseStep, DatabaseCatalogReadProgress, DatabaseCatalogReadRejected, DatabaseCatalogReadResult, DatabaseCatalogRootKey};

#[path = "🧱️read-state-and-lease/🦀️.rs"]
mod read_state_and_lease;
use read_state_and_lease::{DatabaseCatalogReadLease, DatabaseCatalogReadPhase, DatabaseCatalogReadState, DatabaseCatalogReadWake, DatabaseCatalogReadWork};
pub use read_state_and_lease::database_catalog_read_registry;

#[path = "⚙️read-state-operations/🦀️.rs"]
mod read_state_operations;

#[path = "📬️read-future-and-terminal-handles/🦀️.rs"]
mod read_future_and_terminal_handles;
pub use read_future_and_terminal_handles::{
    take_database_catalog_read_terminal, DatabaseCatalogReadFuture, DatabaseCatalogReadTerminalHandle, DatabaseCatalogReadTerminalResult, DATABASE_CATALOG_READ_LIVE,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
