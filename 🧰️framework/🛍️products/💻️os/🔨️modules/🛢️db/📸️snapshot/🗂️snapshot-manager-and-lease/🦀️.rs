//! 🗂️ Snapshot manager and lease: the origin, body, retained publication and policy, the manager, its chain cursor and cursor control, and the snapshot lease. See [`super`].

use super::*;

//#region 🔖️Manager
/// 🌱️ Which lineage a `SnapshotManager::publish` call starts: `FullBaseline` is
/// self-sufficient (safe future `retain_from` floor); `Incremental` chains to the document's
/// current latest generation (errors if there isn't one yet).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnapshotOrigin {
    FullBaseline,
    Incremental,
}

/// 📦️ Everything about a generation that isn't derived from `new_pages`/plumbing — the
/// `SnapshotManager::publish` caller-supplied half of a `SnapshotDescriptor`.
#[derive(Clone, Debug)]
pub struct SnapshotBody {
    pub head_seq: u64,
    pub commit_seq: u64,
    pub epoch: u64,
    pub chain_hash: [u8; 32],
    pub protocol_version: u32,
    pub base_pack_hash: Option<ContentHash>,
    pub roots: Vec<ContentHash>,
    pub created_at_ms: u64,
}

/// 🧵️ Generation-qualified publication witness returning the exact dynamic body owner.
#[derive(Debug)]
pub struct SnapshotRetainedPublication {
    pub(super) generation: u64,
    pub(super) body: SnapshotBody,
}

impl SnapshotRetainedPublication {
    pub fn into_parts(self) -> (u64, SnapshotBody) {
        (self.generation, self.body)
    }
}

/// 🛡️ Atomic publication refusal retaining the exact body owner for incremental close.
#[derive(Debug)]
pub struct SnapshotRetainedPublicationRejected {
    pub(super) error: DbError,
    pub(super) body: SnapshotBody,
}

impl SnapshotRetainedPublicationRejected {
    pub fn into_parts(self) -> (DbError, SnapshotBody) {
        (self.error, self.body)
    }
}

/// 🎛️ Publish-time trigger thresholds — `should_snapshot` fires if any is met. This
/// crate's own choice of shape (the contract fixes only that triggers exist, not their inputs).
#[derive(Clone, Copy, Debug)]
pub struct SnapshotPolicy {
    pub max_ops_since_last: u64,
    pub max_bytes_since_last: u64,
    pub max_ms_since_last: u64,
}

impl SnapshotPolicy {
    pub async fn should_snapshot(&self, ops_since_last: u64, bytes_since_last: u64, ms_since_last: u64) -> bool {
        ops_since_last >= self.max_ops_since_last || bytes_since_last >= self.max_bytes_since_last || ms_since_last >= self.max_ms_since_last
    }
}

/// 🧑️‍💼️ Orchestrates `db_snapshot`'s pack-encoding logic on top of a
/// `db_storage::SnapshotStorage` backend: publish (`build_generation` + `write_generation`), load,
/// chain materialization, retention, and verification.
pub struct SnapshotManager<'storage, S: SnapshotStorage> {
    pub storage: &'storage S,
}

//#region 🔖️ChainCursor
/// ⏳️ One resumable snapshot-chain grant authority.
pub struct SnapshotCursorControl {
    pub cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    pub deadline: std::time::Instant,
    pub fuel: usize,
}

impl SnapshotCursorControl {
    pub fn new(cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>, deadline: std::time::Instant, fuel: usize) -> Result<Self, DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("snapshot cursor fuel"));
        }
        Ok(Self { cancelled, deadline, fuel })
    }

    pub fn replenish(&mut self, fuel: usize, deadline: std::time::Instant) -> Result<(), DbError> {
        if fuel == 0 {
            return Err(DbError::LimitExceeded("snapshot cursor fuel"));
        }
        self.fuel = fuel;
        self.deadline = deadline;
        Ok(())
    }

    pub fn grant(&mut self) -> Result<(), DbError> {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Unavailable("snapshot cursor cancelled".to_string()));
        }
        if std::time::Instant::now() >= self.deadline {
            return Err(DbError::Unavailable("snapshot cursor deadline reached".to_string()));
        }
        self.fuel = self.fuel.checked_sub(1).ok_or(DbError::LimitExceeded("snapshot cursor fuel"))?;
        Ok(())
    }
}

/// ⛓️ Incremental snapshot-chain reader retaining at most one generation owner and one
/// prepared platform slot per grant.
#[must_use]
pub struct SnapshotChainCursor<'manager, 'storage, S: SnapshotStorage> {
    pub manager: &'manager SnapshotManager<'storage, S>,
    pub document: &'manager ArtifactId,
    through_generation: u64,
    pub control: SnapshotCursorControl,
    operation: Option<u64>,
    pub closed: bool,
}

impl<'manager, 'storage, S: SnapshotStorage> SnapshotChainCursor<'manager, 'storage, S> {
    pub fn operation(&self) -> Option<u64> {
        self.operation
    }

    async fn descriptor_at(&mut self, generation: u64) -> Result<(SnapshotDescriptor, usize), DbError> {
        self.control.grant()?;
        let pages = self.manager.storage.read_generation(self.document, generation).await?;
        self.operation = Some(pages.operation());
        let len = pages.len();
        self.control.grant()?;
        let descriptor = open_latest_pages(&pages, &mut self.control).await?.descriptor;
        drop(pages);
        Ok((descriptor, len))
    }

    pub async fn latest_descriptor(&mut self) -> Result<SnapshotDescriptor, DbError> {
        self.descriptor_at(self.through_generation).await.map(|(descriptor, _)| descriptor)
    }

    pub async fn descriptor(&mut self, generation: u64) -> Result<SnapshotDescriptor, DbError> {
        self.descriptor_at(generation).await.map(|(descriptor, _)| descriptor)
    }

    pub async fn read_page(&mut self, hash: ContentHash) -> Result<db_io_core::DbIoPages, DbError> {
        let mut generation = Some(self.through_generation);
        while let Some(current) = generation {
            self.control.grant()?;
            let source = self.manager.storage.read_generation(self.document, current).await?;
            self.operation = Some(source.operation());
            self.control.grant()?;
            let handle = open_latest_pages(&source, &mut self.control).await?;
            if let Some(index) = handle.descriptor.new_pages.iter().position(|candidate| *candidate == hash) {
                let sub = PageSubSource { inner: &source, base: handle.base, len: handle.len };
                let file = pack::PackFile::open_manifest(sub, &pack::PackLimits::default(), pack::os_pack::VerificationLevel::Standard).await?;
                let mut cursor = file.identity_chunk_cursor(pack::ChunkId(index as u32), pack::os_pack::VerificationLevel::Standard)?;
                let mut writer = db_io_core::DbIoPageWriter::try_reserve_for_operation(source.operation(), usize::try_from(cursor.len()).map_err(|_| DbError::LimitExceeded("snapshot identity chunk length"))?.div_ceil(db_io_core::DB_IO_PAGE_BYTES))
                    .map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
                let mut fragment = [0u8; db_io_core::DB_IO_PAGE_BYTES];
                loop {
                    self.control.grant()?;
                    let read = cursor.read_fragment(&mut fragment).await?;
                    if read == 0 {
                        break;
                    }
                    if writer.write_fragment(&fragment[..read])? != read {
                        return Err(DbError::Internal("snapshot identity cursor made partial writer progress".to_string()));
                    }
                }
                drop(cursor);
                drop(file);
                drop(source);
                return writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error);
            }
            generation = handle.descriptor.parent_generation;
            drop(source);
        }
        Err(DbError::NotFound(format!("page {hash} not found anywhere in the snapshot chain")))
    }

    pub async fn materialize_pages(&mut self) -> Result<db_io_core::DbIoPages, DbError> {
        let mut root = self.through_generation;
        let mut current = Some(self.through_generation);
        let mut total = 0usize;
        while let Some(generation) = current {
            let (descriptor, len) = self.descriptor_at(generation).await?;
            root = generation;
            total = total.checked_add(len).ok_or(DbError::LimitExceeded("snapshot chain bytes"))?;
            current = descriptor.parent_generation;
        }
        let page_count = total.div_ceil(db_io_core::DB_IO_PAGE_BYTES);
        if page_count > db_io_core::DB_IO_OPERATION_PAGES {
            return Err(DbError::LimitExceeded("snapshot chain page reservation"));
        }
        let mut writer = db_io_core::DbIoPageWriter::try_reserve(page_count).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
        self.operation = Some(writer.operation());
        let mut expected_parent = None;
        for generation in root..=self.through_generation {
            self.control.grant()?;
            let source = self.manager.storage.read_generation(self.document, generation).await?;
            self.control.grant()?;
            let descriptor = open_latest_pages(&source, &mut self.control).await?.descriptor;
            if descriptor.parent_generation != expected_parent {
                return Err(DbError::Corrupt("snapshot generation lineage is not contiguous".to_string()));
            }
            expected_parent = Some(generation);
            for fragment in source.fragments() {
                let mut offset = 0;
                while offset < fragment.len() {
                    self.control.grant()?;
                    offset += writer.write_fragment(&fragment[offset..])?;
                }
            }
            drop(source);
        }
        writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)
    }

    pub fn close_step(&mut self) -> Result<bool, DbError> {
        self.control.grant()?;
        if self.closed {
            return Ok(false);
        }
        self.closed = true;
        self.operation = None;
        Ok(true)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.operation.is_none()
    }
}

impl<S: SnapshotStorage> Drop for SnapshotChainCursor<'_, '_, S> {
    fn drop(&mut self) {
        self.control.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.operation = None;
        self.closed = true;
    }
}
//#endregion 🔖️ChainCursor

impl<'storage, S: SnapshotStorage> SnapshotManager<'storage, S> {
    pub async fn new(storage: &'storage S) -> SnapshotManager<'storage, S> {
        SnapshotManager { storage }
    }

    pub fn chain_cursor<'manager>(&'manager self, document: &'manager ArtifactId, through_generation: u64, control: SnapshotCursorControl) -> SnapshotChainCursor<'manager, 'storage, S> {
        SnapshotChainCursor { manager: self, document, through_generation, control, operation: None, closed: false }
    }

    /// ✍️ Builds and durably writes the next generation. `origin == Incremental` chains to
    /// the document's current `latest_generation` (`DbError::InvalidArgument` if there is none
    /// yet); `origin == FullBaseline` starts (or restarts) a self-sufficient lineage.
    pub async fn publish(&self, document: &ArtifactId, origin: SnapshotOrigin, new_pages: &[Page], body: SnapshotBody) -> Result<u64, DbError> {
        self.publish_page_source(document, origin, new_pages, body).await
    }

    /// 🧵️ Publishes from a fixed optional page owner array without constructing a
    /// dynamic retained page graph.
    pub async fn publish_retained(&self, document: &ArtifactId, origin: SnapshotOrigin, new_pages: &[Option<Page>], retained_len: usize, body: SnapshotBody) -> Result<u64, DbError> {
        if retained_len > new_pages.len() || new_pages[..retained_len].iter().any(Option::is_none) {
            return Err(DbError::InvalidArgument("snapshot retained page source is not contiguous".to_string()));
        }
        self.publish_page_source(document, origin, &OptionalSnapshotPages { slots: new_pages, len: retained_len }, body).await
    }

    /// 🛡️ Publishes one full baseline only while `expected_generation` owns the atomic document claim.
    pub async fn publish_retained_expected(
        &self,
        document: &ArtifactId,
        expected_generation: u64,
        new_pages: &[Option<Page>],
        retained_len: usize,
        body: SnapshotBody,
        control: &mut SnapshotCursorControl,
    ) -> Result<SnapshotRetainedPublication, SnapshotRetainedPublicationRejected> {
        let publication = async {
            if retained_len > new_pages.len() || new_pages[..retained_len].iter().any(Option::is_none) {
                return Err(DbError::InvalidArgument("snapshot retained page source is not contiguous".to_string()));
            }
            let _claim = SnapshotPublicationClaim::try_claim(self.storage.publication_scope(), document)?;
            let observed = self.storage.latest_generation(document).await?;
            if observed != Some(expected_generation) {
                return Err(DbError::StaleGeneration { expected: GenerationId(expected_generation), actual: GenerationId(observed.unwrap_or(0)) });
            }
            let generation = expected_generation.checked_add(1).ok_or(DbError::LimitExceeded("snapshot publication generation"))?;
            let pages = build_generation_retained_expected(document, generation, &body, &OptionalSnapshotPages { slots: new_pages, len: retained_len }, control).await?;
            self.storage.write_generation(document, generation, pages).await?;
            Ok(generation)
        }
        .await;
        match publication {
            Ok(generation) => Ok(SnapshotRetainedPublication { generation, body }),
            Err(error) => Err(SnapshotRetainedPublicationRejected { error, body }),
        }
    }

    async fn publish_page_source<P: SnapshotPageSource + ?Sized>(&self, document: &ArtifactId, origin: SnapshotOrigin, new_pages: &P, body: SnapshotBody) -> Result<u64, DbError> {
        let _claim = SnapshotPublicationClaim::try_claim(self.storage.publication_scope(), document)?;
        let latest = self.storage.latest_generation(document).await?;
        let (generation, parent_generation, parent_footer_position) = match origin {
            SnapshotOrigin::FullBaseline => (latest.map_or(0, |g| g + 1), None, None),
            SnapshotOrigin::Incremental => {
                let parent_generation = latest.ok_or_else(|| DbError::InvalidArgument("cannot publish an incremental snapshot with no prior generation".to_string()))?;
                let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
                let control = SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), db_io_core::DB_IO_OPERATION_PAGES * 8)?;
                let mut cursor = self.chain_cursor(document, parent_generation, control);
                let mut combined = cursor.materialize_pages().await?;
                let parent_footer_position = combined.len() as u64 - pack::FOOTER_SIZE as u64;
                let _ = combined.close_step()?;
                drop(combined);
                let _ = cursor.close_step()?;
                (parent_generation + 1, Some(parent_generation), Some(parent_footer_position))
            }
        };

        let mut hashes = Vec::with_capacity(new_pages.len());
        for index in 0..new_pages.len() {
            hashes.push(new_pages.page(index).ok_or_else(|| DbError::Internal("snapshot retained page source lost an owner".to_string()))?.hash);
        }
        let descriptor = SnapshotDescriptor {
            document: document.clone(),
            generation,
            parent_generation,
            head_seq: body.head_seq,
            commit_seq: body.commit_seq,
            epoch: body.epoch,
            chain_hash: body.chain_hash,
            protocol_version: body.protocol_version,
            base_pack_hash: body.base_pack_hash,
            roots: body.roots,
            new_pages: hashes,
            created_at_ms: body.created_at_ms,
        };
        let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let pages = build_generation_page_source(&descriptor, new_pages, parent_footer_position, &mut control).await?;
        self.storage.write_generation(document, generation, pages).await?;
        Ok(generation)
    }

    /// 🥇️ The document's latest generation number and descriptor, or `None` if it has no
    /// snapshot yet.
    pub async fn load_latest(&self, document: &ArtifactId) -> Result<Option<(u64, SnapshotDescriptor)>, DbError> {
        match self.storage.latest_generation(document).await? {
            None => Ok(None),
            Some(generation) => {
                let mut bytes = self.storage.read_generation(document, generation).await?;
                let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
                let handle = open_latest_pages(&bytes, &mut control).await?;
                let descriptor = handle.descriptor;
                let _ = bytes.close_step()?;
                drop(bytes);
                Ok(Some((generation, descriptor)))
            }
        }
    }

    /// 🎯️ Selection: the highest-numbered generation whose `head_seq` does not exceed
    /// `at_most_head_seq`, or `None` if no generation qualifies — the snapshot a materializer
    /// should start replaying the WAL suffix from for a point-in-time read.
    pub async fn select_generation(&self, document: &ArtifactId, at_most_head_seq: u64) -> Result<Option<u64>, DbError> {
        let mut best: Option<(u64, u64)> = None;
        let mut generations = self.storage.list_generations(document).await?;
        let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        for generation in generations.as_slice() {
            let mut bytes = self.storage.read_generation(document, *generation).await?;
            let handle = open_latest_pages(&bytes, &mut control).await?;
            if handle.descriptor.head_seq <= at_most_head_seq {
                let better = best.is_none_or(|(_, best_head_seq)| handle.descriptor.head_seq > best_head_seq);
                if better {
                    best = Some((*generation, handle.descriptor.head_seq));
                }
            }
            let _ = bytes.close_step()?;
            drop(bytes);
        }
        let _ = generations.close_step();
        drop(generations);
        Ok(best.map(|(generation, _)| generation))
    }

    /// 🗑️ Deletes every generation strictly below `floor_generation`. `floor_generation`
    /// must itself be a full baseline (`parent_generation.is_none()`) — see module doc's scope
    /// boundary on why an incremental floor is rejected rather than silently breaking its chain.
    pub async fn retain_from(&self, document: &ArtifactId, floor_generation: u64) -> Result<(), DbError> {
        let mut floor_bytes = self.storage.read_generation(document, floor_generation).await?;
        let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let floor_handle = open_latest_pages(&floor_bytes, &mut control).await?;
        let incremental = floor_handle.descriptor.parent_generation.is_some();
        let _ = floor_bytes.close_step()?;
        drop(floor_bytes);
        if incremental {
            return Err(DbError::InvalidArgument("retention floor must be a full-baseline generation (no parent)".to_string()));
        }
        let mut generations = self.storage.list_generations(document).await?;
        for generation in generations.as_slice() {
            if *generation < floor_generation {
                self.storage.delete_generation(document, *generation).await?;
            }
        }
        let _ = generations.close_step();
        drop(generations);
        Ok(())
    }

    /// 🔬️ Verifies generation `generation` decodes cleanly at `level`: the `KIND_SNAPSHOT`
    /// descriptor round-trips (`open_latest` itself decodes it) and every declared local chunk
    /// (`descriptor.new_pages`) decodes — at `VerificationLevel::Full` this transitively checks
    /// each chunk's content hash too, since `pack::PackFile::read_chunk` already validates it
    /// against the chunk table `pack::PackWriter::write_chunk` built from these same page bytes;
    /// no separate hash recomputation needed here.
    pub async fn verify(&self, document: &ArtifactId, generation: u64, level: pack::os_pack::VerificationLevel) -> Result<(), DbError> {
        let bytes = self.storage.read_generation(document, generation).await?;
        let mut control = SnapshotCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let handle = open_latest_pages(&bytes, &mut control).await?;
        let sub = PageSubSource { inner: &bytes, base: 0, len: bytes.len() as u64 };
        let file = pack::PackFile::open_manifest(sub, &pack::PackLimits::default(), level).await?;
        for index in 0..handle.descriptor.new_pages.len() {
            let mut cursor = file.identity_chunk_cursor(pack::ChunkId(index as u32), level)?;
            let mut fragment = [0u8; db_io_core::DB_IO_PAGE_BYTES];
            while cursor.read_fragment(&mut fragment).await? != 0 {
                control.grant()?;
            }
        }
        drop(file);
        drop(bytes);
        Ok(())
    }
}
//#endregion 🔖️Manager

//#region 🔖️Lease
/// ⏳️ The fencing primitive the module doc's "Scope boundary" note references: this crate
/// deliberately keeps `SnapshotManager::publish`/`retain_from` as mechanical, lease-agnostic
/// operations (concurrency coordination is `db_compact`'s "online compaction with manifest CAS +
/// fencing" responsibility) — `SnapshotLease` is the thin `db_storage::LeaseStorage` wrapper a
/// caller (a document actor, or `db_compact`) uses to serialize concurrent `publish`/`retain_from`
/// calls for one document before invoking them, keyed by a resource name derived from the document
/// id alone (so two different documents' snapshot builders never contend on the same lease).
pub struct SnapshotLease;

impl SnapshotLease {
    /// 🏷️ The `LeaseStorage` resource name guarding `document`'s snapshot builder.
    // 🚫️async: E1 pure accessor consumed synchronously by `acquire`/`renew`/`release`/`current` — see R9
    pub fn resource(document: &ArtifactId) -> String {
        format!("snapshot:{document}")
    }

    /// 🤝️ Acquires (or idempotently re-acquires) the snapshot-builder lease for `document`.
    pub async fn acquire(storage: &impl LeaseStorage, document: &ArtifactId, holder: &str, ttl_ms: u64, now_ms: u64) -> Result<EpochFence, DbError> {
        storage.acquire(&Self::resource(document), holder, ttl_ms, now_ms).await
    }

    /// ♻️ Extends `holder`'s existing lease for `document` — e.g. around a long
    /// retained chain cursor + `publish` sequence for a deep incremental chain.
    pub async fn renew(storage: &impl LeaseStorage, document: &ArtifactId, holder: &str, fence: EpochFence, ttl_ms: u64, now_ms: u64) -> Result<(), DbError> {
        storage.renew(&Self::resource(document), holder, fence, ttl_ms, now_ms).await
    }

    /// 🕊️ Releases `holder`'s lease for `document` once its `publish`/`retain_from` call
    /// has completed.
    pub async fn release(storage: &impl LeaseStorage, document: &ArtifactId, holder: &str, fence: EpochFence) -> Result<(), DbError> {
        storage.release(&Self::resource(document), holder, fence).await
    }

    /// 👀️ The lease's current holder/fence for `document`, or `None` if unheld — lets a
    /// caller check whether it's safe to `retain_from` without blindly racing another builder.
    pub async fn current(storage: &impl LeaseStorage, document: &ArtifactId, now_ms: u64) -> Result<Option<LeaseInfo>, DbError> {
        storage.current(&Self::resource(document), now_ms).await
    }
}
//#endregion 🔖️Lease
