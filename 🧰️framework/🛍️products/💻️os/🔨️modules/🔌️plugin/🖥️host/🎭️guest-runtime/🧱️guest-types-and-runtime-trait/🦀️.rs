//! 🧱️ Guest types and runtime trait: the package reference, the compiled handle, the guest instance with its state, the guest fault helpers and the `GuestRuntime` trait. See [`super`].

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
    pub(crate) component: Option<Arc<Component>>,
    pub(crate) owned: Option<OwnedCompiledGuest>,
}

impl CompiledHandle {
    /// 🔌️ Whether a real wasmtime component was compiled; the backend type itself stays inside this implementation.
    pub fn has_component(&self) -> bool {
        self.component.is_some()
    }

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
    pub state: GuestInstanceState,
}

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

pub enum GuestInstanceState {
    Closed,
    /// 🧪️ Only `MockGuestRuntime` (`#[cfg(any(test, feature = "testkit"))]`) ever constructs this variant — `#[allow(dead_code)]`
    /// rather than `#[cfg(any(test, feature = "testkit"))]` on the variant itself, so `GuestInstanceState` stays a REFUTABLE
    /// match target (`Wasmtime` is not the only variant) in every build config; a `#[cfg(any(test, feature = "testkit"))]`
    /// variant would make the `let GuestInstanceState::Wasmtime(state) = .. else { .. }` guards in
    /// `WasmtimeRuntime`'s own methods irrefutable outside `cfg(test)`, turning their `else` arms into
    /// a NEW dead-code warning instead of fixing one.
    #[allow(dead_code)]
    Mock(MockInstanceState),
    Owned(OwnedGuestInstance),
    Wasmtime(WasmtimeInstanceState),
}

/// 🧯️ A Wasmtime original-operation receive failure: a refused admission keeps its typed refusal, any other runtime error traps.
pub(crate) fn receiving_turn_fault(error:semio_framework_plugin_host_operation::ReceiveError)->TurnFault{match error{semio_framework_plugin_host_operation::ReceiveError::Refused(error)=>identity_turn_fault(error),semio_framework_plugin_host_operation::ReceiveError::Runtime(error)=>TurnFault::Trapped(error.to_string())}}

pub(super) fn decode_guest_plugin_error(error: wit_types::PluginError) -> TurnFault {
    match error {
        wit_types::PluginError::Fault(bytes) => decode_guest_fault_bytes(&bytes),
        wit_types::PluginError::OperationRefusal(code)=>identity_turn_fault(semio_framework_plugin_guest_contract::operation_authority::refusal(code)),
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
    async fn admit_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorAdmissionReply,TurnFault>;
    async fn close_actor_context(&self,inst:&mut GuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<semio_framework_actor::retained_context::OriginalActorCloseReply,TurnFault>;
    async fn execute_turn(&self, inst: &mut GuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault>;
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
    async fn checkpoint(&self, inst: &mut GuestInstance, identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError>;
    async fn restore(&self, inst: &mut GuestInstance, state: &[u8], identity: &mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), PluginHostError>;
    async fn drop_instance(&self, inst: GuestInstance);
}
//#endregion 🎭️GuestRuntime
