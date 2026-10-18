//! 🏠️ Wasmtime host state and wit hosts: the actor host state, the wasi context, the store fuel and the trap classification and every wit host interface the actor state implements. See [`super`].

use super::*;

//#region 🎭️GuestRuntime
//#region 🐎️WasmtimeRuntime
// 🧬️ `reactor`/`jobs` are `export`s of `world actor` (design-runtime.md §2's `execute_turn`/`step_job`
// exports), not `import`s, so their generated bindings live under `exports::` — unlike `pure`'s
// argument/return types (`capabilities`/`effects`/`events`/`types`/`ui`), which sit at the top level.

/// 🧬️ `design-runtime.md` §2's slimmed `HostState { plugin_id, actor, caps, effect_sink,
/// asset_map }` — `limiter` is an implementation necessity (`Store::limiter` needs somewhere to
/// read bounds from), not part of the design's literal 5-field list.
pub(crate) struct ActorHostState {
    pub(super) original_job_clock:Option<(u64,fn()->Option<u64>)>,
    pub(super) operation_slot:semio_framework_plugin_host_operation::OperationSlot,
    pub(super) plugin_id: String,
    #[allow(dead_code)]
    pub(super) actor: RuntimeActorId,
    #[allow(dead_code)]
    pub(super) caps: Vec<BrokerCapabilityGrant>,
    /// 🚪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): where `host-async.emit`'s
    /// fire-and-forget effects land mid-turn. Held as the WIT variant, NOT the kernel `Effect`:
    /// `emit` is declared sync in the WIT (a deliberate one-way door), `wit_effect_to_kernel` is
    /// genuinely `async`, and pushing the raw variant lets `execute_turn` do the conversion where
    /// it can actually `.await` — no `block_on` bridge on a wasm host path (R4 forbids one there).
    pub(super) emit_sink: Vec<wit_effects::Effect>,
    /// 🚪️ The `emit-patch` counterpart. Drained by `execute_turn` alongside `emit_sink`.
    pub(super) emit_patch_sink: Vec<wit_ui::UiPatch>,
    #[allow(dead_code)]
    pub(super) asset_map: HashMap<String, Vec<u8>>,
    pub(super) limiter: BudgetLimiter,
    pub(super) wasi_ctx: WasiCtx,
    /// 🩺️ The guest's own stderr, retained in memory, when diagnostics are armed — see
    /// [`guest_wasi_ctx`] and [`GuestInstance::guest_diagnostics_text`].
    pub(super) diagnostics: Option<wasmtime_wasi::p2::pipe::MemoryOutputPipe>,
    pub(super) resource_table: ResourceTable,
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
pub(super) fn guest_wasi_ctx() -> (WasiCtx, Option<wasmtime_wasi::p2::pipe::MemoryOutputPipe>) {
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
pub const GUEST_JOB_WATCHDOG_MS: u64 = 30_000;

/// ⛽️ Grants `fuel` on a metered engine; an unmetered engine has no fuel to grant and bounds the
/// call by its epoch deadline alone — see [`SharedEngineConfig::fuel_metering`].
pub(super) fn arm_store_fuel(store: &mut Store<ActorHostState>, fuel_metering: bool, fuel: u64) -> wasmtime::Result<()> {
    if fuel_metering {
        store.set_fuel(fuel)?;
    }
    Ok(())
}

pub fn classify_guest_trap(trap: &impl std::fmt::Debug) -> TurnFault {
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
impl semio_framework_plugin_host_wit::actor_bindings::semio::framework::pure::Host for ActorHostState {
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

impl semio_framework_plugin_host_operation::OperationHostStore for ActorHostState{
 fn operation_slot(&mut self)->&mut semio_framework_plugin_host_operation::OperationSlot{&mut self.operation_slot}
 fn operation_log(&mut self,level:String,message:String){semio_framework_plugin_host_wit::actor_bindings::semio::framework::pure::Host::log(self,level,message)}
 fn operation_clock_us(&mut self,clock_binding:u64)->Option<u64>{semio_framework_plugin_host_wit::actor_bindings::semio::framework::pure::Host::operation_clock_us(self,clock_binding)}
 fn operation_now_ms(&mut self)->i64{semio_framework_plugin_host_wit::actor_bindings::semio::framework::pure::Host::now_ms(self)}
 fn operation_trace_span(&mut self,name:String){semio_framework_plugin_host_wit::actor_bindings::semio::framework::pure::Host::trace_span(self,name)}
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
//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime
