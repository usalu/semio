//! 📬️ Hello future, result and session: the hello future, the result, the returned frame with its credit and lease, the session and the next future. See [`super`].

use super::*;

//#region 🔖️Hello
//#region 👋️RetainedHello
/// 👋️ Retained generation-qualified sync-hello future driven only by the shared I/O lane.
pub struct DatabaseSyncHelloFuture {
    pub state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
    completed: bool,
}

impl std::fmt::Debug for DatabaseSyncHelloFuture {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseSyncHelloFuture").field("generation", &self.state.as_ref().map(|state| state.generation)).field("completed", &self.completed).finish()
    }
}

impl DatabaseSyncHelloFuture {
    pub fn try_submit(
        pool: std::sync::Arc<semio_framework_async::WorkerPool>,
        storage: std::sync::Arc<db_storage::DbBackend>,
        document: ArtifactId,
        hello_frontier: Option<protocol::RuntimeFrontierSummary>,
        session_id: String,
        origin: protocol::ActorId,
        snapshot_chunk_bytes: usize,
    ) -> Result<Self, DatabaseSyncHelloRejected> {
        let pool_use = match pool.acquire_use() {
            Ok(pool_use) => pool_use,
            Err(error) => {
                let owners = DatabaseSyncHelloOwners { storage: Some(storage), document, hello_frontier, session_id, origin, snapshot_chunk_bytes };
                return Err(DatabaseSyncHelloRejected::new(pool, DbError::Unavailable(format!("database sync-hello WorkerPool use rejected: {error:?}")), owners));
            }
        };
        Self::try_submit_with_use(pool, pool_use, storage, document, hello_frontier, session_id, origin, snapshot_chunk_bytes)
    }

    pub fn try_submit_with_use(
        pool: std::sync::Arc<semio_framework_async::WorkerPool>,
        pool_use: std::sync::Arc<semio_framework_async::WorkerPoolUse>,
        storage: std::sync::Arc<db_storage::DbBackend>,
        document: ArtifactId,
        hello_frontier: Option<protocol::RuntimeFrontierSummary>,
        session_id: String,
        origin: protocol::ActorId,
        snapshot_chunk_bytes: usize,
    ) -> Result<Self, DatabaseSyncHelloRejected> {
        let owners = DatabaseSyncHelloOwners { storage: Some(storage), document, hello_frontier, session_id, origin, snapshot_chunk_bytes };
        let (input_items, input_bytes) = match database_sync_hello_input_credit(&owners) {
            Ok(credit) => credit,
            Err(error) => return Err(DatabaseSyncHelloRejected::new(pool, error, owners)),
        };
        let admission = match DatabaseSyncHelloAdmission::try_claim(input_items, input_bytes) {
            Ok(admission) => admission,
            Err(error) => return Err(DatabaseSyncHelloRejected::new(pool, error, owners)),
        };
        let slot = admission.slot;
        let generation = admission.generation;
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let expired = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let progress = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(DatabaseSyncHelloProgress::Admitted as u8));
        let future = Box::pin(database_sync_hello_execute(owners, cancelled.clone(), expired.clone(), progress.clone()));
        let deadline_ms = pool.now_ms().saturating_add(DATABASE_SYNC_HELLO_DEADLINE_MS);
        DATABASE_SYNC_HELLO_STATE_LIVE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let state = std::sync::Arc::new(DatabaseSyncHelloState {
            pool: pool.clone(),
            _pool_use: pool_use,
            slot,
            generation,
            admission: std::sync::Mutex::new(Some(admission)),
            core: std::sync::Mutex::new(DatabaseSyncHelloCore { future: Some(future), execution: None, frame: None, frame_close: None, returned_frame: None, returned_fallback: None, quarantined: None }),
            driver: std::sync::atomic::AtomicU8::new(DatabaseSyncHelloDriverAuthority::Idle as u8),
            retry_job: std::sync::Mutex::new(None),
            cancelled,
            expired,
            progress_deadline_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            deadline_armed_ms: std::sync::atomic::AtomicU64::new(deadline_ms),
            progress,
            wake_requested: std::sync::atomic::AtomicBool::new(false),
            demand: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            close_armed: std::sync::atomic::AtomicBool::new(false),
            close_requested: std::sync::atomic::AtomicBool::new(false),
            waker: std::sync::Mutex::new(None),
            returned_generation: std::sync::atomic::AtomicU64::new(1),
        });
        database_sync_hello_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner)[slot] = Some(state.clone());
        let deadline = std::sync::Arc::downgrade(&state);
        pool.callback_at(deadline_ms, move || {
            if let Some(state) = deadline.upgrade() {
                state.deadline_callback();
            }
        });
        state.schedule();
        Ok(Self { state: Some(state), completed: false })
    }

    pub fn cancel(&self) {
        if let Some(state) = self.state.as_ref() {
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        }
    }

    pub fn progress(&self) -> DatabaseSyncHelloProgress {
        self.state.as_ref().map_or(DatabaseSyncHelloProgress::Fault, |state| state.progress())
    }
}

impl std::future::Future for DatabaseSyncHelloFuture {
    type Output = Result<DatabaseSyncHelloResult, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed {
            return std::task::Poll::Ready(Err(DbError::Closed));
        }
        let Some(state) = self.state.as_ref().cloned() else { return std::task::Poll::Ready(Err(DbError::Closed)) };
        if !state.current() {
            self.completed = true;
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
            return std::task::Poll::Ready(Err(DbError::StaleGeneration { expected: GenerationId(state.generation), actual: GenerationId(0) }));
        }
        if state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).execution.is_some() {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Ok(DatabaseSyncHelloResult { state: Some(state) }));
        }
        *state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        if state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).execution.is_some() {
            self.completed = true;
            self.state.take();
            return std::task::Poll::Ready(Ok(DatabaseSyncHelloResult { state: Some(state) }));
        }
        state.schedule();
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseSyncHelloFuture {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(state) = self.state.take() {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.schedule();
        }
    }
}

/// 🧾️ Terminal hello witness converted to a retained frame session or closed.
pub struct DatabaseSyncHelloResult {
    pub(super) state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
}

impl DatabaseSyncHelloResult {
    pub fn close_and_take_session(mut self) -> Result<DatabaseSyncHelloSession, DbError> {
        let Some(state) = self.state.take() else { return Err(DbError::Closed) };
        let error = {
            let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let Some(execution) = core.execution.as_mut() else { return Err(DbError::Closed) };
            if execution.prepared.is_err() {
                match std::mem::replace(&mut execution.prepared, Err(DbError::Closed)) {
                    Err(error) => Some(error),
                    Ok(prepared) => {
                        execution.prepared = Ok(prepared);
                        None
                    }
                }
            } else {
                None
            }
        };
        if let Some(error) = error {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
            return Err(error);
        }
        Ok(DatabaseSyncHelloSession { state: Some(state), welcome_taken: false })
    }
}

impl Drop for DatabaseSyncHelloResult {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
        }
    }
}

pub fn database_sync_hello_returned_frame_credit(frame: &protocol::ServerFrame) -> Result<(usize, usize), DbError> {
    match frame {
        protocol::ServerFrame::SnapshotChunk { bytes, .. } => Ok((usize::from(bytes.backing_bytes() != 0), bytes.backing_bytes())),
        protocol::ServerFrame::Commands { envelopes, origin, frontier } => {
            let label = "database sync hello tail frame backing";
            let shell = envelopes.capacity().checked_mul(size_of::<protocol::MutationEnvelope>()).ok_or(DbError::LimitExceeded(label))?;
            let mut items = 0usize;
            let mut bytes = 0usize;
            for capacity in [shell, origin.0.original_allocation_bytes(), frontier.document_id.0.capacity(), frontier.head_edit_id.capacity()] {
                if capacity != 0 {
                    items = items.checked_add(1).ok_or(DbError::LimitExceeded(label))?;
                    bytes = bytes.checked_add(capacity).ok_or(DbError::LimitExceeded(label))?;
                }
            }
            for envelope in envelopes {
                let (envelope_items, envelope_bytes) = database_sync_hello_envelope_credit(envelope)?;
                items = items.checked_add(envelope_items).ok_or(DbError::LimitExceeded(label))?;
                bytes = bytes.checked_add(envelope_bytes).ok_or(DbError::LimitExceeded(label))?;
            }
            Ok((items, bytes))
        }
        _ => Ok((0, 0)),
    }
}

fn database_sync_hello_lease_frame(state: &std::sync::Arc<DatabaseSyncHelloState>, core: &mut DatabaseSyncHelloCore, frame: protocol::ServerFrame) -> Result<DatabaseSyncHelloReturnedFrame, DbError> {
    if core.returned_frame.is_some() {
        let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
        if core.returned_fallback.is_none() {
            core.returned_fallback = Some(DatabaseSyncHelloReturnedFrameLease { generation: 0, items, bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None }) });
            state.schedule();
        }
        return Err(DbError::Unavailable("database sync hello returned frame lease occupied".to_string()));
    }
    let generation = match state.returned_generation.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
        Ok(generation) => generation,
        Err(generation) => {
            let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
            core.returned_frame = Some(DatabaseSyncHelloReturnedFrameLease { generation, items, bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None }) });
            state.schedule();
            return Err(DbError::LimitExceeded("database sync hello returned frame generation"));
        }
    };
    let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
    core.returned_frame = Some(DatabaseSyncHelloReturnedFrameLease { generation, items, bytes, close: None });
    Ok(DatabaseSyncHelloReturnedFrame { state: Some(state.clone()), generation, frame: Some(frame) })
}

/// 📤️ Generation-qualified returned frame whose explicit acknowledgement mounts exact close.
pub struct DatabaseSyncHelloReturnedFrame {
    pub(super) state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
    pub(super) generation: u64,
    pub(super) frame: Option<protocol::ServerFrame>,
}

impl DatabaseSyncHelloReturnedFrame {
    pub fn frame(&self) -> Result<&protocol::ServerFrame, DbError> {
        self.frame.as_ref().ok_or(DbError::Closed)
    }

    fn mount_close(&mut self) -> Result<(), DbError> {
        let Some(frame) = self.frame.take() else { return Err(DbError::Closed) };
        let Some(state) = self.state.as_ref() else {
            self.frame = Some(frame);
            return Err(DbError::Closed);
        };
        let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(lease) = core.returned_frame.as_mut() else {
            let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
            core.returned_frame = Some(DatabaseSyncHelloReturnedFrameLease { generation: self.generation, items, bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None }) });
            drop(core);
            state.schedule();
            return Err(DbError::Closed);
        };
        if lease.generation != self.generation || lease.close.is_some() {
            let actual = lease.generation;
            let (items, bytes) = database_sync_hello_returned_frame_credit(&frame)?;
            if core.returned_fallback.is_some() {
                self.frame = Some(frame);
                return Err(DbError::Unavailable("database sync hello returned frame fallback occupied".to_string()));
            }
            core.returned_fallback = Some(DatabaseSyncHelloReturnedFrameLease { generation: self.generation, items, bytes, close: Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None }) });
            drop(core);
            state.schedule();
            return Err(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(actual) });
        }
        lease.close = Some(DatabaseSyncHelloFrameClose { owner: Some(frame), envelope: None });
        drop(core);
        state.schedule();
        Ok(())
    }

    pub fn acknowledge(mut self) -> Result<(), DbError> {
        self.mount_close()
    }
}

impl Drop for DatabaseSyncHelloReturnedFrame {
    fn drop(&mut self) {
        if self.frame.is_some() {
            match self.mount_close() {
                Ok(()) | Err(DbError::StaleGeneration { .. }) => {}
                Err(_) => {
                    if let Some(state) = self.state.as_ref() {
                        state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                        state.arm_close();
                    }
                }
            }
        }
    }
}

/// 📡️ Backpressured hello output with one bounded I/O-lane opportunity per frame request.
pub struct DatabaseSyncHelloSession {
    pub state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
    welcome_taken: bool,
}

impl DatabaseSyncHelloSession {
    #[cfg(any(test, feature = "testkit"))]
    pub fn retirement_witness(&self) -> DatabaseSyncHelloRetirementWitness {
        DatabaseSyncHelloRetirementWitness(self.state.as_ref().map_or_else(std::sync::Weak::new, std::sync::Arc::downgrade))
    }

    pub fn take_welcome(&mut self) -> Result<DatabaseSyncHelloReturnedFrame, DbError> {
        if self.welcome_taken {
            return Err(DbError::Closed);
        }
        let Some(state) = self.state.as_ref() else { return Err(DbError::Closed) };
        database_sync_hello_control(&state.cancelled, &state.expired)?;
        let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let welcome = core.execution.as_mut().and_then(|execution| execution.prepared.as_mut().ok()).and_then(|prepared| prepared.welcome.take()).ok_or_else(|| DbError::Internal("database sync hello welcome owner missing".to_string()))?;
        let welcome = database_sync_hello_lease_frame(state, &mut core, welcome)?;
        self.welcome_taken = true;
        Ok(welcome)
    }

    pub fn next_frame(&self) -> DatabaseSyncHelloNextFuture {
        DatabaseSyncHelloNextFuture { state: self.state.clone(), completed: false }
    }

    pub fn cancel(&self) {
        if let Some(state) = self.state.as_ref() {
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
        }
    }

    pub fn progress(&self) -> DatabaseSyncHelloProgress {
        self.state.as_ref().map_or(DatabaseSyncHelloProgress::Fault, |state| state.progress())
    }
}

impl Drop for DatabaseSyncHelloSession {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            state.arm_close();
        }
    }
}

pub struct DatabaseSyncHelloNextFuture {
    pub(super) state: Option<std::sync::Arc<DatabaseSyncHelloState>>,
    completed: bool,
}

impl std::future::Future for DatabaseSyncHelloNextFuture {
    type Output = Result<Option<DatabaseSyncHelloReturnedFrame>, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if self.completed {
            return std::task::Poll::Ready(Err(DbError::Closed));
        }
        let Some(state) = self.state.as_ref().cloned() else { return std::task::Poll::Ready(Err(DbError::Closed)) };
        if let Err(error) = database_sync_hello_control(&state.cancelled, &state.expired) {
            self.completed = true;
            state.arm_close();
            return std::task::Poll::Ready(Err(error));
        }
        let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.returned_frame.is_none() {
            if let Some(frame) = core.frame.take() {
                self.completed = true;
                return match frame {
                    Ok(Some(frame)) => std::task::Poll::Ready(database_sync_hello_lease_frame(&state, &mut core, frame).map(Some)),
                    Ok(None) => {
                        drop(core);
                        state.arm_close();
                        std::task::Poll::Ready(Ok(None))
                    }
                    Err(error) => std::task::Poll::Ready(Err(error)),
                };
            }
        }
        drop(core);
        *state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let mut core = state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if core.returned_frame.is_none() {
            if let Some(frame) = core.frame.take() {
                self.completed = true;
                return match frame {
                    Ok(Some(frame)) => std::task::Poll::Ready(database_sync_hello_lease_frame(&state, &mut core, frame).map(Some)),
                    Ok(None) => {
                        drop(core);
                        state.arm_close();
                        std::task::Poll::Ready(Ok(None))
                    }
                    Err(error) => std::task::Poll::Ready(Err(error)),
                };
            }
        }
        let lease_closing = core.returned_frame.as_ref().is_some_and(|lease| lease.close.is_some());
        drop(core);
        if lease_closing {
            state.schedule();
        } else if state.core.lock().unwrap_or_else(std::sync::PoisonError::into_inner).returned_frame.is_none() {
            state.demand.store(true, std::sync::atomic::Ordering::SeqCst);
            state.schedule();
        }
        std::task::Poll::Pending
    }
}

impl Drop for DatabaseSyncHelloNextFuture {
    fn drop(&mut self) {
        if !self.completed {
            if let Some(state) = self.state.take() {
                state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                state.arm_close();
            }
        }
    }
}
//#endregion 👋️RetainedHello
//#endregion 🔖️Hello
