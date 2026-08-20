//! 🧊️ Raw wgpu WASM renderer for declarative framework UiNode trees.
//!
//! 🧭️ Rough correspondence with the React shell (`framework/renderer/react/os-shell.tsx`), as a
//! discoverability breadcrumb rather than a rigorous mapping:
//! - this crate's top-level shell/state struct ~ React's `#region 🔖️types` + `FrameworkOsShell`.
//! - the `dock` module below (window tree, stack chrome, split resize) ~ React's `Mode`
//!   component and the `WindowLayoutNode` tree helpers in `#region ShellHelpers`.
//! - `interpreter`/widget rendering ~ React's `UiNode` component tree rendering.

extern crate semio_framework_os_kernel as store_sync;
extern crate semio_framework_os_kernel as dsl_core;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate framework_surface_node_graph as framework_surface_tiled_map;
extern crate infinite_canvas as infinite_world;
#[macro_export]
macro_rules! action_args_json {
    ($($tt:tt)*) => {
        semio_framework::optional_json_to_dsl(Some(serde_json::json!($($tt)*)))
    };
}

#[path = "../../../../🧱️elements/Dock/🧊️component.rs"]
pub mod dock;

#[path = "../../../../🧱️elements/EngineCanvas/🧊️component.rs"]
pub mod engine_canvas;

#[path = "../../../../🧱️elements/Interpreter/🧊️component.rs"]
pub mod interpreter;

#[path = "../../../../🧱️elements/ProgramBridge/🧊️component.rs"]
pub mod program_bridge;

//#region 🏠️🧳️PluginHostConfig
// 🐛️ Lives at the crate root, not inside `program_bridge` above (see that module's own `PluginHostConfig`
// region for why) — this is the file's real directory, so the 3-`..` climb to
// `framework/plugin/registry/generated/🦀️hosts.rs` actually resolves.
#[path = "../../../../../../🔌️plugin/📇️registry/🤖️generated/🦀️hosts.rs"]
mod generated_plugin_hosts;
//#endregion 🏠️🧳️PluginHostConfig

#[path = "../../../../🧱️elements/Scenes/🧊️component.rs"]
pub mod scenes;

#[path = "../../../../🧱️elements/Shell/🧊️component.rs"]
pub mod shell;

#[path = "../../../../🧱️elements/IconRenderHost/🧊️component.rs"]
pub mod icon_atlas;

//#region 🔖️OsHostDecomposition
// 🏠️ ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host): the seam that ends
// this crate owning the actor kernel and ends its continuous redraw. `deadlines`/`kernel_seam` are
// leaves (no dependency on `os_host`/`winit_app`); `os_host` composes `AppRuntime` with them;
// `winit_app` is the new `ApplicationHandler` — see that file's own module docstring for why it
// hand-rolls the event loop instead of using `ui_host::window::NativeHost<D>` directly. Mounted here,
// away from the peer program's `parallel_runtime` mount just below, per this ticket's own OWNS list.
#[path = "🦀️deadlines.rs"]
mod deadlines;

#[path = "🦀️kernel_seam.rs"]
mod kernel_seam;

#[path = "🦀️os_host.rs"]
mod os_host;

#[path = "🦀️winit_app.rs"]
mod winit_app;
//#endregion 🔖️OsHostDecomposition

// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop): the real multi-shard `Kernel`
// loop — `ParallelRuntime` — used by both `kernel_runtime` (below) and `scale_bench`. Native-only,
// same reason `kernel_runtime`/`scale_bench` themselves are: real OS threads (`ShardExecutor`s +
// their outcome forwarders), not available on wasm32.
#[cfg(not(target_arch = "wasm32"))]
#[path = "🎠️runtime.rs"]
pub mod parallel_runtime;

use infinite_world::{
    apply_glb_bytes, apply_world_action_preview, collect_pending_glb_fetches, fetch_url_bytes, handle_world3d_paint_actions, handle_world3d_pointer_button, handle_world3d_pointer_drag, handle_world3d_pointer_move, handle_world3d_wheel,
    orbit_camera_action,
};
use interpreter::{apply_ui_image_bytes, collect_pending_ui_image_fetches};
use program_bridge::filter_plugins;
#[cfg(not(target_arch = "wasm32"))]
use program_bridge::load_wasm_plugins;
#[cfg(target_arch = "wasm32")]
use program_bridge::parse_plugin_entries;
use shell::ShellState;
use std::cell::RefCell;
use std::io::Read;
use std::rc::Rc;
use std::sync::Arc;
#[cfg(target_arch = "wasm32")]
use ui_wgpu::wgpu::apply_canvas_cursor;
use ui_wgpu::wgpu::ActionDescriptor;
// 🏚️ `dispatch_window_event`/`WindowInputState`/`schedule_frame` no longer imported here — they were
// `SemioApp`/`start_frame_loop`-only (both deleted, packet os-host); `winit_app.rs` normalizes input
// itself via `ui_host::event` instead. See the `OsHostDecomposition — SemioApp deletion` region above.
use ui_wgpu::wgpu::{apply_window_cursor, fetch_font_bytes, resolve_semio_cursor, CursorDragState, DrawList, FontAtlas, GpuContext, IconAtlas, InputState, KeyAction, PointerCallbacks, PointerModifiers, SemioCursor, Theme};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
// 🏚️ `ApplicationHandler`/`WindowEvent`/`ActiveEventLoop`/`EventLoopProxy`/`WindowAttributes`/
// `WindowId` no longer imported here — all `SemioApp`-only (deleted, packet os-host); `winit_app.rs`
// imports each of these itself. `EventLoop`/`Window` stay: `run_native`/`semio_wgpu_mount` still
// construct the event loop and `AppRuntime` still names `Window` throughout.
use winit::event_loop::EventLoop;
#[cfg(not(target_arch = "wasm32"))]
use winit::window::Fullscreen;
use winit::window::Window;

//#region 🎠️KernelRuntime
/// 🎭️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet H3-wgpu-native; upgraded by terra-kernel-loop):
/// `📓️design-runtime.md` §1's "wgpu native" host — `Kernel` runs on a dedicated kernel thread; the
/// winit thread only submits requests and drains outbound results. terra-kernel-loop replaced the
/// original single-`ShardLoop` request-servant with `crate::parallel_runtime::ParallelRuntime`: real
/// `Kernel::submit`/`tick`/`complete` (DRR fairness, failure-ladder/metrics bookkeeping) dispatched
/// across K real `ShardExecutor` OS threads, one per `ShardTable`-pinned shard — see
/// `📓️terra-kernel-loop-report.md` for what is (and, per that report's own honest-gaps section, is
/// NOT) wired all the way through. `ProgramBridgeBackend::Wasm` (in `ProgramBridge/`) holds a
/// [`KernelClient`] instead of the deleted `Arc<WasmPluginRuntime>`; every plugin turn now executes
/// on this thread via `Kernel` + `GuestRuntime`/`WasmtimeRuntime` + `ParallelRuntime`, never
/// in-process on the winit thread.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod kernel_runtime {
    use semio_framework::kernel::{BrokerCapabilityGrant, Budget as TurnBudget, Effect, Event, MessageEndpoint, PatchOp, QuotaSchema, TurnResult, UiPatch as KernelUiPatch};
    use semio_framework_actor::{intersect_capabilities, ActivationEvent, ActorId, ActorKind, Backpressure, CapabilityGrant, Envelope, Lane, Origin, PackageHash, PackageId, Payload};
    use semio_framework_plugin_host::shard::ShardOutcome;
    use semio_framework_plugin_host::{GuestRuntime, GuestRuntimes, PackageRef, SharedEngineConfig, WasmtimeRuntime};
    use std::collections::HashMap;
    use std::future::Future;
    use std::path::PathBuf;
    use std::pin::Pin;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex, OnceLock};
    use std::task::{Context, Poll, Waker};
    use std::time::Duration;
    use ui_wgpu::wgpu::UiNode;

    static SEQ: AtomicU64 = AtomicU64::new(1);
    fn next_seq() -> u64 {
        SEQ.fetch_add(1, Ordering::Relaxed)
    }

    /// ⛽️ One generous constant turn budget until the DRR scheduler threads a real per-lane one
    /// through (same honestly-flagged gap `PluginInstanceHandle`'s `RELAY_JOB_BUDGET` already
    /// documents on the host side for jobs — this is its `reactor::poll` turn-budget twin).
    const TURN_BUDGET: TurnBudget = TurnBudget { fuel: 50_000_000, deadline_ms: 100, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 8 };

    /// ⏳️ terra-kernel-loop: same tripwire shape as `scale_bench`'s own `PUMP_OUTCOME_TIMEOUT` —
    /// how long `run_turn`'s tick loop waits for a granted turn's `ShardOutcome` before giving up.
    const RUN_TURN_OUTCOME_TIMEOUT: Duration = Duration::from_secs(5);

    /// 🧵️ terra-kernel-loop, item 3 of the packet brief: K sized from `semio_framework_async::
    /// thread_plan(cores).shards` rather than a fresh ad-hoc formula — "the global thread budget
    /// exists so no component sizes itself per-CPU." `available_parallelism()` failing (rare; a
    /// sandboxed/exotic host) falls back to `4` cores' worth of shards rather than panicking the
    /// kernel thread before it can even start.
    fn native_shard_count() -> u16 {
        let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
        semio_framework_async::thread_plan(cores).shards as u16
    }

    //#region 🔖️ExtensionIndex
    /// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the descriptor-driven
    /// source of truth for "which extensions activate alongside plugin X" — `📓️design-unified.md`
    /// §M6. Embedded at COMPILE time via `include_str!` (never a runtime path lookup into `🤖️
    /// generated/**`, which is gitignored and has no stable runtime location once packaged) — the
    /// same registry `🦀️hosts.rs` a few lines above this module already mounts as real Rust source,
    /// just read as data here instead of compiled as code. READ-ONLY: this file is `🤖️generated/**`,
    /// registrar-owned; never edited by this packet.
    const PLUGINS_REGISTRY_JSON: &str = include_str!("../../../../../../🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json");

    /// 🧩️ The handful of fields this host actually needs out of one registry entry — every other
    /// field (`hashes`, `dependsOn`, `activationEvents`, …) is irrelevant to activation and left
    /// unparsed (serde ignores unknown JSON fields by default).
    #[derive(serde::Deserialize)]
    struct PluginDescriptorJson {
        #[serde(rename = "pluginId")]
        plugin_id: String,
        role: String,
        #[serde(default)]
        capabilities: Vec<String>,
        #[serde(default)]
        extends: Option<String>,
    }

    /// 🧩️ One extension's activation-relevant identity — `extension_id`/`package` are the SAME
    /// string (the extension crate's own `pluginId`, deliberately distinct from its parent's
    /// `PackageId` — see `activate_extensions_of`'s own doc for why this matters for package-wide
    /// quarantine isolation). `capability_requests` mirrors the registry's `capabilities` array
    /// verbatim as unscoped [`CapabilityGrant`]s (`scope: None` — this host has no capability broker
    /// populating scopes for ANY actor kind yet, extension or plugin).
    #[derive(Clone)]
    struct ExtensionRecord {
        extension_id: String,
        package: PackageId,
        capability_requests: Vec<CapabilityGrant>,
    }

    /// 🧩️ `by_parent[plugin_id]` = every installed extension whose descriptor `extends == plugin_id`
    /// — exactly the "descriptors carry `extends`... zero special-casing" design requirement.
    struct ExtensionIndex {
        by_parent: HashMap<String, Vec<ExtensionRecord>>,
    }

    impl ExtensionIndex {
        fn load() -> Self {
            let mut by_parent: HashMap<String, Vec<ExtensionRecord>> = HashMap::new();
            let entries: Vec<PluginDescriptorJson> = serde_json::from_str(PLUGINS_REGISTRY_JSON).unwrap_or_default();
            for entry in entries {
                if entry.role != "extension" {
                    continue;
                }
                let Some(parent) = entry.extends else { continue };
                let record = ExtensionRecord {
                    extension_id: entry.plugin_id.clone(),
                    package: PackageId(entry.plugin_id),
                    capability_requests: entry.capabilities.into_iter().map(|capability| CapabilityGrant { capability, scope: None }).collect(),
                };
                by_parent.entry(parent).or_default().push(record);
            }
            Self { by_parent }
        }

        fn extensions_of(&self, plugin_id: &str) -> &[ExtensionRecord] {
            self.by_parent.get(plugin_id).map(Vec::as_slice).unwrap_or(&[])
        }
    }

    /// 🧩️ Parsed once, lazily — the registry JSON is fixed at compile time (`include_str!`), so
    /// there is nothing to invalidate or re-read across the process's lifetime.
    fn extension_index() -> &'static ExtensionIndex {
        static INDEX: OnceLock<ExtensionIndex> = OnceLock::new();
        INDEX.get_or_init(ExtensionIndex::load)
    }

    /// 🧩️ Mirrors `program_bridge::load_wasm_plugins`'s own "first `.wasm` file directly inside the
    /// plugin's own directory" convention — kept as a small local helper rather than importing that
    /// module's version, which is embedded inside its own `find`-style closure, not a reusable fn.
    fn find_wasm_artifact(dir: &std::path::Path) -> Option<PathBuf> {
        std::fs::read_dir(dir).ok()?.filter_map(|entry| entry.ok()).map(|entry| entry.path()).find(|path| path.extension().is_some_and(|ext| ext == "wasm"))
    }
    //#endregion 🔖️ExtensionIndex

    //#region 🔖️Requests/Outcomes
    pub(crate) enum KernelRequest {
        CreateApp { wasm_path: PathBuf, plugin_id: String, app_id: String },
        DestroyApp { instance: u32 },
        /// 📡️ `events` is normally one `Event::AppCommandEvent` (the `exchange` collapse,
        /// `📓️design-abi.md` §2/§4) but callers that need `surface-visible` (rendering) or other raw
        /// kernel events pass those directly — a single turn may carry several.
        Exchange { instance: u32, events: Vec<Event> },
    }

    pub(crate) struct ExchangeOutcome {
        pub frames: Vec<protocol::AppFrame>,
        /// 🖼️ Surfaces this turn repainted or retained on desync — reconciled against the kernel
        /// thread's own retained tree (`KernelThreadState::retained`); see that field's doc for the
        /// full-body-vs-desync policy.
        pub surfaces: HashMap<String, UiNode>,
        /// 🧾️ Every effect this turn produced that was NOT one of the `Effect::SendMessage{target:
        /// Shell{..}}` entries already unpacked into `frames` above — `📓️design-abi.md` §2's
        /// replacement for the deleted `AppFrame::Effects` wrapper: effects now travel as real
        /// `kernel::Effect` values on `TurnResult.effects` directly, not re-encoded as an `AppFrame`.
        pub effects: Vec<Effect>,
    }

    pub(crate) enum KernelOutcome {
        Created(Result<u32, String>),
        Exchanged(Result<ExchangeOutcome, String>),
    }
    //#endregion

    //#region 🔖️KernelFuture — the leaf `Future` every `ProgramBridgeEntry` async method awaits
    #[derive(Default)]
    struct ResponseSlot {
        result: Mutex<Option<KernelOutcome>>,
        waker: Mutex<Option<Waker>>,
    }

    impl ResponseSlot {
        fn deliver(&self, outcome: KernelOutcome) {
            *self.result.lock().expect("response slot lock") = Some(outcome);
            if let Some(waker) = self.waker.lock().expect("response slot lock").take() {
                waker.wake();
            }
        }
    }

    /// 🌉 The genuinely-yielding leaf every plugin call now awaits, replacing the old in-process
    /// `WasmPluginRuntime::exchange` blocking call. Whoever drives this to completion (`pollster`'s
    /// own park-based executor for the majority of call sites that are fine staying synchronous, or
    /// `poll_app_tasks`'s tiny task-pool executor for the 3 sites this packet moved off the winit
    /// thread — see `📓️terra-H3-wgpu-native-report.md`) supplies its own `Waker`; this future does
    /// not care which, it only stores+calls whatever it was last polled with.
    struct KernelFuture {
        slot: Arc<ResponseSlot>,
        request: Option<KernelRequest>,
        sender: mpsc::Sender<(KernelRequest, Arc<ResponseSlot>)>,
    }

    impl Future for KernelFuture {
        type Output = KernelOutcome;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.get_mut();
            if let Some(request) = this.request.take() {
                let _ = this.sender.send((request, this.slot.clone()));
            }
            let mut result = this.slot.result.lock().expect("response slot lock");
            if let Some(outcome) = result.take() {
                return Poll::Ready(outcome);
            }
            drop(result);
            *this.slot.waker.lock().expect("response slot lock") = Some(cx.waker().clone());
            Poll::Pending
        }
    }
    //#endregion

    //#region 🔖️KernelClient
    #[derive(Clone)]
    pub(crate) struct KernelClient {
        sender: mpsc::Sender<(KernelRequest, Arc<ResponseSlot>)>,
    }

    fn global_client() -> &'static OnceLock<KernelClient> {
        static CLIENT: OnceLock<KernelClient> = OnceLock::new();
        &CLIENT
    }

    impl KernelClient {
        /// ▶️ Spawns the kernel thread exactly once (lazily, on first use — every `ProgramBridgeEntry`
        /// clones the same handle afterward). Native-only; there is no equivalent on wasm32, where
        /// the JS host already owns the actual plugin execution off this crate's own thread.
        pub(crate) fn get() -> KernelClient {
            global_client()
                .get_or_init(|| {
                    let (tx, rx) = mpsc::channel::<(KernelRequest, Arc<ResponseSlot>)>();
                    std::thread::Builder::new().name("semio-kernel".into()).spawn(move || run_kernel_thread(rx)).expect("spawn kernel thread");
                    KernelClient { sender: tx }
                })
                .clone()
        }

        fn submit(&self, request: KernelRequest) -> KernelFuture {
            KernelFuture { slot: Arc::new(ResponseSlot::default()), request: Some(request), sender: self.sender.clone() }
        }

        pub(crate) async fn create_app(&self, wasm_path: PathBuf, plugin_id: String, app_id: String) -> Result<u32, String> {
            match self.submit(KernelRequest::CreateApp { wasm_path, plugin_id, app_id }).await {
                KernelOutcome::Created(result) => result,
                KernelOutcome::Exchanged(_) => Err("kernel: unexpected Exchanged response for create_app".into()),
            }
        }

        /// ✂️ Fire-and-forget, matching the old `WasmPluginRuntime::destroy_app`'s `fn(&self, u32)`
        /// (no result) shape — the kernel thread frees the actor's `GuestInstance` asynchronously.
        pub(crate) fn destroy_app(&self, instance: u32) {
            let _ = self.sender.send((KernelRequest::DestroyApp { instance }, Arc::new(ResponseSlot::default())));
        }

        pub(crate) async fn exchange_commands(&self, instance: u32, commands: Vec<protocol::AppCommand>) -> Result<ExchangeOutcome, String> {
            let events = commands.into_iter().map(|command| Event::AppCommandEvent { instance: semio_framework::kernel::PluginInstanceId(instance.to_string()), seq: next_seq(), command: protocol::encode_app_command(&command) }).collect();
            self.exchange_events(instance, events).await
        }

        pub(crate) async fn exchange_events(&self, instance: u32, events: Vec<Event>) -> Result<ExchangeOutcome, String> {
            match self.submit(KernelRequest::Exchange { instance, events }).await {
                KernelOutcome::Exchanged(result) => result,
                KernelOutcome::Created(_) => Err("kernel: unexpected Created response for exchange".into()),
            }
        }
    }
    //#endregion

    //#region 🔖️KernelThreadState
    struct RetainedSurface {
        revision: u64,
        node: UiNode,
    }

    struct KernelThreadState {
        guest_runtime: Arc<GuestRuntimes>,
        /// 🎠️ terra-kernel-loop: the real multi-shard engine — replaces the single physical
        /// `ShardLoop`/`Kernel::new(.., 1, 0, ..)` this host used to run. `Kernel::new(Thread, K, 2,
        /// 64)` (`exclusive_reserve: 2` — item 3 of the packet brief — makes `request_exclusive`
        /// real; no caller in this file exercises it yet, but the reserve pool now genuinely exists).
        runtime: crate::parallel_runtime::ParallelRuntime,
        /// ⏱️ Monotonic milliseconds this host's own `Kernel::tick` calls are stamped with — this
        /// crate's purity-respecting clock source (`Kernel` itself takes no clock, per `🎭️actor`'s
        /// own rule), incremented once per `run_turn`-internal tick, never wall-clock-read.
        now_ms: u64,
        plugin_ordinals: HashMap<String, u16>,
        /// 📇️ `instance_id` (the `u32` `ProgramBridgeEntry`'s callers already address plugin apps
        /// by) → the kernel's own bit-packed `ActorId`, minted by `Kernel::activate`.
        instances: HashMap<u32, ActorId>,
        next_instance_id: u32,
        /// 🖼️ `📓️design-runtime.md` §"Scene": one retained `UiNode` per `(instance, surface)`,
        /// reconciled from `TurnResult.ui_patches` on every turn — this crate's stand-in for a full
        /// `SceneStore` snapshot swap (item 4 of the packet: never block the render loop on a plugin
        /// turn, reuse the previous tree on a missed/rejected patch). Only `PatchOp::Replace{path:
        /// "", node}` (a full body) is applied by walking the tree; `📓️design-abi.md` §4's guest-side
        /// diffing (`InsertChild`/`RemoveChild`/non-root `Replace`/`SetProps`) has no guest emitting
        /// it yet (no plugin has migrated to `world actor`, W3 hasn't started) — an unrecognized op
        /// shape is treated as a desync, not walked, exactly like a `base_revision` mismatch.
        retained: HashMap<(u32, String), RetainedSurface>,
        /// 🔁️ Surfaces whose next turn must carry an `Event::PatchRejected` asking the guest to
        /// resend a full body — queued here instead of round-tripping an extra turn synchronously.
        pending_rejections: HashMap<(u32, String), u64>,
    }

    impl KernelThreadState {
        fn new() -> Self {
            let guest_runtime: Arc<GuestRuntimes> = Arc::new(GuestRuntimes::Wasmtime(WasmtimeRuntime::new(SharedEngineConfig::default()).expect("wasmtime engine builds")));
            let runtime = crate::parallel_runtime::ParallelRuntime::new(guest_runtime.clone(), native_shard_count(), 2, 64);
            Self { guest_runtime, runtime, now_ms: 0, plugin_ordinals: HashMap::new(), instances: HashMap::new(), next_instance_id: 1, retained: HashMap::new(), pending_rejections: HashMap::new() }
        }

        fn plugin_ordinal(&mut self, plugin_id: &str) -> u16 {
            let next = self.plugin_ordinals.len() as u16;
            *self.plugin_ordinals.entry(plugin_id.to_string()).or_insert(next)
        }

        fn create_app(&mut self, wasm_path: PathBuf, plugin_id: String, app_id: String) -> Result<u32, String> {
            let bytes = std::fs::read(&wasm_path).map_err(|error| format!("{}: {error}", wasm_path.display()))?;
            let hash = PackageHash(*blake3::hash(&bytes).as_bytes());
            let package_id = PackageId(plugin_id.clone());
            let package_ref = PackageRef { package: package_id.clone(), hash };
            // 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): `GuestRuntime::
            // compile` is `async fn` (the actor-green async migration landed after this call site was
            // written) — bridged with `pollster::block_on`, this file's own established sync↔async
            // bridge (already used repeatedly below, e.g. `poll_world3d_assets`/`fetch_url_bytes`).
            // Pre-existing defect in this exact function, fixed here because it sits directly upstream
            // of this packet's own cascade addition below.
            let compiled = pollster::block_on(self.guest_runtime.compile(&package_ref, &bytes)).map_err(|error| error.to_string())?;
            let instance_id = self.next_instance_id;
            self.next_instance_id += 1;
            let plugin_ordinal = self.plugin_ordinal(&plugin_id);
            let actor = self.runtime.activate(package_id.clone(), plugin_ordinal, ActorKind::PluginApp { plugin: package_id, app_id: app_id.clone(), instance_id }, Lane::Interactive, None, ActivationEvent::Manual, &compiled, &[] as &[BrokerCapabilityGrant], &TURN_BUDGET)?;
            self.instances.insert(instance_id, actor);
            // 🐣️ `InstanceOpen` is the first event a fresh instance must receive (`📓️design-abi.md`
            // §2) — `actor`/`config`/`assets`/`capabilities` are placeholders until a real capability
            // broker/asset-preload pipeline lands (A2b/T1 territory, not this packet's).
            let open = Event::InstanceOpen {
                instance: semio_framework::kernel::PluginInstanceId(instance_id.to_string()),
                app_id: semio_framework::kernel::AppInstanceId(app_id),
                actor: "local".to_string(),
                config: Vec::new(),
                assets: Vec::new(),
                capabilities: Vec::new(),
                quotas: QuotaSchema::default(),
            };
            self.run_turn(actor, instance_id, vec![open])?;
            // 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): descriptor-driven
            // native cascade — M6's own acceptance wording, "activating a parent brings up its N
            // extension actors." `wasm_path` is always `<modules_root>/<plugin_id>/<file>.wasm`
            // (`program_bridge::load_wasm_plugins`'s own layout convention), so the extensions' own
            // wasm artifacts live as siblings under the same `modules_root`.
            if let Some(modules_root) = wasm_path.parent().and_then(|dir| dir.parent()) {
                self.activate_extensions_of(&plugin_id, actor, modules_root);
            }
            Ok(instance_id)
        }

        /// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): for every
        /// descriptor in the generated registry with `extends == plugin_id`, compile (`guest_runtime.
        /// compile` is itself keyed/cached by `PackageRef` content hash — no extra cache layer needed
        /// here) and activate it as `ActorKind::Extension`, `Lane::Background` (design doc M6's
        /// decided default — a UI-contributing extension is re-laned on `SurfaceVisible` by whichever
        /// packet wires that event), with `scoped_grants` = [`semio_framework_actor::
        /// intersect_capabilities`] of the parent's own granted set against the extension's requests
        /// — the never-escalate-past-the-parent property. Records the [`semio_framework_actor::Kernel::
        /// link_extension`] cascade edge so [`Self::destroy_app`] takes every extension down with its
        /// parent. Best-effort per extension (mirrors `program_bridge::load_wasm_plugins`'s own "one
        /// bad plugin does not hold the batch hostage" policy) — a missing/broken extension is logged
        /// and skipped, never fails the parent's own `create_app`.
        ///
        /// 🕳️ Honest gap, not worked around: this activates the extension via `ParallelRuntime::
        /// activate` (least-loaded shard), NOT pinned to the parent's exact shard — `ParallelRuntime`
        /// has no `activate_pinned` entry point today (that facade lives in `🎯️targets/🧊️wgpu/
        /// 🎠️runtime.rs`, owned by a different packet, `kernel-async-native`). The kernel-level
        /// primitive this method WOULD call (`Kernel::activate_pinned`) is built, tested, and green in
        /// `semio-framework-actor`; only the host-level plumbing to reach it through `ParallelRuntime`
        /// is missing. A lease-request for a small additive method is open — see this ticket's report.
        /// `link_extension` (cascade topology, zero-orphan teardown) is unaffected by this gap and
        /// works correctly regardless of which shard the extension landed on.
        fn activate_extensions_of(&mut self, plugin_id: &str, parent: ActorId, modules_root: &std::path::Path) {
            let extensions = extension_index().extensions_of(plugin_id);
            if extensions.is_empty() {
                return;
            }
            let parent_grants = pollster::block_on(self.runtime.kernel().actor_record(parent)).map(|record| record.capabilities).unwrap_or_default();
            for extension in extensions.to_vec() {
                let extension_dir = modules_root.join(&extension.extension_id);
                let Some(extension_wasm_path) = find_wasm_artifact(&extension_dir) else {
                    crate::log_debug(&format!("kernel: extension {} of {plugin_id} has no compiled wasm under {}, skipping", extension.extension_id, extension_dir.display()));
                    continue;
                };
                let extension_bytes = match std::fs::read(&extension_wasm_path) {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        crate::log_debug(&format!("kernel: failed reading extension {} wasm ({}): {error}", extension.extension_id, extension_wasm_path.display()));
                        continue;
                    }
                };
                let extension_hash = PackageHash(*blake3::hash(&extension_bytes).as_bytes());
                let extension_package_ref = PackageRef { package: extension.package.clone(), hash: extension_hash };
                let extension_compiled = match pollster::block_on(self.guest_runtime.compile(&extension_package_ref, &extension_bytes)) {
                    Ok(handle) => handle,
                    Err(error) => {
                        crate::log_debug(&format!("kernel: compile failed for extension {}: {error}", extension.extension_id));
                        continue;
                    }
                };
                let extension_ordinal = self.plugin_ordinal(&extension.extension_id);
                let extension_kind = ActorKind::Extension { plugin: PackageId(plugin_id.to_string()), extension_id: extension.extension_id.clone() };
                // 🕳️ Honest gap: the REAL capability enforcement point for a guest instance is the
                // `caps: &[BrokerCapabilityGrant]` argument below, `&[]` here because this native host
                // has no capability broker wired up for ANY actor kind yet — the parent's own
                // activation above passes the identical empty placeholder (A2b/T1 territory). The
                // `intersect_capabilities` call still records the correctly-scoped grant kernel-side
                // (`set_capabilities` below) so the intersection mechanism is exercised end-to-end and
                // ready the moment a broker starts populating `parent_grants` for real.
                match self.runtime.activate(extension.package.clone(), extension_ordinal, extension_kind, Lane::Background, None, ActivationEvent::Manual, &extension_compiled, &[] as &[BrokerCapabilityGrant], &TURN_BUDGET) {
                    Ok(extension_actor) => {
                        let scoped_grants = pollster::block_on(intersect_capabilities(&parent_grants, &extension.capability_requests));
                        if let Err(error) = pollster::block_on(self.runtime.kernel_mut().set_capabilities(extension_actor, scoped_grants)) {
                            crate::log_debug(&format!("kernel: set_capabilities({extension_actor:?}) failed: {error}"));
                        }
                        if let Err(error) = pollster::block_on(self.runtime.kernel_mut().link_extension(parent, extension_actor)) {
                            crate::log_debug(&format!("kernel: link_extension({parent:?}, {extension_actor:?}) failed: {error}"));
                        }
                    }
                    Err(error) => crate::log_debug(&format!("kernel: activate failed for extension {}: {error}", extension.extension_id)),
                }
            }
        }

        fn destroy_app(&mut self, instance: u32) {
            if let Some(actor) = self.instances.remove(&instance) {
                // 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): cascade
                // teardown — `Kernel::deactivate` walks `actor`'s cascade subtree leaves-first and
                // removes every extension from the KERNEL's own bookkeeping (mailbox/shard/failure
                // state); each removed id ALSO needs its own `ParallelRuntime::unregister` to retire
                // the shard-side `GuestInstance` — the two are separate teardown halves, matching
                // `Kernel`'s own purity boundary (no transport inside the pure crate). Falls back to
                // unregistering just `actor` if `Kernel::deactivate` errors (e.g. already gone) —
                // the pre-existing single-actor behaviour this method had before this packet.
                let removed = pollster::block_on(self.runtime.kernel_mut().deactivate(actor)).unwrap_or_else(|_| vec![actor]);
                for id in removed {
                    self.runtime.unregister(id);
                }
            }
            self.retained.retain(|(inst, _), _| *inst != instance);
            self.pending_rejections.retain(|(inst, _), _| *inst != instance);
        }

        fn exchange(&mut self, instance: u32, mut events: Vec<Event>) -> Result<ExchangeOutcome, String> {
            let Some(&actor) = self.instances.get(&instance) else {
                return Err(format!("kernel: instance {instance} is not registered"));
            };
            let rejections: Vec<(u32, String)> = self.pending_rejections.keys().filter(|(inst, _)| *inst == instance).cloned().collect();
            for key in rejections {
                if let Some(revision) = self.pending_rejections.remove(&key) {
                    events.insert(0, Event::PatchRejected { surface: key.1, revision, reason: "revision-mismatch".to_string() });
                }
            }
            self.run_turn(actor, instance, events)
        }

        /// 🎠️ terra-kernel-loop: the real loop the packet brief's item 1 asks for — `Kernel::submit`
        /// (honouring `Backpressure`; a non-`Accept` result is logged rather than silently ignored,
        /// but does not abort the turn since `Coalesced`/`Dropped` both still leave AT LEAST one
        /// envelope queued and `Rejected` on a freshly-activated actor's own generous Interactive-lane
        /// mailbox should not occur in practice) → `Kernel::tick` → dispatch to the actor's OWN pinned
        /// shard (a REAL `ShardExecutor` OS thread, not the single physical `ShardLoop` this host used
        /// to drive) → wait for that shard's `ShardOutcome` → `Kernel::complete` (closing the bridging
        /// gap this method's OWN doc comment used to flag as unreached) → hand the result to
        /// `apply_turn_result`. Loops `tick_and_dispatch` until nothing is left to grant — normally
        /// one iteration (this host submits for exactly one actor per call), but `Kernel::tick`'s DRR
        /// scheduler is global, so this stays correct if that ever changes.
        ///
        /// 🕳️ Honest gap: `Kernel::commit_frame`/`apply_scene_patch` are NOT called here —
        /// `KernelThreadState::activate` (via `ParallelRuntime::activate`) still passes `window: None`
        /// for every actor, so `Kernel`'s own `SceneStore` would stay permanently empty regardless;
        /// this host's UI pipeline already has its own frame-boundary mechanism (`retained`/
        /// `apply_ui_patch`, "item 4" of the original H3 packet). Wiring per-window `Kernel::
        /// commit_frame` for real would mean migrating THIS host's whole UI-patch pipeline onto
        /// `Kernel`'s `SceneStore`, a substantially larger, separate refactor out of this packet's
        /// scope (see `📓️terra-kernel-loop-report.md`'s own gaps section).
        fn run_turn(&mut self, actor: ActorId, instance: u32, events: Vec<Event>) -> Result<ExchangeOutcome, String> {
            let mut envelopes = Vec::with_capacity(events.len().max(1));
            if events.is_empty() {
                envelopes.push(Envelope { to: actor, from: Origin::Kernel, lane: Lane::Interactive, seq: next_seq(), deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: serde_json::to_vec(&Event::Wake).map_err(|error| error.to_string())? } });
            } else {
                for event in &events {
                    envelopes.push(Envelope { to: actor, from: Origin::Kernel, lane: Lane::Interactive, seq: next_seq(), deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: serde_json::to_vec(event).map_err(|error| error.to_string())? } });
                }
            }
            for envelope in &envelopes {
                if !matches!(self.runtime.submit(envelope), Backpressure::Accept) {
                    crate::log_debug(&format!("kernel: run_turn submit for actor {} was not Accept-ed (mailbox pressure)", actor.0));
                }
            }
            let mut turn_result: Option<TurnResult> = None;
            let mut fault: Option<String> = None;
            loop {
                self.now_ms += 1;
                let decision = self.runtime.tick_and_dispatch(self.now_ms, |_actor| crate::actor_budget_from_turn_budget(TURN_BUDGET, Lane::Interactive));
                if decision.run.is_empty() {
                    break;
                }
                let outcomes = self.runtime.wait_for_outcomes(decision.run.len(), RUN_TURN_OUTCOME_TIMEOUT);
                if outcomes.len() < decision.run.len() {
                    return Err("kernel: shard produced no outcome for this turn".to_string());
                }
                for outcome in &outcomes {
                    match outcome {
                        ShardOutcome::Turn { actor: reported, result } => {
                            let _ = self.runtime.complete(ActorId(*reported), result, 0, 0, self.now_ms);
                            if *reported == actor.0 {
                                turn_result = Some(result.clone());
                            }
                        }
                        // 🎠️ terra-kernel-loop: a trap must ALSO reach `Kernel::complete` — otherwise
                        // the failure ladder (`FailureState::on_signal`) never sees it, staying just as
                        // inert for the trap path as `Kernel::complete` being uncalled at all used to
                        // leave it. `ShardOutcome::Fault` carries no `TurnResult` (no `fuel_used`, no
                        // `Effect`s — the turn never returned one), so a minimal `Faulted` `TurnResult`
                        // is synthesized from its `message` — the same shape `apply_turn_result`'s
                        // caller already treats a fault as `TurnStatus::Faulted` for retry purposes.
                        ShardOutcome::Fault { actor: reported, message } => {
                            let faulted = TurnResult { ui_patches: Vec::new(), effects: Vec::new(), next_wake: None, status: semio_framework::kernel::TurnStatus::Faulted(message.clone().into_bytes()), fuel_used: 0 };
                            let _ = self.runtime.complete(ActorId(*reported), &faulted, 0, 0, self.now_ms);
                            if *reported == actor.0 {
                                fault = Some(message.clone());
                            }
                        }
                        // 🚧️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (K1, landed mid-session):
                        // `ShardOutcome` grew `Job`/`Checkpoint`/`Resumed`/`Cancelled` for job-stepping
                        // and the newly-wired `Payload::Suspend`/`Resume`/`Cancel` dispatch. This
                        // kernel thread never sends those payloads (only `run_turn`'s own
                        // `Payload::Event`), so any of them reaching here — for `actor` OR any other
                        // actor `Kernel::tick` happened to grant in the SAME call — is silently
                        // ignored rather than aborting an otherwise-successful turn; unlike the
                        // ORIGINAL `Fault`/`Job` handling this replaces, this loop may observe outcomes
                        // for actors OTHER than `actor` (DRR is global), so those must not error out.
                        _ => {}
                    }
                }
            }
            if let Some(message) = fault {
                return Err(message);
            }
            match turn_result {
                Some(result) => self.apply_turn_result(actor, instance, result),
                None => Err("kernel: shard produced no outcome for this turn".to_string()),
            }
        }

        fn apply_turn_result(&mut self, actor: ActorId, instance: u32, result: TurnResult) -> Result<ExchangeOutcome, String> {
            // 🎠️ terra-kernel-loop: `Kernel::complete` (the bridge this doc comment used to flag as
            // unreached — "bridging the two needs a real pack-encode step this packet didn't reach")
            // is now genuinely called, from `run_turn`, for EVERY `ShardOutcome::Turn` a tick grants
            // (including `actor`'s own, before this method is even invoked) — so `Kernel`'s
            // failure-ladder/metrics bookkeeping is live for this host now, not skipped.
            let _ = actor;
            let mut frames = Vec::new();
            let mut effects = Vec::new();
            for effect in result.effects {
                if let Effect::SendMessage { target: MessageEndpoint::Shell { instance: target_instance }, payload } = &effect {
                    if target_instance.0 == instance.to_string() {
                        if let Ok(frame) = protocol::decode_app_frame(payload) {
                            frames.push(frame);
                            continue;
                        }
                    }
                }
                effects.push(effect);
            }
            let mut surfaces = HashMap::new();
            for patch in &result.ui_patches {
                self.apply_ui_patch(instance, patch, &mut surfaces);
            }
            Ok(ExchangeOutcome { frames, surfaces, effects })
        }

        fn apply_ui_patch(&mut self, instance: u32, patch: &KernelUiPatch, out: &mut HashMap<String, UiNode>) {
            let key = (instance, patch.surface.clone());
            let full_body = match patch.ops.as_slice() {
                [PatchOp::Replace { path, node }] if path.is_empty() => Some(node.clone()),
                _ => None,
            };
            let local_revision = self.retained.get(&key).map(|surface| surface.revision).unwrap_or(0);
            if let Some(node) = full_body {
                self.retained.insert(key.clone(), RetainedSurface { revision: patch.revision, node: node.clone() });
                self.pending_rejections.remove(&key);
                out.insert(patch.surface.clone(), node);
            } else {
                // 🚧️ Incremental ops or `base_revision` mismatch — queue `PatchRejected` and reuse the
                // previous full-body snapshot (item 4) so `render_with_document` keeps painting stale UI
                // instead of erroring on a missing surface key.
                self.pending_rejections.insert(key.clone(), local_revision);
                if let Some(retained) = self.retained.get(&key) {
                    out.insert(patch.surface.clone(), retained.node.clone());
                }
            }
        }
    }

    fn run_kernel_thread(rx: mpsc::Receiver<(KernelRequest, Arc<ResponseSlot>)>) {
        let mut state = KernelThreadState::new();
        while let Ok((request, slot)) = rx.recv() {
            let outcome = match request {
                KernelRequest::CreateApp { wasm_path, plugin_id, app_id } => KernelOutcome::Created(state.create_app(wasm_path, plugin_id, app_id)),
                KernelRequest::DestroyApp { instance } => {
                    state.destroy_app(instance);
                    continue;
                }
                KernelRequest::Exchange { instance, events } => KernelOutcome::Exchanged(state.exchange(instance, events)),
            };
            slot.deliver(outcome);
        }
    }
    //#endregion

    //#region 🔖️TaskPool — the non-blocking executor for `spawn_app_task`
    // 🌀️ `spawn_app_task`'s native replacement for `pollster::block_on(future)`: pushes onto a
    // thread-local pool polled from `about_to_wait` (which already runs every loop iteration —
    // `ControlFlow::Poll` is set once `RuntimeReady` lands) instead of running the future to
    // completion synchronously on the winit thread. This is safe precisely because
    // `about_to_wait`/`poll_tasks` never hold a `try_borrow_mut()` on `Rc<RefCell<AppRuntime>>`
    // themselves while polling — each queued future re-acquires its OWN borrow only for the
    // instant it needs it (the existing `if let Ok(mut app) = runtime.try_borrow_mut() { ...await
    // inside here... }` pattern every `PointerCallbacks` closure already used before this packet).
    // `Waker::noop()` WAS correct here exactly because the loop used to be continuous `Poll` — that
    // honest gap this comment used to flag ("a real cross-thread `EventLoopProxy` wake... is not
    // implemented") is exactly what `📓️terra-os-host-report.md` (ticket
    // 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY, packet os-host) closes below: now that
    // `winit_app.rs` sets `ControlFlow::WaitUntil`/`Wait` instead, a `KernelFuture` awaiting a kernel
    // round trip needs a REAL wake for its completion to be noticed promptly rather than only on the
    // next unrelated event/deadline — see `install_waker`/`REAL_WAKER` immediately below, and
    // `kernel_seam.rs`'s own module docstring for the full "waker correctness" writeup.
    thread_local! {
        static TASK_POOL: std::cell::RefCell<Vec<Pin<Box<dyn Future<Output = ()>>>>> = const { std::cell::RefCell::new(Vec::new()) };
        static REAL_WAKER: std::cell::RefCell<Option<Waker>> = const { std::cell::RefCell::new(None) };
    }

    pub(crate) fn spawn_task(future: impl Future<Output = ()> + 'static) {
        TASK_POOL.with(|pool| pool.borrow_mut().push(Box::pin(future)));
    }

    /// 🔔️ `winit_app.rs` installs a real `Waker` (built from `ui_host::WakeProxy`, `Send + Sync`, so
    /// it can be called from off the winit thread — see `kernel_seam.rs`) once, at boot, after the
    /// event loop exists. Before that first install, `poll_tasks` falls back to `Waker::noop()`,
    /// harmless for the same reason it always was: nothing is waiting on a `WaitUntil` yet this early.
    pub(crate) fn install_waker(waker: Waker) {
        REAL_WAKER.with(|cell| *cell.borrow_mut() = Some(waker));
    }

    pub(crate) fn poll_tasks() {
        let waker: Waker = REAL_WAKER.with(|cell| cell.borrow().clone()).unwrap_or_else(|| Waker::noop().clone());
        let mut cx = Context::from_waker(&waker);
        TASK_POOL.with(|pool| {
            let mut pool = pool.borrow_mut();
            pool.retain_mut(|task| task.as_mut().poll(&mut cx).is_pending());
        });
    }
    //#endregion
}
//#endregion 🎠️KernelRuntime

//#region 🔖️ActorBudgetBridge
/// ⚖️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop, unblocking terra-shard-grants'
/// `ShardFrame::Grant` wire change): `semio_framework::kernel::Budget` (what this crate's own
/// `kernel_runtime::TURN_BUDGET`/`scale_bench::turn_budget_of` already speak) →
/// `semio_framework_actor::Budget` (what a `Grant` frame carries over `ShardTransport`, replacing
/// the deleted `ShardLoop::pump(|actor| ..)` budget closure). Shared by both native call sites
/// (`kernel_runtime::run_turn`, `scale_bench::Env::send_payload`) rather than duplicated — CLAUDE.md's
/// "if code is repeated, it MUST be close to each other" — so it lives here, the one file both
/// `#[cfg(not(target_arch = "wasm32"))]` modules already share. `memory_bytes`/`ui_nodes`/
/// `mailbox_len` have no source field on the kernel-`Budget` side; defaulted from `lane` via
/// `lane_defaults::budget_for` rather than invented — the same documented-gap shape
/// `🖥️host/🧵️shard/🦀️component.rs`'s own `BudgetBridge` region already uses for the REVERSE
/// direction (`GRANT_BUDGET_DEFAULT_MAX_FRAMES`).
#[cfg(not(target_arch = "wasm32"))]
fn actor_budget_from_turn_budget(budget: semio_framework::kernel::Budget, lane: semio_framework_actor::Lane) -> semio_framework_actor::Budget {
    let base = semio_framework_actor::lane_defaults::budget_for(lane);
    semio_framework_actor::Budget { fuel: budget.fuel, wall_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, ..base }
}
//#endregion 🔖️ActorBudgetBridge

//#region 🔖️ScaleBench
/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet V1b-bench): the native half of the ticket's
/// headline claim — "50+ plugins x 50+ extensions concurrently" — turned from measurable into
/// measured. Drives the REAL `semio-framework-os-scale-fixture` `wasm32-wasip2` component (one
/// compile, many instantiations, exactly the pooling-allocator scenario `build_shared_engine` was
/// built for) through `crate::parallel_runtime::ParallelRuntime` (terra-kernel-loop) — the same engine `//#region 🎠️KernelRuntime`
/// above already wires for the winit renderer, reused here without the winit/GPU half. terra-kernel-loop
/// upgraded this from a single physical `ShardLoop` behind all K shard labels to K real `ShardExecutor`
/// OS threads (see `Env`'s own doc for what this fixed for budgets 3/5/6). `bun ./📜️script.ts bench plugins --renderer native`
/// (`🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts`'s `//#region 🔖️Bench`) drives this via
/// `semio-wgpu-native --scale/--scale-wasm/--shards/--report`.
#[cfg(not(target_arch = "wasm32"))]
pub mod scale_bench {
    use semio_framework::kernel::{
        AppInstanceId, Budget as TurnBudget, CapabilityChange, CapabilityId, Effect, Event, PluginInstanceId, QuotaSchema, TurnResult,
    };
    use semio_framework_actor::{ActivationEvent as ActorActivationTrigger, ActorId, ActorKind, Envelope, Kernel, Lane, Origin, PackageHash, PackageId, Payload};
    use semio_framework_plugin_host::shard::ShardOutcome;
    use semio_framework_plugin_host::{CompiledHandle, GuestRuntime, GuestRuntimes, PackageRef, SharedEngineConfig, WasmtimeRuntime};
    use serde::Deserialize;
    use serde_json::json;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    /// ⏳️ terra-kernel-loop: how long `Env::pump` waits, per `Kernel::tick` call, for that tick's
    /// granted turns' `ShardOutcome`s to arrive from their K real `ShardExecutor` threads before
    /// treating the round as failed — generous (well past any interactive budget this ticket
    /// measures) so it is a genuine "something is stuck" tripwire, never a floor under budget 5's
    /// own timing (budget 5 measures the ACTUAL wait via `wait_for_outcomes`'s elapsed time, not
    /// this ceiling).
    const PUMP_OUTCOME_TIMEOUT: Duration = Duration::from_secs(10);

    //#region 🔖️RegistryJson
    #[derive(Deserialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    struct RegistryQuotas {
        deadline_ms: u32,
        max_effects: u32,
        max_patch_bytes: u32,
        max_frames: u32,
    }

    /// 🌉️ `scaleFixture`/`activationEvents` are kept as raw `serde_json::Value` rather than a second
    /// typed mirror of `FixtureConfig` (`🎭️profile/🦀️component.rs`) — the fixture's own guest re-parses
    /// this crate's re-serialized bytes with `serde_json::from_slice`, so byte-for-byte field-name
    /// fidelity matters more here than a typed struct's convenience, and there is exactly one JSON
    /// shape (the TS generator's) to stay honest to.
    #[derive(Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    struct RegistryRecord {
        id: String,
        kind: String,
        parent_id: Option<String>,
        activation_events: Vec<serde_json::Value>,
        quotas: RegistryQuotas,
        scale_fixture: serde_json::Value,
    }

    #[derive(Deserialize)]
    struct RegistryFile {
        records: Vec<RegistryRecord>,
    }

    fn profile_of(record: &RegistryRecord) -> &str {
        record.scale_fixture.get("profile").and_then(|v| v.as_str()).unwrap_or("idle")
    }

    fn is_startup(record: &RegistryRecord) -> bool {
        record.activation_events.iter().any(|e| e.get("type").and_then(|v| v.as_str()) == Some("on-startup-finished"))
    }

    /// 🎭️ Faults from actors that were NOT supposed to trap. The fixture ships `hang` (393 records)
    /// and `crash` (343 records) precisely so the watchdog and the failure ladder have something to
    /// catch — together 29% of the catalog — so a blanket `faults == 0` pass condition is really
    /// asking the crash profile not to crash, and it failed budgets 2 and 3 on a sample of the
    /// fixture behaving exactly as designed. `📓️design-workforce.md` §4 does not put a fault
    /// criterion on either budget: budget 2 is a deadline plus "only on-startup-finished actors
    /// live", budget 3 is actor count, shard count and per-shard ceiling. A trap from an
    /// `idle`/`cpu`/`ui`/`io`/`stateful` actor IS a real failure and still counts.
    fn unexpected_faults(outcomes: &[ShardOutcome], actors: &[ActorId], records: &[&RegistryRecord]) -> Vec<String> {
        let by_design: std::collections::HashSet<u64> =
            actors.iter().zip(records.iter()).filter(|(_, record)| matches!(profile_of(record), "hang" | "crash")).map(|(actor, _)| actor.0).collect();
        outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                ShardOutcome::Fault { actor, message } if !by_design.contains(actor) => Some(message.clone()),
                _ => None,
            })
            .collect()
    }

    /// ⛽️ Bench-wide fuel ceiling — not per-record (`RegistryQuotas` omits fuel on purpose: wasmtime
    /// dispatch + wit-bindgen overhead in an unoptimized `wasip2` build dwarfs plausible production
    /// per-turn ceilings; measured reference: `🗒️note`'s `describe()` alone burns ~92M fuel in debug).
    /// `deadline_ms`/`max_effects`/`max_patch_bytes`/`max_frames` stay record-derived (real per-turn
    /// dimensions this bench exercises, e.g. budget 6's hang deadline).
    const BENCH_FUEL: u64 = 200_000_000;

    fn turn_budget_of(record: &RegistryRecord) -> TurnBudget {
        TurnBudget { fuel: BENCH_FUEL, deadline_ms: record.quotas.deadline_ms, max_effects: record.quotas.max_effects, max_patch_bytes: record.quotas.max_patch_bytes, max_frames: record.quotas.max_frames }
    }

    fn instance_open_event(record: &RegistryRecord, instance_id: u32) -> Event {
        Event::InstanceOpen {
            instance: PluginInstanceId(instance_id.to_string()),
            app_id: AppInstanceId(record.id.clone()),
            actor: "bench".to_string(),
            config: serde_json::to_vec(&record.scale_fixture).unwrap_or_default(),
            assets: Vec::new(),
            capabilities: Vec::new(),
            quotas: QuotaSchema::default(),
        }
    }
    //#endregion 🔖️RegistryJson

    //#region 🔖️Row
    fn row(id: u32, description: &str, status: &str, measured: serde_json::Value, threshold: serde_json::Value, note: &str) -> serde_json::Value {
        json!({ "id": id, "description": description, "status": status, "measured": measured, "threshold": threshold, "note": note })
    }

    fn skipped(id: u32, description: &str, reason: &str) -> serde_json::Value {
        row(id, description, "skipped", serde_json::Value::Null, serde_json::Value::Null, reason)
    }
    //#endregion 🔖️Row

    //#region 🔖️Env
    /// 🧵️ terra-kernel-loop: `Env` now drives its actors through `crate::parallel_runtime::
    /// ParallelRuntime` — real `Kernel::submit`/`tick`/`complete` plus K real `ShardExecutor` OS
    /// threads, ONE per configured shard, replacing the single physical `ShardLoop` every actor's
    /// turn used to serialize behind regardless of `shard_count`. This is what makes budget 3's
    /// "shard assignment" check and budget 5's "interactive p95 under 40 cpu actors" measure a REAL
    /// K-way-parallel instrument for the first time — see `📓️terra-kernel-loop-report.md`.
    /// `Kernel::complete` is also now genuinely called from `pump` (closing the gap `//#region
    /// 🎠️KernelRuntime`'s own `apply_turn_result` doc and budget 8's own note both used to flag).
    struct Env {
        runtime: super::parallel_runtime::ParallelRuntime,
        budgets: HashMap<u64, TurnBudget>,
        seq: u64,
        ordinals: HashMap<String, u16>,
        now_ms: u64,
        /// 🌀️ `ShardOutcome`s already pulled off `runtime`'s aggregated forwarder channel by `pump`
        /// but not yet handed to a caller's `drain()` — mirrors the pre-existing single-`ShardLoop`
        /// `Env::drain`'s own "whatever's on the wire right now" contract, just sourced from a
        /// buffer instead of a single in-process channel (outcomes now arrive asynchronously from K
        /// real threads, so `pump` must collect them eagerly rather than leaving them for `drain` to
        /// read off a transport that no longer exists as a single queue).
        pending: Vec<ShardOutcome>,
    }

    impl Env {
        fn new(runtime: Arc<GuestRuntimes>, shard_count: u16) -> Self {
            let runtime = super::parallel_runtime::ParallelRuntime::new(runtime, shard_count.max(1), 0, 64);
            Self { runtime, budgets: HashMap::new(), seq: 0, ordinals: HashMap::new(), now_ms: 0, pending: Vec::new() }
        }

        fn kernel(&self) -> &Kernel {
            self.runtime.kernel()
        }

        fn ordinal(&mut self, package_id: &str) -> u16 {
            let next = self.ordinals.len() as u16;
            *self.ordinals.entry(package_id.to_string()).or_insert(next)
        }

        /// 🎚️ Activation lane defaults to `Background` for every budget (unchanged). Budget 5 is the
        /// sole caller of [`Self::activate_on_lane`] with `Lane::Interactive`: the budget's own text
        /// names an *interactive* actor, and the kernel's placement gate keys off the actor's
        /// ACTIVATION lane, so activating the probe as `Background` measured a background actor and
        /// left the interactive path untested. This is an instrument correction, not a threshold change.
        fn activate(&mut self, compiled: &CompiledHandle, record: &RegistryRecord) -> Result<ActorId, String> {
            self.activate_on_lane(compiled, record, Lane::Background)
        }

        fn activate_on_lane(&mut self, compiled: &CompiledHandle, record: &RegistryRecord, lane: Lane) -> Result<ActorId, String> {
            let kind = if record.kind == "extension" {
                ActorKind::Extension { plugin: PackageId(record.parent_id.clone().unwrap_or_default()), extension_id: record.id.clone() }
            } else {
                ActorKind::PluginApp { plugin: PackageId(record.id.clone()), app_id: record.id.clone(), instance_id: 0 }
            };
            let package_id = record.parent_id.clone().unwrap_or_else(|| record.id.clone());
            let ordinal = self.ordinal(&package_id);
            let budget = turn_budget_of(record);
            let actor = self.runtime.activate(PackageId(package_id), ordinal, kind, lane, None, ActorActivationTrigger::Manual, compiled, &[], &budget)?;
            self.budgets.insert(actor.0, budget);
            Ok(actor)
        }

        fn send(&mut self, actor: ActorId, event: &Event) {
            self.send_payload(actor, Payload::Event { bytes: serde_json::to_vec(event).unwrap_or_default() });
        }

        /// 🔀️ `Payload::Suspend`/`Payload::Resume`/`Payload::Cancel` need the same envelope plumbing
        /// as `send`'s `Payload::Event` — factored out so budget 7 (K1's now-unblocked Suspend/Resume
        /// dispatch) can drive them without duplicating the seq/envelope bookkeeping.
        ///
        /// 🐛️ terra-kernel-loop: now a real `Kernel::submit` — the DRR mailbox enqueue, drained by
        /// the NEXT `pump`'s `tick_and_dispatch` call — replacing the ad-hoc direct `ShardFrame::
        /// Grant` this method sent before `Env` had a real `Kernel::tick` loop to submit into.
        /// `Backpressure` is intentionally not surfaced to the caller: every round this bench drives
        /// sends at most a handful of envelopes per actor, far under any lane's mailbox capacity
        /// (128-1024 depending on lane, `lane_defaults::budget_for`), so treating a reject as fatal
        /// here would be testing the mailbox ceiling, not the budget this harness measures.
        fn send_payload(&mut self, actor: ActorId, payload: Payload) {
            self.send_payload_lane(actor, payload, Lane::Background);
        }

        /// 🎯️ terra-bench-instrument: sibling of `send_payload` that lets a caller pick the
        /// envelope's own `Lane` instead of the hardcoded `Lane::Background` every other send in
        /// this harness still uses (`send_payload` now delegates here with `Lane::Background`
        /// unchanged, so every existing call site — budgets 2/3/4/6/7/8's `env.send`, budget 7's
        /// direct `Payload::Suspend`/`Resume` sends — keeps the exact envelope it always sent).
        /// Budget 5 is the ONLY caller that passes `Lane::Interactive`, for the one envelope this
        /// bench ever sends that is meant to model a real interactive command — see that budget's
        /// own round loop for why: the instrument was found to send EVERY bench envelope, including
        /// the "interactive" probe, on `Lane::Background`, which both skips whatever lane-priority
        /// the mailbox/DRR machinery gives `Lane::Interactive` and structurally cannot activate the
        /// terra-interactive-isolation packet's `Kernel::activate`-time placement gate (that gate
        /// reads the ACTOR's own activation lane, set once in `Env::activate` above — unconditionally
        /// `Lane::Background` for every bench actor, out of scope here since `Env::activate` is
        /// shared by every budget, not just 5 — so fixing only this envelope's lane does not, by
        /// itself, make that isolation mechanism reachable from this bench; see this packet's own
        /// report for the honest gap).
        fn send_payload_lane(&mut self, actor: ActorId, payload: Payload, lane: Lane) {
            self.seq += 1;
            let envelope = Envelope { to: actor, from: Origin::Kernel, lane, seq: self.seq, deadline_ms: None, coalesce: None, cancel_of: None, payload };
            let _ = self.runtime.submit(&envelope);
        }

        /// ⏱️ terra-kernel-loop: `Kernel::tick`-drives every actor with a non-empty mailbox to
        /// completion — looping `tick_and_dispatch` until a tick grants nothing (`grants_per_tick`
        /// caps a SINGLE tick's grants at 64, so draining >64 pending actors, e.g. budget 3/4/5's
        /// 100-2550-actor rounds, genuinely takes several ticks; this loop is what makes that real
        /// instead of assuming one call suffices). Each tick's `ShardOutcome`s are awaited via
        /// `wait_for_outcomes` — a genuine blocking wait on the SAME aggregated channel K real
        /// `ShardExecutor` threads report through. `Kernel::complete` is called for every
        /// `ShardOutcome::Turn` collected, closing the gap budget 8's own note used to flag.
        ///
        /// 🎯️ terra-bench-instrument correction: this method's own `start.elapsed()`, timed by a
        /// caller around a WHOLE call, is round wall-time across every actor granted that round —
        /// budget 5 used to time itself this way and that is exactly the defect this packet fixed;
        /// budget 5 now uses `pump_tracking` below instead, which stamps the moment ONE specific
        /// actor's own outcome is observed rather than waiting on this method's own return.
        fn pump(&mut self) -> Result<usize, String> {
            let mut total = 0usize;
            loop {
                self.now_ms += 1;
                // 🔀️ Cloned BEFORE the call (a small `HashMap<u64, TurnBudget>`, one per activated
                // actor) so the closure below borrows THIS local binding, not `self` — `self.runtime.
                // tick_and_dispatch(..)` already holds `self.runtime` mutably for the duration of the
                // call, and a closure capturing `&self.budgets` directly would conflict with that.
                let budgets = self.budgets.clone();
                let fallback = TurnBudget { fuel: BENCH_FUEL, deadline_ms: 50, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
                let decision = self.runtime.tick_and_dispatch(self.now_ms, |actor| crate::actor_budget_from_turn_budget(budgets.get(&actor.0).copied().unwrap_or(fallback), Lane::Background));
                if decision.run.is_empty() {
                    break;
                }
                let outcomes = self.runtime.wait_for_outcomes(decision.run.len(), PUMP_OUTCOME_TIMEOUT);
                if outcomes.len() < decision.run.len() {
                    let missing = decision.run.len() - outcomes.len();
                    self.pending.extend(outcomes);
                    return Err(format!("Env::pump: {missing} of {} granted turns produced no ShardOutcome within {PUMP_OUTCOME_TIMEOUT:?}", decision.run.len()));
                }
                for outcome in &outcomes {
                    match outcome {
                        ShardOutcome::Turn { actor, result } => {
                            let _ = self.runtime.complete(ActorId(*actor), result, 0, 0, self.now_ms);
                        }
                        // 🎠️ terra-kernel-loop: same reasoning as `kernel_runtime::run_turn`'s own
                        // `ShardOutcome::Fault` arm — a trap must reach `Kernel::complete` too, or the
                        // failure ladder never sees the SAME "hang"/"crash" profiles budgets 2/3/6
                        // deliberately exercise.
                        ShardOutcome::Fault { actor, message } => {
                            let faulted = TurnResult { ui_patches: Vec::new(), effects: Vec::new(), next_wake: None, status: semio_framework::kernel::TurnStatus::Faulted(message.clone().into_bytes()), fuel_used: 0 };
                            let _ = self.runtime.complete(ActorId(*actor), &faulted, 0, 0, self.now_ms);
                        }
                        _ => {}
                    }
                }
                total += outcomes.len();
                self.pending.extend(outcomes);
            }
            Ok(total)
        }

        /// 🎯️ terra-bench-instrument: same `Kernel::tick`-drives-every-granted-actor-to-completion
        /// shape as `pump` above — every actor granted this round, `target` included, still gets a
        /// genuine `tick_and_dispatch` → `wait_for_outcomes` → `Kernel::complete` round trip, so the
        /// kernel's own bookkeeping (fuel/throttle/mailbox state) ends this call exactly as
        /// consistent as `pump` would leave it, and the next round starts clean. The ONLY behavioural
        /// difference: `pump`'s own `wait_for_outcomes(decision.run.len(), ..)` blocks for a WHOLE
        /// tick's outcomes at once, so nothing is observable until the SLOWEST of that tick's actors
        /// has reported in. This method instead waits one outcome at a time
        /// (`wait_for_outcomes(1, ..)`) — the same total outcomes arrive, just individually — and
        /// stamps `Instant::now()` the first moment `target`'s own `ShardOutcome` (`Turn` or `Fault`,
        /// whichever arrives) is among them. That stamp, not this call's own return, is budget 5's
        /// actual measurement: see its round loop for why the interval is `send -> this stamp`, not
        /// `send -> this call returning`.
        fn pump_tracking(&mut self, target: ActorId) -> Result<Option<Instant>, String> {
            let mut target_seen: Option<Instant> = None;
            loop {
                self.now_ms += 1;
                let budgets = self.budgets.clone();
                let fallback = TurnBudget { fuel: BENCH_FUEL, deadline_ms: 50, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
                let decision = self.runtime.tick_and_dispatch(self.now_ms, |actor| crate::actor_budget_from_turn_budget(budgets.get(&actor.0).copied().unwrap_or(fallback), Lane::Background));
                if decision.run.is_empty() {
                    break;
                }
                let mut remaining = decision.run.len();
                while remaining > 0 {
                    let outcomes = self.runtime.wait_for_outcomes(1, PUMP_OUTCOME_TIMEOUT);
                    if outcomes.is_empty() {
                        return Err(format!("Env::pump_tracking: {remaining} granted turns produced no ShardOutcome within {PUMP_OUTCOME_TIMEOUT:?}"));
                    }
                    remaining = remaining.saturating_sub(outcomes.len());
                    for outcome in &outcomes {
                        let reporting_actor = match outcome {
                            ShardOutcome::Turn { actor, result } => {
                                let _ = self.runtime.complete(ActorId(*actor), result, 0, 0, self.now_ms);
                                Some(*actor)
                            }
                            ShardOutcome::Fault { actor, message } => {
                                let faulted = TurnResult { ui_patches: Vec::new(), effects: Vec::new(), next_wake: None, status: semio_framework::kernel::TurnStatus::Faulted(message.clone().into_bytes()), fuel_used: 0 };
                                let _ = self.runtime.complete(ActorId(*actor), &faulted, 0, 0, self.now_ms);
                                Some(*actor)
                            }
                            _ => None,
                        };
                        if target_seen.is_none() && reporting_actor == Some(target.0) {
                            target_seen = Some(Instant::now());
                        }
                    }
                    self.pending.extend(outcomes);
                }
            }
            Ok(target_seen)
        }

        fn drain(&mut self) -> Vec<ShardOutcome> {
            self.pending.extend(self.runtime.try_recv_outcomes());
            std::mem::take(&mut self.pending)
        }

        fn unregister(&mut self, actor: ActorId) {
            self.runtime.unregister(actor);
        }
    }
    //#endregion 🔖️Env

    fn process_rss_bytes() -> Option<u64> {
        let pid = std::process::id().to_string();
        let output = std::process::Command::new("ps").args(["-o", "rss=", "-p", &pid]).output().ok()?;
        if !output.status.success() {
            return None;
        }
        let text = String::from_utf8_lossy(&output.stdout);
        text.trim().parse::<u64>().ok().map(|kb| kb * 1024)
    }

    //#region 🔖️Budget2ColdBoot
    fn budget_2_cold_boot(process_start: Instant, runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord], shard_count: u16, native_budget_ms: u64) -> serde_json::Value {
        let startup: Vec<&RegistryRecord> = records.iter().filter(|r| is_startup(r)).collect();
        if startup.is_empty() {
            return skipped(2, "cold boot to first interactive frame, only on-startup-finished actors live", "registry carries no on-startup-finished record");
        }
        let mut env = Env::new(runtime.clone(), shard_count);
        let mut actors = Vec::with_capacity(startup.len());
        for (index, record) in startup.iter().enumerate() {
            match env.activate(compiled, record) {
                Ok(actor) => actors.push(actor),
                Err(error) => return row(2, "cold boot to first interactive frame, only on-startup-finished actors live", "fail", json!({ "error": error }), json!({ "nativeMs": native_budget_ms }), "activate/instantiate failed mid cold-boot"),
            }
            env.send(actors[index], &instance_open_event(record, index as u32 + 1));
        }
        if let Err(error) = env.pump() {
            return row(2, "cold boot to first interactive frame, only on-startup-finished actors live", "fail", json!({ "error": error }), json!({ "nativeMs": native_budget_ms }), "ShardLoop::pump failed");
        }
        let outcomes = env.drain();
        let elapsed_ms = process_start.elapsed().as_millis() as u64;
        let faults = unexpected_faults(&outcomes, &actors, &startup);
        let active = env.kernel().metrics().actors;
        let only_startup_live = active as usize == startup.len();
        let pass = faults.is_empty() && only_startup_live && elapsed_ms <= native_budget_ms;
        row(
            2,
            "cold boot to first interactive frame, only on-startup-finished actors live",
            if pass { "pass" } else { "fail" },
            json!({ "elapsedMs": elapsed_ms, "startupActorCount": startup.len(), "activeActorsAfterBoot": active, "faultCount": faults.len(), "faults": faults.iter().take(5).collect::<Vec<_>>() }),
            json!({ "nativeMs": native_budget_ms }),
            "measured from process entry (before engine build/wasm compile) to the last on-startup-finished actor's InstanceOpen turn completing",
        )
    }
    //#endregion 🔖️Budget2ColdBoot

    //#region 🔖️Budget3Activate100
    fn budget_3_activate_100(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord], shard_count: u16) -> serde_json::Value {
        let plugin_records: Vec<&RegistryRecord> = records.iter().filter(|r| r.kind == "plugin").take(50).collect();
        if plugin_records.is_empty() {
            return skipped(3, "activate 50 plugins + 50 extensions of one plugin", "registry carries no plugin-kind record");
        }
        let target_plugin_id = plugin_records[0].id.clone();
        let ext_records: Vec<&RegistryRecord> = records.iter().filter(|r| r.kind == "extension" && r.parent_id.as_deref() == Some(target_plugin_id.as_str())).collect();
        let mut env = Env::new(runtime.clone(), shard_count);
        let mut activated: Vec<ActorId> = Vec::new();
        let selected: Vec<&RegistryRecord> = plugin_records.iter().chain(ext_records.iter()).copied().collect();
        let mut instance_id = 1u32;
        for record in &selected {
            match env.activate(compiled, record) {
                Ok(actor) => {
                    env.send(actor, &instance_open_event(record, instance_id));
                    instance_id += 1;
                    activated.push(actor);
                }
                Err(error) => return row(3, "activate 50 plugins + 50 extensions of one plugin", "fail", json!({ "error": error }), json!({ "activeActors": 100, "shards": shard_count }), "activate/instantiate failed"),
            }
        }
        if let Err(error) = env.pump() {
            return row(3, "activate 50 plugins + 50 extensions of one plugin", "fail", json!({ "error": error }), json!({ "activeActors": 100, "shards": shard_count }), "ShardLoop::pump failed");
        }
        let outcomes = env.drain();
        let faults = unexpected_faults(&outcomes, &activated, &selected).len();
        let active = env.kernel().metrics().actors;
        let mut per_shard: HashMap<u16, u32> = HashMap::new();
        for actor in &activated {
            if let Some(record) = env.kernel().actor_record(*actor) {
                *per_shard.entry(record.shard.0).or_insert(0) += 1;
            }
        }
        let shards_used = env.kernel().metrics().shards;
        let max_shard_load = per_shard.values().copied().max().unwrap_or(0);
        let ceiling = ((activated.len() as f64) / (shard_count.max(1) as f64)).ceil() as u32 + 1;
        let pass = active as usize == 100 && activated.len() == 100 && faults == 0 && shards_used == shard_count as u32 && max_shard_load <= ceiling;
        row(
            3,
            "activate 50 plugins + 50 extensions of one plugin",
            if pass { "pass" } else { "fail" },
            json!({ "activatedCount": activated.len(), "activeActors": active, "shardsConfigured": shard_count, "shardsReported": shards_used, "maxShardLoad": max_shard_load, "shardCeiling": ceiling, "perShardCounts": per_shard, "faultCount": faults }),
            json!({ "activeActors": 100, "shards": shard_count, "maxShardLoadCeiling": "ceil(100/K)+1" }),
            "shard assignment measured via the real Kernel::activate/ShardTable pin — single physical ShardLoop backs all K shard labels for execution",
        )
    }
    //#endregion 🔖️Budget3Activate100

    //#region 🔖️Budget4And5FullScale
    /// 🏋️ Budgets 4 (memory) and 5 (interactive p95 under 40-cpu-actor load) share one fully-activated
    /// registry ("the" 50x50 scale claim) so budget 5 measures real contention against the same live
    /// fleet budget 4 just measured RSS for, instead of paying for a second 2550-instance activation.
    fn budget_4_and_5(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord], shard_count: u16, memory_budget_bytes: u64) -> (serde_json::Value, serde_json::Value) {
        let mut env = Env::new(runtime.clone(), shard_count);
        let mut activated: Vec<(ActorId, String)> = Vec::with_capacity(records.len());
        let mut instance_id = 1u32;
        for record in records {
            match env.activate(compiled, record) {
                Ok(actor) => {
                    env.send(actor, &instance_open_event(record, instance_id));
                    instance_id += 1;
                    activated.push((actor, profile_of(record).to_string()));
                }
                Err(error) => {
                    let fail = row(4, "memory <= K x 512MiB + 256MiB headroom (native RSS <= 1.5GiB)", "fail", json!({ "error": error }), json!({ "maxBytes": memory_budget_bytes }), "activate/instantiate failed mid full-scale run");
                    return (fail, skipped(5, "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background", "budget 4's full-scale activation failed before this could run"));
                }
            }
        }
        if let Err(error) = env.pump() {
            let fail = row(4, "memory <= K x 512MiB + 256MiB headroom (native RSS <= 1.5GiB)", "fail", json!({ "error": error }), json!({ "maxBytes": memory_budget_bytes }), "ShardLoop::pump failed");
            return (fail, skipped(5, "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background", "budget 4's full-scale activation failed before this could run"));
        }
        let outcomes = env.drain();
        let by_design: std::collections::HashSet<u64> =
            activated.iter().zip(records.iter()).filter(|(_, record)| matches!(profile_of(record), "hang" | "crash")).map(|((actor, _), _)| actor.0).collect();
        let faults = outcomes.iter().filter(|o| matches!(o, ShardOutcome::Fault { actor, .. } if !by_design.contains(actor))).count();
        let rss = process_rss_bytes();
        let active = env.kernel().metrics().actors;
        let pass4 = faults == 0 && active as usize == activated.len() && rss.map(|bytes| bytes <= memory_budget_bytes).unwrap_or(false);
        let row4 = row(
            4,
            "memory <= K x 512MiB + 256MiB headroom (native RSS <= 1.5GiB)",
            if rss.is_none() { "skipped" } else if pass4 { "pass" } else { "fail" },
            json!({ "rssBytes": rss, "activatedCount": activated.len(), "activeActors": active, "faultCount": faults }),
            json!({ "maxBytes": memory_budget_bytes }),
            if rss.is_none() { "`ps -o rss=` did not return a value on this host" } else { "RSS sampled once via `ps -o rss= -p <pid>` immediately after all 2550 records were instantiated and given their InstanceOpen turn" },
        );

        // Budget 5 — reuse the live fleet: 40 cpu-profile actors + 1 idle-profile "interactive" actor.
        let cpu_actors: Vec<ActorId> = activated.iter().filter(|(_, profile)| profile == "cpu").take(40).map(|(actor, _)| *actor).collect();
        let interactive_actor = activated.iter().find(|(_, profile)| profile == "idle").map(|(actor, _)| *actor);
        let row5 = match interactive_actor {
            None => skipped(5, "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background", "no idle-profile record to use as the interactive target"),
            Some(_interactive_actor) if cpu_actors.len() < 40 => skipped(5, "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background", &format!("only {} cpu-profile actors in registry, need 40", cpu_actors.len())),
            Some(interactive_actor) => {
                const ROUNDS: usize = 30;
                const NATIVE_BUDGET_MS: f64 = 8.0;
                let mut samples_ms: Vec<f64> = Vec::with_capacity(ROUNDS);
                let mut round_faults = 0usize;
                for _ in 0..ROUNDS {
                    // 🎯️ terra-bench-instrument: the 40 cpu-actor `Wake`s are submitted BEFORE the
                    // clock starts, on `Lane::Background` (`env.send`, unchanged) — they still get
                    // GRANTED in the SAME `Kernel::tick` as the interactive command just below
                    // (`grants_per_tick` comfortably covers 41 single-turn grants), so they are
                    // genuinely running/contending on their own real `ShardExecutor` threads for the
                    // WHOLE measured interval below, which is exactly the "40 cpu actors saturating
                    // the background" load this budget names. They are just not what stops the clock.
                    for actor in &cpu_actors {
                        env.send(*actor, &Event::Wake);
                    }
                    let start = Instant::now();
                    // 🎯️ terra-bench-instrument: the one envelope in this bench that carries
                    // `Lane::Interactive` (`Env::send_payload_lane`) — every other envelope this
                    // harness ever sends, including the 40 `Wake`s above, stays `Lane::Background`.
                    env.send_payload_lane(
                        interactive_actor,
                        Payload::Event { bytes: serde_json::to_vec(&Event::AppCommandEvent { instance: PluginInstanceId(interactive_actor.0.to_string()), seq: 0, command: Vec::new() }).unwrap_or_default() },
                        Lane::Interactive,
                    );
                    // 🎯️ terra-bench-instrument (THE measurement fix): the interval this bench
                    // records is send -> `interactive_actor`'s OWN `ShardOutcome` being observed,
                    // via `Env::pump_tracking`'s `Instant` stamp — NOT the moment `pump_tracking`
                    // itself returns. `pump_tracking` still drives every actor granted this round
                    // (the 40 cpu actors included) all the way to `Kernel::complete`, exactly like
                    // `pump()` does elsewhere in this file, so kernel bookkeeping stays correct for
                    // the next round; those other 40 completions may land AFTER the stamp below and
                    // are deliberately excluded from `samples_ms`. Before this fix, the interval was
                    // `start.elapsed()` taken AFTER `pump()` (bulk-waits for ALL 41 outcomes) had
                    // already returned — i.e. it timed the slowest of 41 actors every round, not this
                    // one actor's own response; see this packet's own report for why that made the
                    // 8ms budget unreachable by construction, independent of scheduler quality.
                    match env.pump_tracking(interactive_actor) {
                        Ok(Some(seen_at)) => samples_ms.push((seen_at - start).as_secs_f64() * 1000.0),
                        Ok(None) => round_faults += 1,
                        Err(_) => round_faults += 1,
                    }
                    let outcomes = env.drain();
                    if outcomes.iter().any(|o| matches!(o, ShardOutcome::Fault { actor, .. } if *actor == interactive_actor.0)) {
                        round_faults += 1;
                    }
                }
                samples_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let p95 = samples_ms.get(((samples_ms.len() as f64) * 0.95).floor() as usize).copied().unwrap_or(f64::NAN);
                let pass = round_faults == 0 && p95 <= NATIVE_BUDGET_MS;
                row(
                    5,
                    "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background",
                    if pass { "pass" } else { "fail" },
                    json!({ "p95Ms": p95, "rounds": ROUNDS, "roundFaults": round_faults, "samplesMs": samples_ms }),
                    json!({ "nativeMs": NATIVE_BUDGET_MS }),
                    "terra-bench-instrument: measured from the interactive command's own submit to THIS actor's own ShardOutcome (Turn or Fault) being observed on its real ShardExecutor thread (Env::pump_tracking), NOT to global quiescence of all 41 actors granted in the round -- the 40 cpu actors keep running/completing in the background across the measured interval, which is the load this budget specifies, they just no longer gate the clock. The interactive envelope also now carries Lane::Interactive (Env::send_payload_lane); every other envelope in this bench, including the 40 cpu Wakes, stays Lane::Background as before. NOT comparable to any p95 recorded before this fix: those measured full-round wall time across all 41 actors, not this actor's own response.",
                )
            }
        };
        (row4, row5)
    }
    //#endregion 🔖️Budget4And5FullScale

    //#region 🔖️Budget6Hang
    fn budget_6_hang(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord]) -> serde_json::Value {
        let Some(hang_record) = records.iter().find(|r| profile_of(r) == "hang") else {
            return skipped(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "no hang-profile record in registry");
        };
        let sibling_records: Vec<&RegistryRecord> = records.iter().filter(|r| profile_of(r) == "idle").take(3).collect();
        if sibling_records.is_empty() {
            return skipped(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "no idle-profile sibling records in registry");
        }
        let mut env = Env::new(runtime.clone(), 1);
        let deadline_ms = hang_record.quotas.deadline_ms;
        let pause_start = Instant::now();
        let hang_actor = match env.activate(compiled, hang_record) {
            Ok(actor) => actor,
            Err(error) => return row(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "fail", json!({ "error": error }), json!(null), "hang actor activate/instantiate failed"),
        };
        env.send(hang_actor, &instance_open_event(hang_record, 1));
        let mut siblings = Vec::new();
        for (index, record) in sibling_records.iter().enumerate() {
            match env.activate(compiled, record) {
                Ok(actor) => {
                    env.send(actor, &instance_open_event(record, index as u32 + 2));
                    siblings.push(actor);
                }
                Err(error) => return row(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "fail", json!({ "error": error }), json!(null), "sibling activate/instantiate failed"),
            }
        }
        if env.pump().is_err() {
            return row(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "fail", json!(null), json!(null), "ShardLoop::pump failed on InstanceOpen phase");
        }
        // 🐛️ `🎭️profile::turn()` runs unconditionally on EVERY `poll`, including `InstanceOpen` (see
        // `guest::FixtureGuest::poll` in this crate's `🦀️component.rs` — it always calls
        // `on_instance_open` THEN `profile::turn`) — the hang profile's overrun busy-loop, and the
        // epoch-interrupt trap it draws, is therefore typically already hit on THIS first turn, not a
        // dedicated follow-up `Wake`. A wasmtime component instance is permanently poisoned after any
        // trap (cannot be re-entered), so a second call into an already-trapped instance correctly
        // fails with "cannot enter component instance" — that message is CONFIRMING evidence of an
        // earlier kill, not a different failure. Checked here first; falls back to an explicit `Wake`
        // only if the InstanceOpen turn happened not to trigger it.
        let open_outcomes = env.drain();
        let hang_fault_on_open = open_outcomes.iter().find_map(|o| match o {
            ShardOutcome::Fault { actor, message } if *actor == hang_actor.0 => Some(message.clone()),
            _ => None,
        });
        let killed_on_open = hang_fault_on_open.is_some();
        let (killed, hang_fault) = if let Some(message) = hang_fault_on_open {
            (true, Some(message))
        } else {
            env.send(hang_actor, &Event::Wake);
            let _ = env.pump();
            let wake_outcomes = env.drain();
            let message = wake_outcomes.iter().find_map(|o| match o {
                ShardOutcome::Fault { actor, message } if *actor == hang_actor.0 => Some(message.clone()),
                _ => None,
            });
            let killed = message.as_deref().map(|m| { let lower = m.to_ascii_lowercase(); lower.contains("deadline") || lower.contains("fuel") || lower.contains("cannot enter") }).unwrap_or(false);
            (killed, message)
        };
        env.unregister(hang_actor);
        for actor in &siblings {
            env.send(*actor, &Event::Wake);
        }
        let siblings_pumped = env.pump().is_ok();
        let sibling_outcomes = env.drain();
        let siblings_ok = siblings.iter().all(|actor| sibling_outcomes.iter().any(|o| matches!(o, ShardOutcome::Turn { actor: a, .. } if *a == actor.0)));
        let pause_ms = pause_start.elapsed().as_millis() as u64;
        let pass = killed && siblings_pumped && siblings_ok && pause_ms <= 250;
        row(
            6,
            "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms",
            if pass { "pass" } else { "fail" },
            json!({ "declaredDeadlineMs": deadline_ms, "faultMessage": hang_fault, "killed": killed, "killedOnInstanceOpenTurn": killed_on_open, "siblingCount": siblings.len(), "siblingsRestored": siblings_ok, "totalPauseMs": pause_ms }),
            json!({ "killWithinMs": 2 * deadline_ms, "totalPauseMs": 250 }),
            "\"shard rebuilt\" is approximated as unregister+drop of the faulted GuestInstance on the same physical ShardLoop, then a successful next turn for its siblings — no separate OS thread is torn down/recreated in this single-shard-loop harness. Pause is measured from activation, since the hang overrun typically fires on the InstanceOpen turn itself (see note above), not a dedicated follow-up turn.",
        )
    }
    //#endregion 🔖️Budget6Hang

    //#region 🔖️Budget7Stateful
    /// 📸️ K1 landed mid-session (design-workforce.md's own blocker note is now stale): `ShardLoop::
    /// pump` genuinely dispatches `Payload::Suspend{checkpoint:true}` -> `GuestRuntime::checkpoint` ->
    /// `ShardOutcome::Checkpoint` and `Payload::Resume{checkpoint:Some(state)}` -> `GuestRuntime::
    /// restore` -> `ShardOutcome::Resumed`. This measures THAT real dispatch path, not a direct
    /// bypass call — suspend actor A (captures checkpoint bytes), drop A's instance (the "evicted"
    /// half of LRU-suspend), resume a FRESH instance B from those bytes (the "resumed elsewhere"
    /// half), then re-checkpoint B and compare bytes to the original. The LRU eviction TRIGGER itself
    /// (the policy deciding WHEN to suspend) is still not exercised — only the suspend/resume/
    /// checkpoint wire path K1 unblocked.
    const BUDGET_7_DESCRIPTION: &str = "stateful actor LRU-suspended and resumed -> identical state hash";

    fn budget_7_stateful(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord]) -> serde_json::Value {
        let Some(record) = records.iter().find(|r| profile_of(r) == "stateful") else {
            return skipped(7, BUDGET_7_DESCRIPTION, "no stateful-profile record in registry");
        };
        let mut env = Env::new(runtime.clone(), 1);
        let actor_a = match env.activate(compiled, record) {
            Ok(actor) => actor,
            Err(error) => return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "error": error }), json!(null), "activate/instantiate failed"),
        };
        env.send(actor_a, &instance_open_event(record, 1));
        for _ in 0..5 {
            env.send(actor_a, &Event::Wake);
        }
        if env.pump().is_err() {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!(null), json!(null), "pump failed while accumulating state");
        }
        env.drain();

        env.send_payload(actor_a, Payload::Suspend { checkpoint: true });
        if env.pump().is_err() {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!(null), json!(null), "pump failed on Suspend");
        }
        let suspend_outcomes = env.drain();
        let Some(state) = suspend_outcomes.iter().find_map(|o| match o {
            ShardOutcome::Checkpoint { actor, state } if *actor == actor_a.0 => Some(state.clone()),
            _ => None,
        }) else {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "outcomes": format!("{suspend_outcomes:?}") }), json!(null), "no ShardOutcome::Checkpoint for Suspend");
        };

        // The "evicted" half of LRU-suspend: drop A's live instance from this shard.
        env.unregister(actor_a);

        // The "resumed elsewhere" half: a FRESH instance, resumed from the captured checkpoint bytes.
        let actor_b = match env.activate(compiled, record) {
            Ok(actor) => actor,
            Err(error) => return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "error": error }), json!(null), "re-activate/instantiate failed"),
        };
        env.send_payload(actor_b, Payload::Resume { checkpoint: Some(state.clone()) });
        if env.pump().is_err() {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!(null), json!(null), "pump failed on Resume");
        }
        let resume_outcomes = env.drain();
        let resumed = resume_outcomes.iter().any(|o| matches!(o, ShardOutcome::Resumed { actor } if *actor == actor_b.0));

        env.send_payload(actor_b, Payload::Suspend { checkpoint: true });
        if env.pump().is_err() {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "resumed": resumed }), json!(null), "pump failed on post-resume re-Suspend");
        }
        let recheck_outcomes = env.drain();
        let Some(state_after_resume) = recheck_outcomes.iter().find_map(|o| match o {
            ShardOutcome::Checkpoint { actor, state } if *actor == actor_b.0 => Some(state.clone()),
            _ => None,
        }) else {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "resumed": resumed }), json!(null), "no ShardOutcome::Checkpoint after resume");
        };
        let identical = state == state_after_resume;
        let pass = resumed && identical;
        row(
            7,
            BUDGET_7_DESCRIPTION,
            if pass { "pass" } else { "fail" },
            json!({ "resumed": resumed, "checkpointHash": blake3::hash(&state).to_hex().to_string(), "resumedCheckpointHash": blake3::hash(&state_after_resume).to_hex().to_string(), "identical": identical }),
            json!("Resumed outcome received and identical checkpoint bytes before suspend vs. after resume+re-checkpoint"),
            "measured through the REAL production dispatch path (K1, unblocked mid-session): ShardLoop::pump's Payload::Suspend/Resume -> GuestRuntime::checkpoint/restore -> ShardOutcome::Checkpoint/Resumed. The LRU-eviction TRIGGER (the policy deciding WHEN to suspend) is still not exercised here — this proves the suspend/resume/checkpoint wire path end-to-end, which is exactly what was blocked before K1 landed.",
        )
    }
    //#endregion 🔖️Budget7Stateful

    //#region 🔖️Budget8CapabilityRevoke
    fn budget_8_capability_revoke(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord]) -> serde_json::Value {
        let Some(record) = records.iter().find(|r| profile_of(r) == "io") else {
            return skipped(8, "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero", "no io-profile record in registry");
        };
        let cap_id = record.scale_fixture.get("ioCapabilityId").and_then(|v| v.as_str()).unwrap_or("scale-fixture.io").to_string();
        let budget = turn_budget_of(record);
        let actor = ActorId(0xB8_0000_0001);
        let mut inst = match runtime.instantiate(compiled, actor, &[], &budget) {
            Ok(instance) => instance,
            Err(error) => return row(8, "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero", "fail", json!({ "error": error.to_string() }), json!(null), "instantiate failed"),
        };
        // 🐛️ `🎭️profile::turn()` runs unconditionally on EVERY `poll` (see budget 6's identical note) —
        // the `io` profile's ONE-TIME `RequestCapability` effect is therefore typically emitted on
        // THIS very first `InstanceOpen` turn, not a dedicated follow-up. Checked on both turns so a
        // real request is never misread as absent just because it landed on turn 1.
        let open_result = match semio_framework_plugin_host::poll_ready(runtime.execute_turn(&mut inst, &[instance_open_event(record, 1)], budget)) {
            Ok(result) => result,
            Err(fault) => return row(8, "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero", "fail", json!({ "error": fault.to_string() }), json!(null), "InstanceOpen turn failed"),
        };
        let requested_on_open = open_result.effects.iter().any(|effect| matches!(effect, Effect::RequestCapability { capability, .. } if capability.id.0 == cap_id));
        let requested_on_wake = match semio_framework_plugin_host::poll_ready(runtime.execute_turn(&mut inst, &[Event::Wake], budget)) {
            Ok(result) => result.effects.iter().any(|effect| matches!(effect, Effect::RequestCapability { capability, .. } if capability.id.0 == cap_id)),
            Err(fault) => return row(8, "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero", "fail", json!({ "error": fault.to_string() }), json!(null), "capability-request turn failed"),
        };
        let requested = requested_on_open || requested_on_wake;
        let revoke_event = Event::CapabilityChanged { change: CapabilityChange::Revoked { id: CapabilityId(cap_id.clone()) } };
        let revoke_result = semio_framework_plugin_host::poll_ready(runtime.execute_turn(&mut inst, &[revoke_event], budget));
        let survived_revoke = revoke_result.is_ok();
        let revoke_status = match &revoke_result {
            Ok(result) => format!("{:?}", result.status),
            Err(fault) => fault.to_string(),
        };
        let followup = semio_framework_plugin_host::poll_ready(runtime.execute_turn(&mut inst, &[Event::Wake], budget));
        let survived_followup = followup.is_ok();
        let pass = requested && survived_revoke && survived_followup;
        row(
            8,
            "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero",
            if pass { "pass" } else { "fail" },
            json!({ "capabilityId": cap_id, "capabilityRequested": requested, "requestedOnInstanceOpenTurn": requested_on_open, "survivedRevokeTurn": survived_revoke, "statusAfterRevoke": revoke_status, "survivedFollowupTurn": survived_followup }),
            json!("no trap across or after the revoke turn"),
            "\"quota counters zero\" is read here as \"no TurnFault (fuel/deadline/trap) recorded across the revoke turn\": Kernel::complete() (the only path that updates Kernel-level ActorMetrics/ActorStatus) is never called by this harness — same documented gap as the production kernel_runtime module above — so the kernel's own quota counters cannot be read from here.",
        )
    }
    //#endregion 🔖️Budget8CapabilityRevoke

    /// ▶️ `--scale <registry.json> --scale-wasm <fixture.wasm> --shards <K> --report <out.json>`.
    /// Runs budgets 2-8 (budget 1 — registry parse timing — is measured JS-side, no wasm involved) and
    /// writes one JSON report. Returns `0` on a clean harness run (regardless of individual budget
    /// pass/fail — a real measured FAIL is a valid, non-error outcome), `1` if the harness itself could
    /// not set up (bad registry/wasm/report path).
    pub fn run(registry_path: PathBuf, wasm_path: PathBuf, shard_count: u16, report_path: PathBuf) -> i32 {
        let process_start = Instant::now();
        let registry_bytes = match std::fs::read(&registry_path) {
            Ok(bytes) => bytes,
            Err(error) => {
                eprintln!("scale-bench: failed to read {}: {error}", registry_path.display());
                return 1;
            }
        };
        let registry: RegistryFile = match serde_json::from_slice(&registry_bytes) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("scale-bench: failed to parse {}: {error}", registry_path.display());
                return 1;
            }
        };
        let wasm_bytes = match std::fs::read(&wasm_path) {
            Ok(bytes) => bytes,
            Err(error) => {
                eprintln!("scale-bench: failed to read {}: {error}", wasm_path.display());
                return 1;
            }
        };
        let runtime: Arc<GuestRuntimes> = match WasmtimeRuntime::new(SharedEngineConfig::default()) {
            Ok(rt) => Arc::new(GuestRuntimes::Wasmtime(rt)),
            Err(error) => {
                eprintln!("scale-bench: engine build failed: {error}");
                return 1;
            }
        };
        let package_ref = PackageRef { package: PackageId("scale-fixture".to_string()), hash: PackageHash(*blake3::hash(&wasm_bytes).as_bytes()) };
        let compiled = match runtime.compile(&package_ref, &wasm_bytes) {
            Ok(handle) => handle,
            Err(error) => {
                eprintln!("scale-bench: compile failed: {error}");
                return 1;
            }
        };

        let row_2 = budget_2_cold_boot(process_start, &runtime, &compiled, &registry.records, shard_count, 1500);
        let row_3 = budget_3_activate_100(&runtime, &compiled, &registry.records, shard_count);
        let (row_4, row_5) = budget_4_and_5(&runtime, &compiled, &registry.records, shard_count, shard_count as u64 * 512 * 1024 * 1024 + 256 * 1024 * 1024);
        let row_6 = budget_6_hang(&runtime, &compiled, &registry.records);
        let row_7 = budget_7_stateful(&runtime, &compiled, &registry.records);
        let row_8 = budget_8_capability_revoke(&runtime, &compiled, &registry.records);

        let report = json!({
            "renderer": "native",
            "shardCount": shard_count,
            "recordCount": registry.records.len(),
            "wasmPath": wasm_path.display().to_string(),
            "budgets": [row_2, row_3, row_4, row_5, row_6, row_7, row_8],
        });
        if let Some(parent) = report_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        match serde_json::to_string_pretty(&report) {
            Ok(text) => {
                if let Err(error) = std::fs::write(&report_path, text) {
                    eprintln!("scale-bench: failed to write {}: {error}", report_path.display());
                    return 1;
                }
            }
            Err(error) => {
                eprintln!("scale-bench: report encode failed: {error}");
                return 1;
            }
        }
        println!("scale-bench: wrote {}", report_path.display());
        0
    }
}
//#endregion 🔖️ScaleBench

fn spawn_app_task<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    #[cfg(target_arch = "wasm32")]
    spawn_local(future);
    #[cfg(not(target_arch = "wasm32"))]
    kernel_runtime::spawn_task(future);
}

#[cfg(target_arch = "wasm32")]
fn log_debug(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}

#[cfg(not(target_arch = "wasm32"))]
fn log_debug(message: &str) {
    eprintln!("{message}");
}

#[cfg(target_arch = "wasm32")]
fn prefers_dark_scheme() -> bool {
    web_sys::window().and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok().flatten()).map(|query| query.matches()).unwrap_or(true)
}

#[cfg(not(target_arch = "wasm32"))]
fn prefers_dark_scheme() -> bool {
    true
}

fn resolve_theme(appearance_id: &str) -> Theme {
    match appearance_id {
        "light" => Theme::light(),
        "dark" => Theme::dark(),
        _ if prefers_dark_scheme() => Theme::dark(),
        _ => Theme::light(),
    }
}

fn appearance_is_dark(appearance_id: &str) -> bool {
    match appearance_id {
        "light" => false,
        "dark" => true,
        _ => prefers_dark_scheme(),
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn app_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn app_now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_secs_f64() * 1000.0).unwrap_or(0.0)
}

/// 🕒️ Pure sweep for a per-surface "pending camera dispatch" deadline map (`wheel_zoom_deadline_ms`'s
/// single-surface precedent above, generalized to many surfaces at once): returns the surface ids
/// whose deadline is at-or-past `now_ms`, removing them from `pending` — callers build+dispatch each
/// surface's `setCamera` action from whatever per-surface state it still needs to look up. Kept
/// free of any `AppRuntime`/`ShellState` coupling so it's testable with a bare `HashMap` + timestamp.
pub(crate) fn sweep_expired_camera_dispatch_deadlines(pending: &mut std::collections::HashMap<String, f64>, now_ms: f64) -> Vec<String> {
    let expired: Vec<String> = pending.iter().filter(|(_, deadline)| now_ms >= **deadline).map(|(surface_id, _)| surface_id.clone()).collect();
    for surface_id in &expired {
        pending.remove(surface_id);
    }
    expired
}

#[cfg(test)]
mod camera_dispatch_deadline_tests {
    use super::*;

    #[test]
    fn not_yet_expired_deadline_is_left_pending() {
        let mut pending = std::collections::HashMap::from([("s1".to_string(), 1_000.0)]);
        let expired = sweep_expired_camera_dispatch_deadlines(&mut pending, 999.0);
        assert!(expired.is_empty());
        assert_eq!(pending.get("s1"), Some(&1_000.0));
    }

    #[test]
    fn deadline_exactly_at_now_is_expired() {
        let mut pending = std::collections::HashMap::from([("s1".to_string(), 1_000.0)]);
        let expired = sweep_expired_camera_dispatch_deadlines(&mut pending, 1_000.0);
        assert_eq!(expired, vec!["s1".to_string()]);
        assert!(pending.is_empty(), "an expired surface is removed from the map");
    }

    #[test]
    fn already_expired_deadline_is_swept() {
        let mut pending = std::collections::HashMap::from([("s1".to_string(), 500.0)]);
        let expired = sweep_expired_camera_dispatch_deadlines(&mut pending, 1_000.0);
        assert_eq!(expired, vec!["s1".to_string()]);
        assert!(pending.is_empty());
    }

    #[test]
    fn multiple_surfaces_expire_independently() {
        let mut pending = std::collections::HashMap::from([("expired-a".to_string(), 100.0), ("expired-b".to_string(), 200.0), ("still-pending".to_string(), 5_000.0)]);
        let mut expired = sweep_expired_camera_dispatch_deadlines(&mut pending, 1_000.0);
        expired.sort();
        assert_eq!(expired, vec!["expired-a".to_string(), "expired-b".to_string()]);
        assert_eq!(pending.len(), 1, "only the still-pending surface remains");
        assert!(pending.contains_key("still-pending"));
    }
}

struct AppRuntime {
    gpu: GpuContext,
    atlas: FontAtlas,
    icons: IconAtlas,
    shell: ShellState,
    draw: DrawList,
    overlay: DrawList,
    input: InputState<ActionDescriptor>,
    theme: Theme,
    window: Arc<Window>,
    theme_dark: bool,
    last_cursor: Option<(SemioCursor, bool)>,
    last_pointer_x: f32,
    last_pointer_y: f32,
    pointer_down: bool,
    pointer_button: i16,
    modifiers: PointerModifiers,
    wheel_delta: f32,
    space_pressed: bool,
    wheel_zoom_deadline_ms: f64,
    /// 🕒️ World3D wheel-zoom's settle-then-dispatch: surface id -> the timestamp its debounced
    /// `setCamera` should fire at, swept every `frame()` by `sweep_expired_camera_dispatch_deadlines`.
    /// The pointer-release path dispatches immediately instead and clears its surface's entry here
    /// first, so a wheel gesture immediately followed by a release-orbit never double-dispatches.
    world3d_camera_dispatch_deadlines_ms: std::collections::HashMap<String, f64>,
    caret_blink_at_ms: f64,
    caret_blink_visible: bool,
    asset_poll_pending: bool,
    self_weak: std::rc::Weak<RefCell<AppRuntime>>,
    #[cfg(not(target_arch = "wasm32"))]
    plugin_modules_root: std::path::PathBuf,
    #[cfg(not(target_arch = "wasm32"))]
    native_plugin_mtimes: std::collections::HashMap<std::path::PathBuf, std::time::SystemTime>,
    #[cfg(not(target_arch = "wasm32"))]
    native_reload_pending: bool,
}

#[cfg(not(target_arch = "wasm32"))]
fn fetch_map_tile_bytes_blocking(url: &str) -> Option<Vec<u8>> {
    let resolved = resolve_map_tile_fetch_url(url);
    if !resolved.starts_with("http://") && !resolved.starts_with("https://") {
        return None;
    }
    let response = ureq::get(&resolved).call().ok()?;
    let mut bytes = Vec::new();
    response.into_reader().read_to_end(&mut bytes).ok()?;
    Some(bytes)
}

#[cfg(not(target_arch = "wasm32"))]
fn resolve_asset_fetch_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }
    if url.starts_with('/') {
        let base = std::env::var("SEMIO_ASSET_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:6141".to_string());
        return format!("{}{}", base.trim_end_matches('/'), url);
    }
    url.to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn resolve_map_tile_fetch_url(url: &str) -> String {
    resolve_asset_fetch_url(url)
}

#[cfg(target_arch = "wasm32")]
fn fetch_map_tile_bytes_blocking(_url: &str) -> Option<Vec<u8>> {
    None
}

impl AppRuntime {
    #[cfg(not(target_arch = "wasm32"))]
    fn poll_native_plugin_hot_swap(&mut self) {
        let mut changed = false;
        for program in &self.shell.plugins {
            let Some(path) = program.wasm_artifact_path() else {
                continue;
            };
            let Ok(metadata) = std::fs::metadata(path) else {
                continue;
            };
            let Ok(mtime) = metadata.modified() else {
                continue;
            };
            let previous = self.native_plugin_mtimes.get(path);
            if previous.is_some_and(|previous| *previous != mtime) {
                changed = true;
            }
            self.native_plugin_mtimes.insert(path.to_path_buf(), mtime);
        }
        if changed {
            self.native_reload_pending = true;
        }
    }

    /// 🎠️ H3-wgpu-native / terra-shell-unpark — this used to `pollster::block_on(self.shell.boot())`
    /// directly on the winit thread. The H3 comment this replaces reasoned that `frame()` already
    /// holds `self` via `Rc<RefCell<AppRuntime>>`'s `try_borrow_mut()`, so re-borrowing that SAME
    /// cell from INSIDE `frame()`'s own call stack panics rather than working — true, but that only
    /// rules out borrowing synchronously; it does not rule out deferring the whole call. This now
    /// uses the identical `self_weak`/`try_borrow_mut()`-held-across-`.await` pattern `on_context_menu`/
    /// the camera-dispatch closures below already use: `spawn_app_task` queues the future, and it
    /// only actually re-borrows from `about_to_wait`'s `poll_tasks()` tick — strictly AFTER `frame()`
    /// has already returned and dropped its own borrow, so there is no re-entrant conflict. Holding
    /// the borrow across `.boot()`'s `.await` makes `frame()`'s outer `try_borrow_mut()` fail (and
    /// skip redrawing) for however many ticks the reload's kernel round trip takes — a graceful
    /// frame-skip, not a UI-thread park; the winit event loop keeps pumping OS messages the whole
    /// time. See `📓️terra-shell-unpark-report.md`.
    #[cfg(not(target_arch = "wasm32"))]
    fn maybe_reload_native_plugins(&mut self) {
        if !self.native_reload_pending {
            return;
        }
        self.native_reload_pending = false;
        let plugin_filter = self.shell.plugin_filter.clone();
        let modules_root = self.plugin_modules_root.clone();
        let entries = match load_wasm_plugins(&plugin_filter, &modules_root) {
            Ok(entries) => filter_plugins(entries, &plugin_filter),
            Err(error) => {
                log_debug(&format!("wasm program reload failed: {error}"));
                return;
            }
        };
        self.shell.prepare_hot_reload(entries);
        let runtime = self.self_weak.clone();
        spawn_app_task(async move {
            let Some(runtime) = runtime.upgrade() else { return };
            let Ok(mut app) = runtime.try_borrow_mut() else { return };
            if let Err(error) = app.shell.boot().await {
                log_debug(&format!("wasm program hot reload failed: {error}"));
            } else {
                log_debug("wasm program hot reload complete");
            }
        });
    }

    fn frame(&mut self) {
        if std::mem::take(&mut self.shell.fullscreen_toggle_requested) {
            #[cfg(not(target_arch = "wasm32"))]
            {
                let active = self.window.fullscreen().is_none();
                self.window.set_fullscreen(if active { Some(Fullscreen::Borderless(None)) } else { None });
                self.shell.fullscreen_active = active;
            }
            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowExtWebSys;
                if let Some(canvas) = self.window.canvas() {
                    let document = canvas.owner_document();
                    let active = document.as_ref().is_some_and(|document| document.fullscreen_element().is_some());
                    if active {
                        if let Some(document) = document {
                            document.exit_fullscreen();
                        }
                        self.shell.fullscreen_active = false;
                    } else {
                        match canvas.request_fullscreen() {
                            Ok(()) => self.shell.fullscreen_active = true,
                            Err(error) => web_sys::console::error_2(&"Fullscreen request was rejected".into(), &error),
                        }
                    }
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.poll_native_plugin_hot_swap();
            self.maybe_reload_native_plugins();
            // 🎠️ terra-shell-unpark — same `self_weak`/`try_borrow_mut()`-held-across-`.await`
            // deferral as `maybe_reload_native_plugins` above (see its doc comment for why this is
            // sound despite `frame()` itself running inside a borrow): `pump_sync_events` no longer
            // runs to completion before the rest of `frame()` continues below, it resumes from
            // `about_to_wait`'s `poll_tasks()` tick. One tick of staleness on directory/sync events is
            // invisible at `ControlFlow::Poll`'s frame rate. See `📓️terra-shell-unpark-report.md`.
            let runtime = self.self_weak.clone();
            spawn_app_task(async move {
                let Some(runtime) = runtime.upgrade() else { return };
                let Ok(mut app) = runtime.try_borrow_mut() else { return };
                app.shell.pump_sync_events().await;
            });
        }
        self.theme = shell::resolve_theme_for_ids(&shell::active_theme_id(), &self.shell.appearance_id);
        self.theme_dark = appearance_is_dark(&self.shell.appearance_id);
        if !self.pointer_down && self.input.drag.active {
            self.input.end_drag();
        }
        self.input.update_hover(self.last_pointer_x, self.last_pointer_y);
        self.input.clear_frame();
        if self.wheel_zoom_deadline_ms > 0.0 && app_now_ms() >= self.wheel_zoom_deadline_ms {
            self.wheel_zoom_deadline_ms = 0.0;
            engine_canvas::node_graph_clear_wheel_zoom_active();
        }
        // 🕒️ World3D wheel-zoom's settled `setCamera` dispatch — see `world3d_camera_dispatch_deadlines_ms`'s
        // own doc comment; each surface's expiry fires exactly once per settle, same as the graph/map/
        // board wheel-action dispatches just below reuse `spawn_app_task` for their own async hop.
        let expired_world3d_surfaces = sweep_expired_camera_dispatch_deadlines(&mut self.world3d_camera_dispatch_deadlines_ms, app_now_ms());
        if !expired_world3d_surfaces.is_empty() {
            let camera_actions: Vec<ActionDescriptor> = expired_world3d_surfaces.iter().filter_map(|surface_id| self.shell.world3d_states.get(surface_id).map(orbit_camera_action)).collect();
            if !camera_actions.is_empty() {
                let runtime = self.self_weak.clone();
                spawn_app_task(async move {
                    if let Some(runtime) = runtime.upgrade() {
                        if let Ok(mut app) = runtime.try_borrow_mut() {
                            app.dispatch_actions(camera_actions).await;
                        }
                    }
                });
            }
        }
        let scene_camera_actions = scenes::sweep_expired_scene_camera_dispatches(app_now_ms());
        if !scene_camera_actions.is_empty() {
            let runtime = self.self_weak.clone();
            spawn_app_task(async move {
                if let Some(runtime) = runtime.upgrade() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.dispatch_actions(scene_camera_actions).await;
                    }
                }
            });
        }
        if app_now_ms() - self.caret_blink_at_ms >= 500.0 {
            self.caret_blink_at_ms = app_now_ms();
            self.caret_blink_visible = !self.caret_blink_visible;
            engine_canvas::node_graph_sync_caret_blink(self.caret_blink_visible);
        }
        self.draw.clear();
        self.overlay.clear();
        ICON_ATLAS_RUNTIME.with(|cell| {
            if let Some(atlas) = cell.borrow_mut().take() {
                self.icons = atlas;
                self.gpu.upload_icon_atlas(&self.icons);
            }
        });
        // 🎬️ Tutorial tick — advances the playhead/recorder and applies UI/camera synchronously; any
        // resulting document-track operations are queued onto `shell.tutorial_pending_document_ops` and
        // flushed asynchronously below (the plugin bridge's document calls are async, chrome rendering
        // isn't — same reason `scene_events` gets deferred through `spawn_app_task` just after).
        self.shell.tutorial_tick(app_now_ms());
        self.shell.render_chrome(&mut self.draw, &mut self.overlay, &mut self.atlas, &self.icons, &mut self.input, &self.theme, &mut self.gpu);
        let scene_events = self.input.drain_events();
        if !scene_events.is_empty() {
            let runtime = self.self_weak.clone();
            spawn_app_task(async move {
                if let Some(runtime) = runtime.upgrade() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.dispatch_actions(scene_events).await;
                    }
                }
            });
        }
        if !self.shell.tutorial_pending_document_ops.is_empty() {
            let runtime = self.self_weak.clone();
            spawn_app_task(async move {
                if let Some(runtime) = runtime.upgrade() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.shell.tutorial_flush_pending_document_ops().await;
                    }
                }
            });
        }
        let wheel_delta = self.wheel_delta;
        self.wheel_delta = 0.0;
        if wheel_delta.abs() > 0.0 {
            let x = self.last_pointer_x;
            let y = self.last_pointer_y;
            let ctrl = self.modifiers.ctrl;
            self.shell.handle_pointer_wheel(x, y, wheel_delta, &self.input);
            if ShellState::wheel_propagates_to_scene_surface(self.input.hit_at(x, y)) {
                for state in self.shell.world3d_states.values_mut() {
                    if state.bounds.contains(x, y) {
                        handle_world3d_wheel(state, wheel_delta);
                        // 🕒️ Settle-then-dispatch (see `world3d_camera_dispatch_deadlines_ms`): each
                        // further wheel tick just pushes this surface's deadline back out, so a
                        // `setCamera` only fires ~350ms after the LAST wheel tick, not every tick.
                        self.world3d_camera_dispatch_deadlines_ms.insert(state.surface_id.clone(), app_now_ms() + 350.0);
                    }
                }
                let mut graph_actions = Vec::new();
                for (surface_id, surface) in &self.shell.node_graph_states {
                    if surface.bounds.contains(x, y) {
                        graph_actions.extend(engine_canvas::node_graph_wheel(surface_id, &surface.controller_id, surface.bounds, x, y, wheel_delta, ctrl));
                    }
                }
                if !graph_actions.is_empty() {
                    self.wheel_zoom_deadline_ms = app_now_ms() + 120.0;
                    let runtime = self.self_weak.clone();
                    spawn_app_task(async move {
                        if let Some(runtime) = runtime.upgrade() {
                            if let Ok(mut app) = runtime.try_borrow_mut() {
                                app.dispatch_actions(graph_actions).await;
                            }
                        }
                    });
                }
                let mut map_actions = Vec::new();
                for (surface_id, surface) in &self.shell.tiled_map_states {
                    if surface.bounds.contains(x, y) {
                        map_actions.extend(engine_canvas::tiled_map_wheel(surface_id, &surface.controller_id, surface.bounds, x, y, wheel_delta, ctrl));
                    }
                }
                if !map_actions.is_empty() {
                    let runtime = self.self_weak.clone();
                    spawn_app_task(async move {
                        if let Some(runtime) = runtime.upgrade() {
                            if let Ok(mut app) = runtime.try_borrow_mut() {
                                app.dispatch_actions(map_actions).await;
                            }
                        }
                    });
                }
                let mut board_actions = Vec::new();
                for (surface_id, surface) in &self.shell.board2d_states {
                    if surface.bounds.contains(x, y) {
                        board_actions.extend(scenes::puzzle_board_wheel(surface_id, &surface.controller_id, surface.bounds, x, y, wheel_delta));
                    }
                }
                if !board_actions.is_empty() {
                    let runtime = self.self_weak.clone();
                    spawn_app_task(async move {
                        if let Some(runtime) = runtime.upgrade() {
                            if let Ok(mut app) = runtime.try_borrow_mut() {
                                app.dispatch_actions(board_actions).await;
                            }
                        }
                    });
                }
            }
        }
        for upload in scenes::drain_pending_raster_uploads() {
            self.gpu.ensure_raster_texture(&upload.key, &upload.pixels, upload.width, upload.height);
        }
        if self.atlas.take_dirty() {
            self.gpu.upload_font_atlas(&self.atlas);
        }
        let time_seconds = (app_now_ms() / 1000.0) as f32;
        if let Err(err) = self.gpu.render_frame(&self.draw, Some(&self.overlay), time_seconds) {
            log_debug(&format!("render frame: {err}"));
        }
        let hit = self.input.hit_at(self.last_pointer_x, self.last_pointer_y);
        let base_cursor = resolve_semio_cursor(
            hit,
            CursorDragState { tree_drag: self.shell.tree_drag.is_some(), dock_drag: self.shell.dock_drag.is_some(), pointer_drag_active: self.input.drag.active, pointer_drag_axis: self.input.drag.axis, pointer_drag_kind: self.input.drag.kind },
        );
        // 🖱️ The active utility's cursor overrides generic body cursors while the pointer is over the
        // window body (P5), but never a specific control cursor (text inputs, resize handles).
        let cursor = match self.shell.utility_cursor_override(self.last_pointer_x, self.last_pointer_y) {
            Some(utility_cursor) if matches!(base_cursor, SemioCursor::Default | SemioCursor::Grab | SemioCursor::Selectable | SemioCursor::Pointer) => utility_cursor,
            _ => base_cursor,
        };
        apply_window_cursor(&self.window, cursor, self.theme_dark, &mut self.last_cursor);
        if !self.asset_poll_pending {
            self.poll_pending_assets();
        }
    }

    fn poll_pending_assets(&mut self) {
        let mut glb = collect_pending_glb_fetches(&self.shell.world3d_states);
        glb.extend(collect_pending_glb_fetches(&self.shell.icon_render_states));
        let map = engine_canvas::collect_pending_map_tile_fetches();
        let ui_images = collect_pending_ui_image_fetches();
        if glb.is_empty() && map.is_empty() && ui_images.is_empty() {
            pollster::block_on(self.shell.poll_world3d_assets());
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            for item in map {
                let url = resolve_map_tile_fetch_url(&item.url);
                if let Some(bytes) = fetch_map_tile_bytes_blocking(&url) {
                    engine_canvas::apply_map_tile_bytes(&item.surface_id, &item, &bytes);
                }
            }
            for item in glb {
                let url = resolve_asset_fetch_url(&item.url);
                let bytes = fetch_map_tile_bytes_blocking(&url).or_else(|| pollster::block_on(fetch_url_bytes(&item.url)));
                if let Some(bytes) = bytes {
                    if let Some(state) = self.shell.world3d_states.get_mut(&item.surface_id) {
                        apply_glb_bytes(state, &item.url, &bytes);
                    } else if let Some(state) = self.shell.icon_render_states.get_mut(&item.surface_id) {
                        apply_glb_bytes(state, &item.url, &bytes);
                    }
                }
            }
            for item in ui_images {
                if let Some(bytes) = pollster::block_on(fetch_url_bytes(&item.url)) {
                    apply_ui_image_bytes(&item.id, &item.url, &bytes);
                }
            }
            pollster::block_on(self.shell.poll_world3d_assets());
            return;
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.asset_poll_pending = true;
            let runtime = self.self_weak.clone();
            spawn_app_task(async move {
                struct AssetPollReset(std::rc::Weak<RefCell<AppRuntime>>);
                impl Drop for AssetPollReset {
                    fn drop(&mut self) {
                        if let Some(runtime) = self.0.upgrade() {
                            if let Ok(mut app) = runtime.try_borrow_mut() {
                                app.asset_poll_pending = false;
                            }
                        }
                    }
                }
                let _reset = AssetPollReset(runtime.clone());
                let Some(runtime) = runtime.upgrade() else {
                    return;
                };
                let mut fetched_glb = Vec::new();
                for item in glb {
                    if let Some(bytes) = fetch_url_bytes(&item.url).await {
                        fetched_glb.push((item.surface_id, item.url, bytes));
                    }
                }
                let mut fetched_map = Vec::new();
                for item in map {
                    if let Some(bytes) = fetch_url_bytes(&item.url).await {
                        fetched_map.push((item, bytes));
                    }
                }
                let mut fetched_ui_images = Vec::new();
                for item in ui_images {
                    if let Some(bytes) = fetch_url_bytes(&item.url).await {
                        fetched_ui_images.push((item.id, item.url, bytes));
                    }
                }
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    for (surface_id, url, bytes) in fetched_glb {
                        if let Some(state) = app.shell.world3d_states.get_mut(&surface_id) {
                            apply_glb_bytes(state, &url, &bytes);
                        } else if let Some(state) = app.shell.icon_render_states.get_mut(&surface_id) {
                            apply_glb_bytes(state, &url, &bytes);
                        }
                    }
                    for (fetch, bytes) in fetched_map {
                        engine_canvas::apply_map_tile_bytes(&fetch.surface_id, &fetch, &bytes);
                    }
                    for (id, url, bytes) in fetched_ui_images {
                        apply_ui_image_bytes(&id, &url, &bytes);
                    }
                    app.shell.poll_world3d_assets().await;
                };
            });
        }
    }

    fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
        self.gpu.resize(css_width, css_height, dpr);
        self.shell.screen_w = (css_width * dpr).max(1.0);
        self.shell.screen_h = (css_height * dpr).max(1.0);
    }

    fn handle_key(&mut self, action: KeyAction, modifiers: PointerModifiers) {
        if let KeyAction::Space(pressed) = &action {
            if self.shell.context_menu.is_some() && *pressed {
                let runtime = self.self_weak.clone();
                spawn_app_task(async move {
                    if let Some(runtime) = runtime.upgrade() {
                        if let Ok(mut app) = runtime.try_borrow_mut() {
                            let app = &mut *app;
                            if let Err(err) = app.shell.handle_keyboard_async(KeyAction::Space(true), &modifiers, &mut app.input).await {
                                log_debug(&format!("keyboard failed: {err}"));
                            }
                        }
                    }
                });
                return;
            }
            self.space_pressed = *pressed;
            return;
        }
        if engine_canvas::node_graph_apply_note_edit_key(action.clone(), &modifiers) {
            return;
        }
        // 🔌️ w2-input-wiring: spawns the ASYNC `handle_keyboard_async` (mirrors this fn's own
        // `on_button`/`on_move` sibling callbacks above, and the `spawn_app_task` pattern this fn
        // used to hand-roll just for search/find-Enter-activation) instead of calling the sync
        // `handle_keyboard` directly. Before this fix `handle_keyboard_async` was entirely dead code
        // (see `report-w3-shell-input-cutover.md`'s "MAJOR FINDING"): the P4 app-keybinding dispatch,
        // P5 idle-Escape-deactivates-utility, and — worst — committing a focused `Input`'s typed text
        // via Enter/Escape never fired. `handle_keyboard_async`'s own top already reimplements the
        // exact search/find-Enter-activation this fn used to hand-duplicate around the sync call, so
        // that duplication is gone, not just moved.
        let runtime = self.self_weak.clone();
        spawn_app_task(async move {
            if let Some(runtime) = runtime.upgrade() {
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    let app = &mut *app;
                    if let Err(err) = app.shell.handle_keyboard_async(action, &modifiers, &mut app.input).await {
                        log_debug(&format!("keyboard failed: {err}"));
                    }
                }
            }
        });
    }

    async fn dispatch_actions(&mut self, actions: Vec<ActionDescriptor>) {
        for action in actions {
            for state in self.shell.world3d_states.values_mut() {
                if state.controller_id == action.controller_id {
                    apply_world_action_preview(state, &action);
                }
            }
            if let Err(err) = self.shell.dispatch_action(action).await {
                log_debug(&format!("action failed: {err}"));
            }
        }
    }

    async fn handle_pointer_button(&mut self, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        if !down {
            let mut map_actions = Vec::new();
            let map_had_active_drag = self.shell.tiled_map_states.keys().any(|surface_id| scenes::tiled_map_drag_active(surface_id));
            for (surface_id, surface) in &self.shell.tiled_map_states {
                if !surface.bounds.contains(x, y) && !scenes::tiled_map_drag_active(surface_id) {
                    continue;
                }
                map_actions.extend(scenes::tiled_map_pointer_up(surface_id, &surface.controller_id, surface.bounds, x, y));
            }
            if !map_actions.is_empty() {
                self.dispatch_actions(map_actions).await;
            }
            let mut board_actions = Vec::new();
            let board_had_active_drag = self.shell.board2d_states.keys().any(|surface_id| scenes::board2d_drag_active(surface_id));
            for (surface_id, surface) in &self.shell.board2d_states {
                if !surface.bounds.contains(x, y) && !scenes::board2d_drag_active(surface_id) {
                    continue;
                }
                board_actions.extend(scenes::puzzle_board_pointer_up(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt));
            }
            if !board_actions.is_empty() {
                self.dispatch_actions(board_actions).await;
            }
            let board_consumed = self.shell.board2d_states.values().any(|surface| surface.bounds.contains(x, y)) || board_had_active_drag;
            let map_consumed = self.shell.tiled_map_states.values().any(|surface| surface.bounds.contains(x, y)) || map_had_active_drag;
            if map_consumed || board_consumed {
                return;
            }
            if let Err(err) = self.shell.handle_pointer_button(x, y, down, button, &mut self.input, &self.theme).await {
                log_debug(&format!("pointer failed: {err}"));
            }
            let mut world_actions = Vec::new();
            for state in self.shell.world3d_states.values_mut() {
                if !state.bounds.contains(x, y) {
                    continue;
                }
                if let Some(action) = handle_world3d_pointer_button(state, x, y, down, button, &modifiers) {
                    if action.action == "setCamera" {
                        // 🕒️ Immediate dispatch below beats any still-pending wheel-settle deadline
                        // for this surface — drop it so the debounce sweep doesn't re-dispatch a now-stale
                        // orbit pose a moment later.
                        self.world3d_camera_dispatch_deadlines_ms.remove(&state.surface_id);
                    }
                    apply_world_action_preview(state, &action);
                    world_actions.push(action);
                }
                for action in handle_world3d_paint_actions(state, x, y, down, button) {
                    apply_world_action_preview(state, &action);
                    world_actions.push(action);
                }
                if let Some(action) = handle_world3d_pointer_move(state, x, y, down, button) {
                    apply_world_action_preview(state, &action);
                    world_actions.push(action);
                }
            }
            if !world_actions.is_empty() {
                self.dispatch_actions(world_actions).await;
            }
            let mut graph_actions = Vec::new();
            for (surface_id, surface) in &self.shell.node_graph_states {
                if !surface.bounds.contains(x, y) {
                    continue;
                }
                graph_actions.extend(engine_canvas::node_graph_pointer_up(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt));
            }
            if !graph_actions.is_empty() {
                self.dispatch_actions(graph_actions).await;
            }
            return;
        }
        let mut world_actions = Vec::new();
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if let Some(action) = handle_world3d_pointer_button(state, x, y, down, button, &modifiers) {
                if action.action == "setCamera" {
                    self.world3d_camera_dispatch_deadlines_ms.remove(&state.surface_id);
                }
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
            for action in handle_world3d_paint_actions(state, x, y, down, button) {
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
            if let Some(action) = handle_world3d_pointer_move(state, x, y, down, button) {
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
        }
        if !world_actions.is_empty() {
            self.dispatch_actions(world_actions).await;
            return;
        }
        let mut graph_actions = Vec::new();
        for (surface_id, surface) in &self.shell.node_graph_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            if down {
                graph_actions.extend(engine_canvas::node_graph_pointer_down(surface_id, &surface.controller_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, self.space_pressed));
            } else {
                graph_actions.extend(engine_canvas::node_graph_pointer_up(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt));
            }
        }
        if !graph_actions.is_empty() {
            self.dispatch_actions(graph_actions).await;
        }
        let mut map_actions = Vec::new();
        let mut map_pointer_on_surface = false;
        for (surface_id, surface) in &self.shell.tiled_map_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            map_pointer_on_surface = true;
            if down {
                map_actions.extend(scenes::tiled_map_pointer_down(surface_id, &surface.controller_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), &surface.selection_method));
            }
        }
        if !map_actions.is_empty() {
            self.dispatch_actions(map_actions).await;
            return;
        }
        if map_pointer_on_surface && (button == 0 || button == 1) {
            return;
        }
        let mut board_pointer_on_surface = false;
        for (surface_id, surface) in &self.shell.board2d_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            board_pointer_on_surface = true;
            if down {
                scenes::puzzle_board_pointer_down(surface_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta());
            }
        }
        if board_pointer_on_surface && (button == 0 || button == 1) {
            return;
        }
        if let Err(err) = self.shell.handle_pointer_button(x, y, down, button, &mut self.input, &self.theme).await {
            log_debug(&format!("pointer failed: {err}"));
        }
    }

    async fn handle_pointer_move(&mut self, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
        let drag_dx = x - self.last_pointer_x;
        let drag_dy = y - self.last_pointer_y;
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        self.shell.handle_pointer_move(x, y, down, &mut self.input, &self.theme);
        if let Err(err) = self.shell.flush_deferred_actions().await {
            log_debug(&format!("deferred actions: {err}"));
        }
        if down && (button == 0 || button == 2 || button == 1) {
            for state in self.shell.world3d_states.values_mut() {
                if state.bounds.contains(x, y) {
                    handle_world3d_pointer_drag(state, x, y, drag_dx, drag_dy, button, &modifiers);
                }
            }
        }
        let mut world_actions = Vec::new();
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if let Some(action) = handle_world3d_pointer_move(state, x, y, down, button) {
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
            for action in handle_world3d_paint_actions(state, x, y, down, button) {
                apply_world_action_preview(state, &action);
                world_actions.push(action);
            }
        }
        let mut graph_actions = Vec::new();
        for (surface_id, surface) in &self.shell.node_graph_states {
            if surface.bounds.contains(x, y) {
                graph_actions.extend(engine_canvas::node_graph_pointer_move(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt));
            }
        }
        if !graph_actions.is_empty() {
            self.dispatch_actions(graph_actions).await;
        }
        let mut map_actions = Vec::new();
        for (surface_id, surface) in &self.shell.tiled_map_states {
            if !surface.bounds.contains(x, y) && !scenes::tiled_map_drag_active(surface_id) {
                continue;
            }
            map_actions.extend(scenes::tiled_map_pointer_move(surface_id, &surface.controller_id, surface.bounds, x, y, down));
        }
        if !map_actions.is_empty() {
            self.dispatch_actions(map_actions).await;
        }
        let mut board_actions = Vec::new();
        for (surface_id, surface) in &self.shell.board2d_states {
            let inside = surface.bounds.contains(x, y);
            if inside {
                board_actions.extend(scenes::puzzle_board_pointer_move(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt));
            } else {
                board_actions.extend(scenes::puzzle_board_pointer_leave(surface_id, &surface.controller_id, modifiers.alt));
            }
        }
        if !board_actions.is_empty() {
            self.dispatch_actions(board_actions).await;
        }
        if !world_actions.is_empty() {
            self.dispatch_actions(world_actions).await;
        }
    }

    async fn handle_context_menu(&mut self, x: f32, y: f32) {
        let _ = self.shell.handle_pointer_button(x, y, true, 2, &mut self.input, &self.theme).await;
    }
}

//#region 🔖️OsHostDecomposition — SemioApp deletion
// 🏚️ DELETED by ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host):
// `start_frame_loop` (used to live here, ~line 2291 pre-edit — the recursive `schedule_frame`
// rAF/timer chain that called `app.frame()` and immediately rescheduled itself, unconditionally,
// forever), `enum HostUserEvent` and `struct SemioApp` + its `ApplicationHandler` impl (`resumed`
// set `ControlFlow::Poll` at boot — ~line 2383 pre-edit; `window_event`'s `RedrawRequested` arm
// called `window.request_redraw()` unconditionally right after building a frame — ~line 2406
// pre-edit; `about_to_wait` called `kernel_runtime::poll_tasks()` then ALSO unconditionally
// `window.request_redraw()` every single iteration — ~line 2416-2424 pre-edit). Replaced by
// `winit_app::{HostUserEvent, WinitApp}` — same two-phase boot handshake, but steady-state control
// flow is `WaitUntil(next deadline)`/`Wait`, redraw only fires `if let Some(reason) =
// scheduler.should_render(now)`, and `poll_tasks()` now runs once per real wake instead of every
// tick of an infinite `Poll` loop. See `📓️terra-os-host-report.md`'s redraw audit for the full
// before/after per site.
//#endregion 🔖️OsHostDecomposition — SemioApp deletion

async fn boot_runtime(
    window: Arc<Window>,
    plugin_filter: String,
    #[cfg(target_arch = "wasm32")] plugins: Option<wasm_bindgen::JsValue>,
    #[cfg(not(target_arch = "wasm32"))] plugin_modules_root: std::path::PathBuf,
) -> Result<(Rc<RefCell<AppRuntime>>, PointerCallbacks), String> {
    let dpr = window.scale_factor() as f32;
    let size = window.inner_size();
    #[cfg(target_arch = "wasm32")]
    let (css_width, css_height, dpr) = {
        use winit::platform::web::WindowExtWebSys;
        let dpr = web_sys::window().map(|host| host.device_pixel_ratio() as f32).unwrap_or(dpr);
        if let Some(canvas) = window.canvas() {
            let css_width = canvas.client_width().max(1) as f32;
            let css_height = canvas.client_height().max(1) as f32;
            canvas.set_width((css_width * dpr) as u32);
            canvas.set_height((css_height * dpr) as u32);
            (css_width, css_height, dpr)
        } else {
            (size.width as f32 / dpr, size.height as f32 / dpr, dpr)
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let css_width = size.width as f32 / dpr;
    #[cfg(not(target_arch = "wasm32"))]
    let css_height = size.height as f32 / dpr;

    const ANTA_LATIN: &[u8] = include_bytes!("../../../../../../../../../🔨️modules/🖼️assets/🔤️fonts/🔤️anta/🔤️latin.ttf");
    let font_bytes = match fetch_font_bytes("/asset/font/anta/🔤️latin.ttf").await {
        Ok(bytes) if bytes.len() > 256 => bytes,
        _ => ANTA_LATIN.to_vec(),
    };
    let atlas = FontAtlas::from_bytes(&font_bytes).map_err(|err| format!("atlas failed: {err}"))?;
    let icons = icon_atlas::build_icon_atlas();
    let mut gpu = GpuContext::from_window(window.clone()).await.map_err(|err| format!("gpu init failed: {err}"))?;
    gpu.resize(css_width, css_height, dpr);
    gpu.upload_font_atlas(&atlas);
    gpu.upload_icon_atlas(&icons);

    #[cfg(target_arch = "wasm32")]
    let entries = {
        let plugins = plugins.ok_or("missing wasm programs")?;
        filter_plugins(parse_plugin_entries(plugins).map_err(|err| format!("program parse failed: {err}"))?, &plugin_filter)
    };
    #[cfg(not(target_arch = "wasm32"))]
    let entries = filter_plugins(load_wasm_plugins(&plugin_filter, &plugin_modules_root)?, &plugin_filter);

    let mut shell = ShellState::new(entries, plugin_filter.clone());
    shell.screen_w = css_width * dpr;
    shell.screen_h = css_height * dpr;
    shell.boot().await.map_err(|err| format!("shell boot failed: {err}"))?;

    let runtime = Rc::new(RefCell::new(AppRuntime {
        gpu,
        atlas,
        icons,
        shell,
        draw: DrawList::default(),
        overlay: DrawList::default(),
        input: InputState::default(),
        theme: Theme::default(),
        window: window.clone(),
        theme_dark: appearance_is_dark("system"),
        last_cursor: None,
        last_pointer_x: 0.0,
        last_pointer_y: 0.0,
        pointer_down: false,
        pointer_button: 0,
        modifiers: PointerModifiers::default(),
        wheel_delta: 0.0,
        space_pressed: false,
        wheel_zoom_deadline_ms: 0.0,
        world3d_camera_dispatch_deadlines_ms: std::collections::HashMap::new(),
        caret_blink_at_ms: 0.0,
        caret_blink_visible: true,
        asset_poll_pending: false,
        self_weak: std::rc::Weak::new(),
        #[cfg(not(target_arch = "wasm32"))]
        plugin_modules_root: plugin_modules_root.clone(),
        #[cfg(not(target_arch = "wasm32"))]
        native_plugin_mtimes: std::collections::HashMap::new(),
        #[cfg(not(target_arch = "wasm32"))]
        native_reload_pending: false,
    }));
    runtime.borrow_mut().self_weak = Rc::downgrade(&runtime);

    let runtime_pointer = runtime.clone();
    let runtime_move = runtime.clone();
    let runtime_wheel = runtime.clone();
    let runtime_keyboard = runtime.clone();
    let runtime_context = runtime.clone();
    let callbacks = PointerCallbacks {
        on_move: Rc::new(move |x, y, down, button, modifiers| {
            let runtime = runtime_move.clone();
            spawn_app_task(async move {
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    app.handle_pointer_move(x, y, down, button, modifiers).await;
                }
            });
        }),
        on_button: Rc::new(move |x, y, down, button, modifiers| {
            let runtime = runtime_pointer.clone();
            spawn_app_task(async move {
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    app.handle_pointer_button(x, y, down, button, modifiers).await;
                }
            });
        }),
        on_wheel: Rc::new(move |delta, _x, _y, _modifiers| {
            if let Ok(mut app) = runtime_wheel.try_borrow_mut() {
                app.wheel_delta += delta;
            }
        }),
        on_key: Rc::new(move |action, modifiers| {
            if let Ok(mut app) = runtime_keyboard.try_borrow_mut() {
                app.handle_key(action, modifiers);
            }
        }),
        on_context_menu: Rc::new(move |x, y| {
            let runtime = runtime_context.clone();
            spawn_app_task(async move {
                if let Ok(mut app) = runtime.try_borrow_mut() {
                    app.handle_context_menu(x, y).await;
                }
            });
        }),
    };

    log_debug("wgpu renderer booted");
    Ok((runtime, callbacks))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_native(plugin_filter: &str, plugin_modules_root: std::path::PathBuf) {
    let event_loop = EventLoop::<winit_app::HostUserEvent>::with_user_event().build().expect("event loop");
    let proxy = event_loop.create_proxy();
    let mut app = winit_app::WinitApp::new(proxy, plugin_filter.to_string(), plugin_modules_root);
    let _ = event_loop.run_app(&mut app);
}

/// 🧪️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END — headless smoke mode: boots
/// `ShellState` against a live hub (`S_HUB_URL`/`S_USER`/`S_DATA_DIR`) with NO GPU/window at all —
/// `GpuContext`/`winit`/`AppRuntime` are the only GPU-coupled pieces in this crate and this mode never
/// touches any of them, since `ShellState` itself is renderer-agnostic (chrome painting is a separate
/// concern layered on top by `AppRuntime::frame`). Boots, waits (bounded) for identity to mint/restore
/// and the initial directory fold to land, then dumps the Home window's widget tree + a small identity/
/// session summary as JSON to stdout and returns an exit code. An honest, explicit substitute for
/// driving a real window when this environment cannot open one (lane 3-D's brief proposed exactly this
/// shape). Returns `0` on a clean boot+dump, `1` on any hard failure along the way.
#[cfg(not(target_arch = "wasm32"))]
pub fn run_smoke(plugin_filter: &str, plugin_modules_root: std::path::PathBuf) -> i32 {
    pollster::block_on(async {
        let loaded = match load_wasm_plugins(plugin_filter, &plugin_modules_root) {
            Ok(entries) => entries,
            Err(error) => {
                eprintln!("smoke: load_wasm_plugins failed: {error}");
                return 1;
            }
        };
        let entries = filter_plugins(loaded, plugin_filter);
        let mut shell = ShellState::new(entries, plugin_filter.to_string());
        if let Err(error) = shell.boot().await {
            eprintln!("smoke: shell.boot() failed: {error}");
            return 1;
        }
        // 🪪️ Identity mint/restore runs on a background OS thread (contract §C3: never blocks
        // `boot()` itself) — poll the same every-frame pump the real render loop uses (drains the
        // identity bootstrap channel + the directory stream + folds any pending events) for up to 5s
        // so a real hub round trip has time to land before the dump.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while std::time::Instant::now() < deadline {
            shell.pump_sync_events().await;
            if shell.identity.is_some() || shell.identity_env.is_none() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        let _ = shell.refresh_ui().await;
        let identity_summary = shell.identity.as_ref().map(|identity| serde_json::json!({ "userId": identity.user_id, "email": identity.email, "hubBaseUrl": identity.hub_base_url }));
        let report = serde_json::json!({
            "booted": true,
            "identity": identity_summary,
            "identityOffline": shell.identity_offline,
            "openSpaceId": shell.open_space_id,
            "session": shell.session.as_ref().map(|session| serde_json::json!({ "pluginId": session.plugin_id, "appId": session.app.id, "role": format!("{:?}", session.app.role) })),
            "windowUi": &shell.window_ui,
        });
        match serde_json::to_string_pretty(&report) {
            Ok(json) => {
                println!("{json}");
                0
            }
            Err(error) => {
                eprintln!("smoke: report encode failed: {error}");
                1
            }
        }
    })
}

/// 🐚️ Multi-mount: takes an already-created, already-placed canvas from the caller instead of looking
/// up a hardcoded `#root`/`#semio-wgpu-canvas` and taking it over via `set_inner_html("")` — that
/// single-mount assumption meant a second boot call would wipe the first mount's canvas and collide on
/// the same DOM id. The caller (`bootFrameworkOsWgpu` in `📦️index.ts`) now owns creating and placing
/// the canvas, so N independent mounts can coexist on one page.
///
/// Known gap (not yet done — see the plan's Wave 6 D11 notes), **narrowed but not closed** by ticket
/// 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host): the independent, uncancellable
/// `start_frame_loop`/`schedule_frame` recursive `requestAnimationFrame` chain this comment used to
/// describe is DELETED — every redraw now goes through `winit`'s own event loop
/// (`event_loop.spawn_app(app)` below → `winit_app::WinitApp`), and `WindowEvent::CloseRequested`
/// already calls `ActiveEventLoop::exit()`. Whether `exit()` alone now fully tears down a wasm mount
/// (winit's own wasm backend's post-`exit()` behaviour) is UNVERIFIED — this crate still does not
/// build clean (U4, `📓️terra-os-host-report.md`), so a real `semioWgpuUnmount` handle remains
/// deferred, but the mechanism it would need to cancel no longer exists in its old shape.
/// The dozen-plus `thread_local!` globals further up this file (`UI_ENGINE`, `ENGINE_SURFACES`,
/// `SCENE_STATE`, tooltip/dialog/tour chrome state, clipboard mocks, prefs, image-fetch caches, …) are
/// also still page-global, not per-mount — two simultaneous wgpu mounts each render on their own
/// independent GPU device/queue/surface (real, working isolation), but would still cross-talk on shared
/// UI chrome auxiliary state (a tooltip or dialog opened in one mount could show in the other).
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuMount)]
pub fn semio_wgpu_mount(canvas: web_sys::HtmlCanvasElement, plugins: JsValue, plugin_filter: String) -> Result<(), JsValue> {
    let event_loop = EventLoop::<winit_app::HostUserEvent>::with_user_event().build().map_err(|err| JsValue::from_str(&format!("event loop: {err:?}")))?;
    let proxy = event_loop.create_proxy();
    let app = winit_app::WinitApp::new(proxy, plugin_filter, Some(plugins), Some(canvas));
    use winit::platform::web::EventLoopExtWebSys;
    event_loop.spawn_app(app);
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = uploadIconAtlas)]
pub fn upload_icon_atlas(width: u32, height: u32, pixels: &[u8], entries_json: &str) -> Result<(), JsValue> {
    let entries_map: std::collections::HashMap<String, [f32; 4]> = serde_json::from_str(entries_json).map_err(|err| JsValue::from_str(&format!("icon entries parse: {err}")))?;
    let entries: Vec<(String, [f32; 4])> = entries_map.into_iter().collect();
    ICON_ATLAS_RUNTIME.with(|cell| {
        cell.borrow_mut().replace(IconAtlas::from_packed(width, height, pixels.to_vec(), entries));
    });
    Ok(())
}

thread_local! {
    static ICON_ATLAS_RUNTIME: RefCell<Option<IconAtlas>> = RefCell::new(None);
}

//#region 🔖️RoleBoot
// 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §5: boot role from
// `SEMIO_APP_ROLE` (native, read directly)/`VITE_SEMIO_APP_ROLE` (wasm — wasm has no env var
// access, so `🟦️boot.ts` reads `import.meta.env.VITE_SEMIO_APP_ROLE` and calls
// `semioWgpuSetAppRole` before/at mount), default `editor` (`ChromeRole::from_boot_env`'s own
// fallback). Deliberately additive, same idiom as `ICON_ATLAS_RUNTIME` immediately above: a
// `thread_local` a caller opts into reading (`boot_app_role`) rather than a parameter threaded
// through every existing mount/native entry point — this crate currently fails to build clean for
// reasons entirely outside this lease (a concurrent, unrelated plugin-crate refactor breaks a
// transitive dependency; confirmed via `git status` showing 70+ uncommitted stdio-plugin files —
// see `📓️w1-d-report.md`), so a signature change on `run_native`/`semio_wgpu_mount` could not be
// verified to compile and was avoided.
thread_local! {
    static BOOT_APP_ROLE: RefCell<ui_wgpu::wgpu::component::role_chrome::ChromeRole> = RefCell::new(resolve_native_boot_role());
}

#[cfg(not(target_arch = "wasm32"))]
fn resolve_native_boot_role() -> ui_wgpu::wgpu::component::role_chrome::ChromeRole {
    ui_wgpu::wgpu::component::role_chrome::ChromeRole::from_boot_env(std::env::var("SEMIO_APP_ROLE").ok().as_deref())
}

#[cfg(target_arch = "wasm32")]
fn resolve_native_boot_role() -> ui_wgpu::wgpu::component::role_chrome::ChromeRole {
    ui_wgpu::wgpu::component::role_chrome::ChromeRole::Editor
}

/// 🌐️ wasm boot hook — `🟦️boot.ts` calls this once, before/at mount time, with
/// `import.meta.env.VITE_SEMIO_APP_ROLE ?? "editor"`.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetAppRole)]
pub fn semio_wgpu_set_app_role(role: String) {
    BOOT_APP_ROLE.with(|cell| *cell.borrow_mut() = ui_wgpu::wgpu::component::role_chrome::ChromeRole::from_boot_env(Some(role.as_str())));
}

/// 👁️✏️ The boot-resolved role, contract freeze §5 — `SemioApp`'s session/window-open path is meant
/// to read this to call `Shell::set_window_role`/`set_locale`; wiring that specific call site is
/// this lease's documented gap (see `📓️w1-d-report.md` — this crate's own build break blocks
/// verifying any change deep inside `SemioApp`, so this stops at the boundary of what compiles
/// standalone).
pub fn boot_app_role() -> ui_wgpu::wgpu::component::role_chrome::ChromeRole {
    BOOT_APP_ROLE.with(|cell| *cell.borrow())
}
//#endregion 🔖️RoleBoot
