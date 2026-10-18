//! 🥾️ Db engine retained catalog bootstrap operation.

use db_durability::EpochFence;
use db_ids::{DbError, GenerationId};
use db_io_core::CatalogStorage as _;
use semio_framework_async::{Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_engine_base::test_worker_pool;

#[path = "🎟️bootstrap-admission/🦀️.rs"]
mod bootstrap_admission;
use bootstrap_admission::{DatabaseCatalogBootstrapAdmission, DATABASE_CATALOG_BOOTSTRAP_ADMISSION, DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT, DATABASE_CATALOG_BOOTSTRAP_SLOTS};
#[cfg(test)]
use bootstrap_admission::{DatabaseCatalogBootstrapAdmissionState, DATABASE_CATALOG_BOOTSTRAP_PAGES, DATABASE_CATALOG_BOOTSTRAP_TOTAL_BYTES, DATABASE_CATALOG_BOOTSTRAP_TOTAL_ITEMS};

#[path = "🔑️bootstrap-results-and-rejections/🦀️.rs"]
mod bootstrap_results_and_rejections;
pub use bootstrap_results_and_rejections::{
    DatabaseCatalogBootstrapCloseStep, DatabaseCatalogBootstrapKey, DatabaseCatalogBootstrapProgress, DatabaseCatalogBootstrapRejected, DatabaseCatalogBootstrapResult,
    DatabaseCatalogBootstrapTerminalWitness,
};

#[path = "🧱️bootstrap-state-and-wake/🦀️.rs"]
mod bootstrap_state_and_wake;
use bootstrap_state_and_wake::{
    DatabaseCatalogBootstrapDriverAuthority, DatabaseCatalogBootstrapPhase, DatabaseCatalogBootstrapState, DatabaseCatalogBootstrapWake, DatabaseCatalogBootstrapWork,
};
pub use bootstrap_state_and_wake::database_catalog_bootstrap_registry;

#[path = "⚙️bootstrap-state-operations/🦀️.rs"]
mod bootstrap_state_operations;

#[path = "📬️bootstrap-future-and-terminal-handles/🦀️.rs"]
mod bootstrap_future_and_terminal_handles;
pub use bootstrap_future_and_terminal_handles::{
    take_database_catalog_bootstrap_terminal, DatabaseCatalogBootstrapFuture, DatabaseCatalogBootstrapTerminalHandle, DatabaseCatalogBootstrapTerminalResult,
    DATABASE_CATALOG_BOOTSTRAP_LIVE,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
