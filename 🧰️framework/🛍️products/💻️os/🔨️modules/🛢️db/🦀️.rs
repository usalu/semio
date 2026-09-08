//! 🗄️ Db facade — re-exports the complete public surface of every `db_*` crate in the family
//! (`db_core, db_actor, db_state, db_storage, db_wal, db_snapshot, db_index, db_conflict,
//! db_projection, db_query, db_preview, db_security, db_artifact, db_compact, db_sync,
//! db_cluster, db_observe, db_engine`, plus the optional `db_storage_sqlite`/`db_storage_postgres`/
//! `db_storage_neo4j` backends) behind one crate. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, "Stable API" block).
//!
//! 🎯️ Design choice (layout): the frozen `Database`/`ArtifactHandle` API and its companion types
//! (`CommandReceipt`, `Frontier`, `Consistency`, `DurabilityClass`, …) already live in `db_engine`
//! — this crate promotes exactly that surface to its own root (`//#region 🔖️Database`), unchanged,
//! since that is the "primary entry point" every downstream caller (`os-semio_hub`, `semio_compose_rs-semio_hub`,
//! plugin crates) is meant to reach for. `db_engine` already exposes `Database::open_at` verbatim
//! (`FsStorage`, zero-touch) — no extra convenience wrapper is needed here. Every other `db_*`
//! crate is additionally reachable in full through a same-named (minus the `db_` prefix)
//! submodule (`db::core`, `db::state`, `db::query`, …) so nothing in the family is hidden behind
//! the facade; a caller who only needs `db::Database` never has to look past the root, and a
//! caller who needs a lower crate's own types (e.g. `db::state::PMap` for a custom projection)
//! finds them at the obvious path. Submodules are plain `pub use crate_name::*;` glob re-exports —
//! every one of their crates is a mandatory (non-optional) dependency of this facade per its
//! `Cargo.toml`, so no submodule needs its own `#[cfg(feature = ...)]` gate; only the three
//! swappable storage *backends* (`sqlite`/`postgres`/`neo4j`) are truly optional dependencies, so
//! only their submodules are feature-gated, matching this crate's own `Cargo.toml` feature names.

//#region 🔖️Database
/// 🗄️🚪️ The frozen `Database`/`ArtifactHandle` API and its companion types, promoted verbatim from
/// `db_engine` — see the module doc's "Design choice (layout)" note. This is the primary entry
/// point: `db::Database::open_at(pool, root, db::Profile::Dev)` stands up a
/// document database over `FsStorage`.
pub use crate::db_engine::{
    take_database_capability_open_terminal, take_database_catalog_read_terminal, take_next_database_capability_open_terminal, ArtifactHandle, ArtifactHistoryTerminalConstructionFault, ArtifactHistoryTerminalHandle, ArtifactSpec, CatalogEntry,
    CatalogView, CheckpointPublicationSnapshot, CommandReceipt, Consistency, Database, DatabaseCapabilityOpenCloseStep, DatabaseCapabilityOpenFuture, DatabaseCapabilityOpenProgress, DatabaseCapabilityOpenRejected, DatabaseCapabilityOpenResult,
    DatabaseCapabilityOpenTerminalHandle, DatabaseCapabilityOpenTerminalResult, DatabaseCatalogReadCloseStep, DatabaseCatalogReadFuture, DatabaseCatalogReadProgress, DatabaseCatalogReadRejected, DatabaseCatalogReadResult,
    DatabaseCatalogReadTerminalHandle, DatabaseCatalogReadTerminalResult, DatabaseCatalogRootKey, DatabaseDocumentOpenRejected, DatabaseRetainedActivityRejected, DatabaseShutdownBlock, DatabaseShutdownControl, DatabaseShutdownPhase, DatabaseShutdownProgress, DbHealth, HistoryEntry, HistoryView, LiveQuery, LiveQuerySpec, PreviewHandle, Query, QueryResultEntry, QueryStream, SecurityAuthzHook, SnapshotFuture,
    SnapshotKind, SnapshotReceipt, SubmitFuture,
};

/// 🗄️🌿️ The real `vcs`-backed `VersionGraph` — the ONLY place in the whole `db` family
/// allowed to depend on `vcs` (hard dependency rule). Present exactly when this crate's own `vcs`
/// feature (default-on) is enabled, mirroring `db_engine`'s identically-named feature it forwards.
#[cfg(feature = "vcs")]
pub use crate::db_engine::vcs_integration;

//#endregion 🔖️Database

//#region 🔖️Family
/// 🗄️#⃣ Former `db_core` surface — ids, durability, policy, and version-graph seams.
pub mod ids {
    pub use crate::db_ids::*;
}

pub mod durability {
    pub use crate::db_durability::*;
}

pub mod policy {
    pub use crate::db_policy::*;
}

pub mod version_graph {
    pub use crate::db_version_graph::*;
}

/// 🗄️🎭️ `db_actor` — the six-lane bounded-priority mailbox actor runtime every document/catalog
/// actor in the family runs on.
pub mod actor {
    pub use crate::db_actor::*;
}

/// 🗄️🌲️ `db_state` — hand-rolled persistent (structurally-shared) diff-state overlays: `PMap`,
/// `PVec`, `PText`, `PTree`, `PGraph`, plus `TouchedRegion`/`TouchedSet`.
pub mod state {
    pub use crate::db_state::*;
}

/// 🗄️🔌️ `db_storage` — the pluggable storage substrate trait family (`DbStorage`, `WalStorage`,
/// `SnapshotStorage`, `PayloadStorage`, `CatalogStorage`, `IndexStorage`, `LeaseStorage`) plus the
/// zero-touch `MemoryStorage`/`FsStorage` backends.
pub mod storage {
    pub use crate::db_storage::*;
}

/// 🗄️📝️ `db_wal` — the family's write-ahead log: a `.spr`-container-based per-document,
/// per-segment log reusing `protocol`'s framing directly.
pub mod wal {
    pub use crate::db_wal::*;
}

/// 🗄️📸️ `db_snapshot` — pack-file-based document snapshots (`KIND_CHUNK` pages, the
/// `KIND_SNAPSHOT` descriptor segment, incremental generations via the footer chain).
pub mod snapshot {
    pub use crate::db_snapshot::*;
}

/// 🗄️🔎️ `db_index` — the secondary-index engine: LSM-lite sorted runs underneath ten typed
/// per-kind index builders.
pub mod index {
    pub use crate::db_index::*;
}

/// 🗄️🤝️ `db_conflict` — conflict detection for concurrent commands against the same document
/// frontier (touched-region intersection, bloom pre-filter, command-kind matrix, constraints).
pub mod conflict {
    pub use crate::db_conflict::*;
}

/// 🗄️📽️ `db_projection` — the typed, versioned, dependency-DAG projection engine.
pub mod projection {
    pub use crate::db_projection::*;
}

/// 🗄️🔍️ `db_query` — consistency-mode resolution, the dynamic `Value` tree, and the
/// `Predicate`/`Select`/`Query` IR plus its planner/executor.
pub mod query {
    pub use crate::db_query::*;
}

/// 🗄️🌫️ `db_preview` — ephemeral, speculative document overlays: identity, lifecycle,
/// coalescing, TTL, and reconciliation on frontier advance.
pub mod preview {
    pub use crate::db_preview::*;
}

/// 🗄️🛂️ `db_security` — multi-granularity authz, the signing bridge, replay guard, DoS budgets,
/// and field redaction.
pub mod security {
    pub use crate::db_security::*;
}

/// 🗄️🏛️ `db_artifact` — the document authority actor: admit → dedupe → base-resolve → authz →
/// deps → validate → conflict → execute → WAL append → durability → publish → project → vcs →
/// preview-reconcile → receipt.
pub mod document {
    pub use crate::db_artifact::*;
}

/// 🗄️🧹️ `db_compact` — WAL segment retention, payload GC, index compaction, and snapshot chain
/// consolidation.
pub mod compact {
    pub use crate::db_compact::*;
}

/// 🗄️🔁️ `db_sync` — server side of `protocol_wire`: frontier exchange, missing-command transfer,
/// snapshot bootstrap, and resume tokens.
pub mod sync {
    pub use crate::db_sync::*;
}

/// 🗄️🕸️ `db_cluster` — sharding, ownership leases with epoch failover, follower replication,
/// quorum durability, and read/preview routing.
pub mod cluster {
    pub use crate::db_cluster::*;
}

/// 🗄️📡️ `db_observe` — structured/audit event sinks, metrics, spans, health, and a determinism
/// verifier.
pub mod observe {
    pub use crate::db_observe::*;
}

/// 🗄️ `db_engine` — the `Database` supervisor/catalog actor, exposed here in full (beyond the
/// primary-entry-point names already promoted to the crate root above) for callers that want its
/// non-primary items (e.g. `db::engine::CatalogEntry` reached through this path instead of the
/// root — both resolve to the same type).
pub mod engine {
    pub use crate::db_engine::*;
}
//#endregion 🔖️Family

//#region 🔖️StorageBackends
/// 🗄️🪶️ `db_storage_sqlite` — the optional SQLite-backed `DbStorage` implementation (behind this
/// crate's `sqlite` feature), linking the same bundled `rusqlite` version `vcs` already uses.
#[cfg(feature = "sqlite")]
pub mod storage_sqlite {
    pub use crate::db_storage_sqlite::*;
}

/// 🗄️🐘️ `db_storage_postgres` — the optional Postgres-backed `DbStorage` implementation (behind
/// this crate's `postgres` feature).
#[cfg(feature = "postgres")]
pub mod storage_postgres {
    pub use crate::db_storage_postgres::*;
}

/// 🗄️🕸️ `db_storage_neo4j` — the optional Neo4j-backed `DbStorage` implementation (behind this
/// crate's `neo4j` feature).
#[cfg(feature = "neo4j")]
pub mod storage_neo4j {
    pub use crate::db_storage_neo4j::*;
}
//#endregion 🔖️StorageBackends

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
