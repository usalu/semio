//! 🎭️ Scene state: the per-surface scene registry every scene kind reads and writes, and the shared kit the scene crates build on.
//!
//! The scene cell and its statics (scene state, retirement, canvas gesture and catalogue hover, list transfer, camera dispatch), the admitted surface
//! map, the pending raster registry, the camera dispatch ledger, the component surface close bridge and the world camera ledger lifted out of the
//! host and the chrome ledger, and the list, chrome label, accessibility cell and text editor ui kit the scene leaves share.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_renderer_wgpu_canvas2d_paint as canvas2d_paint;
extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;
extern crate semio_framework_os_renderer_wgpu_host_base as host_base;
pub use semio_framework_os_renderer_wgpu_host_base::action_args_json;
use semio_framework_os_renderer_camera as camera_storage;
use semio_framework_os_renderer_wgpu_engine_registry::engine_surface_registry::EngineSurfaceToken;
use crate::element_contracts::WorkerCell;
use crate::element_contracts::{AdmittedSurfaceCloseOwner, AdmittedSurfaceEntry, AdmittedSurfaceFault, AdmittedSurfaceMap, AdmittedSurfaceRejected, AdmittedSurfaceToken, SCENE_SURFACE_CAPACITY, SCENE_SURFACE_ID_BYTE_CAPACITY};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_native_platform::{native_host_temporal_formatter, NativeHostTemporalFormatter};
use crate::element_contracts::FrameworkWidgetContext;
use crate::element_contracts::RasterUploadRefusal;
use semio_framework::interaction::gesture::GestureRecognizer;
use serde::Deserialize;
use serde_json::Value;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::OnceLock;
use ui_wgpu::wgpu::Rect;
use ui_wgpu::wgpu::Rgba;
use ui_wgpu::wgpu::draw_text;
use ui_wgpu::wgpu::TextFace;
use ui_wgpu::wgpu::Theme;
use ui_wgpu::wgpu::UiDriverDrag;
use ui_wgpu::wgpu::ActionDescriptor;
use ui_wgpu::wgpu::PreparedRasterProducer;
use ui_wgpu::wgpu::PreparedRasterRejected;
use ui_wgpu::wgpu::PreparedRasterReservation;
use ui_wgpu::wgpu::SurfaceKind;
use ui_wgpu::wgpu::UiComponentSceneNode;

//#region SceneRuntime
/// 🧵️ Native scene state shared by the shared-pool workers; under `cfg(test)` each test thread owns its own
/// (`element_contracts::test_worker_cell`), so no law meets another law's scenes.
#[cfg(not(target_arch = "wasm32"))]
pub struct SceneCell<T>(#[cfg(not(test))] OnceLock<Mutex<RefCell<T>>>, #[cfg(test)] std::marker::PhantomData<fn() -> T>);

#[cfg(not(target_arch = "wasm32"))]
impl<T> SceneCell<T> {
    const fn new() -> Self {
        #[cfg(not(test))]
        return Self(OnceLock::new());
        #[cfg(test)]
        return Self(std::marker::PhantomData);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Default + 'static> SceneCell<T> {
    pub fn with<R>(&self, f: impl FnOnce(&RefCell<T>) -> R) -> R {
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
    pub static SCENE_STATE: RefCell<AdmittedSurfaceMap<SceneSurfaceState>> = RefCell::new(AdmittedSurfaceMap::default());
    pub static SCENE_SURFACE_RETIREMENT: RefCell<Option<SceneSurfaceRetirement>> = const { RefCell::new(None) };
    pub static CANVAS_GESTURE: RefCell<CanvasGestureSlots> = RefCell::new(CanvasGestureSlots::default());
    pub static CANVAS_CATALOGUE_HOVER: RefCell<Option<CanvasCatalogueHover>> = const { RefCell::new(None) };
    pub static PENDING_RASTER_STATE: RefCell<AdmittedSurfaceMap<PendingRasterSurface>> = RefCell::new(AdmittedSurfaceMap::default());
    pub static PENDING_RASTER_CLOSE_OWNER: RefCell<Option<PendingRasterSurfaceRetirement>> = const { RefCell::new(None) };
    pub static SCENE_CAMERA_DISPATCH_DEADLINES_MS: RefCell<crate::camera_storage::DirectoryOwner<Option<SceneCameraOwnerMetadata>>> = RefCell::new(crate::camera_storage::DirectoryOwner::default());
    pub static SCENE_CAMERA_DISPATCH_FAULT: RefCell<Option<&'static str>> = RefCell::new(None);
}

#[cfg(not(target_arch = "wasm32"))]
pub static SCENE_STATE: SceneCell<AdmittedSurfaceMap<SceneSurfaceState>> = SceneCell::new();
#[cfg(not(target_arch = "wasm32"))]
pub static SCENE_SURFACE_RETIREMENT: SceneCell<Option<SceneSurfaceRetirement>> = SceneCell::new();
#[cfg(not(target_arch = "wasm32"))]
pub static CANVAS_GESTURE: SceneCell<CanvasGestureSlots> = SceneCell::new();
#[cfg(not(target_arch = "wasm32"))]
pub static CANVAS_CATALOGUE_HOVER: SceneCell<Option<CanvasCatalogueHover>> = SceneCell::new();
#[cfg(not(target_arch = "wasm32"))]
pub static PENDING_RASTER_STATE: SceneCell<AdmittedSurfaceMap<PendingRasterSurface>> = SceneCell::new();
#[cfg(not(target_arch = "wasm32"))]
pub static PENDING_RASTER_CLOSE_OWNER: SceneCell<Option<PendingRasterSurfaceRetirement>> = SceneCell::new();
#[cfg(not(target_arch = "wasm32"))]
pub static SCENE_CAMERA_DISPATCH_DEADLINES_MS: SceneCell<crate::camera_storage::DirectoryOwner<Option<SceneCameraOwnerMetadata>>> = SceneCell::new();
#[cfg(not(target_arch = "wasm32"))]
pub static SCENE_CAMERA_DISPATCH_FAULT: SceneCell<Option<&'static str>> = SceneCell::new();

#[derive(Clone, Debug, PartialEq)]
pub enum SceneListTransferSource {
    TableRow { row_id: String, mime: String, payload: String },
    BlockListStep { step_id: String, index: usize },
    BlockListBlock { step_id: String, block_id: String, index: usize },
    BlockListPalette { kind: String, mime: String, payload: String },
}

#[derive(Clone, Debug)]
pub struct SceneListTransferSession {
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
    pub static SCENE_LIST_TRANSFER: RefCell<SceneListTransferAuthority> = RefCell::new(SceneListTransferAuthority::default());
}

#[cfg(not(target_arch = "wasm32"))]
pub static SCENE_LIST_TRANSFER: SceneCell<SceneListTransferAuthority> = SceneCell::new();

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

pub fn scene_host_retiring(host_id: &str) -> bool {
    SCENE_SURFACE_RETIREMENT.with(|cell| cell.borrow().as_ref().is_some_and(|retired| retired.host_id.as_ref() == host_id))
}

pub fn scene_surface_retirement_can_admit() -> bool {
    SCENE_SURFACE_RETIREMENT.with(|cell| cell.borrow().is_none())
}

/// 🪪️ Admits one retained component host for its exact mounted identity.
pub fn mount_scene_identity(owner: &crate::element_contracts::ScenePointerTarget) -> bool {
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
pub fn bind_scene_engine_token(host_id: &str, token: EngineSurfaceToken) -> bool {
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
pub fn mount_canvas_camera(scene: &UiComponentSceneNode, owner: crate::element_contracts::ScenePointerTarget) -> bool {
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
pub fn retire_scene_identity(owner: &crate::element_contracts::ScenePointerTarget) -> bool {
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
        if let Some((key, value)) = take_world_camera_diagnostic(&owner.id) {
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

pub fn scene_state(surface_id: &str) -> SceneSurfaceState {
    SCENE_STATE.with(|cell| cell.borrow_mut().get_or_insert_with(surface_id.to_string(), SceneSurfaceState::default).cloned().unwrap_or_default())
}

pub fn mutate_scene_state(surface_id: &str, f: impl FnOnce(&mut SceneSurfaceState)) {
    SCENE_STATE.with(|cell| {
        let mut map = cell.borrow_mut();
        if let Some(entry) = map.get_or_insert_with(surface_id.to_string(), SceneSurfaceState::default) {
            f(entry);
        }
    });
}
//#endregion SceneRuntime

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🗃️scene-state/🦀️.rs"]
mod surface_state;
pub use surface_state::*;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🪣️pending-raster/🦀️.rs"]
mod pending_raster;
pub use pending_raster::*;

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/📷️scene-camera-dispatch/🦀️.rs"]
mod scene_camera_dispatch;
pub use scene_camera_dispatch::*;

#[path = "📥️lifted/🏠️os-host/🦀️.rs"]
mod component_surface_close;
pub use component_surface_close::*;

#[path = "📥️lifted/🎯️chrome-ledger/🦀️.rs"]
mod world_camera_ledger;
pub use world_camera_ledger::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
