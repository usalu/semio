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

#[path = "🫧️face-overlay-and-fit/🦀️.rs"]
mod face_overlay_and_fit;
pub use face_overlay_and_fit::*;

#[path = "🪄️placeholder-mesh-and-gumball/🦀️.rs"]
mod placeholder_mesh_and_gumball;
pub use placeholder_mesh_and_gumball::*;

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
