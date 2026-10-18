//! 🗄️ `db_storage_postgres` — a `db_storage::DbStorage` backend over PostgreSQL (via `sqlx`),
//! informed by the deleted `os-semio_hub-storage-postgres` crate's connection/schema-bootstrap shape but
//! implementing the generic, semio_hub-agnostic `db_storage` trait family rather than semio_hub-specific
//! tables. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`).
//!
//! ⏳️ **Async-first (design ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet W6)**:
//! every `db_storage` sub-trait method is a plain `async fn` — `sqlx`'s Postgres driver
//! is async-only (Postgres has no blocking C client to wrap, unlike sqlite), and this backend used
//! to bridge that onto the family's then-synchronous trait signatures by owning a dedicated
//! multi-thread `tokio::runtime::Runtime` and `block_on`-ing every call. The `block_on` bridge is
//! GONE and every method body here is the SAME already-async `sqlx` code, handed straight back as
//! `Box::pin(async move { .. })`, so this backend parks no thread per call.
//!
//! 🧵 **Who polls it.** Not the caller's own executor — that claim was wrong and fatal. A
//! `PostgresStorage` call becomes a `DbIoTask` submitted to the process `WorkerPool`, and an
//! async-native backend's future is driven from `Lane::Io`, whose workers are plain `std::thread`s
//! with no `tokio` context at all. `sqlx` needs that context at **poll** time, so the hub aborted
//! with `this functionality requires a Tokio context` on the first Postgres document call (ticket
//! `26/09/18`, D4). This backend therefore names the bounded runtime `db_storage_driver_runtime`
//! owns through [`DbIoTaskExecutor::driver_runtime`], and `Lane::Io` only ever polls the repo-owned
//! rendezvous that runtime hands back. `sqlx`'s `runtime-tokio` feature selects ITS internal
//! executor binding at ITS compile time; the runtime this backend is polled on is the one named
//! here, and there is exactly one per process.
//!
//! 🐘️ On-disk shape: six tables (`db_wal_segment`, `db_snapshot_generation`, `db_payload`,
//! `db_catalog_root`, `db_index_run`, `db_lease`), bootstrapped idempotently on `connect`. Every
//! value column is a raw `BYTEA` blob — this crate never parses WAL/snapshot/index bytes, mirroring
//! `db_storage`'s own "opaque byte blobs" design note. Compare-and-swap (`CatalogStorage::cas_root`,
//! `LeaseStorage`) is real cross-connection fencing via `SELECT ... FOR UPDATE` inside a
//! transaction — stronger than `db_storage::FsStorage`'s documented in-process-only mutex, since
//! Postgres gives us a genuine row lock across concurrent connections/processes for free.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

//#region 🔖️Connection
use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError, check_len};
use db_io_core::{CatalogStorage, DB_IO_LIST_ITEMS, DB_IO_PAGE_BYTES, DbIoArtifactId, DbIoAsyncDriverFuture, DbIoAsyncDriverRuntime, DbIoBackendControl, DbIoDriverReservation, DbIoExecutionStep, DbIoExecutorMode, DbIoLeaseResult, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage};
use db_io_driver::{DbStorageOpenRejected, close_db_io_backend, register_db_io_backend_prepared_with_use, retire_db_io_backend, submit_db_io_task_admitted};
use db_io_registry::{DbIoBackendKind, DbIoBackendRollbackReservation, db_io_close_platform, db_io_copy_observed_text, db_io_hash_pages, db_io_prepare_platform, db_io_transfer_list, db_io_write_observed_bytes, register_db_io_backend};

macro_rules! with_admitted_artifact {
    ($operation:expr, $document:expr, $artifact:ident, $call:expr) => {{
        let mut owner = DbIoArtifactId::try_from_text($operation, $document)?;
        let $artifact = owner.as_artifact()?;
        let terminal = $call.await;
        while owner.close_step()? {
            semio_framework_async::yield_once().await;
        }
        terminal
    }};
}
use db_io_driver::{WalWriterGuard, WalWriterTable};
use db_io_core::DbIoWriterReleaseStep;
use pack::{ByteRange, ContentHash};
use semio_framework_async::WorkerPool;
use sqlx::postgres::{PgConnection, PgPool, PgPoolOptions};
use sqlx::{ConnectOptions, Connection};
use std::sync::Arc;

#[path = "🗃️schema-and-connection/🦀️.rs"]
mod schema_and_connection;
#[cfg(test)]
use schema_and_connection::SCHEMA_STATEMENTS;
use schema_and_connection::{bootstrap_schema, postgres_wal_segment_state, PostgresDbIoExecutor, POSTGRES_WAL_STATE_QUERY};

#[path = "🧯️error-mapping-and-conversions/🦀️.rs"]
mod error_mapping_and_conversions;
use error_mapping_and_conversions::{map_create_error, map_sqlx_error, postgres_ascending_ids, to_i64, validate_read_range, validate_truncate};

#[path = "🔐️wal-writer-fence/🦀️.rs"]
mod wal_writer_fence;
#[cfg(test)]
use wal_writer_fence::{wal_writer_lock_key, WAL_WRITER_APPLICATION_NAME, WAL_WRITER_KEEPALIVE, WAL_WRITER_LOCK_NAMESPACE};
use wal_writer_fence::{map_session_error, PostgresWalWriterGuard, PostgresWalWriterSession};

#[path = "💾️executor-storage-operations/🦀️.rs"]
mod executor_storage_operations;

#[path = "🤝️executor-lease-storage/🦀️.rs"]
mod executor_lease_storage;
#[cfg(test)]
use executor_lease_storage::{lease_acquire_decision, lease_release_check, lease_renew_check, ExistingLease};

#[path = "🧭️typed-executor/🦀️.rs"]
mod typed_executor;

#[path = "🐘️postgres-storage-backend/🦀️.rs"]
mod postgres_storage_backend;
pub use postgres_storage_backend::PostgresStorage;
#[cfg(test)]
use postgres_storage_backend::postgres_capabilities;

use db_io_core::DB_IO_MAX_READ_BYTES;

//#endregion 🔖️Connection

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️tests/🔬️round-trips/🦀️.rs"]
mod round_trips;
//#endregion 🧪️Tests
