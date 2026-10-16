//! 🌉️ World3d scene bridge: dynamic retirement, face overlays, snapshot apply and the guest scene bridge.
//!
//! Crate `semio-framework-os-infinite-world-bridge`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_framework_os_infinite_world_model::{quarantine_world_owner, World3dBuildContext, world_close_string, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_DYNAMIC_MESH_CAPACITY, WORLD_INTERACTION_ID_BYTE_CAPACITY, WORLD_INTERACTION_MESH_CAPACITY, WORLD_INTERACTION_OBJECT_CAPACITY, world_pick_keys_match, world_pick_target_bounds, world_pick_targets_from_json, world_reference_width, world_scene_raster_pool, WorldDrawRebuildCursor, WorldDrawRebuildDescriptor, WorldDynamicFault, WorldDynamicRegistry, WorldDynamicRejected, WorldInteractionId, WorldInteractionMeshRegistry, WorldInteractionMeshToken, WorldInteractionObjectKind, WorldInteractionPlan, WorldInteractionRegistryBuildPhase, WorldInteractionState, WorldInteractionStep, WorldMeshRegistry, WorldMeshSource, WorldNativeMeshAdmission, WorldNativeMeshFailure, WorldNativeMeshSource, WorldOpaqueOwner, WorldReferenceRecord, WorldSelectionTargets, WorldSessionState, WorldSubElementPaint};
use semio_framework_os_infinite_world_components::{GumballHandle, mesh_vertex, pick_targets_instance, World3dCameraFitOwner, World3dCameraFitRequest, World3dSceneGumballConfigRecord, World3dSnapshotApplyCursor, world_component_preview_target, world_mesh_component_id, world_mesh_triangle, WorldCameraState, WorldGumballState, WorldMeshBuffers, WorldNavigationSnapshot, WorldOverlayState, WorldSceneState, WorldSelectionState};
use semio_framework_os_infinite_world_cursors::{CLICK_DRAG_THRESHOLD_PX, live_world3d_mesh_version, pointer_in_pick_rect, world3d_gumball_config, WorldBrushCommitJob, WorldComponentMarqueePublishJob, WorldComponentPickCursor, WorldGumballCommitJob, WorldGumballPickCursor, WorldInteractionAuthorityStep, WorldMarqueePickCursor, WorldMarqueePublishJob, WorldObjectPickCursor, WorldRayPickCursor};
use semio_framework_os_infinite_world_scene::{environment_outline_color, GUMBALL_PLANE_MESH, mesh_id_from_url, mesh_style_paint, parse_color, publish_world3d_mesh_lease, resolve_mesh_style, retire_world_pixels, scene_lod, world3d_scene_digest, world_instance_style, WorldAssetsState};
use ui_wgpu::wgpu::{ActionDescriptor, Camera3d, CameraProjection3d, ComponentSource3d, gumball_extent, gumball_eye, Instance3d, LineVertex3d, lod_grid_step_world, Mat4, Mesh3dField, Mesh3dLease, quat_from_basis, ray_plane_point, ray_segment_distance, Rect, Rgba, rotate_vector, SceneColorSource3d, SceneDraw3d, SceneRasterLease, SceneRasterPool, Theme, Vec3, World3dSnapshotFault, World3dSnapshotItem, World3dSnapshotLease, World3dSnapshotPageKind};
use serde::Deserialize;
use std::collections::HashSet;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::SceneMaterial3d;
use semio_framework_ui_viewport::{Viewport3dProjectionFramePolicy, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};
use ui_wgpu::wgpu::kernel_3d_scene::Mesh3dResourceLoan as WorldNativeMeshLoan;
use semio_framework_ui_runtime::UiNativeMeshResource as WorldNativeMeshResource;
use semio_framework_ui_contract::UiRevision as WorldNativeUiRevision;
use serde_json::json;

/// 🧩️ World3d retirement component: the in-flight dynamic retirement cursor.
pub struct WorldRetirementState {

    pub dynamic_retirement: Option<World3dDynamicRetirement>,
}

/// 🧩️ World3d bridge component: scene bridge, snapshot apply and scene digests.
pub struct WorldBridgeState {

    pub snapshot_lease: Option<World3dSnapshotLease>,

    pub snapshot_apply: Option<World3dSnapshotApplyCursor>,

    /// 🌉️ The mesh-wire → snapshot bridge (`//#region 🌉️World3dSceneBridge`) that gives a producer
    /// publishing only plain-JSON `meshes_json`/`instances_json` the same typed snapshot a
    /// snapshot-native producer hands over.
    pub scene_bridge: Option<World3dSceneBridgeCursor>,

    pub scene_bridge_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<World3dSceneBridgeCursor>>,

    pub scene_bridge_lease: Option<World3dSnapshotLease>,

    pub scene_bridge_retired: Option<(World3dSnapshotLease, bool)>,

    pub scene_bridge_generation: u64,

    pub scene_bridge_digest: Option<u64>,

    pub scene_document_lanes_digest: Option<u64>,
}

impl WorldRetirementState {
    pub fn new() -> Self {
        Self {
            dynamic_retirement: None,
        }
    }
}

impl WorldBridgeState {
    pub fn new() -> Self {
        Self {
            snapshot_lease: None,
            scene_bridge: None,
            scene_bridge_close: None,
            scene_bridge_lease: None,
            scene_bridge_retired: None,
            scene_bridge_generation: 0,
            scene_bridge_digest: None,
            scene_document_lanes_digest: None,
            snapshot_apply: None,
        }
    }
}

//#region 🌉️World3dSceneBridge
/// 🌉️ How many `mesh3d_*` authority writes one bridge step performs. That ladder writes ONE
/// vec3/u32 per call, so a 20k-vertex preview would otherwise need 60k frames to publish; a batch
/// keeps the publication inside the frame transaction's own fuel accounting while still finishing a
/// typical procedural preview in a handful of turns.
pub const WORLD3D_BRIDGE_MESH_WRITES_PER_STEP: u32 = 4_096;

/// 🥽️ Whether this mesh key's geometry is owed by the world-asset pipeline rather than by the
/// placeholder ladder. The snapshot apply consults it before minting a stand-in box — see
/// [`declare_scene_mesh_source`].
pub fn scene_mesh_awaits_its_asset(scene_state: &WorldSceneState, mesh_key: &str) -> bool {
    scene_state.mesh_source_urls.contains_key(mesh_key)
}

/// 🎨️ Resolves the standard-material pair React assigns after geometry provenance is known:
/// inline/procedural meshes use `0/1`, while `GlbInstanceMesh` uses the environment override.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn world3d_standard_material_for_mesh(scene_state: &WorldSceneState, mesh_key: &str, environment: SceneMaterial3d) -> [f32; 2] {
    if scene_state.mesh_source_urls.contains_key(mesh_key) { [environment.metalness, environment.roughness] } else { [0.0, 1.0] }
}

/// 🌉️ One `World3dScene.meshes_json` entry. Three mutually exclusive ways a producer names one
/// mesh, all of which React's `WorldMeshRecord` (`🌐️World3dHost/🟦️.tsx`) accepts:
///
/// - `{"id": …, "data": {positions, normals, indices, …}}` — INLINE geometry (generation3d's
///   `preview_payload`, CAD, fem).
/// - `{"id": "mesh:…", "url": "/mesh/….glb"}` — a GLB the renderer fetches itself
///   (`world3d_meshes_json_from_urls`). React loads it with `GLTFLoader`; here it rides the bounded
///   world-asset pipeline as a [`WorldAssetRequestKind::Glb`], see [`declare_scene_mesh_source`].
/// - `{"id": "box", "kind": "box"}` — a built-in procedural kind
///   (`world3d_mesh_kind_entry`), a REFERENCE whose tessellation both renderers own. React resolves
///   it through `meshDataFromKind`, this one through [`WorldPlaceholderKind::resolve`].
#[derive(Clone, Debug, semio_framework_value_derive::FromValue)]
#[value(rename_all="camelCase")]
#[derive(semio_framework_value_derive::RetireOwned)]
pub struct World3dSceneMeshEntry {
    pub id:String,
    #[value(default)]
    pub data:WorldMeshBuffers,
    #[value(default)]
    pub url:Option<String>,
    #[value(default)]
    pub kind:Option<String>,
}

impl World3dSceneMeshEntry {
    /// 🧊️ Retains original surface, wire, and point geometry in the Scene mesh owner.
    pub fn has_inline_geometry(&self) -> bool {
        !self.data.edge_positions.is_empty() || (self.data.vertex_count() > 0 && (self.data.indices.len() >= 3 || (self.data.indices.is_empty() && self.data.vertex_ids.iter().any(|id|*id!=u32::MAX))))
    }

    /// 🌉️ Resolves retained inline geometry or an authored mesh reference.
    pub fn names_a_mesh(&self) -> bool {
        self.has_inline_geometry() || self.url.is_some() || self.kind.is_some()
    }
}

/// 🧬️ The exact analytic source in the original instance wire.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[derive(semio_framework_value_derive::RetireOwned)]
struct World3dSceneComponentSource {
    pub handle: String,
    pub revision: String,
}

/// 🌉️ One `World3dScene.instances_json` entry — a placement of one mesh, carrying the
/// channel-qualified instance id every pick/hover observation is addressed by.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(semio_framework_value_derive::RetireOwned)]
pub struct World3dSceneInstanceEntry {
    #[serde(default)]
    pub component_source: Option<World3dSceneComponentSource>,
    pub id: String,
    pub mesh_id: String,
    /// 🎯️ The TOPOLOGY target this rendered instance stands for — several instances of one channel
    /// share it. Absent means "the render id is its own target", exactly React's
    /// `instances.find(e => e.id === id)?.interactionId ?? id`
    /// (`🌐️World3dHost/🟦️.tsx`, `world3dInstanceInteractionTarget`).
    #[serde(default)]
    pub interaction_id: Option<String>,
    #[serde(default)]
    pub interaction_granularity_id: Option<String>,
    #[serde(default)]
    pub position: Option<[f64; 3]>,
    #[serde(default)]
    pub rotation: Option<[f64; 4]>,
    #[serde(default)]
    pub scale: Option<[f64; 3]>,
    #[serde(default)]
    pub color: Option<String>,
    /// 🟩️ The instance stands for an entity a running tool placed provisionally (`ArtifactView::tool_run()`).
    #[serde(default)]
    pub provisional: bool,
    /// 🎨️ React's `WorldInstanceRecord.highlighted` — the compatible/suggested state (a catalogue
    /// kind hovered in puzzle), which resolves to the SECONDARY `highlighted` row of
    /// `MESH_STYLE_PAINT`.
    #[serde(default)]
    pub highlighted: bool,
    /// 🎨️ React's `WorldInstanceRecord.disabled` — non-interactive/locked, the muted `disabled` row
    /// at 0.45 opacity.
    #[serde(default)]
    pub disabled: bool,
    /// 🎉️ React's `isWorldInstanceCelebrating` — the `celebrated` row (primary + the conic triad).
    #[serde(default)]
    pub celebrating: bool,
    /// 🌫️ Per-instance opacity MULTIPLIER, applied to the authored colour's alpha before any style
    /// row. This is how a plugin dims a LOCKED object on a committed mesh
    /// (`WORLD_LOCKED_OPACITY_SCALE = 0.35`, `♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:178`): the instance
    /// lane had no opacity field at all, so `target_style`'s locked arm could be computed and never
    /// reach a mesh (ticket 26/09/17 packet W2f §5.5). `None` leaves the alpha untouched.
    #[serde(default)]
    pub opacity: Option<f64>,
}

/// 🎥️ `World3dScene.camera_json` — the window's own camera measure, authored host-side.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(semio_framework_value_derive::RetireOwned)]
pub struct World3dSceneCameraRecord {
    #[serde(default)]
    pub position: Option<[f64; 3]>,
    #[serde(default)]
    pub target: Option<[f64; 3]>,
    #[serde(default)]
    pub up: Option<[f64; 3]>,
    #[serde(default)]
    pub fov: Option<f64>,
    #[serde(default)]
    pub zoom: Option<f64>,
    /// 📐️ The complete composed mode × orientation taxonomy accepted by React and WGPU.
    #[serde(default)]
    pub projection: Option<Viewport3dProjectionSpec>,
    #[serde(default)]
    pub projection_frame: Viewport3dProjectionFramePolicy,
}

impl World3dSceneCameraRecord {
    /// 📐️ React's `worldProjectionFamily` over the delivered spec.
    pub fn projection(&self) -> CameraProjection3d {
        self.projection.map_or(CameraProjection3d::Perspective, ui_wgpu::wgpu::projection_spec_family)
    }

    /// 📐️ React's `worldProjectionPerspectiveFov` fallback chain: the record's own `fov`, else the
    /// mode's, else the 45° `parseCameraState` default.
    fn fov_degrees(&self) -> f64 {
        self.projection.map(|spec| f64::from(ui_wgpu::wgpu::projection_spec_fov_degrees(spec))).or(self.fov).unwrap_or(45.0)
    }
}

/// 🎯️ `World3dScene.fit_json` — the producer's "frame this document once" lane, with the delivered
/// extent it wants framed (`world3d_fit_json`, `🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct World3dSceneFitRecord {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub revision: u32,
    #[serde(default)]
    pub padding: Option<f64>,
    #[serde(default)]
    bounds_min: Option<[f64; 3]>,
    #[serde(default)]
    bounds_max: Option<[f64; 3]>,
}

/// 📷️ React's `WorldProjectionContentFrame` — the one-shot framing every explicit projection
/// seed gets. `onFramed` immediately marks React's viewport owned and unmounts the framer, so later
/// content-bound changes must never move an accepted camera until another selection or external seed
/// creates a new debt.
/// `frameWorldProjectionPose` applies the spec's look and fits both families: orthographic through
/// viewport zoom, perspective/curvilinear through `max(span · 1.5, 2)` eye distance.
///
/// The extent is React's `worldSceneContentBounds`: every non-provisional instance position plus
/// every VISIBLE reference plane's footprint. `camera_user_moved` is the ownership latch — React's
/// `viewportOwned`.
///
/// 📦️ The bridge exposes content positions before its camera lease becomes accepted. Framing in
/// that interval would be overwritten by `step_world3d_snapshot`; wait for the same accepted
/// snapshot boundary used by the camera-fit lane.
pub fn sync_world3d_projection_content_frame(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, scene_state: &mut WorldSceneState, bridge_state: &mut WorldBridgeState) {
    if camera_state.projection_frame_policy == Viewport3dProjectionFramePolicy::PreserveCamera || !camera_state.projection_frame_owed {
        return;
    }
    if camera_state.camera_user_moved {
        camera_state.projection_frame_owed = false;
        return;
    }
    if bridge_state.scene_bridge.is_some()
        || bridge_state.snapshot_apply.is_some()
        || bridge_state.scene_bridge_lease.is_some_and(|lease| bridge_state.snapshot_lease != Some(lease))
    {
        return;
    }
    let Some((minimum, maximum)) = world3d_content_bounds(selection_state, scene_state) else { return };
    let key = world3d_content_bounds_key(minimum, maximum);
    let framed = ui_wgpu::wgpu::frame_projection_orbit_to_spec_bounds(&camera_state.orbit, camera_state.projection_spec, minimum, maximum, session_state.bounds.w, session_state.bounds.h, ui_wgpu::wgpu::WORLD_PROJECTION_FRAME_PADDING);
    camera_state.projection_frame_key = Some(key);
    camera_state.projection_frame_zoom = Some(framed.zoom);
    camera_state.projection_frame_owed = false;
    camera_state.orbit = framed;
    advance_world3d_view_revision(session_state, scene_state);
}

/// 📦️ React's `worldSceneContentBounds` — instance positions and visible reference footprints, with
/// each axis given the same half-unit floor React's does.
pub(crate) fn world3d_content_bounds(selection_state: &WorldSelectionState, scene_state: &WorldSceneState) -> Option<([f32; 3], [f32; 3])> {
    let mut minimum = [f64::INFINITY; 3];
    let mut maximum = [f64::NEG_INFINITY; 3];
    let mut any = false;
    let mut expand = |point: [f64; 3]| {
        for axis in 0..3 {
            minimum[axis] = minimum[axis].min(point[axis]);
            maximum[axis] = maximum[axis].max(point[axis]);
        }
        any = true;
    };
    for (id, position) in &scene_state.instance_positions {
        if selection_state.provisional_instance_ids.contains(id) {
            continue;
        }
        expand(*position);
    }
    for reference in scene_state.references.iter().filter(|reference| !reference.hidden.unwrap_or(false)) {
        let origin = reference.origin.unwrap_or([0.0; 3]);
        let half = f64::from(world_reference_width(reference).max(1e-3)) * 0.5;
        expand([origin[0] - half, origin[1] - half, origin[2]]);
        expand([origin[0] + half, origin[1] + half, origin[2]]);
    }
    if !any || (0..3).any(|axis| !minimum[axis].is_finite() || !maximum[axis].is_finite()) {
        return None;
    }
    let centre = [0, 1, 2].map(|axis| (minimum[axis] + maximum[axis]) * 0.5);
    let half = [0, 1, 2].map(|axis| ((maximum[axis] - minimum[axis]) * 0.5).max(0.5));
    Some(([0, 1, 2].map(|axis| (centre[axis] - half[axis]) as f32), [0, 1, 2].map(|axis| (centre[axis] + half[axis]) as f32)))
}

/// 🔑️ React's `worldSceneContentBoundsKey` — the identity that decides whether a framing is owed.
fn world3d_content_bounds_key(minimum: [f32; 3], maximum: [f32; 3]) -> u64 {
    let mut key = 0xcbf2_9ce4_8422_2325_u64;
    for value in minimum.iter().chain(maximum.iter()) {
        key ^= u64::from((value * 1e3).round() as i32 as u32);
        key = key.wrapping_mul(0x0100_0000_01b3);
    }
    key
}

pub const WORLD3D_RENDERED_OUTLINE_SCALE: f32 = 1.001;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dCameraFitStep {
    Idle,
    Pending,
    Applied,
    Stale,
}

pub fn world3d_camera_fit_key(owner: World3dCameraFitOwner, bounds: Option<([f32; 3], [f32; 3])>) -> u64 {
    let mut key = 0xcbf2_9ce4_8422_2325_u64;
    for value in [u64::from(owner.revision), owner.camera_digest.unwrap_or(0), u64::from(owner.camera_digest.is_some())] {
        key ^= value;
        key = key.wrapping_mul(0x0000_0100_0000_01b3);
    }
    if let Some((minimum, maximum)) = bounds {
        for value in minimum.into_iter().chain(maximum) {
            let rounded = (value * 1e3).round() / 1e3;
            key ^= u64::from(if rounded == 0.0 { 0.0_f32.to_bits() } else { rounded.to_bits() });
            key = key.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    key
}

fn valid_world3d_camera_bounds(minimum: [f32; 3], maximum: [f32; 3]) -> bool {
    (0..3).all(|axis| minimum[axis].is_finite() && maximum[axis].is_finite() && maximum[axis] >= minimum[axis])
        && ((maximum[0] - minimum[0]).powi(2) + (maximum[1] - minimum[1]).powi(2) + (maximum[2] - minimum[2]).powi(2)).sqrt() > 0.0
}

pub fn apply_world3d_camera_fit(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, scene_state: &mut WorldSceneState, key: u64, request: World3dCameraFitRequest, minimum: [f32; 3], maximum: [f32; 3]) -> World3dCameraFitStep {
    if !valid_world3d_camera_bounds(minimum, maximum) {
        camera_state.fit_bounds_cursor = None;
        return World3dCameraFitStep::Idle;
    }
    camera_state.orbit = ui_wgpu::wgpu::frame_orbit_to_bounds(&camera_state.orbit, minimum, maximum, session_state.bounds.w, session_state.bounds.h, request.padding);
    camera_state.fit_applied_key = Some(key);
    camera_state.fit_bounds_cursor = None;
    advance_world3d_view_revision(session_state, scene_state);
    World3dCameraFitStep::Applied
}

/// 🎯️ Retains React's `WorldAutoFit` request. The actual frame waits for the delivered camera and
/// resident draw geometry, then advances through [`step_world3d_camera_fit`] without a traversal burst.
pub fn sync_world3d_scene_fit(camera_state: &mut WorldCameraState, fit_json: Option<&str>) {
    let request = fit_json
        .and_then(|json| serde_json::from_str::<World3dSceneFitRecord>(json).ok())
        .filter(|fit| fit.enabled)
        .map(|fit| World3dCameraFitRequest {
            revision: fit.revision,
            padding: fit.padding.unwrap_or(f64::from(ui_wgpu::wgpu::WORLD_FRAME_BOUNDS_MARGIN)).max(1.0) as f32,
            bounds: fit
                .bounds_min
                .zip(fit.bounds_max)
                .map(|(minimum, maximum)| (minimum.map(|value| value as f32), maximum.map(|value| value as f32)))
                .filter(|(minimum, maximum)| valid_world3d_camera_bounds(*minimum, *maximum)),
        });
    if camera_state.fit_request != request {
        camera_state.fit_request = request;
        camera_state.fit_bounds_cursor = None;
    }
    if request.is_none() {
        camera_state.fit_owner = None;
        camera_state.fit_applied_key = None;
    }
}

/// ⏰️ Whether the fit lane owes another frame. An empty measurement sleeps on the exact residency
/// generations it inspected; a later mesh publication or draw seal differs and wakes it once.
pub fn world3d_camera_fit_work_pending(camera_state: &WorldCameraState, scene_state: &WorldSceneState) -> bool {
    let Some(request) = camera_state.fit_request else { return false };
    let owner = World3dCameraFitOwner { revision: request.revision, camera_digest: camera_state.scene_camera_digest };
    if camera_state.fit_owner != Some(owner) {
        return true;
    }
    if camera_state.fit_applied_key == Some(world3d_camera_fit_key(owner, request.bounds)) || camera_state.camera_user_moved {
        return false;
    }
    if request.bounds.is_some() {
        return true;
    }
    !camera_state.fit_bounds_cursor.is_some_and(|cursor| {
        cursor.empty_generation() && cursor.key == world3d_camera_fit_key(owner, None) && cursor.draw_generation == scene_state.draw_generation && cursor.geometry_generation == scene_state.geometry_generation
    })
}

/// 🎯️ `World3dScene.selection_json` — both the plain-instance half (`ids`/`hoveredId`, what the
/// framework `interactionSelect`/`interactionHover` verbs address) and the component half
/// (`granularity`/`componentIds`/`hoveredComponent`, the unconverted `worldPick` mechanism).
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct World3dSceneSelectionRecord {
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub ids: Vec<String>,
    #[serde(default)]
    pub gumball_selection_ids: Vec<String>,
    #[serde(default)]
    pub gumball_target: Option<[f32; 3]>,
    #[serde(default)]
    hovered_id: Option<String>,
    #[serde(default)]
    pub granularity: Option<String>,
    #[serde(default)]
    pub component_ids: Vec<serde_json::Value>,
    #[serde(default)]
    hovered_component: Option<World3dSceneHoveredComponent>,
    #[serde(default)]
    pub targets: Option<WorldSelectionTargets>,
    #[serde(default)]
    pub active_object_id: Option<String>,
    #[serde(default)]
    pub show_edges: Option<bool>,
    /// 🎛️ `selection_json.gumballConfig` — the PLUGIN-AUTHORED handle groups, which override the
    /// single-mode `transformMode` and let one gumball offer several groups at once (React's
    /// `WorldSelectionRecord.gumballConfig` / `UnifiedGumball`).
    #[serde(default)]
    pub gumball_config: Option<World3dSceneGumballConfigRecord>,
    /// 🎛️ `selection_json.transformMode` — the single-mode fallback
    /// (`gumballConfigForTransformMode`) when no `gumballConfig` is authored.
    #[serde(default)]
    pub transform_mode: Option<String>,
    /// ⚡️ `selection_json.gumballLiveDispatch` — React's `WorldSelectionRecord.gumballLiveDispatch`.
    #[serde(default)]
    pub gumball_live_dispatch: Option<bool>,
}

/// 🎯️ `selection_json.hoveredComponent` — the vertex/edge/face under the pointer, if any.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct World3dSceneHoveredComponent {
    #[serde(default)]
    pub object_id: Option<String>,
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub id: Option<serde_json::Value>,
}

/// 🔢️ A JSON scalar id (`10`, `"10"`) as the decimal string every component registry keys by.
fn scene_component_id_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(text) => Some(text.clone()),
        serde_json::Value::Number(number) => Some(number.to_string()),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value_derive::RetireOwned)]
pub enum World3dSceneBridgePhase {
    Parse,
    ScalarAttributes,
    Meshes,
    Pages,
    Close,
}

/// 🌉️ One in-flight `meshes_json`/`instances_json` → `World3dSnapshotLease` build. Parsing, mesh
/// publication and page admission are three separately budgeted phases so a large preview never
/// blocks a frame.
#[derive(semio_framework_value_derive::RetireOwned)]
pub struct World3dSceneBridgeCursor {
    pub digest: u64,
    pub camera_digest: u64,
    pub meshes_json: String,
    pub instances_json: String,
    pub camera_json: String,
    pub camera_changed: bool,
    pub phase: World3dSceneBridgePhase,
    pub meshes: Vec<World3dSceneMeshEntry>,
    pub instances: Vec<World3dSceneInstanceEntry>,
    pub camera: Option<World3dSceneCameraRecord>,
    pub mesh_cursor: usize,
    pub scalar_mesh_cursor:usize,
    pub scalar_slot_cursor:usize,
    pub retiring_mesh: bool,
    pub mesh_digest:Option<WorldMeshDigestCursor>,
    pub scalar_field: Option<ui_wgpu::wgpu::World3dScalarField>,
}

/// 🎯️ The plain-JSON instance lane may retain at most the objects and identifier bytes the
/// interaction registry can actually address. This is the existing native authority, shared by render
/// ids, topology ids and per-instance granularity overrides; the bridge never invents a second cap.
pub fn world3d_scene_instance_interactions_are_bounded(instances: &[World3dSceneInstanceEntry]) -> bool {
    instances.len() <= WORLD_INTERACTION_OBJECT_CAPACITY
        && instances.iter().all(|instance| {
            instance.id.len() <= WORLD_INTERACTION_ID_BYTE_CAPACITY
                && instance.mesh_id.len() <= WORLD_INTERACTION_ID_BYTE_CAPACITY
                && instance.interaction_id.as_ref().is_none_or(|id| id.len() <= WORLD_INTERACTION_ID_BYTE_CAPACITY)
                && instance.interaction_granularity_id.as_ref().is_none_or(|id| id.len() <= WORLD_INTERACTION_ID_BYTE_CAPACITY)
        })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dSceneBridgeStep {
    Idle,
    Pending,
    Complete,
    Fault,
}

/// 🌉️ Notes a scene whose producer published no typed snapshot of its own, so the next
/// [`step_world3d_scene_bridge`] turns its plain-JSON mesh/instance lanes into one. Cheap and
/// idempotent: an unchanged payload re-stages nothing.
pub fn stage_world3d_scene_bridge(camera_state: &mut WorldCameraState, bridge_state: &mut WorldBridgeState, world: &ui_wgpu::wgpu::World3dScene) {
    let camera_digest = world3d_scene_digest(&[&world.camera_json]);
    let field_key = world.scalar_field.as_ref().map(|field| world.lanes.iter().find(|lane| lane.lane == "scalarField").map_or_else(|| ui_wgpu::wgpu::scene_lane_hash(&ui_wgpu::wgpu::world3d_modelling_lane_text(field)), |lane| lane.hash.clone())).unwrap_or_default();
    let digest = world3d_scene_digest(&[&world.meshes_json, &world.instances_json, &world.camera_json, &field_key]);
    if bridge_state.scene_bridge_digest == Some(digest) || bridge_state.scene_bridge.as_ref().is_some_and(|cursor| cursor.digest == digest) {
        return;
    }
    if bridge_state.scene_bridge.is_some() || bridge_state.scene_bridge_close.is_some() {
        return;
    }
    bridge_state.scene_bridge = Some(World3dSceneBridgeCursor {
        digest,
        camera_digest,
        meshes_json: world.meshes_json.clone(),
        instances_json: world.instances_json.clone(),
        camera_json: world.camera_json.clone(),
        camera_changed: camera_state.scene_camera_digest != Some(camera_digest),
        phase: World3dSceneBridgePhase::Parse,
        meshes: Vec::new(),
        instances: Vec::new(),
        camera: None,
        mesh_cursor: 0,
        scalar_mesh_cursor:0,
        scalar_slot_cursor:0,
        retiring_mesh: false,
        mesh_digest:None,
        scalar_field: world.scalar_field.clone(),
    });
}

/// ♻️ Drains one superseded bridge lease, or the whole staged build, one authority step at a time.
/// Returns `true` when it did work — the retirement ladder polls it before anything else so a
/// world that is being torn down never leaves a snapshot slot reserved.
pub fn step_world3d_scene_bridge_close(bridge_state: &mut WorldBridgeState,context:&mut semio_framework_job::StepContext<'_>) -> bool {
    if let Some(owner)=bridge_state.scene_bridge_close.as_mut(){
        let result=owner.step(context.retained_grant());
        let progress=match result{Ok(step)=>step.progress(),Err(_)=>owner.step_progress()};
        context.consume_retained(progress).expect("original scene cursor retirement fits original remaining grant");
        if owner.terminal_is_empty(){bridge_state.scene_bridge_close=None;}
        return true;
    }
    if bridge_state.scene_bridge.is_some(){
        let grant=context.retained_grant();
        let bytes=size_of::<World3dSceneBridgeCursor>();
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return true;}
        let original=bridge_state.scene_bridge.take().unwrap();
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(original){
            Ok(owner)=>{bridge_state.scene_bridge_close=Some(owner);context.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}).expect("original scene cursor handoff fits admitted typed move");},
            Err((_,original))=>bridge_state.scene_bridge=Some(original),
        }
        return true;
    }
    let Some((lease, begun)) = bridge_state.scene_bridge_retired else {
        let Some(lease) = bridge_state.scene_bridge_lease.take() else { return false };
        bridge_state.scene_bridge_retired = Some((lease, false));
        return true;
    };
    if !begun {
        match ui_wgpu::wgpu::world3d_snapshot_begin_close(lease) {
            Ok(()) => bridge_state.scene_bridge_retired = Some((lease, true)),
            Err(_) => bridge_state.scene_bridge_retired = None,
        }
        return true;
    }
    match ui_wgpu::wgpu::world3d_snapshot_close_step(lease) {
        Ok(true) | Err(_) => bridge_state.scene_bridge_retired = None,
        Ok(false) => {}
    }
    true
}

/// 🔏️ Content digest of one mesh's buffers — what decides whether an already-published mesh key
/// can be reused rather than rewritten through the authority ladder.
#[derive(semio_framework_value_derive::RetireOwned)]
pub struct WorldMeshDigestCursor {pub phase:usize,pub item:usize,pub hash:u64,pub metadata:semio_framework::mesh_io::text::MeshMetadataCursor,pub text:String}

impl Default for WorldMeshDigestCursor {fn default()->Self{Self{phase:0,item:0,hash:0xcbf2_9ce4_8422_2325,metadata:Default::default(),text:String::from("{")}}}

impl WorldMeshDigestCursor {
    pub fn bytes(&mut self,bytes:&[u8]) {for byte in bytes{self.hash^=u64::from(*byte);self.hash=self.hash.wrapping_mul(0x0000_0100_0000_01b3);}}
    pub fn step(&mut self,mesh:&WorldMeshBuffers)->Result<Option<u64>,String>{
        let floats=match self.phase {0=>Some(&mesh.positions),1=>Some(&mesh.normals),2=>Some(&mesh.colors),3=>Some(&mesh.uvs),4=>Some(&mesh.edge_positions),5=>Some(&mesh.edge_uvs),_=>None};
        if let Some(values)=floats {if let Some(value)=values.get(self.item){self.bytes(&value.to_bits().to_le_bytes());self.item+=1;return Ok(None)}}
        let integers=match self.phase {6=>Some(&mesh.indices),7=>Some(&mesh.face_ids),8=>Some(&mesh.vertex_ids),9=>Some(&mesh.edge_ids),_=>None};
        if let Some(values)=integers {if let Some(value)=values.get(self.item){self.bytes(&value.to_le_bytes());self.item+=1;return Ok(None)}}
        if self.phase==10 {if let Some(byte)=mesh.edge_is_seam.get(self.item){self.bytes(&[*byte]);self.item+=1;return Ok(None)}}
        if self.phase==11 {if let Some(byte)=mesh.paint_texture_base64.as_ref().and_then(|text|text.as_bytes().get(self.item)){self.bytes(&[*byte]);self.item+=1;return Ok(None)}}
        if self.phase==12 {
            let start=self.text.len();let done=self.metadata.step(&mesh.attributes,&mesh.materials,&mesh.textures,Some(&mesh.component_references),None,None,&mut self.text)?;
            let bytes=self.text.as_bytes()[start..].to_vec();self.bytes(&bytes);let last=self.text.chars().last().unwrap_or('{');self.text.clear();self.text.push(last);
            if done {self.phase=13;return Ok(Some(self.hash))}return Ok(None)
        }
        if self.phase==13{return Ok(Some(self.hash))}
        self.bytes(&(self.item as u64).to_le_bytes());self.bytes(&[self.phase as u8]);self.phase+=1;self.item=0;Ok(None)
    }
}

/// 🧹️ One turn of the superseded-mesh close ladder — the same steps the full-state retirement runs,
/// reachable outside a teardown so a live scene can replace a mesh key without wedging.
pub fn step_world3d_mesh_close(scene_state: &mut WorldSceneState,context:&mut semio_framework_job::StepContext<'_>){
    if context.retirement_should_yield(){return}
    if scene_state.dynamic_mesh_close.is_none(){if let Some(entry)=scene_state.dynamic_blocked_mesh.take(){scene_state.dynamic_mesh_close=Some(entry);}return}
    let entry=scene_state.dynamic_mesh_close.as_mut().unwrap();let grant=context.retained_grant();
    let result=entry.value.close_step(grant);let Ok((complete,progress))=result else{return};
    if context.consume_retained(progress).is_err(){return}if progress.copied_items!=0{context.consume_fuel(progress.copied_items as u64);return}
    if !complete{return}
    let Ok(step)=world_close_string(&mut entry.id,grant)else{return};if context.consume_retained(step.progress()).is_err(){return}if step.progress().copied_items!=0{context.consume_fuel(1);}if matches!(step,semio_framework_job::InteractiveJobCloseStep::Complete{..}){scene_state.dynamic_mesh_close.take();}
}

/// 🌉️ Builds and seals the snapshot the bridge publishes: one `Mesh` page per draw (flags `31`,
/// carrying both this draw's and the whole snapshot's credits) followed by that draw's `Instance`
/// pages, then a `Camera` page when the window's camera measure itself changed. Every page is built
/// and validated BEFORE the store is touched, so admission can only fail on a bug.
/// 🌉️ Whether a staged build has anything a snapshot can carry: at least one published mesh with at
/// least one instance, or a camera page. A snapshot is at least one page by construction
/// (`world3d_snapshot_begin` refuses `page_count == 0`), so a build with neither publishes NOTHING —
/// it is not a capacity fault.
///
/// 🩸️ It used to go straight to [`publish_world3d_scene_bridge_snapshot`], whose `pages.is_empty()`
/// guard answers `World3dSnapshotFault::Capacity`, and the wgpu host quarantines the surface on any
/// bridge fault. Every mesh update that momentarily published no DRAWABLE geometry therefore killed
/// the preview for good: measured on 6118 after the third Form slider edit, where the guest's
/// intermediate wire was `meshes=169b instances=267b` — mesh records the parse phase drops for
/// carrying no triangles — against `3644b`/`4786b` for every settled sample, and the surface
/// quarantined with `fault=Some(Capacity) state-meshes=3`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-end-to-end-verification-2026-09-14.md` §5.3).
/// The first build of a surface survived only by accident: its camera digest is always new, so it
/// always had a camera page.
pub fn world3d_scene_bridge_has_pages(scene_state: &WorldSceneState, cursor: &World3dSceneBridgeCursor) -> bool {
    if cursor.camera_changed && cursor.camera.is_some() {
        return true;
    }
    cursor.meshes.iter().any(|mesh| (scene_state.meshes.contains_key(&mesh.id) || scene_mesh_awaits_its_asset(scene_state, &mesh.id)) && cursor.instances.iter().any(|instance| instance.mesh_id == mesh.id))
}

/// 🥽️ A url-declared mesh has no geometry yet and still earns its draw: the draw is what
/// `render_world_3d`'s missing-mesh loop reads to reserve the GLB fetch, and what starts
/// painting by itself the frame the decoded mesh lands under this key. Dropping it here was
/// the second half of the dead url lane — the bridge kept the instances and then threw the
/// draw away, so `state-draws=0 state-instances=0` (`📓️w3a-asset-decoder-boot-fault.md` §6).
///
/// 🌫️ Locked dimming rides the authored colour's alpha, so it survives the snapshot
/// page (which carries a colour, not a style) and composes with whatever
/// `MESH_STYLE_PAINT` row the instance later resolves to.
pub fn publish_world3d_scene_bridge_snapshot(session_state: &mut WorldSessionState, selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, bridge_state: &mut WorldBridgeState, cursor: &World3dSceneBridgeCursor) -> Result<World3dSnapshotLease, World3dSnapshotFault> {
    let (neutral, neutral_source) = scene_bridge_neutral_color(scene_state);
    interaction_state.instance_interaction_ids.clear();
    for instance in cursor.instances.iter().filter(|instance| instance.interaction_id.as_deref().is_some_and(|target| target != instance.id)) {
        interaction_state.instance_interaction_ids.insert(instance.id.clone(), instance.interaction_id.clone().expect("instance interaction id filtered above"));
    }
    interaction_state.instance_interaction_granularity_ids.clear();
    for instance in cursor.instances.iter().filter(|instance| instance.interaction_granularity_id.is_some()) {
        interaction_state.instance_interaction_granularity_ids.insert(instance.id.clone(), instance.interaction_granularity_id.clone().expect("instance interaction granularity filtered above"));
    }
    scene_state.instance_positions.clear();
    for instance in &cursor.instances {
        scene_state.instance_positions.insert(instance.id.clone(), instance.position.unwrap_or([0.0; 3]));
    }
    selection_state.provisional_instance_ids = cursor.instances.iter().filter(|instance| instance.provisional).map(|instance| instance.id.clone()).collect();
    selection_state.highlighted_instance_ids = cursor.instances.iter().filter(|instance| instance.highlighted).map(|instance| instance.id.clone()).collect();
    selection_state.disabled_instance_ids = cursor.instances.iter().filter(|instance| instance.disabled).map(|instance| instance.id.clone()).collect();
    selection_state.celebrating_instance_ids = cursor.instances.iter().filter(|instance| instance.celebrating).map(|instance| instance.id.clone()).collect();
    let mut draws: Vec<(&World3dSceneMeshEntry, Vec<&World3dSceneInstanceEntry>)> = Vec::new();
    for mesh in &cursor.meshes {
        if !scene_state.meshes.contains_key(&mesh.id) && !scene_mesh_awaits_its_asset(scene_state, &mesh.id) {
            continue;
        }
        let instances: Vec<&World3dSceneInstanceEntry> = cursor.instances.iter().filter(|instance| instance.mesh_id == mesh.id).collect();
        if !instances.is_empty() {
            draws.push((mesh, instances));
        }
    }
    let draw_count = u16::try_from(draws.len()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let instance_total = u32::try_from(draws.iter().map(|(_, instances)| instances.len()).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let draw_bytes = u32::try_from(draws.iter().map(|(mesh, instances)| mesh.id.len() + instances.iter().map(|instance| instance.id.len()).sum::<usize>()).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let mut pages: Vec<ui_wgpu::wgpu::World3dSnapshotPage> = Vec::new();
    for (ordinal, (mesh, instances)) in draws.iter().enumerate() {
        let own_instances = u32::try_from(instances.len()).map_err(|_| World3dSnapshotFault::Capacity)?;
        let own_bytes = u32::try_from(mesh.id.len() + instances.iter().map(|instance| instance.id.len()).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
        let mut page = ui_wgpu::wgpu::World3dSnapshotPage::new(World3dSnapshotPageKind::Mesh);
        let span = page.push_string(&mesh.id)?;
        page.push_item(World3dSnapshotItem {
            strings: [Some(span), None, None, None],
            indexes: [own_instances, own_bytes, u32::try_from(ordinal).map_err(|_| World3dSnapshotFault::Capacity)?, u32::from(draw_count), instance_total, draw_bytes, 0, 0],
            index_len: 6,
            flags: 31,
            ..Default::default()
        })?;
        page.seal()?;
        pages.push(page);
        for chunk in instances.chunks(ui_wgpu::wgpu::WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY.min(ui_wgpu::wgpu::WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY / (WORLD_DYNAMIC_ID_BYTE_CAPACITY + 128))) {
            let mut page = ui_wgpu::wgpu::World3dSnapshotPage::new(World3dSnapshotPageKind::Instance);
            for instance in chunk {
                let span = page.push_string(&instance.id)?;
                let source_spans = match instance.component_source.as_ref() {
                    Some(source) => {
                        ComponentSource3d::new(&source.handle, &source.revision).ok_or(World3dSnapshotFault::PageState)?;
                        [Some(page.push_string(&source.handle)?), Some(page.push_string(&source.revision)?)]
                    }
                    None => [None, None],
                };
                let position = instance.position.unwrap_or([0.0; 3]);
                let rotation = instance.rotation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                let scale = instance.scale.unwrap_or([1.0; 3]);
                let (mut color, color_source) = instance.color.as_deref().map_or((neutral, neutral_source), |color| (parse_color(color), SceneColorSource3d::Authored));
                if let Some(opacity) = instance.opacity {
                    color[3] *= (opacity as f32).clamp(0.0, 1.0);
                }
                page.push_item(World3dSnapshotItem {
                    strings: [Some(span), source_spans[0], source_spans[1], None],
                    numbers: [position[0], position[1], position[2], rotation[0], rotation[1], rotation[2], rotation[3], scale[0], scale[1], scale[2], f64::from(color[0]), f64::from(color[1]), f64::from(color[2]), f64::from(color[3]), 0.0, 0.0],
                    number_len: 14,
                    flags: color_source.snapshot_flag(),
                    ..Default::default()
                })?;
            }
            page.seal()?;
            pages.push(page);
        }
    }
    if let Some(camera) = cursor.camera.as_ref().filter(|_| cursor.camera_changed) {
        let position = camera.position.unwrap_or([4.0, 4.0, 4.0]);
        let target = camera.target.unwrap_or([0.0; 3]);
        let up = camera.up.unwrap_or([0.0, 0.0, 1.0]);
        let parallel = camera.projection().is_parallel();
        let mut page = ui_wgpu::wgpu::World3dSnapshotPage::new(World3dSnapshotPageKind::Camera);
        page.push_item(World3dSnapshotItem {
            numbers: [position[0], position[1], position[2], target[0], target[1], target[2], up[0], up[1], up[2], camera.fov_degrees(), camera.zoom.unwrap_or(1.0), if parallel { 1.0 } else { 0.0 }, 0.0, 0.0, 0.0, 0.0],
            number_len: 12,
            ..Default::default()
        })?;
        page.seal()?;
        pages.push(page);
    }
    if pages.is_empty() {
        return Err(World3dSnapshotFault::Capacity);
    }
    let page_count = u16::try_from(pages.len()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let item_count = u32::try_from(pages.iter().map(ui_wgpu::wgpu::World3dSnapshotPage::item_count).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
    let byte_count = u32::try_from(pages.iter().map(ui_wgpu::wgpu::World3dSnapshotPage::byte_count).sum::<usize>()).map_err(|_| World3dSnapshotFault::Capacity)?;
    bridge_state.scene_bridge_generation = bridge_state.scene_bridge_generation.wrapping_add(1).max(1);
    let revision = session_state.interaction_revision.checked_add(1).ok_or(World3dSnapshotFault::Capacity)?;
    let descriptor =
        ui_wgpu::wgpu::World3dSnapshotDescriptor { revision, generation: bridge_state.scene_bridge_generation, page_count, item_count, byte_count, draw_count, draw_instance_count: instance_total, draw_byte_count: draw_bytes };
    let token = ui_wgpu::wgpu::world3d_snapshot_begin(descriptor)?;
    for page in pages {
        if let Err(rejected) = ui_wgpu::wgpu::world3d_snapshot_admit_page(token, page) {
            let fault = rejected.fault;
            let _ = ui_wgpu::wgpu::world3d_snapshot_abort_write(token);
            for _ in 0..usize::from(page_count) + 2 {
                if ui_wgpu::wgpu::world3d_snapshot_abort_write_step(token).unwrap_or(true) {
                    break;
                }
            }
            return Err(fault);
        }
    }
    ui_wgpu::wgpu::world3d_snapshot_seal(token)
}

/// 🎨️ The base color an instance without its own `color` paints with: the scene environment's
/// neutral material override when it declares one, else a theme-independent neutral the shaded pass
/// lights (the React reference resolves the same fallback from its `neutral` mesh-style palette).
fn scene_bridge_neutral_color(scene_state: &WorldSceneState) -> ([f32; 4], SceneColorSource3d) {
    scene_state
        .environment
        .material
        .as_ref()
        .and_then(|material| material.color.as_deref())
        .map_or(([1.0; 4], SceneColorSource3d::SemanticNeutral), |color| (parse_color(color), SceneColorSource3d::Environment))
}

/// 🎯️ Applies the scene's own selection document to this world's live selection/hover channels —
/// only when the document itself changed, so an optimistic local preview between two identical
/// refreshes is never clobbered (the rule the React host follows too).
///
/// 🎛️ Authored config WINS over the mode, exactly as React's `gumballConfig ?? gumballConfigForTransformMode(mode)`.
pub fn sync_world3d_scene_selection(session_state: &mut WorldSessionState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, selection_json: &str) {
    let digest = world3d_scene_digest(&[selection_json]);
    if selection_state.scene_selection_digest == Some(digest) {
        return;
    }
    selection_state.scene_selection_digest = Some(digest);
    let Ok(record) = serde_json::from_str::<World3dSceneSelectionRecord>(selection_json) else { return };
    let component_ids:Vec<String>=record.component_ids.iter().filter_map(scene_component_id_text).collect();
    let (hovered_object,hovered_mode,hovered_id)=record.hovered_component.map_or((None,None,None),|hovered|(hovered.object_id,hovered.mode,hovered.id.as_ref().and_then(scene_component_id_text)));
    let gumball_target=record.gumball_target.filter(|target|target.iter().all(|value|value.is_finite()));
    let changed=selection_state.selected_ids!=record.ids || gumball_state.gumball_selection_ids!=record.gumball_selection_ids || gumball_state.gumball_target!=gumball_target
        || selection_state.local_hover_id!=record.hovered_id || selection_state.local_hover_granularity_id.is_some() || selection_state.component_ids!=component_ids
        || selection_state.hovered_component_object_id!=hovered_object || selection_state.hovered_component_mode!=hovered_mode || selection_state.hovered_component_id!=hovered_id
        || selection_state.active_object_id!=record.active_object_id || gumball_state.gumball_config!=record.gumball_config || gumball_state.gumball_live_dispatch!=record.gumball_live_dispatch.unwrap_or(false)
        || record.method.as_ref().is_some_and(|method|selection_state.selection_method!=*method) || record.granularity.as_ref().is_some_and(|granularity|selection_state.granularity!=*granularity)
        || record.targets.as_ref().is_some_and(|targets|selection_state.selection_targets!=*targets) || record.show_edges.is_some_and(|show_edges|selection_state.show_edges!=show_edges)
        || record.transform_mode.as_ref().is_some_and(|mode|gumball_state.transform_mode!=*mode);
    if let Some(method) = record.method {
        selection_state.selection_method = method;
    }
    selection_state.selected_ids = record.ids;
    gumball_state.gumball_selection_ids = record.gumball_selection_ids;
    gumball_state.gumball_target = gumball_target;
    selection_state.local_hover_id = record.hovered_id;
    selection_state.local_hover_granularity_id = None;
    if let Some(granularity) = record.granularity {
        selection_state.granularity = granularity;
    }
    selection_state.component_ids = component_ids;
    selection_state.hovered_component_object_id = hovered_object;
    selection_state.hovered_component_mode = hovered_mode;
    selection_state.hovered_component_id = hovered_id;
    if let Some(targets) = record.targets {
        selection_state.selection_targets = targets;
    }
    selection_state.active_object_id = record.active_object_id;
    if let Some(show_edges) = record.show_edges {
        selection_state.show_edges = show_edges;
    }
    if let Some(transform_mode) = record.transform_mode {
        gumball_state.transform_mode = transform_mode;
    }
    gumball_state.gumball_config = record.gumball_config;
    gumball_state.gumball_live_dispatch = record.gumball_live_dispatch.unwrap_or(false);
    if changed {advance_world3d_view_revision(session_state, scene_state);}
}

/// 🎯️ Constrains the live selection channels to the scene's pick granularity filter, the native twin of
/// `world3dSelectionWithPickFilter` (`🌐️World3dHost/📏️modelling`): hover and selection reach only the chosen
/// granularity, components of another granularity are dropped, and no filter leaves the guest-authored channels alone.
/// Idempotent: it advances the view revision only when it changed something.
pub fn apply_world3d_pick_filter(session_state: &mut WorldSessionState, selection_state: &mut WorldSelectionState, scene_state: &mut WorldSceneState, filter: Option<ui_wgpu::wgpu::World3dPickGranularity>) {
    use ui_wgpu::wgpu::World3dPickGranularity as Granularity;
    let Some(filter) = filter else { return };
    let (granularity, targets) = match filter {
        Granularity::Shape => ("mesh", WorldSelectionTargets { vertex: false, edge: false, face: false }),
        Granularity::Face => ("face", WorldSelectionTargets { vertex: false, edge: false, face: true }),
        Granularity::Edge => ("edge", WorldSelectionTargets { vertex: false, edge: true, face: false }),
        Granularity::Vertex => ("vertex", WorldSelectionTargets { vertex: true, edge: false, face: false }),
    };
    let component_filter = filter != Granularity::Shape;
    let keep_components = component_filter && selection_state.granularity == granularity;
    let keep_hovered = component_filter && selection_state.hovered_component_mode.as_deref() == Some(granularity);
    let mut changed = false;
    if selection_state.granularity != granularity {
        selection_state.granularity = granularity.to_string();
        changed = true;
    }
    if selection_state.selection_targets != targets {
        selection_state.selection_targets = targets;
        changed = true;
    }
    if !keep_components && !selection_state.component_ids.is_empty() {
        selection_state.component_ids.clear();
        changed = true;
    }
    if !keep_hovered && (selection_state.hovered_component_mode.is_some() || selection_state.hovered_component_id.is_some() || selection_state.hovered_component_object_id.is_some()) {
        selection_state.hovered_component_mode = None;
        selection_state.hovered_component_id = None;
        selection_state.hovered_component_object_id = None;
        changed = true;
    }
    if changed {
        advance_world3d_view_revision(session_state, scene_state);
    }
}

pub fn srgb_byte_to_linear(byte: u8) -> f32 {
    let value = f32::from(byte) / 255.0;
    if value <= 0.04045 { value / 12.92 } else { ((value + 0.055) / 1.055).powf(2.4) }
}

/// 🌡️ Validates the original field cardinality before any mesh attribute is displaced.
pub fn world3d_scalar_field_fits(data: &WorldMeshBuffers, field: &ui_wgpu::wgpu::World3dScalarField) -> bool {
    use ui_wgpu::wgpu::World3dScalarDomain;
    let expected = match field.domain {
        World3dScalarDomain::Vertex => data.vertex_count(),
        World3dScalarDomain::Face => data.indices.len() / 3,
    };
    field.values.len()==expected
}

pub fn world3d_apply_scalar_field_colors(data:&mut WorldMeshBuffers,field:&ui_wgpu::wgpu::World3dScalarField){
    use ui_wgpu::wgpu::World3dScalarDomain;
    let bytes = field.color_bytes();
    let linear: Vec<[f32; 4]> = bytes.chunks_exact(3).map(|rgb| [srgb_byte_to_linear(rgb[0]), srgb_byte_to_linear(rgb[1]), srgb_byte_to_linear(rgb[2]), 1.0]).collect();
    let colored: Vec<String> = data.attributes.iter().filter(|(_, attribute)| attribute.semantic == semio_framework::MeshAttributeSemantic::Color).map(|(name, _)| name.clone()).collect();
    for name in colored {
        data.attributes.remove(name.as_str());
    }
    match field.domain {
        World3dScalarDomain::Vertex => data.colors = linear.into_iter().flatten().collect(),
        World3dScalarDomain::Face => {
            data.colors.clear();
            let values = linear.into_iter().map(|color| semio_framework_value::DslValue::Array(color.into_iter().map(|channel| semio_framework_value::DslValue::float(f64::from(channel))).collect())).collect();
            data.attributes.insert(
                "scalarField".into(),
                semio_framework::MeshAttribute { domain: semio_framework::MeshAttributeDomain::Face, semantic: semio_framework::MeshAttributeSemantic::Color, interpolation: semio_framework::MeshAttributeInterpolation::Constant, values, indices: None },
            );
        }
    }
}

/// 🖼️ Applies the scene's vortex/attraction/target-volume/reference JSON lanes — the same payloads
/// React's `World3dHost` parses every frame, but which the wgpu host had never copied into
/// `World3dState` (so document-tree references never reached `render_world_3d`'s textured planes).
///
/// 🧲️ Caps applied at PARSE time (see `world_pick_targets_from_json`), so no later reader has to
/// know the bound and the retained set can never exceed it however large the lane arrives.
pub fn sync_world3d_scene_document_lanes(session_state: &mut WorldSessionState, selection_state: &mut WorldSelectionState, scene_state: &mut WorldSceneState, bridge_state: &mut WorldBridgeState, assets_state: &mut WorldAssetsState, world: &ui_wgpu::wgpu::World3dScene) {
    let digest = world3d_scene_digest(&[
        world.vortices_json.as_deref().unwrap_or(""),
        world.attractions_json.as_deref().unwrap_or(""),
        world.target_volumes_json.as_deref().unwrap_or(""),
        world.references_json.as_deref().unwrap_or(""),
        world.engagement_preview_json.as_deref().unwrap_or(""),
        world.pick_targets_json.as_deref().unwrap_or(""),
    ]);
    if bridge_state.scene_document_lanes_digest == Some(digest) {
        return;
    }
    bridge_state.scene_document_lanes_digest = Some(digest);
    scene_state.vortices = world.vortices_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    scene_state.attractions = world.attractions_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    scene_state.target_volumes = world.target_volumes_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    let references: Vec<WorldReferenceRecord> = world.references_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    reconcile_world3d_reference_urls(scene_state, assets_state, &references);
    scene_state.references = references;
    scene_state.engagement_preview = world.engagement_preview_json.as_deref().map(|json| serde_json::from_str(json).unwrap_or_default()).unwrap_or_default();
    selection_state.pick_targets = world.pick_targets_json.as_deref().map(world_pick_targets_from_json).unwrap_or_default();
    if selection_state.pick_hover_key.as_deref().is_some_and(|key| !selection_state.pick_targets.iter().any(|target| world_pick_keys_match(Some(target.key().as_str()), Some(key)))) {
        selection_state.pick_hover_key = None;
    }
    session_state.interaction_revision = session_state.interaction_revision.wrapping_add(1);
}

fn reconcile_world3d_reference_urls(scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState, references: &[WorldReferenceRecord]) {
    let removed: Vec<String> = scene_state.reference_pixels.keys().filter(|url| !references.iter().any(|reference| reference.url.as_deref() == Some(url.as_str()))).cloned().collect();
    for url in removed {
        if !retire_world_pixels(scene_state, &url, false) {
            break;
        }
    }
    assets_state.pending_image_urls.retain(|url| references.iter().any(|reference| reference.url.as_ref() == Some(url)));
    assets_state.asset_io.cancel_reference_image_urls_except(references);
}

/// 🥽️📥️ Stores one decoded GLB under the id the wire names it by — `mesh_id_from_url` is the exact
/// twin of the guest's `world3d_mesh_id_from_url`, which is what makes a `{"id": "mesh:x", "url":
/// "/mesh/x.glb"}` entry's draw start painting the moment its bytes land. Clears the url's pending
/// mark on success so the lane stops offering it (see [`reserve_world3d_mesh_fetch`]).
pub fn publish_world3d_asset_mesh_lease(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState, url: &str, mesh: Mesh3dLease) -> Result<(), WorldDynamicRejected<Mesh3dLease>> {
    if mesh.revision() != session_state.interaction_revision {
        return Err(WorldDynamicRejected { fault: WorldDynamicFault::StaleToken, id: mesh_id_from_url(url), value: mesh });
    }
    publish_world3d_mesh_lease(interaction_state, scene_state, mesh_id_from_url(url), mesh)?;
    assets_state.pending_glb_urls.remove(url);
    Ok(())
}

#[cfg(any(test, feature = "testkit"))]
fn selection_mode_label(selection_state: &WorldSelectionState) -> &'static str {
    match selection_state.granularity.as_str() {
        "vertex" | "edge" | "face" => "component",
        _ => "mesh",
    }
}

pub fn component_mode_active(selection_state: &WorldSelectionState) -> bool {
    matches!(selection_state.granularity.as_str(), "vertex" | "edge" | "face" | "component")
}

#[cfg(any(test, feature = "testkit"))]
pub fn pointer_drag_distance(selection_state: &WorldSelectionState, x: f32, y: f32) -> f32 {
    let Some(start) = selection_state.marquee_points.first() else {
        return 0.0;
    };
    let dx = x - start[0];
    let dy = y - start[1];
    (dx * dx + dy * dy).sqrt()
}

#[cfg(any(test, feature = "testkit"))]
pub fn is_click_gesture(selection_state: &WorldSelectionState, x: f32, y: f32) -> bool {
    pointer_drag_distance(selection_state, x, y) <= CLICK_DRAG_THRESHOLD_PX
}

fn push_line_segment(lines: &mut Vec<LineVertex3d>, from: Vec3, to: Vec3, color: [f32; 4]) {
    lines.push(LineVertex3d { position: from.to_array(), color });
    lines.push(LineVertex3d { position: to.to_array(), color });
}

pub const VERTEX_MARKER_MESH: &str = "vertex-marker";

pub const VERTEX_BASE_SCALE: f32 = 0.05;

pub const VERTEX_HOVER_SCALE: f32 = 0.09;

const VERTEX_SELECT_SCALE: f32 = 0.09;

/// 🔘️ React's `WORLD_VERTEX_DOT_PX` / `WORLD_VERTEX_MARK_PX` (`🌐️World3dHost/🟦️.tsx:2499`) — the
/// plain and the hovered/selected vertex marker, in SCREEN PIXELS, drawn with
/// `sizeAttenuation={false}`.
///
/// 🩸️ These markers were world-unit crosses of `VERTEX_BASE_SCALE * 0.15` = 0.0075 world units,
/// which is sub-pixel on any scene larger than a few metres — the SAME defect React shipped and
/// fixed on lowpoly (`pointsMaterial size` in world units → `sizeAttenuation={false}` pixels,
/// ticket 26/08/29 round 4). The scales above stay as the RELATIVE weight of the three states.
pub const WORLD_VERTEX_DOT_PX: f32 = 6.0;

const WORLD_VERTEX_MARK_PX: f32 = 11.0;

/// 📏️ World units one screen pixel spans at `distance` from the eye — the inverse of
/// `sizeAttenuation`. Perspective opens with distance; a parallel frustum is distance-invariant and
/// reads straight off `orthographic_half_extent`.
pub fn world_units_per_pixel(camera: &Camera3d, viewport: Rect, distance: f32) -> f32 {
    let height = viewport.h.max(1.0);
    if camera.projection.is_parallel() {
        let (_, half_height) = camera.orthographic_half_extent(viewport.w, height);
        return (half_height * 2.0 / height).max(1e-6);
    }
    let half = (camera.fov_y * 0.5).tan() / camera.zoom.max(1e-6);
    (2.0 * distance.abs().max(1e-3) * half / height).max(1e-6)
}

/// 🔘️ Half the world-space span of one vertex marker cross at `centre` — React's pixel size,
/// converted through [`world_units_per_pixel`] so the marker keeps its apparent size at every zoom.
pub fn vertex_marker_half_extent(camera: &Camera3d, viewport: Rect, centre: Vec3, scale: f32, mark_px: Option<f32>) -> f32 {
    let pixels = if scale > VERTEX_BASE_SCALE { mark_px.unwrap_or(WORLD_VERTEX_MARK_PX) } else { WORLD_VERTEX_DOT_PX };
    pixels * 0.5 * world_units_per_pixel(camera, viewport, camera.position.sub(centre).length())
}

/// 🖍️ Resolves the scene's sub-element highlight tokens through the live theme with the shared `world3d_sub_element_paint` table; a granularity without a token keeps the built-in palette.
pub fn world3d_sub_element_paint_native(theme: &ui_wgpu::wgpu::Theme, highlight: Option<&ui_wgpu::wgpu::World3dHighlight>) -> WorldSubElementPaint {
    let defaults = ui_wgpu::wgpu::World3dSubElementDefaults { select: None, hover: None, edge_hover: None, edge_width: 0.0, vertex_mark_px: 0.0 };
    let paint = ui_wgpu::wgpu::world3d_sub_element_paint(highlight, |tone| Some(world3d_tone_rgba(theme, tone)), defaults);
    WorldSubElementPaint {
        face_select: paint.face_select,
        face_hover: paint.face_hover,
        edge_select: paint.edge_select,
        edge_hover: paint.edge_hover,
        vertex_select: paint.vertex_select,
        vertex_hover: paint.vertex_hover,
        vertex_mark_px: highlight.and_then(|highlight| highlight.vertex).and_then(|style| style.width_px).map(|width| width as f32),
    }
}

/// 🎨️ A theme tone token as an opaque colour: `neutral` is the foreground, `primary`/`secondary`/`tertiary` the celebration triad, the rest the outcome palette of the same name.
pub fn world3d_tone_rgba(theme: &ui_wgpu::wgpu::Theme, tone: ui_wgpu::wgpu::World3dTone) -> [f32; 4] {
    use ui_wgpu::wgpu::World3dTone;
    let color = match tone {
        World3dTone::Neutral => theme.text,
        World3dTone::Primary => theme.celebrate[0],
        World3dTone::Secondary => theme.celebrate[1],
        World3dTone::Tertiary => theme.celebrate[2],
        World3dTone::Success => theme.success,
        World3dTone::Warning => theme.warning,
        World3dTone::Danger => theme.error,
        World3dTone::Info => Rgba::from_token(&ui_styling::colors::INFO),
    };
    [color.r, color.g, color.b, 1.0]
}

pub fn component_overlay_color(id: &str, selected: &HashSet<String>, preview: &HashSet<String>, hovered: &Option<String>, tokens: (Option<[f32; 4]>, Option<[f32; 4]>)) -> Option<([f32; 4], f32)> {
    let (select, hover) = tokens;
    if preview.contains(id) {
        return Some((hover.unwrap_or([1.0, 0.85, 0.35, 1.0]), VERTEX_HOVER_SCALE));
    }
    if hovered.as_deref() == Some(id) {
        return Some((hover.map_or([0.35, 0.75, 1.0, 0.9], |color| [color[0], color[1], color[2], 0.9]), VERTEX_HOVER_SCALE));
    }
    if selected.contains(id) {
        return Some((select.unwrap_or([0.35, 0.75, 1.0, 1.0]), VERTEX_SELECT_SCALE));
    }
    None
}

pub fn mesh_face_id(mesh: Mesh3dLease, tri_index: u32) -> String {
    world_mesh_component_id(mesh, Mesh3dField::FaceIds, tri_index).to_string()
}

pub fn face_component_mode_active(selection_state: &WorldSelectionState) -> bool {
    selection_state.selection_targets.face || selection_state.granularity == "face"
}

fn instance_hovered_component_id(selection_state: &WorldSelectionState, instance_id: &str) -> Option<String> {
    if !pick_targets_instance(selection_state, instance_id) {
        return None;
    }
    if selection_state.hovered_component_mode.as_deref() != Some(selection_state.granularity.as_str()) {
        return None;
    }
    if selection_state.hovered_component_object_id.as_deref().is_some_and(|object_id| object_id != instance_id) {
        return None;
    }
    selection_state.hovered_component_id.clone()
}

pub fn append_component_vertex_spheres() -> Vec<Instance3d> {
    Vec::new()
}

/// 📐️ Axis-aligned box wireframe as 12 line segments between `min` and `max`.
fn append_box_wireframe_lines(lines: &mut Vec<LineVertex3d>, min: Vec3, max: Vec3, color: [f32; 4]) {
    let corner = |ix: usize, iy: usize, iz: usize| Vec3::new(if ix == 0 { min.x } else { max.x }, if iy == 0 { min.y } else { max.y }, if iz == 0 { min.z } else { max.z });
    const EDGES: [((usize, usize, usize), (usize, usize, usize)); 12] = [
        ((0, 0, 0), (1, 0, 0)),
        ((1, 0, 0), (1, 1, 0)),
        ((1, 1, 0), (0, 1, 0)),
        ((0, 1, 0), (0, 0, 0)),
        ((0, 0, 1), (1, 0, 1)),
        ((1, 0, 1), (1, 1, 1)),
        ((1, 1, 1), (0, 1, 1)),
        ((0, 1, 1), (0, 0, 1)),
        ((0, 0, 0), (0, 0, 1)),
        ((1, 0, 0), (1, 0, 1)),
        ((1, 1, 0), (1, 1, 1)),
        ((0, 1, 0), (0, 1, 1)),
    ];
    for ((ax, ay, az), (bx, by, bz)) in EDGES {
        lines.push(LineVertex3d { position: corner(ax, ay, az).to_array(), color });
        lines.push(LineVertex3d { position: corner(bx, by, bz).to_array(), color });
    }
}

const WORLD_REFERENCE_LOCKED_OPACITY_SCALE: f32 = 0.35;

const WORLD_REFERENCE_SELECTED_CONTENT_OPACITY_SCALE: f32 = 0.5;

const WORLD_REFERENCE_OUTLINE_SCALE: f32 = 1.002;

#[derive(Clone, Copy)]
struct WorldReferenceVisual {
    pub background: [f32; 4],
    pub content_opacity: f32,
    pub outline: Option<[f32; 4]>,
}

fn world_reference_state_id_matches(value: &str, id: &str) -> bool {
    value == id || value.strip_prefix("reference:") == Some(id)
}

pub fn world_reference_visual(selection_state: &WorldSelectionState, reference: &WorldReferenceRecord, theme: &ui_wgpu::wgpu::Theme) -> WorldReferenceVisual {
    let id = reference.id.as_deref().or(reference.url.as_deref()).unwrap_or_default();
    let locked = reference.locked.unwrap_or(false);
    let selected = !locked && selection_state.selected_ids.iter().any(|value| world_reference_state_id_matches(value, id));
    let hovered = !locked && selection_state.local_hover_id.as_deref().is_some_and(|value| world_reference_state_id_matches(value, id));
    let base = (reference.opacity.unwrap_or(1.0) as f32).clamp(0.0, 1.0) * if locked { WORLD_REFERENCE_LOCKED_OPACITY_SCALE } else { 1.0 };
    if selected {
        return WorldReferenceVisual {
            background: [theme.selected.r, theme.selected.g, theme.selected.b, theme.selected.a],
            content_opacity: base * WORLD_REFERENCE_SELECTED_CONTENT_OPACITY_SCALE,
            outline: Some([theme.celebrate[0].r, theme.celebrate[0].g, theme.celebrate[0].b, 1.0]),
        };
    }
    if hovered {
        return WorldReferenceVisual {
            background: [theme.row_hover.r, theme.row_hover.g, theme.row_hover.b, theme.row_hover.a],
            content_opacity: base,
            outline: Some([theme.celebrate[1].r, theme.celebrate[1].g, theme.celebrate[1].b, 0.9]),
        };
    }
    WorldReferenceVisual { background: [0.0; 4], content_opacity: base, outline: None }
}

pub fn append_world_reference_outline(lines: &mut Vec<LineVertex3d>, origin: [f64; 3], width: f32, height: f32, color: [f32; 4]) {
    let half_width = width * 0.5 * WORLD_REFERENCE_OUTLINE_SCALE;
    let half_height = height * 0.5 * WORLD_REFERENCE_OUTLINE_SCALE;
    let center = Vec3::new(origin[0] as f32, origin[1] as f32, origin[2] as f32);
    let bottom_left = center.add(Vec3::new(-half_width, -half_height, 0.0));
    let bottom_right = center.add(Vec3::new(half_width, -half_height, 0.0));
    let top_right = center.add(Vec3::new(half_width, half_height, 0.0));
    let top_left = center.add(Vec3::new(-half_width, half_height, 0.0));
    push_line_segment(lines, bottom_left, bottom_right, color);
    push_line_segment(lines, bottom_right, top_right, color);
    push_line_segment(lines, top_right, top_left, color);
    push_line_segment(lines, top_left, bottom_left, color);
}

/// 🎨️ Resolves one `pick_targets_json` style reference — a theme TOKEN id or a `#rrggbb` literal —
/// against this appearance. The token set is the CAD picking engine's `SpatialScenePalette`
/// (`✏️editor/⚙️engine/🧲️picking/🦀️.rs:1108`), resolved through the same CSS chain React does
/// (`🎨️styling/🖌️ui/🎨️.css`): `--color-changed-selected` → `--accent`, `--active-base` and
/// `--color-changed-hovered`/`--accent-secondary` → the primary/secondary celebrate stops, the
/// `--hover-*` grays → the one hover fill this theme carries. An unknown token falls back rather
/// than painting black.
fn world_pick_style_color(theme: &ui_wgpu::wgpu::Theme, reference: Option<&str>, fallback: [f32; 4]) -> [f32; 4] {
    let rgba = |color: Rgba| [color.r, color.g, color.b, color.a];
    let Some(reference) = reference.filter(|value| !value.is_empty()) else { return fallback };
    if reference.starts_with('#') {
        return parse_color(reference);
    }
    match reference {
        "--color-changed-selected" => rgba(theme.accent),
        "--active-base" => rgba(theme.celebrate[0]),
        "--color-changed-hovered" | "--accent-secondary" => rgba(theme.celebrate[1]),
        "--foreground" => rgba(theme.text),
        "--muted-foreground" => rgba(theme.text_muted),
        "--border-normal-color" => rgba(theme.border_normal),
        "--hover-base" | "--hover-panel" | "--hover-window" => rgba(theme.row_hover),
        _ => fallback,
    }
}

/// 🧲️ Paints the HOVERED sub-object pick target in its own `target_style` colour — the wgpu twin of
/// React's `SpatialPickTargetNode` for the one target under the pointer.
///
/// ⚖️ Only the hovered row, deliberately: the whole overlay is already drawn by the
/// `engagementPreview` lane (packet W2f), and painting every pick target a second time here would
/// double the line budget of a Concrete-Forest pane for nothing. A vertex target draws the same
/// screen-sized cross the component markers do; every other kind draws the wireframe of the box the
/// hit test itself uses, so what the user sees IS what was hit.
pub fn append_pick_target_hover_lines(selection_state: &WorldSelectionState, theme: &ui_wgpu::wgpu::Theme, camera: &Camera3d, viewport: Rect, lines: &mut Vec<LineVertex3d>) {
    let Some(key) = selection_state.pick_hover_key.as_deref() else { return };
    let Some(target) = selection_state.pick_targets.iter().find(|target| world_pick_keys_match(Some(target.key().as_str()), Some(key))) else { return };
    let style = target.style.as_ref();
    let mut color = world_pick_style_color(theme, style.and_then(|style| style.color.as_deref()), [theme.row_hover.r, theme.row_hover.g, theme.row_hover.b, 1.0]);
    color[3] = style.and_then(|style| style.opacity).map_or(color[3], |opacity| (opacity as f32).clamp(0.0, 1.0));
    let Some((min, max)) = world_pick_target_bounds(target) else { return };
    if target.kind == "vertex" {
        let centre = Vec3::new((min[0] + max[0]) * 0.5, (min[1] + max[1]) * 0.5, (min[2] + max[2]) * 0.5);
        let d = vertex_marker_half_extent(camera, viewport, centre, VERTEX_HOVER_SCALE, None);
        push_line_segment(lines, centre.sub(Vec3::new(d, 0.0, 0.0)), centre.add(Vec3::new(d, 0.0, 0.0)), color);
        push_line_segment(lines, centre.sub(Vec3::new(0.0, d, 0.0)), centre.add(Vec3::new(0.0, d, 0.0)), color);
        push_line_segment(lines, centre.sub(Vec3::new(0.0, 0.0, d)), centre.add(Vec3::new(0.0, 0.0, d)), color);
        return;
    }
    append_box_wireframe_lines(lines, Vec3::new(min[0], min[1], min[2]), Vec3::new(max[0], max[1], max[2]), color);
}

/// 🤝️ Paints the `engagementPreview` lane as world-space lines — the wgpu twin of React's
/// `EngagementPreviewLayer` (`🌐️World3dHost/🟦️.tsx:4027`), which draws the same four kinds with
/// `colors.hover` (the `hover-interactive-fill` token, `theme.row_hover` here).
///
/// 🖍️ A `point` becomes a three-axis cross rather than a sphere and a `box-preview` a wireframe
/// rather than a wireframe mesh: this target has one line pipeline and no per-item mesh, and React
/// already draws the box `wireframe`, so the silhouette matches. The `0.05` floors on width/depth/
/// height and the "height extrudes up from `cornerA.z`, never from `cornerB.z`" rule are React's.
pub fn append_engagement_preview_lines(scene_state: &WorldSceneState, lines: &mut Vec<LineVertex3d>, color: [f32; 4]) {
    const POINT_CROSS_EXTENT: f32 = 0.08;
    const PREVIEW_MIN_EXTENT: f64 = 0.05;
    for item in &scene_state.engagement_preview {
        match item.kind.as_deref().unwrap_or_default() {
            "point" => {
                let Some(position) = item.position else { continue };
                let center = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
                for axis in [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)] {
                    lines.push(LineVertex3d { position: center.sub(axis.scale(POINT_CROSS_EXTENT)).to_array(), color });
                    lines.push(LineVertex3d { position: center.add(axis.scale(POINT_CROSS_EXTENT)).to_array(), color });
                }
            }
            "segment" => {
                let (Some(from), Some(to)) = (item.from, item.to) else { continue };
                lines.push(LineVertex3d { position: [from[0] as f32, from[1] as f32, from[2] as f32], color });
                lines.push(LineVertex3d { position: [to[0] as f32, to[1] as f32, to[2] as f32], color });
            }
            "box-preview" => {
                let (Some(corner_a), Some(corner_b)) = (item.corner_a, item.corner_b) else { continue };
                let half_width = (corner_b[0] - corner_a[0]).abs().max(PREVIEW_MIN_EXTENT) * 0.5;
                let half_depth = (corner_b[1] - corner_a[1]).abs().max(PREVIEW_MIN_EXTENT) * 0.5;
                let height = item.height.unwrap_or(PREVIEW_MIN_EXTENT).abs().max(PREVIEW_MIN_EXTENT);
                let center_x = (corner_a[0] + corner_b[0]) * 0.5;
                let center_y = (corner_a[1] + corner_b[1]) * 0.5;
                let min = Vec3::new((center_x - half_width) as f32, (center_y - half_depth) as f32, corner_a[2] as f32);
                let max = Vec3::new((center_x + half_width) as f32, (center_y + half_depth) as f32, (corner_a[2] + height) as f32);
                append_box_wireframe_lines(lines, min, max, color);
            }
            "linear-handle" => {
                let (Some(axis), Some(origin)) = (item.axis, item.origin) else { continue };
                let start = Vec3::new(origin[0] as f32, origin[1] as f32, origin[2] as f32);
                let end = start.add(Vec3::new(axis[0] as f32, axis[1] as f32, axis[2] as f32));
                lines.push(LineVertex3d { position: start.to_array(), color });
                lines.push(LineVertex3d { position: end.to_array(), color });
            }
            _ => {}
        }
    }
}

pub fn append_component_overlays(selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState, overlay_state: &WorldOverlayState, theme: &ui_wgpu::wgpu::Theme, camera: &Camera3d, viewport: Rect, lines: &mut Vec<LineVertex3d>) {
    let wire_color = [0.55, 0.65, 0.8, 0.75];
    let outline_color = selection_state.show_edges.then(|| environment_outline_color(&scene_state.environment, theme)).flatten();
    let requested_wire = selection_state.interaction_mode == "paint" || component_mode_active(selection_state) || outline_color.is_some() || selection_state.selection_targets.edge || (selection_state.granularity == "mesh" && !selection_state.component_ids.is_empty());
    {
        for draw in &scene_state.draws {
            let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            let primary_wire = schema.indices == 0 && schema.edges > 0;
            let semantic = schema.edge_ids == schema.edges;
            if schema.edges == 0 || (!requested_wire && !primary_wire) || (outline_color.is_none() && !semantic && !primary_wire) {
                continue;
            }
            for instance in &draw.instances {
                let color = if primary_wire { mesh_style_paint(theme, resolve_mesh_style(world_instance_style(selection_state, instance))).line } else { outline_color.unwrap_or(wire_color) };
                for edge_index in 0..schema.edges {
                    let Ok(edge) = mesh.edge(edge_index) else { continue };
                    let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                    let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                    lines.push(LineVertex3d { position: a.to_array(), color });
                    lines.push(LineVertex3d { position: b.to_array(), color });
                }
            }
        }
    }
    for draw in &scene_state.draws {
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        let Ok(schema) = mesh.schema() else { continue };
        if schema.edge_ids != schema.edges {
            continue;
        }
        for instance in &draw.instances {
            let selected = world_instance_selected_components(selection_state, gumball_state, scene_state,instance,&selection_state.granularity);
            let preview: HashSet<String> = selection_state.marquee_preview_ids.iter().filter(|_|selection_state.active_object_id.as_deref()==Some(&instance.id)).cloned().collect();
            let hovered = instance_hovered_component_id(selection_state, &instance.id);
            if selection_state.granularity.as_str() != "edge" {
                continue;
            }
            if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                continue;
            }
            for edge_index in 0..schema.edges {
                let Ok(edge) = mesh.edge(edge_index) else { continue };
                let id = world_mesh_component_id(mesh, Mesh3dField::EdgeIds, edge_index).to_string();
                let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered, (overlay_state.sub_element_paint.edge_select, overlay_state.sub_element_paint.edge_hover)) else {
                    continue;
                };
                let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                push_line_segment(lines, a, b, color);
            }
        }
    }
    if face_component_mode_active(selection_state) {
        for draw in &scene_state.draws {
            let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            for instance in &draw.instances {
                let selected = world_instance_selected_components(selection_state, gumball_state, scene_state,instance,&selection_state.granularity);
                let preview: HashSet<String> = selection_state.marquee_preview_ids.iter().filter(|_|selection_state.active_object_id.as_deref()==Some(&instance.id)).cloned().collect();
                let hovered = instance_hovered_component_id(selection_state, &instance.id);
                if hovered.is_none() && selected.is_empty() && preview.is_empty() {
                    continue;
                }
                for tri_index in 0..schema.indices / 3 {
                    let Some(tri) = world_mesh_triangle(mesh, tri_index) else { continue };
                    let id = mesh_face_id(mesh, tri_index);
                    let Some((color, _)) = component_overlay_color(&id, &selected, &preview, &hovered, (overlay_state.sub_element_paint.face_select, overlay_state.sub_element_paint.face_hover)) else {
                        continue;
                    };
                    let Some(verts) = Option::zip(Option::zip(mesh_vertex(mesh, tri[0]), mesh_vertex(mesh, tri[1])), mesh_vertex(mesh, tri[2]))
                        .map(|((a, b), c)| [instance.model.transform_point(a), instance.model.transform_point(b), instance.model.transform_point(c)])
                    else {
                        continue;
                    };
                    push_line_segment(lines, verts[0], verts[1], color);
                    push_line_segment(lines, verts[1], verts[2], color);
                    push_line_segment(lines, verts[2], verts[0], color);
                }
            }
        }
    }
    {
        let wire_color = [0.55, 0.65, 0.8, 0.9];
        for draw in &scene_state.draws {
            let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                continue;
            };
            let Ok(schema) = mesh.schema() else { continue };
            if schema.vertex_ids != schema.vertices { continue; }
            let primary_point = schema.indices == 0 && schema.edges == 0;
            if !primary_point && !selection_state.selection_targets.vertex && selection_state.granularity != "vertex" { continue; }
            for instance in &draw.instances {
                let selected = world_instance_selected_components(selection_state, gumball_state, scene_state,instance,"vertex");
                let preview: HashSet<String> = selection_state.marquee_preview_ids.iter().filter(|_|selection_state.active_object_id.as_deref()==Some(&instance.id)).cloned().collect();
                let hovered = instance_hovered_component_id(selection_state, &instance.id);
                for vertex_index in 0..schema.vertices {
                    let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                    let group = world_mesh_component_id(mesh, Mesh3dField::VertexIds, vertex_index);
                    if group == u32::MAX { continue; }
                    let id = group.to_string();
                    let center = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                    let base_color = if primary_point { mesh_style_paint(theme, resolve_mesh_style(world_instance_style(selection_state, instance))).line } else { wire_color };
                    let vertex_tokens = (overlay_state.sub_element_paint.vertex_select, overlay_state.sub_element_paint.vertex_hover);
                    let (color, scale) = component_overlay_color(&id, &selected, &preview, &hovered, vertex_tokens).unwrap_or((base_color, VERTEX_BASE_SCALE));
                    if !primary_point && !selection_state.selection_targets.vertex && component_overlay_color(&id, &selected, &preview, &hovered, vertex_tokens).is_none() {
                        continue;
                    }
                    let d = vertex_marker_half_extent(camera, viewport, center, scale, overlay_state.sub_element_paint.vertex_mark_px);
                    push_line_segment(lines, center.sub(Vec3::new(d, 0.0, 0.0)), center.add(Vec3::new(d, 0.0, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, d, 0.0)), center.add(Vec3::new(0.0, d, 0.0)), color);
                    push_line_segment(lines, center.sub(Vec3::new(0.0, 0.0, d)), center.add(Vec3::new(0.0, 0.0, d)), color);
                }
            }
        }
    }
}

pub fn selection_centroid(selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState) -> Option<Vec3> {
    if let Some(target) = gumball_state.gumball_target {
        return Some(Vec3::new(target[0], target[1], target[2]));
    }
    if selection_state.selected_ids.is_empty() {
        return None;
    }
    let mut sum = Vec3::ZERO;
    let mut count = 0u32;
    for draw in &scene_state.draws {
        for instance in &draw.instances {
            if selection_state.selected_ids.iter().any(|id| id == &instance.id) {
                let t = instance.model.cols[3];
                sum = sum.add(Vec3::new(t[0], t[1], t[2]));
                count += 1;
            }
        }
    }
    if count == 0 {
        None
    } else {
        Some(sum.scale(1.0 / count as f32))
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn pick_gumball_handle_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<GumballHandle> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let pivot = selection_centroid(selection_state, gumball_state, scene_state)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let pick_radius = extent * 0.08;
    let eye = gumball_eye(&camera, pivot);
    let mut best: Option<(f32, GumballHandle)> = None;
    let axes = [(GumballHandle::MoveX, Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]), (GumballHandle::MoveY, Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]), (GumballHandle::MoveZ, Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0])];
    for (handle, axis, _) in axes {
        let end = pivot.add(axis.scale(extent));
        if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
            if dist <= pick_radius && best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                best = Some((dist, handle));
            }
        }
    }
    let planes = [(GumballHandle::MoveXY, Vec3::new(0.0, 0.0, 1.0), extent * 0.35), (GumballHandle::MoveYZ, Vec3::new(1.0, 0.0, 0.0), extent * 0.35), (GumballHandle::MoveXZ, Vec3::new(0.0, 1.0, 0.0), extent * 0.35)];
    for (handle, normal, half) in planes {
        if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
            let offset = hit.sub(pivot);
            let u = if normal.z.abs() > 0.9 {
                offset.x.abs()
            } else if normal.x.abs() > 0.9 {
                offset.y.abs()
            } else {
                offset.x.abs()
            };
            let v = if normal.z.abs() > 0.9 { offset.y.abs() } else { offset.z.abs() };
            if u <= half && v <= half {
                let dist = origin.sub(hit).length();
                if best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    if matches!(gumball_state.transform_mode.as_str(), "rotate" | "rotateSelection") {
        for handle in [GumballHandle::RotateX, GumballHandle::RotateY, GumballHandle::RotateZ] {
            let Some(normal) = handle.plane_normal() else {
                continue;
            };
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let radial = hit.sub(pivot);
                let dist_ring = (radial.length() - extent * 0.85).abs();
                if dist_ring <= pick_radius * 2.0 && best.as_ref().is_none_or(|(best_dist, _)| dist_ring < *best_dist) {
                    best = Some((dist_ring, handle));
                }
            }
        }
    }
    if matches!(gumball_state.transform_mode.as_str(), "scale" | "scaleSelection") {
        for handle in [GumballHandle::ScaleX, GumballHandle::ScaleY, GumballHandle::ScaleZ] {
            let Some(axis) = handle.axis_dir() else {
                continue;
            };
            let end = pivot.add(axis.scale(extent * 1.1));
            if let Some(dist) = ray_segment_distance(origin, dir, pivot, end) {
                if dist <= pick_radius && best.as_ref().is_none_or(|(best_dist, _)| dist < *best_dist) {
                    best = Some((dist, handle));
                }
            }
        }
    }
    let _ = eye;
    best.map(|(_, handle)| handle)
}

/// 🎛️ Same gate the PICK uses (`world3d_gumball_config`), so a handle can never be drawn without
/// being pickable or picked without being drawn — React resolves both from one `gumballConfig`.
///
/// 🚨️ The draw this function pushes is answered on the GPU by `mesh_store.get_versioned`, and a
/// miss there is a FATAL `present_step` fault, not a skipped draw. Every other world draw site
/// pairs its `SceneDraw3d` with exactly this `ensure_mesh`; the gumball did not, so the first
/// frame after a selection died with `prepared frame submit step: prepared world mesh was
/// missing` and took the whole frame loop with it. Unreachable until the wgpu host actually ran
/// the reserved tool job that applies a selection (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn append_gumball_geometry(
    lines: &mut Vec<LineVertex3d>,
    translucent: &mut Vec<SceneDraw3d>,
    gpu: &mut World3dBuildContext,
    selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState,
    camera: &Camera3d,
    meshes: &WorldMeshRegistry,
    mesh_versions: &WorldDynamicRegistry<u64, WORLD_DYNAMIC_MESH_CAPACITY>,
) {
    let Some(pivot) = selection_centroid(selection_state, gumball_state, scene_state) else {
        return;
    };
    let extent = gumball_extent(camera.position.sub(pivot).length());
    let config = world3d_gumball_config(gumball_state);
    let axis_colors = [(GumballHandle::MoveX, Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 1.0]), (GumballHandle::MoveY, Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 1.0]), (GumballHandle::MoveZ, Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 1.0])];
    for (handle, axis, color) in axis_colors {
        if !config.admits(handle) {
            continue;
        }
        let end = pivot.add(axis.scale(extent));
        lines.push(LineVertex3d { position: pivot.to_array(), color });
        lines.push(LineVertex3d { position: end.to_array(), color });
    }
    let ring_segments = 48usize;
    for (handle, normal, color) in
        [(GumballHandle::RotateX, Vec3::new(1.0, 0.0, 0.0), [0.92, 0.25, 0.25, 0.85]), (GumballHandle::RotateY, Vec3::new(0.0, 1.0, 0.0), [0.25, 0.85, 0.35, 0.85]), (GumballHandle::RotateZ, Vec3::new(0.0, 0.0, 1.0), [0.35, 0.55, 0.95, 0.85])]
    {
        if !config.admits(handle) {
            continue;
        }
        let tangent_a = if normal.x.abs() > 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
        let tangent_b = normal.cross(tangent_a).normalize();
        let tangent_a = tangent_b.cross(normal).normalize();
        let radius = extent * 0.85;
        for step in 0..ring_segments {
            let a0 = step as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let a1 = (step + 1) as f32 / ring_segments as f32 * std::f32::consts::TAU;
            let p0 = pivot.add(tangent_a.scale(a0.cos() * radius)).add(tangent_b.scale(a0.sin() * radius));
            let p1 = pivot.add(tangent_a.scale(a1.cos() * radius)).add(tangent_b.scale(a1.sin() * radius));
            lines.push(LineVertex3d { position: p0.to_array(), color });
            lines.push(LineVertex3d { position: p1.to_array(), color });
        }
    }
    if let Some(plane) = meshes.get(GUMBALL_PLANE_MESH) {
        let mesh_version = *mesh_versions.get(GUMBALL_PLANE_MESH).unwrap_or(&0);
        gpu.ensure_mesh(GUMBALL_PLANE_MESH, mesh_version, *plane);
        let half = extent * 0.35;
        let plane_specs = [(GumballHandle::MoveXY, Vec3::new(0.0, 0.0, 1.0), [half, half, 1.0]), (GumballHandle::MoveYZ, Vec3::new(1.0, 0.0, 0.0), [1.0, half, half]), (GumballHandle::MoveXZ, Vec3::new(0.0, 1.0, 0.0), [half, 1.0, half])];
        for (handle, normal, scale) in plane_specs {
            if !config.admits(handle) {
                continue;
            }
            let tangent = if normal.z.abs() > 0.9 {
                Vec3::new(1.0, 0.0, 0.0)
            } else if normal.x.abs() > 0.9 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            };
            let bitangent = normal.cross(tangent).normalize();
            let tangent = bitangent.cross(normal).normalize();
            let rotation = quat_from_basis(tangent, bitangent, normal);
            translucent.push(SceneDraw3d {
                mesh_key: GUMBALL_PLANE_MESH.into(),
                mesh_version,
                instances: vec![Instance3d { component_source: None, id: GUMBALL_PLANE_MESH.into(), model: Instance3d::model_from_trs(pivot.to_array(), rotation, scale), color: [0.75, 0.8, 0.9, 0.22], selected: false, hovered: false, material: Default::default() }],
                shadow_role: Default::default(),
            });
        }
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn apply_gumball_preview(selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState) {
    let Some(handle) = gumball_state.gumball_handle else {
        return;
    };
    for draw in &mut scene_state.draws {
        for instance in &mut draw.instances {
            if selection_state.selected_ids.iter().any(|id| id == &instance.id) {
                instance.model = gumball_preview_in_place(instance.model, handle, gumball_state.gumball_preview_translate, gumball_state.gumball_preview_angle, gumball_state.gumball_preview_scale);
            }
        }
    }
}

/// 🎚️ One selected instance's gumball preview, exactly as the release commits it: a move translates the instance, a
/// turn rotates its basis about the handle's world axis IN PLACE — its own origin stays, because every 3D gumball
/// verb's leaf (puzzle `rotate-selection`, shooting `rotate-assets`, …) turns each target about its own origin — and
/// a scale stretches the instance's own local axis, as the per-axis factors of the scale leaves do. The gumball pivot
/// only anchors the widget; the selection never orbits it, so the multi-object preview equals the committed state.
pub fn gumball_preview_in_place(model: Mat4, handle: GumballHandle, translate: Vec3, angle: f32, scale: Vec3) -> Mat4 {
    let mut preview = model;
    if translate.length() > 1e-4 {
        let next = Vec3::new(model.cols[3][0], model.cols[3][1], model.cols[3][2]).add(translate);
        preview.cols[3] = [next.x, next.y, next.z, model.cols[3][3]];
    } else if angle.abs() > 1e-6 {
        if let Some(axis) = handle.axis_dir() {
            for column in 0..3 {
                let turned = rotate_vector(Vec3::new(model.cols[column][0], model.cols[column][1], model.cols[column][2]), axis, angle);
                preview.cols[column] = [turned.x, turned.y, turned.z, model.cols[column][3]];
            }
        }
    } else {
        let (column, factor) = match handle {
            GumballHandle::ScaleX => (0, scale.x),
            GumballHandle::ScaleY => (1, scale.y),
            GumballHandle::ScaleZ => (2, scale.z),
            _ => return preview,
        };
        preview.cols[column] = [model.cols[column][0] * factor, model.cols[column][1] * factor, model.cols[column][2] * factor, model.cols[column][3]];
    }
    preview
}

#[cfg(any(test, feature = "testkit"))]
pub fn reset_gumball_preview(gumball_state: &mut WorldGumballState) {
    gumball_state.gumball_preview_translate = Vec3::ZERO;
    gumball_state.gumball_preview_angle = 0.0;
    gumball_state.gumball_preview_scale = Vec3::new(1.0, 1.0, 1.0);
}

#[cfg(any(test, feature = "testkit"))]
pub fn gumball_commit_action(session_state: &WorldSessionState, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState) -> Option<ActionDescriptor> {
    let handle = gumball_state.gumball_handle?;
    let ids = selection_state.selected_ids.clone();
    if ids.is_empty() {
        return None;
    }
    if handle.is_translate() && gumball_state.gumball_preview_translate.length() > 1e-4 {
        let delta = gumball_state.gumball_preview_translate;
        return Some(ActionDescriptor {
            controller_id: session_state.controller_id.clone(),
            action: "translateSelection".into(),
            args: Some(semio_framework::dsl_value!({
                "surfaceId": session_state.surface_id,
                "windowId": session_state.surface_id,
                "mode": selection_mode_label(selection_state),
                "ids": ids,
                "dx": delta.x as f64,
                "dy": delta.y as f64,
                "dz": delta.z as f64,
            })),
        });
    }
    if handle.is_rotate() && gumball_state.gumball_preview_angle.abs() > 1e-6 {
        let axis = handle.axis_dir()?;
        return Some(ActionDescriptor {
            controller_id: session_state.controller_id.clone(),
            action: "rotateSelection".into(),
            args: Some(semio_framework::dsl_value!({
                "surfaceId": session_state.surface_id,
                "windowId": session_state.surface_id,
                "mode": selection_mode_label(selection_state),
                "ids": ids,
                "ax": axis.x as f64,
                "ay": axis.y as f64,
                "az": axis.z as f64,
                "angle": gumball_state.gumball_preview_angle as f64,
            })),
        });
    }
    if handle.is_scale() {
        let scale = match handle {
            GumballHandle::ScaleX => gumball_state.gumball_preview_scale.x,
            GumballHandle::ScaleY => gumball_state.gumball_preview_scale.y,
            GumballHandle::ScaleZ => gumball_state.gumball_preview_scale.z,
            _ => 1.0,
        };
        if (scale - 1.0).abs() > 1e-6 {
            let (sx, sy, sz) = match handle {
                GumballHandle::ScaleX => (scale, 1.0, 1.0),
                GumballHandle::ScaleY => (1.0, scale, 1.0),
                _ => (1.0, 1.0, scale),
            };
            let args = semio_framework::dsl_value!({
                "surfaceId": session_state.surface_id,
                "windowId": session_state.surface_id,
                "mode": selection_mode_label(selection_state),
                "ids": ids,
                "sx": sx,
                "sy": sy,
                "sz": sz,
            });
            return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "scaleSelection".into(), args: Some(args) });
        }
    }
    None
}

/// 🕒️ `pub` (was crate-private) so the wgpu renderer's wheel-zoom settle-then-dispatch sweep
/// (`AppRuntime::frame`'s `pending_camera_dispatch_deadlines_ms`) can build the same `setCamera`
/// action the pointer-release path below already dispatches — one orbit-to-action mapping, two
/// trigger sites (immediate on release, debounced on wheel settle).
/// 🔎️ The `zoom` scalar of a camera's `setCamera` pose — React's `captureNavigationSnapshot`:
/// `camera instanceof ThreeOrthographicCamera ? camera.zoom : 1`. A perspective pane reports the
/// identity whatever its camera carries; a parallel one reports the live frustum scale, which is the
/// only number its wheel ever moves ([`OrbitController::zoom`]).
pub fn world3d_camera_zoom(camera: &Camera3d) -> f64 {
    if camera.projection.is_parallel() {
        f64::from(camera.zoom)
    } else {
        1.0
    }
}

/// 📦️ Advances camera/view ownership without invalidating a sealed content publication. Snapshot
/// apply exposes its lease one transaction before the retained draw swap; camera reports, initial
/// templates, and projection framing may all legitimately land in that interval.
pub fn advance_world3d_view_revision(session_state: &mut WorldSessionState, scene_state: &mut WorldSceneState) {
    session_state.interaction_revision = session_state.interaction_revision.wrapping_add(1);
    if let Some(rebuild) = scene_state.draw_rebuild.as_mut() {
        rebuild.descriptor.revision = session_state.interaction_revision;
    }
}

/// 🔀️ Applies one locally selected projection while retaining the accepted viewport-owned pose.
/// React's `WorldProjectionSnapDriver` changes family/FOV and rotates only when orientation changes;
/// its content framer stays unmounted after initial ownership. External/initial seeds arm framing in
/// the scene bridge instead.
pub fn apply_world3d_projection_spec(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, scene_state: &mut WorldSceneState, spec: Viewport3dProjectionSpec) -> bool {
    if camera_state.projection_selected && camera_state.projection_spec == spec {
        return false;
    }
    let orientation_changed = camera_state.projection_spec.orientation != spec.orientation;
    let projection = ui_wgpu::wgpu::projection_spec_family(spec);
    if camera_state.orbit.projection != projection {
        if projection.is_parallel() {
            camera_state.orbit.zoom = ui_wgpu::wgpu::world_projection_matched_ortho_zoom(camera_state.orbit.fov_y.to_degrees(), camera_state.orbit.distance, session_state.bounds.h);
        }
    }
    camera_state.orbit.projection = projection;
    camera_state.orbit.fov_y = ui_wgpu::wgpu::projection_spec_fov_degrees(spec).to_radians();
    if orientation_changed {
        let (direction, up) = ui_wgpu::wgpu::projection_spec_orientation_look(spec);
        let direction = direction.normalize();
        camera_state.orbit.yaw = direction.y.atan2(direction.x);
        camera_state.orbit.pitch = direction.z.clamp(-1.0, 1.0).asin();
        camera_state.orbit.up = up;
    }
    camera_state.projection_spec = spec;
    camera_state.projection_selected = true;
    camera_state.projection_frame_policy = Viewport3dProjectionFramePolicy::Content;
    camera_state.projection_frame_owed = false;
    advance_world3d_view_revision(session_state, scene_state);
    true
}

/// 🧭️ Changes a projection mode while preserving the live gizmo orientation.
pub fn apply_world3d_projection_mode(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, scene_state: &mut WorldSceneState, mode: Viewport3dProjectionMode) -> bool {
    apply_world3d_projection_spec(session_state, camera_state, scene_state, Viewport3dProjectionSpec { mode, orientation: camera_state.projection_spec.orientation })
}

/// 🧭️ The live orientation retained across projection-mode changes.
pub fn world3d_projection_orientation(camera_state: &WorldCameraState) -> ui_wgpu::wgpu::WorldProjectionOrientation {
    match camera_state.projection_spec.orientation {
        Viewport3dProjectionOrientation::Cardinal { view } => ui_wgpu::wgpu::WorldProjectionOrientation::Cardinal(match view {
            semio_framework_ui_viewport::Viewport3dOrthographicView::Plan | semio_framework_ui_viewport::Viewport3dOrthographicView::Top => ui_wgpu::wgpu::WorldCardinalView::Top,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Bottom => ui_wgpu::wgpu::WorldCardinalView::Bottom,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Front => ui_wgpu::wgpu::WorldCardinalView::Front,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Back => ui_wgpu::wgpu::WorldCardinalView::Back,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Left => ui_wgpu::wgpu::WorldCardinalView::Left,
            semio_framework_ui_viewport::Viewport3dOrthographicView::Right => ui_wgpu::wgpu::WorldCardinalView::Right,
        }),
        _ => ui_wgpu::wgpu::WorldProjectionOrientation::Free,
    }
}

/// 📐️ The complete accepted projection retained across snapshot delivery and remounts.
pub fn world3d_projection_spec(camera_state: &WorldCameraState) -> Viewport3dProjectionSpec { camera_state.projection_spec }

/// 📐️ Applies a display-template seed once, after the delivered camera has landed. React's pending
/// window template seeds `WorldProjectionRig` before `WorldAutoFit`; keeping the live distance and
/// target here is equivalent for the fit because it consumes only this look direction, then replaces
/// both with the measured rendered bounds.
///
/// 📦️ The snapshot lease becomes resident one transaction before its sealed draw rebuild publishes.
/// Shell applies this camera-only template in that interval, so the live rebuild carries the new
/// view-state revision instead of losing the document and its GLB request ledger as stale.
pub fn apply_world3d_initial_projection_seed(
    session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, scene_state: &mut WorldSceneState, bridge_state: &mut WorldBridgeState,
    spec: Viewport3dProjectionSpec,
) -> bool {
    if camera_state.projection_selected || bridge_state.snapshot_lease.is_none() {
        return false;
    }
    let (direction, up) = ui_wgpu::wgpu::projection_spec_orientation_look(spec);
    let length = direction.length();
    if !length.is_finite() || length <= 1e-6 || up.to_array().into_iter().any(|value| !value.is_finite()) {
        return false;
    }
    let direction = direction.scale(1.0 / length);
    camera_state.orbit.yaw = direction.y.atan2(direction.x);
    camera_state.orbit.pitch = direction.z.clamp(-1.0, 1.0).asin();
    camera_state.orbit.up = up;
    let projection = ui_wgpu::wgpu::projection_spec_family(spec);
    camera_state.orbit.projection = projection;
    camera_state.orbit.fov_y = ui_wgpu::wgpu::projection_spec_fov_degrees(spec).to_radians();
    camera_state.projection_spec = spec;
    camera_state.projection_selected = true;
    camera_state.projection_frame_policy = Viewport3dProjectionFramePolicy::Content;
    camera_state.projection_frame_owed = true;
    camera_state.projection_frame_key = None;
    camera_state.projection_frame_zoom = None;
    advance_world3d_view_revision(session_state, scene_state);
    true
}

/// 🔎️ This surface's live camera family, for the pane chip that reports it.
pub fn world3d_camera_projection(camera_state: &WorldCameraState) -> CameraProjection3d {
    ui_wgpu::wgpu::projection_spec_family(camera_state.projection_spec)
}

/// 🎥️ This surface's LIVE orbit in the very shape `setCamera` carries — the row `dumpMeshStats`
/// reports beside the wire camera, so a probe can diff "what the guest published" against "what the
/// pane is looking through". Carries the two members the wire shape has no room for and a local
/// camera move turns on: the projection FAMILY and the framing ORIENTATION.
pub fn world3d_live_camera_json(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, scene_state: &WorldSceneState) -> serde_json::Value {
    let camera = camera_state.orbit.to_camera();
    let references = scene_state.references.iter().filter(|reference| !reference.hidden.unwrap_or(false)).map(|reference| {
        let url = reference.url.as_deref().unwrap_or_default();
        let [width, height] = world_reference_plane_size(scene_state, url, world_reference_width(reference));
        json!({
            "id": reference.id.as_deref(),
            "url": url,
            "origin": reference.origin.unwrap_or([0.0; 3]),
            "planeSize": [width, height],
        })
    }).collect::<Vec<_>>();
    let content_bounds = world3d_content_bounds(selection_state, scene_state).map(|(minimum, maximum)| json!({ "minimum": minimum, "maximum": maximum }));
    let lod = scene_lod(session_state, camera_state, scene_state);
    let grid_step = lod_grid_step_world(lod, scene_state.lod.grid_factor);
    json!({
        "position": [camera.position.x as f64, camera.position.y as f64, camera.position.z as f64],
        "target": [camera.target.x as f64, camera.target.y as f64, camera.target.z as f64],
        "up": [f64::from(camera.up.x), f64::from(camera.up.y), f64::from(camera.up.z)],
        "zoom": world3d_camera_zoom(&camera),
        "fov": f64::from(camera.fov_y.to_degrees()),
        "projection": camera_state.projection_spec,
        "projectionFrame": camera_state.projection_frame_policy,
        "viewport": [session_state.bounds.x, session_state.bounds.y, session_state.bounds.w, session_state.bounds.h],
        "contentBounds": content_bounds,
        "projectionFrameOwed": camera_state.projection_frame_owed,
        "references": references,
        "grid": { "lod": lod, "factor": scene_state.lod.grid_factor, "step": grid_step },
        "userMoved": camera_state.camera_user_moved,
    })
}

pub fn orbit_camera_action(session_state: &WorldSessionState, camera_state: &WorldCameraState) -> ActionDescriptor {
    let camera = camera_state.orbit.to_camera();
    ActionDescriptor {
        controller_id: session_state.controller_id.clone(),
        action: "setCamera".into(),
        args: Some(semio_framework::dsl_value!({
            "windowId": session_state.surface_id,
            "camera": {
                "position": [
                    camera.position.x as f64,
                    camera.position.y as f64,
                    camera.position.z as f64,
                ],
                "target": [
                    camera.target.x as f64,
                    camera.target.y as f64,
                    camera.target.z as f64,
                ],
                "zoom": world3d_camera_zoom(&camera),
                "up": [f64::from(camera.up.x), f64::from(camera.up.y), f64::from(camera.up.z)],
            }
        })),
    }
}

/// 📥️ Borrows the validated host destination without moving its owner or reconstructing geometry.
pub fn world3d_admit_native_mesh_step(session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState,donor:&mut Option<WorldNativeMeshResource>,receipt:semio_framework::kernel::ActorUiPatchReceipt,revision:WorldNativeUiRevision,resource_ordinal:usize,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),WorldNativeMeshFailure>{
    let Some(row)=donor.as_ref()else{return Err(WorldNativeMeshFailure::Binding)};
    let identity=row.owner.identity();let key=row.mesh_key.as_str();
    if !receipt.is_valid()||revision.0==0||row.surface.as_str()!=session_state.surface_id||key.is_empty()||key.len()>WORLD_INTERACTION_ID_BYTE_CAPACITY{return Err(WorldNativeMeshFailure::Binding)}
    if let Some(WorldMeshSource::Native(source))=scene_state.meshes.0.get(key){return if source.receipt==receipt&&source.revision==revision&&source.resource_ordinal==resource_ordinal&&source.loan.identity()==identity&&source.loan.lease()==row.owner.lease()&&!source.frozen{Ok((true,Default::default()))}else{Err(WorldNativeMeshFailure::Binding)}}
    if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}
    if scene_state.native_mesh_admission.is_none(){
        if scene_state.dynamic_mesh_close.is_some()||scene_state.dynamic_blocked_mesh.is_some(){return Ok((false,Default::default()))}
        let plan=scene_state.meshes.plan_insert(key).map_err(WorldNativeMeshFailure::Registry)?;if plan.occupied{return Err(WorldNativeMeshFailure::Registry(WorldDynamicFault::StaleToken))}
        scene_state.native_mesh_admission=Some(WorldNativeMeshAdmission{identity,lease:row.owner.lease().ok_or(WorldNativeMeshFailure::Binding)?,receipt,revision,resource_ordinal,key:String::new(),extent:key.len(),interaction_id:WorldInteractionId{bytes:[0;WORLD_INTERACTION_ID_BYTE_CAPACITY],len:0},interaction_offset:0,allocated:false});
        return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    let cursor=scene_state.native_mesh_admission.as_mut().unwrap();
    if cursor.identity!=identity||Some(cursor.lease)!=row.owner.lease()||cursor.receipt!=receipt||cursor.revision!=revision||cursor.resource_ordinal!=resource_ordinal||cursor.extent!=key.len()||!key.starts_with(&cursor.key){return Err(WorldNativeMeshFailure::Binding)}
    if !cursor.allocated{if grant.maximum_capacity_bytes<cursor.extent{return Ok((false,Default::default()))}cursor.key.try_reserve_exact(cursor.extent).map_err(|_|WorldNativeMeshFailure::Registry(WorldDynamicFault::ByteCapacity))?;cursor.allocated=true;return Ok((false,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:cursor.key.capacity(),..Default::default()}))}
    if cursor.key.len()<cursor.extent{let start=cursor.key.len();let mut end=start.saturating_add(grant.maximum_copy_bytes).min(cursor.extent);while !key.is_char_boundary(end){end-=1;}if end==start{return Ok((false,Default::default()))}cursor.key.push_str(&key[start..end]);return Ok((false,RetainedCloneProgress{copied_items:1,copied_bytes:end-start,..Default::default()}))}
    if cursor.interaction_offset<cursor.extent{let start=cursor.interaction_offset;let end=start.saturating_add(grant.maximum_copy_bytes).min(cursor.extent);if end==start{return Ok((false,Default::default()))}cursor.interaction_id.bytes[start..end].copy_from_slice(&key.as_bytes()[start..end]);cursor.interaction_offset=end;if end==cursor.extent{cursor.interaction_id.len=end as u16;}return Ok((false,RetainedCloneProgress{copied_items:1,copied_bytes:end-start,..Default::default()}))}
    let plan=scene_state.meshes.plan_insert(key).map_err(WorldNativeMeshFailure::Registry)?;if plan.occupied{return Err(WorldNativeMeshFailure::Registry(WorldDynamicFault::StaleToken))}
    let lease=row.owner.lease().ok_or(WorldNativeMeshFailure::Binding)?;let mut interaction=interaction_state.interaction_meshes.plan_admit_id(cursor.interaction_id,identity.cache_epoch,lease).map_err(WorldNativeMeshFailure::Registry)?;interaction.value.generation=interaction.epoch.checked_add(1).ok_or(WorldNativeMeshFailure::Registry(WorldDynamicFault::StaleToken))?;
    let Some((loan,progress))=row.owner.loan(grant).map_err(WorldNativeMeshFailure::Mesh)?else{return Ok((false,Default::default()))};
    let cursor=scene_state.native_mesh_admission.take().unwrap();let native=WorldNativeMeshSource{loan,receipt,revision,resource_ordinal,lookup_extent:cursor.extent,frozen:false,gpu_returned:false};
    scene_state.meshes.0.commit_insert(plan,cursor.key,WorldMeshSource::Native(native)).expect("exclusive World transaction preserves its prevalidated empty mesh slot");
    interaction_state.interaction_meshes.commit_admit(interaction).expect("exclusive World transaction preserves its prevalidated interaction slot");scene_state.geometry_generation=scene_state.geometry_generation.wrapping_add(1).max(1);
    Ok((true,progress))
}

/// ♻️ Closes a replaced host destination loan only after its exact GPU cache returns.
#[cfg(not(all(target_arch="wasm32",target_env="p2")))]
pub fn world3d_close_native_mesh_step(scene_state: &mut WorldSceneState,key:&str,receipt:semio_framework::kernel::ActorUiPatchReceipt,revision:WorldNativeUiRevision,resource_ordinal:usize,grant:RetainedCloneGrant,retire_gpu:impl FnOnce(&str,&WorldNativeMeshLoan)->Result<Option<ui_gpu::MeshGpuSourceReturn>,&'static str>)->Result<(bool,RetainedCloneProgress),WorldNativeMeshFailure>{
    if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}
    if scene_state.dynamic_mesh_close.is_none(){
        let Some(WorldMeshSource::Native(source))=scene_state.meshes.0.get(key)else{return Ok((true,Default::default()))};
        if source.receipt!=receipt||source.revision!=revision||source.resource_ordinal!=resource_ordinal{return Err(WorldNativeMeshFailure::Binding)}
        let mut entry=scene_state.meshes.remove(key).unwrap();let WorldMeshSource::Native(source)=&mut entry.value else{unreachable!()};source.frozen=true;scene_state.dynamic_mesh_close=Some(entry);return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    let entry=scene_state.dynamic_mesh_close.as_mut().unwrap();let WorldMeshSource::Native(source)=&mut entry.value else{return Err(WorldNativeMeshFailure::Binding)};
    if source.receipt!=receipt||source.revision!=revision||source.resource_ordinal!=resource_ordinal||source.lookup_extent!=key.len()||!key.starts_with(&entry.id){return Err(WorldNativeMeshFailure::Binding)}
    if !source.gpu_returned{let Some(witness)=retire_gpu(key,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?else{return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))};if !witness.validates(key,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?{return Err(WorldNativeMeshFailure::Binding)}source.gpu_returned=true;return Ok((false,RetainedCloneProgress{copied_items:1,..Default::default()}))}
    if !source.loan.terminal_is_empty(){let step=source.loan.return_step(grant).map_err(WorldNativeMeshFailure::Mesh)?;return Ok((false,step.progress))}
    let close=world_close_string(&mut entry.id,grant).map_err(WorldNativeMeshFailure::Close)?;let complete=matches!(close,semio_framework_job::InteractiveJobCloseStep::Complete{..});let progress=if complete{RetainedCloneProgress{copied_items:1,..close.progress()}}else{close.progress()};if complete{scene_state.dynamic_mesh_close.take();}Ok((complete,progress))
}

/// 🧹️ Cancels an unfinished lookup without touching its retained destination owner.
pub fn world3d_close_native_mesh_admission_step(scene_state: &mut WorldSceneState,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),WorldNativeMeshFailure>{if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}let Some(cursor)=scene_state.native_mesh_admission.as_mut()else{return Ok((true,Default::default()))};let step=world_close_string(&mut cursor.key,grant).map_err(WorldNativeMeshFailure::Close)?;let complete=matches!(step,semio_framework_job::InteractiveJobCloseStep::Complete{..});let progress=if complete{RetainedCloneProgress{copied_items:1,..step.progress()}}else{step.progress()};if complete{scene_state.native_mesh_admission.take();}Ok((complete,progress))}

/// 🧊️ The closing World source retains its loan until the original renderer returns its cache.
#[cfg(not(all(target_arch="wasm32",target_env="p2")))]
pub fn world3d_return_native_mesh_gpu_step(scene_state: &mut WorldSceneState,grant:RetainedCloneGrant,retire_gpu:impl FnOnce(&str,&WorldNativeMeshLoan)->Result<Option<ui_gpu::MeshGpuSourceReturn>,&'static str>)->Result<RetainedCloneProgress,WorldNativeMeshFailure>{
    let Some(entry)=scene_state.dynamic_mesh_close.as_mut()else{return Ok(Default::default())};let WorldMeshSource::Native(source)=&mut entry.value else{return Ok(Default::default())};
    if source.gpu_returned||grant.maximum_items==0||grant.maximum_depth==0{return Ok(Default::default())}source.frozen=true;
    let Some(witness)=retire_gpu(&entry.id,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?else{return Ok(RetainedCloneProgress{copied_items:1,..Default::default()})};if !witness.validates(&entry.id,&source.loan).map_err(WorldNativeMeshFailure::Gpu)?{return Err(WorldNativeMeshFailure::Binding)}source.gpu_returned=true;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})
}

pub fn retain_world_blocked_owner(scene_state: &mut WorldSceneState, owner: WorldOpaqueOwner) {
    assert!(scene_state.dynamic_blocked_owner.is_none(), "world dynamic producers must stop after an observable ownership fault");
    scene_state.dynamic_blocked_owner = Some(owner);
}

#[derive(Debug)]
/// 🎨️ Generation-owned pooled paint raster published after asynchronous image decoding.
pub struct DecodedMeshPaintImage {
    pub raster: SceneRasterLease,
}

/// 🖼️ Offers ONE reference underlay's decoded payload to the frame, KEEPING the state's own copy.
///
/// ⚖️ The copy stays for two reasons: a sibling surface of the same document holds its own
/// `World3dState` and must be able to offer the same image, and an offer can be LOST — a producer
/// rides the frame it was admitted into, and a frame whose present is ABORTED drops its staged
/// texture (`RasterTextureTable::abort_presented_step`). The re-offer on the next render is what
/// heals that, which is why this lane offers per render rather than once.
///
/// 🩸️ It briefly `mem::take`-d the payload, so the image crossed exactly once ever — and both panes
/// then painted no plan at all, because the one frame that carried it did not commit (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, W9b's `🗑️generated/w9b-final/final.png` against
/// `🗑️generated/w8b-boot-2/final.png`). What made the per-render offer dangerous was never the offer:
/// it was reporting a refused admission as a frame fault, which
/// [`World3dBuildContext::ensure_world_plane_texture`] no longer does.
///
/// ⏭️ The honest end state is an upload ACK from the raster table — the worker cannot see `live`, so
/// it cannot know the texture is resident and must keep offering. Until that exists, the table's own
/// key-dedupe absorbs the repeat.
pub fn world_raster_upload_in_pool(scene_state: &WorldSceneState, id: &str, gpu_key: &str, paint: bool, pool: &SceneRasterPool) -> Option<SceneRasterLease> {
    let registry = if paint { &scene_state.mesh_paint_textures } else { &scene_state.reference_pixels };
    let raster = registry.get(id)?;
    if pool.gpu_resident(gpu_key, raster.identity) {
        raster.pending.lock().ok()?.take();
        return None;
    }
    raster.pending.lock().ok()?.take().or_else(|| pool.acquire(raster.identity).ok())
}

pub fn world_raster_upload(scene_state: &WorldSceneState, id: &str, gpu_key: &str, paint: bool) -> Option<SceneRasterLease> {
    world_raster_upload_in_pool(scene_state, id, gpu_key, paint, &world_scene_raster_pool())
}

pub fn world_reference_raster_ready_in_pool(scene_state: &WorldSceneState, url: &str, pool: &SceneRasterPool) -> bool {
    let Some(raster) = scene_state.reference_pixels.get(url) else { return false };
    if raster.pending.lock().is_ok_and(|pending| pending.is_some()) {
        return true;
    }
    pool.cpu_resident(raster.identity) || pool.gpu_resident(url, raster.identity)
}

pub fn world_reference_raster_ready(scene_state: &WorldSceneState, url: &str) -> bool {
    world_reference_raster_ready_in_pool(scene_state, url, &world_scene_raster_pool())
}

pub fn reference_underlay_upload(scene_state: &WorldSceneState, url: &str) -> Option<SceneRasterLease> {
    world_raster_upload(scene_state, url, url, false)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dSnapshotApplyStep {
    Idle,
    Pending,
    Complete,
    Stale,
    Fault,
}

pub fn close_world3d_snapshot_apply_step(bridge_state: &mut WorldBridgeState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.should_yield() {
        return false;
    }
    let Some(cursor) = bridge_state.snapshot_apply.as_mut() else {
        return true;
    };
    let complete = cursor.close_step();
    context.consume_fuel(1);
    if complete {
        bridge_state.snapshot_apply = None;
    }
    complete
}

/// 🖍️ Resolves source targets per instance while retaining the ordinary active-object projection.
pub(crate) fn world_instance_selected_components(selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState,instance:&Instance3d,kind:&str)->HashSet<String> {
    let mut selected: HashSet<String> = gumball_state.gumball_selection_ids.iter().filter_map(|id|world_component_preview_target(scene_state,id,kind)).filter(|(object,_)|*object==instance.id).map(|(_,group)|group.to_string()).collect();
    if instance.component_source.is_none() && selection_state.active_object_id.as_deref()==Some(&instance.id) { selected.extend(selection_state.component_ids.iter().cloned()); }
    selected
}

pub fn reference_image_aspect(scene_state: &WorldSceneState, url: &str) -> f32 {
    scene_state
        .reference_pixels
        .get(url)
        .map_or(1.0, |raster| {
            let descriptor = raster.identity.descriptor();
            descriptor.width as f32 / descriptor.height.max(1) as f32
        })
        .max(0.01)
}

pub fn world_reference_plane_size(scene_state: &WorldSceneState, url: &str, width: f32) -> [f32; 2] {
    [width, width / reference_image_aspect(scene_state, url)]
}

pub struct WorldInteractionRegistryBuildCursor {
    pub revision: u64,
    initialized: bool,
    pub phase: WorldInteractionRegistryBuildPhase,
    pub draw: usize,
    pub instance: usize,
    pub mesh_probe: u16,
    pub vortex: usize,
    pub reference: usize,
    pub faulted: bool,
}

impl WorldInteractionRegistryBuildCursor {
    pub fn new(revision: u64) -> Self {
        Self { revision, initialized: false, phase: WorldInteractionRegistryBuildPhase::Instances, draw: 0, instance: 0, mesh_probe: 0, vortex: 0, reference: 0, faulted: false }
    }

    /// 🫥️ An EMPTY probe slot is "this mesh was never published", not a defect: the
    /// guest publishes a draw list and its mesh leases in separate deliveries, so a
    /// draw naming a glb whose lease has not landed is normal for the frames in
    /// between. Faulting here killed the whole page with `world3d retained
    /// interaction authority faulted` on the first pointer move after a mesh-bearing
    /// document mounted (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12c`).
    /// Skipping the draw is React's own behaviour — a `GlbInstanceMesh` whose loader
    /// has not resolved renders nothing and raycasts to nothing — and is the
    /// interaction twin of W11a's "a miss is a skipped draw, not a quarantine".
    pub fn step(&mut self, session_state: &mut WorldSessionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionStep {
        if context.should_yield() {
            return WorldInteractionStep::Pending;
        }
        if self.faulted || interaction_state.interaction_objects.faulted || interaction_state.interaction_meshes.faulted {
            return WorldInteractionStep::Fault;
        }
        if self.revision != session_state.interaction_revision {
            return WorldInteractionStep::Stale;
        }
        if !self.initialized {
            interaction_state.interaction_objects.begin_instance_order();
            self.initialized = true;
            context.consume_fuel(1);
            return WorldInteractionStep::Pending;
        }
        match self.phase {
            WorldInteractionRegistryBuildPhase::Instances => {
                let Some(draw) = scene_state.draws.get(self.draw) else {
                    self.phase = WorldInteractionRegistryBuildPhase::Vortices;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                if usize::from(self.mesh_probe) == WORLD_INTERACTION_MESH_CAPACITY || draw.mesh_key.len() > WORLD_INTERACTION_ID_BYTE_CAPACITY {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                let mesh_slot = (WorldInteractionMeshRegistry::hash(&draw.mesh_key) + usize::from(self.mesh_probe)) % WORLD_INTERACTION_MESH_CAPACITY;
                let mesh = match interaction_state.interaction_meshes.slots[mesh_slot] {
                    Some(entry) if entry.id.as_str() == draw.mesh_key && entry.version == live_world3d_mesh_version(scene_state, &draw.mesh_key) => Some(WorldInteractionMeshToken { slot: mesh_slot as u16, generation: entry.generation }),
                    Some(_) => {
                        self.mesh_probe += 1;
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    }
                    None => {
                        self.draw += 1;
                        self.instance = 0;
                        self.mesh_probe = 0;
                        context.consume_fuel(1);
                        return WorldInteractionStep::Pending;
                    }
                };
                let Some(instance) = draw.instances.get(self.instance) else {
                    self.draw += 1;
                    self.instance = 0;
                    self.mesh_probe = 0;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let translation = instance.model.cols[3];
                let Some(token) = interaction_state.interaction_objects.admit(
                    self.revision,
                    WorldInteractionObjectKind::Instance,
                    &instance.id,
                    mesh,
                    instance.model,
                    [self.draw as f32, self.instance as f32, if instance.selected { 1.0 } else { 0.0 }, translation[0], translation[1], translation[2], 0.0, 0.0], instance.component_source,
                ) else {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                };
                if !interaction_state.interaction_objects.push_instance_order(token) {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                self.instance += 1;
            }
            WorldInteractionRegistryBuildPhase::Vortices => {
                let Some(vortex) = scene_state.vortices.get(self.vortex) else {
                    self.phase = WorldInteractionRegistryBuildPhase::References;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
                let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
                let values = [position[0] as f32, position[1] as f32, position[2] as f32, direction[0] as f32, direction[1] as f32, direction[2] as f32, vortex.radius.unwrap_or(0.36) as f32, 0.0];
                if interaction_state.interaction_objects.admit(self.revision, WorldInteractionObjectKind::Vortex, &vortex.full_id, None, Mat4::identity(), values, None).is_none() {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
                self.vortex += 1;
            }
            WorldInteractionRegistryBuildPhase::References => {
                let Some(reference) = scene_state.references.get(self.reference) else {
                    self.phase = WorldInteractionRegistryBuildPhase::Complete;
                    interaction_state.interaction_objects.revision = self.revision;
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                self.reference += 1;
                if reference.hidden.unwrap_or(false) || reference.locked.unwrap_or(false) {
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                }
                let Some(url) = reference.url.as_deref() else {
                    context.consume_fuel(1);
                    return WorldInteractionStep::Pending;
                };
                let origin = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
                let width = world_reference_width(reference);
                let aspect = reference_image_aspect(scene_state, url);
                let values = [origin[0] as f32, origin[1] as f32, origin[2] as f32, width, width / aspect, 0.0, 0.0, 0.0];
                let id = reference.id.as_deref().unwrap_or(url);
                if interaction_state.interaction_objects.admit(self.revision, WorldInteractionObjectKind::Reference, id, None, Mat4::identity(), values, None).is_none() {
                    self.faulted = true;
                    return WorldInteractionStep::Fault;
                }
            }
            WorldInteractionRegistryBuildPhase::Complete => return WorldInteractionStep::Complete,
        }
        context.consume_fuel(1);
        WorldInteractionStep::Pending
    }

    pub fn close_step(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> bool {
        if context.should_yield() {
            return false;
        }
        self.phase = WorldInteractionRegistryBuildPhase::Complete;
        context.consume_fuel(1);
        true
    }
}

#[expect(clippy::large_enum_variant, reason = "Interaction transitions retain fixed action claims and cursor storage without a fresh allocation between admitted steps.")]
pub enum WorldInteractionActive {
    Plan { plan: WorldInteractionPlan, retirement: Option<WorldInteractionAuthorityStep> },
    Pick { cursor: WorldRayPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    ObjectPick { cursor: WorldObjectPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    ComponentPick { cursor: WorldComponentPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    MarqueePick { cursor: WorldMarqueePickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    MarqueePublish { job: WorldMarqueePublishJob, retirement: Option<WorldInteractionAuthorityStep> },
    ComponentMarqueePublish { job: WorldComponentMarqueePublishJob, retirement: Option<WorldInteractionAuthorityStep> },
    GumballPick { cursor: WorldGumballPickCursor, retirement: Option<WorldInteractionAuthorityStep> },
    GumballCommit { job: WorldGumballCommitJob, retirement: Option<WorldInteractionAuthorityStep> },
    BrushCommit { job: WorldBrushCommitJob, retirement: Option<WorldInteractionAuthorityStep> },
}

/// 📏️ React's `WORLD_NAVIGATION_THRESHOLDS_DEFAULT`, verbatim.
pub const WORLD_NAVIGATION_PAN_RATIO: f32 = 0.02;

pub const WORLD_NAVIGATION_ZOOM_RATIO: f32 = 0.03;

pub const WORLD_NAVIGATION_ORBIT_RADIANS: f32 = 0.05;

/// 🧭️ The three gestures a camera movement may have performed, in React's own push order — `pan`,
/// `zoom`, `orbit` — so the `gestures` array this lane publishes is byte-identical to the DOM host's.
pub const WORLD_NAVIGATION_GESTURE_IDS: [&str; 3] = ["pan", "zoom", "orbit"];

fn world_navigation_distance(left: [f32; 3], right: [f32; 3]) -> f32 {
    ((left[0] - right[0]).powi(2) + (left[1] - right[1]).powi(2) + (left[2] - right[2]).powi(2)).sqrt()
}

fn world_navigation_direction(from: [f32; 3], to: [f32; 3]) -> [f32; 3] {
    let delta = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    let length = (delta[0].powi(2) + delta[1].powi(2) + delta[2].powi(2)).sqrt();
    if length > 1e-9 {
        [delta[0] / length, delta[1] / length, delta[2] / length]
    } else {
        [0.0, 0.0, 0.0]
    }
}

/// 📸️ This pane's live camera as the snapshot a navigation diff measures.
pub fn world_navigation_snapshot(camera_state: &WorldCameraState) -> WorldNavigationSnapshot {
    let camera = camera_state.orbit.to_camera();
    WorldNavigationSnapshot {
        position: [camera.position.x, camera.position.y, camera.position.z],
        target: [camera.target.x, camera.target.y, camera.target.z],
        zoom: if camera.projection.is_parallel() { camera.zoom } else { 1.0 },
        parallel: camera.projection.is_parallel(),
    }
}

/// 🧭️ Which navigation gestures a camera movement performed, as a `[pan, zoom, orbit]` mask — the
/// port of React's `classifyWorldNavigationGestures`: pan when the orbit TARGET moved, zoom when the
/// parallel zoom factor or the perspective dolly distance changed, orbit when the eye's direction
/// around the target rotated. One drag may answer more than one, so this is a set, not a verdict.
pub fn classify_world_navigation_gestures(before: WorldNavigationSnapshot, after: WorldNavigationSnapshot) -> [bool; 3] {
    let reference = world_navigation_distance(before.position, before.target).max(1.0);
    let pan = world_navigation_distance(before.target, after.target) > WORLD_NAVIGATION_PAN_RATIO * reference;
    let zoom_delta = if before.parallel { (after.zoom / if before.zoom == 0.0 { 1.0 } else { before.zoom } - 1.0).abs() } else { (world_navigation_distance(after.position, after.target) / reference - 1.0).abs() };
    let zoom = zoom_delta > WORLD_NAVIGATION_ZOOM_RATIO;
    let before_direction = world_navigation_direction(before.target, before.position);
    let after_direction = world_navigation_direction(after.target, after.position);
    let dot = (before_direction[0] * after_direction[0] + before_direction[1] * after_direction[1] + before_direction[2] * after_direction[2]).clamp(-1.0, 1.0);
    [pan, zoom, dot.acos() > WORLD_NAVIGATION_ORBIT_RADIANS]
}
//#endregion 🧭️WorldCameraSync

//#region 🧹️World3dDynamicRetirement

pub struct World3dDynamicRetirement {
    pub phase: u8,
    pub blocked: Option<WorldDynamicFault>,
}

impl World3dDynamicRetirement {
    pub fn begin(camera_state: &mut WorldCameraState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState) -> Self {
        scene_state.meshes.begin_close();
        scene_state.mesh_versions.begin_close();
        scene_state.mesh_appearances.begin_close();
        scene_state.draws.begin_close();
        scene_state.reference_pixels.begin_close();
        scene_state.mesh_paint_textures.begin_close();
        assets_state.asset_io.begin_close();
        camera_state.fit_bounds_cursor = None;
        Self { phase: 0, blocked: None }
    }

    pub fn step(&mut self, selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, bridge_state: &mut WorldBridgeState, context:&mut semio_framework_job::StepContext<'_>) -> bool {
        self.blocked = None;
        if scene_state.native_mesh_admission.is_some(){if let Ok((_,progress))=world3d_close_native_mesh_admission_step(scene_state,context.retained_grant()){if context.consume_retained(progress).is_ok()&&progress.copied_items!=0{context.consume_fuel(1);}}return false}
        if step_world3d_scene_bridge_close(bridge_state,context) {
            return false;
        }
        if scene_state.dynamic_mesh_close.is_none() {
            if let Some(entry) = scene_state.dynamic_blocked_mesh.take() {
                scene_state.dynamic_mesh_close = Some(entry);
                return false;
            }
        }
        if let Some(owner) = scene_state.dynamic_blocked_owner.take() {
            if let Err(owner) = quarantine_world_owner(owner) {
                scene_state.dynamic_blocked_owner = Some(owner);
                self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
            }
            return false;
        }
        match self.phase {
            0 => {
                if scene_state.dynamic_mesh_close.is_some() {
                    step_world3d_mesh_close(scene_state,context);
                    return false;
                }
                let Some(entry) = scene_state.meshes.take_one() else {
                    self.phase = 1;
                    return false;
                };
                scene_state.dynamic_mesh_close = Some(entry);
            }
            1 => {
                if scene_state.mesh_versions.take_one().is_some() {
                    return false;
                }
                self.phase = 2;
                return false;
            }
            2 => {
                if scene_state.mesh_appearances.take_one().is_some() {
                    return false;
                }
                self.phase = 3;
                return false;
            }
            3 => {
                let Some(entry) = scene_state.draws.take_last() else {
                    self.phase = 4;
                    return false;
                };
                if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                    let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("draw quarantine owner") };
                    scene_state.draws.restore_last(entry);
                    self.blocked = Some(WorldDynamicFault::QuarantineCapacity);
                }
            }
            4 => {
                let Some(entry) = scene_state.reference_pixels.take_one() else {
                    self.phase = 5;
                    return false;
                };
                drop(entry);
            }
            5 => {
                let Some(entry) = scene_state.mesh_paint_textures.take_one() else {
                    self.phase = 6;
                    return false;
                };
                drop(entry);
            }
            6 => {
                let Some(id) = interaction_state.instance_interaction_ids.keys().next().cloned() else {
                    self.phase = 7;
                    return false;
                };
                interaction_state.instance_interaction_ids.remove(&id);
            }
            7 => {
                let Some(id) = interaction_state.instance_interaction_granularity_ids.keys().next().cloned() else {
                    self.phase = 8;
                    return false;
                };
                interaction_state.instance_interaction_granularity_ids.remove(&id);
            }
            8 => {
                if selection_state.local_hover_granularity_id.take().is_some() {
                    return false;
                }
                self.phase = 9;
                return false;
            }
            _ => return true,
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.phase >= 9 && self.blocked.is_none()
    }
}

pub fn begin_world3d_dynamic_retirement(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, assets_state: &mut WorldAssetsState) -> bool {
    if retirement_state.dynamic_retirement.is_some() {
        return false;
    }
    session_state.snapshot_fault = Some(World3dSnapshotFault::Closing);
    retirement_state.dynamic_retirement = Some(World3dDynamicRetirement::begin(camera_state, scene_state, assets_state));
    true
}

//#endregion 🧹️World3dDynamicRetirement

//#region 🧱️WorldDrawRebuild
pub fn begin_world3d_draw_rebuild(session_state: &mut WorldSessionState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, descriptor: WorldDrawRebuildDescriptor) -> Result<(), WorldDynamicFault> {
    if retirement_state.dynamic_retirement.is_some() || scene_state.draw_rebuild.is_some() || scene_state.retired_draws.is_some() || scene_state.dynamic_blocked_owner.is_some() {
        return Err(WorldDynamicFault::Closing);
    }
    if descriptor.revision != session_state.interaction_revision || descriptor.generation != scene_state.draw_generation.wrapping_add(1) {
        return Err(WorldDynamicFault::StaleToken);
    }
    scene_state.draw_rebuild = Some(WorldDrawRebuildCursor::new(descriptor)?);
    Ok(())
}

pub fn world3d_draw_rebuild_admit_draw(scene_state: &mut WorldSceneState, mesh_key: &str, mesh_version: u64, instance_count: u16) -> Result<(), WorldDynamicFault> {
    let Some(cursor) = scene_state.draw_rebuild.as_mut() else {
        return Err(WorldDynamicFault::StaleToken);
    };
    cursor.admit_draw(mesh_key, mesh_version, instance_count)
}

pub fn world3d_draw_rebuild_admit_instance(scene_state: &mut WorldSceneState, draw: u16, id: &str, model: Mat4, color: [f32; 4], selected: bool, hovered: bool) -> Result<(), WorldDynamicFault> {
    world3d_draw_rebuild_admit_retained_instance(scene_state, draw, id, model, color, SceneColorSource3d::Authored, selected, hovered, None)
}

pub fn world3d_draw_rebuild_admit_retained_instance(
    scene_state: &mut WorldSceneState,
    draw: u16,
    id: &str,
    model: Mat4,
    color: [f32; 4],
    color_source: SceneColorSource3d,
    selected: bool,
    hovered: bool,
    component_source: Option<ComponentSource3d>,
) -> Result<(), WorldDynamicFault> {
    let Some(cursor) = scene_state.draw_rebuild.as_mut() else {
        return Err(WorldDynamicFault::StaleToken);
    };
    cursor.admit_instance(draw, id, model, color, color_source, selected, hovered, component_source)
}

pub fn world3d_draw_rebuild_seal(scene_state: &mut WorldSceneState) -> Result<(), WorldDynamicFault> {
    scene_state.draw_rebuild.as_mut().ok_or(WorldDynamicFault::StaleToken)?.seal()
}

pub fn close_world3d_draw_rebuild_step(scene_state: &mut WorldSceneState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.retirement_should_yield() {
        return false;
    }
    if let Some(retired) = scene_state.retired_draws.as_mut() {
        if let Some(entry) = retired.take_last() {
            if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
                let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("retired draw owner") };
                retired.restore_last(entry);
                return false;
            }
            context.consume_fuel(1);
            return false;
        }
        scene_state.retired_draws = None;
        context.consume_fuel(1);
        return false;
    }
    let Some(cursor) = scene_state.draw_rebuild.as_mut() else {
        return true;
    };
    if let Some(output) = cursor.output.as_mut() {
        if output.instances.pop().is_some() {
            context.consume_fuel(1);
            return false;
        }
        cursor.output = None;
        context.consume_fuel(1);
        return false;
    }
    if let Some(draft) = cursor.drafts.iter_mut().flatten().find(|draft| !draft.terminal_is_empty()) {
        drop(draft.take_next());
        context.consume_fuel(1);
        return false;
    }
    if let Some(slot) = cursor.drafts.iter().position(Option::is_some) {
        let draft = cursor.drafts[slot].take().expect("terminal draft shell");
        assert!(draft.terminal_is_empty());
        drop(draft);
        context.consume_fuel(1);
        return false;
    }
    if let Some(entry) = cursor.staged.take_last() {
        if let Err(owner) = quarantine_world_owner(WorldOpaqueOwner::Draw(entry)) {
            let WorldOpaqueOwner::Draw(entry) = owner else { unreachable!("staged draw owner") };
            cursor.staged.restore_last(entry);
            return false;
        }
        context.consume_fuel(1);
        return false;
    }
    assert!(cursor.terminal_is_empty());
    scene_state.draw_rebuild = None;
    context.consume_fuel(1);
    true
}

pub fn world3d_draw_rebuild_terminal_is_empty(scene_state: &WorldSceneState) -> bool {
    scene_state.draw_rebuild.is_none() && scene_state.retired_draws.is_none()
}
//#endregion 🧱️WorldDrawRebuild
