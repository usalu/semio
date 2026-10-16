//! 🎛️ World3d host: the composed `World3dState`, frame sync and render orchestration.
//!
//! Crate `semio-framework-os-infinite-world-host`; a downward producer of the infinite world family (wasi-excluded except tests).

#![cfg(any(test, not(all(target_arch = "wasm32", target_env = "p2"))))]

use semio_framework_os_infinite_world_model::{tool_run_trace, World3dBuildContext, World3dScalarFieldStatus, WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY, world_reference_width, world_scene_raster_pool, WorldDrawRebuildDescriptor, WorldDynamicFault, WorldInteractionState, WorldReferenceRecord, WorldSessionState};
use semio_framework_os_infinite_world_components::{world3d_offers_transform_gumball, World3dCameraBoundsCursor, World3dCameraFitOwner, World3dPreparedStatus, World3dSnapshotApplyCursor, WorldCameraState, WorldFaceOverlayMeshCursor, WorldFaceOverlayMeshStep, WorldFaceOverlayPhase, WorldGumballState, WorldOverlayState, WorldPlaceholderKind, WorldPlaceholderMeshCursor, WorldPlaceholderOwner, WorldSceneState, WorldSelectionState, WorldTraceState};
use semio_framework_os_infinite_world_cursors::{live_world3d_mesh_version, render_pick_viewport, world_merge_mode, world_pick_rect, WorldInteractionAuthorityStep};
use semio_framework_os_infinite_world_scene::{apply_terrain_style_if_changed, CELEBRATE_CONIC_SPIN_SECONDS, environment_light_dir, environment_scene_material, GUMBALL_PLANE_MESH, instance_chunk_visible, mesh_style_paint, MeshStyleKind, offer_missing_mesh_fetches, parse_color, publish_world3d_mesh_lease, reserve_world3d_asset_request, resolve_mesh_style, retain_ensured_world_draws, retain_ensured_world_material_draws, retire_world_mesh, scene_lod, sync_mesh_pool, sync_world3d_brush_mesh_feedback, update_visible_chunks, world3d_scene_digest, world3d_style_paint, world_instance_style, WORLD_LOD_EPSILON, WorldAssetRequestKind, WorldAssetsState, WorldTerrainState};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_scene::{environment_scene_shadow, sort_world3d_translucent_material_draws, world3d_shadow_role, World3dShadowGeometry, World3dShadowProfile};
use semio_framework_os_infinite_world_bridge::{advance_world3d_view_revision, append_component_overlays, append_component_vertex_spheres, append_engagement_preview_lines, append_gumball_geometry, append_pick_target_hover_lines, append_world_reference_outline, apply_world3d_camera_fit, apply_world3d_pick_filter, begin_world3d_draw_rebuild, close_world3d_draw_rebuild_step, component_mode_active, face_component_mode_active, orbit_camera_action, publish_world3d_scene_bridge_snapshot, reference_underlay_upload, scene_mesh_awaits_its_asset, stage_world3d_scene_bridge, step_world3d_mesh_close, step_world3d_scene_bridge_close, sync_world3d_projection_content_frame, sync_world3d_scene_document_lanes, sync_world3d_scene_fit, sync_world3d_scene_selection, VERTEX_MARKER_MESH, world3d_apply_scalar_field_colors, WORLD3D_BRIDGE_MESH_WRITES_PER_STEP, world3d_camera_fit_key, world3d_camera_fit_work_pending, world3d_draw_rebuild_admit_draw, world3d_draw_rebuild_admit_instance, world3d_draw_rebuild_admit_retained_instance, world3d_draw_rebuild_seal, world3d_draw_rebuild_terminal_is_empty, WORLD3D_RENDERED_OUTLINE_SCALE, world3d_scalar_field_fits, world3d_scene_bridge_has_pages, world3d_scene_instance_interactions_are_bounded, world3d_sub_element_paint_native, world3d_tone_rgba, World3dCameraFitStep, World3dSceneBridgePhase, World3dSceneBridgeStep, World3dSceneCameraRecord, World3dSceneInstanceEntry, World3dSceneMeshEntry, World3dSnapshotApplyStep, world_raster_upload, world_reference_plane_size, world_reference_raster_ready, world_reference_visual, WorldBridgeState, WorldMeshDigestCursor, WorldRetirementState};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_bridge::{gumball_commit_action, is_click_gesture, pick_gumball_handle_at, reset_gumball_preview};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_bridge::world3d_standard_material_for_mesh;
use semio_framework_os_infinite_world_interaction::{append_box_wireframe, append_vortex_arrow_draws, begin_world_placeholder_mesh, brush_preview_mesh_id, declare_scene_mesh_source, environment_authored_material, environment_clear_color, environment_explicit_clear_color, environment_scene_lighting, ghost_mesh_id, mark_world_dynamic_fault, preview_scale, retained_gumball_preview_model, step_world_placeholder_mesh, step_world_placeholder_mesh_batch, sync_terrain, world_interaction_active_census, world_marquee_overlay_points, WorldAuthorityState};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_interaction::{marquee_select_action, pick_instance_at, pick_paint_hit, pick_select_action, pick_surface_at, pick_vortex_at, start_gumball_drag};
use ui_wgpu::wgpu::{ActionDescriptor, Camera3d, camera_grid_fade_distance, CameraProjection3d, ComponentSource3d, HitKind, HitTarget, Instance3d, LineVertex3d, lod_grid_step_world, marquee_is_crossing_from_path, Mat4, Mesh3dLease, OrbitController, PointerModifiers, quat_from_basis, Rect, Rgba, SceneColorSource3d, SceneDraw3d, Theme, UiComponentSceneNode, Vec3, world3d_snapshot_claim_draw_permit, world3d_snapshot_with_page, World3dPresentation, World3dPresentationClear, World3dSnapshotFault, World3dSnapshotPageKind};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{aabb_intersects_frustum, directional_shadow_frustum_planes, frustum_planes, LineDraw3d, paint_selection_marquee, ProceduralGrid3d, SceneInstanceMaterial3d, SceneMaterialAlpha3d, SceneMaterialDraw3d, SceneMaterialKind3d, ScenePass3d, SceneSectionRole3d, SceneShadowRole3d, SceneViewportMask3d, TexturedDraw3d, TexturedInstance3d, transform_aabb};
use semio_framework_tool_run::ToolRunTraceCursor;
use semio_framework_ui_viewport::{Viewport3dProjectionFramePolicy, Viewport3dProjectionMode};
#[cfg(any(test, feature = "testkit"))]
use ui_wgpu::wgpu::gizmo;
use serde_json::json;

pub struct World3dState {
    /// Surface identity, bounds, domain binding, utilities and presentation of one 3D surface.
    pub session: WorldSessionState,
    /// Orbit rig, projection, camera fit and camera sync.
    pub camera: WorldCameraState,
    /// Hover, selection, marquee and pick-target state.
    pub selection: WorldSelectionState,
    /// Gumball, brush preview, catalogue drop preview and relocate drag.
    pub gumball: WorldGumballState,
    /// Interaction mesh/object registries and instance interaction ids.
    pub interaction: WorldInteractionState,
    /// The interaction authority state machine with its active cursor or job.
    pub authority: WorldAuthorityState,
    /// The in-flight dynamic retirement cursor.
    pub retirement: WorldRetirementState,
    /// Resident meshes, draws, scene records (vortices, references, lod, environment), placeholder meshes and dynamic retirement slots.
    pub scene: WorldSceneState,
    /// Terrain style, tiles, session and build.
    pub terrain: WorldTerrainState,
    /// Face overlay build, sub-element paint, scalar field and prepared status.
    pub overlay: WorldOverlayState,
    /// Scene bridge, snapshot apply and scene digests.
    pub bridge: WorldBridgeState,
    /// Asset io authority, url bookkeeping and brush mesh announce.
    pub assets: WorldAssetsState,
    /// Tool run trace layer.
    pub trace: WorldTraceState,
}

impl World3dState {
    /// 🩺️ What this surface's mesh ingest actually holds, for the one `[TRACE] ` line the browser
    /// Worker can emit. `draws=0` on a surface whose `meshes_json` carries a solid is ambiguous
    /// between "the bridge never ran", "the bridge ran and produced nothing" and "the draws exist
    /// but no frame has painted them since" — this separates the three
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn ingest_census(&self) -> String {
        format!(
            "state-draws={} state-instances={} state-meshes={} bridge={} bridge-lease={} apply={} apply-page={}/{} apply-item={} apply-faulted={} rebuild={} retired-draws={} retirement={} blocked={} snapshot-lease={} fault={:?}",
            self.scene.draws.len,
            self.scene.draws.iter().map(|draw| draw.instances.len()).sum::<usize>(),
            self.scene.meshes.len,
            self.bridge.scene_bridge.is_some(),
            self.bridge.scene_bridge_lease.is_some(),
            self.bridge.snapshot_apply.is_some(),
            self.bridge.snapshot_apply.as_ref().map_or(0, |cursor| cursor.page),
            self.bridge.snapshot_apply.as_ref().map_or(0, |cursor| cursor.lease.page_count),
            self.bridge.snapshot_apply.as_ref().map_or(0, |cursor| cursor.item),
            self.bridge.snapshot_apply.as_ref().is_some_and(|cursor| cursor.faulted),
            self.scene.draw_rebuild.is_some(),
            self.scene.retired_draws.is_some(),
            self.retirement.dynamic_retirement.is_some(),
            self.scene.dynamic_blocked_owner.is_some(),
            self.bridge.snapshot_lease.is_some(),
            self.session.snapshot_fault,
        )
    }

    /// 🩺️🧊️ One row per RESIDENT mesh — its key, its published vertex/index counts and its local
    /// bounds. `state-meshes=5` is ambiguous between "five real meshes" and "five empty leases", and a
    /// draw that submits an empty GPU mesh paints exactly nothing while every count above it reads
    /// healthy (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3d-world3d-glb-url-lane.md`).
    pub fn mesh_geometry_census(&self) -> String {
        let mut rows = String::from("meshes=[");
        for (index, key) in self.scene.meshes.keys().enumerate() {
            let Some(mesh) = self.scene.meshes.get(key) else { continue };
            let schema = mesh.schema().ok();
            let bounds = mesh.aabb().ok();
            if index > 0 {
                rows.push(',');
            }
            rows.push_str(&format!(
                "{key}:v{}/i{}{}",
                schema.map_or(0, |schema| schema.vertices),
                schema.map_or(0, |schema| schema.indices),
                bounds.map_or_else(String::new, |(min, max)| format!("@[{:.2},{:.2},{:.2}]..[{:.2},{:.2},{:.2}]", min[0], min[1], min[2], max[0], max[1], max[2]))
            ));
        }
        rows.push(']');
        rows
    }

    /// 🕹️ What this surface's INTERACTION authority actually holds, for the one `[TRACE] ` line the
    /// browser Worker can emit.
    ///
    /// ⚖️ "hover, click and wheel changed nothing" is ambiguous between five different stops — no
    /// intent ever arrived, the intent sat behind a generation the pump never asked for, the ray
    /// found no triangle, the plan was built but never published, or the action was published and
    /// the guest never took it. `ingest_census` separates the MESH half; this separates the INPUT
    /// half (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    ///
    /// 🎥️ The surface's OWN camera, not the guest's `cameraJson`. A wheel or orbit turns the local
    /// rig first (`publish_world3d_plan_step`'s `publish_with`) and only then asks the guest for the
    /// same pose, so the local rig is the honest witness that the gesture was understood — the
    /// scene's `cameraJson` cannot move until the guest answers.
    pub fn interaction_census(&self) -> String {
        let Some(authority) = self.authority.interaction_authority.as_ref() else {
            return "authority=none".into();
        };
        let front = authority.queue.front().map_or_else(
            || "front=none".to_string(),
            |intent| {
                format!(
                    "front={:?}/g{} button={} down={} at={},{} d={},{} wheel={} mods={}{}{}{}",
                    intent.phase,
                    intent.generation,
                    intent.button,
                    intent.down,
                    intent.x,
                    intent.y,
                    intent.dx,
                    intent.dy,
                    intent.delta,
                    if intent.shift { "s" } else { "-" },
                    if intent.ctrl { "c" } else { "-" },
                    if intent.alt { "a" } else { "-" },
                    if intent.meta { "m" } else { "-" },
                )
            },
        );
        let active = authority.active.as_ref().map_or_else(|| "none".to_string(), world_interaction_active_census);
        let eye = self.camera.orbit.to_camera();
        format!(
            "camera=[{:.3},{:.3},{:.3}]->[{:.3},{:.3},{:.3}]/{:.1}deg {front} queued={} next-g={} active={active} blocked={} registry={} marquee={} gumball={} right-press={} faulted={} closing={} sync-owed={} sync-navigating={} sync-hover-owed={} revision={} objects-revision={} objects={} draws={} utility={} granularity={} mode={} bounds={}x{}+{},{} pick={}x{}+{},{} hover={:?} selected={}",
            eye.position.x,
            eye.position.y,
            eye.position.z,
            eye.target.x,
            eye.target.y,
            eye.target.z,
            eye.fov_y.to_degrees(),
            authority.queue.len,
            authority.next_generation,
            authority.blocked.is_some(),
            authority.registry.is_some(),
            authority.marquee.is_some(),
            authority.gumball.is_some(),
            authority.right_press.is_some(),
            authority.faulted,
            authority.closing,
            self.camera.camera_sync.owed,
            self.camera.camera_sync.navigating,
            self.camera.camera_sync.hover_owed,
            self.session.interaction_revision,
            self.interaction.interaction_objects.revision,
            self.interaction.interaction_objects.instance_len,
            self.scene.draws.len,
            self.session.active_utility,
            self.selection.granularity,
            self.selection.interaction_mode,
            self.session.bounds.w,
            self.session.bounds.h,
            self.session.bounds.x,
            self.session.bounds.y,
            self.session.pick_bounds.w,
            self.session.pick_bounds.h,
            self.session.pick_bounds.x,
            self.session.pick_bounds.y,
            self.selection.local_hover_id,
            self.selection.selected_ids.len(),
        )
    }

    pub fn new(surface_id: String, controller_id: String) -> Self {
        Self {
            session: WorldSessionState::new(surface_id, controller_id),
            camera: WorldCameraState::new(),
            selection: WorldSelectionState::new(),
            gumball: WorldGumballState::new(),
            interaction: WorldInteractionState::new(),
            authority: WorldAuthorityState::new(),
            retirement: WorldRetirementState::new(),
            scene: WorldSceneState::new(),
            terrain: WorldTerrainState::new(),
            overlay: WorldOverlayState::new(),
            bridge: WorldBridgeState::new(),
            assets: WorldAssetsState::new(),
            trace: WorldTraceState::new(),
        }
    }

    /// 📤️ Creates an isolated render owner at the revision of the decoded asset handed to it.
    pub fn request_owned(surface_id: String, controller_id: String, revision: u64) -> Self {
        let mut state = Self::new(surface_id, controller_id);
        state.interaction_revision = revision;
        state
    }

    pub fn dynamic_retirement_is_idle(&self) -> bool {
        self.retirement.dynamic_retirement.is_none()
    }

    /// 🌍️ Selects the language the modelling overlays (annotations, legend, their accessible text) are written in: German, else English.
    pub fn set_modelling_locale(&mut self, german: bool) {
        self.session.modelling_locale = if german { "de" } else { "en" };
    }

    /// 🌡️ Whether the scene's scalar field painted the mesh it names.
    pub fn scalar_field_status(&self) -> World3dScalarFieldStatus {
        self.overlay.scalar_field_status
    }

    /// 🚨️ The last snapshot fault this surface recorded, for a host that reports why a world window
    /// went dark instead of guessing. `None` once a snapshot published, and `None` while a staged
    /// scene bridge is still building.
    pub fn snapshot_fault(&self) -> Option<World3dSnapshotFault> {
        self.session.snapshot_fault
    }

    /// 🔺️ The published geometry one draw's `mesh_key` resolves to — the lease `render_world_3d`
    /// itself hands to `World3dBuildContext::ensure_mesh`, so a host can read back the triangle and
    /// vertex counts it is actually drawing.
    pub fn mesh_lease(&self, mesh_key: &str) -> Option<Mesh3dLease> {
        self.scene.meshes.get(mesh_key).copied()
    }
}

#[cfg(not(any(test, feature = "testkit")))]
impl Drop for World3dState {
    fn drop(&mut self) {
        assert!(world3d_dynamic_retirement_terminal_is_empty(self) || std::thread::panicking(), "World3dState reached Drop before retained dynamic owners reached terminal empty");
    }
}
//#endregion World3dState

/// 📦️ Advances at most one retained draw transition or one transformed mesh-AABB corner. This is
/// Three's `Box3.setFromObject(instancesGroup)` expressed over the resident WGPU draw registry.
pub fn step_world3d_camera_fit(state: &mut World3dState) -> World3dCameraFitStep {
    if state.retirement.dynamic_retirement.is_some() {
        state.camera.fit_bounds_cursor = None;
        return World3dCameraFitStep::Idle;
    }
    let Some(request) = state.camera.fit_request else { return World3dCameraFitStep::Idle };
    let owner = World3dCameraFitOwner { revision: request.revision, camera_digest: state.camera.scene_camera_digest };
    if state.camera.fit_owner != Some(owner) {
        state.camera.fit_owner = Some(owner);
        state.camera.fit_bounds_cursor = None;
        state.camera.camera_user_moved = false;
    }
    let key = world3d_camera_fit_key(owner, request.bounds);
    if state.camera.fit_applied_key == Some(key) || state.camera.camera_user_moved {
        state.camera.fit_bounds_cursor = None;
        return World3dCameraFitStep::Idle;
    }
    if state.bridge.scene_bridge.is_some()
        || state.bridge.snapshot_apply.is_some()
        || state.scene.draw_rebuild.is_some()
        || state.scene.retired_draws.is_some()
        || state.bridge.scene_bridge_lease.is_some_and(|lease| state.bridge.snapshot_lease != Some(lease))
    {
        return World3dCameraFitStep::Pending;
    }
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    if let Some((minimum, maximum)) = request.bounds {
        return apply_world3d_camera_fit(&mut state.session, &mut state.camera, &mut state.scene, key, request, minimum, maximum);
    }
    let mut cursor = state.camera.fit_bounds_cursor.take().unwrap_or_else(|| World3dCameraBoundsCursor::new(key, state.scene.draw_generation, state.scene.geometry_generation));
    if cursor.key != key || cursor.draw_generation != state.scene.draw_generation || cursor.geometry_generation != state.scene.geometry_generation {
        let completed_empty_generation = cursor.empty_generation();
        cursor = World3dCameraBoundsCursor::new(key, state.scene.draw_generation, state.scene.geometry_generation);
        if !completed_empty_generation {
            state.camera.fit_bounds_cursor = Some(cursor);
            return World3dCameraFitStep::Stale;
        }
    }
    if usize::from(cursor.draw) >= usize::from(state.scene.draws.len) {
        if cursor.any {
            return apply_world3d_camera_fit(&mut state.session, &mut state.camera, &mut state.scene, key, request, cursor.minimum, cursor.maximum);
        }
        cursor.draw = u16::MAX;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Idle;
    }
    let Some(draw) = state.scene.draws.get(usize::from(cursor.draw)) else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let Some(mesh) = state.scene.meshes.get(&draw.mesh_key).copied() else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let Ok((local_minimum, local_maximum)) = mesh.aabb() else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let Some(instance) = draw.instances.get(cursor.instance as usize) else {
        cursor.draw += 1;
        cursor.instance = 0;
        cursor.corner = 0;
        state.camera.fit_bounds_cursor = Some(cursor);
        return World3dCameraFitStep::Pending;
    };
    let corner = cursor.corner;
    let rendered_scale = if state.selection.show_edges
        && !state.selection.provisional_instance_ids.contains(&instance.id)
        && mesh.schema().is_ok_and(|schema| schema.edges == 0)
    {
        WORLD3D_RENDERED_OUTLINE_SCALE
    } else {
        1.0
    };
    let point = instance.model.transform_point(Vec3::new(
        (if corner & 1 == 0 { local_minimum[0] } else { local_maximum[0] }) * rendered_scale,
        (if corner & 2 == 0 { local_minimum[1] } else { local_maximum[1] }) * rendered_scale,
        (if corner & 4 == 0 { local_minimum[2] } else { local_maximum[2] }) * rendered_scale,
    ));
    if point.x.is_finite() && point.y.is_finite() && point.z.is_finite() {
        cursor.expand(point);
    }
    cursor.corner += 1;
    if cursor.corner == 8 {
        cursor.corner = 0;
        cursor.instance += 1;
    }
    state.camera.fit_bounds_cursor = Some(cursor);
    World3dCameraFitStep::Pending
}

/// 🌉️ One budgeted turn of the mesh-wire → snapshot bridge. Drives the staged payload through
/// parse → retained attribute candidates → per-mesh publication → snapshot page admission, then hands the sealed lease to
/// [`sync_world3d_state`], which feeds it to the very same apply ladder a typed producer's own
/// snapshot rides.
///
/// 📐️ The delivered spec seeds the framing plane only while the pane holds none of its
/// own — React's `cameraState.projectionSpec ?? externalPendingProjectionSpec` precedence
/// (`🌐️World3dHost/🟦️.tsx`), the same rule the snapshot apply keeps for the family.
pub fn step_world3d_scene_bridge(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> World3dSceneBridgeStep {
    if context.should_yield() {
        return World3dSceneBridgeStep::Pending;
    }
    if state.bridge.scene_bridge_retired.is_some() && state.bridge.snapshot_apply.is_none() && state.bridge.scene_bridge_lease == state.bridge.snapshot_lease {
        step_world3d_scene_bridge_close(&mut state.bridge,context);
        context.consume_fuel(1);
        return World3dSceneBridgeStep::Pending;
    }
    if state.bridge.scene_bridge_close.is_some()||state.bridge.scene_bridge.as_ref().is_some_and(|cursor|matches!(cursor.phase,World3dSceneBridgePhase::Close)){
        step_world3d_scene_bridge_close(&mut state.bridge,context);
        context.consume_fuel(1);
        return World3dSceneBridgeStep::Pending;
    }
    let Some(mut cursor) = state.bridge.scene_bridge.take() else {
        return World3dSceneBridgeStep::Idle;
    };
    context.consume_fuel(1);
    match cursor.phase {
        World3dSceneBridgePhase::Parse => {
            if cursor.instances_json.len() > WORLD_INTERACTION_TOPOLOGY_BYTE_CAPACITY {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault;
            }
            cursor.meshes = semio_framework_pack_json::from_json_str::<Vec<World3dSceneMeshEntry>>(&cursor.meshes_json,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default();
            cursor.instances = serde_json::from_str::<Vec<World3dSceneInstanceEntry>>(&cursor.instances_json).unwrap_or_default();
            cursor.camera = serde_json::from_str::<World3dSceneCameraRecord>(&cursor.camera_json).ok();
            if let Some(camera) = cursor.camera.as_ref().filter(|_| !state.camera.projection_selected) {
                state.camera.projection_frame_policy = camera.projection_frame;
                if camera.projection_frame == Viewport3dProjectionFramePolicy::PreserveCamera {
                    state.camera.projection_frame_owed = false;
                    state.camera.projection_frame_key = None;
                    state.camera.projection_frame_zoom = None;
                }
            }
            if let Some(camera) = cursor.camera.as_ref().filter(|_| !state.camera.projection_selected) {
                if let Some(spec) = camera.projection {
                    state.camera.projection_spec = spec;
                    if cursor.camera_changed && camera.projection_frame == Viewport3dProjectionFramePolicy::Content {
                        state.camera.projection_frame_owed = true;
                        state.camera.projection_frame_key = None;
                        state.camera.projection_frame_zoom = None;
                        state.camera.camera_user_moved = false;
                    }
                }
            }
            cursor.meshes.retain(World3dSceneMeshEntry::names_a_mesh);
            for mesh in &mut cursor.meshes {
                if mesh.data.normals.len() != mesh.data.positions.len() && !mesh.data.attributes.values().any(|attribute| attribute.semantic == semio_framework::MeshAttributeSemantic::Normal) {
                    mesh.data.compute_normals();
                }
            }
            if !world3d_scene_instance_interactions_are_bounded(&cursor.instances) {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault;
            }
            cursor.instances.retain(|instance| cursor.meshes.iter().any(|mesh| mesh.id == instance.mesh_id));
            state.overlay.scalar_field_status=if cursor.scalar_field.is_some(){World3dScalarFieldStatus::Missing}else{World3dScalarFieldStatus::None};
            cursor.phase = World3dSceneBridgePhase::ScalarAttributes;
            state.bridge.scene_bridge = Some(cursor);
            World3dSceneBridgeStep::Pending
        }
        World3dSceneBridgePhase::ScalarAttributes=>{
            let Some(field)=cursor.scalar_field.as_ref()else{cursor.phase=World3dSceneBridgePhase::Meshes;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending};
            let Some(mesh)=cursor.meshes.get_mut(cursor.scalar_mesh_cursor)else{cursor.phase=World3dSceneBridgePhase::Meshes;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending};
            if mesh.id!=field.mesh_id{cursor.scalar_mesh_cursor+=1;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending;}
            if !world3d_scalar_field_fits(&mesh.data,field){state.overlay.scalar_field_status=World3dScalarFieldStatus::Mismatch;cursor.phase=World3dSceneBridgePhase::Meshes;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending;}
            if cursor.scalar_slot_cursor<mesh.data.attributes.slot_count(){
                let result=mesh.data.attributes.retire_slot_if(cursor.scalar_slot_cursor,|_,attribute|attribute.semantic!=semio_framework::MeshAttributeSemantic::Color,context.retained_grant());
                match result{Ok(progress)=>{context.consume_retained(progress).expect("original attribute move fits supplied caller grant");if progress.copied_items!=0{cursor.scalar_slot_cursor+=1;}},Err(_)=>{state.session.snapshot_fault=Some(World3dSnapshotFault::Capacity);cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault;}}
                state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending;
            }
            world3d_apply_scalar_field_colors(&mut mesh.data,field);
            state.overlay.scalar_field_status=World3dScalarFieldStatus::Applied;
            cursor.phase=World3dSceneBridgePhase::Meshes;
            state.bridge.scene_bridge=Some(cursor);
            World3dSceneBridgeStep::Pending
        }
        World3dSceneBridgePhase::Close=>unreachable!("original scene cursor close is drained before advancing"),
        World3dSceneBridgePhase::Meshes => {
            if state.scene.dynamic_mesh_close.is_some() || state.scene.dynamic_blocked_mesh.is_some() {
                step_world3d_mesh_close(&mut state.scene,context);
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            if state.scene.placeholder_build.is_some() {
                step_world_placeholder_mesh_batch(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, WORLD3D_BRIDGE_MESH_WRITES_PER_STEP);
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            let Some(entry) = cursor.meshes.get(cursor.mesh_cursor) else {
                cursor.phase = World3dSceneBridgePhase::Pages;
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            };
            if !entry.has_inline_geometry() {
                let (id, url, kind) = (entry.id.clone(), entry.url.clone(), entry.kind.clone());
                declare_scene_mesh_source(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, &id, url.as_deref(), kind.as_deref());
                cursor.mesh_cursor += 1;
                cursor.retiring_mesh = false;
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            let hashing=cursor.mesh_digest.get_or_insert_with(WorldMeshDigestCursor::default);
            let digest=match hashing.step(&entry.data){Ok(Some(digest))=>digest,Ok(None)=>{state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Pending},Err(_)=>{state.session.snapshot_fault=Some(World3dSnapshotFault::Capacity);cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);return World3dSceneBridgeStep::Fault}};
            if state.scene.scene_mesh_digests.get(&entry.id) == Some(&digest) && state.scene.meshes.contains_key(&entry.id) {
                cursor.mesh_cursor += 1;
                cursor.retiring_mesh = false;
                cursor.mesh_digest=None;
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            if state.scene.meshes.contains_key(&entry.id) && !cursor.retiring_mesh {
                cursor.retiring_mesh = true;
                retire_world_mesh(&mut state.scene, &entry.id.clone());
                state.bridge.scene_bridge = Some(cursor);
                return World3dSceneBridgeStep::Pending;
            }
            let id=entry.id.clone();
            let buffers=std::mem::take(&mut cursor.meshes[cursor.mesh_cursor].data);
            state.scene.placeholder_generation = state.scene.placeholder_generation.wrapping_add(1).max(1);
            match WorldPlaceholderMeshCursor::inline(&id, buffers, state.scene.placeholder_generation, state.session.interaction_revision) {
                Ok(build) => {
                    state.scene.placeholder_build = Some(build);
                    state.scene.scene_mesh_digests.insert(id, digest);
                    cursor.mesh_digest=None;
                    cursor.mesh_cursor += 1;
                    cursor.retiring_mesh = false;
                    state.bridge.scene_bridge = Some(cursor);
                    World3dSceneBridgeStep::Pending
                }
                Err(_) => {
                    state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                    cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);
                World3dSceneBridgeStep::Fault
                }
            }
        }
        World3dSceneBridgePhase::Pages if !world3d_scene_bridge_has_pages(&state.scene, &cursor) => {
            state.bridge.scene_bridge_digest = Some(cursor.digest);
            if cursor.camera_changed {
                state.camera.scene_camera_digest = Some(cursor.camera_digest);
            }
            cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);
            World3dSceneBridgeStep::Complete
        }
        World3dSceneBridgePhase::Pages => match publish_world3d_scene_bridge_snapshot(&mut state.session, &mut state.selection, &mut state.interaction, &mut state.scene, &mut state.bridge, &cursor) {
            Ok(lease) => {
                if let Some(previous) = state.bridge.scene_bridge_lease.replace(lease) {
                    state.bridge.scene_bridge_retired = Some((previous, false));
                }
                state.bridge.scene_bridge_digest = Some(cursor.digest);
                if cursor.camera_changed {
                    state.camera.scene_camera_digest = Some(cursor.camera_digest);
                }
                cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);
            World3dSceneBridgeStep::Complete
            }
            Err(fault) => {
                state.session.snapshot_fault = Some(fault);
                cursor.phase=World3dSceneBridgePhase::Close;state.bridge.scene_bridge=Some(cursor);
                World3dSceneBridgeStep::Fault
            }
        },
    }
}

pub fn sync_world3d_state(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    state.session.bounds = bounds;
    let Some(world) = scene.world_3d.as_ref() else {
        state.session.snapshot_fault = Some(World3dSnapshotFault::Unavailable);
        return;
    };
    state.session.bound_domain_id = world.domain_id.clone();
    state.session.bound_domain_granularity_id = world.domain_granularity_id.clone();
    state.session.presentation = world3d_presentation(world.presentation_json.as_deref());
    state.scene.environment = world.environment_json.as_deref().and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
    sync_world3d_tool_run_trace(&mut state.trace, world);
    sync_world3d_scene_fit(&mut state.camera, world.fit_json.as_deref());
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &world.selection_json);
    apply_world3d_pick_filter(&mut state.session, &mut state.selection, &mut state.scene, world.modelling_options.as_ref().and_then(|options| options.pick_filter));
    sync_world3d_scene_document_lanes(&mut state.session, &mut state.selection, &mut state.scene, &mut state.bridge, &mut state.assets, world);
    sync_world3d_brush_mesh_feedback(&mut state.assets, world.interaction_json.as_deref());
    let lease = match world.snapshot {
        Some(lease) => lease,
        None => {
            stage_world3d_scene_bridge(&mut state.camera, &mut state.bridge, world);
            let Some(lease) = state.bridge.scene_bridge_lease else {
                if state.bridge.scene_bridge.is_none() {
                    state.session.snapshot_fault = Some(World3dSnapshotFault::Unavailable);
                }
                return;
            };
            lease
        }
    };
    if state.bridge.snapshot_lease == Some(lease) || state.bridge.snapshot_apply.as_ref().is_some_and(|cursor| cursor.lease == lease) {
        return;
    }
    if state.bridge.snapshot_apply.is_some() {
        state.session.snapshot_fault = Some(World3dSnapshotFault::Closing);
        return;
    }
    state.bridge.snapshot_apply = Some(World3dSnapshotApplyCursor::new(lease));
    state.session.snapshot_fault = None;
}

fn world3d_presentation(json: Option<&str>) -> World3dPresentation {
    let Some(json) = json else { return World3dPresentation::default() };
    let Ok(value) = serde_json::from_str::<World3dPresentation>(json) else { return World3dPresentation::default() };
    if value.source_aspect.is_some_and(|aspect| !aspect.is_finite() || aspect <= 0.0) {
        return World3dPresentation::default();
    }
    value
}

/// ⏯️ Feeds `World3dScene.tool_run_trace` into the resident trace layer and re-derives the mesh index →
/// mesh key table the trace subjects address (`meshes_json` order) only when that lane changed.
fn sync_world3d_tool_run_trace(trace_state: &mut WorldTraceState, world: &ui_wgpu::wgpu::World3dScene) {
    let _ = trace_state.tool_run_trace.apply_lane(world.tool_run_trace.as_deref());
    if trace_state.tool_run_trace.is_empty() {
        return;
    }
    let digest = world3d_scene_digest(&[&world.meshes_json]);
    if trace_state.tool_run_trace_meshes_digest == Some(digest) {
        return;
    }
    #[derive(Deserialize)]
    struct MeshKey {
        id: String,
    }
    trace_state.tool_run_trace_mesh_keys = serde_json::from_str::<Vec<MeshKey>>(&world.meshes_json).map(|meshes| meshes.into_iter().map(|mesh| mesh.id).collect()).unwrap_or_default();
    trace_state.tool_run_trace_meshes_digest = Some(digest);
}

/// 🧭️ The cursor this surface echoes as `toolRunTraceCursorByWindowId[window]` in its window instance view state.
pub fn world3d_tool_run_trace_cursor(trace_state: &WorldTraceState) -> Option<semio_framework_tool_run::ToolRunTraceCursor> {
    trace_state.tool_run_trace.cursor()
}

/// 🧭️ Every `(window instance, cursor)` echo the given world surfaces owe their view state — surfaces without
/// a bound window or without an applied trace lane echo nothing.
pub fn world3d_tool_run_trace_cursors<'a>(states: impl IntoIterator<Item = &'a World3dState>) -> HashMap<String, semio_framework_tool_run::ToolRunTraceCursor> {
    states.into_iter().filter_map(|state| Some((state.trace.tool_run_trace_window_id.clone()?, state.trace.tool_run_trace.cursor()?))).collect()
}

/// 🎨️ Appends the trace layer's instanced `(mesh, verdict)` draws; a mesh index with no published mesh
/// falls back to the placeholder box, exactly like the brush ghost.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn append_tool_run_trace_draws(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, trace_state: &mut WorldTraceState, gpu: &mut World3dBuildContext, theme: &ui_wgpu::wgpu::Theme, translucent_draws: &mut Vec<SceneDraw3d>) {
    if trace_state.tool_run_trace.is_empty() {
        return;
    }
    let palette = tool_run_trace::ToolRunTracePalette::from_theme(theme);
    for draw in trace_state.tool_run_trace.draws(&palette, trace_state.tool_run_trace_visibility) {
        let mesh_key = trace_state.tool_run_trace_mesh_keys.get(draw.mesh as usize).filter(|key| scene_state.meshes.contains_key(key.as_str())).cloned().unwrap_or_else(|| brush_preview_mesh_id(None));
        if !scene_state.meshes.contains_key(&mesh_key) {
            begin_world_placeholder_mesh(session_state, authority_state, retirement_state, scene_state, &mesh_key, WorldPlaceholderKind::Box);
        }
        let mesh_version = live_world3d_mesh_version(scene_state, &mesh_key);
        if let Some(mesh) = scene_state.meshes.get(&mesh_key).copied() {
            gpu.ensure_mesh(&mesh_key, mesh_version, mesh);
            translucent_draws.push(SceneDraw3d { mesh_key, mesh_version, instances: draw.instances, shadow_role: Default::default() });
        }
    }
}

/// 🎯️ A published id may name the rendered INSTANCE or the TOPOLOGY target it stands for, and both
/// reach this state: the guest's `selectionJson.ids` carries render ids (pinned by
/// `🧫️fixtures/🌉️scene-bridge/🔣️.json`'s `selectedIds`), while this surface's OWN optimistic
/// `local_hover_id` is the topology id it just dispatched in `interactionHover`. Matching only one
/// of the two leaves half the highlights dark. React resolves the same pair
/// (`useWorldInstanceChrome(instance.id)` over a selection the host maps back through
/// `interactionId`). See [`instance_interaction_id`].
///
/// 🎨️ Selection/hover flags must follow the live selection snapshot — OR-ing with the
/// instancesJson bits left deselected meshes painted selected until a later hover rebuild.
pub fn apply_runtime_draw_flags(selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState) {
    let granularity = selection_state.granularity.clone();
    let component_ids: HashSet<String> = selection_state.component_ids.iter().cloned().collect();
    let local_hover_id = selection_state.local_hover_id.clone();
    let hovered_component_object_id = selection_state.hovered_component_object_id.clone();
    let selected_ids: HashSet<String> = selection_state.selected_ids.iter().cloned().collect();
    let interaction_ids = interaction_state.instance_interaction_ids.clone();
    let mut object_index_map = HashMap::new();
    let mut index = 0u32;
    for draw in &scene_state.draws {
        for instance in &draw.instances {
            object_index_map.insert(instance.id.clone(), index);
            index += 1;
        }
    }
    let component_mode = component_mode_active(selection_state);
    for draw in &mut scene_state.draws {
        for instance in &mut draw.instances {
            let mesh_selected = granularity == "mesh" && object_index_map.get(&instance.id).is_some_and(|object_index| component_ids.contains(&object_index.to_string()));
            let target_id = interaction_ids.get(&instance.id).map_or(instance.id.as_str(), String::as_str);
            let local_hovered = if component_mode { false } else { local_hover_id.as_deref() == Some(instance.id.as_str()) || local_hover_id.as_deref() == Some(target_id) || hovered_component_object_id.as_deref() == Some(instance.id.as_str()) };
            let local_selected = selected_ids.contains(&instance.id) || selected_ids.contains(target_id) || mesh_selected;
            instance.hovered = local_hovered;
            instance.selected = local_selected;
        }
    }
}

/// ✂️ Closes the cut of a capped section plane over the clipped solids, the native twin of the React `World3dSectionClip` cap: the clipped solids are drawn once with `StencilToggle`
/// (each fragment inverts the stencil's section bit, so after both faces of a cut solid the bit is set exactly where the cut exposes its inside), then the flat cap plane in the cap tone
/// is drawn only where the bit is set, then the solids are drawn a second time to invert the bit back to zero. The three passes ride the translucent draw list in that order,
/// after the opaque solids, so the cap is depth-tested against the retained surface.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn append_world3d_section_cap(session_state: &mut WorldSessionState, authority_state: &mut WorldAuthorityState, retirement_state: &mut WorldRetirementState, scene_state: &mut WorldSceneState, gpu: &mut World3dBuildContext, theme: &ui_wgpu::wgpu::Theme, section: &ui_wgpu::wgpu::World3dSection, solids: &[SceneDraw3d], translucent: &mut Vec<SceneDraw3d>) {
    let Some(cap) = section.cap else { return };
    if solids.iter().all(|draw| draw.instances.is_empty()) {
        return;
    }
    if !scene_state.meshes.contains_key(GUMBALL_PLANE_MESH) {
        begin_world_placeholder_mesh(session_state, authority_state, retirement_state, scene_state, GUMBALL_PLANE_MESH, WorldPlaceholderKind::Plane);
    }
    let Some(plane) = scene_state.meshes.get(GUMBALL_PLANE_MESH).copied() else { return };
    let plane_version = live_world3d_mesh_version(scene_state, GUMBALL_PLANE_MESH);
    gpu.ensure_mesh(GUMBALL_PLANE_MESH, plane_version, plane);
    let (mut low, mut high) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
    for draw in solids {
        let Some(Ok((mesh_low, mesh_high))) = scene_state.meshes.get(&draw.mesh_key).map(|mesh| mesh.aabb()) else { continue };
        for instance in &draw.instances {
            let (instance_low, instance_high) = transform_aabb(instance.model, mesh_low, mesh_high);
            for axis in 0..3 {
                low[axis] = low[axis].min(instance_low[axis]);
                high[axis] = high[axis].max(instance_high[axis]);
            }
        }
    }
    if !(0..3).all(|axis| low[axis].is_finite() && high[axis].is_finite()) {
        return;
    }
    let diagonal = Vec3::new(high[0] - low[0], high[1] - low[1], high[2] - low[2]).length();
    let side = (4.0 * diagonal).max(2.0);
    let normal = section.unit_normal();
    let normal = Vec3::new(normal[0] as f32, normal[1] as f32, normal[2] as f32);
    let helper = if normal.y.abs() < 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
    let tangent = helper.cross(normal).normalize();
    let rotation = quat_from_basis(tangent, normal, tangent.cross(normal));
    let origin = [section.origin[0] as f32, section.origin[1] as f32, section.origin[2] as f32];
    let toggled = || -> Vec<SceneDraw3d> {
        solids
            .iter()
            .filter(|draw| !draw.instances.is_empty())
            .map(|draw| SceneDraw3d {
                mesh_key: draw.mesh_key.clone(),
                mesh_version: draw.mesh_version,
                instances: draw.instances.iter().map(|instance| Instance3d { material: SceneInstanceMaterial3d { section: SceneSectionRole3d::StencilToggle, ..instance.material }, ..instance.clone() }).collect(),
                shadow_role: Default::default(),
            })
            .collect()
    };
    translucent.extend(toggled());
    translucent.push(SceneDraw3d {
        mesh_key: GUMBALL_PLANE_MESH.into(),
        mesh_version: plane_version,
        instances: vec![Instance3d {
            component_source: None,
            id: "section-cap".into(),
            model: Instance3d::model_from_trs(origin, rotation, [side, 1.0, side]),
            color: world3d_tone_rgba(theme, cap.tone),
            selected: false,
            hovered: false,
            material: SceneInstanceMaterial3d { section: SceneSectionRole3d::Cap, ..Default::default() },
        }],
        shadow_role: Default::default(),
    });
    translucent.extend(toggled());
}

//#region 📏️Modelling overlay
const WORLD_OVERLAY_ARC_SEGMENT_PX: f64 = 6.0;

const WORLD_OVERLAY_LEGEND_MARGIN_PX: f32 = 8.0;

const WORLD_OVERLAY_LEGEND_PADDING_PX: f32 = 6.0;

const WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX: f32 = 12.0;

const WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX: f32 = 96.0;

const WORLD_OVERLAY_LEGEND_BAR_SLICES: usize = 32;

const WORLD_OVERLAY_LEGEND_GAP_PX: f32 = 6.0;

const WORLD_OVERLAY_LABEL_PADDING_X_PX: f32 = 4.0;

const WORLD_OVERLAY_LABEL_PADDING_Y_PX: f32 = 1.0;

/// 📐️ The three corners of a dimension arrowhead whose tip sits at `(x, y)` and points along `angle` — React's `arrowPoints`.
fn world3d_arrow_points(x: f64, y: f64, angle: f64) -> [[f32; 2]; 3] {
    let length = ui_wgpu::wgpu::WORLD3D_ANNOTATION_ARROW_PX;
    let (cos, sin) = (angle.cos(), angle.sin());
    let (base_x, base_y) = (x - cos * length, y - sin * length);
    let half = length * 0.35;
    [[x as f32, y as f32], [(base_x - sin * half) as f32, (base_y + cos * half) as f32], [(base_x + sin * half) as f32, (base_y - cos * half) as f32]]
}

/// 🌙️ The polyline of an angle arc, one vertex per `WORLD_OVERLAY_ARC_SEGMENT_PX` of arc length (at least four, at most thirty-two segments).
fn world3d_arc_points(arc: &ui_wgpu::wgpu::World3dProjectedArc) -> Vec<[f32; 2]> {
    let segments = ((arc.sweep.abs() * arc.radius / WORLD_OVERLAY_ARC_SEGMENT_PX).ceil() as usize).clamp(4, 32);
    (0..=segments)
        .map(|index| {
            let angle = arc.start + arc.sweep * index as f64 / segments as f64;
            [(arc.cx + angle.cos() * arc.radius) as f32, (arc.cy + angle.sin() * arc.radius) as f32]
        })
        .collect()
}

fn world3d_circle_points(x: f64, y: f64, radius: f64) -> Vec<[f32; 2]> {
    (0..=16).map(|index| index as f64 / 16.0 * std::f64::consts::TAU).map(|angle| [(x + angle.cos() * radius) as f32, (y + angle.sin() * radius) as f32]).collect()
}

fn world3d_overlay_color(theme: &ui_wgpu::wgpu::Theme, tone: ui_wgpu::wgpu::World3dTone) -> Rgba {
    let color = world3d_tone_rgba(theme, tone);
    Rgba::new(color[0], color[1], color[2], color[3])
}

/// 📏️ Paints the projected annotation layer on top of the world pass: lines, arrowheads, arcs and marks at screen-constant size in the tone colours of the live theme,
/// and the caller's text in the active language on a translucent plate. Clipped to the viewport by the caller's scissor.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn paint_world3d_annotations(ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, layer: &ui_wgpu::wgpu::World3dAnnotationLayer, view_proj: &[f32; 16], inner: Rect, locale: &str) {
    use ui_wgpu::wgpu::{World3dLabelAlign, World3dMarkerShape};
    let theme = ctx.theme;
    let stroke = ui_wgpu::wgpu::WORLD3D_ANNOTATION_STROKE_PX as f32;
    let marker_radius = ui_wgpu::wgpu::WORLD3D_ANNOTATION_MARKER_PX / 2.0;
    let size = theme.font_size_small;
    let (origin_x, origin_y) = (f64::from(inner.x), f64::from(inner.y));
    let at = |points: Vec<[f32; 2]>| -> Vec<[f32; 2]> { points.into_iter().map(|point| [point[0] + inner.x, point[1] + inner.y]).collect() };
    let projected = ui_wgpu::wgpu::project_world3d_annotations(layer, view_proj, inner.w, inner.h);
    for (item, entry) in layer.items.iter().zip(&projected).filter(|(_, entry)| entry.visible) {
        let color = world3d_overlay_color(theme, entry.tone);
        for line in &entry.lines {
            ctx.draw.push_line_overlay(inner.x + line[0] as f32, inner.y + line[1] as f32, inner.x + line[2] as f32, inner.y + line[3] as f32, color, stroke);
        }
        for arrow in &entry.arrows {
            ctx.draw.push_triangle_fan_overlay(&at(world3d_arrow_points(arrow.x, arrow.y, arrow.angle).to_vec()), color);
        }
        if let Some(arc) = entry.arc.as_ref() {
            for pair in at(world3d_arc_points(arc)).windows(2) {
                ctx.draw.push_line_overlay(pair[0][0], pair[0][1], pair[1][0], pair[1][1], color, stroke);
            }
        }
        if let Some(marker) = entry.marker {
            let (x, y) = (marker.x + origin_x, marker.y + origin_y);
            match marker.shape {
                World3dMarkerShape::Dot => ctx.draw.push_rounded_overlay([(x - marker_radius / 2.0) as f32, (y - marker_radius / 2.0) as f32, marker_radius as f32, marker_radius as f32], color, marker_radius as f32 / 2.0),
                World3dMarkerShape::Cross => {
                    for (dx, dy) in [(-1.0, -1.0), (-1.0, 1.0)] {
                        ctx.draw.push_line_overlay((x + dx * marker_radius) as f32, (y + dy * marker_radius) as f32, (x - dx * marker_radius) as f32, (y - dy * marker_radius) as f32, color, stroke);
                    }
                }
                World3dMarkerShape::Ring => {
                    for pair in world3d_circle_points(x, y, marker_radius).windows(2) {
                        ctx.draw.push_line_overlay(pair[0][0], pair[0][1], pair[1][0], pair[1][1], color, stroke);
                    }
                }
            }
        }
        if let Some(label) = entry.label {
            let text = item.text().resolve(locale);
            let (width, height) = ctx.atlas.measure_text(text, size);
            let left = match label.align {
                World3dLabelAlign::Middle => label.x - f64::from(width) / 2.0,
                World3dLabelAlign::Start => label.x,
                World3dLabelAlign::End => label.x - f64::from(width),
            } as f32
                + inner.x;
            let top = label.y as f32 + inner.y - height / 2.0;
            let plate = Rgba::new(theme.background.r, theme.background.g, theme.background.b, 0.8);
            ctx.draw.push_rounded_overlay([left - WORLD_OVERLAY_LABEL_PADDING_X_PX, top - WORLD_OVERLAY_LABEL_PADDING_Y_PX, width + 2.0 * WORLD_OVERLAY_LABEL_PADDING_X_PX, height + 2.0 * WORLD_OVERLAY_LABEL_PADDING_Y_PX], plate, theme.border_radius.min(4.0));
            ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, text, left, label.y as f32 + inner.y + size * 0.35, size, color);
        }
    }
}

/// 🌡️ The panel rectangle of a scalar-field legend (or its mismatch message) pinned to the viewport's lower left, `content` pixels wide and tall.
fn world3d_legend_rect(inner: Rect, content: (f32, f32)) -> [f32; 4] {
    let (width, height) = (content.0 + 2.0 * WORLD_OVERLAY_LEGEND_PADDING_PX, content.1 + 2.0 * WORLD_OVERLAY_LEGEND_PADDING_PX);
    [inner.x + WORLD_OVERLAY_LEGEND_MARGIN_PX, inner.y + inner.h - WORLD_OVERLAY_LEGEND_MARGIN_PX - height, width, height]
}

/// 🌡️ Paints the scalar-field legend: the title with its unit, the ramp as a vertical gradient (range minimum at the bottom), the evenly spaced tick values beside it and the no-data caption.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn paint_world3d_legend(ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, field: &ui_wgpu::wgpu::World3dScalarField, inner: Rect, locale: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_small;
    let lines = field.legend_lines(locale);
    let (title_width, title_height) = ctx.atlas.measure_text(&lines.title, size);
    let tick_width = lines.ticks.iter().map(|tick| ctx.atlas.measure_text(&tick.text, size).0).fold(0.0_f32, f32::max);
    let no_data = lines.no_data.map(|text| (text, ctx.atlas.measure_text(text, size)));
    let ramp_width = WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX + WORLD_OVERLAY_LEGEND_GAP_PX + tick_width;
    let no_data_width = no_data.map_or(0.0, |(_, (width, _))| 10.0 + WORLD_OVERLAY_LEGEND_GAP_PX + width);
    let no_data_height = no_data.map_or(0.0, |(_, (_, height))| WORLD_OVERLAY_LEGEND_GAP_PX + height.max(10.0));
    let content = (title_width.max(ramp_width).max(no_data_width), title_height + WORLD_OVERLAY_LEGEND_GAP_PX + WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX + no_data_height);
    let rect = world3d_legend_rect(inner, content);
    ctx.draw.push_rounded_overlay(rect, Rgba::new(theme.panel.r, theme.panel.g, theme.panel.b, 0.9), theme.border_radius.min(4.0));
    let (left, top) = (rect[0] + WORLD_OVERLAY_LEGEND_PADDING_PX, rect[1] + WORLD_OVERLAY_LEGEND_PADDING_PX);
    ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, &lines.title, left, top + size, size, theme.text);
    let bar_top = top + title_height + WORLD_OVERLAY_LEGEND_GAP_PX;
    let slice = WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX / WORLD_OVERLAY_LEGEND_BAR_SLICES as f32;
    for index in 0..WORLD_OVERLAY_LEGEND_BAR_SLICES {
        let [r, g, b] = field.ramp.sample(1.0 - (index as f64 + 0.5) / WORLD_OVERLAY_LEGEND_BAR_SLICES as f64);
        ctx.draw.push_solid_overlay([left, bar_top + index as f32 * slice, WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX, slice + 0.5], Rgba::from_srgb8(r, g, b, 255));
    }
    let steps = lines.ticks.len().saturating_sub(1).max(1) as f32;
    for (index, tick) in lines.ticks.iter().enumerate() {
        let y = bar_top + WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX - index as f32 / steps * WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX;
        ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, &tick.text, left + WORLD_OVERLAY_LEGEND_BAR_WIDTH_PX + WORLD_OVERLAY_LEGEND_GAP_PX, y + size * 0.35, size, theme.text);
    }
    if let Some((text, (_, height))) = no_data {
        let y = bar_top + WORLD_OVERLAY_LEGEND_BAR_HEIGHT_PX + WORLD_OVERLAY_LEGEND_GAP_PX;
        let [r, g, b] = ui_wgpu::wgpu::WORLD3D_SCALAR_NO_DATA_RGB;
        ctx.draw.push_solid_overlay([left, y, 10.0, 10.0], Rgba::from_srgb8(r, g, b, 255));
        ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, text, left + 10.0 + WORLD_OVERLAY_LEGEND_GAP_PX, y + height.max(10.0) * 0.5 + size * 0.35, size, theme.text);
    }
}

/// 🌡️ Announces on the viewport that the scalar field does not fit the mesh it names, in place of a wrong heatmap.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn paint_world3d_scalar_mismatch(ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>, inner: Rect, locale: &str) {
    let theme = ctx.theme;
    let size = theme.font_size_small;
    let text = ui_wgpu::wgpu::world3d_modelling_strings(locale).mismatch;
    let (width, height) = ctx.atlas.measure_text(text, size);
    let rect = world3d_legend_rect(inner, (width, height));
    ctx.draw.push_rounded_overlay(rect, Rgba::new(theme.panel.r, theme.panel.g, theme.panel.b, 0.9), theme.border_radius.min(4.0));
    ui_wgpu::wgpu::widgets::draw_text_overlay(ctx, text, rect[0] + WORLD_OVERLAY_LEGEND_PADDING_PX, rect[1] + WORLD_OVERLAY_LEGEND_PADDING_PX + size, size, theme.error);
}

/// ♿️ One node of the accessibility tree a world surface publishes for its modelling overlays: a stable key, an ARIA role, the text a screen reader speaks and its nesting below the surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World3dAccessibilityEntry {
    pub key: String,
    pub role: &'static str,
    pub label: String,
    pub depth: u8,
    pub polite: bool,
}

/// ♿️ The text alternative of everything `render_world_3d` paints for the modelling layer, in the surface's language and with the same structure as the React host: a `list` of annotations
/// named by the layer title, one `listitem` per annotation (`Dimension: Width 40 mm`), a `group` for an applied scalar-field legend with its scale ticks and no-data caption, or a polite `status`
/// when the field does not fit its mesh. The shell hands these to the accessibility mirror (the screen reader's `aria-live` region), so nothing drawn here is pointer-only.
pub fn world3d_modelling_accessibility(session_state: &WorldSessionState, overlay_state: &WorldOverlayState, scene: &UiComponentSceneNode) -> Vec<World3dAccessibilityEntry> {
    let Some(world) = scene.world_3d.as_ref() else { return Vec::new() };
    let locale = session_state.modelling_locale;
    let strings = ui_wgpu::wgpu::world3d_modelling_strings(locale);
    let host = scene.host_id.as_str();
    let entry = |key: String, role: &'static str, label: String, depth: u8, polite: bool| World3dAccessibilityEntry { key, role, label, depth, polite };
    let mut entries = Vec::new();
    if let Some(layer) = world.annotations.as_ref().filter(|layer| !layer.items.is_empty()) {
        entries.push(entry(format!("{host}::annotations"), "list", layer.title.as_ref().map_or(strings.annotations, |title| title.resolve(locale)).to_string(), 0, false));
        entries.extend(layer.items.iter().map(|item| entry(format!("{host}::annotation::{}", item.id()), "listitem", item.accessible_name(locale), 1, false)));
    }
    if let Some(field) = world.scalar_field.as_ref() {
        match overlay_state.scalar_field_status {
            World3dScalarFieldStatus::Applied => {
                let lines = field.legend_lines(locale);
                entries.push(entry(format!("{host}::legend"), "group", format!("{}: {}", strings.legend, field.legend.title.resolve(locale)), 0, false));
                entries.push(entry(format!("{host}::legend::scale"), "list", strings.scale.to_string(), 1, false));
                entries.extend(lines.ticks.iter().enumerate().map(|(index, tick)| entry(format!("{host}::legend::tick::{index}"), "listitem", tick.text.clone(), 2, false)));
                if let Some(caption) = lines.no_data {
                    entries.push(entry(format!("{host}::legend::no-data"), "paragraph", caption.to_string(), 1, false));
                }
            }
            World3dScalarFieldStatus::Mismatch => entries.push(entry(format!("{host}::scalar-field-mismatch"), "status", strings.mismatch.to_string(), 0, true)),
            World3dScalarFieldStatus::None | World3dScalarFieldStatus::Missing => {}
        }
    }
    entries
}
//#endregion 📏️Modelling overlay

// 🖥️ Native/browser rendering-host entry point only (called from `📺️renderer`'s engine, never from
// wasip2 plugin guest logic — confirmed by grepping every `render_world_3d` call site in the repo).
// `WidgetContext` bundles the font/icon atlases, which are genuinely GPU-adjacent (real `wgpu`
// crate reachable through `wgpu-engine`), so this one function stays excluded from
// `wasm32-wasip2`: `target_arch = "wasm32"` is TRUE for wasip2 too, hence `not(target_env = "p2")`.
/// 👻️ Mirrors `BrushPreviewGhost`: renders whenever `origin` is present, regardless of
/// `meshUrl` — a translucent unit box is the fallback ghost when there's no mesh URL (or
/// its GLB hasn't resolved into `state.meshes` yet), not "nothing at all". The box itself is
/// a placeholder LEASE, so the ghost appears on the frame that lease lands, never before it.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn render_world_3d(
    scene: &UiComponentSceneNode,
    bounds: Rect,
    ctx: &mut ui_wgpu::wgpu::widgets::WidgetContext<'_, ActionDescriptor>,
    state: &mut World3dState,
    gpu: &mut World3dBuildContext,
    shadow_profile: World3dShadowProfile,
) {
    use ui_wgpu::wgpu::widgets::{draw_text, gizmo as gpu_gizmo};
    let theme = ctx.theme;
    let modelling = scene.world_3d.as_ref();
    let section = modelling.and_then(|world| world.modelling_options.as_ref()).and_then(|options| options.section);
    let sub_element_paint = world3d_sub_element_paint_native(theme, modelling.and_then(|world| world.modelling_options.as_ref()).and_then(|options| options.highlight.as_ref()));
    if state.overlay.sub_element_paint != sub_element_paint {
        state.overlay.sub_element_paint = sub_element_paint;
        advance_world3d_view_revision(&mut state.session, &mut state.scene);
    }
    step_world_placeholder_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene);
    state.session.pick_bounds = ctx.pick_clip.unwrap_or(bounds);
    sync_world3d_state(state, scene, bounds);
    let _ = step_world3d_camera_fit(state);
    apply_terrain_style_if_changed(&mut state.session, &mut state.scene, &mut state.terrain, gpu);
    let current_lod = scene_lod(&state.session, &state.camera, &state.scene);
    let lod_changed = state.camera.resolved_lod_pick.is_none_or(|previous| (previous - current_lod).abs() > WORLD_LOD_EPSILON);
    if lod_changed {
        state.camera.resolved_lod_pick = Some(current_lod);
    }
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
    let inner = bounds;
    let clear_color = match state.session.presentation.clear {
        World3dPresentationClear::Theme => Some(environment_clear_color(&state.scene.environment, theme.canvas_clear)),
        World3dPresentationClear::Transparent => environment_explicit_clear_color(&state.scene.environment),
    };
    if state.session.presentation.viewport_mask == SceneViewportMask3d::Rectangle {
        if let Some(clear_color) = clear_color {
            ctx.draw.push_solid([inner.x, inner.y, inner.w, inner.h], clear_color);
        }
    }
    let camera = state.camera.orbit.to_camera();
    let light_dir = environment_light_dir(&state.scene.environment);
    let lighting = environment_scene_lighting(&state.scene.environment);
    let neutral_material = environment_scene_material(&state.scene.environment);
    let shadow = environment_scene_shadow(&state.scene.environment, light_dir, shadow_profile);
    let mut terrain_draws = sync_terrain(&mut state.session, &mut state.authority, &mut state.scene, &mut state.terrain, &mut state.assets, gpu, &camera);
    for draw in &mut terrain_draws {
        draw.shadow_role = world3d_shadow_role(World3dShadowGeometry::Terrain, shadow.enabled);
    }
    update_visible_chunks(&mut state.scene, camera.position);
    let projection_height = state.session.presentation.source_aspect.map_or(inner.h, |aspect| inner.w / aspect as f32);
    let view_proj = ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, projection_height);
    let planes = frustum_planes(view_proj);
    let shadow_planes = directional_shadow_frustum_planes(light_dir);
    let mut culled_draws = Vec::new();
    let mut material_draws = Vec::new();
    let mut shadow_draws = Vec::new();
    let mut culled_count = 0u32;
    let mut needed_mesh_keys = HashSet::new();
    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    for (draw_index, draw) in state.scene.draws.iter().enumerate() {
        let Some(&mesh) = state.scene.meshes.get(&draw.mesh_key) else {
            continue;
        };
        let Ok((mesh_min, mesh_max)) = mesh.aabb() else { continue };
        let mesh_version = live_world3d_mesh_version(&state.scene, &draw.mesh_key);
        let mut instances = Vec::new();
        let mut painted_instances = Vec::new();
        let mut celebration_instances = Vec::new();
        let mut authored_instances = Vec::new();
        let mut shadow_instances = Vec::new();
        let schema = mesh.schema().ok();
        let has_vertex_colors = schema.is_some_and(|schema| schema.colors > 0);
        let paint_texture = schema.filter(|schema| schema.uvs == schema.vertices && schema.uvs > 0).and_then(|_| state.scene.mesh_paint_textures.get(&draw.mesh_key)).map(|raster| raster.identity);
        let [metalness, roughness] = world3d_standard_material_for_mesh(&state.scene, &draw.mesh_key, neutral_material);
        for (instance_index, source) in draw.instances.iter().enumerate() {
            let style = world_instance_style(&state.selection, source);
            let mut instance = world3d_style_paint(theme, style, has_vertex_colors, source.clone());
            if section.is_some() {
                instance.material.section = SceneSectionRole3d::Clipped;
            }
            let style_kind = resolve_mesh_style(style);
            instance.material.metalness = metalness;
            instance.material.roughness = roughness;
            instance.model = retained_gumball_preview_model(&state.interaction, &state.authority, draw_index, instance_index, instance.model);
            let position = state.scene.instance_positions.get(&instance.id).copied().unwrap_or([0.0, 0.0, 0.0]);
            if !instance_chunk_visible(&state.scene, position) {
                continue;
            }
            let (min, max) = transform_aabb(instance.model, mesh_min, mesh_max);
            if shadow.enabled && aabb_intersects_frustum(&shadow_planes, min, max) {
                shadow_instances.push(instance.clone());
            }
            if aabb_intersects_frustum(&planes, min, max) {
                if matches!(style_kind,MeshStyleKind::Neutral|MeshStyleKind::Disabled) && state.scene.mesh_appearances.contains_key(&draw.mesh_key) {
                    authored_instances.push(instance);
                } else if style_kind == MeshStyleKind::Celebrated && !instance.material.preserve_vertex_color {
                    celebration_instances.push(instance);
                } else if paint_texture.is_some() {
                    painted_instances.push(instance);
                } else {
                    instances.push(instance);
                }
            } else {
                culled_count += 1;
            }
        }
        if !instances.is_empty() || !painted_instances.is_empty() || !celebration_instances.is_empty() || !authored_instances.is_empty() || !shadow_instances.is_empty() {
            needed_mesh_keys.insert(draw.mesh_key.clone());
            gpu.ensure_mesh(&draw.mesh_key, mesh_version, mesh);
        }
        if paint_texture.is_some() && !painted_instances.is_empty() {
            let texture_key = format!("mesh-paint:{}", draw.mesh_key);
            if let Some(lease) = world_raster_upload(&state.scene, &draw.mesh_key, &texture_key, true) {
                gpu.ensure_world_plane_texture(&texture_key, lease);
            }
        }
        if !authored_instances.is_empty() {
            let pool = world_scene_raster_pool();
            let override_material = environment_authored_material(&state.scene.environment);
            if let Some(appearance) = state.scene.mesh_appearances.get(&draw.mesh_key) {
                for primitive in appearance.primitives() {
                    let material = override_material.clone().unwrap_or_else(|| primitive.material.clone());
                    for texture_key in SceneMaterialKind3d::Authored(material.clone()).texture_keys() {if let Some(lease)=appearance.texture_upload(texture_key,&pool){gpu.ensure_world_plane_texture(texture_key,lease);}}
                    material_draws.push(SceneMaterialDraw3d {
                        mesh_key: draw.mesh_key.clone(),
                        mesh_version,
                        first_index: primitive.first_index,
                        index_count: primitive.index_count,
                        instances: authored_instances.clone(),
                        translucent: material.alpha == SceneMaterialAlpha3d::Blend,
                        material: SceneMaterialKind3d::Authored(material),
                    });
                }
            }
        }
        let glb_shadow_role = world3d_shadow_role(World3dShadowGeometry::Glb, shadow.enabled);
        if !shadow_instances.is_empty() {
            shadow_draws.push(SceneDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, instances: shadow_instances, shadow_role: glb_shadow_role });
        }
        let instances = instances.into_iter().map(|instance| {
            if state.selection.provisional_instance_ids.contains(&instance.id) {
                Instance3d { selected: false, hovered: false, ..instance }
            } else {
                instance
            }
        });
        let (translucent, opaque): (Vec<_>, Vec<_>) = instances.partition(|instance| instance.color[3] < 1.0);
        if !opaque.is_empty() {
            culled_draws.push(SceneDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, instances: opaque, shadow_role: glb_shadow_role });
        }
        if !translucent.is_empty() {
            material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: translucent, material: SceneMaterialKind3d::Standard, translucent: true });
        }
        if !painted_instances.is_empty() {
            let texture_key = format!("mesh-paint:{}", draw.mesh_key);
            let (opaque, translucent): (Vec<_>, Vec<_>) = painted_instances.into_iter().partition(|instance| instance.color[3] >= 1.0);
            if !opaque.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: opaque, material: SceneMaterialKind3d::Painted { texture_key: texture_key.clone() }, translucent: false });
            }
            if !translucent.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: translucent, material: SceneMaterialKind3d::Painted { texture_key }, translucent: true });
            }
        }
        if !celebration_instances.is_empty() {
            let stops = mesh_style_paint(theme, MeshStyleKind::Celebrated).conic.expect("celebrated style owns the conic triad");
            let angle = (ctx.draw.clock_seconds() / CELEBRATE_CONIC_SPIN_SECONDS).rem_euclid(1.0) * std::f32::consts::TAU;
            let (opaque, translucent): (Vec<_>, Vec<_>) = celebration_instances.into_iter().partition(|instance| instance.color[3] >= 1.0);
            if !opaque.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: opaque, material: SceneMaterialKind3d::Celebration { stops, angle }, translucent: false });
            }
            if !translucent.is_empty() {
                material_draws.push(SceneMaterialDraw3d { mesh_key: draw.mesh_key.clone(), mesh_version, first_index: 0, index_count: u32::MAX, instances: translucent, material: SceneMaterialKind3d::Celebration { stops, angle }, translucent: true });
            }
        }
    }
    sync_mesh_pool(&mut state.scene, &mut state.assets, &needed_mesh_keys, gpu);
    let procedural_grid = if state.session.presentation.show_grid && state.scene.lod.show_grid {
        let datum = state.scene.lod.grid_datum.unwrap_or([0.0, 0.0, 0.0]);
        lod_grid_step_world(current_lod, state.scene.lod.grid_factor).and_then(|step| {
            let cell_size = step as f32;
            if !cell_size.is_finite() || cell_size <= 0.0 {
                return None;
            }
            let plane_z = datum[2] as f32 + 0.001;
            Some(ProceduralGrid3d {
                plane_z,
                camera_plane_projection: [camera.position.x, camera.position.y, plane_z],
                cell_size,
                fade_distance: camera_grid_fade_distance(&camera, datum[2] as f32, step, inner.w, inner.h),
                cell_color: [theme.text_element.r, theme.text_element.g, theme.text_element.b],
            })
        })
    } else {
        None
    };
    let mut line_vertices = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, theme, &camera, inner, &mut line_vertices);
    append_engagement_preview_lines(&state.scene, &mut line_vertices, [theme.row_hover.r, theme.row_hover.g, theme.row_hover.b, 1.0]);
    append_pick_target_hover_lines(&state.selection, theme, &camera, inner, &mut line_vertices);
    for attraction in &state.scene.attractions {
        let Some(from) = attraction.from else { continue };
        let Some(to) = attraction.to else { continue };
        let color = parse_color(attraction.color.as_deref().unwrap_or("#60a5fa"));
        line_vertices.push(LineVertex3d { position: [from[0] as f32, from[1] as f32, from[2] as f32], color });
        line_vertices.push(LineVertex3d { position: [to[0] as f32, to[1] as f32, to[2] as f32], color });
    }
    for volume in &state.scene.target_volumes {
        append_box_wireframe(&mut line_vertices, volume.origin.unwrap_or([0.0, 0.0, 0.0]), volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), volume.scale.unwrap_or([1.0, 1.0, 1.0]), parse_color(volume.color.as_deref().unwrap_or("#f472b6")));
    }
    let mut extra_draws = Vec::new();
    append_vortex_arrow_draws(&mut state.session, &mut state.selection, &mut state.authority, &mut state.retirement, &mut state.scene, gpu, &mut extra_draws);
    let vertex_instances = append_component_vertex_spheres();
    if !vertex_instances.is_empty() {
        let mesh_version = live_world3d_mesh_version(&state.scene, VERTEX_MARKER_MESH);
        if let Some(mesh) = state.scene.meshes.get(VERTEX_MARKER_MESH) {
            gpu.ensure_mesh(VERTEX_MARKER_MESH, mesh_version, *mesh);
            extra_draws.push(SceneDraw3d { mesh_key: VERTEX_MARKER_MESH.into(), mesh_version, instances: vertex_instances, shadow_role: Default::default() });
        }
    }
    let mut translucent_draws = Vec::new();
    append_component_face_translucent_overlays(state, gpu, &mut translucent_draws);
    append_tool_run_trace_draws(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, &mut state.trace, gpu, theme, &mut translucent_draws);
    if let Some(preview) = state.gumball.catalogue_drop_preview.clone() {
        let mesh_id = ghost_mesh_id(&state.scene, preview.mesh_url.as_deref());
        if !state.scene.meshes.contains_key(&mesh_id) {
            begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, &mesh_id, WorldPlaceholderKind::Box);
        }
        let mesh_version = live_world3d_mesh_version(&state.scene, &mesh_id);
        if let Some(mesh) = state.scene.meshes.get(&mesh_id).copied() {
            gpu.ensure_mesh(&mesh_id, mesh_version, mesh);
            translucent_draws.push(SceneDraw3d {
                mesh_key: mesh_id,
                mesh_version,
                instances: vec![Instance3d { component_source: None,
                    id: "catalogue-drop-preview".into(),
                    model: Instance3d::model_from_trs([preview.origin[0] as f32, preview.origin[1] as f32, preview.origin[2] as f32], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]),
                    color: {
                        let mut color = parse_color("#59bfff");
                        color[3] = 0.55;
                        color
                    },
                    selected: false,
                    hovered: false,
                 material: Default::default() }],
                shadow_role: Default::default(),
            });
        }
    }
    if let Some(preview) = state.gumball.brush_preview.clone() {
        if let Some(origin) = preview.origin {
            let mesh_id = ghost_mesh_id(&state.scene, preview.mesh_url.as_deref());
            if !state.scene.meshes.contains_key(&mesh_id) {
                begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, &mesh_id, WorldPlaceholderKind::Box);
            }
            let rotation = preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            let scale = preview_scale(preview.scale.as_ref());
            let mesh_version = live_world3d_mesh_version(&state.scene, &mesh_id);
            if let Some(mesh) = state.scene.meshes.get(&mesh_id).copied() {
                gpu.ensure_mesh(&mesh_id, mesh_version, mesh);
                translucent_draws.push(SceneDraw3d {
                    mesh_key: mesh_id,
                    mesh_version,
                    instances: vec![Instance3d { component_source: None,
                        id: "brush-preview".into(),
                        model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [rotation[0] as f32, rotation[1] as f32, rotation[2] as f32, rotation[3] as f32], scale),
                        color: {
                            let mut color = parse_color(preview.color.as_deref().unwrap_or("#59bfff"));
                            color[3] = 0.45;
                            color
                        },
                        selected: false,
                        hovered: false,
                     material: Default::default() }],
                    shadow_role: Default::default(),
                });
            }
        }
    }
    let mut textured_draws = Vec::new();
    if !state.scene.meshes.contains_key("reference-plane") {
        begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, "reference-plane", WorldPlaceholderKind::Plane);
    }
    let mut textured_instances = Vec::new();
    let reference_image_urls: Vec<String> = state.scene
        .references
        .iter()
        .filter(|reference| !reference.hidden.unwrap_or(false))
        .filter_map(|reference| reference.url.clone())
        .filter(|url| !world_reference_raster_ready(&state.scene, url))
        .filter(|url| !state.assets.asset_url_misses.contains(url))
        .collect();
    for url in reference_image_urls {
        let _ = reserve_world3d_asset_request(&mut state.session, &mut state.assets, WorldAssetRequestKind::ReferenceImage, &url);
    }
    let visible_references: Vec<WorldReferenceRecord> = state.scene.references.iter().filter(|reference| !reference.hidden.unwrap_or(false)).cloned().collect();
    for reference in visible_references {
        let Some(url) = reference.url.as_deref() else { continue };
        let origin = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let width = world_reference_width(&reference);
        let [width, height] = world_reference_plane_size(&state.scene, url, width);
        let visual = world_reference_visual(&state.selection, &reference, theme);
        textured_instances.push(TexturedInstance3d {
            texture_key: url.to_string(),
            model: Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [0.0, 0.0, 0.0, 1.0], [width, height, 1.0]),
            background: visual.background,
            appearance: [visual.content_opacity, 0.0, 0.0, 0.0],
        });
        if let Some(outline) = visual.outline {
            append_world_reference_outline(&mut line_vertices, origin, width, height, outline);
        }
        if let Some(lease) = reference_underlay_upload(&state.scene, url) {
            gpu.ensure_world_plane_texture(url, lease);
        }
    }
    if !textured_instances.is_empty() {
        textured_draws.push(TexturedDraw3d { instances: textured_instances });
    }
    if !state.scene.meshes.contains_key(GUMBALL_PLANE_MESH) {
        begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, GUMBALL_PLANE_MESH, WorldPlaceholderKind::Plane);
    }
    if !state.selection.selected_ids.is_empty() && state.session.active_utility == "select" && world3d_offers_transform_gumball(&state.session) {
        append_gumball_geometry(&mut line_vertices, &mut translucent_draws, gpu, &state.selection, &state.gumball, &state.scene, &camera, &state.scene.meshes, &state.scene.mesh_versions);
    }
    let paint_shadow_role = world3d_shadow_role(World3dShadowGeometry::Paint, shadow.enabled);
    for draw in extra_draws.iter_mut().chain(translucent_draws.iter_mut()).filter(|draw| draw.shadow_role == SceneShadowRole3d::default()) {
        draw.shadow_role = paint_shadow_role;
    }
    if let Some(section) = section.filter(|section| section.cap.is_some()) {
        append_world3d_section_cap(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, gpu, theme, &section, &culled_draws, &mut translucent_draws);
    }
    culled_draws.extend(extra_draws);
    culled_draws.extend(terrain_draws);
    sort_world3d_translucent_material_draws(&state.scene.meshes, view_proj, &mut material_draws);
    retain_ensured_world_draws(gpu, &mut shadow_draws);
    retain_ensured_world_draws(gpu, &mut culled_draws);
    retain_ensured_world_draws(gpu, &mut translucent_draws);
    retain_ensured_world_material_draws(gpu, &mut material_draws);
    ctx.draw.push_scene_pass(ScenePass3d {
        viewport: [inner.x, inner.y, inner.w, inner.h],
        viewport_mask: state.session.presentation.viewport_mask,
        render_profile: shadow_profile.render_profile(),
        clear_color: (state.session.presentation.viewport_mask == SceneViewportMask3d::Ellipse).then(|| clear_color.map(|color| [color.r, color.g, color.b, color.a])).flatten(),
        view_proj: view_proj.to_cols_array(),
        camera_position: camera.position.to_array(),
        light_dir,
        lighting,
        neutral_material,
        shadow,
        section_clip: section.map(|section| section.clip_plane().map(|component| component as f32)),
        procedural_grid,
        shadow_draws,
        draws: culled_draws,
        line_draws: if line_vertices.is_empty() { Vec::new() } else { vec![LineDraw3d { vertices: line_vertices }] },
        translucent_draws,
        material_draws,
        textured_draws,
        curvilinear: match state.camera.projection_spec.mode {
            Viewport3dProjectionMode::Curvilinear { fov, strength, .. } => Some(ui_wgpu::wgpu::SceneCurvilinear3d { fov_radians: (fov as f32).min(160.0).to_radians(), strength: strength as f32 }),
            _ => None,
        },
        ..Default::default()
    });
    if let Some(world) = modelling {
        let paints_legend = world.scalar_field.is_some() && matches!(state.overlay.scalar_field_status, World3dScalarFieldStatus::Applied | World3dScalarFieldStatus::Mismatch);
        if world.annotations.is_some() || paints_legend {
            let locale = state.session.modelling_locale;
            ctx.draw.push_scissor(inner);
            if let Some(layer) = world.annotations.as_ref() {
                paint_world3d_annotations(ctx, layer, &view_proj.to_cols_array(), inner, locale);
            }
            if let Some(field) = world.scalar_field.as_ref() {
                match state.overlay.scalar_field_status {
                    World3dScalarFieldStatus::Applied => paint_world3d_legend(ctx, field, inner, locale),
                    World3dScalarFieldStatus::Mismatch => paint_world3d_scalar_mismatch(ctx, inner, locale),
                    World3dScalarFieldStatus::None | World3dScalarFieldStatus::Missing => {}
                }
            }
            ctx.draw.pop_scissor();
        }
    }
    if state.session.presentation.interactive {
        if let Some(points) = world_marquee_overlay_points(&state.selection, &state.authority) {
        let lasso = state.selection.selection_method == "lasso";
        let crossing = marquee_is_crossing_from_path(&points, lasso);
        paint_selection_marquee(ctx.draw, theme, crossing, lasso, &points, true);
        }
    }
    if state.session.presentation.interactive && state.session.presentation.show_gizmo {
        gpu_gizmo::paint_orbit_view_gizmo(ctx, &camera, inner, state.camera.gizmo_hovered_tip);
    }
    for (index, status) in state.overlay.prepared_status.iter().flatten().enumerate() {
        if let Some(text) = status.text() {
            let y = inner.y + 20.0 + index as f32 * (theme.font_size_small + 6.0);
            draw_text(ctx, text, inner.x + 12.0, y, theme.font_size_small, theme.text);
        }
    }
    if let Some(status) = state.overlay.prepared_status[0] {
        let total = f64::from(status.total).max(1.0);
        let ratio = (status.progress[2].min(total) / total) as f32;
        let tone = if status.state == 6 { theme.text } else { theme.text_element };
        ctx.draw.push_solid([inner.x + 12.0, inner.y + 54.0, 160.0 * ratio, 3.0], tone);
    }
    if scene.world_3d.is_none() {
        draw_text(ctx, "world-3d (empty)", inner.x + 12.0, inner.y + 20.0, theme.font_size_small, theme.text_muted);
    }
    if world3d_cursor_work_pending(state) {
        gpu.request_cursor_wake();
    }
    if state.session.presentation.interactive {
        ctx.input.register_hit(HitTarget { rect: inner, event: None, control_id: Some(state.session.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None });
    }
}

pub fn step_component_face_overlay_build(state: &mut World3dState) {
    let Some(mut cursor) = state.overlay.face_overlay_build.take() else { return };
    match cursor.step(&state.selection, &state.gumball, &state.scene) {
        WorldFaceOverlayMeshStep::Pending => state.overlay.face_overlay_build = Some(cursor),
        WorldFaceOverlayMeshStep::Ready { index, color, key, lease } => match publish_world3d_mesh_lease(&mut state.interaction, &mut state.scene, key, lease) {
            Ok(()) => {
                cursor.published_colors[index] = Some(color);
                state.overlay.face_overlay_build = Some(cursor);
            }
            Err(rejected) => {
                cursor.owner = WorldPlaceholderOwner::Ready(rejected.value);
                cursor.retry_key = rejected.id;
                cursor.phase = WorldFaceOverlayPhase::Publish;
                cursor.close_started = false;
                state.overlay.face_overlay_build = Some(cursor);
            }
        },
        WorldFaceOverlayMeshStep::Complete { generation, revision, draw_generation, colors } => {
            state.overlay.face_overlay_retired_generation = state.overlay.face_overlay_generation.replace(generation);
            state.overlay.face_overlay_colors = colors;
            state.overlay.face_overlay_applied_revision = revision;
            state.overlay.face_overlay_applied_draw_generation = draw_generation;
        }
        WorldFaceOverlayMeshStep::Stale => {}
        WorldFaceOverlayMeshStep::Fault => mark_world_dynamic_fault(&mut state.session, &mut state.authority, &mut state.scene, WorldDynamicFault::Closing),
    }
}

pub(crate) fn append_component_face_translucent_overlays(state: &mut World3dState, gpu: &mut World3dBuildContext, translucent: &mut Vec<SceneDraw3d>) {
    if !face_component_mode_active(&state.selection) {
        state.overlay.face_overlay_applied_revision = u64::MAX;
        state.overlay.face_overlay_applied_draw_generation = u64::MAX;
        if let Some(cursor) = state.overlay.face_overlay_build.as_mut() {
            cursor.faulted = true;
            cursor.stale = true;
            step_component_face_overlay_build(state);
            return;
        }
        if state.overlay.face_overlay_generation.is_some() {
            state.overlay.face_overlay_retired_generation = state.overlay.face_overlay_generation.take();
            state.overlay.face_overlay_colors = [None; 3];
        }
        return;
    }
    let had_build = state.overlay.face_overlay_build.is_some();
    step_component_face_overlay_build(state);
    if !had_build
        && state.overlay.face_overlay_build.is_none()
        && state.overlay.face_overlay_retired_generation.is_none()
        && (state.overlay.face_overlay_applied_revision != state.session.interaction_revision || state.overlay.face_overlay_applied_draw_generation != state.scene.draw_generation)
        && state.scene.dynamic_mesh_close.is_none()
        && state.scene.dynamic_blocked_mesh.is_none()
        && state.session.snapshot_fault.is_none()
    {
        let generation = state.scene.placeholder_generation.checked_add(3);
        match generation.and_then(|next| WorldFaceOverlayMeshCursor::new(&state.session.surface_id, state.scene.placeholder_generation + 1, state.session.interaction_revision, state.scene.draw_generation).ok().map(|cursor| (next, cursor))) {
            Some((next, cursor)) => {
                state.scene.placeholder_generation = next;
                state.overlay.face_overlay_build = Some(cursor);
            }
            None => mark_world_dynamic_fault(&mut state.session, &mut state.authority, &mut state.scene, WorldDynamicFault::ByteCapacity),
        }
    }
    let Some(generation) = state.overlay.face_overlay_generation else { return };
    for index in 0..3 {
        let Some(color) = state.overlay.face_overlay_colors[index] else { continue };
        let mesh_key = format!("component-face-overlay:{}:{generation}:{index}", state.session.surface_id);
        let Some(mesh) = state.scene.meshes.get(&mesh_key) else { continue };
        let mesh_version = live_world3d_mesh_version(&state.scene, &mesh_key);
        gpu.ensure_mesh(&mesh_key, mesh_version, *mesh);
        translucent.push(SceneDraw3d { mesh_key, mesh_version, instances: vec![Instance3d { component_source: None, id: format!("face-overlay-{index}"), model: Mat4::identity(), color, selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    }
}

/// 🔢️ The revision this apply INSTALLS, decided BEFORE the draw rebuild it began is sealed.
///
/// 🩸️ `lease.revision` is this world's proposed source-publication revision: the bridge reserves
/// the next interaction revision for each changed payload — `publish_world3d_scene_bridge_snapshot`
/// is its one production writer —
/// so adopting it verbatim moves the counter BACKWARDS whenever anything bumped the revision
/// while the lease was in flight: a document-lane change (`sync_world3d_scene_document_lanes`),
/// a parallel framing (`sync_world3d_projection_content_frame`), a fit, a camera report. The
/// rollback then invalidated the very draws this apply had just built, because
/// `step_world3d_draw_rebuild` refuses a cursor whose `descriptor.revision` no longer equals
/// `state.interaction_revision`. Measured on 6118: generation3d's `procedural-preview` sat at
/// `state-meshes=3 state-instances=0 state-draws=0` with three `world3d draw rebuild step=Stale`
/// lines immediately after `world3d delivery applied`, and the tessellated column never painted
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w14b-generation3d-labels-preview-layout.md`).
///
/// 🧱️ The draws this apply built ARE the content of the revision it installs, so they are
/// re-stamped with it rather than left carrying whatever revision happened to be in force
/// when the first `Mesh` page opened the rebuild.
///
/// 📐️ A LOCALLY selected projection is view state the wire never overrides — React holds
/// it in `externalPendingProjectionSpec` and only falls back to the delivered
/// `cameraState.projectionSpec` when the pane has none (`🌐️World3dHost/🟦️.tsx`).
///
/// 🩸️ Without this the switch undid itself within one round trip: the template press
/// queues a settle, the settle publishes `setCamera`, the guest echoes a camera back, and
/// `setCamera`'s payload has no `projection` member at all (`orbit_camera_action`), so the
/// echo always arrives in the DELIVERED family. Measured live: the press moved the pane's
/// target onto the content centre and `liveCamera` still read `perspective` 8 s later
/// (`📓️w9b-projection-pane-framing-grid-materials.md` §1).
pub fn step_world3d_snapshot(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> World3dSnapshotApplyStep {
    if context.should_yield() {
        return World3dSnapshotApplyStep::Pending;
    }
    let Some(mut cursor) = state.bridge.snapshot_apply.take() else {
        return World3dSnapshotApplyStep::Idle;
    };
    if cursor.faulted {
        state.bridge.snapshot_apply = Some(cursor);
        return World3dSnapshotApplyStep::Fault;
    }
    if cursor.page == cursor.lease.page_count {
        let revision = state.session.interaction_revision.max(cursor.lease.revision);
        if cursor.draw_started {
            if let Some(rebuild) = state.scene.draw_rebuild.as_mut() {
                rebuild.descriptor.revision = revision;
            }
            if world3d_draw_rebuild_seal(&mut state.scene).is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        if let Some(orbit) = cursor.staged_orbit.take() {
            state.camera.orbit = if state.camera.projection_selected {
                OrbitController { projection: ui_wgpu::wgpu::projection_spec_family(state.camera.projection_spec), fov_y: ui_wgpu::wgpu::projection_spec_fov_degrees(state.camera.projection_spec).to_radians(), ..orbit }
            } else {
                orbit
            };
        }
        state.session.interaction_revision = revision;
        if let Some(gesture) = state.authority.interaction_authority.as_mut().and_then(|authority| authority.gumball.as_mut()).filter(|gesture| gesture.live && gesture.streamed) {
            gesture.revision = revision;
        }
        state.bridge.snapshot_lease = Some(cursor.lease);
        state.overlay.prepared_status = cursor.status;
        state.session.snapshot_fault = None;
        context.consume_fuel(1);
        return World3dSnapshotApplyStep::Complete;
    }
    let resolved = world3d_snapshot_with_page(cursor.lease, cursor.page, |page| {
        let item_count = page.item_count();
        let item = page.item(usize::from(cursor.item)).copied();
        let id = item.and_then(|item| item.strings[0]).and_then(|span| page.string(span)).map(str::to_owned);
        let status = item.and_then(|item| {
            let text = item.strings[1].and_then(|span| page.string(span))?;
            World3dPreparedStatus::from_parts(text, [item.numbers[0], item.numbers[1], item.numbers[2]], item.indexes[0], item.indexes[1])
        });
        let component_source = if page.kind() == World3dSnapshotPageKind::Instance {
            item.map(|item| match (item.strings[1], item.strings[2]) {
                (None, None) => Ok(None),
                (Some(handle), Some(revision)) => ComponentSource3d::new(page.string(handle).ok_or(World3dSnapshotFault::PageState)?, page.string(revision).ok_or(World3dSnapshotFault::PageState)?).map(Some).ok_or(World3dSnapshotFault::PageState),
                _ => Err(World3dSnapshotFault::PageState),
            }).transpose()
        } else { Ok(None) };
        (page.kind(), item_count, item, id, status, component_source)
    });
    let (kind, item_count, item, id, status, component_source) = match resolved {
        Ok(resolved) => resolved,
        Err(fault) => {
            state.session.snapshot_fault = Some(fault);
            cursor.faulted = true;
            state.bridge.snapshot_apply = Some(cursor);
            return World3dSnapshotApplyStep::Stale;
        }
    };
    if usize::from(cursor.item) == item_count {
        cursor.page += 1;
        cursor.item = 0;
        state.bridge.snapshot_apply = Some(cursor);
        context.consume_fuel(1);
        return World3dSnapshotApplyStep::Pending;
    }
    let component_source = match component_source {
        Ok(source) => source.flatten(),
        Err(fault) => { state.session.snapshot_fault = Some(fault); cursor.faulted = true; state.bridge.snapshot_apply = Some(cursor); return World3dSnapshotApplyStep::Fault; }
    };
    let Some(item) = item else {
        state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
        cursor.faulted = true;
        state.bridge.snapshot_apply = Some(cursor);
        return World3dSnapshotApplyStep::Fault;
    };
    match kind {
        World3dSnapshotPageKind::Mesh if item.flags == 30 && item.index_len >= 2 => {
            let Some(mesh_key) = id.as_deref() else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let instance_count = item.indexes[0];
            let byte_count = item.indexes[1].checked_add(size_of::<SceneDraw3d>() as u32).and_then(|bytes| bytes.checked_add(instance_count.checked_mul(size_of::<Instance3d>() as u32)?));
            let Some(byte_count) = byte_count else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let draw_bytes = item.indexes[1];
            let permit = match world3d_snapshot_claim_draw_permit(cursor.lease, 1, instance_count, draw_bytes) {
                Ok(permit) => permit,
                Err(fault) => {
                    state.session.snapshot_fault = Some(fault);
                    cursor.faulted = true;
                    state.bridge.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                }
            };
            if !scene_mesh_awaits_its_asset(&state.scene, mesh_key) {
                begin_world_placeholder_mesh(&mut state.session, &mut state.authority, &mut state.retirement, &mut state.scene, mesh_key, WorldPlaceholderKind::Box);
            }
            let descriptor = WorldDrawRebuildDescriptor { generation: state.scene.draw_generation.wrapping_add(1), revision: state.session.interaction_revision, draw_count: 1, instance_count, byte_count };
            let mesh_version = live_world3d_mesh_version(&state.scene, mesh_key);
            let admitted = begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, descriptor).and_then(|()| world3d_draw_rebuild_admit_draw(&mut state.scene, mesh_key, mesh_version, u16::try_from(instance_count).map_err(|_| WorldDynamicFault::InstanceCapacity)?));
            if admitted.is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
            cursor.draw_permit = Some(permit);
            cursor.draw_started = true;
            cursor.draw_index = 0;
        }
        World3dSnapshotPageKind::Mesh if item.flags == 31 && item.index_len >= 6 => {
            let Some(mesh_key) = id.as_deref() else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let (own_instances, ordinal, total_draws, total_instances, total_bytes) = (item.indexes[0], item.indexes[2], item.indexes[3], item.indexes[4], item.indexes[5]);
            let Ok(total_draws) = u16::try_from(total_draws) else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            if ordinal == 0 {
                let byte_count = total_bytes.checked_add(u32::from(total_draws).saturating_mul(size_of::<SceneDraw3d>() as u32)).and_then(|bytes| bytes.checked_add(total_instances.checked_mul(size_of::<Instance3d>() as u32)?));
                let permit = byte_count.ok_or(World3dSnapshotFault::Capacity).and_then(|_| world3d_snapshot_claim_draw_permit(cursor.lease, total_draws, total_instances, total_bytes));
                let (Some(byte_count), Ok(permit)) = (byte_count, permit) else {
                    state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                    cursor.faulted = true;
                    state.bridge.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                };
                let descriptor = WorldDrawRebuildDescriptor { generation: state.scene.draw_generation.wrapping_add(1), revision: state.session.interaction_revision, draw_count: total_draws, instance_count: total_instances, byte_count };
                if begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, descriptor).is_err() {
                    state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                    cursor.faulted = true;
                    state.bridge.snapshot_apply = Some(cursor);
                    return World3dSnapshotApplyStep::Fault;
                }
                cursor.draw_permit = Some(permit);
                cursor.draw_started = true;
            }
            let mesh_version = live_world3d_mesh_version(&state.scene, mesh_key);
            let admitted = u16::try_from(own_instances).map_err(|_| WorldDynamicFault::InstanceCapacity).and_then(|count| world3d_draw_rebuild_admit_draw(&mut state.scene, mesh_key, mesh_version, count));
            let Ok(ordinal) = u16::try_from(ordinal) else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            if admitted.is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
            cursor.draw_index = ordinal;
        }
        World3dSnapshotPageKind::Instance if item.number_len >= 14 && cursor.draw_started => {
            let Some(id) = id.as_deref() else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let position = [item.numbers[0] as f32, item.numbers[1] as f32, item.numbers[2] as f32];
            let rotation = [item.numbers[3] as f32, item.numbers[4] as f32, item.numbers[5] as f32, item.numbers[6] as f32];
            let scale = [item.numbers[7] as f32, item.numbers[8] as f32, item.numbers[9] as f32];
            let color = [item.numbers[10] as f32, item.numbers[11] as f32, item.numbers[12] as f32, item.numbers[13] as f32];
            let model = Instance3d::model_from_trs(position, rotation, scale);
            if world3d_draw_rebuild_admit_retained_instance(&mut state.scene, cursor.draw_index, id, model, color, SceneColorSource3d::from_snapshot_flag(item.flags), false, false, component_source).is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        World3dSnapshotPageKind::Camera if item.number_len >= 12 => {
            let projection = if item.numbers[11] >= 0.5 { CameraProjection3d::Orthographic } else { CameraProjection3d::Perspective };
            cursor.staged_orbit = Some(OrbitController::from_camera(&Camera3d {
                position: Vec3::new(item.numbers[0] as f32, item.numbers[1] as f32, item.numbers[2] as f32),
                target: Vec3::new(item.numbers[3] as f32, item.numbers[4] as f32, item.numbers[5] as f32),
                up: Vec3::new(item.numbers[6] as f32, item.numbers[7] as f32, item.numbers[8] as f32),
                fov_y: item.numbers[9] as f32 * std::f32::consts::PI / 180.0,
                near: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_NEAR,
                far: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_MIN_FAR,
                projection,
                zoom: item.numbers[10] as f32,
            }));
        }
        World3dSnapshotPageKind::Camera => {
            state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
            cursor.faulted = true;
            state.bridge.snapshot_apply = Some(cursor);
            return World3dSnapshotApplyStep::Fault;
        }
        World3dSnapshotPageKind::Status if (10..=14).contains(&item.flags) && item.number_len >= 14 && cursor.draw_started => {
            let Some(id) = id.as_deref() else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            let vector = [item.numbers[0], item.numbers[1], item.numbers[2]];
            let magnitude = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt().max(item.numbers[3].abs()).max(1.0e-6) as f32;
            let position = [item.numbers[11] as f32, item.numbers[12] as f32, item.numbers[13] as f32];
            let color = match item.flags {
                10 => [0.957, 0.447, 0.714, 1.0],
                11 => [0.918, 0.702, 0.031, 1.0],
                12 => [0.984, 0.443, 0.522, 1.0],
                13 => [0.133, 0.827, 0.933, 1.0],
                _ => [0.655, 0.545, 0.98, 1.0],
            };
            let model = Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [0.025, 0.025, magnitude.min(1.0)]);
            if world3d_draw_rebuild_admit_instance(&mut state.scene, 0, id, model, color, false, false).is_err() {
                state.session.snapshot_fault = Some(World3dSnapshotFault::Capacity);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            }
        }
        World3dSnapshotPageKind::Status if item.flags == 20 && item.number_len >= 3 && item.index_len >= 2 => {
            let Some(status) = status else {
                state.session.snapshot_fault = Some(World3dSnapshotFault::PageState);
                cursor.faulted = true;
                state.bridge.snapshot_apply = Some(cursor);
                return World3dSnapshotApplyStep::Fault;
            };
            cursor.status[usize::from(id.as_deref() == Some("de"))] = Some(status);
        }
        _ => {}
    }
    cursor.item += 1;
    state.bridge.snapshot_apply = Some(cursor);
    context.consume_fuel(1);
    World3dSnapshotApplyStep::Pending
}

/// 🖱️📋️ A right RELEASE owes the unconditional camera report and NOTHING else: the menu is
/// the shell's, opened on the press (see the authority's own right-release arm).
#[cfg(any(test, feature = "testkit"))]
pub fn handle_world3d_pointer_button(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16, modifiers: &PointerModifiers) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(&state.session);
    if !inner.contains(x, y) {
        return None;
    }
    let shift = modifiers.shift;
    let ctrl = modifiers.ctrl;
    if down {
        if button == 0 {
            if state.session.active_utility == "surfaceBrush" {
                return None;
            }
            if state.selection.interaction_mode == "paint" {
                state.selection.paint_stroke_active = true;
                let (object_id, u, v) = pick_paint_hit(&state.session, &state.camera, &state.scene, x, y, inner)?;
                return Some(ActionDescriptor { controller_id: state.session.controller_id.clone(), action: "paintAt".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.session.surface_id, "objectId": object_id, "u": u, "v": v, "phase": "stream" })) });
            }
            if state.session.active_utility == "brush" || (state.session.active_utility == "select" && state.selection.granularity == "vertex") {
                if let Some(full_id) = pick_vortex_at(&state.session, &state.camera, &state.scene, x, y, inner) {
                    let merge = world_merge_mode(shift, ctrl).wire_label();
                    return Some(ActionDescriptor { controller_id: state.session.controller_id.clone(), action: "worldVortexSelect".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.session.surface_id, "fullId": full_id, "merge": merge })) });
                }
            } else if state.session.active_utility == "select" {
                if !state.selection.selected_ids.is_empty() {
                    if let Some(handle) = pick_gumball_handle_at(&state.session, &state.camera, &state.selection, &state.gumball, &state.scene, x, y, inner) {
                        start_gumball_drag(&mut state.camera, &mut state.selection, &mut state.gumball, &mut state.scene, handle, x, y, inner);
                        return None;
                    }
                }
                if state.gumball.gumball_handle.is_none() && !component_mode_active(&state.selection) {
                    state.selection.press_object_id = pick_instance_at(&state.session, &state.camera, &state.scene, x, y, inner);
                } else {
                    state.selection.press_object_id = None;
                }
            }
            state.selection.marquee_active = true;
            state.selection.marquee_points = vec![[x, y]];
            return None;
        }
        if button == 2 {
            state.selection.marquee_active = false;
            state.selection.marquee_points.clear();
            state.selection.right_press_point = Some([x, y]);
        }
        return None;
    }
    if button == 0 && state.selection.marquee_active {
        state.selection.marquee_active = false;
        if is_click_gesture(&state.selection, x, y) {
            state.selection.marquee_points.clear();
            state.selection.marquee_preview_ids.clear();
            state.selection.press_object_id = None;
            state.selection.drag_object_id = None;
            state.selection.drag_last_position = None;
            if state.session.active_utility == "surfaceBrush" {
                if let Some((object_id, position, normal)) = pick_surface_at(&state.session, &state.camera, &state.scene, x, y, inner) {
                    return Some(ActionDescriptor {
                        controller_id: state.session.controller_id.clone(),
                        action: "worldSurfacePlace".into(),
                        args: Some(semio_framework::dsl_value!({
                            "surfaceId": state.session.surface_id,
                            "pane": state.session.surface_id,
                            "objectId": object_id,
                            "position": position,
                            "normal": normal,
                        })),
                    });
                }
                return None;
            }
            if state.gumball.gumball_handle.is_none() {
                return pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, x, y, inner, shift, ctrl);
            }
        } else {
            return marquee_select_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, render_pick_viewport(&state.session), shift, ctrl);
        }
    }
    if button == 0 && state.selection.interaction_mode == "paint" && state.selection.paint_stroke_active {
        state.selection.paint_stroke_active = false;
        let mut actions = Vec::new();
        actions.push(ActionDescriptor { controller_id: state.session.controller_id.clone(), action: "paintAt".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.session.surface_id, "phase": "commit" })) });
        return actions.first().cloned();
    }
    if button == 0 {
        if state.session.active_utility == "surfaceBrush" {
            if let Some((object_id, position, normal)) = pick_surface_at(&state.session, &state.camera, &state.scene, x, y, inner) {
                return Some(ActionDescriptor {
                    controller_id: state.session.controller_id.clone(),
                    action: "worldSurfacePlace".into(),
                    args: Some(semio_framework::dsl_value!({
                        "surfaceId": state.session.surface_id,
                        "pane": state.session.surface_id,
                        "objectId": object_id,
                        "position": position,
                        "normal": normal,
                    })),
                });
            }
            return None;
        }
        if let Some(action) = gumball_commit_action(&state.session, &state.selection, &state.gumball) {
            state.gumball.gumball_handle = None;
            reset_gumball_preview(&mut state.gumball);
            return Some(action);
        }
        state.gumball.gumball_handle = None;
        reset_gumball_preview(&mut state.gumball);
        state.selection.press_object_id = None;
        if let Some(object_id) = state.selection.drag_object_id.take() {
            if let Some(position) = state.selection.drag_last_position {
                if !is_click_gesture(&state.selection, x, y) {
                    return Some(ActionDescriptor {
                        controller_id: state.session.controller_id.clone(),
                        action: "worldRelocate".into(),
                        args: Some(semio_framework::dsl_value!({
                            "surfaceId": state.session.surface_id,
                            "objectId": object_id,
                            "position": position,
                        })),
                    });
                }
            }
        }
        if state.session.active_utility == "brush" {
            if let Some(preview) = state.gumball.brush_preview.clone() {
                if let (Some(target), Some(kind), Some(index)) = (preview.target_vortex_full_id, preview.object_kind_id, preview.source_vortex_index) {
                    let origin = preview.origin.unwrap_or([0.0, 0.0, 0.0]);
                    return Some(ActionDescriptor {
                        controller_id: state.session.controller_id.clone(),
                        action: "addBrushObject".into(),
                        args: Some(semio_framework::dsl_value!({
                            "targetVortexFullId": target,
                            "objectKindId": kind,
                            "sourceVortexIndex": index,
                            "origin": origin,
                            "orientation": preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                            "scale": preview.scale.as_ref().map(semio_framework::DslValue::from),
                        })),
                    });
                }
            }
        }
        return pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, x, y, inner, shift, ctrl);
    }
    if button == 2 {
        state.selection.right_press_point = None;
        return Some(orbit_camera_action(&state.session, &state.camera));
    }
    if button == 1 {
        return Some(orbit_camera_action(&state.session, &state.camera));
    }
    None
}

pub fn step_world3d_interaction(state: &mut World3dState, generation: u64, input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>, context: &mut semio_framework_job::StepContext<'_>) -> WorldInteractionAuthorityStep {
    let Some(mut authority) = state.authority.interaction_authority.take() else {
        return WorldInteractionAuthorityStep::Fault;
    };
    let step = authority.step(&state.selection, &state.gumball, &state.scene, generation, input, context);
    state.authority.interaction_authority = Some(authority);
    step
}

pub fn step_world3d_dynamic_retirement(state: &mut World3dState, context: &mut semio_framework_job::StepContext<'_>) -> bool {
    if context.retirement_should_yield() {
        return false;
    }
    if let Some(cursor) = state.scene.placeholder_build.as_mut() {
        if !cursor.close_step() || !cursor.terminal_is_empty() {
            context.consume_fuel(1);
            return false;
        }
        state.scene.placeholder_build = None;
        context.consume_fuel(1);
        return false;
    }
    if state.terrain.terrain_pending.is_some() || state.terrain.terrain_build.is_some() {return false;}
    if let Some(cursor) = state.overlay.face_overlay_build.as_mut() {
        if !cursor.close_step() || !cursor.terminal_is_empty() {
            context.consume_fuel(1);
            return false;
        }
        state.overlay.face_overlay_build = None;
        context.consume_fuel(1);
        return false;
    }
    if let Some(index) = state.overlay.face_overlay_colors.iter().position(Option::is_some) {
        state.overlay.face_overlay_colors[index] = None;
        context.consume_fuel(1);
        return false;
    }
    if state.overlay.face_overlay_generation.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if state.overlay.face_overlay_retired_generation.take().is_some() {
        context.consume_fuel(1);
        return false;
    }
    if !state.assets.asset_io.terminal_is_empty() {
        let _ = state.assets.asset_io.close_step();
        context.consume_fuel(1);
        return false;
    }
    if !close_world3d_draw_rebuild_step(&mut state.scene, context) {
        return false;
    }
    let Some(mut retirement) = state.retirement.dynamic_retirement.take() else {
        return true;
    };
    let complete = retirement.step(&state.selection, &state.gumball, &state.scene,context);
    context.consume_fuel(1);
    if complete && retirement.terminal_is_empty() {
        return true;
    }
    state.retirement.dynamic_retirement = Some(retirement);
    false
}

pub fn world3d_dynamic_retirement_terminal_is_empty(state: &World3dState) -> bool {
    state.retirement.dynamic_retirement.is_none()
        && state.bridge.scene_bridge.is_none()
        && state.bridge.scene_bridge_close.is_none()
        && state.bridge.scene_bridge_lease.is_none()
        && state.bridge.scene_bridge_retired.is_none()
        && state.scene.dynamic_blocked_owner.is_none()
        && state.scene.dynamic_mesh_close.is_none()
        && state.scene.dynamic_blocked_mesh.is_none()
        && state.scene.native_mesh_admission.is_none()
        && state.scene.placeholder_build.is_none()
        && state.terrain.terrain_build.is_none() && state.terrain.terrain_pending.is_none()
        && state.overlay.face_overlay_build.is_none()
        && state.overlay.face_overlay_generation.is_none()
        && state.overlay.face_overlay_retired_generation.is_none()
        && state.overlay.face_overlay_colors.iter().all(Option::is_none)
        && state.camera.fit_bounds_cursor.is_none()
        && world3d_draw_rebuild_terminal_is_empty(&state.scene)
        && state.scene.meshes.is_empty()
        && state.scene.mesh_versions.is_empty()
        && state.scene.draws.is_empty()
        && state.scene.reference_pixels.is_empty()
        && state.scene.mesh_paint_textures.is_empty()
        && state.interaction.instance_interaction_ids.is_empty()
        && state.interaction.instance_interaction_granularity_ids.is_empty()
        && state.selection.local_hover_granularity_id.is_none()
        && state.assets.asset_io.terminal_is_empty()
}

/// ⏰️ Whether this surface still owes its own frame — the one predicate the renderer turns into a
/// cursor wake, and therefore the only reason a settled, event-driven shell schedules another frame
/// for a World3d surface at all.
///
/// ⚖️ It used to name only the three MESH builders. The two cursors that turn a producer's
/// `meshes_json`/`instances_json` into `state.draws` — the scene bridge and the typed snapshot apply
/// — were missing, so a shell that had quiesced never scheduled the frames they needed: measured on
/// 6118 as `procedural-preview` parked at `bridge=false bridge-lease=true apply=true state-meshes=2
/// draws=0` for 95 s with the solid `extrude@solid` sitting unconsumed in its mesh lane
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Every bounded cursor the frame transaction's
/// `World3dSnapshot` phase drives belongs here, or its work only advances while something ELSE keeps
/// the frames coming.
pub fn world3d_cursor_work_pending(state: &World3dState) -> bool {
    state.scene.native_mesh_admission.is_some()
        || state.scene.placeholder_build.is_some()
        || state.terrain.terrain_build.is_some() || state.terrain.terrain_pending.is_some()
        || state.overlay.face_overlay_build.is_some()
        || state.bridge.scene_bridge.is_some()
        || state.bridge.snapshot_apply.is_some()
        || state.scene.draw_rebuild.is_some()
        || state.scene.retired_draws.is_some()
        || state.retirement.dynamic_retirement.is_some()
        || state.bridge.scene_bridge_retired.is_some()
        || state.bridge.scene_bridge_lease.is_some_and(|lease| state.bridge.snapshot_lease != Some(lease))
        || world3d_camera_fit_work_pending(&state.camera, &state.scene)
}

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️testkit/🦀️.rs"]
mod testkit;
