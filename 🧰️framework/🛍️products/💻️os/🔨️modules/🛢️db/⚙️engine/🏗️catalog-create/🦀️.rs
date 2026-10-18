//! 🏗️ Db engine retained create-document catalog operation.

use db_durability::EpochFence;
use db_engine_base::{CatalogEntry, CatalogState, DatabaseCreateCatalogToken, decimal_u64};
use db_ids::{ArtifactId, DbError, GenerationId};
use db_io_core::CatalogStorage as _;
use db_policy::worker_submit_retry_attempt;
use semio_framework_async::{Lane, WorkerPool, WorkerPoolUse};
use std::{future::Future, sync::Arc, sync::Mutex};
#[cfg(test)]
use db_engine_base::{encode_catalog_pages, test_worker_pool};

#[path = "🧮️limits-ledger-and-admission/🦀️.rs"]
mod limits_ledger_and_admission;
use limits_ledger_and_admission::{
    DatabaseCreateCatalogAdmission, DatabaseCreateCatalogBackingLedger,
    DATABASE_CREATE_CATALOG_ADMISSION, DATABASE_CREATE_CATALOG_ARC_CONTROL_BYTES,
    DATABASE_CREATE_CATALOG_COPY_BYTES, DATABASE_CREATE_CATALOG_DEADLINE_MS,
    DATABASE_CREATE_CATALOG_MAX_ENTRIES, DATABASE_CREATE_CATALOG_MAX_ID_BYTES,
    DATABASE_CREATE_CATALOG_MAX_PAGES, DATABASE_CREATE_CATALOG_RETRY_LIMIT,
    DATABASE_CREATE_CATALOG_SLOTS,
};
#[cfg(test)]
use limits_ledger_and_admission::DATABASE_CREATE_CATALOG_ITEMS;

#[path = "📋️progress-result-and-rejection/🦀️.rs"]
mod progress_result_and_rejection;
pub use progress_result_and_rejection::{
    DatabaseCreateCatalogCloseStep, DatabaseCreateCatalogProgress, DatabaseCreateCatalogRejected,
    DatabaseCreateCatalogResult, DatabaseCreateCatalogTerminalWitness,
};
use progress_result_and_rejection::DatabaseCreateCatalogBackendFuture;

#[path = "🧵️work-cursor-phase-and-state/🦀️.rs"]
mod work_cursor_phase_and_state;
use work_cursor_phase_and_state::{
    DatabaseCreateCatalogCursor, DatabaseCreateCatalogDriverAuthority,
    DatabaseCreateCatalogEncodeCursor, DatabaseCreateCatalogPhase, DatabaseCreateCatalogState,
    DatabaseCreateCatalogWake, DatabaseCreateCatalogWork,
};
pub use work_cursor_phase_and_state::database_create_catalog_registry;

#[path = "🚦️state-driver/🦀️.rs"]
mod state_driver;

#[path = "🔎️state-scan-clone-and-encode/🦀️.rs"]
mod state_scan_clone_and_encode;

#[path = "📌️state-claim-revalidate-and-publish/🦀️.rs"]
mod state_claim_revalidate_and_publish;

#[path = "📤️future-terminal-and-state-drop/🦀️.rs"]
mod future_terminal_and_state_drop;
pub use future_terminal_and_state_drop::{
    take_database_create_catalog_terminal, DatabaseCreateCatalogFuture,
    DatabaseCreateCatalogTerminalHandle, DATABASE_CREATE_CATALOG_LIVE,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
