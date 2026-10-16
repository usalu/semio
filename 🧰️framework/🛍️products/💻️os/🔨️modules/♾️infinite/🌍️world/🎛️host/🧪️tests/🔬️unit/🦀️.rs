//! 🧪️ host tests (🔬️unit): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{tool_run_trace, World3dBuildContext, WORLD_DYNAMIC_DRAW_BYTE_CAPACITY, WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_INTERACTION_INTENT_CAPACITY, WORLD_INTERACTION_MARQUEE_POINT_CAPACITY, WORLD_INTERACTION_OBJECT_CAPACITY, world_mesh_ownership_grant, world_pick_target_ray_score, world_pick_targets_from_json, world_scene_raster_pool, WorldBrushPreviewRecord, WorldCancelReason, WorldCursorWakeAuthority, WorldDrawRebuildDescriptor, WorldDrawRebuildStep, WorldDynamicFault, WorldInteractionIntent, WorldInteractionMeshToken, WorldInteractionObjectKind, WorldInteractionObjectToken, WorldInteractionPhase, WorldInteractionRegistryBuildPhase, WorldInteractionStep, WorldReferenceRecord, WorldSceneRaster, WorldVortexRecord};
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
use ui_wgpu::wgpu::{ActionDescriptor, Camera3d, CameraProjection3d, Instance3d, marquee_is_crossing_from_path, Mat4, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_seal, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, OrbitController, PointerModifiers, PreparedRenderUpload, Rect, SceneColorSource3d, SceneDraw3d, SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterPool, SceneRasterProfile, SceneRasterWriteMode, SurfaceKind, Theme, UiComponentSceneNode, UiPresence, Vec3, World3dScene, World3dSnapshotItem, World3dSnapshotPageKind};
use std::sync::Mutex;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::SceneMaterial3d;
#[cfg(test)]
use ui_wgpu::wgpu::{SceneRasterMeshSeal, screen_select_components, screen_select_instances};
use serde_json::json;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.
fn read_static(path_from_manifest: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path_from_manifest);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("test input {}: {error}", path.display())).into_boxed_str())
}

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(read_static(path));
    }
    Box::leak(joined.into_boxed_str())
}

fn test_scene_raster_lease(width: u32, height: u32, profile: SceneRasterProfile, mesh: Option<SceneRasterMeshSeal>, fill: u8) -> SceneRasterLease {
    static REVISION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let revision = REVISION.fetch_add(1, std::sync::atomic::Ordering::Relaxed).max(1);
    let descriptor = SceneRasterDescriptor { width, height, source_digest: [revision, revision.rotate_left(17)], source_revision: revision, profile, mesh };
    let pool = world_scene_raster_pool();
    let mut writer = None;
    for _ in 0..=ui_wgpu::wgpu::SCENE_RASTER_POOL_SLOTS {
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

pub(crate) fn take_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
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
    let grant=world_mesh_ownership_grant(0,ui_wgpu::wgpu::mesh3d_begin_capacity_byte_demand(),0,1);
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
        ui_wgpu::wgpu::mesh3d_write_vec2(token, Mesh3dField::Uvs, *value).unwrap();
    }
    for value in data.colors.as_chunks::<4>().0 {
        ui_wgpu::wgpu::mesh3d_write_vec4(token, Mesh3dField::Colors, *value).unwrap();
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

#[test]
fn terrain_visibility_requires_the_complete_family_marker() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let tile = (3, 4, 5);
    assert!(!terrain_family_visible(&state.terrain, tile));
    state.terrain.terrain_built_tiles.insert(tile);
    assert!(terrain_family_visible(&state.terrain, tile));
}

fn allocate_oracle_mesh(token:Mesh3dWriteToken) {
    loop {
        let grant=world_mesh_ownership_grant(0,ui_wgpu::wgpu::mesh3d_allocate_capacity_byte_demand(token).unwrap(),0,1);
        let step=mesh3d_allocate_step(token,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
}

fn close_oracle_mesh(lease:Mesh3dLease) {
    loop {
        let demand=ui_wgpu::wgpu::mesh3d_close_demands(lease,4096).unwrap();
        let grant=world_mesh_ownership_grant(4096,demand.capacity_bytes,demand.release_bytes,demand.depth);
        let step=mesh3d_close_step(lease,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
}

fn face_overlay_test_mesh_with_faces(generation: u64, revision: u64, face_ids: &[u32]) -> Mesh3dLease {
    let triangles = u32::try_from(face_ids.len()).expect("fixture triangle count");
    let vertices = triangles * 3;
    let schema = Mesh3dSchema { vertices, indices: vertices, face_ids: triangles, vertex_ids: 0, edges: 0, edge_ids: 0, uvs: 0, colors: 0 ,surface_uvs:[0;4],tangents:0,};
    let grant=world_mesh_ownership_grant(0,ui_wgpu::wgpu::mesh3d_begin_capacity_byte_demand(),0,1);
    let (token,progress)=mesh3d_begin(generation,revision,schema,grant).expect("face overlay fixture claim");
    assert!(progress.fits(grant));
    allocate_oracle_mesh(token);
    for triangle in 0..triangles {
        let x = triangle as f32 * 2.0;
        for position in [[x, 0.0, 0.0], [x + 1.0, 0.0, 0.0], [x, 1.0, 0.0]] {
            mesh3d_write_vec3(token, Mesh3dField::Positions, position).unwrap();
            mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0]).unwrap();
        }
    }
    for index in 0..vertices {
        mesh3d_write_u32(token, Mesh3dField::Indices, index).unwrap();
    }
    for face_id in face_ids {
        mesh3d_write_u32(token, Mesh3dField::FaceIds, *face_id).unwrap();
    }
    mesh3d_seal(token).expect("face overlay fixture lease")
}

fn face_overlay_test_mesh(generation: u64, revision: u64, face_id: u32) -> Mesh3dLease {
    face_overlay_test_mesh_with_faces(generation, revision, &[face_id])
}

#[test]
fn face_overlay_writer_matches_legacy_winding_and_retires_stale_generation() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 7;
    state.scene.draw_generation = 11;
    state.selection.granularity = "face".into();
    state.selection.active_object_id = Some("object".into());
    state.selection.component_ids.push("12".into());
    let source = face_overlay_test_mesh(400, 7, 12);
    publish_world3d_mesh_lease(&mut state.interaction, &mut state.scene, "source".into(), source).unwrap();
    let version = *state.scene.mesh_versions.get("source").expect("source mesh version");
    state.scene.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { component_source: None, id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();

    let mut cursor = WorldFaceOverlayMeshCursor::new("surface", 500, 7, 11).unwrap();
    let mut published = None;
    for _ in 0..128 {
        match cursor.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay) {
            WorldFaceOverlayMeshStep::Pending => {}
            WorldFaceOverlayMeshStep::Ready { index, color, key, lease } => {
                assert_eq!((index, color, key.as_str()), (0, [0.35, 0.75, 1.0, 0.62], "component-face-overlay:surface:500:0"));
                let schema = lease.schema().unwrap();
                assert_eq!((schema.vertices, schema.indices), (3, 6));
                assert_eq!(lease.vec3(Mesh3dField::Positions, 0).unwrap(), [0.0, 0.0, FACE_OVERLAY_OFFSET * 0.5]);
                assert_eq!(lease.vec3(Mesh3dField::Positions, 1).unwrap(), [1.0, 0.0, FACE_OVERLAY_OFFSET * 0.5]);
                assert_eq!(lease.vec3(Mesh3dField::Positions, 2).unwrap(), [0.0, 1.0, FACE_OVERLAY_OFFSET * 0.5]);
                assert_eq!((0..6).map(|item| lease.u32(Mesh3dField::Indices, item).unwrap()).collect::<Vec<_>>(), vec![0, 1, 2, 0, 2, 1]);
                published = Some(lease);
                cursor.published_colors[index] = Some(color);
            }
            WorldFaceOverlayMeshStep::Complete { generation, revision, draw_generation, colors } => {
                assert_eq!((generation, revision, draw_generation), (500, 7, 11));
                assert_eq!(colors, [Some([0.35, 0.75, 1.0, 0.62]), None, None]);
                break;
            }
            WorldFaceOverlayMeshStep::Stale | WorldFaceOverlayMeshStep::Fault => panic!("valid overlay fixture faulted"),
        }
    }
    assert!(cursor.terminal_is_empty());
    let published = published.expect("selected overlay published");
    mesh3d_begin_close(published).unwrap();
    close_oracle_mesh(published);

    let mut stale = WorldFaceOverlayMeshCursor::new("surface", 600, 7, 11).unwrap();
    assert!(matches!(stale.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay), WorldFaceOverlayMeshStep::Pending));
    state.session.interaction_revision = 8;
    let mut terminal = false;
    for _ in 0..32 {
        if matches!(stale.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay), WorldFaceOverlayMeshStep::Stale) {
            terminal = true;
            break;
        }
    }
    assert!(terminal && stale.terminal_is_empty());
    assert!(WorldFaceOverlayMeshCursor::new(&"x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY), 700, 1, 1).is_err());
}

#[test]
fn face_overlay_family_becomes_visible_only_after_every_bucket_is_published() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 7;
    state.scene.draw_generation = 11;
    state.selection.granularity = "face".into();
    state.selection.active_object_id = Some("object".into());
    state.selection.marquee_preview_ids.push("10".into());
    state.selection.component_ids.push("12".into());
    state.selection.hovered_component_id = Some("11".into());
    state.selection.hovered_component_object_id = Some("object".into());
    state.selection.hovered_component_mode = Some("face".into());
    let source = face_overlay_test_mesh_with_faces(750, 7, &[10, 11, 12]);
    publish_world3d_mesh_lease(&mut state.interaction, &mut state.scene, "source".into(), source).unwrap();
    let version = *state.scene.mesh_versions.get("source").expect("source mesh version");
    state.scene.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { component_source: None, id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();
    state.overlay.face_overlay_build = Some(WorldFaceOverlayMeshCursor::new("surface", 800, 7, 11).unwrap());

    let mut staged_seen = false;
    for _ in 0..1_024 {
        step_component_face_overlay_build(&mut state);
        if state.scene.meshes.contains_key("component-face-overlay:surface:800:0") && state.overlay.face_overlay_generation.is_none() {
            staged_seen = true;
        }
        if state.overlay.face_overlay_generation == Some(800) {
            break;
        }
    }
    assert!(staged_seen, "a completed bucket remains invisible while the family transaction is partial");
    assert_eq!(state.overlay.face_overlay_generation, Some(800));
    assert_eq!(state.overlay.face_overlay_colors, [Some([1.0, 0.85, 0.35, 0.36]), Some([0.35, 0.75, 1.0, 0.48]), Some([0.35, 0.75, 1.0, 0.62])]);
    for index in 0..3 {
        assert!(state.scene.meshes.contains_key(&format!("component-face-overlay:surface:800:{index}")), "every nonempty category publishes before family visibility");
    }

    state.overlay.face_overlay_build = Some(WorldFaceOverlayMeshCursor::new("surface", 900, 7, 11).unwrap());
    for _ in 0..1_024 {
        step_component_face_overlay_build(&mut state);
        if state.scene.meshes.contains_key("component-face-overlay:surface:900:0") {
            break;
        }
    }
    assert_eq!(state.overlay.face_overlay_generation, Some(800), "superseding partial family never changes the visible generation");
    state.session.interaction_revision = 8;
    for _ in 0..64 {
        step_component_face_overlay_build(&mut state);
        if state.overlay.face_overlay_build.is_none() {
            break;
        }
    }
    assert_eq!(state.overlay.face_overlay_generation, Some(800), "stale family retirement preserves the last complete publication");
    assert!(state.scene.meshes.contains_key("component-face-overlay:surface:900:0"), "stale partial publication remains owned by the generation-qualified registry until bounded retirement");
}

#[test]
fn world_object_registry_build_is_one_owner_per_turn_and_interruptible() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 7;
    let mesh = publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]));
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, "mesh".into(), mesh);
    let mesh_version = *state.scene.mesh_versions.get("mesh").expect("mesh version");
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { component_source: None, id: "instance".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.scene.vortices.push(WorldVortexRecord { full_id: "vortex".into(), position: Some([1.0, 2.0, 3.0]), radius: Some(0.5), ..Default::default() });
    state.scene.references.push(WorldReferenceRecord { url: Some("reference".into()), origin: Some([4.0, 5.0, 6.0]), width_world: Some(2.0), hidden: Some(false), ..Default::default() });
    state.scene.reference_pixels.insert("reference".into(), test_world_scene_raster(2, 1, 0));

    let mut cursor = WorldInteractionRegistryBuildCursor::new(7);
    assert_eq!(with_world_step_context(0, |context| cursor.step(&mut state.session, &mut state.interaction, &mut state.scene, context)), WorldInteractionStep::Pending);
    let mut turns = 0;
    while with_world_step_context(1, |context| cursor.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) == WorldInteractionStep::Pending {
        turns += 1;
        assert!(turns < 16);
    }
    assert!(state.interaction.interaction_objects.terminal_for_revision(7));
    let live: Vec<_> = state.interaction.interaction_objects.slots.iter().flatten().filter(|slot| slot.revision == 7).collect();
    assert_eq!(live.len(), 3);
    assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Instance && slot.id.as_str() == "instance"));
    assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Vortex && slot.id.as_str() == "vortex"));
    assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Reference && slot.id.as_str() == "reference"));

    let token = state.interaction
        .interaction_objects
        .slots
        .iter()
        .enumerate()
        .find_map(|(index, slot)| slot.as_ref().filter(|slot| slot.kind == WorldInteractionObjectKind::Reference).map(|slot| WorldInteractionObjectToken { slot: index as u16, generation: slot.generation, revision: slot.revision }))
        .expect("reference token");
    assert_eq!(state.interaction.interaction_objects.resolve(token).map(|slot| slot.values[4]), Some(1.0));

    let mut interrupted = WorldInteractionRegistryBuildCursor::new(8);
    assert!(with_world_step_context(1, |context| interrupted.close_step(context)));
    assert_eq!(interrupted.phase, WorldInteractionRegistryBuildPhase::Complete);
}

#[test]
fn world_vortex_and_reference_pick_cursors_resume_revalidate_and_close() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 3;
    state.interaction.interaction_objects.revision = 3;
    let vortex = state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Vortex, "vortex", None, Mat4::identity(), [0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.5, 0.0], None).expect("vortex token");
    state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Reference, "reference", None, Mat4::identity(), [0.0, 0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 0.0], None).expect("reference token");

    let mut vortex_cursor = WorldObjectPickCursor::from_ray(3, 8, WorldObjectPickPurpose::VortexSelect, Vec3::new(0.0, -2.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(with_world_step_context(0, |context| vortex_cursor.step(&state.session, &state.interaction, 8, context)), WorldInteractionStep::Pending);
    for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
        if with_world_step_context(1, |context| vortex_cursor.step(&state.session, &state.interaction, 8, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    assert_eq!(vortex_cursor.best.map(|(token, _)| token), Some(vortex));
    let mut plan = vortex_cursor.finish_plan(&state.session, &state.selection, &state.interaction, 8).expect("vortex plan").expect("vortex hit");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);

    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let mut reference_cursor = WorldObjectPickCursor::from_ray(state.session.interaction_revision, 9, WorldObjectPickPurpose::ReferenceHover, Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
    for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
        if with_world_step_context(1, |context| reference_cursor.step(&state.session, &state.interaction, 9, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    assert!(reference_cursor.best.is_some());
    state.session.interaction_revision = state.session.interaction_revision.wrapping_add(1);
    assert!(matches!(reference_cursor.finish_plan(&state.session, &state.selection, &state.interaction, 9), Err(WorldInteractionStep::Stale)));
    assert!(!reference_cursor.close_step());
    assert!(reference_cursor.close_step());
}

#[test]
fn world_component_cursor_steps_one_topology_item_and_rejects_aba() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 5;
    state.interaction.interaction_objects.revision = 5;
    state.selection.granularity = "face".into();
    let mesh = publish_oracle_mesh(triangle_mesh_oracle());
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, "mesh".into(), mesh);
    let mesh_token = state.interaction
        .interaction_meshes
        .slots
        .iter()
        .enumerate()
        .find_map(|(index, slot)| slot.as_ref().filter(|slot| slot.id.as_str() == "mesh").map(|slot| WorldInteractionMeshToken { slot: index as u16, generation: slot.generation }))
        .expect("mesh token");
    let object = state.interaction.interaction_objects.admit(5, WorldInteractionObjectKind::Instance, "object", Some(mesh_token), Mat4::identity(), [0.0; 8], None).expect("object token");
    let mut cursor = WorldComponentPickCursor {
        revision: 5,
        generation: 10,
        purpose: WorldComponentPickPurpose::Select,
        kind: WorldComponentKind::Face,
        local_x: 0.0,
        local_y: 0.0,
        viewport: Rect { x: 0.0, y: 0.0, w: 100.0, h: 100.0 },
        view_projection: Mat4::identity(),
        projection_spec: ui_wgpu::wgpu::default_projection_spec(),
        origin: Vec3::new(0.0, 0.0, 2.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
        slot: object.slot,
        current: None,
        topology: 0,
        merge: 0,
        best: None,
        complete: false,
    };
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)), WorldInteractionStep::Pending);
    assert_eq!(cursor.current, Some(object));
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)), WorldInteractionStep::Pending);
    assert_eq!(cursor.topology, 1);
    assert_eq!(cursor.best.map(|hit| hit.id), Some(0));
    cursor.current = None;
    cursor.slot = WORLD_INTERACTION_OBJECT_CAPACITY as u16;
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)), WorldInteractionStep::Pending);
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)), WorldInteractionStep::Complete);
    let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).expect("component plan").expect("component hit");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert_eq!(take_actions(&mut input).len(), 1);

    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let mut replacement_model = Mat4::identity();
    replacement_model.cols[3][0] = 1.0;
    let replacement = state.interaction.interaction_objects.admit(state.session.interaction_revision, WorldInteractionObjectKind::Instance, "object", Some(mesh_token), replacement_model, [0.0; 8], None).expect("replacement object token");
    assert_ne!(replacement, object);
    cursor.revision = state.session.interaction_revision;
    cursor.complete = true;
    cursor.best = Some(WorldComponentHit { object, id: 0, primary: 1.0, secondary: 0.0 });
    assert!(matches!(cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10), Err(WorldInteractionStep::Stale)));
    assert!(!cursor.close_step());
    assert!(cursor.close_step());
}

/// 🖱️📋️ **The right-gesture law.** A right press/drag/release over a world surface publishes no
/// action while the gesture runs — the menu is the shell's and the camera report is the settle's.
/// Plain right is bound to NOTHING in React's orbit map (`resolveWorldOrbitMouseButtonsIdle`:
/// `RIGHT: null`, pan on Shift, orbit on Alt), so an unmodified right-drag moves no camera either.
#[test]
fn a_right_gesture_publishes_nothing_while_it_runs() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 4;
    state.interaction.interaction_objects.revision = 4;
    state.selection.hovered_vortex_id = Some("vortex".into());
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut authority = WorldInteractionAuthority::default();
    authority.next_generation = 4;
    authority.queue.push(WorldInteractionIntent::pointer_button(0.0, 0.0, true, 2, &PointerModifiers::default())).unwrap();
    authority.queue.slots[0].as_mut().unwrap().generation = 1;
    authority.queue.push(WorldInteractionIntent::pointer_move(10.0, 0.0, 10.0, 0.0, true, 2, &PointerModifiers::default())).unwrap();
    authority.queue.slots[1].as_mut().unwrap().generation = 2;
    authority.queue.push(WorldInteractionIntent::pointer_button(10.0, 0.0, false, 2, &PointerModifiers::default())).unwrap();
    authority.queue.slots[2].as_mut().unwrap().generation = 3;
    state.authority.interaction_authority = Some(authority);
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    for generation in 1..=3 {
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context)), WorldInteractionAuthorityStep::Complete);
    }
    assert!(take_actions(&mut input).is_empty());
}

/// 🖱️ **The stale-aim law.** A queued intent outlives a relayout: dragging the dock's split gutter
/// moves a pane out from under the moves already enqueued against its previous bounds. Every pick
/// cursor refuses a point outside the CURRENT pick rect, and that refusal used to FAULT the authority
/// — a frame fault, which kills the page, so the whole wgpu shell tore down mid-drag (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` step 15). A stale aim
/// retires like any other answered intent; a point the surface DOES cover still faults, because there
/// the refusal is real.
///
/// 🖱️ The button no CAMERA gesture claims still answers, and never faults — a middle press used to
/// fall through to the unclaimed-intent fault, which killed the page on the first middle-click of
/// any world surface. What it answers is the selection React publishes for any button (packet
/// W13c's `plan_world3d_non_primary_press_pick`, through R3F's `onPointerMissed`); the release
/// retires as before.
#[test]
fn an_intent_aimed_outside_the_surface_retires_instead_of_faulting_the_frame() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.bounds = Rect { x: 536.0, y: 54.0, w: 1061.0, h: 913.0 };
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut authority = WorldInteractionAuthority::default();
    authority.next_generation = 2;
    authority.queue.push(WorldInteractionIntent::pointer_move(534.4, 500.0, -34.0, -453.0, false, 0, &PointerModifiers::default())).unwrap();
    authority.queue.slots[0].as_mut().unwrap().generation = 1;
    state.authority.interaction_authority = Some(authority);
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Complete, "🖱️ a hover the pane no longer covers is retired");
    assert_eq!(world3d_interaction_front_generation(&state.camera, &state.authority), None, "🖱️ and it leaves the queue rather than blocking it");
    assert!(!state.authority.interaction_authority.as_ref().expect("authority").faulted, "🖱️ the authority stays usable for the next gesture");
    assert!(take_actions(&mut input).is_empty());

    let mut authority = WorldInteractionAuthority::default();
    authority.next_generation = 3;
    authority.queue.push(WorldInteractionIntent::pointer_button(1000.0, 400.0, true, 1, &PointerModifiers::default())).unwrap();
    authority.queue.slots[0].as_mut().unwrap().generation = 1;
    authority.queue.push(WorldInteractionIntent::pointer_button(1000.0, 400.0, false, 1, &PointerModifiers::default())).unwrap();
    authority.queue.slots[1].as_mut().unwrap().generation = 2;
    state.authority.interaction_authority = Some(authority);
    for generation in 1..=2 {
        let mut turns = 0;
        loop {
            let step = with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context));
            assert_ne!(step, WorldInteractionAuthorityStep::Fault, "🖱️ a middle press inside the pane never faults");
            turns += 1;
            assert!(turns < 64, "🖱️ a bounded press answers in bounded turns: generation={generation}, census={}", state.interaction_census());
            if step == WorldInteractionAuthorityStep::Complete {
                break;
            }
            assert_eq!(step, WorldInteractionAuthorityStep::Pending);
        }
    }
    assert!(!state.authority.interaction_authority.as_ref().expect("authority").faulted);
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), ["interactionSelect"], "🖱️ the press publishes the selection React's own `handleEmptyClick` publishes for any button");
}

#[test]
fn world_marquee_overlay_points_reads_authority_gesture() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut gesture = WorldMarqueeGesture::new(1, 1, [10.0, 20.0]);
    assert!(gesture.push([100.0, 80.0]));
    state.authority.interaction_authority.as_mut().unwrap().marquee = Some(gesture);
    assert!(!state.selection.marquee_active);
    let points = world_marquee_overlay_points(&state.selection, &state.authority).expect("overlay points");
    assert_eq!(points, vec![[10.0, 20.0], [100.0, 80.0]]);
}

#[test]
fn world_marquee_click_retires_points_before_exact_release_retry() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let modifiers = PointerModifiers::default();
    let mut down = WorldInteractionIntent::pointer_button(0.0, 0.0, true, 0, &modifiers);
    down.generation = 1;
    let mut up = WorldInteractionIntent::pointer_button(1.0, 1.0, false, 0, &modifiers);
    up.generation = 2;
    state.authority.interaction_authority.as_mut().unwrap().queue.push(down).unwrap();
    state.authority.interaction_authority.as_mut().unwrap().queue.push(up).unwrap();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Complete);
    assert_eq!(state.authority.interaction_authority.as_ref().unwrap().marquee.as_ref().map(|gesture| gesture.len), Some(1));
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Pending);
    assert!(state.authority.interaction_authority.as_ref().unwrap().marquee.as_ref().is_some_and(|gesture| gesture.retiring));
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Stale);
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Stale);
    assert!(state.authority.interaction_authority.as_ref().unwrap().marquee.is_none());
    assert_eq!(world3d_interaction_front_generation(&state.camera, &state.authority), Some(2));
}

/// 🎯️ `targets` is a JSON-encoded STRING, not a structured array: the framework's own decoder is
/// `parse_interaction_targets`, which reads the arg with `DslValue::as_str`
/// (`💻️os/🔨️modules/🔌️plugin/🦀️.rs`), and the manifest declares it `ActionArgDef::text`. An array
/// makes `as_str` answer `None` and the guest faults the job — ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`.
#[test]
fn world_marquee_pages_build_one_target_per_grant_and_publish_atomically_fifo() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 6;
    state.interaction.interaction_objects.revision = 6;
    let mut results = WorldMarqueeResultPages::default();
    for index in 0..=WORLD_MARQUEE_RESULT_PAGE_CAPACITY {
        let id = format!("target-{index:03}");
        let token = state.interaction.interaction_objects.admit(6, WorldInteractionObjectKind::Instance, &id, None, Mat4::identity(), [0.0; 8], None).expect("marquee result token");
        assert!(results.push(WorldMarqueeResult::Object(token), id.len()));
    }
    let mut gesture = WorldMarqueeGesture::new(6, 7, [0.0, 0.0]);
    assert!(gesture.push([100.0, 100.0]));
    let mut job = WorldMarqueePublishJob::new(8, gesture, results, false, false);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    loop {
        turns += 1;
        let step = with_world_step_context(1, |context| job.step(&state.session, &state.interaction, 8, &mut input, context)).expect("bounded marquee page step");
        if step == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 400);
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 2);
    let page_targets = |action: &ActionDescriptor| {
        let raw = action.args.as_ref().and_then(|args| args.get("targets")).and_then(|value| value.as_str()).expect("targets is JSON text");
        serde_json::from_str::<Vec<serde_json::Value>>(raw).expect("targets decodes as the framework does")
    };
    assert_eq!(page_targets(&actions[0]).len(), WORLD_MARQUEE_RESULT_PAGE_CAPACITY);
    assert_eq!(page_targets(&actions[1]).len(), 1);
    assert_eq!(page_targets(&actions[0])[0]["granularity"], resolved_domain_granularity_id(&state.session), "a marquee target uses the scene's declared interaction granularity");
    assert_eq!(job.results.page_len, 0);
    assert_eq!(job.gesture.len, 0);
}

#[test]
fn world_marquee_page_claim_saturation_preserves_all_results_for_exact_retry() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 6;
    state.interaction.interaction_objects.revision = 6;
    let id = "target";
    let token = state.interaction.interaction_objects.admit(6, WorldInteractionObjectKind::Instance, id, None, Mat4::identity(), [0.0; 8], None).expect("target token");
    let mut results = WorldMarqueeResultPages::default();
    assert!(results.push(WorldMarqueeResult::Object(token), id.len()));
    let mut gesture = WorldMarqueeGesture::new(6, 7, [0.0, 0.0]);
    assert!(gesture.push([100.0, 100.0]));
    let mut job = WorldMarqueePublishJob::new(8, gesture, results, false, false);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut blockers = Vec::new();
    while let Ok(claim) = input.claim_action(1) {
        blockers.push(claim);
    }
    assert!(matches!(with_world_step_context(1, |context| job.step(&state.session, &state.interaction, 8, &mut input, context)), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)));
    assert!(job.prepared.is_none());
    assert_eq!(job.results.lens[0], 1);
    input.release_action_claim(blockers.pop().expect("retry claim")).expect("release retry claim");
    assert_eq!(with_world_step_context(1, |context| job.step(&state.session, &state.interaction, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(job.prepared.is_some());
    while !job.close_step(&mut input) {}
    for claim in blockers {
        input.release_action_claim(claim).expect("release blocker");
    }
    assert!(take_actions(&mut input).is_empty());
    assert_eq!(job.results.page_len, 0);
    assert_eq!(job.gesture.len, 0);
}

/// 🔺️ The marquee triangle carrying its OWN vertex, edge and face id buffers — what a decoded GLB
/// brings and what `screen_select_components` needs before it can answer anything but the whole
/// instance: with empty id buffers its `"vertex"`/`"edge"` arms are guarded off
/// (`🎬️scene/📐️math/🎯️picking/🦀️.rs:299`, `:316`) and the oracle falls through to the instance-id arm, which
/// is not a component census at all.
fn component_triangle_mesh_oracle() -> TerrainMeshOracleData {
    let mut data = triangle_mesh_oracle();
    data.vertex_ids = vec![11, 12, 13];
    data.face_ids = vec![31];
    data.edge_positions = vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, -1.0, -1.0, 0.0];
    data.edge_ids = vec![21, 22, 23];
    data
}

fn world_marquee_geometry_fixture(instance_count: usize) -> World3dState {
    world_marquee_geometry_fixture_from(triangle_mesh_oracle(), instance_count)
}

fn world_marquee_geometry_fixture_from(data: TerrainMeshOracleData, instance_count: usize) -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.pick_bounds = state.session.bounds;
    state.session.interaction_revision = 2;
    let mesh = publish_oracle_mesh(data);
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, "mesh".into(), mesh);
    let mesh_version = *state.scene.mesh_versions.get("mesh").expect("mesh version");
    let instances = (0..instance_count).map(|index| Instance3d { component_source: None, id: format!("object-{index:03}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }).collect();
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances, shadow_role: Default::default() });
    let mut registry = WorldInteractionRegistryBuildCursor::new(2);
    let mut turns = 0;
    while with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) == WorldInteractionStep::Pending {
        turns += 1;
        assert!(turns < instance_count + 16);
    }
    assert!(state.interaction.interaction_objects.terminal_for_revision(2));
    state
}

fn world_marquee_cursor_ids(state: &World3dState, points: &[[f32; 2]]) -> Vec<String> {
    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 8, gesture).expect("marquee cursor");
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 8, context)) == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 8_000);
    }
    let mut ids = Vec::new();
    for page in 0..usize::from(cursor.results.page_len) {
        for item in 0..usize::from(cursor.results.lens[page]) {
            let WorldMarqueeResult::Object(token) = cursor.results.pages[page][item].expect("marquee result token") else {
                panic!("object marquee produced component token");
            };
            ids.push(state.interaction.interaction_objects.resolve(token).expect("live result token").id.as_str().to_owned());
        }
    }
    ids
}

#[test]
fn world_marquee_mesh_cursor_matches_legacy_window_crossing_disjoint_and_degenerate_cases() {
    let state = world_marquee_geometry_fixture(1);
    let viewport = render_pick_viewport(&state.session);
    let view_projection = state.camera.orbit.to_camera().view_proj(800.0, 800.0);
    let mesh = state.scene.meshes.get("mesh").unwrap();
    let projected: Vec<_> = (0..3).map(|index| ui_wgpu::wgpu::project_point(view_projection, world_mesh_vertex(*mesh, index).unwrap(), viewport.w, viewport.h).unwrap()).collect();
    let min_x = projected.iter().map(|point| point[0]).fold(f32::INFINITY, f32::min);
    let max_x = projected.iter().map(|point| point[0]).fold(f32::NEG_INFINITY, f32::max);
    let min_y = projected.iter().map(|point| point[1]).fold(f32::INFINITY, f32::min);
    let max_y = projected.iter().map(|point| point[1]).fold(f32::NEG_INFINITY, f32::max);
    let cases = [[[min_x - 2.0, min_y - 2.0], [max_x + 2.0, max_y + 2.0]], [[(min_x + max_x) * 0.5, min_y - 2.0], [min_x - 2.0, (min_y + max_y) * 0.5]], [[0.0, 0.0], [2.0, 2.0]], [[3.0, 3.0], [3.0, 3.0]]];
    for points in cases {
        let crossing = marquee_is_crossing_from_path(&points, false);
        let (meshes, draws) = scene_geometry(&state.scene);
        let legacy = screen_select_instances(&meshes, &draws, view_projection, ui_wgpu::wgpu::default_projection_spec(), viewport.w, viewport.h, &points, true, crossing);
        let retained = world_marquee_cursor_ids(&state, &points);
        assert_eq!(retained, legacy, "points={points:?}");
    }
}

#[test]
fn world_marquee_mesh_cursor_preserves_legacy_multi_page_draw_order() {
    let state = world_marquee_geometry_fixture(WORLD_MARQUEE_RESULT_PAGE_CAPACITY + 1);
    let points = [[0.0, 0.0], [400.0, 400.0]];
    let view_projection = state.camera.orbit.to_camera().view_proj(800.0, 800.0);
    let (meshes, draws) = scene_geometry(&state.scene);
    let legacy = screen_select_instances(&meshes, &draws, view_projection, ui_wgpu::wgpu::default_projection_spec(), 400.0, 400.0, &points, true, false);
    let retained = world_marquee_cursor_ids(&state, &points);
    assert_eq!(retained, legacy);
    assert_eq!(retained.len(), WORLD_MARQUEE_RESULT_PAGE_CAPACITY + 1);
}

#[test]
fn world_marquee_lasso_edge_cursor_matches_legacy_and_rejects_object_aba() {
    let mut state = world_marquee_geometry_fixture(1);
    state.selection.selection_method = "lasso".into();
    let viewport = render_pick_viewport(&state.session);
    let view_projection = state.camera.orbit.to_camera().view_proj(800.0, 800.0);
    let points = [[0.0, 0.0], [400.0, 0.0], [400.0, 400.0], [0.0, 400.0]];
    let (meshes, draws) = scene_geometry(&state.scene);
    let legacy = screen_select_instances(&meshes, &draws, view_projection, ui_wgpu::wgpu::default_projection_spec(), viewport.w, viewport.h, &points, false, marquee_is_crossing_from_path(&points, true));
    assert_eq!(world_marquee_cursor_ids(&state, &points), legacy);

    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 8, gesture).unwrap();
    while cursor.current.is_none() {
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 8, context)), WorldInteractionStep::Pending);
    }
    let token = cursor.current.unwrap();
    let entry = *state.interaction.interaction_objects.resolve(token).unwrap();
    let mut replacement = entry.model;
    replacement.cols[3][0] += 1.0;
    let next = state.interaction.interaction_objects.admit(state.session.interaction_revision, WorldInteractionObjectKind::Instance, entry.id.as_str(), entry.mesh, replacement, entry.values, None).expect("replacement token");
    assert_ne!(next, token);
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 8, context)), WorldInteractionStep::Stale);
    let mut turns = 0;
    while !cursor.close_step() {
        turns += 1;
        assert!(turns < WORLD_INTERACTION_MARQUEE_POINT_CAPACITY + 4);
    }
    assert_eq!(cursor.gesture.len, 0);
}

fn world_component_marquee_cursor_ids(state: &World3dState, points: &[[f32; 2]]) -> Vec<u32> {
    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 8, gesture).expect("component marquee cursor");
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 8, context)) == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 8_000);
    }
    let mut ids = Vec::new();
    for item in 0..usize::from(cursor.results.lens[0]) {
        let WorldMarqueeResult::Component { id, .. } = cursor.results.pages[0][item].expect("component marquee result") else {
            panic!("component marquee produced object result");
        };
        ids.push(id);
    }
    ids.sort_unstable();
    ids
}

#[test]
fn world_component_marquee_cursor_matches_legacy_vertex_edge_face_geometry() {
    let mut state = world_marquee_geometry_fixture_from(component_triangle_mesh_oracle(), 1);
    let viewport = render_pick_viewport(&state.session);
    let view_projection = state.camera.orbit.to_camera().view_proj(800.0, 800.0);
    let points = [[0.0, 0.0], [400.0, 400.0]];
    for granularity in ["vertex", "edge", "face"] {
        state.selection.granularity = granularity.into();
        let (meshes, draws) = scene_geometry(&state.scene);
        let mut legacy: Vec<u32> = screen_select_components(&meshes, &draws, view_projection, ui_wgpu::wgpu::default_projection_spec(), viewport.w, viewport.h, &points, true, granularity, None, false).into_iter().map(|id| id.parse().expect("numeric component id")).collect();
        legacy.sort_unstable();
        assert!(!legacy.is_empty(), "the oracle must answer a component census, not an empty one: granularity={granularity}");
        assert_eq!(world_component_marquee_cursor_ids(&state, &points), legacy, "granularity={granularity}");
    }
}

#[test]
fn world_component_marquee_publish_preserves_shared_merge_and_exact_targets() {
    let mut state = world_marquee_geometry_fixture(1);
    state.selection.granularity = "vertex".into();
    state.selection.component_ids = vec!["7".into()];
    state.gumball.gumball_selection_ids = vec!["object-000.vertex.7".into()];
    state.selection.active_object_id = Some("object-000".into());
    let object = state.interaction.interaction_objects.instance_order[0].expect("instance token");
    let mut results = WorldMarqueeResultPages::default();
    assert!(results.push(WorldMarqueeResult::Component { object, id: 8 }, 0));
    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 7, [0.0, 0.0]);
    assert!(gesture.push([400.0, 400.0]));
    let mut job = WorldComponentMarqueePublishJob::new(8, gesture, results, WorldComponentKind::Vertex, true, false);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| job.step(&state.session, &state.interaction, &state.scene, 8, &mut input, context)).expect("component marquee step") == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 128);
    }
    let events = take_actions(&mut input);
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].action, "interactionSelect");
    let args = events[0].args.as_ref().unwrap();
    assert_eq!(args.get("merge").and_then(|value| value.as_str()), Some("additive"));
    let targets: serde_json::Value = serde_json::from_str(args.get("targets").and_then(|value| value.as_str()).unwrap()).unwrap();
    assert_eq!(targets, serde_json::json!([{"granularity":"vertex","id":"surface/object-000.vertex.8"}]));
    apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &events[0]);
    assert_eq!(state.selection.component_ids, vec!["7".to_string(), "8".to_string()]);
    assert_eq!(job.results.page_len, 0);
    assert_eq!(job.gesture.len, 0);
}

#[test]
fn world_gumball_cursor_caps_selected_tokens_and_closes_one_owner_per_grant() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 9;
    state.interaction.interaction_objects.revision = 9;
    for index in 0..=WORLD_GUMBALL_SELECTED_CAPACITY {
        let mut model = Mat4::identity();
        model.cols[3][0] = index as f32;
        assert!(state.interaction.interaction_objects.admit(9, WorldInteractionObjectKind::Instance, &format!("selected-{index}"), None, model, [0.0, 0.0, 1.0, index as f32, 0.0, 0.0, 0.0, 0.0], None).is_some());
    }
    let mut cursor = WorldGumballPickCursor::new(&state.session, &state.selection, 12, 0.0, 0.0);
    for _ in 0..WORLD_INTERACTION_OBJECT_CAPACITY + 1 {
        let step = with_world_step_context(1, |context| cursor.step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene, 12, context));
        if step == WorldInteractionStep::Fault {
            break;
        }
    }
    assert!(cursor.faulted);
    assert_eq!(cursor.selected_len as usize, WORLD_GUMBALL_SELECTED_CAPACITY);
    let mut turns = 0;
    while !cursor.close_step() {
        turns += 1;
        assert!(turns <= WORLD_GUMBALL_SELECTED_CAPACITY + 1);
    }
    assert_eq!(turns, WORLD_GUMBALL_SELECTED_CAPACITY);
    assert!(cursor.selected.iter().all(Option::is_none));
}

#[test]
fn world_gumball_update_validates_one_selected_aba_token_per_turn() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 3;
    state.interaction.interaction_objects.revision = 3;
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 100.0, h: 100.0 };
    let token = state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, Mat4::identity(), [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], None).expect("selected token");
    let mut selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
    selected[0] = Some(WorldGumballTarget::new(token, None, "selected").expect("original selected target"));
    let mut gesture = WorldGumballGesture {
        revision: 3,
        start_generation: 5,
        handle: GumballHandle::MoveX,
        pivot: Vec3::ZERO,
        anchor: 0.0,
        start: Vec3::ZERO,
        translate: Vec3::ZERO,
        angle: 0.0,
        scale: Vec3::new(1.0, 1.0, 1.0),
        mode: None,
        selected,
        selected_len: 1,
        selected_bytes: "selected".len() as u16,
        validation: 0,
        pending: None,
        live: false,
        streamed: false,
        sent_translate: Vec3::ZERO,
        sent_angle: 0.0,
        sent_scale: Vec3::new(1.0, 1.0, 1.0),
    };
    assert_eq!(gesture.begin_update(6, 50.0, 50.0), WorldInteractionStep::Pending);
    assert_eq!(gesture.update_step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene), WorldInteractionStep::Pending);
    let mut replacement = Mat4::identity();
    replacement.cols[3][0] = 1.0;
    let next = state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, replacement, [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0], None).expect("replacement token");
    assert_ne!(next, token);
    gesture.validation = 0;
    assert_eq!(gesture.update_step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene), WorldInteractionStep::Stale);
    assert!(!gesture.close_step());
    assert!(!gesture.close_step());
    assert!(gesture.close_step());
}

fn world_gumball_commit_host_snapshot() -> (World3dState, WorldGumballGesture) {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 3;
    state.interaction.interaction_objects.revision = 3;
    let token = state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, Mat4::identity(), [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], None).expect("selected token");
    let mut selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
    selected[0] = Some(WorldGumballTarget::new(token, None, "selected").expect("original selected target"));
    let gesture = WorldGumballGesture {
        revision: 3,
        start_generation: 5,
        handle: GumballHandle::MoveX,
        pivot: Vec3::ZERO,
        anchor: 0.0,
        start: Vec3::ZERO,
        translate: Vec3::new(2.0, 0.0, 0.0),
        angle: 0.0,
        scale: Vec3::new(1.0, 1.0, 1.0),
        mode: None,
        selected,
        selected_len: 1,
        selected_bytes: "selected".len() as u16,
        validation: 0,
        pending: None,
        live: false,
        streamed: false,
        sent_translate: Vec3::ZERO,
        sent_angle: 0.0,
        sent_scale: Vec3::new(1.0, 1.0, 1.0),
    };
    (state, gesture)
}

/// 🪟️ A transform verb is window-owned: it addresses its own window instance rather than
/// whatever has focus — see `WorldGumballCommitJob::step`'s `windowId` stage.
#[test]
fn world_gumball_commit_builds_one_flat_node_per_grant_then_retires_tokens() {
    let (state, gesture) = world_gumball_commit_host_snapshot();
    let mut job = WorldGumballCommitJob::new(8, gesture, WorldGumballPhase::Once);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(0, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    let mut turns = 0;
    loop {
        turns += 1;
        let step = with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).expect("bounded gumball commit");
        if step == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 32);
    }
    assert!(job.terminal_is_empty());
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "translateSelection");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("windowId")).and_then(|value| value.as_str()), Some(state.session.surface_id.as_str()));
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(|value| value.as_str()), Some(state.session.surface_id.as_str()));
    assert!(turns > 8);
}

/// 🪟️ Five staged nodes precede the first selected-id resolve: `{`, `surfaceId`, `windowId`,
/// `mode`, `ids[`.
#[test]
fn world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority() {
    let (mut state, gesture) = world_gumball_commit_host_snapshot();
    let mut job = WorldGumballCommitJob::new(8, gesture, WorldGumballPhase::Once);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut blockers = Vec::new();
    while let Ok(claim) = input.claim_action(1) {
        blockers.push(claim);
    }
    assert!(matches!(with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)));
    assert!(job.claim.is_none());
    input.release_action_claim(blockers.pop().expect("retry credit")).expect("release retry credit");
    assert_eq!(with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(job.claim.is_some());
    assert_eq!(with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(job.draft.is_some());
    let mut replacement = Mat4::identity();
    replacement.cols[3][0] = 1.0;
    state.interaction.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, replacement, [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0], None).expect("ABA replacement");
    for _ in 0..5 {
        let _ = with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context));
    }
    assert!(matches!(with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)), Err(ui_wgpu::wgpu::BoundedActionFault::Structure)));
    let mut close_turns = 0;
    while !job.close_step(&mut input) {
        close_turns += 1;
        assert!(close_turns < 8);
    }
    for claim in blockers {
        input.release_action_claim(claim).expect("release blocker");
    }
    assert!(take_actions(&mut input).is_empty());
    assert_eq!(job.gesture.selected_len, 0);
    assert_eq!(job.gesture.selected_bytes, 0);
}

#[test]
fn world_gumball_fixed_gesture_projects_preview_without_mutating_source_draw() {
    let (mut state, gesture) = world_gumball_commit_host_snapshot();
    state.authority.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    let source = Mat4::identity();
    let preview = retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source);
    assert_eq!(source.cols[3], [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(preview.cols[3], [2.0, 0.0, 0.0, 1.0]);
    let unmatched = retained_gumball_preview_model(&state.interaction, &state.authority, 0, 1, source);
    assert_eq!(unmatched.cols, source.cols);
}

/// 📨️ Drives one gumball publication job to completion and answers what it published (nothing for a `Skip`).
fn world_gumball_publish(state: &World3dState, gesture: WorldGumballGesture, phase: WorldGumballPhase) -> Option<ActionDescriptor> {
    let mut job = WorldGumballCommitJob::new(8, gesture, phase);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    for _ in 0..4096 {
        if with_world_step_context(1, |context| job.step(&state.session, &state.gumball, &state.interaction, &state.scene, 8, &mut input, context)).expect("bounded gumball publication") == WorldInteractionStep::Complete {
            break;
        }
    }
    assert!(job.terminal_is_empty(), "the publication retires its tokens");
    let mut actions = take_actions(&mut input);
    assert!(actions.len() <= 1, "one publication is at most one dispatch");
    actions.pop()
}

/// 🛠️ LAW (`🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json`, the protocol React's `worldGumballStep` answers
/// step by step): the wgpu gesture publishes every case's terminal dispatch — the local ONE net delta, the live commit
/// tail (the identity when it moved nothing), the abort with its reason, or nothing — exactly as the fixture states it,
/// never streams a local gesture, ignores a cancel before the first tick, swallows the drag after an abort, and its stream
/// ticks (one per completed pose update; the intent queue already coalesces moves, so there is no in-flight hold) sum to
/// exactly the motion React streamed.
#[test]
fn world_gumball_gesture_answers_the_shared_live_protocol() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🛠️gumball-live-protocol.json")).expect("the protocol fixture parses");
    let vector = |value: &serde_json::Value| Vec3::new(value[0].as_f64().expect("x") as f32, value[1].as_f64().expect("y") as f32, value[2].as_f64().expect("z") as f32);
    let handle = |name: &str| match name {
        "moveX" => GumballHandle::MoveX,
        "rotateZ" => GumballHandle::RotateZ,
        "scaleUniform" => GumballHandle::ScaleX,
        other => panic!("no wgpu handle for fixture handle {other}"),
    };
    let motion_keys = ["dx", "dy", "dz", "ax", "ay", "az", "angle", "sx", "sy", "sz"];
    let number = |args: &serde_json::Value, key: &str| args.get(key).and_then(serde_json::Value::as_f64);
    let mut cases = 0;
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let live = case["live"].as_bool().expect("live");
        let grabbed = !case["targets"]["ids"].as_array().expect("ids").is_empty();
        let (state, template) = world_gumball_commit_host_snapshot();
        let mut gesture: Option<WorldGumballGesture> = None;
        let mut swallowed = false;
        let (mut start_translate, mut start_scale) = (Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0));
        let mut published: Vec<ActionDescriptor> = Vec::new();
        let mut expected: Vec<serde_json::Value> = Vec::new();
        for step in case["steps"].as_array().expect("steps") {
            if !step["dispatch"].is_null() {
                expected.push(step["dispatch"].clone());
            }
            let event = &step["event"];
            let pose = |key: &str| (vector(&event[key]["position"]), vector(&event[key]["scale"]));
            match event["kind"].as_str().expect("event kind") {
                "start" => {
                    let (translate, scale) = pose("pose");
                    (start_translate, start_scale) = (translate, scale);
                    swallowed = false;
                    let mut grab = template.publication(world_gumball_identity_motion());
                    grab.handle = handle(event["handle"].as_str().expect("handle"));
                    grab.live = live;
                    grab.streamed = false;
                    if !grabbed {
                        grab.selected_len = 0;
                        grab.selected_bytes = 0;
                        grab.selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
                    }
                    gesture = Some(grab);
                }
                "drag" if !swallowed => {
                    let (translate, scale) = pose("pose");
                    let current = gesture.as_mut().expect("a drag after its grab");
                    current.translate = translate.sub(start_translate);
                    current.scale = Vec3::new(scale.x / start_scale.x, scale.y / start_scale.y, scale.z / start_scale.z);
                    if let Some((phase, motion)) = current.stream() {
                        published.extend(world_gumball_publish(&state, current.publication(motion), phase));
                    }
                }
                "release" if !swallowed => {
                    let (translate, scale) = pose("after");
                    let mut released = gesture.take().expect("a release after its grab");
                    released.translate = translate.sub(start_translate);
                    released.scale = Vec3::new(scale.x / start_scale.x, scale.y / start_scale.y, scale.z / start_scale.z);
                    let (phase, (translate, angle, scale)) = released.release();
                    published.extend(world_gumball_publish(&state, WorldGumballGesture { translate, angle, scale, ..released }, phase));
                }
                "release" => swallowed = false,
                "cancel" => {
                    let reason = if event["reason"].as_str() == Some("blur") { WorldCancelReason::Blur } else { WorldCancelReason::CaptureLost };
                    if let Some((phase, (translate, angle, scale))) = gesture.as_ref().and_then(|current| current.cancel(reason)) {
                        let cancelled = gesture.take().expect("the cancelled gesture");
                        swallowed = true;
                        published.extend(world_gumball_publish(&state, WorldGumballGesture { translate, angle, scale, ..cancelled }, phase));
                    }
                }
                _ => {}
            }
        }
        let terminal = |dispatches: &mut Vec<serde_json::Value>| dispatches.last().filter(|dispatch| dispatch["args"]["phase"].as_str() != Some("stream")).cloned().inspect(|_| {
            dispatches.pop();
        });
        let expected_terminal = terminal(&mut expected);
        let mut actual: Vec<serde_json::Value> = published.iter().map(|action| serde_json::json!({ "action": action.action, "args": serde_json::to_value(action.args.as_ref().expect("args")).expect("args as JSON") })).collect();
        let actual_terminal = terminal(&mut actual);
        match (&expected_terminal, &actual_terminal) {
            (None, None) => {}
            (Some(expected), Some(actual)) => {
                assert_eq!(actual["action"], expected["action"], "{name}: the terminal verb");
                for key in motion_keys.iter().filter(|key| expected["args"].get(**key).is_some()) {
                    assert_eq!(number(&actual["args"], key), number(&expected["args"], key), "{name}: terminal {key}");
                }
                assert_eq!(actual["args"].get("phase"), expected["args"].get("phase"), "{name}: terminal phase");
                assert_eq!(actual["args"].get("reason"), expected["args"].get("reason"), "{name}: terminal reason");
            }
            _ => panic!("{name}: terminal dispatch {actual_terminal:?} where the protocol states {expected_terminal:?}"),
        }
        assert!(live || actual.is_empty(), "{name}: a local gesture never streams");
        let summed = |dispatches: &[serde_json::Value], key: &str, product: bool| dispatches.iter().filter_map(|dispatch| number(&dispatch["args"], key)).fold(if product { 1.0 } else { 0.0 }, |total, value| if product { total * value } else { total + value });
        for (key, product) in [("dx", false), ("dy", false), ("dz", false), ("sx", true), ("sy", true), ("sz", true)] {
            assert!((summed(&actual, key, product) - summed(&expected, key, product)).abs() < 1e-6, "{name}: the stream ticks carry the streamed {key}");
        }
        assert!(actual.iter().all(|dispatch| dispatch["args"]["phase"].as_str() == Some("stream")), "{name}: only ticks stream");
        cases += 1;
    }
    assert!(cases >= 10, "every protocol case is consumed: {cases}");
}

/// 🎚️ A turn or a scale previews IN PLACE, exactly as the release commits it (every 3D gumball leaf turns and
/// scales each target about its own origin): an instance far from the gumball pivot keeps its translation while its
/// basis turns about the handle axis, and a scale stretches only its own local axis — the selection never orbits
/// the pivot, so a multi-object preview equals the committed state.
#[test]
fn world_gumball_turn_and_scale_preview_each_instance_in_place() {
    let (mut state, mut gesture) = world_gumball_commit_host_snapshot();
    let mut source = Mat4::identity();
    source.cols[3] = [5.0, 0.0, 0.0, 1.0];
    gesture.translate = Vec3::ZERO;
    gesture.handle = GumballHandle::RotateZ;
    gesture.angle = std::f32::consts::FRAC_PI_2;
    state.authority.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    let turned = retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source);
    assert_eq!(turned.cols[3], [5.0, 0.0, 0.0, 1.0], "a turned instance keeps its own origin; it never orbits the pivot at the world origin");
    let near = |column: [f32; 4], expected: [f32; 3]| (0..3).all(|axis| (column[axis] - expected[axis]).abs() < 1e-6);
    assert!(near(turned.cols[0], [0.0, 1.0, 0.0]) && near(turned.cols[1], [-1.0, 0.0, 0.0]) && near(turned.cols[2], [0.0, 0.0, 1.0]), "the basis turns a quarter about +z: {:?}", turned.cols);
    let gesture = state.authority.interaction_authority.as_mut().unwrap().gumball.as_mut().unwrap();
    gesture.handle = GumballHandle::ScaleY;
    gesture.angle = 0.0;
    gesture.scale = Vec3::new(1.0, 3.0, 1.0);
    let scaled = retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source);
    assert_eq!((scaled.cols[0], scaled.cols[1], scaled.cols[2], scaled.cols[3]), ([1.0, 0.0, 0.0, 0.0], [0.0, 3.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [5.0, 0.0, 0.0, 1.0]), "a scale stretches the instance's own y axis and keeps its origin");
}

fn world_brush_commit_host_snapshot(target: String) -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.interaction_revision = 4;
    state.gumball.brush_preview = Some(WorldBrushPreviewRecord {
        target_vortex_full_id: Some(target),
        object_kind_id: Some("kind".into()),
        source_vortex_index: Some(7),
        origin: Some([1.0, 2.0, 3.0]),
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: Some(serde_json::json!([1.0, 2.0, 3.0])),
        ..Default::default()
    });
    state
}

#[test]
fn world_brush_commit_copies_and_revalidates_fixed_chunks_before_claimed_publication() {
    let state = world_brush_commit_host_snapshot("v".repeat(WORLD_BRUSH_COPY_CHUNK_BYTES * 2 + 1));
    let mut job = WorldBrushCommitJob::new(&state.session, &state.gumball, 9).unwrap().expect("brush job");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    loop {
        turns += 1;
        let step = with_world_step_context(1, |context| job.step(&state.session, &state.gumball, 9, &mut input, context)).expect("bounded brush step");
        if step == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 64);
    }
    assert!(turns > 20);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "addBrushObject");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("targetVortexFullId")).and_then(semio_framework_value::DslValue::as_str).map(str::len), Some(WORLD_BRUSH_COPY_CHUNK_BYTES * 2 + 1));
}

#[test]
fn world_brush_validation_detects_same_length_replacement_and_close_releases_draft_claim() {
    let mut state = world_brush_commit_host_snapshot("first".into());
    let mut stale = WorldBrushCommitJob::new(&state.session, &state.gumball, 9).unwrap().expect("brush job");
    while !stale.validating {
        assert_eq!(with_world_step_context(1, |context| stale.step(&state.session, &state.gumball, 9, &mut ui_wgpu::wgpu::InputState::default(), context)).unwrap(), WorldInteractionStep::Pending);
    }
    state.gumball.brush_preview.as_mut().unwrap().target_vortex_full_id = Some("other".into());
    assert!(matches!(with_world_step_context(1, |context| stale.step(&state.session, &state.gumball, 9, &mut ui_wgpu::wgpu::InputState::default(), context)), Err(ui_wgpu::wgpu::BoundedActionFault::Structure)));

    let state = world_brush_commit_host_snapshot("target".into());
    let mut interrupted = WorldBrushCommitJob::new(&state.session, &state.gumball, 10).unwrap().expect("brush job");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    while !interrupted.complete {
        let _ = with_world_step_context(1, |context| interrupted.step(&state.session, &state.gumball, 10, &mut input, context));
    }
    assert_eq!(with_world_step_context(1, |context| interrupted.step(&state.session, &state.gumball, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(interrupted.claim.is_some());
    assert_eq!(with_world_step_context(1, |context| interrupted.step(&state.session, &state.gumball, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(interrupted.draft.is_some());
    assert!(!interrupted.close_step(&mut input));
    assert!(!interrupted.close_step(&mut input));
    assert!(interrupted.close_step(&mut input));
    assert!(take_actions(&mut input).is_empty());
}

fn world_intent(generation: u64, delta: f32) -> WorldInteractionIntent {
    WorldInteractionIntent { phase: WorldInteractionPhase::Wheel, generation, x: 0.0, y: 0.0, dx: 0.0, dy: 0.0, delta, button: 0, down: false, shift: false, ctrl: false, alt: false, meta: false }
}

/// 🧭️ A navigation step moves the orbit and publishes NOTHING: the wire report is the debounced
/// settle's, one per gesture (see `WorldCameraSync`).
#[test]
fn world_wheel_plan_revalidates_before_mutation_and_publishes_flat_action() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let original_distance = state.camera.orbit.distance;
    let mut stale = plan_world3d_wheel(&state.session, &state.camera, 7, 20.0).expect("bounded wheel plan");
    state.session.interaction_revision = state.session.interaction_revision.wrapping_add(1);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let stale_step = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut stale, 7, &mut input, context)).unwrap();
    assert_eq!(stale_step, WorldInteractionStep::Stale);
    assert_eq!(state.camera.orbit.distance, original_distance);
    assert!(take_actions(&mut input).is_empty());

    let mut plan = plan_world3d_wheel(&state.session, &state.camera, 8, 200.0).expect("bounded wheel plan");
    let pending = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 8, &mut input, context)).unwrap();
    assert_eq!(pending, WorldInteractionStep::Pending);
    assert_ne!(state.camera.orbit.distance, original_distance);
    assert!(take_actions(&mut input).is_empty());
    let complete = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 8, &mut input, context)).unwrap();
    assert_eq!(complete, WorldInteractionStep::Complete);
    assert!(plan.terminal_is_empty());

    let mut settle = plan_world3d_camera_settle(&state.session, &state.camera, 9).expect("a moved camera owes a settle");
    let mut turns = 0;
    while with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut settle, 9, &mut input, context)).unwrap() != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 8, "a bounded settle terminates");
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), vec!["noteWorldNavigation", "setCamera"]);
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("gestures")).and_then(semio_framework_value::DslValue::as_array).map(|ids| ids.iter().filter_map(semio_framework_value::DslValue::as_str).collect::<Vec<_>>()), Some(vec!["zoom"]));
}

#[test]
fn world_drag_and_paint_plans_reserve_before_exact_mutation() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let original_target = state.camera.orbit.target;
    let modifiers = PointerModifiers { shift: false, ctrl: false, alt: false, meta: false };
    let mut drag = plan_world3d_drag(&state.session, &state.camera, 3, 8.0, -4.0, 1, &modifiers).expect("middle drag plan");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let step = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut drag, 3, &mut input, context)).unwrap();
    assert_eq!(step, WorldInteractionStep::Pending);
    assert_ne!(state.camera.orbit.target, original_target);
    assert!(take_actions(&mut input).is_empty(), "a pan step moves the orbit and owes the settle, it never publishes per move");

    state.selection.interaction_mode = "paint".into();
    assert!(plan_world3d_paint_release(&state.session, &state.selection, 4, 0).is_none(), "a release without an open paint gesture commits nothing");
    state.selection.paint_stroke_active = true;
    let mut release = plan_world3d_paint_release(&state.session, &state.selection, 4, 0).expect("paint release plan");
    let zero = with_world_step_context(0, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut release, 4, &mut input, context)).unwrap();
    assert_eq!(zero, WorldInteractionStep::Pending);
    assert!(state.selection.paint_stroke_active, "the gesture stays open until the commit is reserved");
    let published = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut release, 4, &mut input, context)).unwrap();
    assert_eq!(published, WorldInteractionStep::Pending);
    assert!(!state.selection.paint_stroke_active);
    let actions = take_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), vec!["paintAt"]);
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("phase")).and_then(semio_framework_value::DslValue::as_str), Some("commit"), "the release commits the app's open paint transaction");

    state.selection.paint_stroke_active = true;
    let mut end = plan_world3d_paint_release(&state.session, &state.selection, 5, 0).expect("paint release plan");
    state.session.interaction_revision = state.session.interaction_revision.wrapping_add(1);
    let stale = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut end, 5, &mut input, context)).unwrap();
    assert_eq!(stale, WorldInteractionStep::Stale);
    assert!(state.selection.paint_stroke_active);
    assert!(take_actions(&mut input).is_empty());
}

/// 🧯️ LAW: a host cancel aborts an open paint stroke with ONE `paintAt{phase:"abort", reason}` and clears the stroke only
/// once the abort is reserved; with no stroke open it owes nothing.
#[test]
fn world_paint_cancel_aborts_the_open_stroke_with_its_reason() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.selection.interaction_mode = "paint".into();
    assert!(plan_world3d_paint_abort(&state.session, &state.selection, 4, WorldCancelReason::Blur).is_none(), "no open stroke, nothing to abort");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    for (generation, reason) in [(4, WorldCancelReason::Blur), (5, WorldCancelReason::CaptureLost)] {
        state.selection.paint_stroke_active = true;
        let mut abort = plan_world3d_paint_abort(&state.session, &state.selection, generation, reason).expect("paint abort plan");
        let published = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut abort, generation, &mut input, context)).unwrap();
        assert_eq!(published, WorldInteractionStep::Pending);
        assert!(!state.selection.paint_stroke_active, "the abort closes the stroke");
        let actions = take_actions(&mut input);
        assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), vec!["paintAt"]);
        let arg = |key: &str| actions[0].args.as_ref().and_then(|args| args.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
        assert_eq!((arg("phase"), arg("reason")), (Some("abort".to_string()), Some(reason.code().to_string())));
    }
}

/// ⚡️ LAW: a live gesture never previews locally — the guest previews the open transaction it streamed — while a local
/// gesture keeps the host's in-place preview.
#[test]
fn world_live_gumball_skips_the_local_preview() {
    let (mut state, mut gesture) = world_gumball_commit_host_snapshot();
    gesture.live = true;
    state.authority.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    let source = Mat4::identity();
    assert_eq!(retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source).cols, source.cols, "the guest draws the live answer");
    state.authority.interaction_authority.as_mut().unwrap().gumball.as_mut().expect("gesture").live = false;
    assert_eq!(retained_gumball_preview_model(&state.interaction, &state.authority, 0, 0, source).cols[3], [2.0, 0.0, 0.0, 1.0], "a local drag previews itself");
}

/// 🧯️ LAW: a cancel is owed exactly while a live gumball gesture has streamed or a paint stroke is open.
#[test]
fn world_cancel_is_owed_only_by_a_streamed_live_gesture_or_an_open_stroke() {
    let (mut state, mut gesture) = world_gumball_commit_host_snapshot();
    assert!(!world3d_cancel_owed(&state.selection, &state.authority), "nothing open, nothing owed");
    gesture.live = true;
    state.authority.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    assert!(!world3d_cancel_owed(&state.selection, &state.authority), "a live gesture that streamed nothing owes nothing");
    state.authority.interaction_authority.as_mut().unwrap().gumball.as_mut().expect("gesture").streamed = true;
    assert!(world3d_cancel_owed(&state.selection, &state.authority));
    state.authority.interaction_authority.as_mut().unwrap().gumball = None;
    state.selection.interaction_mode = "paint".into();
    state.selection.paint_stroke_active = true;
    assert!(world3d_cancel_owed(&state.selection, &state.authority), "an open stroke owes its abort");
}

fn world_pick_fixture() -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut data = triangle_mesh_oracle();
    data.uvs = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, "mesh".into(), publish_oracle_mesh(data));
    let mesh_version = *state.scene.mesh_versions.get("mesh").expect("mesh version");
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { component_source: None, id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state
}

#[test]
fn world_ray_pick_cursor_advances_one_triangle_or_boundary_per_grant() {
    let state = world_pick_fixture();
    let mut cursor = WorldRayPickCursor {
        revision: state.session.interaction_revision,
        generation: 9,
        purpose: WorldRayPickPurpose::Paint,
        origin: Vec3::new(0.0, 0.0, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
        local_x: 0.0,
        local_y: 0.0,
        viewport: render_pick_viewport(&state.session),
        view_projection: ui_wgpu::wgpu::projection_spec_view_proj(&state.camera.orbit.to_camera(), state.camera.projection_spec, render_pick_viewport(&state.session).w, render_pick_viewport(&state.session).h),
        projection_spec: state.camera.projection_spec,
        draw: 0,
        instance: 0,
        primitive: 0,
        mesh: None,
        mesh_probe: 0,
        merge: 0,
        best: None,
        pick_target: 0,
        best_target: None,
        complete: false,
        faulted: false,
    };
    let zero = with_world_step_context(0, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context));
    assert_eq!(zero, WorldInteractionStep::Pending);
    assert_eq!(cursor.primitive, 0);
    let first = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context));
    assert_eq!(first, WorldInteractionStep::Pending);
    assert_eq!(cursor.primitive, 0);
    assert!(cursor.mesh.is_some());
    let triangle = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context));
    assert_eq!(triangle, WorldInteractionStep::Pending);
    assert_eq!(cursor.primitive, 1);
    assert!(cursor.best.is_some());
    for _ in 0..3 {
        let _ = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context));
    }
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 9, context)), WorldInteractionStep::Complete);
    let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 9).expect("live cursor").expect("paint hit");
    let mut state = state;
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 9, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["paintAt"]);
}

#[test]
fn world_ray_pick_cursor_stale_and_interrupted_close_do_not_publish() {
    let mut state = world_pick_fixture();
    let mut cursor = WorldRayPickCursor {
        revision: state.session.interaction_revision,
        generation: 12,
        purpose: WorldRayPickPurpose::Surface,
        origin: Vec3::new(0.0, 0.0, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
        local_x: 0.0,
        local_y: 0.0,
        viewport: render_pick_viewport(&state.session),
        view_projection: ui_wgpu::wgpu::projection_spec_view_proj(&state.camera.orbit.to_camera(), state.camera.projection_spec, render_pick_viewport(&state.session).w, render_pick_viewport(&state.session).h),
        projection_spec: state.camera.projection_spec,
        draw: 0,
        instance: 0,
        primitive: 0,
        mesh: None,
        mesh_probe: 0,
        merge: 0,
        best: None,
        pick_target: 0,
        best_target: None,
        complete: false,
        faulted: false,
    };
    let _ = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context));
    let _ = with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context));
    state.session.interaction_revision = state.session.interaction_revision.wrapping_add(1);
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context)), WorldInteractionStep::Stale);
    assert_eq!(cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 12).err().expect("stale cursor"), WorldInteractionStep::Stale);
    assert!(!cursor.close_step());
    assert!(cursor.close_step());
    assert!(cursor.terminal_is_empty());
}

/// 🧭️ **The settle-saturation law.** A navigation intent publishes nothing at all — it moves the
/// orbit and leaves the debt — so the plan that can saturate the output is the SETTLE's, and a
/// saturated settle is RETAINED and retried in its own order (`noteWorldNavigation`, then
/// `setCamera`), never dropped. See `WorldCameraSync`.
#[test]
fn world_authority_retains_front_plan_across_output_saturation_and_retries_in_order() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let original_distance = state.camera.orbit.distance;
    enqueue_world3d_intent(&mut state.authority, world_intent(1, 200.0)).expect("wheel intent");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)) != WorldInteractionAuthorityStep::Complete {
        turns += 1;
        assert!(turns < 8, "a wheel intent settles in a bounded number of steps");
    }
    assert_ne!(state.camera.orbit.distance, original_distance);
    assert!(take_actions(&mut input).is_empty(), "a navigation step publishes nothing");

    let settle = world3d_interaction_front_generation(&state.camera, &state.authority).expect("a drained queue still owes the settle");
    let mut claims = Vec::new();
    while let Ok(claim) = input.claim_action(1) {
        claims.push(claim);
    }
    assert!(!claims.is_empty());
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_interaction(&mut state, settle, &mut input, context)) != WorldInteractionAuthorityStep::OutputBlocked {
        turns += 1;
        assert!(turns < 16, "a settle with no output credit blocks rather than dropping its plan");
    }
    assert!(take_actions(&mut input).is_empty());
    for claim in claims {
        input.release_action_claim(claim).expect("release retained credit");
    }
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_interaction(&mut state, settle, &mut input, context)) != WorldInteractionAuthorityStep::Complete {
        turns += 1;
        assert!(turns < 8, "a retried settle completes in a bounded number of steps");
    }
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["noteWorldNavigation", "setCamera"]);
}

#[test]
fn world_authority_close_drains_active_and_queued_fixed_owners_to_terminal() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    enqueue_world3d_intent(&mut state.authority, world_intent(1, 10.0)).expect("active intent");
    enqueue_world3d_intent(&mut state.authority, world_intent(2, 20.0)).expect("queued intent");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let _ = with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context));
    begin_world3d_interaction_close(&mut state.authority);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(!with_world_step_context(0, |context| close_world3d_interaction_step(&mut state.authority, &mut input, context)));
    assert!(!world3d_interaction_terminal_is_empty(&state.authority));
    let mut turns = 0;
    while !with_world_step_context(1, |context| close_world3d_interaction_step(&mut state.authority, &mut input, context)) {
        turns += 1;
        assert!(turns < 8);
    }
    assert!(world3d_interaction_terminal_is_empty(&state.authority));
    assert!(take_actions(&mut input).is_empty());
}

/// 🗃️ The blocked owner transfers on the FIRST turn that finds a freed slot, whatever else that
/// turn's authority owes first (a moved camera re-dirties the object registry, and its rebuild is
/// itself several turns).
#[test]
fn world_saturation_owner_blocks_new_ingress_until_exact_fifo_transfer() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    for generation in 1..=WORLD_INTERACTION_INTENT_CAPACITY as u64 + 1 {
        enqueue_world3d_intent(&mut state.authority, world_intent(generation, generation as f32)).expect("queue or retained saturation owner");
    }
    let rejected_generation = WORLD_INTERACTION_INTENT_CAPACITY as u64 + 2;
    assert_eq!(enqueue_world3d_intent(&mut state.authority, world_intent(rejected_generation, rejected_generation as f32)).expect_err("blocked owner seals ingress").generation, rejected_generation);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while state.authority.interaction_authority.as_ref().and_then(|authority| authority.queue.front()).map(|intent| intent.generation) == Some(1) {
        let _ = with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context));
        turns += 1;
        assert!(turns < 16, "the front intent is answered in a bounded number of turns");
    }
    assert!(take_actions(&mut input).is_empty(), "a navigation intent moves the orbit and owes the settle, it publishes nothing");
    let mut turns = 0;
    while state.authority.interaction_authority.as_ref().is_some_and(|authority| authority.blocked.is_some()) {
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Pending);
        turns += 1;
        assert!(turns < 16, "a freed slot admits the retained saturation owner");
    }
    let authority = state.authority.interaction_authority.as_mut().expect("authority");
    for generation in 2..=WORLD_INTERACTION_INTENT_CAPACITY as u64 + 1 {
        assert_eq!(authority.queue.front().map(|intent| intent.generation), Some(generation));
        assert!(authority.queue.retire_front(generation));
    }
    assert!(authority.queue.front().is_none());
}

/// 🖼️ The reference underlay is OFFERED, never given away, and a refused admission is back-pressure
/// rather than a frame fault.
///
/// Two regressions meet here. Reporting the refusal as a fault killed the page on a dock-window close
/// (`frame world resource admission exceeded fixed credits`), and taking the payload to offer it only
/// once left both panes with no plan at all when that one frame's present was aborted (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w9c-behaviour-parity-run-2.md` §1 and W9b's regression).
#[test]
fn a_reference_underlay_is_offered_every_render_and_a_refused_admission_is_not_a_fault() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.scene.reference_pixels.insert("/plan.jpg".into(), test_world_scene_raster(4, 2, 7)).expect("reference pixels admitted");
    for render in 0..4 {
        let offered = reference_underlay_upload(&state.scene, "/plan.jpg").expect("every render offers the payload");
        assert_eq!(offered.identity().descriptor().byte_len(), Some(32), "🖼️ render {render}");
    }
    assert_eq!(state.scene.reference_pixels.get("/plan.jpg").and_then(|raster| raster.identity.descriptor().byte_len()), Some(32), "🖼️ and the surface keeps the immutable identity while the pool owns pixels");
    assert_eq!(reference_image_aspect(&state.scene, "/plan.jpg"), 2.0, "🖼️ the dimensions answer the aspect every frame reads");
    assert!(reference_underlay_upload(&state.scene, "/missing.jpg").is_none());

    let pool = world_scene_raster_pool();
    let over_budget = (ui_wgpu::wgpu::SCENE_RASTER_ITEM_BYTES / 4 + 1) as u32;
    let descriptor = SceneRasterDescriptor { width: over_budget, height: 1, source_digest: [91, 92], source_revision: 91, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    assert!(matches!(pool.begin(descriptor, 91, SceneRasterWriteMode::Moved { capacity_bytes: descriptor.byte_len().unwrap() }), SceneRasterBegin::Refused(_)), "🧯️ the full-quality pool refuses item +1 before it owns an allocation");
}

#[test]
fn six_reference_gpu_residents_progress_through_two_cpu_slots_and_a_lost_texture_resumes_its_source() {
    fn publish(pool: &SceneRasterPool, descriptor: SceneRasterDescriptor, owner: u64, fill: u8) -> SceneRasterLease {
        for _ in 0..=2 {
            match pool.begin(descriptor, owner, SceneRasterWriteMode::Streamed) {
                SceneRasterBegin::Writer(writer) => {
                    let writer = pool.push(writer, &[fill; 8]).expect("one exact row");
                    return pool.seal(writer).expect("reference seal");
                }
                SceneRasterBegin::Backpressure("scene raster retirement is pending") => assert!(!pool.maintenance_step()),
                _ => panic!("reference CPU slot must make bounded progress"),
            }
        }
        panic!("reference CPU slot did not progress after one bounded retirement");
    }

    let limits = ui_wgpu::wgpu::SceneRasterPoolLimits { item_bytes: 16, pool_bytes: 32, slot_capacity: 2, lease_capacity_per_slot: 4, transfer_bytes: 8, retire_bytes: 8, gpu_resident_bytes: 48 };
    let pool = SceneRasterPool::try_new(limits).expect("reference pool");
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut gpu = Vec::new();
    let mut identities = Vec::new();
    for revision in 1u64..=6 {
        let url = format!("/reference-{revision}.png");
        state.scene.references.push(WorldReferenceRecord { url: Some(url.clone()), origin: Some([0.0; 3]), width_world: Some(2.0), hidden: Some(false), ..Default::default() });
        let descriptor = SceneRasterDescriptor { width: 2, height: 1, source_digest: [revision, revision.rotate_left(17)], source_revision: revision, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
        let lease = publish(&pool, descriptor, revision, revision as u8);
        let identity = lease.identity();
        state.scene.reference_pixels.insert(url.clone(), WorldSceneRaster { identity, pending: Mutex::new(Some(lease)) }).expect("reference identity");
        let upload = world_raster_upload_in_pool(&state.scene, &url, &url, false, &pool).expect("pending CPU upload");
        let mut producer = World3dBuildContext::new(WorldCursorWakeAuthority::new());
        producer.ensure_world_plane_texture(&url, upload);
        let mut input = ui_wgpu::wgpu::PreparedRenderInput::try_new(revision, revision, ui_wgpu::wgpu::DrawList::default(), None, 0.0).unwrap_or_else(|mut rejected| {
            let fault=rejected.fault();while !rejected.close_step() {}
            panic!("prepared reference input admission: {fault}");
        });
        assert!(matches!(producer.append_step(&mut input), Ok(false)), "lease enters actual prepared upload");
        assert!(matches!(producer.append_step(&mut input), Ok(false)), "producer retires its exact key");
        assert!(matches!(producer.append_step(&mut input), Ok(true)), "producer reaches terminal");
        let PreparedRenderUpload::SceneRaster { key, lease } = input.uploads.get(0).expect("actual producer publishes the scene lease") else { panic!("scene raster upload") };
        assert_eq!(key, &url);
        gpu.push(lease.release_witness().expect("CPU witness").commit_gpu(&url).expect("GPU commit"));
        assert!(world_reference_raster_ready_in_pool(&state.scene, &url, &pool));
        identities.push(identity);
        drop(input);
        let mut retired = false;
        for _ in 0..64 {
            if matches!(ui_wgpu::wgpu::PreparedRenderInput::close_abandoned_step(world_original_fixture_grant()),semio_framework_job::InteractiveJobCloseStep::Complete{..}) {
                retired = true;
                break;
            }
        }
        assert!(retired, "each submitted frame retires before the next producer admission");
    }
    assert_eq!(pool.reserved_bytes(), 16, "six resident textures retain only two CPU backing slots");
    assert_eq!(pool.gpu_resident_bytes(), 48);
    assert!(!pool.cpu_resident(identities[0]), "the oldest source pixels were evicted after later GPU commits");

    drop(gpu.remove(0));
    assert!(!world_reference_raster_ready_in_pool(&state.scene, "/reference-1.png", &pool), "GPU loss plus CPU eviction makes the exact source requestable again");
    let request = reserve_world3d_asset_request(&mut state.session, &mut state.assets, WorldAssetRequestKind::ReferenceImage, "/reference-1.png").expect("lost source is requestable");
    let owner = take_next_world3d_asset(&mut state.assets).expect("lost source reaches actual fetch ownership");
    assert_eq!(owner.token(), request);
    assert_eq!(owner.url(), "/reference-1.png");
    assert_eq!(owner.kind(), WorldAssetRequestKind::ReferenceImage);
    return_world3d_asset(&mut state.assets, owner).unwrap_or_else(|_| panic!("source owner returns for bounded close"));

    let descriptor = identities[0].descriptor();
    let replacement = publish(&pool, descriptor, 100, 1);
    assert!(apply_decoded_reference_raster(&mut state.session, &mut state.authority, &mut state.scene, &mut state.assets, "/reference-1.png", replacement));
    assert!(world_reference_raster_ready_in_pool(&state.scene, "/reference-1.png", &pool));
    assert_eq!(reference_image_aspect(&state.scene, "/reference-1.png"), 2.0, "recovery preserves the decoded natural dimensions");
    state.assets.asset_io.begin_close();
    while !state.assets.asset_io.close_step() {}
    assert!(state.assets.asset_io.terminal_is_empty());
}

fn topology_mesh() -> Mesh3dLease {
    let mut data = mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2, 1, 3, 2]);
    data.face_ids = vec![10, 11];
    data.vertex_ids = vec![1, 2, 3, 4];
    data.edge_positions = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
    data.edge_ids = vec![5, 6];
    publish_oracle_mesh(data)
}

fn scene_with_selection(selection_json: &str) -> UiComponentSceneNode {
    scene_with_selection_and_domain(selection_json, None)
}

/// 🪟️ Like `scene_with_selection`, but also stamps `World3dScene.domain_id`/`domain_granularity_id`
/// when `domain` is `Some((domain_id, granularity_id))` — exercises the app-bound-domain path of
/// `resolved_domain_id`/`resolved_domain_granularity_id`/`resolved_item_id`.
fn scene_with_selection_and_domain(selection_json: &str, domain: Option<(&str, &str)>) -> UiComponentSceneNode {
    UiComponentSceneNode {
        presence: UiPresence::default(),
        host_id: "surface-1".into(),
        surface_id: "surface-1".into(),
        controller_id: "controller-1".into(),
        component_kind: SurfaceKind::World3d,
        pane_id: None,
        binding_id: None,
        canvas_2d: None,
        world_3d: Some(World3dScene {
            snapshot: None,
            camera_json: r#"{"position":[4.0,4.0,4.0],"target":[0.0,0.0,0.0],"up":[0.0,0.0,1.0],"fov":45.0}"#.into(),
            meshes_json: r#"[{"id":"mesh-1","data":{"positions":[0,0,0,1,0,0,0,1,0],"normals":[0,0,1,0,0,1,0,0,1],"indices":[0,1,2],"faceIds":[10],"vertexIds":[1,2,3],"edgePositions":[0,0,0,1,0,0],"edgeIds":[5]}}]"#.into(),
            instances_json: r#"[{"id":"obj-1","meshId":"mesh-1","position":[0,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into(),
            instances_delta_json: None,
            selection_json: selection_json.into(),
            vortices_json: None,
            attractions_json: None,
            target_volumes_json: None,
            references_json: None,
            brush_preview_json: None,
            interaction_json: None,
            engagement_preview_json: None,
            pick_targets_json: None,
            lod_json: None,
            presentation_json: None,
            chunking_json: None,
            environment_json: None,
            frame_json: None,
            fit_json: None,
            terrain_json: None,
            points_json: None,
            status_json: None,
            tool_run_trace: None,
            annotations: None,
            scalar_field: None,
            modelling_options: None,
            domain_id: domain.map(|(id, _)| id.to_string()),
            domain_granularity_id: domain.map(|(_, granularity)| granularity.to_string()),
            lanes: Vec::new(),
        }),
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    }
}

#[test]
fn sync_parses_scene_reference_lanes() {
    let references = r#"[{"id":"ref-masterarbeit","url":"/infinite-assets/plan.jpg","origin":[7.0,0.0,0.01],"widthWorld":50.0,"locked":true,"hidden":false}]"#;
    let mut scene = scene_with_selection("{}");
    if let Some(world) = scene.world_3d.as_mut() {
        world.references_json = Some(references.into());
    }
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.scene.references.len(), 1);
    assert_eq!(state.scene.references[0].url.as_deref(), Some("/infinite-assets/plan.jpg"));
    assert_eq!(state.scene.references[0].width_world, Some(50.0));
    assert!(!state.scene.references[0].hidden.unwrap_or(true));
}

/// 🤝️ One world surface fed the `engagementPreview` lane a construction statechart publishes —
/// the four kinds React's `EngagementPreviewLayer` draws, in the spelling `cad`'s
/// `interaction::preview_display_items` emits.
const ENGAGEMENT_PREVIEW_JSON: &str = r#"[
    {"kind":"point","role":"corner","position":[1.0,2.0,0.0]},
    {"kind":"segment","role":"edge","from":[0.0,0.0,0.0],"to":[1.0,0.0,0.0]},
    {"kind":"box-preview","cornerA":[0.0,0.0,0.0],"cornerB":[2.0,3.0,0.0],"height":4.0},
    {"kind":"linear-handle","axis":[0.0,0.0,2.0],"origin":[1.0,1.0,0.0]}
]"#;

fn scene_with_engagement_preview(preview_json: Option<&str>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection("{}");
    if let Some(world) = scene.world_3d.as_mut() {
        world.engagement_preview_json = preview_json.map(str::to_string);
    }
    scene
}

#[test]
fn sync_parses_the_engagement_preview_lane() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some(ENGAGEMENT_PREVIEW_JSON)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.scene.engagement_preview.len(), 4);
    assert_eq!(state.scene.engagement_preview[0].kind.as_deref(), Some("point"));
    assert_eq!(state.scene.engagement_preview[0].position, Some([1.0, 2.0, 0.0]));
    assert_eq!(state.scene.engagement_preview[1].from, Some([0.0, 0.0, 0.0]));
    assert_eq!(state.scene.engagement_preview[2].corner_b, Some([2.0, 3.0, 0.0]));
    assert_eq!(state.scene.engagement_preview[2].height, Some(4.0));
    assert_eq!(state.scene.engagement_preview[3].axis, Some([0.0, 0.0, 2.0]));
}

#[test]
fn an_absent_or_broken_engagement_preview_lane_leaves_the_state_empty() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(None), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.scene.engagement_preview.is_empty());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some("not json")), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.scene.engagement_preview.is_empty());
}

/// 🖍️ 3 axis crosses (2 vertices each) + 1 segment + 12 box edges + 1 handle = 6 + 2 + 24 + 2 = 34
/// line vertices, and the box wireframe extrudes UP from `cornerA.z` by `height`.
#[test]
fn the_engagement_preview_lane_paints_world_lines() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some(ENGAGEMENT_PREVIEW_JSON)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let mut lines = Vec::new();
    append_engagement_preview_lines(&state.scene, &mut lines, [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(lines.len(), 34);
    let zs: Vec<f32> = lines.iter().map(|vertex| vertex.position[2]).collect();
    assert!(zs.iter().any(|z| (*z - 4.0).abs() < 1e-6), "the box top sits at cornerA.z + height");
    assert!(lines.iter().all(|vertex| vertex.color == [1.0, 0.0, 0.0, 1.0]));
}

/// 🩹️ React floors width/depth/height at `0.05`, so a degenerate footprint still shows a sliver.
#[test]
fn a_degenerate_box_preview_keeps_the_react_minimum_extent() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some(r#"[{"kind":"box-preview","cornerA":[0.0,0.0,0.0],"cornerB":[0.0,0.0,0.0]}]"#)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let mut lines = Vec::new();
    append_engagement_preview_lines(&state.scene, &mut lines, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(lines.len(), 24);
    let max_x = lines.iter().map(|vertex| vertex.position[0]).fold(f32::MIN, f32::max);
    let min_x = lines.iter().map(|vertex| vertex.position[0]).fold(f32::MAX, f32::min);
    assert!((max_x - min_x - 0.05).abs() < 1e-6, "width floors at 0.05, got {}", max_x - min_x);
    let max_z = lines.iter().map(|vertex| vertex.position[2]).fold(f32::MIN, f32::max);
    assert!((max_z - 0.05).abs() < 1e-6, "height floors at 0.05, got {max_z}");
}

#[test]
fn an_engagement_preview_item_missing_its_geometry_is_skipped_not_faulted() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(
        &mut state,
        &scene_with_engagement_preview(Some(r#"[{"kind":"point"},{"kind":"segment","from":[0,0,0]},{"kind":"box-preview","cornerA":[0,0,0]},{"kind":"linear-handle","origin":[0,0,0]},{"kind":"unknown-kind","position":[1,1,1]}]"#)),
        Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 },
    );
    assert_eq!(state.scene.engagement_preview.len(), 5);
    let mut lines = Vec::new();
    append_engagement_preview_lines(&state.scene, &mut lines, [0.0, 0.0, 0.0, 1.0]);
    assert!(lines.is_empty());
}

//#region 🧲️PickTargetLane
/// 🧲️ One pane's `pickTargets` lane as `cad`'s `pick_target_lane_items` publishes it: a vertex on
/// the orbit target, an edge through it, and a locked face that RENDERS but is never hit.
const PICK_TARGETS_JSON: &str = r#"[
    {"kind":"object","id":"solid-1","point":[0.0,0.0,0.0],"points":[[-2.0,-2.0,-2.0],[2.0,2.0,2.0]],"typology":"wall","selectable":true,
     "style":{"color":"--muted-foreground","emissive":"--hover-panel","opacity":0.28,"lineWidth":7.0}},
    {"kind":"vertex","id":"v-1","point":[0.0,0.0,0.0],"points":[],"selectable":true,
     "style":{"color":"--foreground","emissive":"--hover-base","opacity":1.0,"lineWidth":5.0}},
    {"kind":"face","id":"f-locked","point":[0.0,0.0,0.0],"points":[[-1.0,-1.0,0.0],[1.0,1.0,0.0]],"selectable":false,
     "style":{"color":"--accent-secondary","emissive":"--hover-window","opacity":0.042,"lineWidth":4.0}}
]"#;

fn scene_with_pick_targets(pick_targets_json: Option<&str>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection_and_domain("{}", Some(("cad", "object")));
    if let Some(world) = scene.world_3d.as_mut() {
        world.pick_targets_json = pick_targets_json.map(str::to_string);
    }
    scene
}

/// 🧲️ A pane aiming straight down the world -Z axis at the origin, with the lane loaded — the
/// smallest scene in which a ray hits every pick target at once, so what the laws below observe is
/// the ORDERING, not the geometry.
fn pick_target_state(pick_targets_json: &str) -> World3dState {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.pick_bounds = state.session.bounds;
    state.session.active_utility = "select".into();
    state.session.bound_domain_id = Some("cad".into());
    state.session.bound_domain_granularity_id = Some("object".into());
    state.selection.pick_targets = world_pick_targets_from_json(pick_targets_json);
    state
}

fn pick_target_ray(state: &World3dState) -> (Vec3, Vec3) {
    let camera = state.camera.orbit.to_camera();
    camera.ray_from_screen(200.0, 200.0, 400.0, 400.0)
}

/// 🧲️ LAW (packet W14g item 1): the `pickTargets` lane is READ, bounded, hit-tested with React's own
/// `targetRayScore` ordering, and dispatched as `interactionSelect`/`interactionHover` at the
/// target's own granularity.
///
/// ⚖️ Four things at once, because they are one mechanism: a lane that parses but never hit-tests is
/// exactly the gap this packet closed (`📓️w2f-cad-spatial-editor-wgpu.md` §5.1 — the overlay could
/// be SHOWN and not picked).
///
/// 🖱️ A real click through the retained authority — the pick target beats the instance under it.
///
/// 🎯️ `targets` is JSON TEXT on the wire (see `push_interaction_targets`), never a nested array.
#[test]
fn the_pick_target_lane_hit_tests_finest_first_skips_unselectable_rows_and_dispatches_react_payloads() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_pick_targets(Some(PICK_TARGETS_JSON)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.selection.pick_targets.len(), 3, "the lane is parsed off the scene, not only off a hand-built state");

    let mut state = pick_target_state(PICK_TARGETS_JSON);
    let (origin, direction) = pick_target_ray(&state);
    let scores: Vec<Option<f32>> = state.selection.pick_targets.iter().map(|target| world_pick_target_ray_score(target, origin, direction)).collect();
    assert!(scores.iter().all(Option::is_some), "the ray through the origin crosses every padded box");
    let vertex = scores[1].expect("vertex score");
    let object = scores[0].expect("object score");
    assert!(object < vertex, "the solid's front face is nearer than its center vertex ({object} < {vertex})");
    state.selection.pick_targets[0].points = vec![[-0.04; 3], [0.04; 3]];
    let tied_object = world_pick_target_ray_score(&state.selection.pick_targets[0], origin, direction).expect("coincident padded object score");
    assert!(vertex < tied_object, "React's spatialPickPriority makes the finest kind win at coincident padded bounds ({vertex} < {tied_object})");

    let modifiers = PointerModifiers::default();
    enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::pointer_button(200.0, 200.0, true, 0, &modifiers)).expect("press admitted");
    enqueue_world3d_event(&mut state.selection, &mut state.authority, WorldInteractionIntent::pointer_button(200.0, 200.0, false, 0, &modifiers)).expect("release admitted");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut published: Vec<ActionDescriptor> = Vec::new();
    let mut turns = 0;
    while let Some(generation) = world3d_interaction_front_generation(&state.camera, &state.authority) {
        let step = with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context));
        assert_ne!(step, WorldInteractionAuthorityStep::Fault, "a pick-target click never faults the authority");
        published.extend(take_actions(&mut input));
        turns += 1;
        assert!(turns < 4096, "a bounded pick terminates");
        if step == WorldInteractionAuthorityStep::Idle {
            break;
        }
    }
    let select = published.iter().find(|action| action.action == "interactionSelect").expect("a sub-object pick publishes interactionSelect");
    let args = select.args.as_ref().expect("interactionSelect carries args");
    assert_eq!(args.get("domainId").and_then(semio_framework_value::DslValue::as_str), Some("cad"), "the bound domain, not the world fallback");
    let targets = args.get("targets").and_then(semio_framework_value::DslValue::as_str).expect("targets is JSON text");
    assert!(targets.contains(r#""granularity":"vertex""#), "the WINNING target's own kind is the granularity: {targets}");
    assert!(targets.contains(r#""id":"v-1""#), "the kernel entity id, not the instance id: {targets}");
    assert!(!targets.contains("f-locked"), "an unselectable (hidden/locked) row is rendered and never hit: {targets}");
}

//#region 🌫️InstanceStyleWire
/// 🌫️ LAW (packet W14g items 3 + 4): the instance lane carries the per-instance OPACITY a locked
/// object is dimmed by, and the `highlighted`/`disabled`/`celebrating` flags that reach the three
/// `MESH_STYLE_PAINT` rows nothing published before.
///
/// 🎨️ Each flag resolves to its OWN row of React's table, not to the two rows that existed here.
#[test]
fn the_instance_lane_carries_locked_opacity_and_the_three_unreachable_style_rows() {
    let instances = r##"[
        {"id":"obj-1","meshId":"mesh-1","position":[0,0,0],"color":"#ffffff","disabled":true,"opacity":0.35},
        {"id":"obj-2","meshId":"mesh-1","position":[1,0,0],"color":"#ffffff","highlighted":true},
        {"id":"obj-3","meshId":"mesh-1","position":[2,0,0],"color":"#ffffff","celebrating":true}
    ]"##;
    let mut scene = scene_with_selection("{}");
    if let Some(world) = scene.world_3d.as_mut() {
        world.instances_json = instances.into();
    }
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.selection.disabled_instance_ids.contains("obj-1"));
    assert!(state.selection.highlighted_instance_ids.contains("obj-2"));
    assert!(state.selection.celebrating_instance_ids.contains("obj-3"));
    let alpha = state.scene.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == "obj-1").map(|instance| instance.color[3]).expect("the locked instance reaches a draw");
    assert!((alpha - 0.35).abs() < 1e-3, "WORLD_LOCKED_OPACITY_SCALE multiplies the authored alpha, got {alpha}");
    let opaque = state.scene.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == "obj-2").map(|instance| instance.color[3]).expect("obj-2 reaches a draw");
    assert!((opaque - 1.0).abs() < 1e-3, "an instance with no opacity keeps its authored alpha, got {opaque}");
    assert_eq!(resolve_mesh_style(MeshStyleState { disabled: true, ..MeshStyleState::default() }), MeshStyleKind::Disabled);
    assert_eq!(resolve_mesh_style(MeshStyleState { highlighted: true, ..MeshStyleState::default() }), MeshStyleKind::Highlighted);
    assert_eq!(resolve_mesh_style(MeshStyleState { celebrating: true, ..MeshStyleState::default() }), MeshStyleKind::Celebrated);
    retire_bridged_surface(&mut state);
}

/// 🎨️ LAW: actual JSON bridge ingestion retains the authority that supplied a neutral color; the
/// semantic case crosses the snapshot as white and is resolved from the live theme only at paint.
#[test]
fn the_scene_bridge_retains_semantic_authored_and_environment_color_provenance() {
    for (id, authored, environment, expected_source) in [
        ("semantic", None, None, ui_wgpu::wgpu::SceneColorSource3d::SemanticNeutral),
        ("authored", Some("#3f80bf"), None, ui_wgpu::wgpu::SceneColorSource3d::Authored),
        ("environment", None, Some("#b82e0f"), ui_wgpu::wgpu::SceneColorSource3d::Environment),
    ] {
        let mut scene = scene_with_selection("{}");
        let world = scene.world_3d.as_mut().expect("world scene");
        world.instances_json = serde_json::json!([{
            "id": id,
            "meshId": "mesh-1",
            "position": [0, 0, 0],
            "rotation": [0, 0, 0, 1],
            "scale": [1, 1, 1],
            "color": authored,
        }])
        .to_string();
        world.environment_json = environment.map(|color| serde_json::json!({ "material": { "color": color } }).to_string());
        let mut state = World3dState::new(format!("surface-{id}"), "controller".into());
        drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 320.0, 240.0));
        let retained = state.scene.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == id).expect("bridged instance retained");
        assert_eq!(retained.material.color_source, expected_source);
        if expected_source == ui_wgpu::wgpu::SceneColorSource3d::SemanticNeutral {
            assert_eq!(retained.color, [1.0; 4], "semantic authority is not materialized as a baked gray");
        }
        retire_bridged_surface(&mut state);
    }
}
//#endregion 🌫️InstanceStyleWire

//#region 🎛️GumballConfig
/// 🎛️ LAW (packet W14g item 5): the gumball's handle set is the PLUGIN-AUTHORED `gumballConfig` when
/// the producer published one (`cad`'s dislocate utility does), the single-mode fallback otherwise,
/// and the drafting-plane subset intersects both — React's `gumballHandleEnabled`.
///
/// 🎛️ No authored config → React's `gumballConfigForTransformMode`, whose `rotate` arm offers the
/// rings ONLY. This host used to admit the three move axes unconditionally in every mode.
///
/// 📐️ A Top pane's `xy` drafting plane keeps the in-plane move/scale pair and the NORMAL rotation.
#[test]
fn the_gumball_offers_the_authored_config_then_the_transform_mode_fallback_intersected_with_the_plane() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(
        &mut state,
        &scene_with_selection(r#"{"transformMode":"transform","gumballConfig":{"moveAxes":true,"movePlanes":true,"rotate":true,"scaleAxes":false,"scalePlanes":false,"scaleUniform":false}}"#),
        Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 },
    );
    let config = world3d_gumball_config(&state.gumball);
    assert!(config.admits(GumballHandle::MoveX) && config.admits(GumballHandle::MoveXY) && config.admits(GumballHandle::RotateZ));
    assert!(!config.admits(GumballHandle::ScaleX), "an authored config that disables scaling hides AND unpicks the scale handles");

    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(r#"{"transformMode":"rotateSelection"}"#), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let config = world3d_gumball_config(&state.gumball);
    assert!(config.admits(GumballHandle::RotateX));
    assert!(!config.admits(GumballHandle::MoveX), "React's rotate mode offers no translation handle");
    assert!(!config.admits(GumballHandle::ScaleZ));

    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(r#"{"transformMode":"transform","gumballConfig":{"plane":"xy"}}"#), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let config = world3d_gumball_config(&state.gumball);
    assert!(config.admits(GumballHandle::MoveX) && config.admits(GumballHandle::MoveY) && config.admits(GumballHandle::MoveXY) && config.admits(GumballHandle::RotateZ));
    assert!(!config.admits(GumballHandle::MoveZ) && !config.admits(GumballHandle::RotateX) && !config.admits(GumballHandle::MoveYZ));
    assert!(config.admits(GumballHandle::ScaleX), "an ABSENT group key means enabled — React reads every group as `!== false`");
}
//#endregion 🎛️GumballConfig

//#region 🔘️VertexMarkerPixels
/// 🔘️ LAW (packet W14g item 5): a vertex marker is sized in SCREEN PIXELS
/// (`WORLD_VERTEX_DOT_PX`/`WORLD_VERTEX_MARK_PX`, `sizeAttenuation={false}`), so it keeps its
/// apparent size as the camera pulls away instead of collapsing to a sub-pixel world-unit cross —
/// the same defect React shipped and fixed on lowpoly.
///
/// 🩸️ The pre-W14g cross was `VERTEX_BASE_SCALE * 0.15` = 0.0075 world units, at ANY distance.
#[test]
fn vertex_markers_hold_their_pixel_size_at_every_camera_distance() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    let viewport = state.session.bounds;
    let near = {
        let mut orbit = state.camera.orbit.clone();
        orbit.distance = 4.0;
        orbit.to_camera()
    };
    let far = {
        let mut orbit = state.camera.orbit.clone();
        orbit.distance = 400.0;
        orbit.to_camera()
    };
    let centre = Vec3::new(0.0, 0.0, 0.0);
    let near_half = vertex_marker_half_extent(&near, viewport, centre, VERTEX_BASE_SCALE, None);
    let far_half = vertex_marker_half_extent(&far, viewport, centre, VERTEX_BASE_SCALE, None);
    assert!(far_half > near_half * 10.0, "a marker 100x further away spans ~100x more world units to keep its pixels ({near_half} → {far_half})");
    let near_pixels = near_half / world_units_per_pixel(&near, viewport, near.position.sub(centre).length());
    assert!((near_pixels - WORLD_VERTEX_DOT_PX * 0.5).abs() < 1e-3, "the plain dot is React's 6 px, got {}", near_pixels * 2.0);
    let mark = vertex_marker_half_extent(&near, viewport, centre, VERTEX_HOVER_SCALE, None);
    assert!(mark > near_half, "a hovered/selected marker is React's larger 11 px mark");
    assert!(far_half > 0.0075, "the old world-unit cross was sub-pixel at this distance");
}
//#endregion 🔘️VertexMarkerPixels

#[test]
fn sync_parses_selection_targets_and_active_object() {
    let selection = r#"{
            "granularity":"vertex",
            "targets":{"mesh":true,"vertex":true,"edge":false,"face":false},
            "activeObjectId":"obj-1"
        }"#;
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(selection), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.selection.selection_targets.vertex);
    assert!(!state.selection.selection_targets.edge);
    assert_eq!(state.selection.active_object_id.as_deref(), Some("obj-1"));
}

/// 🎯️ One world surface fed a `fit` lane, as the generation3d preview publishes it.
fn scene_with_fit(fit_json: Option<&str>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection("{}");
    if let Some(world) = scene.world_3d.as_mut() {
        world.meshes_json = "[]".into();
        world.instances_json = "[]".into();
        world.fit_json = fit_json.map(str::to_string);
    }
    scene
}

/// 🎯️ The generation3d `box-fillet-preview` delivery extent, as `📚️examples/📐️box-fillet-preview/🧫️fixtures/🧩️example/🔣️.json` commits it.
const BOOT_FRAME_FIT_JSON: &str = r#"{"enabled":true,"revision":7,"padding":1.12,"boundsMin":[0.0,0.0,0.0],"boundsMax":[2.0,2.0,2.0]}"#;

#[test]
fn a_world_surface_frames_the_producers_delivered_bounds_on_the_first_delivery_of_a_document() {
    let bounds = Rect { x: 0.0, y: 0.0, w: 1600.0, h: 900.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene_with_fit(Some(r#"{"enabled":true,"revision":7,"padding":1.12}"#)), bounds);
    let seed = state.camera.orbit.clone();
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.camera.orbit.distance, seed.distance, "a fit lane without an extent or geometry has nothing to frame");
    sync_world3d_state(&mut state, &scene_with_fit(Some(BOOT_FRAME_FIT_JSON)), bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    assert_eq!([state.camera.orbit.target.x, state.camera.orbit.target.y, state.camera.orbit.target.z], [1.0, 1.0, 1.0], "the framed camera looks at the delivered box centre");
    let camera = state.camera.orbit.to_camera();
    let half_vertical = (camera.fov_y * 0.5).tan();
    let aspect = bounds.w / bounds.h;
    let forward = [camera.target.x - camera.position.x, camera.target.y - camera.position.y, camera.target.z - camera.position.z];
    let forward_length = (forward[0] * forward[0] + forward[1] * forward[1] + forward[2] * forward[2]).sqrt();
    let unit_forward = [forward[0] / forward_length, forward[1] / forward_length, forward[2] / forward_length];
    let right = [unit_forward[1] * camera.up.z - unit_forward[2] * camera.up.y, unit_forward[2] * camera.up.x - unit_forward[0] * camera.up.z, unit_forward[0] * camera.up.y - unit_forward[1] * camera.up.x];
    let right_length = (right[0] * right[0] + right[1] * right[1] + right[2] * right[2]).sqrt();
    let unit_right = [right[0] / right_length, right[1] / right_length, right[2] / right_length];
    let unit_up = [unit_right[1] * unit_forward[2] - unit_right[2] * unit_forward[1], unit_right[2] * unit_forward[0] - unit_right[0] * unit_forward[2], unit_right[0] * unit_forward[1] - unit_right[1] * unit_forward[0]];
    for corner in 0..8 {
        let point = [if corner & 1 == 0 { 0.0 } else { 2.0 }, if corner & 2 == 0 { 0.0 } else { 2.0 }, if corner & 4 == 0 { 0.0 } else { 2.0 }];
        let relative = [point[0] - camera.position.x, point[1] - camera.position.y, point[2] - camera.position.z];
        let depth = relative[0] * unit_forward[0] + relative[1] * unit_forward[1] + relative[2] * unit_forward[2];
        assert!(depth > 0.0, "corner {corner} is behind the framed camera");
        let ndc_y = (relative[0] * unit_up[0] + relative[1] * unit_up[1] + relative[2] * unit_up[2]) / (depth * half_vertical);
        let ndc_x = (relative[0] * unit_right[0] + relative[1] * unit_right[1] + relative[2] * unit_right[2]) / (depth * half_vertical * aspect);
        assert!(ndc_x.abs() <= 1.0 && ndc_y.abs() <= 1.0, "corner {corner} projects to ({ndc_x}, {ndc_y}) — outside the viewport");
    }
}

#[test]
fn a_re_delivery_of_the_same_document_never_takes_back_a_camera_the_user_moved() {
    let bounds = Rect { x: 0.0, y: 0.0, w: 1600.0, h: 900.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let scene = scene_with_fit(Some(BOOT_FRAME_FIT_JSON));
    drive_scene_bridge(&mut state, &scene, bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    let framed = state.camera.orbit.clone();
    state.camera.orbit.orbit(40.0, 12.0);
    state.camera.orbit.zoom(-120.0);
    state.camera.camera_user_moved = true;
    let moved = state.camera.orbit.clone();
    state.bridge.scene_bridge_digest = None;
    drive_scene_bridge(&mut state, &scene, bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.camera.orbit.yaw, moved.yaw, "a second delivery of the same document re-framed a camera the user had moved");
    assert_eq!(state.camera.orbit.distance, moved.distance);
    assert_ne!(moved.yaw, framed.yaw, "the gesture must actually have moved the camera for this law to say anything");
    let next_document = BOOT_FRAME_FIT_JSON.replace("\"revision\":7", "\"revision\":8");
    sync_world3d_state(&mut state, &scene_with_fit(Some(&next_document)), bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    assert_eq!(state.camera.orbit.yaw, moved.yaw, "framing a new document keeps the look direction the user chose");
    assert_eq!([state.camera.orbit.target.x, state.camera.orbit.target.y, state.camera.orbit.target.z], [1.0, 1.0, 1.0]);
    assert!((state.camera.orbit.distance - framed.distance).abs() < 1e-3, "the new document is framed again, at the fitting distance");
}

#[test]
fn sync_parses_numeric_component_ids_and_hovered_component() {
    let selection = r#"{
            "granularity":"vertex",
            "componentIds":[1,2],
            "hoveredComponent":{"objectId":"obj-1","mode":"vertex","id":3},
            "showEdges":true
        }"#;
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(selection), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.selection.granularity, "vertex");
    assert_eq!(state.selection.component_ids, vec!["1".to_string(), "2".to_string()]);
    assert_eq!(state.selection.hovered_component_id.as_deref(), Some("3"));
    assert_eq!(state.selection.hovered_component_object_id.as_deref(), Some("obj-1"));
    assert_eq!(state.selection.hovered_component_mode.as_deref(), Some("vertex"));
    assert!(state.selection.show_edges);
}

/// ⚪️ 4 from edges + 24 from 4 vertex crosses
#[test]
fn append_component_vertex_spheres_render_base_vertices() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.selection.selection_targets.vertex = true;
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let instances = append_component_vertex_spheres();
    assert_eq!(instances.len(), 0);

    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &ui_wgpu::wgpu::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert_eq!(lines.len(), 28);
}

/// 🔘️ The viewport the component-overlay laws measure vertex markers against — their size is now in
/// SCREEN PIXELS (`vertex_marker_half_extent`), so every one of them needs a camera and a rect.
const COMPONENT_OVERLAY_VIEWPORT: Rect = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

#[test]
fn append_component_overlays_highlights_only_hovered_edge() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "edge".into();
    state.selection.selection_targets.edge = true;
    state.selection.hovered_component_id = Some("5".into());
    state.selection.hovered_component_object_id = Some("obj-1".into());
    state.selection.hovered_component_mode = Some("edge".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &ui_wgpu::wgpu::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(lines.len() >= 2);
    assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
}

#[test]
fn append_component_overlays_highlights_selected_edge() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "edge".into();
    state.selection.component_ids = vec!["6".into()];
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &ui_wgpu::wgpu::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(lines.len() >= 2);
    assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
}

#[test]
fn component_mode_does_not_apply_mesh_instance_hover() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.hovered_component_object_id = Some("obj-1".into());
    state.selection.local_hover_id = None;
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
    assert!(!state.scene.draws[0].instances[0].hovered);
}

#[test]
fn runtime_draw_flags_clear_stale_instance_selected_when_selection_is_empty() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.selected_ids.clear();
    state.selection.local_hover_id = None;
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: true, hovered: true, material: Default::default() }], shadow_role: Default::default() });
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
    assert!(!state.scene.draws[0].instances[0].selected, "empty selection must clear a stale instancesJson selected bit");
    assert!(!state.scene.draws[0].instances[0].hovered, "empty hover must clear a stale instancesJson hovered bit");
}

#[test]
fn pick_select_emits_original_vertex_interaction_target() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::ZERO, inner.w, inner.h).expect("vertex projects");
    let action = pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, screen[0], screen[1], inner, false, false).expect("pick action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert_eq!(targets[0], serde_json::json!({"granularity":"vertex","id":"surface-1/obj-1.vertex.1"}));
}

#[test]
fn marquee_preview_respects_pick_bounds_offset() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 100.0, y: 50.0, w: 400.0, h: 400.0 };
    state.session.pick_bounds = inner;
    state.selection.marquee_points = vec![[110.0, 60.0], [490.0, 450.0]];
    update_marquee_preview(&mut state.camera, &mut state.selection, &mut state.scene, inner);
    assert!(!state.selection.marquee_preview_ids.is_empty(), "preview ids: {:?}", state.selection.marquee_preview_ids);
}

#[test]
fn marquee_crossing_includes_partial_overlap_window_does_not() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "mesh".into();
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let view_proj = camera.view_proj(800.0, 800.0);
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for corner in [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]] {
        let screen = ui_wgpu::wgpu::project_point(view_proj, Vec3::from_array(corner), inner.w, inner.h).expect("screen");
        min_x = min_x.min(screen[0]);
        min_y = min_y.min(screen[1]);
        max_x = max_x.max(screen[0]);
        max_y = max_y.max(screen[1]);
    }
    let center_x = (min_x + max_x) * 0.5;
    let center_y = (min_y + max_y) * 0.5;
    state.selection.marquee_points = vec![[inner.x + min_x, inner.y + min_y], [inner.x + center_x, inner.y + center_y]];
    update_marquee_preview(&mut state.camera, &mut state.selection, &mut state.scene, inner);
    assert!(state.selection.marquee_preview_ids.is_empty(), "window marquee should not select partially enclosed mesh");
    state.selection.marquee_points = vec![[inner.x + center_x, inner.y + min_y], [inner.x + min_x, inner.y + center_y]];
    update_marquee_preview(&mut state.camera, &mut state.selection, &mut state.scene, inner);
    assert!(!state.selection.marquee_preview_ids.is_empty(), "crossing marquee should select partially enclosed mesh");
}

#[test]
fn marquee_component_mode_emits_exact_interaction_targets() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.selection.component_ids = vec!["1".into()];
    state.selection.marquee_points = vec![[10.0, 10.0], [390.0, 390.0]];
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let action = marquee_select_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, true, false).expect("marquee action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert!(!targets.as_array().unwrap().is_empty());
    assert!(targets.as_array().unwrap().iter().all(|target| target["granularity"] == "vertex" && target["id"].as_str().unwrap().starts_with("surface-1/obj-1.vertex.")));
}

#[test]
fn click_release_routes_to_pick_select_instead_of_empty_marquee() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.active_utility = "select".into();
    state.selection.granularity = "mesh".into();
    state.selection.marquee_active = true;
    state.selection.marquee_points = vec![[120.0, 140.0]];
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let action = handle_world3d_pointer_button(&mut state, 120.0, 140.0, false, 0, &PointerModifiers::default()).expect("click should pick");
    assert_eq!(action.action, "interactionSelect");
    assert!(!state.selection.marquee_active);
}

#[test]
fn marquee_face_preview_and_overlay_use_logical_face_ids() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = bounds;
    state.session.pick_bounds = bounds;
    state.selection.marquee_points = vec![[390.0, 10.0], [10.0, 390.0]];
    update_marquee_preview(&mut state.camera, &mut state.selection, &mut state.scene, bounds);
    assert!(state.selection.marquee_preview_ids.iter().any(|id| id == "10" || id == "11"), "preview ids: {:?}", state.selection.marquee_preview_ids);
    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &ui_wgpu::wgpu::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(!lines.is_empty(), "face marquee preview should draw triangle edge lines");
}

#[test]
fn fresh_scene_selection_without_hover_clears_previous_component_hover() {
    let selection = r#"{"granularity":"face","componentIds":[10]}"#;
    let scene = scene_with_selection(selection);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.hovered_component_id = Some("11".into());
    state.selection.hovered_component_object_id = Some("obj-1".into());
    state.selection.hovered_component_mode = Some("face".into());
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.selection.hovered_component_id,None);
    assert_eq!(state.selection.hovered_component_object_id,None);
    assert_eq!(state.selection.hovered_component_mode,None);
}

#[test]
fn apply_world_action_preview_updates_component_hover_and_selection() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0;4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();
    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionHover".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "targets": serde_json::json!([{"granularity":"vertex","id":"surface-1/obj-1.vertex.2"}]).to_string(),
                "channel": "pointer",
            })),
        },
    );
    assert_eq!(state.selection.hovered_component_id.as_deref(), Some("2"));
    assert_eq!(state.selection.hovered_component_mode.as_deref(), Some("vertex"));
    assert!(state.selection.local_hover_id.is_none());

    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "targets": serde_json::json!([{"granularity":"vertex","id":"surface-1/obj-1.vertex.4"}]).to_string(),
                "method": "pick",
                "merge": "replace",
            })),
        },
    );
    assert_eq!(state.selection.component_ids, vec!["4".to_string()]);
    assert_eq!(state.selection.granularity, "vertex");
}

#[test]
fn preview_survives_sync_when_scene_json_unchanged() {
    let selection = r#"{"granularity":"vertex","componentIds":[1],"hoveredComponent":{"objectId":"obj-1","mode":"vertex","id":2}}"#;
    let scene = scene_with_selection(selection);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0;4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();
    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "targets": serde_json::json!([{"granularity":"vertex","id":"surface-1/obj-1.vertex.5"}]).to_string(),
                "method": "pick",
                "merge": "replace",
            })),
        },
    );
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.selection.component_ids, vec!["5".to_string()]);
}

#[test]
fn typed_camera_snapshot_matches_current_camera_fixture_without_production_parsing() {
    let mut scene = scene_with_selection("{}");
    let mut page = ui_wgpu::wgpu::World3dSnapshotPage::new(World3dSnapshotPageKind::Camera);
    page.push_item(World3dSnapshotItem { numbers: [4.0, 4.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0], number_len: 12, ..Default::default() }).unwrap();
    page.seal().unwrap();
    let descriptor = ui_wgpu::wgpu::World3dSnapshotDescriptor { revision: 5, generation: 7, page_count: 1, item_count: 1, byte_count: 0, draw_count: 0, draw_instance_count: 0, draw_byte_count: 0 };
    let token = ui_wgpu::wgpu::world3d_snapshot_begin(descriptor).unwrap();
    ui_wgpu::wgpu::world3d_snapshot_admit_page(token, page).unwrap();
    let lease = ui_wgpu::wgpu::world3d_snapshot_seal(token).unwrap();
    scene.world_3d.as_mut().unwrap().snapshot = Some(lease);

    let bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    let mut typed = World3dState::new("surface".into(), "controller".into());
    sync_world3d_state(&mut typed, &scene, bounds);
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| step_world3d_snapshot(&mut typed, context)) == World3dSnapshotApplyStep::Complete {
            break;
        }
        assert!(turns < 8);
    }
    let oracle: serde_json::Value = serde_json::from_str(&scene.world_3d.as_ref().unwrap().camera_json).expect("camera fixture");
    let vector = |key: &str| Vec3::new(oracle[key][0].as_f64().unwrap() as f32, oracle[key][1].as_f64().unwrap() as f32, oracle[key][2].as_f64().unwrap() as f32);
    let expected = OrbitController::from_camera(&Camera3d {
        position: vector("position"),
        target: vector("target"),
        up: vector("up"),
        fov_y: oracle["fov"].as_f64().unwrap() as f32 * std::f32::consts::PI / 180.0,
        near: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_NEAR,
        far: ui_wgpu::wgpu::WORLD_ORBIT_CAMERA_MIN_FAR,
        projection: ui_wgpu::wgpu::CameraProjection3d::Perspective,
        zoom: oracle["zoom"].as_f64().unwrap_or(1.0) as f32,
    })
    .to_camera();
    let actual = typed.camera.orbit.to_camera();
    assert_eq!(actual.position, expected.position);
    assert_eq!(actual.target, expected.target);
    assert_eq!(actual.up, expected.up);
    assert_eq!(actual.fov_y, expected.fov_y);
    assert_eq!(typed.bridge.snapshot_lease, Some(lease));

    ui_wgpu::wgpu::world3d_snapshot_begin_close(lease).unwrap();
    assert!(!ui_wgpu::wgpu::world3d_snapshot_close_step(lease).unwrap());
    assert!(ui_wgpu::wgpu::world3d_snapshot_close_step(lease).unwrap());
}

/// 🗑️ A url the renderer's decoder refused is offered ONCE. Both request loops re-derive their
/// missing urls from the scene every frame (`missing_mesh_urls`, `reference_image_urls`), so without
/// this ledger one refused response is re-fetched for the lifetime of the surface — the shape
/// `terrain_tile_misses` already gives DEM tiles and React gives a failed texture url.
#[test]
fn a_refused_asset_url_is_never_offered_again_by_its_surface() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.scene.mesh_source_urls.insert("mesh".into(), "/mesh/🧊️refused.glb".into());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances: Vec::new(), shadow_role: Default::default() }).expect("one scene draw");
    state.scene.references.push(WorldReferenceRecord { url: Some("/plan.jpg".into()), origin: Some([0.0; 3]), width_world: Some(1.0), hidden: Some(false), ..Default::default() });
    let offered = |state: &World3dState| -> (Vec<String>, Vec<String>) {
        let meshes = state.scene.draws.iter().filter(|draw| !state.scene.meshes.contains_key(&draw.mesh_key)).filter_map(|draw| state.scene.mesh_source_urls.get(&draw.mesh_key).cloned()).filter(|url| !state.assets.asset_url_misses.contains(url)).collect();
        let references = state.scene.references.iter().filter_map(|reference| reference.url.clone()).filter(|url| !world_reference_raster_ready(&state.scene, url)).filter(|url| !state.assets.asset_url_misses.contains(url)).collect();
        (meshes, references)
    };
    assert_eq!(offered(&state), (vec!["/mesh/🧊️refused.glb".to_string()], vec!["/plan.jpg".to_string()]), "both loops offer an un-refused url");
    assert!(!world3d_asset_url_missed(&state.assets, "/mesh/🧊️refused.glb"));
    state.assets.pending_glb_urls.insert("/mesh/🧊️refused.glb".into());
    state.assets.pending_image_urls.insert("/plan.jpg".into());

    mark_world3d_asset_miss(&mut state.assets, "/mesh/🧊️refused.glb");
    mark_world3d_asset_miss(&mut state.assets, "/plan.jpg");
    assert!(world3d_asset_url_missed(&state.assets, "/mesh/🧊️refused.glb"));
    assert!(world3d_asset_url_missed(&state.assets, "/plan.jpg"));
    assert!(!world3d_asset_url_missed(&state.assets, "/mesh/🧊️other.glb"));
    assert!(!state.assets.pending_glb_urls.contains("/mesh/🧊️refused.glb"));
    assert!(!state.assets.pending_image_urls.contains("/plan.jpg"));
    assert_eq!(offered(&state), (Vec::new(), Vec::new()), "neither loop offers a refused url again");
}

#[test]
fn dynamic_world_owners_retire_one_nested_owner_per_grant_to_terminal_empty() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mesh = publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0; 3 * 128], vec![0.0; 3 * 128], vec![0; 3 * 64]));
    state.scene.meshes.insert("mesh".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances: (0..32).map(|index| Instance3d { component_source: None, id: format!("instance-{index}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }).collect(), shadow_role: Default::default() });
    state.scene.reference_pixels.insert("reference".into(), test_world_scene_raster(64, 64, 0));
    let paint = test_scene_raster_lease(64, 64, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 3 }), 0);
    state.scene.mesh_paint_textures.insert("paint".into(), WorldSceneRaster { identity: paint.identity(), pending: Mutex::new(Some(paint)) });
    assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    assert!(!begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    assert!(!with_world_step_context(0, |context| terrain_test_retire_then_dynamic(&mut state,context)));
    let mut turns = 0;
    while !with_world_step_context(1, |context| terrain_test_retire_then_dynamic(&mut state,context)) {
        turns += 1;
        assert!(turns < 16);
    }
    assert!(turns >= 8, "each registry transition and opaque owner transfer consumes a distinct grant");
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
}

fn admit_draw_fixture_instance(state: &mut World3dState, draw: u16, id: &str) -> Result<(), WorldDynamicFault> {
    world3d_draw_rebuild_admit_instance(&mut state.scene, draw, id, Mat4::identity(), [1.0; 4], false, false)
}

fn draw_fixture_bytes(draws: &[(&str, &[&str])]) -> u32 {
    draws.iter().map(|(mesh, instances)| mesh.len() + size_of::<SceneDraw3d>() + instances.iter().map(|id| id.len() + size_of::<Instance3d>()).sum::<usize>()).sum::<usize>() as u32
}

#[test]
fn retained_draw_rebuild_preserves_mixed_group_and_instance_fifo_then_swaps_atomically() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let bytes = draw_fixture_bytes(&[("mesh-a", &["a-0", "a-1"]), ("mesh-b", &["b-0"])]);
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 2, instance_count: 3, byte_count: bytes }).unwrap();
    world3d_draw_rebuild_admit_draw(&mut state.scene, "mesh-a", 1, 2).unwrap();
    world3d_draw_rebuild_admit_draw(&mut state.scene, "mesh-b", 2, 1).unwrap();
    admit_draw_fixture_instance(&mut state, 0, "a-0").unwrap();
    admit_draw_fixture_instance(&mut state, 0, "a-1").unwrap();
    admit_draw_fixture_instance(&mut state, 1, "b-0").unwrap();
    world3d_draw_rebuild_seal(&mut state.scene).unwrap();
    assert!(state.scene.draws.is_empty(), "sealed drafts never publish partially");
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)) == WorldDrawRebuildStep::Pending {
        turns += 1;
        assert!(turns < 16);
    }
    assert_eq!(state.scene.draws.iter().map(|draw| draw.mesh_key.as_str()).collect::<Vec<_>>(), vec!["mesh-a", "mesh-b"]);
    assert_eq!(state.scene.draws[0].instances.iter().map(|instance| instance.id.as_str()).collect::<Vec<_>>(), vec!["a-0", "a-1"]);
    assert_eq!(state.scene.draws[1].instances[0].id, "b-0");
    assert_eq!(state.scene.draw_generation, 1);
}

#[test]
fn retained_draw_rebuild_rejects_byte_and_identifier_plus_one_before_owner_publication() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 0, byte_count: 0 }).unwrap();
    let rejected = world3d_draw_rebuild_admit_draw(&mut state.scene, "mesh", 1, 0).expect_err("byte +1 leaves borrowed mesh owner at producer");
    assert_eq!(rejected, WorldDynamicFault::ByteCapacity);
    while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state.scene, context)) {}
    let long = "x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY + 1);
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 0, byte_count: WORLD_DYNAMIC_DRAW_BYTE_CAPACITY as u32 }).unwrap();
    let rejected = world3d_draw_rebuild_admit_draw(&mut state.scene, &long, 1, 0).expect_err("ID +1 leaves borrowed mesh owner at producer");
    assert_eq!(rejected, WorldDynamicFault::IdCapacity);
    while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state.scene, context)) {}
}

#[test]
fn retained_draw_rebuild_rejects_aggregate_instance_plus_one_before_draft_ownership() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let result = begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: (WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY + 1) as u32, byte_count: WORLD_DYNAMIC_DRAW_BYTE_CAPACITY as u32 });
    assert_eq!(result, Err(WorldDynamicFault::InstanceCapacity));
    assert!(world3d_draw_rebuild_terminal_is_empty(&state.scene));
}

#[test]
fn retained_draw_rebuild_stale_and_interrupted_close_never_publish() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let bytes = draw_fixture_bytes(&[("mesh", &["one", "two"])]);
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 2, byte_count: bytes }).unwrap();
    world3d_draw_rebuild_admit_draw(&mut state.scene, "mesh", 1, 2).unwrap();
    admit_draw_fixture_instance(&mut state, 0, "one").unwrap();
    admit_draw_fixture_instance(&mut state, 0, "two").unwrap();
    world3d_draw_rebuild_seal(&mut state.scene).unwrap();
    state.session.interaction_revision = 1;
    assert_eq!(with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)), WorldDrawRebuildStep::Stale);
    assert!(!with_world_step_context(0, |context| close_world3d_draw_rebuild_step(&mut state.scene, context)));
    let mut turns = 0;
    while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state.scene, context)) {
        turns += 1;
        assert!(turns < 8);
    }
    assert!(state.scene.draws.is_empty());
    assert!(world3d_draw_rebuild_terminal_is_empty(&state.scene));
}

#[test]
fn pick_viewport_uses_render_bounds_not_pick_clip_offset() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "vertex".into();
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let bounds = Rect { x: 0.0, y: 50.0, w: 400.0, h: 400.0 };
    let clip = Rect { x: 0.0, y: 100.0, w: 400.0, h: 400.0 };
    state.session.bounds = bounds;
    state.session.pick_bounds = clip;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, bounds.w, bounds.h), Vec3::ZERO, bounds.w, bounds.h).expect("vertex projects");
    let global_x = bounds.x + screen[0];
    let global_y = bounds.y + screen[1];
    let picked = pick_component_at(&state.session, &state.camera, &state.selection, &state.scene, global_x, global_y, bounds).expect("vertex pick respects render viewport");
    assert_eq!(picked.1, "1");
}

#[test]
fn append_component_face_overlay_lines_include_hovered_face() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.hovered_component_mode = Some("face".into());
    state.selection.hovered_component_id = Some("10".into());
    state.selection.hovered_component_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let mut lines = Vec::new();
    append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay, &ui_wgpu::wgpu::Theme::default(), &state.camera.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(lines.len() >= 6, "hovered face should emit triangle edge lines, got {}", lines.len());
}

#[test]
fn pick_component_at_face_mode_uses_ray_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "face".into();
    state.selection.active_object_id = Some("obj-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let mesh_ref = state.scene.meshes.get("mesh-1").expect("mesh");
    let tri = world_mesh_triangle(*mesh_ref, 0).expect("triangle");
    let centroid = mesh_vertex(*mesh_ref, tri[0]).expect("first vertex").add(mesh_vertex(*mesh_ref, tri[1]).expect("second vertex")).add(mesh_vertex(*mesh_ref, tri[2]).expect("third vertex")).scale(1.0 / 3.0);
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), centroid, inner.w, inner.h).expect("face centroid projects");
    let picked = pick_component_at(&state.session, &state.camera, &state.selection, &state.scene, screen[0], screen[1], inner).expect("face pick");
    assert_eq!(picked.0, "face");
    assert_eq!(picked.2, "obj-1");
}

#[test]
fn pick_component_at_edge_mode_uses_ray_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.granularity = "edge".into();
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let edge = state.scene.meshes.get("mesh-1").expect("mesh").edge(0).expect("edge");
    let a = Vec3::new(edge[0][0], edge[0][1], edge[0][2]);
    let b = Vec3::new(edge[1][0], edge[1][1], edge[1][2]);
    let mid = a.add(b).scale(0.5);
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), mid, inner.w, inner.h).expect("edge midpoint projects");
    let picked = pick_component_at(&state.session, &state.camera, &state.selection, &state.scene, screen[0], screen[1], inner).expect("edge pick");
    assert_eq!(picked.0, "edge");
    assert_eq!(picked.2, "obj-1");
}

#[test]
fn mesh_pool_release_clears_at_zero_refcount() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.mesh_pool.acquire("mesh-1".into());
    assert!(state.scene.mesh_pool.release(&"mesh-1".into()));
    assert!(!state.scene.mesh_pool.contains(&"mesh-1".to_string()));
    state.scene.mesh_pool.acquire("mesh-1".into());
    state.scene.mesh_pool.acquire("mesh-1".into());
    assert!(!state.scene.mesh_pool.release(&"mesh-1".into()));
    assert!(state.scene.mesh_pool.contains(&"mesh-1".to_string()));
    assert!(state.scene.mesh_pool.release(&"mesh-1".into()));
    assert!(!state.scene.mesh_pool.contains(&"mesh-1".to_string()));
}

#[test]
fn resolve_physical_mesh_id_picks_closest_lod_url() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.mesh_lod_catalog.insert("tower".into(), vec![WorldMeshLodEntry { lod: 1.0, url: "https://example.com/tower-high.glb".into() }, WorldMeshLodEntry { lod: 100.0, url: "https://example.com/tower-low.glb".into() }]);
    let detailed = resolve_physical_mesh_id(&state.scene, "tower", 2.0);
    let coarse = resolve_physical_mesh_id(&state.scene, "tower", 200.0);
    assert_eq!(detailed, "mesh:tower-high");
    assert_eq!(coarse, "mesh:tower-low");
}

fn publish_retained_draw_fixture(state: &mut World3dState, vector: &serde_json::Value) {
    let mesh = vector["mesh"].as_str().unwrap();
    let instances = vector["instances"].as_array().unwrap();
    let ids: Vec<_> = instances.iter().map(|instance| instance["id"].as_str().unwrap()).collect();
    begin_world3d_draw_rebuild(&mut state.session, &mut state.retirement, &mut state.scene, WorldDrawRebuildDescriptor { generation: state.scene.draw_generation + 1, revision: state.session.interaction_revision, draw_count: 1, instance_count: instances.len() as u32, byte_count: draw_fixture_bytes(&[(mesh, &ids)]) })
        .unwrap();
    world3d_draw_rebuild_admit_draw(&mut state.scene, mesh, vector["version"].as_u64().unwrap(), instances.len() as u16).unwrap();
    for instance in instances {
        let color = std::array::from_fn(|index| instance["color"][index].as_f64().unwrap() as f32);
        world3d_draw_rebuild_admit_instance(&mut state.scene, 0, instance["id"].as_str().unwrap(), Mat4::identity(), color, false, false).unwrap();
    }
    world3d_draw_rebuild_seal(&mut state.scene).unwrap();
    for _ in 0..16 {
        match with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)) {
            WorldDrawRebuildStep::Complete => return,
            WorldDrawRebuildStep::Pending => {}
            other => panic!("draw fixture failed: {other:?}"),
        }
    }
    panic!("draw fixture exceeded its bounded turn count");
}

#[test]
fn retained_draw_rebuild_keeps_url_backed_asset_authority() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🔣️draws.json")).unwrap();
    let vector = &fixture["draws"][1];
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let token = reserve_world3d_asset_request(&mut state.session, &mut state.assets, WorldAssetRequestKind::Glb, vector["url"].as_str().unwrap()).unwrap();
    publish_retained_draw_fixture(&mut state, vector);
    assert!(!state.scene.meshes.contains_key(vector["mesh"].as_str().unwrap()));
    assert!(state.scene.placeholder_build.is_none(), "retained draws never replace an admitted asset request with a primitive");
    let mut owner = take_next_world3d_asset(&mut state.assets).expect("exact retained asset request");
    assert_eq!(owner.token(), token);
    assert_eq!(owner.url(), vector["url"].as_str().unwrap());
    owner.begin_close();
    return_world3d_asset(&mut state.assets, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state.assets) {}
    assert!(state.assets.asset_io.terminal_is_empty());
    assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    while !with_world_step_context(1, |context| terrain_test_retire_then_dynamic(&mut state,context)) {}
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
}

/// 🥅️ The puzzle3d mesh wire: two built-in KINDS, then one GLB named by URL. The bridge must keep
/// all three and every instance placed on them.
fn url_mesh_wire_scene() -> UiComponentSceneNode {
    let mut scene = scene_with_selection_and_domain("{}", None);
    let world = scene.world_3d.as_mut().expect("world scene");
    world.meshes_json = r#"[{"id":"box","kind":"box"},{"id":"vortex-marker","kind":"vortex-marker"},{"id":"mesh:🧊️left","url":"/mesh/🧊️left.glb"}]"#.into();
    world.instances_json = r#"[{"id":"obj-box","meshId":"box","position":[0,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]},{"id":"obj-left","meshId":"mesh:🧊️left","position":[2,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into();
    scene
}

const URL_MESH_ID: &str = "mesh:🧊️left";

const URL_MESH_URL: &str = "/mesh/🧊️left.glb";

/// 🧹️ Retires everything a bridged surface holds — snapshot leases live in ONE process-wide
/// fixed pool, so a test that publishes one and walks away starves every later test with
/// `World3dSnapshotFault::Unavailable`.
fn retire_bridged_surface(state: &mut World3dState) {
    assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
    for _ in 0..4_096 {
        if with_world_step_context(1, |context| terrain_test_retire_then_dynamic(state,context)) {
            break;
        }
    }
    assert!(world3d_dynamic_retirement_terminal_is_empty(state), "the surface must reach terminal empty");
}

/// 🥽️🌉️ A mesh the wire declares by URL keeps its draw and its instances, and is left PENDING
/// rather than stood in for.
///
/// 🩸️ `World3dSceneBridgePhase::Parse` used to `retain` only meshes that already carried triangles,
/// then drop every instance whose mesh was gone — so puzzle3d's `{"id":"mesh:…","url":"/mesh/….glb"}`
/// entries AND its `{"id":"box","kind":"box"}` entries vanished with every object placed on them
/// (`state-draws=0 state-instances=0`, `dumpMeshStats` all `indices:0`). Ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3a-asset-decoder-boot-fault.md` §6.
#[test]
fn the_scene_bridge_keeps_url_and_kind_declared_meshes_with_their_instances() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    assert_eq!(state.session.snapshot_fault, None);
    let drawn: Vec<(String, Vec<String>)> = state.scene.draws.iter().map(|draw| (draw.mesh_key.clone(), draw.instances.iter().map(|instance| instance.id.clone()).collect())).collect();
    assert_eq!(drawn, vec![("box".to_string(), vec!["obj-box".to_string()]), (URL_MESH_ID.to_string(), vec!["obj-left".to_string()])], "both the kind mesh and the url mesh keep their draw and their instances");

    assert!(state.scene.meshes.contains_key("box"), "a built-in kind resolves through the placeholder ladder both renderers own");
    assert!(!state.scene.meshes.contains_key(URL_MESH_ID), "a url mesh is owed by the asset pipeline and must NOT be stood in for: a placeholder suppresses its own fetch forever");
    assert_eq!(state.scene.mesh_source_urls.get(URL_MESH_ID).map(String::as_str), Some(URL_MESH_URL), "the bridge binds the mesh id to the url the fetch loop reads");
    retire_bridged_surface(&mut state);
}

/// 🎨️ LAW: geometry provenance selects React's standard-material pair inside one delivered scene:
/// inline geometry remains `0/1`, while the adjacent GLB consumes the environment override.
#[test]
fn one_scene_resolves_distinct_inline_and_glb_standard_materials() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎨️scene-shading/🔣️.json")).unwrap();
    let mut scene = url_mesh_wire_scene();
    scene.world_3d.as_mut().unwrap().environment_json = Some(serde_json::json!({ "material": fixture["mixedMaterialScope"]["environment"] }).to_string());
    let mut state = World3dState::new("surface-material".into(), "controller-material".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    let environment = environment_scene_material(&state.scene.environment);
    assert_eq!(world3d_standard_material_for_mesh(&state.scene, "box", environment), [0.0, 1.0]);
    assert_eq!(world3d_standard_material_for_mesh(&state.scene, URL_MESH_ID, environment), [0.35, 0.58]);
    assert_eq!(world3d_standard_material_for_mesh(&state.scene, URL_MESH_ID, SceneMaterial3d::default()), [0.0, 1.0]);
    retire_bridged_surface(&mut state);
}

/// 🥽️📡️ The url lane end to end inside the surface: the draw offers its url exactly once, the
/// decoded mesh lands under the wire's own id, and the instance draws from then on.
#[test]
fn a_url_declared_mesh_reserves_one_glb_fetch_and_draws_once_its_mesh_lands() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    assert!(state.assets.pending_glb_urls.contains(URL_MESH_URL), "the live predicate that keeps a per-frame offer idempotent");
    let mut owner = take_next_world3d_asset(&mut state.assets).expect("exactly one admitted GLB request");
    assert_eq!(owner.url(), URL_MESH_URL);
    assert_eq!(owner.kind(), WorldAssetRequestKind::Glb);
    assert!(take_next_world3d_asset(&mut state.assets).is_none(), "a re-offered url never doubles its request");

    let mesh = publish_oracle_mesh_at_revision(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]), state.session.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, URL_MESH_URL, mesh).expect("the decoded GLB publishes under the wire's mesh id");
    let resident = *state.scene.meshes.get(URL_MESH_ID).expect("the fetched GLB is resident under the id the wire named");
    let schema = resident.schema().expect("resident mesh schema");
    assert_eq!((schema.vertices, schema.indices), (3, 3), "the resident mesh carries real positions and indices");
    assert!(!state.assets.pending_glb_urls.contains(URL_MESH_URL), "a landed mesh stops being pending");

    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    assert!(take_next_world3d_asset(&mut state.assets).is_none(), "a resident mesh is never re-fetched");
    let draw = state.scene.draws.iter().find(|draw| draw.mesh_key == URL_MESH_ID).expect("the url mesh keeps its draw");
    assert_eq!(draw.instances.len(), 1, "the instance draws now that its mesh is resident");

    owner.begin_close();
    return_world3d_asset(&mut state.assets, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state.assets) {}
    retire_bridged_surface(&mut state);
}

/// 🗑️ A refused GLB is a miss, not an endless retry.
#[test]
fn a_refused_mesh_url_is_never_offered_again_by_its_surface() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    let mut owner = take_next_world3d_asset(&mut state.assets).expect("admitted GLB request");
    owner.begin_close();
    return_world3d_asset(&mut state.assets, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state.assets) {}

    mark_world3d_asset_miss(&mut state.assets, URL_MESH_URL);
    assert!(!state.assets.pending_glb_urls.contains(URL_MESH_URL), "a refusal clears the pending mark it replaces");
    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    assert!(take_next_world3d_asset(&mut state.assets).is_none(), "a refused url is never offered again");
    retire_bridged_surface(&mut state);
}

/// 👻️ A ghost over a mesh the surface is still fetching draws through the shared `box` primitive, not
/// under that mesh's own id — minting a stand-in there would make the id resident and silence
/// `offer_missing_mesh_fetches` for the mesh the ghost is standing in for. Once the GLB lands the
/// ghost switches to the real mesh, exactly like React's `BrushPreviewGhost`.
#[test]
fn a_ghost_never_stands_in_under_the_id_of_a_mesh_the_surface_is_still_fetching() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    assert_eq!(ghost_mesh_id(&state.scene, Some(URL_MESH_URL)), "box", "a pending url ghosts as the shared primitive");
    assert_eq!(ghost_mesh_id(&state.scene, None), "box");
    assert_eq!(ghost_mesh_id(&state.scene, Some("/mesh/🧊️unrelated.glb")), "mesh:🧊️unrelated", "a url the scene never named keeps its own lazy ghost key");

    let mesh = publish_oracle_mesh_at_revision(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]), state.session.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, URL_MESH_URL, mesh).expect("the decoded GLB publishes");
    assert_eq!(ghost_mesh_id(&state.scene, Some(URL_MESH_URL)), URL_MESH_ID, "once the mesh lands the ghost draws the real thing");
    retire_bridged_surface(&mut state);
}

/// 🌉️ Drives the staged bridge to its sealed lease, then the snapshot apply ladder to `Complete`.
pub(crate) fn drive_scene_bridge(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_scene_bridge(state, context)) {
            World3dSceneBridgeStep::Complete => break,
            World3dSceneBridgeStep::Pending => {}
            step => panic!("scene bridge stopped at {step:?} on turn {turn} (fault {:?})", state.session.snapshot_fault),
        }
        assert!(turn < 4_095, "scene bridge did not seal within its turn ceiling");
    }
    sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        assert_ne!(with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)), WorldDrawRebuildStep::Fault, "draw rebuild faulted on turn {turn}");
        match with_world_step_context(64, |context| step_world3d_snapshot(state, context)) {
            World3dSnapshotApplyStep::Complete | World3dSnapshotApplyStep::Idle => break,
            World3dSnapshotApplyStep::Pending => {}
            step => panic!("snapshot apply stopped at {step:?} on turn {turn} (fault {:?})", state.session.snapshot_fault),
        }
        assert!(turn < 4_095, "snapshot apply did not complete within its turn ceiling");
    }
    for turn in 0..4_096 {
        let draw_step = with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context));
        assert_ne!(draw_step, WorldDrawRebuildStep::Fault, "draw retirement faulted on turn {turn}");
        let bridge_step = with_world_step_context(64, |context| step_world3d_scene_bridge(state, context));
        assert_ne!(bridge_step, World3dSceneBridgeStep::Fault, "bridge retirement faulted on turn {turn}");
        if world3d_draw_rebuild_terminal_is_empty(&state.scene) && state.bridge.scene_bridge_retired.is_none() && state.bridge.scene_bridge_close.is_none() && state.bridge.scene_bridge.is_none() {
            break;
        }
        assert!(turn < 4_095, "scene bridge retirement did not complete within its turn ceiling");
    }
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
}

fn terrain_test_grant()->semio_framework_value::retained_clone::RetainedCloneGrant {semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64}}

pub(crate) fn terrain_test_retire_then_dynamic(state:&mut World3dState,context:&mut semio_framework_job::StepContext<'_>)->bool {
    let grant=terrain_test_grant();
    let terrain=step_world3d_terrain_retirement(&mut state.session, &mut state.terrain,grant,context);
    assert!(terrain.ownership.fits(grant));
    match terrain.step {
        WorldTerrainMeshPublicationStep::Idle=>step_world3d_dynamic_retirement(state,context),
        WorldTerrainMeshPublicationStep::Pending|WorldTerrainMeshPublicationStep::Complete=>false,
        WorldTerrainMeshPublicationStep::Fault(fault)=>panic!("typed terrain retirement refusal: {fault:?}"),
    }
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(read_static("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}
