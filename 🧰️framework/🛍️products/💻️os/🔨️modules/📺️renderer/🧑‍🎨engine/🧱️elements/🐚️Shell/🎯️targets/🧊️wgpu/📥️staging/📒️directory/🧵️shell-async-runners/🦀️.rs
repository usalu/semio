//! 📒️ `shell-directory` staging of `shell-async-runners`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellTypes
#[cfg(not(target_arch = "wasm32"))]
type ShellDirectoryStream = DirectoryStream<ShellDirectoryTransport>;

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

    pub(super) fn schedule(self: &std::sync::Arc<Self>) {
        self.notified.store(true, std::sync::atomic::Ordering::Release);
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) || self.scheduled.swap(true, std::sync::atomic::Ordering::AcqRel) {
            return;
        }
        let runner = self.clone();
        self.pool.submit(Lane::Io, Box::new(move || runner.run_turn()));
    }

    pub(super) fn run_turn(self: std::sync::Arc<Self>) {
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

    pub(super) fn schedule(self: &std::sync::Arc<Self>) {
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

    pub(super) fn run_turn(self: std::sync::Arc<Self>) {
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

//#endregion ShellTypes

