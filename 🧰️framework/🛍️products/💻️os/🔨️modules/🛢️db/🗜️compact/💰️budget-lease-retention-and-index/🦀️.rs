//! 💰️ Budget, lease, retention and index: the compaction budget and lease, WAL retention, payload GC and index compaction. See [`super`].

use super::*;

//#region 🔖️Budget
/// 💰️ Bounds how much work one `Compactor::run` pass does across every subsystem — the
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
    /// ♾️ No cap on any subsystem — for tests and offline/maintenance runs where bounded
    /// latency doesn't matter.
    pub const fn unlimited() -> CompactionBudget {
        CompactionBudget { max_wal_segments: u64::MAX, max_snapshot_generations: u64::MAX, max_payloads: u64::MAX }
    }
}

impl Default for CompactionBudget {
    /// 🏗️ This crate's own choice of defaults (the contract fixes that budgets exist, not
    /// their numbers): generous enough that an ordinary document's compaction pass finishes in one
    /// call, small enough that a pathological document (a snapshot chain thousands deep) can't
    /// stall an online compactor indefinitely.
    fn default() -> Self {
        CompactionBudget { max_wal_segments: 64, max_snapshot_generations: 64, max_payloads: 4_096 }
    }
}
//#endregion 🔖️Budget

//#region 🔖️Lease
/// 🚧️ The fencing primitive behind "online compaction with manifest CAS + fencing" (see
/// module doc's design-choice note on why this wraps `LeaseStorage` rather than `CatalogStorage`).
/// Mirrors `db_snapshot::SnapshotLease`'s shape exactly, under its own `"compact:"`-prefixed
/// resource namespace so a document's snapshot builder and its compactor never contend on the
/// same lease.
pub struct CompactionLease;

impl CompactionLease {
    /// 🏷️ The `LeaseStorage` resource name guarding `document`'s compaction pass.
    pub fn resource(document: &ArtifactId) -> String {
        format!("compact:{document}")
    }

    /// 🤝️ Acquires (or idempotently re-acquires) the compaction lease for `document`.
    pub async fn acquire(storage: &impl db_io_core::LeaseStorage, document: &ArtifactId, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        storage.acquire(&Self::resource(document), holder, ttl_ms, now_ms).await
    }

    /// ♻️ Extends `holder`'s existing compaction lease for `document`.
    pub async fn renew(storage: &impl db_io_core::LeaseStorage, document: &ArtifactId, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        storage.renew(&Self::resource(document), holder, fence, ttl_ms, now_ms).await
    }

    /// 🕊️ Releases `holder`'s compaction lease for `document` — `Compactor::run` always
    /// calls this once, even if the pass itself failed (see that method's doc).
    pub async fn release(storage: &impl db_io_core::LeaseStorage, document: &ArtifactId, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        storage.release(&Self::resource(document), holder, fence).await
    }

    /// 👀️ The compaction lease's current holder/fence for `document`, or `None` if unheld.
    pub async fn current(storage: &impl db_io_core::LeaseStorage, document: &ArtifactId, now_ms: u64) -> Result<Option<db_io_core::LeaseInfo>, DbError> {
        storage.current(&Self::resource(document), now_ms).await
    }
}

/// ⏳️ This crate's own default compaction-lease TTL (the contract doesn't fix a number):
/// long enough that a slow consolidate-and-prune pass over a deep snapshot chain doesn't lose the
/// lease mid-flight, short enough that a crashed compactor's stale lease self-heals quickly.
pub(super) const DEFAULT_LEASE_TTL_MS: u64 = 5 * 60 * 1_000;
//#endregion 🔖️Lease

//#region 🔖️WalRetention
/// 🧾️ One WAL segment's role in a retention decision: its index and the highest
/// `head_seq` any `WAL_FRONTIER`/`WAL_SNAPSHOT_PUB` record within its span reached.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SegmentHorizon {
    pub segment_index: u64,
    /// ❓️ `None` if the segment carries no frontier marker at all — such a segment is
    /// never a `plan_wal_retention` candidate, since there is no way to prove everything in it is
    /// covered by a given floor.
    pub max_head_seq: Option<u64>,
}

/// 🪢️ Groups already-admitted records in segment-then-on-disk order by the
/// `WalRecord::SegmentHeader` boundaries that open each span —
/// the shared traversal `segment_horizons`/`sweep_payloads` both build on.
/// 📊️ Computes every segment's `SegmentHorizon` from a document's full replayed record
/// stream.
#[cfg(test)]
pub async fn segment_horizons<'record>(records: impl IntoIterator<Item = &'record db_wal_log::WalRecord>) -> Vec<SegmentHorizon> {
    let mut horizons = Vec::new();
    let mut current: Option<SegmentHorizon> = None;
    for record in records {
        match record {
            db_wal_log::WalRecord::SegmentHeader { segment_index, .. } => {
                if let Some(horizon) = current.replace(SegmentHorizon { segment_index: *segment_index, max_head_seq: None }) {
                    horizons.push(horizon);
                }
            }
            db_wal_log::WalRecord::Frontier(frontier) | db_wal_log::WalRecord::SnapshotPub { frontier, .. } => {
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

/// 🧹️ Selects which SEALED WAL segments are safe to delete: strictly below the highest
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

/// 🗑️ Applies `plan_wal_retention`'s output: deletes each selected segment from `storage`.
/// Idempotent (`WalStorage::delete_segment` already is). Returns how many were selected (and thus
/// attempted).
#[cfg(test)]
async fn release_compaction_test_writer(writer: db_io_core::WalWriterPermit) -> Result<(), DbError> {
    let mut release = writer.release();
    let mut first_error = None;
    loop {
        match release.await {
            Ok(()) => return first_error.map_or(Ok(()), Err),
            Err(failure) => {
                let (error, returned) = failure.into_parts();
                first_error.get_or_insert(error);
                release = returned.retry();
            }
        }
    }
}

#[cfg(test)]
pub async fn apply_wal_retention(storage: &impl db_io_core::WalStorage, document: &ArtifactId, segments: &[u64]) -> Result<u64, DbError> {
    let writer = storage.acquire_writer(document).await?;
    let result = async {
        for &index in segments {
            storage.delete_segment(&writer, index).await?;
        }
        Ok(segments.len() as u64)
    }
    .await;
    let release = release_compaction_test_writer(writer).await;
    match (result, release) {
        (Err(error), _) => Err(error),
        (Ok(_), Err(error)) => Err(error),
        (Ok(deleted), Ok(())) => Ok(deleted),
    }
}
//#endregion 🔖️WalRetention

//#region 🔖️PayloadGc
/// 📊️ What `sweep_payloads` did.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PayloadGcReport {
    pub candidates_checked: u64,
    pub deleted: u64,
}

/// 🔗️ Ref-traced payload GC: `candidates` are the `WalPayloadRef::CasRef` hashes that
/// appeared ONLY within `deleted_segments`' span of `records` (i.e. payloads that just lost their
/// one known reference); any candidate NOT also referenced by a record outside
/// `deleted_segments` is genuinely orphaned and deleted from `payload_storage`.
///
/// 🎯️ Scope boundary: `db_storage::PayloadStorage` has no enumeration method (by design — CAS
/// stores don't need one for `put`/`get`/`delete`), so this crate cannot do a full mark-and-sweep
/// over every payload ever stored; it can only trace liveness for a caller-supplied candidate set,
/// which is exactly what `Compactor::run` derives from its own WAL retention pass.
#[cfg(test)]
pub async fn sweep_payloads<'record>(payload_storage: &impl db_io_core::PayloadStorage, records: impl IntoIterator<Item = &'record db_wal_log::WalRecord>, deleted_segments: &[u64], budget: &CompactionBudget) -> Result<PayloadGcReport, DbError> {
    let deleted_set: std::collections::HashSet<u64> = deleted_segments.iter().copied().collect();
    let mut candidates = std::collections::HashSet::new();
    let mut live = std::collections::HashSet::new();
    let mut segment_index = 0;
    for record in records {
        if let db_wal_log::WalRecord::SegmentHeader { segment_index: next, .. } = record {
            segment_index = *next;
        } else if let db_wal_log::WalRecord::Payload(db_wal_log::WalPayloadRef::CasRef(hash)) = record {
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
/// 🧹️ One `IndexKind`'s post-compaction shape.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IndexKindReport {
    pub kind: db_index::IndexKind,
    pub stats: db_index::IndexStats,
}

const COMPACTION_INDEX_REPORTS: usize = db_index::IndexKind::ALL.len();

/// 🗂️ Fixed admitted index-report owner returned by one compaction generation.
#[derive(Clone, Debug, PartialEq)]
pub struct CompactionIndexReports {
    pub(super) slots: [Option<IndexKindReport>; COMPACTION_INDEX_REPORTS],
    pub(super) len: u8,
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

    pub(super) fn push(&mut self, report: IndexKindReport) -> Result<(), DbError> {
        let index = self.len();
        let slot = self.slots.get_mut(index).ok_or(DbError::LimitExceeded("db_compact fixed index reports"))?;
        *slot = Some(report);
        self.len = self.len.checked_add(1).ok_or(DbError::LimitExceeded("db_compact index report cursor"))?;
        Ok(())
    }

    pub(super) fn close_step(&mut self) -> bool {
        if self.len == 0 {
            return false;
        }
        self.len -= 1;
        self.slots[usize::from(self.len)] = None;
        true
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.slots.iter().all(Option::is_none)
    }
}

impl Default for CompactionIndexReports {
    fn default() -> Self {
        Self { slots: [None; COMPACTION_INDEX_REPORTS], len: 0 }
    }
}

/// 🧹️ Compacts every `db_index::IndexKind` for `document`: merges all live runs into one
/// per kind, physically dropping tombstones shadowed beneath them (`db_index::IndexHandle::
/// compact`'s own law) — the mechanism behind the contract's "index merge" and, for
/// `IndexKind::Preview`/`Conflict` specifically, its share of "tombstone/preview GC": once a
/// withdrawn preview key or a resolved conflict marker is the only thing a tombstone shadows,
/// compacting reclaims it for good.
#[cfg(test)]
pub async fn compact_all_indexes(storage: &impl db_io_core::IndexStorage, document: &ArtifactId) -> Result<CompactionIndexReports, DbError> {
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
