//! 🧪️ host tests (🔬️unit): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{tool_run_trace, World3dBuildContext, WORLD_DYNAMIC_DRAW_BYTE_CAPACITY, WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_INTERACTION_INTENT_CAPACITY, WORLD_INTERACTION_MARQUEE_POINT_CAPACITY, WORLD_INTERACTION_OBJECT_CAPACITY, world_mesh_ownership_grant, world_pick_target_ray_score, world_pick_targets_from_json, world_scene_raster_pool, WorldBrushPreviewRecord, WorldCancelReason, WorldCursorWakeAuthority, WorldDrawRebuildDescriptor, WorldDrawRebuildStep, WorldDynamicFault, WorldInteractionIntent, WorldInteractionMeshToken, WorldInteractionObjectKind, WorldInteractionObjectToken, WorldInteractionPhase, WorldInteractionRegistryBuildPhase, WorldInteractionStep, WorldReferenceRecord, WorldSceneRaster, WorldVortexRecord};
use semio_framework_test_fixtures::fixture_text;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_model::WorldMeshLodEntry;
use semio_framework_os_infinite_world_components::{FACE_OVERLAY_OFFSET, GumballHandle, mesh_vertex, world_mesh_triangle, world_mesh_vertex, WorldFaceOverlayMeshCursor, WorldFaceOverlayMeshStep};
use semio_framework_os_infinite_world_cursors::{render_pick_viewport, resolved_domain_granularity_id, world3d_gumball_config, WORLD_BRUSH_COPY_CHUNK_BYTES, world_gumball_identity_motion, WORLD_GUMBALL_SELECTED_CAPACITY, WORLD_INTERACTION_DOMAIN_ID, WORLD_MARQUEE_RESULT_PAGE_CAPACITY, WorldBrushCommitJob, WorldComponentHit, WorldComponentKind, WorldComponentMarqueePublishJob, WorldComponentPickCursor, WorldComponentPickPurpose, WorldGumballCommitJob, WorldGumballGesture, WorldGumballPhase, WorldGumballPickCursor, WorldGumballTarget, WorldInteractionAuthorityStep, WorldMarqueeGesture, WorldMarqueePickCursor, WorldMarqueePublishJob, WorldMarqueeResult, WorldMarqueeResultPages, WorldObjectPickCursor, WorldObjectPickPurpose, WorldRayPickCursor, WorldRayPickPurpose};
use semio_framework_os_infinite_world_scene::{environment_scene_material, mark_world3d_asset_miss, MeshStyleKind, MeshStyleState, offer_missing_mesh_fetches, publish_world3d_mesh_lease, reserve_world3d_asset_request, resolve_mesh_style, retire_cancelled_world3d_asset_step, return_world3d_asset, step_world3d_terrain_retirement, take_next_world3d_asset, terrain_family_visible, world3d_asset_url_missed, WorldAssetRequestKind, WorldTerrainMeshPublicationStep};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_scene::{resolve_physical_mesh_id, TerrainMeshOracleData};
use semio_framework_os_infinite_world_bridge::{append_component_overlays, append_component_vertex_spheres, append_engagement_preview_lines, begin_world3d_draw_rebuild, begin_world3d_dynamic_retirement, close_world3d_draw_rebuild_step, publish_world3d_asset_mesh_lease, reference_image_aspect, reference_underlay_upload, VERTEX_BASE_SCALE, VERTEX_HOVER_SCALE, vertex_marker_half_extent, world3d_draw_rebuild_admit_draw, world3d_draw_rebuild_admit_instance, world3d_draw_rebuild_seal, world3d_draw_rebuild_terminal_is_empty, World3dCameraFitStep, World3dSceneBridgeStep, World3dSnapshotApplyStep, world_raster_upload_in_pool, world_reference_raster_ready, world_reference_raster_ready_in_pool, world_units_per_pixel, WORLD_VERTEX_DOT_PX, WorldInteractionRegistryBuildCursor};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_bridge::world3d_standard_material_for_mesh;
use semio_framework_os_infinite_world_interaction::{apply_decoded_reference_raster, apply_world_action_preview, begin_world3d_interaction_close, close_world3d_interaction_step, enqueue_world3d_event, enqueue_world3d_intent, ghost_mesh_id, plan_world3d_camera_settle, plan_world3d_drag, plan_world3d_paint_abort, plan_world3d_paint_release, plan_world3d_wheel, publish_world3d_plan_step, retained_gumball_preview_model, step_world3d_draw_rebuild, store_mesh, world3d_cancel_owed, world3d_interaction_front_generation, world3d_interaction_terminal_is_empty, world_marquee_overlay_points, WorldInteractionAuthority};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_interaction::{marquee_select_action, pick_component_at, pick_select_action, scene_geometry, update_marquee_preview};
use semio_framework_os_infinite_world_host::{apply_runtime_draw_flags, step_component_face_overlay_build, step_world3d_camera_fit, step_world3d_dynamic_retirement, step_world3d_interaction, step_world3d_scene_bridge, step_world3d_snapshot, sync_world3d_state, world3d_dynamic_retirement_terminal_is_empty, World3dState};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_host::handle_world3d_pointer_button;
use semio_framework_ui_declaration::layout::ActionDescriptor;
use ui_scene::math::{Camera3d, CameraProjection3d, Instance3d, marquee_is_crossing_from_path, Mat4, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_seal, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, OrbitController, SceneColorSource3d, SceneDraw3d, Vec3};
use semio_framework_ui_wgpu_input::PointerModifiers;
use semio_framework_ui_prepared::PreparedRenderUpload;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_raster_owner::{SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterPool, SceneRasterProfile, SceneRasterWriteMode};
use semio_framework_ui_declaration::ui::{SurfaceKind, UiComponentSceneNode, UiPresence, World3dScene, World3dSnapshotItem, World3dSnapshotPageKind};
use semio_framework_ui_theme::Theme;
use std::sync::Mutex;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] use ui_scene::math::SceneMaterial3d;
#[cfg(test)] use semio_framework_ui_raster_owner::SceneRasterMeshSeal;
#[cfg(test)] use ui_scene::math::{screen_select_components, screen_select_instances};
use serde_json::json;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

fn test_scene_raster_lease(width: u32, height: u32, profile: SceneRasterProfile, mesh: Option<SceneRasterMeshSeal>, fill: u8) -> SceneRasterLease {
    static REVISION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let revision = REVISION.fetch_add(1, std::sync::atomic::Ordering::Relaxed).max(1);
    let descriptor = SceneRasterDescriptor { width, height, source_digest: [revision, revision.rotate_left(17)], source_revision: revision, profile, mesh };
    let pool = world_scene_raster_pool();
    let mut writer = None;
    for _ in 0..=semio_framework_ui_raster_owner::SCENE_RASTER_POOL_SLOTS {
        match pool.begin(descriptor, revision, SceneRasterWriteMode::Moved { capacity_bytes: descriptor.byte_len().unwrap() }) {
            SceneRasterBegin::Writer(candidate) => {
                writer = Some(candidate);
                break;
            }
            SceneRasterBegin::Reused(lease) => return lease,
            SceneRasterBegin::Backpressure(_) => {
                let _ = pool.maintenance_step();
            }
            SceneRasterBegin::Refused(fault) => panic!("test scene raster refused: {fault}"),
        }
    }
    let writer = writer.expect("test scene raster progresses within the fixed slot budget");
    let pixels = vec![fill; descriptor.byte_len().expect("test raster bytes")];
    pool.seal_moved(writer, pixels).map_err(|(_, fault)| fault).expect("test raster publishes")
}

fn test_world_scene_raster(width: u32, height: u32, fill: u8) -> WorldSceneRaster {
    let lease = test_scene_raster_lease(width, height, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, fill);
    WorldSceneRaster { identity: lease.identity(), pending: Mutex::new(Some(lease)) }
}

pub(crate) fn take_actions(input: &mut semio_framework_ui_wgpu_input::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority live") {
        actions.push(action.into_descriptor().expect("bounded action materializes"));
    }
    actions
}

fn triangle_mesh_oracle() -> TerrainMeshOracleData {
    mesh_oracle_from_buffers(vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])
}

pub(crate) fn mesh_oracle_from_buffers(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> TerrainMeshOracleData {
    TerrainMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors: Vec::new() }
}

pub(crate) fn publish_oracle_mesh(data: TerrainMeshOracleData) -> Mesh3dLease {
    publish_oracle_mesh_at_revision(data, 0)
}

/// 🧊️ An oracle mesh stamped with a chosen interaction revision — what
/// `publish_world3d_asset_mesh_lease` witnesses before it stores a decoded GLB under its url's id.
pub(crate) fn publish_oracle_mesh_at_revision(data: TerrainMeshOracleData, revision: u64) -> Mesh3dLease {
    assert!(data.positions.len().is_multiple_of(3));
    assert_eq!(data.normals.len(), data.positions.len());
    assert!(data.edge_positions.len().is_multiple_of(6));
    assert!(data.uvs.len().is_multiple_of(2));
    assert!(data.colors.len().is_multiple_of(4));
    static GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(10_000);
    let schema = Mesh3dSchema {
        vertices: (data.positions.len() / 3) as u32,
        indices: data.indices.len() as u32,
        face_ids: data.face_ids.len() as u32,
        vertex_ids: data.vertex_ids.len() as u32,
        edges: (data.edge_positions.len() / 6) as u32,
        edge_ids: data.edge_ids.len() as u32,
        uvs: (data.uvs.len() / 2) as u32,
        colors: (data.colors.len() / 4) as u32,
    surface_uvs:[0;4],tangents:0,};
    let generation = GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let grant=world_mesh_ownership_grant(0,ui_scene::math::mesh3d_begin_capacity_byte_demand(),0,1);
    let (token,progress)=mesh3d_begin(generation,revision,schema,grant).expect("oracle mesh claim");
    assert!(progress.fits(grant));
    allocate_oracle_mesh(token);
    for value in data.positions.as_chunks::<3>().0 {
        mesh3d_write_vec3(token, Mesh3dField::Positions, *value).unwrap();
    }
    for value in data.normals.as_chunks::<3>().0 {
        mesh3d_write_vec3(token, Mesh3dField::Normals, *value).unwrap();
    }
    for value in data.indices {
        mesh3d_write_u32(token, Mesh3dField::Indices, value).unwrap();
    }
    for value in data.face_ids {
        mesh3d_write_u32(token, Mesh3dField::FaceIds, value).unwrap();
    }
    for value in data.vertex_ids {
        mesh3d_write_u32(token, Mesh3dField::VertexIds, value).unwrap();
    }
    for value in data.edge_positions.as_chunks::<6>().0 {
        mesh3d_write_edge(token, [value[..3].try_into().unwrap(), value[3..].try_into().unwrap()]).unwrap();
    }
    for value in data.edge_ids {
        mesh3d_write_u32(token, Mesh3dField::EdgeIds, value).unwrap();
    }
    for value in data.uvs.as_chunks::<2>().0 {
        ui_scene::math::mesh3d_write_vec2(token, Mesh3dField::Uvs, *value).unwrap();
    }
    for value in data.colors.as_chunks::<4>().0 {
        ui_scene::math::mesh3d_write_vec4(token, Mesh3dField::Colors, *value).unwrap();
    }
    mesh3d_seal(token).expect("oracle mesh publication")
}

pub(crate) fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let grant=world_original_fixture_grant();
    let mut receipt=semio_framework_value::RetainedCloneProgress::default();
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX,grant),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
        &mut receipt,
    );
    let result=step(&mut context);
    assert!(context.retained_progress().fits(grant));
    result
}

include!("🔲️cursor-and-marquee-laws/🦀️.rs");

include!("🕹️gumball-and-authority-laws/🦀️.rs");

include!("🖼️reference-and-pick-lane-laws/🦀️.rs");

include!("🧲️selection-and-retirement-laws/🦀️.rs");
