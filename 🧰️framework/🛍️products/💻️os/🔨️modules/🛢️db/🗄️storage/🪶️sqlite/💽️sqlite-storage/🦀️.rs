//! 💽️ Sqlite storage module: the schema and connection, the executor, the task executor and the storage backend of the sqlite storage, with the shared imports of the former inline module. See [`super`].

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError, check_len};
use db_io_driver::{WalFileWriterGuard, WalWriterGuard, WalWriterTable};
use db_io_core::{CatalogStorage, DB_IO_PAGE_BYTES, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoExecutionStep, DbIoLeaseResult, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, DbIoWriterReleaseStep, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage, WalWriterPermit};
use db_io_driver::{DbStorageOpenRejected, close_db_io_backend, register_db_io_backend_prepared_with_use, retire_db_io_backend, submit_db_io_task_admitted};
use db_io_registry::{DbIoBackendKind, DbIoBackendRollbackReservation};
use pack::{ByteRange, ContentHash};
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use semio_framework_async::WorkerPool;
use std::sync::{Arc, Mutex};

#[path = "🧬️schema-and-connection/🦀️.rs"]
mod schema_and_connection;
use schema_and_connection::{
    decode_wal_segment_state, init_connection, sqlite_err, to_sql_i64, SqliteDbIoExecutor, SqliteWalWriterGuard, MAX_BLOB_BYTES, SQLITE_OPERATION_OWNERS, SQLITE_READERS,
    SQLITE_READER_CACHE_KIB, STAGE_APPEND_SQL, WAL_APPEND_STAGE_SQL,
};

#[path = "🧰️sqlite-executor/🦀️.rs"]
mod sqlite_executor;

#[path = "📨️sqlite-task-executor/🦀️.rs"]
mod sqlite_task_executor;

#[path = "🪶️sqlite-storage-backend/🦀️.rs"]
mod sqlite_storage_backend;
pub use sqlite_storage_backend::SqliteStorage;

//#region 🧪️Tests
#[cfg(test)]
include!("../🧪️tests/🔬️sqlite-storage-unit/🦀️.rs");
//#endregion 🧪️Tests
