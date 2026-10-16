//! 🧠️ Wasmtime node host: the native node host that opens, drives and closes each node's app instance.
//!
//! Holds the `🔖️WasmtimeNodeHost`, `🔖️OpeningCommands` regions of the module; the root re-exports its items.

use super::*;

//#region 🔖️WasmtimeNodeHost
/// 🧩️ Native `AppChannelHost` over `semio-framework-plugin-host`'s `GuestRuntime` — the
/// `WasmPluginRuntime`-based synchronous-exchange design this doc used to describe is gone
/// (`📌️important.md`'s "Replace, never wrap" list); `world actor` exports only `reactor::poll`/
/// `jobs::{start-job,step-job,cancel-job}`/`checkpoint`/`describe`, no per-verb calls and no
/// `exchange` at all. Channel v12 (`📡️spr/🧵️channel/🦀️.rs`) already anticipates this:
/// "lifecycle now arrives through the reactor ABI's `Event::InstanceOpen`/`InstanceClose`" — `open`
/// now instantiates a real `GuestInstance` and submits `Event::InstanceOpen`; `exchange` submits the
/// batch through the retained command-page ingress owner and reads back `Effect::Respond`
/// effects correlated by `req.0 == seq`.
///
/// ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (R1-native-manifest): `load_runtime_recursive` now
/// reads a real `PluginManifest` — via `PackageDescriptor` (packet E1's build-time-emitted
/// `🛂️.descriptor.semio`), NOT via a wasm call. `design-abi.md` §3's freshness test
/// (`descriptor_is_fresh`) already byte-compares each plugin's OWN native `describe()` build against
/// its committed `🛂️.descriptor.semio`, and this ticket's whole premise (2550 installed records, zero
/// runtime resources) forbids re-instantiating a component just to read its manifest — so
/// `descriptor_path_for_plugin` (populated from the registry's `cratePath`, see `🏗️bootstrap/🦀️.rs`) is tried
/// FIRST: zero instantiations. Only a plugin with no committed descriptor yet (most of the 33 today
/// — packet D0 is still landing the emission plumbing) falls through to a hard, honest error; a live
/// `describe()` fallback is DESIGNED (this struct's own doc, `📓️terra-R1-report.md` §1) but not
/// wired here — it needs a `GuestRuntime::describe` seam on `🔌️plugin/🖥️host/🦀️.rs`, a
/// file two sibling packets (P1, T1) are live in right now (lease-requested, not edited).
///
/// 🚧️ **Still a documented gap**: `run_transaction`/`undo_transaction_group`'s `exec`/`plan`
/// closures have a DIFFERENT gap one layer up — full per-command reply correlation and
/// contributed-mutation planning over `execute_turn`'s effects is a real post-turn dispatch loop that
/// belongs with the kernel/scheduler (H1-H4/T1), not this packet — they compile and are correctly
/// typed, but always return a clearly-worded `TransactionError` today.
#[cfg(not(target_arch = "wasm32"))]
// 🔀️ `B: BlobStore + 'static = NoBlobStore` — same open-extension-point reasoning as `SpaceRunner`'s
// own `B` (see its doc comment). `blob_store` here is `#[allow(dead_code)]` (reserved, unused) but
// still a real caller-supplied `new()` parameter, so the same generic treatment applies.
pub struct WasmtimeNodeHost<B: BlobStore + 'static = NoBlobStore> {
    /// 🧬️ Compiles components and (for `load_runtime_recursive`'s own plugin-service instance only)
    /// instantiates them directly — see `kernel`'s own doc for why that ONE call site does not go
    /// through the full kernel-activation facade.
    guest_runtime: Arc<semio_framework_plugin_host::GuestRuntimes>,
    host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy,
    host_driver:Arc<semio_framework_plugin_host::GuestRelayWakeDriver>,
    direct_identity:semio_framework_plugin_host::shard::OriginalShardIdentityIssuer,
    service_identities:HashMap<String,semio_framework_plugin_host::shard::OriginalShardIdentity>,
    service_openings:Option<ServiceActorOpening>,
    service_identity_refusal:Option<semio_framework_plugin_host::shard::ShardIdentityIssueRefused>,
    /// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the real kernel this
    /// host now drives instead of minting `RuntimeActorId`s ad hoc — see `semio-framework-os`'s
    /// `🎠️activation/🦀️.rs` module doc. `open`'s real per-app-instance actor uses `kernel.activate`
    /// (mints the id, pins a shard, hands the `GuestInstance` to that shard's own `ShardExecutor`
    /// thread); `load_runtime_recursive`'s per-plugin router-registration instance uses only
    /// `kernel.kernel_mut().activate` (id-minting/bookkeeping, no shard handoff) because
    /// `PluginInstanceHandle` still calls `GuestRuntime::execute_turn` directly on its own
    /// `GuestInstance` — see that field's own doc for why those are genuinely two different shapes.
    kernel: NativeKernelRuntime,
    plugin_path_for_plugin: HashMap<String, PathBuf>,
    /// 📦️ `plugin_id -> its committed 🛂️.descriptor.semio path` (registry `cratePath`-derived, see
    /// `🏗️bootstrap/🦀️.rs`'s `resolve_descriptor_paths`) — `load_runtime_recursive`'s FIRST, zero-instantiation
    /// manifest source (this struct's own doc). A plugin absent from this map, or whose file does not
    /// exist on disk yet, has no committed descriptor — `load_runtime_recursive` fails loudly for it
    /// rather than falling back to a live `describe()` call (not wired here — see this struct's doc).
    descriptor_path_for_plugin: HashMap<String, PathBuf>,
    /// 🧬️ Content-addressed compiled-component cache, one entry per plugin id — mirrors
    /// `SharedWasmtimeEngine`'s own on-disk `.cwasm` cache one level up (in-process, per-host-instance).
    compiled_for_plugin: HashMap<String, semio_framework_plugin_host::CompiledHandle>,
    /// 🗺️ Every plugin id this host has successfully loaded a manifest for — see this struct's own
    /// doc for why `load_runtime_recursive` cannot populate this yet.
    manifests: HashMap<String, PluginManifest>,
    /// 🚧️ Reserved for the future post-turn effect dispatcher (`Effect::BlobLoad`/`BlobWrite`) — the
    /// OLD synchronous `register_host_blob_store`/`HostState.blob_store` wiring is gone with
    /// `HostState` itself; nothing calls into this yet.
    #[allow(dead_code)]
    pub(super) blob_store: Arc<B>,
    /// 🌉️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION (D3): shared
    /// across every plugin this host lazily loads, so any loaded plugin's routed `io-run`/`io-sniff`
    /// job dispatch can reach any OTHER loaded plugin's registry — see `IoRouter`'s own doc comment.
    io_router: Arc<semio_framework_plugin_host::IoRouter>,
    /// 🕸️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A): every loaded
    /// plugin's manifest, dependency-validated — `runtime_for` registers into this BEFORE a
    /// plugin's own routers, and only after every declared dependency has itself been loaded (see
    /// `load_runtime_recursive`).
    plugin_graph: Arc<semio_framework_plugin_host::PluginGraph>,
    /// 🎯️ Every loaded plugin's `contributor.list-artifact-mutations` roster, merged.
    mutation_router: Arc<semio_framework_plugin_host::ArtifactMutationRouter>,
    /// 💡️ Contributor-aware artifact inference router.
    inference_router: Arc<semio_framework_plugin_host::ArtifactInferenceRouter>,
    /// 🗺️ `ArtifactRef -> (plugin_id, instance_id, artifact_kind)`, populated by `open` at
    /// instantiate-app time.
    instance_directory: Arc<semio_framework_plugin_host::InstanceDirectory>,
    /// 🎯️ Drives contract §5 transactions over this host's own loaded instances — see
    /// `run_transaction`.
    transaction_coordinator: Arc<semio_framework_plugin_host::HostTransactionCoordinator>,
    /// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (W1-A): every loaded plugin's
    /// viewer/editor surfaces — see `resolve_open_artifact`/`set_default_app`/`clear_default_app`.
    app_router: Arc<semio_framework_plugin_host::AppRouter>,
    /// 🎚️ Folded `OpeningPreferences` snapshot — event-sourced (contract §4): every write goes
    /// through `apply_opening_config_mutation` on a typed `OpeningConfigMutation`, never a direct
    /// field mutation. Persisted local-only for now (no `ConfigStore`/disk binding wired yet — see
    /// `set_default_app`'s doc); a real multi-session deployment would fold this from a durable op
    /// log at boot instead of starting empty every run.
    opening_preferences: semio_framework_os_config::opening_config::OpeningPreferences,
    /// 🧬️ `open`'s real body mints the returned handle from this counter — see that method.
    pub(super) next_handle: u32,
    /// ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): `Kernel::activate`
    /// takes a caller-supplied `plugin_ordinal: u16`, one per DISTINCT plugin id (not one per
    /// activation — the SAME convention `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🦀️.rs`'s own `kernel_runtime::
    /// plugin_ordinal` uses), so both `load_runtime_recursive`'s plugin-service actor and `open`'s
    /// per-app-instance actors for the SAME plugin share one ordinal. Replaces the old raw
    /// `next_actor_ordinal: u64` counter, which minted every actor id ad hoc outside the kernel.
    plugin_ordinals: HashMap<String, u16>,
    /// 🗺️ `handle -> (plugin_id, instance_id)` — the SAME addressing scheme `WasmPluginRuntime` used
    /// (one runtime hosting many `create_app`-minted instance ids); the live `GuestInstance` for
    /// `(plugin_id, instance_id)` lives in `guest_instances` below, since `design-abi.md §4`'s actor
    /// model gives each app instance its OWN `GuestInstance`/`RuntimeActorId` now, not a shared store.
    instances: HashMap<u32, (String, u32)>,
    /// ✅️ `handle -> ActorId` for every instance `open` has activated through `kernel` — `run_turn`'s
    /// own lookup key for `Kernel::submit`/`tick_and_dispatch`/`complete`.
    instance_actors: HashMap<u32, RuntimeActorId>,
    /// 🧬️ Not populated yet — `open`'s own `GuestInstance` is owned by its pinned `ShardExecutor`
    /// thread now (see `kernel`'s own doc), not held here; this map is reserved for a future
    /// suspend/resume/checkpoint path that needs to read a live instance's state back out.
    #[allow(dead_code)]
    guest_instances: HashMap<(String, u32), semio_framework_plugin_host::GuestInstance>,
    /// ⏱️ Monotonic milliseconds this host's own `Kernel::tick` calls are stamped with — same
    /// purity-respecting clock source as `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🦀️.rs`'s `KernelThreadState::now_ms`
    /// (`Kernel` itself takes no clock, per `🎭️actor`'s own rule), incremented once per `run_turn`.
    now_ms: u64,
    /// 🔢️ `Envelope.seq` source for this host's own turns — distinct
    /// from `SpaceRunner::compute_node`'s own per-node `next_seq` closure (that one numbers
    /// `AppCommand`s within a single batched `exchange` call; this one numbers kernel envelopes).
    next_turn_seq: u64,
    retained_command_closes: semio_framework::kernel::CommandDriverRegistry<1>,
    rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry<1>,
}

#[cfg(not(target_arch="wasm32"))]
struct ServiceActorOpening{actor:RuntimeActorId,plugin_ordinal:u16,package_hash:[u8;32],opening:Option<semio_framework_plugin_host::GuestActorOpening>,instance:Option<semio_framework_plugin_host::GuestInstance>,input:Option<semio_framework_actor::retained_turn::OriginalActorInput<semio_framework_plugin_host::GuestInstance>>,scope:Option<semio_framework_actor::RetainedTurnInput>,execution:Option<semio_framework_actor::OriginalGuestExecutionBudget>,return_pending:Option<semio_framework_actor::RetainedTurnReceipt>,returned:Option<semio_framework_actor::RetainedTurnReceipt>,admitted:Option<semio_framework_actor::RetainedTurnInput>,pub(super) fault:Option<semio_framework_plugin_guest_contract::TurnFault>,refusal:Option<(semio_framework_actor::RetainedTurnInput,semio_framework_value::ValueRefusalKind)>}

#[cfg(not(target_arch="wasm32"))]
fn run_original_input(cx:&semio_framework_job::StepContext<'_>)->Result<semio_framework_actor::RetainedTurnInput,semio_framework_job::OriginalTurnFailure>{Ok(semio_framework_actor::RetainedTurnInput::borrow_original(cx)?)}

#[cfg(not(target_arch = "wasm32"))]
impl<B: BlobStore + 'static> WasmtimeNodeHost<B> {
    /// 🗺️ `plugin_path_for_plugin` maps a plugin id (`WorkflowNode::plugin_id`, the same id
    /// `PLUGIN_WASM_ARTIFACTS`' first tuple element names) to the compiled `.wasm` component path the
    /// dev shell build already produces under `framework/os/dev/plugin-modules/<plugin id>/`.
    /// `descriptor_path_for_plugin` maps the SAME plugin id to its committed `🛂️.descriptor.semio`
    /// (registry `cratePath`-derived — see `🏗️bootstrap/🦀️.rs`'s `resolve_descriptor_paths`); absent entries
    /// simply have no committed descriptor yet (this struct's own doc).
    /// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): `async` now — it
    /// builds `kernel` (`NativeKernelRuntime::new`, itself `async` since `Kernel::new`/`Kernel::
    /// activate` are, per the crate-wide universal-async decree, O1). `shard_count: 1`: this host's
    /// own contract is strictly sequential, one node/turn at a time (`AppChannelHost`'s own doc:
    /// "the guest contract is still exactly one turn at a time per instance ... a caller that wants
    /// concurrency must open distinct instances, never overlap two turns on one" — and
    /// `SpaceRunner::run`'s node loop never opens two instances concurrently either), so a second
    /// shard thread would sit idle. The type stays general (same `shard_count` parameter
    /// `ParallelRuntime::new` takes) for a future caller that does want more.
    pub async fn new(plugin_path_for_plugin: HashMap<String, PathBuf>, descriptor_path_for_plugin: HashMap<String, PathBuf>, blob_store: Arc<B>, identity:semio_framework_plugin_host::shard::OriginalShardIdentityIssuer, host_driver_policy:semio_framework_plugin_host::GuestRelayDriverPolicy, original_clock:fn()->Option<u64>, original_cancel:CancelToken, direct_identity:semio_framework_plugin_host::shard::OriginalShardIdentityIssuer) -> Result<Self,RunError> {
        let host_driver=Arc::new(semio_framework_plugin_host::GuestRelayWakeDriver::new(host_driver_policy,original_clock,Box::new(move||original_cancel.is_cancelled_now())).map_err(|error|RunError::Original(error.into()))?);
        let guest_runtime = Arc::new(semio_framework_plugin_host::GuestRuntimes::Owned(semio_framework_plugin_interpreter_runtime::OwnedRuntime::new(original_clock)));
        let kernel = NativeKernelRuntime::new(guest_runtime.clone(), 1, 0, 64,{let mut original=Some(identity);move |_|original.take().expect("sequential host has one original shard issuer")}).await;
        Ok(Self {
            guest_runtime,
            host_driver_policy,
            host_driver,direct_identity,service_identities:HashMap::new(),service_openings:None,service_identity_refusal:None,
            kernel,
            plugin_path_for_plugin,
            descriptor_path_for_plugin,
            compiled_for_plugin: HashMap::new(),
            manifests: HashMap::new(),
            blob_store,
            io_router: Arc::new(semio_framework_plugin_host::IoRouter::new()),
            plugin_graph: Arc::new(semio_framework_plugin_host::PluginGraph::new()),
            mutation_router: Arc::new(semio_framework_plugin_host::ArtifactMutationRouter::new()),
            inference_router: Arc::new(semio_framework_plugin_host::ArtifactInferenceRouter::new()),
            instance_directory: Arc::new(semio_framework_plugin_host::InstanceDirectory::new()),
            transaction_coordinator: Arc::new(semio_framework_plugin_host::HostTransactionCoordinator::new()),
            app_router: Arc::new(semio_framework_plugin_host::AppRouter::new()),
            opening_preferences: semio_framework_os_config::opening_config::OpeningPreferences::default(),
            next_handle: 1,
            plugin_ordinals: HashMap::new(),
            instances: HashMap::new(),
            instance_actors: HashMap::new(),
            guest_instances: HashMap::new(),
            now_ms: 0,
            next_turn_seq: 1,
            retained_command_closes: semio_framework::kernel::CommandDriverRegistry::new(),
            rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry::new(),
        })
    }

    /// 🔢️ One ordinal per DISTINCT plugin id — see `plugin_ordinals`' own doc.
    fn plugin_ordinal(&mut self, plugin_id: &str) -> u16 {
        let next = self.plugin_ordinals.len() as u16;
        *self.plugin_ordinals.entry(plugin_id.to_string()).or_insert(next)
    }

    /// 🌱️ The actual Run root lends every opening slice and keeps refused or pending instance custody before Handle publication.
    async fn prepare_service_original(&mut self,plugin_id:&str,compiled:&semio_framework_plugin_host::CompiledHandle)->Result<(semio_framework_plugin_host::GuestInstance,semio_framework_actor::RetainedTurnInput),RunError>{
        use semio_framework_actor::retained_turn::{ActorGuestPhase,OriginalActorInput};
        use semio_framework_plugin_interpreter_runtime::OwnedActorStartupStatus;
        while !self.host_driver.admit_root_step().map_err(RunError::Original)?{semio_framework_async::yield_once().await;}
        loop{
            let state=self.service_openings.as_mut().expect("original service actor is retained before opening");
            if let Some(receipt)=state.return_pending{if !self.host_driver.return_actor_turn(receipt).map_err(RunError::Original)?{semio_framework_async::yield_once().await;continue}state.return_pending=None;state.scope=None;state.returned=Some(receipt);}
            if let Some(fault)=&state.fault{return Err(RunError::Host(fault.to_string()))}
            if let Some((input,kind))=state.refusal{return Err(RunError::Host(format!("original service context {}/{}/{} refused: {kind:?}; its owner remains retained",input.operation,input.generation,input.epoch)))}
            if state.scope.is_none(){let issued=self.host_driver.begin_actor_turn(state.returned).map_err(|error|RunError::Original(error.into()))?;if issued.is_some(){state.returned=None;state.scope=issued;}}
            let Some(scope)=state.scope else{semio_framework_async::yield_once().await;continue};
            if state.input.as_ref().is_some_and(|input|input.phase()==ActorGuestPhase::Ready){
                let taken=self.host_driver.dispatch_actor_step(scope,|cx|{let copy=std::mem::size_of::<ServiceActorOpening>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(None)}cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);state.input.as_mut().unwrap().take_admitted_input_step(cx)}).map_err(RunError::Original)?;
                if let Some(Some(instance))=taken{self.host_driver.finish_actor_step(scope).map_err(RunError::Original)?;state.scope=None;let input=state.admitted.expect("actual guest Ready preserves its admitted input");self.service_openings.take();return Ok((instance,input));}
                state.returned=Some(self.host_driver.finish_actor_step(scope).map_err(RunError::Original)?);state.scope=None;
            }else if state.input.is_some(){
                if state.input.as_ref().unwrap().issued_input().is_none(){let budget=self.host_driver.actor_budget(scope).map_err(|error|RunError::Original(error.into()))?;state.input.as_mut().unwrap().receive_next_issued(budget.retained).map_err(|error|RunError::Original(error.into()))?;}
                let budget=self.host_driver.actor_budget(scope).map_err(|error|RunError::Original(error.into()))?;
                let identity=self.service_identities.get_mut(plugin_id).expect("original receiving identity stays owned through guest admission");
                if let Some(call)=identity.admit_owned_actor_original(&self.guest_runtime,state.input.as_mut().unwrap(),budget,self.host_driver_policy.drive).await.map_err(|error|RunError::Original(error.into()))?{
                    match call.call{Ok(reply)=>{if let Some(refusal)=reply.refusal{state.refusal=Some((reply.input,refusal));}else if reply.ready{state.admitted=Some(reply.input);}},Err(fault)=>state.fault=Some(fault)}
                    state.return_pending=Some(call.receiving.map_err(|error|RunError::Original(error.into()))?);
                }
            }else if state.instance.is_some(){
                self.host_driver.dispatch_actor_step(scope,|cx|{
                    let copy=std::mem::size_of::<OriginalActorInput<semio_framework_plugin_host::GuestInstance>>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(())}
                    cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);let issued=run_original_input(cx)?;let instance=state.instance.take().unwrap();match OriginalActorInput::new(instance,issued){Ok(input)=>state.input=Some(input),Err((error,instance))=>{state.instance=Some(instance);return Err(error.into())}}Ok(())
                }).map_err(RunError::Original)?;
                if state.input.is_none(){state.returned=Some(self.host_driver.finish_actor_step(scope).map_err(RunError::Original)?);state.scope=None;}
            }else{
                if state.execution.is_none()&&state.opening.as_ref().is_some_and(semio_framework_plugin_host::GuestActorOpening::requires_execution){state.execution=Some(self.host_driver.checkout_actor_execution_budget(scope).map_err(|error|RunError::Original(error.into()))?);}let execution=state.execution;
                let call=self.host_driver.dispatch_actor_step(scope,|cx|{
                    if state.opening.is_none(){state.opening=self.guest_runtime.begin_original_actor_step(compiled,state.actor,cx)?;return Ok(None)}
                    if state.opening.as_ref().unwrap().ready(){let input=run_original_input(cx)?;state.instance=self.guest_runtime.take_original_actor_step(&mut state.opening,compiled,input,cx)?;return Ok(None)}
                    self.guest_runtime.advance_original_actor_step(state.opening.as_mut().unwrap(),compiled,execution,cx).map(Some)
                }).map_err(RunError::Original)?;
                if let Some(Some(call))=call{if let Some(receipt)=call.execution{self.host_driver.receive_actor_execution_receipt(scope,receipt).map_err(|error|RunError::Original(error.into()))?;state.execution=None;}let status=call.result;state.returned=Some(self.host_driver.finish_actor_step(scope).map_err(RunError::Original)?);state.scope=None;match status.map_err(RunError::Original)?{OwnedActorStartupStatus::Pending|OwnedActorStartupStatus::Ready|OwnedActorStartupStatus::Deadline=>{},OwnedActorStartupStatus::Cancelled=>return Err(RunError::Host("original service startup was cancelled; its owner remains retained".into())),OwnedActorStartupStatus::ChannelMismatch{expected,observed}=>return Err(RunError::Host(format!("original service channel differs: expected {expected}, observed {observed}; its owner remains retained")))}}
                else if call.is_some(){state.returned=Some(self.host_driver.finish_actor_step(scope).map_err(RunError::Original)?);state.scope=None;}
            }
            semio_framework_async::yield_once().await;
        }
    }

    /// 🎬️ `Envelope` sequence source — see `next_turn_seq`'s own field doc.
    fn take_turn_seq(&mut self) -> u64 {
        let seq = self.next_turn_seq;
        self.next_turn_seq += 1;
        seq
    }

    /// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): drives one batch of
    /// `events` for `actor`/`instance` genuinely through the kernel — `Kernel::submit` → tick-and-
    /// dispatch to `actor`'s own pinned shard → wait for its `ShardOutcome` → `Kernel::complete` —
    /// mirroring `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🦀️.rs`'s own `KernelThreadState::run_turn` almost line for
    /// line (that file's own doc explains each step; this is the same mechanism, minus UI-surface
    /// reconciliation, which `run` has no use for). `RUN_TURN_OUTCOME_TIMEOUT`/`NODE_TURN_BUDGET` are
    /// this host's own constants, same values `load_runtime_recursive` already used for its own
    /// plugin-service instantiate budget, now shared with the real per-node turn path too.
    async fn run_turn(&mut self, actor: RuntimeActorId, events: Vec<Event>) -> Result<semio_framework::kernel::TurnResult, RunError> {
        const RUN_TURN_OUTCOME_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
        let mut envelopes = Vec::with_capacity(events.len().max(1));
        for event in &events {
            let seq = self.take_turn_seq();
            envelopes.push(Envelope { to: actor, from: Origin::Kernel, lane: Lane::Background, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: serde_json::to_vec(event).map_err(RunError::Serde)? } });
        }
        for envelope in &envelopes {
            if !matches!(self.kernel.submit(envelope).await, Backpressure::Accept) {
                return Err(RunError::Host(format!("kernel: run_turn submit for actor {} was not Accept-ed (mailbox pressure) — a single sequential run should never see this", actor.0)));
            }
        }
        let mut turn_result: Option<semio_framework::kernel::TurnResult> = None;
        let mut fault: Option<String> = None;
        loop {
            self.now_ms += 1;
            let policy=self.host_driver_policy.root;
            let decision = self.kernel.tick_and_dispatch(self.now_ms, move |_actor, original| semio_framework_plugin_host::shard::grant::ShardResourceBudget{fuel:original.fuel.min(policy.turn_execution_fuel),max_effects:original.max_effects.min(policy.max_effects),max_patch_bytes:original.max_patch_bytes.min(policy.max_patch_bytes),max_frames:original.max_frames.min(policy.max_frames),..original}).await?;
            if decision.run.is_empty() {
                break;
            }
            let outcomes = self.kernel.wait_for_outcomes(decision.run.len(), RUN_TURN_OUTCOME_TIMEOUT);
            if outcomes.len() < decision.run.len() {
                return Err(RunError::Host("kernel: shard produced no outcome for this turn (timed out)".to_string()));
            }
            for outcome in &outcomes {
                match outcome {
                    semio_framework_plugin_host::shard::ShardOutcome::OriginalCall { actor: reported, returned, continuation, call } => {
                        self.kernel.kernel_mut().receive_original_call(RuntimeActorId(*reported),*returned,*continuation).map_err(RunError::Kernel)?;
                        if let Some(original)=call{
                            if original.input!=returned.input||original.spent!=returned.spent{return Err(RunError::ActorCall(*original));}
                            use semio_framework_plugin_host::original_actor_context::OriginalActorFixedKind;
                            if *reported==actor.0&&matches!(original.kind,OriginalActorFixedKind::Failure{..}|OriginalActorFixedKind::Admission{refusal:Some(_),..}|OriginalActorFixedKind::Close{refusal:Some(_),..}){return Err(RunError::ActorCall(*original));}
                        }
                    }
                    semio_framework_plugin_host::shard::ShardOutcome::Turn { actor: reported, result, .. } => {
                        self.kernel.kernel_mut().complete(RuntimeActorId(*reported), result, self.now_ms).await.map_err(|error| RunError::Host(format!("kernel completion failed: {error:?}")))?;
                        if *reported == actor.0 {
                            let status = match &result.status {
                                semio_framework_actor::TurnStatus::Idle => semio_framework::kernel::TurnStatus::Idle,
                                semio_framework_actor::TurnStatus::MoreWork => semio_framework::kernel::TurnStatus::MoreWork,
                                semio_framework_actor::TurnStatus::CheckpointReady { checkpoint } => semio_framework::kernel::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() },
                                semio_framework_actor::TurnStatus::Faulted { detail } => semio_framework::kernel::TurnStatus::Faulted(detail.clone()),
                                status => return Err(RunError::Host(format!("unexpected reactor turn status: {status:?}"))),
                            };
                            turn_result = Some(semio_framework::kernel::TurnResult {
                                ui_patches: semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&result.ui_patches, *reported)
                                    .map_err(|error| RunError::Host(format!("kernel: decode ui patch transport: {error}")))?
                                    .take_owner()
                                    .map_err(|_| RunError::Host("kernel: turn patch transport lease lost its exact owner".to_string()))?,
                                effects: serde_json::from_slice(&result.effects).map_err(RunError::Serde)?,
                                presence: Vec::new(),
                                next_wake: result.next_wake,
                                status,
                                fuel_used: result.usage.fuel,
                                command_ingress: serde_json::from_slice(&result.command_ingress).map_err(RunError::Serde)?,
                                cold_pair_ingress: result.cold_pair_ingress.clone(),
                                lifecycle_receipt: result.lifecycle_receipt,
                                ui_patch_receipt: result.ui_patch_receipt,
                            });
                        } else {
                            let _ = semio_framework::kernel::close_ui_turn_patch_transport_session_one(*reported);
                            let _ = semio_framework::kernel::close_ui_turn_patch_transport_one();
                        }
                    }
                    semio_framework_plugin_host::shard::ShardOutcome::Fault { actor: reported, message } => {
                        let faulted = semio_framework::kernel::TurnResult {
                            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
                            effects: Vec::new(),
                            presence: Vec::new(),
                            next_wake: None,
                            status: semio_framework::kernel::TurnStatus::Faulted(message.clone().into_bytes()),
                            fuel_used: 0,
                            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
                            cold_pair_ingress: semio_framework::kernel::ColdPairIngressStatus::Idle,
                            lifecycle_receipt: None,
                            ui_patch_receipt: None,
                        };
                        self.kernel.complete(RuntimeActorId(*reported), faulted, 0, 0, self.now_ms).await.map_err(|error| RunError::Host(format!("kernel fault completion failed: {error:?}")))?;
                        if *reported == actor.0 {
                            fault = Some(message.clone());
                        }
                    }
                    semio_framework_plugin_host::shard::ShardOutcome::Preempted { actor: reported } if *reported == actor.0 => {
                        let seq = self.take_turn_seq();
                        let resume = Envelope { to: actor, from: Origin::Kernel, lane: Lane::Background, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).map_err(RunError::Serde)? } };
                        if !matches!(self.kernel.submit(&resume).await, Backpressure::Accept) {
                            return Err(RunError::Host(format!("kernel: actor {} was preempted and its resume was not Accept-ed", actor.0)));
                        }
                    }
                    // 🚧️ `run`'s own turns never send `Suspend`/`Resume`/`Cancel` payloads — any
                    // other `ShardOutcome` variant reaching here belongs to a DIFFERENT actor `Kernel::
                    // tick` happened to grant in the same call (DRR is global even with one shard),
                    // same "silently ignored, never aborts an otherwise-successful turn" policy the
                    // wgpu target's own `run_turn` documents for the identical reason.
                    _ => {}
                }
            }
        }
        if let Some(message) = fault {
            return Err(RunError::Host(message));
        }
        match turn_result {
            Some(result) => Ok(result),
            None => Err(RunError::Host("kernel: shard produced no outcome for this turn".to_string())),
        }
    }

    /// 📊️ `(plugins loaded so far, distinct route keys)` — surfaced by the dev-boot smoke test so a
    /// zero-plugin/zero-key router (the router silently doing nothing) is visible, not just "boot
    /// didn't crash." `IoRouter::stats` only errors on a poisoned lock — a diagnostic-only stat line
    /// degrades to `(0, 0)` rather than panicking a whole run over it.
    pub async fn io_router_stats(&self) -> (usize, usize) {
        self.io_router.stats().await.unwrap_or((0, 0))
    }

    pub fn plugin_graph(&self) -> &semio_framework_plugin_host::PluginGraph {
        &self.plugin_graph
    }

    pub fn mutation_router(&self) -> &semio_framework_plugin_host::ArtifactMutationRouter {
        &self.mutation_router
    }

    pub fn inference_router(&self) -> &semio_framework_plugin_host::ArtifactInferenceRouter {
        &self.inference_router
    }

    pub fn instance_directory(&self) -> &semio_framework_plugin_host::InstanceDirectory {
        &self.instance_directory
    }

    pub fn app_router(&self) -> &semio_framework_plugin_host::AppRouter {
        &self.app_router
    }

    async fn manifest_for(&mut self, plugin_id: &str) -> Result<&PluginManifest, RunError> {
        let mut loading = Vec::new();
        self.load_runtime_recursive(plugin_id, &mut loading).await?;
        Ok(self.manifests.get(plugin_id).expect("just loaded or already present"))
    }

    /// 🕸️ Scout-2 §3's binding requirement: a dependency is loaded (recursively) before its
    /// dependent, and `PluginGraph::register` validates the whole graph (missing dependency,
    /// version mismatch, cycle — contract §4 rule 5) before this plugin's own routers see it.
    /// `loading` guards against a cycle that only becomes visible once a manifest is actually read
    /// (unlike `PluginGraph`'s own cycle check, which only fires once every member is registered).
    ///
    /// ✅️ R1-native-manifest: reads `plugin_id`'s manifest from `descriptor_path_for_plugin`'s
    /// committed `🛂️.descriptor.semio` (packet E1's `describe_component` emitter output, decoded the
    /// SAME way `🔌️plugin/🖨️describe/📦️packages/🦀️rust/🦀️.rs`'s own `describe_component` decodes
    /// a live `describe()` return: `store::pack_rt::decode_wire_value` then `semio_framework_value::FromValue::from_value`) —
    /// zero wasm instantiations (this struct's own doc). No live-`describe()` fallback yet — that
    /// needs a `GuestRuntime` seam on `🔌️plugin/🖥️host/🦀️.rs`, out of `path_scope` this
    /// packet (lease-requested, see `📓️terra-R1-report.md`) — so a plugin with no committed
    /// descriptor fails loudly here instead of silently instantiating to compensate.
    /// 🎁️ The package assets `plugin_id`'s committed descriptor declares, read as files below the repository root above that
    /// descriptor and checked against their declared size and SHA-256 (`semio_framework_plugin_host::package_assets`), for the
    /// `assets` of its `Event::InstanceOpen`. A plugin that declares none, or whose descriptor sits outside a repository,
    /// delivers none.
    fn package_assets_for(&self, plugin_id: &str) -> Result<Vec<(String, Vec<u8>)>, RunError> {
        let descriptor = self.read_committed_descriptor(plugin_id)?;
        if descriptor.assets.is_empty() {
            return Ok(Vec::new());
        }
        let root = self.descriptor_path_for_plugin.get(plugin_id).and_then(|path| semio_framework_plugin_host::package_assets::repository_root_above(path));
        match root {
            Some(root) => semio_framework_plugin_host::package_assets::package_file_assets(&root, &descriptor.assets).map_err(|message| RunError::Host(format!("plugin `{plugin_id}`: {message}"))),
            None => Ok(Vec::new()),
        }
    }

    fn read_committed_descriptor(&self, plugin_id: &str) -> Result<PackageDescriptor, RunError> {
        let Some(path) = self.descriptor_path_for_plugin.get(plugin_id) else {
            return Err(RunError::Host(format!(
                "plugin `{plugin_id}` has no committed `🛂️.descriptor.semio` registered (regenerate the registry: `bun repo run @semio-tech/plugin-registry:generate`) and no live `describe()` fallback is wired yet (see `WasmtimeNodeHost`'s own doc)"
            )));
        };
        if !path.is_file() {
            return Err(RunError::Host(format!(
                "plugin `{plugin_id}` has no committed descriptor at `{}` yet (packet D0 is still landing per-plugin emission) and no live `describe()` fallback is wired yet (see `WasmtimeNodeHost`'s own doc)",
                path.display()
            )));
        }
        let bytes = std::fs::read(path).map_err(|error| RunError::Io { path: path.clone(), source: error })?;
        let decoded = store::pack_rt::decode_wire_value(&bytes).map_err(|error| RunError::Host(format!("plugin `{plugin_id}`: decoding `{}` as a pack: {error}", path.display())))?;
        semio_framework_value::FromValue::from_value(decoded).map_err(|error| RunError::Host(format!("plugin `{plugin_id}`: decoding `{}` as a PackageDescriptor: {error}", path.display())))
    }

    /// ✅️ R1-native-manifest: the real recursive-load shape the OLD gap's own doc comment spelled
    /// out — compile the component, decode its committed descriptor (`read_committed_descriptor`,
    /// zero instantiations), recurse over `manifest.dependencies` BEFORE this plugin's own routers
    /// see it (contract §4 rule 5's load-order requirement), instantiate ONE `GuestInstance` for
    /// post-turn job dispatch (`PluginInstanceHandle`'s own doc — `io-run`/`io-sniff`/`infer`, never
    /// this turn's own `execute_turn`), register with every router in the SAME order the OLD
    /// `WasmPluginRuntime`-backed version did, then the SAME `owned_surface_gaps` hard-gate.
    async fn load_runtime_recursive(&mut self, plugin_id: &str, loading: &mut Vec<String>) -> Result<(), RunError> {
        if self.manifests.contains_key(plugin_id) {
            return Ok(());
        }
        if loading.contains(&plugin_id.to_string()) {
            loading.push(plugin_id.to_string());
            return Err(RunError::Host(format!("plugin dependency cycle while loading: {}", loading.join(" -> "))));
        }
        loading.push(plugin_id.to_string());
        let path = self.plugin_path_for_plugin.get(plugin_id).cloned().ok_or_else(|| RunError::Host(format!("no compiled program registered for plugin `{plugin_id}`")))?;
        if !self.compiled_for_plugin.contains_key(plugin_id) {
            let bytes = std::fs::read(&path).map_err(|error| RunError::Io { path: path.clone(), source: error })?;
            // 🔏️ The component's OWN 32-byte digest, not `hash_bytes`' 64-char HEX STRING: that
            // string's `into_bytes()` is 64 long, so `try_into::<[u8; 32]>()` always failed and every
            // plugin loaded here took the `[0u8; 32]` fallback. `WasmtimeRuntime::compile` keys its
            // on-disk `.cwasm` cache on this hash, so the placeholder made every plugin in this
            // process collide on ONE cache entry and replay whichever component compiled first
            // (found 2026-09-20 by ticket 26/09/18 slice WR1, which carried the same defect).
            let package = semio_framework_plugin_host::PackageRef { package: PackageId(plugin_id.to_string()), hash: semio_framework_plugin_host::PackageHash(*framework_hash::hash(&bytes).as_bytes()) };
            let compiled = self.guest_runtime.compile(&package, &bytes).await.map_err(|error| RunError::Host(error.to_string()))?;
            self.compiled_for_plugin.insert(plugin_id.to_string(), compiled);
        }

        let descriptor = self.read_committed_descriptor(plugin_id)?;
        let manifest = descriptor.manifest;

        for dependency in &manifest.dependencies {
            Box::pin(self.load_runtime_recursive(&dependency.plugin_id, loading)).await?;
        }

        let compiled = self.compiled_for_plugin.get(plugin_id).cloned().expect("just compiled or already present above");
        // 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the ad hoc
        // `RuntimeActorId::new(0, 0, self.next_actor_ordinal as u32, 0)` counter this used to mint
        // from is GONE — `Kernel::activate` mints the id and pins a shard for real now (this is the
        // literal "🏃️run ... mints its own RuntimeActorIds" bypass this packet's brief names).
        // `kernel.kernel_mut().activate` (NOT the full `kernel.activate` facade below) deliberately:
        // this instance's `GuestInstance` stays HELD BY `PluginInstanceHandle`, which calls
        // `GuestRuntime::execute_turn` on it directly for post-turn job dispatch
        // (`io_router`/`inference_router`) — routing that through `ShardExecutor` ownership (what the
        // full facade does) would take the instance away from `PluginInstanceHandle` entirely, a much
        // bigger change than this packet's own `path_scope` (`🔌️plugin/🖥️host/**`'s `io_router`
        // internals are not owned here) — see `kernel`'s own struct-field doc.
        let plugin_ordinal=self.plugin_ordinal(plugin_id);
        let actor=if let Some(original)=&self.service_openings{if original.plugin_ordinal!=plugin_ordinal||original.package_hash!=compiled.package_hash{return Err(RunError::Host("another original service opening still owns the fixed pending slot".into()))}original.actor}else{
            let kind=ActorKind::PluginApp{plugin:PackageId(plugin_id.to_string()),app_id:"<plugin-runtime>".to_string(),instance_id:0};
            self.kernel.kernel_mut().activate(PackageId(plugin_id.to_string()),plugin_ordinal,kind,Lane::Background,None,ActivationEvent::Manual).await
        };
        if self.service_openings.is_none(){
            while !self.host_driver.admit_root_step().map_err(RunError::Original)?{semio_framework_async::yield_once().await;}
            loop{
                let Some(scope)=self.host_driver.begin_actor_turn(None).map_err(|error|RunError::Original(error.into()))?else{semio_framework_async::yield_once().await;continue};
                self.host_driver.dispatch_actor_step(scope,|cx|{
                    let copy=std::mem::size_of::<ServiceActorOpening>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(())}
                    cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;cx.consume_fuel(1);
                    self.service_openings=Some(ServiceActorOpening{actor,plugin_ordinal,package_hash:compiled.package_hash,opening:None,instance:None,input:None,scope:None,execution:None,return_pending:None,returned:None,admitted:None,fault:None,refusal:None});Ok(())
                }).map_err(RunError::Original)?;
                self.host_driver.finish_actor_step(scope).map_err(RunError::Original)?;
                if self.service_openings.is_some(){break}semio_framework_async::yield_once().await;
            }
        }
        if let Some(refusal)=&self.service_identity_refusal{return Err(RunError::Host(refusal.error.to_string()))}
        if !self.service_identities.contains_key(plugin_id){match(self.direct_identity)(actor){Ok(identity)=>{self.service_identities.insert(plugin_id.to_string(),identity);},Err(refusal)=>{let message=refusal.error.to_string();self.service_identity_refusal=Some(refusal);return Err(RunError::Host(message))}}}
        let(instance,original_input)=self.prepare_service_original(plugin_id,&compiled).await?;
        let handle=Arc::new(semio_framework_plugin_host::PluginInstanceHandle::new(actor,Arc::clone(&self.guest_runtime),instance,original_input,semio_framework_plugin_host::GuestRelayWakeAuthority{drive_policy:self.host_driver_policy.drive,policy:self.host_driver_policy.wake,issuer:semio_framework_job_worker::admit_original_thread_worker_wake,receiver:self.host_driver.clone()}).await);

        let artifact_dialect_entries: Vec<_> = descriptor.contributions.composer_entries.iter().map(|entry| (entry.writes.clone(), entry.reads.clone())).collect();
        // 🕳️ `ContributionSet.io_entries` (owner/counterpart/direction) carries no `fidelity`/
        // `sniffs` — `io_schema::IoEntryDescriptor`'s fields (a DIFFERENT type of the same name in
        // `🚪️io/🧬️schema/🦀️component.rs`) the descriptor schema does not emit yet. `&[]` here is
        // honest, not a shortcut: fabricating a fidelity/sniffs value would be a real correctness
        // lie, and `note`'s own committed descriptor has zero `io_entries` either way, so this gap
        // is invisible on today's only fully-wired smoke path. A future packet must either widen
        // `ContributionSet.io_entries` with those two fields at emission time (E1/A2) or resolve
        // them from `IoFidelityDeclaration` elsewhere before this can be more than `&[]`.
        self.io_router.register_plugin(plugin_id, Arc::clone(&handle), &artifact_dialect_entries, &[]).await.map_err(|error| RunError::Host(error.to_string()))?;

        let mut mutation_roster: Vec<semio_framework_plugin_host::HostMutationRosterEntry> = descriptor
            .contributions
            .mutation_services
            .iter()
            .map(|service| semio_framework_plugin_host::HostMutationRosterEntry {
                mutation_id: service.mutation_id.clone(),
                verb: service.semantics.verb.clone(),
                entity: service.semantics.entity.clone(),
                kind: service.semantics.kind.clone(),
                record: service.semantics.record.clone(),
                contributor: None,
                artifact_kind: None,
                inputs: semio_framework_plugin_host::HostMutationInputs::Opaque,
            })
            .collect();
        for contribution in &descriptor.contributions.artifact_contributions {
            for mutation in &contribution.mutations {
                mutation_roster.push(semio_framework_plugin_host::HostMutationRosterEntry {
                    mutation_id: mutation.mutation_id.clone(),
                    verb: mutation.semantics.verb.clone(),
                    entity: mutation.semantics.entity.clone(),
                    kind: mutation.semantics.kind.clone(),
                    record: mutation.semantics.record.clone(),
                    contributor: Some(plugin_id.to_string()),
                    artifact_kind: Some(contribution.artifact_kind.clone()),
                    inputs: semio_framework_plugin_host::HostMutationInputs::Opaque,
                });
            }
        }
        self.mutation_router.register_roster_with_runtime(plugin_id, &manifest.dependencies, Arc::clone(&handle), mutation_roster).await.map_err(|error| RunError::Host(error.to_string()))?;

        let inference_roster: Vec<serde_json::Value> = descriptor
            .contributions
            .inference_services
            .iter()
            .chain(descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter()))
            .map(serde_json::to_value)
            .collect::<Result<Vec<_>, _>>()
            .map_err(RunError::Serde)?;
        let inference_wire_bytes = serde_json::to_vec(&inference_roster).map_err(RunError::Serde)?;
        self.inference_router.register_plugin(plugin_id, &manifest.dependencies, Arc::clone(&handle), &inference_wire_bytes).await.map_err(|error| RunError::Host(error.to_string()))?;

        self.plugin_graph.register(manifest.clone()).await.map_err(|error| RunError::Host(error.to_string()))?;
        self.app_router.register_manifest(plugin_id, &manifest).await.map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?;

        let gaps = self.app_router.owned_surface_gaps().await;
        if !gaps.is_empty() {
            return Err(RunError::Host(format!("plugin `{plugin_id}` loaded but left {} owned-surface gap(s): {}", gaps.len(), gaps.iter().map(|fault| format!("{}: {}", fault.code.0, fault.message)).collect::<Vec<_>>().join("; "))));
        }

        self.manifests.insert(plugin_id.to_string(), manifest);
        Ok(())
    }

    /// ✂️ Contract §4.5: refused while any OTHER loaded plugin still depends on `plugin_id`.
    pub async fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), RunError> {
        self.plugin_graph.guard_unload(plugin_id).await.map_err(|error| RunError::Host(error.to_string()))?;
        self.io_router.unregister_plugin(plugin_id).await.map_err(|error| RunError::Host(error.to_string()))?;
        self.mutation_router.unregister_plugin(plugin_id).await.map_err(|error| RunError::Host(error.to_string()))?;
        self.inference_router.unregister_plugin(plugin_id).await.map_err(|error| RunError::Host(error.to_string()))?;
        self.app_router.unregister_plugin(plugin_id).await;
        self.plugin_graph.unregister(plugin_id).await.map_err(|error| RunError::Host(error.to_string()))?;
        self.manifests.remove(plugin_id);
        self.compiled_for_plugin.remove(plugin_id);
        Ok(())
    }

    /// 🩹️ Contract §4.5: re-validates the WHOLE graph with `plugin_id` replaced (so a version bump
    /// that would break a live dependent is rejected before anything swaps), then atomically replaces
    /// the registered manifest and every router entry for it.
    ///
    /// 🚧️ Same gap as `load_runtime_recursive` — always fails until a manifest decoder exists.
    pub async fn hot_reload_plugin(&mut self, plugin_id: &str) -> Result<(), RunError> {
        self.manifests.remove(plugin_id);
        self.compiled_for_plugin.remove(plugin_id);
        self.manifest_for(plugin_id).await.map(|_| ())
    }

    /// 🎯️ Contract §5 end to end: `initiator_handle` must already have proposed (its own
    /// `dispatch_emit` stashed a `TransactionProposalDraft` instead of applying — the caller drains
    /// it via `exchange` and passes `local_ops`/`foreign` straight through here).
    ///
    /// 🚧️ `exec`/`plan` always return `TransactionError` today — this struct's own doc comment
    /// explains why (a real per-command reply correlation / contributed-mutation-plan dispatch over
    /// `GuestRuntime::execute_turn`'s effects needs a post-turn dispatch loop this packet does not
    /// build). Kept as a real, correctly-typed call into `HostTransactionCoordinator::run_transaction`
    /// (unlike deleting the method) so `TransactionCoordinator`'s own resolution/gating logic — which
    /// IS real and IS tested (`host_transaction_coordinator_tests`) — stays reachable from here the
    /// moment the two closures below get real bodies.
    pub async fn run_transaction(&self, initiator_handle: u32, local_ops: Vec<Vec<u8>>, foreign: Vec<protocol::ForeignStep>) -> Result<semio_framework_plugin_host::TransactionOutcome, semio_framework_plugin_host::TransactionError> {
        let (plugin_id, instance_id) = self.instances.get(&initiator_handle).cloned().ok_or_else(|| semio_framework_plugin_host::TransactionError::Rejected { code: "transaction.unknown-target".into(), message: format!("unknown node handle {initiator_handle}") })?;
        let initiator = semio_framework_plugin_host::TransactionMember { plugin_id, instance_id };
        self.transaction_coordinator.run_transaction(
            &self.instance_directory,
            &self.mutation_router,
            |plugin_id, _instance_id, _command| {
                Err(semio_framework_plugin_host::TransactionError::Rejected {
                    code: "transaction.not-wired".into(),
                    message: format!("plugin `{plugin_id}`: transaction exchange requires post-turn effect dispatch"),
                })
            },
            |contributor, _artifact_kind, _mutation_id, _member, _payload| {
                Err(semio_framework_plugin_host::TransactionError::Rejected { code: "transaction.not-wired".into(), message: format!("contributor `{contributor}`: artifact mutation planning requires post-turn effect dispatch") })
            },
            initiator,
            local_ops,
            foreign,
        ).await
    }

    /// 🚧️ Same gap as `run_transaction` — see its own doc comment.
    pub async fn undo_transaction_group(&self, members: &[semio_framework_plugin_host::TransactionMember], group_id: &str) {
        self.transaction_coordinator.undo_group(
            |plugin_id, _instance_id, _command| {
                Err(semio_framework_plugin_host::TransactionError::Rejected { code: "transaction.not-wired".into(), message: format!("plugin `{plugin_id}`: transaction exchange requires post-turn effect dispatch") })
            },
            members,
            group_id,
        ).await
    }

    //#region 🔖️OpeningCommands
    /// 🚪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (W1-A), contract §3: the host-level
    /// implementation of `os.open-artifact`. Empty `plugin_id`/`app_id` means "ask `OpeningResolver`";
    /// both set means the caller already knows which surface it wants (validated, echoed back
    /// unchanged — the SDK-side `relay_open_artifact` in `semio-framework-plugin`'s
    /// `OpeningCommandRelay` region runs the SAME shape of validation before the shell relay; this is
    /// the host's own independent check, reusing its exact `opening.*` fault-code vocabulary so a
    /// caller sees one consistent error grammar on either side of the wire). Returns
    /// `semio_framework::Fault` directly (not `RunError`) so the frozen fault codes (`surface.*` from
    /// `OpeningResolver`, `opening.*` here) reach the caller verbatim instead of being flattened into
    /// an opaque string — `exchange` below re-encodes it onto the wire unchanged. This is a HOST-LEVEL
    /// operation (which plugin/app to instantiate) rather than an already-open-instance operation, so
    /// — like `run_transaction` above — it is a direct method rather than only reachable through
    /// `exchange`'s per-`node` interface; `exchange` also intercepts the wire `AppCommand::OpenArtifact`
    /// and delegates here.
    pub async fn resolve_open_artifact(&self, artifact_ref: &str, role_wire: u8, plugin_id: &str, app_id: &str) -> Result<semio_framework::AppRef, semio_framework::Fault> {
        let role = opening_role_from_wire(role_wire)?;
        let (dialect, artifact_role) = semio_framework::parse_surface_app_id(artifact_ref).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("opening.invalid-artifact-ref"), error))?;
        if role != artifact_role {
            return Err(semio_framework::Fault::new(
                semio_framework::FaultOrigin::Os,
                semio_framework::FaultCode::new("opening.role-mismatch"),
                format!("artifact ref {artifact_ref} declares {} but the command declares {}", artifact_role.as_str(), role.as_str()),
            ));
        }
        match (plugin_id.is_empty(), app_id.is_empty()) {
            (true, true) => {
                let user_default = self.opening_preferences.defaults.iter().find(|entry| entry.dialect == dialect && entry.role == role).map(|entry| &entry.app);
                semio_framework_plugin_host::OpeningResolver::resolve(&self.app_router, &dialect, role, user_default).await
            }
            (false, false) => Ok(semio_framework::AppRef { plugin_id: plugin_id.to_string(), app_id: app_id.to_string() }),
            _ => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("opening.partial-app-ref"), "plugin_id and app_id must either both be empty or both be set")),
        }
    }

    /// ✏️ Contract §3's `os.set-default-viewer`/`os.set-default-editor`: pins `app` as the default
    /// `role` surface for `(artifact_kind, standard, subset)` — refuses to pin a surface the
    /// `AppRouter` does not actually know about (a stale/typo'd `AppRef` would otherwise silently
    /// pin to nothing, since `OpeningResolver` already falls through past any default not present in
    /// the router; refusing up front is a better failure than a default that quietly never applies).
    /// Applies through the SAME event-sourced `OpeningConfigMutation`/`apply_opening_config_mutation`
    /// the schema facet's own `MutationDiff` impl defines (contract §4: "never a mutable map") — never
    /// a direct field write onto `self.opening_preferences`.
    pub async fn set_default_app(&mut self, artifact_kind: &str, standard: &str, subset: &str, role_wire: u8, plugin_id: &str, app_id: &str) -> Result<(), semio_framework::Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let role = opening_role_from_wire(role_wire)?;
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: artifact_kind.to_string(), standard: standard.to_string(), subset: subset.to_string() };
        let app = semio_framework::AppRef { plugin_id: plugin_id.to_string(), app_id: app_id.to_string() };
        if !self.app_router.surfaces_for(&dialect, role).await.contains(&app) {
            return Err(semio_framework::Fault::new(
                semio_framework::FaultOrigin::Os,
                semio_framework::FaultCode::new("opening.invalid-app-ref"),
                format!("`{}` is not a registered {} surface for `{}`", app.app_id, role.as_str(), dialect.to_coordinate()),
            ));
        }
        let mutation = semio_framework_os_config::opening_config::mutations::set_default_app::set_default_app(dialect, role, app);
        semio_framework_os_config::opening_config::apply_opening_config_mutation(&mut self.opening_preferences, &mutation).map_err(|error| semio_framework::Fault::from(error.to_string()))?;
        Ok(())
    }

    /// 🧹 Contract §3's `os.clear-default-app`: unpins whatever default is set for
    /// `(artifact_kind, standard, subset, role)`, if any — infallible past role validation (clearing
    /// an already-absent default is a no-op, matching `ClearDefaultApp`'s own diff), same
    /// event-sourced apply path as `set_default_app`.
    pub fn clear_default_app(&mut self, artifact_kind: &str, standard: &str, subset: &str, role_wire: u8) -> Result<(), semio_framework::Fault> {
        let role = opening_role_from_wire(role_wire)?;
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: artifact_kind.to_string(), standard: standard.to_string(), subset: subset.to_string() };
        let mutation = semio_framework_os_config::opening_config::mutations::clear_default_app::clear_default_app(dialect, role);
        semio_framework_os_config::opening_config::apply_opening_config_mutation(&mut self.opening_preferences, &mutation).map_err(|error| semio_framework::Fault::from(error.to_string()))?;
        Ok(())
    }
    //#endregion 🔖️OpeningCommands
}

/// 🔤️ Wire `role: u8` -> `AppRole` (`0` Viewer, `1` Editor, contract §3) — the host-side twin of the
/// SDK guest's `opening_role` in `semio-framework-plugin`'s `OpeningCommandRelay` region; same fault
/// code (`opening.invalid-role`) on both sides of the wire.
fn opening_role_from_wire(role_wire: u8) -> Result<semio_framework::AppRole, semio_framework::Fault> {
    match role_wire {
        0 => Ok(semio_framework::AppRole::Viewer),
        1 => Ok(semio_framework::AppRole::Editor),
        other => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("opening.invalid-role"), format!("opening role {other} must be 0 (viewer) or 1 (editor)"))),
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<B: BlobStore + 'static> AppChannelHost for WasmtimeNodeHost<B> {
    /// ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the real body —
    /// `manifest_for` (real for any plugin with a committed descriptor since R1-native-manifest;
    /// this method's own doc used to say "manifest_for always errors today", which stopped being
    /// true once that packet landed), `kernel.activate` (mints the `ActorId`, pins a shard,
    /// instantiates, hands the `GuestInstance` to that shard's own `ShardExecutor` thread), then
    /// `run_turn` submits `Event::InstanceOpen` through the kernel (channel v12's own doc:
    /// "lifecycle now arrives through ... `Event::InstanceOpen`") and drives it to completion exactly
    /// like a real per-node turn — this is the SAME `run_turn` `exchange` uses below, not a separate
    /// path. `artifact_ref` binding into `instance_directory` (this struct's own field doc) is a
    /// SEPARATE, still-unwired concern this packet does not touch (out of scope: it needs the
    /// manifest's own app entry's `io.artifact_schema`, not anything kernel-shaped).
    async fn open(&mut self, plugin_id: &str, app_id: &str, _artifact_ref: &str) -> Result<u32, RunError> {
        self.manifest_for(plugin_id).await?;
        let compiled = self.compiled_for_plugin.get(plugin_id).cloned().expect("manifest_for just compiled or already present");
        let plugin_ordinal = self.plugin_ordinal(plugin_id);
        let instance_handle = self.next_handle;
        self.next_handle += 1;
        let kind = ActorKind::PluginApp { plugin: PackageId(plugin_id.to_string()), app_id: app_id.to_string(), instance_id: instance_handle };
        let actor = self.kernel.activate(PackageId(plugin_id.to_string()), plugin_ordinal, kind, Lane::Background, None, ActivationEvent::Manual, &compiled, &[], &NODE_TURN_BUDGET).await.map_err(activation_refusal_error)?;
        self.instances.insert(instance_handle, (plugin_id.to_string(), instance_handle));
        self.instance_actors.insert(instance_handle, actor);
        let assets = self.package_assets_for(plugin_id)?;
        let open_event = Event::InstanceOpen {
            request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: instance_handle, request_sequence: 1 },
            app_id: AppInstanceId(app_id.to_string()),
            actor: "local".to_string(),
            config: Vec::new(),
            assets,
            capabilities: Vec::new(),
            quotas: QuotaSchema::default(),
        };
        let mut first_turn = Vec::with_capacity(2);
        first_turn.extend(semio_framework_plugin_host::activation::activation_turn_event(app_id));
        first_turn.push(open_event);
        let _ = self.run_turn(actor, first_turn).await?;
        Ok(instance_handle)
    }

    /// 🚪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (W1-A) contract §3: intercepts the
    /// three OS-level `AppCommand`s (`OpenArtifact`/`SetDefaultApp`/`ClearDefaultApp`) BEFORE the
    /// generic per-instance forward below — they are host-level (which plugin/app to resolve, or how
    /// to fold `OpeningPreferences`), never something a guest's `VcsArtifactApp` could handle, and a
    /// guest given one of these bytes would just fail to decode it as any of ITS OWN typed commands.
    /// `node` still selects which loaded runtime the REMAINING (non-opening) commands in the same
    /// batch forward to — resolved once opening frames are peeled off, so a batch of pure opening
    /// commands never needs a live instance at all. Simplification, documented: opening-command
    /// frames are appended BEFORE any passthrough guest frames rather than interleaved at their
    /// original position — fine while a real caller sends either an opening batch or a document batch,
    /// never both mixed in one call (today's only caller, the SDK's `OpeningCommandRelay`, never mixes
    /// them).
    async fn exchange(&mut self, _ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
        let mut frames = Vec::new();
        let mut passthrough = Vec::new();
        for command in commands {
            match command {
                AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } => {
                    frames.push(match self.resolve_open_artifact(&artifact_ref, role, &plugin_id, &app_id).await {
                        Ok(_resolved) => AppFrame::Done { in_reply_to: seq },
                        Err(fault) => AppFrame::Error { in_reply_to: Some(seq), fault: semio_framework_diagnostic::encode_fault_bytes(&fault), report: Vec::new() },
                    });
                }
                AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } => {
                    frames.push(match self.set_default_app(&artifact_kind, &standard, &subset, role, &plugin_id, &app_id).await {
                        Ok(()) => AppFrame::Done { in_reply_to: seq },
                        Err(fault) => AppFrame::Error { in_reply_to: Some(seq), fault: semio_framework_diagnostic::encode_fault_bytes(&fault), report: Vec::new() },
                    });
                }
                AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } => {
                    frames.push(match self.clear_default_app(&artifact_kind, &standard, &subset, role) {
                        Ok(()) => AppFrame::Done { in_reply_to: seq },
                        Err(fault) => AppFrame::Error { in_reply_to: Some(seq), fault: semio_framework_diagnostic::encode_fault_bytes(&fault), report: Vec::new() },
                    });
                }
                other => passthrough.push(other),
            }
        }
        if !passthrough.is_empty() {
            if !self.rejected_command_builds.terminal_is_empty() {
                let (complete, _, _) = self.rejected_command_builds.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                return Err(RunError::Host(if complete { "rejected command-build owner reached terminal empty; retry the exchange".to_string() } else { "rejected command-build owner closed one exact page; retry the exchange".to_string() }));
            }
            if !self.retained_command_closes.terminal_is_empty() {
                let (complete, _, _) = self.retained_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                return Err(RunError::Host(if complete {
                    "previous cancelled command owner reached terminal empty; retry the exchange".to_string()
                } else {
                    "previous cancelled command owner is closing one exact page; retry the exchange".to_string()
                }));
            }
            if !persistent_command_completion_port_ready() {
                return Err(RunError::Host("persistent command completion submit/poll/cancel authority is not admitted".to_string()));
            }
            let operation = self.take_turn_seq();
            if !self.retained_command_closes.can_insert(operation) || !self.rejected_command_builds.can_insert(operation) {
                return Err(RunError::Host("retained command close registry is saturated or collided".to_string()));
            }
            // ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the real body
            // this comment used to describe as future work — `node` now DOES resolve, since `open`
            // (above) genuinely activates an instance through the kernel. Each passthrough
            // `AppCommand` becomes a retained fixed-page owner. One page is lowered per reactor turn;
            // the next command remains untouched until the guest acknowledges the current terminal.
            // `TurnResult.effects` is then scanned for `Effect::Respond{req, result}`,
            // decoding `RequestOutcome::Ok(bytes)` as the guest's own `protocol::AppFrame` reply and
            // `Err(bytes)` as `AppFrame::Error` — exactly the shape this comment always described.
            let actor = *self.instance_actors.get(&node).ok_or_else(|| RunError::Host(format!("unknown node handle {node}")))?;
            let mut envelopes = semio_framework::kernel::CommandEnvelopeSet::try_new().map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?;
            for command in &passthrough {
                let seq = app_command_seq(command);
                let command = protocol::encode_app_command(command).await.map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?;
                if let Err((fault, rejected)) = envelopes.try_push(semio_framework::kernel::CommandEnvelope { instance: node, seq, command }) {
                    self.rejected_command_builds.insert_admitted(operation, semio_framework::kernel::RejectedCommandBuild::new(envelopes, rejected));
                    return Err(RunError::Host(format!("{}: {}", fault.code.0, fault.message)));
                }
            }
            let generation = self.take_turn_seq();
            let batch = match semio_framework::kernel::CommandBatch::try_new(generation, envelopes) {
                Ok(batch) => batch,
                Err((fault, owners)) => {
                    self.rejected_command_builds.insert_admitted(operation, semio_framework::kernel::RejectedCommandBuild::from_admitted(owners));
                    return Err(RunError::Host(format!("{}: {}", fault.code.0, fault.message)));
                }
            };
            self.retained_command_closes.insert_admitted(operation, generation, semio_framework::kernel::CommandBatchDriver::new(operation, batch));
            let mut effects = Vec::new();
            loop {
                let events = match self.retained_command_closes.with_driver_mut(operation, generation, |driver| driver.next_page()).map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?.map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))? {
                    Some((cursor, bytes)) => vec![Event::CommandIngressPage { cursor, bytes }],
                    None => vec![Event::Wake],
                };
                self.retained_command_closes.prepare_suspend(operation, generation).map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?;
                let turn = self.run_turn(actor, events).await?;
                self.retained_command_closes.resume(operation, generation).map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?;
                let progress = self
                    .retained_command_closes
                    .with_driver_mut(operation, generation, |driver| driver.observe(&turn.command_ingress, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES))
                    .map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?
                    .map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?;
                effects.extend(turn.effects);
                match progress {
                    semio_framework::kernel::CommandBatchProgress::Complete => {
                        self.retained_command_closes.remove_terminal(operation, generation).map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?;
                        break;
                    }
                    semio_framework::kernel::CommandBatchProgress::Faulted => {
                        self.retained_command_closes.begin_close(operation, generation).map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?;
                        let (complete, _, _) = self.retained_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                        return Err(RunError::Host(if complete { "command ingress faulted after terminal exact-owner cleanup".to_string() } else { "command ingress faulted; retained owner closed one exact page and awaits retry".to_string() }));
                    }
                    semio_framework::kernel::CommandBatchProgress::PageReady | semio_framework::kernel::CommandBatchProgress::Waiting => {}
                }
            }
            for effect in effects {
                if let Effect::Respond { req, result } = effect {
                    match result {
                        RequestOutcome::Ok(bytes) => match protocol::decode_app_frame(&bytes).await {
                            Ok(frame) => frames.push(frame),
                            Err(error) => frames.push(AppFrame::Error { in_reply_to: Some(req.0), fault: semio_framework_diagnostic::encode_fault_bytes(&semio_framework::Fault::from(error.to_string())), report: Vec::new() }),
                        },
                        RequestOutcome::Err(bytes) => frames.push(AppFrame::Error { in_reply_to: Some(req.0), fault: bytes, report: Vec::new() }),
                    }
                }
            }
        }
        Ok(frames)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn persistent_command_completion_port_ready() -> bool {
    false
}

/// 🔢️ Every `AppCommand` variant's leading `seq: u64` field, used verbatim as the kernel envelope's
/// own command-ingress sequence (see `exchange`'s own passthrough doc for why reusing it — rather
/// than minting a fresh kernel-side sequence and maintaining a translation table back to it — is
/// sound: one `exchange` batch's commands already carry distinct seqs, minted by `SpaceRunner::
/// compute_node`'s own local `next_seq` closure, and `Effect::Respond{req}` correlates directly
/// against whichever seq the retained command envelope carried). An exhaustive match, deliberately —
/// a future `AppCommand` variant missing here is a compile error, not a silent seq-of-zero.
#[cfg(not(target_arch = "wasm32"))]
fn app_command_seq(command: &AppCommand) -> u64 {
    match command {
        AppCommand::ConfigCommand { seq, .. }
        | AppCommand::Command { seq, .. }
        | AppCommand::CommandText { seq, .. }
        | AppCommand::ContextMenu { seq, .. }
        | AppCommand::LocalInteractionQuery { seq, .. }
        | AppCommand::ArtifactCommand { seq, .. }
        | AppCommand::ApplyEnvelopes { seq, .. }
        | AppCommand::ReadDocument { seq }
        | AppCommand::LoadDocumentArchive { seq, .. }
        | AppCommand::ReadDocumentArchive { seq }
        | AppCommand::PollDocumentArchiveLoad { seq, .. }
        | AppCommand::CancelDocumentArchiveLoad { seq, .. }
        | AppCommand::AcknowledgeDocumentArchiveLoad { seq, .. }
        | AppCommand::LoadConfig { seq, .. }
        | AppCommand::ReadConfig { seq }
        | AppCommand::LoadWindowConfig { seq, .. }
        | AppCommand::ReadWindowConfigs { seq }
        | AppCommand::MediaIn { seq, .. }
        | AppCommand::MediaOut { seq, .. }
        | AppCommand::MediaFingerprint { seq, .. }
        | AppCommand::PureCommand { seq, .. }
        | AppCommand::LoadChildren { seq, .. }
        | AppCommand::ReadChildren { seq }
        | AppCommand::ReadHistory { seq }
        | AppCommand::TransactionPrepare { seq, .. }
        | AppCommand::TransactionCommit { seq, .. }
        | AppCommand::TransactionRollback { seq, .. }
        | AppCommand::TransactionUndo { seq, .. }
        | AppCommand::TransactionRedo { seq, .. }
        | AppCommand::OpenArtifact { seq, .. }
        | AppCommand::SetDefaultApp { seq, .. }
        | AppCommand::ClearDefaultApp { seq, .. }
        | AppCommand::SetMergePolicy { seq, .. }
        | AppCommand::ResolveConflict { seq, .. }
        | AppCommand::ReadConflicts { seq }
        | AppCommand::Presence { seq, .. }
        | AppCommand::SubmitMediaExport { seq, .. }
        | AppCommand::PollMediaExport { seq, .. }
        | AppCommand::CancelMediaExport { seq, .. }
        | AppCommand::ReadDocumentIdentity { seq }
        | AppCommand::ReadChildHeads { seq }
        | AppCommand::MergeDocumentArchive { seq, .. }
        | AppCommand::TakeMediaExportChunk { seq, .. } => *seq,
    }
}
//#endregion 🔖️WasmtimeNodeHost
