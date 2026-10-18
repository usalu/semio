//! 🛠️ Database catalog, shutdown and hello: the document creation, the catalog and health queries, the shutdown steps, the compaction and the sync hello admission over the emit runtime. See [`super`].

use super::*;

//#region 🔖️Database
impl<E: Emit + 'static> Database<E> {
    /// 🪴️ Admits the exact create-document catalog transaction before any catalog owner is copied.
    pub fn create_document_catalog_retained(&self, document: protocol::ArtifactId) -> Result<DatabaseCreateCatalogFuture, DatabaseRetainedActivityRejected<DatabaseCreateCatalogRejected>> {
        let pool_use = self.require_open_use().map_err(DatabaseRetainedActivityRejected::Closed)?;
        DatabaseCreateCatalogFuture::try_prepare_with_use(self.pool.clone(), pool_use, self.catalog.clone(), self.storage.clone(), document, true).map_err(DatabaseRetainedActivityRejected::Retained)
    }

    /// 🌱️ The frozen `create_document`: mints a brand-new document, durably records it in the
    /// catalog root (CAS-fenced), spawns its `ArtifactAuthority`, and returns a live handle.
    pub async fn create_document(&self, spec: ArtifactSpec) -> Result<ArtifactHandle, DatabaseDocumentOpenRejected> {
        self.mount_document(spec.document, DatabaseDocumentMountPolicy::Create).await
    }

    /// 🪢️ Joins or installs the exact retained first-mount owner for one document.
    pub async fn ensure_document(&self, id: &protocol::ArtifactId) -> Result<ArtifactHandle, DatabaseDocumentOpenRejected> {
        self.mount_document(id.clone(), DatabaseDocumentMountPolicy::Ensure).await
    }

    /// 📄️ The frozen `document`: returns a live handle to an already-cataloged document,
    /// reusing an already-open `ArtifactAuthority` if one exists, else recovering it fresh from its
    /// WAL.
    pub async fn document(&self, id: &protocol::ArtifactId) -> Result<ArtifactHandle, DatabaseDocumentOpenRejected> {
        self.mount_document(id.clone(), DatabaseDocumentMountPolicy::Open).await
    }

    /// 📇️ The frozen `catalog`: a point-in-time read of every document this `Database`
    /// knows about.
    pub async fn catalog(&self) -> CatalogView {
        let entries = {
            let catalog = self.catalog.lock().expect("db_engine: catalog mutex poisoned");
            Arc::clone(&catalog.entries)
        };
        CatalogView { artifacts: entries.as_ref().clone() }
    }

    /// 🩺️ The frozen `health`: this `Database`'s `HealthRegistry` snapshot plus its own open
    /// document count.
    pub async fn health(&self) -> DbHealth {
        DbHealth { report: self.health.report(), open_artifacts: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").ready_count() }
    }

    /// 🚪️ Advances at most one retained shutdown owner. Cancellation, deadline, and every error
    /// return leave the exact authority mounted on this Database for retry.
    pub async fn shutdown_step(&mut self, control: &DatabaseShutdownControl) -> Result<DatabaseShutdownProgress, DbError> {
        if self.shutdown_complete {
            return Ok(DatabaseShutdownProgress::Complete);
        }
        if control.interrupted() {
            return Ok(DatabaseShutdownProgress::Interrupted);
        }
        if let Some((_, authority)) = self.closing_authority.as_ref() {
            if !authority.shutdown_step() {
                if let Some(progress) = authority.close_retry_progress().filter(db_artifact::ArtifactCloseRetryProgress::is_blocked) {
                    return Ok(DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::ArtifactClose(progress)));
                }
                return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() + 1 });
            }
            self.closing_authority.take();
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() });
        }
        let closing = {
            let mut registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            let document = registry.slots.iter().find_map(|(document, slot)| matches!(slot, DatabaseDocumentMountSlot::Closing { .. }).then(|| document.clone()));
            document.and_then(|document| registry.slots.remove(&document).map(|slot| (document, slot)))
        };
        if let Some((document, slot)) = closing {
            let DatabaseDocumentMountSlot::Closing { mut terminal } = slot else { return Err(DbError::Internal("selected closing mount was not closing".to_string())) };
            let retired = std::future::poll_fn(|context| std::task::Poll::Ready(std::pin::Pin::new(&mut terminal).poll(context).is_ready())).await;
            if !retired {
                self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").slots.insert(document, DatabaseDocumentMountSlot::Closing { terminal });
                semio_framework_async::yield_once().await;
            }
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() });
        }
        let opening = {
            let registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            registry.slots.values().find_map(|slot| match slot {
                DatabaseDocumentMountSlot::Opening { owner, .. } => Some(owner.clone()),
                DatabaseDocumentMountSlot::Ready(_) | DatabaseDocumentMountSlot::Closing { .. } => None,
            })
        };
        if let Some(owner) = opening {
            if let Some(kind) = owner.scheduler_fault() {
                return Ok(DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Executor(kind)));
            }
            owner.drive();
            semio_framework_async::yield_once().await;
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() });
        }
        let next = {
            let registry = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned");
            registry
                .slots
                .iter()
                .filter_map(|(document, slot)| match slot {
                    DatabaseDocumentMountSlot::Ready(completion) if completion.handles.load(std::sync::atomic::Ordering::Acquire) == 0 => Some(document.clone()),
                    _ => None,
                })
                .min()
        };
        if let Some(document) = next {
            let slot = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").slots.remove(&document).expect("selected shutdown authority disappeared");
            let DatabaseDocumentMountSlot::Ready(completion) = slot else { return Err(DbError::Internal("selected shutdown mount was not ready".to_string())) };
            self.closing_authority = Some((document, completion.authority));
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Authority, remaining_authorities: self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len() + 1 });
        }
        let shared_authorities = self.open_artifacts.lock().expect("db_engine: open_artifacts mutex poisoned").len();
        if shared_authorities != 0 {
            return Ok(DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities(shared_authorities)));
        }
        if !self.shutdown_emit_started {
            self.shutdown_emit_started = true;
            self.emit.emit(EmitEvent::new("db_engine.database_shutdown")).await;
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::Emit, remaining_authorities: 0 });
        }
        if self.pool_use.as_ref().is_some_and(|pool_use| Arc::strong_count(pool_use) != 1) {
            if let Some(progress) = db_artifact::artifact_close_retry_progress(&self.pool).filter(db_artifact::ArtifactCloseRetryProgress::is_blocked) {
                return Ok(DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::ArtifactClose(progress)));
            }
            return Ok(DatabaseShutdownProgress::Progress { phase: DatabaseShutdownPhase::PoolUse, remaining_authorities: 0 });
        }
        self.pool_use.take();
        self.shutdown_complete = true;
        Ok(DatabaseShutdownProgress::Complete)
    }

    /// 🔂️ Grants every faulted artifact close this Database still retires (its closing authority and the
    /// dropped authorities retiring on its pool) a fresh bounded retry budget.
    pub fn readmit_artifact_close_retries(&self) -> usize {
        let closing = self.closing_authority.as_ref().is_some_and(|(_, authority)| authority.readmit_close_retry());
        usize::from(closing) + db_artifact::artifact_close_retry_readmit(&self.pool)
    }

    /// 🛑️ Stops the automatic retries of every faulted artifact close this Database still retires.
    pub fn cancel_artifact_close_retries(&self) -> usize {
        let closing = self.closing_authority.as_ref().is_some_and(|(_, authority)| authority.cancel_close_retry());
        usize::from(closing) + db_artifact::artifact_close_retry_cancel(&self.pool)
    }

    /// 🛬️ Drives bounded shutdown steps to a terminal acknowledgement while borrowing the exact
    /// Database. Error and future cancellation therefore preserve caller retry authority. Each call
    /// is the product retry owner of faulted artifact closes: it re-admits exhausted or cancelled
    /// ones once, lets the pool timer drive their bounded backoff, reports an exhausted budget as a
    /// terminal error and cancels the retries when the caller cancels.
    pub async fn shutdown(&mut self, control: &DatabaseShutdownControl) -> Result<(), DbError> {
        self.readmit_artifact_close_retries();
        let mut reached = None;
        loop {
            match self.shutdown_step(control).await? {
                DatabaseShutdownProgress::Progress { phase, .. } => {
                    reached = Some(phase);
                    semio_framework_async::yield_once().await;
                }
                DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Authorities(count)) => {
                    return Err(DbError::Conflict(format!("database shutdown retains {count} shared artifact authorities")));
                }
                DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::Executor(kind)) => {
                    return Err(DbError::Unavailable(format!("database shutdown retains a non-runnable document-mount job: {kind:?}")));
                }
                DatabaseShutdownProgress::Blocked(DatabaseShutdownBlock::ArtifactClose(progress)) => {
                    return Err(DbError::Unavailable(format!("database shutdown retains a faulted artifact close after {} of {} retries ({:?})", progress.attempts, progress.limit, progress.state)));
                }
                DatabaseShutdownProgress::Interrupted => {
                    if control.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                        self.cancel_artifact_close_retries();
                    }
                    return Err(control.interruption_error(DatabaseShutdownInterruptionWitness {
                        phase: reached,
                        open_artifacts: self.open_artifacts.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len(),
                        emit_started: self.shutdown_emit_started,
                        pool_use_owners: self.pool_use.as_ref().map_or(0, Arc::strong_count),
                        retained_slots: DatabaseRetainedPoolUseCensus::observe(),
                    }));
                }
                DatabaseShutdownProgress::Complete => return Ok(()),
            }
        }
    }

    /// 🧰️ What this `Database` instance negotiated at `open` time.
    pub async fn capabilities(&self) -> DbCapabilities {
        self.capabilities
    }

    /// 🔌️ The underlying storage substrate this `Database` was opened with — an escape
    /// hatch for callers below the document-actor boundary that need direct `PayloadStorage`/
    /// `WalStorage` access (e.g. `os-semio_hub`'s content-addressed blob routes, or a wire-v2 semio_hub
    /// session driving `db_sync::handle_frontier_advertise` directly). Additive: not part of the
    /// contract-frozen `Database` API surface listed in `contract.md`'s "Stable API" block, so it
    /// carries no compatibility promise beyond this crate's own semver.
    pub async fn storage(&self) -> Arc<db_storage::DbBackend> {
        self.storage.clone()
    }

    /// 🧵️ Queues compaction on the document actor that already owns its WAL writer.
    pub async fn compact_document_retained(
        &self,
        document: &protocol::ArtifactId,
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<ArtifactCompactionFuture, DatabaseDocumentOpenRejected> {
        Ok(self.document(document).await?.compact_retained(holder, consolidate_snapshots, db_compact::CompactionBudget::default(), now_ms, cancelled))
    }

    /// 🧹️ Mounts one retained compaction authority and awaits only its terminal witness.
    pub async fn compact_document(&self, document: &protocol::ArtifactId, holder: &str, consolidate_snapshots: bool) -> Result<db_compact::CompactionReport, DatabaseDocumentOpenRejected> {
        let holder = db_io_core::DbIoText::try_from_str(holder)?;
        let requested_at_ms = now_ms().await;
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let result = self.compact_document_retained(document, holder, consolidate_snapshots, requested_at_ms, cancelled).await?.await.map_err(DatabaseDocumentOpenRejected::Database)?;
        result.map_err(DatabaseDocumentOpenRejected::Database)
    }

    /// 👋️ Pre-admits the exact sync-hello owners before mounting shared-pool I/O work.
    pub fn hello_retained(
        &self,
        document: ArtifactId,
        hello_frontier: Option<protocol::RuntimeFrontierSummary>,
        session_id: String,
        origin: protocol::ActorId,
        snapshot_chunk_bytes: usize,
    ) -> Result<db_sync::DatabaseSyncHelloFuture, DatabaseRetainedActivityRejected<db_sync::DatabaseSyncHelloRejected>> {
        let pool_use = self.require_open_use().map_err(DatabaseRetainedActivityRejected::Closed)?;
        db_sync::DatabaseSyncHelloFuture::try_submit_with_use(self.pool.clone(), pool_use, self.storage.clone(), document, hello_frontier, session_id, origin, snapshot_chunk_bytes).map_err(DatabaseRetainedActivityRejected::Retained)
    }

    /// 📡️ Mounts one retained hello authority and returns its backpressured frame session. A hello
    /// that finds every admission slot taken waits for one (at most [`DATABASE_HELLO_ADMISSION_WAIT_MS`])
    /// instead of refusing its socket; any other refusal is answered at once.
    pub async fn hello(
        &self,
        document: protocol::ArtifactId,
        hello_frontier: Option<protocol::RuntimeFrontierSummary>,
        session_id: String,
        origin: protocol::ActorId,
        snapshot_chunk_bytes: usize,
    ) -> Result<db_sync::DatabaseSyncHelloSession, DbError> {
        let document = to_core_document_id(&document).await;
        let deadline_ms = self.pool.now_ms().saturating_add(DATABASE_HELLO_ADMISSION_WAIT_MS);
        loop {
            let hello = match self.hello_retained(document.clone(), hello_frontier.clone(), session_id.clone(), origin.clone(), snapshot_chunk_bytes) {
                Ok(hello) => hello,
                Err(DatabaseRetainedActivityRejected::Closed(error)) => return Err(error),
                Err(DatabaseRetainedActivityRejected::Retained(rejected)) => {
                    let saturated = rejected.admission_saturated();
                    let error = rejected.close_and_take_error();
                    if !saturated {
                        return Err(error);
                    }
                    db_sync::DatabaseSyncHelloAdmissionReady::new(self.pool.clone(), deadline_ms).await?;
                    continue;
                }
            };
            return hello.await?.close_and_take_session();
        }
    }
}

/// ⏳️ How long [`Database::hello`] waits for a sync-hello admission slot before it answers the
/// refusal: the hub's document-socket frame deadline.
pub const DATABASE_HELLO_ADMISSION_WAIT_MS: u64 = 30_000;
//#endregion 🔖️Database
