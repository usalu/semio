//! 🧊️ Cold archive and compactor: the cold archive build and the compactor. See [`super`].

use super::*;

//#region 🔖️ColdArchive
/// 🧊️ Builds one document's cold-tier archive: the full, self-contained byte concatenation
/// of every snapshot generation from the chain's root through `through_generation` (via
/// the snapshot retained chain cursor), independently reopenable with
/// `db_snapshot::open_latest` — ready to hand to whatever cold-tier object store a deployment
/// configures.
///
/// 🎯️ Scope boundary: `db_storage` defines no `ColdStorage` trait, so this crate returns the
/// archive bytes rather than inventing a storage seam unilaterally in a crate that isn't
/// `db_storage`'s own.
#[cfg(test)]
pub async fn build_cold_archive(storage: &impl db_io_core::SnapshotStorage, document: &ArtifactId, through_generation: u64) -> Result<db_io_core::DbIoPages, DbError> {
    let manager = db_snapshot::SnapshotManager::new(storage).await;
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
    let mut cursor = manager.chain_cursor(document, through_generation, control);
    let pages = cursor.materialize_pages().await?;
    let _ = cursor.close_step()?;
    Ok(pages)
}
//#endregion 🔖️ColdArchive

//#region 🔖️Compactor
/// 📋️ What one `Compactor::run` pass did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CompactionReport {
    pub wal_segments_deleted: u64,
    pub payloads_deleted: u64,
    pub index_reports: CompactionIndexReports,
    pub snapshot_consolidated_generation: Option<u64>,
    pub snapshot_generations_pruned: u64,
}

/// 🧑️‍💼️ The top-level, fenced, budgeted orchestrator gluing every subsystem in this crate
/// together over one `db_storage::DbStorage` backend — "online compaction with manifest CAS +
/// fencing" (see module doc's design-choice note on the fencing mechanism).
#[cfg(test)]
pub struct Compactor<'storage> {
    pub(super) storage: &'storage db_storage::DbBackend,
}

pub(super) async fn close_compaction_wal_open_rejected(mut rejected: db_wal::ArtifactWalOpenRejected) -> DbError {
    loop {
        match rejected.retry_close().await {
            Ok(cause) => return cause,
            Err(retained) => rejected = retained,
        }
    }
}

#[cfg(test)]
impl<'storage> Compactor<'storage> {
    pub async fn new(storage: &'storage db_storage::DbBackend) -> Compactor<'storage> {
        Compactor { storage }
    }

    /// 🚀️ One bounded, fenced compaction pass over `document`: WAL segment retention below
    /// `wal_floor_head_seq`, ref-traced payload GC over whatever WAL retention just orphaned, every
    /// index kind's merge/tombstone-GC, and — if `consolidate_snapshots` is set — rolling the
    /// snapshot chain into a fresh full baseline and pruning everything below it. Acquires
    /// `CompactionLease::resource(document)` for the whole pass; a lease held elsewhere surfaces as
    /// `DbError::Conflict` rather than silently racing another compactor. The lease is ALWAYS
    /// released before returning, including when a step fails partway through — a failed pass must
    /// never leave a document permanently unfenceable.
    pub async fn run(&self, document: &ArtifactId, holder: &str, wal_floor_head_seq: u64, consolidate_snapshots: bool, budget: &CompactionBudget, now_ms: u64) -> Result<CompactionReport, DbError> {
        self.run_owned(document, holder, Some(wal_floor_head_seq), consolidate_snapshots, budget, now_ms).await
    }

    /// 🧭️ Convenience over `run`: derives `wal_floor_head_seq` from `document`'s current
    /// latest snapshot generation (or `0`, i.e. nothing deletable, if it has none yet).
    pub async fn run_from_latest_snapshot(&self, document: &ArtifactId, holder: &str, consolidate_snapshots: bool, budget: &CompactionBudget, now_ms: u64) -> Result<CompactionReport, DbError> {
        self.run_owned(document, holder, None, consolidate_snapshots, budget, now_ms).await
    }

    async fn run_owned(&self, document: &ArtifactId, holder: &str, floor: Option<u64>, consolidate_snapshots: bool, budget: &CompactionBudget, now_ms: u64) -> Result<CompactionReport, DbError> {
        let wal_storage = self.storage.wal().await;
        let (mut wal, _) = match db_wal::ArtifactWal::open(&wal_storage, document.clone(), db_wal_log::GroupCommitPolicy::default(), now_ms).await {
            Ok(opened) => opened,
            Err(rejected) => return Err(close_compaction_wal_open_rejected(rejected).await),
        };
        let fence = match CompactionLease::acquire(&self.storage.lease().await, document, holder, DEFAULT_LEASE_TTL_MS, now_ms).await {
            Ok(fence) => fence,
            Err(error) => {
                wal.close().await?;
                return Err(error);
            }
        };
        let result = async {
            let floor = match floor {
                Some(floor) => floor,
                None => {
                    let snapshot = self.storage.snapshot().await;
                    db_snapshot::SnapshotManager::new(&snapshot).await.load_latest(document).await?.map_or(0, |(_, descriptor)| descriptor.head_seq)
                }
            };
            self.run_under_lease(&mut wal, document, floor, consolidate_snapshots, budget).await
        }
        .await;
        let release = CompactionLease::release(&self.storage.lease().await, document, holder, fence).await;
        let close = wal.close().await;
        match (result, release, close) {
            (Err(error), _, _) => Err(error),
            (Ok(_), Err(error), _) | (Ok(_), Ok(()), Err(error)) => Err(error),
            (Ok(report), Ok(()), Ok(())) => Ok(report),
        }
    }

    async fn run_under_lease(&self, wal: &mut db_wal::ArtifactWal, document: &ArtifactId, wal_floor_head_seq: u64, consolidate_snapshots: bool, budget: &CompactionBudget) -> Result<CompactionReport, DbError> {
        let mut report = CompactionReport::default();

        let wal_storage = self.storage.wal().await;
        let cancelled = std::sync::atomic::AtomicBool::new(false);
        let horizons = committed_compaction_horizons(&wal_storage, document, &cancelled).await?;
        let active_segment = wal.active_segment_index().await;
        let mut selected = DatabaseCompactionSegmentOwners::new();
        for index in 0..horizons.len() {
            let horizon = horizons.get(index).ok_or_else(|| DbError::Internal("database compaction horizon owner lost".to_string()))?;
            if horizon.segment_index != active_segment && horizon.max_head_seq.is_some_and(|head| head <= wal_floor_head_seq) && selected.len() < usize::try_from(budget.max_wal_segments).unwrap_or(usize::MAX) {
                selected.push(horizon)?;
            }
        }
        for index in 0..selected.len() {
            let segment = selected.get(index).ok_or_else(|| DbError::Internal("database compaction selected segment owner lost".to_string()))?;
            wal.delete_compacted_sealed_segment(&wal_storage, segment.segment_index).await?;
            report.wal_segments_deleted += 1;
        }
        drop(wal_storage);

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
