use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
#[cfg(test)]
#[path="../../../🧪️tests/🎟️original-turn/🦀️.rs"]
mod fixture_original_turn;
#[cfg(target_arch = "wasm32")]
extern crate semio_framework_async as wasm_bindgen_futures;
#[cfg(test)]
use ui_wgpu::wgpu::Mesh3dItem;

#[cfg(test)]
include!("../../../🧪️tests/🧊️wgpu-renderer-standalone/🦀️.rs");

#[cfg(test)]
include!("../../../🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs");

// 🧊️ Raw wgpu WASM renderer for declarative framework UiNode trees.
//
// 🧭️ Rough correspondence with the React shell (`framework/renderer/react/os-shell.tsx`), as a
// discoverability breadcrumb rather than a rigorous mapping:
// - this crate's top-level shell/state struct ~ React's `#region 🔖️types` + `FrameworkOsShell`.
// - the `dock` module below (window tree, stack chrome, split resize) ~ React's `Mode`
//   component and the `WindowLayoutNode` tree helpers in `#region ShellHelpers`.
// - `interpreter`/widget rendering ~ React's `UiNode` component tree rendering.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_kernel as dsl;
#[cfg(not(target_arch = "wasm32"))]
extern crate semio_framework_os_kernel as dsl_core;
extern crate semio_framework_os_kernel as protocol;
#[cfg(any(test, not(target_arch = "wasm32")))]
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as store_sync;
// 🥽️ Target-neutral: the catalog is `include_str!`-embedded, and `scenes::render_icon_render` needs
// `mesh_asset_transport_url` on the browser build too — the same public-id → transport-path rewrite
// React's `IconRenderHost` applies before handing a `/mesh/…` id to its loader.
#[path = "../../../🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod dock;
#[path = "../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🦀️.rs"]
pub mod mesh_assets;

//#region 🤖️AgentBridgeElements
// 🌉️ WGPU-RENDERER-REACT-PARITY packet W1j — the four `os.agent.*` elements React's `🏛️ShellHost`
// mounts (`useAgentBridge` + `AgentChatPanel` + `AgentApprovals`) had no wgpu twin at all, so an
// approval request delivered to a wgpu-rendered shell could never reach the user. Declared in
// dependency order for readability only; `agent_bridge` owns the frame codec and the consumer
// state, the other three are its renderers.
#[path = "../../../🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod agent_bridge;

#[path = "../../../🧱️elements/🚦️AgentPresence/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod agent_presence;

#[path = "../../../🧱️elements/🤖️AgentApprovals/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod agent_approvals;

#[path = "../../../🧱️elements/💬️AgentChatPanel/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod agent_chat_panel;

// 🌉️ WGPU-RENDERER-REACT-PARITY packet W15e — the socket itself, the one third of React's
// `useAgentBridge` W1j deliberately left out: the browser half rides the duplex page door, the native
// half a `tokio-tungstenite` connection on `Lane::Io`. Both drive the same bounded `SocketLane`.
#[path = "../🌉️agent-bridge-door/🦀️.rs"]
pub mod agent_bridge_door;
//#endregion 🤖️AgentBridgeElements

// 🔤️ The compiler's font port: the one place each renderer host hands the math, emoji and text icon
// fonts to `semio_framework_canvas`, before the first icon resolves.
#[path = "../🔤️compiler-fonts/🦀️.rs"]
pub mod compiler_fonts;

// 🛂️ WGPU-RENDERER-REACT-PARITY packet W15e — the administration sheet's CHROME. Its operation and
// control set have been live on both targets since W1e; nothing painted them until this module.
#[path = "../../../🧱️elements/🛂️SpaceAdministration/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod space_administration;

//#region 🔐️HubElements
// 🔐️ Ticket 26/09/18 slice WG6 (G8 items WG-6 and WG-5): the three os-renderer elements AU2/AU3
// landed for React only — `🔐️HubSignIn`, `🏘️SpaceBrowser`, `🔗️HubConnection` — so a wgpu-rendered
// shell had no hub identity of its own at all (its only identity was an inherited local-bootstrap
// credential or a same-origin cookie). Declared in dependency order for readability only:
// `hub_sign_in` and `space_browser` are pure contract twins with no renderer and no transport,
// `hub_connection` composes them with the shell's own `DirectoryTransport` and builds the retained
// workspace tree.
#[path = "../../../🧱️elements/🔐️HubSignIn/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod hub_sign_in;

#[path = "../../../🧱️elements/🏘️SpaceBrowser/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod space_browser;

#[path = "../../../🧱️elements/🔗️HubConnection/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod hub_connection;
//#endregion 🔐️HubElements

#[path = "../../../🧱️elements/👕️canvas-presence/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod canvas_presence;

#[path = "../🧬️element-contracts/🦀️.rs"]
pub mod element_contracts;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod engine_canvas;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod interpreter;

#[path = "../🎬️media-slots/🦀️.rs"]
pub mod media_slots;

#[path = "../../../🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod program_bridge;

//#region 🏠️🧳️PluginHostConfig
// 🐛️ Lives at the crate root, not inside `program_bridge` above (see that module's own `PluginHostConfig`
// region for why) — this is the file's real directory, so the 3-`..` climb to
// `framework/plugin/registry/generated/🖥️hosts/🦀️.rs` actually resolves.
#[path="../📇️catalog/🦀️.rs"]
pub mod plugin_catalog;
pub use plugin_catalog::{install_plugin_catalog_v1,install_plugin_catalog_json_v1};
//#endregion 🏠️🧳️PluginHostConfig

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod scenes;
use semio_framework_os_renderer_camera as camera_storage;

#[path = "../../../🧱️elements/📐️Canvas2dHost/🎯️targets/🧊️wgpu/🦀️.rs"]
pub(crate) mod canvas2d_gumball;

#[path = "../../../🧱️elements/📐️Canvas2dHost/🎨️paint/🦀️.rs"]
pub(crate) mod canvas2d_paint;

#[cfg(not(target_arch = "wasm32"))]
#[path = "../🕰️native-temporal/🦀️.rs"]
mod native_temporal;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod shell;

#[path = "../../../🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod icon_atlas;

//#region 🔖️OsHostDecomposition
// 🏠️ ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host): the seam that ends
// this crate owning the actor kernel and ends its continuous redraw. `deadlines` is a leaf (no
// dependency on `os_host`/`winit_app`); `os_host` composes `AppRuntime` with it; `winit_app` is the
// new `ApplicationHandler` — see that file's own module docstring for why it hand-rolls the event
// loop instead of using `ui_host::window::NativeHost<D>` directly. Mounted here, away from the peer
// program's `parallel_runtime` mount just below, per this ticket's own OWNS list.
//
// 🪢️ There used to be a second, `#[cfg(test)]`-only `kernel_seam` module here: a bounded
// intent/outcome mailbox whose own docstring admitted "Test fixture … Production rendering dispatches
// through RuntimeMailbox; this fixture has no production router." It had no production consumer at
// all (its `KernelSeam`/`AppKernelSeam`/`HostWaker` were named only by its own test) and it shadowed
// the name of the REAL production exchange, `KernelOutcome` in `🔖️KernelClient` below. Ticket
// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W2k retired it: `📮️RuntimeMailbox` is the one production
// router, and it is already the exact counterpart of React's own dispatch seam — `onActionStable`
// taking a descriptor built by `uiIntentToActionDescriptor` (`🛠️ShellHelpers/🟦️.tsx:2051`). Nothing
// about wgpu's runtime semantics needed the fixture; what DID differ was the controller identity that
// descriptor carried, and that is fixed at the source in `ui_wgpu`'s `UiIntentCommand::descriptor`.
#[path = "../⏰️deadlines/🦀️.rs"]
mod deadlines;

#[path = "../⏱️frame-latency/🦀️.rs"]
pub(crate) mod frame_latency;

#[path = "../🏠️os-host/🦀️.rs"]
mod os_host;
#[cfg(not(target_arch="wasm32"))]
#[path="../🎟️native-caller/🦀️.rs"]
mod native_caller;

#[path = "../📸️render-snapshot/🦀️.rs"]
mod render_snapshot;
#[path="../🌐️browser-worker/📤️output/🦀️.rs"]
mod browser_original_output;

#[path = "../📮️runtime-mailbox-core/🦀️.rs"]
mod runtime_mailbox_core;

// 🧵️ P3b (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): the `InteractiveJob` seam for the
// slice of `AppRuntime::frame()` that genuinely is `Send`-safe today — see that file's own module
// docstring for exactly what moves and, more importantly, what still cannot.
#[path = "../🧵️frame-job/🦀️.rs"]
mod frame_job;
#[path = "../../../🪙️authority/🖼️frame/🦀️.rs"]
mod frame_authority;

#[path = "../📐️surface-lane/🦀️.rs"]
mod surface_lane;

#[path = "../🎮️input-wire/🦀️.rs"]
mod input_wire;

// 🔌️ The DUPLEX door — the second browser door, beside the request/response mailbox. Mounted before
// `directory_door` because that door's `open_ws` is one of its two consumers (the other is the MCP
// agent bridge). See its module doc for why a socket cannot ride the mailbox.
#[path = "../🔌️socket-door/🦀️.rs"]
pub mod socket_door;

// 📇️ The ONE directory transport seam the shell's identity/command/Space-Administration lanes are
// parameterized over — native `ureq` here, the page `fetch` door there. See its module doc.
#[path = "../📇️directory-door/🦀️.rs"]
pub mod directory_door;

#[cfg(target_arch = "wasm32")]
#[path = "../🌐️browser-worker/🦀️.rs"]
mod browser_worker;

#[path = "../🪟️winit-app/🦀️.rs"]
mod winit_app;

// ♿️ The native shell's platform accessibility bridge: every window's accessibility projection as one AccessKit tree
// (NSAccessibility / UI Automation / AT-SPI), AccessKit kept behind this one module. The browser build's DOM mirror
// (`♿️accessibility-mirror`) is its twin.
#[cfg(not(target_arch = "wasm32"))]
#[path = "../♿️native-accessibility/🦀️.rs"]
pub(crate) mod native_accessibility;
//#endregion 🔖️OsHostDecomposition

// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop): the real multi-shard `Kernel`
// loop — `ParallelRuntime` — used by both `kernel_runtime` (below) and `scale_bench`. Native-only,
// same reason `kernel_runtime`/`scale_bench` themselves are: native guest execution and the shared
// native worker pool are not available on wasm32.
#[cfg(not(target_arch = "wasm32"))]
#[path = "../🎠️runtime/🦀️.rs"]
pub mod parallel_runtime;

#[path = "🗂️asset-authority/🦀️.rs"]
mod asset_authority;
pub(crate) use asset_authority::*;

#[path = "🦴️glb-decode/🦀️.rs"]
mod glb_decode;
pub(crate) use glb_decode::*;

#[path = "🔁️asset-decode-jobs/🦀️.rs"]
mod asset_decode_jobs;
pub(crate) use asset_decode_jobs::*;

//#region 🔖️ScaleBench
#[cfg(not(target_arch = "wasm32"))]
#[path = "📊️scale-bench/🦀️.rs"]
pub mod scale_bench;
//#endregion 🔖️ScaleBench

#[path = "🛎️frame-actions/🦀️.rs"]
mod frame_actions;
pub(crate) use frame_actions::*;

#[path = "🎨️presentation-raster/🦀️.rs"]
mod presentation_raster;
pub(crate) use presentation_raster::*;

#[path = "📮️mailbox-impl/🦀️.rs"]
mod mailbox_impl;
pub(crate) use mailbox_impl::*;

#[path = "🧾️frame-build/🦀️.rs"]
mod frame_build;
pub(crate) use frame_build::*;

#[path = "🖥️frame-presentation/🦀️.rs"]
mod frame_presentation;
pub(crate) use frame_presentation::*;

#[path = "🧠️app-runtime/🦀️.rs"]
mod app_runtime;
pub(crate) use app_runtime::*;

#[path = "🔬️native-socket-probe/🦀️.rs"]
mod native_socket_probe;
pub use native_socket_probe::*;

/// 🔎️ The whole renderer source tree as one text, for the source-text law tests that pin where the renderer calls what.
#[cfg(test)]
pub(crate) const RENDERER_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🗂️asset-authority/🦀️.rs"),
    include_str!("🦴️glb-decode/🦀️.rs"),
    include_str!("🔁️asset-decode-jobs/🦀️.rs"),
    include_str!("📦️kernel-runtime/🦀️.rs"),
    include_str!("📦️kernel-runtime/🧮️kernel-core/🦀️.rs"),
    include_str!("📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs"),
    include_str!("📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs"),
    include_str!("📦️kernel-runtime/🏊️kernel-pool/🦀️.rs"),
    include_str!("📊️scale-bench/🦀️.rs"),
    include_str!("🛎️frame-actions/🦀️.rs"),
    include_str!("🎨️presentation-raster/🦀️.rs"),
    include_str!("📮️mailbox-impl/🦀️.rs"),
    include_str!("🧾️frame-build/🦀️.rs"),
    include_str!("🖥️frame-presentation/🦀️.rs"),
    include_str!("🧠️app-runtime/🦀️.rs"),
    include_str!("🔬️native-socket-probe/🦀️.rs")
);

use crate::scenes::AdmittedSurfaceToken;
use infinite_world::world::world3d_asset_cancellation_requested;
#[cfg(not(target_arch = "wasm32"))]
use infinite_world::world::WORLD_ASSET_RESPONSE_BYTE_CAPACITY;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use infinite_world::world::{apply_decoded_reference_raster, world3d_reference_url_is_current, DecodedReferenceImage};
use infinite_world::world::{apply_world3d_terrain_tile_bytes, collect_world3d_asset_bytes, mark_world3d_asset_miss};
use infinite_world::world::{
    begin_world3d_dynamic_retirement, close_world3d_draw_rebuild_step, enqueue_world3d_event, finish_world3d_asset, publish_world3d_asset_mesh, reserve_world3d_asset_response, retire_cancelled_world3d_asset_step, return_world3d_asset,
    seal_world3d_asset_response, step_world3d_camera_fit, step_world3d_draw_rebuild, step_world3d_dynamic_retirement, step_world3d_interaction, step_world3d_scene_bridge, step_world3d_snapshot, take_next_completed_world3d_asset_step,
    take_next_world3d_asset, world3d_dynamic_retirement_terminal_is_empty, world3d_interaction_front_generation, World3dSceneBridgeStep, World3dSnapshotApplyStep, WorldAssetFault, WorldAssetFetchOwner, WorldAssetIoAuthority, WorldAssetMetadataId,
    WorldAssetRequestKind, WorldAssetRequestToken, WorldAssetResponsePage, WorldDrawRebuildStep, WorldDynamicFault, WorldInteractionAuthorityStep, WorldInteractionIntent, WORLD_ASSET_REQUEST_CAPACITY, WORLD_ASSET_RESPONSE_PAGE_BYTES,
    World3dMeshAsset, World3dMeshAppearance, World3dPrimitiveMaterial, WORLD_ASSET_RESPONSE_PAGE_CAPACITY,
};
use infinite_world::world::{world3d_hover_clear_is_owed, world3d_hover_is_published};
use program_bridge::filter_plugins;
#[cfg(not(target_arch = "wasm32"))]
use program_bridge::load_wasm_plugins;
#[cfg(target_arch = "wasm32")]
use program_bridge::parse_plugin_entries;
use shell::{PointerCapture, PointerHitOwner, ShellState};
use std::cell::RefCell;
use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use ui_wgpu::wgpu::{ActionDescriptor, SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterPool, SceneRasterProfile, SceneRasterWriteMode, SceneRasterWriter};
// 🏚️ `dispatch_window_event`/`WindowInputState`/`schedule_frame` no longer imported here — they were
// `SemioApp`/`start_frame_loop`-only (both deleted, packet os-host); `winit_app.rs` normalizes input
// itself via `ui_host::event` instead. See the `OsHostDecomposition — SemioApp deletion` region above.
#[cfg(target_arch = "wasm32")]
use semio_framework_async::browser::spawn_local;
use ui_wgpu::wgpu::{
    apply_window_cursor, fetch_font_bytes, mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_read_write_u32, mesh3d_read_write_vec3, mesh3d_seal, mesh3d_update_vec3, mesh3d_write_u32,
    mesh3d_write_edge, mesh3d_write_vec2, mesh3d_write_vec3, mesh3d_write_vec4, resolve_semio_cursor, CursorDragState, DrawList, FontAtlas, GpuContext, IconAtlas, InputState, KeyAction, Mesh3dFault, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, PointerModifiers, SemioCursor,
    Theme,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
// 🏚️ `ApplicationHandler`/`WindowEvent`/`ActiveEventLoop`/`EventLoopProxy`/`WindowAttributes`/
// `WindowId` no longer imported here — all `SemioApp`-only (deleted, packet os-host); `winit_app.rs`
// imports each of these itself. `EventLoop`/`Window` stay: `run_native`/`semio_wgpu_mount` still
// construct the event loop and `AppRuntime` still names `Window` throughout.
#[cfg(not(target_arch = "wasm32"))]
use winit::event_loop::EventLoop;
#[cfg(not(target_arch = "wasm32"))]
use winit::window::Fullscreen;
use winit::window::Window;

//#region 🎠️KernelRuntime
#[cfg(not(target_arch = "wasm32"))]
#[path = "📦️kernel-runtime/🦀️.rs"]
pub(crate) mod kernel_runtime;
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
/// `🖥️host/🧵️shard/🦀️.rs`'s own `BudgetBridge` region already uses for the REVERSE
/// direction (`GRANT_BUDGET_DEFAULT_MAX_FRAMES`).
#[cfg(not(target_arch = "wasm32"))]
fn actor_budget_from_turn_budget(budget: semio_framework::kernel::Budget, original: semio_framework_plugin_host::shard::grant::ShardResourceBudget) -> semio_framework_plugin_host::shard::grant::ShardResourceBudget {
    semio_framework_plugin_host::shard::grant::ShardResourceBudget { fuel: budget.fuel, wall_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes,max_frames:budget.max_frames, ..original }
}
//#endregion 🔖️ActorBudgetBridge

/// 🧑‍🏭️ Advances bounded native I/O turns on application and kernel workers before presentation exists.
#[cfg(not(target_arch = "wasm32"))]
async fn drive_renderer_worker_task<F: Future>(future: F) -> F::Output {
    let mut future = Box::pin(future);
    std::future::poll_fn(|cx| {
        let result = future.as_mut().poll(cx);
        let advanced = pump_renderer_io_sessions(16);
        if result.is_pending() && advanced != 0 {
            cx.waker().wake_by_ref();
        }
        result
    }).await
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_app_task<F>(future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    let _ = kernel_runtime::KernelPoolFuture::spawn(renderer_worker_pool(), semio_framework_async::Lane::Interactive, future);
}

#[cfg(target_arch = "wasm32")]
fn spawn_app_task<F>(future: F)
where
    F: Future<Output = ()> + 'static,
{
    spawn_local(future);
}

/// 🩺️ Temporary transition-only tracing so a per-frame predicate can be observed without 60 lines a
/// second. Remove with the `[TRACE]` lines it serves.
pub(crate) fn log_debug_diagnostic_once_per_transition(site: &'static str, state: bool, message: &str) {
    if semio_framework_trace::runtime_diagnostics_enabled() {
        log_debug_once_per_transition(site, state, message);
    }
}

fn log_debug_once_per_transition(site: &'static str, state: bool, message: &str) {
    use std::sync::Mutex;
    static LAST: Mutex<Option<Vec<(&'static str, bool)>>> = Mutex::new(None);
    let mut guard = LAST.lock().expect("debug transition ledger");
    let entries = guard.get_or_insert_with(Vec::new);
    match entries.iter_mut().find(|(key, _)| *key == site) {
        Some(entry) if entry.1 == state => return,
        Some(entry) => entry.1 = state,
        None => entries.push((site, state)),
    }
    drop(guard);
    log_debug(message);
}

/// 🩺️ A PER-FRAME `[TRACE]` dump. Printed only while runtime diagnostics are armed
/// (`SEMIO_RUNTIME_DIAGNOSTICS` — the process environment natively, the url stamp the UI isolate puts
/// on the frame Worker in a browser, `localStorage` in React's own shell), because sixty of these a
/// second per surface is not a trace, it is a mask: the boot fault this gate was added with
/// (`asset retained structure decoder rejected malformed input`) sat under ~2 000 lines of frame
/// census in the console. One-shot and transition-only sites keep using [`log_debug`] directly.
pub(crate) fn log_debug_diagnostic(message: &str) {
    if semio_framework_trace::runtime_diagnostics_enabled() {
        log_debug(message);
    }
}

#[cfg(target_arch = "wasm32")]
fn log_debug(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}

#[cfg(not(target_arch = "wasm32"))]
fn log_debug(message: &str) {
    eprintln!("{message}");
}

//#region 🌓️HostAppearance
/// 🌓️ The persisted `os.config.ui-preferences` appearance as the HOST read it — React's
/// `resolveUiPreferences(preferences, { appearance: "system", … }).appearance`, where
/// [`HostAppearancePreference::Unset`] is React's `preferences.appearance == null` (the store holds
/// no `setAppearance` event) and therefore falls through to the shell's own default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HostAppearancePreference {
    #[default]
    Unset,
    System,
    Light,
    Dark,
}

impl HostAppearancePreference {
    /// 🌓️ The wire spelling — `""` for unset, so a host that read nothing says nothing.
    pub fn from_id(id: &str) -> Self {
        match id {
            "system" => Self::System,
            "light" => Self::Light,
            "dark" => Self::Dark,
            _ => Self::Unset,
        }
    }

    pub fn as_id(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::System => "system",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// 🌓️ The two appearance inputs this renderer cannot read for itself, published by whoever owns a
/// window: the persisted preference above and the host's live `prefers-color-scheme`.
///
/// 🩸️ What this replaces: a `web_sys::window().match_media("(prefers-color-scheme: dark)")` read
/// with an `unwrap_or(true)` fallback. The browser renderer runs inside the frame Worker
/// (`🎞️frame-worker/🟦️.ts`), whose realm owns NO `window` — so that read always missed and every
/// browser boot resolved DARK whatever the machine preferred, while React's own
/// `resolveElementsSurfaceChromeDark` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1691`) resolved LIGHT on the
/// same host. `system_dark` therefore defaults to `false`, byte-for-byte that function's
/// `typeof window === "undefined"` arm, and is fed live by the page thread
/// (`🚀️browser-boot/🟦️.ts`), by the embeddable door (`🎬️renderer-boot/🟦️.ts`) and natively by
/// winit's own `Window::theme()`/`WindowEvent::ThemeChanged` (`🪟️winit-app/🦀️.rs`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct HostAppearance {
    pub preference: HostAppearancePreference,
    pub system_dark: bool,
}

thread_local! {
    static HOST_APPEARANCE: std::cell::Cell<HostAppearance> = const { std::cell::Cell::new(HostAppearance { preference: HostAppearancePreference::Unset, system_dark: false }) };
}

/// 🌓️ Publishes the host's appearance inputs. This value store owns no scheduler: each live host
/// requests a frame after a change, and the next frame's `ThemeResolve` phase re-reads it.
pub fn set_host_appearance(appearance: HostAppearance) {
    HOST_APPEARANCE.with(|cell| cell.set(appearance));
}

/// 🌓️ Both published inputs, for a caller that changes one and must keep the other.
pub fn host_appearance() -> HostAppearance {
    HOST_APPEARANCE.with(|cell| cell.get())
}

/// 🌓️ The persisted preference the host read, or `None` when its store held none — the shell folds
/// this in exactly where React folds `preferences.appearance`.
pub fn host_appearance_preference() -> Option<&'static str> {
    match HOST_APPEARANCE.with(|cell| cell.get().preference) {
        HostAppearancePreference::Unset => None,
        other => Some(other.as_id()),
    }
}

/// 🌓️ wasm appearance hook — the browser twin of winit's `Window::theme()`. The frame Worker has no
/// `window`, so the page thread resolves both inputs and hands them here.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHostAppearance)]
pub fn semio_wgpu_set_host_appearance(preference: String, system_dark: bool) {
    set_host_appearance(HostAppearance { preference: HostAppearancePreference::from_id(&preference), system_dark });
}

fn prefers_dark_scheme() -> bool {
    HOST_APPEARANCE.with(|cell| cell.get().system_dark)
}
//#endregion 🌓️HostAppearance

//#region 🎓️HostIntroduction
thread_local! {
    static HOST_INTRODUCTION_SUPPRESSED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// 🎓️ Carries one host's introduction policy inside its independently owned renderer realm.
pub fn set_host_introduction_suppressed(suppressed: bool) {
    HOST_INTRODUCTION_SUPPRESSED.with(|cell| cell.set(suppressed));
}

/// 🪆️ Reads the current host policy before arming or reconciling its introduction.
pub fn host_introduction_suppressed() -> bool {
    HOST_INTRODUCTION_SUPPRESSED.with(|cell| cell.get())
}

/// 🛰️ Publishes a browser mount's initial or live policy into its own frame Worker.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHostIntroductionSuppressed)]
pub fn semio_wgpu_set_host_introduction_suppressed(suppressed: bool) {
    set_host_introduction_suppressed(suppressed);
}
//#endregion 🎓️HostIntroduction

//#region ⌨️HostPlatform
/// ⌨️ Byte-for-byte React's `keybindingPlatformUsesMetaV1`
/// (`🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`): the ONE rule deciding what `mod` means
/// on this machine. `userAgentData.platform` answers `"macOS"` exactly; the legacy `navigator.platform`
/// strings (`"MacIntel"`, `"iPhone"`, …) are matched case-insensitively, which is what React's
/// `/mac|iphone|ipad|ipod/i` does.
pub fn keybinding_platform_uses_meta(platform: &str) -> bool {
    let platform = platform.to_ascii_lowercase();
    ["mac", "iphone", "ipad", "ipod"].into_iter().any(|needle| platform.contains(needle))
}

/// ⌨️ The host OS as a MANIFEST platform — the axis every `PlatformKeybinding.platform` is declared
/// against. Derived from the SAME string `keybinding_platform_uses_meta` reads, so the door can never
/// answer "this is a Mac" for `mod` and "this is Linux" for a keybinding filter in the same breath.
/// React does not re-derive it per call site either: `ui.platform` is resolved once for the whole
/// shell.
pub fn keybinding_platform_kind(platform: &str) -> semio_framework::manifest::Platform {
    if keybinding_platform_uses_meta(platform) {
        return semio_framework::manifest::Platform::MacOs;
    }
    if platform.to_ascii_lowercase().contains("win") {
        return semio_framework::manifest::Platform::Windows;
    }
    semio_framework::manifest::Platform::Linux
}

/// ⌨️ The compile-time platform a NATIVE build runs on — the initializer both door cells start from.
///
/// 🌐️ `wasm32-unknown-unknown` lands here too, which is exactly the defect this door exists for:
/// the value is a PLACEHOLDER until the page publishes the real one, never an answer.
const fn compiled_host_platform() -> semio_framework::manifest::Platform {
    #[cfg(target_os = "macos")]
    {
        semio_framework::manifest::Platform::MacOs
    }
    #[cfg(target_os = "windows")]
    {
        semio_framework::manifest::Platform::Windows
    }
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    {
        semio_framework::manifest::Platform::Linux
    }
}

thread_local! {
    static HOST_PLATFORM_USES_META: std::cell::Cell<bool> = const { std::cell::Cell::new(cfg!(target_os = "macos")) };
    static HOST_PLATFORM: std::cell::Cell<semio_framework::manifest::Platform> = const { std::cell::Cell::new(compiled_host_platform()) };
}

/// ⌨️ Publishes the host's platform reading. Natively nothing calls this: the initializer already IS
/// the OS answer (`cfg!(target_os = "macos")`), the same compile-time fact winit runs on. In the
/// browser the frame Worker realm owns no `navigator.platform` worth trusting — a dedicated Worker
/// reports the page's own user agent, but `userAgentData.platform` is page-thread-only — so the page
/// resolves it and hands it across with the boot message.
pub fn set_host_platform_uses_meta(uses_meta: bool) {
    HOST_PLATFORM_USES_META.with(|cell| cell.set(uses_meta));
}

/// ⌨️ Publishes the host's platform KIND. Always set together with [`set_host_platform_uses_meta`] by
/// [`set_host_platform`]; separate setters exist only because the `mod` rule and the manifest axis are
/// two different questions with two different answers on `iphone`/`ipad`.
pub fn set_host_platform_kind(platform: semio_framework::manifest::Platform) {
    HOST_PLATFORM.with(|cell| cell.set(platform));
}

/// ⌨️ Publishes BOTH platform readings from the one string the page resolved — the only setter any
/// door should call.
pub fn set_host_platform(platform: &str) {
    set_host_platform_uses_meta(keybinding_platform_uses_meta(platform));
    set_host_platform_kind(keybinding_platform_kind(platform));
}

/// ⌨️ Whether `mod` is Command on this host — read by every chord formatter in the shell.
pub fn host_platform_uses_meta() -> bool {
    HOST_PLATFORM_USES_META.with(std::cell::Cell::get)
}

/// ⌨️ The host platform every `PlatformKeybinding.platform` filter is answered against — read by the
/// shell's `command_host_platform()`.
pub fn host_platform() -> semio_framework::manifest::Platform {
    HOST_PLATFORM.with(std::cell::Cell::get)
}

/// ⌨️ wasm platform hook — the browser twin of `cfg!(target_os = "macos")`.
///
/// 🩸️ What this replaces: `format_keybinding_shortcut` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) read
/// `cfg!(target_os = "macos")`, which is FALSE on `wasm32-unknown-unknown` whatever machine the page
/// runs on. A macOS browser therefore painted `Ctrl+Alt+E` on the navbar role chip where React,
/// reading the live navigator, painted `⌘️⌥️E` — and the chord that actually fires is Command.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHostPlatform)]
pub fn semio_wgpu_set_host_platform(platform: String) {
    set_host_platform(&platform);
}
//#endregion ⌨️HostPlatform

//#region 🗣️HostLocale
/// 🗣️ Byte-for-byte React's `normalizeUiLocale` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx`): a BCP47 tag
/// (`navigator.language`, `"de-AT"`) folds onto the chrome's two tongues. `""` is "the host said
/// nothing", which leaves the door unset rather than pinning a tongue nobody chose.
pub fn host_locale_from_tag(tag: &str) -> Option<&'static str> {
    semio_framework_ui_locale::Locale::from_language_tag(tag).ok().map(|locale| locale.as_str())
}

thread_local! {
    static HOST_LOCALE: std::cell::Cell<Option<&'static str>> = const { std::cell::Cell::new(None) };
}

/// 🗣️ Publishes the host's own language read. The shell folds it in exactly where React's
/// `ShellHost` folds `detectShellLocale(navigator.language)`: after a lock and after the persisted
/// preference, never over either.
pub fn set_host_locale(tag: &str) {
    HOST_LOCALE.with(|cell| cell.set(host_locale_from_tag(tag)));
}

/// 🗣️ The host's published tongue, or `None` when no door has spoken.
pub fn host_locale() -> Option<&'static str> {
    HOST_LOCALE.with(std::cell::Cell::get)
}

/// 🗣️ wasm locale hook — the browser twin of React's `navigator.language` read.
///
/// 🩸️ What this replaces: nothing reached the renderer. The page resolved `de` for a German
/// browser and handed it to the frame Worker, which spent it on one boot error message; the shell
/// fell through to `"en"`, so a de-DE user saw English chrome where React, on the same browser,
/// spoke German.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHostLocale)]
pub fn semio_wgpu_set_host_locale(tag: String) {
    set_host_locale(&tag);
}
//#endregion 🗣️HostLocale

/// 🌓️ Byte-for-byte React's `resolveElementsSurfaceChromeDark` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1691`):
/// `"dark"` is dark, `"light"` is light, everything else (including `"system"`) asks the host.
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

//#region 🔖️AsyncBoundaryTests
#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs"]
mod async_boundary_tests;
//#endregion 🔖️AsyncBoundaryTests

//#region 🔖️FrameActionLedgerTests
#[cfg(test)]
#[path = "../../../🧪️tests/🧾️frame-action-ledger/🦀️.rs"]
mod frame_action_ledger_tests;
//#endregion 🔖️FrameActionLedgerTests

//#region 🫀️SettlePumpTests
#[cfg(test)]
#[path = "../../../🧪️tests/🫀️settle-pump/🦀️.rs"]
mod settle_pump_tests;
//#endregion 🫀️SettlePumpTests

//#region 🔖️WheelApplicationPointTests
#[cfg(test)]
#[path = "../../../🧪️tests/🖱️wheel-application-point/🦀️.rs"]
mod wheel_application_point_tests;
//#endregion 🔖️WheelApplicationPointTests

//#region 🧭️BootAxisParityTests
#[cfg(test)]
#[path = "../../../🧪️tests/🧭️boot-axis-parity/🦀️.rs"]
mod boot_axis_parity_tests;
//#endregion 🧭️BootAxisParityTests

//#region 🧭️BootDescriptor
// 🧭️ ONE boot-axis vocabulary for all three wgpu entry points — the trunk page
// (`🚀️browser-boot/🟦️.ts`), the embeddable library door (`🎬️renderer-boot/🟦️.ts`) and the native CLI
// (`⌨️native-entrypoint/🦀️.rs`) — and the Rust twin of `🧭️boot-descriptor/🟦️.ts`. React has a single
// door (`FrameworkOsBootOptions`); wgpu had three, each carrying a different subset of the same axes,
// so `--plugin cad --example concrete-forest` could not open natively what
// `?plugin=cad&example=concrete-forest` opened in the browser. The standing law that keeps the four
// field sets equal is `🧪️tests/🧭️boot-axis-parity/🦀️.rs`.
//
// Still a `thread_local` a caller opts into reading rather than a parameter threaded through every
// mount signature: the shell reads it while it opens the session (`ShellState::boot`), which is why
// every door applies it BEFORE mounting.

/// 📏️ Bound on ONE descriptor field, the twin of `🧭️boot-descriptor/🟦️.ts`'s
/// `WGPU_BOOT_FIELD_CAPACITY` — the browser door refuses an oversized url before it ever reaches
/// here, and this is the same refusal for the native and library doors.
pub const WGPU_BOOT_FIELD_CAPACITY: usize = 2048;

/// 🔒️ Boot-time preference LOCKS — React's `FrameworkOsLocks`. `""` means unset, never "locked to the
/// empty string"; a locked preference loses its in-app switcher.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootLocks {
    pub example_id: String,
    pub locale: String,
    pub terminology: String,
    pub theme_id: String,
    pub appearance: String,
}

/// 🎛️ Boot-time preference DEFAULTS — React's `FrameworkOsDefaults`: seeds that KEEP their switcher.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootDefaults {
    pub example_id: String,
}

/// 🏷️ The brand REGISTRY row this boot resolved — React's `resolveShellBrandById(brandId)` answer
/// (`🧑‍💻dev/🏷️brand/🟦️.ts`), reduced to the facts the shell itself reads. The table stays where it is
/// (a TypeScript catalogue over the demonstrator's own brand definitions); what crosses into the
/// renderer is the resolved ROW, exactly as `locks`/`defaults` already do — so a brand added there
/// needs no Rust edit and the two halves cannot drift into two catalogues.
///
/// `brand_id` alone was carried before this (`📓️w1d-boot-axis-parity.md` gap 1): the shell knew it was
/// branded and nothing else, which is why the tour's persisted-seen key dropped React's brand prefix
/// on every branded playground (`📓️w5a-browser-prefs-persistence.md` §7.1, demonstrated on
/// `aggregator`).
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootBrand {
    /// 🏷️ React's `ShellBrand.windowTitle` — the native window's title; `""` leaves the default.
    pub window_title: String,
    /// 🧊️ React's `ShellBrand.ephemeral`: the shell never reads or writes device-local state.
    pub ephemeral: bool,
    /// 🎓️ React's `ShellBrand.replayIntroductionOnLoad` — auto-start every load, persist no seen flag.
    /// `isEphemeralShellBrand` implies it there, and [`WgpuBootBrand::replays_introduction`] implies it
    /// here, so the two predicates cannot disagree.
    pub replay_introduction_on_load: bool,
}

impl WgpuBootBrand {
    /// 🎓️ React's `shouldReplayIntroductionOnLoad(brand)`.
    pub fn replays_introduction(&self) -> bool {
        self.ephemeral || self.replay_introduction_on_load
    }

    /// 🎓️ React's `shouldPersistIntroductionSeen(brand)`.
    pub fn persists_introduction_seen(&self) -> bool {
        !self.replays_introduction()
    }
}

/// 🌐️ The hub trio (`?hub=&user=&dataDir=`, `--hub/--user/--data-dir`).
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootHub {
    pub hub_url: String,
    pub user: String,
    pub data_dir: String,
}

/// 🧭️ Every boot axis, resolved. Absent axes are `""`/`Default` rather than `Option`, so the three
/// doors cannot disagree about what "unset" serializes to.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WgpuBootDescriptor {
    pub plugin_variant: String,
    pub app_id: String,
    pub app_role: String,
    pub app_mode: String,
    pub app_example: String,
    pub brand_id: String,
    pub brand: WgpuBootBrand,
    pub broker_proof: String,
    pub locks: WgpuBootLocks,
    pub defaults: WgpuBootDefaults,
    pub hub: Option<WgpuBootHub>,
}

impl WgpuBootDescriptor {
    /// 📏️ Refuses an oversized field by NAME, so the fault says which axis overflowed — the Rust half
    /// of `boundedBootField`. Applied before any axis is stored, never after.
    pub fn validated(self) -> Result<Self, String> {
        let hub = self.hub.as_ref();
        let fields: [(&str, &str); 14] = [
            ("plugin", self.plugin_variant.as_str()),
            ("app", self.app_id.as_str()),
            ("role", self.app_role.as_str()),
            ("mode", self.app_mode.as_str()),
            ("example", self.app_example.as_str()),
            ("brand", self.brand_id.as_str()),
            ("brand.windowTitle", self.brand.window_title.as_str()),
            ("brokerProof", self.broker_proof.as_str()),
            ("locks.exampleId", self.locks.example_id.as_str()),
            ("locks.locale", self.locks.locale.as_str()),
            ("locks.terminology", self.locks.terminology.as_str()),
            ("locks.themeId", self.locks.theme_id.as_str()),
            ("locks.appearance", self.locks.appearance.as_str()),
            ("defaults.exampleId", self.defaults.example_id.as_str()),
        ];
        let hub_fields: [(&str, &str); 3] = [("hub", hub.map_or("", |hub| hub.hub_url.as_str())), ("user", hub.map_or("", |hub| hub.user.as_str())), ("dataDir", hub.map_or("", |hub| hub.data_dir.as_str()))];
        for (field, value) in fields.into_iter().chain(hub_fields) {
            if value.chars().count() > WGPU_BOOT_FIELD_CAPACITY {
                return Err(format!("boot-descriptor-overflow: {field} exceeds {WGPU_BOOT_FIELD_CAPACITY} code units"));
            }
        }
        Ok(self)
    }
}

thread_local! {
    static BOOT_DESCRIPTOR: RefCell<WgpuBootDescriptor> = RefCell::new(resolve_environment_boot_descriptor());
}

/// 🖥️ The native per-server seeds, read straight off the process env under the SAME names React's
/// serve projects into `VITE_SEMIO_*` (`🧑‍💻dev/🟦️.ts`). The CLI's own per-navigation flags overwrite
/// these in `apply_boot_descriptor`; a browser build has no process env and starts empty.
///
/// 🏷️ The resolved brand ROW, under the same `SEMIO_*` names the serve injects as
/// `<meta semio-brand-*>` for the browser door — one vocabulary, three doors.
#[cfg(not(target_arch = "wasm32"))]
fn resolve_environment_boot_descriptor() -> WgpuBootDescriptor {
    let read = |name: &str| std::env::var(name).ok().filter(|value| !value.is_empty()).unwrap_or_default();
    WgpuBootDescriptor {
        app_id: read("SEMIO_APP_ID"),
        app_role: read("SEMIO_APP_ROLE"),
        brand_id: read("SEMIO_BRAND"),
        brand: WgpuBootBrand { window_title: read("SEMIO_BRAND_WINDOW_TITLE"), ephemeral: read("SEMIO_BRAND_EPHEMERAL") == "true", replay_introduction_on_load: read("SEMIO_BRAND_REPLAY_INTRODUCTION") == "true" },
        app_example: read("SEMIO_DEFAULT_EXAMPLE"),
        defaults: WgpuBootDefaults { example_id: read("SEMIO_DEFAULT_EXAMPLE") },
        locks: WgpuBootLocks { example_id: read("SEMIO_LOCKED_EXAMPLE"), locale: read("SEMIO_LOCKED_LOCALE"), terminology: read("SEMIO_LOCKED_TERMINOLOGY"), theme_id: read("SEMIO_LOCKED_THEME"), appearance: read("SEMIO_LOCKED_APPEARANCE") },
        ..WgpuBootDescriptor::default()
    }
}

#[cfg(target_arch = "wasm32")]
fn resolve_environment_boot_descriptor() -> WgpuBootDescriptor {
    WgpuBootDescriptor::default()
}

/// 🧭️ Installs the resolved axes, replacing the environment seeds this thread started from. Every
/// door calls it once, before the mount that opens the session.
pub fn apply_boot_descriptor(descriptor: WgpuBootDescriptor) -> Result<(), String> {
    let descriptor = descriptor.validated()?;
    BOOT_DESCRIPTOR.with(|cell| *cell.borrow_mut() = descriptor);
    Ok(())
}

/// 🌐️ wasm boot hook — the frame Worker (`🎞️frame-worker/🟦️.ts`) and the page-mounted library door
/// (`🎬️renderer-boot/🟦️.ts`) each call this once with the JSON `🧭️boot-descriptor/🟦️.ts` resolved.
/// 🩺️ wasm diagnostics hook — the browser twin of the `SEMIO_RUNTIME_DIAGNOSTICS` environment
/// variable a native process reads and of the `localStorage` key React's `ShellHost` reads. A Worker
/// realm owns neither, so the UI isolate resolves the preference, stamps it on the frame Worker's url
/// (`🎭️actor/🩺️diagnostics/🟦️.ts`'s `stampShardWorkerDiagnostics`) and the Worker hands it here —
/// before the first frame, because every per-frame `[TRACE]` dump in this crate is gated on it.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetRuntimeDiagnostics)]
pub fn semio_wgpu_set_runtime_diagnostics(enabled: bool) {
    semio_framework_trace::set_runtime_diagnostics(enabled);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetBootDescriptor)]
pub fn semio_wgpu_set_boot_descriptor(descriptor_json: String) -> Result<(), JsValue> {
    let descriptor: WgpuBootDescriptor = serde_json::from_str(&descriptor_json).map_err(|error| JsValue::from_str(&format!("boot-descriptor parse: {error}")))?;
    apply_boot_descriptor(descriptor).map_err(|error| JsValue::from_str(&error))
}

/// 🧭️ The whole resolved descriptor, for a reader that needs more than one axis.
pub fn boot_descriptor() -> WgpuBootDescriptor {
    BOOT_DESCRIPTOR.with(|cell| cell.borrow().clone())
}

fn boot_axis(read: impl Fn(&WgpuBootDescriptor) -> &str) -> Option<String> {
    BOOT_DESCRIPTOR.with(|cell| {
        let descriptor = cell.borrow();
        let value = read(&descriptor);
        (!value.is_empty()).then(|| value.to_string())
    })
}

/// 📌️ The boot-pinned app id, or `None`. React's `appId`: an empty value is NOT a pin, it falls
/// through to the variant's own default app (`🧑‍💻dev/🟦️.ts`'s own note on `VITE_SEMIO_APP_ID`).
pub fn boot_app_id() -> Option<String> {
    boot_axis(|descriptor| &descriptor.app_id)
}

/// 🎭️ The boot-requested mode id, or `None` when nothing named one. A mode the open app does not
/// declare is ignored by the reader (`ShellState::apply_boot_mode`), never a boot failure.
pub fn boot_app_mode() -> Option<String> {
    boot_axis(|descriptor| &descriptor.app_mode)
}

/// 📚️ The boot-requested example id, or `None`. It already collapses `?example=`/`--example` over the
/// `SEMIO_DEFAULT_EXAMPLE` seed the way React collapses the query over `VITE_SEMIO_DEFAULT_EXAMPLE`
/// into ONE `defaults.exampleId`. An id the open DIALECT does not author is ignored by the reader
/// (`shell::resolve_boot_example_id`), never a boot failure.
pub fn boot_app_example() -> Option<String> {
    boot_axis(|descriptor| &descriptor.app_example)
}

/// 🏷️ The boot brand id, or `None` — React's `brand`, resolved there by `resolveShellBrandById`.
pub fn boot_brand_id() -> Option<String> {
    boot_axis(|descriptor| &descriptor.brand_id)
}

/// 🏷️ The resolved brand ROW — React's `resolveShellBrandById(brandId)` answer. An UNBRANDED boot
/// answers `WgpuBootBrand::default()`, whose predicates read exactly as React's do for `undefined`
/// (`shouldReplayIntroductionOnLoad(undefined) === false`), so no caller needs an `Option` arm.
pub fn boot_brand() -> WgpuBootBrand {
    BOOT_DESCRIPTOR.with(|cell| cell.borrow().brand.clone())
}

/// 🏷️ The shell window's title — React's `ShellBrand.windowTitle`, `"Semio"` when this boot resolved no brand — which the
/// native window shows and its platform accessibility root announces.
pub fn boot_window_title() -> String {
    let title = boot_brand().window_title;
    if title.is_empty() {
        "Semio".to_string()
    } else {
        title
    }
}

/// #️⃣ The one-shot `#semio-broker=` proof the page carried, or `None`. React reads the same hash at
/// `🏛️ShellHost/🟦️.tsx:209` and hands it to its browser broker client.
pub fn boot_broker_proof() -> Option<String> {
    boot_axis(|descriptor| &descriptor.broker_proof)
}

/// 🔒️ The boot preference locks — read by the shell's `env_lock`, so a browser build locks exactly as
/// a native `SEMIO_LOCKED_*` deployment does instead of being unlockable for want of a process env.
pub fn boot_locks() -> WgpuBootLocks {
    BOOT_DESCRIPTOR.with(|cell| cell.borrow().locks.clone())
}

/// 👁️✏️ The boot-resolved role, contract freeze §5 — `"viewer"`/`"editor"`, anything else is
/// `ChromeRole::from_boot_env`'s own `editor` fallback.
pub fn boot_app_role() -> ui_wgpu::wgpu::component::role_chrome::ChromeRole {
    BOOT_DESCRIPTOR.with(|cell| ui_wgpu::wgpu::component::role_chrome::ChromeRole::from_boot_env(Some(cell.borrow().app_role.as_str())))
}
//#endregion 🧭️BootDescriptor

#[cfg(target_arch="wasm32")]
#[wasm_bindgen(js_name=semioWgpuSetPluginCatalog)]
pub fn semio_wgpu_set_plugin_catalog(serialized:String)->Result<(),JsValue>{install_plugin_catalog_json_v1(&serialized).map_err(|error|JsValue::from_str(&error))}
