//! 🧵️ Detached work, pool futures, directory/inference runners and document retirement.
//!
//! `ShellDetached`, the pool future, the directory and inference runners, the retained Home projection and the
//! document-retirement registry that drain asynchronous work back into [`ShellState`].
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellTypes
//#region 🧵️ShellDetached
/// 🧵️ One request the shell hands off and never waits on: the future owns everything it touches and
/// runs on the shared pool (native) or the page's executor (browser), and the frame pump only asks,
/// once per frame, whether it answered — so the guest turn behind it never holds the shell, and with it
/// the frame build, checked out.
pub(crate) struct ShellDetached<T> {
    pub(crate) answer: std::sync::Arc<std::sync::Mutex<Option<T>>>,
}

impl<T: 'static> ShellDetached<T> {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn spawn(request: impl std::future::Future<Output = T> + Send + 'static) -> Self
    where
        T: Send,
    {
        let answer = std::sync::Arc::new(std::sync::Mutex::new(None));
        let slot = answer.clone();
        crate::spawn_app_task(async move {
            let value = request.await;
            *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(value);
        });
        Self { answer }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn spawn(request: impl std::future::Future<Output = T> + 'static) -> Self {
        let answer = std::sync::Arc::new(std::sync::Mutex::new(None));
        let slot = answer.clone();
        crate::spawn_app_task(async move {
            let value = request.await;
            *slot.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(value);
        });
        Self { answer }
    }

    /// 📬️ The answer, once, if it has arrived.
    pub(crate) fn take(&self) -> Option<T> {
        self.answer.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }
}

/// 🪢️ Loads a hub document's canonical checkpoint pair into the guest instance, off the shell — the one
/// seed a native, a wasm32 and a React shell open a hub document on
/// ([`semio_framework_os_kernel::os_directory::client::DirectoryClient::document_canonical_checkpoint_pair`]):
/// fetched, its digests verified, admitted only as the checkpoint the open's lease authorizes, then loaded
/// before the document's actor exists, so every mutation the guest authors names the document and its
/// baseline is the hub's (a door artifact's genesis, a checked-in document's check-in). A document bound to
/// no hub keeps the guest's own document.
pub(crate) async fn seed_hub_document(plugin: ProgramBridgeEntry, instance_id: u32, seed: Option<ShellHubDocumentSeed>) -> Result<Option<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1>, crate::program_bridge::ProgramFault> {
    let Some(seed) = seed else { return Ok(None) };
    let pair = seed.client.document_canonical_checkpoint_pair(&seed.ctx, &seed.scope, &seed.checkpoint).await.map_err(|error| format!("canonical checkpoint pair: {error}"))?;
    plugin.load_app_document_pack(instance_id, &pair.pack_bytes, &pair.spr_bytes).await?;
    Ok(Some(pair))
}
/// 🧾️ The reserved sections one owed refresh reads besides its guest bodies: the app catalogue (only on
/// the instance's one claimed fetch), the window engagements, the window measures and the tool measures.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ShellRefreshSections {
    pub(crate) catalogue: bool,
    pub(crate) engagements: bool,
    pub(crate) measures: bool,
    pub(crate) tools: bool,
}

/// 🖼️ What one owed refresh read off the shell for one app instance: the guest bodies it names, keyed by
/// surface id, each with the host effects its render returned, and the reserved sections it read. A read
/// the refresh no longer wants is handed back by its document's own `Drop`.
pub(crate) struct ShellRenderedRefresh {
    pub(crate) instance_id: u32,
    pub(crate) surfaces: HashMap<String, (Result<UiDocumentLease, crate::program_bridge::ProgramFault>, Vec<semio_framework::kernel::Effect>)>,
    pub(crate) catalogue: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
    pub(crate) engagements: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
    pub(crate) measures: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
    pub(crate) tools: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
}

impl ShellRenderedRefresh {
    /// 🫙️ Nothing read ahead: every wanted body and section is read by the refresh itself.
    pub(crate) fn empty(instance_id: u32) -> Self {
        Self { instance_id, surfaces: HashMap::new(), catalogue: None, engagements: None, measures: None, tools: None }
    }
}

/// 🖼️ Reads everything one owed refresh names, in order, off the shell: every body render and every
/// section read is a guest turn, so the settle lane hands them off ([`ShellState::advance_owed_refresh`])
/// and the frame loop keeps building frames while they run; the refresh that consumes them then only
/// applies documents ([`ShellState::refresh_ui_rendered`]).
pub(crate) async fn render_refresh_detached(program: ProgramBridgeEntry, instance_id: u32, jobs: Vec<(String, String, ViewModel)>, sections: ShellRefreshSections, panel_view: ViewModel, view_state: ViewModel) -> ShellRenderedRefresh {
    let mut rendered = ShellRenderedRefresh::empty(instance_id);
    for (surface_id, body_key, view) in jobs {
        let mut effects = Vec::new();
        let document = program.render_with_document(instance_id, &surface_id, &body_key, &view, None, Some(&mut effects)).await;
        rendered.surfaces.insert(surface_id, (document, effects));
    }
    if sections.catalogue {
        let body_key = semio_framework::UiRefreshSection::Catalogue.body_key();
        rendered.catalogue = Some(program.render_with_document(instance_id, body_key, body_key, &panel_view, None, None).await);
    }
    if sections.engagements {
        rendered.engagements = Some(program.window_engagements_section(instance_id, &view_state).await);
    }
    if sections.measures {
        rendered.measures = Some(program.window_measures_section(instance_id, &view_state).await);
    }
    if sections.tools {
        rendered.tools = Some(program.tool_measures_section(instance_id, &view_state).await);
    }
    rendered
}
//#endregion 🧵️ShellDetached

#[cfg(not(target_arch = "wasm32"))]
/// 🔄️ A retained-waker future polled once per shared-pool turn; no worker waits for completion.
pub(crate) struct ShellPoolFuture {
    pub(crate) pool: WorkerPool,
    pub(crate) lane: Lane,
    pub(crate) future: std::sync::Mutex<Option<std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>>>,
    pub(crate) scheduled: std::sync::atomic::AtomicBool,
    pub(crate) notified: std::sync::atomic::AtomicBool,
    pub(crate) cancelled: std::sync::atomic::AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShellPoolFuture {
    pub(crate) fn spawn(pool: WorkerPool, lane: Lane, future: impl std::future::Future<Output = ()> + Send + 'static) -> std::sync::Arc<Self> {
        let task = std::sync::Arc::new(Self {
            pool,
            lane,
            future: std::sync::Mutex::new(Some(Box::pin(future))),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            notified: std::sync::atomic::AtomicBool::new(true),
            cancelled: std::sync::atomic::AtomicBool::new(false),
        });
        task.schedule();
        task
    }

    fn schedule(self: &std::sync::Arc<Self>) {
        self.notified.store(true, std::sync::atomic::Ordering::Release);
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.scheduled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let task = self.clone();
        self.pool.submit(self.lane, Box::new(move || task.run_turn()));
    }

    fn run_turn(self: std::sync::Arc<Self>) {
        self.notified.store(false, std::sync::atomic::Ordering::Release);
        let future = self.future.lock().expect("ShellPoolFuture mutex poisoned").take();
        if let Some(mut future) = future {
            if !self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                let waker = std::task::Waker::from(self.clone());
                let mut context = std::task::Context::from_waker(&waker);
                if std::future::Future::poll(future.as_mut(), &mut context).is_pending() && !self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                    *self.future.lock().expect("ShellPoolFuture mutex poisoned") = Some(future);
                }
            }
        }
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if self.notified.load(std::sync::atomic::Ordering::Acquire) && self.future.lock().expect("ShellPoolFuture mutex poisoned").is_some() {
            self.schedule();
        }
    }

    pub(crate) fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.future.lock().expect("ShellPoolFuture mutex poisoned").take();
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl std::task::Wake for ShellPoolFuture {
    fn wake(self: std::sync::Arc<Self>) {
        self.schedule();
    }

    fn wake_by_ref(self: &std::sync::Arc<Self>) {
        self.schedule();
    }
}

#[cfg(not(target_arch = "wasm32"))]
type ShellDirectoryStream = DirectoryStream<ShellDirectoryTransport>;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn close_unowned_directory_dial<T: DirectoryWsConnection>(result: Result<T, TransportError>) {
    if let Ok(mut connection) = result {
        connection.close();
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// 📡️ Finite directory-stream actor: bounded turns, I/O-lane dials, timer-wheel wakeups, ordered output.
pub(crate) struct ShellDirectoryRunner {
    pub(crate) pool: WorkerPool,
    pub(crate) stream: std::sync::Mutex<ShellDirectoryStream>,
    pub(crate) context: OperationContext,
    pub(crate) events: std::sync::Mutex<std::collections::VecDeque<DirectoryStreamMessage>>,
    pub(crate) scheduled: std::sync::atomic::AtomicBool,
    pub(crate) notified: std::sync::atomic::AtomicBool,
    pub(crate) cancelled: std::sync::atomic::AtomicBool,
    pub(crate) terminal: std::sync::atomic::AtomicBool,
    pub(crate) timer_deadline_ms: std::sync::atomic::AtomicU64,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShellDirectoryRunner {
    const MAX_EVENTS: usize = 256;
    const MAX_MESSAGES_PER_TURN: usize = 32;
    const MAX_TURN_MS: u64 = 4;
    const SOCKET_POLL_MS: u64 = 8;
    const DIAL_TIMEOUT_MS: u64 = 1_000;

    pub(crate) fn start(pool: WorkerPool, stream: ShellDirectoryStream, context: OperationContext) -> std::sync::Arc<Self> {
        let runner = std::sync::Arc::new(Self {
            pool,
            stream: std::sync::Mutex::new(stream),
            context,
            events: std::sync::Mutex::new(std::collections::VecDeque::new()),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            notified: std::sync::atomic::AtomicBool::new(true),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            terminal: std::sync::atomic::AtomicBool::new(false),
            timer_deadline_ms: std::sync::atomic::AtomicU64::new(0),
        });
        runner.schedule();
        runner
    }

    fn schedule(self: &std::sync::Arc<Self>) {
        self.notified.store(true, std::sync::atomic::Ordering::Release);
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.scheduled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let runner = self.clone();
        self.pool.submit(Lane::Io, Box::new(move || runner.run_turn()));
    }

    fn run_turn(self: std::sync::Arc<Self>) {
        self.notified.store(false, std::sync::atomic::Ordering::Release);
        let started = self.pool.now_ms();
        let mut processed = 0usize;
        let mut wake_at = None;
        while processed < Self::MAX_MESSAGES_PER_TURN && self.pool.now_ms().saturating_sub(started) < Self::MAX_TURN_MS {
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.events.lock().expect("directory events mutex poisoned").len() >= Self::MAX_EVENTS {
                break;
            }
            let action = self.stream.lock().expect("directory stream mutex poisoned").turn(&self.context, self.pool.now_ms());
            match action {
                DirectoryStreamTurn::Dial { client, since } => {
                    let weak = std::sync::Arc::downgrade(&self);
                    let context = self.context.clone();
                    self.pool.submit(
                        Lane::Io,
                        Box::new(move || {
                            let result = client.open_stream_ws(&context, since, Self::DIAL_TIMEOUT_MS).map_err(|error| TransportError::Io(error.to_string()));
                            if let Some(runner) = weak.upgrade() {
                                let now_ms = runner.pool.now_ms();
                                let _ = runner.stream.lock().expect("directory stream mutex poisoned").complete_dial(now_ms, result);
                                runner.schedule();
                            } else {
                                close_unowned_directory_dial(result);
                            }
                        }),
                    );
                    break;
                }
                DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Revoked(_) => {
                    self.terminal.store(true, std::sync::atomic::Ordering::Release);
                    break;
                }
                DirectoryStreamTurn::Message(message) => {
                    self.events.lock().expect("directory events mutex poisoned").push_back(message);
                    processed += 1;
                }
                DirectoryStreamTurn::ReconnectAt(deadline_ms) => {
                    wake_at = Some(deadline_ms);
                    break;
                }
                DirectoryStreamTurn::Idle => {
                    wake_at = Some(self.pool.now_ms().saturating_add(Self::SOCKET_POLL_MS));
                    break;
                }
                DirectoryStreamTurn::Closed => {
                    if !self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                        self.terminal.store(true, std::sync::atomic::Ordering::Release);
                    }
                    break;
                }
            }
        }
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if self.notified.load(std::sync::atomic::Ordering::Acquire) || processed == Self::MAX_MESSAGES_PER_TURN {
            self.schedule();
        } else if let Some(deadline_ms) = wake_at {
            self.arm_timer(deadline_ms);
        }
    }

    fn arm_timer(self: &std::sync::Arc<Self>, deadline_ms: u64) {
        let current = self.timer_deadline_ms.load(std::sync::atomic::Ordering::Acquire);
        if current != 0 && current <= deadline_ms {
            return;
        }
        self.timer_deadline_ms.store(deadline_ms, std::sync::atomic::Ordering::Release);
        let weak = std::sync::Arc::downgrade(self);
        let pool = self.pool.clone();
        let sleep_pool = pool.clone();
        ShellPoolFuture::spawn(pool, Lane::Timer, async move {
            sleep_pool.timer().sleep_until(deadline_ms).await;
            if let Some(runner) = weak.upgrade() {
                if runner.timer_deadline_ms.compare_exchange(deadline_ms, 0, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    runner.schedule();
                }
            }
        });
    }

    pub(crate) fn drain(self: &std::sync::Arc<Self>) -> Vec<DirectoryStreamMessage> {
        let events = self.events.lock().expect("directory events mutex poisoned").drain(..).collect();
        self.schedule();
        events
    }

    pub(crate) fn take_terminal(&self) -> bool {
        self.terminal.swap(false, std::sync::atomic::Ordering::AcqRel)
    }

    pub(crate) fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.context.cancel.cancel_now();
        self.stream.lock().expect("directory stream mutex poisoned").close();
    }
}

//#region 💡️InferencePort
/// 💡️ Finite inference-port actor: bounded turns, I/O-lane calls through the native
/// `DirectoryClient`, timer-wheel wakeups and ordered status output — the exact discipline
/// `ShellDirectoryRunner` above already follows for the directory stream. No blocking call ever
/// enters the render loop, and a cancel is a hard terminal.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct ShellInferenceRunner {
    pub(crate) port: std::sync::Arc<CompiledDocumentHttpPortV1>,
    pub(crate) pool: WorkerPool,
    pub(crate) client: std::sync::Arc<ShellDirectoryClient>,
    pub(crate) context: OperationContext,
    pub(crate) owner: String,
    pub(crate) service_id: String,
    pub(crate) scope: DocumentScope,
    pub(crate) driver: std::sync::Mutex<Box<dyn InstalledServiceDriverV1>>,
    pub(crate) statuses: std::sync::Mutex<std::collections::VecDeque<InstalledServiceStatusV1>>,
    pub(crate) scheduled: std::sync::atomic::AtomicBool,
    pub(crate) notified: std::sync::atomic::AtomicBool,
    pub(crate) cancelled: std::sync::atomic::AtomicBool,
    pub(crate) timer_deadline_ms: std::sync::atomic::AtomicU64,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShellInferenceRunner {
    const MAX_STATUSES: usize = 64;
    const MAX_ACTIONS_PER_TURN: usize = 4;
    const MAX_TURN_MS: u64 = 4;

    pub(crate) fn start(pool: WorkerPool, client: std::sync::Arc<ShellDirectoryClient>, port: std::sync::Arc<CompiledDocumentHttpPortV1>, context: OperationContext, contribution: InstalledServiceContributionV1, scope: DocumentScope, driver: Box<dyn InstalledServiceDriverV1>) -> std::sync::Arc<Self> {
        let runner = std::sync::Arc::new(Self {
            port,
            pool,
            client,
            context,
            owner: contribution.owner.into(),
            service_id: contribution.service_id.into(),
            scope,
            driver: std::sync::Mutex::new(driver),
            statuses: std::sync::Mutex::new(std::collections::VecDeque::new()),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            notified: std::sync::atomic::AtomicBool::new(true),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            timer_deadline_ms: std::sync::atomic::AtomicU64::new(0),
        });
        runner.schedule();
        runner
    }

    fn schedule(self: &std::sync::Arc<Self>) {
        self.notified.store(true, std::sync::atomic::Ordering::Release);
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.scheduled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let runner = self.clone();
        self.pool.submit(Lane::Io, Box::new(move || runner.run_turn()));
    }

    pub(crate) fn intend(self: &std::sync::Arc<Self>, action: &str, payload: DslValue) {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) { return; }
        if self.driver.lock().expect("service driver mutex poisoned").intend(action,payload).is_err() { return; }
        self.publish();
        self.schedule();
    }

    pub(crate) fn publish(self: &std::sync::Arc<Self>) {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) { return; }
        let status = InstalledServiceStatusV1 { owner:self.owner.clone(), service_id:self.service_id.clone(), payload:self.driver.lock().expect("service driver mutex poisoned").status() };
        if store::pack_rt::encode_wire_value(&status.payload).len()>16*1024 {self.cancel();return;}
        let mut statuses = self.statuses.lock().expect("inference statuses mutex poisoned");
        if statuses.back() == Some(&status) {
            return;
        }
        if statuses.len() >= Self::MAX_STATUSES {
            statuses.pop_front();
        }
        statuses.push_back(status);
    }

    fn run_turn(self: std::sync::Arc<Self>) {
        self.notified.store(false, std::sync::atomic::Ordering::Release);
        let started = self.pool.now_ms();
        let mut processed = 0usize;
        let mut wake_at = None;
        while processed < Self::MAX_ACTIONS_PER_TURN && self.pool.now_ms().saturating_sub(started) < Self::MAX_TURN_MS {
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                break;
            }
            let action = self.driver.lock().expect("inference driver mutex poisoned").turn(self.pool.now_ms());
            self.publish();
            match action {
                InstalledServiceTurnV1::Idle | InstalledServiceTurnV1::Terminal => break,
                InstalledServiceTurnV1::WaitUntil(deadline_ms) => {
                    wake_at = Some(deadline_ms);
                    break;
                }
                other => {
                    self.submit_action(other);
                    processed += 1;
                }
            }
        }
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        if self.notified.load(std::sync::atomic::Ordering::Acquire) {
            self.schedule();
        } else if let Some(deadline_ms) = wake_at {
            self.arm_timer(deadline_ms);
        }
    }

    /// 📡 Invokes one declared operation; retirement suppresses every completion callback.
    pub(crate) fn submit_action(self: &std::sync::Arc<Self>, turn: InstalledServiceTurnV1) {
        let InstalledServiceTurnV1::Call {action,payload}=turn else {return};
        let weak=std::sync::Arc::downgrade(self);
        let client=self.client.clone();
        let context=self.context.clone();
        let scope=self.scope.clone();
        let port=self.port.clone();
        ShellPoolFuture::spawn(self.pool.clone(),Lane::Io,async move {
            let result=port.call(&client,&context,&scope,&action,&payload).await;
            if let Some(runner)=weak.upgrade() {
                if runner.cancelled.load(std::sync::atomic::Ordering::Acquire) {return;}
                runner.driver.lock().expect("service driver mutex poisoned").complete(&action,&payload,result);
                runner.publish();
                runner.schedule();
            }
        });
    }

    fn arm_timer(self: &std::sync::Arc<Self>, deadline_ms: u64) {
        let current = self.timer_deadline_ms.load(std::sync::atomic::Ordering::Acquire);
        if current != 0 && current <= deadline_ms {
            return;
        }
        self.timer_deadline_ms.store(deadline_ms, std::sync::atomic::Ordering::Release);
        let weak = std::sync::Arc::downgrade(self);
        let pool = self.pool.clone();
        let sleep_pool = pool.clone();
        ShellPoolFuture::spawn(pool, Lane::Timer, async move {
            sleep_pool.timer().sleep_until(deadline_ms).await;
            if let Some(runner) = weak.upgrade() {
                if runner.timer_deadline_ms.compare_exchange(deadline_ms, 0, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    runner.schedule();
                }
            }
        });
    }

    pub(crate) fn drain(self: &std::sync::Arc<Self>) -> Vec<InstalledServiceStatusV1> {
        self.statuses.lock().expect("inference statuses mutex poisoned").drain(..).collect()
    }

    pub(crate) fn cancel(&self) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.context.cancel.cancel_now();
    }
}
//#endregion 💡️InferencePort

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type ShellDirectoryPageResult = (u64, Result<CanonicalDirectoryEventPageV1, DirectoryClientError>);

#[cfg(not(target_arch = "wasm32"))]
pub(crate) enum ShellDirectoryHomePublicationOutcome {
    Published(DirectoryEventPageAckV1),
    Rejected(crate::program_bridge::ProgramFault),
    Terminal(String),
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type ShellDirectoryHomePublicationResult = (u64, u32, DirectoryEventPageAckV1, ShellDirectoryHomePublicationOutcome);

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct DirectoryHomeProjection {
    pub(crate) plugin_id: String,
    pub(crate) instance_id: u32,
    pub(crate) app: AppDefinition,
    pub(crate) view_state: ViewModel,
    pub(crate) bootstrap: DirectoryEventPageBootstrapV1,
    pub(crate) page_task: Option<std::sync::Arc<ShellPoolFuture>>,
    pub(crate) page_cancel: Option<CancelToken>,
    pub(crate) page_rx: Option<std::sync::mpsc::Receiver<ShellDirectoryPageResult>>,
    pub(crate) home_task: Option<std::sync::Arc<ShellPoolFuture>>,
    pub(crate) home_rx: Option<std::sync::mpsc::Receiver<ShellDirectoryHomePublicationResult>>,
    pub(crate) stream: Option<std::sync::Arc<ShellDirectoryRunner>>,
    pub(crate) retry_at_ms: u64,
    pub(crate) closed: bool,
    destroy_authority: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl DirectoryHomeProjection {
    pub(crate) const RETRY_DELAY_MS: u64 = 50;

    pub(crate) fn new(plugin_id: String, instance_id: u32, app: AppDefinition, view_state: ViewModel) -> Result<Self, DirectoryClientError> {
        Ok(Self {
            plugin_id,
            instance_id,
            app,
            view_state,
            bootstrap: DirectoryEventPageBootstrapV1::new(1, 0)?,
            page_task: None,
            page_cancel: None,
            page_rx: None,
            home_task: None,
            home_rx: None,
            stream: None,
            retry_at_ms: 0,
            closed: false,
            destroy_authority: true,
        })
    }

    pub(crate) fn is_instance(&self, plugin_id: &str, instance_id: u32) -> bool {
        self.plugin_id == plugin_id && self.instance_id == instance_id
    }

    pub(crate) fn active_session(&self) -> ActiveSession {
        ActiveSession { plugin_id: self.plugin_id.clone(), instance_id: self.instance_id, app: self.app.clone(), view_state: self.view_state.clone() }
    }

    fn cancel_io(&mut self) {
        if let Some(cancel) = self.page_cancel.take() {
            cancel.cancel_now();
        }
        if let Some(task) = self.page_task.take() {
            task.cancel();
        }
        self.page_rx.take();
        if let Some(task) = self.home_task.take() {
            task.cancel();
        }
        self.home_rx.take();
        if let Some(stream) = self.stream.take() {
            stream.cancel();
        }
    }

    pub(crate) fn begin_epoch(&mut self, after: u64) -> Result<u64, DirectoryClientError> {
        self.cancel_io();
        self.bootstrap.close();
        let epoch = self.bootstrap.bootstrap_epoch().checked_add(1).ok_or_else(|| DirectoryClientError::Decode("native directory bootstrap epoch exhausted".into()))?;
        self.bootstrap = DirectoryEventPageBootstrapV1::new(epoch, after)?;
        self.retry_at_ms = 0;
        self.closed = false;
        Ok(epoch)
    }

    pub(crate) fn retry(&mut self, now_ms: u64, receipt_sha256: &str) -> Result<u64, DirectoryClientError> {
        let after = self.bootstrap.reject(self.bootstrap.bootstrap_epoch(), receipt_sha256)?;
        self.retry_at_ms = now_ms.saturating_add(Self::RETRY_DELAY_MS);
        Ok(after)
    }

    /// 🛰️ Answers the CURSOR the woken refetch must start from, never the bootstrap epoch.
    ///
    /// 🩸️ The rebootstrap arm returned `begin_epoch`'s new epoch, so a caller that asked for a
    /// rebootstrap refetched from the epoch number instead of the reset frontier `0` — the kernel's
    /// own `DirectoryEventPageBootstrapV1::wake` has always answered the frontier
    /// (`📇️directory/🔌️client/🦀️.rs:374`) and the live arm below already did.
    pub(crate) fn wake(&mut self, rebootstrap: bool) -> Result<Option<u64>, DirectoryClientError> {
        if rebootstrap {
            self.begin_epoch(0)?;
            return Ok(Some(self.bootstrap.after()));
        }
        self.cancel_io();
        Ok(self.bootstrap.wake(false))
    }

    pub(crate) fn close(&mut self) {
        self.cancel_io();
        self.bootstrap.close();
        self.closed = true;
    }

    pub(crate) fn take_destroy_authority(&mut self) -> Option<(String, u32)> {
        self.close();
        if !self.destroy_authority {
            return None;
        }
        self.destroy_authority = false;
        Some((self.plugin_id.clone(), self.instance_id))
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn terminal_directory_home_ack(result: &semio_framework::kernel::InvocationResult, bootstrap_epoch: u64) -> Result<DirectoryEventPageAckV1, String> {
    const RECEIPT_SCHEMA: &str = "semio.space.home.directory-projection-receipt.v1";
    let mut acknowledgement = None;
    for effect in &result.requested_effects {
        let semio_framework::kernel::Effect::PublishEvent { topic, payload } = effect else {
            return Err("retained Home directory publication emitted a non-receipt effect".into());
        };
        if topic != RECEIPT_SCHEMA || acknowledgement.is_some() {
            return Err("retained Home directory publication emitted an unexpected receipt".into());
        }
        let value = store::pack_rt::decode_wire_value(payload).map_err(|error| error.to_string())?;
        let semio_framework_value::DslValue::Object(fields) = &value else {
            return Err("retained Home directory receipt is not an object".into());
        };
        let expected_keys = ["schema", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive", "receiptSha256"];
        if fields.len() != expected_keys.len() || !fields.iter().all(|(key, _)| expected_keys.contains(&key.as_str())) {
            return Err("retained Home directory receipt has an inexact field set".into());
        }
        let field = |key: &str| value.get(key).ok_or_else(|| format!("retained Home directory receipt lacks {key}"));
        if field("schema")?.as_str() != Some(RECEIPT_SCHEMA) {
            return Err("retained Home directory receipt schema mismatch".into());
        }
        acknowledgement = Some(DirectoryEventPageAckV1 {
            bootstrap_epoch,
            session_binding_sha256: field("sessionBindingSha256")?.as_str().ok_or("retained Home directory receipt binding is not text")?.to_string(),
            authorization_generation: field("authorizationGeneration")?.as_u64().ok_or("retained Home directory receipt generation is not an unsigned integer")?,
            through_seq_inclusive: field("throughSeqInclusive")?.as_u64().ok_or("retained Home directory receipt frontier is not an unsigned integer")?,
            receipt_sha256: field("receiptSha256")?.as_str().ok_or("retained Home directory page receipt is not text")?.to_string(),
        });
    }
    if !result.mutations.is_empty() {
        return Err("retained Home directory publication escaped its Config-only lane".into());
    }
    acknowledgement.ok_or_else(|| "retained Home directory publication returned no terminal receipt".into())
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../🧪️tests/🔬️wgpu-shell-pool-future/🦀️.rs"]
mod shell_pool_future_tests;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) enum ShellIoCompletion {
    Actions(Vec<ActionDescriptor>),
    Import(PickedImport),
    MediaFrames { controller_id: String, frames: Vec<ActionDescriptor> },
    Program(Result<(), crate::program_bridge::ProgramFault>),
    Finished,
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct PendingShellIo {
    pub(crate) receiver: std::sync::mpsc::Receiver<ShellIoCompletion>,
    pub(crate) task: Option<std::sync::Arc<ShellPoolFuture>>,
}

//#region 📄️ShellDocumentRetirement
const SHELL_DOCUMENT_RETIREMENT_CAPACITY: usize = UI_DOCUMENT_LEASE_SLOTS * UI_DOCUMENT_LEASE_ALIASES as usize;
const SHELL_DOCUMENT_RETIREMENT_WORD_BITS: usize = u64::BITS as usize;
const SHELL_DOCUMENT_RETIREMENT_WORDS: usize = (SHELL_DOCUMENT_RETIREMENT_CAPACITY + SHELL_DOCUMENT_RETIREMENT_WORD_BITS - 1) / SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
const SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES: usize = std::mem::size_of::<[u64; SHELL_DOCUMENT_RETIREMENT_WORDS]>() + std::mem::size_of::<u64>();
const _: [(); 72] = [(); SHELL_DOCUMENT_RETIREMENT_INDEX_BYTES];

struct ShellDocumentRetirementSlot {
    pub(crate) epoch: u64,
    pub(crate) generation: u64,
    pub(crate) surface: Option<SurfaceId>,
    pub(crate) document: UiDocumentLease,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ShellDocumentRetirementAdmission {
    pub(crate) index: usize,
    pub(crate) epoch: u64,
}

pub(crate) struct ShellDocumentRetirementRegistry {
    pub(crate) slots: Box<[Option<ShellDocumentRetirementSlot>; SHELL_DOCUMENT_RETIREMENT_CAPACITY]>,
    pub(crate) epochs: [u64; SHELL_DOCUMENT_RETIREMENT_CAPACITY],
    pub(crate) occupied: [u64; SHELL_DOCUMENT_RETIREMENT_WORDS],
    occupied_count: u64,
    pub(crate) cursor: usize,
}

impl Default for ShellDocumentRetirementRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), epochs: [0; SHELL_DOCUMENT_RETIREMENT_CAPACITY], occupied: [0; SHELL_DOCUMENT_RETIREMENT_WORDS], occupied_count: 0, cursor: 0 }
    }
}

impl ShellDocumentRetirementRegistry {
    pub(crate) fn first_vacant_index(&self) -> Option<usize> {
        self.occupied.iter().enumerate().find_map(|(word_index, occupied)| {
            let first_index = word_index * SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
            let remaining = SHELL_DOCUMENT_RETIREMENT_CAPACITY.saturating_sub(first_index);
            let valid = if remaining >= SHELL_DOCUMENT_RETIREMENT_WORD_BITS { u64::MAX } else { (1_u64 << remaining) - 1 };
            let vacant = !occupied & valid;
            (vacant != 0).then_some(first_index + vacant.trailing_zeros() as usize)
        })
    }

    fn next_occupied_index(&self) -> Option<usize> {
        if self.occupied_count == 0 {
            return None;
        }
        let start_word = self.cursor / SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
        let start_bit = self.cursor % SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
        let first = self.occupied[start_word] & (u64::MAX << start_bit);
        if first != 0 {
            return Some(start_word * SHELL_DOCUMENT_RETIREMENT_WORD_BITS + first.trailing_zeros() as usize);
        }
        for word_index in start_word + 1..SHELL_DOCUMENT_RETIREMENT_WORDS {
            let occupied = self.occupied[word_index];
            if occupied != 0 {
                return Some(word_index * SHELL_DOCUMENT_RETIREMENT_WORD_BITS + occupied.trailing_zeros() as usize);
            }
        }
        for word_index in 0..start_word {
            let occupied = self.occupied[word_index];
            if occupied != 0 {
                return Some(word_index * SHELL_DOCUMENT_RETIREMENT_WORD_BITS + occupied.trailing_zeros() as usize);
            }
        }
        if start_bit == 0 {
            return None;
        }
        let wrapped = self.occupied[start_word] & ((1_u64 << start_bit) - 1);
        (wrapped != 0).then_some(start_word * SHELL_DOCUMENT_RETIREMENT_WORD_BITS + wrapped.trailing_zeros() as usize)
    }

    fn mark_occupied(&mut self, index: usize) {
        let word = index / SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
        let bit = 1_u64 << (index % SHELL_DOCUMENT_RETIREMENT_WORD_BITS);
        assert_eq!(self.occupied[word] & bit, 0, "shell document retirement slot was indexed twice");
        self.occupied[word] |= bit;
        self.occupied_count = self.occupied_count.checked_add(1).expect("shell document retirement occupied count overflow");
    }

    fn clear_occupied(&mut self, index: usize) {
        let word = index / SHELL_DOCUMENT_RETIREMENT_WORD_BITS;
        let bit = 1_u64 << (index % SHELL_DOCUMENT_RETIREMENT_WORD_BITS);
        assert_ne!(self.occupied[word] & bit, 0, "shell document retirement slot lost its occupied index");
        self.occupied[word] &= !bit;
        self.occupied_count = self.occupied_count.checked_sub(1).expect("shell document retirement occupied count underflow");
    }

    pub(crate) fn try_admit(&mut self, document: UiDocumentLease) -> Result<(), UiDocumentLease> {
        let Some(admission) = self.try_reserve_admission() else { return Err(document) };
        self.admit_reserved(admission, document);
        Ok(())
    }

    pub(crate) fn try_reserve_admission(&self) -> Option<ShellDocumentRetirementAdmission> {
        let index = self.first_vacant_index()?;
        let epoch = self.epochs[index].checked_add(1)?;
        Some(ShellDocumentRetirementAdmission { index, epoch })
    }

    pub(crate) fn admit_reserved(&mut self, admission: ShellDocumentRetirementAdmission, document: UiDocumentLease) {
        assert_eq!(self.first_vacant_index(), Some(admission.index), "shell document retirement admission lost its exact vacancy");
        assert_eq!(self.epochs[admission.index].checked_add(1), Some(admission.epoch), "shell document retirement admission epoch changed before transfer");
        assert!(self.slots[admission.index].is_none(), "shell document retirement vacancy index disagreed with owner storage");
        let generation = document.generation();
        let surface = document.header().ok().map(|header| header.surface);
        self.epochs[admission.index] = admission.epoch;
        self.slots[admission.index] = Some(ShellDocumentRetirementSlot { epoch: admission.epoch, generation, surface, document });
        self.mark_occupied(admission.index);
    }

    /// ♻️ Advances the NEXT occupied slot by one PAGE.
    ///
    /// ⚖️ Two prices were wrong here and both were invisible: the cursor advanced by one position per
    /// call whether or not that position held anything, so a registry holding six documents in a
    /// 512-slot ring spent 99 % of its steps on vacancies; and each real step was granted
    /// `close_step()`'s one item / 4 KiB, which is a per-ITEM price for a walk whose unit is a page.
    /// Together they made [`SHELL_DOCUMENT_RETIREMENT_STEPS`] buy ~128 real steps per document, far
    /// short of what one 120-node document's typed descendants need — so a refresh's previous set was
    /// still resident when the next asked the process-wide aggregate for credit, and every surface of
    /// the second refresh was refused `Capacity`
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Priced per page, exactly as
    /// `📓️close-ladder-budget-2026-09-12.md` §3 prices every other retirement in this shell.
    /// The fixed occupied-word index preserves the same physical-slot cursor order while bounding
    /// sparse discovery to eight words; `occupied_count` makes the empty decision scalar.
    pub(crate) fn close_one(&mut self) -> bool {
        let Some(index) = self.next_occupied_index() else { return false };
        self.cursor = (index + 1) % SHELL_DOCUMENT_RETIREMENT_CAPACITY;
        let slot = self.slots[index].as_mut().expect("the located shell document retirement slot is occupied");
        let terminal = slot.document.close_step_with_grant(SHELL_DOCUMENT_RETIREMENT_ITEMS, SHELL_DOCUMENT_RETIREMENT_BYTES).is_ok_and(|step| step.complete) && slot.document.terminal_is_empty();
        if terminal {
            let terminal = self.slots[index].take().expect("terminal shell document retirement slot");
            assert!(terminal.document.terminal_is_empty(), "shell document retirement witness changed before removal");
            self.clear_occupied(index);
        }
        true
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.occupied_count == 0
    }

    #[cfg(test)]
    fn qualified_owners(&self) -> impl Iterator<Item = (u64, u64, Option<&SurfaceId>)> {
        self.slots.iter().flatten().map(|slot| (slot.epoch, slot.generation, slot.surface.as_ref()))
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-shell-document-retirement/🦀️.rs"]
mod shell_document_retirement_tests;
//#endregion 📄️ShellDocumentRetirement
//#endregion ShellTypes
