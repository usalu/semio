//! 🎭️ The guest runtime of the plugin host: the `GuestRuntime` trait, the scripted mock, the owned interpreter runtime and the wasmtime runtime with its generated actor bindings.

use super::*;

//#region 🎭️GuestRuntime
/// 🧬️ `design-runtime.md` §2's `GuestRuntime` trait, verbatim. Replaces regions `🔖️WasmPluginRuntime`
/// and `🔖️ExtensionRuntime` (below) — NOT deleted yet: `WasmtimeRuntime`'s `impl GuestRuntime` needs
/// `wasmtime::component::bindgen!` to compile against the new `actor` world, which is currently
/// broken (see `📓️terra-B1-host-native-report.md`, `blocked-on-A2`: `📜️wit/📜️effects.wit:44`'s
/// `stream: bool` field uses a WIT-reserved keyword, so BOTH this file's `bindgen!` calls —
/// `plugin-world` and `extension-world`, themselves stale names since `world.wit` now only
/// declares `actor` — fail to parse). Landing the trait + `MockGuestRuntime` now, without
/// `WasmtimeRuntime`, still unblocks every downstream packet (H1-H4, T1) that only needs to code
/// against the interface.
///
/// `RuntimeActorId` never shadows `kernel::ActorId` (`📌️important.md`'s naming-hazard note) — this
/// file does not import `kernel::ActorId` at all, only `semio_framework_actor::ActorId` (aliased
/// `RuntimeActorId` in the top-of-file `use`).
/// 📦️ What [`GuestRuntime::compile`] compiles — a package identity plus the content hash that also
/// keys the compiled-artifact cache (`shared_engine_config_hash`/`compiled_cache_path` above).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageRef {
    pub package: PackageId,
    pub hash: PackageHash,
}

/// 🧩️ A compiled, not-yet-instantiated component. `component` is `None` for [`MockGuestRuntime`]
/// (which never compiles real wasm) and `Some` for [`WasmtimeRuntime`] (`//#region 🐎️WasmtimeRuntime`
/// below).
#[derive(Clone)]
pub struct CompiledHandle {
    pub package_hash: [u8; 32],
    pub(super) component: Option<Arc<Component>>,
    pub(super) owned: Option<OwnedCompiledGuest>,
}

impl CompiledHandle {
    /// 📏️ The bytes this compiled guest holds beyond its parsed component: the owned interpreter's codec origin once a
    /// codec call assembled it ([`OwnedCompiledGuest::codec_origin_bytes`]), else none. A residency budget charges a compiled
    /// guest this on top of its component.
    pub fn codec_origin_bytes(&self) -> u64 {
        self.owned.as_ref().map_or(0, OwnedCompiledGuest::codec_origin_bytes)
    }
}

impl std::fmt::Debug for CompiledHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledHandle").field("package_hash", &hex_encode(&self.package_hash)).field("has_component", &self.component.is_some()).field("has_owned_actor", &self.owned.is_some()).finish()
    }
}

/// 🏃️ One running actor instance, host-owned. `Mock(..)` backs [`MockGuestRuntime`]; `Wasmtime(..)`
/// backs [`WasmtimeRuntime`] (`//#region 🐎️WasmtimeRuntime` below).
pub struct GuestInstance {
    pub actor: RuntimeActorId,
    pub(super) state: GuestInstanceState,
}

#[path="../🏗️instance/🦀️.rs"]
mod original_instance_opening;
pub use original_instance_opening::GuestActorOpening;

impl GuestInstance {
    pub fn bind_original_clock_from_source(&mut self,source:&semio_framework_job::OriginalCallerSourceLoan)->Result<(),semio_framework_value::ValueError>{let(binding,clock,_)=source.original_clock_source()?;match &mut self.state{GuestInstanceState::Owned(state)=>state.bind_original_clock_from_source(source),GuestInstanceState::Wasmtime(state)=>{state.store.data_mut().original_job_clock=Some((binding,clock));Ok(())},_=>Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original clock provider requires its actual live guest"))}}
    pub fn original_clock_observation(&self,binding:u64)->Option<u64>{match &self.state{GuestInstanceState::Owned(state)=>state.original_clock_observation(binding),GuestInstanceState::Wasmtime(state)=>state.store.data().original_job_clock.filter(|(original,_)|*original==binding).and_then(|(_,clock)|clock()),_=>None}}
    pub fn release_original_clock(&mut self,binding:u64)->bool{match &mut self.state{GuestInstanceState::Owned(state)=>state.release_original_clock(binding),GuestInstanceState::Wasmtime(state)=>{let original=&mut state.store.data_mut().original_job_clock;if original.is_some_and(|source|source.0==binding){*original=None;true}else{false}},_=>false}}
    /// 🎟️ The paid Core retains the exact supplied startup input without inferring it from the actor identifier.
    /// 👁️ Borrows funded original memory while the complete instance remains owned.
    pub fn original_memory(&self,index:u32)->Option<&[u8]>{match &self.state{GuestInstanceState::Owned(owner)=>owner.original_memory(index),_=>None}}
    /// 🚪️ Reports the actual terminal state reached only by paid complete instance destruction.
    pub fn is_closed(&self)->bool{matches!(self.state,GuestInstanceState::Closed)}
    pub fn original_startup_input(&self)->Option<semio_framework_actor::RetainedTurnInput>{match &self.state{GuestInstanceState::Owned(state)=>state.original_startup_input(),_=>None}}
    /// 🔁️ True while an owned `poll` has not completed — the guest yielded on its fuel or deadline
    /// and owns the host's next call. A host that wants to submit NEW events must first resume with
    /// none until this is false; `execute_actor_turn` refuses the alternative rather than dropping
    /// them (ticket 26/09/18 slice A2). A compiled instance is never in flight: its turn is atomic.
    pub fn turn_in_flight(&self) -> bool {
        matches!(&self.state, GuestInstanceState::Owned(state) if state.turn_in_flight())
    }

    /// 📈️ The guest's linear-memory size in bytes, read off the store's own [`BudgetLimiter`]
    /// witness — `None` for a mock instance, which owns no wasm memory. Wasm memory never shrinks,
    /// so sampling this once per turn IS the growth curve a leak law needs.
    pub fn guest_linear_memory_bytes(&self) -> Option<usize> {
        match &self.state {
            GuestInstanceState::Wasmtime(state) => Some(state.store.data().limiter.peak_memory_bytes),
            GuestInstanceState::Mock(_) | GuestInstanceState::Owned(_) | GuestInstanceState::Closed => None,
        }
    }

    /// 🩺️ Everything this guest has written to `wasi:cli/stderr` so far — `None` unless the host's
    /// own diagnostics were armed when the instance was built, because that is the only condition
    /// under which the guest is given an environment that makes it print at all (see
    /// [`guest_wasi_ctx`]). This is the wasm-hosted counterpart of reading the browser console: the
    /// SAME `[TRACE]`/`[BUDGET]` lines the native in-process suites print, now observable from a
    /// real `wasm32-wasip2` component.
    pub fn guest_diagnostics_text(&self) -> Option<String> {
        match &self.state {
            GuestInstanceState::Wasmtime(state) => state.store.data().diagnostics.as_ref().map(|pipe| String::from_utf8_lossy(&pipe.contents()).into_owned()),
            GuestInstanceState::Owned(state) => Some(String::from_utf8_lossy(state.diagnostics()).into_owned()),
            GuestInstanceState::Mock(_) | GuestInstanceState::Closed => None,
        }
    }
}

/// 🧬️ Shallow, actor-id-only — `WasmtimeInstanceState.store: Store<ActorHostState>` and
/// `bindings: actor_bindings::Actor` implement neither `Debug` (wasmtime's `Store` deliberately
/// doesn't; bindgen-generated binding structs don't either), so `#[derive(Debug)]` cannot reach
/// through them. Callers (`Result::expect_err` on a `GuestInstance`-returning call, mainly) only ever
/// need enough to identify WHICH instance failed.
impl std::fmt::Debug for GuestInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GuestInstance").field("actor", &self.actor).finish()
    }
}

pub(super) enum GuestInstanceState {
    Closed,
    /// 🧪️ Only `MockGuestRuntime` (`#[cfg(test)]`) ever constructs this variant — `#[allow(dead_code)]`
    /// rather than `#[cfg(test)]` on the variant itself, so `GuestInstanceState` stays a REFUTABLE
    /// match target (`Wasmtime` is not the only variant) in every build config; a `#[cfg(test)]`
    /// variant would make the `let GuestInstanceState::Wasmtime(state) = .. else { .. }` guards in
    /// `WasmtimeRuntime`'s own methods irrefutable outside `cfg(test)`, turning their `else` arms into
    /// a NEW dead-code warning instead of fixing one.
    #[allow(dead_code)]
    Mock(MockInstanceState),
    Owned(OwnedGuestInstance),
    Wasmtime(WasmtimeInstanceState),
}


/// 🧯️ A Wasmtime original-operation receive failure: a refused admission keeps its typed refusal, any other runtime error traps.
pub(super) fn receiving_turn_fault(error:operation::wasmtime::ReceiveError)->TurnFault{match error{operation::wasmtime::ReceiveError::Refused(error)=>identity_turn_fault(error),operation::wasmtime::ReceiveError::Runtime(error)=>TurnFault::Trapped(error.to_string())}}

fn decode_guest_plugin_error(error: wit_types::PluginError) -> TurnFault {
    match error {
        wit_types::PluginError::Fault(bytes) => decode_guest_fault_bytes(&bytes),
        wit_types::PluginError::OperationRefusal(code)=>identity_turn_fault(crate::operation_authority::refusal(code)),
    }
}

/// ♻️ A guest lifecycle turn that ran past its own strict µs authority and RETAINED its receipt —
/// progress is kept, so the only correct host answer is another turn. `pub` because every host that
/// opens an instance owes the guest this retry: the shard loop, and (since ticket 26/09/18 slice A2)
/// the MCP gateway, whose `ensure_instance` used to surface it to the agent as a failed tool call.
/// Unavoidable under the owned interpreter, which executes a lifecycle turn orders of magnitude
/// slower than the JIT the contract was measured against.
pub fn retryable_lifecycle_turn(fault: &TurnFault, events: &[Event]) -> bool {
    matches!(fault, TurnFault::Guest(fault) if fault.code.0 == "plugin.reactor-turn-deadline" && fault.retryable)
        && events.len() <= 1
        && events.iter().all(|event| matches!(event, Event::InstanceOpen { .. } | Event::InstanceClose(_) | Event::InstanceLifecycleAck(_)))
}

#[cfg(test)]
#[path = "../🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs"]
mod guest_fault_tests;

#[path = "../📥️cold-pair/🦀️.rs"]
pub(super) mod cold_pair;
use cold_pair::{kernel_cold_page_to_wit, wit_cold_ingress_to_kernel};

/// 🐎️ Host-side driver for one actor's execution — `design-runtime.md` §2. `WasmtimeRuntime` (the
/// native implementation, backed by [`build_shared_engine`]/[`EpochDeadlines`]/[`BudgetLimiter`]/the
/// compiled-artifact cache above), `MockGuestRuntime` (test double, below), and
/// `shard::RecordingRuntime` (`🧵️shard`'s own test double) all implement this; nothing else in the
/// host — `ShardLoop`, the task manager, `WasmtimeNodeHost` — talks to a guest through any other
/// surface. The closed set of impls is dispatched through the [`GuestRuntimes`] enum, never `dyn`
/// (O1/R1 — `async fn` in a trait is not dyn-compatible, and dyn-Future erasure in trait-method
/// return position is banned).
///
/// 👶️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (host-dedyn): the turn/job/checkpoint methods are
/// plain `async fn` returning their `Result` directly — the double-future `HostFuture<Result<..>>`
/// wrapper terra-trait-asyncify staged here is gone (R1: `dyn Future` is banned from trait-method
/// return position). `compile`/`instantiate`/`drop_instance` stay plain, non-`async fn`: `compile`
/// is CPU-bound with no await point, `instantiate` only BUILDS a task spec for an async backend
/// (never runs one), and `drop_instance` is a destructor. `PluginInstanceHandle` treats every
/// implementation as genuinely suspending: it moves the guest instance into a retained-waker
/// future and polls it through the process worker pool without parking a worker or holding the
/// instance mutex across suspension.
pub trait GuestRuntime: Send + Sync {
    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError>;
    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError>;
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>;
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>;
    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault>;
    /// 📥️ A prepared original job source returns intact when guest admission refuses it.
    async fn start_job(&self,inst:&mut GuestInstance,source:Option<OriginalJobSource>,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStartReply>;
    /// ▶️ The actual guest reply retains its descriptor and complete original spending tuple.
    async fn step_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStepReply<OriginalJobOutcome>>;
    /// 📦️ Bounded original output pages precede their paid source closure acknowledgement.
    async fn read_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputReply>;
    /// 🤝️ Only the actual offered output identity can acknowledge its guest owner.
    async fn acknowledge_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputAckReply>;
    /// 🛑️ Original cancellation admission returns all actual work and retained effects.
    async fn cancel_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>;
    /// ♻️ Jobs and retained outputs close before their parent actor context.
    async fn close_jobs(&self,inst:&mut GuestInstance,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>;
    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError>;
    async fn restore(&self, inst: &mut GuestInstance, state: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError>;
    async fn drop_instance(&self, inst: GuestInstance);
}

//#region 🔖️MockGuestRuntime
/// 🎬️ One actor's scripted future: either the next `execute_turn`/`step_job` call returns this
/// `TurnResult`/`JobStep`, or the runtime raises this fault instead. `#[allow(dead_code)]`, not
/// `#[cfg(test)]`, for the same reason `GuestInstanceState::Mock` isn't `cfg(test)`-gated either —
/// see that variant's own doc comment.
#[allow(dead_code)]
enum ScriptedOutcome {
    Turn(Box<TurnResult>),
    Job(JobStep),
    #[cfg(test)]
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

#[derive(Default)]
pub(super) struct MockInstanceState {
    pub(super) actor_context:crate::original_actor_context::OriginalActorGuestEntry,
    #[allow(dead_code)]
    pub(super) checkpoint: Option<Vec<u8>>,
}

#[cfg(test)]
#[derive(Default)]
pub(super) struct MockJobStepGate {
    ready: AtomicBool,
    pub(super) polls: std::sync::atomic::AtomicUsize,
    pub(super) waker: Mutex<Option<std::task::Waker>>,
}

#[cfg(test)]
impl MockJobStepGate {
    pub(super) async fn wait(&self) {
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

    pub(super) fn release(&self) {
        self.ready.store(true, std::sync::atomic::Ordering::Release);
        if let Some(waker) = self.waker.lock().expect("mock job gate lock poisoned").take() {
            waker.wake();
        }
    }

    pub(super) fn is_waiting(&self) -> bool {
        self.polls.load(std::sync::atomic::Ordering::SeqCst) > 0
    }
}

/// 🎭️ `design-runtime.md` §2's `MockGuestRuntime` (`#[cfg(test)]`): scripted turns + a controllable
/// clock, backing scheduler/failure-ladder tests without a real wasm component or `bindgen!` — the
/// TS twin is `createMockShard()`. `script_turn`/`script_fault` queue outcomes FIFO per actor;
/// `execute_turn` on an actor with an empty queue returns `TurnFault::Exhausted` rather than
/// silently fabricating an idle turn, so a test that forgets to script a call fails loudly instead
/// of passing by accident.
#[cfg(test)]
pub struct MockGuestRuntime {
    pub(super) fail_instantiate: AtomicBool,
    refuse_instantiate: Mutex<Option<semio_framework::Fault>>,
    pub(super) instantiate_admissions: std::sync::atomic::AtomicUsize,
    pub(super) drop_admissions: std::sync::atomic::AtomicUsize,
    pub(super) now_ms: std::sync::atomic::AtomicI64,
    scripts: Mutex<HashMap<u64, VecDeque<ScriptedOutcome>>>,
    pub(super) start_admissions: std::sync::atomic::AtomicUsize,
    pub(super) step_admissions: std::sync::atomic::AtomicUsize,
    pub(super) cancel_admissions: std::sync::atomic::AtomicUsize,
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
    pub(super) observed_events: Mutex<HashMap<u64, Vec<Event>>>,
    pub(super) observed_turns: Mutex<Vec<(u64, Vec<Event>, Budget)>>,
}

#[cfg(test)]
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

#[cfg(test)]
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

    pub(super) async fn script_pending_job_step(&self, actor: RuntimeActorId, step: JobStep) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        self.queue_for(actor).await.get_mut(&actor.0).expect("just inserted").push_back(ScriptedOutcome::PendingJob { gate: Arc::clone(&gate), step });
        gate
    }

    pub(super) fn script_pending_cancel_job(&self) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        *self.relay_cancel_release.lock().expect("mock cancel release lock poisoned") = Some(Arc::clone(&gate));
        gate
    }

    pub(super) fn start_admissions(&self) -> usize {
        self.start_admissions.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub(super) fn step_admissions(&self) -> usize {
        self.step_admissions.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub(super) fn cancel_admissions(&self) -> usize {
        self.cancel_admissions.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub(super) fn panic_next_start(&self) {
        self.panic_start.store(true, std::sync::atomic::Ordering::Release);
    }

    pub(super) fn fail_next_start(&self) {
        self.fail_start.store(true, std::sync::atomic::Ordering::Release);
    }

    pub(super) fn panic_next_step(&self) {
        self.panic_step.store(true, std::sync::atomic::Ordering::Release);
    }

    pub(super) fn panic_next_cancel(&self) {
        self.panic_cancel.store(true, std::sync::atomic::Ordering::Release);
    }

    pub(super) fn fail_next_cancel(&self) {
        self.fail_cancel.store(true, std::sync::atomic::Ordering::Release);
    }

    pub(super) fn pause_after_next_start_failure_release(&self) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        *self.relay_start_failure_release.lock().expect("mock start release barrier lock poisoned") = Some(Arc::clone(&gate));
        gate
    }

    pub(super) fn pause_after_next_step_failure_release(&self) -> Arc<MockJobStepGate> {
        let gate = Arc::new(MockJobStepGate::default());
        *self.relay_step_failure_release.lock().expect("mock step release barrier lock poisoned") = Some(Arc::clone(&gate));
        gate
    }

    pub(super) fn take_relay_start_failure_release(&self) -> Option<Arc<MockJobStepGate>> {
        self.relay_start_failure_release.lock().expect("mock start release barrier lock poisoned").take()
    }

    pub(super) fn take_relay_step_failure_release(&self) -> Option<Arc<MockJobStepGate>> {
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

#[cfg(test)]
impl GuestRuntime for MockGuestRuntime {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{let _=(budget,identity);let GuestInstanceState::Mock(state)=&mut inst.state else{return Err(identity_turn_fault(crate::operation_authority::refusal(4)))};Ok(semio_framework_actor::retained_context::OriginalActorAdmissionReply::borrow_original(&state.actor_context.admit_step(input)))}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{let _=(budget,identity);let GuestInstanceState::Mock(state)=&mut inst.state else{return Err(identity_turn_fault(crate::operation_authority::refusal(4)))};Ok(semio_framework_actor::retained_context::OriginalActorCloseReply::borrow_original(&state.actor_context.close_step(input)))}

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
    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], _budget: Budget, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
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
    async fn start_job(&self, inst: &mut GuestInstance, _job: u64, _kind: &str, _input: Vec<u8>) -> Result<(), TurnFault> {
        self.start_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        assert!(!self.panic_start.swap(false, std::sync::atomic::Ordering::AcqRel), "scripted start-job panic after guest instance acquisition");
        if self.fail_start.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return Err(TurnFault::Trapped("scripted start-job failure".to_string()));
        }
        if !self.scripts.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?.contains_key(&inst.actor.0) {
            return Err(TurnFault::Exhausted);
        }
        Ok(())
    }

    async fn step_job(&self, inst: &mut GuestInstance, _job: u64, _budget: JobBudget) -> Result<JobStep, TurnFault> {
        self.step_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        assert!(!self.panic_step.swap(false, std::sync::atomic::Ordering::AcqRel), "scripted step-job panic after guest instance acquisition");
        let scripted = {
            let mut scripts = self.scripts.lock().map_err(|_| TurnFault::Host(PluginHostError::LockPoisoned("mock runtime")))?;
            scripts.entry(inst.actor.0).or_default().pop_front()
        };
        match scripted {
            Some(ScriptedOutcome::Job(step)) => Ok(step),
            Some(ScriptedOutcome::PendingJob { gate, step }) => {
                gate.wait().await;
                Ok(step)
            }
            Some(ScriptedOutcome::Turn(_)) => Err(TurnFault::Trapped("scripted outcome was a turn, not a job step".to_string())),
            Some(ScriptedOutcome::Fault(message)) => Err(TurnFault::Trapped(message)),
            Some(ScriptedOutcome::GuestFault(fault)) => Err(TurnFault::Guest(fault)),
            Some(ScriptedOutcome::DeadlineExceeded) => Err(TurnFault::DeadlineExceeded),
            None => Err(TurnFault::Exhausted),
        }
    }

    /// 🛑️ Cancellation succeeds without consuming the step queue by default; dedicated
    /// one-shot failure/panic flags prove callers retain or retire uncertain guest state.
    async fn cancel_job(&self, _inst: &mut GuestInstance, _job: u64) -> Result<(), TurnFault> {
        self.cancel_admissions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        assert!(!self.panic_cancel.swap(false, std::sync::atomic::Ordering::AcqRel), "scripted cancel-job panic after guest instance acquisition");
        if self.fail_cancel.swap(false, std::sync::atomic::Ordering::AcqRel) {
            return Err(TurnFault::Trapped("scripted cancel-job failure".to_string()));
        }
        let release = self.relay_cancel_release.lock().expect("mock cancel release lock poisoned").take();
        if let Some(release) = release {
            release.wait().await;
        }
        Ok(())
    }

    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Mock(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("MockGuestRuntime::checkpoint called on a non-mock GuestInstance".to_string()));
        };
        let bytes = format!("mock-checkpoint:{}", inst.actor.0).into_bytes();
        state.checkpoint = Some(bytes.clone());
        Ok(bytes)
    }

    async fn restore(&self, inst: &mut GuestInstance, state: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
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

#[cfg(test)]
#[path = "../🧪️tests/🔬️mock-guest-runtime/🦀️.rs"]
mod mock_guest_runtime_tests;
//#endregion 🔖️MockGuestRuntime

//#region 🧠️OwnedRuntime
/// 🧠️ The owned compiled guest of `compiled`; a handle another runtime compiled has none.
pub(super) fn owned_compiled(compiled: &CompiledHandle) -> Result<&OwnedCompiledGuest, PluginHostError> {
    compiled.owned.as_ref().ok_or_else(|| PluginHostError::Plugin("CompiledHandle has no repository-owned actor artifact".to_string()))
}

/// 🧠️ The owned instance of `inst`; an instance another runtime built has none.
fn owned_instance_mut(inst: &mut GuestInstance) -> Result<&mut OwnedGuestInstance, TurnFault> {
    match &mut inst.state {
        GuestInstanceState::Owned(state) => Ok(state),
        _ => Err(TurnFault::Trapped("owned runtime called on a non-owned GuestInstance".to_string())),
    }
}

/// 🧳️ Copies a component snapshot's dialect coordinate and payload into storage the caller's original authority admitted,
/// for the Wasmtime codec exports that take both by value.
pub(super) fn snapshot_component_transport<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(dialect:&str,payload:&[u8],identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>)->Result<(String,Vec<u8>),TurnFault>{
    identity.encode(|control|{
        control.checkpoint()?;
        control.charge(dialect.len().checked_add(payload.len()).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"component snapshot transport length overflow"))?)?;
        let mut coordinate=String::new();
        coordinate.try_reserve_exact(dialect.len()).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"component snapshot dialect storage refused"))?;
        coordinate.push_str(dialect);
        let mut bytes=Vec::new();
        bytes.try_reserve_exact(payload.len()).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"component snapshot payload storage refused"))?;
        bytes.extend_from_slice(payload);
        Ok::<_,semio_framework_value::ValueError>((coordinate,bytes))
    }).map_err(identity_turn_fault)
}

impl GuestRuntime for OwnedRuntime {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{self.admit_context(owned_instance_mut(inst)?,input,budget,identity)}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{self.close_context(owned_instance_mut(inst)?,input,budget,identity)}

    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        Ok(CompiledHandle { package_hash: package.hash.0, component: None, owned: Some(self.compile_component(bytes)?) })
    }

    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, _caps: &[BrokerCapabilityGrant], _budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        Ok(GuestInstance { actor, state: GuestInstanceState::Owned(self.instantiate_actor(owned_compiled(compiled)?)?) })
    }

    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        self.execute_actor_turn(owned_instance_mut(inst)?, events, budget, identity)
    }

    async fn start_job(&self, inst: &mut GuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {
        self.start_actor_job(owned_instance_mut(inst)?, job, kind, input)
    }

    async fn step_job(&self, inst: &mut GuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        self.step_actor_job(owned_instance_mut(inst)?, job, budget)
    }

    async fn cancel_job(&self, inst: &mut GuestInstance, job: u64) -> Result<(), TurnFault> {
        self.cancel_actor_job(owned_instance_mut(inst)?, job)
    }

    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Owned(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("checkpoint called on a non-owned GuestInstance".to_string()));
        };
        self.checkpoint_actor(state, identity)
    }

    async fn restore(&self, inst: &mut GuestInstance, checkpoint: &[u8], _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
        let GuestInstanceState::Owned(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("restore called on a non-owned GuestInstance".to_string()));
        };
        self.restore_actor(state, checkpoint)
    }

    async fn drop_instance(&self, _inst: GuestInstance) {}
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️owned-instance-open/🦀️.rs"]
mod owned_instance_open_tests;
//#endregion 🧠️OwnedRuntime

//#region 🐎️WasmtimeRuntime
/// 🧬️ The real `impl GuestRuntime for WasmtimeRuntime` — `design-runtime.md` §2. Nested `mod
/// actor_bindings` (mirrors the file's own `mod extension_bindings` idiom below, "wasmtime's
/// `bindgen!` cannot be invoked twice at the same module scope") so this coexists with the OLD
/// `plugin-world`/`extension-world` `bindgen!` calls until the deletion pass — both of which are
/// now unconditionally broken independent of this packet (their `world` names no longer exist in
/// `📜️world.wit`, which declares only `world actor`), a finding recorded in
/// `📓️terra-B1-host-native-report.md`.
pub(crate) mod actor_bindings {
    // 🐛️ `additional_derives` carries NEITHER `Debug` NOR `Clone`, each for its own reason.
    // `Debug`: wasmtime-wit-bindgen's `type_record`/`print_rust_enum` always emit a hand-written
    // `impl core::fmt::Debug` for every WIT `record`/`variant`/`enum` regardless of
    // `additional_derives`, so requesting it produces `E0119: conflicting implementations` for
    // every generated type — it is already available on all of them.
    // `Clone`: MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse) — `world actor` now
    // imports `host-async`, which carries `stream<u8>` (`blob-read`, `http-fetch`'s
    // `http-response.body`). That lowers to `StreamReader<u8>`, a one-shot resource handle that is
    // deliberately not `Clone`, and `additional_derives` applies blanket to every generated type
    // rather than only the plain data records. Nothing in this crate cloned a generated `wit_*`
    // value (verified by grep before removing the derive — every `.clone()` near a `wit_*`
    // constructor is on the KERNEL-side `String`/`Vec<u8>` being moved INTO it).
    //
    // 🧬️ This is the crate's ONE `bindgen!` invocation for `semio:framework` — `📥️imports/🦀️.rs`'s 24
    // `host-async` implementations re-export this module rather than generating a second, nominally
    // distinct copy of every type (which is what the two-world split forced before the collapse).
    wasmtime::component::bindgen!({
        world: "actor",
        path: "../../../🧬️schema",
    });
}

pub(crate) use actor_bindings::semio::framework::{capabilities as wit_capabilities, effects as wit_effects, events as wit_events, host_async as wit_host_async, instance_lifetime as wit_lifetime, types as wit_types, ui as wit_ui};
pub(super) use wasmtime::component::Accessor;
// 🧬️ `reactor`/`jobs` are `export`s of `world actor` (design-runtime.md §2's `execute_turn`/`step_job`
// exports), not `import`s, so their generated bindings live under `exports::` — unlike `pure`'s
// argument/return types (`capabilities`/`effects`/`events`/`types`/`ui`), which sit at the top level.
pub(super) use actor_bindings::exports::semio::framework::{jobs as wit_jobs, reactor as wit_reactor};

/// 🧬️ `design-runtime.md` §2's slimmed `HostState { plugin_id, actor, caps, effect_sink,
/// asset_map }` — `limiter` is an implementation necessity (`Store::limiter` needs somewhere to
/// read bounds from), not part of the design's literal 5-field list.
pub(super) struct ActorHostState {
    original_job_clock:Option<(u64,fn()->Option<u64>)>,
    operation_slot:operation::slot::OperationSlot,
    plugin_id: String,
    #[allow(dead_code)]
    actor: RuntimeActorId,
    #[allow(dead_code)]
    caps: Vec<BrokerCapabilityGrant>,
    /// 🚪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): where `host-async.emit`'s
    /// fire-and-forget effects land mid-turn. Held as the WIT variant, NOT the kernel `Effect`:
    /// `emit` is declared sync in the WIT (a deliberate one-way door), `wit_effect_to_kernel` is
    /// genuinely `async`, and pushing the raw variant lets `execute_turn` do the conversion where
    /// it can actually `.await` — no `block_on` bridge on a wasm host path (R4 forbids one there).
    emit_sink: Vec<wit_effects::Effect>,
    /// 🚪️ The `emit-patch` counterpart. Drained by `execute_turn` alongside `emit_sink`.
    emit_patch_sink: Vec<wit_ui::UiPatch>,
    #[allow(dead_code)]
    asset_map: HashMap<String, Vec<u8>>,
    limiter: BudgetLimiter,
    wasi_ctx: WasiCtx,
    /// 🩺️ The guest's own stderr, retained in memory, when diagnostics are armed — see
    /// [`guest_wasi_ctx`] and [`GuestInstance::guest_diagnostics_text`].
    diagnostics: Option<wasmtime_wasi::p2::pipe::MemoryOutputPipe>,
    resource_table: ResourceTable,
}

/// 🌐️ WASI Preview 2 for every real `wasm32-wasip2` component. `world actor` itself imports only
/// `pure` (`log`/`now-ms`/`trace-span`), but the Rust `wasm32-wasip2` target's own runtime shim
/// pulls in `wasi:io/poll` and friends transitively, so any genuinely-built plugin fails
/// instantiation with "component imports instance `wasi:io/poll@0.2.9`, but a matching
/// implementation was not found in the linker" unless the linker carries full WASI. The ctx is the
/// sandboxed default — no inherited stdio, filesystem, network or environment — matching this
/// crate's capability-gated stance; widen per [`BrokerCapabilityGrant`] only when a capability
/// actually needs real WASI access.
impl WasiView for ActorHostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView { ctx: &mut self.wasi_ctx, table: &mut self.resource_table }
    }
}

/// 🩺️ The one exception to that sandboxed-empty environment: `wasi:cli/environment` is the
/// schema-declared door a component's own `std::env::var` reads, and therefore the door a host arms
/// the GUEST's perf traces through. `semio_framework_trace::RUNTIME_DIAGNOSTICS_ENV` resolves lazily
/// and exactly once inside the guest, so handing it across at instantiate is enough. A host whose
/// own diagnostics are off hands across NOTHING, which is what keeps a release run clean with no
/// second switch and no build-profile cfg — the guest's `[TRACE]` sites stay compiled in and stay
/// silent. Mirrors the browser host's `localStorage`-seeded `getEnvironment` shim (ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-guest-tick-cost-2026-09-12.md` §4 rank 1).
///
/// 🩺️ A guest whose traces are armed must also have somewhere to write them: the sandboxed
/// default ctx drops `wasi:cli/stderr` on the floor, which is why a wasmtime-hosted guest's
/// `eprintln!` has never been readable the way the browser console reads it.
fn guest_wasi_ctx() -> (WasiCtx, Option<wasmtime_wasi::p2::pipe::MemoryOutputPipe>) {
    let mut builder = WasiCtxBuilder::new();
    if !semio_framework_job::runtime_diagnostics_enabled() {
        return (builder.build(), None);
    }
    let diagnostics = wasmtime_wasi::p2::pipe::MemoryOutputPipe::new(GUEST_DIAGNOSTICS_CAPACITY_BYTES);
    builder.env(semio_framework_job::RUNTIME_DIAGNOSTICS_ENV, "1").stderr(diagnostics.clone());
    (builder.build(), Some(diagnostics))
}


/// 🩺️ What a wasmtime trap actually WAS, for every async-lifted export of `world actor`.
///
/// `Display` on a wasmtime error is only its FIRST line ("error while executing at wasm backtrace:
/// …"); the trap code itself ("all fuel consumed by WebAssembly", "epoch deadline reached", "wasm
/// trap: unreachable executed", a `rust_oom` abort) lives in the source chain, which `Debug`
/// renders. `execute_turn` classified off `Debug` from the start; `start-job`/`step-job`/
/// `cancel-job` did not, so every fuel or epoch cut inside a guest JOB arrived as an unattributed
/// `Trapped(<backtrace>)` — the shape `📓️wi1-…md` §7.2 measured, where the same call answered a
/// different innermost frame on every run because a fuel cut lands on whatever instruction the
/// counter reached. One classifier for all four exports.
/// ⏱️ The host's WEDGED watchdog for ONE `start-job`/`step-job` crossing — NOT the guest's
/// cooperative step budget.
///
/// `JobBudget` carries TWO different quantities that were being spent as one. `deadline_ms`/`fuel`
/// are the grant the GUEST's own bounded state machine reads and yields against (`💼️jobs`' work-unit
/// prices; `BatchDriveConfig.step_budget_us`); the wasmtime store's epoch deadline and fuel are the
/// host's hard kill, after which the component instance is unrecoverable ("cannot enter component
/// instance"). Setting the second from the first gave a `step-job` crossing
/// `USER_VISIBLE_LANE_WALL_US / 1_000` = **2** ms of wall time —
/// the host killed the guest at the exact instant the guest was supposed to yield cooperatively,
/// with zero margin, inside a JIT, on a machine running a whole fleet. A state action that
/// legitimately costs more than 2 ms (`💼️jobs`' `Execute`, priced `WORK_UNITS_EXECUTE` = 1 024 for
/// "one whole unchunked native dispatch", or one `Pump` of a real solve) could therefore never
/// complete: the cut lands on whatever instruction the guest reached, which is why the same
/// `s.wfc.bitmap.solve` call answered `alloc::Global::deallocate` under
/// `drop_glue::<wit_bindgen…FutureState>` on one run and `[async-lift]…#step-job` on the next
/// (`📓️wi1-…md` §7.2, `🗑️generated/gj1-trap-probe{,-2}.txt`). 30 s is the same "declared wedged"
/// ceiling the gateway's own cold-open budget uses, three orders of magnitude above the cooperative
/// slice it now stops impersonating.
const GUEST_JOB_WATCHDOG_MS: u64 = 30_000;



/// ⛽️ Grants `fuel` on a metered engine; an unmetered engine has no fuel to grant and bounds the
/// call by its epoch deadline alone — see [`SharedEngineConfig::fuel_metering`].
fn arm_store_fuel(store: &mut Store<ActorHostState>, fuel_metering: bool, fuel: u64) -> wasmtime::Result<()> {
    if fuel_metering {
        store.set_fuel(fuel)?;
    }
    Ok(())
}

fn classify_guest_trap(trap: &impl std::fmt::Debug) -> TurnFault {
    let message = format!("{trap:?}");
    let lowered = message.to_ascii_lowercase();
    if lowered.contains("fuel") {
        TurnFault::FuelExhausted
    } else if lowered.contains("epoch") || lowered.contains("interrupt") {
        TurnFault::DeadlineExceeded
    } else {
        TurnFault::Trapped(message)
    }
}

/// 🧬️ `pure` (`📜️wit/📜️pure.wit`) is `world actor`'s ONLY import — `log`/`now-ms`/`trace-span`,
/// none fallible, none async.
impl actor_bindings::semio::framework::pure::Host for ActorHostState {
    /// 🚫️async: E1 — `wasmtime::component::bindgen!` generates this `Host` trait from the WIT
    /// interface, which declares `log`/`now-ms`/`trace-span` sync (see doc comment above); the
    /// trait's signature is external and fixed, not chosen by this repo. See R9/R2 E1.
    fn log(&mut self, level: String, message: String) {
        eprintln!("[actor:{}:{level}] {message}", self.plugin_id);
    }

    fn operation_clock_us(&mut self,clock_binding:u64)->Option<u64>{self.original_job_clock.filter(|(binding,_)|*binding==clock_binding).and_then(|(_,clock)|clock())}

    fn now_ms(&mut self) -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as i64)
    }

    fn trace_span(&mut self, name: String) {
        eprintln!("[actor:{}:trace] {name}", self.plugin_id);
    }
}

impl operation::wasmtime::OperationHostStore for ActorHostState{
 fn operation_slot(&mut self)->&mut operation::slot::OperationSlot{&mut self.operation_slot}
 fn operation_log(&mut self,level:String,message:String){actor_bindings::semio::framework::pure::Host::log(self,level,message)}
 fn operation_clock_us(&mut self,clock_binding:u64)->Option<u64>{actor_bindings::semio::framework::pure::Host::operation_clock_us(self,clock_binding)}
 fn operation_now_ms(&mut self)->i64{actor_bindings::semio::framework::pure::Host::now_ms(self)}
 fn operation_trace_span(&mut self,name:String){actor_bindings::semio::framework::pure::Host::trace_span(self,name)}
}

/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): the seven type-only interfaces
/// (`byte-page`/`instance-lifetime` referenced by `reactor::turn-result`'s `command-ingress`/
/// `lifecycle-receipt`/`ui-patch-receipt` fields).
/// `Actor::add_to_linker` — the whole-world linker call the collapse requires (see
/// `WasmtimeRuntime::new`) — demands a `Host` impl for every interface `wit-parser` surfaces as an
/// import, including the ones present ONLY because an exported function's signature references
/// their types. Those traits declare no methods at all (the schema-parity test's
/// `functional_import_names`/`type_only_import_names` split asserts exactly this distinction), so
/// each impl is empty by construction, not by omission.
impl wit_types::Host for ActorHostState {}
impl wit_capabilities::Host for ActorHostState {}
impl wit_effects::Host for ActorHostState {}
impl wit_events::Host for ActorHostState {}
impl wit_ui::Host for ActorHostState {}
impl wit_lifetime::Host for ActorHostState {}

/// 🪧️ `ui.wit`'s `resource surface` is an empty MARKER resource — declared purely so the design
/// table's vocabulary has a nominal type, referenced by no function signature in the world (see
/// `interface ui`'s own doc comment). `bindgen!` still generates a `HostSurface` trait with the
/// mandatory `drop`, so this impl exists to satisfy the linker and can never actually be called:
/// no host function hands a `Surface` handle to the guest, so no handle exists to drop.
impl wit_ui::HostSurface for ActorHostState {
    /// 🚫️async: E1 — `bindgen!` fixes this signature (the resource-destructor hook wasmtime calls
    /// when a guest handle goes out of scope); it is not chosen by this repo. See R9/R2 E1.
    fn drop(&mut self, _rep: wasmtime::component::Resource<wit_ui::Surface>) -> wasmtime::Result<()> {
        Ok(())
    }
}

//#region 🚪️host-async on the poll-backed runtime
/// 🚪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `emit`/`emit-patch`, the two
/// deliberate one-way doors. On THIS runtime they are fully functional — the sinks are drained by
/// `execute_turn` at the end of the same turn and merged into the `TurnResult`, which is precisely
/// the poll world's own delivery mechanism, so an `emit`ed effect reaches the host through exactly
/// the path an effect returned from `poll` would.
impl wit_host_async::Host for ActorHostState {
    /// 🚫️async: E1 — `bindgen!` declares `emit`/`emit-patch` sync because the WIT does (see
    /// `host-async`'s own doc comment on why these two stay fire-and-forget). Both bodies are a
    /// single `Vec::push` with no suspension point, so nothing is lost: the ASYNC half of the work
    /// (`wit_effect_to_kernel`) is deferred to `execute_turn`, which can await it properly.
    fn emit(&mut self, value: wit_effects::Effect) {
        self.emit_sink.push(value);
    }

    fn emit_patch(&mut self, patch: wit_ui::UiPatch) {
        self.emit_patch_sink.push(patch);
    }
}

/// 🧯️ The fault every direct `host-async` await resolves to under [`WasmtimeRuntime`] — the
/// poll-backed runtime, whose contract is one call in, one `turn-result` out.
///
/// This is a real semantic boundary, not a stub: a `host-async` import must resolve DURING the
/// guest's own call, but this runtime answers host operations by returning the effect in the
/// `turn-result` and delivering an `event.completed` on a LATER `poll`. There is no point in the
/// turn at which such a future could complete, so failing loudly with a typed fault is the only
/// honest answer — silently parking would deadlock the turn, and trapping would kill the actor.
/// The runtime that DOES serve these awaits is the one built on `📥️imports/🦀️.rs`'s
/// `AsyncActorHostState` (24 real implementations, dispatching straight onto `AsyncServices`),
/// mounted by the `async-plugin-runtime` packet.
async fn poll_backed_direct_await_fault(name: &str) -> Vec<u8> {
    store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&semio_framework_diagnostic::Fault::new(
        semio_framework_diagnostic::FaultOrigin::Os,
        semio_framework_diagnostic::FaultCode::new("host-async.poll-backed"),
        format!("host-async {name} cannot be awaited directly on the poll-backed WasmtimeRuntime — emit the matching `effect` from `poll` and await its `event.completed` on a later turn"),
    )))
}

/// ⏳️ The 24 awaitable imports. Every one delegates to [`poll_backed_direct_await_fault`] — see its
/// doc for why that is the correct behaviour here rather than a gap.
impl wit_host_async::HostWithStore<ActorHostState> for wasmtime::component::HasSelf<ActorHostState> {
    async fn storage_read(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::StorageReadParams) -> Result<Option<Vec<u8>>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("storage-read").await)
    }

    async fn storage_write(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::StorageWriteParams) -> Result<(), Vec<u8>> {
        Err(poll_backed_direct_await_fault("storage-write").await)
    }

    async fn storage_delete(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::StorageDeleteParams) -> Result<(), Vec<u8>> {
        Err(poll_backed_direct_await_fault("storage-delete").await)
    }

    async fn blob_load(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::BlobLoadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("blob-load").await)
    }

    async fn blob_write(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::BlobWriteParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("blob-write").await)
    }

    async fn blob_read(_accessor: &Accessor<ActorHostState, Self>, _hash: String) -> Result<wasmtime::component::StreamReader<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("blob-read").await)
    }

    async fn http_fetch(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::HttpParams) -> Result<wit_host_async::HttpResponse, Vec<u8>> {
        Err(poll_backed_direct_await_fault("http-fetch").await)
    }

    async fn artifact_read(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::ArtifactReadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("artifact-read").await)
    }

    async fn artifact_write(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::ArtifactWriteParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("artifact-write").await)
    }

    async fn link_resolve(_accessor: &Accessor<ActorHostState, Self>, _link: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("link-resolve").await)
    }

    async fn registry_query(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::RegistryQueryParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("registry-query").await)
    }

    async fn io_compose(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::IoComposeParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("io-compose").await)
    }

    async fn io_run(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::IoRunParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("io-run").await)
    }

    async fn cache_derive(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::CacheDeriveParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("cache-derive").await)
    }

    async fn cache_read(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::CacheReadParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("cache-read").await)
    }

    async fn invoke_extension(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::InvokeExtensionParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("invoke-extension").await)
    }

    async fn open_window(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::OpenWindowParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("open-window").await)
    }

    async fn open_dialog(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::OpenDialogParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("open-dialog").await)
    }

    async fn dispatch_action(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::DispatchActionParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("dispatch-action").await)
    }

    async fn spawn_plugin_instance(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::SpawnPluginInstanceParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("spawn-plugin-instance").await)
    }

    async fn request_file_open(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::RequestFileOpenParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("request-file-open").await)
    }

    async fn request_media_frames(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::RequestMediaFramesParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("request-media-frames").await)
    }

    async fn request_capability(_accessor: &Accessor<ActorHostState, Self>, _params: wit_effects::RequestCapabilityParams) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("request-capability").await)
    }

    async fn spawn_job(_accessor: &Accessor<ActorHostState, Self>, _job: u64, _kind: String, _input: Vec<u8>, _placement: wit_effects::JobPlacement) -> Result<Vec<u8>, Vec<u8>> {
        Err(poll_backed_direct_await_fault("spawn-job").await)
    }
}
//#endregion 🚪️host-async on the poll-backed runtime

pub(super) struct WasmtimeInstanceState {
    pub(super) store: Store<ActorHostState>,
    pub(super) bindings: actor_bindings::Actor,
    /// ⏱️ This store's wall deadline, read by the callback [`EpochDeadlines::install`] put on it.
    pub(super) deadline: Arc<EpochDeadlineCell>,
    /// 🪪️ "One actor per app instance is the default" (design-abi.md §4) — minted once at
    /// `instantiate` and used to fill the `instance` field WIT's per-instance events carry but
    /// `semio_framework::kernel::Event`'s own lifecycle variants (`InstanceClose`/`Activate`/
    /// `SuspendRequest`/`CapabilityChanged`/`QuotaChanged`) dropped (kernel's `Event` is already
    /// actor-scoped by construction, so re-carrying the instance id per-event was redundant there).
    instance_id: u32,
}

/// 🐎️ ONE shared `Engine` per process (`build_shared_engine`), one `Store<ActorHostState>` per
/// actor (`instantiate`). Wraps the type-independent primitives in `//#region 🔧️SharedWasmtimeEngine`
/// above — this is where they get consumed.
pub struct WasmtimeRuntime {
    original_clock:fn()->Option<u64>,
    pub(super) engine: Engine,
    pub(super) epoch: EpochDeadlines,
    linker: Linker<ActorHostState>,
    cache_root: PathBuf,
    engine_config_hash: [u8; 32],
    fuel_metering: bool,
    isolated_engine: String,
    next_instance_id: std::sync::atomic::AtomicU32,
}

impl WasmtimeRuntime {
    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `Actor::add_to_linker`
    /// defines BOTH of the collapsed world's imports in one call (`pure` + `host-async`) —
    /// adding only `pure` now leaves 24 imports unresolved and every instantiation fails with
    /// "a matching implementation was not found in the linker".
    ///
    /// 🌐️ `add_to_linker_async`, not `_sync`: the Store is component-model-async now, and the
    /// sync WASI shim installs host functions that cannot be called from an async-lifted guest.
    pub async fn new(cfg: SharedEngineConfig,original_clock:fn()->Option<u64>) -> Result<Self, PluginHostError> {
        let (engine, pooling_active) = build_shared_engine(cfg).await?;
        let epoch = EpochDeadlines::new(&engine, &plugin_host_worker_pool());
        let mut linker = Linker::new(&engine);
        actor_bindings::Actor::add_to_linker::<ActorHostState, wasmtime::component::HasSelf<ActorHostState>>(&mut linker, |state: &mut ActorHostState| state).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        operation::wasmtime::register(&mut linker).map_err(|error|PluginHostError::Wasmtime(error.to_string()))?;
        wasmtime_wasi::p2::add_to_linker_async(&mut linker).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let engine_config_hash = shared_engine_config_hash(&cfg, pooling_active).await;
        let isolated_engine = SharedEngineConfig { force_on_demand: !pooling_active, ..cfg }.to_isolated_arg();
        Ok(Self { original_clock,engine, epoch, linker, cache_root: default_compiled_cache_root().await, engine_config_hash, fuel_metering: cfg.fuel_metering, isolated_engine, next_instance_id: std::sync::atomic::AtomicU32::new(1) })
    }

    /// 🕰️ Returns the clock supplied by the original runtime embedding.
    pub fn clock_source(&self)->fn()->Option<u64>{self.original_clock}
}

impl WasmtimeRuntime {
    /// ⏱️ Arms one guest crossing with the host's watchdog instead of the guest's own grant, for as
    /// long as the returned demand lives. Fuel is deliberately disarmed, for the reason the gateway's
    /// cold-open budget already states: a fuel yield is exactly as unresumable as an epoch cut under
    /// the JIT, so arming both only adds a second, harder-to-read way to say "wedged".
    fn arm_guest_watchdog(&self, store: &mut Store<ActorHostState>, deadline: &EpochDeadlineCell) -> Result<EpochDemand, PluginHostError> {
        arm_store_fuel(store, self.fuel_metering, u64::MAX).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        Ok(self.epoch.arm(store, deadline, GUEST_JOB_WATCHDOG_MS))
    }

    /// 🌱️ The JIT half of [`OwnedRuntime::codec_call`]: one of the component's four pure `codec`
    /// functions on a throwaway instance. The hub itself runs these under the owned interpreter, so
    /// this exists as the A/B oracle — the two runtimes share nothing but the component bytes, and a
    /// fault that reproduces in both is in the GUEST rather than in an interpreter. That is exactly
    /// the bisect ticket 26/09/18 slice A1 used to clear the owned interpreter of the shadow-stack
    /// trap, and what slice TC3d used to place the `codec.genesis` trap in the guest's own resolver.
    pub(super) async fn codec_instance(&self, compiled: &CompiledHandle, budget: &Budget) -> Result<GuestInstance, TurnFault> {
        self.instantiate(compiled, RuntimeActorId(0), &[], budget).await.map_err(TurnFault::Host)
    }

    /// 🏛️ Retrieves the component's handwritten SQL for an exact snapshot dialect.
    pub async fn codec_sqlite_schema(&self, compiled: &CompiledHandle, dialect: &str, budget: &Budget) -> Result<String, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else { return Err(TurnFault::Trapped("SQLite schema requires Wasmtime instance".into())); };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        store.run_concurrent(async |accessor| bindings.semio_framework_codec().call_sqlite_schema(accessor, dialect.to_string()).await).await.map_err(|error| TurnFault::Trapped(error.to_string()))?.map_err(|error| TurnFault::Trapped(error.to_string()))?.map_err(decode_guest_plugin_error)
    }

    /// 📤️ Executes a semantic snapshot projection within the guest's resource bounds.
    pub async fn codec_sqlite_export<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &CompiledHandle, dialect: &str, encoding: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotFileResult, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else { return Err(TurnFault::Trapped("SQLite export requires Wasmtime instance".into())); };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let encoding = match encoding { "binary" => actor_bindings::exports::semio::framework::codec::SnapshotEncoding::Binary, "text" => actor_bindings::exports::semio::framework::codec::SnapshotEncoding::Text, _ => return Err(TurnFault::Trapped("invalid snapshot encoding".into())) };
        let limits = wit_snapshot_limits(limits);
        let (coordinate,bytes)=snapshot_component_transport(dialect,payload,identity)?;
        let mut receiver=operation::OriginalOperationReceiver::new(identity);
        let output = observe_sqlite_guest(store.run_concurrent(async |accessor| operation::wasmtime::receive(accessor,&mut receiver,bindings.semio_framework_codec().call_sqlite_export(accessor, coordinate, encoding, bytes, limits, wit_snapshot_grant(native_grant))).await), &self.epoch, deadline, progress, cancellation).await?.map_err(|error| TurnFault::Trapped(error.to_string()))?.map_err(receiving_turn_fault)?.map_err(decode_guest_plugin_error)?;
        Ok(match output { actor_bindings::exports::semio::framework::codec::SnapshotFileResult::Done(output) => sqlite_wire::SnapshotFileResult::Done(sqlite_wire::SnapshotFile { bytes: output.bytes, diagnostics: output.diagnostics }), actor_bindings::exports::semio::framework::codec::SnapshotFileResult::Rejected(output) => sqlite_wire::SnapshotFileResult::Rejected(wit_snapshot_rejection(output)), actor_bindings::exports::semio::framework::codec::SnapshotFileResult::Pending(output)=>sqlite_wire::SnapshotFileResult::Pending(wit_snapshot_retirement(output)) })
    }

    /// 📥️ Reconstructs a native snapshot from the exact declared semantic SQLite schema.
    pub async fn codec_sqlite_import<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &CompiledHandle, dialect: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotPayloadResult, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else { return Err(TurnFault::Trapped("SQLite import requires Wasmtime instance".into())); };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let limits = wit_snapshot_limits(limits);
        let (coordinate,bytes)=snapshot_component_transport(dialect,payload,identity)?;
        let mut receiver=operation::OriginalOperationReceiver::new(identity);
        let output = observe_sqlite_guest(store.run_concurrent(async |accessor| operation::wasmtime::receive(accessor,&mut receiver,bindings.semio_framework_codec().call_sqlite_import(accessor, coordinate, bytes, limits, wit_snapshot_grant(native_grant))).await), &self.epoch, deadline, progress, cancellation).await?.map_err(|error| TurnFault::Trapped(error.to_string()))?.map_err(receiving_turn_fault)?.map_err(decode_guest_plugin_error)?;
        Ok(match output {
            actor_bindings::exports::semio::framework::codec::SnapshotPayloadResult::Done(output) => {
                let encoding = match output.encoding { actor_bindings::exports::semio::framework::codec::SnapshotEncoding::Binary => sqlite_wire::SnapshotEncoding::Binary, actor_bindings::exports::semio::framework::codec::SnapshotEncoding::Text => sqlite_wire::SnapshotEncoding::Text };
                sqlite_wire::SnapshotPayloadResult::Done(sqlite_wire::SnapshotPayload { encoding, bytes: output.bytes, diagnostics: output.diagnostics })
            }
            actor_bindings::exports::semio::framework::codec::SnapshotPayloadResult::Rejected(output) => sqlite_wire::SnapshotPayloadResult::Rejected(wit_snapshot_rejection(output)),
            actor_bindings::exports::semio::framework::codec::SnapshotPayloadResult::Pending(output)=>sqlite_wire::SnapshotPayloadResult::Pending(wit_snapshot_retirement(output)),
        })
    }

    /// 🧬️ `codec.pack-schema-hash` — the kind's 32-byte structural snapshot fingerprint.
    ///
    /// 🚫️ A guest's own `plugin-error` from any `codec` export is its typed refusal, decoded as the owned interpreter
    /// decodes it ([`decode_guest_plugin_error`]) — never a "trap" carrying the fault's raw bytes, which is what every
    /// agent opening a `stdio` document on hub 7800/p33 read instead of the guest's reason.
    pub async fn codec_pack_schema_hash(&self, compiled: &CompiledHandle, artifact_schema: &str, budget: &Budget) -> Result<[u8; 32], TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.pack-schema-hash called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let bytes = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_pack_schema_hash(accessor, artifact_schema).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        <[u8; 32]>::try_from(bytes.as_slice()).map_err(|_| TurnFault::Trapped("guest pack schema hash is not 32 bytes".to_string()))
    }

    /// 🌱️ `codec.genesis` — the canonical empty document of `artifact_schema` at `document_id`.
    pub async fn codec_genesis(&self, compiled: &CompiledHandle, artifact_schema: &str, document_id: &str, budget: &Budget) -> Result<GuestDocumentPair, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.genesis called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let document_id = document_id.to_string();
        let pair = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_genesis(accessor, artifact_schema, document_id).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        Ok(GuestDocumentPair { pack: pair.pack, spr: pair.spr })
    }

    /// 📥️ `codec.print-mirror` — the host's pair-validation fence for a package whose Rust codec the
    /// host does not link. A pair the component cannot print back is not a document of this kind,
    /// which is what makes this callable as a DISCRIMINATOR and not only as a printer.
    pub async fn codec_print_mirror(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], budget: &Budget) -> Result<GuestDocumentMirror, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.print-mirror called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let pair = actor_bindings::exports::semio::framework::codec::DocumentPair { pack: pack.to_vec(), spr: spr.to_vec() };
        let mirror = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_print_mirror(accessor, artifact_schema, pair).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        Ok(GuestDocumentMirror { dsl: mirror.0, ops: mirror.1 })
    }

    /// 🧩️ `codec.apply-ops` — the host-authoritative edit apply for a package whose Rust codec the
    /// host does not link.
    pub async fn codec_apply_ops(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], budget: &Budget, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<GuestDocumentPair, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.apply-ops called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let pair = actor_bindings::exports::semio::framework::codec::DocumentPair { pack: pack.to_vec(), spr: spr.to_vec() };
        let ops = ops.to_vec();
        let next = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_apply_ops(accessor, artifact_schema, pair, ops).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        Ok(GuestDocumentPair { pack: next.pack, spr: next.spr })
    }

    /// 📜️ `codec.replay-envelopes` — the hub's Check In fold for a package whose Rust codec the host
    /// does not link.
    pub async fn codec_replay_envelopes(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], envelopes: &[u8], budget: &Budget) -> Result<GuestDocumentPair, TurnFault> {
        let mut instance = self.codec_instance(compiled, budget).await?;
        let GuestInstanceState::Wasmtime(state) = &mut instance.state else {
            return Err(TurnFault::Trapped("codec.replay-envelopes called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let artifact_schema = artifact_schema.to_string();
        let pair = actor_bindings::exports::semio::framework::codec::DocumentPair { pack: pack.to_vec(), spr: spr.to_vec() };
        let envelopes = envelopes.to_vec();
        let next = store
            .run_concurrent(async |accessor| bindings.semio_framework_codec().call_replay_envelopes(accessor, artifact_schema, pair, envelopes).await)
            .await
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(|error| TurnFault::Trapped(error.to_string()))?
            .map_err(decode_guest_plugin_error)?;
        Ok(GuestDocumentPair { pack: next.pack, spr: next.spr })
    }
}

pub(super) async fn observe_sqlite_guest<F: std::future::Future>(future: F, epoch: &EpochDeadlines, deadline: &EpochDeadlineCell, mut progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<F::Output, TurnFault> {
    let started = std::time::Instant::now();
    let pool = &epoch.inner.pool;
    let mut next_tick = pool.now_ms();
    let mut future = std::pin::pin!(future);
    loop {
        match semio_framework_async::select2(future.as_mut(), pool.timer().sleep_until(next_tick)).await {
            semio_framework_async::Either::Left(result) => return if cancellation.is_cancelled() { Err(TurnFault::Cancelled) } else { Ok(result) },
            semio_framework_async::Either::Right(()) => {
                progress(0, started.elapsed());
                if cancellation.is_cancelled() {
                    deadline.0.store(0, std::sync::atomic::Ordering::Release);
                    epoch.inner.engine.increment_epoch();
                }
                next_tick = pool.now_ms().saturating_add(16);
            }
        }
    }
}

/// 🏷️ Restores the actual codec ABI's complete closed provider authority.
pub(super) fn wit_snapshot_retirement(value:actor_bindings::exports::semio::framework::codec::SnapshotRetirement)->sqlite_wire::SnapshotRetirement {
 sqlite_wire::SnapshotRetirement{ticket:value.ticket,owned_bytes:value.owned_bytes,refusal:value.refusal,native_pending:value.native_pending,output_pending:value.output_pending,terminal:value.terminal,demands:sqlite_wire::SnapshotDemands{items:value.demands.items,copy_bytes:value.demands.copy_bytes,capacity_bytes:value.demands.capacity_bytes,release_bytes:value.demands.release_bytes,depth:value.demands.depth}}
}

pub(super) fn wit_snapshot_rejection(value: actor_bindings::exports::semio::framework::codec::SnapshotRejection) -> sqlite_wire::SnapshotRejection {
    use actor_bindings::exports::semio::framework::codec::ValueRefusalKind as Wire;
    use semio_framework_value::ValueRefusalKind as Native;
    let kind = match value.kind { Wire::InvalidValue => Native::InvalidValue, Wire::Canceled => Native::Canceled, Wire::OwnershipLimit => Native::OwnershipLimit, Wire::AllocationFailed => Native::AllocationFailed, Wire::WorkLimit => Native::WorkLimit, Wire::DepthLimit => Native::DepthLimit, Wire::UnsupportedOwner => Native::UnsupportedOwner, Wire::InvariantViolated => Native::InvariantViolated };
    sqlite_wire::SnapshotRejection { kind, message: value.message, diagnostics: value.diagnostics }
}

pub(super) fn wit_snapshot_limits(value: semio_framework::sqlite_snapshot::SqliteDatabaseLimits) -> actor_bindings::exports::semio::framework::codec::SnapshotLimits {
    let value = sqlite_wire::SnapshotLimits::from(value);
    actor_bindings::exports::semio::framework::codec::SnapshotLimits { max_file_bytes: value.max_file_bytes, max_value_bytes: value.max_value_bytes, max_allocation_bytes: value.max_allocation_bytes, max_schema_bytes: value.max_schema_bytes, max_rows: value.max_rows, max_columns: value.max_columns, max_tables: value.max_tables, max_pages: value.max_pages }
}

pub(super) fn wit_snapshot_grant(value:semio_framework_value::RetainedCloneGrant)->actor_bindings::semio::framework::types::RetainedCloneGrant{
    let value=sqlite_wire::SnapshotGrant::from(value);
    actor_bindings::semio::framework::types::RetainedCloneGrant{maximum_items:value.maximum_items,maximum_copy_bytes:value.maximum_copy_bytes,maximum_capacity_bytes:value.maximum_capacity_bytes,maximum_release_bytes:value.maximum_release_bytes,maximum_depth:value.maximum_depth}
}

//#region 🗂️GuestCodecDispatch
/// 🗂️ The component's own `codec` interface, dispatched across whichever runtime a caller actually
/// holds. Both concrete runtimes implement all four functions; [`GuestRuntimes`] did not forward any
/// of them, so a caller holding the enum — which is what `🏃️run` and `🌉️mcp` both hold, because a
/// `wasmtime::Component` belongs to the `Engine` that compiled it and cannot be moved to a second
/// runtime — could not reach a package's codec at all. That is what made a hub-bound MCP gateway
/// unable to register a `store::os_store::serialization::ArtifactCodec` for a fourth package's document kind, and therefore
/// unable to open that document's socket (ticket 26/09/18 slices M9 §8.1 / M10 §3.3).
///
/// ⛽️ Every one of these is pure by the WIT's own contract: it reads no storage, opens no window,
/// emits no effect, and runs on a throwaway instance the runtime creates and drops per call.
impl GuestRuntimes {
    /// ♻️ `package`'s compiled code when this runtime already has it on disk, without compiling.
    pub async fn load_compiled(&self, package: &PackageRef) -> Option<CompiledHandle> {
        let Self::Wasmtime(runtime) = self else { return None };
        let cache_path = compiled_cache_path(&runtime.cache_root, &runtime.engine_config_hash, &package.hash.0).await;
        load_compiled_component(&runtime.engine, &cache_path).await.map(|component| CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)), owned: None })
    }

    /// 🧊️ Where an isolated compile of `package` must land for this runtime to load it — `None` for a
    /// runtime whose compile is not a JIT (the owned interpreter prepares in memory).
    pub async fn isolated_compile_target(&self, package: &PackageRef) -> Option<IsolatedCompileTarget> {
        let Self::Wasmtime(runtime) = self else { return None };
        Some(IsolatedCompileTarget { cache_path: compiled_cache_path(&runtime.cache_root, &runtime.engine_config_hash, &package.hash.0).await, engine: runtime.isolated_engine.clone() })
    }

    /// 🏛️ Queries the exact component's declared handwritten semantic snapshot schema.
    pub async fn codec_sqlite_schema(&self, compiled: &CompiledHandle, dialect: &str, budget: &Budget) -> Result<String, TurnFault> {
        match self { Self::Owned(runtime) => runtime.codec_sqlite_schema(owned_compiled(compiled)?, dialect, budget.clone()).await, Self::Wasmtime(runtime) => runtime.codec_sqlite_schema(compiled, dialect, budget).await, #[cfg(test)] Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("semantic SQLite schema requires a real component".into())) }
    }

    /// 📤️ Executes the component's typed projection with caller cancellation and resource bounds.
    pub async fn codec_sqlite_export<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &CompiledHandle, dialect: &str, encoding: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotFileResult, TurnFault> {
        if cancellation.is_cancelled() { return Err(TurnFault::Cancelled); }
        match self { Self::Owned(runtime) => runtime.codec_sqlite_export(owned_compiled(compiled)?, dialect, encoding, payload, limits, budget.clone(), progress, cancellation, native_grant, identity).await, Self::Wasmtime(runtime) => runtime.codec_sqlite_export(compiled, dialect, encoding, payload, limits, budget, progress, cancellation, native_grant, identity).await, #[cfg(test)] Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("semantic SQLite export requires a real component".into())) }
    }

    /// 📥️ Executes exact semantic reconstruction with caller cancellation and resource bounds.
    pub async fn codec_sqlite_import<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &CompiledHandle, dialect: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotPayloadResult, TurnFault> {
        if cancellation.is_cancelled() { return Err(TurnFault::Cancelled); }
        match self { Self::Owned(runtime) => runtime.codec_sqlite_import(owned_compiled(compiled)?, dialect, payload, limits, budget.clone(), progress, cancellation, native_grant, identity).await, Self::Wasmtime(runtime) => runtime.codec_sqlite_import(compiled, dialect, payload, limits, budget, progress, cancellation, native_grant, identity).await, #[cfg(test)] Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("semantic SQLite import requires a real component".into())) }
    }

    /// 🧬️ `codec.pack-schema-hash` — the kind's 32-byte structural snapshot fingerprint.
    pub async fn codec_pack_schema_hash(&self, compiled: &CompiledHandle, artifact_schema: &str, budget: &Budget) -> Result<[u8; 32], TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_pack_schema_hash(owned_compiled(compiled)?, artifact_schema, budget.clone()).await,
            Self::Wasmtime(runtime) => runtime.codec_pack_schema_hash(compiled, artifact_schema, budget).await,
            #[cfg(test)]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.pack-schema-hash has no scripted runtime — it needs a real component".to_string())),
        }
    }

    /// 🌱️ `codec.genesis` — the canonical empty document of `artifact_schema` at `document_id`.
    pub async fn codec_genesis(&self, compiled: &CompiledHandle, artifact_schema: &str, document_id: &str, budget: &Budget) -> Result<GuestDocumentPair, TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_genesis(owned_compiled(compiled)?, artifact_schema, document_id, budget.clone()).await,
            Self::Wasmtime(runtime) => runtime.codec_genesis(compiled, artifact_schema, document_id, budget).await,
            #[cfg(test)]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.genesis has no scripted runtime — it needs a real component".to_string())),
        }
    }

    /// 📥️ `codec.print-mirror` — the pair-validation fence.
    pub async fn codec_print_mirror(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], budget: &Budget) -> Result<GuestDocumentMirror, TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_print_mirror(owned_compiled(compiled)?, artifact_schema, pack, spr, budget.clone()).await,
            Self::Wasmtime(runtime) => runtime.codec_print_mirror(compiled, artifact_schema, pack, spr, budget).await,
            #[cfg(test)]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.print-mirror has no scripted runtime — it needs a real component".to_string())),
        }
    }

    /// 🧩️ `codec.apply-ops` — the host-authoritative edit apply.
    pub async fn codec_apply_ops(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], budget: &Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<GuestDocumentPair, TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_apply_ops(owned_compiled(compiled)?, artifact_schema, pack, spr, ops, budget.clone(), identity).await,
            Self::Wasmtime(runtime) => runtime.codec_apply_ops(compiled, artifact_schema, pack, spr, ops, budget, identity).await,
            #[cfg(test)]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.apply-ops has no scripted runtime — it needs a real component".to_string())),
        }
    }

    /// 📜️ `codec.replay-envelopes` — the hub's Check In fold; `progress` sees the owned interpreter's
    /// fuel observations and `cancellation` ends its interpretation between two steps.
    pub async fn codec_replay_envelopes(&self, compiled: &CompiledHandle, artifact_schema: &str, pack: &[u8], spr: &[u8], envelopes: &[u8], budget: &Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<GuestDocumentPair, TurnFault> {
        match self {
            Self::Owned(runtime) => runtime.codec_replay_envelopes_observed(owned_compiled(compiled)?, artifact_schema, pack, spr, envelopes, budget.clone(), progress, cancellation).await,
            Self::Wasmtime(runtime) => runtime.codec_replay_envelopes(compiled, artifact_schema, pack, spr, envelopes, budget).await,
            #[cfg(test)]
            Self::Mock(_) | Self::Recording(_) => Err(TurnFault::Trapped("codec.replay-envelopes has no scripted runtime — it needs a real component".to_string())),
        }
    }
}
//#endregion 🗂️GuestCodecDispatch

impl GuestRuntime for WasmtimeRuntime {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{let _=identity;let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return Err(identity_turn_fault(crate::operation_authority::refusal(4)))};let WasmtimeInstanceState{store,bindings,deadline,..}=state;arm_store_fuel(store,self.fuel_metering,budget.fuel).map_err(|error|TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;let _epoch=self.epoch.arm_original(store,deadline,budget.deadline_us)?;let input=retained_turn_wire::input_to_wit(input);let original=store.run_concurrent(async|accessor|bindings.semio_framework_reactor().call_admit_actor_context(accessor,input,retained_turn_wire::grant_to_wit(budget.retained_policy)).await).await.and_then(|result|result).map_err(|trap|classify_guest_trap(&trap))?;retained_turn_wire::admission_from_wit(original)}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{let _=identity;let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return Err(identity_turn_fault(crate::operation_authority::refusal(4)))};let WasmtimeInstanceState{store,bindings,deadline,..}=state;arm_store_fuel(store,self.fuel_metering,budget.fuel).map_err(|error|TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;let _epoch=self.epoch.arm_original(store,deadline,budget.deadline_us)?;let input=retained_turn_wire::input_to_wit(input);let original=store.run_concurrent(async|accessor|bindings.semio_framework_reactor().call_close_actor_context(accessor,input,retained_turn_wire::grant_to_wit(budget.retained_policy)).await).await.and_then(|result|result).map_err(|trap|classify_guest_trap(&trap))?;retained_turn_wire::close_from_wit(original)}

    /// 🚫️async: R13/R14 corollary — `let _ = <async call>;` used to suppress the lint while
    /// silently dropping this call's future: the on-disk compilation cache write never ran, so
    /// `compile` was recompiling from wasm bytes on every single call regardless of
    /// `load_compiled_component`'s cache-hit path above. Best-effort by design (a failed cache
    /// write must not fail `compile` itself), so the `Result` still discards, only the future
    /// is now actually driven.
    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        let cache_path = compiled_cache_path(&self.cache_root, &self.engine_config_hash, &package.hash.0).await;
        if let Some(component) = load_compiled_component(&self.engine, &cache_path).await {
            return Ok(CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)), owned: None });
        }
        let component = Component::from_binary(&self.engine, bytes).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let _ = store_compiled_component(&component, &cache_path).await;
        Ok(CompiledHandle { package_hash: package.hash.0, component: Some(Arc::new(component)), owned: None })
    }

    /// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-wasmtime-upgrade): wasmtime 22.0.1's
    /// `bindgen!`-generated `Actor::instantiate` returned `(Actor, wasmtime::component::Instance)`;
    /// wasmtime 47.0.3 dropped the raw `Instance` from the convenience wrapper (it was never used
    /// here anyway — every subsequent call goes through `bindings`' own typed accessors) and
    /// returns bare `Actor` instead. See `path2.rs`'s expanded bindgen output in
    /// `wasmtime-internal-component-macro-47.0.3/tests/expanded/` for the confirmed new shape.
    ///
    /// 🧬️ B1 world-collapse: `instantiate_async`, not `instantiate` — the world's `host-async`
    /// imports are async host functions, and the sync entry point refuses a Store whose linker
    /// carries any.
    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        let component = compiled.component.as_ref().ok_or_else(|| PluginHostError::Plugin("CompiledHandle has no wasmtime Component — built by MockGuestRuntime::compile, not WasmtimeRuntime::compile".to_string()))?;
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let (wasi, diagnostics) = guest_wasi_ctx();
        let host_state = ActorHostState {
            original_job_clock:None,
            operation_slot:operation::slot::OperationSlot::new(),
            plugin_id: format!("actor-{}", actor.0),
            actor,
            caps: caps.to_vec(),
            emit_sink: Vec::new(),
            emit_patch_sink: Vec::new(),
            asset_map: HashMap::new(),
            limiter: BudgetLimiter::default(),
            wasi_ctx: wasi,
            diagnostics,
            resource_table: ResourceTable::new(),
        };
        let mut store = Store::new(&self.engine, host_state);
        store.limiter(|state| &mut state.limiter as &mut dyn ResourceLimiter);
        arm_store_fuel(&mut store, self.fuel_metering, budget.fuel).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let deadline = Arc::new(EpochDeadlineCell(std::sync::atomic::AtomicU64::new(u64::MAX),std::sync::atomic::AtomicU64::new(u64::MAX),Some(self.original_clock)));
        self.epoch.install(&mut store, Arc::clone(&deadline));
        let remaining=remaining_original_deadline_us((self.original_clock)(),budget.deadline_us).map_err(|error|PluginHostError::NativeIo(semio_framework_value::ValueError::literal(match error{OriginalDeadlineError::MissingClock=>semio_framework_value::ValueRefusalKind::UnsupportedOwner,OriginalDeadlineError::Expired=>semio_framework_value::ValueRefusalKind::WorkLimit},"original guest admission requires its live caller deadline")))?;
        let _epoch = self.epoch.arm(&mut store, &deadline, remaining.div_ceil(1_000));
        deadline.1.store(budget.deadline_us,std::sync::atomic::Ordering::Release);
        let bindings = actor_bindings::Actor::instantiate_async(&mut store, component, &self.linker).await.map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        let guest = store.run_concurrent(async |accessor| bindings.semio_framework_reactor().call_channel_version(accessor).await).await.and_then(|inner| inner).map_err(|error| PluginHostError::Wasmtime(error.to_string()))?;
        protocol::admit_guest_channel_version(guest, protocol::CHANNEL_VERSION).map_err(|fault| PluginHostError::Refused(Box::new(fault)))?;
        Ok(GuestInstance { actor, state: GuestInstanceState::Wasmtime(WasmtimeInstanceState { store, bindings, instance_id, deadline }) })
    }

    /// 🚫️async: R10 residue shape 1 — `kernel_event_to_wit` is async, hoisted out of the sync
    /// `Iterator::map` closure via a plain loop.
    ///
    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): `poll` is `async func`
    /// now, so it is driven through `Store::run_concurrent`'s `Accessor` rather than called
    /// directly against `&mut Store` — that is the ONLY shape wasmtime offers for an async-lifted
    /// export, and it is what lets the guest suspend on a `host-async` import mid-turn without
    /// unwinding the call. Args are owned (moved into the concurrent task), not borrowed.
    /// A turn's pages are staged FIRST, in the same concurrent task, so the guest reads them off
    /// its staging slot instead of carrying them through `poll`'s leaked parameter area.
    ///
    /// 🚪️ B1 world-collapse: everything the guest pushed through `host-async.emit` during THIS
    /// turn is delivered on the same `turn-result` as the effects it returned — one merged list,
    /// emitted-first (they happened earlier in the turn, by construction).
    ///
    /// 👥️ M2 render-plane presence (sol's ruling, 26/08/20): `presence-update.update` is a
    /// pack-encoded `ui_contract::PresenceUpdate`, NOT the replication `PresencePeer` the
    /// record originally declared — the consumer of a turn result is the renderer, which
    /// needs `(surface, node_key)` addressing and a TTL, while the collaboration roster keeps
    /// its own channel (`ephemeral_snapshot` out, `AppCommand::Presence`/`adopt_presence`
    /// in). Symmetric with the guest's `kernel_presence_update_to_wit`.
    ///
    /// A malformed entry is SKIPPED rather than failing the turn, matching `AppCommand::
    /// Presence`'s own roster-decode convention: presence is best-effort and TTL-scoped, so
    /// one bad update must not cost the actor an otherwise valid turn's patches and effects.
    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(TurnFault::Trapped("execute_turn called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, instance_id, deadline } = state;
        arm_store_fuel(store, self.fuel_metering, budget.fuel).map_err(|error| TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))?;
        let _epoch = self.epoch.arm_original(store, deadline, budget.deadline_us)?;
        let wit_budget = wit_reactor::Budget { retained:retained_turn_wire::input_to_wit(budget.retained), retained_policy:retained_turn_wire::grant_to_wit(budget.retained_policy),work_fuel:budget.work_fuel, fuel: budget.fuel, deadline_us: budget.deadline_us, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames };
        let (wit_events, wit_command_page, wit_cold_pair_page) = kernel_turn_inputs_to_wit(events, *instance_id).await?;
        let call_result = store
            .run_concurrent(async |accessor| {
                if let Some((cursor, bytes)) = wit_command_page {
                    if let Err(error) = bindings.semio_framework_reactor().call_stage_command_page(accessor, cursor, bytes).await? {
                        return Ok(Err(decode_guest_plugin_error(error)));
                    }
                }
                if let Some(page) = wit_cold_pair_page {
                    if let Err(error) = bindings.semio_framework_reactor().call_stage_cold_pair_page(accessor, page).await? {
                        return Ok(Err(decode_guest_plugin_error(error)));
                    }
                }
                bindings.semio_framework_reactor().call_poll(accessor, wit_events, wit_budget).await.map(|original|original.map_err(retained_turn_wire::failure_from_wit))
            })
            .await
            .and_then(|inner| inner);
        let poll_result = match call_result {
            Ok(inner) => inner,
            Err(trap) => return Err(classify_guest_trap(&trap)),
        };
        let wit_turn_result = poll_result?;
        let retained_receipt = retained_turn_wire::receipt_from_wit(wit_turn_result.retained_receipt,budget.retained)?;
        let emitted: Vec<wit_effects::Effect> = std::mem::take(&mut store.data_mut().emit_sink);
        let emitted_patches: Vec<wit_ui::UiPatch> = std::mem::take(&mut store.data_mut().emit_patch_sink);
        let mut effects = Vec::with_capacity(emitted.len() + wit_turn_result.effects.len());
        for effect in emitted.into_iter().chain(wit_turn_result.effects) {
            effects.push(wit_effect_to_kernel(effect).await.map_err(TurnFault::Host)?);
        }
        let ui_patch_receipt = wit_turn_result.ui_patch_receipt.map(wit_patch_receipt_to_kernel);
        let ui_patches = ui_patch::wit_ui_patches_to_kernel(*instance_id, budget.max_patch_bytes, emitted_patches, wit_turn_result.ui_patches, ui_patch_receipt).map_err(|error| TurnFault::Host(PluginHostError::Plugin(error)))?;
        Ok(TurnResult {
            retained_receipt,
            ui_patches,
            effects,
            presence: {
                let mut updates = Vec::with_capacity(wit_turn_result.presence.len());
                for entry in wit_turn_result.presence {
                    let Ok(value) = store::pack_rt::decode_wire_value(&entry.update) else { continue };
                    if let Ok(update) = serde_json::from_value::<semio_framework::kernel::PresenceUpdate>(serde_json::Value::from(&value)) {
                        updates.push(update);
                    }
                }
                updates
            },
            next_wake: wit_turn_result.next_wake,
            status: wit_turn_status_to_kernel(wit_turn_result.status).await,
            fuel_used: wit_turn_result.fuel_used,
            command_ingress: wit_command_ingress_to_kernel(wit_turn_result.command_ingress),
            cold_pair_ingress: wit_cold_ingress_to_kernel(wit_turn_result.cold_pair_ingress)?,
            lifecycle_receipt: wit_turn_result.lifecycle_receipt.map(wit_lifecycle_receipt_to_kernel),
            ui_patch_receipt,
        })
    }

    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): every export below is
    /// `async func` in the WIT now, so each goes through `Store::run_concurrent` + `Accessor` — the
    /// same shape `execute_turn` uses, and the only one wasmtime offers for an async-lifted export.
    /// The doubled `Result` is unchanged in meaning: outer = trap, inner = `plugin-error`.
    ///
    /// ⏱️ The host's own ceilings, never the guest's grant — see [`GUEST_JOB_WATCHDOG_MS`]. Armed
    /// HERE because `start-job` armed nothing at all and inherited whatever the preceding
    /// `execute_turn`/`step-job` left in the store: a start after a fully spent turn began with
    /// an already-passed epoch and no fuel.
    async fn start_job(&self,inst:&mut GuestInstance,source:Option<OriginalJobSource>,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStartReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,source,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,source,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,source,identity_turn_fault(crate::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,source,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,source,error.into())};
        let wit_budget=retained_turn_wire::jobs::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_start_job(accessor,source.map(retained_turn_wire::jobs::source_to_wit),wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(retained_turn_wire::jobs::start_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn step_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStepReply<OriginalJobOutcome>>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(crate::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=retained_turn_wire::jobs::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_step_job(accessor,job,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(retained_turn_wire::jobs::step_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn read_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(crate::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=retained_turn_wire::jobs::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_read_job_output(accessor,job,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(retained_turn_wire::jobs::output_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn acknowledge_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputAckReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(crate::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=retained_turn_wire::jobs::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_acknowledge_job_output(accessor,job,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(retained_turn_wire::jobs::ack_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn cancel_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(crate::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=retained_turn_wire::jobs::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_cancel_job(accessor,job,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(retained_turn_wire::jobs::cancel_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn close_jobs(&self,inst:&mut GuestInstance,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{
        if let Err(error)=execution.validate_for(budget){return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(error))}
        if !self.fuel_metering{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original job execution requires actual instruction metering")))}
        let GuestInstanceState::Wasmtime(state)=&mut inst.state else{return OriginalGuestJobCall::unsubmitted(execution,None,identity_turn_fault(crate::operation_authority::refusal(4)))};
        let WasmtimeInstanceState{store,bindings,deadline,..}=state;
        if let Err(error)=arm_store_fuel(store,true,execution.fuel){return OriginalGuestJobCall::unsubmitted(execution,None,TurnFault::Host(PluginHostError::Wasmtime(error.to_string())))}
        let _epoch=match self.epoch.arm_original(store,deadline,execution.deadline_us){Ok(epoch)=>epoch,Err(error)=>return OriginalGuestJobCall::unsubmitted(execution,None,error.into())};
        let wit_budget=retained_turn_wire::jobs::budget_to_wit(budget);
        let returned=store.run_concurrent(async|accessor|bindings.semio_framework_jobs().call_close_jobs(accessor,wit_budget).await).await.and_then(|result|result).map_err(|error|classify_guest_trap(&error));
        let spent_fuel=store.get_fuel().ok().and_then(|remaining|execution.fuel.checked_sub(remaining));
        OriginalGuestJobCall{original:returned.and_then(retained_turn_wire::jobs::cancel_from_wit),execution:OriginalGuestExecutionReceipt{budget:execution,spent_fuel,submitted:true},returned_unsubmitted_source:None}
    }

    async fn checkpoint(&self, inst: &mut GuestInstance, _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("checkpoint called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.arm_guest_watchdog(store, deadline)?;
        store
            .run_concurrent(async |accessor| bindings.semio_framework_checkpoint().call_checkpoint(accessor).await)
            .await
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Plugin(format!("{error:?}")))
    }

    async fn restore(&self, inst: &mut GuestInstance, state_bytes: &[u8], _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
        let GuestInstanceState::Wasmtime(state) = &mut inst.state else {
            return Err(PluginHostError::Plugin("restore called on a non-wasmtime GuestInstance".to_string()));
        };
        let WasmtimeInstanceState { store, bindings, deadline, .. } = state;
        let _epoch = self.arm_guest_watchdog(store, deadline)?;
        let state_bytes = state_bytes.to_vec();
        store
            .run_concurrent(async |accessor| bindings.semio_framework_checkpoint().call_restore(accessor, state_bytes).await)
            .await
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Wasmtime(error.to_string()))?
            .map_err(|error| PluginHostError::Plugin(format!("{error:?}")))
    }

    /// 🗑️ `Store<ActorHostState>` and its `Component` `Arc` drop with `_inst` — nothing else to
    /// release; the pooling allocator reclaims the instance's slab on `Store` drop.
    async fn drop_instance(&self, _inst: GuestInstance) {
    }
}

//#region 🎛️GuestRuntimes
/// 🎛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (host-dedyn): the closed-set enum replacing every
/// `Arc<dyn GuestRuntime>` in this crate (O1/R1 — `async fn` in a trait cannot be `dyn`-dispatched).
/// Hand-written, not `#[dyn_enum]`/`dyn_enum_close!` (`semio-framework-dispatch-macros`): TWO of the
/// three variants are `#[cfg(test)]`-gated, and the macro's own acceptance suite
/// (`📓️terra-dyn-enum-macro-report.md`) never exercised cfg-gated variants at all — a hand-written
/// match-delegation is the lower-risk choice here, exactly the fallback the packet brief
/// anticipates ("if cfg-gated variants defeat the macro, hand-write the enum ... say so").
///
/// `Mock`/`Recording` wrap an `Arc<..>` of their concrete type, not the bare value — many existing
/// tests hold their OWN `Arc<MockGuestRuntime>`/`Arc<RecordingRuntime>` to call inherent,
/// non-`GuestRuntime` methods (`script_turn`, `observed_events`, `last_turn_budget`, ...) on the
/// SAME instance a `PluginInstanceHandle`/`ShardLoop` is driving; a bare-value variant would force
/// every one of those call sites to construct a SECOND, disconnected instance instead of sharing
/// state through the clone `Arc` already gives for free. `Wasmtime` needs no such indirection — no
/// test in this crate keeps a second handle to the SAME `WasmtimeRuntime` for out-of-band inherent
/// calls — so it stays a bare value, one indirection (the enum's own `Arc<GuestRuntimes>` wrapper at
/// every call site) instead of two.
#[cfg_attr(target_pointer_width = "64", expect(clippy::large_enum_variant, reason = "The runtime is retained behind shared Arc owners; another box would add indirection and allocation to the dominant native runtime."))]
pub enum GuestRuntimes {
    Owned(OwnedRuntime),
    Wasmtime(WasmtimeRuntime),
    /// 🔮️ a later packet adds `AsyncActor(AsyncPluginRuntime)` here, backed by wasmtime's
    /// `component-model-async` — do not mount `⏳️runtime/🦀️.rs` from this packet (out of scope, needs a
    /// rewritten schema; see this ticket's brief).
    #[cfg(test)]
    Mock(Arc<MockGuestRuntime>),
    #[cfg(test)]
    Recording(Arc<shard::RecordingRuntime>),
}

impl GuestRuntime for GuestRuntimes {
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>{match self{Self::Owned(runtime)=>runtime.admit_actor_context(inst,input,budget,identity).await,Self::Wasmtime(runtime)=>runtime.admit_actor_context(inst,input,budget,identity).await,#[cfg(test)]Self::Mock(runtime)=>runtime.admit_actor_context(inst,input,budget,identity).await,#[cfg(test)]Self::Recording(runtime)=>runtime.admit_actor_context(inst,input,budget,identity).await}}
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>{match self{Self::Owned(runtime)=>runtime.close_actor_context(inst,input,budget,identity).await,Self::Wasmtime(runtime)=>runtime.close_actor_context(inst,input,budget,identity).await,#[cfg(test)]Self::Mock(runtime)=>runtime.close_actor_context(inst,input,budget,identity).await,#[cfg(test)]Self::Recording(runtime)=>runtime.close_actor_context(inst,input,budget,identity).await}}

    async fn compile(&self, package: &PackageRef, bytes: &[u8]) -> Result<CompiledHandle, PluginHostError> {
        match self {
            Self::Owned(r) => r.compile(package, bytes).await,
            Self::Wasmtime(r) => r.compile(package, bytes).await,
            #[cfg(test)]
            Self::Mock(r) => r.compile(package, bytes).await,
            #[cfg(test)]
            Self::Recording(r) => r.compile(package, bytes).await,
        }
    }

    async fn instantiate(&self, compiled: &CompiledHandle, actor: RuntimeActorId, caps: &[BrokerCapabilityGrant], budget: &Budget) -> Result<GuestInstance, PluginHostError> {
        match self {
            Self::Owned(r) => r.instantiate(compiled, actor, caps, budget).await,
            Self::Wasmtime(r) => r.instantiate(compiled, actor, caps, budget).await,
            #[cfg(test)]
            Self::Mock(r) => r.instantiate(compiled, actor, caps, budget).await,
            #[cfg(test)]
            Self::Recording(r) => r.instantiate(compiled, actor, caps, budget).await,
        }
    }

    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        match self {
            Self::Owned(r) => r.execute_turn(inst, events, budget, identity).await,
            Self::Wasmtime(r) => r.execute_turn(inst, events, budget, identity).await,
            #[cfg(test)]
            Self::Mock(r) => r.execute_turn(inst, events, budget, identity).await,
            #[cfg(test)]
            Self::Recording(r) => r.execute_turn(inst, events, budget, identity).await,
        }
    }

    async fn start_job(&self,inst:&mut GuestInstance,source:Option<OriginalJobSource>,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStartReply>{match self{Self::Owned(runtime)=>runtime.start_job(inst,source,budget,execution).await,Self::Wasmtime(runtime)=>runtime.start_job(inst,source,budget,execution).await,#[cfg(test)]Self::Mock(runtime)=>runtime.start_job(inst,source,budget,execution).await,#[cfg(test)]Self::Recording(runtime)=>runtime.start_job(inst,source,budget,execution).await}}
    async fn step_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobStepReply<OriginalJobOutcome>>{match self{Self::Owned(runtime)=>runtime.step_job(inst,job,budget,execution).await,Self::Wasmtime(runtime)=>runtime.step_job(inst,job,budget,execution).await,#[cfg(test)]Self::Mock(runtime)=>runtime.step_job(inst,job,budget,execution).await,#[cfg(test)]Self::Recording(runtime)=>runtime.step_job(inst,job,budget,execution).await}}
    async fn read_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputReply>{match self{Self::Owned(runtime)=>runtime.read_job_output(inst,job,budget,execution).await,Self::Wasmtime(runtime)=>runtime.read_job_output(inst,job,budget,execution).await,#[cfg(test)]Self::Mock(runtime)=>runtime.read_job_output(inst,job,budget,execution).await,#[cfg(test)]Self::Recording(runtime)=>runtime.read_job_output(inst,job,budget,execution).await}}
    async fn acknowledge_job_output(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobOutputAckReply>{match self{Self::Owned(runtime)=>runtime.acknowledge_job_output(inst,job,budget,execution).await,Self::Wasmtime(runtime)=>runtime.acknowledge_job_output(inst,job,budget,execution).await,#[cfg(test)]Self::Mock(runtime)=>runtime.acknowledge_job_output(inst,job,budget,execution).await,#[cfg(test)]Self::Recording(runtime)=>runtime.acknowledge_job_output(inst,job,budget,execution).await}}
    async fn cancel_job(&self,inst:&mut GuestInstance,job:u64,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{match self{Self::Owned(runtime)=>runtime.cancel_job(inst,job,budget,execution).await,Self::Wasmtime(runtime)=>runtime.cancel_job(inst,job,budget,execution).await,#[cfg(test)]Self::Mock(runtime)=>runtime.cancel_job(inst,job,budget,execution).await,#[cfg(test)]Self::Recording(runtime)=>runtime.cancel_job(inst,job,budget,execution).await}}
    async fn close_jobs(&self,inst:&mut GuestInstance,budget:OriginalJobBudget,execution:OriginalGuestExecutionBudget)->OriginalGuestJobCall<OriginalJobCancelReply>{match self{Self::Owned(runtime)=>runtime.close_jobs(inst,budget,execution).await,Self::Wasmtime(runtime)=>runtime.close_jobs(inst,budget,execution).await,#[cfg(test)]Self::Mock(runtime)=>runtime.close_jobs(inst,budget,execution).await,#[cfg(test)]Self::Recording(runtime)=>runtime.close_jobs(inst,budget,execution).await}}

    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        match self {
            Self::Owned(r) => r.checkpoint(inst, identity).await,
            Self::Wasmtime(r) => r.checkpoint(inst, identity).await,
            #[cfg(test)]
            Self::Mock(r) => r.checkpoint(inst, identity).await,
            #[cfg(test)]
            Self::Recording(r) => r.checkpoint(inst, identity).await,
        }
    }

    async fn restore(&self, inst: &mut GuestInstance, state: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError> {
        match self {
            Self::Owned(r) => r.restore(inst, state, identity).await,
            Self::Wasmtime(r) => r.restore(inst, state, identity).await,
            #[cfg(test)]
            Self::Mock(r) => r.restore(inst, state, identity).await,
            #[cfg(test)]
            Self::Recording(r) => r.restore(inst, state, identity).await,
        }
    }

    async fn drop_instance(&self, inst: GuestInstance) {
        match self {
            Self::Owned(r) => r.drop_instance(inst).await,
            Self::Wasmtime(r) => r.drop_instance(inst).await,
            #[cfg(test)]
            Self::Mock(r) => r.drop_instance(inst).await,
            #[cfg(test)]
            Self::Recording(r) => r.drop_instance(inst).await,
        }
    }
}

impl From<WasmtimeRuntime> for GuestRuntimes {
    fn from(r: WasmtimeRuntime) -> Self {
        Self::Wasmtime(r)
    }
}
impl From<OwnedRuntime> for GuestRuntimes {
    fn from(r: OwnedRuntime) -> Self {
        Self::Owned(r)
    }
}
#[cfg(test)]
impl From<Arc<MockGuestRuntime>> for GuestRuntimes {
    fn from(r: Arc<MockGuestRuntime>) -> Self {
        Self::Mock(r)
    }
}
#[cfg(test)]
impl From<Arc<shard::RecordingRuntime>> for GuestRuntimes {
    fn from(r: Arc<shard::RecordingRuntime>) -> Self {
        Self::Recording(r)
    }
}
//#endregion 🎛️GuestRuntimes

//#region 🔀️EffectEventMarshal
/// 🌉️ `wit_effect_to_kernel`/`kernel_event_to_wit` are the host-side half of `design-abi.md` §2's
/// "WIT variants mirror [`Effect`/`Event`] field-for-field; the guest SDK glue converts between
/// them" — this is that conversion, mirrored for the host. It is NOT fully field-for-field in
/// practice: several real shape gaps between `📜️wit/*.wit` (packet A2) and `🎠️kernel/🦀️.rs`
/// (packet A3) surfaced while writing this and are called out inline + in
/// `📓️terra-B1-host-native-report.md`'s `## blocked-on` (most notably: `📜️wit/📜️effects.wit`'s
/// `io-run` effect has no `Effect::IoRun` counterpart yet).
pub(super) async fn decode_dsl(bytes: &[u8]) -> Option<DslValue> {
    if bytes.is_empty() {
        return None;
    }
    store::pack_rt::decode_wire_value(bytes).ok()
}

pub(super) async fn decode_json<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Option<T> {
    if bytes.is_empty() {
        return None;
    }
    serde_json::from_slice(bytes).ok()
}

async fn encode_json<T: serde::Serialize>(value: &T) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}

pub(super) async fn wit_message_endpoint_to_kernel(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    match endpoint {
        wit_types::MessageEndpoint::Shell(instance) => MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        wit_types::MessageEndpoint::Backbone(uri) => MessageEndpoint::Backbone { uri },
        wit_types::MessageEndpoint::PluginInstance(instance) => MessageEndpoint::PluginInstance { id: semio_framework::kernel::PluginInstanceId(instance.to_string()) },
        wit_types::MessageEndpoint::Extension(id) => MessageEndpoint::Extension { id },
        wit_types::MessageEndpoint::Topic(name) => MessageEndpoint::Topic { name },
    }
}

async fn kernel_message_endpoint_to_wit(endpoint: &MessageEndpoint) -> wit_types::MessageEndpoint {
    match endpoint {
        MessageEndpoint::Shell { instance } => wit_types::MessageEndpoint::Shell(instance.0.parse().unwrap_or(0)),
        MessageEndpoint::Backbone { uri } => wit_types::MessageEndpoint::Backbone(uri.clone()),
        MessageEndpoint::PluginInstance { id } => wit_types::MessageEndpoint::PluginInstance(id.0.parse().unwrap_or(0)),
        MessageEndpoint::Extension { id } => wit_types::MessageEndpoint::Extension(id.clone()),
        MessageEndpoint::Topic { name } => wit_types::MessageEndpoint::Topic(name.clone()),
    }
}

async fn kernel_request_outcome_to_wit(result: &RequestOutcome) -> wit_events::CompletionResult {
    match result {
        RequestOutcome::Ok(bytes) => wit_events::CompletionResult::Ok(bytes.clone()),
        RequestOutcome::Err(bytes) => wit_events::CompletionResult::Fault(bytes.clone()),
    }
}

async fn wit_turn_status_to_kernel(status: wit_reactor::TurnStatus) -> TurnStatus {
    match status {
        wit_reactor::TurnStatus::Idle => TurnStatus::Idle,
        wit_reactor::TurnStatus::MoreWork => TurnStatus::MoreWork,
        wit_reactor::TurnStatus::CheckpointReady(checkpoint) => TurnStatus::CheckpointReady { checkpoint: semio_framework_actor::JobCheckpoint { state: checkpoint.state, applied_progress: checkpoint.applied_progress } },
        wit_reactor::TurnStatus::Faulted(bytes) => TurnStatus::Faulted(bytes),
    }
}

/// 📄️ One staged command page as `reactor.stage-command-page` takes it — the cursor and the page's
/// live bytes, never the fixed 4 KiB tail. See that function's WIT doc for why the page left
/// `poll`'s parameter list.
fn kernel_command_page_to_wit(cursor: &semio_framework::kernel::CommandPageCursor, page: &semio_framework::kernel::FixedCommandPage) -> (wit_reactor::CommandPageCursor, Vec<u8>) {
    (kernel_command_cursor_to_wit(cursor), page.as_slice().to_vec())
}

fn wit_command_cursor_to_kernel(cursor: wit_reactor::CommandPageCursor) -> semio_framework::kernel::CommandPageCursor {
    semio_framework::kernel::CommandPageCursor {
        owner: cursor.owner,
        generation: cursor.generation,
        command_index: cursor.command_index,
        command_count: cursor.command_count,
        instance: cursor.instance,
        seq: cursor.seq,
        kind: cursor.kind,
        page_index: cursor.page_index,
        page_count: cursor.page_count,
        item_count: cursor.item_count,
        metadata: cursor.metadata,
    }
}

fn kernel_command_cursor_to_wit(cursor: &semio_framework::kernel::CommandPageCursor) -> wit_reactor::CommandPageCursor {
    wit_reactor::CommandPageCursor {
        owner: cursor.owner,
        generation: cursor.generation,
        command_index: cursor.command_index,
        command_count: cursor.command_count,
        instance: cursor.instance,
        seq: cursor.seq,
        kind: cursor.kind,
        page_index: cursor.page_index,
        page_count: cursor.page_count,
        item_count: cursor.item_count,
        metadata: cursor.metadata,
    }
}

fn wit_command_ingress_to_kernel(status: wit_reactor::CommandIngressStatus) -> semio_framework::kernel::CommandIngressStatus {
    let cursor = wit_command_cursor_to_kernel(status.cursor);
    match status.kind {
        0 => semio_framework::kernel::CommandIngressStatus::Idle,
        1 => semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor),
        2 => semio_framework::kernel::CommandIngressStatus::Backpressure(cursor),
        3 => semio_framework::kernel::CommandIngressStatus::CommandPending(cursor),
        4 => semio_framework::kernel::CommandIngressStatus::CommandComplete(cursor),
        5 => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: status.fault },
        _ => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-ingress-kind".to_vec() },
    }
}

/// 🎁️ Every effect that carries a `req: request-id` becomes `RequestId(req)` — one line, so it's
/// inlined at each call site below rather than its own helper.
/// 🐛️ Guest → host: WIT `effect` (`📜️wit/📜️effects.wit`) to `semio_framework::kernel::Effect`.
/// `Err` is returned (never a silently-wrong `Effect`) for `io-run` — the one variant with no
/// kernel counterpart yet (`## blocked-on` in the report).
///
/// 🚧️ blocked-on-A3: no `Effect::IoRun` variant exists yet (`## blocked-on` in the report).
///
/// 🚫️async: R10 residue shape 1 — `decode_dsl` is async, hoisted out of `Option::and_then`'s
/// sync closure below (and at every other `.args.and_then(|bytes| decode_dsl(&bytes))` site
/// in this match).
///
/// 🧬️ A2b narrowed `request-media-frames-effect.payload` from `option<pack>` to
/// `option<string>` (correctly honoring the kernel as SSOT) — already a `String`, no
/// decode needed.
#[cfg(test)]
#[path = "../🧪️tests/💡️service-operation/🦀️.rs"]
mod service_operation_tests;

pub(super) async fn wit_effect_to_kernel(effect: wit_effects::Effect) -> Result<Effect, PluginHostError> {
    use wit_effects::Effect as E;
    Ok(match effect {
        E::SendMessage(inner) => Effect::SendMessage { target: kernel_message_endpoint_to_wit_reverse(inner.target).await, payload: inner.payload },
        E::PublishEvent(inner) => Effect::PublishEvent { topic: inner.topic, payload: inner.payload },
        E::BlobLoad(inner) => Effect::BlobLoad { req: RequestId(inner.req), hash: inner.params.hash },
        E::BlobWrite(inner) => Effect::BlobWrite {
            req: RequestId(inner.req),
            media_type: decode_json(&inner.params.media_type).await.unwrap_or(semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value }),
            bytes: inner.params.bytes,
        },
        E::HttpRequest(inner) => Effect::HttpRequest { req: RequestId(inner.req), method: inner.params.method, url: inner.params.url, headers: inner.params.headers, body: inner.params.body, stream: inner.params.streaming },
        E::ArtifactRead(inner) => Effect::DocumentRead { req: RequestId(inner.req), doc: ArtifactHandle(inner.params.doc as u128), lane: inner.params.lane },
        E::ArtifactWrite(inner) => Effect::DocumentWrite { req: RequestId(inner.req), doc: ArtifactHandle(inner.params.doc as u128), lane: inner.params.lane, ops: inner.params.ops },
        E::LinkResolve(inner) => Effect::LinkResolve { req: RequestId(inner.req), link: String::from_utf8_lossy(&inner.link).into_owned() },
        E::RegistryQuery(inner) => Effect::RegistryQuery { req: RequestId(inner.req), kind: inner.params.kind, filter: decode_dsl(&inner.params.filter).await },
        E::IoCompose(inner) => Effect::IoCompose { req: RequestId(inner.req), key: String::from_utf8_lossy(&inner.params.key).into_owned(), sources: decode_json(&inner.params.sources).await.unwrap_or_default() },
        E::IoRun(_inner) => return Err(PluginHostError::Plugin("effect io-run has no semio_framework::kernel::Effect variant yet (needs A3 to add Effect::IoRun) — see 📓️terra-B1-host-native-report.md".to_string())),
        E::CacheDerive(inner) => Effect::CacheDerive { req: RequestId(inner.req), engine_id: inner.params.engine_id, input: inner.params.input },
        E::CacheRead(inner) => Effect::CacheRead { req: RequestId(inner.req), engine_id: inner.params.engine_id, key: String::from_utf8_lossy(&inner.params.key).into_owned() },
        E::OpenWindow(inner) => Effect::OpenWindow { req: RequestId(inner.req), kind: WindowKindId(inner.params.kind), params: decode_dsl(&inner.params.params).await.unwrap_or(DslValue::Null) },
        E::CloseWindow(inner) => Effect::CloseWindow { window: WindowHandle(inner.window as u128) },
        E::DispatchAction(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::DispatchAction { req: RequestId(inner.req), action: inner.params.action, args, delay_ms: inner.params.delay_ms }
        }
        E::InvokeExtension(inner) => Effect::invoke_extension(RequestId(inner.req), inner.params.extension_id, inner.params.capability, String::from_utf8_lossy(&inner.params.payload).into_owned()),
        E::Notify(inner) => Effect::Notify { message: inner.message },
        E::ClipboardWrite(inner) => Effect::ClipboardWrite { fragment: decode_json(&inner.fragment).await.ok_or_else(|| PluginHostError::Plugin("clipboard-write-effect.fragment failed to decode as JSON ClipboardFragment".to_string()))? },
        E::Navigate(inner) => Effect::Navigate { uri: inner.uri },
        E::OpenExternalUrl(inner) => Effect::OpenExternalUrl { url: inner.url },
        E::SetPanel(inner) => Effect::SetPanel { panel_json: inner.panel_json },
        E::SetActiveUtility(inner) => Effect::SetActiveUtility { window_id: inner.window_id, utility_id: inner.utility_id },
        E::SetActiveTool(inner) => Effect::SetActiveTool { tool_id: inner.tool_id },
        E::ReplayShellCommand(inner) => {
            let args = match inner.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::ReplayShellCommand { action_id: inner.action_id, args }
        }
        E::SpawnPluginInstance(inner) => {
            Effect::SpawnPluginInstance { req: RequestId(inner.req), plugin_id: inner.params.plugin_id, app_id: inner.params.app_id, os_instance_id: inner.params.os_instance_id, label: inner.params.label, document_json: inner.params.artifact_json }
        }
        E::OpenPluginInstance(inner) => Effect::OpenPluginInstance { plugin_id: inner.plugin_id, app_id: inner.app_id, os_instance_id: inner.os_instance_id },
        E::OpenDialog(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::OpenDialog { req: RequestId(inner.req), dialog_id: inner.params.dialog_id, args }
        }
        E::IconRenderExport(inner) => Effect::IconRenderExport { items: decode_json(&inner.items).await.unwrap_or_default() },
        E::VideoRenderExport(inner) => Effect::VideoRenderExport { filename: inner.filename, program: decode_dsl(&inner.program).await.and_then(|value| semio_framework_value::FromValue::from_value(value).ok()).unwrap_or_default() },
        E::DownloadMediaExport(inner) => Effect::DownloadMediaExport { filename: inner.filename, mime_type: inner.mime_type, data: inner.data, encoding: inner.encoding },
        E::RequestFileOpen(inner) => {
            let args = match inner.params.args { Some(bytes) => decode_dsl(&bytes).await, None => None };
            Effect::RequestFileOpen { req: RequestId(inner.req), accept: inner.params.accept, read_as: inner.params.read_as, import_action: inner.params.import_action, multiple: inner.params.multiple, args }
        },
        E::RequestMediaFrames(inner) => {
            let args = match inner.params.args {
                Some(bytes) => decode_dsl(&bytes).await,
                None => None,
            };
            Effect::RequestMediaFrames {
                req: RequestId(inner.req),
                accept: inner.params.accept,
                frame_action: String::new(),
                done_action: String::new(),
                fallback_action: String::new(),
                sample_stride: inner.params.sample_stride,
                max_frames: inner.params.max_frames,
                max_long_edge_px: inner.params.max_long_edge_px,
                fps_hint: inner.params.fps_hint,
                payload: inner.params.payload,
                args,
            }
        }
        E::LoadDocument(inner) => Effect::LoadDocument { pack: inner.doc_pack, spr: inner.spr },
        E::RequestSync => Effect::RequestSync,
        E::SetTimer(inner) => Effect::SetTimer { id: inner.id, after_ms: inner.after_ms as u64, repeat: inner.repeat },
        E::SpawnJob(inner) => Effect::SpawnJob {
            job: inner.job,
            kind: inner.kind,
            input: inner.input,
            placement: match inner.placement {
                wit_effects::JobPlacement::Inline => JobPlacement::Inline,
                wit_effects::JobPlacement::Isolated => JobPlacement::Isolated,
                wit_effects::JobPlacement::Exclusive => JobPlacement::Exclusive,
            },
        },
        E::CancelJob(inner) => Effect::CancelJob { job: inner.job },
        E::Respond(inner) => Effect::Respond {
            req: RequestId(inner.req),
            result: match inner.outcome {
                wit_effects::RespondResult::Ok(bytes) => RequestOutcome::Ok(bytes),
                wit_effects::RespondResult::Fault(bytes) => RequestOutcome::Err(bytes),
            },
        },
        E::StorageRead(inner) => Effect::StorageRead { req: RequestId(inner.req), key: inner.params.key },
        E::StorageWrite(inner) => Effect::StorageWrite { req: RequestId(inner.req), key: inner.params.key, bytes: inner.params.value },
        E::StorageDelete(inner) => Effect::StorageDelete { req: RequestId(inner.req), key: inner.params.key },
        E::RequestCapability(inner) => {
            Effect::RequestCapability { req: RequestId(inner.req), capability: CapabilityRequest { id: CapabilityId(inner.params.id), scope: inner.params.scope, reason: inner.params.reason, optional: inner.params.optional } }
        }
        E::ReleaseCapability(inner) => Effect::ReleaseCapability { id: CapabilityId(inner.id) },
        E::Subscribe(inner) => Effect::Subscribe { topic: inner.topic },
        E::Unsubscribe(inner) => Effect::Unsubscribe { topic: inner.topic },
        E::RequestServiceOperation(inner) => Effect::RequestServiceOperation {
            owner: inner.owner,
            service_id: inner.service_id,
            action: inner.action,
            payload: decode_dsl(&inner.payload).await.ok_or_else(|| PluginHostError::Plugin("Invalid service operation payload".into()))?,
        },
    })
}

/// 🐛️ `SendMessageEffect.target` is `wit_types::MessageEndpoint` (from the `types` interface via
/// `use types.{message-endpoint}` in `effects.wit`) — same generated Rust type as
/// `wit_message_endpoint_to_kernel` above takes, just named to keep the giant match arm list above
/// readable.
async fn kernel_message_endpoint_to_wit_reverse(endpoint: wit_types::MessageEndpoint) -> MessageEndpoint {
    wit_message_endpoint_to_kernel(endpoint).await
}

async fn wit_surface_ref(instance_id: u32, surface: &str) -> wit_ui::SurfaceRef {
    let body_key = surface.split_once(':').map_or(surface, |(_, body_key)| body_key);
    wit_ui::SurfaceRef { instance: instance_id, surface: body_key.to_owned() }
}

/// 🏁️ Host → guest: `semio_framework::kernel::Event` to WIT `event` (`📜️wit/📜️events.wit`).
/// `instance_id` fills the WIT `instance` field several kernel lifecycle variants dropped (see
/// `WasmtimeInstanceState::instance_id`'s docstring).
fn kernel_lifetime_to_wit(value: semio_framework::kernel::ActorInstanceLifetime) -> wit_lifetime::Lifetime {
    wit_lifetime::Lifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
}

fn wit_lifetime_to_kernel(value: wit_lifetime::Lifetime) -> semio_framework::kernel::ActorInstanceLifetime {
    semio_framework::kernel::ActorInstanceLifetime { activation_generation: value.activation_generation, instance_id: value.instance_id, guest_lifetime: value.guest_lifetime }
}

fn kernel_patch_receipt_to_wit(value: semio_framework::kernel::ActorUiPatchReceipt) -> wit_lifetime::UiPatchReceipt {
    wit_lifetime::UiPatchReceipt { lifetime: kernel_lifetime_to_wit(value.lifetime), patch_sequence: value.patch_sequence }
}

fn wit_patch_receipt_to_kernel(value: wit_lifetime::UiPatchReceipt) -> semio_framework::kernel::ActorUiPatchReceipt {
    semio_framework::kernel::ActorUiPatchReceipt { lifetime: wit_lifetime_to_kernel(value.lifetime), patch_sequence: value.patch_sequence }
}

fn kernel_lifecycle_receipt_to_wit(value: semio_framework::kernel::ActorInstanceLifecycleReceipt) -> wit_lifetime::Receipt {
    use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
    match value {
        R::Captured { lifetime, request_sequence } => wit_lifetime::Receipt::Captured(wit_lifetime::CapturedReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence }),
        R::Accepted { lifetime, request_sequence, close_generation } => wit_lifetime::Receipt::Accepted(wit_lifetime::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
        R::Retired { lifetime, request_sequence, close_generation } => wit_lifetime::Receipt::Retired(wit_lifetime::CloseReceipt { lifetime: kernel_lifetime_to_wit(lifetime), request_sequence, close_generation }),
    }
}

fn wit_lifecycle_receipt_to_kernel(value: wit_lifetime::Receipt) -> semio_framework::kernel::ActorInstanceLifecycleReceipt {
    use semio_framework::kernel::ActorInstanceLifecycleReceipt as R;
    match value {
        wit_lifetime::Receipt::Captured(value) => R::Captured { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence },
        wit_lifetime::Receipt::Accepted(value) => R::Accepted { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
        wit_lifetime::Receipt::Retired(value) => R::Retired { lifetime: wit_lifetime_to_kernel(value.lifetime), request_sequence: value.request_sequence, close_generation: value.close_generation },
    }
}

async fn kernel_turn_inputs_to_wit(events: &[Event], instance_id: u32) -> Result<(Vec<wit_events::Event>, Option<(wit_reactor::CommandPageCursor, Vec<u8>)>, Option<wit_reactor::ColdDocumentPairPage>), TurnFault> {
    let mut ordinary = Vec::with_capacity(events.len());
    let mut command = None;
    let mut cold = None;
    for event in events {
        match event {
            Event::CommandIngressPage { cursor, bytes } => {
                if command.is_some() || bytes.len() > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES || (bytes.is_empty() && !(cursor.kind == 28 && cursor.item_count == 0)) {
                    return Err(TurnFault::Trapped("turn carries duplicate or invalid command page".into()));
                }
                command = Some(kernel_command_page_to_wit(cursor, bytes));
            }
            Event::ColdDocumentPairPage(page) => {
                if cold.is_some() {
                    return Err(TurnFault::Trapped("turn carries duplicate cold-pair page".into()));
                }
                cold = Some(kernel_cold_page_to_wit(page)?);
            }
            event => ordinary.push(kernel_event_to_wit(event, instance_id).await),
        }
    }
    Ok((ordinary, command, cold))
}

/// 🚫️async: R10 residue shape 1 — `kernel_broker_grant_to_wit` is async, hoisted out of
/// the sync `Iterator::map` via a plain loop.
///
/// 🐛️ WIT `request-event.from` was renamed `origin` — `from` is WIT-reserved (the SAME
/// reserved-keyword class B1's report already fixed for `stream`/`result`; this one was
/// fixed by A2 between B1's last pass and now, per the packet brief's "guest side is green").
pub(crate) async fn kernel_event_to_wit(event: &Event, instance_id: u32) -> wit_events::Event {
    match event {
        Event::InstanceOpen { request, app_id, actor, config, assets, capabilities, quotas } => {
            let mut wit_capabilities = Vec::with_capacity(capabilities.len());
            for capability in capabilities {
                wit_capabilities.push(kernel_broker_grant_to_wit(capability).await);
            }
            wit_events::Event::InstanceOpen(wit_events::InstanceOpenEvent {
                instance: request.instance_id,
                activation_generation: request.activation_generation,
                request_sequence: request.request_sequence,
                app_id: app_id.0.clone(),
                actor: actor.clone(),
                config: config.clone(),
                assets: assets.clone(),
                capabilities: wit_capabilities,
                quotas: encode_json(quotas).await,
            })
        }
        Event::InstanceClose(request) => wit_events::Event::InstanceClose(wit_lifetime::CloseRequest { lifetime: kernel_lifetime_to_wit(request.lifetime), request_sequence: request.request_sequence }),
        Event::InstanceLifecycleAck(ack) => wit_events::Event::InstanceLifecycleAck(kernel_lifecycle_receipt_to_wit(ack.receipt)),
        Event::Activate { reason } => wit_events::Event::Activate(wit_events::ActivateEvent { instance: instance_id, reason: kernel_activation_event_to_wit(reason).await }),
        Event::SuspendRequest => wit_events::Event::SuspendRequest(wit_events::SuspendRequestEvent { instance: instance_id }),
        Event::CapabilityChanged { change } => wit_events::Event::CapabilityChanged(wit_events::CapabilityChangedEvent { instance: instance_id, change: kernel_capability_change_to_wit(change).await }),
        Event::QuotaChanged { quotas } => wit_events::Event::QuotaChanged(wit_events::QuotaChangedEvent { instance: instance_id, quotas: encode_json(quotas).await }),
        Event::ColdDocumentPairPage(_) => unreachable!("cold pages use the dedicated poll input"),
        Event::CommandIngressPage { .. } => unreachable!("command pages are lifted through reactor.poll's dedicated page argument"),
        Event::UiIntent { instance, intent } => wit_events::Event::UiIntent(wit_events::UiIntentEvent { instance: instance.0.parse().unwrap_or(instance_id), intent: intent.clone() }),
        Event::SurfaceVisible { surface, body_key, view_state } => {
            wit_events::Event::SurfaceVisible(wit_events::SurfaceVisibleEvent { surface: wit_surface_ref(instance_id, surface).await, body_key: body_key.clone(), view_state: view_state.clone() })
        }
        Event::SurfaceHidden { surface } => wit_events::Event::SurfaceHidden(wit_events::SurfaceHiddenEvent { surface: wit_surface_ref(instance_id, surface).await }),
        Event::SurfaceResized { surface, width, height } => wit_events::Event::SurfaceResized(wit_events::SurfaceResizedEvent { surface: wit_surface_ref(instance_id, surface).await, width: *width, height: *height }),
        Event::PatchAck { receipt, surface, revision } => wit_events::Event::PatchAck(wit_events::PatchAckEvent { receipt: kernel_patch_receipt_to_wit(*receipt), surface: wit_surface_ref(instance_id, surface).await, revision: *revision }),
        Event::PatchRejected { receipt, surface, revision, reason } => {
            wit_events::Event::PatchRejected(wit_events::PatchRejectedEvent { receipt: kernel_patch_receipt_to_wit(*receipt), surface: wit_surface_ref(instance_id, surface).await, revision: *revision, reason: reason.clone() })
        }
        Event::Completed { req, result } => wit_events::Event::Completed(wit_events::CompletedEvent { req: req.0, outcome: kernel_request_outcome_to_wit(result).await }),
        Event::HttpChunk { req, bytes, done } => wit_events::Event::HttpChunk(wit_events::HttpChunkEvent { req: req.0, params: wit_events::HttpChunkParams { bytes: bytes.clone(), done: *done } }),
        Event::JobProgress { job, progress } => wit_events::Event::JobProgress(wit_events::JobProgressEvent { job: *job, progress: progress.clone().unwrap_or_default() }),
        Event::JobCompleted { job, result } => wit_events::Event::JobCompleted(wit_events::JobCompletedEvent { job: *job, outcome: kernel_request_outcome_to_wit(result).await }),
        Event::Message { source, payload } => wit_events::Event::Message(wit_events::MessageEvent { source: kernel_message_endpoint_to_wit(source).await, payload: payload.clone() }),
        Event::Timer { id } => wit_events::Event::Timer(wit_events::TimerEvent { id: *id }),
        Event::Wake => wit_events::Event::Wake,
        Event::Request { req, from, capability, payload } => {
            wit_events::Event::Request(wit_events::RequestEvent { req: req.0, params: wit_events::RequestParams { origin: kernel_message_endpoint_to_wit(from).await, capability: capability.clone(), payload: payload.clone() } })
        }
    }
}

async fn kernel_activation_event_to_wit(reason: &semio_framework::kernel::ActivationEvent) -> wit_events::ActivationEvent {
    use semio_framework::kernel::ActivationEvent as A;
    match reason {
        A::OnCommand { id } => wit_events::ActivationEvent::OnCommand(id.clone()),
        A::OnViewVisible { id } => wit_events::ActivationEvent::OnViewVisible(id.clone()),
        A::OnFileType { ext } => wit_events::ActivationEvent::OnFileType(ext.clone()),
        A::OnArtifactKind { kind } => wit_events::ActivationEvent::OnArtifactKind(kind.clone()),
        A::OnExtensionRequest { point } => wit_events::ActivationEvent::OnExtensionRequest(point.clone()),
        A::OnStartupFinished => wit_events::ActivationEvent::OnStartupFinished,
    }
}

async fn kernel_capability_change_to_wit(change: &semio_framework::kernel::CapabilityChange) -> wit_capabilities::CapabilityChange {
    use semio_framework::kernel::CapabilityChange as C;
    match change {
        C::Granted { id: _, grant } => wit_capabilities::CapabilityChange::Granted(kernel_broker_grant_to_wit(grant).await),
        C::Revoked { id } => wit_capabilities::CapabilityChange::Revoked(id.0.clone()),
        C::Narrowed { id: _, grant } => wit_capabilities::CapabilityChange::Narrowed(kernel_broker_grant_to_wit(grant).await),
    }
}

async fn kernel_broker_grant_to_wit(grant: &BrokerCapabilityGrant) -> wit_capabilities::CapabilityGrant {
    wit_capabilities::CapabilityGrant { token: wit_capabilities::CapabilityToken { id: grant.id.0.clone(), token: grant.token.0 as u64 }, scope: grant.scope.clone(), expires_ms: grant.expires_ms.map(|value| value as i64) }
}
//#endregion 🔀️EffectEventMarshal

#[cfg(test)]
#[path = "../🧪️tests/🔬️wasmtime-runtime/🦀️.rs"]
mod wasmtime_runtime_tests;
//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime
