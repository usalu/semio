//! 🗄️ Db facade: the stable `db` surface the hub consumes, re-exported explicitly from the crates that define it.
#![cfg(not(target_arch = "wasm32"))]
extern crate semio_framework_os_kernel as pack;

pub use semio_framework_async;
pub use db_durability::{DurabilityClass};
pub use db_engine::{Database, DatabaseShutdownControl};
pub use db_engine_base::{CheckpointPublicationSnapshot};
pub use db_engine_handle::{ArtifactHandle};
pub use db_ids::{ArtifactId, DbError};
pub use db_observe::{NullEmit};
pub use db_policy::{DbConfig, Profile};
pub use pack::{ContentHash};

pub mod document {
    pub use db_artifact_contract::{ArtifactDurableGroupRecoveredCheckpointV1, ArtifactDurableGroupRecoveryOwnerV1, CommandBatch, DB_PATHMAP_SCHEMA, SubmitOptions, encode_pathmap_json};
    pub use db_artifact_engine::{ArtifactLedgerPoint, artifact_ledger_tail};
}

pub mod engine {
    pub use db_engine::DatabaseOpenAtRejected;
    pub use db_engine_base::Frontier;
}

pub mod security {
    pub use db_security::{Action, BudgetRegistry, Grant, Principal, ReplayGuard, RoleBasedPolicy, SecurityGate, TenantId};
}

pub mod storage {
    pub use db_io_core::{DB_IO_OPERATION_PAGES, DB_IO_PAGE_BYTES, DbIoPageWriter, DbIoPages, DbIoTaskKind, PayloadStorage, WalStorage, claimed_backend};
    pub use db_io_driver::DbStorageOpenRejected;
    pub use db_io_registry::{db_io_census, db_io_copy_pages};
    pub use db_storage::{DbBackend, WalRef};
    pub use db_storage_memory::MemoryStorage;
}

pub mod storage_neo4j {
    pub use db_storage_neo4j::Neo4jStorage;
}

pub mod storage_postgres {
    pub use db_storage_postgres::PostgresStorage;
}

pub mod storage_sqlite {
    pub use db_storage_sqlite::SqliteStorage;
}

pub mod sync {
    pub use db_sync::handle_frontier_advertise;
}

pub mod wal {
    pub use db_wal_log::{WAL_EVENT, WAL_SEGMENT_HEADER, WAL_TX_ABORT, WAL_TX_BEGIN, WAL_TX_COMMIT, WalCursorControl, WalRecord, WalReplayCursor, WalReplayStep};
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
