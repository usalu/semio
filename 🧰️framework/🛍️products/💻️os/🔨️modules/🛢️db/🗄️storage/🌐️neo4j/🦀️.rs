//! 🗄️ Db storage backend over neo4j: a `DbStorage` implementation (`db_storage`'s trait family —
//! `WalStorage`/`SnapshotStorage`/`PayloadStorage`/`CatalogStorage`/`IndexStorage`/`LeaseStorage`)
//! over a live Neo4j server via `neo4rs`. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, "storage substrate is pluggable"). Informed by the deleted
//! `os-semio_hub-storage-neo4j` crate's schema conventions (native Bolt byte properties, `MERGE`
//! `ON CREATE`/`ON MATCH` freshness flags for idempotent-create-with-conflict-detection).
//!
//! 🕸️ Schema shape: every trait's records are flat, labeled nodes keyed by their trait-level
//! identity (`document`+numeric index, or a single string key) — never a chained graph. An
//! append-only WAL segment or an immutable snapshot/index run gains nothing from
//! `(:Prev)-[:NEXT]->(:Next)` edges that an indexed property lookup doesn't already give; graph
//! traversal is not this trait family's shape (see `db_storage`'s module doc: every trait here
//! stores/retrieves opaque byte blobs). Byte payloads use Bolt's native byte owner, prepared from
//! the repository fixed platform arena; no whole base64 string or decoded vector exists.
//!
//! ⏳️ **Async-first (design ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet W6)**:
//! `neo4rs` is fully async (`tokio`-native); every `db_storage` sub-trait method here is
//! a plain `async fn`. This backend used to bridge that onto the family's then-synchronous
//! trait signatures by owning a dedicated `tokio::runtime::Runtime` and `block_on`-ing every call —
//! that runtime (and the `block_on` bridge) is GONE: every method body below is the SAME
//! already-async `neo4rs` code that runtime used to drive, now handed straight back as
//! `Box::pin(async move { .. })`. `connect`/`connect_to_database` are async too, for the same
//! reason. This crate names no `tokio` anywhere (the repo's "`tokio` only in `🛎️services`" rule).
//!
//! 💾️ Durability: every write here is its own committed (or txn-committed) Cypher statement — by
//! the time `append`/`write_generation`/`cas_root`/etc. resolves, Neo4j has already durably
//! committed it server-side. There is nothing weaker to fall back to and nothing stronger to force
//! forward, so `WalStorage::sync` is a documented no-op regardless of the requested
//! `DurabilityClass` (see `sync`'s doc below) — unlike a file-backed backend, this one never has an
//! unflushed OS buffer to force out.
//!
//! 🔐️ WAL writer fence: Neo4j has no session-scoped lock, so a document's writer is a
//! `(:WalWriter {document})` lease node (unique per document) carrying the holder, a monotonic
//! fencing `token` and a server-clock expiry. Acquisition claims an absent or expired lease and
//! advances the token; the permit renews it on the driver runtime every `renewEveryMs`; and every
//! WAL mutation runs in one transaction that first write-locks the lease node and proves
//! `holder + token + unexpired` before touching a segment, so a writer whose lease lapsed (paused,
//! partitioned or crashed) is fenced by the storage itself, never by its own belief. A crashed
//! holder's lease frees itself after `leaseTtlMs`. Contract: `🔐️writer/🧫️fixtures/🌐️remote-guard`.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_durability::{DurabilityClass, EpochFence};
use db_ids::{ArtifactId, DbError, check_len};
use db_io_core::{CatalogStorage, DB_IO_PAGE_BYTES, DbIoArtifactId, DbIoAsyncDriverFuture, DbIoAsyncDriverRuntime, DbIoBackendControl, DbIoDriverReservation, DbIoExecutionStep, DbIoExecutorMode, DbIoExternalBytes, DbIoLeaseResult, DbIoPageWriter, DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalSegmentState, WalStorage};
use db_io_driver::{DbStorageOpenRejected, close_db_io_backend, register_db_io_backend_prepared_with_use, retire_db_io_backend, submit_db_io_task_admitted};
use db_io_registry::{DbIoBackendKind, DbIoBackendRollbackReservation, db_io_close_platform, db_io_copy_observed_text, db_io_hash_pages, db_io_prepare_platform, db_io_prepare_platform_slices, db_io_transfer_list, db_io_write_observed_bytes_range, register_db_io_backend};

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
use db_storage_driver_runtime::DbIoDriverPeriodic;
use neo4rs::{query, BoltBytes, Graph, Query, Txn};
use pack::{ByteRange, ContentHash};
use semio_framework_async::WorkerPool;
use std::sync::Arc;

//#region 🔖️Codec
use db_io_core::DB_IO_MAX_READ_BYTES;

#[path = "🧬️codec-laws-and-error-mapping/🦀️.rs"]
mod codec_laws_and_error_mapping;
use codec_laws_and_error_mapping::{
    decide_acquire_fence, i64_to_u64, map_de_error, map_neo4rs_error, slice_range, u64_to_i64,
    validate_release, validate_renew, write_driver_bytes, LeaseRow,
};
#[cfg(test)]
use codec_laws_and_error_mapping::{apply_append, apply_truncate};

#[path = "📜️cypher-statements/🦀️.rs"]
mod cypher_statements;
use cypher_statements::{
    neo4j_wal_segment_state, CYPHER_CATALOG_CAS, CYPHER_CATALOG_READ, CYPHER_INDEX_DELETE,
    CYPHER_INDEX_LIST, CYPHER_INDEX_READ, CYPHER_INDEX_WRITE, CYPHER_LEASE_DELETE,
    CYPHER_LEASE_READ, CYPHER_LEASE_WRITE, CYPHER_PAYLOAD_CONTAINS, CYPHER_PAYLOAD_DELETE,
    CYPHER_PAYLOAD_GET, CYPHER_PAYLOAD_LEN, CYPHER_PAYLOAD_PUT, CYPHER_SNAPSHOT_DELETE,
    CYPHER_SNAPSHOT_LATEST, CYPHER_SNAPSHOT_LIST, CYPHER_SNAPSHOT_READ, CYPHER_SNAPSHOT_WRITE,
    CYPHER_WAL_CREATE_SEGMENT, CYPHER_WAL_DELETE_SEGMENT, CYPHER_WAL_LIST_SEGMENTS,
    CYPHER_WAL_READ_ROW, CYPHER_WAL_SEAL, CYPHER_WAL_STATE, CYPHER_WAL_WRITER_ACQUIRE,
    CYPHER_WAL_WRITER_FENCE, CYPHER_WAL_WRITER_RELEASE, CYPHER_WAL_WRITER_RENEW,
    CYPHER_WAL_WRITE_BYTES, SCHEMA_STATEMENTS,
};

#[path = "🔒️writer-fence-and-executor-core/🦀️.rs"]
mod writer_fence_and_executor_core;
#[cfg(test)]
use writer_fence_and_executor_core::{WAL_WRITER_LEASE_TTL_MS, WAL_WRITER_RENEW_EVERY_MS};
use writer_fence_and_executor_core::{Neo4jDbIoExecutor, Neo4jWalWriterGuard, Neo4jWalWriterLease};

#[path = "🗃️executor-storage-impls/🦀️.rs"]
mod executor_storage_impls;

#[path = "⚙️typed-executor/🦀️.rs"]
mod typed_executor;

#[path = "🕸️neo4j-storage-backend/🦀️.rs"]
mod neo4j_storage_backend;
pub use neo4j_storage_backend::Neo4jStorage;

//#endregion 🔖️Codec

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
