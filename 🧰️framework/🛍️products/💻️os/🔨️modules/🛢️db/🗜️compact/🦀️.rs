//! 🗄️ 🧹️ `db_compact` — the `db` crate family's compaction and GC engine: WAL segment retention
//! (folding the "merge/recompress" goal into deleting whole snapshot-covered sealed segments;
//! the committed cursor's complete segment ledger keeps its highest live segment protected, and
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
use db_storage::{IndexStorage as _, LeaseStorage as _, PayloadStorage as _, SnapshotStorage as _, WalStorage as _};
use semio_framework_async::{Lane, WorkerPool};
use std::future::Future;
use std::sync::Arc;

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

/// @emoji 🪢️ Groups already-admitted records in segment-then-on-disk order by the
/// `WalRecord::SegmentHeader` boundaries that open each span —
/// the shared traversal `segment_horizons`/`sweep_payloads` both build on.
/// @emoji 📊️ Computes every segment's `SegmentHorizon` from a document's full replayed record
/// stream.
#[cfg(test)]
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
/// segment index present (the live segment, retained even when it contains only its header), with
/// a known `max_head_seq` at or below `floor_head_seq`, capped at
/// `budget.max_wal_segments`. Ascending order (oldest first).
// 🚫️async: E1 pure accessor consumed synchronously by `run_under_lease` and tests — see R9
#[cfg(test)]
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
#[cfg(test)]
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
#[cfg(test)]
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

const COMPACTION_INDEX_REPORTS: usize = db_index::IndexKind::ALL.len();

/// 🗂️ Fixed admitted index-report owner returned by one compaction generation.
#[derive(Clone, Debug, PartialEq)]
pub struct CompactionIndexReports {
    slots: [Option<IndexKindReport>; COMPACTION_INDEX_REPORTS],
    len: u8,
}

impl CompactionIndexReports {
    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &IndexKindReport> {
        self.slots[..self.len()].iter().flatten()
    }

    fn push(&mut self, report: IndexKindReport) -> Result<(), DbError> {
        let index = self.len();
        let slot = self.slots.get_mut(index).ok_or(DbError::LimitExceeded("db_compact fixed index reports"))?;
        *slot = Some(report);
        self.len = self.len.checked_add(1).ok_or(DbError::LimitExceeded("db_compact index report cursor"))?;
        Ok(())
    }

    fn close_step(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.len -= 1;
        self.slots[usize::from(self.len)] = None;
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.slots.iter().all(Option::is_none)
    }
}

impl Default for CompactionIndexReports {
    fn default() -> Self {
        Self { slots: [None; COMPACTION_INDEX_REPORTS], len: 0 }
    }
}

/// @emoji 🧹️ Compacts every `db_index::IndexKind` for `document`: merges all live runs into one
/// per kind, physically dropping tombstones shadowed beneath them (`db_index::IndexHandle::
/// compact`'s own law) — the mechanism behind the contract's "index merge" and, for
/// `IndexKind::Preview`/`Conflict` specifically, its share of "tombstone/preview GC": once a
/// withdrawn preview key or a resolved conflict marker is the only thing a tombstone shadows,
/// compacting reclaims it for good.
#[cfg(test)]
pub async fn compact_all_indexes(storage: &impl db_storage::IndexStorage, document: &ArtifactId) -> Result<CompactionIndexReports, DbError> {
    let mut reports = CompactionIndexReports::default();
    for kind in db_index::IndexKind::ALL {
        let handle = db_index::IndexHandle::new(storage, document.clone(), kind).await;
        let mut control = handle.operation_control(65_536)?;
        let stats = handle.compact(&mut control).await?;
        reports.push(IndexKindReport { kind, stats })?;
    }
    Ok(reports)
}
//#endregion 🔖️IndexCompaction

//#region 🔖️SnapshotConsolidation
const COMPACTION_RETAINED_PAGE_OWNERS: usize = 64;
const COMPACTION_RETIREMENT_SLOTS: usize = 64;

/// @emoji 🧱️ Fixed, page-credit-witnessed snapshot consolidation owner set.
struct CompactionRetainedPages {
    pages: [Option<db_state::Page>; COMPACTION_RETAINED_PAGE_OWNERS],
    credits: [u8; COMPACTION_RETAINED_PAGE_OWNERS],
    len: u8,
    retirement: Option<CompactionRetirementReservation>,
}

impl CompactionRetainedPages {
    fn new() -> Self {
        Self { pages: std::array::from_fn(|_| None), credits: [0; COMPACTION_RETAINED_PAGE_OWNERS], len: 0, retirement: None }
    }

    fn contains(&self, hash: pack::ContentHash) -> bool {
        self.pages[..self.len()].iter().flatten().any(|page| page.hash == hash)
    }

    fn preflight_push(&mut self) -> Result<(), DbError> {
        if self.len() == COMPACTION_RETAINED_PAGE_OWNERS {
            return Err(DbError::LimitExceeded("db_compact fixed retained snapshot pages"));
        }
        if self.len == 0 && self.retirement.is_none() {
            self.retirement = Some(reserve_compaction_retirement().ok_or_else(|| DbError::Unavailable("compaction page retirement pressure refused admission".to_string()))?);
        }
        Ok(())
    }

    fn push_preflighted(&mut self, page: db_state::Page) {
        let index = self.len();
        self.credits[index] = page.pages().page_count();
        self.pages[index] = Some(page);
        self.len += 1;
    }

    fn try_push(&mut self, page: db_state::Page) -> Result<(), db_state::Page> {
        if self.preflight_push().is_err() {
            return Err(page);
        }
        self.push_preflighted(page);
        Ok(())
    }

    fn len(&self) -> usize {
        self.len as usize
    }

    fn slots(&self) -> &[Option<db_state::Page>] {
        &self.pages
    }

    fn close_step(&mut self) -> Result<bool, DbError> {
        if self.len == 0 {
            return Ok(false);
        }
        let index = self.len() - 1;
        let page = self.pages[index].as_mut().ok_or_else(|| DbError::Internal("compaction close lost retained page".to_string()))?;
        if page.close_step()?.is_some() {
            self.credits[index] = self.credits[index].checked_sub(1).ok_or_else(|| DbError::Internal("compaction page credit returned twice".to_string()))?;
            return Ok(true);
        }
        if self.credits[index] != 0 || !page.terminal_is_empty() {
            return Err(DbError::Internal("compaction page reached a false empty witness".to_string()));
        }
        self.pages[index] = None;
        self.len -= 1;
        if self.len == 0 {
            release_compaction_retirement(&mut self.retirement);
        }
        Ok(true)
    }

    fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.credits.iter().all(|credit| *credit == 0) && self.pages.iter().all(Option::is_none)
    }
}

static COMPACTION_PAGE_RETIREMENT: std::sync::Mutex<[Option<CompactionRetainedPages>; COMPACTION_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; COMPACTION_RETIREMENT_SLOTS]);
static COMPACTION_PAGE_RETIREMENT_OVERFLOW: std::sync::Mutex<[Option<CompactionRetainedPages>; COMPACTION_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; COMPACTION_RETIREMENT_SLOTS]);
static COMPACTION_PAGE_RETIREMENT_QUARANTINE: std::sync::Mutex<[Option<CompactionRetainedPages>; COMPACTION_RETIREMENT_SLOTS]> = std::sync::Mutex::new([const { None }; COMPACTION_RETIREMENT_SLOTS]);
static COMPACTION_RETIREMENT_RESERVATIONS: [std::sync::atomic::AtomicU64; 3] = [const { std::sync::atomic::AtomicU64::new(0) }; 3];
static COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[derive(Clone, Copy)]
struct CompactionRetirementReservation {
    tier: u8,
    index: u8,
}

fn reserve_compaction_retirement() -> Option<CompactionRetirementReservation> {
    for tier in 0..3u8 {
        for index in 0..COMPACTION_RETIREMENT_SLOTS as u8 {
            let bit = 1u64 << index;
            if COMPACTION_RETIREMENT_RESERVATIONS[tier as usize].fetch_or(bit, std::sync::atomic::Ordering::AcqRel) & bit != 0 {
                continue;
            }
            let vacant = match tier {
                0 => COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                1 => COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
                _ => COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[index as usize].is_none(),
            };
            if vacant {
                if tier != 0 {
                    COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
                }
                return Some(CompactionRetirementReservation { tier, index });
            }
            COMPACTION_RETIREMENT_RESERVATIONS[tier as usize].fetch_and(!bit, std::sync::atomic::Ordering::AcqRel);
        }
    }
    COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
    None
}

fn release_compaction_retirement(reservation: &mut Option<CompactionRetirementReservation>) {
    if let Some(reservation) = reservation.take() {
        COMPACTION_RETIREMENT_RESERVATIONS[reservation.tier as usize].fetch_and(!(1u64 << reservation.index), std::sync::atomic::Ordering::AcqRel);
    }
}

fn compaction_vacant_retirement_slot(tier: usize, slots: &[Option<CompactionRetainedPages>]) -> Option<usize> {
    let reserved = COMPACTION_RETIREMENT_RESERVATIONS[tier].load(std::sync::atomic::Ordering::Acquire);
    slots.iter().enumerate().position(|(index, slot)| slot.is_none() && reserved & (1u64 << index) == 0)
}

fn install_reserved_compaction_pages(owner: CompactionRetainedPages) {
    let reservation = owner.retirement.unwrap_or(CompactionRetirementReservation { tier: 0, index: 0 });
    match reservation.tier {
        0 => COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        1 => COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
        _ => COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)[reservation.index as usize] = Some(owner),
    }
}

fn retire_compaction_pages(owner: CompactionRetainedPages) -> Result<(), CompactionRetainedPages> {
    if owner.retirement.is_some() {
        install_reserved_compaction_pages(owner);
        return Ok(());
    }
    let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(index) = compaction_vacant_retirement_slot(0, &retired[..]) {
        retired[index] = Some(owner);
        Ok(())
    } else {
        drop(retired);
        COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.store(true, std::sync::atomic::Ordering::Release);
        let mut overflow = COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(index) = compaction_vacant_retirement_slot(1, &overflow[..]) {
            overflow[index] = Some(owner);
            return Ok(());
        }
        drop(overflow);
        let mut quarantine = COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = compaction_vacant_retirement_slot(2, &quarantine[..]) else { return Err(owner) };
        quarantine[index] = Some(owner);
        Ok(())
    }
}

pub fn compaction_page_maintenance_step() -> Result<bool, DbError> {
    let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(slot) = retired.iter_mut().find(|slot| slot.is_some()) else {
        drop(retired);
        let mut overflow = COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(index) = overflow.iter().position(Option::is_some) else {
            drop(overflow);
            let mut quarantine = COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(index) = quarantine.iter().position(Option::is_some) else { return Ok(false) };
            let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(target) = compaction_vacant_retirement_slot(0, &retired[..]) else {
                drop(retired);
                let owner = quarantine[index].as_mut().ok_or_else(|| DbError::Internal("compaction quarantine retirement changed page owner".to_string()))?;
                if !owner.close_step()? {
                    quarantine[index] = None;
                }
                return Ok(true);
            };
            retired[target] = quarantine[index].take();
            return Ok(true);
        };
        let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(target) = compaction_vacant_retirement_slot(0, &retired[..]) else {
            drop(retired);
            let owner = overflow[index].as_mut().ok_or_else(|| DbError::Internal("compaction overflow retirement changed page owner".to_string()))?;
            if !owner.close_step()? {
                overflow[index] = None;
            }
            return Ok(true);
        };
        retired[target] = overflow[index].take();
        return Ok(true);
    };
    let owner = slot.as_mut().ok_or_else(|| DbError::Internal("compaction retirement changed owner".to_string()))?;
    if owner.close_step()? {
        return Ok(true);
    }
    *slot = None;
    Ok(true)
}

impl Drop for CompactionRetainedPages {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            release_compaction_retirement(&mut self.retirement);
            return;
        }
        install_reserved_compaction_pages(std::mem::replace(self, Self::new()));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CompactionCloseExit {
    Running,
    Closed,
    Fault,
}

/// @emoji 🛰️ Mounted close state that advances one retained page opportunity per poll.
struct MountedCompactionPageClose<'owner> {
    owner: &'owner mut CompactionRetainedPages,
    exit: CompactionCloseExit,
}

impl<'owner> MountedCompactionPageClose<'owner> {
    fn new(owner: &'owner mut CompactionRetainedPages) -> Self {
        Self { owner, exit: CompactionCloseExit::Running }
    }
}

impl std::future::Future for MountedCompactionPageClose<'_> {
    type Output = Result<CompactionCloseExit, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        match self.owner.close_step() {
            Ok(true) => {
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Ok(false) => {
                self.exit = CompactionCloseExit::Closed;
                std::task::Poll::Ready(Ok(self.exit))
            }
            Err(error) => {
                self.exit = CompactionCloseExit::Fault;
                std::task::Poll::Ready(Err(error))
            }
        }
    }
}

/// @emoji 🌳️ Walks the snapshot chain from `through_generation` back to its full-baseline root,
/// returning the latest generation's own descriptor plus every page introduced anywhere in the
/// chain, deduplicated by content hash — `SnapshotConsolidator::consolidate`'s input.
#[cfg(test)]
async fn collect_chain_pages<S: db_storage::SnapshotStorage>(
    manager: &db_snapshot::SnapshotManager<'_, S>,
    document: &ArtifactId,
    through_generation: u64,
    budget: &CompactionBudget,
) -> Result<(db_snapshot::SnapshotDescriptor, CompactionRetainedPages), DbError> {
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let mut cursor = manager.chain_cursor(document, through_generation, control);
    let latest_descriptor = cursor.latest_descriptor().await?;
    let mut descriptor = latest_descriptor.clone();
    let mut pages = CompactionRetainedPages::new();
    let mut generations_walked = 0u64;
    loop {
        generations_walked += 1;
        check_len(generations_walked, budget.max_snapshot_generations, "db_compact::snapshot_chain_depth")?;
        for hash in descriptor.new_pages.iter().copied() {
            if !pages.contains(hash) {
                pages.preflight_push()?;
                let bytes = cursor.read_page(hash).await?;
                let page = db_state::Page::try_from_pages(bytes).await?;
                pages.push_preflighted(page);
            }
        }
        match descriptor.parent_generation {
            Some(parent) => descriptor = cursor.descriptor(parent).await?,
            None => break,
        }
    }
    let _ = cursor.close_step()?;
    Ok((latest_descriptor, pages))
}

/// @emoji 🧑️‍💼️ Rolls up a document's incremental snapshot chain into a fresh, self-sufficient
/// full baseline — the responsibility `db_snapshot`'s own module doc explicitly defers to this
/// crate (see that crate's "Scope boundary" note).
#[cfg(test)]
pub struct SnapshotConsolidator<'storage, S: db_storage::SnapshotStorage> {
    manager: db_snapshot::SnapshotManager<'storage, S>,
}

#[cfg(test)]
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
        let (latest, mut pages) = collect_chain_pages(&self.manager, document, through_generation, budget).await?;
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
        let publication = self.manager.publish_retained(document, db_snapshot::SnapshotOrigin::FullBaseline, pages.slots(), pages.len(), body).await;
        let close = MountedCompactionPageClose::new(&mut pages).await;
        match (publication, close) {
            (Ok(generation), Ok(CompactionCloseExit::Closed)) => Ok(generation),
            (Err(error), _) | (_, Err(error)) => Err(error),
            _ => Err(DbError::Internal("compaction close produced a false exit witness".to_string())),
        }
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
#[cfg(test)]
pub async fn build_cold_archive(storage: &impl db_storage::SnapshotStorage, document: &ArtifactId, through_generation: u64) -> Result<db_storage::DbIoPages, DbError> {
    let manager = db_snapshot::SnapshotManager::new(storage).await;
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let mut cursor = manager.chain_cursor(document, through_generation, control);
    let pages = cursor.materialize_pages().await?;
    let _ = cursor.close_step()?;
    Ok(pages)
}
//#endregion 🔖️ColdArchive

//#region 🔖️Compactor
/// @emoji 📋️ What one `Compactor::run` pass did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompactionReport {
    pub wal_segments_deleted: u64,
    pub payloads_deleted: u64,
    pub index_reports: CompactionIndexReports,
    pub snapshot_consolidated_generation: Option<u64>,
    pub snapshot_generations_pruned: u64,
}

/// @emoji 🧑️‍💼️ The top-level, fenced, budgeted orchestrator gluing every subsystem in this crate
/// together over one `db_storage::DbStorage` backend — "online compaction with manifest CAS +
/// fencing" (see module doc's design-choice note on the fencing mechanism).
#[cfg(test)]
pub struct Compactor<'storage> {
    storage: &'storage db_storage::DbBackend,
}

#[cfg(test)]
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
        let cancelled = std::sync::atomic::AtomicBool::new(false);
        let horizons = committed_compaction_horizons(&wal, document, &cancelled).await?;
        let active_segment = horizons.get(horizons.len().saturating_sub(1)).map(|horizon| horizon.segment_index);
        let mut selected = DatabaseCompactionSegmentOwners::new();
        for index in 0..horizons.len() {
            let horizon = horizons.get(index).ok_or_else(|| DbError::Internal("database compaction horizon owner lost".to_string()))?;
            if Some(horizon.segment_index) != active_segment && horizon.max_head_seq.is_some_and(|head| head <= wal_floor_head_seq) && selected.len() < usize::try_from(budget.max_wal_segments).unwrap_or(usize::MAX) {
                selected.push(horizon)?;
            }
        }
        let (candidates, live) = committed_compaction_payloads(&wal, document, &selected, &cancelled).await?;
        for index in 0..selected.len() {
            let segment = selected.get(index).ok_or_else(|| DbError::Internal("database compaction selected segment owner lost".to_string()))?;
            wal.delete_segment(document, segment.segment_index).await?;
            report.wal_segments_deleted += 1;
        }
        let payloads = self.storage.payload().await;
        for index in 0..candidates.len().min(usize::try_from(budget.max_payloads).unwrap_or(usize::MAX)) {
            let hash = candidates.get(index).ok_or_else(|| DbError::Internal("database compaction candidate hash owner lost".to_string()))?;
            if !live.contains(hash, &cancelled).await? {
                payloads.delete(&hash).await?;
                report.payloads_deleted += 1;
            }
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

//#region 🧵️RetainedCompactionJob
const DATABASE_COMPACTION_SLOTS: usize = 32;
const DATABASE_COMPACTION_MAX_SEGMENTS: usize = 64;
const DATABASE_COMPACTION_MAX_HASHES: usize = 4_096;
const DATABASE_COMPACTION_OPERATION_ITEMS: u64 = 16_768;
const DATABASE_COMPACTION_OPERATION_BYTES: u64 = 2 * 1024 * 1024;
const DATABASE_COMPACTION_TOTAL_ITEMS: u64 = DATABASE_COMPACTION_OPERATION_ITEMS * DATABASE_COMPACTION_SLOTS as u64;
const DATABASE_COMPACTION_TOTAL_BYTES: u64 = DATABASE_COMPACTION_OPERATION_BYTES * DATABASE_COMPACTION_SLOTS as u64;
const DATABASE_COMPACTION_RETRY_LIMIT: u8 = 8;
const DATABASE_COMPACTION_DEADLINE_MS: u64 = 30_000;
const DATABASE_COMPACTION_TURN_MS: u64 = 8;
const DATABASE_COMPACTION_INDEX_FUEL: usize = 256;

#[derive(Default)]
struct DatabaseCompactionBackingLedger {
    items: u64,
    bytes: u64,
}

impl DatabaseCompactionBackingLedger {
    fn observe(&mut self, items: usize, bytes: usize, label: &'static str) -> Result<(), DbError> {
        let items = u64::try_from(items).map_err(|_| DbError::LimitExceeded(label))?;
        let bytes = u64::try_from(bytes).map_err(|_| DbError::LimitExceeded(label))?;
        let next_items = self.items.checked_add(items).ok_or(DbError::LimitExceeded(label))?;
        let next_bytes = self.bytes.checked_add(bytes).ok_or(DbError::LimitExceeded(label))?;
        if next_items > DATABASE_COMPACTION_OPERATION_ITEMS || next_bytes > DATABASE_COMPACTION_OPERATION_BYTES {
            return Err(DbError::LimitExceeded(label));
        }
        self.items = next_items;
        self.bytes = next_bytes;
        Ok(())
    }

    fn release(&mut self, items: usize, bytes: usize) -> Result<(), DbError> {
        self.items = self.items.checked_sub(u64::try_from(items).map_err(|_| DbError::LimitExceeded("database compaction backing release items"))?).ok_or_else(|| DbError::Internal("database compaction backing items returned twice".to_string()))?;
        self.bytes = self.bytes.checked_sub(u64::try_from(bytes).map_err(|_| DbError::LimitExceeded("database compaction backing release bytes"))?).ok_or_else(|| DbError::Internal("database compaction backing bytes returned twice".to_string()))?;
        Ok(())
    }
}

fn database_compaction_observe_backing(ledger: &mut DatabaseCompactionBackingLedger, items: usize, bytes: usize, label: &'static str) -> Result<(), DbError> {
    ledger.observe(items, bytes, label)
}

fn database_compaction_descriptor_backing(descriptor: &db_snapshot::SnapshotDescriptor) -> Result<(usize, usize), DbError> {
    let items = descriptor
        .roots
        .capacity()
        .checked_add(descriptor.new_pages.capacity())
        .and_then(|value| value.checked_add(usize::from(descriptor.vcs_head.is_some())))
        .and_then(|value| value.checked_add(1))
        .ok_or(DbError::LimitExceeded("database compaction snapshot backing items"))?;
    let hash_bytes =
        descriptor.roots.capacity().checked_add(descriptor.new_pages.capacity()).and_then(|value| value.checked_mul(std::mem::size_of::<pack::ContentHash>())).ok_or(DbError::LimitExceeded("database compaction snapshot backing bytes"))?;
    let bytes = hash_bytes.checked_add(descriptor.document.0.capacity()).and_then(|value| value.checked_add(descriptor.vcs_head.as_ref().map_or(0, String::capacity))).ok_or(DbError::LimitExceeded("database compaction snapshot backing bytes"))?;
    Ok((items, bytes))
}

async fn retire_compaction_descriptor(mut descriptor: db_snapshot::SnapshotDescriptor) {
    let new_pages = std::mem::take(&mut descriptor.new_pages);
    semio_framework_async::yield_once().await;
    drop(new_pages);
    let roots = std::mem::take(&mut descriptor.roots);
    semio_framework_async::yield_once().await;
    drop(roots);
    if let Some(vcs_head) = descriptor.vcs_head.take() {
        semio_framework_async::yield_once().await;
        drop(vcs_head);
    }
    let document = std::mem::take(&mut descriptor.document.0);
    semio_framework_async::yield_once().await;
    drop(document);
}

async fn close_compaction_descriptor(descriptor: db_snapshot::SnapshotDescriptor, ledger: &mut DatabaseCompactionBackingLedger) -> Result<(), DbError> {
    let (items, bytes) = database_compaction_descriptor_backing(&descriptor)?;
    retire_compaction_descriptor(descriptor).await;
    ledger.release(items, bytes)
}

async fn retire_compaction_snapshot_body(mut body: db_snapshot::SnapshotBody) {
    let roots = std::mem::take(&mut body.roots);
    semio_framework_async::yield_once().await;
    drop(roots);
    if let Some(vcs_head) = body.vcs_head.take() {
        semio_framework_async::yield_once().await;
        drop(vcs_head);
    }
}

async fn close_compaction_snapshot_body(body: db_snapshot::SnapshotBody, ledger: &mut DatabaseCompactionBackingLedger, charge: (usize, usize)) -> Result<(), DbError> {
    retire_compaction_snapshot_body(body).await;
    ledger.release(charge.0, charge.1)
}

async fn database_compaction_admit_descriptor(descriptor: db_snapshot::SnapshotDescriptor, ledger: &mut DatabaseCompactionBackingLedger) -> Result<db_snapshot::SnapshotDescriptor, DbError> {
    let observed = database_compaction_descriptor_backing(&descriptor).and_then(|(items, bytes)| database_compaction_observe_backing(ledger, items, bytes, "database compaction snapshot backing"));
    if let Err(error) = observed {
        retire_compaction_descriptor(descriptor).await;
        return Err(error);
    }
    Ok(descriptor)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseCompactionProgress {
    Admitted,
    SnapshotFloor,
    LeaseAcquire,
    WalHorizon,
    PayloadTrace,
    WalDelete,
    PayloadDelete,
    IndexMerge,
    SnapshotCollect,
    SnapshotPublish,
    SnapshotRetain,
    LeaseRelease,
    Completed,
    Cancelled,
    Fault,
}

impl DatabaseCompactionProgress {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Admitted,
            1 => Self::SnapshotFloor,
            2 => Self::LeaseAcquire,
            3 => Self::WalHorizon,
            4 => Self::PayloadTrace,
            5 => Self::WalDelete,
            6 => Self::PayloadDelete,
            7 => Self::IndexMerge,
            8 => Self::SnapshotCollect,
            9 => Self::SnapshotPublish,
            10 => Self::SnapshotRetain,
            11 => Self::LeaseRelease,
            12 => Self::Completed,
            13 => Self::Cancelled,
            _ => Self::Fault,
        }
    }
}

#[derive(Clone, Copy)]
struct DatabaseCompactionAdmissionSlot {
    generation: u64,
    occupied: bool,
}

const EMPTY_DATABASE_COMPACTION_SLOT: DatabaseCompactionAdmissionSlot = DatabaseCompactionAdmissionSlot { generation: 0, occupied: false };

struct DatabaseCompactionAdmissionState {
    slots: [DatabaseCompactionAdmissionSlot; DATABASE_COMPACTION_SLOTS],
    items: u64,
    bytes: u64,
    next_generation: u64,
}

impl DatabaseCompactionAdmissionState {
    fn try_claim(&mut self, document: &ArtifactId) -> Result<(usize, u64), DbError> {
        if document.0.len() > db_storage::DbIoText::maximum_capacity() || document.0.capacity() > db_storage::DbIoText::maximum_capacity() {
            return Err(DbError::LimitExceeded("database compaction document backing"));
        }
        let slot = self.slots.iter().position(|entry| !entry.occupied).ok_or(DbError::LimitExceeded("database compaction admission slots"))?;
        let items = self.items.checked_add(DATABASE_COMPACTION_OPERATION_ITEMS).ok_or(DbError::LimitExceeded("database compaction aggregate items"))?;
        let bytes = self.bytes.checked_add(DATABASE_COMPACTION_OPERATION_BYTES).ok_or(DbError::LimitExceeded("database compaction aggregate bytes"))?;
        if items > DATABASE_COMPACTION_TOTAL_ITEMS || bytes > DATABASE_COMPACTION_TOTAL_BYTES {
            return Err(DbError::LimitExceeded("database compaction aggregate capacity"));
        }
        let generation = self.next_generation;
        self.next_generation = generation.checked_add(1).ok_or(DbError::LimitExceeded("database compaction generation"))?;
        self.slots[slot] = DatabaseCompactionAdmissionSlot { generation, occupied: true };
        self.items = items;
        self.bytes = bytes;
        Ok((slot, generation))
    }

    fn is_current(&self, slot: usize, generation: u64) -> bool {
        self.slots.get(slot).is_some_and(|entry| entry.occupied && entry.generation == generation)
    }

    fn release(&mut self, slot: usize, generation: u64) -> bool {
        if !self.is_current(slot, generation) {
            return false;
        }
        let Some(items) = self.items.checked_sub(DATABASE_COMPACTION_OPERATION_ITEMS) else { return false };
        let Some(bytes) = self.bytes.checked_sub(DATABASE_COMPACTION_OPERATION_BYTES) else { return false };
        self.slots[slot] = EMPTY_DATABASE_COMPACTION_SLOT;
        self.items = items;
        self.bytes = bytes;
        true
    }
}

static DATABASE_COMPACTION_ADMISSION: std::sync::Mutex<DatabaseCompactionAdmissionState> =
    std::sync::Mutex::new(DatabaseCompactionAdmissionState { slots: [EMPTY_DATABASE_COMPACTION_SLOT; DATABASE_COMPACTION_SLOTS], items: 0, bytes: 0, next_generation: 1 });

struct DatabaseCompactionAdmission {
    slot: usize,
    generation: u64,
}

impl DatabaseCompactionAdmission {
    fn try_claim(document: &ArtifactId) -> Result<Self, DbError> {
        let (slot, generation) = DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).try_claim(document)?;
        Ok(Self { slot, generation })
    }

    fn is_current(&self) -> bool {
        DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_current(self.slot, self.generation)
    }
}

impl Drop for DatabaseCompactionAdmission {
    fn drop(&mut self) {
        DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).release(self.slot, self.generation);
    }
}

struct DatabaseCompactionSegmentOwners {
    slots: [Option<SegmentHorizon>; DATABASE_COMPACTION_MAX_SEGMENTS],
    len: u8,
}

impl DatabaseCompactionSegmentOwners {
    fn new() -> Self {
        Self { slots: [None; DATABASE_COMPACTION_MAX_SEGMENTS], len: 0 }
    }

    fn push(&mut self, value: SegmentHorizon) -> Result<(), DbError> {
        let index = usize::from(self.len);
        *self.slots.get_mut(index).ok_or(DbError::LimitExceeded("database compaction WAL segment owners"))? = Some(value);
        self.len = self.len.checked_add(1).ok_or(DbError::LimitExceeded("database compaction WAL segment cursor"))?;
        Ok(())
    }

    fn get(&self, index: usize) -> Option<SegmentHorizon> {
        self.slots.get(index).copied().flatten()
    }

    fn len(&self) -> usize {
        usize::from(self.len)
    }

    fn observe_head(&mut self, segment_index: u64, head_seq: u64) -> Result<(), DbError> {
        let horizon = self
            .slots
            .iter_mut()
            .take(usize::from(self.len))
            .flatten()
            .find(|horizon| horizon.segment_index == segment_index)
            .ok_or_else(|| DbError::Corrupt("committed WAL transaction names an unknown segment".to_string()))?;
        horizon.max_head_seq = Some(horizon.max_head_seq.map_or(head_seq, |head| head.max(head_seq)));
        Ok(())
    }
}

struct DatabaseCompactionHashOwners {
    slots: [Option<pack::ContentHash>; DATABASE_COMPACTION_MAX_HASHES],
    len: u16,
}

impl DatabaseCompactionHashOwners {
    fn new() -> Self {
        Self { slots: [None; DATABASE_COMPACTION_MAX_HASHES], len: 0 }
    }

    async fn contains(&self, hash: pack::ContentHash, cancelled: &std::sync::atomic::AtomicBool) -> Result<bool, DbError> {
        for slot in self.slots.iter().take(usize::from(self.len)) {
            compaction_opportunity(cancelled).await?;
            if *slot == Some(hash) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn insert(&mut self, hash: pack::ContentHash, cancelled: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
        if self.contains(hash, cancelled).await? {
            return Ok(());
        }
        let index = usize::from(self.len);
        *self.slots.get_mut(index).ok_or(DbError::LimitExceeded("database compaction payload hash owners"))? = Some(hash);
        self.len = self.len.checked_add(1).ok_or(DbError::LimitExceeded("database compaction payload hash cursor"))?;
        Ok(())
    }

    fn get(&self, index: usize) -> Option<pack::ContentHash> {
        self.slots.get(index).copied().flatten()
    }

    fn len(&self) -> usize {
        usize::from(self.len)
    }
}

async fn compaction_opportunity(cancelled: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
    semio_framework_async::yield_once().await;
    if cancelled.load(std::sync::atomic::Ordering::Acquire) {
        return Err(DbError::Closed);
    }
    Ok(())
}

fn compaction_resource(document: &ArtifactId) -> Result<db_storage::DbIoText, DbError> {
    use std::fmt::Write as _;
    let mut resource = db_storage::DbIoText::try_from_str("compact:")?;
    write!(&mut resource, "{}", document.0).map_err(|_| DbError::LimitExceeded("database compaction lease resource"))?;
    Ok(resource)
}

async fn close_compaction_replay<S: db_storage::WalStorage>(replay: &mut db_wal::WalCommittedCursor<'_, S>) -> Result<(), DbError> {
    close_compaction_owner(|| replay.close_owner_step()).await
}

async fn close_compaction_owner(mut close: impl FnMut() -> Result<bool, DbError>) -> Result<(), DbError> {
    std::future::poll_fn(move |context| match close() {
        Ok(true) => {
            context.waker().wake_by_ref();
            std::task::Poll::Pending
        }
        Ok(false) => std::task::Poll::Ready(Ok(())),
        Err(error) => std::task::Poll::Ready(Err(error)),
    })
    .await
}

async fn close_compaction_page(mut page: db_state::Page) -> Result<(), DbError> {
    close_compaction_owner(|| Ok(page.close_step()?.is_some())).await
}

async fn committed_compaction_horizons<S: db_storage::WalStorage>(
    storage: &S,
    document: &ArtifactId,
    cancelled: &std::sync::atomic::AtomicBool,
) -> Result<DatabaseCompactionSegmentOwners, DbError> {
    let control = db_wal::WalCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
    let mut replay = db_wal::replay_committed_document(storage, document, control).await?;
    let scan = async {
        let mut horizons = DatabaseCompactionSegmentOwners::new();
        for segment_index in replay.segment_indices() {
            horizons.push(SegmentHorizon { segment_index: *segment_index, max_head_seq: None })?;
        }
        loop {
            compaction_opportunity(cancelled).await?;
            match replay.next_transaction_step().await? {
                db_wal::WalCommittedStep::Transaction(mut transaction) => {
                    let segment_index = transaction.segment_index();
                    loop {
                        match transaction.next_record_step()? {
                            db_wal::WalCommittedRecordStep::Record(record) => {
                                let observed = if let db_wal::WalRecord::Frontier(frontier) | db_wal::WalRecord::SnapshotPub { frontier, .. } = record {
                                    horizons.observe_head(segment_index, frontier.head_seq)
                                } else {
                                    Ok(())
                                };
                                let closed = close_compaction_owner(|| transaction.close_record_step()).await;
                                observed?;
                                closed?;
                            }
                            db_wal::WalCommittedRecordStep::Yield => compaction_opportunity(cancelled).await?,
                            db_wal::WalCommittedRecordStep::Done => break,
                        }
                    }
                    transaction.finish()?;
                }
                db_wal::WalCommittedStep::Yield => {}
                db_wal::WalCommittedStep::Done => break,
            }
        }
        Ok(horizons)
    }
    .await;
    let close = close_compaction_replay(&mut replay).await;
    match (scan, close) {
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(error),
        (Ok(horizons), Ok(())) if replay.terminal_is_empty() => Ok(horizons),
        (Ok(_), Ok(())) => Err(DbError::Internal("database compaction committed horizon cursor retained owners".to_string())),
    }
}

async fn committed_compaction_payloads<S: db_storage::WalStorage>(
    storage: &S,
    document: &ArtifactId,
    selected: &DatabaseCompactionSegmentOwners,
    cancelled: &std::sync::atomic::AtomicBool,
) -> Result<(DatabaseCompactionHashOwners, DatabaseCompactionHashOwners), DbError> {
    let control = db_wal::WalCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
    let mut replay = db_wal::replay_committed_document(storage, document, control).await?;
    let scan = async {
        let mut candidates = DatabaseCompactionHashOwners::new();
        let mut live = DatabaseCompactionHashOwners::new();
        loop {
            compaction_opportunity(cancelled).await?;
            match replay.next_transaction_step().await? {
                db_wal::WalCommittedStep::Transaction(mut transaction) => {
                    let segment_index = transaction.segment_index();
                    loop {
                        let hash = match transaction.next_record_step()? {
                            db_wal::WalCommittedRecordStep::Record(db_wal::WalRecord::Payload(db_wal::WalPayloadRef::CasRef(hash))) => Some(*hash),
                            db_wal::WalCommittedRecordStep::Record(_) => None,
                            db_wal::WalCommittedRecordStep::Yield => {
                                compaction_opportunity(cancelled).await?;
                                continue;
                            }
                            db_wal::WalCommittedRecordStep::Done => break,
                        };
                        close_compaction_owner(|| transaction.close_record_step()).await?;
                        if let Some(hash) = hash {
                            let mut deleted = false;
                            for index in 0..selected.len() {
                                compaction_opportunity(cancelled).await?;
                                deleted |= selected.get(index).is_some_and(|candidate| candidate.segment_index == segment_index);
                            }
                            if deleted { candidates.insert(hash, cancelled).await? } else { live.insert(hash, cancelled).await? }
                        }
                    }
                    transaction.finish()?;
                }
                db_wal::WalCommittedStep::Yield => {}
                db_wal::WalCommittedStep::Done => break,
            }
        }
        Ok((candidates, live))
    }
    .await;
    let close = close_compaction_replay(&mut replay).await;
    match (scan, close) {
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(error),
        (Ok(owners), Ok(())) if replay.terminal_is_empty() => Ok(owners),
        (Ok(_), Ok(())) => Err(DbError::Internal("database compaction committed payload cursor retained owners".to_string())),
    }
}

async fn retained_compaction_under_lease(
    storage: &db_storage::DbBackend,
    document: &ArtifactId,
    floor_head_seq: u64,
    consolidate_snapshots: bool,
    budget: CompactionBudget,
    cancelled: &Arc<std::sync::atomic::AtomicBool>,
    progress: &std::sync::atomic::AtomicU8,
    ledger: &mut DatabaseCompactionBackingLedger,
) -> Result<CompactionReport, DbError> {
    let mut report = CompactionReport::default();
    let wal = storage.wal().await;
    progress.store(DatabaseCompactionProgress::WalHorizon as u8, std::sync::atomic::Ordering::Release);
    let horizons = committed_compaction_horizons(&wal, document, cancelled).await?;
    let active_segment = horizons.get(horizons.len().saturating_sub(1)).map(|horizon| horizon.segment_index);
    let mut selected = DatabaseCompactionSegmentOwners::new();
    for index in 0..horizons.len() {
        compaction_opportunity(cancelled).await?;
        let horizon = horizons.get(index).ok_or_else(|| DbError::Internal("database compaction horizon owner lost".to_string()))?;
        if Some(horizon.segment_index) != active_segment && horizon.max_head_seq.is_some_and(|head| head <= floor_head_seq) && selected.len() < usize::try_from(budget.max_wal_segments).unwrap_or(usize::MAX) {
            selected.push(horizon)?;
        }
    }

    progress.store(DatabaseCompactionProgress::PayloadTrace as u8, std::sync::atomic::Ordering::Release);
    let (candidates, live) = committed_compaction_payloads(&wal, document, &selected, cancelled).await?;

    progress.store(DatabaseCompactionProgress::WalDelete as u8, std::sync::atomic::Ordering::Release);
    for index in 0..selected.len() {
        compaction_opportunity(cancelled).await?;
        let segment = selected.get(index).ok_or_else(|| DbError::Internal("database compaction selected segment owner lost".to_string()))?;
        wal.delete_segment(document, segment.segment_index).await?;
        report.wal_segments_deleted = report.wal_segments_deleted.checked_add(1).ok_or(DbError::LimitExceeded("database compaction deleted segments"))?;
    }
    drop(wal);

    progress.store(DatabaseCompactionProgress::PayloadDelete as u8, std::sync::atomic::Ordering::Release);
    let payload = storage.payload().await;
    for index in 0..candidates.len().min(usize::try_from(budget.max_payloads).unwrap_or(usize::MAX)) {
        compaction_opportunity(cancelled).await?;
        let hash = candidates.get(index).ok_or_else(|| DbError::Internal("database compaction candidate hash owner lost".to_string()))?;
        if !live.contains(hash, cancelled).await? {
            payload.delete(&hash).await?;
            report.payloads_deleted = report.payloads_deleted.checked_add(1).ok_or(DbError::LimitExceeded("database compaction deleted payloads"))?;
        }
    }
    drop(payload);

    progress.store(DatabaseCompactionProgress::IndexMerge as u8, std::sync::atomic::Ordering::Release);
    let index_storage = storage.index().await;
    for kind in db_index::IndexKind::ALL {
        compaction_opportunity(cancelled).await?;
        let index_document = document.clone();
        let index_document_bytes = index_document.0.capacity();
        database_compaction_observe_backing(ledger, 1, index_document_bytes, "database compaction index document backing")?;
        let handle = db_index::IndexHandle::new(&index_storage, index_document, kind).await;
        let stats = loop {
            let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_COMPACTION_TURN_MS);
            let mut control = match handle.retained_operation_control(cancelled.clone(), deadline, DATABASE_COMPACTION_INDEX_FUEL) {
                Ok(control) => control,
                Err(error) => break Err(error),
            };
            match handle.compact(&mut control).await {
                Ok(stats) => break Ok(stats),
                Err(DbError::LimitExceeded("index cursor fuel")) => {
                    if let Err(error) = compaction_opportunity(cancelled).await {
                        break Err(error);
                    }
                }
                Err(DbError::Unavailable(message)) if message == "index cursor deadline reached" => {
                    if let Err(error) = compaction_opportunity(cancelled).await {
                        break Err(error);
                    }
                }
                Err(DbError::Unavailable(message)) if message == "index cursor cancelled" => break Err(DbError::Closed),
                Err(error) => break Err(error),
            }
        };
        drop(handle);
        ledger.release(1, index_document_bytes)?;
        let stats = stats?;
        report.index_reports.push(IndexKindReport { kind, stats })?;
    }
    drop(index_storage);

    if consolidate_snapshots {
        retained_compaction_snapshot(storage, document, budget, cancelled, progress, ledger, &mut report).await?;
    }
    Ok(report)
}

async fn retained_compaction_snapshot(
    storage: &db_storage::DbBackend,
    document: &ArtifactId,
    budget: CompactionBudget,
    cancelled: &Arc<std::sync::atomic::AtomicBool>,
    progress: &std::sync::atomic::AtomicU8,
    ledger: &mut DatabaseCompactionBackingLedger,
    report: &mut CompactionReport,
) -> Result<(), DbError> {
    let snapshot = storage.snapshot().await;
    let manager = db_snapshot::SnapshotManager::new(&snapshot).await;
    let Some((latest_generation, latest_descriptor)) = manager.load_latest(document).await? else { return Ok(()) };
    let mut latest_descriptor = Some(database_compaction_admit_descriptor(latest_descriptor, ledger).await?);
    let control = match db_snapshot::SnapshotCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_COMPACTION_TURN_MS), DATABASE_COMPACTION_INDEX_FUEL) {
        Ok(control) => control,
        Err(error) => {
            if let Some(owner) = latest_descriptor.take() {
                close_compaction_descriptor(owner, ledger).await?;
            }
            return Err(error);
        }
    };
    let mut cursor = manager.chain_cursor(document, latest_generation, control);
    let mut descriptor = None;
    let mut pages = CompactionRetainedPages::new();
    let mut page_items = 0usize;
    let mut page_bytes = 0usize;
    let mut generations = 0u64;
    progress.store(DatabaseCompactionProgress::SnapshotCollect as u8, std::sync::atomic::Ordering::Release);
    let collection = async {
        descriptor = Some(database_compaction_admit_descriptor(cursor.descriptor(latest_generation).await?, ledger).await?);
        loop {
            generations = generations.checked_add(1).ok_or(DbError::LimitExceeded("database compaction snapshot generations"))?;
            check_len(generations, budget.max_snapshot_generations, "database compaction snapshot generations")?;
            let hash_count = descriptor.as_ref().ok_or_else(|| DbError::Internal("database compaction descriptor cursor lost".to_string()))?.new_pages.len();
            for index in 0..hash_count {
                let hash = descriptor.as_ref().and_then(|owner| owner.new_pages.get(index)).copied().ok_or_else(|| DbError::Internal("database compaction descriptor hash cursor lost".to_string()))?;
                compaction_opportunity(cancelled).await?;
                if !pages.contains(hash) {
                    pages.preflight_push()?;
                    let source = cursor.read_page(hash).await?;
                    let page = db_state::Page::try_from_pages(source).await?;
                    let items = usize::from(page.pages().page_count());
                    let bytes = items.checked_mul(db_storage::DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("database compaction snapshot page backing"))?;
                    let next_items = match page_items.checked_add(items) {
                        Some(next_items) => next_items,
                        None => {
                            close_compaction_page(page).await?;
                            return Err(DbError::LimitExceeded("database compaction snapshot page items"));
                        }
                    };
                    let next_bytes = match page_bytes.checked_add(bytes) {
                        Some(next_bytes) => next_bytes,
                        None => {
                            close_compaction_page(page).await?;
                            return Err(DbError::LimitExceeded("database compaction snapshot page bytes"));
                        }
                    };
                    if let Err(error) = database_compaction_observe_backing(ledger, items, bytes, "database compaction snapshot page backing") {
                        close_compaction_page(page).await?;
                        return Err(error);
                    }
                    page_items = next_items;
                    page_bytes = next_bytes;
                    pages.push_preflighted(page);
                }
            }
            match descriptor.as_ref().and_then(|owner| owner.parent_generation) {
                Some(parent) => {
                    let next = database_compaction_admit_descriptor(cursor.descriptor(parent).await?, ledger).await?;
                    let previous = descriptor.replace(next).ok_or_else(|| DbError::Internal("database compaction descriptor replacement lost".to_string()))?;
                    close_compaction_descriptor(previous, ledger).await?;
                }
                None => {
                    let previous = descriptor.take().ok_or_else(|| DbError::Internal("database compaction descriptor terminal lost".to_string()))?;
                    close_compaction_descriptor(previous, ledger).await?;
                    break;
                }
            }
        }
        Ok::<(), DbError>(())
    }
    .await;
    if let Err(error) = collection {
        if let Some(owner) = descriptor.take() {
            close_compaction_descriptor(owner, ledger).await?;
        }
        if let Some(owner) = latest_descriptor.take() {
            close_compaction_descriptor(owner, ledger).await?;
        }
        close_compaction_owner(|| cursor.close_step()).await?;
        close_compaction_owner(|| pages.close_step()).await?;
        ledger.release(page_items, page_bytes)?;
        return Err(error);
    }
    let prepublication = async {
        close_compaction_owner(|| cursor.close_step()).await?;
        progress.store(DatabaseCompactionProgress::SnapshotPublish as u8, std::sync::atomic::Ordering::Release);
        compaction_opportunity(cancelled).await
    }
    .await;
    if let Err(error) = prepublication {
        if let Some(owner) = latest_descriptor.take() {
            close_compaction_descriptor(owner, ledger).await?;
        }
        close_compaction_owner(|| pages.close_step()).await?;
        ledger.release(page_items, page_bytes)?;
        return Err(error);
    }
    let latest_descriptor = latest_descriptor.take().ok_or_else(|| DbError::Internal("database compaction latest descriptor lost".to_string()))?;
    let latest_descriptor_charge = database_compaction_descriptor_backing(&latest_descriptor)?;
    let db_snapshot::SnapshotDescriptor { document: ArtifactId(latest_document), head_seq, commit_seq, epoch, chain_hash, protocol_version, vcs_head, base_pack_hash, roots, new_pages: latest_new_pages, created_at_ms, .. } = latest_descriptor;
    semio_framework_async::yield_once().await;
    drop(latest_new_pages);
    semio_framework_async::yield_once().await;
    drop(latest_document);
    let mut body = db_snapshot::SnapshotBody { head_seq, commit_seq, epoch, chain_hash, protocol_version, vcs_head, base_pack_hash, roots, created_at_ms };
    let new_generation = loop {
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_COMPACTION_TURN_MS);
        let mut control = db_snapshot::SnapshotCursorControl::new(cancelled.clone(), deadline, DATABASE_COMPACTION_INDEX_FUEL)?;
        match manager.publish_retained_expected(document, latest_generation, pages.slots(), pages.len(), body, &mut control).await {
            Ok(publication) => {
                let (generation, returned) = publication.into_parts();
                close_compaction_snapshot_body(returned, ledger, latest_descriptor_charge).await?;
                break generation;
            }
            Err(rejected) => {
                let (error, returned) = rejected.into_parts();
                body = returned;
                match error {
                    DbError::LimitExceeded("snapshot cursor fuel") => compaction_opportunity(cancelled).await?,
                    DbError::Unavailable(message) if message == "snapshot cursor deadline reached" => compaction_opportunity(cancelled).await?,
                    DbError::Unavailable(message) if message == "snapshot cursor cancelled" => {
                        close_compaction_snapshot_body(body, ledger, latest_descriptor_charge).await?;
                        close_compaction_owner(|| pages.close_step()).await?;
                        ledger.release(page_items, page_bytes)?;
                        return Err(DbError::Closed);
                    }
                    error => {
                        close_compaction_snapshot_body(body, ledger, latest_descriptor_charge).await?;
                        close_compaction_owner(|| pages.close_step()).await?;
                        ledger.release(page_items, page_bytes)?;
                        return Err(error);
                    }
                }
            }
        }
    };
    report.snapshot_consolidated_generation = Some(new_generation);
    progress.store(DatabaseCompactionProgress::SnapshotRetain as u8, std::sync::atomic::Ordering::Release);
    let mut generations = match snapshot.list_generations(document).await {
        Ok(generations) => generations,
        Err(error) => {
            close_compaction_owner(|| pages.close_step()).await?;
            ledger.release(page_items, page_bytes)?;
            return Err(error);
        }
    };
    let retention = async {
        for generation in generations.as_slice().iter().copied() {
            compaction_opportunity(cancelled).await?;
            if generation < new_generation {
                snapshot.delete_generation(document, generation).await?;
                report.snapshot_generations_pruned = report.snapshot_generations_pruned.checked_add(1).ok_or(DbError::LimitExceeded("database compaction pruned generations"))?;
            }
        }
        Ok::<(), DbError>(())
    }
    .await;
    close_compaction_owner(|| Ok(generations.close_step())).await?;
    close_compaction_owner(|| pages.close_step()).await?;
    ledger.release(page_items, page_bytes)?;
    retention
}

struct DatabaseCompactionExecution {
    storage: Arc<db_storage::DbBackend>,
    document: ArtifactId,
    holder: db_storage::DbIoText,
    result: Result<CompactionReport, DbError>,
}

type DatabaseCompactionLeaseReleaseFuture = std::pin::Pin<Box<dyn Future<Output = Result<(), DbError>> + Send + 'static>>;

struct DatabaseCompactionLeaseRecovery {
    storage: Arc<db_storage::DbBackend>,
    resource: db_storage::DbIoText,
    holder: db_storage::DbIoText,
    fence: std::sync::Mutex<Option<EpochFence>>,
    releasing: std::sync::atomic::AtomicBool,
    released: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    controlled_release_failures: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    release_attempts: std::sync::atomic::AtomicUsize,
}

impl DatabaseCompactionLeaseRecovery {
    fn new(storage: Arc<db_storage::DbBackend>, resource: db_storage::DbIoText, holder: db_storage::DbIoText) -> Arc<Self> {
        Arc::new(Self {
            storage,
            resource,
            holder,
            fence: std::sync::Mutex::new(None),
            releasing: std::sync::atomic::AtomicBool::new(false),
            released: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            controlled_release_failures: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            release_attempts: std::sync::atomic::AtomicUsize::new(0),
        })
    }

    fn install(&self, fence: EpochFence) {
        *self.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(fence);
    }

    fn release_future(self: &Arc<Self>) -> Option<DatabaseCompactionLeaseReleaseFuture> {
        if self.released.load(std::sync::atomic::Ordering::Acquire) || self.releasing.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return None;
        }
        let fence = *self.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(fence) = fence else {
            self.released.store(true, std::sync::atomic::Ordering::Release);
            return None;
        };
        let recovery = self.clone();
        Some(Box::pin(async move {
            #[cfg(test)]
            recovery.release_attempts.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            #[cfg(test)]
            let injected = recovery.controlled_release_failures.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |remaining| remaining.checked_sub(1)).is_ok();
            #[cfg(not(test))]
            let injected = false;
            let result = if injected { Err(DbError::Unavailable("database compaction controlled lease release failure".to_string())) } else { recovery.storage.lease().await.release(recovery.resource.as_str(), recovery.holder.as_str(), fence).await };
            if result.is_ok() {
                recovery.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                recovery.released.store(true, std::sync::atomic::Ordering::Release);
            } else {
                recovery.releasing.store(false, std::sync::atomic::Ordering::Release);
            }
            result
        }))
    }

    fn retry_release_after_panic(self: &Arc<Self>) -> Option<DatabaseCompactionLeaseReleaseFuture> {
        self.releasing.store(false, std::sync::atomic::Ordering::Release);
        self.release_future()
    }

    #[cfg(test)]
    fn fail_release_attempts(&self, attempts: usize) {
        self.controlled_release_failures.store(attempts, std::sync::atomic::Ordering::Release);
    }
}

async fn retained_compaction_execute(
    storage: Arc<db_storage::DbBackend>,
    document: ArtifactId,
    holder: db_storage::DbIoText,
    consolidate_snapshots: bool,
    budget: CompactionBudget,
    now_ms: u64,
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    expired: Arc<std::sync::atomic::AtomicBool>,
    progress: Arc<std::sync::atomic::AtomicU8>,
    lease_recovery: Arc<DatabaseCompactionLeaseRecovery>,
) -> DatabaseCompactionExecution {
    let mut result = async {
        let mut ledger = DatabaseCompactionBackingLedger::default();
        compaction_opportunity(cancelled.as_ref()).await?;
        progress.store(DatabaseCompactionProgress::SnapshotFloor as u8, std::sync::atomic::Ordering::Release);
        let snapshot = storage.snapshot().await;
        let floor = match db_snapshot::SnapshotManager::new(&snapshot).await.load_latest(&document).await? {
            Some((_, descriptor)) => {
                let descriptor = database_compaction_admit_descriptor(descriptor, &mut ledger).await?;
                let floor = descriptor.head_seq;
                close_compaction_descriptor(descriptor, &mut ledger).await?;
                floor
            }
            None => 0,
        };
        drop(snapshot);
        compaction_opportunity(cancelled.as_ref()).await?;
        progress.store(DatabaseCompactionProgress::LeaseAcquire as u8, std::sync::atomic::Ordering::Release);
        let lease = storage.lease().await;
        let fence = lease.acquire(lease_recovery.resource.as_str(), holder.as_str(), DEFAULT_LEASE_TTL_MS, now_ms).await?;
        drop(lease);
        lease_recovery.install(fence);
        let run = retained_compaction_under_lease(storage.as_ref(), &document, floor, consolidate_snapshots, budget, &cancelled, progress.as_ref(), &mut ledger).await;
        progress.store(DatabaseCompactionProgress::LeaseRelease as u8, std::sync::atomic::Ordering::Release);
        let release = match lease_recovery.release_future() {
            Some(release) => release.await,
            None if lease_recovery.released.load(std::sync::atomic::Ordering::Acquire) => Ok(()),
            None => Err(DbError::Internal("database compaction lease release claim lost".to_string())),
        };
        match (run, release) {
            (Ok(report), Ok(())) => Ok(report),
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
        }
    }
    .await;
    if matches!(result, Err(DbError::Closed)) && expired.load(std::sync::atomic::Ordering::Acquire) {
        result = Err(DbError::Timeout("database compaction deadline".to_string()));
    }
    progress.store(
        match result {
            Ok(_) => DatabaseCompactionProgress::Completed,
            Err(DbError::Closed) => DatabaseCompactionProgress::Cancelled,
            Err(_) => DatabaseCompactionProgress::Fault,
        } as u8,
        std::sync::atomic::Ordering::Release,
    );
    DatabaseCompactionExecution { storage, document, holder, result }
}

type DatabaseCompactionExecutionFuture = std::pin::Pin<Box<dyn Future<Output = DatabaseCompactionExecution> + Send + 'static>>;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DatabaseCompactionDriverAuthority {
    Idle,
    Queued,
    Driving,
    Retry,
}

struct DatabaseCompactionTerminalOwners {
    storage: Option<Arc<db_storage::DbBackend>>,
    document: Option<ArtifactId>,
    holder: Option<db_storage::DbIoText>,
    result: Option<Result<CompactionReport, DbError>>,
}

impl DatabaseCompactionTerminalOwners {
    fn from_execution(execution: DatabaseCompactionExecution) -> Self {
        Self { storage: Some(execution.storage), document: Some(execution.document), holder: Some(execution.holder), result: Some(execution.result) }
    }

    fn close_one(&mut self) -> bool {
        if let Some(Ok(report)) = self.result.as_mut() {
            if report.index_reports.close_step() {
                return true;
            }
        }
        if self.result.take().is_some() {
            return true;
        }
        if self.holder.as_mut().is_some_and(db_storage::DbIoText::close_step) {
            return true;
        }
        if self.holder.take().is_some() {
            return true;
        }
        self.document.take().is_some() || self.storage.take().is_some()
    }

    fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.document.is_none() && self.holder.is_none() && self.result.is_none()
    }
}

struct DatabaseCompactionCore {
    future: Option<DatabaseCompactionExecutionFuture>,
    output: Option<DatabaseCompactionExecution>,
    release_waiting: Option<DatabaseCompactionExecution>,
    release_fault: Option<DbError>,
    release_retry_fault: Option<DbError>,
    quarantined: Option<DatabaseCompactionExecutionFuture>,
    release_quarantined: Option<DatabaseCompactionLeaseReleaseFuture>,
    panic_release: Option<DatabaseCompactionLeaseReleaseFuture>,
}

struct DatabaseCompactionState {
    pool: Arc<WorkerPool>,
    slot: usize,
    generation: u64,
    admission: std::sync::Mutex<Option<DatabaseCompactionAdmission>>,
    core: std::sync::Mutex<DatabaseCompactionCore>,
    terminal: std::sync::Mutex<Option<DatabaseCompactionTerminalOwners>>,
    driver: std::sync::atomic::AtomicU8,
    retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    terminal_job: std::sync::Mutex<Option<semio_framework_async::Job>>,
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    expired: Arc<std::sync::atomic::AtomicBool>,
    deadline_ms: std::sync::atomic::AtomicU64,
    progress: Arc<std::sync::atomic::AtomicU8>,
    abandoned: std::sync::atomic::AtomicBool,
    wake_requested: std::sync::atomic::AtomicBool,
    callback_close: std::sync::atomic::AtomicBool,
    callback_armed: std::sync::atomic::AtomicBool,
    release_retry_armed: std::sync::atomic::AtomicBool,
    panic_fault: std::sync::atomic::AtomicBool,
    panic_retired: std::sync::atomic::AtomicBool,
    lease_recovery: Arc<DatabaseCompactionLeaseRecovery>,
    waker: std::sync::Mutex<Option<std::task::Waker>>,
}

fn database_compaction_registry() -> &'static std::sync::Mutex<[Option<Arc<DatabaseCompactionState>>; DATABASE_COMPACTION_SLOTS]> {
    static REGISTRY: std::sync::OnceLock<std::sync::Mutex<[Option<Arc<DatabaseCompactionState>>; DATABASE_COMPACTION_SLOTS]>> = std::sync::OnceLock::new();
    REGISTRY.get_or_init(|| std::sync::Mutex::new(std::array::from_fn(|_| None)))
}

impl std::task::Wake for DatabaseCompactionState {
    fn wake(self: Arc<Self>) {
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }
}

impl DatabaseCompactionState {
    fn current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCompactionAdmission::is_current)
    }

    fn observed_generation(&self) -> u64 {
        DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.get(self.slot).map_or(0, |slot| slot.generation)
    }

    fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        let execution_terminal = {
            let core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            core.future.is_none() && core.panic_release.is_none() && core.release_waiting.is_none() && core.release_fault.is_none() && core.release_retry_fault.is_none() && !self.release_retry_armed.load(Ordering::Acquire)
        };
        if self.callback_close.load(Ordering::Acquire) && execution_terminal {
            self.arm_callback_close();
            return;
        }
        if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Idle as u8, DatabaseCompactionDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.submit_exact(Box::new(move || state.drive_one()), 0);
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                let next = attempt.checked_add(1).map_or(DATABASE_COMPACTION_RETRY_LIMIT, |value| value.min(DATABASE_COMPACTION_RETRY_LIMIT));
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Queued as u8, DatabaseCompactionDriverAuthority::Retry as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    let state = self.clone();
                    self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
                }
            }
        }
    }

    fn retry(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        if self.pool.now_ms() >= self.deadline_ms.load(Ordering::Acquire) {
            self.expired.store(true, Ordering::Release);
            self.cancelled.store(true, Ordering::Release);
        }
        if attempt >= DATABASE_COMPACTION_RETRY_LIMIT {
            self.cancelled.store(true, Ordering::Release);
        }
        if self.cancelled.load(Ordering::Acquire) && self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Retry as u8, DatabaseCompactionDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.submit_exact(job, attempt);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            return;
        }
        if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Retry as u8, DatabaseCompactionDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.submit_exact(job, attempt);
        } else {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
        }
    }

    fn arm_release_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.release_retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.release_retry_callback());
    }

    fn release_retry_callback(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.release_retry_armed.store(false, Ordering::Release);
        if self.lease_recovery.released.load(Ordering::Acquire) {
            let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(error) = core.release_retry_fault.take().or_else(|| core.release_fault.take()) {
                drop(core);
                drop(error);
                self.arm_release_retry();
                return;
            }
            if let Some(output) = core.release_waiting.take() {
                core.output = Some(output);
                drop(core);
                if self.abandoned.load(Ordering::Acquire) || self.callback_close.load(Ordering::Acquire) {
                    self.move_output_to_terminal();
                    self.arm_callback_close();
                } else if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    waker.wake();
                }
            } else {
                drop(core);
                self.callback_close.store(true, Ordering::Release);
                self.arm_callback_close();
            }
            return;
        }
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(error) = core.release_retry_fault.take() {
            drop(error);
        }
        if core.panic_release.is_none() {
            core.panic_release = self.lease_recovery.release_future();
        }
        let mounted = core.panic_release.is_some();
        drop(core);
        if mounted {
            self.schedule();
        } else {
            self.arm_release_retry();
        }
    }

    fn drive_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Queued as u8, DatabaseCompactionDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let pending = self.poll_one();
        let wake = self.wake_requested.swap(false, Ordering::AcqRel);
        self.driver.store(DatabaseCompactionDriverAuthority::Idle as u8, Ordering::Release);
        if pending || wake {
            self.schedule();
        }
    }

    fn poll_one(self: &Arc<Self>) -> bool {
        if !self.current() {
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.progress.store(DatabaseCompactionProgress::Fault as u8, std::sync::atomic::Ordering::Release);
        }
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.future.is_none() {
            let Some(mut release) = core.panic_release.take() else { return false };
            drop(core);
            let waker = std::task::Waker::from(self.clone());
            let mut context = std::task::Context::from_waker(&waker);
            return match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| release.as_mut().poll(&mut context))) {
                Ok(std::task::Poll::Pending) => {
                    self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).panic_release = Some(release);
                    true
                }
                Ok(std::task::Poll::Ready(Ok(()))) => {
                    self.arm_release_retry();
                    false
                }
                Ok(std::task::Poll::Ready(Err(error))) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    if core.release_fault.is_none() {
                        core.release_fault = Some(error);
                    } else {
                        core.release_retry_fault = Some(error);
                    }
                    drop(core);
                    self.arm_release_retry();
                    false
                }
                Err(_) => {
                    let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    core.release_quarantined = Some(release);
                    core.panic_release = self.lease_recovery.retry_release_after_panic();
                    if core.panic_release.is_none() {
                        self.callback_close.store(true, std::sync::atomic::Ordering::Release);
                    }
                    true
                }
            };
        }
        drop(core);
        let mut future = {
            let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            match core.future.take() {
                Some(future) => future,
                None => return false,
            }
        };
        let waker = std::task::Waker::from(self.clone());
        let mut context = std::task::Context::from_waker(&waker);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
        let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match polled {
            Ok(std::task::Poll::Pending) => {
                core.future = Some(future);
                true
            }
            Ok(std::task::Poll::Ready(output)) => {
                if !self.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire) {
                    core.release_waiting = Some(output);
                    drop(core);
                    self.arm_release_retry();
                    return false;
                }
                core.output = Some(output);
                drop(core);
                if self.abandoned.load(std::sync::atomic::Ordering::Acquire) || self.callback_close.load(std::sync::atomic::Ordering::Acquire) {
                    self.move_output_to_terminal();
                    self.arm_callback_close();
                } else if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    waker.wake();
                }
                false
            }
            Err(_) => {
                core.quarantined = Some(future);
                core.panic_release = if self.lease_recovery.releasing.load(std::sync::atomic::Ordering::Acquire) && !self.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire) {
                    self.lease_recovery.retry_release_after_panic()
                } else {
                    self.lease_recovery.release_future()
                };
                let release_pending = core.panic_release.is_some();
                self.panic_fault.store(true, std::sync::atomic::Ordering::Release);
                self.progress.store(DatabaseCompactionProgress::Fault as u8, std::sync::atomic::Ordering::Release);
                if !release_pending {
                    self.callback_close.store(true, std::sync::atomic::Ordering::Release);
                }
                true
            }
        }
    }

    fn move_output_to_terminal(&self) {
        let output = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).output.take();
        if let Some(output) = output {
            *self.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCompactionTerminalOwners::from_execution(output));
        }
    }

    fn drive_close_claimed(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        let pending = if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            true
        } else {
            self.move_output_to_terminal();
            self.close_terminal_one()
        };
        self.driver.store(DatabaseCompactionDriverAuthority::Idle as u8, Ordering::Release);
        if pending || !self.terminal_is_empty() {
            self.arm_callback_close();
        } else {
            self.callback_close.store(false, Ordering::Release);
        }
    }

    fn close_terminal_one(&self) -> bool {
        if self.panic_fault.load(std::sync::atomic::Ordering::Acquire) && self.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire) {
            let mut core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(quarantine) = core.release_quarantined.take() {
                drop(quarantine);
                return true;
            }
            if let Some(quarantine) = core.quarantined.take() {
                drop(quarantine);
                return true;
            }
        }
        let mut terminal = self.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.as_mut().is_some_and(DatabaseCompactionTerminalOwners::close_one) {
            return true;
        }
        let terminal_released = terminal.as_ref().is_some_and(DatabaseCompactionTerminalOwners::terminal_is_empty);
        if terminal_released {
            terminal.take();
        }
        drop(terminal);
        if terminal_released {
            self.release_terminal();
        }
        if self.panic_fault.load(std::sync::atomic::Ordering::Acquire) && self.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire) {
            let core_empty = {
                let core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                core.future.is_none()
                    && core.output.is_none()
                    && core.release_waiting.is_none()
                    && core.release_fault.is_none()
                    && core.release_retry_fault.is_none()
                    && core.quarantined.is_none()
                    && core.release_quarantined.is_none()
                    && core.panic_release.is_none()
            };
            if core_empty {
                self.release_terminal();
                self.panic_retired.store(true, std::sync::atomic::Ordering::Release);
                if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    waker.wake();
                }
            }
        }
        false
    }

    fn arm_callback_close(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.callback_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.callback_close_one());
    }

    fn callback_close_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.callback_armed.store(false, Ordering::Release);
        if self.driver.compare_exchange(DatabaseCompactionDriverAuthority::Idle as u8, DatabaseCompactionDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            self.arm_callback_close();
            return;
        }
        self.drive_close_claimed();
    }

    fn release_terminal(&self) {
        let mut registry = database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
            registry[self.slot] = None;
        }
        drop(registry);
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    }

    fn terminal_is_empty(&self) -> bool {
        let core = self.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let terminal_empty = self.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_none_or(DatabaseCompactionTerminalOwners::terminal_is_empty);
        core.future.is_none()
            && core.output.is_none()
            && core.release_waiting.is_none()
            && core.release_fault.is_none()
            && core.release_retry_fault.is_none()
            && core.quarantined.is_none()
            && core.release_quarantined.is_none()
            && core.panic_release.is_none()
            && !self.release_retry_armed.load(std::sync::atomic::Ordering::Acquire)
            && terminal_empty
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }
}

/// 🧹 Retained completion carrying exact storage, document, holder and report owners.
pub struct DatabaseCompactionResult {
    state: Option<Arc<DatabaseCompactionState>>,
    execution: Option<DatabaseCompactionExecution>,
}

impl std::fmt::Debug for DatabaseCompactionResult {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCompactionResult").field("generation", &self.state.as_ref().map(|state| state.generation)).field("execution", &self.execution.is_some()).finish()
    }
}

impl DatabaseCompactionResult {
    pub fn into_parts(mut self) -> Result<(Arc<db_storage::DbBackend>, ArtifactId, db_storage::DbIoText, Result<CompactionReport, DbError>), Self> {
        let execution = self.execution.take();
        match execution {
            Some(execution) => {
                if let Some(state) = self.state.take() {
                    state.release_terminal();
                }
                Ok((execution.storage, execution.document, execution.holder, execution.result))
            }
            None => Err(self),
        }
    }

    pub fn close_and_take_report(mut self) -> Result<CompactionReport, DbError> {
        let Some(execution) = self.execution.take() else { return Err(DbError::Internal("database compaction result owner missing".to_string())) };
        let DatabaseCompactionExecution { storage, document, holder, result } = execution;
        if let Some(state) = self.state.take() {
            *state.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCompactionTerminalOwners { storage: Some(storage), document: Some(document), holder: Some(holder), result: None });
            state.callback_close.store(true, std::sync::atomic::Ordering::Release);
            state.arm_callback_close();
        }
        result
    }
}

impl Drop for DatabaseCompactionResult {
    fn drop(&mut self) {
        let Some(state) = self.state.take() else { return };
        if let Some(execution) = self.execution.take() {
            *state.terminal.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCompactionTerminalOwners::from_execution(execution));
        }
        state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        state.callback_close.store(true, std::sync::atomic::Ordering::Release);
        state.arm_callback_close();
    }
}

/// 🚦 Future facade for the generation-qualified compaction registry authority.
pub struct DatabaseCompactionFuture {
    state: Option<Arc<DatabaseCompactionState>>,
    completed: bool,
}

impl std::fmt::Debug for DatabaseCompactionFuture {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCompactionFuture").field("generation", &self.generation()).field("completed", &self.completed).finish()
    }
}

impl DatabaseCompactionFuture {
    pub fn try_submit(pool: Arc<WorkerPool>, storage: Arc<db_storage::DbBackend>, document: ArtifactId, holder: db_storage::DbIoText, consolidate_snapshots: bool, budget: CompactionBudget, now_ms: u64) -> Result<Self, DatabaseCompactionRejected> {
        let admission = match DatabaseCompactionAdmission::try_claim(&document) {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseCompactionRejected::new(pool, error, storage, document, holder)),
        };
        let slot = admission.slot;
        let generation = admission.generation;
        let resource = match compaction_resource(&document) {
            Ok(resource) => resource,
            Err(error) => return Err(DatabaseCompactionRejected::new(pool, error, storage, document, holder)),
        };
        let lease_recovery = DatabaseCompactionLeaseRecovery::new(storage.clone(), resource, holder.clone());
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let expired = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let progress = Arc::new(std::sync::atomic::AtomicU8::new(DatabaseCompactionProgress::Admitted as u8));
        let future = Box::pin(retained_compaction_execute(storage, document, holder, consolidate_snapshots, budget, now_ms, cancelled.clone(), expired.clone(), progress.clone(), lease_recovery.clone()));
        let deadline_ms = pool.now_ms().saturating_add(DATABASE_COMPACTION_DEADLINE_MS);
        let state = Arc::new(DatabaseCompactionState {
            pool: pool.clone(),
            slot,
            generation,
            admission: std::sync::Mutex::new(Some(admission)),
            core: std::sync::Mutex::new(DatabaseCompactionCore { future: Some(future), output: None, release_waiting: None, release_fault: None, release_retry_fault: None, quarantined: None, release_quarantined: None, panic_release: None }),
            terminal: std::sync::Mutex::new(None),
            driver: std::sync::atomic::AtomicU8::new(DatabaseCompactionDriverAuthority::Idle as u8),
            retry_job: std::sync::Mutex::new(None),
            terminal_job: std::sync::Mutex::new(None),
            cancelled,
            expired,
            deadline_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            progress,
            abandoned: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            callback_close: std::sync::atomic::AtomicBool::new(false),
            callback_armed: std::sync::atomic::AtomicBool::new(false),
            release_retry_armed: std::sync::atomic::AtomicBool::new(false),
            panic_fault: std::sync::atomic::AtomicBool::new(false),
            panic_retired: std::sync::atomic::AtomicBool::new(false),
            lease_recovery,
            waker: std::sync::Mutex::new(None),
        });
        database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        let deadline = state.clone();
        pool.callback_at(deadline_ms, move || deadline.deadline_callback());
        state.schedule();
        Ok(Self { state: Some(state), completed: false })
    }

    pub fn generation(&self) -> u64 {
        self.state.as_ref().map_or(0, |state| state.generation)
    }

    pub fn progress(&self) -> DatabaseCompactionProgress {
        self.state.as_ref().map_or(DatabaseCompactionProgress::Fault, |state| state.progress())
    }

    pub fn cancel(&self) {
        if let Some(state) = self.state.as_ref() {
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        }
    }
}

impl DatabaseCompactionState {
    fn progress(&self) -> DatabaseCompactionProgress {
        DatabaseCompactionProgress::from_u8(self.progress.load(std::sync::atomic::Ordering::Acquire))
    }

    fn deadline_callback(self: &Arc<Self>) {
        if self.current() && !matches!(self.progress(), DatabaseCompactionProgress::Completed | DatabaseCompactionProgress::Cancelled | DatabaseCompactionProgress::Fault) {
            self.expired.store(true, std::sync::atomic::Ordering::Release);
            self.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.schedule();
        }
    }
}

impl Future for DatabaseCompactionFuture {
    type Output = Result<DatabaseCompactionResult, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed {
            return std::task::Poll::Ready(Err(DbError::Closed));
        }
        let Some(state) = self.state.as_ref().cloned() else { return std::task::Poll::Ready(Err(DbError::Closed)) };
        if state.panic_retired.load(std::sync::atomic::Ordering::Acquire) {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Err(DbError::Internal("database compaction worker panic released lease and retired quarantine".to_string())));
        }
        if !state.current() {
            self.completed = true;
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.callback_close.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
            return std::task::Poll::Ready(Err(DbError::StaleGeneration { expected: crate::db_ids::GenerationId(state.generation), actual: crate::db_ids::GenerationId(state.observed_generation()) }));
        }
        let execution = { state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).output.take() };
        if let Some(execution) = execution {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Ok(DatabaseCompactionResult { state: Some(state), execution: Some(execution) }));
        }
        *state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let execution = { state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).output.take() };
        if let Some(execution) = execution {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Ok(DatabaseCompactionResult { state: Some(state), execution: Some(execution) }));
        }
        state.schedule();
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseCompactionFuture {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(state) = self.state.take() {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.callback_close.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        }
    }
}

struct DatabaseCompactionRejectedClose {
    pool: Arc<WorkerPool>,
    owners: std::sync::Mutex<Option<DatabaseCompactionTerminalOwners>>,
    queued: std::sync::atomic::AtomicBool,
    retries: std::sync::atomic::AtomicU8,
}

impl DatabaseCompactionRejectedClose {
    fn schedule(self: &Arc<Self>) {
        if self.queued.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let close = self.clone();
        match self.pool.try_submit(Lane::Io, Box::new(move || close.drive_one())) {
            Ok(()) => {}
            Err(error) => {
                drop(error.into_job());
                self.queued.store(false, std::sync::atomic::Ordering::Release);
                let attempt = self.retries.fetch_add(1, std::sync::atomic::Ordering::AcqRel).saturating_add(1);
                let close = self.clone();
                self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
                    if attempt >= DATABASE_COMPACTION_RETRY_LIMIT {
                        close.drive_one();
                    } else {
                        close.schedule();
                    }
                });
            }
        }
    }

    fn drive_one(self: Arc<Self>) {
        self.queued.store(false, std::sync::atomic::Ordering::Release);
        let mut owners = self.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let pending = owners.as_mut().is_some_and(DatabaseCompactionTerminalOwners::close_one);
        if owners.as_ref().is_some_and(DatabaseCompactionTerminalOwners::terminal_is_empty) {
            owners.take();
        }
        drop(owners);
        if pending {
            self.schedule();
        }
    }
}

/// ⛔ Lossless pre-admission refusal carrying exact compaction input identities.
pub struct DatabaseCompactionRejected {
    error: Option<DbError>,
    close: Arc<DatabaseCompactionRejectedClose>,
}

impl std::fmt::Debug for DatabaseCompactionRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseCompactionRejected").field("error", &self.error).finish()
    }
}

impl DatabaseCompactionRejected {
    fn new(pool: Arc<WorkerPool>, error: DbError, storage: Arc<db_storage::DbBackend>, document: ArtifactId, holder: db_storage::DbIoText) -> Self {
        Self {
            error: Some(error),
            close: Arc::new(DatabaseCompactionRejectedClose {
                pool,
                owners: std::sync::Mutex::new(Some(DatabaseCompactionTerminalOwners { storage: Some(storage), document: Some(document), holder: Some(holder), result: None })),
                queued: std::sync::atomic::AtomicBool::new(false),
                retries: std::sync::atomic::AtomicU8::new(0),
            }),
        }
    }

    pub fn into_parts(mut self) -> Result<(DbError, Arc<db_storage::DbBackend>, ArtifactId, db_storage::DbIoText), Self> {
        let error = self.error.take();
        let mut owners = self.close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let storage = owners.as_mut().and_then(|owners| owners.storage.take());
        let document = owners.as_mut().and_then(|owners| owners.document.take());
        let holder = owners.as_mut().and_then(|owners| owners.holder.take());
        drop(owners);
        match (error, storage, document, holder) {
            (Some(error), Some(storage), Some(document), Some(holder)) => Ok((error, storage, document, holder)),
            (error, storage, document, holder) => {
                self.error = error;
                *self.close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCompactionTerminalOwners { storage, document, holder, result: None });
                Err(self)
            }
        }
    }

    pub fn close_and_take_error(mut self) -> DbError {
        let error = self.error.take().unwrap_or(DbError::LimitExceeded("database compaction refusal error"));
        self.close.schedule();
        error
    }
}

impl Drop for DatabaseCompactionRejected {
    fn drop(&mut self) {
        if self.close.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            self.close.schedule();
        }
    }
}
//#endregion 🧵️RetainedCompactionJob
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

    fn committed_compaction_fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️fixtures/🧾️committed-effects/🔣️.json")).unwrap()
    }

    async fn append_fixture_record(writer: &mut protocol::SprWriter<Vec<u8>>, mut record: WalRecord) {
        let (kind, critical, payload) = record.encode().await;
        let mut frame = writer.begin_identity_record(kind, critical, payload.len()).await.unwrap();
        frame.write_fragment(&payload).await.unwrap();
        frame.finish().await.unwrap();
        while record.close_step().unwrap() {}
    }

    async fn append_committed_compaction_segment(
        storage: &MemoryStorage,
        document: &ArtifactId,
        row: &serde_json::Value,
        previous: Option<[u8; 32]>,
        aborted: pack::ContentHash,
        committed: pack::ContentHash,
    ) -> [u8; 32] {
        let index = row["index"].as_u64().unwrap();
        let options = protocol::format::WriteOptions { required_flags: protocol::wire::REQUIRED_HASH_CHAIN, optional_flags: 0 };
        let mut writer = protocol::SprWriter::begin(Vec::new(), &options).await.unwrap();
        append_fixture_record(&mut writer, WalRecord::SegmentHeader { document: document.clone(), segment_index: index, prev_chain_hash: previous }).await;
        writer.commit().await.unwrap();
        for transaction in row["transactions"].as_array().unwrap() {
            let tx_id = transaction["id"].as_u64().unwrap();
            append_fixture_record(&mut writer, WalRecord::TxBegin { tx_id }).await;
            for record in transaction["records"].as_array().unwrap() {
                let record = match record["kind"].as_str().unwrap() {
                    "frontier" => WalRecord::Frontier(frontier(document, record["headSeq"].as_u64().unwrap()).await),
                    "snapshot" => WalRecord::SnapshotPub { generation: 1, frontier: frontier(document, record["headSeq"].as_u64().unwrap()).await },
                    "payload" => WalRecord::Payload(WalPayloadRef::CasRef(match record["payload"].as_str().unwrap() {
                        "aborted" => aborted,
                        "committed" => committed,
                        other => panic!("unknown committed compaction payload {other}"),
                    })),
                    other => panic!("unknown committed compaction record {other}"),
                };
                append_fixture_record(&mut writer, record).await;
            }
            let record_count = transaction["records"].as_array().unwrap().len() as u32;
            let terminal = match transaction["outcome"].as_str().unwrap() {
                "commit" => WalRecord::TxCommit { tx_id, record_count },
                "abort" => WalRecord::TxAbort { tx_id },
                other => panic!("unknown committed compaction outcome {other}"),
            };
            append_fixture_record(&mut writer, terminal).await;
            writer.commit().await.unwrap();
        }
        let bytes = writer.into_sink().await;
        let mut verification = protocol::format::retained::RetainedSprVerification::new(bytes.len() as u64, protocol::format::retained::RetainedSprLimits::default()).unwrap();
        let mut fuel = bytes.len();
        assert_eq!(verification.push(&bytes, &mut fuel).unwrap(), bytes.len());
        let span = verification.finish().unwrap();
        assert_eq!(span.tail(), 0);
        let chain = *span.chain();
        storage.create_segment(document, index).await.unwrap();
        assert_eq!(storage.append(document, index, pages(&bytes)).await.unwrap(), bytes.len() as u64);
        if row["state"] == "sealed" { storage.seal(document, index).await.unwrap(); }
        chain
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

    fn held_compaction_worker_pool() -> (Arc<WorkerPool>, Arc<std::sync::atomic::AtomicBool>) {
        let pool = Arc::new(WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1)));
        let entered = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let held = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let worker_entered = entered.clone();
        let worker_held = held.clone();
        pool.try_submit(
            Lane::Maintenance,
            Box::new(move || {
                worker_entered.store(true, std::sync::atomic::Ordering::Release);
                while worker_held.load(std::sync::atomic::Ordering::Acquire) {
                    std::thread::yield_now();
                }
            }),
        )
        .ok()
        .expect("compaction blocker admission");
        while !entered.load(std::sync::atomic::Ordering::Acquire) {
            std::thread::yield_now();
        }
        (pool, held)
    }

    async fn retained_compaction_storage() -> Arc<db_storage::DbBackend> {
        Arc::new(db_storage::DbBackend::Memory(MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap()))
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_handoff_to_first_poll_cancel_uses_real_io_lane_and_releases_exact_owners_under_eight_ms() {
        let (pool, held) = held_compaction_worker_pool();
        let storage = retained_compaction_storage().await;
        let storage_identity = Arc::as_ptr(&storage) as usize;
        let document = ArtifactId(String::from("p1y-handoff-cancel"));
        let document_identity = document.0.as_ptr();
        let holder = db_storage::DbIoText::try_from_str("p1y-holder").unwrap();
        let started = std::time::Instant::now();
        let future = DatabaseCompactionFuture::try_submit(pool.clone(), storage, document, holder, false, CompactionBudget::default(), 0).unwrap();
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
        let generation = future.generation();
        let state = future.state.as_ref().unwrap().clone();
        assert_eq!(state.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseCompactionDriverAuthority::Queued as u8);
        future.cancel();
        held.store(false, std::sync::atomic::Ordering::Release);
        let result = future.await.unwrap();
        let (storage, document, mut holder, report) = result.into_parts().unwrap();
        assert_eq!(Arc::as_ptr(&storage) as usize, storage_identity);
        assert_eq!(document.0.as_ptr(), document_identity);
        assert_eq!(holder.as_str(), "p1y-holder");
        assert_eq!(report, Err(DbError::Closed));
        assert!(holder.close_step());
        assert_eq!(state.progress(), DatabaseCompactionProgress::Cancelled);
        assert!(!database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().flatten().any(|owner| owner.generation == generation));
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_actual_deadline_callback_lost_wake_and_drop_close_release_lease_once() {
        let (pool, held) = held_compaction_worker_pool();
        let storage = retained_compaction_storage().await;
        let storage_identity = Arc::as_ptr(&storage) as usize;
        let future = DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(String::from("p1y-deadline")), db_storage::DbIoText::try_from_str("deadline-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
        let state = future.state.as_ref().unwrap().clone();
        state.deadline_ms.store(0, std::sync::atomic::Ordering::Release);
        state.deadline_callback();
        std::task::Wake::wake_by_ref(&state);
        held.store(false, std::sync::atomic::Ordering::Release);
        let result = future.await.unwrap();
        let (storage, document, mut holder, report) = result.into_parts().unwrap();
        assert_eq!(Arc::as_ptr(&storage) as usize, storage_identity);
        assert_eq!(document.0, "p1y-deadline");
        assert_eq!(report, Err(DbError::Timeout("database compaction deadline".to_string())));
        assert!(holder.close_step());
        assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
        assert_eq!(state.driver.load(std::sync::atomic::Ordering::Acquire), DatabaseCompactionDriverAuthority::Idle as u8);
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_max_plus_one_capacity_refusal_preserves_storage_document_holder_and_hash_authority() {
        let (pool, held) = held_compaction_worker_pool();
        let mut admitted = Vec::with_capacity(DATABASE_COMPACTION_SLOTS);
        for index in 0..DATABASE_COMPACTION_SLOTS {
            admitted.push(
                DatabaseCompactionFuture::try_submit(pool.clone(), retained_compaction_storage().await, ArtifactId(format!("p1y-max-{index}")), db_storage::DbIoText::try_from_str("max-holder").unwrap(), false, CompactionBudget::default(), 0)
                    .unwrap(),
            );
        }
        let slot_storage = retained_compaction_storage().await;
        let slot_storage_identity = Arc::as_ptr(&slot_storage) as usize;
        let slot_document = ArtifactId(String::from("p1y-slot-max-plus-one"));
        let slot_document_identity = slot_document.0.as_ptr();
        let slot_rejected = DatabaseCompactionFuture::try_submit(pool.clone(), slot_storage, slot_document, db_storage::DbIoText::try_from_str("slot-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap_err();
        let (slot_error, slot_storage, slot_document, mut slot_holder) = slot_rejected.into_parts().unwrap();
        assert_eq!(slot_error, DbError::LimitExceeded("database compaction admission slots"));
        assert_eq!(Arc::as_ptr(&slot_storage) as usize, slot_storage_identity);
        assert_eq!(slot_document.0.as_ptr(), slot_document_identity);
        assert!(slot_holder.close_step());
        let storage = retained_compaction_storage().await;
        let storage_identity = Arc::as_ptr(&storage) as usize;
        let mut external = String::with_capacity(db_storage::DbIoText::maximum_capacity() + 1);
        external.push_str("p1y-max-plus-one");
        let document_identity = external.as_ptr();
        let rejected = DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(external), db_storage::DbIoText::try_from_str("exact-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap_err();
        let (error, storage, document, mut holder) = rejected.into_parts().unwrap();
        assert_eq!(error, DbError::LimitExceeded("database compaction document backing"));
        assert_eq!(Arc::as_ptr(&storage) as usize, storage_identity);
        assert_eq!(document.0.as_ptr(), document_identity);
        assert_eq!(holder.as_str(), "exact-holder");
        assert!(holder.close_step());
        let cancelled = std::sync::atomic::AtomicBool::new(false);
        let mut hashes = DatabaseCompactionHashOwners { slots: [Some(pack::ContentHash([7; 32])); DATABASE_COMPACTION_MAX_HASHES], len: DATABASE_COMPACTION_MAX_HASHES as u16 };
        assert_eq!(hashes.insert(pack::ContentHash([9; 32]), &cancelled).await, Err(DbError::LimitExceeded("database compaction payload hash owners")));
        let descriptor = db_snapshot::SnapshotDescriptor {
            document: ArtifactId(String::from("p1y-observed-backing")),
            generation: 1,
            parent_generation: None,
            head_seq: 1,
            commit_seq: 1,
            epoch: 1,
            chain_hash: [0; 32],
            protocol_version: 1,
            vcs_head: None,
            base_pack_hash: None,
            roots: Vec::with_capacity(DATABASE_COMPACTION_OPERATION_ITEMS as usize),
            new_pages: Vec::new(),
            created_at_ms: 1,
        };
        let mut descriptor_ledger = DatabaseCompactionBackingLedger::default();
        assert_eq!(database_compaction_admit_descriptor(descriptor, &mut descriptor_ledger).await.unwrap_err(), DbError::LimitExceeded("database compaction snapshot backing"));
        for future in &admitted {
            future.cancel();
        }
        drop(admitted);
        held.store(false, std::sync::atomic::Ordering::Release);
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_stale_aba_drop_and_partial_terminal_close_keep_one_generation_owner_per_opportunity() {
        let (pool, held) = held_compaction_worker_pool();
        let future =
            DatabaseCompactionFuture::try_submit(pool.clone(), retained_compaction_storage().await, ArtifactId(String::from("p1y-stale")), db_storage::DbIoText::try_from_str("stale-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
        let state = future.state.as_ref().unwrap().clone();
        let replacement = state.generation.checked_add(1).unwrap();
        DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[state.slot].generation = replacement;
        assert_eq!(future.await.unwrap_err(), DbError::StaleGeneration { expected: crate::db_ids::GenerationId(state.generation), actual: crate::db_ids::GenerationId(replacement) });
        assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_some());
        DATABASE_COMPACTION_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots[state.slot].generation = state.generation;
        held.store(false, std::sync::atomic::Ordering::Release);
        while state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            std::thread::yield_now();
        }
        let mut reports = CompactionIndexReports::default();
        for kind in db_index::IndexKind::ALL {
            reports.push(IndexKindReport { kind, stats: db_index::IndexStats { run_count: 0, entry_count: 0, total_bytes: 0 } }).unwrap();
        }
        let mut owners = DatabaseCompactionTerminalOwners {
            storage: Some(retained_compaction_storage().await),
            document: Some(ArtifactId(String::from("p1y-close"))),
            holder: Some(db_storage::DbIoText::try_from_str("close-holder").unwrap()),
            result: Some(Ok(CompactionReport { index_reports: reports, ..CompactionReport::default() })),
        };
        let before = owners.result.as_ref().and_then(|result| result.as_ref().ok()).map(|report| report.index_reports.len()).unwrap();
        assert!(owners.close_one());
        let after = owners.result.as_ref().and_then(|result| result.as_ref().ok()).map(|report| report.index_reports.len()).unwrap();
        assert_eq!(before - after, 1);
        while owners.close_one() {}
        assert!(owners.terminal_is_empty());
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_index_child_uses_exact_parent_cancel_and_eight_ms_control() {
        let storage = retained_compaction_storage().await;
        let index_storage = storage.index().await;
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let handle = db_index::IndexHandle::new(&index_storage, ArtifactId(String::from("p1y-index-cancel")), db_index::IndexKind::Command).await;
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_COMPACTION_TURN_MS);
        let mut control = handle.retained_operation_control(cancelled.clone(), deadline, DATABASE_COMPACTION_INDEX_FUEL).unwrap();
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        assert_eq!(handle.compact(&mut control).await, Err(DbError::Unavailable("index cursor cancelled".to_string())));
        assert_eq!(DATABASE_COMPACTION_TURN_MS, 8);
        assert_ne!(DATABASE_COMPACTION_INDEX_FUEL, 65_536);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_expected_snapshot_publication_never_persists_stale_baseline() {
        let storage = retained_compaction_storage().await;
        let snapshot = storage.snapshot().await;
        let manager = db_snapshot::SnapshotManager::new(&snapshot).await;
        let document = ArtifactId(String::from("p1y-publication-cas"));
        assert_eq!(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(1).await).await.unwrap(), 0);
        let expected = snapshot.latest_generation(&document).await.unwrap().unwrap();
        assert_eq!(manager.publish(&document, db_snapshot::SnapshotOrigin::FullBaseline, &[], sample_body(2).await).await.unwrap(), 1);
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let mut control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_millis(8), DATABASE_COMPACTION_INDEX_FUEL).unwrap();
        let rejected = manager.publish_retained_expected(&document, expected, &[], 0, sample_body(1).await, &mut control).await.unwrap_err();
        let (error, body) = rejected.into_parts();
        assert_eq!(error, DbError::StaleGeneration { expected: crate::db_ids::GenerationId(0), actual: crate::db_ids::GenerationId(1) });
        retire_compaction_snapshot_body(body).await;
        let mut generations = snapshot.list_generations(&document).await.unwrap();
        assert_eq!(generations.as_slice(), &[0, 1]);
        close_compaction_owner(|| Ok(generations.close_step())).await.unwrap();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_panic_after_lease_acquire_releases_once_before_public_fault_and_registry_drain() {
        let (pool, held) = held_compaction_worker_pool();
        let storage = retained_compaction_storage().await;
        let lease_storage = storage.clone();
        let future = DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(String::from("p1y-panic-release")), db_storage::DbIoText::try_from_str("panic-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
        let state = future.state.as_ref().unwrap().clone();
        let fence = lease_storage.lease().await.acquire(state.lease_recovery.resource.as_str(), state.lease_recovery.holder.as_str(), DEFAULT_LEASE_TTL_MS, 0).await.unwrap();
        state.lease_recovery.install(fence);
        let injected: DatabaseCompactionExecutionFuture = Box::pin(async { panic!("p1y injected post-lease panic") });
        let original = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.replace(injected).unwrap();
        drop(original);
        held.store(false, std::sync::atomic::Ordering::Release);
        assert_eq!(future.await.unwrap_err(), DbError::Internal("database compaction worker panic released lease and retired quarantine".to_string()));
        assert!(lease_storage.lease().await.current(state.lease_recovery.resource.as_str(), 0).await.unwrap().is_none());
        assert!(state.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire));
        assert!(state.panic_retired.load(std::sync::atomic::Ordering::Acquire));
        assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
        assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_none());
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_release_error_retries_through_real_worker_loop_until_success_before_public_fault() {
        let (pool, held) = held_compaction_worker_pool();
        let storage = retained_compaction_storage().await;
        let lease_storage = storage.clone();
        let future =
            DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(String::from("p1y-release-error-success")), db_storage::DbIoText::try_from_str("release-error-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
        let state = future.state.as_ref().unwrap().clone();
        let fence = lease_storage.lease().await.acquire(state.lease_recovery.resource.as_str(), state.lease_recovery.holder.as_str(), DEFAULT_LEASE_TTL_MS, 0).await.unwrap();
        state.lease_recovery.install(fence);
        state.lease_recovery.fail_release_attempts(1);
        let injected: DatabaseCompactionExecutionFuture = Box::pin(async { panic!("p1y release error then success") });
        let original = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.replace(injected).unwrap();
        drop(original);
        held.store(false, std::sync::atomic::Ordering::Release);
        assert_eq!(future.await.unwrap_err(), DbError::Internal("database compaction worker panic released lease and retired quarantine".to_string()));
        assert!(state.lease_recovery.release_attempts.load(std::sync::atomic::Ordering::Acquire) >= 2);
        assert!(state.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire));
        assert!(state.lease_recovery.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
        assert!(lease_storage.lease().await.current(state.lease_recovery.resource.as_str(), 0).await.unwrap().is_none());
        let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(core.future.is_none() && core.release_fault.is_none() && core.release_retry_fault.is_none());
        drop(core);
        assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
        assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_none());
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_perpetual_release_error_keeps_fence_fault_admission_and_registry_discoverable() {
        let (pool, held) = held_compaction_worker_pool();
        let storage = retained_compaction_storage().await;
        let lease_storage = storage.clone();
        let future =
            DatabaseCompactionFuture::try_submit(pool.clone(), storage, ArtifactId(String::from("p1y-perpetual-release-error")), db_storage::DbIoText::try_from_str("perpetual-release-holder").unwrap(), false, CompactionBudget::default(), 0).unwrap();
        let state = future.state.as_ref().unwrap().clone();
        let fence = lease_storage.lease().await.acquire(state.lease_recovery.resource.as_str(), state.lease_recovery.holder.as_str(), DEFAULT_LEASE_TTL_MS, 0).await.unwrap();
        state.lease_recovery.install(fence);
        state.lease_recovery.fail_release_attempts(usize::MAX);
        let injected: DatabaseCompactionExecutionFuture = Box::pin(async { panic!("p1y perpetual release error") });
        let original = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).future.replace(injected).unwrap();
        drop(original);
        held.store(false, std::sync::atomic::Ordering::Release);
        std::thread::sleep(std::time::Duration::from_millis(20));
        assert!(state.lease_recovery.release_attempts.load(std::sync::atomic::Ordering::Acquire) >= 2);
        assert!(!state.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire));
        assert_eq!(*state.lease_recovery.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner), Some(fence));
        let core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        assert!(core.future.is_none());
        assert!(core.release_fault.is_some());
        assert!(core.panic_release.is_some() || state.release_retry_armed.load(std::sync::atomic::Ordering::Acquire));
        drop(core);
        assert!(!state.panic_retired.load(std::sync::atomic::Ordering::Acquire));
        assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_some());
        assert!(lease_storage.lease().await.current(state.lease_recovery.resource.as_str(), 0).await.unwrap().is_some());
        state.lease_recovery.fail_release_attempts(0);
        assert_eq!(future.await.unwrap_err(), DbError::Internal("database compaction worker panic released lease and retired quarantine".to_string()));
        assert!(state.lease_recovery.released.load(std::sync::atomic::Ordering::Acquire));
        assert!(state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none());
        assert!(database_compaction_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[state.slot].is_none());
        pool.shutdown();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_compaction_cumulative_observed_backing_rejects_individually_valid_combined_max_plus_one() {
        let mut ledger = DatabaseCompactionBackingLedger::default();
        let first = Vec::<u8>::with_capacity(DATABASE_COMPACTION_OPERATION_BYTES as usize / 2);
        let second = Vec::<u8>::with_capacity(DATABASE_COMPACTION_OPERATION_BYTES as usize / 2 + 1);
        let first_identity = first.as_ptr();
        let second_identity = second.as_ptr();
        database_compaction_observe_backing(&mut ledger, 1, first.capacity(), "p1y cumulative first").unwrap();
        assert_eq!(database_compaction_observe_backing(&mut ledger, 1, second.capacity(), "p1y cumulative max plus one"), Err(DbError::LimitExceeded("p1y cumulative max plus one")));
        assert_eq!(first.as_ptr(), first_identity);
        assert_eq!(second.as_ptr(), second_identity);
        ledger.release(1, first.capacity()).unwrap();
        database_compaction_observe_backing(&mut ledger, 1, second.capacity(), "p1y cumulative recovered").unwrap();
        ledger.release(1, second.capacity()).unwrap();
        assert_eq!((ledger.items, ledger.bytes), (0, 0));
        semio_framework_async::yield_once().await;
        drop(first);
        semio_framework_async::yield_once().await;
        drop(second);
    }

    #[semio_framework_async_macros::async_test]
    async fn compaction_fixed_pages_success_refusal_cancel_stale_fault_drop_interrupted_close_and_max_plus_one_return_exact_credit() {
        while compaction_page_maintenance_step().unwrap() {}
        let mut retained = CompactionRetainedPages::new();
        for index in 0..COMPACTION_RETAINED_PAGE_OWNERS {
            assert!(retained.try_push(state_page(&[index as u8]).await).is_ok());
        }
        let rejected = retained.try_push(state_page(b"max-plus-one").await).unwrap_err();
        assert_eq!(retained.len(), COMPACTION_RETAINED_PAGE_OWNERS);
        let exit = MountedCompactionPageClose::new(&mut retained).await.unwrap();
        assert_eq!(exit, CompactionCloseExit::Closed);
        assert!(retained.terminal_is_empty());
        {
            let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in retired.iter_mut() {
                *slot = Some(CompactionRetainedPages::new());
            }
        }
        {
            let mut overflow = COMPACTION_PAGE_RETIREMENT_OVERFLOW.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in overflow.iter_mut() {
                *slot = Some(CompactionRetainedPages::new());
            }
        }
        COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.store(false, std::sync::atomic::Ordering::Release);
        let mut refusal = CompactionRetainedPages::new();
        assert!(refusal.try_push(rejected).is_ok());
        let exact_operation = refusal.slots()[0].as_ref().expect("exact refused compaction page").operation();
        let mut second_refusal = CompactionRetainedPages::new();
        assert!(second_refusal.try_push(state_page(b"max-plus-two").await).is_ok());
        let second_operation = second_refusal.slots()[0].as_ref().expect("second exact refused compaction page").operation();
        assert_eq!(refusal.retirement.map(|reservation| reservation.tier), Some(2));
        assert_eq!(second_refusal.retirement.map(|reservation| reservation.tier), Some(2));
        assert!(retire_compaction_pages(refusal).is_ok());
        assert!(retire_compaction_pages(second_refusal).is_ok());
        assert!(COMPACTION_PAGE_RETIREMENT_PRESSURE_FAULT.load(std::sync::atomic::Ordering::Acquire));
        {
            let quarantine = COMPACTION_PAGE_RETIREMENT_QUARANTINE.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert_eq!(quarantine.iter().flatten().find_map(|owner| owner.slots()[0].as_ref().map(db_state::Page::operation)), Some(exact_operation));
            assert!(quarantine.iter().flatten().any(|owner| owner.slots()[0].as_ref().map(db_state::Page::operation) == Some(second_operation)));
        }
        {
            let mut retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in retired.iter_mut() {
                *slot = None;
            }
        }
        for _ in 0..COMPACTION_RETIREMENT_SLOTS * 2 {
            assert!(compaction_page_maintenance_step().unwrap());
        }
        assert!(compaction_page_maintenance_step().unwrap());
        {
            let retired = COMPACTION_PAGE_RETIREMENT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            assert_eq!(retired.iter().flatten().find_map(|owner| owner.slots()[0].as_ref().map(db_state::Page::operation)), Some(exact_operation));
        }
        while compaction_page_maintenance_step().unwrap() {}

        for tier in [&COMPACTION_PAGE_RETIREMENT, &COMPACTION_PAGE_RETIREMENT_OVERFLOW, &COMPACTION_PAGE_RETIREMENT_QUARANTINE] {
            let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in owners.iter_mut() {
                *slot = Some(CompactionRetainedPages::new());
            }
        }
        let exact_refusal = state_page(b"exact-all-tier-compaction-refusal").await;
        let exact_refusal_operation = exact_refusal.operation();
        let mut refused = CompactionRetainedPages::new();
        let exact_refusal = refused.try_push(exact_refusal).unwrap_err();
        assert_eq!(exact_refusal.operation(), exact_refusal_operation);
        for tier in [&COMPACTION_PAGE_RETIREMENT, &COMPACTION_PAGE_RETIREMENT_OVERFLOW, &COMPACTION_PAGE_RETIREMENT_QUARANTINE] {
            let mut owners = tier.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            for slot in owners.iter_mut() {
                *slot = None;
            }
        }
        let mut recovered = CompactionRetainedPages::new();
        assert!(recovered.try_push(exact_refusal).is_ok());
        assert_eq!(MountedCompactionPageClose::new(&mut recovered).await.unwrap(), CompactionCloseExit::Closed);
        assert!(refused.terminal_is_empty());
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
    async fn compaction_applies_only_committed_frontier_snapshot_and_payload_effects() {
        let fixture = committed_compaction_fixture();
        let storage = MemoryStorage::new(crate::db_storage::db_io_test_pool()).await.unwrap();
        let document = doc("committed-compaction-effects").await;
        let aborted = storage.put(pages(b"aborted-payload")).await.unwrap();
        let committed = storage.put(pages(b"committed-payload")).await.unwrap();
        let mut previous = None;
        for row in fixture["segments"].as_array().unwrap() {
            previous = Some(append_committed_compaction_segment(&storage, &document, row, previous, aborted, committed).await);
        }
        let backend = db_storage::DbBackend::Memory(storage);
        let report = Compactor::new(&backend)
            .await
            .run(&document, "committed-compaction-holder", fixture["floorHeadSeq"].as_u64().unwrap(), false, &CompactionBudget::default(), 0)
            .await
            .unwrap();
        assert_eq!(report.wal_segments_deleted, fixture["expected"]["deletedSegments"].as_u64().unwrap());
        assert_eq!(report.payloads_deleted, fixture["expected"]["deletedPayloads"].as_u64().unwrap());
        let wal = backend.wal().await;
        let remaining_segments: Vec<u64> = fixture["expected"]["remainingSegments"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap()).collect();
        assert_eq!(wal.list_segments(&document).await.unwrap().as_slice(), remaining_segments.as_slice(), "header-only highest segment remains the active horizon");
        let payload = backend.payload().await;
        let retained_payloads = fixture["expected"]["retainedPayloads"].as_array().unwrap();
        assert_eq!(payload.contains(&aborted).await.unwrap(), retained_payloads.iter().any(|value| value == "aborted"), "aborted CAS reference never becomes a deletion candidate");
        assert_eq!(payload.contains(&committed).await.unwrap(), retained_payloads.iter().any(|value| value == "committed"), "committed CAS reference in the deleted segment is reclaimed");
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
