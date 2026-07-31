//! 🗄️ Db facade — re-exports the complete public surface of every `db_*` crate in the family
//! (`db_core, db_actor, db_state, db_storage, db_wal, db_snapshot, db_index, db_conflict,
//! db_projection, db_query, db_preview, db_security, db_document, db_compact, db_sync,
//! db_cluster, db_observe, db_engine`, plus the optional `db_storage_sqlite`/`db_storage_postgres`/
//! `db_storage_neo4j` backends) behind one crate. Frozen contract:
//! `.🦑repo/🎫tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, "Stable API" block).
//!
//! 🎯 Design choice (layout): the frozen `Database`/`DocumentHandle` API and its companion types
//! (`CommandReceipt`, `Frontier`, `Consistency`, `DurabilityClass`, …) already live in `db_engine`
//! — this crate promotes exactly that surface to its own root (`//#region 🔖Database`), unchanged,
//! since that is the "primary entry point" every downstream caller (`os-hub`, `compose-hub`,
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

//#region 🔖Database
/// 🗄️🚪 The frozen `Database`/`DocumentHandle` API and its companion types, promoted verbatim from
/// `db_engine` — see the module doc's "Design choice (layout)" note. This is the primary entry
/// point: `db::Database::open_at(root, db::Profile::Dev)` is the zero-touch way to stand up a
/// document database over `FsStorage`.
pub use db_engine::{
    CatalogEntry, CatalogView, CommandReceipt, Consistency, Database, DbCapabilities, DbConfig, DbHealth, DbStorage, DocumentHandle, DocumentSpec, DurabilityClass, Frontier, HistoryEntry,
    HistoryView, LiveQuery, LiveQuerySpec, PreviewHandle, Profile, Query, QueryStream, SecurityAuthzHook, SnapshotFuture, SnapshotKind, SnapshotReceipt, SubmitFuture,
};

/// 🗄️🌿 The real `vcs`-backed `db_core::VersionGraph` — the ONLY place in the whole `db` family
/// allowed to depend on `vcs` (hard dependency rule). Present exactly when this crate's own `vcs`
/// feature (default-on) is enabled, mirroring `db_engine`'s identically-named feature it forwards.
#[cfg(feature = "vcs")]
pub use db_engine::vcs_integration;

/// 🗄️ The single error type every `db_*` crate returns — re-exported at the root too (not just
/// via `db::core::DbError`) since it appears in the signature of virtually every facade-level call.
pub use db_core::DbError;

/// 🗄️#️⃣ `CommandReceipt.state_hash`'s type — hashing is pack-style `ContentHash` throughout the
/// `db` family per the contract, so it is nameable at the facade root without reaching past this
/// crate into `pack`/`pack_core` directly.
pub use pack::ContentHash;
//#endregion 🔖Database

//#region 🔖Family
/// 🗄️#️⃣ `db_core` — ids, `DbError`, limits, durability, frontier, epoch fencing, mailbox priority,
/// capabilities, config/profiles, the `VersionGraph` seam, and the `Emit` observability seam.
pub mod core {
    pub use db_core::*;
}

/// 🗄️🎭 `db_actor` — the six-lane bounded-priority mailbox actor runtime every document/catalog
/// actor in the family runs on.
pub mod actor {
    pub use db_actor::*;
}

/// 🗄️🌲 `db_state` — hand-rolled persistent (structurally-shared) diff-state overlays: `PMap`,
/// `PVec`, `PText`, `PTree`, `PGraph`, plus `TouchedRegion`/`TouchedSet`.
pub mod state {
    pub use db_state::*;
}

/// 🗄️🔌 `db_storage` — the pluggable storage substrate trait family (`DbStorage`, `WalStorage`,
/// `SnapshotStorage`, `PayloadStorage`, `CatalogStorage`, `IndexStorage`, `LeaseStorage`) plus the
/// zero-touch `MemoryStorage`/`FsStorage` backends.
pub mod storage {
    pub use db_storage::*;
}

/// 🗄️📝 `db_wal` — the family's write-ahead log: a `.spr`-container-based per-document,
/// per-segment log reusing `protocol`'s framing directly.
pub mod wal {
    pub use db_wal::*;
}

/// 🗄️📸 `db_snapshot` — pack-file-based document snapshots (`KIND_CHUNK` pages, the
/// `KIND_SNAPSHOT` descriptor segment, incremental generations via the footer chain).
pub mod snapshot {
    pub use db_snapshot::*;
}

/// 🗄️🔎 `db_index` — the secondary-index engine: LSM-lite sorted runs underneath ten typed
/// per-kind index builders.
pub mod index {
    pub use db_index::*;
}

/// 🗄️🤝 `db_conflict` — conflict detection for concurrent commands against the same document
/// frontier (touched-region intersection, bloom pre-filter, command-kind matrix, constraints).
pub mod conflict {
    pub use db_conflict::*;
}

/// 🗄️📽️ `db_projection` — the typed, versioned, dependency-DAG projection engine.
pub mod projection {
    pub use db_projection::*;
}

/// 🗄️🔍 `db_query` — consistency-mode resolution, the dynamic `Value` tree, and the
/// `Predicate`/`Select`/`Query` IR plus its planner/executor.
pub mod query {
    pub use db_query::*;
}

/// 🗄️🌫️ `db_preview` — ephemeral, speculative document overlays: identity, lifecycle,
/// coalescing, TTL, and reconciliation on frontier advance.
pub mod preview {
    pub use db_preview::*;
}

/// 🗄️🛂 `db_security` — multi-granularity authz, the signing bridge, replay guard, DoS budgets,
/// and field redaction.
pub mod security {
    pub use db_security::*;
}

/// 🗄️🏛️ `db_document` — the document authority actor: admit → dedupe → base-resolve → authz →
/// deps → validate → conflict → execute → WAL append → durability → publish → project → vcs →
/// preview-reconcile → receipt.
pub mod document {
    pub use db_document::*;
}

/// 🗄️🧹 `db_compact` — WAL segment retention, payload GC, index compaction, and snapshot chain
/// consolidation.
pub mod compact {
    pub use db_compact::*;
}

/// 🗄️🔁 `db_sync` — server side of `protocol_wire`: frontier exchange, missing-command transfer,
/// snapshot bootstrap, and resume tokens.
pub mod sync {
    pub use db_sync::*;
}

/// 🗄️🕸️ `db_cluster` — sharding, ownership leases with epoch failover, follower replication,
/// quorum durability, and read/preview routing.
pub mod cluster {
    pub use db_cluster::*;
}

/// 🗄️📡 `db_observe` — structured/audit event sinks, metrics, spans, health, and a determinism
/// verifier.
pub mod observe {
    pub use db_observe::*;
}

/// 🗄️🗄️ `db_engine` — the `Database` supervisor/catalog actor, exposed here in full (beyond the
/// primary-entry-point names already promoted to the crate root above) for callers that want its
/// non-primary items (e.g. `db::engine::CatalogEntry` reached through this path instead of the
/// root — both resolve to the same type).
pub mod engine {
    pub use db_engine::*;
}
//#endregion 🔖Family

//#region 🔖StorageBackends
/// 🗄️🪶 `db_storage_sqlite` — the optional SQLite-backed `DbStorage` implementation (behind this
/// crate's `sqlite` feature), linking the same bundled `rusqlite` version `vcs` already uses.
#[cfg(feature = "sqlite")]
pub mod storage_sqlite {
    pub use db_storage_sqlite::*;
}

/// 🗄️🐘 `db_storage_postgres` — the optional Postgres-backed `DbStorage` implementation (behind
/// this crate's `postgres` feature).
#[cfg(feature = "postgres")]
pub mod storage_postgres {
    pub use db_storage_postgres::*;
}

/// 🗄️🕸️ `db_storage_neo4j` — the optional Neo4j-backed `DbStorage` implementation (behind this
/// crate's `neo4j` feature).
#[cfg(feature = "neo4j")]
pub mod storage_neo4j {
    pub use db_storage_neo4j::*;
}
//#endregion 🔖StorageBackends

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸Fixtures
    fn tempdir(name: &str) -> std::path::PathBuf {
        let mut dir = std::env::temp_dir();
        dir.push(format!("db-facade-test-{name}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn envelope(id: &str, deps: &[&str], actor: &str, document: &protocol::DocumentId, entries: &[(&str, serde_json::Value)]) -> protocol::OperationEnvelope {
        let mut payload = serde_json::Map::new();
        for (path, value) in entries {
            payload.insert((*path).to_string(), value.clone());
        }
        protocol::OperationEnvelope {
            operation_id: protocol::OperationId(id.to_string()),
            document_id: document.clone(),
            actor: protocol::ActorId(actor.to_string()),
            dependencies: deps.iter().map(|dep| protocol::OperationId((*dep).to_string())).collect(),
            diff: protocol::DocumentDiff {
                schema: protocol::SchemaId(document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::Value::Object(payload)).unwrap(),
            },
            inverse: protocol::InverseOperation {
                schema: protocol::SchemaId(document::DB_PATHMAP_SCHEMA.to_string()),
                payload: serde_json::to_vec(&serde_json::Value::Object(serde_json::Map::new())).unwrap(),
            },
            timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        }
    }
    //#endregion 🧸Fixtures

    //#region 🔖Facade round trip
    /// @emoji 🧪 The facade's own core law: everything needed for a real submit -> durable ->
    /// query -> frontier -> history round trip is reachable through THIS crate's re-exported
    /// names alone (`db::Database`, `db::DocumentSpec`, `db::Consistency`, `db::Query`,
    /// `db::document::CommandBatch`/`SubmitOptions`) — never by reaching past the facade into
    /// `db_engine`/`db_document` directly. A rename or a dropped re-export in `//#region
    /// 🔖Database`/`//#region 🔖Family` would fail this test to compile.
    #[test]
    fn full_round_trip_reachable_purely_through_facade_reexports() {
        let root = tempdir("round-trip");
        let database = Database::open_at(&root, Profile::Dev).unwrap();
        let document = protocol::DocumentId("doc-1".to_string());
        let handle = database.create_document(DocumentSpec::new(document.clone())).unwrap();

        let batch = document::CommandBatch::new(vec![envelope("op-1", &[], "alice", &document, &[("name", serde_json::json!("hello"))])]).unwrap();
        let receipt = actor::block_on(handle.submit(batch, document::SubmitOptions { durability: DurabilityClass::Fsync })).unwrap().unwrap();
        assert_eq!(receipt.command_id, protocol::OperationId("op-1".to_string()));
        assert_eq!(receipt.frontier.head_seq, 1);
        assert!(receipt.conflicts.is_empty());

        let queried = handle.query(Query::Get { path: "name".to_string() }, Consistency::Canonical).unwrap();
        let value: serde_json::Value = serde_json::from_slice(queried.results[0].1.as_ref().unwrap()).unwrap();
        assert_eq!(value, serde_json::json!("hello"));

        let frontier = handle.frontier().unwrap();
        assert!(frontier.dominates(&receipt.frontier).unwrap());

        let history = handle.history().unwrap();
        assert_eq!(history.entries.len(), 1);

        assert_eq!(database.catalog().documents.len(), 1);
        database.shutdown(std::time::Duration::from_secs(1)).unwrap();
    }

    #[test]
    fn database_error_type_is_reachable_at_the_facade_root() {
        let root = tempdir("db-error");
        let database = Database::open_at(&root, Profile::Test).unwrap();
        let result = database.document(&protocol::DocumentId("never-created".to_string()));
        assert!(matches!(result, Err(DbError::NotFound(_))));
    }
    //#endregion 🔖Facade round trip

    //#region 🔖Family submodule smoke
    /// @emoji 🧪 One representative construction per `db_*` family submodule, proving the facade's
    /// glob re-exports actually surface each crate's headline public items at the path this
    /// crate's module doc promises (`db::core::…`, `db::state::…`, …) — a wiring/rename regression
    /// in `//#region 🔖Family` breaks this test to compile, independent of any single crate's own
    /// internal test suite.
    #[test]
    fn every_family_submodule_reexports_its_headline_public_surface() {
        let limits = core::DbLimits::default();
        assert!(limits.max_command_bytes > 0);
        core::check_len(1, 2, "smoke").unwrap();

        let (_address, _receiver) = actor::mailbox::<u8>(core::MailboxCapacities::default());

        let mut map: state::PMap<String, i32> = state::PMap::new();
        map = map.insert("k".to_string(), 1);
        assert_eq!(map.len(), 1);

        let memory_storage = storage::MemoryStorage::new();
        let _: &dyn DbStorage = &memory_storage;

        assert_eq!(wal::WAL_SEGMENT_HEADER, 0x40);
        assert!(wal::is_wal_record_kind(wal::WAL_COMMAND));

        assert_eq!(snapshot::SnapshotOrigin::FullBaseline, snapshot::SnapshotOrigin::FullBaseline);

        assert_eq!(index::IndexKind::ALL.len(), 10);

        let touch = conflict::CommandKind::from("write");
        assert_eq!(touch.0, "write");

        let value: query::Value = 42i64.into();
        assert_eq!(value, query::Value::Int(42));

        let preview_id = preview::PreviewId("preview-1".to_string());
        assert_eq!(preview_id.to_string(), "preview-1");

        let tenant = security::TenantId::from("tenant-1");
        assert_eq!(tenant.to_string(), "tenant-1");

        let budget = compact::CompactionBudget::default();
        assert!(budget.max_wal_segments > 0);

        let node = cluster::NodeId::from("node-1");
        assert_eq!(node.to_string(), "node-1");

        let sink = observe::MemorySink::new();
        assert!(sink.lines().is_empty());

        let from = core::Frontier::genesis(core::DocumentId("doc-1".to_string()));
        let to = core::Frontier { head_seq: 3, commit_seq: 3, ..core::Frontier::genesis(core::DocumentId("doc-1".to_string())) };
        let delta = sync::frontier_delta(&from, &to).unwrap();
        assert_eq!(delta.commands, 3);
    }
    //#endregion 🔖Family submodule smoke
}
//#endregion 🧪Tests
