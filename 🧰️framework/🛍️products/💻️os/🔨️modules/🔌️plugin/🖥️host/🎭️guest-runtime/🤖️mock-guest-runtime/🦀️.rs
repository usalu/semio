//! 🤖️ Mock guest runtime: the scripted outcomes, the mock instance state and job step gate and the scripted mock guest runtime with its runtime implementation. See [`super`].

use super::*;

//#region 🎭️GuestRuntime
//#region 🔖️MockGuestRuntime
/// 🎬️ One actor's scripted future: either the next `execute_turn`/`step_job` call returns this
/// `TurnResult`/`JobStep`, or the runtime raises this fault instead. `#[allow(dead_code)]`, not
/// `#[cfg(any(test, feature = "testkit"))]`, for the same reason `GuestInstanceState::Mock` isn't `cfg(test)`-gated either —
/// see that variant's own doc comment.
#[allow(dead_code)]
enum ScriptedOutcome {
    Turn(Box<TurnResult>),
    Job(JobStep),
    #[cfg(any(test, feature = "testkit"))]
    PendingJob {
        gate: Arc<MockJobStepGate>,
        step: JobStep,
    },
    Fault(String),
    GuestFault(semio_framework::Fault),
    /// 🛑️ terra-shard-lane piece 2: distinct from `Fault(String)` (which always becomes
    /// `TurnFault::Trapped`) — this scripts the SPECIFIC `TurnFault::DeadlineExceeded` variant a
    /// real `WasmtimeRuntime::execute_turn` raises when an epoch deadline armed from
    /// `budget.deadline_ms` is hit, so shard-level tests can prove `ShardLoop::pump_primed`
    /// converts it into a graceful `ShardOutcome::Turn{status: MoreWork}` instead of a
    /// `ShardOutcome::Fault` — see `📓️terra-shard-lane-report.md`.
    DeadlineExceeded,
}

/// 📦️ The scripted body of one finished mock job step, offered once as a single page before its source closes.
pub(super) struct MockJobOutput {
    state: OriginalJobOutputState,
    bytes: Vec<u8>,
    offered: bool,
}

/// 🧾️ Wraps one scripted result into a submitted original guest call that spent no instructions.
pub(super) fn mock_job_call<T>(execution: OriginalGuestExecutionBudget, original: Result<T, TurnFault>) -> OriginalGuestJobCall<T> {
    OriginalGuestJobCall { original, execution: OriginalGuestExecutionReceipt { budget: execution, spent_fuel: Some(0), submitted: true }, returned_unsubmitted_source: None }
}

#[derive(Default)]
pub struct MockInstanceState {
    pub actor_context:semio_framework_plugin_guest_contract::original_actor_context::OriginalActorGuestEntry,
    pub(super) outputs: HashMap<u64, MockJobOutput>,
    #[allow(dead_code)]
    pub checkpoint: Option<Vec<u8>>,
}

#[cfg(any(test, feature = "testkit"))]
#[derive(Default)]
pub struct MockJobStepGate {
    pub(super) ready: AtomicBool,
    pub(crate) polls: std::sync::atomic::AtomicUsize,
    pub waker: Mutex<Option<std::task::Waker>>,
}

#[cfg(any(test, feature = "testkit"))]
impl MockJobStepGate {
    pub async fn wait(&self) {
        std::future::poll_fn(|context| {
            self.polls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if self.ready.load(std::sync::atomic::Ordering::Acquire) {
                std::task::Poll::Ready(())
            } else {
                *self.waker.lock().expect("mock job gate lock poisoned") = Some(context.waker().clone());
                std::task::Poll::Pending
            }
        })
        .await
    }

    pub fn release(&self) {
        self.ready.store(true, std::sync::atomic::Ordering::Release);
        if let Some(waker) = self.waker.lock().expect("mock job gate lock poisoned").take() {
            waker.wake();
        }
    }

    pub fn is_waiting(&self) -> bool {
        self.polls.load(std::sync::atomic::Ordering::SeqCst) > 0
    }
}

/// 🎭️ `design-runtime.md` §2's `MockGuestRuntime` (`#[cfg(any(test, feature = "testkit"))]`): scripted turns + a controllable
/// clock, backing scheduler/failure-ladder tests without a real wasm component or `bindgen!` — the
/// TS twin is `createMockShard()`. `script_turn`/`script_fault` queue outcomes FIFO per actor;
/// `execute_turn` on an actor with an empty queue returns `TurnFault::Exhausted` rather than
/// silently fabricating an idle turn, so a test that forgets to script a call fails loudly instead
/// of passing by accident.
#[cfg(any(test, feature = "testkit"))]
pub struct MockGuestRuntime {
    pub fail_instantiate: AtomicBool,
    refuse_instantiate: Mutex<Option<semio_framework::Fault>>,
    pub instantiate_admissions: std::sync::atomic::AtomicUsize,
    pub drop_admissions: std::sync::atomic::AtomicUsize,
    pub now_ms: std::sync::atomic::AtomicI64,
    pub(super) scripts: Mutex<HashMap<u64, VecDeque<ScriptedOutcome>>>,
    pub start_admissions: std::sync::atomic::AtomicUsize,
    pub step_admissions: std::sync::atomic::AtomicUsize,
    pub cancel_admissions: std::sync::atomic::AtomicUsize,
    fail_start: AtomicBool,
    fail_cancel: AtomicBool,
    panic_start: AtomicBool,
    panic_step: AtomicBool,
    panic_cancel: AtomicBool,
    relay_start_failure_release: Mutex<Option<Arc<MockJobStepGate>>>,
    relay_step_failure_release: Mutex<Option<Arc<MockJobStepGate>>>,
    relay_cancel_release: Mutex<Option<Arc<MockJobStepGate>>>,
    /// 📼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1): every `events` slice `execute_turn` was
    /// ever called with, per actor, in call order — `_events` used to be ignored entirely. A test
    /// proving a job's completion actually reaches the ORIGINATING actor (not just that
    /// `step_job` returned `Done`) needs to see the synthesized `Event::JobCompleted` really was
    /// handed to a later `execute_turn` call, which this makes assertable without a real guest.
    pub observed_events: Mutex<HashMap<u64, Vec<Event>>>,
    pub observed_turns: Mutex<Vec<(u64, Vec<Event>, Budget)>>,
}

#[cfg(any(test, feature = "testkit"))]
impl Default for MockGuestRuntime {
    fn default() -> Self {
        Self {
            fail_instantiate: AtomicBool::new(false),
            refuse_instantiate: Mutex::new(None),
            instantiate_admissions: std::sync::atomic::AtomicUsize::new(0),
            drop_admissions: std::sync::atomic::AtomicUsize::new(0),
            now_ms: std::sync::atomic::AtomicI64::new(0),
            scripts: Mutex::new(HashMap::new()),
            start_admissions: std::sync::atomic::AtomicUsize::new(0),
            step_admissions: std::sync::atomic::AtomicUsize::new(0),
            cancel_admissions: std::sync::atomic::AtomicUsize::new(0),
            fail_start: AtomicBool::new(false),
            fail_cancel: AtomicBool::new(false),
            panic_start: AtomicBool::new(false),
            panic_step: AtomicBool::new(false),
            panic_cancel: AtomicBool::new(false),
            relay_start_failure_release: Mutex::new(None),
            relay_step_failure_release: Mutex::new(None),
            relay_cancel_release: Mutex::new(None),
            observed_events: Mutex::new(HashMap::new()),
            observed_turns: Mutex::new(Vec::new()),
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
impl MockGuestRuntime {
    pub async fn new() -> Self {
        Self::default()
    }

    pub async fn now_ms(&self) -> i64 {
        self.now_ms.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub async fn set_now_ms(&self, ms: i64) {
        self.now_ms.store(ms, std::sync::atomic::Ordering::SeqCst);
    }

    pub async fn advance_ms(&self, delta: i64) {
        self.now_ms.fetch_add(delta, std::sync::atomic::Ordering::SeqCst);
    }

    async fn queue_for(&self, actor: RuntimeActorId) -> std::sync::MutexGuard<'_, HashMap<u64, VecDeque<ScriptedOutcome>>> {
        let mut scripts = self.scripts.lock().expect("mock runtime lock poisoned");
        scripts.entry(actor.0).or_default();
        scripts
    }

    pub async fn script_turn(&self, actor: RuntimeActorId, result: TurnResult) {
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::Turn(Box::new(result)));
    }

    /// 🤝️ The next `instantiate` refuses the guest at admission with `fault`, as a real runtime refuses a guest of
    /// another app channel (`PluginHostError::Refused`).
    pub async fn script_instantiate_refusal(&self, fault: semio_framework::Fault) {
        *self.refuse_instantiate.lock().expect("mock runtime lock poisoned") = Some(fault);
    }

    pub async fn script_guest_fault(&self, actor: RuntimeActorId, fault: semio_framework::Fault) {
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::GuestFault(fault));
    }

    pub async fn script_job_step(&self, actor: RuntimeActorId, step: JobStep) {
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::Job(step));
    }

    pub async fn script_pending_job_step(&self, actor: RuntimeActorId, step: JobStep) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::PendingJob { gate: Arc::clone(&gate), step });
        gate
    }

    pub fn script_pending_cancel_job(&self) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        *self.relay_cancel_release.lock().expect("mock cancel release lock poisoned") = Some(Arc::clone(&gate));
        gate
    }

    pub fn start_admissions(&self) -> usize {
        self.start_admissions.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn step_admissions(&self) -> usize {
        self.step_admissions.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn cancel_admissions(&self) -> usize {
        self.cancel_admissions.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn panic_next_start(&self) {
        self.panic_start.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn fail_next_start(&self) {
        self.fail_start.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn panic_next_step(&self) {
        self.panic_step.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn panic_next_cancel(&self) {
        self.panic_cancel.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn fail_next_cancel(&self) {
        self.fail_cancel.store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn pause_after_next_start_failure_release(&self) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        *self.relay_start_failure_release.lock().expect("mock start release barrier lock poisoned") = Some(Arc::clone(&gate));
        gate
    }

    pub fn pause_after_next_step_failure_release(&self) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        *self.relay_step_failure_release.lock().expect("mock step release barrier lock poisoned") = Some(Arc::clone(&gate));
        gate
    }

    pub fn take_relay_start_failure_release(&self) -> Option<Arc<MockJobStepGate>> {
        self.relay_start_failure_release.lock().expect("mock start release barrier lock poisoned").take()
    }

    pub fn take_relay_step_failure_release(&self) -> Option<Arc<MockJobStepGate>> {
        self.relay_step_failure_release.lock().expect("mock step release barrier lock poisoned").take()
    }

    pub async fn script_fault(&self, actor: RuntimeActorId, message: impl Into<String>) {
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::Fault(message.into()));
    }

    /// 🛑️ terra-shard-lane piece 2: schedules `TurnFault::DeadlineExceeded` specifically — see
    /// `ScriptedOutcome::DeadlineExceeded`'s own doc for why this is not just `script_fault` with a
    /// fixed message.
    pub async fn script_deadline_exceeded(&self, actor: RuntimeActorId) {
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::DeadlineExceeded);
    }

    /// 🏁️ A plain `Idle`, no-effects, no-patches turn result — convenience for tests that only
    /// care about scheduling/backpressure, not turn content.
    pub async fn idle_turn(original:semio_framework::kernel::RetainedTurnInput) -> TurnResult {
        TurnResult {
            retained_receipt:original.return_original(semio_framework_value::RetainedCloneProgress::default()).expect("scripted idle has no original retained effects"),
            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
            effects: Vec::new(),
            presence: Vec::new(),
            next_wake: None,
            status: TurnStatus::Idle,
            fuel_used: 0,
            lifecycle_receipt: None,
            ui_patch_receipt: None,
            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
        }
    }

    /// 📼️ Every `events` slice `execute_turn` has been called with for `actor`, flattened across
    /// every call, in order — see `observed_events`'s own doc comment for why this exists.
    pub async fn observed_events(&self, actor: RuntimeActorId) -> Vec<Event> {
        self.observed_events.lock().expect("mock runtime lock poisoned").get(&actor.0).cloned().unwrap_or_default()
    }
}

#[cfg(any(test, feature = "testkit"))]
impl GuestRuntime for MockGuestRuntime {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{let _=(budget,identity);let GuestInstanceState::Mock(state)=&mut inst.state else{return Err(identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};Ok(semio_framework_actor::retained_context::OriginalActorAdmissionReply::borrow_original(&state.actor_context.admit_step(input)))}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{let _=(budget,identity);let GuestInstanceState::Mock(state)=&mut inst.state else{return Err(identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(4)))};Ok(semio_framework_actor::retained_context::OriginalActorCloseReply::borrow_original(&state.actor_context.close_step(input)))}

    async fn compile(&self, package: &PackageRef, _bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        Ok(CompiledHandle { package_hash: package.hash.0, component: None, owned: None })
    }

    async fn instantiate(&self, _compiled: &CompiledHandle, actor: RuntimeActorId, _caps: &[BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        self.instantiate_admissions.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        if let Some(fault) = self.refuse_instantiate.lock().expect("mock runtime lock poisoned").take() {
            return Err(PluginHostError::Refused(Box::new(fault)));
        }
        if self.fail_instantiate.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return Err(PluginHostError::Plugin("scripted instance creation failure".into()));
        }
        drop(self.queue_for(actor));
        Ok(GuestInstance { actor, state: GuestInstanceState::Mock(MockInstanceState::default()) })
    }

    /// 👶️ host-dedyn: identical body to before this packet — no suspension point, so this
    /// resolves on its very first poll, same contract `GuestRuntime`'s own doc comment names.
    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], _budget: Budget, _identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        self.observed_turns.lock().expect("mock turn trace").push((inst.actor.0, events.to_vec(), _budget));
        self.observed_events.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?.entry(inst.actor.0).or_default().extend_from_slice(events);
        let mut scripts = self.scripts.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?;
        let queue = scripts.entry(inst.actor.0).or_default();
        match queue.pop_front() {
            Some(ScriptedOutcome::Turn(result)) => Ok(*result),
            Some(ScriptedOutcome::Job(_)) => Err(TurnFault::Trapped("scripted outcome was a job step, not a turn".to_string())),
            Some(ScriptedOutcome::PendingJob { .. }) => Err(TurnFault::Trapped("scripted outcome was a pending job step, not a turn".to_string())),
            Some(ScriptedOutcome::Fault(message)) => Err(TurnFault::Trapped(message)),
            Some(ScriptedOutcome::GuestFault(fault)) => Err(TurnFault::Guest(fault)),
            Some(ScriptedOutcome::DeadlineExceeded) => Err(TurnFault::DeadlineExceeded),
            None => Err(TurnFault::Exhausted),
        }
    }

    /// 🎬️ `start-job` has no interesting success payload, so normal outcomes do not consume
    /// `ScriptedOutcome`; dedicated one-shot failure/panic flags exercise relay recovery directly.
    async fn start_job(&self,inst:&mut GuestInstance,source:Option<OriginalJobSource>,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStartReply>{
        self.start_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        assert!(!self.panic_start.swap(false, std::sync::atomic::Ordering::AcqRel), "scripted start-job panic after guest instance acquisition");
        if self.fail_start.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return OriginalGuestJobCall::unsubmitted(execution, source, TurnFault::Trapped("scripted start-job failure".to_string()));
        }
        match self.scripts.lock() {
            Ok(scripts) if scripts.contains_key(&inst.actor.0) => {}
            Ok(_) => return OriginalGuestJobCall::unsubmitted(execution, source, TurnFault::Exhausted),
            Err(_) => return OriginalGuestJobCall::unsubmitted(execution, source, TurnFault::Host(PluginHostError::LockPoisoned("mock runtime"))),
        }
        mock_job_call(execution, Ok(OriginalJobStartReply { input: budget.input, spent: Default::default(), work_units_spent: 0, admitted: true, returned_source: None, refusal: None }))
    }

    async fn step_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStepReply<OriginalJobOutcome>>{
        self.step_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        assert!(!self.panic_step.swap(false, std::sync::atomic::Ordering::AcqRel), "scripted step-job panic after guest instance acquisition");
        let scripted = match self.scripts.lock() {
            Ok(mut scripts) => scripts.entry(inst.actor.0).or_default().pop_front(),
            Err(_) => return mock_job_call(execution, Err(TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))),
        };
        let step = match scripted {
            Some(ScriptedOutcome::Job(step)) => step,
            Some(ScriptedOutcome::PendingJob { gate, step }) => {
                gate.wait().await;
                step
            }
            Some(ScriptedOutcome::Turn(_)) => return mock_job_call(execution, Err(TurnFault::Trapped("scripted outcome was a turn, not a job step".to_string()))),
            Some(ScriptedOutcome::Fault(message)) => return mock_job_call(execution, Err(TurnFault::Trapped(message))),
            Some(ScriptedOutcome::GuestFault(fault)) => return mock_job_call(execution, Err(TurnFault::Guest(fault))),
            Some(ScriptedOutcome::DeadlineExceeded) => return mock_job_call(execution, Err(TurnFault::DeadlineExceeded)),
            None => return mock_job_call(execution, Err(TurnFault::Exhausted)),
        };
        let (state, bytes) = match step {
            JobStep::Running { progress } => (OriginalJobOutputState::Running, progress.unwrap_or_default()),
            JobStep::Done { output } => (OriginalJobOutputState::Done, output),
            JobStep::Failed { error } => (OriginalJobOutputState::Failed, error),
        };
        let GuestInstanceState::Mock(mock) = &mut inst.state else {
            return mock_job_call(execution, Err(TurnFault::Trapped("MockGuestRuntime::step_job called on a non-mock GuestInstance".to_string())));
        };
        let outcome = OriginalJobOutcome { job, operation: budget.input.operation, generation: budget.input.generation, state, byte_length: bytes.len() as u64 };
        mock.outputs.insert(job, MockJobOutput { state, bytes, offered: false });
        mock_job_call(execution, Ok(OriginalJobStepReply { input: budget.input, spent: Default::default(), work_units_spent: 0, outcome: Some(outcome), source_closed: false, refusal: None }))
    }

    /// 📦️ The scripted body is offered as one page, then the output reports itself closed.
    async fn read_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputReply>{
        let GuestInstanceState::Mock(mock) = &mut inst.state else {
            return mock_job_call(execution, Err(TurnFault::Trapped("MockGuestRuntime::read_job_output called on a non-mock GuestInstance".to_string())));
        };
        let Some(output) = mock.outputs.get_mut(&job) else {
            return mock_job_call(execution, Ok(OriginalJobOutputReply { input: budget.input, spent: Default::default(), work_units_spent: 0, chunk: None, source_closed: false, refusal: Some(OriginalJobCallFailure::Authority(semio_framework_value::ValueRefusalKind::OwnershipLimit)) }));
        };
        let chunk = if output.bytes.is_empty() || output.offered {
            OriginalJobOutputChunk { offset: output.bytes.len() as u64, bytes: None, output_closed: true }
        } else {
            output.offered = true;
            OriginalJobOutputChunk { offset: 0, bytes: Some(output.bytes.clone()), output_closed: false }
        };
        mock_job_call(execution, Ok(OriginalJobOutputReply { input: budget.input, spent: Default::default(), work_units_spent: 0, chunk: Some(chunk), source_closed: false, refusal: None }))
    }

    async fn acknowledge_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputAckReply>{
        let GuestInstanceState::Mock(mock) = &mut inst.state else {
            return mock_job_call(execution, Err(TurnFault::Trapped("MockGuestRuntime::acknowledge_job_output called on a non-mock GuestInstance".to_string())));
        };
        let output = mock.outputs.remove(&job);
        let closed = output.as_ref().is_some_and(|output| output.state != OriginalJobOutputState::Running);
        mock_job_call(execution, Ok(OriginalJobOutputAckReply { input: budget.input, spent: Default::default(), work_units_spent: 0, acknowledged: output.is_some(), source_closed: closed, refusal: None }))
    }

    /// 🛑️ Cancellation succeeds without consuming the step queue by default; dedicated
    /// one-shot failure/panic flags prove callers retain or retire uncertain guest state.
    async fn cancel_job(&self,_inst:&mut GuestInstance,_job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{
        self.cancel_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        assert!(!self.panic_cancel.swap(false, std::sync::atomic::Ordering::AcqRel), "scripted cancel-job panic after guest instance acquisition");
        if self.fail_cancel.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return mock_job_call(execution, Err(TurnFault::Trapped("scripted cancel-job failure".to_string())));
        }
        let release = self.relay_cancel_release.lock().expect("mock cancel release lock poisoned").take();
        if let Some(release) = release {
            release.wait().await;
        }
        mock_job_call(execution, Ok(OriginalJobCancelReply { input: budget.input, spent: Default::default(), work_units_spent: 0, admitted: true, refusal: None }))
    }

    async fn close_jobs(&self,inst:&mut GuestInstance,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{
        if let GuestInstanceState::Mock(mock) = &mut inst.state {
            mock.outputs.clear();
        }
        mock_job_call(execution, Ok(OriginalJobCancelReply { input: budget.input, spent: Default::default(), work_units_spent: 0, admitted: true, refusal: None }))
    }

    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Mock(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("MockGuestRuntime::checkpoint called on a non-mock GuestInstance".to_string()));
        };
        let bytes = format!("mock-checkpoint:{}", inst.actor.0).into_bytes();
        state.checkpoint = Some(bytes.clone());
        Ok(bytes)
    }

    async fn restore(&self, inst: &mut GuestInstance, state: &[u8], identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
        let GuestInstanceState::Mock(mock_state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("MockGuestRuntime::restore called on a non-mock GuestInstance".to_string()));
        };
        mock_state.checkpoint = Some(state.to_vec());
        Ok(())
    }

    async fn drop_instance(&self, inst: GuestInstance) {
        self.drop_admissions.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.scripts.lock().map(|mut scripts| scripts.remove(&inst.actor.0)).ok();
    }
}
//#endregion 🔖️MockGuestRuntime
//#endregion 🎭️GuestRuntime
