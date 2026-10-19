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
use semio_framework_ui_declaration::layout::ActionDescriptor;
use ui_scene::math::{Camera3d, camera_grid_fade_distance, CameraProjection3d, ComponentSource3d, Instance3d, LineVertex3d, lod_grid_step_world, marquee_is_crossing_from_path, Mat4, Mesh3dLease, OrbitController, quat_from_basis, SceneColorSource3d, SceneDraw3d, Vec3};
use semio_framework_ui_wgpu_input::{HitKind, HitTarget, PointerModifiers};
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::{Rgba, Theme};
use semio_framework_ui_declaration::ui::{UiComponentSceneNode, world3d_snapshot_claim_draw_permit, world3d_snapshot_with_page, World3dPresentation, World3dPresentationClear, World3dSnapshotFault, World3dSnapshotPageKind};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] use ui_scene::math::{aabb_intersects_frustum, directional_shadow_frustum_planes, frustum_planes, LineDraw3d, ProceduralGrid3d, SceneInstanceMaterial3d, SceneMaterialAlpha3d, SceneMaterialDraw3d, SceneMaterialKind3d, ScenePass3d, SceneSectionRole3d, SceneShadowRole3d, SceneViewportMask3d, TexturedDraw3d, TexturedInstance3d, transform_aabb};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] use semio_framework_ui_draw_contract::paint_selection_marquee;
use semio_framework_tool_run::ToolRunTraceCursor;
use semio_framework_ui_viewport::{Viewport3dProjectionFramePolicy, Viewport3dProjectionMode};
#[cfg(any(test, feature = "testkit"))] use semio_framework_ui_draw_contract::gizmo;
use serde_json::json;

pub struct World3dState {
    /// 📸️ Surface identity, bounds, domain binding, utilities and presentation of one 3D surface.
    pub session: WorldSessionState,
    /// 📷️ Orbit rig, projection, camera fit and camera sync.
    pub camera: WorldCameraState,
    /// 👉️ Hover, selection, marquee and pick-target state.
    pub selection: WorldSelectionState,
    /// 🧭️ Gumball, brush preview, catalogue drop preview and relocate drag.
    pub gumball: WorldGumballState,
    /// 🤝️ Interaction mesh/object registries and instance interaction ids.
    pub interaction: WorldInteractionState,
    /// 👑️ The interaction authority state machine with its active cursor or job.
    pub authority: WorldAuthorityState,
    /// 🖱️ The in-flight dynamic retirement cursor.
    pub retirement: WorldRetirementState,
    /// 🎬️ Resident meshes, draws, scene records (vortices, references, lod, environment), placeholder meshes and dynamic retirement slots.
    pub scene: WorldSceneState,
    /// 🗺️ Terrain style, tiles, session and build.
    pub terrain: WorldTerrainState,
    /// 💬️ Face overlay build, sub-element paint, scalar field and prepared status.
    pub overlay: WorldOverlayState,
    /// 🌉️ Scene bridge, snapshot apply and scene digests.
    pub bridge: WorldBridgeState,
    /// 🎞️ Asset io authority, url bookkeeping and brush mesh announce.
    pub assets: WorldAssetsState,
    /// 📒️ Tool run trace layer.
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

#[path = "🔄️sync-and-overlay/🦀️.rs"]
mod sync_and_overlay;
pub use sync_and_overlay::*;

#[path = "🎬️render/🦀️.rs"]
mod render;
pub use render::*;

#[path = "📥️snapshot-and-input/🦀️.rs"]
mod snapshot_and_input;
pub use snapshot_and_input::*;

#[cfg(any(test, feature = "testkit"))]
#[path = "🧰️testkit/🦀️.rs"]
mod testkit;
