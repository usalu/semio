//! 🗄️ 🧹️ `db_compact` — the `db` crate family's compaction and GC engine: WAL segment retention
//! (folding the "merge/recompress" goal into deleting whole snapshot-covered sealed segments;
//! the committed cursor's complete segment ledger keeps its highest live segment protected, and
//! `db_wal`'s own `.spr` segments accept only its own critical `WAL_*` record kinds — see the
//! `//#region 🔖️WalRetention` doc for the full rationale), snapshot chain consolidation into a
//! fresh full baseline, ref-traced payload GC, index-kind merge (which also physically drops
//! shadowed tombstones — `db_compact`'s share of "tombstone/preview GC"), cold-tier pack
//! archival, and a fenced, budgeted top-level `Compactor` orchestrator. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_compact` row).
//!
//! 🎯️ Scope boundary — "diff collapse": per the contract's hard dependency rule, no db crate
//! below `db_artifact` interprets operation semantics; `WAL_DIFF`/`WAL_INVERSE` records are
//! opaque bytes to this crate (see `db_wal::WalRecord`'s own doc). Collapsing two diffs into one
//! requires knowing what a diff *means*, which only `db_artifact`/`protocol` know — this crate's
//! honest contribution is structural, not semantic: once `WalRetention` proves a whole segment's
//! diffs are covered by a published snapshot, the segment (diffs included) is deleted outright
//! rather than rewritten record-by-record.
//!
//! 🎯️ Design choice — "manifest CAS": `db_storage::CatalogStorage::cas_root` guards ONE global
//! root blob shared by every document in a `DbStorage` instance, owned end-to-end by `db_engine`/
//! the catalog actor; this crate has no schema for that blob's contents and touching it here would
//! silently race whatever the engine layer is doing with it. This crate's fencing instead reuses
//! the `db_storage::LeaseStorage` primitive exactly like `db_snapshot::SnapshotLease` does — a
//! `CompactionLease` scoped per document (a distinct resource namespace from the snapshot lease,
//! since a snapshot builder and a compactor may legitimately run concurrently for two different
//! documents, but never for the SAME document at the same time).
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

use db_ids::{ArtifactId, DbError, check_len};
use db_durability::{DurabilityClass, EpochFence, Frontier};
use db_ids::GenerationId;
use pack::ContentHash;
use db_io_core::{LeaseStorage as _, SnapshotStorage as _};
use semio_framework_async::{Lane, WorkerPool, WorkerPoolUse};
use std::future::Future;
use std::sync::Arc;

#[path = "💰️budget-lease-retention-and-index/🦀️.rs"]
mod budget_lease_retention_and_index;
pub use budget_lease_retention_and_index::{
    CompactionBudget, CompactionIndexReports, CompactionLease, IndexKindReport, PayloadGcReport,
    SegmentHorizon,
};
use budget_lease_retention_and_index::DEFAULT_LEASE_TTL_MS;
#[cfg(test)]
pub use budget_lease_retention_and_index::{
    apply_wal_retention, compact_all_indexes, plan_wal_retention, segment_horizons, sweep_payloads,
};

#[path = "📸️snapshot-consolidation/🦀️.rs"]
mod snapshot_consolidation;
#[cfg(test)]
use snapshot_consolidation::{
    retire_compaction_pages, CompactionCloseExit, MountedCompactionPageClose,
    COMPACTION_PAGE_RETIREMENT, COMPACTION_PAGE_RETIREMENT_OVERFLOW,
    COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT, COMPACTION_PAGE_RETIREMENT_QUARANTINE,
    COMPACTION_RETAINED_PAGE_OWNERS, COMPACTION_RETIREMENT_SLOTS,
};
use snapshot_consolidation::CompactionRetainedPages;
pub use snapshot_consolidation::compaction_page_maintenance_step;
#[cfg(test)]
pub use snapshot_consolidation::SnapshotConsolidator;

#[path = "🧊️cold-archive-and-compactor/🦀️.rs"]
mod cold_archive_and_compactor;
#[cfg(test)]
pub use cold_archive_and_compactor::{build_cold_archive, Compactor};
pub use cold_archive_and_compactor::CompactionReport;
use cold_archive_and_compactor::close_compaction_wal_open_rejected;

#[path = "🧮️job-limits-ledger-and-admission/🦀️.rs"]
mod job_limits_ledger_and_admission;
use job_limits_ledger_and_admission::{
    close_compaction_descriptor, close_compaction_snapshot_body,
    database_compaction_admit_descriptor, database_compaction_descriptor_backing,
    database_compaction_observe_backing, DatabaseCompactionAdmission,
    DatabaseCompactionBackingLedger, DatabaseCompactionSegmentOwners, DATABASE_COMPACTION_ADMISSION,
    DATABASE_COMPACTION_DEADLINE_MS, DATABASE_COMPACTION_INDEX_FUEL,
    DATABASE_COMPACTION_RETRY_LIMIT, DATABASE_COMPACTION_SLOTS, DATABASE_COMPACTION_TURN_MS,
};
#[cfg(test)]
use job_limits_ledger_and_admission::{
    retire_compaction_snapshot_body, DatabaseCompactionHashOwners, DATABASE_COMPACTION_MAX_HASHES,
    DATABASE_COMPACTION_OPERATION_BYTES, DATABASE_COMPACTION_OPERATION_ITEMS,
};
pub use job_limits_ledger_and_admission::DatabaseCompactionProgress;

#[path = "🏗️job-execution/🦀️.rs"]
mod job_execution;
#[cfg(test)]
use job_execution::{close_compaction_owner, committed_compaction_horizons, compaction_opportunity};
use job_execution::{
    compaction_resource, retained_compaction_execute, DatabaseCompactionExecution,
    DatabaseCompactionLeaseRecovery, DatabaseCompactionLeaseReleaseFuture,
};
pub use job_execution::retained_compaction_with_wal;

#[path = "🧵️job-state/🦀️.rs"]
mod job_state;
#[cfg(test)]
use job_state::DatabaseCompactionExecutionFuture;
use job_state::{
    database_compaction_registry, DatabaseCompactionCore, DatabaseCompactionDriverAuthority,
    DatabaseCompactionState, DatabaseCompactionTerminalOwners,
};

#[path = "📬️job-result-future-and-rejection/🦀️.rs"]
mod job_result_future_and_rejection;
pub use job_result_future_and_rejection::{
    DatabaseCompactionFuture, DatabaseCompactionRejected, DatabaseCompactionResult,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
