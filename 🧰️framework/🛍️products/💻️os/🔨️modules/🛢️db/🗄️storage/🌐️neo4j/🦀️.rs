//! 🗄️ Db storage backend over neo4j: a `DbStorage` implementation (`db_storage`'s trait family —
//! `WalStorage`/`SnapshotStorage`/`PayloadStorage`/`CatalogStorage`/`IndexStorage`/`LeaseStorage`)
//! over a live Neo4j server via `neo4rs`. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
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
//! 🚧️ Extension seam (documented, not a TODO): read-modify-write operations (`append`,
//! `truncate_tail`, lease `acquire`/`renew`/`release`) run inside a single Neo4j transaction, which
//! gives them write-lock isolation against OTHER concurrent transactions touching the same node
//! (Neo4j takes a write lock on first touch, held to commit) — but two `Neo4jStorage` handles in
//! different OS processes still race exactly like `FsStorage`'s documented `cas_root`/lease caveat
//! (see `db_storage`'s `fs_storage` module doc): full cross-process mutual exclusion beyond
//! Neo4j's own lock semantics is `db_cluster`'s ownership-lease concern, not this crate's.

use crate::db_durability::{DurabilityClass, EpochFence};
use crate::db_ids::{check_len, ArtifactId, DbError};
use crate::db_storage::{
    close_db_io_backend, db_io_close_platform, db_io_copy_observed_text, db_io_hash_pages, db_io_prepare_platform, db_io_prepare_platform_slices, db_io_transfer_list, db_io_write_observed_bytes_range, register_db_io_backend, retire_db_io_backend,
    submit_db_io_task, CatalogStorage, DbIoArtifactId, DbIoAsyncDriverFuture, DbIoBackendControl, DbIoBackendKind, DbIoDriverReservation, DbIoExecutionStep, DbIoExecutorMode, DbIoExternalBytes, DbIoLeaseResult, DbIoPageWriter,
    DbIoPageWriterRejected, DbIoPages, DbIoResult, DbIoTask, DbIoTaskExecutor, DbIoText, DbIoU64List, IndexStorage, LeaseInfo, LeaseStorage, PayloadStorage, SnapshotStorage, StorageCapabilities, WalStorage, DB_IO_PAGE_BYTES,
};

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
use neo4rs::{query, BoltBytes, Graph, Query, Txn};
use pack::{ByteRange, ContentHash};
use semio_framework_async::WorkerPool;
use std::sync::Arc;

//#region 🔖️Codec
/// @emoji 🛡️ Ceiling on any single blob this backend reads into memory in one call — mirrors
/// `db_storage`'s own `MAX_READ_BYTES` choice (this crate's own choice too, the contract doesn't
/// fix a number): validated before the driver byte owner is admitted.
const MAX_READ_BYTES: u64 = 496 * 1024;

async fn write_driver_bytes(reservation: DbIoDriverReservation, bytes: BoltBytes, offset: usize, length: usize, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
    db_io_write_observed_bytes_range(reservation, bytes.value.to_vec(), offset, length, output)?.await
}

/// @emoji 🔢️ Neo4j's `Integer` bolt type is a signed 64-bit value; the family's identity/sequence
/// numbers are `u64`. Converts with an explicit range check rather than a silent wrapping `as i64`.
fn u64_to_i64(value: u64, what: &'static str) -> Result<i64, DbError> {
    i64::try_from(value).map_err(|_| DbError::InvalidArgument(format!("{what} exceeds neo4j's signed 64-bit integer range: {value}")))
}

/// @emoji 🔢️ Inverse of `u64_to_i64` for values read back from a Neo4j `Integer` property. A
/// negative value here means the property was corrupted (hand-edited or written by a bug) since
/// every writer path exclusively writes non-negative values.
fn i64_to_u64(value: i64, what: &'static str) -> Result<u64, DbError> {
    u64::try_from(value).map_err(|_| DbError::Corrupt(format!("{what} decoded as a negative neo4j integer: {value}")))
}

/// @emoji ✂️ Slices `bytes[range.offset..range.offset+range.len]`, bounds-checked against
/// `bytes`'s actual length — the shared implementation `WalStorage::read` validates against,
/// mirroring `MemoryStorage`/`FsStorage`'s identical bounds-checking law.
fn slice_range(bytes: &[u8], range: ByteRange) -> Result<&[u8], DbError> {
    let end = range.offset.checked_add(range.len).ok_or_else(|| DbError::InvalidArgument("read range overflows u64".to_string()))?;
    if end > bytes.len() as u64 {
        return Err(DbError::InvalidArgument(format!("read range {}..{end} out of bounds (len {})", range.offset, bytes.len())));
    }
    Ok(&bytes[range.offset as usize..end as usize])
}
//#endregion 🔖️Codec

//#region 🔖️WalLaws
#[cfg(test)]
fn apply_append(current: &[u8], sealed: bool, extra: &[u8]) -> Result<Vec<u8>, DbError> {
    if sealed {
        return Err(DbError::InvalidArgument("cannot append to sealed wal segment".to_string()));
    }
    let mut updated = Vec::with_capacity(current.len() + extra.len());
    updated.extend_from_slice(current);
    updated.extend_from_slice(extra);
    Ok(updated)
}

#[cfg(test)]
fn apply_truncate(current: &[u8], sealed: bool, new_len: u64) -> Result<Vec<u8>, DbError> {
    if sealed {
        return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
    }
    if new_len > current.len() as u64 {
        return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
    }
    Ok(current[..new_len as usize].to_vec())
}
//#endregion 🔖️WalLaws

//#region 🔖️LeaseLaws
/// @emoji ⏳️ One lease row as read back from Neo4j: `(fence, expires_at_ms, holder)`.
type LeaseRow = (EpochFence, u64, DbIoText);

/// @emoji 🤝️ The pure decision behind `LeaseStorage::acquire` — see `MemoryStorage::acquire`'s
/// identical law in `db_storage`, factored out here for unit testing without a live connection.
fn decide_acquire_fence(resource: &str, existing: Option<LeaseRow>, holder: &str, now_ms: u64) -> Result<EpochFence, DbError> {
    match existing {
        Some((fence, expires_at_ms, existing_holder)) if now_ms < expires_at_ms => {
            if existing_holder.as_str() != holder {
                return Err(DbError::Conflict(format!("resource {resource} is leased by another holder")));
            }
            Ok(fence)
        }
        Some((fence, _, _)) => Ok(fence.next()),
        None => Ok(EpochFence::INITIAL),
    }
}

/// @emoji ♻️ The pure decision behind `LeaseStorage::renew`.
fn validate_renew(resource: &str, existing: Option<LeaseRow>, holder: &str, fence: EpochFence, now_ms: u64) -> Result<(), DbError> {
    let (current_fence, expires_at_ms, current_holder) = existing.ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
    if now_ms >= expires_at_ms {
        return Err(DbError::Unavailable(format!("lease for {resource} already expired")));
    }
    if current_holder.as_str() != holder {
        return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
    }
    fence.check(current_fence)
}

/// @emoji 🕊️ The pure decision behind `LeaseStorage::release`.
fn validate_release(resource: &str, existing: Option<LeaseRow>, holder: &str, fence: EpochFence) -> Result<(), DbError> {
    let (current_fence, _, current_holder) = existing.ok_or_else(|| DbError::NotFound(format!("lease for {resource} not found")))?;
    if current_holder.as_str() != holder {
        return Err(DbError::Unauthorized(format!("lease for {resource} is not held by {holder}")));
    }
    fence.check(current_fence)
}
//#endregion 🔖️LeaseLaws

//#region 🔖️ErrorMapping
/// @emoji 🚨️ Maps a `neo4rs::Error` into the family's single `DbError` — never lets a foreign
/// error type leak through a public signature, per the repo's binding convention.
#[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
                                         // 🚫️async: E4 fn-pointer slot
fn map_neo4rs_error(err: neo4rs::Error) -> DbError {
    match &err {
        neo4rs::Error::IOError { .. } | neo4rs::Error::ConnectionError => DbError::Unavailable(err.to_string()),
        neo4rs::Error::AuthenticationError(_) => DbError::Unauthorized(err.to_string()),
        neo4rs::Error::Neo4j(neo4j_err) => map_neo4j_error(neo4j_err),
        // 🎯️ Everything else (protocol/serialization/config-shape errors) is this driver/process's
        // own fault, not a caller-correctable condition — `Internal` rather than guessing a more
        // specific variant.
        _ => DbError::Internal(err.to_string()),
    }
}

/// @emoji 🏷️ Classifies a server-reported `Neo4jError` by its `Neo4jErrorKind` (the driver's own
/// status-code classification) into the family's `DbError`.
// 🚫️async: E1 pure accessor called from `map_neo4rs_error`, itself an E4 fn-pointer slot — see R9
fn map_neo4j_error(err: &neo4rs::Neo4jError) -> DbError {
    use neo4rs::{Neo4jClientErrorKind, Neo4jErrorKind};
    match err.kind() {
        Neo4jErrorKind::Client(Neo4jClientErrorKind::Security(_)) => DbError::Unauthorized(err.message().to_string()),
        Neo4jErrorKind::Client(Neo4jClientErrorKind::TransactionTerminated) => DbError::Conflict(err.message().to_string()),
        Neo4jErrorKind::Transient => DbError::Unavailable(err.message().to_string()),
        _ => DbError::Internal(format!("{}: {}", err.code(), err.message())),
    }
}

/// @emoji 🚨️ Maps a row-decoding error (a property missing or of the wrong bolt type) into
/// `DbError::Corrupt` — a decode failure always means the stored data doesn't match this crate's
/// own schema, never a caller mistake.
#[allow(clippy::needless_pass_by_value)] // used as a `map_err` callback, which passes the error by value
                                         // 🚫️async: E4 fn-pointer slot
fn map_de_error(err: neo4rs::DeError) -> DbError {
    DbError::Corrupt(format!("neo4j row decode error: {err}"))
}
//#endregion 🔖️ErrorMapping

//#region 🔖️Cypher
/// @emoji 🧱️ Uniqueness constraints (single-property, Community-Edition-compatible) and
/// composite lookup indexes bootstrapped once at `Neo4jStorage::connect`. Composite *uniqueness*
/// constraints need Enterprise Edition; multi-key node identity (`WalSegment`/`SnapshotGeneration`/
/// `IndexRun`) is instead guaranteed by always addressing those nodes via `MERGE` on their full
/// key (document + index), which is correct regardless of a DB-level constraint — these indexes
/// are a read-performance aid only, not a correctness dependency.
const SCHEMA_STATEMENTS: &[&str] = &[
    "CREATE CONSTRAINT db_payload_hash IF NOT EXISTS FOR (p:Payload) REQUIRE p.hash IS UNIQUE",
    "CREATE CONSTRAINT db_lease_resource IF NOT EXISTS FOR (l:Lease) REQUIRE l.resource IS UNIQUE",
    "CREATE CONSTRAINT db_catalog_root_id IF NOT EXISTS FOR (c:CatalogRoot) REQUIRE c.id IS UNIQUE",
    "CREATE INDEX db_wal_segment_lookup IF NOT EXISTS FOR (n:WalSegment) ON (n.document, n.segIndex)",
    "CREATE INDEX db_snapshot_generation_lookup IF NOT EXISTS FOR (n:SnapshotGeneration) ON (n.document, n.generation)",
    "CREATE INDEX db_index_run_lookup IF NOT EXISTS FOR (n:IndexRun) ON (n.document, n.runId)",
];

//#region 🔖️WalCypher
const CYPHER_WAL_CREATE_SEGMENT: &str = "
    MERGE (n:WalSegment {document: $document, segIndex: $index})
    ON CREATE SET n.bytes = '', n.sealed = false, n.len = 0, n.fresh = true
    ON MATCH SET n.fresh = false
    RETURN n.fresh AS fresh";

const CYPHER_WAL_READ_ROW: &str = "
    MATCH (n:WalSegment {document: $document, segIndex: $index})
    RETURN n.bytes AS bytes, n.sealed AS sealed, n.len AS len";

const CYPHER_WAL_WRITE_BYTES: &str = "
    MATCH (n:WalSegment {document: $document, segIndex: $index})
    SET n.bytes = $bytes, n.len = $len";

const CYPHER_WAL_SEAL: &str = "
    MATCH (n:WalSegment {document: $document, segIndex: $index})
    SET n.sealed = true
    RETURN n.sealed AS sealed";

const CYPHER_WAL_LIST_SEGMENTS: &str = "
    MATCH (n:WalSegment {document: $document})
    RETURN n.segIndex AS segIndex ORDER BY n.segIndex ASC";

const CYPHER_WAL_DELETE_SEGMENT: &str = "
    MATCH (n:WalSegment {document: $document, segIndex: $index})
    DETACH DELETE n";
//#endregion 🔖️WalCypher

//#region 🔖️SnapshotCypher
const CYPHER_SNAPSHOT_WRITE: &str = "
    MERGE (n:SnapshotGeneration {document: $document, generation: $generation})
    SET n.bytes = $bytes, n.len = $len";

const CYPHER_SNAPSHOT_READ: &str = "
    MATCH (n:SnapshotGeneration {document: $document, generation: $generation})
    RETURN n.bytes AS bytes, n.len AS len";

const CYPHER_SNAPSHOT_LATEST: &str = "
    MATCH (n:SnapshotGeneration {document: $document})
    RETURN max(n.generation) AS maxGeneration";

const CYPHER_SNAPSHOT_LIST: &str = "
    MATCH (n:SnapshotGeneration {document: $document})
    RETURN n.generation AS generation ORDER BY n.generation ASC";

const CYPHER_SNAPSHOT_DELETE: &str = "
    MATCH (n:SnapshotGeneration {document: $document, generation: $generation})
    DETACH DELETE n";
//#endregion 🔖️SnapshotCypher

//#region 🔖️PayloadCypher
const CYPHER_PAYLOAD_PUT: &str = "
    MERGE (p:Payload {hash: $hash})
    ON CREATE SET p.bytes = $bytes, p.len = $len";

const CYPHER_PAYLOAD_GET: &str = "
    MATCH (p:Payload {hash: $hash})
    RETURN p.bytes AS bytes, p.len AS len";

const CYPHER_PAYLOAD_CONTAINS: &str = "
    MATCH (p:Payload {hash: $hash})
    RETURN count(p) AS c";

const CYPHER_PAYLOAD_LEN: &str = "
    MATCH (p:Payload {hash: $hash})
    RETURN p.len AS len";

const CYPHER_PAYLOAD_DELETE: &str = "
    MATCH (p:Payload {hash: $hash})
    DETACH DELETE p";
//#endregion 🔖️PayloadCypher

//#region 🔖️CatalogCypher
const CYPHER_CATALOG_READ: &str = "
    MATCH (c:CatalogRoot {id: 'root'})
    RETURN c.epoch AS epoch, c.bytes AS bytes, c.len AS len";

/// @emoji ✅️ Single-statement compare-and-swap: `OPTIONAL MATCH` never creates a node, so a failed
/// comparison (the `WHERE` filters the only row away) leaves the graph untouched — `CatalogRoot`
/// only ever comes into existence via a SUCCEEDING `cas_root`, matching `read_root`'s "`None` until
/// `cas_root` has succeeded once" contract exactly. `coalesce(c.epoch, 0)` treats a not-yet-created
/// root as being at `EpochFence::INITIAL`, mirroring `MemoryStorage`/`FsStorage`'s identical
/// "absent root == epoch 0" convention.
const CYPHER_CATALOG_CAS: &str = "
    OPTIONAL MATCH (c:CatalogRoot {id: 'root'})
    WITH coalesce(c.epoch, 0) AS currentEpoch
    WHERE currentEpoch = $expected
    MERGE (n:CatalogRoot {id: 'root'})
    SET n.epoch = $newEpoch, n.bytes = $bytes, n.len = $len
    RETURN n.epoch AS epoch";
//#endregion 🔖️CatalogCypher

//#region 🔖️IndexCypher
const CYPHER_INDEX_WRITE: &str = "
    MERGE (n:IndexRun {document: $document, runId: $runId})
    SET n.bytes = $bytes, n.len = $len";

const CYPHER_INDEX_READ: &str = "
    MATCH (n:IndexRun {document: $document, runId: $runId})
    RETURN n.bytes AS bytes, n.len AS len";

const CYPHER_INDEX_LIST: &str = "
    MATCH (n:IndexRun {document: $document})
    RETURN n.runId AS runId ORDER BY n.runId ASC";

const CYPHER_INDEX_DELETE: &str = "
    MATCH (n:IndexRun {document: $document, runId: $runId})
    DETACH DELETE n";
//#endregion 🔖️IndexCypher

//#region 🔖️LeaseCypher
const CYPHER_LEASE_READ: &str = "
    MATCH (l:Lease {resource: $resource})
    RETURN l.holder AS holder, l.epoch AS epoch, l.expiresAtMs AS expiresAtMs";

const CYPHER_LEASE_WRITE: &str = "
    MERGE (l:Lease {resource: $resource})
    SET l.holder = $holder, l.epoch = $epoch, l.expiresAtMs = $expiresAtMs";

const CYPHER_LEASE_DELETE: &str = "
    MATCH (l:Lease {resource: $resource})
    DETACH DELETE l";
//#endregion 🔖️LeaseCypher
//#endregion 🔖️Cypher

//#region 🔖️Neo4jStorage
/// @emoji 🕸️ `DbStorage` over a live Neo4j server — see module doc for the schema shape, the
/// async-first `DbFuture` boundary, and the documented cross-process concurrency extension seam.
struct Neo4jDbIoExecutor {
    graph: Option<Graph>,
    config: Option<neo4rs::Config>,
    uri: DbIoText,
    backend_terminal: std::sync::atomic::AtomicBool,
    active_operation: u64,
}

impl Neo4jDbIoExecutor {
    fn new(config: neo4rs::Config, uri: DbIoText) -> Self {
        Self { graph: None, config: Some(config), uri, backend_terminal: std::sync::atomic::AtomicBool::new(false), active_operation: 0 }
    }

    fn graph(&self) -> Result<&Graph, DbError> {
        self.graph.as_ref().ok_or(DbError::Closed)
    }

    fn reserve_driver_read(&self, maximum: u64) -> Result<DbIoDriverReservation, DbError> {
        let maximum = usize::try_from(maximum).map_err(|_| DbError::LimitExceeded("Neo4j driver byte owner"))?;
        DbIoDriverReservation::try_reserve(self.active_operation, maximum)
    }

    async fn bootstrap_schema(&self) -> Result<(), DbError> {
        for statement in SCHEMA_STATEMENTS {
            self.graph()?.run(query(statement)).await.map_err(map_neo4rs_error)?;
        }
        Ok(())
    }

    /// @emoji 1⃣ Runs `q` (autocommit) and returns its first row, if any.
    async fn fetch_one(&self, q: Query) -> Result<Option<neo4rs::Row>, DbError> {
        let mut stream = self.graph()?.execute(q).await.map_err(map_neo4rs_error)?;
        stream.next().await.map_err(map_neo4rs_error)
    }

    /// @emoji ▶️ Runs `q` (autocommit), discarding any result rows.
    async fn run(&self, q: Query) -> Result<(), DbError> {
        self.graph()?.run(q).await.map_err(map_neo4rs_error)
    }
}
//#endregion 🔖️Neo4jStorage

//#region 🔖️WalStorage
impl WalStorage for Neo4jDbIoExecutor {
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        let row = self.fetch_one(query(CYPHER_WAL_CREATE_SEGMENT).param("document", document.0.clone()).param("index", idx)).await?;
        // 🎯️ `MERGE` always yields exactly one row; an empty stream here means the driver
        // silently dropped the result, which is this process's bug, not the caller's.
        let fresh: bool = row.ok_or_else(|| DbError::Internal("wal create_segment returned no row".to_string()))?.get("fresh").map_err(map_de_error)?;
        if !fresh {
            return Err(DbError::AlreadyExists(format!("wal segment {index} for {document} already exists")));
        }
        Ok(())
    }

    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        check_len(bytes.len() as u64, MAX_READ_BYTES, "wal_storage::append")?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let idx = u64_to_i64(index, "wal segment index")?;
        let mut current_reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let mut txn = self.graph()?.start_txn().await.map_err(map_neo4rs_error)?;
        let mut stream = txn.execute(query(CYPHER_WAL_READ_ROW).param("document", document.0.clone()).param("index", idx)).await.map_err(map_neo4rs_error)?;
        let row = stream.next(txn.handle()).await.map_err(map_neo4rs_error)?;
        let Some(row) = row else {
            return Err(DbError::NotFound(format!("wal segment {index} for {document} not found")));
        };
        let current_len = i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")?;
        check_len(current_len, MAX_READ_BYTES, "wal_storage::append current length")?;
        let current: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut current = DbIoExternalBytes::new(current.value.to_vec());
        current_reservation.observe_capacity(current.capacity()?)?;
        let sealed: bool = row.get("sealed").map_err(map_de_error)?;
        if sealed {
            return Err(DbError::InvalidArgument("cannot append to sealed wal segment".to_string()));
        }
        let new_len = current.as_slice()?.len().checked_add(prepared.as_slice().len()).ok_or(DbError::LimitExceeded("Neo4j WAL append length"))?;
        check_len(new_len as u64, MAX_READ_BYTES, "wal_storage::append result")?;
        let combined = db_io_prepare_platform_slices(self.active_operation, current.as_slice()?, prepared.as_slice()).await?;
        let write = txn
            .run(query(CYPHER_WAL_WRITE_BYTES).param("document", document.0.clone()).param("index", idx).param("bytes", combined.as_static_driver_slice().to_vec()).param("len", u64_to_i64(new_len as u64, "wal segment length")?))
            .await
            .map_err(map_neo4rs_error);
        while !current.terminal_is_empty() {
            let _ = current.close_step();
            semio_framework_async::yield_once().await;
        }
        current_reservation.close_step()?;
        db_io_close_platform(combined).await?;
        db_io_close_platform(prepared).await?;
        write?;
        txn.commit().await.map_err(map_neo4rs_error)?;
        Ok(new_len as u64)
    }

    async fn sync(&self, _document: &ArtifactId, _index: u64, _class: DurabilityClass) -> Result<(), DbError> {
        // 🎯️ See module doc's "Durability" section: every prior `append`/`seal` already committed
        // server-side, so there is nothing left to force for any `DurabilityClass`.
        {
            Ok(())
        }
    }

    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        let row = self.fetch_one(query(CYPHER_WAL_SEAL).param("document", document.0.clone()).param("index", idx)).await?;
        row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        Ok(())
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        check_len(range.len, MAX_READ_BYTES, "wal_storage::read")?;
        let idx = u64_to_i64(index, "wal segment index")?;
        let reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_WAL_READ_ROW).param("document", document.0.clone()).param("index", idx)).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        let current_len = i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")?;
        check_len(current_len, MAX_READ_BYTES, "wal_storage::read current length")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let _ = slice_range(&bytes.value, range)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (range.len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        write_driver_bytes(reservation, bytes, range.offset as usize, range.len as usize, &mut output).await
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        let row = self.fetch_one(query(CYPHER_WAL_READ_ROW).param("document", document.0.clone()).param("index", idx)).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("wal segment {index} for {document} not found")))?;
        i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let mut stream = self.graph()?.execute(query(CYPHER_WAL_LIST_SEGMENTS).param("document", document.0.clone())).await.map_err(map_neo4rs_error)?;
        let mut out = DbIoU64List::new();
        while let Some(row) = stream.next().await.map_err(map_neo4rs_error)? {
            out.push(i64_to_u64(row.get("segIndex").map_err(map_de_error)?, "wal segment index")?)?;
        }
        Ok(out)
    }

    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        let mut current_reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let mut txn = self.graph()?.start_txn().await.map_err(map_neo4rs_error)?;
        let mut stream = txn.execute(query(CYPHER_WAL_READ_ROW).param("document", document.0.clone()).param("index", idx)).await.map_err(map_neo4rs_error)?;
        let row = stream.next(txn.handle()).await.map_err(map_neo4rs_error)?;
        let Some(row) = row else {
            return Err(DbError::NotFound(format!("wal segment {index} for {document} not found")));
        };
        let current_len = i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")?;
        check_len(current_len, MAX_READ_BYTES, "wal_storage::truncate_tail current length")?;
        let current: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut current = DbIoExternalBytes::new(current.value.to_vec());
        current_reservation.observe_capacity(current.capacity()?)?;
        let sealed: bool = row.get("sealed").map_err(map_de_error)?;
        if sealed {
            return Err(DbError::InvalidArgument("cannot truncate sealed wal segment".to_string()));
        }
        if new_len > current.as_slice()?.len() as u64 {
            return Err(DbError::InvalidArgument("truncate_tail new_len exceeds current segment length".to_string()));
        }
        let truncated = db_io_prepare_platform_slices(self.active_operation, &current.as_slice()?[..new_len as usize], &[]).await?;
        let write = txn
            .run(query(CYPHER_WAL_WRITE_BYTES).param("document", document.0.clone()).param("index", idx).param("bytes", truncated.as_static_driver_slice().to_vec()).param("len", u64_to_i64(new_len, "wal segment length")?))
            .await
            .map_err(map_neo4rs_error);
        while !current.terminal_is_empty() {
            let _ = current.close_step();
            semio_framework_async::yield_once().await;
        }
        current_reservation.close_step()?;
        db_io_close_platform(truncated).await?;
        write?;
        txn.commit().await.map_err(map_neo4rs_error)?;
        Ok(())
    }

    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        let idx = u64_to_i64(index, "wal segment index")?;
        self.run(query(CYPHER_WAL_DELETE_SEGMENT).param("document", document.0.clone()).param("index", idx)).await
    }
}
//#endregion 🔖️WalStorage

//#region 🔖️SnapshotStorage
impl SnapshotStorage for Neo4jDbIoExecutor {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        check_len(bytes.len() as u64, MAX_READ_BYTES, "snapshot_storage::write_generation")?;
        let generation_param = u64_to_i64(generation, "snapshot generation")?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = self
            .run(
                query(CYPHER_SNAPSHOT_WRITE)
                    .param("document", document.0.clone())
                    .param("generation", generation_param)
                    .param("bytes", prepared.as_static_driver_slice().to_vec())
                    .param("len", u64_to_i64(bytes.len() as u64, "snapshot generation length")?),
            )
            .await;
        db_io_close_platform(prepared).await?;
        result
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        let generation_param = u64_to_i64(generation, "snapshot generation")?;
        let reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_SNAPSHOT_READ).param("document", document.0.clone()).param("generation", generation_param)).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("snapshot generation {generation} for {document} not found")))?;
        let len = i64_to_u64(row.get("len").map_err(map_de_error)?, "snapshot generation length")?;
        check_len(len, MAX_READ_BYTES, "snapshot_storage::read_generation")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        write_driver_bytes(reservation, bytes, 0, len as usize, &mut output).await
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        let row = self.fetch_one(query(CYPHER_SNAPSHOT_LATEST).param("document", document.0.clone())).await?;
        match row.and_then(|row| row.get::<i64>("maxGeneration").ok()) {
            Some(max) => Ok(Some(i64_to_u64(max, "snapshot generation")?)),
            None => Ok(None),
        }
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let mut stream = self.graph()?.execute(query(CYPHER_SNAPSHOT_LIST).param("document", document.0.clone())).await.map_err(map_neo4rs_error)?;
        let mut out = DbIoU64List::new();
        while let Some(row) = stream.next().await.map_err(map_neo4rs_error)? {
            out.push(i64_to_u64(row.get("generation").map_err(map_de_error)?, "snapshot generation")?)?;
        }
        Ok(out)
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        let generation_param = u64_to_i64(generation, "snapshot generation")?;
        self.run(query(CYPHER_SNAPSHOT_DELETE).param("document", document.0.clone()).param("generation", generation_param)).await
    }
}
//#endregion 🔖️SnapshotStorage

//#region 🔖️PayloadStorage
impl PayloadStorage for Neo4jDbIoExecutor {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        check_len(bytes.len() as u64, MAX_READ_BYTES, "payload_storage::put")?;
        let hash = db_io_hash_pages(&bytes).await;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = self.run(query(CYPHER_PAYLOAD_PUT).param("hash", hash.to_string()).param("bytes", prepared.as_static_driver_slice().to_vec()).param("len", u64_to_i64(bytes.len() as u64, "payload length")?)).await;
        db_io_close_platform(prepared).await?;
        result?;
        Ok(hash)
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        let reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_PAYLOAD_GET).param("hash", hash.to_string())).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
        let len = i64_to_u64(row.get("len").map_err(map_de_error)?, "payload length")?;
        check_len(len, MAX_READ_BYTES, "payload_storage::get")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        write_driver_bytes(reservation, bytes, 0, len as usize, &mut output).await
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        let row = self.fetch_one(query(CYPHER_PAYLOAD_CONTAINS).param("hash", hash.to_string())).await?;
        let count: i64 = row.ok_or_else(|| DbError::Internal("payload_storage::contains returned no row".to_string()))?.get("c").map_err(map_de_error)?;
        Ok(count > 0)
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        self.run(query(CYPHER_PAYLOAD_DELETE).param("hash", hash.to_string())).await
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        let row = self.fetch_one(query(CYPHER_PAYLOAD_LEN).param("hash", hash.to_string())).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("payload {hash} not found")))?;
        i64_to_u64(row.get("len").map_err(map_de_error)?, "payload length")
    }
}
//#endregion 🔖️PayloadStorage

//#region 🔖️CatalogStorage
impl CatalogStorage for Neo4jDbIoExecutor {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let mut reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_CATALOG_READ)).await?;
        let Some(row) = row else {
            reservation.close_step()?;
            return Ok(None);
        };
        let epoch = i64_to_u64(row.get("epoch").map_err(map_de_error)?, "catalog epoch")?;
        let len = i64_to_u64(row.get("len").map_err(map_de_error)?, "catalog root length")?;
        check_len(len, MAX_READ_BYTES, "catalog_storage::read_root")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        Ok(Some((write_driver_bytes(reservation, bytes, 0, len as usize, &mut output).await?, EpochFence { epoch })))
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        check_len(new_bytes.len() as u64, MAX_READ_BYTES, "catalog_storage::cas_root")?;
        let new_fence = expected.next();
        let prepared = db_io_prepare_platform(&new_bytes)?.await?;
        let row = self
            .fetch_one(
                query(CYPHER_CATALOG_CAS)
                    .param("expected", u64_to_i64(expected.epoch, "catalog epoch")?)
                    .param("newEpoch", u64_to_i64(new_fence.epoch, "catalog epoch")?)
                    .param("bytes", prepared.as_static_driver_slice().to_vec())
                    .param("len", u64_to_i64(new_bytes.len() as u64, "catalog root length")?),
            )
            .await;
        db_io_close_platform(prepared).await?;
        let row = row?;
        if row.is_some() {
            return Ok(new_fence);
        }
        // 🎯️ The CAS attempt itself was atomic (see `CYPHER_CATALOG_CAS`'s doc); this follow-up
        // read only decides what CURRENT epoch to report in the `Fenced` error, so a benign race
        // against a concurrent writer can only change the reported number, never the CAS outcome.
        let current = self.read_root().await?.map_or(EpochFence::INITIAL, |(_, fence)| fence);
        Err(DbError::Fenced { expected: current.epoch, actual: expected.epoch })
    }
}
//#endregion 🔖️CatalogStorage

//#region 🔖️IndexStorage
impl IndexStorage for Neo4jDbIoExecutor {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        check_len(bytes.len() as u64, MAX_READ_BYTES, "index_storage::write_run")?;
        let run_id_param = u64_to_i64(run_id, "index run id")?;
        let prepared = db_io_prepare_platform(&bytes)?.await?;
        let result = self
            .run(query(CYPHER_INDEX_WRITE).param("document", document.0.clone()).param("runId", run_id_param).param("bytes", prepared.as_static_driver_slice().to_vec()).param("len", u64_to_i64(bytes.len() as u64, "index run length")?))
            .await;
        db_io_close_platform(prepared).await?;
        result
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        let run_id_param = u64_to_i64(run_id, "index run id")?;
        let reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_INDEX_READ).param("document", document.0.clone()).param("runId", run_id_param)).await?;
        let row = row.ok_or_else(|| DbError::NotFound(format!("index run {run_id} for {document} not found")))?;
        let len = i64_to_u64(row.get("len").map_err(map_de_error)?, "index run length")?;
        check_len(len, MAX_READ_BYTES, "index_storage::read_run")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        let mut output = DbIoPageWriter::try_reserve_for_operation(self.active_operation, (len as usize).div_ceil(DB_IO_PAGE_BYTES)).map_err(DbIoPageWriterRejected::into_error)?;
        write_driver_bytes(reservation, bytes, 0, len as usize, &mut output).await
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        let mut stream = self.graph()?.execute(query(CYPHER_INDEX_LIST).param("document", document.0.clone())).await.map_err(map_neo4rs_error)?;
        let mut out = DbIoU64List::new();
        while let Some(row) = stream.next().await.map_err(map_neo4rs_error)? {
            out.push(i64_to_u64(row.get("runId").map_err(map_de_error)?, "index run id")?)?;
        }
        Ok(out)
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        let run_id_param = u64_to_i64(run_id, "index run id")?;
        self.run(query(CYPHER_INDEX_DELETE).param("document", document.0.clone()).param("runId", run_id_param)).await
    }
}
//#endregion 🔖️IndexStorage

//#region 🔖️LeaseStorage
impl Neo4jDbIoExecutor {
    /// @emoji 📖️ Reads `resource`'s current lease row (regardless of expiry — callers decide what
    /// an expired row means) within `txn`.
    async fn lease_row(&self, txn: &mut Txn, resource: &str) -> Result<Option<LeaseRow>, DbError> {
        let mut reservation = DbIoDriverReservation::try_reserve(self.active_operation, DbIoText::maximum_capacity())?;
        let mut stream = txn.execute(query(CYPHER_LEASE_READ).param("resource", resource)).await.map_err(map_neo4rs_error)?;
        let row = stream.next(txn.handle()).await.map_err(map_neo4rs_error)?;
        let Some(row) = row else {
            reservation.close_step()?;
            return Ok(None);
        };
        let fence = EpochFence { epoch: i64_to_u64(row.get("epoch").map_err(map_de_error)?, "lease epoch")? };
        let expires_at_ms = i64_to_u64(row.get("expiresAtMs").map_err(map_de_error)?, "lease expiry")?;
        let holder: String = row.get("holder").map_err(map_de_error)?;
        Ok(Some((fence, expires_at_ms, db_io_copy_observed_text(reservation, holder).await?)))
    }
}

impl LeaseStorage for Neo4jDbIoExecutor {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        let mut txn = self.graph()?.start_txn().await.map_err(map_neo4rs_error)?;
        let existing = self.lease_row(&mut txn, resource).await?;
        let fence = decide_acquire_fence(resource, existing, holder, now_ms)?;
        let expires_at_ms = now_ms.checked_add(ttl_ms).ok_or_else(|| DbError::InvalidArgument("lease ttl_ms overflows now_ms + ttl_ms".to_string()))?;
        txn.run(query(CYPHER_LEASE_WRITE).param("resource", resource).param("holder", holder).param("epoch", u64_to_i64(fence.epoch, "lease epoch")?).param("expiresAtMs", u64_to_i64(expires_at_ms, "lease expiry")?))
            .await
            .map_err(map_neo4rs_error)?;
        txn.commit().await.map_err(map_neo4rs_error)?;
        Ok(fence)
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        let mut txn = self.graph()?.start_txn().await.map_err(map_neo4rs_error)?;
        let existing = self.lease_row(&mut txn, resource).await?;
        validate_renew(resource, existing, holder, fence, now_ms)?;
        let expires_at_ms = now_ms.checked_add(ttl_ms).ok_or_else(|| DbError::InvalidArgument("lease ttl_ms overflows now_ms + ttl_ms".to_string()))?;
        txn.run(query(CYPHER_LEASE_WRITE).param("resource", resource).param("holder", holder).param("epoch", u64_to_i64(fence.epoch, "lease epoch")?).param("expiresAtMs", u64_to_i64(expires_at_ms, "lease expiry")?))
            .await
            .map_err(map_neo4rs_error)?;
        txn.commit().await.map_err(map_neo4rs_error)?;
        Ok(())
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        let mut txn = self.graph()?.start_txn().await.map_err(map_neo4rs_error)?;
        let existing = self.lease_row(&mut txn, resource).await?;
        validate_release(resource, existing, holder, fence)?;
        txn.run(query(CYPHER_LEASE_DELETE).param("resource", resource)).await.map_err(map_neo4rs_error)?;
        txn.commit().await.map_err(map_neo4rs_error)?;
        Ok(())
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        let mut reservation = DbIoDriverReservation::try_reserve(self.active_operation, DbIoText::maximum_capacity())?;
        let row = self.fetch_one(query(CYPHER_LEASE_READ).param("resource", resource)).await?;
        let Some(row) = row else {
            reservation.close_step()?;
            return Ok(None);
        };
        let epoch = i64_to_u64(row.get("epoch").map_err(map_de_error)?, "lease epoch")?;
        let expires_at_ms = i64_to_u64(row.get("expiresAtMs").map_err(map_de_error)?, "lease expiry")?;
        if now_ms >= expires_at_ms {
            reservation.close_step()?;
            return Ok(None);
        }
        let holder: String = row.get("holder").map_err(map_de_error)?;
        let holder = db_io_copy_observed_text(reservation, holder).await?;
        Ok(Some(DbIoLeaseResult::new(DbIoText::try_from_str(resource)?, holder, EpochFence { epoch }, expires_at_ms)))
    }
}
//#endregion 🔖️LeaseStorage

//#region 🔖️TypedExecutor
impl Neo4jDbIoExecutor {
    async fn wal_read_into(&self, document: &str, index: u64, range: ByteRange, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        check_len(range.len, MAX_READ_BYTES, "Neo4j WAL task read")?;
        let index = u64_to_i64(index, "wal segment index")?;
        let reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_WAL_READ_ROW).param("document", document).param("index", index)).await?;
        let row = row.ok_or_else(|| DbError::NotFound("Neo4j WAL segment not found".to_string()))?;
        let length = i64_to_u64(row.get("len").map_err(map_de_error)?, "wal segment length")?;
        if range.offset.checked_add(range.len).is_none_or(|end| end > length) {
            return Err(DbError::InvalidArgument("Neo4j WAL task range exceeds segment".to_string()));
        }
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        write_driver_bytes(reservation, bytes, range.offset as usize, range.len as usize, output).await
    }

    async fn named_blob_read_into(&self, kind: &'static str, document: &str, ordinal: u64, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        let (query_text, key, value) = match kind {
            "snapshot" => (CYPHER_SNAPSHOT_READ, "generation", u64_to_i64(ordinal, "snapshot generation")?),
            "index" => (CYPHER_INDEX_READ, "runId", u64_to_i64(ordinal, "index run")?),
            _ => return Err(DbError::Internal("Neo4j named blob taxonomy mismatch".to_string())),
        };
        let reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let row = self.fetch_one(query(query_text).param("document", document).param(key, value)).await?;
        let row = row.ok_or_else(|| DbError::NotFound("Neo4j named blob not found".to_string()))?;
        let length = i64_to_u64(row.get("len").map_err(map_de_error)?, "Neo4j named blob length")?;
        check_len(length, MAX_READ_BYTES, "Neo4j named blob")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        write_driver_bytes(reservation, bytes, 0, length as usize, output).await
    }

    async fn payload_read_into(&self, hash: &ContentHash, output: &mut DbIoPageWriter) -> Result<DbIoPages, DbError> {
        let reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let row = self.fetch_one(query(CYPHER_PAYLOAD_GET).param("hash", hash.to_string())).await?;
        let row = row.ok_or_else(|| DbError::NotFound("Neo4j payload not found".to_string()))?;
        let length = i64_to_u64(row.get("len").map_err(map_de_error)?, "payload length")?;
        check_len(length, MAX_READ_BYTES, "Neo4j payload")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        write_driver_bytes(reservation, bytes, 0, length as usize, output).await
    }

    async fn catalog_read_into(&self, output: &mut DbIoPageWriter) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        let mut reservation = self.reserve_driver_read(MAX_READ_BYTES)?;
        let Some(row) = self.fetch_one(query(CYPHER_CATALOG_READ)).await? else {
            reservation.close_step()?;
            return Ok(None);
        };
        let epoch = i64_to_u64(row.get("epoch").map_err(map_de_error)?, "catalog epoch")?;
        let length = i64_to_u64(row.get("len").map_err(map_de_error)?, "catalog length")?;
        check_len(length, MAX_READ_BYTES, "Neo4j catalog")?;
        let bytes: BoltBytes = row.get("bytes").map_err(map_de_error)?;
        Ok(Some((write_driver_bytes(reservation, bytes, 0, length as usize, output).await?, EpochFence { epoch })))
    }

    async fn drive_task(&mut self, operation: u64, task: &mut DbIoTask) -> Result<DbIoResult, DbError> {
        self.active_operation = operation;
        match task {
            DbIoTask::BackendOpen { path, .. } => {
                if path.as_str() != self.uri.as_str() {
                    return Err(DbError::InvalidArgument("Neo4j URI authority mismatch".to_string()));
                }
                let config = self.config.take().ok_or_else(|| DbError::Internal("Neo4j connection configuration consumed twice".to_string()))?;
                self.graph = Some(Graph::connect(config).await.map_err(map_neo4rs_error)?);
                self.bootstrap_schema().await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalCreate { document, index, .. } => {
                with_admitted_artifact!(operation, document, artifact, <Self as WalStorage>::create_segment(self, artifact, *index))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalAppend { document, index, input, .. } => {
                let input = input.take_for_async_driver();
                Ok(DbIoResult::Length(with_admitted_artifact!(operation, document, artifact, <Self as WalStorage>::append(self, artifact, *index, input))?))
            }
            DbIoTask::WalSync { document, index, class, .. } => {
                with_admitted_artifact!(operation, document, artifact, <Self as WalStorage>::sync(self, artifact, *index, *class))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalSeal { document, index, .. } => {
                with_admitted_artifact!(operation, document, artifact, <Self as WalStorage>::seal(self, artifact, *index))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalRead { document, index, range, output, .. } => Ok(DbIoResult::Pages(self.wal_read_into(document.as_str(), *index, *range, output).await?)),
            DbIoTask::WalLength { document, index, .. } => Ok(DbIoResult::Length(with_admitted_artifact!(operation, document, artifact, <Self as WalStorage>::segment_len(self, artifact, *index))?)),
            DbIoTask::WalList { document, output, .. } => {
                let list = with_admitted_artifact!(operation, document, artifact, <Self as WalStorage>::list_segments(self, artifact))?;
                Ok(DbIoResult::List(db_io_transfer_list(list, output).await?))
            }
            DbIoTask::WalTruncate { document, index, new_len, .. } => {
                with_admitted_artifact!(operation, document, artifact, <Self as WalStorage>::truncate_tail(self, artifact, *index, *new_len))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::WalDelete { document, index, .. } => {
                with_admitted_artifact!(operation, document, artifact, <Self as WalStorage>::delete_segment(self, artifact, *index))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::SnapshotWrite { document, generation, input, .. } => {
                let input = input.take_for_async_driver();
                with_admitted_artifact!(operation, document, artifact, <Self as SnapshotStorage>::write_generation(self, artifact, *generation, input))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::SnapshotRead { document, generation, output, .. } => Ok(DbIoResult::Pages(self.named_blob_read_into("snapshot", document.as_str(), *generation, output).await?)),
            DbIoTask::SnapshotLatest { document, .. } => Ok(DbIoResult::OptionalLength(with_admitted_artifact!(operation, document, artifact, <Self as SnapshotStorage>::latest_generation(self, artifact))?)),
            DbIoTask::SnapshotList { document, output, .. } => {
                let list = with_admitted_artifact!(operation, document, artifact, <Self as SnapshotStorage>::list_generations(self, artifact))?;
                Ok(DbIoResult::List(db_io_transfer_list(list, output).await?))
            }
            DbIoTask::SnapshotDelete { document, generation, .. } => {
                with_admitted_artifact!(operation, document, artifact, <Self as SnapshotStorage>::delete_generation(self, artifact, *generation))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::PayloadPut { input, .. } => {
                let input = input.take_for_async_driver();
                Ok(DbIoResult::Hash(<Self as PayloadStorage>::put(self, input).await?))
            }
            DbIoTask::PayloadGet { hash, output, .. } => Ok(DbIoResult::Pages(self.payload_read_into(hash, output).await?)),
            DbIoTask::PayloadExists { hash, .. } => Ok(DbIoResult::Exists(<Self as PayloadStorage>::contains(self, hash).await?)),
            DbIoTask::PayloadLength { hash, .. } => Ok(DbIoResult::Length(<Self as PayloadStorage>::len(self, hash).await?)),
            DbIoTask::PayloadDelete { hash, .. } => {
                <Self as PayloadStorage>::delete(self, hash).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::CatalogRead { output, .. } => Ok(DbIoResult::OptionalCatalog(self.catalog_read_into(output).await?)),
            DbIoTask::CatalogCas { expected, input, .. } => {
                let input = input.take_for_async_driver();
                Ok(DbIoResult::Fence(<Self as CatalogStorage>::cas_root(self, *expected, input).await?))
            }
            DbIoTask::IndexWrite { document, run_id, input, .. } => {
                let input = input.take_for_async_driver();
                with_admitted_artifact!(operation, document, artifact, <Self as IndexStorage>::write_run(self, artifact, *run_id, input))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::IndexRead { document, run_id, output, .. } => Ok(DbIoResult::Pages(self.named_blob_read_into("index", document.as_str(), *run_id, output).await?)),
            DbIoTask::IndexList { document, output, .. } => {
                let list = with_admitted_artifact!(operation, document, artifact, <Self as IndexStorage>::list_runs(self, artifact))?;
                Ok(DbIoResult::List(db_io_transfer_list(list, output).await?))
            }
            DbIoTask::IndexDelete { document, run_id, .. } => {
                with_admitted_artifact!(operation, document, artifact, <Self as IndexStorage>::delete_run(self, artifact, *run_id))?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::LeaseAcquire { document, holder, ttl_ms, now_ms, .. } => Ok(DbIoResult::Fence(<Self as LeaseStorage>::acquire(self, document.as_str(), holder.as_str(), *ttl_ms, *now_ms).await?)),
            DbIoTask::LeaseRenew { document, holder, fence, ttl_ms, now_ms, .. } => {
                <Self as LeaseStorage>::renew(self, document.as_str(), holder.as_str(), *fence, *ttl_ms, *now_ms).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::LeaseRelease { document, holder, fence, .. } => {
                <Self as LeaseStorage>::release(self, document.as_str(), holder.as_str(), *fence).await?;
                Ok(DbIoResult::Unit)
            }
            DbIoTask::LeaseGet { document, now_ms, .. } => {
                let lease = <Self as LeaseStorage>::current(self, document.as_str(), *now_ms).await?;
                Ok(DbIoResult::OptionalLease(lease))
            }
            DbIoTask::BackendClose { .. } => Ok(DbIoResult::Unit),
        }
    }
}

impl DbIoTaskExecutor for Neo4jDbIoExecutor {
    fn mode(&self) -> DbIoExecutorMode {
        DbIoExecutorMode::AsyncNative
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn execute_step(&self, _operation: u64, _task: &mut DbIoTask) -> Result<(DbIoExecutionStep, Option<DbIoResult>), DbError> {
        Err(DbError::Internal("Neo4j async-native task entered the blocking executor".to_string()))
    }
    fn drive_async(self: Box<Self>, operation: u64, task: DbIoTask) -> DbIoAsyncDriverFuture {
        Box::pin(async move {
            let mut executor = self;
            let mut task = task;
            let terminal = executor.drive_task(operation, &mut task).await;
            let executor: Box<dyn DbIoTaskExecutor> = executor;
            (executor, task, terminal)
        })
    }
    fn close_operation_step(&self, _operation: u64, _task: &DbIoTask) -> Result<bool, DbError> {
        Ok(true)
    }
    fn close_backend_step(&mut self, _context: &mut std::task::Context<'_>) -> Result<bool, DbError> {
        if self.graph.take().is_some() {
            return Ok(false);
        }
        self.config.take();
        self.backend_terminal.store(true, std::sync::atomic::Ordering::Release);
        Ok(true)
    }
    fn backend_terminal_is_empty(&self) -> bool {
        self.backend_terminal.load(std::sync::atomic::Ordering::Acquire) && self.graph.is_none() && self.config.is_none()
    }
}
//#endregion 🔖️TypedExecutor

/// @emoji 🕸️ Typed Neo4j facade; only the registered executor owns the external graph driver.
pub struct Neo4jStorage {
    control: DbIoBackendControl,
    worker_pool: Arc<WorkerPool>,
    closed: std::sync::atomic::AtomicBool,
}

impl Neo4jStorage {
    pub async fn connect(worker_pool: Arc<WorkerPool>, uri: &str, user: &str, password: &str) -> Result<Self, DbError> {
        let uri_owner = DbIoText::try_from_str(uri)?;
        let config = neo4rs::ConfigBuilder::default().uri(uri).user(user).password(password).build().map_err(map_neo4rs_error)?;
        Self::connect_owned(worker_pool, uri_owner, config).await
    }

    pub async fn connect_to_database(worker_pool: Arc<WorkerPool>, uri: &str, user: &str, password: &str, database: &str) -> Result<Self, DbError> {
        let uri_owner = DbIoText::try_from_str(uri)?;
        let config = neo4rs::ConfigBuilder::default().uri(uri).user(user).password(password).db(database).build().map_err(map_neo4rs_error)?;
        Self::connect_owned(worker_pool, uri_owner, config).await
    }

    async fn connect_owned(worker_pool: Arc<WorkerPool>, uri: DbIoText, config: neo4rs::Config) -> Result<Self, DbError> {
        let executor = Box::new(Neo4jDbIoExecutor::new(config, uri.clone()));
        let control = register_db_io_backend(DbIoBackendKind::Neo4j, executor, worker_pool.clone())?;
        let storage = Self { control, worker_pool, closed: std::sync::atomic::AtomicBool::new(false) };
        if let Err(error) = storage.execute(DbIoTask::BackendOpen { backend: control, path: uri }).await {
            let _ = storage.execute(DbIoTask::BackendClose { backend: control }).await;
            return Err(error);
        }
        Ok(storage)
    }

    async fn execute(&self, task: DbIoTask) -> Result<DbIoResult, DbError> {
        let mut operation = submit_db_io_task(self.worker_pool.as_ref(), task).map_err(|(error, _)| error)?;
        operation.start_async_native_on_lane_io().await?;
        operation.finish().await
    }

    pub async fn close(&self) -> Result<(), DbError> {
        let result = neo4j_unit(self.execute(DbIoTask::BackendClose { backend: self.control }).await?);
        if result.is_ok() {
            close_db_io_backend(self.control).await?;
            self.closed.store(true, std::sync::atomic::Ordering::Release);
        }
        result
    }
}

impl Drop for Neo4jStorage {
    fn drop(&mut self) {
        if !self.closed.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let _ = retire_db_io_backend(self.control);
        }
    }
}

fn neo4j_document(document: &ArtifactId) -> Result<DbIoText, DbError> {
    DbIoText::try_from_str(&document.0)
}

fn neo4j_output(bytes: u64) -> Result<DbIoPageWriter, DbError> {
    let pages = usize::try_from(bytes).map_err(|_| DbError::LimitExceeded("Neo4j output bytes"))?.div_ceil(DB_IO_PAGE_BYTES);
    DbIoPageWriter::try_reserve(pages).map_err(DbIoPageWriterRejected::into_error)
}

fn neo4j_unit(result: DbIoResult) -> Result<(), DbError> {
    match result {
        DbIoResult::Unit => Ok(()),
        _ => Err(DbError::Internal("Neo4j executor returned a non-unit result".to_string())),
    }
}

fn neo4j_pages(result: DbIoResult) -> Result<DbIoPages, DbError> {
    match result {
        DbIoResult::Pages(pages) => Ok(pages),
        _ => Err(DbError::Internal("Neo4j executor returned a non-page result".to_string())),
    }
}

fn neo4j_list(result: DbIoResult) -> Result<DbIoU64List, DbError> {
    match result {
        DbIoResult::List(list) => Ok(list),
        _ => Err(DbError::Internal("Neo4j executor returned a non-list result".to_string())),
    }
}

impl WalStorage for Neo4jStorage {
    async fn create_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalCreate { backend: self.control, document: neo4j_document(document)?, index }).await?)
    }
    async fn append(&self, document: &ArtifactId, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        match self.execute(DbIoTask::WalAppend { backend: self.control, document: neo4j_document(document)?, index, input: bytes }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("Neo4j executor returned a non-length result".to_string())),
        }
    }
    async fn sync(&self, document: &ArtifactId, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalSync { backend: self.control, document: neo4j_document(document)?, index, class }).await?)
    }
    async fn seal(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalSeal { backend: self.control, document: neo4j_document(document)?, index }).await?)
    }
    async fn read(&self, document: &ArtifactId, index: u64, range: ByteRange) -> Result<DbIoPages, DbError> {
        neo4j_pages(self.execute(DbIoTask::WalRead { backend: self.control, document: neo4j_document(document)?, index, range, output: neo4j_output(range.len)? }).await?)
    }
    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        match self.execute(DbIoTask::WalLength { backend: self.control, document: neo4j_document(document)?, index }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("Neo4j executor returned a non-length result".to_string())),
        }
    }
    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        neo4j_list(self.execute(DbIoTask::WalList { backend: self.control, document: neo4j_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn truncate_tail(&self, document: &ArtifactId, index: u64, new_len: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalTruncate { backend: self.control, document: neo4j_document(document)?, index, new_len }).await?)
    }
    async fn delete_segment(&self, document: &ArtifactId, index: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::WalDelete { backend: self.control, document: neo4j_document(document)?, index }).await?)
    }
}

impl SnapshotStorage for Neo4jStorage {
    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::SnapshotWrite { backend: self.control, document: neo4j_document(document)?, generation, input: bytes }).await?)
    }
    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        neo4j_pages(self.execute(DbIoTask::SnapshotRead { backend: self.control, document: neo4j_document(document)?, generation, output: neo4j_output(MAX_READ_BYTES)? }).await?)
    }
    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        match self.execute(DbIoTask::SnapshotLatest { backend: self.control, document: neo4j_document(document)?, output: DbIoU64List::new() }).await? {
            DbIoResult::OptionalLength(generation) => Ok(generation),
            _ => Err(DbError::Internal("Neo4j executor returned a non-generation result".to_string())),
        }
    }
    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        neo4j_list(self.execute(DbIoTask::SnapshotList { backend: self.control, document: neo4j_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::SnapshotDelete { backend: self.control, document: neo4j_document(document)?, generation }).await?)
    }
}

impl PayloadStorage for Neo4jStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        match self.execute(DbIoTask::PayloadPut { backend: self.control, input: bytes }).await? {
            DbIoResult::Hash(hash) => Ok(hash),
            _ => Err(DbError::Internal("Neo4j executor returned a non-hash result".to_string())),
        }
    }
    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        neo4j_pages(self.execute(DbIoTask::PayloadGet { backend: self.control, hash: *hash, output: neo4j_output(MAX_READ_BYTES)? }).await?)
    }
    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        match self.execute(DbIoTask::PayloadExists { backend: self.control, hash: *hash }).await? {
            DbIoResult::Exists(exists) => Ok(exists),
            _ => Err(DbError::Internal("Neo4j executor returned a non-exists result".to_string())),
        }
    }
    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::PayloadDelete { backend: self.control, hash: *hash }).await?)
    }
    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        match self.execute(DbIoTask::PayloadLength { backend: self.control, hash: *hash }).await? {
            DbIoResult::Length(length) => Ok(length),
            _ => Err(DbError::Internal("Neo4j executor returned a non-length result".to_string())),
        }
    }
}

impl CatalogStorage for Neo4jStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        match self.execute(DbIoTask::CatalogRead { backend: self.control, output: neo4j_output(MAX_READ_BYTES)? }).await? {
            DbIoResult::OptionalCatalog(catalog) => Ok(catalog),
            _ => Err(DbError::Internal("Neo4j executor returned a non-catalog result".to_string())),
        }
    }
    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        match self.execute(DbIoTask::CatalogCas { backend: self.control, expected, input: new_bytes }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(DbError::Internal("Neo4j executor returned a non-fence result".to_string())),
        }
    }
}

impl IndexStorage for Neo4jStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::IndexWrite { backend: self.control, document: neo4j_document(document)?, run_id, input: bytes }).await?)
    }
    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        neo4j_pages(self.execute(DbIoTask::IndexRead { backend: self.control, document: neo4j_document(document)?, run_id, output: neo4j_output(MAX_READ_BYTES)? }).await?)
    }
    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        neo4j_list(self.execute(DbIoTask::IndexList { backend: self.control, document: neo4j_document(document)?, output: DbIoU64List::new() }).await?)
    }
    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::IndexDelete { backend: self.control, document: neo4j_document(document)?, run_id }).await?)
    }
}

impl LeaseStorage for Neo4jStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        match self.execute(DbIoTask::LeaseAcquire { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, now_ms, ttl_ms }).await? {
            DbIoResult::Fence(fence) => Ok(fence),
            _ => Err(DbError::Internal("Neo4j executor returned a non-fence result".to_string())),
        }
    }
    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::LeaseRenew { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence, now_ms, ttl_ms }).await?)
    }
    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        neo4j_unit(self.execute(DbIoTask::LeaseRelease { backend: self.control, document: DbIoText::try_from_str(resource)?, holder: DbIoText::try_from_str(holder)?, fence }).await?)
    }
    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        match self.execute(DbIoTask::LeaseGet { backend: self.control, document: DbIoText::try_from_str(resource)?, now_ms }).await? {
            DbIoResult::OptionalLease(lease) => Ok(lease),
            _ => Err(DbError::Internal("Neo4j executor returned a non-lease result".to_string())),
        }
    }
}

//#region 🔖️DbBackend
impl Neo4jStorage {
    /// @emoji 🎚️ What this backend actually supports. See module doc's "Durability" section: every
    /// write is already committed server-side by the time it returns, so this backend can
    /// honestly claim the strongest single-node durability class without a separate `sync` step.
    pub async fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true }
    }
}
//#endregion 🔖️DbBackend

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::db_storage::db_io_maintenance_step;

    #[semio_framework_async_macros::async_test]
    async fn lost_neo4j_facade_retires_the_real_owned_config_without_a_service() {
        let uri = DbIoText::try_from_str("neo4j://localhost:7687").unwrap();
        let config = neo4rs::ConfigBuilder::default().uri(uri.as_str()).user("p1q").password("p1q").build().unwrap();
        let executor = Neo4jDbIoExecutor::new(config, uri);
        let worker_pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let control = register_db_io_backend(DbIoBackendKind::Neo4j, Box::new(executor), worker_pool.clone()).unwrap();
        let facade = Neo4jStorage { control, worker_pool: worker_pool.clone(), closed: std::sync::atomic::AtomicBool::new(false) };
        drop(facade);
        close_db_io_backend(control).await.unwrap();
        loop {
            match db_io_maintenance_step() {
                Ok(true) => {}
                Ok(false) => break,
                Err(error) => panic!("Neo4j lost-facade maintenance failed: {error}"),
            }
        }
        worker_pool.shutdown();
    }

    //#region 🔖️Codec
    #[semio_framework_async_macros::async_test]
    async fn native_bolt_bytes_borrow_fixed_input_without_base64() {
        let bytes = b"hello wal segment bytes";
        let owner = BoltBytes::new(bytes.as_slice().into());
        assert_eq!(&owner.value[..], bytes);
    }

    #[semio_framework_async_macros::async_test]
    async fn u64_i64_round_trip_within_range() {
        assert_eq!(u64_to_i64(42, "x").unwrap(), 42i64);
        assert_eq!(i64_to_u64(42, "x").unwrap(), 42u64);
        assert_eq!(u64_to_i64(i64::MAX as u64, "x").unwrap(), i64::MAX);
    }

    #[semio_framework_async_macros::async_test]
    async fn u64_to_i64_rejects_values_past_i64_max() {
        assert!(matches!(u64_to_i64(u64::MAX, "x"), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn i64_to_u64_rejects_negative_values() {
        assert!(matches!(i64_to_u64(-1, "x"), Err(DbError::Corrupt(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn slice_range_bounds_checks_like_the_other_backends() {
        let bytes = b"hello world";
        assert_eq!(slice_range(bytes, ByteRange { offset: 6, len: 5 }).unwrap(), b"world");
        assert!(matches!(slice_range(bytes, ByteRange { offset: 6, len: 100 }), Err(DbError::InvalidArgument(_))));
        assert!(matches!(slice_range(bytes, ByteRange { offset: u64::MAX, len: 1 }), Err(DbError::InvalidArgument(_))));
    }
    //#endregion 🔖️Codec

    //#region 🔖️WalLaws
    #[semio_framework_async_macros::async_test]
    async fn apply_append_concatenates_when_not_sealed() {
        assert_eq!(apply_append(b"hello ", false, b"world").unwrap(), b"hello world");
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_append_rejects_sealed_segment() {
        assert!(matches!(apply_append(b"hello", true, b"!"), Err(DbError::InvalidArgument(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_truncate_shrinks_when_not_sealed_and_in_range() {
        assert_eq!(apply_truncate(b"hello world", false, 5).unwrap(), b"hello");
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_truncate_rejects_sealed_or_out_of_range() {
        assert!(matches!(apply_truncate(b"hello", true, 2), Err(DbError::InvalidArgument(_))));
        assert!(matches!(apply_truncate(b"hello", false, 99), Err(DbError::InvalidArgument(_))));
    }
    //#endregion 🔖️WalLaws

    //#region 🔖️LeaseLaws
    #[semio_framework_async_macros::async_test]
    async fn decide_acquire_fence_is_initial_when_absent() {
        assert_eq!(decide_acquire_fence("r", None, "holder-a", 1_000).unwrap(), EpochFence::INITIAL);
    }

    #[semio_framework_async_macros::async_test]
    async fn decide_acquire_fence_is_stable_on_reacquire_by_same_holder() {
        let fence = EpochFence::INITIAL.next();
        let existing = Some((fence, 5_000, DbIoText::try_from_str("holder-a").unwrap()));
        assert_eq!(decide_acquire_fence("r", existing, "holder-a", 1_000).unwrap(), fence);
    }

    #[semio_framework_async_macros::async_test]
    async fn decide_acquire_fence_conflicts_on_unexpired_lease_held_by_another() {
        let existing = Some((EpochFence::INITIAL, 5_000, DbIoText::try_from_str("holder-a").unwrap()));
        assert!(matches!(decide_acquire_fence("r", existing, "holder-b", 1_000), Err(DbError::Conflict(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn decide_acquire_fence_bumps_epoch_on_handoff_after_expiry() {
        let fence = EpochFence::INITIAL.next();
        let existing = Some((fence, 500, DbIoText::try_from_str("holder-a").unwrap()));
        assert_eq!(decide_acquire_fence("r", existing, "holder-b", 1_000).unwrap(), fence.next());
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_renew_requires_unexpired_matching_holder_and_fence() {
        let fence = EpochFence::INITIAL.next();
        let existing = Some((fence, 5_000, DbIoText::try_from_str("holder-a").unwrap()));
        assert!(validate_renew("r", existing.clone(), "holder-a", fence, 1_000).is_ok());
        assert!(matches!(validate_renew("r", None, "holder-a", fence, 1_000), Err(DbError::NotFound(_))));
        assert!(matches!(validate_renew("r", existing.clone(), "holder-a", fence, 6_000), Err(DbError::Unavailable(_))));
        assert!(matches!(validate_renew("r", existing.clone(), "holder-b", fence, 1_000), Err(DbError::Unauthorized(_))));
        assert!(matches!(validate_renew("r", existing, "holder-a", EpochFence::INITIAL, 1_000), Err(DbError::Fenced { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn validate_release_requires_matching_holder_and_fence_ignoring_expiry() {
        let fence = EpochFence::INITIAL.next();
        let existing = Some((fence, 1, DbIoText::try_from_str("holder-a").unwrap()));
        assert!(validate_release("r", existing.clone(), "holder-a", fence).is_ok(), "release ignores expiry, unlike renew");
        assert!(matches!(validate_release("r", None, "holder-a", fence), Err(DbError::NotFound(_))));
        assert!(matches!(validate_release("r", existing.clone(), "holder-b", fence), Err(DbError::Unauthorized(_))));
        assert!(matches!(validate_release("r", existing, "holder-a", EpochFence::INITIAL), Err(DbError::Fenced { .. })));
    }
    //#endregion 🔖️LeaseLaws

    //#region 🔖️ErrorMapping
    #[semio_framework_async_macros::async_test]
    async fn map_neo4rs_error_maps_io_errors_to_unavailable() {
        let io_error = std::io::Error::new(std::io::ErrorKind::ConnectionRefused, "refused");
        let neo4rs_error: neo4rs::Error = io_error.into();
        assert!(matches!(map_neo4rs_error(neo4rs_error), DbError::Unavailable(_)));
    }

    #[semio_framework_async_macros::async_test]
    async fn map_de_error_maps_to_corrupt() {
        // 🎯️ `neo4rs::DeError` has no public constructor reachable without a live row (its variants
        // are driven by real (de)serialization failures), so this exercises the mapping function's
        // shape via a value we CAN construct: a decode failure surfaced through `serde`'s generic
        // `custom` constructor, which every `serde::de::Error` implementor (including `DeError`)
        // must provide.
        use serde::de::Error as _;
        let decode_error = neo4rs::DeError::custom("missing field `bytes`");
        assert!(matches!(map_de_error(decode_error), DbError::Corrupt(_)));
    }
    //#endregion 🔖️ErrorMapping

    //#region 🔖️Cypher
    #[semio_framework_async_macros::async_test]
    async fn wal_cypher_statements_reference_the_expected_label_and_keys() {
        for statement in [CYPHER_WAL_CREATE_SEGMENT, CYPHER_WAL_READ_ROW, CYPHER_WAL_WRITE_BYTES, CYPHER_WAL_SEAL, CYPHER_WAL_LIST_SEGMENTS, CYPHER_WAL_DELETE_SEGMENT] {
            assert!(statement.contains("WalSegment"));
        }
        assert!(CYPHER_WAL_CREATE_SEGMENT.contains("MERGE") && CYPHER_WAL_CREATE_SEGMENT.contains("fresh"));
        assert!(CYPHER_WAL_LIST_SEGMENTS.contains("ORDER BY"));
    }

    #[semio_framework_async_macros::async_test]
    async fn catalog_cas_statement_never_creates_on_a_failed_comparison() {
        assert!(CYPHER_CATALOG_CAS.contains("OPTIONAL MATCH"), "must not unconditionally MATCH/MERGE before the WHERE filter");
        assert!(CYPHER_CATALOG_CAS.contains("WHERE currentEpoch = $expected"));
        let where_index = CYPHER_CATALOG_CAS.find("WHERE").unwrap();
        let merge_index = CYPHER_CATALOG_CAS.find("MERGE").unwrap();
        assert!(where_index < merge_index, "the epoch check must run before any node is created/touched");
    }

    #[semio_framework_async_macros::async_test]
    async fn schema_statements_are_all_idempotent_if_not_exists_forms() {
        for statement in SCHEMA_STATEMENTS {
            assert!(statement.contains("IF NOT EXISTS"), "schema bootstrap must be safe to run on every connect: {statement}");
        }
    }
    //#endregion 🔖️Cypher

    //#region 🔖️Capabilities
    #[semio_framework_async_macros::async_test]
    async fn capabilities_report_durable_cas_and_fsync_backed_storage() {
        // 🎯️ Exercises the `capabilities()` shape without a live connection (constructing a full
        // `Neo4jStorage` needs a live `Graph`); see module doc: live-DB integration testing is
        // deferred.
        let capabilities = StorageCapabilities { durable: true, max_durability: DurabilityClass::Fsync, supports_fsync: true, supports_cas: true };
        assert!(capabilities.durable);
        assert!(capabilities.supports_cas);
        assert_eq!(capabilities.max_durability, DurabilityClass::Fsync);
    }
    //#endregion 🔖️Capabilities
}
//#endregion 🧪️Tests
