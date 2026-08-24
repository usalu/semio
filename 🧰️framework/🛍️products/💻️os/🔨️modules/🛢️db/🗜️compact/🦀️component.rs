//! 🗄️ 🧹️ `db_compact` — the `db` crate family's compaction and GC engine: WAL segment retention
//! (folding the "merge/recompress" goal into deleting whole snapshot-covered sealed segments,
//! since `db_storage::WalStorage` deliberately exposes no "is this segment sealed" query and
//! `db_wal`'s own `.spr` segments accept only its own critical `WAL_*` record kinds — see the
//! `//#region 🔖️WalRetention` doc for the full rationale), snapshot chain consolidation into a
//! fresh full baseline, ref-traced payload GC, index-kind merge (which also physically drops
//! shadowed tombstones — `db_compact`'s share of "tombstone/preview GC"), cold-tier pack
//! archival, and a fenced, budgeted top-level `Compactor` orchestrator. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
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
//! 🎯️ Scope boundary — "branch GC": branches are a `vcs::Alternative` concept; only `db_engine`
//! (behind the `vcs` Cargo feature) may depend on `vcs`, and this crate has no `VersionGraph`
//! handle to call. Left as a `db_engine`-layer extension, not faked here.
//!
//! 🎯️ Design choice — "manifest CAS": `db_storage::CatalogStorage::cas_root` guards ONE global
//! root blob shared by every document in a `DbStorage` instance, owned end-to-end by `db_engine`/
//! the catalog actor; this crate has no schema for that blob's contents and touching it here would
//! silently race whatever the engine layer is doing with it. This crate's fencing instead reuses
//! the `db_storage::LeaseStorage` primitive exactly like `db_snapshot::SnapshotLease` does — a
//! `CompactionLease` scoped per document (a distinct resource namespace from the snapshot lease,
//! since a snapshot builder and a compactor may legitimately run concurrently for two different
//! documents, but never for the SAME document at the same time).

use crate::db_ids::{check_len, ArtifactId, DbError};
use crate::*;
use db_storage::SnapshotStorage as _;

//#region 🔖️Budget
/// @emoji 💰️ Bounds how much work one `Compactor::run` pass does across every subsystem — the
/// contract's "budgets" line. Every loop this crate runs (WAL segment selection, snapshot chain
/// depth, payload deletions) is capped by one of these fields so a single compaction pass never
/// turns into unbounded work against a document with a very long history.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CompactionBudget {
    pub max_wal_segments: u64,
    pub max_snapshot_generations: u64,
    pub max_payloads: u64,
}

impl CompactionBudget {
    /// @emoji ♾️ No cap on any subsystem — for tests and offline/maintenance runs where bounded
    /// latency doesn't matter.
    pub const fn unlimited() -> CompactionBudget {
        CompactionBudget { max_wal_segments: u64::MAX, max_snapshot_generations: u64::MAX, max_payloads: u64::MAX }
    }
}

impl Default for CompactionBudget {
    /// @emoji 🏗️ This crate's own choice of defaults (the contract fixes that budgets exist, not
    /// their numbers): generous enough that an ordinary document's compaction pass finishes in one
    /// call, small enough that a pathological document (a snapshot chain thousands deep) can't
    /// stall an online compactor indefinitely.
    fn default() -> Self {
        CompactionBudget { max_wal_segments: 64, max_snapshot_generations: 64, max_payloads: 4_096 }
    }
}
//#endregion 🔖️Budget

//#region 🔖️Lease
/// @emoji 🚧️ The fencing primitive behind "online compaction with manifest CAS + fencing" (see
/// module doc's design-choice note on why this wraps `LeaseStorage` rather than `CatalogStorage`).
/// Mirrors `db_snapshot::SnapshotLease`'s shape exactly, under its own `"compact:"`-prefixed
/// resource namespace so a document's snapshot builder and its compactor never contend on the
/// same lease.
pub struct CompactionLease;

impl CompactionLease {
    /// @emoji 🏷️ The `LeaseStorage` resource name guarding `document`'s compaction pass.
    // 🚫️async: E1 pure accessor consumed synchronously by `acquire`/`renew`/`release`/`current` — see R9
    pub fn resource(document: &ArtifactId) -> String {
        format!("compact:{document}")
    }

    /// @emoji 🤝️ Acquires (or idempotently re-acquires) the compaction lease for `document`.
    pub async fn acquire(storage: &impl db_storage::LeaseStorage, document: &ArtifactId, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        storage.acquire(&Self::resource(document), holder, ttl_ms, now_ms).await
    }

    /// @emoji ♻️ Extends `holder`'s existing compaction lease for `document`.
    pub async fn renew(storage: &impl db_storage::LeaseStorage, document: &ArtifactId, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        storage.renew(&Self::resource(document), holder, fence, ttl_ms, now_ms).await
    }

    /// @emoji 🕊️ Releases `holder`'s compaction lease for `document` — `Compactor::run` always
    /// calls this once, even if the pass itself failed (see that method's doc).
    pub async fn release(storage: &impl db_storage::LeaseStorage, document: &ArtifactId, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        storage.release(&Self::resource(document), holder, fence).await
    }

    /// @emoji 👀️ The compaction lease's current holder/fence for `document`, or `None` if unheld.
    pub async fn current(storage: &impl db_storage::LeaseStorage, document: &ArtifactId, now_ms: u64) -> Result<Option<db_storage::LeaseInfo>, DbError> {
        storage.current(&Self::resource(document), now_ms).await
    }
}

/// @emoji ⏳️ This crate's own default compaction-lease TTL (the contract doesn't fix a number):
/// long enough that a slow consolidate-and-prune pass over a deep snapshot chain doesn't lose the
/// lease mid-flight, short enough that a crashed compactor's stale lease self-heals quickly.
const DEFAULT_LEASE_TTL_MS: u64 = 5 * 60 * 1_000;
//#endregion 🔖️Lease

//#region 🔖️WalRetention
/// @emoji 🧾️ One WAL segment's role in a retention decision: its index and the highest
/// `head_seq` any `WAL_FRONTIER`/`WAL_SNAPSHOT_PUB` record within its span reached.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SegmentHorizon {
    pub segment_index: u64,
    /// @emoji ❓️ `None` if the segment carries no frontier marker at all — such a segment is
    /// never a `plan_wal_retention` candidate, since there is no way to prove everything in it is
    /// covered by a given floor.
    pub max_head_seq: Option<u64>,
}

/// @emoji 🪢️ Groups `records` (as returned by `db_wal::replay_document`, already in
/// segment-then-on-disk order) by the `WalRecord::SegmentHeader` boundaries that open each span —
/// the shared traversal `segment_horizons`/`sweep_payloads` both build on.
/// @emoji 📊️ Computes every segment's `SegmentHorizon` from a document's full replayed record
/// stream.
pub async fn segment_horizons<'record>(records: impl IntoIterator<Item = &'record db_wal::WalRecord>) -> Vec<SegmentHorizon> {
    let mut horizons = Vec::new();
    let mut current: Option<SegmentHorizon> = None;
    for record in records {
        match record {
            db_wal::WalRecord::SegmentHeader { segment_index, .. } => {
                if let Some(horizon) = current.replace(SegmentHorizon { segment_index: *segment_index, max_head_seq: None }) {
                    horizons.push(horizon);
                }
            }
            db_wal::WalRecord::Frontier(frontier) | db_wal::WalRecord::SnapshotPub { frontier, .. } => {
                if let Some(horizon) = current.as_mut() {
                    horizon.max_head_seq = Some(horizon.max_head_seq.map_or(frontier.head_seq, |head| head.max(frontier.head_seq)));
                }
            }
            _ => {}
        }
    }
    if let Some(horizon) = current {
        horizons.push(horizon);
    }
    horizons
}

/// @emoji 🧹️ Selects which SEALED WAL segments are safe to delete: strictly below the highest
/// segment index present (the presumed-active segment — `db_storage::WalStorage` has no "is this
/// sealed" query, so this crate never risks touching one that might still be live, per the module
/// doc), with a known `max_head_seq` at or below `floor_head_seq`, capped at
/// `budget.max_wal_segments`. Ascending order (oldest first).
// 🚫️async: E1 pure accessor consumed synchronously by `run_under_lease` and tests — see R9
pub fn plan_wal_retention(horizons: &[SegmentHorizon], floor_head_seq: u64, budget: &CompactionBudget) -> Vec<u64> {
    let Some(max_index) = horizons.iter().map(|horizon| horizon.segment_index).max() else {
        return Vec::new();
    };
    let mut selected: Vec<u64> = horizons.iter().filter(|horizon| horizon.segment_index != max_index).filter(|horizon| horizon.max_head_seq.is_some_and(|seq| seq <= floor_head_seq)).map(|horizon| horizon.segment_index).collect();
    selected.sort_unstable();
    selected.truncate(usize::try_from(budget.max_wal_segments).unwrap_or(usize::MAX));
    selected
}

/// @emoji 🗑️ Applies `plan_wal_retention`'s output: deletes each selected segment from `storage`.
/// Idempotent (`WalStorage::delete_segment` already is). Returns how many were selected (and thus
/// attempted).
pub async fn apply_wal_retention(storage: &impl db_storage::WalStorage, document: &ArtifactId, segments: &[u64]) -> Result<u64, DbError> {
    for &index in segments {
        storage.delete_segment(document, index).await?;
    }
    Ok(segments.len() as u64)
}
//#endregion 🔖️WalRetention

//#region 🔖️PayloadGc
/// @emoji 📊️ What `sweep_payloads` did.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PayloadGcReport {
    pub candidates_checked: u64,
    pub deleted: u64,
}

/// @emoji 🔗️ Ref-traced payload GC: `candidates` are the `WalPayloadRef::CasRef` hashes that
/// appeared ONLY within `deleted_segments`' span of `records` (i.e. payloads that just lost their
/// one known reference); any candidate NOT also referenced by a record outside
/// `deleted_segments` is genuinely orphaned and deleted from `payload_storage`.
///
/// 🎯️ Scope boundary: `db_storage::PayloadStorage` has no enumeration method (by design — CAS
/// stores don't need one for `put`/`get`/`delete`), so this crate cannot do a full mark-and-sweep
/// over every payload ever stored; it can only trace liveness for a caller-supplied candidate set,
/// which is exactly what `Compactor::run` derives from its own WAL retention pass.
pub async fn sweep_payloads<'record>(payload_storage: &impl db_storage::PayloadStorage, records: impl IntoIterator<Item = &'record db_wal::WalRecord>, deleted_segments: &[u64], budget: &CompactionBudget) -> Result<PayloadGcReport, DbError> {
    let deleted_set: std::collections::HashSet<u64> = deleted_segments.iter().copied().collect();
    let mut candidates = std::collections::HashSet::new();
    let mut live = std::collections::HashSet::new();
    let mut segment_index = 0;
    for record in records {
        if let db_wal::WalRecord::SegmentHeader { segment_index: next, .. } = record {
            segment_index = *next;
        } else if let db_wal::WalRecord::Payload(db_wal::WalPayloadRef::CasRef(hash)) = record {
            let target = if deleted_set.contains(&segment_index) { &mut candidates } else { &mut live };
            target.insert(*hash);
        }
    }
    let mut report = PayloadGcReport::default();
    for hash in candidates {
        if report.deleted >= budget.max_payloads {
            break;
        }
        report.candidates_checked += 1;
        if !live.contains(&hash) {
            payload_storage.delete(&hash).await?;
            report.deleted += 1;
        }
    }
    Ok(report)
}
//#endregion 🔖️PayloadGc

//#region 🔖️IndexCompaction
/// @emoji 🧹️ One `IndexKind`'s post-compaction shape.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IndexKindReport {
    pub kind: db_index::IndexKind,
    pub stats: db_index::IndexStats,
}

/// @emoji 🧹️ Compacts every `db_index::IndexKind` for `document`: merges all live runs into one
/// per kind, physically dropping tombstones shadowed beneath them (`db_index::IndexHandle::
/// compact`'s own law) — the mechanism behind the contract's "index merge" and, for
/// `IndexKind::Preview`/`Conflict` specifically, its share of "tombstone/preview GC": once a
/// withdrawn preview key or a resolved conflict marker is the only thing a tombstone shadows,
/// compacting reclaims it for good.
pub async fn compact_all_indexes(storage: &impl db_storage::IndexStorage, document: &ArtifactId) -> Result<Vec<IndexKindReport>, DbError> {
    let mut reports = Vec::with_capacity(db_index::IndexKind::ALL.len());
    for kind in db_index::IndexKind::ALL {
        let handle = db_index::IndexHandle::new(storage, document.clone(), kind).await;
        let mut control = handle.operation_control(65_536)?;
        let stats = handle.compact(&mut control).await?;
        reports.push(IndexKindReport { kind, stats });
    }
    Ok(reports)
}
//#endregion 🔖️IndexCompaction

//#region 🔖️SnapshotConsolidation
/// @emoji 🌳️ Walks the snapshot chain from `through_generation` back to its full-baseline root,
/// returning the latest generation's own descriptor plus every page introduced anywhere in the
/// chain, deduplicated by content hash — `SnapshotConsolidator::consolidate`'s input.
async fn collect_chain_pages<S: db_storage::SnapshotStorage>(
    manager: &db_snapshot::SnapshotManager<'_, S>,
    document: &ArtifactId,
    through_generation: u64,
    budget: &CompactionBudget,
) -> Result<(db_snapshot::SnapshotDescriptor, Vec<db_state::Page>), DbError> {
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let mut cursor = manager.chain_cursor(document, through_generation, control);
    let latest_descriptor = cursor.latest_descriptor().await?;
    let mut descriptor = latest_descriptor.clone();
    let mut seen = std::collections::HashSet::new();
    let mut pages = Vec::new();
    let mut generations_walked = 0u64;
    loop {
        generations_walked += 1;
        check_len(generations_walked, budget.max_snapshot_generations, "db_compact::snapshot_chain_depth")?;
        for hash in descriptor.new_pages.clone() {
            if seen.insert(hash) {
                let bytes = cursor.read_page(hash).await?;
                pages.push(db_state::Page::try_from_pages(bytes).await?);
            }
        }
        match descriptor.parent_generation {
            Some(parent) => descriptor = cursor.descriptor(parent).await?,
            None => break,
        }
    }
    while cursor.close_step()? {}
    Ok((latest_descriptor, pages))
}

/// @emoji 🧑️‍💼️ Rolls up a document's incremental snapshot chain into a fresh, self-sufficient
/// full baseline — the responsibility `db_snapshot`'s own module doc explicitly defers to this
/// crate (see that crate's "Scope boundary" note).
pub struct SnapshotConsolidator<'storage, S: db_storage::SnapshotStorage> {
    manager: db_snapshot::SnapshotManager<'storage, S>,
}

impl<'storage, S: db_storage::SnapshotStorage> SnapshotConsolidator<'storage, S> {
    pub async fn new(storage: &'storage S) -> SnapshotConsolidator<'storage, S> {
        SnapshotConsolidator { manager: db_snapshot::SnapshotManager::new(storage).await }
    }

    /// @emoji 🧵️ Publishes a new full-baseline generation carrying the union of every page from
    /// the chain's root through `through_generation` (deduplicated by content hash), with the
    /// latest generation's own frontier/provenance/`roots`. Returns the new generation number; the
    /// caller is responsible for `retain_from` afterward once satisfied nothing still needs an old
    /// generation (e.g. an in-flight replica read — this method itself never prunes).
    ///
    /// 🎯️ Scope boundary: this is a page-union roll-up, not a reachability GC — a page that became
    /// unreferenced somewhere along the chain is still carried into the new baseline (harmless:
    /// `roots` still resolves correctly, the baseline is just not maximally compact). Pruning truly
    /// unreachable pages needs the application-level page-tree structure this crate doesn't have
    /// (see module doc's "diff collapse" scope note for the same underlying constraint) and is left
    /// as this crate's deliberate extension seam.
    pub async fn consolidate(&self, document: &ArtifactId, through_generation: u64, budget: &CompactionBudget) -> Result<u64, DbError> {
        let (latest, pages) = collect_chain_pages(&self.manager, document, through_generation, budget).await?;
        let body = db_snapshot::SnapshotBody {
            head_seq: latest.head_seq,
            commit_seq: latest.commit_seq,
            epoch: latest.epoch,
            chain_hash: latest.chain_hash,
            protocol_version: latest.protocol_version,
            vcs_head: latest.vcs_head,
            base_pack_hash: latest.base_pack_hash,
            roots: latest.roots,
            created_at_ms: latest.created_at_ms,
        };
        self.manager.publish(document, db_snapshot::SnapshotOrigin::FullBaseline, &pages, body).await
    }

    /// @emoji 🗑️ Forwards to `db_snapshot::SnapshotManager::retain_from` — `floor_generation` must
    /// itself be a full baseline (typically the generation `consolidate` just returned).
    pub async fn retain_from(&self, document: &ArtifactId, floor_generation: u64) -> Result<(), DbError> {
        self.manager.retain_from(document, floor_generation).await
    }
}
//#endregion 🔖️SnapshotConsolidation

//#region 🔖️ColdArchive
/// @emoji 🧊️ Builds one document's cold-tier archive: the full, self-contained byte concatenation
/// of every snapshot generation from the chain's root through `through_generation` (via
/// the snapshot retained chain cursor), independently reopenable with
/// `db_snapshot::open_latest` — ready to hand to whatever cold-tier object store a deployment
/// configures.
///
/// 🎯️ Scope boundary: `db_storage` defines no `ColdStorage` trait, so this crate returns the
/// archive bytes rather than inventing a storage seam unilaterally in a crate that isn't
/// `db_storage`'s own.
pub async fn build_cold_archive(storage: &impl db_storage::SnapshotStorage, document: &ArtifactId, through_generation: u64) -> Result<db_storage::DbIoPages, DbError> {
    let manager = db_snapshot::SnapshotManager::new(storage).await;
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let mut cursor = manager.chain_cursor(document, through_generation, control);
    let pages = cursor.materialize_pages().await?;
    while cursor.close_step()? {}
    Ok(pages)
}
//#endregion 🔖️ColdArchive

//#region 🔖️Compactor
/// @emoji 📋️ What one `Compactor::run` pass did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompactionReport {
    pub wal_segments_deleted: u64,
    pub payloads_deleted: u64,
    pub index_reports: Vec<IndexKindReport>,
    pub snapshot_consolidated_generation: Option<u64>,
    pub snapshot_generations_pruned: u64,
}

/// @emoji 🧑️‍💼️ The top-level, fenced, budgeted orchestrator gluing every subsystem in this crate
/// together over one `db_storage::DbStorage` backend — "online compaction with manifest CAS +
/// fencing" (see module doc's design-choice note on the fencing mechanism).
pub struct Compactor<'storage> {
    storage: &'storage db_storage::DbBackend,
}

impl<'storage> Compactor<'storage> {
    pub async fn new(storage: &'storage db_storage::DbBackend) -> Compactor<'storage> {
        Compactor { storage }
    }

    /// @emoji 🚀️ One bounded, fenced compaction pass over `document`: WAL segment retention below
    /// `wal_floor_head_seq`, ref-traced payload GC over whatever WAL retention just orphaned, every
    /// index kind's merge/tombstone-GC, and — if `consolidate_snapshots` is set — rolling the
    /// snapshot chain into a fresh full baseline and pruning everything below it. Acquires
    /// `CompactionLease::resource(document)` for the whole pass; a lease held elsewhere surfaces as
    /// `DbError::Conflict` rather than silently racing another compactor. The lease is ALWAYS
    /// released before returning, including when a step fails partway through — a failed pass must
    /// never leave a document permanently unfenceable.
    pub async fn run(&self, document: &ArtifactId, holder: &str, wal_floor_head_seq: u64, consolidate_snapshots: bool, budget: &CompactionBudget, now_ms: u64) -> Result<CompactionReport, DbError> {
        let fence = CompactionLease::acquire(&self.storage.lease().await, document, holder, DEFAULT_LEASE_TTL_MS, now_ms).await?;
        let result = self.run_under_lease(document, wal_floor_head_seq, consolidate_snapshots, budget).await;
        let release_result = CompactionLease::release(&self.storage.lease().await, document, holder, fence).await;
        match (result, release_result) {
            (Ok(report), Ok(())) => Ok(report),
            (Err(run_error), _) => Err(run_error),
            (Ok(_), Err(release_error)) => Err(release_error),
        }
    }

    /// @emoji 🧭️ Convenience over `run`: derives `wal_floor_head_seq` from `document`'s current
    /// latest snapshot generation (or `0`, i.e. nothing deletable, if it has none yet).
    pub async fn run_from_latest_snapshot(&self, document: &ArtifactId, holder: &str, consolidate_snapshots: bool, budget: &CompactionBudget, now_ms: u64) -> Result<CompactionReport, DbError> {
        let snapshot = self.storage.snapshot().await;
        let floor = db_snapshot::SnapshotManager::new(&snapshot).await.load_latest(document).await?.map_or(0, |(_, descriptor)| descriptor.head_seq);
        drop(snapshot);
        self.run(document, holder, floor, consolidate_snapshots, budget, now_ms).await
    }

    async fn run_under_lease(&self, document: &ArtifactId, wal_floor_head_seq: u64, consolidate_snapshots: bool, budget: &CompactionBudget) -> Result<CompactionReport, DbError> {
        let mut report = CompactionReport::default();

        let wal = self.storage.wal().await;
        let control = db_wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
        let mut replay = db_wal::replay_document(&wal, document, control).await?;
        let mut horizons = Vec::new();
        let mut current: Option<SegmentHorizon> = None;
        while let Some(mut record) = replay.next().await? {
            match &record {
                db_wal::WalRecord::SegmentHeader { segment_index, .. } => {
                    if let Some(horizon) = current.replace(SegmentHorizon { segment_index: *segment_index, max_head_seq: None }) {
                        horizons.push(horizon);
                    }
                }
                db_wal::WalRecord::Frontier(frontier) | db_wal::WalRecord::SnapshotPub { frontier, .. } => {
                    if let Some(horizon) = current.as_mut() {
                        horizon.max_head_seq = Some(horizon.max_head_seq.map_or(frontier.head_seq, |head| head.max(frontier.head_seq)));
                    }
                }
                _ => {}
            }
            while record.close_step()? {}
        }
        if let Some(horizon) = current {
            horizons.push(horizon);
        }
        while replay.close_step().await? {}
        let selected = plan_wal_retention(&horizons, wal_floor_head_seq, budget);

        let deleted: std::collections::HashSet<u64> = selected.iter().copied().collect();
        let control = db_wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
        let mut replay = db_wal::replay_document(&wal, document, control).await?;
        let mut segment = 0u64;
        let mut candidates = std::collections::HashSet::new();
        let mut live = std::collections::HashSet::new();
        while let Some(mut record) = replay.next().await? {
            match &record {
                db_wal::WalRecord::SegmentHeader { segment_index, .. } => segment = *segment_index,
                db_wal::WalRecord::Payload(db_wal::WalPayloadRef::CasRef(hash)) => {
                    if deleted.contains(&segment) {
                        candidates.insert(*hash);
                    } else {
                        live.insert(*hash);
                    }
                }
                _ => {}
            }
            while record.close_step()? {}
        }
        while replay.close_step().await? {}
        report.wal_segments_deleted = apply_wal_retention(&wal, document, &selected).await?;
        let payloads = self.storage.payload().await;
        for hash in candidates.difference(&live).take(budget.max_payloads as usize) {
            payloads.delete(hash).await?;
            report.payloads_deleted += 1;
        }
        drop(payloads);
        drop(wal);

        report.index_reports = compact_all_indexes(&self.storage.index().await, document).await?;

        if consolidate_snapshots {
            let snapshot = self.storage.snapshot().await;
            let manager = db_snapshot::SnapshotManager::new(&snapshot).await;
            if let Some((latest_generation, _)) = manager.load_latest(document).await? {
                let consolidator = SnapshotConsolidator::new(&snapshot).await;
                let new_generation = consolidator.consolidate(document, latest_generation, budget).await?;
                let before_retain = snapshot.list_generations(document).await?.len() as u64;
                consolidator.retain_from(document, new_generation).await?;
                let after_retain = snapshot.list_generations(document).await?.len() as u64;
                report.snapshot_consolidated_generation = Some(new_generation);
                report.snapshot_generations_pruned = before_retain.saturating_sub(after_retain);
            }
        }

        Ok(report)
    }
}
//#endregion 🔖️Compactor
//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use db_storage::{MemoryStorage, PayloadStorage as _, WalStorage as _};
    use db_wal::{WalPayloadRef, WalRecord};
    use {DurabilityClass, Frontier};

    fn pages(bytes: &[u8]) -> db_storage::DbIoPages {
        let mut writer = db_storage::DbIoPageWriter::try_reserve(bytes.len().div_ceil(db_storage::DB_IO_PAGE_BYTES)).expect("test compaction writer admitted");
        for fragment in bytes.chunks(db_storage::DB_IO_PAGE_BYTES) {
            assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
        }
        writer.finish().unwrap()
    }

    async fn doc(id: &str) -> ArtifactId {
        ArtifactId::from(id)
    }

    async fn frontier(document: &ArtifactId, head_seq: u64) -> Frontier {
        Frontier { document: document.clone(), head_seq, commit_seq: head_seq, chain_hash: [0u8; 32], epoch: 0 }
    }

    async fn sample_body(head_seq: u64) -> db_snapshot::SnapshotBody {
        db_snapshot::SnapshotBody { head_seq, commit_seq: head_seq, epoch: 0, chain_hash: [0u8; 32], protocol_version: 1, vcs_head: None, base_pack_hash: None, roots: vec![], created_at_ms: head_seq * 1_000 }
    }

    async fn wal_bytes(source: &[u8]) -> db_wal::WalBytes {
        let mut control = db_wal::WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        db_wal::WalBytes::try_admit(source.to_vec(), 1024 * 1024, &mut control).await.unwrap()
    }

    async fn submit_record(storage: &MemoryStorage, wal: &mut db_wal::ArtifactWal, record: WalRecord, now_ms: u64) {
        let mut records = db_wal::WalRecordBatch::new();
        assert!(records.push(record).is_ok());
        wal.submit(storage, &records, DurabilityClass::Fsync, now_ms).await.unwrap();
        while records.close_step().unwrap() {}
    }

    async fn index_put(handle: &db_index::IndexHandle<'_, MemoryStorage>, key: &[u8], value: &[u8]) {
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = db_index::IndexCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let key = db_index::IndexBytes::try_admit(key.to_vec(), 1024 * 1024, &mut control).await.unwrap();
        let value = db_index::IndexBytes::try_admit(value.to_vec(), 1024 * 1024, &mut control).await.unwrap();
        handle.put(key, value, &mut control).await.unwrap();
    }

    async fn state_page(source: &[u8]) -> db_state::Page {
        db_state::Page::try_from_pages(pages(source)).await.unwrap()
    }

    //#region 🔖️Budget
    #[semio_framework_async_macros::async_test]
    async fn compaction_budget_default_is_finite_and_unlimited_is_boundless() {
        let default = CompactionBudget::default();
        assert!(default.max_wal_segments > 0 && default.max_wal_segments < u64::MAX);
        assert!(default.max_snapshot_generations > 0 && default.max_snapshot_generations < u64::MAX);
        assert!(default.max_payloads > 0 && default.max_payloads < u64::MAX);

        let unlimited = CompactionBudget::unlimited();
        assert_eq!(unlimited.max_wal_segments, u64::MAX);
        assert_eq!(unlimited.max_snapshot_generations, u64::MAX);
        assert_eq!(unlimited.max_payloads, u64::MAX);
    }
    //#endregion 🔖️Budget

    //#region 🔖️Lease
    #[semio_framework_async_macros::async_test]
    async fn compaction_lease_round_trips_and_is_scoped_distinctly_from_the_snapshot_lease() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;

        let fence = db_actor::block_on(CompactionLease::acquire(&storage, &document, "holder-a", 1_000, 0)).unwrap();
        assert!(db_actor::block_on(CompactionLease::current(&storage, &document, 0)).unwrap().is_some());

        db_actor::block_on(CompactionLease::renew(&storage, &document, "holder-a", fence, 1_000, 500)).unwrap();
        assert!(matches!(db_actor::block_on(CompactionLease::acquire(&storage, &document, "holder-b", 1_000, 500)), Err(DbError::Conflict(_))));

        db_actor::block_on(CompactionLease::release(&storage, &document, "holder-a", fence)).unwrap();
        assert!(db_actor::block_on(CompactionLease::current(&storage, &document, 500)).unwrap().is_none());

        assert_ne!(CompactionLease::resource(&document), db_snapshot::SnapshotLease::resource(&document));
    }
    //#endregion 🔖️Lease

    //#region 🔖️WalRetention
    #[semio_framework_async_macros::async_test]
    async fn segment_horizons_tracks_the_max_head_seq_seen_within_each_segment_span() {
        let document = doc("doc-1").await;
        let mut records = db_wal::WalRecordBatch::new();
        for record in [
            WalRecord::SegmentHeader { document: document.clone(), segment_index: 0, prev_chain_hash: None },
            WalRecord::Command(wal_bytes(b"a").await),
            WalRecord::Frontier(frontier(&document, 3).await),
            WalRecord::Frontier(frontier(&document, 7).await),
            WalRecord::SegmentHeader { document: document.clone(), segment_index: 1, prev_chain_hash: Some([1u8; 32]) },
            WalRecord::Command(wal_bytes(b"b").await),
        ] {
            assert!(records.push(record).is_ok());
        }
        let horizons = segment_horizons(records.iter()).await;
        assert_eq!(horizons, vec![SegmentHorizon { segment_index: 0, max_head_seq: Some(7) }, SegmentHorizon { segment_index: 1, max_head_seq: None },]);
        while records.close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_wal_retention_never_selects_the_highest_segment_index_even_if_it_qualifies() {
        let horizons = vec![SegmentHorizon { segment_index: 0, max_head_seq: Some(5) }, SegmentHorizon { segment_index: 1, max_head_seq: Some(5) }];
        let selected = plan_wal_retention(&horizons, 100, &CompactionBudget::default());
        assert_eq!(selected, vec![0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_wal_retention_never_selects_a_segment_with_no_known_horizon() {
        let horizons = vec![SegmentHorizon { segment_index: 0, max_head_seq: None }, SegmentHorizon { segment_index: 1, max_head_seq: Some(999) }];
        let selected = plan_wal_retention(&horizons, 10, &CompactionBudget::default());
        assert!(selected.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_wal_retention_only_selects_segments_at_or_below_the_floor() {
        let horizons = vec![SegmentHorizon { segment_index: 0, max_head_seq: Some(5) }, SegmentHorizon { segment_index: 1, max_head_seq: Some(15) }, SegmentHorizon { segment_index: 2, max_head_seq: Some(20) }];
        let selected = plan_wal_retention(&horizons, 10, &CompactionBudget::default());
        assert_eq!(selected, vec![0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn plan_wal_retention_respects_the_budget_cap() {
        let horizons = vec![SegmentHorizon { segment_index: 0, max_head_seq: Some(1) }, SegmentHorizon { segment_index: 1, max_head_seq: Some(1) }, SegmentHorizon { segment_index: 2, max_head_seq: Some(1) }];
        let budget = CompactionBudget { max_wal_segments: 1, ..CompactionBudget::default() };
        let selected = plan_wal_retention(&horizons, 100, &budget);
        assert_eq!(selected.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn apply_wal_retention_deletes_selected_segments_and_is_idempotent() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        db_actor::block_on(storage.create_segment(&document, 0)).unwrap();
        db_actor::block_on(storage.create_segment(&document, 1)).unwrap();
        db_actor::block_on(storage.create_segment(&document, 2)).unwrap();

        let deleted = db_actor::block_on(apply_wal_retention(&storage, &document, &[0, 1])).unwrap();
        assert_eq!(deleted, 2);
        assert_eq!(db_actor::block_on(storage.list_segments(&document)).unwrap(), vec![2]);

        db_actor::block_on(apply_wal_retention(&storage, &document, &[0, 1])).unwrap();
        assert_eq!(db_actor::block_on(storage.list_segments(&document)).unwrap(), vec![2]);
    }
    //#endregion 🔖️WalRetention

    //#region 🔖️PayloadGc
    #[semio_framework_async_macros::async_test]
    async fn sweep_payloads_deletes_orphaned_candidates_but_keeps_hashes_still_referenced_elsewhere() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let orphan_hash = db_actor::block_on(storage.put(pages(b"orphan-payload"))).unwrap();
        let shared_hash = db_actor::block_on(storage.put(pages(b"shared-payload"))).unwrap();
        let document = doc("doc-1").await;

        let mut records = db_wal::WalRecordBatch::new();
        for record in [
            WalRecord::SegmentHeader { document: document.clone(), segment_index: 0, prev_chain_hash: None },
            WalRecord::Payload(WalPayloadRef::CasRef(orphan_hash)),
            WalRecord::Payload(WalPayloadRef::CasRef(shared_hash)),
            WalRecord::SegmentHeader { document, segment_index: 1, prev_chain_hash: Some([0u8; 32]) },
            WalRecord::Payload(WalPayloadRef::CasRef(shared_hash)),
        ] {
            assert!(records.push(record).is_ok());
        }

        let report = db_actor::block_on(sweep_payloads(&storage, records.iter(), &[0], &CompactionBudget::default())).unwrap();
        assert_eq!(report.candidates_checked, 2);
        assert_eq!(report.deleted, 1);
        assert!(!db_actor::block_on(storage.contains(&orphan_hash)).unwrap());
        assert!(db_actor::block_on(storage.contains(&shared_hash)).unwrap());
        while records.close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn sweep_payloads_respects_the_budget_cap() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let hash_a = db_actor::block_on(storage.put(pages(b"a"))).unwrap();
        let hash_b = db_actor::block_on(storage.put(pages(b"b"))).unwrap();
        let document = doc("doc-1").await;
        let mut records = db_wal::WalRecordBatch::new();
        assert!(records.push(WalRecord::SegmentHeader { document, segment_index: 0, prev_chain_hash: None }).is_ok());
        assert!(records.push(WalRecord::Payload(WalPayloadRef::CasRef(hash_a))).is_ok());
        assert!(records.push(WalRecord::Payload(WalPayloadRef::CasRef(hash_b))).is_ok());
        let budget = CompactionBudget { max_payloads: 1, ..CompactionBudget::default() };
        let report = db_actor::block_on(sweep_payloads(&storage, records.iter(), &[0], &budget)).unwrap();
        assert_eq!(report.deleted, 1);
        while records.close_step().unwrap() {}
    }
    //#endregion 🔖️PayloadGc

    //#region 🔖️IndexCompaction
    #[semio_framework_async_macros::async_test]
    async fn compact_all_indexes_reports_every_kind_and_merges_multiple_runs_into_one() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let handle = db_index::IndexHandle::new(&storage, document.clone(), db_index::IndexKind::Command).await;
        index_put(&handle, b"a", b"1").await;
        index_put(&handle, b"b", b"2").await;
        let mut control = db_index::IndexCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        assert!(handle.stats(&mut control).await.unwrap().run_count >= 2, "two separate put calls must land in separate runs below the auto-merge threshold");

        let reports = db_actor::block_on(compact_all_indexes(&storage, &document)).unwrap();
        assert_eq!(reports.len(), db_index::IndexKind::ALL.len());

        let command_report = reports.iter().find(|report| report.kind == db_index::IndexKind::Command).unwrap();
        assert_eq!(command_report.stats.run_count, 1);
        assert_eq!(command_report.stats.entry_count, 2);
    }
    //#endregion 🔖️IndexCompaction

    //#region 🔖️SnapshotConsolidation
    #[semio_framework_async_macros::async_test]
    async fn consolidate_produces_a_self_sufficient_full_baseline_covering_the_whole_chain() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let manager = db_snapshot::SnapshotManager::new(&storage).await;

        let gen0_pages = vec![state_page(b"base-a").await, state_page(b"base-b").await];
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &gen0_pages, sample_body(0).await)).unwrap();

        let gen1_pages = vec![state_page(b"delta-a").await];
        let mut body1 = sample_body(5).await;
        body1.roots = vec![gen1_pages[0].hash, gen0_pages[1].hash];
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &gen1_pages, body1.clone())).unwrap();

        let consolidator = SnapshotConsolidator::new(&storage).await;
        let new_generation = db_actor::block_on(consolidator.consolidate(&document, 1, &CompactionBudget::default())).unwrap();
        assert_eq!(new_generation, 2);

        let mut bytes = db_actor::block_on(storage.read_generation(&document, new_generation)).unwrap();
        let mut prepared = db_storage::db_io_prepare_platform(&bytes).unwrap().await.unwrap();
        let handle = db_snapshot::open_latest(prepared.as_slice()).await.unwrap();
        assert!(handle.parent_footer_offset().await.is_none(), "a consolidated generation must be a self-sufficient full baseline");
        assert_eq!(handle.descriptor.roots, body1.roots);
        while prepared.close_step().unwrap() {}
        while bytes.close_step().unwrap().is_some() {}

        let control = db_snapshot::SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut cursor = manager.chain_cursor(&document, new_generation, control);
        for page in gen0_pages.iter().chain(gen1_pages.iter()) {
            let mut read_back = cursor.read_page(page.hash).await.unwrap();
            assert_eq!(read_back, *page.pages());
            while read_back.close_step().unwrap().is_some() {}
        }
        while cursor.close_step().unwrap() {}
    }

    #[semio_framework_async_macros::async_test]
    async fn retain_from_after_consolidate_prunes_every_generation_below_the_new_baseline() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let manager = db_snapshot::SnapshotManager::new(&storage).await;
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(0).await)).unwrap();
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &[], sample_body(1).await)).unwrap();

        let consolidator = SnapshotConsolidator::new(&storage).await;
        let new_generation = db_actor::block_on(consolidator.consolidate(&document, 1, &CompactionBudget::default())).unwrap();
        db_actor::block_on(consolidator.retain_from(&document, new_generation)).unwrap();

        assert_eq!(db_actor::block_on(storage.list_generations(&document)).unwrap(), vec![new_generation]);
    }

    #[semio_framework_async_macros::async_test]
    async fn consolidate_respects_the_snapshot_chain_depth_budget() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let manager = db_snapshot::SnapshotManager::new(&storage).await;
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(0).await)).unwrap();
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &[], sample_body(1).await)).unwrap();
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &[], sample_body(2).await)).unwrap();

        let consolidator = SnapshotConsolidator::new(&storage).await;
        let tight_budget = CompactionBudget { max_snapshot_generations: 1, ..CompactionBudget::default() };
        assert!(matches!(db_actor::block_on(consolidator.consolidate(&document, 2, &tight_budget)), Err(DbError::LimitExceeded(_))));
    }
    //#endregion 🔖️SnapshotConsolidation

    //#region 🔖️ColdArchive
    #[semio_framework_async_macros::async_test]
    async fn build_cold_archive_matches_materialize_chain_and_reopens_independently() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let manager = db_snapshot::SnapshotManager::new(&storage).await;
        let pages = vec![state_page(b"page-a").await];
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &pages, sample_body(0).await)).unwrap();

        let mut archive = db_actor::block_on(build_cold_archive(&storage, &document, 0)).unwrap();
        let control = db_snapshot::SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut cursor = manager.chain_cursor(&document, 0, control);
        let mut expected = db_actor::block_on(cursor.materialize_pages()).unwrap();
        assert_eq!(archive, expected);

        let mut prepared = db_storage::db_io_prepare_platform(&archive).unwrap().await.unwrap();
        let handle = db_snapshot::open_latest(prepared.as_slice()).await.unwrap();
        assert_eq!(handle.generation().await, 0);
        while prepared.close_step().unwrap() {}
        while expected.close_step().unwrap().is_some() {}
        while archive.close_step().unwrap().is_some() {}
        while cursor.close_step().unwrap() {}
    }
    //#endregion 🔖️ColdArchive

    //#region 🔖️Compactor
    #[semio_framework_async_macros::async_test]
    async fn run_never_deletes_the_sole_or_active_wal_segment() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let mut wal = db_actor::block_on(db_wal::ArtifactWal::create(&storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
        submit_record(&storage, &mut wal, WalRecord::Frontier(frontier(&document, 100).await), 0).await;
        let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

        let compactor = Compactor::new(&storage).await;
        let report = db_actor::block_on(compactor.run(&document, "holder-a", 1_000, false, &CompactionBudget::default(), 0)).unwrap();
        assert_eq!(report.wal_segments_deleted, 0);
        assert_eq!(db_actor::block_on(async { storage.wal().await.list_segments(&document).await }).unwrap(), vec![0]);
    }

    #[semio_framework_async_macros::async_test]
    async fn run_end_to_end_compacts_indexes_and_consolidates_snapshots_then_releases_the_lease() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let manager = db_snapshot::SnapshotManager::new(&storage).await;
        let gen0_pages = vec![state_page(b"p0").await];
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &gen0_pages, sample_body(0).await)).unwrap();
        let gen1_pages = vec![state_page(b"p1").await];
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::Incremental, &gen1_pages, sample_body(5).await)).unwrap();

        let command_handle = db_index::IndexHandle::new(&storage, document.clone(), db_index::IndexKind::Command).await;
        index_put(&command_handle, b"k1", b"v1").await;
        index_put(&command_handle, b"k2", b"v2").await;
        let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

        let compactor = Compactor::new(&storage).await;
        let report = db_actor::block_on(compactor.run(&document, "holder-a", 0, true, &CompactionBudget::default(), 0)).unwrap();

        assert_eq!(report.snapshot_consolidated_generation, Some(2));
        assert_eq!(report.snapshot_generations_pruned, 2);
        assert_eq!(db_actor::block_on(async { storage.snapshot().await.list_generations(&document).await }).unwrap(), vec![2]);

        assert_eq!(report.index_reports.len(), db_index::IndexKind::ALL.len());
        let command_report = report.index_reports.iter().find(|entry| entry.kind == db_index::IndexKind::Command).unwrap();
        assert_eq!(command_report.stats.run_count, 1);

        assert!(db_actor::block_on(async { CompactionLease::current(&storage.lease().await, &document, 0).await }).unwrap().is_none(), "a successful run must release its lease");
    }

    #[semio_framework_async_macros::async_test]
    async fn run_fails_with_conflict_when_another_holder_already_holds_the_compaction_lease() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let fence = db_actor::block_on(CompactionLease::acquire(&storage, &document, "holder-a", 10_000, 0)).unwrap();
        let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

        let compactor = Compactor::new(&storage).await;
        let result = db_actor::block_on(compactor.run(&document, "holder-b", 0, false, &CompactionBudget::default(), 0));
        assert!(matches!(result, Err(DbError::Conflict(_))));

        db_actor::block_on(async { CompactionLease::release(&storage.lease().await, &document, "holder-a", fence).await }).unwrap();
    }

    #[semio_framework_async_macros::async_test]
    async fn run_releases_the_compaction_lease_even_when_a_step_fails() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        db_actor::block_on(storage.create_segment(&document, 0)).unwrap();
        db_actor::block_on(storage.append(&document, 0, pages(b"not a valid spr segment at all"))).unwrap();
        db_actor::block_on(storage.seal(&document, 0)).unwrap();
        let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

        let compactor = Compactor::new(&storage).await;
        let budget = CompactionBudget::default();
        let result = db_actor::block_on(compactor.run(&document, "holder-a", 0, false, &budget, 0));
        assert!(result.is_err());

        assert!(db_actor::block_on(async { CompactionLease::current(&storage.lease().await, &document, 0).await }).unwrap().is_none(), "the lease must be freed despite the failure");
    }

    #[semio_framework_async_macros::async_test]
    async fn run_from_latest_snapshot_derives_the_floor_from_the_current_snapshot_head_seq() {
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("doc-1").await;
        let manager = db_snapshot::SnapshotManager::new(&storage).await;
        db_actor::block_on(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(42).await)).unwrap();

        let mut wal = db_actor::block_on(db_wal::ArtifactWal::create(&storage, document.clone(), db_wal::GroupCommitPolicy::default(), 0)).unwrap();
        submit_record(&storage, &mut wal, WalRecord::Frontier(frontier(&document, 42).await), 0).await;
        let storage: db_storage::DbBackend = db_storage::DbBackend::Memory(storage);

        let compactor = Compactor::new(&storage).await;
        let report = db_actor::block_on(compactor.run_from_latest_snapshot(&document, "holder-a", false, &CompactionBudget::default(), 0)).unwrap();
        assert_eq!(report.wal_segments_deleted, 0, "the sole segment must still never be touched, even though its horizon is at the floor");
    }
    //#endregion 🔖️Compactor
}
//#endregion 🧪️Tests
