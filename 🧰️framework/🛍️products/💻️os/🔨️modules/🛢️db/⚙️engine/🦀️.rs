//! 🗄️ `db_engine` — the `Database` supervisor and catalog actor: the crate that assembles every
//! other `db_*` crate into the stable, contract-frozen `Database`/`ArtifactHandle` API
//! (`Database::{open, open_at, create_document, document, catalog, health, shutdown}`;
//! `ArtifactHandle::{submit, query, subscribe, frontier, preview, history, snapshot_now}`).
//! Frozen contract: `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_engine` row + "Stable API" block).
//!
//! 🎯️ Design choice (compatibility surface): `db_artifact` (a concurrent sibling session) commits
//! explicitly, in its own module doc, to keeping its local
//! `ConflictRecord{command_id, conflicting_with, path}` shape byte-for-byte stable specifically
//! because THIS crate constructs every one of those verbatim. `SubmitOptions{durability, policy}`
//! and `CommandReceipt{.., messages}` both gained a field under
//! `MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C9 — every construction site below
//! and `to_engine_receipt`'s field-for-field bridge were updated in lockstep (never a `..Default`
//! spread over a frozen-shape struct, so a future field addition here fails loud, not silent).
//!
//! 🎯️ Design choice (scope): per this wave's instructions, this crate makes `Database::open_at`
//! (zero-touch `FsStorage`) and a full submit → durable → query round trip over a REAL
//! `db_artifact::ArtifactAuthority` genuinely work end to end (see `//#region 🧪️Tests`), composing
//! the guaranteed-complete `db_state`/`db_wal`/`db_storage`/`db_artifact` crates against their real,
//! current APIs throughout. `db_cluster` is still an unimplemented stub upstream of this wave (its
//! `lib.rs` declares no public items at all) — nothing in this crate can call into it yet; every
//! cluster-shaped concern (sharding, ownership leases, quorum durability, split-brain repair) is
//! deferred wholesale, documented here rather than faked. `db_compact`/`db_sync`/`db_security`/
//! `db_observe` ARE genuinely wired, but narrowly: `Database::compact_document` drives a real
//! retained compaction pass, and `Database::hello` mounts `db_sync::DatabaseSyncHelloFuture` for the wire-v2
//! handshake (no transport of its own — that is CW5/CW6's job), and `Database::open`/`open_at` wire a
//! real `db_observe::StructuredSink`/`HealthRegistry` pair by default. `ArtifactHandle::preview`/
//! `subscribe` return `DbError::Unimplemented` (not a panic, not a fake success): `db_artifact`'s
//! own `ArtifactAuthority` mailbox (`db/document/rs/lib.rs`'s `ArtifactMessage` enum) only carries
//! `Submit`/`Query`/`Frontier` variants — there is no way to drive its preview/commit-log machinery
//! through the actor boundary without editing `db_artifact` itself, which is out of this crate's
//! ownership this wave. `snapshot_now` is likewise `Unimplemented`: `db_artifact`'s own module doc
//! documents that `DocumentState` materializes purely from the WAL suffix with no full-state
//! enumeration to serialize into a pack snapshot, and `db_snapshot` is not even a direct dependency
//! of this crate per its `Cargo.toml`. `ArtifactHandle::history` replays the WAL through the
//! document authority's retained cursor: the WAL is the only record of a document's history.
#![allow(async_fn_in_trait)]
#![cfg(not(target_arch = "wasm32"))]

use db_durability::EpochFence;
#[cfg(test)]
use db_engine_base::DatabaseDocumentMountPublishedHook;
use db_engine_base::{CatalogState, CatalogView, DatabaseDocumentMountFailure, DatabaseDocumentMountFuture, DatabaseDocumentMountOwner, DatabaseDocumentMountPolicy, DatabaseDocumentMountRegistry, DatabaseDocumentMountReply, DatabaseDocumentMountSlot, DatabaseDocumentMountWait, DatabaseDocumentMountWaiter, database_document_mount_work_live_slots, decode_catalog, default_emit, encode_catalog_pages, now_ms, to_core_document_id};
use db_engine_capability_open::{DATABASE_CAPABILITY_OPEN_LIVE, DatabaseCapabilityOpenFuture, DatabaseCapabilityOpenRejected, database_capability_open_registry};
use db_engine_catalog_bootstrap::{DATABASE_CATALOG_BOOTSTRAP_LIVE, DatabaseCatalogBootstrapFuture, DatabaseCatalogBootstrapKey, DatabaseCatalogBootstrapRejected, database_catalog_bootstrap_registry};
use db_engine_catalog_create::{DATABASE_CREATE_CATALOG_LIVE, DatabaseCreateCatalogFuture, DatabaseCreateCatalogRejected, database_create_catalog_registry};
use db_engine_catalog_read::{DATABASE_CATALOG_READ_LIVE, DatabaseCatalogReadFuture, DatabaseCatalogReadRejected, DatabaseCatalogRootKey, database_catalog_read_registry};
use db_engine_handle::{ArtifactCompactionFuture, ArtifactHandle, ArtifactHandleLease};
use db_ids::{ActorId, ArtifactId, DbError};
use db_io_core::CatalogStorage as _;
use db_observe::{Emit, EmitEvent, EmitField};
use db_policy::MailboxCapacities;
use semio_framework_async::{WorkerPool, WorkerPoolUse};
use std::{sync::Arc, sync::Mutex};
#[cfg(test)]
use db_engine_base::{Consistency, DATABASE_DOCUMENT_MOUNT_WAITERS, DatabaseDocumentMountCleanupFaultHook, DatabaseDocumentMountDriver, DatabaseDocumentMountParkedHook, DatabaseDocumentMountWork, LiveQuerySpec, Query, QueryResultEntry, QueryStream, SnapshotKind, test_worker_pool};
#[cfg(test)]
use db_engine_handle::{ARTIFACT_HISTORY_ADMISSION, ARTIFACT_HISTORY_OPERATION_SLOTS, ArtifactHistoryAdmission, ArtifactHistoryAdmissionError, ArtifactHistoryState, HistoryFuture};
#[cfg(test)]
use db_ids::GenerationId;
#[cfg(test)]
use semio_framework_async::Lane;
#[cfg(test)]
use std::future::Future;

//#region 🔖️Reexports
pub use db_durability::DurabilityClass;

pub use db_policy::{DbCapabilities, DbConfig, Profile};
//#endregion 🔖️Reexports

#[path = "🩺️database-spec-health-and-shutdown/🦀️.rs"]
mod database_spec_health_and_shutdown;
pub use database_spec_health_and_shutdown::{ArtifactSpec, DatabaseShutdownBlock, DatabaseShutdownControl, DatabaseShutdownPhase, DatabaseShutdownProgress, DbHealth};
use database_spec_health_and_shutdown::{
    DatabaseMountFutureLiveGuardV1, DatabasePoolUseSiteGuardV1, DatabaseRetainedPoolUseCensus, DatabaseShutdownInterruptionWitness, DATABASE_POOL_USE_MOUNT_WAIT_LIVE,
};

#[path = "🚫️database-open-rejections/🦀️.rs"]
mod database_open_rejections;
pub use database_open_rejections::{DatabaseDocumentOpenRejected, DatabaseRetainedActivityRejected};

#[path = "🧷️database-handle/🦀️.rs"]
mod database_handle;
pub use database_handle::{Database, DatabaseOpenAtRejected};

#[path = "🔓️database-open-and-mount/🦀️.rs"]
mod database_open_and_mount;

#[path = "🛠️database-catalog-shutdown-and-hello/🦀️.rs"]
mod database_catalog_shutdown_and_hello;
pub use database_catalog_shutdown_and_hello::DATABASE_HELLO_ADMISSION_WAIT_MS;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/⏱️throughput/🦀️.rs"]
mod throughput_tests;
//#endregion 🧪️Tests
