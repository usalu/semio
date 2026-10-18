//! 🏗️ Job execution: the horizons, the WAL, lease and snapshot steps and the retained execution. See [`super`].

use super::*;

//#region 🧵️RetainedCompactionJob
pub(super) async fn compaction_opportunity(cancelled: &std::sync::atomic::AtomicBool) -> Result<(), DbError> {
    semio_framework_async::yield_once().await;
    if cancelled.load(std::sync::atomic::Ordering::Acquire) {
        return Err(DbError::Closed);
    }
    Ok(())
}

pub(super) fn compaction_resource(document: &ArtifactId) -> Result<db_io_core::DbIoText, DbError> {
    use std::fmt::Write as _;
    let mut resource = db_io_core::DbIoText::try_from_str("compact:")?;
    write!(&mut resource, "{}", document.0).map_err(|_| DbError::LimitExceeded("database compaction lease resource"))?;
    Ok(resource)
}

async fn close_compaction_replay<S: db_io_core::WalStorage>(replay: &mut db_wal_log::WalCommittedCursor<'_, S>) -> Result<(), DbError> {
    close_compaction_owner(|| replay.close_owner_step()).await
}

pub(super) async fn close_compaction_owner(mut close: impl FnMut() -> Result<bool, DbError>) -> Result<(), DbError> {
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

pub(super) async fn committed_compaction_horizons<S: db_io_core::WalStorage>(storage: &S, document: &ArtifactId, cancelled: &std::sync::atomic::AtomicBool) -> Result<DatabaseCompactionSegmentOwners, DbError> {
    let control = db_wal_log::WalCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
    let mut replay = db_wal_log::replay_committed_document(storage, document, control).await?;
    let scan = async {
        let mut horizons = DatabaseCompactionSegmentOwners::new();
        for segment_index in replay.segment_indices() {
            horizons.push(SegmentHorizon { segment_index: *segment_index, max_head_seq: None })?;
        }
        loop {
            compaction_opportunity(cancelled).await?;
            replay.renew_step()?;
            match replay.next_transaction_step().await? {
                db_wal_log::WalCommittedStep::Transaction(mut transaction) => {
                    let segment_index = transaction.segment_index();
                    loop {
                        match transaction.next_record_step()? {
                            db_wal_log::WalCommittedRecordStep::Record(record) => {
                                let observed = if let db_wal_log::WalRecord::Frontier(frontier) | db_wal_log::WalRecord::SnapshotPub { frontier, .. } = record { horizons.observe_head(segment_index, frontier.head_seq) } else { Ok(()) };
                                let closed = close_compaction_owner(|| transaction.close_record_step()).await;
                                observed?;
                                closed?;
                            }
                            db_wal_log::WalCommittedRecordStep::Yield => compaction_opportunity(cancelled).await?,
                            db_wal_log::WalCommittedRecordStep::Done => break,
                        }
                    }
                    transaction.finish()?;
                }
                db_wal_log::WalCommittedStep::Yield => {}
                db_wal_log::WalCommittedStep::Done => break,
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

/// 🧵 Runs live compaction while the document actor retains its original WAL writer.
pub async fn retained_compaction_with_wal(
    storage: Arc<db_storage::DbBackend>,
    artifact_wal: &mut db_wal::ArtifactWal,
    holder: db_io_core::DbIoText,
    consolidate_snapshots: bool,
    budget: CompactionBudget,
    now_ms: u64,
    cancelled: Arc<std::sync::atomic::AtomicBool>,
) -> Result<CompactionReport, DbError> {
    compaction_opportunity(cancelled.as_ref()).await?;
    let document = artifact_wal.document().await.clone();
    let fence = CompactionLease::acquire(&storage.lease().await, &document, holder.as_str(), DEFAULT_LEASE_TTL_MS, now_ms).await?;
    let mut ledger = DatabaseCompactionBackingLedger::default();
    let progress = std::sync::atomic::AtomicU8::new(DatabaseCompactionProgress::LeaseAcquire as u8);
    let result = async {
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
        retained_compaction_under_lease(storage.as_ref(), artifact_wal, &document, floor, consolidate_snapshots, budget, &cancelled, &progress, &mut ledger).await
    }
    .await;
    progress.store(DatabaseCompactionProgress::LeaseRelease as u8, std::sync::atomic::Ordering::Release);
    let release = CompactionLease::release(&storage.lease().await, &document, holder.as_str(), fence).await;
    match (result, release) {
        (Ok(report), Ok(())) => Ok(report),
        (Err(error), Ok(())) => Err(error),
        (Ok(_), Err(error)) => Err(error),
        (Err(error), Err(release_error)) => Err(DbError::Unavailable(format!("database compaction failed ({error}) and lease release failed ({release_error})"))),
    }
}

async fn retained_compaction_under_lease(
    storage: &db_storage::DbBackend,
    artifact_wal: &mut db_wal::ArtifactWal,
    document: &ArtifactId,
    floor_head_seq: u64,
    consolidate_snapshots: bool,
    budget: CompactionBudget,
    cancelled: &Arc<std::sync::atomic::AtomicBool>,
    progress: &std::sync::atomic::AtomicU8,
    ledger: &mut DatabaseCompactionBackingLedger,
) -> Result<CompactionReport, DbError> {
    let mut report = CompactionReport::default();
    let wal_storage = storage.wal().await;
    progress.store(DatabaseCompactionProgress::WalHorizon as u8, std::sync::atomic::Ordering::Release);
    let horizons = committed_compaction_horizons(&wal_storage, document, cancelled).await?;
    let active_segment = artifact_wal.active_segment_index().await;
    let mut selected = DatabaseCompactionSegmentOwners::new();
    for index in 0..horizons.len() {
        compaction_opportunity(cancelled).await?;
        let horizon = horizons.get(index).ok_or_else(|| DbError::Internal("database compaction horizon owner lost".to_string()))?;
        if horizon.segment_index != active_segment && horizon.max_head_seq.is_some_and(|head| head <= floor_head_seq) && selected.len() < usize::try_from(budget.max_wal_segments).unwrap_or(usize::MAX) {
            selected.push(horizon)?;
        }
    }

    progress.store(DatabaseCompactionProgress::WalDelete as u8, std::sync::atomic::Ordering::Release);
    for index in 0..selected.len() {
        compaction_opportunity(cancelled).await?;
        let segment = selected.get(index).ok_or_else(|| DbError::Internal("database compaction selected segment owner lost".to_string()))?;
        artifact_wal.delete_compacted_sealed_segment(&wal_storage, segment.segment_index).await?;
        report.wal_segments_deleted = report.wal_segments_deleted.checked_add(1).ok_or(DbError::LimitExceeded("database compaction deleted segments"))?;
    }
    drop(wal_storage);

    progress.store(DatabaseCompactionProgress::IndexMerge as u8, std::sync::atomic::Ordering::Release);
    let index_storage = storage.index().await;
    for kind in db_index::IndexKind::ALL {
        compaction_opportunity(cancelled).await?;
        let index_document = document.clone();
        let index_document_bytes = index_document.0.capacity();
        database_compaction_observe_backing(ledger, 1, index_document_bytes, "database compaction index document backing")?;
        let handle = db_index::IndexHandle::new(&index_storage, index_document, kind).await;
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(DATABASE_COMPACTION_TURN_MS);
        let mut control = handle.retained_operation_control(cancelled.clone(), deadline, DATABASE_COMPACTION_INDEX_FUEL)?;
        let stats = handle.compact(&mut control).await;
        drop(handle);
        ledger.release(1, index_document_bytes)?;
        let stats = match stats {
            Err(DbError::Unavailable(message)) if message == "index cursor cancelled" => return Err(DbError::Closed),
            result => result?,
        };
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
                    let bytes = items.checked_mul(db_io_core::DB_IO_PAGE_BYTES).ok_or(DbError::LimitExceeded("database compaction snapshot page backing"))?;
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
    let db_snapshot::SnapshotDescriptor { document: ArtifactId(latest_document), head_seq, commit_seq, epoch, chain_hash, protocol_version, base_pack_hash, roots, new_pages: latest_new_pages, created_at_ms, .. } = latest_descriptor;
    semio_framework_async::yield_once().await;
    drop(latest_new_pages);
    semio_framework_async::yield_once().await;
    drop(latest_document);
    let mut body = db_snapshot::SnapshotBody { head_seq, commit_seq, epoch, chain_hash, protocol_version, base_pack_hash, roots, created_at_ms };
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

pub(super) struct DatabaseCompactionExecution {
    pub(super) storage: Arc<db_storage::DbBackend>,
    pub(super) document: ArtifactId,
    pub(super) holder: db_io_core::DbIoText,
    pub(super) result: Result<CompactionReport, DbError>,
}

pub(super) type DatabaseCompactionLeaseReleaseFuture = std::pin::Pin<Box<dyn Future<Output = Result<(), DbError>> + Send + 'static>>;

pub(super) struct DatabaseCompactionLeaseRecovery {
    pub(super) storage: Arc<db_storage::DbBackend>,
    pub(super) resource: db_io_core::DbIoText,
    pub(super) holder: db_io_core::DbIoText,
    pub(super) fence: std::sync::Mutex<Option<EpochFence>>,
    pub(super) releasing: std::sync::atomic::AtomicBool,
    pub(super) released: std::sync::atomic::AtomicBool,
    #[cfg(test)]
    controlled_release_failures: std::sync::atomic::AtomicUsize,
    #[cfg(test)]
    pub(super) release_attempts: std::sync::atomic::AtomicUsize,
}

impl DatabaseCompactionLeaseRecovery {
    pub(super) fn new(storage: Arc<db_storage::DbBackend>, resource: db_io_core::DbIoText, holder: db_io_core::DbIoText) -> Arc<Self> {
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

    pub(super) fn install(&self, fence: EpochFence) {
        *self.fence.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(fence);
    }

    pub(super) fn release_future(self: &Arc<Self>) -> Option<DatabaseCompactionLeaseReleaseFuture> {
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
            let injected = recovery.controlled_release_failures.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |remaining| remaining.checked_sub(1)).is_ok();
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

    pub(super) fn retry_release_after_panic(self: &Arc<Self>) -> Option<DatabaseCompactionLeaseReleaseFuture> {
        self.releasing.store(false, std::sync::atomic::Ordering::Release);
        self.release_future()
    }

    #[cfg(test)]
    pub(super) fn fail_release_attempts(&self, attempts: usize) {
        self.controlled_release_failures.store(attempts, std::sync::atomic::Ordering::Release);
    }
}

pub(super) async fn retained_compaction_execute(
    storage: Arc<db_storage::DbBackend>,
    document: ArtifactId,
    holder: db_io_core::DbIoText,
    consolidate_snapshots: bool,
    budget: CompactionBudget,
    now_ms: u64,
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    expired: Arc<std::sync::atomic::AtomicBool>,
    progress: Arc<std::sync::atomic::AtomicU8>,
    lease_recovery: Arc<DatabaseCompactionLeaseRecovery>,
) -> DatabaseCompactionExecution {
    let mut result = async {
        compaction_opportunity(cancelled.as_ref()).await?;
        let wal_storage = storage.wal().await;
        let (mut artifact_wal, _) = match db_wal::ArtifactWal::open(&wal_storage, document.clone(), db_wal_log::GroupCommitPolicy::default(), now_ms).await {
            Ok(opened) => opened,
            Err(rejected) => return Err(close_compaction_wal_open_rejected(rejected).await),
        };
        let operation = async {
            let mut ledger = DatabaseCompactionBackingLedger::default();
            compaction_opportunity(cancelled.as_ref()).await?;
            progress.store(DatabaseCompactionProgress::LeaseAcquire as u8, std::sync::atomic::Ordering::Release);
            let lease = storage.lease().await;
            let fence = lease.acquire(lease_recovery.resource.as_str(), holder.as_str(), DEFAULT_LEASE_TTL_MS, now_ms).await?;
            drop(lease);
            lease_recovery.install(fence);
            let run = async {
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
                retained_compaction_under_lease(storage.as_ref(), &mut artifact_wal, &document, floor, consolidate_snapshots, budget, &cancelled, progress.as_ref(), &mut ledger).await
            }
            .await;
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
        };
        let operation = operation.await;
        let close = artifact_wal.close().await;
        match (operation, close) {
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
            (Ok(report), Ok(())) => Ok(report),
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
//#endregion 🧵️RetainedCompactionJob
