//! 📤️ Future, terminal and state drop: the create-catalog future, its terminal handle, the live counter and the state drop. See [`super`].

use super::*;

//#region 🔖️CreateDocumentCatalogCas
pub struct DatabaseCreateCatalogFuture {
    pub state: Arc<DatabaseCreateCatalogState>,
    pub resolved: bool,
}

impl DatabaseCreateCatalogFuture {
    pub fn try_submit(pool: Arc<WorkerPool>, catalog: Arc<Mutex<CatalogState>>, storage: Arc<db_storage::DbBackend>, document: protocol::ArtifactId) -> Result<Self, DatabaseCreateCatalogRejected> {
        Self::try_prepare(pool, catalog, storage, document, true)
    }

    pub fn try_prepare(pool: Arc<WorkerPool>, catalog: Arc<Mutex<CatalogState>>, storage: Arc<db_storage::DbBackend>, document: protocol::ArtifactId, schedule: bool) -> Result<Self, DatabaseCreateCatalogRejected> {
        let pool_use = match pool.acquire_use() {
            Ok(pool_use) => pool_use,
            Err(error) => return Err(DatabaseCreateCatalogRejected::new(pool, DbError::Unavailable(format!("database create-catalog WorkerPool use rejected: {error:?}")), storage, document)),
        };
        Self::try_prepare_with_use(pool, pool_use, catalog, storage, document, schedule)
    }

    pub fn try_prepare_with_use(pool: Arc<WorkerPool>, pool_use: Arc<WorkerPoolUse>, catalog: Arc<Mutex<CatalogState>>, storage: Arc<db_storage::DbBackend>, document: protocol::ArtifactId, schedule: bool) -> Result<Self, DatabaseCreateCatalogRejected> {
        let admission = match DatabaseCreateCatalogAdmission::try_claim(&document) {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseCreateCatalogRejected::new(pool, error, storage, document)),
        };
        let slot = admission.slot;
        let generation = admission.generation;
        let (base, base_epoch, base_revision, backing) = {
            let catalog = catalog.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if catalog.entries.len() >= DATABASE_CREATE_CATALOG_MAX_ENTRIES {
                drop(catalog);
                drop(admission);
                return Err(DatabaseCreateCatalogRejected::new(pool, DbError::LimitExceeded("database create-catalog entry capacity"), storage, document));
            }
            let backing = match DatabaseCreateCatalogBackingLedger::new(document.0.capacity(), catalog.entries.capacity()) {
                Ok(backing) => backing,
                Err(error) => {
                    drop(catalog);
                    drop(admission);
                    return Err(DatabaseCreateCatalogRejected::new(pool, error, storage, document));
                }
            };
            (Arc::clone(&catalog.entries), catalog.epoch, catalog.revision, backing)
        };
        if database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot].is_some() {
            drop(admission);
            return Err(DatabaseCreateCatalogRejected::new(pool, DbError::LimitExceeded("database create-catalog terminal slot"), storage, document));
        }
        let base_identity = Arc::as_ptr(&base) as usize;
        let created_at_ms = pool.now_ms();
        let deadline_ms = created_at_ms.checked_add(DATABASE_CREATE_CATALOG_DEADLINE_MS).unwrap_or(u64::MAX);
        DATABASE_CREATE_CATALOG_LIVE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let state = Arc::new(DatabaseCreateCatalogState {
            pool,
            pool_use: Mutex::new(Some(pool_use)),
            catalog,
            slot,
            generation,
            admission: Mutex::new(Some(admission)),
            storage: Mutex::new(Some(storage)),
            document: Mutex::new(Some(document)),
            cursor: Mutex::new(DatabaseCreateCatalogCursor {
                base: Some(base),
                backing,
                base_epoch,
                base_revision,
                base_identity,
                created_at_ms,
                encoded_len: 2,
                scan_entry: 0,
                scan_byte: 0,
                scan_started: false,
                candidate: None,
                clone_entry: 0,
                clone_byte: 0,
                clone_text: None,
                snapshot: None,
                encode: DatabaseCreateCatalogEncodeCursor::new(),
                writer: None,
                pages: None,
            }),
            work: Mutex::new(None),
            poll_work: Mutex::new(None),
            terminal_work: Mutex::new(None),
            outcome: Mutex::new(None),
            completion: Mutex::new(None),
            terminal_completion: Mutex::new(None),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            waker: Mutex::new(None),
            driver_authority: std::sync::atomic::AtomicU8::new(DatabaseCreateCatalogDriverAuthority::Idle as u8),
            polling: std::sync::atomic::AtomicBool::new(false),
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            closing: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            pending_owned: std::sync::atomic::AtomicBool::new(false),
            terminal_checked_out: std::sync::atomic::AtomicBool::new(false),
            retry_closing: std::sync::atomic::AtomicBool::new(false),
            callback_close_armed: std::sync::atomic::AtomicBool::new(false),
            catalog_contention_armed: std::sync::atomic::AtomicBool::new(false),
            phase: std::sync::atomic::AtomicU8::new(DatabaseCreateCatalogPhase::Scan as u8),
            progress: std::sync::atomic::AtomicU8::new(DatabaseCreateCatalogProgress::Admitted as u8),
            deadline_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            opportunities: std::sync::atomic::AtomicU64::new(0),
            #[cfg(any(test, feature = "testkit"))]
            active_drivers: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(any(test, feature = "testkit"))]
            max_active_drivers: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(any(test, feature = "testkit"))]
            poll_worker_thread: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            spent_submission_attempts: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            backend_polls: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            callback_worker_thread: std::sync::atomic::AtomicBool::new(false),
            #[cfg(test)]
            terminal_job_retirements: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            controlled_capacity_overage: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            controlled_driver_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_publication_before_waker_hook: Mutex::new(None),
        });
        database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        if schedule {
            state.schedule();
        }
        Ok(Self { state, resolved: false })
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> DatabaseCreateCatalogProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => DatabaseCreateCatalogProgress::Admitted,
            1 => DatabaseCreateCatalogProgress::Scanning,
            2 => DatabaseCreateCatalogProgress::Copying,
            3 => DatabaseCreateCatalogProgress::Encoding,
            4 => DatabaseCreateCatalogProgress::Polling,
            5 => DatabaseCreateCatalogProgress::Revalidating,
            6 => DatabaseCreateCatalogProgress::Completed,
            7 => DatabaseCreateCatalogProgress::Cancelled,
            _ => DatabaseCreateCatalogProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        if self.state.phase() == DatabaseCreateCatalogPhase::Terminal {
            self.state.begin_callback_close();
        } else {
            self.state.schedule();
        }
    }
}

impl Future for DatabaseCreateCatalogFuture {
    type Output = Result<DatabaseCreateCatalogResult, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let completion = { self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() };
        if let Some(result) = completion {
            self.resolved = true;
            return std::task::Poll::Ready(result);
        }
        #[cfg(test)]
        if let Some(hook) = self.state.controlled_publication_before_waker_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook();
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let completion = { self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() };
        if let Some(result) = completion {
            self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.resolved = true;
            return std::task::Poll::Ready(result);
        }
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseCreateCatalogFuture {
    fn drop(&mut self) {
        if self.resolved {
            return;
        }
        self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.state.schedule();
    }
}

pub struct DatabaseCreateCatalogTerminalHandle {
    pub state: Arc<DatabaseCreateCatalogState>,
}

impl DatabaseCreateCatalogTerminalHandle {
    pub fn witness(&self) -> DatabaseCreateCatalogTerminalWitness {
        DatabaseCreateCatalogTerminalWitness { generation: self.state.generation, retained_owners: self.state.retained_owner_count(), terminal_empty: self.state.terminal_is_empty() }
    }

    pub fn close_step(&self) -> DatabaseCreateCatalogCloseStep {
        if self.state.terminal_is_empty() {
            return DatabaseCreateCatalogCloseStep::Complete;
        }
        self.state.closing.store(true, std::sync::atomic::Ordering::Release);
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        let authority = self.state.driver_authority.load(std::sync::atomic::Ordering::Acquire);
        if authority == DatabaseCreateCatalogDriverAuthority::Driving as u8 || authority == DatabaseCreateCatalogDriverAuthority::Retry as u8 {
            DatabaseCreateCatalogCloseStep::Blocked
        } else if self.state.drive_explicit_close_one() {
            if self.state.terminal_is_empty() {
                DatabaseCreateCatalogCloseStep::Complete
            } else {
                DatabaseCreateCatalogCloseStep::Progress
            }
        } else {
            self.state.begin_callback_close();
            DatabaseCreateCatalogCloseStep::Blocked
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty()
    }
}

impl Drop for DatabaseCreateCatalogTerminalHandle {
    fn drop(&mut self) {
        self.state.terminal_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

pub fn take_database_create_catalog_terminal(generation: u64) -> Option<DatabaseCreateCatalogTerminalHandle> {
    use std::sync::atomic::Ordering;
    let registry = database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let state = registry.iter().filter_map(Option::as_ref).find(|state| state.generation == generation && state.abandoned.load(Ordering::Acquire))?.clone();
    if state.terminal_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
        return None;
    }
    Some(DatabaseCreateCatalogTerminalHandle { state })
}
//#endregion 🔖️CreateDocumentCatalogCas

pub static DATABASE_CREATE_CATALOG_LIVE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl Drop for DatabaseCreateCatalogState {
    fn drop(&mut self) {
        DATABASE_CREATE_CATALOG_LIVE.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
    }
}
