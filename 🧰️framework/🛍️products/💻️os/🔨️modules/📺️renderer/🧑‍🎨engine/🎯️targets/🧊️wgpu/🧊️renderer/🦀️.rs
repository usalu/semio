use semio_framework_job::{InteractiveJobCloseStep, RetainedCloneGrant, RetainedCloneProgress};
// 🧭️ Mount hubs: the module mounts below live in one file per work package of PLAN-RENDERER-WGPU-CRATES.md,
// spliced into this crate root, so every `crate::<module>` path is unchanged.
include!("📥️mounts/🌱️wp1-foundation/🦀️.rs");
include!("📥️mounts/🎞️wp2-scenes/🦀️.rs");
include!("📥️mounts/🧵️wp3-frame-core/🦀️.rs");
include!("📥️mounts/🐚️wp4-shell-state/🦀️.rs");
include!("📥️mounts/🎛️wp5-shell-views/🦀️.rs");
#[cfg(target_arch = "wasm32")]
extern crate semio_framework_async as wasm_bindgen_futures;

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

use semio_framework_os_renderer_camera as camera_storage;

/// 🔎️ The whole renderer source tree as one text, for the source-text law tests that pin where the renderer calls what.

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
use semio_framework_ui_declaration::layout::ActionDescriptor;
use semio_framework_ui_raster_owner::{SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterPool, SceneRasterProfile, SceneRasterWriteMode, SceneRasterWriter};
// 🏚️ `dispatch_window_event`/`WindowInputState`/`schedule_frame` no longer imported here — they were
// `SemioApp`/`start_frame_loop`-only (both deleted, packet os-host); `winit_app.rs` normalizes input
// itself via `ui_host::event` instead. See the `OsHostDecomposition — SemioApp deletion` region above.
#[cfg(target_arch = "wasm32")]
use semio_framework_async::browser::spawn_local;
use semio_framework_ui_wgpu_hosting::cursor::{apply_window_cursor, resolve_semio_cursor, CursorDragState, SemioCursor};
use semio_framework_ui_text::{fetch_font_bytes, FontAtlas};
use ui_scene::math::{mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_read_write_u32, mesh3d_read_write_vec3, mesh3d_seal, mesh3d_update_vec3, mesh3d_write_u32, mesh3d_write_edge, mesh3d_write_vec2, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dFault, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken};
use semio_framework_ui_draw_contract::{DrawList, IconAtlas};
use semio_framework_ui_wgpu_input::{InputState, KeyAction, PointerModifiers};
use semio_framework_ui_theme::Theme;
use ui_gpu::{GpuContext};
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

pub use semio_framework_os_renderer_wgpu_host_base::{action_args_json, mesh_assets, plugin_catalog, runtime_assets};
pub(crate) use semio_framework_os_renderer_wgpu_host_base::frame_latency;
pub use semio_framework_os_renderer_wgpu_host_base::{HostAppearance, HostAppearancePreference, WGPU_BOOT_FIELD_CAPACITY, WgpuBootBrand, WgpuBootDefaults, WgpuBootDescriptor, WgpuBootHub, WgpuBootLocks, apply_boot_descriptor, boot_app_example, boot_app_id, boot_app_mode, boot_app_role, boot_brand, boot_brand_id, boot_broker_proof, boot_descriptor, boot_locks, boot_window_title, host_appearance, host_appearance_preference, host_introduction_suppressed, host_locale, host_locale_from_tag, host_platform, host_platform_uses_meta, keybinding_platform_kind, keybinding_platform_uses_meta, set_host_appearance, set_host_introduction_suppressed, set_host_locale, set_host_platform, set_host_platform_kind, set_host_platform_uses_meta};
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use semio_framework_os_renderer_wgpu_host_base::KernelPoolFuture;
pub(crate) use semio_framework_os_renderer_wgpu_host_base::{app_now_ms, appearance_is_dark, boot_axis, log_debug, log_debug_diagnostic, log_debug_diagnostic_once_per_transition, log_debug_once_per_transition, prefers_dark_scheme, renderer_worker_pool, resolve_environment_boot_descriptor, resolve_theme, spawn_app_task};
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use semio_framework_os_renderer_wgpu_host_base::{RENDERER_IO_CHECKED_OUT, RENDERER_IO_EXHAUSTED, RENDERER_IO_LIVE, RENDERER_IO_PUMP_CURSOR, RENDERER_IO_SESSION_SLOTS, RENDERER_IO_SLOTS, RENDERER_IO_VACANT, RENDERER_IO_WAKE, RendererIoHandle, RendererIoNode, RendererIoSlot, actor_budget_from_turn_budget, drive_renderer_worker_task, renderer_io_generation_live, renderer_io_sessions_pending, renderer_io_with_node, run_renderer_io, submit_renderer_io};

//#region 🌓️HostAppearance
/// 🌓️ wasm appearance hook — the browser twin of winit's `Window::theme()`. The frame Worker has no
/// `window`, so the page thread resolves both inputs and hands them here.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHostAppearance)]
pub fn semio_wgpu_set_host_appearance(preference: String, system_dark: bool) {
    set_host_appearance(HostAppearance { preference: HostAppearancePreference::from_id(&preference), system_dark });
}
//#endregion 🌓️HostAppearance

//#region 🎓️HostIntroduction
/// 🛰️ Publishes a browser mount's initial or live policy into its own frame Worker.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHostIntroductionSuppressed)]
pub fn semio_wgpu_set_host_introduction_suppressed(suppressed: bool) {
    set_host_introduction_suppressed(suppressed);
}
//#endregion 🎓️HostIntroduction

//#region ⌨️HostPlatform
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

//#region 🧭️BootDescriptor
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
//#endregion 🧭️BootDescriptor

#[cfg(target_arch="wasm32")]
#[wasm_bindgen(js_name=semioWgpuSetPluginCatalog)]
pub fn semio_wgpu_set_plugin_catalog(serialized:String)->Result<(),JsValue>{install_plugin_catalog_json_v1(&serialized).map_err(|error|JsValue::from_str(&error))}

//#region 🏠️ShellHostHooks
/// 🏠️ wasm hub environment hook: the page hands the hub url, user and data directory to the Shell before the first frame.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHubEnv)]
pub fn semio_wgpu_set_hub_env(hub_url: String, user: String, data_dir: String) {
    let data_dir = if data_dir.is_empty() { None } else { Some(data_dir) };
    shell::BOOT_HUB_ENV.with(|cell| *cell.borrow_mut() = Some((hub_url, user, data_dir)));
}

/// 🪟️ wasm window blur hook: the page tells the Shell its host window lost focus.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuHostWindowBlur)]
pub fn semio_wgpu_host_window_blur() {
    shell::note_host_window_blur();
}

/// 🗄️ wasm host storage hook: the page seeds the Shell's persisted keys from its own storage.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetHostStorage)]
pub fn semio_wgpu_set_host_storage(snapshot_json: String) {
    let accepted = shell::seed_host_storage(&snapshot_json);
    shell::debug_log(&format!("[TRACE] wgpu-shell host storage seeded entries={accepted}"));
}

/// 🔭️ wasm agent bridge diagnostics hook: reads the offer scope the Shell published.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = dumpAgentBridgeOfferScope)]
pub fn dump_agent_bridge_offer_scope() -> String {
    shell::PUBLISHED_AGENT_BRIDGE_OFFER_SCOPE.with(|slot| slot.borrow().clone())
}

/// 🌉️ wasm agent bridge hook: arms the bridge configuration the next frame admits.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetAgentBridgeConfig)]
pub fn semio_wgpu_set_agent_bridge_config(url: String, admission_proof: String) {
    let config = if url.trim().is_empty() {
        None
    } else {
        match crate::agent_bridge::AgentBridgeConfig::admit(&url, &admission_proof) {
            Ok(config) => Some(config),
            Err(error) => {
                shell::debug_log(&format!("[TRACE] wgpu-shell agent bridge config refused: {error}"));
                return;
            }
        }
    };
    let armed = config.is_some();
    shell::PENDING_AGENT_BRIDGE_CONFIG.with(|slot| *slot.borrow_mut() = Some(config));
    shell::debug_log(&format!("[TRACE] wgpu-shell agent bridge config armed={armed}"));
}
//#endregion 🏠️ShellHostHooks

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use semio_framework_os_renderer_wgpu_host_plumbing::native_launch;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use semio_framework_os_renderer_wgpu_host_plumbing::native_io::{RendererIoPath, RendererSourceIoHandle, RendererSourceIoError, RendererSourceIoFailureKind, open_renderer_source_io, read_renderer_source_bytes, read_renderer_resident_bytes, find_renderer_wasm_path, start_renderer_modified, renderer_source_io_pending, pump_renderer_io_sessions};

