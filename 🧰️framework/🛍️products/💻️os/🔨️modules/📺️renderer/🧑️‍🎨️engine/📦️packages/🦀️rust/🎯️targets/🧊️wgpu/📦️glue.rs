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
extern crate semio_framework_os_kernel as vcs;
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
#[path = "../../../../../../🔌️plugin/📦️packages/🟦️typescript/📇️registry/🤖️generated/🦀️hosts.rs"]
mod generated_plugin_hosts;
//#endregion 🏠️🧳️PluginHostConfig

#[path = "../../../../🧱️elements/Scenes/🧊️component.rs"]
pub mod scenes;

#[path = "../../../../🧱️elements/Shell/🧊️component.rs"]
pub mod shell;

#[path = "../../../../🧱️elements/IconRenderHost/🧊️component.rs"]
pub mod icon_atlas;

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
use ui_wgpu::wgpu::{
    apply_window_cursor, dispatch_window_event, fetch_font_bytes, resolve_semio_cursor, schedule_frame, CursorDragState, DrawList, FontAtlas, GpuContext, IconAtlas, InputState, KeyAction, PointerCallbacks, PointerModifiers, SemioCursor, Theme,
    WindowInputState,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes, WindowId};

fn spawn_app_task<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    #[cfg(target_arch = "wasm32")]
    spawn_local(future);
    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(future);
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
    let mut response = ureq::get(&resolved).call().ok()?;
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
                log_debug(&format!("[DEBUG] wasm program reload failed: {error}"));
                return;
            }
        };
        self.shell.prepare_hot_reload(entries);
        if let Err(error) = pollster::block_on(self.shell.boot()) {
            log_debug(&format!("[DEBUG] wasm program hot reload failed: {error}"));
        } else {
            log_debug("[DEBUG] wasm program hot reload complete");
        }
    }

    fn frame(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.poll_native_plugin_hot_swap();
            self.maybe_reload_native_plugins();
            pollster::block_on(self.shell.pump_sync_events());
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
            log_debug(&format!("[DEBUG] render frame: {err}"));
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
                                log_debug(&format!("[DEBUG] keyboard failed: {err}"));
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
                        log_debug(&format!("[DEBUG] keyboard failed: {err}"));
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
                log_debug(&format!("[DEBUG] action failed: {err}"));
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
                log_debug(&format!("[DEBUG] pointer failed: {err}"));
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
            log_debug(&format!("[DEBUG] pointer failed: {err}"));
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
            log_debug(&format!("[DEBUG] deferred actions: {err}"));
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

fn start_frame_loop(window: Arc<Window>, runtime: Rc<RefCell<AppRuntime>>) {
    let next = runtime.clone();
    let window_next = window.clone();
    schedule_frame(&window, move || {
        if let Ok(mut app) = next.try_borrow_mut() {
            app.frame();
        }
        start_frame_loop(window_next.clone(), next.clone());
    });
}

enum HostUserEvent {
    RuntimeReady { runtime: Rc<RefCell<AppRuntime>>, callbacks: PointerCallbacks },
}

struct SemioApp {
    proxy: EventLoopProxy<HostUserEvent>,
    plugin_filter: String,
    #[cfg(target_arch = "wasm32")]
    plugins: Option<wasm_bindgen::JsValue>,
    #[cfg(target_arch = "wasm32")]
    canvas: Option<web_sys::HtmlCanvasElement>,
    #[cfg(not(target_arch = "wasm32"))]
    plugin_modules_root: std::path::PathBuf,
    window: Option<Arc<Window>>,
    runtime: Option<Rc<RefCell<AppRuntime>>>,
    callbacks: Option<PointerCallbacks>,
    window_input: WindowInputState,
}

impl SemioApp {
    fn new(
        proxy: EventLoopProxy<HostUserEvent>,
        plugin_filter: String,
        #[cfg(target_arch = "wasm32")] plugins: Option<wasm_bindgen::JsValue>,
        #[cfg(target_arch = "wasm32")] canvas: Option<web_sys::HtmlCanvasElement>,
        #[cfg(not(target_arch = "wasm32"))] plugin_modules_root: std::path::PathBuf,
    ) -> Self {
        Self {
            proxy,
            plugin_filter,
            #[cfg(target_arch = "wasm32")]
            plugins,
            #[cfg(target_arch = "wasm32")]
            canvas,
            #[cfg(not(target_arch = "wasm32"))]
            plugin_modules_root,
            window: None,
            runtime: None,
            callbacks: None,
            window_input: WindowInputState::default(),
        }
    }
}

impl ApplicationHandler<HostUserEvent> for SemioApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let mut attributes = WindowAttributes::default().with_title("Semio");
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowAttributesExtWebSys;
            if let Some(canvas) = self.canvas.clone() {
                let dpr = web_sys::window().map(|window| window.device_pixel_ratio() as f32).unwrap_or(1.0);
                let css_width = canvas.client_width().max(1) as f32;
                let css_height = canvas.client_height().max(1) as f32;
                let _ = canvas.style().set_property("width", "100%");
                let _ = canvas.style().set_property("height", "100vh");
                attributes = attributes.with_inner_size(winit::dpi::LogicalSize::new(css_width, css_height)).with_canvas(Some(canvas)).with_append(true);
                let _ = dpr;
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            attributes = attributes.with_inner_size(winit::dpi::LogicalSize::new(1280.0, 800.0));
        }
        let window = Arc::new(event_loop.create_window(attributes).expect("create window"));
        self.window = Some(window.clone());
        let proxy = self.proxy.clone();
        let plugin_filter = self.plugin_filter.clone();
        #[cfg(target_arch = "wasm32")]
        let plugins = self.plugins.clone();
        #[cfg(not(target_arch = "wasm32"))]
        let plugin_modules_root = self.plugin_modules_root.clone();
        spawn_app_task(async move {
            let result = boot_runtime(
                window,
                plugin_filter,
                #[cfg(target_arch = "wasm32")]
                plugins,
                #[cfg(not(target_arch = "wasm32"))]
                plugin_modules_root,
            )
            .await;
            match result {
                Ok((runtime, callbacks)) => {
                    let _ = proxy.send_event(HostUserEvent::RuntimeReady { runtime, callbacks });
                }
                Err(error) => log_debug(&format!("[DEBUG] boot_runtime failed: {error}")),
            }
        });
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: HostUserEvent) {
        if let HostUserEvent::RuntimeReady { runtime, callbacks } = event {
            if let Some(window) = self.window.clone() {
                start_frame_loop(window, runtime.clone());
            }
            self.runtime = Some(runtime);
            self.callbacks = Some(callbacks);
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.clone() else {
            return;
        };
        match &event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(runtime) = self.runtime.as_ref() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        let dpr = window.scale_factor() as f32;
                        app.resize(size.width as f32 / dpr, size.height as f32 / dpr, dpr);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(runtime) = self.runtime.as_ref() {
                    if let Ok(mut app) = runtime.try_borrow_mut() {
                        app.frame();
                    }
                }
                window.request_redraw();
            }
            _ => {
                if let Some(callbacks) = self.callbacks.as_ref() {
                    dispatch_window_event(&window, &event, &mut self.window_input, callbacks);
                }
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

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
    let atlas = FontAtlas::from_bytes(&font_bytes).map_err(|err| format!("[DEBUG] atlas failed: {err}"))?;
    let icons = icon_atlas::build_icon_atlas();
    let mut gpu = GpuContext::from_window(window.clone()).await.map_err(|err| format!("[DEBUG] gpu init failed: {err}"))?;
    gpu.resize(css_width, css_height, dpr);
    gpu.upload_font_atlas(&atlas);
    gpu.upload_icon_atlas(&icons);

    #[cfg(target_arch = "wasm32")]
    let entries = {
        let plugins = plugins.ok_or("missing wasm programs")?;
        filter_plugins(parse_plugin_entries(plugins).map_err(|err| format!("[DEBUG] program parse failed: {err}"))?, &plugin_filter)
    };
    #[cfg(not(target_arch = "wasm32"))]
    let entries = filter_plugins(load_wasm_plugins(&plugin_filter, &plugin_modules_root)?, &plugin_filter);

    let mut shell = ShellState::new(entries, plugin_filter.clone());
    shell.screen_w = css_width * dpr;
    shell.screen_h = css_height * dpr;
    shell.boot().await.map_err(|err| format!("[DEBUG] shell boot failed: {err}"))?;

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

    log_debug("[DEBUG] wgpu renderer booted");
    Ok((runtime, callbacks))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_native(plugin_filter: &str, plugin_modules_root: std::path::PathBuf) {
    let event_loop = EventLoop::<HostUserEvent>::with_user_event().build().expect("event loop");
    let proxy = event_loop.create_proxy();
    let mut app = SemioApp::new(proxy, plugin_filter.to_string(), plugin_modules_root);
    let _ = event_loop.run_app(&mut app);
}

/// 🐚️ Multi-mount: takes an already-created, already-placed canvas from the caller instead of looking
/// up a hardcoded `#root`/`#semio-wgpu-canvas` and taking it over via `set_inner_html("")` — that
/// single-mount assumption meant a second boot call would wipe the first mount's canvas and collide on
/// the same DOM id. The caller (`bootFrameworkOsWgpu` in `📦️index.ts`) now owns creating and placing
/// the canvas, so N independent mounts can coexist on one page.
///
/// Known gap (not yet done — see the plan's Wave 6 D11 notes): this does not yet return a disposable
/// handle. `start_frame_loop`/`schedule_frame` (this crate's `ui_wgpu` dependency) reschedule themselves
/// via a fire-and-forget `requestAnimationFrame` closure with no captured cancellation id, independent of
/// the winit event loop's own control flow — so `ActiveEventLoop::exit()` alone (the pattern already used
/// for `WindowEvent::CloseRequested` below) would NOT stop rendering; the recursive rAF chain would keep
/// calling `app.frame()` forever. A real `semioWgpuUnmount` needs a shared disposal flag threaded through
/// `AppRuntime`/`start_frame_loop` that the frame closure checks before each reschedule, verified against
/// an actual browser run — deferred rather than shipped unverified, since this crate does not currently
/// build clean (a concurrent, unrelated `dsl`/`store` import break) and Rust/closure-lifetime bugs here
/// can't be caught by anything short of a successful compile + a real mount/unmount browser check.
/// The dozen-plus `thread_local!` globals further up this file (`UI_ENGINE`, `ENGINE_SURFACES`,
/// `SCENE_STATE`, tooltip/dialog/tour chrome state, clipboard mocks, prefs, image-fetch caches, …) are
/// also still page-global, not per-mount — two simultaneous wgpu mounts each render on their own
/// independent GPU device/queue/surface (real, working isolation), but would still cross-talk on shared
/// UI chrome auxiliary state (a tooltip or dialog opened in one mount could show in the other).
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuMount)]
pub fn semio_wgpu_mount(canvas: web_sys::HtmlCanvasElement, plugins: JsValue, plugin_filter: String) -> Result<(), JsValue> {
    let event_loop = EventLoop::<HostUserEvent>::with_user_event().build().map_err(|err| JsValue::from_str(&format!("[DEBUG] event loop: {err:?}")))?;
    let proxy = event_loop.create_proxy();
    let app = SemioApp::new(proxy, plugin_filter, Some(plugins), Some(canvas));
    use winit::platform::web::EventLoopExtWebSys;
    event_loop.spawn_app(app);
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = uploadIconAtlas)]
pub fn upload_icon_atlas(width: u32, height: u32, pixels: &[u8], entries_json: &str) -> Result<(), JsValue> {
    let entries_map: std::collections::HashMap<String, [f32; 4]> = serde_json::from_str(entries_json).map_err(|err| JsValue::from_str(&format!("[DEBUG] icon entries parse: {err}")))?;
    let entries: Vec<(String, [f32; 4])> = entries_map.into_iter().collect();
    ICON_ATLAS_RUNTIME.with(|cell| {
        cell.borrow_mut().replace(IconAtlas::from_packed(width, height, pixels.to_vec(), entries));
    });
    Ok(())
}

thread_local! {
    static ICON_ATLAS_RUNTIME: RefCell<Option<IconAtlas>> = RefCell::new(None);
}
