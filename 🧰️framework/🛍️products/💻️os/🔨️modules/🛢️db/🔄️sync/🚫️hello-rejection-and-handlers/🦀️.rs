//! 🚫️ Hello rejection and handlers: the rejected close and registry, the rejected terminal witness, the rejected hello, the welcome response, the bootstrap lowering, the hello handlers and the frontier advertise handler. See [`super`].

use super::*;

//#region 🔖️Hello
//#region 👋️RetainedHello
struct DatabaseSyncHelloRejectedClose {
    pub(super) pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    pub owners: std::sync::Mutex<Option<DatabaseSyncHelloOwners>>,
    queued: std::sync::atomic::AtomicBool,
    pub(super) retry_job: std::sync::Mutex<Option<(semio_framework_async::Job, u8)>>,
    pub(super) cancelled: std::sync::atomic::AtomicBool,
    pub(super) deadline_ms: u64,
    pub(super) terminal: std::sync::atomic::AtomicU8,
    terminal_callback: std::sync::atomic::AtomicBool,
    pub submissions: std::sync::atomic::AtomicU8,
    registry_generation: std::sync::atomic::AtomicU64,
    registry_next: std::sync::Mutex<Option<std::sync::Arc<DatabaseSyncHelloRejectedClose>>>,
    registry_released: std::sync::atomic::AtomicBool,
}

fn database_sync_hello_rejected_registry() -> &'static std::sync::Mutex<Option<std::sync::Arc<DatabaseSyncHelloRejectedClose>>> {
    static REGISTRY: std::sync::Mutex<Option<std::sync::Arc<DatabaseSyncHelloRejectedClose>>> = std::sync::Mutex::new(None);
    &REGISTRY
}

fn database_sync_hello_install_rejected_registry(owner: &std::sync::Arc<DatabaseSyncHelloRejectedClose>) -> Result<u64, DbError> {
    static GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let generation = GENERATION
        .try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |generation| generation.checked_add(1).filter(|generation| *generation != 0))
        .map_err(|_| DbError::LimitExceeded("database sync hello rejection generation"))?;
    owner.registry_generation.store(generation, std::sync::atomic::Ordering::Release);
    let mut registry = database_sync_hello_rejected_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    *owner.registry_next.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = registry.take();
    *registry = Some(owner.clone());
    Ok(generation)
}

fn database_sync_hello_release_rejected_registry(owner: &std::sync::Arc<DatabaseSyncHelloRejectedClose>) {
    owner.registry_released.store(true, std::sync::atomic::Ordering::Release);
    let mut registry = database_sync_hello_rejected_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(root) = registry.as_ref() else { return };
    if !std::sync::Arc::ptr_eq(root, owner) || !root.registry_released.load(std::sync::atomic::Ordering::Acquire) {
        return;
    }
    let next = root.registry_next.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
    *registry = next;
    let cleanup = registry.as_ref().filter(|next| next.registry_released.load(std::sync::atomic::Ordering::Acquire)).cloned();
    drop(registry);
    if let Some(next) = cleanup {
        let pool = next.pool.clone();
        pool.callback_at(pool.now_ms().saturating_add(1), move || database_sync_hello_release_rejected_registry(&next));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DatabaseSyncHelloRejectedTerminalWitness {
    pub generation: u64,
    pub terminal: u8,
    pub job_retained: bool,
    pub owners_retained: bool,
    pub submissions: u8,
}

pub fn database_sync_hello_rejected_terminal_witness(generation: u64) -> Option<DatabaseSyncHelloRejectedTerminalWitness> {
    let registry = database_sync_hello_rejected_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut owner = registry.clone();
    drop(registry);
    while let Some(current) = owner {
        if current.registry_generation.load(std::sync::atomic::Ordering::Acquire) == generation {
            return Some(DatabaseSyncHelloRejectedTerminalWitness {
                generation,
                terminal: current.terminal.load(std::sync::atomic::Ordering::Acquire),
                job_retained: current.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
                owners_retained: current.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
                submissions: current.submissions.load(std::sync::atomic::Ordering::Acquire),
            });
        }
        owner = current.registry_next.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
    }
    None
}

impl DatabaseSyncHelloRejectedClose {
    fn claim_submission(&self) -> bool {
        self.submissions.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |submissions| submissions.checked_add(1).filter(|next| *next <= DATABASE_SYNC_HELLO_RETRY_LIMIT)).is_ok()
    }

    fn publish_terminal_recovery(self: &std::sync::Arc<Self>, terminal: u8) {
        self.terminal.store(terminal, std::sync::atomic::Ordering::Release);
        if self.terminal_callback.compare_exchange(false, true, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return;
        }
        let close = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || close.retry_terminal_once());
    }

    fn retry_terminal_once(self: std::sync::Arc<Self>) {
        self.terminal_callback.store(false, std::sync::atomic::Ordering::Release);
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        if !self.claim_submission() {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            self.terminal.store(3, std::sync::atomic::Ordering::Release);
            return;
        }
        self.queued.store(true, std::sync::atomic::Ordering::Release);
        match self.pool.try_submit(semio_framework_async::Lane::Io, job) {
            Ok(()) => self.terminal.store(0, std::sync::atomic::Ordering::Release),
            Err(error) => {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), attempt));
                self.queued.store(false, std::sync::atomic::Ordering::Release);
            }
        }
    }

    fn terminal_witness(&self, attempt: u8) -> u8 {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return 1;
        }
        if self.pool.now_ms() >= self.deadline_ms {
            return 2;
        }
        if attempt >= DATABASE_SYNC_HELLO_RETRY_LIMIT.saturating_sub(1) {
            return 3;
        }
        0
    }

    fn retain_retry(self: &std::sync::Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
        self.queued.store(false, std::sync::atomic::Ordering::Release);
        let terminal = self.terminal_witness(attempt);
        if terminal != 0 {
            self.publish_terminal_recovery(terminal);
            return;
        }
        let close = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || close.retry());
    }

    pub(super) fn schedule(self: &std::sync::Arc<Self>) {
        if self.terminal.load(std::sync::atomic::Ordering::Acquire) != 0 {
            return;
        }
        if self.queued.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        if self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            self.queued.store(false, std::sync::atomic::Ordering::Release);
            return;
        }
        if !self.claim_submission() {
            self.queued.store(false, std::sync::atomic::Ordering::Release);
            self.publish_terminal_recovery(3);
            return;
        }
        let close = self.clone();
        match self.pool.try_submit(semio_framework_async::Lane::Io, Box::new(move || close.drive_one())) {
            Ok(()) => {}
            Err(error) => self.retain_retry(error.into_job(), 1),
        }
    }

    pub(super) fn retry(self: std::sync::Arc<Self>) {
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        let terminal = self.terminal_witness(attempt);
        if terminal != 0 {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            self.publish_terminal_recovery(terminal);
            return;
        }
        if !self.claim_submission() {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            self.publish_terminal_recovery(3);
            return;
        }
        self.queued.store(true, std::sync::atomic::Ordering::Release);
        match self.pool.try_submit(semio_framework_async::Lane::Io, job) {
            Ok(()) => {}
            Err(error) => self.retain_retry(error.into_job(), attempt.saturating_add(1)),
        }
    }

    pub(super) fn drive_one(self: std::sync::Arc<Self>) {
        self.queued.store(false, std::sync::atomic::Ordering::Release);
        let mut owners = self.owners.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(owner) = owners.as_mut() else { return };
        if owner.close_one() {
            let terminal = owner.terminal_is_empty();
            if terminal {
                owners.take();
                self.terminal.store(4, std::sync::atomic::Ordering::Release);
                database_sync_hello_release_rejected_registry(&self);
            }
            drop(owners);
            if !terminal {
                self.schedule();
            }
            return;
        }
        owners.take();
        self.terminal.store(4, std::sync::atomic::Ordering::Release);
        database_sync_hello_release_rejected_registry(&self);
    }
}

/// ⛔️ Lossless pre-admission refusal retaining every exact hello input owner.
pub struct DatabaseSyncHelloRejected {
    pub error: Option<DbError>,
    pub close: std::sync::Arc<DatabaseSyncHelloRejectedClose>,
}

impl DatabaseSyncHelloRejected {
    /// 🚦️ Whether this hello was refused only because every admission slot was taken, so
    /// the same hello is admitted once [`DatabaseSyncHelloAdmissionReady`] resolves.
    pub fn admission_saturated(&self) -> bool {
        matches!(&self.error, Some(DbError::Unavailable(message)) if message == DATABASE_SYNC_HELLO_ADMISSION_SATURATED)
    }

    pub fn new(pool: std::sync::Arc<semio_framework_async::WorkerPool>, error: DbError, owners: DatabaseSyncHelloOwners) -> Self {
        let deadline_ms = pool.now_ms().saturating_add(DATABASE_SYNC_HELLO_DEADLINE_MS);
        let close = std::sync::Arc::new(DatabaseSyncHelloRejectedClose {
            pool,
            owners: std::sync::Mutex::new(Some(owners)),
            queued: std::sync::atomic::AtomicBool::new(false),
            retry_job: std::sync::Mutex::new(None),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            deadline_ms,
            terminal: std::sync::atomic::AtomicU8::new(0),
            terminal_callback: std::sync::atomic::AtomicBool::new(false),
            submissions: std::sync::atomic::AtomicU8::new(0),
            registry_generation: std::sync::atomic::AtomicU64::new(0),
            registry_next: std::sync::Mutex::new(None),
            registry_released: std::sync::atomic::AtomicBool::new(false),
        });
        let error = match database_sync_hello_install_rejected_registry(&close) {
            Ok(_) => error,
            Err(registry_error) => {
                let mut registry = database_sync_hello_rejected_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                *close.registry_next.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = registry.take();
                *registry = Some(close.clone());
                registry_error
            }
        };
        Self { error: Some(error), close }
    }

    pub fn retirement_generation(&self) -> u64 {
        self.close.registry_generation.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn cancel_retirement(&self) {
        self.close.cancelled.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn close_and_take_error(mut self) -> DbError {
        let error = self.error.take().unwrap_or(DbError::Closed);
        self.close.schedule();
        error
    }
}

impl std::fmt::Debug for DatabaseSyncHelloRejected {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("DatabaseSyncHelloRejected").field("error", &self.error).finish()
    }
}

impl Drop for DatabaseSyncHelloRejected {
    fn drop(&mut self) {
        self.close.schedule();
    }
}
//#endregion 👋️RetainedHello

/// 🚀️ What `handle_hello` produces: the `Welcome` frame itself, plus whatever follow-up
/// frames the chosen bootstrap needs (a single `Commands` frame for `Tail`; one `SnapshotChunk`
/// per chunk plus a trailing `SnapshotDone` for a non-inlined `Snapshot`; none for `None` or an
/// inlined `Snapshot`, whose bytes already travel inside `Welcome.bootstrap`).
#[cfg(any(test, feature = "testkit"))]
#[derive(Clone, Debug, PartialEq)]
pub struct WelcomeResponse {
    pub welcome: protocol::ServerFrame,
    pub follow_up: Vec<protocol::ServerFrame>,
}

#[cfg(any(test, feature = "testkit"))]
struct BootstrapPageRangeCopy<'a> {
    pub(super) pages: &'a db_io_core::DbIoPages,
    skip: usize,
    pub(super) remaining: usize,
    pub(super) page: u8,
    output: Option<Vec<u8>>,
}

#[cfg(any(test, feature = "testkit"))]
impl std::future::Future for BootstrapPageRangeCopy<'_> {
    type Output = Result<Vec<u8>, DbError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let owner = self.as_mut().get_mut();
        if owner.remaining == 0 {
            return std::task::Poll::Ready(Ok(owner.output.take().expect("bootstrap page range output consumed once")));
        }
        let Some(fragment) = owner.pages.page(owner.page) else {
            return std::task::Poll::Ready(Err(DbError::Corrupt("bootstrap page range exceeded its retained owner".to_string())));
        };
        if owner.skip >= fragment.len() {
            owner.skip -= fragment.len();
            owner.page += 1;
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        let available = &fragment[owner.skip..];
        let written = available.len().min(owner.remaining);
        owner.output.as_mut().expect("bootstrap page range output retained").extend_from_slice(&available[..written]);
        owner.skip = 0;
        owner.remaining -= written;
        owner.page += 1;
        context.waker().wake_by_ref();
        std::task::Poll::Pending
    }
}

#[cfg(any(test, feature = "testkit"))]
fn bootstrap_page_range_copy(pages: &db_io_core::DbIoPages, start: usize, len: usize) -> Result<BootstrapPageRangeCopy<'_>, DbError> {
    let end = start.checked_add(len).ok_or(DbError::LimitExceeded("bootstrap page range"))?;
    if end > pages.len() {
        return Err(DbError::InvalidArgument("bootstrap page range exceeds its retained owner".to_string()));
    }
    let mut output = Vec::new();
    output.try_reserve_exact(len).map_err(|_| DbError::Unavailable("bootstrap protocol result capacity exhausted".to_string()))?;
    Ok(BootstrapPageRangeCopy { pages, skip: start, remaining: len, page: 0, output: Some(output) })
}

/// 🏗️ Lowers a `BootstrapPlan` to the wire `protocol::Bootstrap` shape plus its follow-up
/// frames. A `Snapshot` whose bytes fit within `snapshot_chunk_bytes` is inlined directly into
/// `Bootstrap::Snapshot.inline` (no follow-up frames); a larger one is chunked instead — this
/// crate's own choice of threshold behavior, since the contract fixes `Bootstrap::Snapshot`'s two
/// shapes but not when to prefer one over the other.
#[cfg(any(test, feature = "testkit"))]
async fn lower_bootstrap_plan(plan: &BootstrapPlan, state: &ArtifactSyncState, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> Result<(protocol::Bootstrap, Vec<protocol::ServerFrame>), DbError> {
    match plan {
        BootstrapPlan::None => Ok((protocol::Bootstrap::None, Vec::new())),
        BootstrapPlan::Tail { envelopes } => Ok((protocol::Bootstrap::Tail, vec![commands_server_frame(state, envelopes.clone(), origin.clone()).await])),
        BootstrapPlan::Snapshot { pages, pack_hash, .. } => {
            if pages.len() <= snapshot_chunk_bytes {
                let bytes = bootstrap_page_range_copy(pages, 0, pages.len())?.await?;
                Ok((protocol::Bootstrap::Snapshot { pack_hash: *pack_hash, inline: Some(bytes) }, Vec::new()))
            } else {
                let chunks = pages.len().div_ceil(snapshot_chunk_bytes);
                if chunks > db_io_core::DB_IO_OPERATION_ITEMS {
                    return Err(DbError::LimitExceeded("bootstrap protocol chunk items"));
                }
                let mut follow_up = Vec::new();
                follow_up.try_reserve_exact(chunks + 1).map_err(|_| DbError::Unavailable("bootstrap protocol item capacity exhausted".to_string()))?;
                for seq in 0..chunks {
                    let start = seq * snapshot_chunk_bytes;
                    let len = snapshot_chunk_bytes.min(pages.len() - start);
                    let bytes = bootstrap_page_range_copy(pages, start, len)?.await?;
                    let fixed = protocol::SnapshotChunkBytes::try_from_slice(&bytes).ok_or(DbError::LimitExceeded("bootstrap protocol fixed snapshot chunk"))?;
                    follow_up.push(protocol::ServerFrame::SnapshotChunk { seq: seq as u32, bytes: fixed });
                }
                follow_up.push(protocol::ServerFrame::SnapshotDone { seq_count: chunks as u32 });
                Ok((protocol::Bootstrap::Snapshot { pack_hash: *pack_hash, inline: None }, follow_up))
            }
        }
    }
}

/// 🏗️ Builds the full `WelcomeResponse` for `plan` against `state`. `snapshot_chunk_bytes`
/// must be non-zero (validated before `lower_bootstrap_plan` could otherwise divide the snapshot
/// into a runaway number of zero-progress chunks).
#[cfg(any(test, feature = "testkit"))]
pub async fn build_welcome(state: &ArtifactSyncState, plan: &BootstrapPlan, session_id: String, origin: &protocol::ActorId, snapshot_chunk_bytes: usize) -> Result<WelcomeResponse, DbError> {
    if snapshot_chunk_bytes == 0 {
        return Err(DbError::InvalidArgument("snapshot_chunk_bytes must be non-zero".to_string()));
    }
    let resume_token = issue_resume_token(&state.frontier).await?;
    let (bootstrap, follow_up) = lower_bootstrap_plan(plan, state, origin, snapshot_chunk_bytes).await?;
    let welcome = protocol::ServerFrame::Welcome { session_id, resume_token, server_frontier: state_frontier_summary(state).await, bootstrap };
    Ok(WelcomeResponse { welcome, follow_up })
}

/// 👋️ The top-level entry point for a `protocol::ClientFrame::SocketHelloV1`: replays `document`'s
/// current sync state, decides a bootstrap plan against `hello_frontier` (the replica's advertised
/// `RuntimeFrontierSummary`, `None` for a totally fresh replica — see module doc for why this
/// crate reads `SocketHelloV1.frontier` rather than decoding `SocketHelloV1.resume_token`), and lowers it to a
/// `WelcomeResponse`.
#[cfg(any(test, feature = "testkit"))]
pub async fn handle_hello(
    pool: std::sync::Arc<semio_framework_async::WorkerPool>,
    storage: std::sync::Arc<db_storage::DbBackend>,
    document: ArtifactId,
    hello_frontier: Option<protocol::RuntimeFrontierSummary>,
    session_id: String,
    origin: protocol::ActorId,
    snapshot_chunk_bytes: usize,
) -> Result<WelcomeResponse, DbError> {
    let future = DatabaseSyncHelloFuture::try_submit(pool, storage, document, hello_frontier, session_id, origin, snapshot_chunk_bytes).map_err(DatabaseSyncHelloRejected::close_and_take_error)?;
    let mut session = future.await?.close_and_take_session()?;
    let welcome_owner = session.take_welcome()?;
    let welcome = welcome_owner.frame()?.clone();
    welcome_owner.acknowledge()?;
    let mut follow_up = Vec::new();
    while let Some(frame_owner) = session.next_frame().await? {
        follow_up.push(frame_owner.frame()?.clone());
        frame_owner.acknowledge()?;
    }
    Ok(WelcomeResponse { welcome, follow_up })
}

/// 📡️ Mid-session catch-up: a connected replica sends `ClientFrame::FrontierAdvertise`
/// (e.g. after a period of being caught up passively via broadcast, to confirm its position) and
/// the semio_hub replies with whatever commands it's still missing, or `None` if it's already current.
pub async fn handle_frontier_advertise(storage: &impl db_io_core::WalStorage, document: ArtifactId, advertised: &protocol::RuntimeFrontierSummary, origin: protocol::ActorId) -> Result<Option<protocol::ServerFrame>, DbError> {
    let state = replay_sync_state(storage, document).await?;
    let replica = from_frontier_summary(advertised);
    let missing = missing_commands(&state, &replica).await?;
    Ok(if missing.is_empty() { None } else { Some(commands_server_frame(&state, missing, origin).await) })
}
//#endregion 🔖️Hello
