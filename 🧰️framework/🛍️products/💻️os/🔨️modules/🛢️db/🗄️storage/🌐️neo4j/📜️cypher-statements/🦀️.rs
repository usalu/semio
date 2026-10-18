//! 📜️ Cypher statements: the schema statements and every Cypher text of the WAL, snapshot, payload, catalog, index, writer and lease rows. See [`super`].

use super::*;

//#region 🔖️Cypher
/// 🧱️ Uniqueness constraints (single-property, Community-Edition-compatible) and
/// composite lookup indexes bootstrapped once at `Neo4jStorage::connect`. Composite *uniqueness*
/// constraints need Enterprise Edition; multi-key node identity (`WalSegment`/`SnapshotGeneration`/
/// `IndexRun`) is instead guaranteed by always addressing those nodes via `MERGE` on their full
/// key (document + index), which is correct regardless of a DB-level constraint — these indexes
/// are a read-performance aid only, not a correctness dependency.
pub(super) const SCHEMA_STATEMENTS: &[&str] = &[
    "CREATE CONSTRAINT db_payload_hash IF NOT EXISTS FOR (p:Payload) REQUIRE p.hash IS UNIQUE",
    "CREATE CONSTRAINT db_lease_resource IF NOT EXISTS FOR (l:Lease) REQUIRE l.resource IS UNIQUE",
    "CREATE CONSTRAINT db_catalog_root_id IF NOT EXISTS FOR (c:CatalogRoot) REQUIRE c.id IS UNIQUE",
    "CREATE CONSTRAINT db_wal_writer_document IF NOT EXISTS FOR (w:WalWriter) REQUIRE w.document IS UNIQUE",
    "CREATE INDEX db_wal_segment_lookup IF NOT EXISTS FOR (n:WalSegment) ON (n.document, n.segIndex)",
    "CREATE INDEX db_snapshot_generation_lookup IF NOT EXISTS FOR (n:SnapshotGeneration) ON (n.document, n.generation)",
    "CREATE INDEX db_index_run_lookup IF NOT EXISTS FOR (n:IndexRun) ON (n.document, n.runId)",
];

//#region 🔖️WalCypher
pub(super) const CYPHER_WAL_CREATE_SEGMENT: &str = "
    MERGE (n:WalSegment {document: $document, segIndex: $index})
    ON CREATE SET n.bytes = $empty, n.sealed = false, n.len = 0, n.fresh = true
    ON MATCH SET n.fresh = false
    RETURN n.fresh AS fresh";

pub(super) const CYPHER_WAL_READ_ROW: &str = "
    MATCH (n:WalSegment {document: $document, segIndex: $index})
    RETURN n.bytes AS bytes, n.sealed AS sealed, n.len AS len";

pub(super) const CYPHER_WAL_STATE: &str = "
    MATCH (n:WalSegment {document: $document, segIndex: $index})
    RETURN n.sealed AS sealed";

pub(super) fn neo4j_wal_segment_state(sealed: bool) -> WalSegmentState {
    if sealed {
        WalSegmentState::Sealed
    } else {
        WalSegmentState::Active
    }
}

pub(super) const CYPHER_WAL_WRITE_BYTES: &str = "
    MATCH (n:WalSegment {document: $document, segIndex: $index})
    SET n.bytes = $bytes, n.len = $len";

pub(super) const CYPHER_WAL_SEAL: &str = "
    MATCH (n:WalSegment {document: $document, segIndex: $index})
    SET n.sealed = true
    RETURN n.sealed AS sealed";

pub(super) const CYPHER_WAL_LIST_SEGMENTS: &str = "
    MATCH (n:WalSegment {document: $document})
    RETURN n.segIndex AS segIndex ORDER BY n.segIndex ASC";

pub(super) const CYPHER_WAL_DELETE_SEGMENT: &str = "
    MATCH (n:WalSegment {document: $document, segIndex: $index})
    DETACH DELETE n";
//#endregion 🔖️WalCypher

//#region 🔖️SnapshotCypher
pub(super) const CYPHER_SNAPSHOT_WRITE: &str = "
    MERGE (n:SnapshotGeneration {document: $document, generation: $generation})
    SET n.bytes = $bytes, n.len = $len";

pub(super) const CYPHER_SNAPSHOT_READ: &str = "
    MATCH (n:SnapshotGeneration {document: $document, generation: $generation})
    RETURN n.bytes AS bytes, n.len AS len";

pub(super) const CYPHER_SNAPSHOT_LATEST: &str = "
    MATCH (n:SnapshotGeneration {document: $document})
    RETURN max(n.generation) AS maxGeneration";

pub(super) const CYPHER_SNAPSHOT_LIST: &str = "
    MATCH (n:SnapshotGeneration {document: $document})
    RETURN n.generation AS generation ORDER BY n.generation ASC";

pub(super) const CYPHER_SNAPSHOT_DELETE: &str = "
    MATCH (n:SnapshotGeneration {document: $document, generation: $generation})
    DETACH DELETE n";
//#endregion 🔖️SnapshotCypher

//#region 🔖️PayloadCypher
pub(super) const CYPHER_PAYLOAD_PUT: &str = "
    MERGE (p:Payload {hash: $hash})
    ON CREATE SET p.bytes = $bytes, p.len = $len";

pub(super) const CYPHER_PAYLOAD_GET: &str = "
    MATCH (p:Payload {hash: $hash})
    RETURN p.bytes AS bytes, p.len AS len";

pub(super) const CYPHER_PAYLOAD_CONTAINS: &str = "
    MATCH (p:Payload {hash: $hash})
    RETURN count(p) AS c";

pub(super) const CYPHER_PAYLOAD_LEN: &str = "
    MATCH (p:Payload {hash: $hash})
    RETURN p.len AS len";

pub(super) const CYPHER_PAYLOAD_DELETE: &str = "
    MATCH (p:Payload {hash: $hash})
    DETACH DELETE p";
//#endregion 🔖️PayloadCypher

//#region 🔖️CatalogCypher
pub(super) const CYPHER_CATALOG_READ: &str = "
    MATCH (c:CatalogRoot {id: 'root'})
    RETURN c.epoch AS epoch, c.bytes AS bytes, c.len AS len";

/// ✅️ Single-statement compare-and-swap: `OPTIONAL MATCH` never creates a node, so a failed
/// comparison (the `WHERE` filters the only row away) leaves the graph untouched — `CatalogRoot`
/// only ever comes into existence via a SUCCEEDING `cas_root`, matching `read_root`'s "`None` until
/// `cas_root` has succeeded once" contract exactly. `coalesce(c.epoch, 0)` treats a not-yet-created
/// root as being at `EpochFence::INITIAL`, mirroring `MemoryStorage`/`FsStorage`'s identical
/// "absent root == epoch 0" convention.
pub(super) const CYPHER_CATALOG_CAS: &str = "
    OPTIONAL MATCH (c:CatalogRoot {id: 'root'})
    WITH coalesce(c.epoch, 0) AS currentEpoch
    WHERE currentEpoch = $expected
    MERGE (n:CatalogRoot {id: 'root'})
    SET n.epoch = $newEpoch, n.bytes = $bytes, n.len = $len
    RETURN n.epoch AS epoch";
//#endregion 🔖️CatalogCypher

//#region 🔖️IndexCypher
pub(super) const CYPHER_INDEX_WRITE: &str = "
    MERGE (n:IndexRun {document: $document, runId: $runId})
    SET n.bytes = $bytes, n.len = $len";

pub(super) const CYPHER_INDEX_READ: &str = "
    MATCH (n:IndexRun {document: $document, runId: $runId})
    RETURN n.bytes AS bytes, n.len AS len";

pub(super) const CYPHER_INDEX_LIST: &str = "
    MATCH (n:IndexRun {document: $document})
    RETURN n.runId AS runId ORDER BY n.runId ASC";

pub(super) const CYPHER_INDEX_DELETE: &str = "
    MATCH (n:IndexRun {document: $document, runId: $runId})
    DETACH DELETE n";
//#endregion 🔖️IndexCypher

//#region 🔖️WalWriterCypher
/// 🔐️ Claims the document's writer lease. The first `SET` write-locks the lease node before
/// anything is read, so two contenders serialize on it and the second sees the first's commit; only
/// an absent or expired lease is claimed, and every claim advances the fencing token.
pub(super) const CYPHER_WAL_WRITER_ACQUIRE: &str = "
    MERGE (w:WalWriter {document: $document})
    ON CREATE SET w.token = 0, w.expiresAtMs = 0
    SET w.contendedAtMs = timestamp()
    WITH w
    WHERE w.holder IS NULL OR w.expiresAtMs <= timestamp()
    SET w.holder = $holder, w.token = w.token + 1, w.expiresAtMs = timestamp() + $ttlMs
    RETURN w.token AS token";

/// 💓 Extends a still-owned lease; no row means the lease was lost.
pub(super) const CYPHER_WAL_WRITER_RENEW: &str = "
    MATCH (w:WalWriter {document: $document})
    SET w.renewedAtMs = timestamp()
    WITH w
    WHERE w.holder = $holder AND w.token = $token AND w.expiresAtMs > timestamp()
    SET w.expiresAtMs = timestamp() + $ttlMs
    RETURN w.token AS token";

/// 🛡️ The first statement of every WAL mutation transaction: write-locks the lease node and
/// proves ownership (renewing it) before any segment is touched.
pub(super) const CYPHER_WAL_WRITER_FENCE: &str = "
    MATCH (w:WalWriter {document: $document})
    SET w.fencedAtMs = timestamp()
    WITH w, (w.holder = $holder AND w.token = $token AND w.expiresAtMs > timestamp()) AS owned
    SET w.expiresAtMs = CASE WHEN owned THEN timestamp() + $ttlMs ELSE w.expiresAtMs END
    RETURN owned, w.token AS token";

/// 🕊️ Frees an owned lease; the node and its token stay so the next claim advances it.
pub(super) const CYPHER_WAL_WRITER_RELEASE: &str = "
    MATCH (w:WalWriter {document: $document})
    SET w.releasedAtMs = timestamp()
    WITH w
    WHERE w.holder = $holder AND w.token = $token
    SET w.holder = null, w.expiresAtMs = 0";
//#endregion 🔖️WalWriterCypher

//#region 🔖️LeaseCypher
pub(super) const CYPHER_LEASE_READ: &str = "
    MATCH (l:Lease {resource: $resource})
    RETURN l.holder AS holder, l.epoch AS epoch, l.expiresAtMs AS expiresAtMs";

pub(super) const CYPHER_LEASE_WRITE: &str = "
    MERGE (l:Lease {resource: $resource})
    SET l.holder = $holder, l.epoch = $epoch, l.expiresAtMs = $expiresAtMs";

pub(super) const CYPHER_LEASE_DELETE: &str = "
    MATCH (l:Lease {resource: $resource})
    DETACH DELETE l";
//#endregion 🔖️LeaseCypher
//#endregion 🔖️Cypher
