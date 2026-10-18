//! 🧯️ Fault storage: the fault script and the fault injecting storage with its storage trait implementations, available to tests and the testkit. See [`super`].

use super::*;

//#region 🔖️FaultStorage
#[cfg(any(test, feature = "testkit"))]
/// 💥️ One knob of `FaultStorage`'s injectable fault script — every field independently
/// optional/off by default, so a freshly `FaultScript::default()`ed `FaultStorage` behaves exactly
/// like its inner backend.
#[derive(Clone, Copy, Default, Debug)]
pub struct FaultScript {
    /// 🚫️ The Nth call to `WalStorage::append` (1-indexed, counting EVERY append this
    /// storage sees, including a document's own genesis header write) fails outright with
    /// `DbError::Io` before reaching the inner backend — models a crash where the physical write
    /// never lands at all.
    pub fail_nth_write: Option<u64>,
    /// ✂️ `(nth, keep_bytes)`: the Nth `append` call forwards only its first `keep_bytes`
    /// bytes to the inner backend (still succeeding, reporting the inner backend's real new
    /// length) — models a crash mid-`write(2)`, the "torn write" fault `db_wal`'s own recovery
    /// (`WalRecoveryReport.torn_tail_bytes`) exists to detect and truncate.
    pub torn_write_at: Option<(u64, u64)>,
    /// 🚫️ The Nth call to `WalStorage::sync` (1-indexed) fails before reaching the
    /// inner backend, after any preceding append has already completed.
    pub fail_nth_sync: Option<u64>,
    /// 🤥️ `WalStorage::sync` returns `Ok(())` without ever forwarding to the inner backend —
    /// models a storage device that acknowledges `fsync` without actually forcing data to physical
    /// storage (a caller relying on `DurabilityClass::Fsync` alone, without independently verifying
    /// durability, cannot tell the difference from the return value).
    pub fsync_lies: bool,
    /// ⚔️ The Nth call to `CatalogStorage::cas_root` (1-indexed) fails with a synthetic
    /// `DbError::Fenced` (as if a competing writer had already advanced the epoch) without ever
    /// touching the inner backend — models exercising a caller's CAS-retry path without needing a
    /// second real writer.
    pub cas_conflict_nth: Option<u64>,
}

#[cfg(any(test, feature = "testkit"))]
/// 💥️ Wraps a real `db_storage::DbStorage` backend with a scriptable `FaultScript`, injected
/// only at the WAL append/sync and catalog-CAS boundaries (see `FaultScript`'s fields) — every
/// other operation (segment lifecycle, snapshot/payload/index/lease storage) passes straight
/// through to `inner` untouched, since those are outside this testkit's stated crash-simulation
/// scope (fail-nth-write/sync / torn write / fsync-lie / CAS-conflict injection).
pub struct FaultStorage {
    pub inner: Arc<DbBackend>,
    pub script: Mutex<FaultScript>,
    pub append_calls: AtomicU64,
    pub sync_calls: AtomicU64,
    pub(super) sync_delegated_calls: AtomicU64,
    pub(super) cas_calls: AtomicU64,
}

#[cfg(any(test, feature = "testkit"))]
impl FaultStorage {
    pub async fn new(inner: Arc<DbBackend>) -> FaultStorage {
        FaultStorage { inner, script: Mutex::new(FaultScript::default()), append_calls: AtomicU64::new(0), sync_calls: AtomicU64::new(0), sync_delegated_calls: AtomicU64::new(0), cas_calls: AtomicU64::new(0) }
    }

    pub async fn set_script(&self, script: FaultScript) {
        *self.script.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = script;
    }

    pub async fn script(&self) -> FaultScript {
        *self.script.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// 🔢️ Every `WalStorage::append` call this storage has seen so far, faulted or not —
    /// `CrashHarness` uses this to discover exactly how many write boundaries a workload has.
    pub async fn append_calls(&self) -> u64 {
        self.append_calls.load(Ordering::SeqCst)
    }

    /// 🔢️ Every `WalStorage::sync` call this storage has seen, faulted or not.
    pub async fn sync_calls(&self) -> u64 {
        self.sync_calls.load(Ordering::SeqCst)
    }

    /// 🔢️ How many `WalStorage::sync` calls actually reached the inner backend (as opposed
    /// to being lied about under `FaultScript::fsync_lies`).
    pub async fn sync_delegated_calls(&self) -> u64 {
        self.sync_delegated_calls.load(Ordering::SeqCst)
    }

    pub async fn cas_calls(&self) -> u64 {
        self.cas_calls.load(Ordering::SeqCst)
    }

    /// 🎚️ Passes the inner backend's own capabilities straight through — fault injection
    /// never changes what the backend claims to support, only what it actually does on a call.
    pub async fn capabilities(&self) -> StorageCapabilities {
        self.inner.capabilities().await
    }

    /// 🔚️ Faults never script a close; it closes the wrapped backend.
    pub async fn close(&self) -> Result<(), DbError> {
        self.inner.close().await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl WalStorage for FaultStorage {
    async fn acquire_writer(&self, document: &ArtifactId) -> Result<db_io_core::WalWriterPermit, DbError> {
        self.inner.wal().await.acquire_writer(document).await
    }
    async fn create_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.wal().await.create_segment(writer, index).await
    }

    async fn append(&self, writer: &db_io_core::WalWriterPermit, index: u64, bytes: DbIoPages) -> Result<u64, DbError> {
        let call = self.append_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let script = self.script().await;
        {
            if script.fail_nth_write == Some(call) {
                return Err(DbError::Io(format!("fault_storage: injected failure on wal append #{call}")));
            }
            if let Some((torn_call, keep_bytes)) = script.torn_write_at {
                if torn_call == call {
                    let keep = (keep_bytes as usize).min(bytes.len());
                    let pages = match bytes.try_prefix(keep) {
                        Ok(pages) => pages,
                        Err(_) => return Err(DbError::Internal("fault_storage torn prefix exceeded the retained DB page owner".to_string())),
                    };
                    return self.inner.wal().await.append(writer, index, pages).await;
                }
            }
            self.inner.wal().await.append(writer, index, bytes).await
        }
    }

    async fn sync(&self, writer: &db_io_core::WalWriterPermit, index: u64, class: DurabilityClass) -> Result<(), DbError> {
        let call = self.sync_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let script = self.script().await;
        if script.fail_nth_sync == Some(call) {
            return Err(DbError::Io(format!("fault_storage: injected failure on wal sync #{call}")));
        }
        if script.fsync_lies {
            return Ok(());
        }
        self.sync_delegated_calls.fetch_add(1, Ordering::SeqCst);
        self.inner.wal().await.sync(writer, index, class).await
    }

    async fn seal(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.wal().await.seal(writer, index).await
    }

    async fn read(&self, document: &ArtifactId, index: u64, range: pack::ByteRange) -> Result<DbIoPages, DbError> {
        self.inner.wal().await.read(document, index, range).await
    }

    async fn segment_len(&self, document: &ArtifactId, index: u64) -> Result<u64, DbError> {
        self.inner.wal().await.segment_len(document, index).await
    }

    async fn segment_state(&self, document: &ArtifactId, index: u64) -> Result<WalSegmentState, DbError> {
        self.inner.wal().await.segment_state(document, index).await
    }

    async fn list_segments(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        self.inner.wal().await.list_segments(document).await
    }

    async fn truncate_tail(&self, writer: &db_io_core::WalWriterPermit, index: u64, new_len: u64) -> Result<(), DbError> {
        self.inner.wal().await.truncate_tail(writer, index, new_len).await
    }

    async fn delete_segment(&self, writer: &db_io_core::WalWriterPermit, index: u64) -> Result<(), DbError> {
        self.inner.wal().await.delete_segment(writer, index).await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl SnapshotStorage for FaultStorage {
    fn publication_scope(&self) -> usize {
        self.inner.publication_scope()
    }

    async fn write_generation(&self, document: &ArtifactId, generation: u64, bytes: DbIoPages) -> Result<(), DbError> {
        self.inner.snapshot().await.write_generation(document, generation, bytes).await
    }

    async fn read_generation(&self, document: &ArtifactId, generation: u64) -> Result<DbIoPages, DbError> {
        self.inner.snapshot().await.read_generation(document, generation).await
    }

    async fn latest_generation(&self, document: &ArtifactId) -> Result<Option<u64>, DbError> {
        self.inner.snapshot().await.latest_generation(document).await
    }

    async fn list_generations(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        self.inner.snapshot().await.list_generations(document).await
    }

    async fn delete_generation(&self, document: &ArtifactId, generation: u64) -> Result<(), DbError> {
        self.inner.snapshot().await.delete_generation(document, generation).await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl PayloadStorage for FaultStorage {
    async fn put(&self, bytes: DbIoPages) -> Result<ContentHash, DbError> {
        self.inner.payload().await.put(bytes).await
    }

    async fn get(&self, hash: &ContentHash) -> Result<DbIoPages, DbError> {
        self.inner.payload().await.get(hash).await
    }

    async fn contains(&self, hash: &ContentHash) -> Result<bool, DbError> {
        self.inner.payload().await.contains(hash).await
    }

    async fn delete(&self, hash: &ContentHash) -> Result<(), DbError> {
        self.inner.payload().await.delete(hash).await
    }

    async fn len(&self, hash: &ContentHash) -> Result<u64, DbError> {
        self.inner.payload().await.len(hash).await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl CatalogStorage for FaultStorage {
    async fn read_root(&self) -> Result<Option<(DbIoPages, EpochFence)>, DbError> {
        self.inner.catalog().await.read_root().await
    }

    async fn cas_root(&self, expected: EpochFence, new_bytes: DbIoPages) -> Result<EpochFence, DbError> {
        let call = self.cas_calls.fetch_add(1, Ordering::SeqCst) + 1;
        let conflict = self.script().await.cas_conflict_nth == Some(call);
        {
            if conflict {
                let current = self.inner.catalog().await.read_root().await?.map_or(EpochFence::INITIAL, |(_, fence)| fence);
                return Err(DbError::Fenced { expected: current.epoch, actual: expected.epoch });
            }
            self.inner.catalog().await.cas_root(expected, new_bytes).await
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
impl IndexStorage for FaultStorage {
    async fn write_run(&self, document: &ArtifactId, run_id: u64, bytes: DbIoPages) -> Result<(), DbError> {
        self.inner.index().await.write_run(document, run_id, bytes).await
    }

    async fn read_run(&self, document: &ArtifactId, run_id: u64) -> Result<DbIoPages, DbError> {
        self.inner.index().await.read_run(document, run_id).await
    }

    async fn list_runs(&self, document: &ArtifactId) -> Result<DbIoU64List, DbError> {
        self.inner.index().await.list_runs(document).await
    }

    async fn delete_run(&self, document: &ArtifactId, run_id: u64) -> Result<(), DbError> {
        self.inner.index().await.delete_run(document, run_id).await
    }
}

#[cfg(any(test, feature = "testkit"))]
impl LeaseStorage for FaultStorage {
    async fn acquire(&self, resource: &str, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        self.inner.lease().await.acquire(resource, holder, ttl_ms, now_ms).await
    }

    async fn renew(&self, resource: &str, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        self.inner.lease().await.renew(resource, holder, fence, ttl_ms, now_ms).await
    }

    async fn release(&self, resource: &str, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        self.inner.lease().await.release(resource, holder, fence).await
    }

    async fn current(&self, resource: &str, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        self.inner.lease().await.current(resource, now_ms).await
    }
}
//#endregion 🔖️FaultStorage
