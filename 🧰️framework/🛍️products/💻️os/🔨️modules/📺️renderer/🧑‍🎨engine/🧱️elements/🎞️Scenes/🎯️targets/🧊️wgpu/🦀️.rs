//! 🎬️ framework/products/os/modules/renderer/engine/elements/🎞️Scenes/component.rs — wgpu render
//! implementation for the Scenes element, extracted from lib.rs's inline `pub mod scenes { ... }`
//! body (ticket 26/08/05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE). Wired via `#[path =
//! "../../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"] pub mod scenes;` in lib.rs in place of the former
//! inline block; the module name `scenes` is unchanged, so every existing `crate::scenes::...`
//! call site elsewhere in the crate keeps resolving with zero other changes.
//! 🎬️ Native component scene hosts for canvas-2d, tables, graphs, and 3D views.

use crate::engine_canvas;
use crate::element_contracts::{FrameworkWidgetContext, RasterUploadRefusal};
use crate::shell::{try_push_find_item, ShellFindItem};
use base64::Engine;
use semio_framework::interaction::gesture::{apply_pinch_to_camera, Camera2d, GesturePointer, GestureRecognizer, GestureVerdict, ZoomBounds};
use semio_framework::IconName;
use serde::Deserialize;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::io::Write as _;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Arc, Mutex, OnceLock};
use ui_wgpu::wgpu::input::{DragAxis, KeyAction};
use ui_wgpu::wgpu::Rect;
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::UiPresence;
use ui_wgpu::wgpu::{draw_text, draw_text_face, draw_text_weighted, draw_text_wrapped, foreground_on_fill, push_icon, render_widget, HitKind, HitTarget, TextFace, Theme, UiDriverDrag, WidgetNode};
use ui_wgpu::wgpu::{ActionDescriptor, PreparedRasterProducer, PreparedRasterRejected, PreparedRasterReservation, SurfaceKind, UiComponentSceneNode};

#[path = "🗃️scene-state/🦀️.rs"]
mod scene_state;
pub use scene_state::*;

#[path = "🪣️pending-raster/🦀️.rs"]
mod pending_raster;
pub use pending_raster::*;

#[path = "📷️scene-camera-dispatch/🦀️.rs"]
mod scene_camera_dispatch;
pub use scene_camera_dispatch::*;

#[path = "⌨️scene-input/🦀️.rs"]
mod scene_input;
pub use scene_input::*;

#[path = "🎨️scene-render-entry/🦀️.rs"]
mod scene_render_entry;
pub use scene_render_entry::*;

#[path = "🧮️table-and-paint2d/🦀️.rs"]
mod table_and_paint2d;
pub(crate) use table_and_paint2d::*;

#[path = "🧱️block-list-and-diff/🦀️.rs"]
mod block_list_and_diff;
pub(crate) use block_list_and_diff::*;

#[path = "📰️event-feed-and-timeline/🦀️.rs"]
mod event_feed_and_timeline;
pub(crate) use event_feed_and_timeline::*;

#[path = "🖌️canvas2d/🦀️.rs"]
mod canvas2d;
pub(crate) use canvas2d::*;

#[path = "✒️ink-canvas-model-state/🦀️.rs"]
mod ink_canvas_model_state;
pub(crate) use ink_canvas_model_state::*;

#[path = "🖋️ink-canvas-render/🦀️.rs"]
mod ink_canvas_render;
pub(crate) use ink_canvas_render::*;

#[path = "🧭️node-graph-and-tiled-map/🦀️.rs"]
mod node_graph_and_tiled_map;
pub use node_graph_and_tiled_map::*;

#[path = "🖼️icon-render/🦀️.rs"]
mod icon_render;
pub(crate) use icon_render::*;

#[path = "🧩️board2d-and-vfs/🦀️.rs"]
mod board2d_and_vfs;
pub use board2d_and_vfs::*;

#[path = "✍️text-editor/🦀️.rs"]
mod text_editor;
pub use text_editor::*;

#[cfg(test)]
/// 🔎️ The whole Scenes module as the one text the unsplit file held: this file and every child, for the source-text laws.
pub(crate) const SCENES_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🗃️scene-state/🦀️.rs"),
    include_str!("🪣️pending-raster/🦀️.rs"),
    include_str!("📷️scene-camera-dispatch/🦀️.rs"),
    include_str!("⌨️scene-input/🦀️.rs"),
    include_str!("🎨️scene-render-entry/🦀️.rs"),
    include_str!("🧮️table-and-paint2d/🦀️.rs"),
    include_str!("🧱️block-list-and-diff/🦀️.rs"),
    include_str!("📰️event-feed-and-timeline/🦀️.rs"),
    include_str!("🖌️canvas2d/🦀️.rs"),
    include_str!("✒️ink-canvas-model-state/🦀️.rs"),
    include_str!("🖋️ink-canvas-render/🦀️.rs"),
    include_str!("🧭️node-graph-and-tiled-map/🦀️.rs"),
    include_str!("🖼️icon-render/🦀️.rs"),
    include_str!("🧩️board2d-and-vfs/🦀️.rs"),
    include_str!("✍️text-editor/🦀️.rs")
);

//#region SceneRuntime
/// 🧵️ Native scene state shared by the shared-pool workers; under `cfg(test)` each test thread owns its own
/// (`crate::element_contracts::test_worker_cell`), so no law meets another law's scenes.
#[cfg(not(target_arch = "wasm32"))]
struct WorkerCell<T>(#[cfg(not(test))] OnceLock<Mutex<RefCell<T>>>, #[cfg(test)] std::marker::PhantomData<fn() -> T>);

#[cfg(not(target_arch = "wasm32"))]
impl<T> WorkerCell<T> {
    const fn new() -> Self {
        #[cfg(not(test))]
        return Self(OnceLock::new());
        #[cfg(test)]
        return Self(std::marker::PhantomData);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Default + 'static> WorkerCell<T> {
    fn with<R>(&self, f: impl FnOnce(&RefCell<T>) -> R) -> R {
        #[cfg(not(test))]
        let state = self.0.get_or_init(|| Mutex::new(RefCell::new(T::default())));
        #[cfg(test)]
        let state = crate::element_contracts::test_worker_cell::<RefCell<T>>(std::ptr::from_ref(self).addr(), || RefCell::new(T::default()));
        let guard = state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        f(&guard)
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static SCENE_STATE: RefCell<AdmittedSurfaceMap<SceneSurfaceState>> = RefCell::new(AdmittedSurfaceMap::default());
    static SCENE_SURFACE_RETIREMENT: RefCell<Option<SceneSurfaceRetirement>> = const { RefCell::new(None) };
    static CANVAS_GESTURE: RefCell<CanvasGestureSlots> = RefCell::new(CanvasGestureSlots::default());
    static CANVAS_CATALOGUE_HOVER: RefCell<Option<CanvasCatalogueHover>> = const { RefCell::new(None) };
    static PENDING_RASTER_STATE: RefCell<AdmittedSurfaceMap<PendingRasterSurface>> = RefCell::new(AdmittedSurfaceMap::default());
    static PENDING_RASTER_CLOSE_OWNER: RefCell<Option<PendingRasterSurfaceRetirement>> = const { RefCell::new(None) };
    static SCENE_CAMERA_DISPATCH_DEADLINES_MS: RefCell<crate::camera_storage::DirectoryOwner<Option<SceneCameraOwnerMetadata>>> = RefCell::new(crate::camera_storage::DirectoryOwner::default());
    static SCENE_CAMERA_DISPATCH_FAULT: RefCell<Option<&'static str>> = RefCell::new(None);
}

#[cfg(not(target_arch = "wasm32"))]
static SCENE_STATE: WorkerCell<AdmittedSurfaceMap<SceneSurfaceState>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static SCENE_SURFACE_RETIREMENT: WorkerCell<Option<SceneSurfaceRetirement>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static CANVAS_GESTURE: WorkerCell<CanvasGestureSlots> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static CANVAS_CATALOGUE_HOVER: WorkerCell<Option<CanvasCatalogueHover>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static PENDING_RASTER_STATE: WorkerCell<AdmittedSurfaceMap<PendingRasterSurface>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static PENDING_RASTER_CLOSE_OWNER: WorkerCell<Option<PendingRasterSurfaceRetirement>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static SCENE_CAMERA_DISPATCH_DEADLINES_MS: WorkerCell<crate::camera_storage::DirectoryOwner<Option<SceneCameraOwnerMetadata>>> = WorkerCell::new();
#[cfg(not(target_arch = "wasm32"))]
static SCENE_CAMERA_DISPATCH_FAULT: WorkerCell<Option<&'static str>> = WorkerCell::new();

#[cfg(not(target_arch = "wasm32"))]
pub trait NativeHostTemporalFormatter: Send + Sync {
    fn format_temporal_values(&self, request: &ui_contract::HostTemporalFormatRequestV1) -> Result<ui_contract::HostTemporalFormatReplyV1, String>;
}

#[cfg(not(target_arch = "wasm32"))]
static NATIVE_HOST_TEMPORAL_FORMATTER: OnceLock<Mutex<Option<Arc<dyn NativeHostTemporalFormatter>>>> = OnceLock::new();

/// 🖥️ Installs the native application's operating-system temporal formatter. The renderer owns only
/// the neutral request/reply and never synthesizes locale or time-zone data.
#[cfg(not(target_arch = "wasm32"))]
pub fn install_native_host_temporal_formatter(formatter: Arc<dyn NativeHostTemporalFormatter>) {
    *NATIVE_HOST_TEMPORAL_FORMATTER.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(formatter);
}

#[cfg(not(target_arch = "wasm32"))]
fn native_host_temporal_formatter() -> Option<Arc<dyn NativeHostTemporalFormatter>> {
    NATIVE_HOST_TEMPORAL_FORMATTER.get_or_init(|| Mutex::new(None)).lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

#[derive(Clone, Debug, PartialEq)]
enum SceneListTransferSource {
    TableRow { row_id: String, mime: String, payload: String },
    BlockListStep { step_id: String, index: usize },
    BlockListBlock { step_id: String, block_id: String, index: usize },
    BlockListPalette { kind: String, mime: String, payload: String },
}

#[derive(Clone, Debug)]
struct SceneListTransferSession {
    pointer_id: ui_render::PointerId,
    authority_generation: u64,
    source_window_id: String,
    source_host_id: String,
    source_document_generation: u64,
    driver_drag: UiDriverDrag,
    source: SceneListTransferSource,
    source_rect: Rect,
    start_x: f32,
    start_y: f32,
    promoted: bool,
}

#[derive(Debug)]
struct SceneListTransferAuthority {
    next_generation: u64,
    active: Option<SceneListTransferSession>,
}

impl Default for SceneListTransferAuthority {
    fn default() -> Self {
        Self { next_generation: 1, active: None }
    }
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static SCENE_LIST_TRANSFER: RefCell<SceneListTransferAuthority> = RefCell::new(SceneListTransferAuthority::default());
}

#[cfg(not(target_arch = "wasm32"))]
static SCENE_LIST_TRANSFER: WorkerCell<SceneListTransferAuthority> = WorkerCell::new();

/** 📁️ Toggles VFS row expand/collapse in scene-local state. */
pub fn toggle_vfs_row_expanded(surface_id: &str, row_id: &str) {
    mutate_scene_state(surface_id, |state| {
        if state.vfs_expanded_ids.contains(row_id) {
            state.vfs_expanded_ids.remove(row_id);
        } else {
            state.vfs_expanded_ids.insert(row_id.to_string());
        }
    });
}

/** 📁️ Seeds default expanded VFS roots on first render. */
pub fn seed_vfs_expanded(surface_id: &str, row_ids: &[String]) {
    mutate_scene_state(surface_id, |state| {
        if state.vfs_expanded_ids.is_empty() {
            for id in row_ids {
                state.vfs_expanded_ids.insert(id.clone());
            }
        }
    });
}

/** 📁️ Computes VFS multi-select ids for shift/meta click semantics. */
pub fn vfs_selection_for_click(surface_id: &str, row_id: &str, ordered_ids: &[String], shift: bool, additive: bool) -> Vec<String> {
    let mut next = Vec::new();
    mutate_scene_state(surface_id, |state| {
        if shift {
            let anchor = state.vfs_selection_anchor.as_deref().unwrap_or(row_id);
            if let (Some(a), Some(b)) = (ordered_ids.iter().position(|id| id == anchor), ordered_ids.iter().position(|id| id == row_id)) {
                let (start, end) = if a <= b { (a, b) } else { (b, a) };
                next.extend_from_slice(&ordered_ids[start..=end]);
            }
        }
        if next.is_empty() && additive {
            next.extend(ordered_ids.iter().filter(|id| state.selected_ids.contains(id.as_str())).cloned());
            if state.selected_ids.contains(row_id) {
                next.retain(|id| id != row_id);
            } else {
                next.push(row_id.to_string());
            }
        }
        if next.is_empty() && !(additive && state.selected_ids.contains(row_id)) {
            next.push(row_id.to_string());
        }
        state.selected_ids = next.iter().cloned().collect();
        state.vfs_selection_anchor = Some(row_id.to_string());
    });
    next
}

pub(crate) fn scene_host_retiring(host_id: &str) -> bool {
    SCENE_SURFACE_RETIREMENT.with(|cell| cell.borrow().as_ref().is_some_and(|retired| retired.host_id.as_ref() == host_id))
}

pub(crate) fn scene_surface_retirement_can_admit() -> bool {
    SCENE_SURFACE_RETIREMENT.with(|cell| cell.borrow().is_none())
}

/// 🪪️ Admits one retained component host for its exact mounted identity.
pub(crate) fn mount_scene_identity(owner: &crate::element_contracts::ScenePointerTarget) -> bool {
    if owner.host_id.is_empty() {
        return false;
    }
    if SCENE_SURFACE_RETIREMENT.with(|cell| cell.borrow().as_ref().is_some_and(|retired| retired.host_id.as_ref() == owner.host_id)) {
        return false;
    }
    SCENE_STATE.with(|cell| {
        let mut states = cell.borrow_mut();
        let Some(state) = states.get_or_insert_with(owner.host_id.clone(), SceneSurfaceState::default) else { return false };
        if state.mount_owner.as_ref().is_some_and(|current| !current.same_component_host(owner)) {
            return false;
        }
        if state.mount_owner.is_none() {
            state.mount_owner = Some(owner.clone());
        }
        true
    })
}

/// 🎟️ Attaches an engine allocation to the component that admitted its paint.
pub(crate) fn bind_scene_engine_token(host_id: &str, token: engine_canvas::EngineSurfaceToken) -> bool {
    if scene_host_retiring(host_id) {
        return false;
    }
    SCENE_STATE.with(|cell| {
        let mut states = cell.borrow_mut();
        let Some(state) = states.get_mut(host_id) else { return false };
        if state.mount_owner.is_none() || state.engine_token.is_some_and(|current| current != token) {
            return false;
        }
        state.engine_token = Some(token);
        true
    })
}

/// 🪪️ Seeds a mounted Canvas2d camera once for its exact retained component identity.
pub(crate) fn mount_canvas_camera(scene: &UiComponentSceneNode, owner: crate::element_contracts::ScenePointerTarget) -> bool {
    let Some(canvas) = scene.canvas_2d.as_ref() else { return false };
    SCENE_STATE.with(|cell| {
        let mut states = cell.borrow_mut();
        let Some(state) = states.get_or_insert_with(scene.host_id.clone(), SceneSurfaceState::default) else { return false };
        if !state.canvas_camera_owner.as_ref().is_some_and(|current| current.same_component_host(&owner)) {
            state.viewport = Viewport { x: canvas.camera_x as f32, y: canvas.camera_y as f32, zoom: canvas.zoom as f32 };
            state.canvas_camera_owner = Some(owner);
            state.canvas_framing_revision = None;
            state.camera_dispatch_controller_id = None;
            state.canvas_click = None;
            state.drag = None;
            state.pointer_was_down = false;
            remove_scene_camera_deadline(&scene.host_id);
        }
        true
    })
}

/// 🧹️ Transfers an exact removed component into the bounded scene retirement owner.
pub(crate) fn retire_scene_identity(owner: &crate::element_contracts::ScenePointerTarget) -> bool {
    if !scene_surface_retirement_can_admit() {
        return false;
    }
    if owner.kind == ui_wgpu::wgpu::SurfaceKind::TiledMap {
        #[cfg(test)]
        let before = SCENE_STATE.with(|cell| {
            cell.borrow()
                .get(&owner.host_id)
                .map(|state| (state.mount_owner.as_ref().is_some_and(|current| current.same_component_host(owner)), state.map_interaction_owner.as_ref().is_some_and(|current| current.same_component_host(owner)), state.drag.is_some()))
        });
        let _interaction = retire_tiled_map_scene_identity(owner);
        #[cfg(test)]
        eprintln!("[TRACE] scene retirement map host={} before={before:?} exact-interaction={_interaction}", owner.host_id);
    }
    if owner.kind == ui_wgpu::wgpu::SurfaceKind::Canvas2d {
        request_canvas_pointer_gesture_cancel_for_host(&owner.host_id);
    }
    let retired = SCENE_STATE.with(|cell| {
        let mut states = cell.borrow_mut();
        let state = states.get(&owner.host_id)?;
        if !state.mount_owner.as_ref().is_some_and(|current| current.same_component_host(owner)) && !state.canvas_camera_owner.as_ref().is_some_and(|current| current.same_component_host(owner)) {
            return None;
        }
        let token = states.token(&owner.host_id)?;
        states.take_exact_to_retirement(token)
    });
    let retained = retired.is_some();
    if let Some(owner) = retired {
        let external_engine_token = owner.value.engine_token;
        let external_target = (external_engine_token.is_some() || matches!(owner.value.mount_owner.as_ref().map(|owner| owner.kind), Some(SurfaceKind::World3d | SurfaceKind::IconRender))).then(|| owner.value.mount_owner.clone()).flatten();
        let mut values = Vec::new();
        if let Some((key, value)) = crate::interpreter::take_world_camera_diagnostic(&owner.id) {
            values.push(SceneValueRetirement::Text(key));
            values.push(SceneValueRetirement::Value(value));
        }
        SCENE_SURFACE_RETIREMENT.with(|cell| {
            let mut slot = cell.borrow_mut();
            assert!(slot.is_none(), "the previous component retirement must finish before admitting another");
            *slot = Some(SceneSurfaceRetirement {
                host_id: owner.id.clone().into_boxed_str(),
                owner,
                external_target,
                external_engine_token,
                external_close: None,
                text: String::new(),
                values,
                gesture: None,
                hover: None,
                raster: None,
                raster_started: false,
                transfer: None,
            });
        });
    }
    let camera_matches = SCENE_CAMERA_DISPATCH_DEADLINES_MS.with(|cell| cell.borrow().get(&owner.host_id).is_some_and(|record| camera_owner_matches(record, owner)));
    if camera_matches { remove_scene_camera_deadline(&owner.host_id); }

    retained
}

fn scene_state(surface_id: &str) -> SceneSurfaceState {
    SCENE_STATE.with(|cell| cell.borrow_mut().get_or_insert_with(surface_id.to_string(), SceneSurfaceState::default).cloned().unwrap_or_default())
}

fn mutate_scene_state(surface_id: &str, f: impl FnOnce(&mut SceneSurfaceState)) {
    SCENE_STATE.with(|cell| {
        let mut map = cell.borrow_mut();
        if let Some(entry) = map.get_or_insert_with(surface_id.to_string(), SceneSurfaceState::default) {
            f(entry);
        }
    });
}
//#endregion SceneRuntime

