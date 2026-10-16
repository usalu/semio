//! 🧩️ World3d state components: the per-concern state structs the host composes, with the types each one owns.
//!
//! Crate `semio-framework-os-infinite-world-components`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_framework_os_infinite_world_model::{default_distance_reference, default_grid_factor, default_manual_lod, RefCountPool, ROTATE_SELECTION_ACTION_ID, SCALE_SELECTION_ACTION_ID, tool_run_trace, TRANSLATE_SELECTION_ACTION_ID, World3dMeshAppearance, World3dPrimitiveMaterial, World3dScalarFieldStatus, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_DYNAMIC_MESH_CAPACITY, WORLD_DYNAMIC_PIXEL_CAPACITY, WORLD_INTERACTION_ID_BYTE_CAPACITY, world_mesh_ownership_complete, world_mesh_ownership_grant, world_scene_raster_pool, WorldAttractionRecord, WorldBrushPreviewRecord, WorldCatalogueDropPreviewRecord, WorldChunkingRecord, WorldDrawRebuildCursor, WorldDrawRegistry, WorldDynamicEntry, WorldDynamicFault, WorldDynamicRegistry, WorldEngagementPreviewRecord, WorldEnvironmentRecord, WorldInteractionObjectSlot, WorldInteractionState, WorldLodRecord, WorldMeshRegistry, WorldMeshSource, WorldNativeMeshAdmission, WorldOpaqueOwner, WorldPickTargetRecord, WorldReferenceRecord, WorldSceneRaster, WorldSelectionTargets, WorldSessionState, WorldSubElementPaint, WorldTargetVolumeRecord, WorldTutorialGeometry, WorldVortexRecord};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_model::WorldMeshLodEntry;
use ui_wgpu::wgpu::{ComponentSource3d, mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_seal, mesh3d_terminal_is_empty, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, OrbitController, SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterProfile, SceneRasterWriteMode, Vec3, World3dSnapshotDrawPermit, World3dSnapshotLease};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{SceneAuthoredMaterial3d, SceneMaterialAlpha3d};
use semio_framework_ui_viewport::{Viewport3dProjectionFramePolicy, Viewport3dProjectionSpec};

/// 👻️ Projects one tutorial scene point through the live world camera into surface-local pixels.
pub fn world3d_tutorial_scene_point(session_state: &WorldSessionState, camera_state: &WorldCameraState, position: [f64; 3]) -> Option<[f32; 2]> {
    let camera = camera_state.orbit.to_camera();
    let view_projection = ui_wgpu::wgpu::projection_spec_view_proj(&camera, camera_state.projection_spec, session_state.bounds.w.max(1.0), session_state.bounds.h.max(1.0));
    ui_wgpu::wgpu::projection_spec_project_point(view_projection, camera_state.projection_spec, Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32), session_state.bounds.w.max(1.0), session_state.bounds.h.max(1.0))
}

/// 👻️ Resolves the world host's semantic pick vocabulary from its retained live scene geometry.
pub fn world3d_tutorial_entity_geometry(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, domain: &str, entity: &str) -> Option<WorldTutorialGeometry> {
    let center = [session_state.bounds.w * 0.5, session_state.bounds.h * 0.5];
    let nearest = |points: Vec<[f64; 3]>| {
        points
            .into_iter()
            .filter_map(|position| world3d_tutorial_scene_point(session_state, camera_state, position).map(|point| (point, (point[0] - center[0]).hypot(point[1] - center[1]))))
            .min_by(|left, right| left.1.total_cmp(&right.1))
            .map(|(point, _)| WorldTutorialGeometry { point, polyline: None })
    };
    match domain {
        "vortex" => {
            if entity == "*" {
                return nearest(scene_state.vortices.iter().filter_map(|vortex| vortex.position).collect());
            }
            let position = scene_state.vortices.iter().find(|vortex| vortex.full_id == entity)?.position?;
            world3d_tutorial_scene_point(session_state, camera_state, position).map(|point| WorldTutorialGeometry { point, polyline: None })
        }
        "object" => {
            if entity == "*" {
                return nearest(scene_state.instance_positions.values().copied().collect());
            }
            let position = *scene_state.instance_positions.get(entity)?;
            world3d_tutorial_scene_point(session_state, camera_state, position).map(|point| WorldTutorialGeometry { point, polyline: None })
        }
        "attraction" => {
            let attraction = if entity == "*" { scene_state.attractions.iter().find(|attraction| attraction.from.is_some() && attraction.to.is_some()) } else { scene_state.attractions.iter().find(|attraction| attraction.id.as_deref() == Some(entity)) }?;
            let from = world3d_tutorial_scene_point(session_state, camera_state, attraction.from?)?;
            let to = world3d_tutorial_scene_point(session_state, camera_state, attraction.to?)?;
            Some(WorldTutorialGeometry { point: [(from[0] + to[0]) * 0.5, (from[1] + to[1]) * 0.5], polyline: Some(vec![from, to]) })
        }
        _ => None,
    }
}

/// 🧩️ World3d camera component: orbit rig, projection, camera fit and camera sync.
pub struct WorldCameraState {

    pub orbit: OrbitController,

    pub resolved_lod_pick: Option<f64>,

    pub gizmo_hovered_tip: Option<usize>,

    pub scene_camera_digest: Option<u64>,

    /// 🎯️ React's retained `WorldAutoFit` request, ownership key, applied key, and bounded live-geometry
    /// measurement. A missing published extent stays owed until rendered mesh bounds exist.
    pub fit_request: Option<World3dCameraFitRequest>,

    pub fit_owner: Option<World3dCameraFitOwner>,

    pub fit_applied_key: Option<u64>,

    pub fit_bounds_cursor: Option<World3dCameraBoundsCursor>,

    /// 📷️ The content extent consumed by this pane's last explicit projection-frame debt.
    pub projection_frame_key: Option<u64>,

    /// 📷️ The zoom produced by that accepted one-shot frame, retained for diagnostics.
    pub projection_frame_zoom: Option<f32>,

    /// 📐️ The accepted renderer-neutral projection mode × orientation for this pane.
    pub projection_spec: Viewport3dProjectionSpec,

    /// 📐️ This pane holds its OWN projection spec — React's `externalPendingProjectionSpec`, which
    /// the delivered `cameraState.projectionSpec` never overrides while it is set.
    pub projection_selected: bool,

    /// 📐️ A local selection owes exactly ONE content framing, and it is immune to
    /// `camera_user_moved`: the press is not a gesture the latch exists to protect.
    pub projection_frame_owed: bool,

    /// 📷️ The delivered camera decides whether scene content or its authored pose owns framing.
    pub projection_frame_policy: Viewport3dProjectionFramePolicy,

    /// 🔒️ The user has moved this camera since the live fit revision arrived — the latch that keeps a
    /// re-evaluation of the SAME document from yanking the view back
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️boot-camera-framing-2026-09-15.md`).
    pub camera_user_moved: bool,

    /// 🧭️ This pane's camera-report debt — see [`WorldCameraSync`].
    pub camera_sync: WorldCameraSync,
}

/// 🧩️ World3d selection component: hover, selection, marquee and pick-target state.
pub struct WorldSelectionState {

    pub selection_method: String,

    pub local_hover_id: Option<String>,

    pub local_hover_granularity_id: Option<String>,

    pub marquee_points: Vec<[f32; 2]>,

    pub marquee_active: bool,

    /// 🧲️ The bounded sub-object pick targets of `pick_targets_json`, retained so a pointer move or
    /// a click can hit-test them without re-parsing the lane.
    pub pick_targets: Vec<WorldPickTargetRecord>,

    /// 🧲️ The `kind:id` of the pick target this host last published an `interactionHover` for —
    /// the sub-object twin of `local_hover_id`, deduped through [`world_pick_keys_match`] so an
    /// alias spelling never re-publishes the same hover.
    pub pick_hover_key: Option<String>,

    pub hovered_vortex_id: Option<String>,

    #[cfg(any(test, feature = "testkit"))]
    pub drag_object_id: Option<String>,

    #[cfg(any(test, feature = "testkit"))]
    pub drag_object_z: f32,

    #[cfg(any(test, feature = "testkit"))]
    pub drag_last_position: Option<[f32; 3]>,

    pub selected_ids: Vec<String>,

    pub granularity: String,

    pub component_ids: Vec<String>,

    pub interaction_mode: String,

    pub marquee_preview_ids: Vec<String>,

    pub paint_stroke_active: bool,

    pub hovered_component_id: Option<String>,

    pub hovered_component_object_id: Option<String>,

    pub hovered_component_mode: Option<String>,

    pub show_edges: bool,

    pub selection_targets: WorldSelectionTargets,

    pub active_object_id: Option<String>,

    #[cfg(any(test, feature = "testkit"))]
    pub press_object_id: Option<String>,

    #[cfg(any(test, feature = "testkit"))]
    pub right_press_point: Option<[f32; 2]>,

    pub scene_selection_digest: Option<u64>,

    /// 🟩️ Instance ids a running tool placed provisionally; painted with the `provisional` style token.
    /// Filled from the `provisional: true` flag the scene producer stamps on `instances_json` records.
    pub provisional_instance_ids: HashSet<String>,

    /// 🎨️ The instances whose wire rows carry `highlighted`/`disabled`/`celebrating` — the three
    /// `MESH_STYLE_PAINT` rows that had a table entry and NO producer before ticket 26/09/17 packet
    /// W14g (`📓️w9b-projection-pane-framing-grid-materials.md` §7.3).
    pub highlighted_instance_ids: HashSet<String>,

    pub disabled_instance_ids: HashSet<String>,

    pub celebrating_instance_ids: HashSet<String>,
}

/// 🧩️ World3d gumball component: gumball, brush preview, catalogue drop preview and relocate drag.
pub struct WorldGumballState {

    pub brush_preview: Option<WorldBrushPreviewRecord>,

    pub catalogue_drop_preview: Option<WorldCatalogueDropPreviewRecord>,

    /// 🚚️ The live relocate drag, or `None`. Armed by a primary press under the `worldRelocate`
    /// utility, previewed as a catalogue-drop ghost on every move, and closed by the release (commit)
    /// or by Escape (abort) — React's `relocateSessionRef` (`🌐️World3dHost/🟦️.tsx:6845`).
    pub relocate: Option<World3dRelocateSession>,

    /// 🎯️ The Guest Scene's exact source-bound component interaction targets.
    pub gumball_selection_ids: Vec<String>,

    pub transform_mode: String,

    /// 🎛️ `selection_json.gumballConfig` when the producer authored one — see [`world3d_gumball_config`].
    pub gumball_config: Option<World3dSceneGumballConfigRecord>,

    /// ⚡️ `selection_json.gumballLiveDispatch`: the guest owns the mid-drag pose — a gesture streams its ticks and the host
    /// draws the guest's answer instead of a local preview (`🌐️World3dHost` `worldGumballStep`, live branch).
    pub gumball_live_dispatch: bool,

    #[cfg(any(test, feature = "testkit"))]
    pub gumball_handle: Option<GumballHandle>,

    #[cfg(any(test, feature = "testkit"))]
    pub gumball_pivot: Vec3,

    #[cfg(any(test, feature = "testkit"))]
    pub gumball_drag_anchor: f32,

    #[cfg(any(test, feature = "testkit"))]
    pub gumball_drag_start_vec: Vec3,

    #[cfg(any(test, feature = "testkit"))]
    pub gumball_preview_translate: Vec3,

    #[cfg(any(test, feature = "testkit"))]
    pub gumball_preview_angle: f32,

    #[cfg(any(test, feature = "testkit"))]
    pub gumball_preview_scale: Vec3,

    pub gumball_target: Option<[f32; 3]>,
}

/// 🧩️ World3d scene component: resident meshes, draws, scene records (vortices, references, lod, environment), placeholder meshes and dynamic retirement slots.
pub struct WorldSceneState {

    pub meshes: WorldMeshRegistry,

    pub mesh_versions: WorldDynamicRegistry<u64, WORLD_DYNAMIC_MESH_CAPACITY>,

    pub mesh_appearances: WorldDynamicRegistry<World3dMeshAppearance, WORLD_DYNAMIC_MESH_CAPACITY>,

    pub draws: WorldDrawRegistry,

    pub vortices: Vec<WorldVortexRecord>,

    pub attractions: Vec<WorldAttractionRecord>,

    pub target_volumes: Vec<WorldTargetVolumeRecord>,

    pub references: Vec<WorldReferenceRecord>,

    pub engagement_preview: Vec<WorldEngagementPreviewRecord>,

    pub reference_pixels: WorldDynamicRegistry<WorldSceneRaster, WORLD_DYNAMIC_PIXEL_CAPACITY>,

    pub mesh_paint_textures: WorldDynamicRegistry<WorldSceneRaster, WORLD_DYNAMIC_PIXEL_CAPACITY>,

    pub lod: WorldLodRecord,

    pub chunking: Option<WorldChunkingRecord>,

    pub visible_chunks: HashSet<(i64, i64, i64)>,

    #[cfg(any(test, feature = "testkit"))]
    pub mesh_lod_catalog: HashMap<String, Vec<WorldMeshLodEntry>>,

    #[cfg(any(test, feature = "testkit"))]
    pub mesh_url_fallback: HashMap<String, String>,

    pub instance_positions: HashMap<String, [f64; 3]>,

    pub mesh_pool: RefCountPool<String>,

    pub mesh_source_urls: HashMap<String, String>,

    pub environment: WorldEnvironmentRecord,

    pub scene_mesh_digests: HashMap<String, u64>,

    pub dynamic_blocked_owner: Option<WorldOpaqueOwner>,

    pub dynamic_mesh_close: Option<WorldDynamicEntry<WorldMeshSource>>,

    pub dynamic_blocked_mesh: Option<WorldDynamicEntry<WorldMeshSource>>,

    pub native_mesh_admission: Option<WorldNativeMeshAdmission>,

    pub placeholder_generation: u64,

    pub placeholder_build: Option<WorldPlaceholderMeshCursor>,

    pub dynamic_fault: Option<WorldDynamicFault>,

    pub draw_generation: u64,

    pub geometry_generation: u64,

    pub draw_rebuild: Option<WorldDrawRebuildCursor>,

    pub retired_draws: Option<WorldDrawRegistry>,
}

/// 🧩️ World3d overlay component: face overlay build, sub-element paint, scalar field and prepared status.
pub struct WorldOverlayState {

    pub prepared_status: [Option<World3dPreparedStatus>; 2],

    pub face_overlay_build: Option<WorldFaceOverlayMeshCursor>,

    pub face_overlay_generation: Option<u64>,

    pub face_overlay_retired_generation: Option<u64>,

    pub face_overlay_colors: [Option<[f32; 4]>; 3],

    pub scalar_field_status: World3dScalarFieldStatus,

    pub sub_element_paint: WorldSubElementPaint,

    pub face_overlay_applied_revision: u64,

    pub face_overlay_applied_draw_generation: u64,
}

/// 🧩️ World3d trace component: tool run trace layer.
pub struct WorldTraceState {

    /// ⏯️ The resident tool run trace fed by `World3dScene.tool_run_trace` — see [`tool_run_trace`].
    pub tool_run_trace: tool_run_trace::ToolRunTraceLayer,

    /// 👁️ The trace legend toggles this window's config exposes.
    pub tool_run_trace_visibility: tool_run_trace::ToolRunTraceVisibility,

    pub tool_run_trace_meshes_digest: Option<u64>,

    pub tool_run_trace_mesh_keys: Vec<String>,

    /// 🪟️ The window instance whose view state echoes this layer's trace cursor as
    /// `toolRunTraceCursorByWindowId[window]`; set by the host that paints the surface into that window.
    pub tool_run_trace_window_id: Option<String>,
}

impl WorldCameraState {
    pub fn new() -> Self {
        Self {
            orbit: OrbitController::default(),
            resolved_lod_pick: None,
            gizmo_hovered_tip: None,
            scene_camera_digest: None,
            fit_request: None,
            fit_owner: None,
            fit_applied_key: None,
            fit_bounds_cursor: None,
            projection_frame_key: None,
            projection_frame_zoom: None,
            projection_spec: ui_wgpu::wgpu::default_projection_spec(),
            projection_selected: false,
            projection_frame_owed: false,
            projection_frame_policy: Viewport3dProjectionFramePolicy::Content,
            camera_user_moved: false,
            camera_sync: WorldCameraSync::default(),
        }
    }
}

impl WorldSelectionState {
    pub fn new() -> Self {
        Self {
            selection_method: "rectangle".into(),
            local_hover_id: None,
            local_hover_granularity_id: None,
            marquee_points: Vec::new(),
            marquee_active: false,
            pick_targets: Vec::new(),
            pick_hover_key: None,
            hovered_vortex_id: None,
            #[cfg(any(test, feature = "testkit"))]
                        drag_object_id: None,
            #[cfg(any(test, feature = "testkit"))]
                        drag_object_z: 0.0,
            #[cfg(any(test, feature = "testkit"))]
                        drag_last_position: None,
            selected_ids: Vec::new(),
            granularity: "object".into(),
            component_ids: Vec::new(),
            interaction_mode: "model".into(),
            marquee_preview_ids: Vec::new(),
            paint_stroke_active: false,
            hovered_component_id: None,
            hovered_component_object_id: None,
            hovered_component_mode: None,
            show_edges: true,
            selection_targets: WorldSelectionTargets::default(),
            active_object_id: None,
            #[cfg(any(test, feature = "testkit"))]
                        press_object_id: None,
            #[cfg(any(test, feature = "testkit"))]
                        right_press_point: None,
            scene_selection_digest: None,
            provisional_instance_ids: HashSet::new(),
            highlighted_instance_ids: HashSet::new(),
            disabled_instance_ids: HashSet::new(),
            celebrating_instance_ids: HashSet::new(),
        }
    }
}

impl WorldGumballState {
    pub fn new() -> Self {
        Self {
            brush_preview: None,
            catalogue_drop_preview: None,
            relocate: None,
            gumball_selection_ids: Vec::new(),
            transform_mode: "translate".into(),
            gumball_config: None,
            gumball_live_dispatch: false,
            #[cfg(any(test, feature = "testkit"))]
                        gumball_handle: None,
            #[cfg(any(test, feature = "testkit"))]
                        gumball_pivot: Vec3::ZERO,
            #[cfg(any(test, feature = "testkit"))]
                        gumball_drag_anchor: 0.0,
            #[cfg(any(test, feature = "testkit"))]
                        gumball_drag_start_vec: Vec3::ZERO,
            #[cfg(any(test, feature = "testkit"))]
                        gumball_preview_translate: Vec3::ZERO,
            #[cfg(any(test, feature = "testkit"))]
                        gumball_preview_angle: 0.0,
            #[cfg(any(test, feature = "testkit"))]
                        gumball_preview_scale: Vec3::new(1.0, 1.0, 1.0),
            gumball_target: None,
        }
    }
}

impl WorldSceneState {
    pub fn new() -> Self {
        Self {
            meshes: WorldMeshRegistry::default(),
            mesh_versions: WorldDynamicRegistry::default(),
            mesh_appearances: WorldDynamicRegistry::default(),
            draws: WorldDrawRegistry::default(),
            vortices: Vec::new(),
            attractions: Vec::new(),
            target_volumes: Vec::new(),
            references: Vec::new(),
            engagement_preview: Vec::new(),
            reference_pixels: WorldDynamicRegistry::default(),
            mesh_paint_textures: WorldDynamicRegistry::default(),
            lod: WorldLodRecord {
                automatic: true,
                manual: default_manual_lod(),
                distance_reference: default_distance_reference(),
                depth_variable: false,
                grid_factor: default_grid_factor(),
                grid_snap_enabled: false,
                show_grid: true,
                grid_datum: Some([0.0, 0.0, 0.0]),
            },
            chunking: None,
            visible_chunks: HashSet::new(),
            #[cfg(any(test, feature = "testkit"))]
                        mesh_lod_catalog: HashMap::new(),
            #[cfg(any(test, feature = "testkit"))]
                        mesh_url_fallback: HashMap::new(),
            instance_positions: HashMap::new(),
            mesh_pool: RefCountPool::new(),
            mesh_source_urls: HashMap::new(),
            environment: WorldEnvironmentRecord::default(),
            scene_mesh_digests: HashMap::new(),
            dynamic_blocked_owner: None,
            dynamic_mesh_close: None,
            native_mesh_admission: None,
            dynamic_blocked_mesh: None,
            placeholder_generation: 0,
            placeholder_build: None,
            dynamic_fault: None,
            draw_generation: 0,
            geometry_generation: 0,
            draw_rebuild: None,
            retired_draws: None,
        }
    }
}

impl WorldOverlayState {
    pub fn new() -> Self {
        Self {
            prepared_status: [None; 2],
            face_overlay_build: None,
            face_overlay_generation: None,
            face_overlay_retired_generation: None,
            face_overlay_colors: [None; 3],
            scalar_field_status: World3dScalarFieldStatus::None,
            sub_element_paint: WorldSubElementPaint::default(),
            face_overlay_applied_revision: u64::MAX,
            face_overlay_applied_draw_generation: u64::MAX,
        }
    }
}

impl WorldTraceState {
    pub fn new() -> Self {
        Self {
            tool_run_trace: tool_run_trace::ToolRunTraceLayer::default(),
            tool_run_trace_visibility: tool_run_trace::ToolRunTraceVisibility::default(),
            tool_run_trace_meshes_digest: None,
            tool_run_trace_mesh_keys: Vec::new(),
            tool_run_trace_window_id: None,
        }
    }
}

/// 📇️ Republishes the action ids the window kind hosting this surface declares. Idempotent: an
/// unchanged declaration rewrites nothing, so a host may call it on every frame.
///
/// 🔗️ The one source is the app manifest (`window_kind_action_refs` → `WindowKindDefinition.actions`);
/// see [`World3dState::declared_action_ids`] for what a lost declaration cost on 6118.
pub fn set_world3d_declared_actions(session_state: &mut WorldSessionState, action_ids: &[String]) {
    if session_state.declared_action_ids.len() == action_ids.len() && session_state.declared_action_ids.iter().zip(action_ids).all(|(held, declared)| held == declared) {
        return;
    }
    session_state.declared_action_ids.clear();
    session_state.declared_action_ids.extend_from_slice(action_ids);
}

/// 📇️ Whether this surface's window kind declares `action_id`, i.e. whether the surface may mint it.
pub fn world3d_declares_action(session_state: &WorldSessionState, action_id: &str) -> bool {
    session_state.declared_action_ids.iter().any(|declared| declared == action_id)
}

/// 🎚️ Whether this surface may offer a transform gumball at all — true when its window kind declares
/// at least one of the three verbs a gumball handle commits as.
pub fn world3d_offers_transform_gumball(session_state: &WorldSessionState) -> bool {
    [TRANSLATE_SELECTION_ACTION_ID, ROTATE_SELECTION_ACTION_ID, SCALE_SELECTION_ACTION_ID].iter().any(|action_id| world3d_declares_action(session_state, action_id))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dCameraFitRequest {
    pub revision: u32,
    pub padding: f32,
    pub bounds: Option<([f32; 3], [f32; 3])>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World3dCameraFitOwner {
    pub revision: u32,
    pub camera_digest: Option<u64>,
}

#[derive(Clone, Copy, Debug)]
pub struct World3dCameraBoundsCursor {
    pub key: u64,
    pub draw_generation: u64,
    pub geometry_generation: u64,
    pub draw: u16,
    pub instance: u32,
    pub corner: u8,
    pub minimum: [f32; 3],
    pub maximum: [f32; 3],
    pub any: bool,
}

impl World3dCameraBoundsCursor {
    pub fn new(key: u64, draw_generation: u64, geometry_generation: u64) -> Self {
        Self { key, draw_generation, geometry_generation, draw: 0, instance: 0, corner: 0, minimum: [f32::INFINITY; 3], maximum: [f32::NEG_INFINITY; 3], any: false }
    }

    /// 💤️ A completed empty generation parks on the draw index no bounded registry can publish.
    /// Reusing this existing scalar preserves the cursor's `Option` niche and the measured surface
    /// table receipt; another boolean enlarged every admitted World state by eight bytes.
    pub fn empty_generation(self) -> bool {
        self.draw == u16::MAX
    }

    pub fn expand(&mut self, point: Vec3) {
        for (axis, value) in [point.x, point.y, point.z].into_iter().enumerate() {
            self.minimum[axis] = self.minimum[axis].min(value);
            self.maximum[axis] = self.maximum[axis].max(value);
        }
        self.any = true;
    }
}

/// 🎛️ `selection_json.gumballConfig` — React's `GumballConfig`
/// (`🌐️World3dHost/🟦️.tsx:236`, `gumballConfigForTransformMode`).
///
/// ⚖️ Every flag is `Option<bool>`, not `bool`: React's `resolveGumballConfig` reads each group as
/// `config?.group !== false`, so an ABSENT key means ENABLED and only an explicit `false` hides a
/// group. A plain `#[serde(default)] bool` would read every authored config as "all groups off".
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct World3dSceneGumballConfigRecord {
    #[serde(default)]
    move_axes: Option<bool>,
    #[serde(default)]
    move_planes: Option<bool>,
    #[serde(default)]
    rotate: Option<bool>,
    #[serde(default)]
    scale_axes: Option<bool>,
    #[serde(default)]
    scale_planes: Option<bool>,
    #[serde(default)]
    scale_uniform: Option<bool>,
    /// 📐️ The planar subset a window projection implies (`xy`/`xz`/`yz`), from React's
    /// `worldGumballConfigForProjection`. `None` offers all three axes.
    #[serde(default)]
    pub plane: Option<String>,
}

impl World3dSceneGumballConfigRecord {
    /// 🎛️ React's `gumballConfigForTransformMode` (`🌐️World3dHost/🟦️.tsx:2683`) — the single-mode
    /// fallback when a producer authors no `gumballConfig`.
    ///
    /// ⚖️ React switches on three literals (`transform`/`rotate`/`scale`) and falls through to the
    /// move config; this host's own `transform_mode` also carries the verb spellings
    /// (`rotateSelection`/`scaleSelection`, and the default `translate`), so the arms match on the
    /// PREFIX. Same three groups, same fall-through.
    pub fn for_transform_mode(mode: &str) -> Self {
        let flags = |move_axes, move_planes, rotate, scale_axes, scale_planes, scale_uniform| Self {
            move_axes: Some(move_axes),
            move_planes: Some(move_planes),
            rotate: Some(rotate),
            scale_axes: Some(scale_axes),
            scale_planes: Some(scale_planes),
            scale_uniform: Some(scale_uniform),
            plane: None,
        };
        if mode.starts_with("transform") {
            return flags(true, true, true, false, false, false);
        }
        if mode.starts_with("rotate") {
            return flags(false, false, true, false, false, false);
        }
        if mode.starts_with("scale") {
            return flags(false, false, false, true, true, true);
        }
        flags(true, true, false, false, false, false)
    }

    /// 🎛️ React's `gumballHandleEnabled` — the handle's own group flag ∩ the drafting-plane subset
    /// (`🧱️elements/🎬️Scene/🟦️.tsx:489-506`, `GUMBALL_PLANE_HANDLES`).
    pub fn admits(&self, handle: GumballHandle) -> bool {
        let group = match handle {
            GumballHandle::MoveX | GumballHandle::MoveY | GumballHandle::MoveZ => self.move_axes,
            GumballHandle::MoveXY | GumballHandle::MoveYZ | GumballHandle::MoveXZ => self.move_planes,
            GumballHandle::RotateX | GumballHandle::RotateY | GumballHandle::RotateZ => self.rotate,
            GumballHandle::ScaleX | GumballHandle::ScaleY | GumballHandle::ScaleZ => self.scale_axes,
        };
        if group == Some(false) {
            return false;
        }
        match self.plane.as_deref() {
            None => true,
            Some("xy") => matches!(handle, GumballHandle::MoveX | GumballHandle::MoveY | GumballHandle::MoveXY | GumballHandle::RotateZ | GumballHandle::ScaleX | GumballHandle::ScaleY),
            Some("yz") => matches!(handle, GumballHandle::MoveY | GumballHandle::MoveZ | GumballHandle::MoveYZ | GumballHandle::RotateX | GumballHandle::ScaleY | GumballHandle::ScaleZ),
            Some("xz") => matches!(handle, GumballHandle::MoveX | GumballHandle::MoveZ | GumballHandle::MoveXZ | GumballHandle::RotateY | GumballHandle::ScaleX | GumballHandle::ScaleZ),
            Some(_) => true,
        }
    }
}

/// 🥽️ One page of a `registerBrushMesh` upload run this surface owns. `bytes` is the mesh's whole
/// wire payload — little-endian `f32` positions followed by little-endian `u32` indices, the exact
/// layout the guest's `decode_brush_mesh_page_values` reassembles and its `brush_mesh_digest`
/// hashes — read back once out of the resident [`Mesh3dLease`] when the run opened.
pub struct WorldBrushMeshRun {
    pub url: String,
    pub digest: String,
    pub bytes: Vec<u8>,
    pub position_values: usize,
    pub capacity: usize,
    pub page: u32,
    pub page_count: u32,
    pub revision: u64,
}

pub const WORLD_REFERENCE_SOURCE_PIXELS: u64 = 64 * 1024 * 1024 / 4;

/// 🎨️ The existing owned image decoder preserves admitted mesh texture pixels without reference-image resizing.
pub fn decode_mesh_surface_image_bytes(bytes:&[u8])->Option<semio_framework_pixels::RasterImage> {
    use image::ImageDecoder as _;
    let reader=image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().ok()?;let mut decoder=reader.into_decoder().ok()?;
    let (width,height)=decoder.dimensions();if u64::from(width).checked_mul(u64::from(height))?>WORLD_REFERENCE_SOURCE_PIXELS{return None}
    let orientation=decoder.orientation().unwrap_or(image::metadata::Orientation::NoTransforms);let mut image=image::DynamicImage::from_decoder(decoder).ok()?;image.apply_orientation(orientation);let rgba=image.to_rgba8();
    Some(semio_framework_pixels::RasterImage{width:rgba.width(),height:rgba.height(),pixels:rgba.into_raw()})
}

pub const FACE_OVERLAY_OFFSET: f32 = 0.003;

pub fn pick_targets_instance(selection_state: &WorldSelectionState, instance_id: &str) -> bool {
    selection_state.active_object_id.as_deref().is_none_or(|active_id| active_id == instance_id)
}

pub fn mesh_vertex(mesh: Mesh3dLease, index: u32) -> Option<Vec3> {
    world_mesh_vertex(mesh, index)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldFaceOverlayPhase {
    Count,
    Prepare,
    Allocate,
    Geometry,
    Indices,
    Seal,
    Publish,
    NextBucket,
    Complete,
}

#[derive(Clone, Copy)]
struct WorldFaceOverlayMatch {
    pub draw: u16,
    pub instance: u16,
    pub triangle: u32,
    pub category: u8,
    pub hovered: bool,
}

#[derive(Clone, Copy, Default)]
pub struct WorldFaceOverlayScan {
    pub draw: u16,
    pub instance: u16,
    pub triangle: u32,
    pub candidate: Option<u32>,
    pub hovered: bool,
    pub preview: usize,
    pub selected: usize,
}

pub enum WorldFaceOverlayScanStep {
    Pending,
    Match(WorldFaceOverlayMatch),
    Complete,
}

impl WorldFaceOverlayScan {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn finish_candidate(&mut self, category: Option<u8>) -> WorldFaceOverlayScanStep {
        let matched = category.map(|category| WorldFaceOverlayMatch { draw: self.draw, instance: self.instance, triangle: self.triangle, category, hovered: self.hovered });
        self.triangle = self.triangle.saturating_add(1);
        self.candidate = None;
        self.hovered = false;
        self.preview = 0;
        self.selected = 0;
        matched.map_or(WorldFaceOverlayScanStep::Pending, WorldFaceOverlayScanStep::Match)
    }

    pub fn step(&mut self, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState) -> WorldFaceOverlayScanStep {
        if let Some(face_id) = self.candidate {
            let instance = &scene_state.draws.get(usize::from(self.draw)).expect("retained overlay draw").instances[usize::from(self.instance)];
            if self.preview < selection_state.marquee_preview_ids.len() {
                let matches = selection_state.active_object_id.as_deref()==Some(&instance.id) && decimal_component_id_matches(&selection_state.marquee_preview_ids[self.preview], face_id);
                self.preview += 1;
                return if matches { self.finish_candidate(Some(0)) } else { WorldFaceOverlayScanStep::Pending };
            }
            if self.hovered {
                return self.finish_candidate(Some(1));
            }
            if self.selected < gumball_state.gumball_selection_ids.len() {
                let matches = world_component_preview_target(scene_state,&gumball_state.gumball_selection_ids[self.selected],"face").is_some_and(|(object,group)|object==instance.id && group==face_id);
                self.selected += 1;
                return if matches { self.finish_candidate(Some(2)) } else { WorldFaceOverlayScanStep::Pending };
            }
            let numeric = self.selected - gumball_state.gumball_selection_ids.len();
            if instance.component_source.is_none() && selection_state.active_object_id.as_deref()==Some(&instance.id) && numeric < selection_state.component_ids.len() {
                let matches = decimal_component_id_matches(&selection_state.component_ids[numeric],face_id);
                self.selected += 1;
                return if matches { self.finish_candidate(Some(2)) } else { WorldFaceOverlayScanStep::Pending };
            }
            return self.finish_candidate(None);
        }
        if usize::from(self.draw) >= usize::from(scene_state.draws.len) {
            return WorldFaceOverlayScanStep::Complete;
        }
        let Some(draw) = scene_state.draws.get(usize::from(self.draw)) else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        if usize::from(self.instance) >= draw.instances.len() {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        }
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        let Ok(schema) = mesh.schema() else {
            self.draw += 1;
            self.instance = 0;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        };
        if self.triangle >= schema.indices / 3 {
            self.instance += 1;
            self.triangle = 0;
            return WorldFaceOverlayScanStep::Pending;
        }
        let face_id = world_mesh_component_id(mesh, Mesh3dField::FaceIds, self.triangle);
        self.candidate = Some(face_id);
        self.hovered = instance_hovered_component_matches(selection_state, &draw.instances[usize::from(self.instance)].id, face_id);
        WorldFaceOverlayScanStep::Pending
    }
}

#[derive(Clone, Copy)]
struct WorldFaceOverlayGeometry {
    pub vertices: [Vec3; 3],
    pub normal: Vec3,
}

pub enum WorldFaceOverlayMeshStep {
    Pending,
    Ready { index: usize, color: [f32; 4], key: String, lease: Mesh3dLease },
    Complete { generation: u64, revision: u64, draw_generation: u64, colors: [Option<[f32; 4]>; 3] },
    Stale,
    Fault,
}

pub struct WorldFaceOverlayMeshCursor {
    pub surface_id: String,
    pub generation: u64,
    pub revision: u64,
    pub draw_generation: u64,
    pub phase: WorldFaceOverlayPhase,
    pub scan: WorldFaceOverlayScan,
    pub counts: [u32; 3],
    order: [u8; 3],
    order_len: u8,
    pub published_colors: [Option<[f32; 4]>; 3],
    bucket: u8,
    pub geometry: Option<WorldFaceOverlayGeometry>,
    geometry_item: u8,
    index_item: u32,
    pub owner: WorldPlaceholderOwner,
    pub retry_key: String,
    pub close_started: bool,
    pub faulted: bool,
    pub stale: bool,
}

impl WorldFaceOverlayMeshCursor {
    pub fn new(surface_id: &str, generation: u64, revision: u64, draw_generation: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        if surface_id.len().checked_add("component-face-overlay::2".len()).is_none_or(|bytes| bytes > WORLD_DYNAMIC_ID_BYTE_CAPACITY) {
            return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity);
        }
        Ok(Self {
            surface_id: surface_id.to_owned(),
            generation,
            revision,
            draw_generation,
            phase: WorldFaceOverlayPhase::Count,
            scan: WorldFaceOverlayScan::default(),
            counts: [0; 3],
            order: [0; 3],
            order_len: 0,
            published_colors: [None; 3],
            bucket: 0,
            geometry: None,
            geometry_item: 0,
            index_item: 0,
            owner: WorldPlaceholderOwner::Empty,
            retry_key: String::new(),
            close_started: false,
            faulted: false,
            stale: false,
        })
    }

    pub fn key(&self, index: usize) -> String {
        format!("component-face-overlay:{}:{}:{index}", self.surface_id, self.generation)
    }

    pub fn token(&self) -> Result<Mesh3dWriteToken, ui_wgpu::wgpu::Mesh3dFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(ui_wgpu::wgpu::Mesh3dFault::Stale),
        }
    }

    pub fn step(&mut self, session_state: &WorldSessionState, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState, overlay_state: &WorldOverlayState) -> WorldFaceOverlayMeshStep {
        if session_state.interaction_revision != self.revision || scene_state.draw_generation != self.draw_generation {
            self.faulted = true;
            self.stale = true;
        }
        if self.faulted {
            return if self.close_step() && self.terminal_is_empty() {
                if self.stale {
                    WorldFaceOverlayMeshStep::Stale
                } else {
                    WorldFaceOverlayMeshStep::Fault
                }
            } else {
                WorldFaceOverlayMeshStep::Pending
            };
        }
        match self.step_live(selection_state, gumball_state, scene_state, overlay_state) {
            Ok(step) => step,
            Err(_) => {
                self.faulted = true;
                let _ = self.close_step();
                WorldFaceOverlayMeshStep::Pending
            }
        }
    }

    pub fn step_live(&mut self, selection_state: &WorldSelectionState, gumball_state: &WorldGumballState, scene_state: &WorldSceneState, overlay_state: &WorldOverlayState) -> Result<WorldFaceOverlayMeshStep, ui_wgpu::wgpu::Mesh3dFault> {
        match self.phase {
            WorldFaceOverlayPhase::Count => match self.scan.step(selection_state, gumball_state, scene_state) {
                WorldFaceOverlayScanStep::Pending => {}
                WorldFaceOverlayScanStep::Match(found) => {
                    let category = usize::from(found.category);
                    if self.counts[category] == 0 {
                        self.order[usize::from(self.order_len)] = found.category;
                        self.order_len += 1;
                    }
                    self.counts[category] = self.counts[category].checked_add(1).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
                }
                WorldFaceOverlayScanStep::Complete => {
                    self.scan.reset();
                    self.phase = WorldFaceOverlayPhase::Prepare;
                }
            },
            WorldFaceOverlayPhase::Prepare => {
                if self.bucket == self.order_len {
                    self.phase = WorldFaceOverlayPhase::Complete;
                } else {
                    let count = self.counts[usize::from(self.order[usize::from(self.bucket)])];
                    let vertices = count.checked_mul(3).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
                    let indices = count.checked_mul(6).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
                    let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_begin_capacity_byte_demand(), 0, 1);
                    let (token, progress) = mesh3d_begin(self.generation + u64::from(self.bucket), self.revision, Mesh3dSchema::triangle_mesh(vertices, indices), grant)?;
                    self.owner = WorldPlaceholderOwner::Writing(token);
                    if !progress.fits(grant) { return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity); }
                    self.phase = WorldFaceOverlayPhase::Allocate;
                }
            }
            WorldFaceOverlayPhase::Allocate => {
                let token = self.token()?;
                let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_allocate_capacity_byte_demand(token)?, 0, 1);
                if world_mesh_ownership_complete(grant, mesh3d_allocate_step(token, grant)?)? {
                    self.scan.reset();
                    self.phase = WorldFaceOverlayPhase::Geometry;
                }
            }
            WorldFaceOverlayPhase::Geometry => {
                if let Some(geometry) = self.geometry {
                    let vertex = usize::from(self.geometry_item / 2);
                    let field = self.geometry_item % 2;
                    if field == 0 {
                        mesh3d_write_vec3(self.token()?, Mesh3dField::Positions, geometry.vertices[vertex].to_array())?;
                    } else {
                        mesh3d_write_vec3(self.token()?, Mesh3dField::Normals, geometry.normal.to_array())?;
                    }
                    self.geometry_item += 1;
                    if self.geometry_item == 6 {
                        self.geometry_item = 0;
                        self.geometry = None;
                    }
                } else {
                    match self.scan.step(selection_state, gumball_state, scene_state) {
                        WorldFaceOverlayScanStep::Pending => {}
                        WorldFaceOverlayScanStep::Match(found) if found.category == self.order[usize::from(self.bucket)] => {
                            self.geometry = Some(face_overlay_geometry(scene_state, found).ok_or(ui_wgpu::wgpu::Mesh3dFault::Stale)?);
                        }
                        WorldFaceOverlayScanStep::Match(_) => {}
                        WorldFaceOverlayScanStep::Complete => {
                            self.index_item = 0;
                            self.phase = WorldFaceOverlayPhase::Indices;
                        }
                    }
                }
            }
            WorldFaceOverlayPhase::Indices => {
                let category = usize::from(self.order[usize::from(self.bucket)]);
                let total = self.counts[category] * 6;
                let pattern = [0, 1, 2, 0, 2, 1];
                let value = (self.index_item / 6) * 3 + pattern[(self.index_item % 6) as usize];
                mesh3d_write_u32(self.token()?, Mesh3dField::Indices, value)?;
                self.index_item += 1;
                if self.index_item == total {
                    self.phase = WorldFaceOverlayPhase::Seal;
                }
            }
            WorldFaceOverlayPhase::Seal => {
                self.owner = WorldPlaceholderOwner::Ready(mesh3d_seal(self.token()?)?);
                self.phase = WorldFaceOverlayPhase::Publish;
            }
            WorldFaceOverlayPhase::Publish => {
                let WorldPlaceholderOwner::Ready(lease) = self.owner else { return Err(ui_wgpu::wgpu::Mesh3dFault::Stale) };
                self.owner = WorldPlaceholderOwner::Empty;
                self.phase = WorldFaceOverlayPhase::NextBucket;
                let index = usize::from(self.bucket);
                let key = if self.retry_key.is_empty() { self.key(index) } else { std::mem::take(&mut self.retry_key) };
                return Ok(WorldFaceOverlayMeshStep::Ready { index, color: face_overlay_color(&overlay_state.sub_element_paint, self.order[index]), key, lease });
            }
            WorldFaceOverlayPhase::NextBucket => {
                self.bucket += 1;
                self.scan.reset();
                self.phase = WorldFaceOverlayPhase::Prepare;
            }
            WorldFaceOverlayPhase::Complete => {
                self.surface_id.clear();
                return Ok(WorldFaceOverlayMeshStep::Complete { generation: self.generation, revision: self.revision, draw_generation: self.draw_generation, colors: self.published_colors });
            }
        }
        Ok(WorldFaceOverlayMeshStep::Pending)
    }

    pub fn close_step(&mut self) -> bool {
        self.geometry = None;
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => {
                if !self.close_started {
                    match mesh3d_abort(token) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_wgpu::wgpu::mesh3d_abort_demands(token, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_abort_step(token, grant)?) }) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Ready(lease) => {
                if !self.close_started {
                    match mesh3d_begin_close(lease) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_wgpu::wgpu::mesh3d_close_demands(lease, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_close_step(lease, grant)?) }) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Empty => {
                if self.retry_key.pop().is_some() {
                    return false;
                }
                if self.surface_id.pop().is_some() {
                    return false;
                }
                true
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        matches!(self.owner, WorldPlaceholderOwner::Empty) && self.surface_id.is_empty() && self.retry_key.is_empty() && self.geometry.is_none()
    }
}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldFaceOverlayMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "face overlay mesh cursor reached Drop before its exact authority witness");
    }
}

fn decimal_component_id_matches(value: &str, mut expected: u32) -> bool {
    if expected == 0 {
        return value == "0";
    }
    let digits = (expected.ilog10() + 1) as usize;
    if value.len() != digits || !value.is_ascii() {
        return false;
    }
    for digit in value.as_bytes().iter().rev() {
        if *digit != b'0' + (expected % 10) as u8 {
            return false;
        }
        expected /= 10;
    }
    true
}

fn instance_hovered_component_matches(selection_state: &WorldSelectionState, instance_id: &str, face_id: u32) -> bool {
    pick_targets_instance(selection_state, instance_id)
        && selection_state.hovered_component_mode.as_deref() == Some(selection_state.granularity.as_str())
        && selection_state.hovered_component_object_id.as_deref().is_none_or(|object_id| object_id == instance_id)
        && selection_state.hovered_component_id.as_deref().is_some_and(|id| decimal_component_id_matches(id, face_id))
}

pub fn face_overlay_color(paint: &WorldSubElementPaint, category: u8) -> [f32; 4] {
    let with_alpha = |color: [f32; 4], alpha: f32| [color[0], color[1], color[2], alpha];
    match category {
        0 => with_alpha(paint.face_hover.unwrap_or([1.0, 0.85, 0.35, 1.0]), 0.36),
        1 => with_alpha(paint.face_hover.unwrap_or([0.35, 0.75, 1.0, 1.0]), 0.48),
        _ => with_alpha(paint.face_select.unwrap_or([0.35, 0.75, 1.0, 1.0]), 0.62),
    }
}

fn face_overlay_geometry(scene_state: &WorldSceneState, found: WorldFaceOverlayMatch) -> Option<WorldFaceOverlayGeometry> {
    let draw = scene_state.draws.get(usize::from(found.draw))?;
    let instance = draw.instances.get(usize::from(found.instance))?;
    let mesh = *scene_state.meshes.get(&draw.mesh_key)?;
    let triangle = world_mesh_triangle(mesh, found.triangle)?;
    let vertices = [instance.model.transform_point(mesh_vertex(mesh, triangle[0])?), instance.model.transform_point(mesh_vertex(mesh, triangle[1])?), instance.model.transform_point(mesh_vertex(mesh, triangle[2])?)];
    let normal = vertices[1].sub(vertices[0]).cross(vertices[2].sub(vertices[0])).normalize();
    let offset = if found.hovered { FACE_OVERLAY_OFFSET } else { FACE_OVERLAY_OFFSET * 0.5 };
    Some(WorldFaceOverlayGeometry { vertices: vertices.map(|vertex| vertex.add(normal.scale(offset))), normal })
}

const WORLD3D_PREPARED_STATUS_BYTES: usize = 192;

#[derive(Clone, Copy)]
pub struct World3dPreparedStatus {
    pub bytes: [u8; WORLD3D_PREPARED_STATUS_BYTES],
    pub len: u8,
    pub progress: [f64; 3],
    pub total: u32,
    pub state: u32,
}

impl World3dPreparedStatus {
    pub fn from_parts(text: &str, progress: [f64; 3], total: u32, state: u32) -> Option<Self> {
        if text.len() > WORLD3D_PREPARED_STATUS_BYTES {
            return None;
        }
        let mut bytes = [0; WORLD3D_PREPARED_STATUS_BYTES];
        bytes[..text.len()].copy_from_slice(text.as_bytes());
        Some(Self { bytes, len: text.len() as u8, progress, total, state })
    }

    pub fn text(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok()
    }
}

pub struct World3dSnapshotApplyCursor {
    pub lease: World3dSnapshotLease,
    pub page: u16,
    pub item: u8,
    pub staged_orbit: Option<OrbitController>,
    pub draw_started: bool,
    /// 🧱️ Which draw the `Instance` items being applied right now belong to. A single-draw producer
    /// (`Mesh` item flags `30`) leaves it at 0; a multi-draw one (flags `31`) re-points it at every
    /// `Mesh` page, which is why those pages interleave with their own instance pages.
    pub draw_index: u16,
    pub draw_permit: Option<World3dSnapshotDrawPermit>,
    pub faulted: bool,
    pub status: [Option<World3dPreparedStatus>; 2],
}

impl World3dSnapshotApplyCursor {
    pub fn new(lease: World3dSnapshotLease) -> Self {
        Self { lease, page: 0, item: 0, staged_orbit: None, draw_started: false, draw_index: 0, draw_permit: None, faulted: false, status: [None; 2] }
    }

    pub fn close_step(&mut self) -> bool {
        if self.staged_orbit.take().is_some() {
            return false;
        }
        self.page = self.lease.page_count;
        self.item = 0;
        true
    }
}

/// Applies hover/selection action payloads to renderer-local world state before the plugin round-trip.
///
/// 🕹️ An undecodable merge leaves this OPTIMISTIC LOCAL PREVIEW untouched — the word is
/// outside `🕹️interaction`'s schema enum, so the authoritative path faults on it and the
/// preview must not invent a meaning for it (see `MergeMode::from_wire_label`).
///
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: replaces the deleted ad-hoc
/// `worldSelect`/`worldHover` command strings with the framework interaction verbs
/// (`domainId`/`targets`/`merge`/`method`, `domainId`/`channel`/`targets`) — see
/// `resolved_domain_id` (the domain this surface actually targets: its bound app domain, or the
/// OS `world` fallback). This is still the OPTIMISTIC LOCAL PREVIEW only; the framework's
/// `next_selection`/`next_hover` machine (not this file) is the source of truth once the
/// round-trip settles.
///
/// 🎯️ Validates a component target against the original retained instance and mesh label.
pub fn world_component_preview_target<'a>(scene_state: &WorldSceneState, id: &'a str, granularity: &str) -> Option<(&'a str, u32)> {
    let address = WorldComponentAddress::parse(id, granularity)?;
    let (draw, instance) = scene_state.draws.iter().find_map(|draw| draw.instances.iter().find(|instance| instance.id == address.object).map(|instance| (draw, instance)))?;
    address.matches_source(scene_state, instance.component_source, &draw.mesh_key).then_some((address.object, address.group))
}

//#region WorldRelocateGesture
/// 🚚️ One live relocate drag. `origin` is where the grabbed object sat when the press landed and
/// `from` is the ground point under that press; the commit applies the ground-plane travel
/// `from → to` to `origin`, never an incremental pose delta — React's `World3dRelocateSession`
/// (`🌐️World3dHost/🟦️.tsx:4752`).
#[derive(Clone, Debug, PartialEq)]
pub struct World3dRelocateSession {
    pub object_id: String,
    pub origin: [f32; 3],
    pub from: [f32; 3],
}

#[derive(Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct TerrainTileMeshPayload {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
    pub uvs: Vec<f32>,
}

//#region Gumball
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GumballHandle {
    MoveX,
    MoveY,
    MoveZ,
    MoveXY,
    MoveYZ,
    MoveXZ,
    RotateX,
    RotateY,
    RotateZ,
    ScaleX,
    ScaleY,
    ScaleZ,
}

impl GumballHandle {
    pub fn axis_dir(self) -> Option<Vec3> {
        match self {
            Self::MoveX | Self::RotateX | Self::ScaleX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveY | Self::RotateY | Self::ScaleY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::MoveZ | Self::RotateZ | Self::ScaleZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    pub fn plane_normal(self) -> Option<Vec3> {
        match self {
            Self::MoveXY => Some(Vec3::new(0.0, 0.0, 1.0)),
            Self::MoveYZ => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::MoveXZ => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateX => Some(Vec3::new(1.0, 0.0, 0.0)),
            Self::RotateY => Some(Vec3::new(0.0, 1.0, 0.0)),
            Self::RotateZ => Some(Vec3::new(0.0, 0.0, 1.0)),
            _ => None,
        }
    }

    pub fn is_translate(self) -> bool {
        matches!(self, Self::MoveX | Self::MoveY | Self::MoveZ | Self::MoveXY | Self::MoveYZ | Self::MoveXZ)
    }

    pub fn is_rotate(self) -> bool {
        matches!(self, Self::RotateX | Self::RotateY | Self::RotateZ)
    }

    pub fn is_scale(self) -> bool {
        matches!(self, Self::ScaleX | Self::ScaleY | Self::ScaleZ)
    }
}
//#endregion Gumball

//#region MeshHelpers
//#region PlaceholderMeshAuthority
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldPlaceholderKind {
    Box,
    Plane,
    Cylinder,
    Cone,
    Icosphere,
}

impl WorldPlaceholderKind {
    pub fn resolve(value: &str) -> Self {
        match value {
            "vortex-marker" => Self::Icosphere,
            "plane" => Self::Plane,
            "cylinder" => Self::Cylinder,
            "cone" => Self::Cone,
            _ => Self::Box,
        }
    }

    pub fn triangles(self) -> u32 {
        match self {
            Self::Box => 12,
            Self::Plane => 2,
            Self::Cylinder => 64,
            Self::Cone => 32,
            Self::Icosphere => 80,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WorldPlaceholderMeshPhase {
    Allocate,
    Positions,
    Normals,
    Tangents,
    Uvs,
    Colors,
    Indices,
    FaceIds,
    VertexIds,
    Edges,
    EdgeIds,
    Materials,
    Textures,
    Provenance,
    AdmitProvenance,
    Seal,
    Publish,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldPlaceholderOwner {
    Writing(Mesh3dWriteToken),
    Ready(Mesh3dLease),
    Empty,
}

/// 🧊️ Where one in-flight mesh publication reads its vertices from: a generated marker primitive,
/// or an inline `World3dScene.meshes_json` buffer set bridged in by [`stage_world3d_scene_bridge`].
/// Both ride the identical fixed-credit `mesh3d_*` authority ladder below.
pub enum WorldMeshInput {
    Placeholder(WorldPlaceholderKind),
    Inline(Box<WorldMeshBuffers>),
}

impl WorldMeshInput {
    pub fn position(&self, item: u32) -> [f32; 3] {
        match self {
            Self::Placeholder(kind) => placeholder_triangle(*kind, item / 3)[(item % 3) as usize],
            Self::Inline(buffers) if buffers.positions.is_empty() => {
                let offset = item as usize * 3;
                [buffers.edge_positions[offset], buffers.edge_positions[offset + 1], buffers.edge_positions[offset + 2]]
            }
            Self::Inline(buffers) => buffers.position(item),
        }
    }

    pub fn normal(&self, item: u32) -> [f32; 3] {
        match self {
            Self::Placeholder(kind) => placeholder_triangle_normal(placeholder_triangle(*kind, item / 3)),
            Self::Inline(buffers) => buffers.normal(item),
        }
    }

    pub fn index(&self, item: u32) -> u32 {
        match self {
            Self::Placeholder(_) => item,
            Self::Inline(buffers) => buffers.indices.get(item as usize).copied().unwrap_or(0),
        }
    }
}

pub struct WorldPlaceholderMeshCursor {
    pub key: String,
    pub source: WorldMeshInput,
    vertex_items: u32,
    index_items: u32,
    pub phase: WorldPlaceholderMeshPhase,
    pub item: u32,
    pub owner: WorldPlaceholderOwner,
    pub close_started: bool,
    pub faulted: bool,
    pub expanded: bool,
    source_tail_start: u32,
    pub schema:Mesh3dSchema,
    uv_role:usize,
    texture_cursor:usize,
    texture_role:usize,
    texture_name:Option<String>,
    pub texture_job:Option<semio_framework_pixels::png_decoding::PngDecodeJob>,
    pub texture_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<semio_framework_pixels::png_decoding::PngDecodeJob>>,
    texture_image:Option<semio_framework_pixels::RasterImage>,
    texture_digest:[u64;2],
    texture_bytes:Vec<u8>,
    texture_byte:usize,
    texture_scan:usize,
    texture_required:bool,
    texture_hash:semio_framework_hash::Sha256,
    texture_submission:Option<semio_framework_async::Job>,
    texture_cancelled:std::sync::Arc<std::sync::atomic::AtomicBool>,
    texture_worker:Option<std::sync::Arc<Mutex<Option<Option<semio_framework_pixels::RasterImage>>>>>,
    pub primitives:Vec<World3dPrimitiveMaterial>,
    pub textures:Vec<(String,SceneRasterLease)>,
    pub appearance:Option<World3dMeshAppearance>,
    pub source_retirement:Option<semio_framework_value::retirement::controlled::ControlledRetirement<Box<WorldMeshBuffers>>>,
}

pub enum WorldPlaceholderMeshStep {
    Pending,
    Ready(String, Mesh3dLease,Option<World3dMeshAppearance>),
    Fault,
}

impl WorldPlaceholderMeshCursor {
    pub fn new(key: &str, kind: WorldPlaceholderKind, generation: u64, revision: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        let items = kind.triangles().checked_mul(3).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
        Self::begin(key, WorldMeshInput::Placeholder(kind), items, items, generation, revision)
    }

    /// 🌉️ The inline-wire twin of [`Self::new`]: an already-welded `meshes_json` buffer set, whose
    /// vertex and index counts are independent (the marker primitives above emit one vertex per
    /// face corner, a tessellated preview mesh does not).
    pub fn inline(key: &str, buffers: WorldMeshBuffers, generation: u64, revision: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        let edge_only = buffers.positions.is_empty() && !buffers.edge_positions.is_empty();
        if !buffers.positions.len().is_multiple_of(3) || !buffers.edge_positions.len().is_multiple_of(6)
            || (edge_only && (!buffers.indices.is_empty() || !buffers.vertex_ids.is_empty())) {
            return Err(ui_wgpu::wgpu::Mesh3dFault::Schema);
        }
        let vertex_items = u32::try_from(if edge_only { buffers.edge_positions.len() / 3 } else { buffers.vertex_count() }).map_err(|_| ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
        let index_items = u32::try_from(buffers.indices.len()).map_err(|_| ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
        if vertex_items == 0 || !index_items.is_multiple_of(3) || buffers.indices.iter().any(|index| *index >= vertex_items) {
            return Err(ui_wgpu::wgpu::Mesh3dFault::Schema);
        }
        semio_framework::validate_mesh_surface_assets(&buffers.materials,&buffers.textures).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        for(name,attribute)in &buffers.attributes {let uses=if attribute.domain==semio_framework::MeshAttributeDomain::Edge{buffers.edge_ids.len()}else{index_items as usize};semio_framework::validate_mesh_attribute(name,attribute,vertex_items as usize,index_items as usize/3,uses,&buffers.materials).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?;}
        let expanded=!buffers.indices.is_empty() && buffers.attributes.values().any(|attribute|attribute.domain==semio_framework::MeshAttributeDomain::Corner || attribute.domain==semio_framework::MeshAttributeDomain::Face);
        let source_tail_start = if expanded { buffers.vertex_ids.iter().position(|id| *id != u32::MAX).map_or(vertex_items, |item| item as u32) } else { vertex_items };
        let expanded_items = index_items.checked_add(vertex_items - source_tail_start).ok_or(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)?;
        let mut cursor=Self::begin(key,WorldMeshInput::Inline(Box::new(buffers)),if expanded {expanded_items}else{vertex_items},index_items,generation,revision)?;
        cursor.expanded=expanded;
        cursor.source_tail_start=source_tail_start;
        Ok(cursor)
    }

    pub fn begin(key: &str, source: WorldMeshInput, vertex_items: u32, index_items: u32, generation: u64, revision: u64) -> Result<Self, ui_wgpu::wgpu::Mesh3dFault> {
        if key.len() > WORLD_DYNAMIC_ID_BYTE_CAPACITY {
            return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity);
        }
        let (uvs,colors)=match &source {WorldMeshInput::Inline(buffers)=>(!buffers.uvs.is_empty()||buffers.attributes.values().any(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Uv),!buffers.colors.is_empty()||buffers.attributes.values().any(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Color)),_ => (false,false)};
        let (face_ids,vertex_ids,edges,edge_ids)=match &source {WorldMeshInput::Inline(mesh)=>(mesh.face_ids.len() as u32,if mesh.vertex_ids.is_empty(){0}else{vertex_items},mesh.edge_positions.len() as u32/6,mesh.edge_ids.len() as u32),_ => (0,0,0,0)};
        let schema=Mesh3dSchema{face_ids,vertex_ids,edges,edge_ids,uvs:if uvs {vertex_items}else{0},colors:if colors {vertex_items}else{0},surface_uvs:if uvs {[vertex_items;4]}else{[0;4]},tangents:if matches!(&source,WorldMeshInput::Inline(mesh) if mesh.attributes.contains_key("tangent")){vertex_items}else{0},..Mesh3dSchema::triangle_mesh(vertex_items,index_items)};
        let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_begin_capacity_byte_demand(), 0, 1);
        let (token, progress) = mesh3d_begin(generation, revision, schema, grant)?;
        let owner = WorldPlaceholderOwner::Writing(token);
        assert!(progress.fits(grant), "original mesh constructor exceeded its admitted ownership grant");
        Ok(Self { key: key.to_owned(), source, vertex_items, index_items, phase: WorldPlaceholderMeshPhase::Allocate, item: 0, owner, close_started: false, faulted: false,expanded:false,source_tail_start:0,schema,uv_role:0,texture_cursor:0,texture_role:0,texture_name:None,texture_job:None,texture_retirement:None,texture_image:None,texture_digest:[0;2],texture_bytes:Vec::new(),texture_byte:0,texture_scan:0,texture_required:false,texture_hash:Default::default(),texture_submission:None,texture_cancelled:std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),texture_worker:None,primitives:Vec::new(),textures:Vec::new(),appearance:None,source_retirement:None })
    }

    fn source_vertex(&self,item:u32)->u32 {if self.expanded {if item < self.index_items {self.source.index(item)}else{self.source_tail_start + item - self.index_items}}else{item}}

    pub fn sample<const N:usize>(&self,semantic:semio_framework::MeshAttributeSemantic,item:u32,fallback:[f32;N])->Result<[f32;N],ui_wgpu::wgpu::Mesh3dFault> {
        let WorldMeshInput::Inline(mesh)=&self.source else{return Ok(fallback)};
        let Some(attribute)=mesh.attributes.values().find(|attribute|attribute.semantic==semantic) else{return Ok(fallback)};
        if self.expanded && item >= self.index_items && attribute.domain != semio_framework::MeshAttributeDomain::Vertex { return Ok(fallback); }
        let domain=match attribute.domain {semio_framework::MeshAttributeDomain::Vertex=>self.source_vertex(item) as usize,semio_framework::MeshAttributeDomain::Corner=>item as usize,semio_framework::MeshAttributeDomain::Face=>item as usize/3,_=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
        let values=attribute.value_at(domain).and_then(|value|value.as_array()).filter(|values|values.len()==N).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        let mut result=[0.0;N];for axis in 0..N {result[axis]=values[axis].as_f64().filter(|value|value.is_finite()&&value.abs()<=f32::MAX as f64).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32;}Ok(result)
    }

    fn uv_sample(&self,role:usize,item:u32)->Result<[f32;2],ui_wgpu::wgpu::Mesh3dFault> {
        let WorldMeshInput::Inline(mesh)=&self.source else{return Ok([0.0;2])};
        let fields=["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"];
        let name=mesh.attributes.values().find(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Material).and_then(|attribute|attribute.value_at(item as usize/3)).and_then(|value|value.as_str()).or_else(||mesh.materials.keys().next().map(String::as_str));
        let material=name.and_then(|name|mesh.materials.get(name)).and_then(|value|value.as_object());
        let coordinates=material.and_then(|fields|fields.iter().find_map(|(key,value)|(key=="textureCoordinates").then_some(value))).and_then(|value|value.as_object());
        let set=coordinates.and_then(|values|values.iter().find_map(|(key,value)|(key==fields[role]).then_some(value))).and_then(|value|value.as_u64()).unwrap_or(0) as usize;
        let declared=mesh.attributes.get(if set==0{"uv"}else{""}).or_else(||mesh.attributes.get(&format!("uv{set}")));
        let attribute=declared.or_else(||(set==0).then(||mesh.attributes.values().find(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Uv)).flatten());
        let Some(attribute)=attribute else {if set!=0{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}let at=self.source_vertex(item) as usize*2;return Ok([mesh.uvs.get(at).copied().unwrap_or(0.0),mesh.uvs.get(at+1).copied().unwrap_or(0.0)])};
        if self.expanded && item >= self.index_items && attribute.domain != semio_framework::MeshAttributeDomain::Vertex { let at=self.source_vertex(item) as usize*2;return Ok([mesh.uvs.get(at).copied().unwrap_or(0.0),mesh.uvs.get(at+1).copied().unwrap_or(0.0)]); }
        let domain=match attribute.domain{semio_framework::MeshAttributeDomain::Vertex=>self.source_vertex(item) as usize,semio_framework::MeshAttributeDomain::Corner=>item as usize,semio_framework::MeshAttributeDomain::Face=>item as usize/3,_=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
        let values=attribute.value_at(domain).and_then(|value|value.as_array()).filter(|values|values.len()==2).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        Ok([values[0].as_f64().ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32,values[1].as_f64().ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32])
    }

    fn material_step(&mut self)->Result<(),ui_wgpu::wgpu::Mesh3dFault> {
        let WorldMeshInput::Inline(mesh)=&self.source else{return Ok(())};
        if mesh.materials.is_empty(){return Ok(())}
        let attribute=mesh.attributes.values().find(|attribute|attribute.semantic==semio_framework::MeshAttributeSemantic::Material);
        let name=attribute.and_then(|attribute|attribute.value_at(self.item as usize/3)).and_then(|value|value.as_str()).or_else(||mesh.materials.keys().next().map(String::as_str)).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        let source=mesh.materials.get(name).and_then(|value|value.as_object()).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
        let find=|name:&str|source.iter().find_map(|(key,value)|(key==name).then_some(value));
        let tuple=|name:&str,defaults:[f32;4],width:usize|->Result<[f32;4],ui_wgpu::wgpu::Mesh3dFault>{let Some(value)=find(name)else{return Ok(defaults)};let values=value.as_array().filter(|values|values.len()==width).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;let mut result=defaults;for axis in 0..width{result[axis]=values[axis].as_f64().ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32;}Ok(result)};
        let sampler=|role:usize| {let fields=["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"];let owned=find("textureSamplers").and_then(|value|value.as_object()).and_then(|values|values.iter().find_map(|(key,value)|(key==fields[role]).then_some(value))).and_then(|value|value.as_object());let value=|name:&str,default:u64|owned.and_then(|values|values.iter().find_map(|(key,value)|(key==name).then_some(value))).and_then(|value|value.as_u64()).unwrap_or(default);let wrap=|value|match value{33071=>ui_wgpu::wgpu::SceneTextureWrap3d::ClampToEdge,33648=>ui_wgpu::wgpu::SceneTextureWrap3d::MirrorRepeat,_=>ui_wgpu::wgpu::SceneTextureWrap3d::Repeat};let filter=|value|match value{9728=>ui_wgpu::wgpu::SceneTextureFilter3d::Nearest,9984=>ui_wgpu::wgpu::SceneTextureFilter3d::NearestMipmapNearest,9985=>ui_wgpu::wgpu::SceneTextureFilter3d::LinearMipmapNearest,9986=>ui_wgpu::wgpu::SceneTextureFilter3d::NearestMipmapLinear,9987=>ui_wgpu::wgpu::SceneTextureFilter3d::LinearMipmapLinear,_=>ui_wgpu::wgpu::SceneTextureFilter3d::Linear};ui_wgpu::wgpu::SceneTextureSampler3d{wrap_u:wrap(value("wrapS",10497)),wrap_v:wrap(value("wrapT",10497)),mag_filter:filter(value("magFilter",9729)),min_filter:filter(value("minFilter",9729))}};
        let scalar=|name:&str,default:f32|find(name).and_then(|value|value.as_f64()).map_or(default,|value|value as f32);
        let alpha=match find("alphaMode").and_then(|value|value.as_str()).unwrap_or("OPAQUE"){"OPAQUE"=>SceneMaterialAlpha3d::Opaque,"MASK"=>SceneMaterialAlpha3d::Mask,"BLEND"=>SceneMaterialAlpha3d::Blend,_=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
        let mut color=tuple("baseColor",[1.0;4],4)?;if alpha==SceneMaterialAlpha3d::Opaque{color[3]=1.0;}let emissive=tuple("emissive",[0.0;4],3)?;
        let reference=|field:&str,role:usize|find(field).and_then(|value|value.as_str()).map(|id|format!("inline:{}:{}:{}",self.key,id,if role==0||role==4{"srgb"}else{"linear"}));
        let material=SceneAuthoredMaterial3d{base_color:color,emissive:[emissive[0],emissive[1],emissive[2]],metalness:scalar("metallic",1.0),roughness:scalar("roughness",1.0),alpha,alpha_cutoff:scalar("alphaCutoff",0.5),double_sided:find("doubleSided").and_then(|value|value.as_bool()).unwrap_or(false),preserve_vertex_color:self.schema.colors>0,base_color_texture:reference("baseColorTexture",0),texture_sampler:sampler(0),metallic_roughness_texture:reference("metallicRoughnessTexture",1),normal_texture:reference("normalTexture",2),occlusion_texture:reference("occlusionTexture",3),emissive_texture:reference("emissiveTexture",4),additional_texture_samplers:[sampler(1),sampler(2),sampler(3),sampler(4)],normal_scale:find("normalScale").and_then(|value|value.as_array()).map(|values|[values[0].as_f64().unwrap_or(1.0) as f32,values[1].as_f64().unwrap_or(1.0) as f32]).unwrap_or([scalar("normalScale",1.0);2]),occlusion_strength:scalar("occlusionStrength",1.0),};
        if let Some(previous)=self.primitives.last_mut().filter(|previous|previous.material==material){previous.index_count+=3;}else{if self.primitives.len()==WORLD_DYNAMIC_MESH_CAPACITY{return Err(ui_wgpu::wgpu::Mesh3dFault::ItemCapacity)}self.primitives.push(World3dPrimitiveMaterial{first_index:self.item,index_count:3,material});}Ok(())
    }

    fn texture_step(&mut self)->Result<bool,ui_wgpu::wgpu::Mesh3dFault> {
        if self.texture_retirement.is_some(){self.retire_texture_job()?;return Ok(false)}
        let WorldMeshInput::Inline(mesh)=&mut self.source else{return Ok(true)};
        if self.texture_name.is_none(){let Some(id)=mesh.textures.keys().nth(self.texture_cursor)else{return Ok(true)};self.texture_name=Some(id.clone());self.texture_scan=0;self.texture_required=false;}
        let id=self.texture_name.as_ref().unwrap();let role=if self.texture_role==0 {"srgb"}else{"linear"};let key=format!("inline:{}:{id}:{role}",self.key);
        if key.len()>WORLD_DYNAMIC_ID_BYTE_CAPACITY{return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity)}
        if self.texture_scan<self.primitives.len(){let material=&self.primitives[self.texture_scan].material;let references=if self.texture_role==0 {[material.base_color_texture.as_ref(),material.emissive_texture.as_ref(),None]}else{[material.metallic_roughness_texture.as_ref(),material.normal_texture.as_ref(),material.occlusion_texture.as_ref()]};self.texture_required|=references.into_iter().flatten().any(|reference|reference==&key);self.texture_scan+=1;return Ok(false)}
        if !self.texture_required {self.next_texture_role();return Ok(false)}
        if self.texture_image.is_none() {
            if let Some(submission)=self.texture_submission.take(){
                let cores=if cfg!(target_arch="wasm32"){1}else{std::thread::available_parallelism().map_or(1,std::num::NonZeroUsize::get)};
                let pool=semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative,cores));
                if let Err(refusal)=pool.try_submit(semio_framework_async::Lane::Maintenance,submission){self.texture_submission=Some(refusal.into_job());}return Ok(false)
            }
            if let Some(worker)=self.texture_worker.as_ref(){let result=worker.lock().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?.take();let Some(image)=result else{return Ok(false)};self.texture_image=Some(image.ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?);self.texture_worker=None;return Ok(false)}
            if self.texture_job.is_none() {
                let texture=mesh.textures.get(id).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
                if self.texture_byte<texture.bytes.len(){if self.texture_byte==0{self.texture_bytes.try_reserve_exact(texture.bytes.len()).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::ByteCapacity)?;}let end=(self.texture_byte+4096).min(texture.bytes.len());let bytes=&texture.bytes[self.texture_byte..end];self.texture_bytes.extend_from_slice(bytes);self.texture_hash.update(bytes);self.texture_byte=end;return Ok(false)}
                let digest=std::mem::take(&mut self.texture_hash).finalize();self.texture_digest=[u64::from_be_bytes(digest[..8].try_into().unwrap()),u64::from_be_bytes(digest[8..16].try_into().unwrap())];
                if texture.mime=="image/png" {match semio_framework_pixels::png_decoding::PngDecodeJob::new(semio_framework_pixels::png_decoding::PngDecodeInput{data:std::mem::take(&mut self.texture_bytes),max_pixels:WORLD_REFERENCE_SOURCE_PIXELS as usize,max_bytes:16_000_000,max_chunks:65536}){Ok(job)=>self.texture_job=Some(job),Err((_,input))=>{self.texture_bytes=input.data;return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}}}
                else if texture.mime=="image/jpeg" {
                    let worker=std::sync::Arc::new(Mutex::new(None));let output=worker.clone();let canceled=self.texture_cancelled.clone();let bytes=std::mem::take(&mut self.texture_bytes);
                    self.texture_submission=Some(Box::new(move ||{let image=if canceled.load(std::sync::atomic::Ordering::Acquire){None}else{decode_mesh_surface_image_bytes(&bytes)};*output.lock().unwrap_or_else(|poisoned|poisoned.into_inner())=Some(image);}));self.texture_worker=Some(worker);return Ok(false)
                }else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}
            }
            let job=self.texture_job.as_mut().unwrap();
            let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:job.next_copy_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?.max(32)*4096,maximum_capacity_bytes:job.next_capacity_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?,maximum_release_bytes:0,maximum_depth:1};
            let (progress,receipt)=job.advance(grant).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?;
            if !receipt.fits(grant){return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}
            if progress.done {
                let grant=world_mesh_ownership_grant(size_of::<semio_framework_pixels::RasterImage>(),0,0,1);
                let(image,receipt)=job.take_result(grant).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Schema)?.ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
                if !receipt.fits(grant){return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}
                self.texture_image=Some(image);
                self.retire_texture_job()?;
            }
            return Ok(false)
        }
        let image=self.texture_image.as_ref().unwrap();
        let descriptor=SceneRasterDescriptor{width:image.width,height:image.height,source_digest:self.texture_digest,source_revision:1,profile:if self.texture_role==0{SceneRasterProfile::MeshBaseColorSrgb}else{SceneRasterProfile::MeshNumericLinear},mesh:None};
        let pool=world_scene_raster_pool();let owner=self.texture_digest[0].wrapping_add(self.texture_digest[1]).max(1);
        let lease=match pool.begin(descriptor,owner,SceneRasterWriteMode::Moved { capacity_bytes: image.pixels.capacity() }){SceneRasterBegin::Reused(lease)=>{self.texture_image=None;lease},SceneRasterBegin::Writer(writer)=>{let image=self.texture_image.take().unwrap();match pool.prepare_moved(writer,image.pixels).and_then(|prepared|pool.seal_prepared_moved(prepared)){Ok(lease)=>lease,Err((pixels,_))=>{self.texture_image=Some(semio_framework_pixels::RasterImage{width:descriptor.width,height:descriptor.height,pixels});return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity)}}},SceneRasterBegin::Backpressure(_)=>return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity),SceneRasterBegin::Refused(_)=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
        self.textures.push((key,lease));self.next_texture_role();Ok(false)
    }

    fn next_texture_role(&mut self){self.texture_role+=1;if self.texture_role==2{self.texture_role=0;self.texture_cursor+=1;}self.texture_name=None;self.texture_byte=0;self.texture_scan=0;self.texture_required=false;}

    pub fn token(&self) -> Result<Mesh3dWriteToken, ui_wgpu::wgpu::Mesh3dFault> {
        match self.owner {
            WorldPlaceholderOwner::Writing(token) => Ok(token),
            _ => Err(ui_wgpu::wgpu::Mesh3dFault::Stale),
        }
    }

    pub fn step(&mut self) -> WorldPlaceholderMeshStep {
        if self.faulted {
            return if self.close_step() && self.terminal_is_empty() { WorldPlaceholderMeshStep::Fault } else { WorldPlaceholderMeshStep::Pending };
        }
        let result = self.step_live();
        if result.is_err() {
            self.faulted = true;
            let _ = self.close_step();
            return WorldPlaceholderMeshStep::Pending;
        }
        result.expect("placeholder result checked above")
    }

    pub fn step_live(&mut self) -> Result<WorldPlaceholderMeshStep, ui_wgpu::wgpu::Mesh3dFault> {
        match self.phase {
            WorldPlaceholderMeshPhase::Allocate => {
                let token = self.token()?;
                let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_allocate_capacity_byte_demand(token)?, 0, 1);
                if world_mesh_ownership_complete(grant, mesh3d_allocate_step(token, grant)?)? {
                    self.phase = WorldPlaceholderMeshPhase::Positions;
                }
            }
            WorldPlaceholderMeshPhase::Positions => {
                mesh3d_write_vec3(self.token()?, Mesh3dField::Positions, self.source.position(self.source_vertex(self.item)))?;
                self.item += 1;
                if self.item == self.vertex_items {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::Normals;
                }
            }
            WorldPlaceholderMeshPhase::Normals => {
                mesh3d_write_vec3(self.token()?, Mesh3dField::Normals, self.sample::<3>(semio_framework::MeshAttributeSemantic::Normal,self.item,self.source.normal(self.source_vertex(self.item)))?)?;
                self.item += 1;
                if self.item == self.vertex_items {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::Tangents;
                }
            }
            WorldPlaceholderMeshPhase::Tangents=>{
                if self.schema.tangents>0 {
                    let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
                    let attribute=mesh.attributes.get("tangent").ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
                    let mut value=[1.0,0.0,0.0,1.0];
                    if !(self.expanded && self.item>=self.index_items && attribute.domain!=semio_framework::MeshAttributeDomain::Vertex) {
                        let domain=match attribute.domain{semio_framework::MeshAttributeDomain::Vertex=>self.source_vertex(self.item) as usize,semio_framework::MeshAttributeDomain::Corner=>self.item as usize,semio_framework::MeshAttributeDomain::Face=>self.item as usize/3,_=>return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};
                        let values=attribute.value_at(domain).and_then(|value|value.as_array()).filter(|values|values.len()==4).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;
                        for axis in 0..4 {value[axis]=values[axis].as_f64().filter(|value|value.is_finite() && value.abs()<=f32::MAX as f64).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)? as f32;}
                    }
                    if value[..3].iter().all(|value|*value==0.0) || value[3].abs()!=1.0{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)}
                    mesh3d_write_vec4(self.token()?,Mesh3dField::Tangents,value)?;self.item+=1;
                }
                if self.schema.tangents==0 || self.item==self.vertex_items{self.item=0;self.phase=WorldPlaceholderMeshPhase::Uvs;}
            }
            WorldPlaceholderMeshPhase::Uvs => {
                if self.schema.uvs>0 {let role=self.uv_role;let value=self.uv_sample(role,self.item)?;ui_wgpu::wgpu::mesh3d_write_vec2(self.token()?,[Mesh3dField::Uvs,Mesh3dField::UvsMetallicRoughness,Mesh3dField::UvsNormal,Mesh3dField::UvsOcclusion,Mesh3dField::UvsEmissive][role],value)?;self.uv_role+=1;if self.uv_role==5{self.uv_role=0;self.item+=1;}}
                if self.schema.uvs==0 || self.item==self.vertex_items {self.item=0;self.phase=WorldPlaceholderMeshPhase::Colors;}
            }
            WorldPlaceholderMeshPhase::Colors => {
                let token=self.token()?;
                if self.schema.colors>0 {let fallback=match &self.source {WorldMeshInput::Inline(mesh)=>{let at=self.source_vertex(self.item) as usize*4;std::array::from_fn(|axis|mesh.colors.get(at+axis).copied().unwrap_or(1.0))},_=>[1.0;4]};mesh3d_write_vec4(token,Mesh3dField::Colors,self.sample::<4>(semio_framework::MeshAttributeSemantic::Color,self.item,fallback)?)?;self.item+=1;}
                if self.schema.colors==0 || self.item==self.vertex_items {self.item=0;self.phase=WorldPlaceholderMeshPhase::Indices;}
            }
            WorldPlaceholderMeshPhase::Indices => {
                if self.item < self.index_items {
                    mesh3d_write_u32(self.token()?, Mesh3dField::Indices, if self.expanded {self.item}else{self.source.index(self.item)})?;
                    self.item += 1;
                } else {
                    self.item = 0;
                    self.phase = WorldPlaceholderMeshPhase::FaceIds;
                }
            }
            WorldPlaceholderMeshPhase::FaceIds => {
                if self.item < self.schema.face_ids {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};mesh3d_write_u32(self.token()?,Mesh3dField::FaceIds,mesh.face_ids[self.item as usize])?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::VertexIds;}
            }
            WorldPlaceholderMeshPhase::VertexIds => {
                if self.item < self.schema.vertex_ids {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};let id=*mesh.vertex_ids.get(self.source_vertex(self.item) as usize).ok_or(ui_wgpu::wgpu::Mesh3dFault::Schema)?;mesh3d_write_u32(self.token()?,Mesh3dField::VertexIds,id)?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::Edges;}
            }
            WorldPlaceholderMeshPhase::Edges => {
                if self.item < self.schema.edges {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};let at=self.item as usize*6;ui_wgpu::wgpu::mesh3d_write_edge(self.token()?,[[mesh.edge_positions[at],mesh.edge_positions[at+1],mesh.edge_positions[at+2]],[mesh.edge_positions[at+3],mesh.edge_positions[at+4],mesh.edge_positions[at+5]]])?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::EdgeIds;}
            }
            WorldPlaceholderMeshPhase::EdgeIds => {
                if self.item < self.schema.edge_ids {let WorldMeshInput::Inline(mesh)=&self.source else{return Err(ui_wgpu::wgpu::Mesh3dFault::Schema)};mesh3d_write_u32(self.token()?,Mesh3dField::EdgeIds,mesh.edge_ids[self.item as usize])?;self.item+=1;}
                else {self.item=0;self.phase=WorldPlaceholderMeshPhase::Materials;}
            }
            WorldPlaceholderMeshPhase::Materials => {
                if self.item==self.index_items {self.item=0;self.phase=WorldPlaceholderMeshPhase::Textures;}
                else {self.material_step()?;self.item+=3;}
            }
            WorldPlaceholderMeshPhase::Textures => {
                if self.texture_step()? {
                    if !self.primitives.is_empty() {self.appearance=Some(World3dMeshAppearance::new(std::mem::take(&mut self.primitives),std::mem::take(&mut self.textures)).map_err(|(primitives,textures)|{self.primitives=primitives;self.textures=textures;ui_wgpu::wgpu::Mesh3dFault::ByteCapacity})?);}
                    self.phase=WorldPlaceholderMeshPhase::Provenance;
                }
            }
            WorldPlaceholderMeshPhase::Provenance => {
                let token = self.token()?;
                if let WorldMeshInput::Inline(mesh) = &mut self.source {
                    let grant = world_mesh_ownership_grant(0, ui_wgpu::wgpu::mesh3d_component_capacity_byte_demand(&mesh.component_references)?, 0, 1);
                    let progress = ui_wgpu::wgpu::mesh3d_move_component_references(token, &mut mesh.component_references, grant)?;
                    if !progress.fits(grant) { return Err(ui_wgpu::wgpu::Mesh3dFault::ByteCapacity); }
                }
                self.phase = WorldPlaceholderMeshPhase::AdmitProvenance;
            }
            WorldPlaceholderMeshPhase::AdmitProvenance => {
                let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, 0, 0, 1);
                let step = ui_wgpu::wgpu::mesh3d_component_admission_step(self.token()?, grant)?;
                if !step.ownership.fits(grant) { return Err(ui_wgpu::wgpu::Mesh3dFault::Closing); }
                if step.complete { self.phase = WorldPlaceholderMeshPhase::Seal; }
            }
            WorldPlaceholderMeshPhase::Seal => {
                let lease = mesh3d_seal(self.token()?)?;
                self.owner = WorldPlaceholderOwner::Ready(lease);
                self.phase = WorldPlaceholderMeshPhase::Publish;
            }
            WorldPlaceholderMeshPhase::Publish => {
                let grant=self.source_retirement_grant(4096)?;
                if !matches!(self.retire_source_step(grant)?,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){return Ok(WorldPlaceholderMeshStep::Pending)}
                let WorldPlaceholderOwner::Ready(lease) = self.owner else { return Err(ui_wgpu::wgpu::Mesh3dFault::Stale) };
                self.owner = WorldPlaceholderOwner::Empty;
                return Ok(WorldPlaceholderMeshStep::Ready(std::mem::take(&mut self.key), lease,self.appearance.take()));
            }
        }
        Ok(WorldPlaceholderMeshStep::Pending)
    }

    pub fn source_retirement_grant(&self,copy_bytes:usize)->Result<semio_framework_value::retained_clone::RetainedCloneGrant,ui_wgpu::wgpu::Mesh3dFault> {
        let Some(owner)=self.source_retirement.as_ref() else{return Ok(world_mesh_ownership_grant(0,0,0,usize::from(matches!(self.source,WorldMeshInput::Inline(_)))))};
        owner.next_copy_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?;
        Ok(world_mesh_ownership_grant(copy_bytes,owner.next_capacity_byte_demand(copy_bytes).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?,owner.next_release_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?,owner.next_depth_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?))
    }

    pub fn retire_source_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ui_wgpu::wgpu::Mesh3dFault> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        if let Some(owner)=self.source_retirement.as_mut(){
            let step=owner.step(grant).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?;
            let progress=match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};
            if !progress.fits(grant){return Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}
            if matches!(step,RetainedCloneStep::Complete(_)){
                if !owner.terminal_is_empty(){return Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}
                self.source_retirement=None;
            }
            return Ok(step)
        }
        if !matches!(self.source,WorldMeshInput::Inline(_)){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))}
        if grant.maximum_items==0 || grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()))}
        let WorldMeshInput::Inline(mesh)=std::mem::replace(&mut self.source,WorldMeshInput::Placeholder(WorldPlaceholderKind::Plane)) else{unreachable!()};
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(mesh){
            Ok(owner)=>{self.source_retirement=Some(owner);Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..RetainedCloneProgress::default()}))},
            Err((_,mesh))=>{self.source=WorldMeshInput::Inline(mesh);Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}
        }
    }

    fn retire_texture_job(&mut self)->Result<bool,ui_wgpu::wgpu::Mesh3dFault> {
        if let Some(owner)=self.texture_retirement.as_mut(){
            let copy=owner.next_copy_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?;
            let grant=world_mesh_ownership_grant(copy,owner.next_capacity_byte_demand(copy).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?,owner.next_release_byte_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?,owner.next_depth_demand().map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?);
            let step=owner.step(grant).map_err(|_|ui_wgpu::wgpu::Mesh3dFault::Closing)?;
            let receipt=match step{semio_framework_value::retained_clone::RetainedCloneStep::Progress(receipt)|semio_framework_value::retained_clone::RetainedCloneStep::Complete(receipt)=>receipt};
            if !receipt.fits(grant){return Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}
            if matches!(step,semio_framework_value::retained_clone::RetainedCloneStep::Complete(_)){self.texture_retirement=None;return Ok(true)}
            return Ok(false)
        }
        let Some(job)=self.texture_job.take() else{return Ok(true)};
        match semio_framework_value::retirement::controlled::ControlledRetirement::new(job){Ok(owner)=>{self.texture_retirement=Some(owner);Ok(false)},Err((_,job))=>{self.texture_job=Some(job);Err(ui_wgpu::wgpu::Mesh3dFault::Closing)}}
    }

    pub fn close_step(&mut self) -> bool {
        self.texture_cancelled.store(true,std::sync::atomic::Ordering::Release);
        if self.texture_submission.take().is_some(){self.texture_worker=None;return false}
        if let Some(worker)=self.texture_worker.as_ref(){let Ok(mut output)=worker.try_lock()else{return false};let Some(result)=output.take()else{return false};self.texture_image=result;drop(output);self.texture_worker=None;return false}
        if self.texture_bytes.pop().is_some(){return false}
        if let Some(name)=self.texture_name.as_mut(){if name.pop().is_some(){return false}self.texture_name=None;return false}
        match self.source_retirement_grant(4096).and_then(|grant|self.retire_source_step(grant)){
            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(_))=>{},
            Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(_))=>return false,
            Err(_)=>{self.faulted=true;return false}
        }
        if let Some(appearance)=self.appearance.as_mut(){if !appearance.close_step(){return false}self.appearance=None;return false}
        if self.primitives.pop().is_some()||self.textures.pop().is_some(){return false}
        if let Some(job)=self.texture_job.as_mut(){job.cancel();}
        match self.retire_texture_job(){Ok(true)=>{},Ok(false)=>return false,Err(_)=>{self.faulted=true;return false}}
        if let Some(image)=self.texture_image.as_mut(){if image.pixels.pop().is_some(){return false}self.texture_image=None;return false}

        match self.owner {
            WorldPlaceholderOwner::Writing(token) => {
                if !self.close_started {
                    match mesh3d_abort(token) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_wgpu::wgpu::mesh3d_abort_demands(token, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_abort_step(token, grant)?) }) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Ready(lease) => {
                if !self.close_started {
                    match mesh3d_begin_close(lease) {
                        Ok(()) | Err(ui_wgpu::wgpu::Mesh3dFault::Closing) => self.close_started = true,
                        Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                        Err(_) => return false,
                    }
                    return false;
                }
                match ui_wgpu::wgpu::mesh3d_close_demands(lease, ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES).and_then(|demand| { let grant = world_mesh_ownership_grant(ui_wgpu::wgpu::kernel_3d_scene::MESH3D_PAGE_BYTES, demand.capacity_bytes, demand.release_bytes, demand.depth); world_mesh_ownership_complete(grant, mesh3d_close_step(lease, grant)?) }) {
                    Ok(true) | Err(ui_wgpu::wgpu::Mesh3dFault::Stale) => self.owner = WorldPlaceholderOwner::Empty,
                    Ok(false) | Err(_) => return false,
                }
                false
            }
            WorldPlaceholderOwner::Empty => {
                if self.key.pop().is_some() {
                    return false;
                }
                true
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self.owner {
            WorldPlaceholderOwner::Writing(_) => false,
            WorldPlaceholderOwner::Ready(lease) => mesh3d_terminal_is_empty(lease),
            WorldPlaceholderOwner::Empty => self.key.is_empty() && self.source_retirement.is_none() && !matches!(self.source,WorldMeshInput::Inline(_)) && self.appearance.is_none() && self.primitives.is_empty() && self.textures.is_empty() && self.texture_job.is_none() && self.texture_retirement.is_none() && self.texture_image.is_none() && self.texture_worker.is_none() && self.texture_submission.is_none() && self.texture_bytes.is_empty() && self.texture_name.is_none(),
        }
    }
}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for WorldPlaceholderMeshCursor {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty() || std::thread::panicking(), "placeholder mesh cursor reached Drop before its exact authority witness");
    }
}

pub fn placeholder_triangle_normal(triangle: [[f32; 3]; 3]) -> [f32; 3] {
    let ab = [triangle[1][0] - triangle[0][0], triangle[1][1] - triangle[0][1], triangle[1][2] - triangle[0][2]];
    let ac = [triangle[2][0] - triangle[0][0], triangle[2][1] - triangle[0][1], triangle[2][2] - triangle[0][2]];
    placeholder_normalize3([ab[1] * ac[2] - ab[2] * ac[1], ab[2] * ac[0] - ab[0] * ac[2], ab[0] * ac[1] - ab[1] * ac[0]])
}

pub fn placeholder_triangle(kind: WorldPlaceholderKind, triangle: u32) -> [[f32; 3]; 3] {
    match kind {
        WorldPlaceholderKind::Box => {
            let faces = [
                ([-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]),
                ([0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5]),
                ([-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5]),
                ([-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, 0.5]),
                ([0.5, -0.5, 0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5]),
                ([-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5], [-0.5, 0.5, 0.5], [-0.5, 0.5, -0.5]),
            ];
            let (a, b, c, d) = faces[(triangle / 2) as usize];
            if triangle.is_multiple_of(2) {
                [a, b, c]
            } else {
                [a, c, d]
            }
        }
        WorldPlaceholderKind::Plane => {
            if triangle == 0 {
                [[-0.5, 0.0, -0.5], [0.5, 0.0, -0.5], [0.5, 0.0, 0.5]]
            } else {
                [[-0.5, 0.0, -0.5], [0.5, 0.0, 0.5], [-0.5, 0.0, 0.5]]
            }
        }
        WorldPlaceholderKind::Cylinder => {
            let segment = triangle / 4;
            let a0 = segment as f32 / 16.0 * std::f32::consts::TAU;
            let a1 = (segment + 1) as f32 / 16.0 * std::f32::consts::TAU;
            let p00 = [0.5 * a0.cos(), -0.5, 0.5 * a0.sin()];
            let p01 = [0.5 * a1.cos(), -0.5, 0.5 * a1.sin()];
            let p10 = [0.5 * a0.cos(), 0.5, 0.5 * a0.sin()];
            let p11 = [0.5 * a1.cos(), 0.5, 0.5 * a1.sin()];
            match triangle % 4 {
                0 => [p00, p01, p11],
                1 => [p00, p11, p10],
                2 => [[0.0, -0.5, 0.0], p01, p00],
                _ => [[0.0, 0.5, 0.0], p10, p11],
            }
        }
        WorldPlaceholderKind::Cone => {
            let segment = triangle / 2;
            let a0 = segment as f32 / 16.0 * std::f32::consts::TAU;
            let a1 = (segment + 1) as f32 / 16.0 * std::f32::consts::TAU;
            let p0 = [0.5 * a0.cos(), 0.0, 0.5 * a0.sin()];
            let p1 = [0.5 * a1.cos(), 0.0, 0.5 * a1.sin()];
            if triangle.is_multiple_of(2) {
                [[0.0, 1.0, 0.0], p1, p0]
            } else {
                [[0.0, 0.0, 0.0], p0, p1]
            }
        }
        WorldPlaceholderKind::Icosphere => {
            let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
            let vertices = [[-1.0, t, 0.0], [1.0, t, 0.0], [-1.0, -t, 0.0], [1.0, -t, 0.0], [0.0, -1.0, t], [0.0, 1.0, t], [0.0, -1.0, -t], [0.0, 1.0, -t], [t, 0.0, -1.0], [t, 0.0, 1.0], [-t, 0.0, -1.0], [-t, 0.0, 1.0]];
            let faces =
                [[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
            let face = faces[(triangle / 4) as usize];
            let a = placeholder_normalize3(vertices[face[0]]);
            let b = placeholder_normalize3(vertices[face[1]]);
            let c = placeholder_normalize3(vertices[face[2]]);
            let ab = placeholder_normalize3([(a[0] + b[0]) * 0.5, (a[1] + b[1]) * 0.5, (a[2] + b[2]) * 0.5]);
            let bc = placeholder_normalize3([(b[0] + c[0]) * 0.5, (b[1] + c[1]) * 0.5, (b[2] + c[2]) * 0.5]);
            let ca = placeholder_normalize3([(c[0] + a[0]) * 0.5, (c[1] + a[1]) * 0.5, (c[2] + a[2]) * 0.5]);
            let triangle = match triangle % 4 {
                0 => [a, ab, ca],
                1 => [b, bc, ab],
                2 => [c, ca, bc],
                _ => [ab, bc, ca],
            };
            triangle.map(|vertex| placeholder_scale3(vertex, 0.12))
        }
    }
}

//#region WorldMeshBuffers
pub type WorldMeshBuffers=semio_framework::MeshData;

trait WorldMeshSurface {
    fn position(&self,vertex:u32)->[f32;3];
    fn normal(&self,vertex:u32)->[f32;3];
}

impl WorldMeshSurface for WorldMeshBuffers {
    fn position(&self,vertex:u32)->[f32;3] {let at=vertex as usize*3;[self.positions.get(at).copied().unwrap_or(0.0),self.positions.get(at+1).copied().unwrap_or(0.0),self.positions.get(at+2).copied().unwrap_or(0.0)]}
    fn normal(&self,vertex:u32)->[f32;3] {let at=vertex as usize*3;[self.normals.get(at).copied().unwrap_or(0.0),self.normals.get(at+1).copied().unwrap_or(0.0),self.normals.get(at+2).copied().unwrap_or(1.0)]}
}

pub fn placeholder_normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

pub fn placeholder_scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

//#region 🧵️WorldInteractionTransaction
pub fn world_mesh_vertex(mesh: Mesh3dLease, index: u32) -> Option<Vec3> {
    let point = mesh.vec3(Mesh3dField::Positions, index).ok()?;
    Some(Vec3::new(point[0], point[1], point[2]))
}

pub fn world_mesh_triangle(mesh: Mesh3dLease, triangle: u32) -> Option<[u32; 3]> {
    let base = triangle.checked_mul(3)?;
    Some([mesh.u32(Mesh3dField::Indices, base).ok()?, mesh.u32(Mesh3dField::Indices, base + 1).ok()?, mesh.u32(Mesh3dField::Indices, base + 2).ok()?])
}

pub fn world_mesh_component_id(mesh: Mesh3dLease, field: Mesh3dField, index: u32) -> u32 {
    mesh.u32(field, index).unwrap_or(if field == Mesh3dField::VertexIds { u32::MAX } else { index })
}

pub const WORLD_COMPONENT_TARGET_BYTE_CAPACITY: usize = WORLD_INTERACTION_ID_BYTE_CAPACITY + 1 + 6 + 1 + 10 + 1 + 64 + 1 + 20 + 1 + 64;

pub struct WorldComponentAddress<'a> {
    pub object: &'a str,
    pub group: u32,
    pub field: Mesh3dField,
    pub source: Option<(ComponentSource3d, u64)>,
}

impl<'a> WorldComponentAddress<'a> {
    pub fn parse(id: &'a str, kind: &str) -> Option<Self> {
        if id.len() > WORLD_COMPONENT_TARGET_BYTE_CAPACITY { return None; }
        let (address, source) = id.split_once('~').map_or((id, None), |(address, source)| (address, Some(source)));
        let (prefix, group) = address.rsplit_once('.')?;
        let (object, actual_kind) = prefix.rsplit_once('.')?;
        if actual_kind != kind || group.is_empty() || (group.len() > 1 && group.starts_with('0')) || !group.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
        let field = match kind { "vertex" => Mesh3dField::VertexIds, "edge" => Mesh3dField::EdgeIds, "face" => Mesh3dField::FaceIds, _ => return None };
        let source = match source {
            None => None,
            Some(source) => {
                let mut fields = source.split('~');
                let handle = fields.next()?; let label = fields.next()?; let revision = fields.next()?;
                if fields.next().is_some() || label.is_empty() || label.starts_with('0') || label.len() > 20 || !label.bytes().all(|byte| byte.is_ascii_digit()) { return None; }
                Some((ComponentSource3d::new(handle, revision)?, label.parse().ok()?))
            }
        };
        Some(Self { object, group: group.parse().ok()?, field, source })
    }

    pub(crate) fn matches_source(&self, scene_state: &WorldSceneState, source: Option<ComponentSource3d>, mesh_id: &str) -> bool {
        match (self.source, source) {
            (None, None) => true,
            (Some((expected, label)), Some(actual)) if expected == actual => scene_state.meshes.get(mesh_id).is_some_and(|mesh| mesh.component_label(self.field, self.group).ok().flatten() == Some(label)),
            _ => false,
        }
    }

    pub fn matches_registry(&self, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, entry: &WorldInteractionObjectSlot) -> bool {
        let Some(mesh) = entry.mesh.and_then(|mesh| interaction_state.interaction_meshes.resolve(mesh)) else { return false; };
        self.matches_source(scene_state, entry.component_source, mesh.id.as_str())
    }
}

//#region 🧭️WorldCameraSync
/** 🧭️ A world pane's camera-report debt, and the parity contract behind it.
 *
 * ⚖️ React's `WorldOrbitGated` (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`) reports through three's
 * `OrbitControls` events, and the two of them do NOT fire on the same condition:
 *
 * - `end` is dispatched on EVERY pointer-up over the canvas, whatever the button and whether or not
 *   any camera state moved (`three/examples/jsm/controls/OrbitControls.js`'s `onPointerUp`, which
 *   calls `dispatchEvent(_endEvent)` unconditionally once the last pointer lifts). `onEnd` always
 *   calls `reportCamera`, so a plain left click and a plain right click each owe ONE `setCamera` —
 *   which is exactly what the React reference journals for the `pick-instance` and `context-menu`
 *   steps, neither of which moves the camera at all
 *   (`🗑️generated/parity-run-2/steps.json`).
 * - `start` is dispatched only when a mouse button actually claimed a navigation state, so
 *   `navigationSnapshotRef` is set only for a real gesture, and `onNavigationGestures` — the
 *   `noteWorldNavigation` dispatch — additionally requires the before/after diff to classify
 *   non-empty. The same run journals `noteWorldNavigation` for `pan-drag` and `zoom-wheel` and NOT
 *   for `orbit-drag`, because the left button is bound to nothing
 *   (`resolveWorldOrbitMouseButtonsIdle`: `LEFT: null`, `MIDDLE: PAN`, right → pan on Shift and
 *   orbit on Alt) — so the left drag marquees and the camera never moves.
 *
 * ⏱️ `reportCamera`'s dispatch is TRAILING-DEBOUNCED (`CAMERA_SYNC_DEBOUNCE_MS`), so a whole drag or
 * a whole wheel burst publishes ONE `setCamera`. This lane owes no timer to reproduce that: an
 * interaction intent queue that has drained IS the settle point, so the debt is paid once the last
 * intent of the gesture is answered, and a burst coalesces by construction. `noteWorldNavigation` is
 * NOT debounced in React, so it is published first — the order the reference journals
 * (`noteWorldNavigation` then `setCamera`).
 *
 * 🩸️ Before this, a nav drag published one `setCamera` per pointer MOVE — eight per gesture on the
 * parity probe's eight-sample drag — and a gesture that moved nothing published none at all; there
 * was no `noteWorldNavigation` anywhere in the target
 * (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w10a-world-interaction-journal-parity.md`). */
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WorldCameraSync {
    pub owed: bool,
    pub navigating: bool,
    pub start: Option<WorldNavigationSnapshot>,
    /// 🎯️ A settled camera owes ONE re-published hover: the scene moved under a pointer that did not,
    /// so what it is over changed. React gets this for free — r3f re-raycasts the moved scene and
    /// `dispatchInstanceHover` publishes again, which is the trailing `interactionHover` every one of
    /// the reference's five world steps ends on.
    pub hover_owed: bool,
    /// 🖱️ The last point a pointer intent named on this surface, for that re-published hover.
    pub at: Option<[f32; 2]>,
}

/// 📸️ The subset of camera state [`classify_world_navigation_gestures`] diffs before/after a
/// gesture — React's `WorldNavigationSnapshot`, whose `zoom` is the live frustum scale for a
/// parallel pane and a bare `1` for a perspective one (`captureNavigationSnapshot`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldNavigationSnapshot {
    pub position: [f32; 3],
    pub target: [f32; 3],
    pub zoom: f32,
    pub parallel: bool,
}
