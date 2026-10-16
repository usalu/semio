//! 🧪️ host tests (🔭️unit-2): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{tool_run_trace, TRANSLATE_SELECTION_ACTION_ID, World3dBuildContext, WORLD_INTERACTION_OBJECT_CAPACITY, world_mesh_ownership_grant, world_reference_width, world_scene_raster_pool, WorldCancelReason, WorldCursorWakeAuthority, WorldDrawRebuildDescriptor, WorldDrawRebuildStep, WorldDrawRegistry, WorldInteractionIntent, WorldInteractionObjectKind, WorldInteractionPhase, WorldInteractionStep, WorldReferenceRecord, WorldSceneRaster, WorldTerrainStyle};
use semio_framework_os_infinite_world_components::{GumballHandle, set_world3d_declared_actions, world_mesh_component_id, WorldFaceOverlayMeshCursor, WorldFaceOverlayMeshStep, WorldFaceOverlayScan, WorldFaceOverlayScanStep, WorldPlaceholderOwner};
use semio_framework_os_infinite_world_cursors::{live_world3d_mesh_version, resolved_domain_granularity_id, resolved_domain_id, WORLD_INTERACTION_DOMAIN_ID, WORLD_ITEM_GRANULARITY_ID, WorldComponentKind, WorldComponentMarqueePublishJob, WorldComponentPickCursor, WorldComponentPickPurpose, WorldGumballCommitJob, WorldGumballGesture, WorldGumballPhase, WorldGumballPickCursor, WorldInteractionAuthorityStep, WorldMarqueeGesture, WorldMarqueePickCursor, WorldMarqueeResult, WorldObjectPickCursor, WorldObjectPickPurpose, WorldRayPickCursor, WorldRayPickPurpose};
use semio_framework_os_infinite_world_scene::{apply_terrain_style_if_changed_state, apply_world3d_terrain_tile_bytes, environment_light_dir, GUMBALL_PLANE_MESH, offer_missing_mesh_fetches, parse_color, reset_world3d_brush_mesh_registry, retire_cancelled_world3d_asset_step, return_world3d_asset, scene_lod, step_world3d_brush_mesh_announce, step_world3d_terrain_retirement, sync_world3d_brush_mesh_feedback, take_next_world3d_asset, terrain_tile_mesh_key, terrain_tile_url, world3d_brush_mesh_announce_pending, WORLD_BRUSH_MESH_MAX_PAGES, WorldAssetRequestKind, WorldTerrainMeshPublicationStep};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_scene::{sort_world3d_translucent_material_draws, World3dShadowProfile};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_scene::TerrainMeshOracleData;
use semio_framework_os_infinite_world_bridge::{append_component_overlays, append_gumball_geometry, apply_world3d_initial_projection_seed, apply_world3d_projection_spec, begin_world3d_draw_rebuild, begin_world3d_dynamic_retirement, DecodedMeshPaintImage, publish_world3d_asset_mesh_lease, sync_world3d_projection_content_frame, sync_world3d_scene_selection, world3d_draw_rebuild_admit_draw, world3d_draw_rebuild_admit_instance, world3d_draw_rebuild_seal, world3d_draw_rebuild_terminal_is_empty, world3d_live_camera_json, World3dCameraFitStep, World3dSceneBridgeStep, World3dSnapshotApplyStep, world_reference_plane_size, world_reference_visual, WorldInteractionRegistryBuildCursor};
use semio_framework_os_infinite_world_interaction::{apply_decoded_mesh_paint_image, apply_world_action_preview, begin_world3d_interaction_close, close_world3d_interaction_step, enqueue_world3d_event, publish_world3d_plan_step, resolve_world_context_menu_target, step_world3d_draw_rebuild, step_world3d_terrain, store_mesh, sync_terrain_state, world3d_interaction_terminal_is_empty, world_merge_code};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_interaction::{marquee_select_action, pick_hover_action, pick_instance_at, pick_select_action, resolved_item_id};
use semio_framework_os_infinite_world_host::{apply_runtime_draw_flags, step_world3d_camera_fit, step_world3d_dynamic_retirement, step_world3d_interaction, step_world3d_scene_bridge, step_world3d_snapshot, sync_world3d_state, world3d_cursor_work_pending, world3d_dynamic_retirement_terminal_is_empty, world3d_tool_run_trace_cursors, World3dState};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_host::handle_world3d_pointer_button;
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_host::render_world_3d;
use ui_wgpu::wgpu::{ActionDescriptor, Camera3d, camera_grid_fade_distance, CameraProjection3d, Instance3d, lod_grid_step_world, Mat4, mesh3d_allocate_step, mesh3d_begin, mesh3d_seal, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, OrbitController, PointerModifiers, Rect, Rgba, SceneDraw3d, SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterProfile, SceneRasterWriteMode, SurfaceKind, Theme, UiComponentSceneNode, UiPresence, Vec3, World3dPresentation, World3dPresentationClear, World3dScene};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Mutex;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use ui_wgpu::wgpu::{SceneMaterialDraw3d, SceneMaterialKind3d, SceneShadowRole3d, SceneViewportMask3d, WORLD_SHADOW_MAP_SIZE};
#[cfg(test)]
use ui_wgpu::wgpu::SceneRasterMeshSeal;
use semio_framework_tool_run::{ToolRunTraceCursor, ToolRunTraceDelta};
use semio_framework_ui_viewport::{Viewport3dProjectionFramePolicy, Viewport3dProjectionMode, Viewport3dProjectionOrientation, Viewport3dProjectionSpec};
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

fn allocate_oracle_mesh(token:Mesh3dWriteToken) {
    loop {
        let grant=world_mesh_ownership_grant(0,ui_wgpu::wgpu::mesh3d_allocate_capacity_byte_demand(token).unwrap(),0,1);
        let step=mesh3d_allocate_step(token,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
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

/// 🎯️ The generation3d `box-fillet-preview` delivery extent, as `📚️examples/📐️box-fillet-preview/🧫️fixtures/🧩️example/🔣️.json` commits it.
const BOOT_FRAME_FIT_JSON: &str = r#"{"enabled":true,"revision":7,"padding":1.12,"boundsMin":[0.0,0.0,0.0],"boundsMax":[2.0,2.0,2.0]}"#;

fn draw_fixture_bytes(draws: &[(&str, &[&str])]) -> u32 {
    draws.iter().map(|(mesh, instances)| mesh.len() + size_of::<SceneDraw3d>() + instances.iter().map(|id| id.len() + size_of::<Instance3d>()).sum::<usize>()).sum::<usize>() as u32
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

//#endregion GlbAssetTests

//#region 🥽️BrushMeshAnnounceTests
/// 🧫️ React's measured `registerBrushMesh` rows and the arg shapes it sends, from the 2026-09-18
/// side-by-side run (`🗑️generated/w11a-parity-run-14/steps.json`).
#[allow(non_snake_case)]
fn BRUSH_MESH_REGISTRATION() -> &'static str {
    read_static("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🥽️brush-mesh-registration.json")
}

/// 🔒️ The announcement registry is process-wide by design (React's is a module singleton), so the
/// laws below take it one at a time whatever `--test-threads` says.
static BRUSH_MESH_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn brush_mesh_fixture() -> serde_json::Value {
    serde_json::from_str(BRUSH_MESH_REGISTRATION()).expect("brush mesh registration fixture parses")
}

/// 🧫️ A mesh whose payload spans SEVERAL pages, so the run's positions-then-indices split falls
/// inside a page the way a real GLB's does: 400 vertices (1 200 position values) plus 900 indices is
/// 2 100 values, three pages at [`WORLD_BRUSH_MESH_PAGE_VALUES`].
fn brush_mesh_oracle() -> TerrainMeshOracleData {
    let vertices = 400_u32;
    let positions: Vec<f32> = (0..vertices).flat_map(|vertex| [vertex as f32, (vertex % 7) as f32, (vertex % 13) as f32]).collect();
    let normals: Vec<f32> = (0..vertices).flat_map(|_| [0.0, 0.0, 1.0]).collect();
    let indices: Vec<u32> = (0..900_u32).map(|index| index % vertices).collect();
    mesh_oracle_from_buffers(positions, normals, indices)
}

/// 🧫️ A bridged world pane whose url-declared mesh is RESIDENT — the state React reaches when its
/// `useLoader(GLTFLoader, …)` resolves and `BrushMeshRegistrar` fires.
fn resident_brush_mesh_surface(surface_id: &str) -> World3dState {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new(surface_id.into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });
    let mesh = publish_oracle_mesh_at_revision(brush_mesh_oracle(), state.session.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, URL_MESH_URL, mesh).expect("the decoded GLB publishes under the wire's mesh id");
    state
}

fn brush_mesh_text(action: &ActionDescriptor, key: &str) -> Option<String> {
    action.args.as_ref().and_then(|args| args.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string)
}

fn brush_mesh_number(action: &ActionDescriptor, key: &str) -> Option<u64> {
    action.args.as_ref().and_then(|args| args.get(key)).and_then(semio_framework_value::DslValue::as_f64).map(|value| value as u64)
}

fn brush_mesh_carries(action: &ActionDescriptor, key: &str) -> bool {
    action.args.as_ref().and_then(|args| args.get(key)).is_some()
}

fn drain_brush_mesh_run(state: &mut World3dState) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    for _ in 0..(WORLD_BRUSH_MESH_MAX_PAGES as usize + 2) {
        let Some(action) = step_world3d_brush_mesh_announce(&mut state.session, &mut state.scene, &mut state.assets) else { break };
        actions.push(action);
    }
    actions
}

/// 🥽️ LAW: a resident mesh emits exactly ONE `registerBrushMesh` per window instance per revision,
/// with React's own arg shape — and never a second one in the same frame or the next.
///
/// 🩸️ The wgpu host dispatched nothing at all for family B of the parity diff, so the guest's brush
/// and volume-brush utilities had no collision body on this renderer while React announced on all
/// seven measured steps (`📓️w11a-prepared-world-mesh-missing.md` §6 family B).
#[test]
fn a_resident_mesh_announces_one_register_brush_mesh_per_window_with_reacts_args() {
    let _guard = BRUSH_MESH_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_world3d_brush_mesh_registry();
    let fixture = brush_mesh_fixture();
    let windows: Vec<String> = fixture["windows"].as_array().expect("window list").iter().map(|entry| entry.as_str().expect("window id").to_string()).collect();
    assert_eq!(windows.len(), 2, "React journals both window instances on every measured step");

    let mut first = resident_brush_mesh_surface(&windows[0]);
    let run = drain_brush_mesh_run(&mut first);
    assert!(!run.is_empty(), "the first window pages the geometry it just made resident");
    assert!(run.iter().all(|action| action.action == fixture["action"].as_str().expect("action id")), "every row is a registerBrushMesh");
    assert_eq!(brush_mesh_text(&run[0], "surfaceId").as_deref(), Some(windows[0].as_str()), "React sends the pane's own surface id");
    assert_eq!(brush_mesh_text(&run[0], "windowId").as_deref(), Some(windows[0].as_str()), "and scopes the announcement to the window instance that owns the mesh");
    assert_eq!(brush_mesh_text(&run[0], "url").as_deref(), Some(URL_MESH_URL));
    assert_eq!(brush_mesh_number(&run[0], "pageCount"), Some(run.len() as u64), "the run declares its own length on every page");
    let digest = brush_mesh_text(&run[0], "digest").expect("the identity is the digest the guest verifies the run against");
    assert_eq!(digest.len(), 64, "unkeyed BLAKE3, hex");
    assert!(step_world3d_brush_mesh_announce(&mut first.session, &mut first.scene, &mut first.assets).is_none(), "an announced identity is never re-announced at the same revision");
    assert!(!world3d_brush_mesh_announce_pending(&first.scene, &first.assets), "and the surface owes the guest nothing more");

    let mut second = resident_brush_mesh_surface(&windows[1]);
    let sibling = drain_brush_mesh_run(&mut second);
    assert_eq!(sibling.len(), 1, "a sibling window announces the identity the registry already holds by id alone");
    assert_eq!(brush_mesh_text(&sibling[0], "windowId").as_deref(), Some(windows[1].as_str()), "scoped to ITS window, which is why React journals a row per window");
    assert_eq!(brush_mesh_text(&sibling[0], "url").as_deref(), Some(URL_MESH_URL));
    assert_eq!(brush_mesh_text(&sibling[0], "digest"), Some(digest), "by the digest the first window put into the guest");
    assert!(!brush_mesh_carries(&sibling[0], "page"), "and with no payload at all");
    assert!(!brush_mesh_carries(&sibling[0], "positionsB64"));

    retire_bridged_surface(&mut first);
    retire_bridged_surface(&mut second);
    reset_world3d_brush_mesh_registry();
}

/// 🥽️ LAW: a page run carries the mesh's little-endian positions followed by its little-endian
/// indices, positions filling each page first — the exact layout the guest's
/// `decode_brush_mesh_page_values` reassembles and its `brush_mesh_digest` hashes.
#[test]
fn a_brush_mesh_run_pages_positions_then_indices_under_the_announced_digest() {
    let _guard = BRUSH_MESH_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_world3d_brush_mesh_registry();
    let oracle = brush_mesh_oracle();
    let mut expected: Vec<u8> = oracle.positions.iter().flat_map(|value| value.to_le_bytes()).collect();
    expected.extend(oracle.indices.iter().flat_map(|value| value.to_le_bytes()));

    let mut state = resident_brush_mesh_surface("puzzle3d-main-top");
    let run = drain_brush_mesh_run(&mut state);
    assert!(run.len() > 2, "the oracle spans several pages, so the positions-then-indices split falls inside one");
    assert!(run.iter().filter(|action| brush_mesh_carries(action, "positionsB64") && brush_mesh_carries(action, "indicesB64")).count() == 1, "exactly one page carries the seam");
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    for action in &run {
        assert_eq!(brush_mesh_text(action, "digest"), Some(semio_framework_hash::hash_bytes(&expected)), "every page names the identity the whole run hashes to");
        if let Some(encoded) = brush_mesh_text(action, "positionsB64") {
            positions.extend(base64_codec::base64_standard_decode(&encoded).expect("page payload is base64"));
        }
        if let Some(encoded) = brush_mesh_text(action, "indicesB64") {
            indices.extend(base64_codec::base64_standard_decode(&encoded).expect("page payload is base64"));
        }
    }
    positions.extend(indices);
    assert_eq!(positions, expected, "the run reassembles into the resident mesh's own geometry, positions first");

    retire_bridged_surface(&mut state);
    reset_world3d_brush_mesh_registry();
}

/// 🚚️ LAW: the guest's own two published facts drive every re-announcement — a residency that fell
/// is a restarted guest and voids every claim, and a standing `meshReuploadUrls` entry is an
/// identity whose bytes the guest is waiting for. React folds the identical pair into
/// `brushMeshRevisions` (`🌐️World3dHost/🟦️.tsx`).
#[test]
fn a_guest_restart_or_a_reupload_request_makes_every_surface_announce_again() {
    let _guard = BRUSH_MESH_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_world3d_brush_mesh_registry();
    let mut state = resident_brush_mesh_surface("puzzle3d-main-top");
    assert!(drain_brush_mesh_run(&mut state).iter().any(|action| brush_mesh_carries(action, "positionsB64")), "the first run pages the bytes");
    assert!(!world3d_brush_mesh_announce_pending(&state.scene, &state.assets));

    sync_world3d_brush_mesh_feedback(&mut state.assets, Some(r#"{"meshResidency":7,"meshReuploadUrls":[]}"#));
    assert!(!world3d_brush_mesh_announce_pending(&state.scene, &state.assets), "a climbing residency is the guest confirming what it holds, not news");

    sync_world3d_brush_mesh_feedback(&mut state.assets, Some(&format!(r#"{{"meshResidency":7,"meshReuploadUrls":["{URL_MESH_URL}"]}}"#)));
    assert!(world3d_brush_mesh_announce_pending(&state.scene, &state.assets), "a standing request is an identity the guest cannot serve");
    let reupload = drain_brush_mesh_run(&mut state);
    assert!(reupload.iter().any(|action| brush_mesh_carries(action, "positionsB64")), "a claimed request takes the PAGE path, never the id-only one");

    sync_world3d_brush_mesh_feedback(&mut state.assets, Some(r#"{"meshResidency":0,"meshReuploadUrls":[]}"#));
    assert!(world3d_brush_mesh_announce_pending(&state.scene, &state.assets), "a residency below the high-water mark is proof of a restarted guest");
    assert!(drain_brush_mesh_run(&mut state).iter().any(|action| brush_mesh_carries(action, "positionsB64")), "and the voided claim pages the bytes again");

    retire_bridged_surface(&mut state);
    reset_world3d_brush_mesh_registry();
}

/// 🪟️ LAW: every React-measured trigger is answered per WINDOW INSTANCE — a rebuilt surface starts
/// with an empty announced set and announces again, which is exactly what `window-reopen`,
/// `role-viewer` and `role-editor` journal on both panes.
#[test]
fn every_react_measured_trigger_announces_once_per_window_instance() {
    let _guard = BRUSH_MESH_TEST_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    reset_world3d_brush_mesh_registry();
    let fixture = brush_mesh_fixture();
    let steps = fixture["reactSteps"].as_array().expect("measured steps").clone();
    assert_eq!(steps.len(), 7, "the parity run measured seven family-B steps");

    for step in &steps {
        let per_window = step["perWindow"].as_object().expect("rows per window");
        for (window, rows) in per_window {
            assert!(rows.as_u64().is_some_and(|rows| rows > 0), "React journals at least one row per window on {}", step["step"]);
            let mut state = resident_brush_mesh_surface(window);
            let announced = drain_brush_mesh_run(&mut state);
            assert!(!announced.is_empty(), "the wgpu twin answers {} on {window}", step["step"]);
            for action in &announced {
                assert_eq!(brush_mesh_text(action, "windowId").as_deref(), Some(window.as_str()), "every row is scoped to the window that owns the mesh");
            }
            assert!(step_world3d_brush_mesh_announce(&mut state.session, &mut state.scene, &mut state.assets).is_none(), "and no duplicate follows in the same frame");
            retire_bridged_surface(&mut state);
        }
    }
    reset_world3d_brush_mesh_registry();
}
//#endregion 🥽️BrushMeshAnnounceTests

#[test]
fn world_environment_reaches_the_actual_scene_pass() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🌞️scene-lighting/🔣️.json")).unwrap();
    let shadow_fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🌑️scene-shadows/🔣️.json")).unwrap();
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().expect("world fixture");
    world.environment_json = Some(fixture["worldEnvironment"].to_string());
    world.camera_json = serde_json::json!({
        "position": fixture["iconRenderRequest"]["camera"]["position"],
        "target": fixture["iconRenderRequest"]["camera"]["target"],
        "up": [0, 0, 1],
        "fov": 45
    })
    .to_string();

    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let bounds = Rect::new(0.0, 0.0, 128.0, 96.0);
    drive_scene_bridge(&mut state, &scene, bounds);
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = ui_wgpu::wgpu::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = ui_wgpu::wgpu::widgets::WidgetContext {
        draw: &mut draw,
        overlay: None,
        atlas: &mut atlas,
        icons: None,
        input: &mut input,
        theme: &theme,
        scroll_offsets: &mut scroll,
        collapsed_sections: &mut collapsed,
        open_selects: &mut selects,
        interaction_maps: None,
        pick_clip: None,
        viewport_height: 96.0,
    };
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::World);

    let pass = draw.scene_passes.last().expect("World3d publishes its real pass");
    let environment = &fixture["worldEnvironment"];
    let ambient = parse_color(environment["ambient"]["color"].as_str().unwrap());
    let sun = parse_color(environment["sun"]["color"].as_str().unwrap());
    let emissive = parse_color(environment["material"]["emissive"].as_str().unwrap());
    assert_eq!(pass.camera_position, [4.0, -3.0, 8.0]);
    assert_eq!(pass.lighting.ambient_color, <[f32; 3]>::try_from(&ambient[..3]).unwrap());
    assert_eq!(pass.lighting.ambient_intensity, 0.42);
    assert_eq!(pass.lighting.sun_color, <[f32; 3]>::try_from(&sun[..3]).unwrap());
    assert_eq!(pass.lighting.sun_intensity, 1.7);
    assert!(pass.lighting.sun_enabled);
    assert_eq!(pass.neutral_material.metalness, 0.63);
    assert_eq!(pass.neutral_material.roughness, 0.27);
    assert_eq!(pass.neutral_material.emissive, <[f32; 3]>::try_from(&emissive[..3]).unwrap());
    assert_eq!(pass.neutral_material.emissive_intensity, 2.4);
    assert!(pass.shadow.enabled);
    assert_eq!(pass.shadow.map_size, WORLD_SHADOW_MAP_SIZE);
    assert!((pass.light_dir[0] + 0.4531539).abs() < 1e-5);
    assert!((pass.light_dir[1] - 0.7848856).abs() < 1e-5);
    assert!((pass.light_dir[2] - 0.42261827).abs() < 1e-5);
    let texture_matrix = ui_wgpu::wgpu::directional_shadow_texture_matrix(pass.light_dir);
    for (actual, expected) in texture_matrix.into_iter().zip(shadow_fixture["oracle"]["shadowMatrix"].as_array().unwrap()) {
        assert!((actual - expected.as_f64().unwrap() as f32).abs() < 2e-6, "shadow matrix drifted from Three: {actual} != {expected}");
    }
    retire_bridged_surface(&mut state);
}

#[test]
fn offscreen_light_visible_glb_is_retained_only_in_the_actual_shadow_channel() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🌑️scene-shadow-parity/🔣️.json")).unwrap();
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().expect("world fixture");
    world.camera_json = serde_json::json!({
        "position": fixture["frusta"]["mainCamera"]["position"],
        "target": fixture["frusta"]["mainCamera"]["target"],
        "up": fixture["frusta"]["mainCamera"]["up"],
        "fov": fixture["frusta"]["mainCamera"]["fov"],
    })
    .to_string();
    world.environment_json = Some(serde_json::json!({
        "sun": { "enabled": true, "azimuth": fixture["frusta"]["sun"]["azimuth"], "elevation": fixture["frusta"]["sun"]["elevation"] },
        "shadow": { "enabled": true, "opacity": fixture["profiles"]["world"]["opacityInput"], "softness": fixture["profiles"]["world"]["softnessInput"] },
    })
    .to_string());

    let mut state = World3dState::new("surface-shadow".into(), "controller-shadow".into());
    let bounds = Rect::new(0.0, 0.0, 128.0, 128.0);
    drive_scene_bridge(&mut state, &scene, bounds);
    state.scene.meshes.insert("shadow-mesh".into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("mesh lease admitted");
    state.scene.mesh_versions.insert("shadow-mesh".into(), 7).expect("mesh version admitted");
    state.scene
        .draws
        .push(SceneDraw3d {
            mesh_key: "shadow-mesh".into(),
            mesh_version: 7,
            instances: vec![
                Instance3d { component_source: None, id: "offscreen-caster".into(), model: Instance3d::model_from_trs([-7.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [0.25, 0.25, 0.25]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() },
                Instance3d { component_source: None, id: "visible-receiver".into(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [0.25, 0.25, 0.25]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() },
            ],
            shadow_role: Default::default(),
        })
        .expect("source draw admitted");

    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = ui_wgpu::wgpu::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = ui_wgpu::wgpu::widgets::WidgetContext {
        draw: &mut draw,
        overlay: None,
        atlas: &mut atlas,
        icons: None,
        input: &mut input,
        theme: &theme,
        scroll_offsets: &mut scroll,
        collapsed_sections: &mut collapsed,
        open_selects: &mut selects,
        interaction_maps: None,
        pick_clip: None,
        viewport_height: 128.0,
    };
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::World);

    let pass = draw.scene_passes.last().expect("actual scene pass");
    let color_ids: Vec<&str> = pass.draws.iter().chain(&pass.translucent_draws).flat_map(|draw| draw.instances.iter().map(|instance| instance.id.as_str())).collect();
    let shadow_ids: Vec<&str> = pass.shadow_draws.iter().flat_map(|draw| draw.instances.iter().map(|instance| instance.id.as_str())).collect();
    assert!(!color_ids.contains(&"offscreen-caster") && color_ids.contains(&"visible-receiver"), "the main camera owns only its visible GLB instances");
    assert!(shadow_ids.contains(&"offscreen-caster") && shadow_ids.contains(&"visible-receiver"), "the light frustum retains the offscreen caster and visible receiver");
    assert!(pass.shadow_draws.iter().all(|draw| draw.shadow_role == SceneShadowRole3d { casts: true, receives: true }));
    assert!(pass.draws.iter().all(|draw| draw.shadow_role == SceneShadowRole3d { casts: true, receives: true }));
    assert_eq!(pass.shadow.map_size, fixture["profiles"]["world"]["mapSize"].as_u64().unwrap() as u32);
    assert!(gpu.has_mesh_request("shadow-mesh", 7), "an offscreen-only caster participates in this frame's mesh upload ownership");
    assert!(state.scene.mesh_pool.contains(&"shadow-mesh".to_string()), "the offscreen-only caster participates in retained mesh ownership");
    retire_bridged_surface(&mut state);
}

#[test]
fn retained_draw_rebuild_preserves_prepared_material_colors_from_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🔣️draws.json")).unwrap();
    for vector in fixture["draws"].as_array().unwrap() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        publish_retained_draw_fixture(&mut state, vector);
        let expected = vector["instances"].as_array().unwrap();
        assert_eq!(state.scene.draws[0].mesh_key, vector["mesh"].as_str().unwrap());
        assert_eq!(state.scene.draws[0].mesh_version, vector["version"].as_u64().unwrap());
        assert_eq!(state.scene.draws[0].instances.len(), expected.len());
        for (actual, expected) in state.scene.draws[0].instances.iter().zip(expected) {
            let color: [f32; 4] = serde_json::from_value(expected["color"].clone()).unwrap();
            assert_eq!(actual.id, expected["id"].as_str().unwrap());
            assert_eq!(actual.color, color);
        }
        assert!(begin_world3d_dynamic_retirement(&mut state.session, &mut state.camera, &mut state.retirement, &mut state.scene, &mut state.assets));
        while !with_world_step_context(1, |context| terrain_test_retire_then_dynamic(&mut state,context)) {}
        assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
    }
}

/// 📡️ The queue is not a to-do list nobody reads: an uncached visible tile is ADMITTED into the
/// bounded world-asset pipeline, so the renderer host's generic fetch pump drains it exactly as it
/// drains GLB meshes and reference images. Before this wiring the queue had no consumer outside a
/// unit test and terrain never loaded in production at all.
///
/// 🕸️ The tile mesh is built by a BOUNDED cursor — ONE scalar write per turn — so the draw appears
/// many turns after the bytes land, not on the very next frame. The turns are driven directly
/// (`step_world_terrain_mesh`) rather than through whole `sync_terrain_state` frames, which would
/// re-serialize every visible tile's mesh JSON per step.
#[test]
fn sync_terrain_state_queues_fetch_for_uncached_tile_and_builds_after_upload() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.terrain.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 9.7382, project_origin_lat: 52.3759, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    apply_terrain_style_if_changed_state(&mut state.session, &mut state.scene, &mut state.terrain);
    let camera = Camera3d { position: Vec3::new(0.0, 0.0, 300.0), target: Vec3::ZERO, up: Vec3::new(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0, projection: ui_wgpu::wgpu::CameraProjection3d::Perspective, zoom: 1.0 };
    let (tile_draws, evicted) = sync_terrain_state(&mut state.session, &mut state.authority, &mut state.scene, &mut state.terrain, &mut state.assets, &camera);
    assert!(tile_draws.is_empty(), "no elevation data uploaded yet, nothing to draw");
    assert!(evicted.is_empty(), "nothing was cached yet, nothing to evict");
    assert!(!state.terrain.pending_terrain_tile_urls.is_empty(), "an uncached visible tile should be queued for byte-fetch");

    let owner = take_next_world3d_asset(&mut state.assets).expect("a visible uncached DEM tile is admitted as a fetch request");
    let (z, x, y) = match owner.kind() {
        WorldAssetRequestKind::Terrain { z, x, y } => (z, x, y),
        other => panic!("a DEM tile is admitted as a Terrain request, got {other:?}"),
    };
    assert_eq!(owner.url(), terrain_tile_url("/dem/{z}/{x}/{y}.png", z, x, y));
    return_world3d_asset(&mut state.assets, owner).expect("the admitted owner returns to its claim");
    let value = (100.0_f64 + 32768.0).round() as i64;
    let r = ((value >> 8) & 0xff) as u8;
    let g = (value - ((r as i64) << 8)).clamp(0, 255) as u8;
    let mut image = image::RgbaImage::new(256, 256);
    for pixel in image.pixels_mut() {
        *pixel = image::Rgba([r, g, 0, 255]);
    }
    let mut bytes = Vec::new();
    image::DynamicImage::ImageRgba8(image).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
    assert!(apply_world3d_terrain_tile_bytes(&mut state.terrain, z, x, y, &bytes), "fetched DEM bytes reach the terrain session");
    assert!(!state.terrain.pending_terrain_tile_urls.contains_key(&terrain_tile_url("/dem/{z}/{x}/{y}.png", z, x, y)), "an applied tile leaves the pending set");

    sync_terrain_state(&mut state.session, &mut state.authority, &mut state.scene, &mut state.terrain, &mut state.assets, &camera);
    assert!(state.terrain.terrain_pending.is_some() || state.terrain.terrain_build.is_some(), "an uploaded tile opens its mesh-build owner");
    let mut turns = 0;
    while state.terrain.terrain_pending.is_some() || state.terrain.terrain_build.is_some() {
        with_world_step_context(1,|context|{let result=step_world3d_terrain(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, &mut state.terrain,terrain_test_grant(),context);assert!(result.ownership.fits(terrain_test_grant()));assert!(!matches!(result.step,WorldTerrainMeshPublicationStep::Fault(_)));});
        turns += 1;
        assert!(turns < 1_000_000, "an uploaded tile converges to its vertex-coloured mesh");
    }
    let (tile_draws_after_upload, _) = sync_terrain_state(&mut state.session, &mut state.authority, &mut state.scene, &mut state.terrain, &mut state.assets, &camera);
    assert_eq!(tile_draws_after_upload.len(), 1, "one draw per tile — the ramp rides the mesh's own vertex colours, not ten flat bands");
}

#[test]
fn apply_terrain_style_if_changed_state_purges_stale_meshes_on_origin_change() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.terrain.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 0.0, project_origin_lat: 0.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    assert!(apply_terrain_style_if_changed_state(&mut state.session, &mut state.scene, &mut state.terrain).is_empty(), "first application has nothing to purge");
    let mesh_key = terrain_tile_mesh_key(&state.session.surface_id, 10, 1, 2);
    store_mesh(&mut state.session, &mut state.interaction, &mut state.authority, &mut state.scene, mesh_key, publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])));
    state.terrain.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 5.0, project_origin_lat: 5.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    let purged = apply_terrain_style_if_changed_state(&mut state.session, &mut state.scene, &mut state.terrain);
    assert_eq!(purged.len(), 1, "an origin change should purge the previously-cached terrain mesh");
    assert!(state.scene.meshes.is_empty());
}
//#endregion TerrainTests

//#region ContextMenuTests
#[test]
fn resolve_world_context_menu_target_prioritizes_vortex_over_object_over_reference() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selection.local_hover_id = Some("reference:site-plan.png".into());
    assert_eq!(resolve_world_context_menu_target(&state.selection), Some(("reference", "site-plan.png".to_string())));

    state.selection.hovered_component_mode = Some("face".into());
    state.selection.hovered_component_object_id = Some("obj-1".into());
    assert_eq!(resolve_world_context_menu_target(&state.selection), Some(("object", "obj-1".to_string())));

    state.selection.hovered_vortex_id = Some("vortex-1".into());
    assert_eq!(resolve_world_context_menu_target(&state.selection), Some(("vortex", "vortex-1".to_string())));
}

#[test]
fn resolve_world_context_menu_target_is_none_without_any_hover() {
    let state = World3dState::new("surface-1".into(), "controller-1".into());
    assert_eq!(resolve_world_context_menu_target(&state.selection), None);
}

/// 🖱️📋️ **The dead-verb law.** A right click on a world surface publishes NO action of its own. The
/// menu is the SHELL's (`open_context_menu`, whose `resolve_context_menu_surface` fills a World3d
/// surface's hits/selection from `world3d_context_menu_surface`), and React deleted its own
/// `contextMenuAt` twin when the target moved onto the menu REQUEST — dispatching it anyway only
/// produced an `undeclaredActionDiagnostic` drop (`world3dContextMenuSurfaceV1`'s docstring). What a
/// release DOES owe is the unconditional camera report three's `OrbitControls` fires on every
/// pointer-up (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY).
#[test]
fn right_click_publishes_no_menu_verb_and_still_reports_the_camera() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    state.selection.hovered_vortex_id = Some("vortex-1".into());
    handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
    let action = handle_world3d_pointer_button(&mut state, 200.0, 200.0, false, 2, &PointerModifiers::default()).expect("a right release reports the camera");
    assert_eq!(action.action, "setCamera");
    assert!(action.args.expect("args").get("camera").is_some(), "the report carries the pose, never a menu target");
}

#[test]
fn right_drag_reports_the_camera_like_every_other_release() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    state.selection.hovered_vortex_id = Some("vortex-1".into());
    handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
    let action = handle_world3d_pointer_button(&mut state, 260.0, 260.0, false, 2, &PointerModifiers::default()).expect("right release should still sync camera");
    assert_eq!(action.action, "setCamera", "a right-drag reports the camera, and no renderer opens a menu from a drag");
}
//#endregion ContextMenuTests

/// 🕹️ default `granularity` ("object") is neither component-mode nor "mesh" — this is the
/// plain `world`-domain item pick path (see `pick_select_action`'s final fallback branch).
#[test]
fn pick_select_emits_batched_interaction_select_for_plain_object_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::new(0.25, 0.25, 0.0), inner.w, inner.h).expect("object projects");
    let action = pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, screen[0], screen[1], inner, true, false).expect("pick action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!(WORLD_INTERACTION_DOMAIN_ID));
    assert_eq!(args["method"], json!("pick"));
    assert_eq!(args["merge"], json!("additive"), "shift modifier maps to the canonical MergeMode label");
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0]["granularity"], json!(WORLD_ITEM_GRANULARITY_ID));
    assert_eq!(targets[0]["id"], json!("surface-1/obj-1"), "item target id is surfaceId/objectId (PathDelimited)");
}

#[test]
fn marquee_select_emits_batched_interaction_select_with_rectangle_method() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.selection.marquee_points = vec![[0.0, 0.0], [400.0, 400.0]];
    let action = marquee_select_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, false, true).expect("marquee action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    assert_eq!(args["method"], json!("rectangle"));
    assert_eq!(args["merge"], json!("subtractive"), "ctrl modifier maps to the canonical MergeMode label");
    assert!(state.selection.marquee_points.is_empty(), "marquee is consumed after gathering targets");
}

/// 🖱️ Moving off the instance clears — empty `targets` is `next_hover`'s clear signal.
#[test]
fn pick_hover_emits_interaction_hover_and_clears_when_nothing_hit() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.scene.meshes.insert("mesh-1".into(), mesh);
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::new(0.25, 0.25, 0.0), inner.w, inner.h).expect("object projects");
    let action = pick_hover_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, screen[0], screen[1], inner).expect("hover action");
    assert_eq!(action.action, "interactionHover");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!(WORLD_INTERACTION_DOMAIN_ID));
    assert_eq!(args["channel"], json!("pointer"));
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets[0]["id"], json!("surface-1/obj-1"));

    let action = pick_hover_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, 5.0, 5.0, inner).expect("clear action");
    assert_eq!(action.action, "interactionHover");
    let args = action.args.expect("args");
    assert_eq!(args["targets"].as_str(), Some("[]"));
}

#[test]
fn apply_world_action_preview_applies_interaction_select_and_hover_for_world_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "targets": serde_json::json!([{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-1" }]).to_string(),
                "merge": "replace",
                "method": "pick",
            })),
        },
    );
    assert_eq!(state.selection.selected_ids, vec!["obj-1".to_string()], "surfaceId/ prefix is stripped for this surface");

    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionHover".into(),
            args: Some(semio_framework::dsl_value!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "channel": "pointer",
                "targets": serde_json::json!([{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-2" }]).to_string(),
            })),
        },
    );
    assert_eq!(state.selection.local_hover_id.as_deref(), Some("obj-2"));

    apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &ActionDescriptor { controller_id: "controller-1".into(), action: "interactionHover".into(), args: Some(semio_framework::dsl_value!({ "domainId": WORLD_INTERACTION_DOMAIN_ID, "channel": "pointer", "targets": serde_json::json!([]).to_string() })) });
    assert!(state.selection.local_hover_id.is_none(), "empty targets clears hover");
}

#[test]
fn pick_select_emits_bare_id_into_bound_app_domain_when_window_binds_one() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bound_domain_id = Some("cad".into());
    state.session.bound_domain_granularity_id = Some("object".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::new(0.25, 0.25, 0.0), inner.w, inner.h).expect("object projects");
    let action = pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, screen[0], screen[1], inner, false, false).expect("pick action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!("cad"), "targets the window's bound app domain, not the OS `world` fallback");
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets[0]["granularity"], json!("object"), "uses the bound domain's own granularity, not `item`");
    assert_eq!(targets[0]["id"], json!("obj-1"), "bare id — a bound domain is single-surface-scoped, no surfaceId/ prefix");
}

#[test]
fn pick_hover_emits_bare_id_into_bound_app_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bound_domain_id = Some("cad".into());
    state.session.bound_domain_granularity_id = Some("object".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.session.bounds = inner;
    state.session.pick_bounds = inner;
    let camera = state.camera.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, inner.w, inner.h), Vec3::new(0.25, 0.25, 0.0), inner.w, inner.h).expect("object projects");
    let action = pick_hover_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, screen[0], screen[1], inner).expect("hover action");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!("cad"));
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets[0]["granularity"], json!("object"));
    assert_eq!(targets[0]["id"], json!("obj-1"));
}

#[test]
fn marquee_select_emits_bare_ids_into_bound_app_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bound_domain_id = Some("features".into());
    state.session.bound_domain_granularity_id = Some("pin".into());
    state.scene.meshes.insert("mesh-1".into(), topology_mesh());
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.selection.marquee_points = vec![[0.0, 0.0], [400.0, 400.0]];
    let action = marquee_select_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, false, false).expect("marquee action");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!("features"));
    let targets_value: serde_json::Value = serde_json::from_str(args["targets"].as_str().expect("targets text")).expect("targets JSON");
    let targets = targets_value.as_array().expect("targets array");
    assert_eq!(targets[0]["granularity"], json!("pin"));
    assert_eq!(targets[0]["id"], json!("obj-1"));
}

/// 🚫️ An action for the OS `world` fallback domain must NOT apply once this window is bound to
/// its own app domain — otherwise the same click could ever light up two selection universes.
#[test]
fn apply_world_action_preview_respects_bound_app_domain_and_ignores_other_domains() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.bound_domain_id = Some("cad".into());
    state.session.bound_domain_granularity_id = Some("object".into());
    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({ "domainId": WORLD_INTERACTION_DOMAIN_ID, "targets": serde_json::json!([{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-1" }]).to_string(), "merge": "replace", "method": "pick" })),
        },
    );
    assert!(state.selection.selected_ids.is_empty(), "an unbound-domain action must not apply once this window binds its own domain");

    apply_world_action_preview(
        &mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,
        &ActionDescriptor { controller_id: "controller-1".into(), action: "interactionSelect".into(), args: Some(semio_framework::dsl_value!({ "domainId": "cad", "targets": serde_json::json!([{ "granularity": "object", "id": "obj-1" }]).to_string(), "merge": "replace", "method": "pick" })) },
    );
    assert_eq!(state.selection.selected_ids, vec!["obj-1".to_string()], "bare id applies as-is — no surfaceId/ stripping for a bound domain");
}

#[test]
fn sync_world3d_state_captures_scene_bound_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let scene = scene_with_selection_and_domain("{}", Some(("cad", "object")));
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.session.bound_domain_id.as_deref(), Some("cad"));
    assert_eq!(state.session.bound_domain_granularity_id.as_deref(), Some("object"));
    assert_eq!(resolved_domain_id(&state.session), "cad");
    assert_eq!(resolved_domain_granularity_id(&state.session), "object");
}
//#endregion 🔖️WorldInteractionVerbs

//#region 🌉️World3dSceneBridge
/// 🌉️ The committed generation3d preview payload this lane drives the bridge with. Produced by the
/// real pipeline (`parse_dsl` → `FlowHost::evaluate` → `tessellate_geometry(0.05)` →
/// `preview_payload_from_eval`) in `s.procedural.generation3d`'s own example-geometry harness, whose
/// `parry3d` oracle independently confirms the same solid's volume and extent — see the fixture's
/// `provenance` block, and the twin assertion in that harness that keeps this file from rotting.
#[allow(non_snake_case)]
fn SCENE_BRIDGE_FIXTURE() -> &'static str {
    read_static("../../../🧪️tests/🧫️fixtures/🌉️scene-bridge/🔣️.json")
}

pub(crate) fn scene_bridge_fixture() -> serde_json::Value {
    serde_json::from_str(SCENE_BRIDGE_FIXTURE()).expect("scene bridge fixture parses")
}

/// 🎬️ Builds the exact `World3dScene` the generation3d preview window publishes for the fixture.
pub(crate) fn scene_from_bridge_fixture(fixture: &serde_json::Value) -> UiComponentSceneNode {
    let mut scene = scene_with_selection_and_domain("{}", Some((fixture["domainId"].as_str().expect("domain"), fixture["domainGranularityId"].as_str().expect("granularity"))));
    let world = scene.world_3d.as_mut().expect("world scene");
    world.meshes_json = fixture["meshesJson"].to_string();
    world.instances_json = fixture["instancesJson"].to_string();
    world.camera_json = fixture["cameraJson"].to_string();
    world.selection_json = fixture["selectionJson"].to_string();
    world.environment_json = Some(fixture["environmentJson"].to_string());
    scene
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

#[allow(non_snake_case)]
fn CAMERA_FRAMING_FIXTURE() -> &'static str {
    read_static("../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🎥️world3d-camera-framing/🔣️.json")
}

fn camera_framing_fixture() -> serde_json::Value {
    serde_json::from_str(CAMERA_FRAMING_FIXTURE()).expect("camera framing fixture parses")
}

fn camera_framing_scene(fixture: &serde_json::Value, camera: &str, geometry: bool, revision: Option<u64>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().expect("world scene");
    world.meshes_json = if geometry { serde_json::Value::Array(vec![fixture["mesh"].clone()]).to_string() } else { "[]".into() };
    world.instances_json = if geometry { fixture["instances"].to_string() } else { "[]".into() };
    let mut delivered=fixture["cameras"][camera].clone();delivered["projectionFrame"]=fixture["framePolicies"]["delivered"].clone();
    world.camera_json = delivered.to_string();
    let mut fit = fixture["fit"].clone();
    if let Some(revision) = revision {
        fit["revision"] = serde_json::Value::from(revision);
    }
    world.fit_json = Some(fit.to_string());
    scene
}

fn drive_camera_fit(state: &mut World3dState) -> usize {
    let mut pending = 0;
    for turn in 0..128 {
        match step_world3d_camera_fit(state) {
            World3dCameraFitStep::Applied => return pending,
            World3dCameraFitStep::Pending | World3dCameraFitStep::Stale => pending += 1,
            World3dCameraFitStep::Idle => panic!("camera fit became idle before applying on turn {turn}"),
        }
    }
    panic!("camera fit did not finish inside its fixed fixture ceiling")
}

fn assert_camera_fit(state: &World3dState, expected: &serde_json::Value) {
    let camera = state.camera.orbit.to_camera();
    let actual_position = [camera.position.x, camera.position.y, camera.position.z];
    let actual_target = [camera.target.x, camera.target.y, camera.target.z];
    for axis in 0..3 {
        assert!((actual_position[axis] - expected["position"][axis].as_f64().expect("position") as f32).abs() < 1e-4, "position axis {axis}: {actual_position:?}");
        assert!((actual_target[axis] - expected["target"][axis].as_f64().expect("target") as f32).abs() < 1e-5, "target axis {axis}: {actual_target:?}");
    }
    assert!((state.camera.orbit.distance - expected["distance"].as_f64().expect("distance") as f32).abs() < 1e-4);
    assert!((state.camera.orbit.zoom - expected["zoom"].as_f64().expect("zoom") as f32).abs() < 1e-5,"original camera zoom {} expected {}",state.camera.orbit.zoom,expected["zoom"]);
}

/// 🎥️ The neutral fixture enters through the actual JSON scene bridge, publishes a real mesh lease,
/// and advances one transformed AABB corner per retained step before matching Three's Box3 oracle.
#[test]
fn live_scene_mesh_bounds_frame_both_camera_families_like_current_react() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    for (camera, expected) in [("perspective", "perspective"), ("orthographicTop", "orthographicTop")] {
        let scene = camera_framing_scene(&fixture, camera, true, None);
        let mut state = World3dState::new(format!("surface-{camera}"), "controller".into());
        drive_scene_bridge(&mut state, &scene, viewport);
        let pending = drive_camera_fit(&mut state);
        assert!(pending >= fixture["instances"].as_array().unwrap().len() * 8, "each transformed AABB corner consumes its own retained step");
        assert_camera_fit(&state, &fixture["expect"][expected]);
    }
}

/// ⏰️ Two resident panes that first measured an empty generation sleep without polling, then
/// both re-arm when their actual draw generations seal and keep the renderer awake until every
/// transformed corner has been consumed.
#[test]
fn resident_generation_wakes_every_visible_camera_fit_until_both_complete() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let mut panes = [
        (World3dState::new("surface-top".into(), "controller".into()), "orthographicTop", "orthographicTopAfterRawContent", "delayedContent"),
        (World3dState::new("surface-perspective".into(), "controller".into()), "perspective", "perspective", "delivered"),
        (World3dState::new("surface-perspective-content".into(), "controller".into()), "perspective", "perspectiveThreePoint", "delayedContent"),
    ];
    for (state, camera, _, policy) in &mut panes {
        let mut scene=camera_framing_scene(&fixture,camera,false,None);let world=scene.world_3d.as_mut().unwrap();let mut delivered:serde_json::Value=serde_json::from_str(&world.camera_json).unwrap();delivered["projectionFrame"]=fixture["framePolicies"][*policy].clone();world.camera_json=delivered.to_string();
        drive_scene_bridge(state,&scene,viewport);
        assert_eq!(step_world3d_camera_fit(state), World3dCameraFitStep::Idle);
        assert!(!world3d_cursor_work_pending(state), "an exactly witnessed empty generation sleeps");
    }
    for (state, camera, _, policy) in &mut panes {
        let mut scene=camera_framing_scene(&fixture,camera,true,None);let world=scene.world_3d.as_mut().unwrap();let mut delivered:serde_json::Value=serde_json::from_str(&world.camera_json).unwrap();delivered["projectionFrame"]=fixture["framePolicies"][*policy].clone();world.camera_json=delivered.to_string();
        drive_scene_bridge(state,&scene,viewport);
        assert!(world3d_cursor_work_pending(state), "a sealed draw generation re-arms its own fit lane");
    }

    let ceiling = fixture["instances"].as_array().unwrap().len() * 8 + 2;
    let mut applied = [false; 3];
    for turn in 0..ceiling {
        for (index, (state, _, _, _)) in panes.iter_mut().enumerate() {
            if !applied[index] {
                applied[index] = step_world3d_camera_fit(state) == World3dCameraFitStep::Applied;
            }
        }
        if applied.into_iter().all(|complete| complete) {
            break;
        }
        assert!(panes.iter().any(|(state, _, _, _)| world3d_cursor_work_pending(state)), "turn {turn}: unfinished retained camera work owns the next frame");
    }
    assert!(applied.into_iter().all(|complete| complete), "both visible panes finish inside one exact corner walk");
    for (state, _, expected, _) in &panes {
        assert_camera_fit(state, &fixture["expect"][expected]);
        assert!(!world3d_cursor_work_pending(state), "an applied fit releases its wake");
    }
    for (state,_,_,_) in &mut panes {retire_bridged_surface(state);}
}

/// 📐️ A dock template lands after the producer camera lease and before auto-fit. Its look
/// direction owns the fitted eye, while the measured geometry owns target and distance.
#[test]
fn initial_projection_template_seeds_the_actual_delivered_camera_before_fit() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let mut state = World3dState::new("surface-template".into(), "controller".into());
    let spec = ui_wgpu::wgpu::default_projection_spec();
    assert!(!apply_world3d_initial_projection_seed(&mut state.session, &mut state.camera, &mut state.scene, &mut state.bridge, spec), "a template never races ahead of the delivered camera lease");

    drive_scene_bridge(&mut state, &camera_framing_scene(&fixture, "perspective", true, None), viewport);
    assert!(apply_world3d_initial_projection_seed(&mut state.session, &mut state.camera, &mut state.scene, &mut state.bridge, spec));
    assert!(!apply_world3d_initial_projection_seed(&mut state.session, &mut state.camera, &mut state.scene, &mut state.bridge, spec), "the same pane consumes its initial template once");
    drive_camera_fit(&mut state);
    assert_camera_fit(&state, &fixture["expect"]["perspectiveThreePoint"]);
}

/// 📦️ The live Shell sees the applied snapshot lease one transaction before its sealed draw
/// rebuild publishes. Its initial projection seed is camera-only: it must preserve that rebuild,
/// because the url-backed draw is also the sole ledger that admits the document's GLB request.
#[test]
fn initial_projection_seed_preserves_the_delivered_url_draw_asset_request_and_loaded_fit() {
    let mut scene = url_mesh_wire_scene();
    let world = scene.world_3d.as_mut().expect("world scene");
    world.instances_json = r#"[{"id":"obj-left","meshId":"mesh:🧊️left","position":[2,0,0],"rotation":[0,0,0,1],"scale":[1,1,1]}]"#.into();
    world.fit_json = Some(r#"{"enabled":true,"revision":7,"padding":1.12}"#.into());
    let bounds = Rect::new(0.0, 0.0, 800.0, 600.0);
    let mut state = World3dState::new("surface-url-template".into(), "controller".into());

    sync_world3d_state(&mut state, &scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_scene_bridge(&mut state, context)) {
            World3dSceneBridgeStep::Complete => break,
            World3dSceneBridgeStep::Pending => {}
            step => panic!("url bridge stopped at {step:?} on turn {turn}"),
        }
        assert!(turn < 4_095, "url bridge did not seal inside its fixture ceiling");
    }
    assert_eq!(state.scene.instance_positions.get("obj-left"), Some(&[2.0, 0.0, 0.0]), "the bridge retains React's raw pre-asset position for projection framing");
    sync_world3d_state(&mut state, &scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_snapshot(&mut state, context)) {
            World3dSnapshotApplyStep::Complete => break,
            World3dSnapshotApplyStep::Pending => {}
            step => panic!("url snapshot stopped at {step:?} on turn {turn}"),
        }
        assert!(turn < 4_095, "url snapshot did not apply inside its fixture ceiling");
    }
    assert!(state.scene.draw_rebuild.is_some(), "the applied delivery retains its sealed rebuild for the next transaction");
    let top_spec = semio_framework_ui_viewport::Viewport3dProjectionSpec { mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {}, orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top } };
    assert!(apply_world3d_initial_projection_seed(&mut state.session, &mut state.camera, &mut state.scene, &mut state.bridge, top_spec));
    sync_world3d_state(&mut state, &scene, bounds);
    assert!(!state.camera.projection_frame_owed, "the exact Top document sync consumes its content-frame request");
    assert!((state.camera.orbit.target.x - 2.0).abs() < 1e-5, "the pre-asset Top frame targets the raw instance position");
    assert!(state.scene.draw_rebuild.is_some(), "the following document sync keeps the camera-only revision on the same delivery");
    for turn in 0..64 {
        match with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)) {
            WorldDrawRebuildStep::Complete => break,
            WorldDrawRebuildStep::Pending => {}
            step => panic!("camera-only seed invalidated the delivered rebuild at {step:?} on turn {turn}"),
        }
        assert!(turn < 63, "url draw rebuild did not publish inside its exact fixture ceiling");
    }
    let draw = state.scene.draws.iter().find(|draw| draw.mesh_key == URL_MESH_ID).expect("url draw survives the template seed");
    assert_eq!(draw.instances.iter().map(|instance| instance.id.as_str()).collect::<Vec<_>>(), ["obj-left"]);

    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    offer_missing_mesh_fetches(&mut state.session, &mut state.scene, &mut state.assets);
    let mut owner = take_next_world3d_asset(&mut state.assets).expect("the surviving draw admits one GLB request");
    assert_eq!((owner.kind(), owner.url()), (WorldAssetRequestKind::Glb, URL_MESH_URL));
    assert!(take_next_world3d_asset(&mut state.assets).is_none(), "the delivered url is admitted exactly once");
    let mesh = publish_oracle_mesh_at_revision(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]), state.session.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state.session, &mut state.interaction, &mut state.scene, &mut state.assets, URL_MESH_URL, mesh).expect("the requested GLB publishes into the delivered draw");
    drive_camera_fit(&mut state);
    assert!(state.camera.fit_applied_key.is_some(), "the loaded mesh generation completes the retained auto-fit");
    assert!(state.camera.orbit.target.x > 2.0 && state.camera.orbit.target.y > 0.0, "the fit measures the translated loaded mesh rather than a placeholder origin");

    owner.begin_close();
    return_world3d_asset(&mut state.assets, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state.assets) {}
    retire_bridged_surface(&mut state);
}

/// 🎥️ Empty geometry leaves the key unconsumed; the same revision applies when its real mesh arrives.
#[test]
fn delayed_geometry_retires_stale_measurement_without_fitting_a_placeholder_origin() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let empty = camera_framing_scene(&fixture, "perspective", false, None);
    let mut state = World3dState::new("surface-delayed".into(), "controller".into());
    drive_scene_bridge(&mut state, &empty, viewport);
    let seed = state.camera.orbit.clone();
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.camera.fit_applied_key, None, "an empty group does not consume React's fit key");
    assert_eq!(state.camera.orbit.target, seed.target, "no placeholder origin is framed");

    let loaded = camera_framing_scene(&fixture, "perspective", true, None);
    drive_scene_bridge(&mut state, &loaded, viewport);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Pending);
    state.scene.geometry_generation = state.scene.geometry_generation.wrapping_add(1);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Stale, "a residency change retires the partial measurement");
    drive_camera_fit(&mut state);
    assert_camera_fit(&state, &fixture["expect"]["perspective"]);
}

/// 🧭️ Genuine navigation consumes an owed content frame before late geometry arrives.
#[test]
fn world_camera_content_frame_preserves_original_user_orbit() {
    let fixture=camera_framing_fixture();let viewport=Rect::new(0.0,0.0,fixture["viewport"][0].as_f64().unwrap() as f32,fixture["viewport"][1].as_f64().unwrap() as f32);
    let mut scene=camera_framing_scene(&fixture,"perspective",false,None);let world=scene.world_3d.as_mut().unwrap();let mut delivered:serde_json::Value=serde_json::from_str(&world.camera_json).unwrap();delivered["projectionFrame"]=fixture["framePolicies"]["delayedContent"].clone();world.camera_json=delivered.to_string();
    let mut state=World3dState::new("surface-user-frame".into(),"controller".into());drive_scene_bridge(&mut state,&scene,viewport);
    assert_eq!(step_world3d_camera_fit(&mut state),World3dCameraFitStep::Idle);assert!(state.camera.projection_frame_owed);
    state.camera.orbit.orbit(0.4,-0.2);state.camera.camera_user_moved=true;let moved=state.camera.orbit.clone();
    let mut loaded=camera_framing_scene(&fixture,"perspective",true,None);loaded.world_3d.as_mut().unwrap().camera_json=scene.world_3d.as_ref().unwrap().camera_json.clone();drive_scene_bridge(&mut state,&loaded,viewport);
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert_eq!(state.camera.orbit.yaw,moved.yaw);assert_eq!(state.camera.orbit.pitch,moved.pitch);assert_eq!(state.camera.orbit.target,moved.target);
    assert!(!state.camera.projection_frame_owed);assert_eq!(step_world3d_camera_fit(&mut state),World3dCameraFitStep::Idle);
    println!("[DEBUG] originalContentFrameUserOwnership pendingDebtConsumed=true lateGeometryPreservesOrbit=true");
}

/// 🎥️ User orbit owns one revision/seed; a new document revision or external seed owns a new frame.
#[test]
fn camera_fit_ownership_preserves_user_orbit_until_revision_or_seed_changes() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let scene = camera_framing_scene(&fixture, "perspective", true, None);
    let mut state = World3dState::new("surface-owner".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, viewport);
    drive_camera_fit(&mut state);
    state.camera.orbit.orbit(0.4, -0.2);
    state.camera.camera_user_moved = true;
    let moved = state.camera.orbit.clone();
    sync_world3d_state(&mut state, &scene, viewport);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.camera.orbit.yaw, moved.yaw, "same owner never takes back a user orbit");

    let next_revision = camera_framing_scene(&fixture, "perspective", true, Some(fixture["fit"]["revision"].as_u64().unwrap() + 1));
    sync_world3d_state(&mut state, &next_revision, viewport);
    assert!(drive_camera_fit(&mut state) >= fixture["instances"].as_array().unwrap().len() * 8, "a new revision remeasures resident geometry through the bounded cursor");
    assert_eq!(state.camera.orbit.yaw, moved.yaw, "reframing preserves the user's look direction");
    state.camera.orbit.orbit(-0.3, 0.1);
    state.camera.camera_user_moved = true;

    let mut next_seed_fixture = fixture.clone();
    next_seed_fixture["cameras"]["perspective"]["position"][0] = serde_json::Value::from(12.0);
    let next_seed = camera_framing_scene(&next_seed_fixture, "perspective", true, Some(fixture["fit"]["revision"].as_u64().unwrap() + 1));
    drive_scene_bridge(&mut state, &next_seed, viewport);
    drive_camera_fit(&mut state);
    assert!(state.camera.fit_applied_key.is_some(), "an external camera seed changes the fit key and owns a fresh frame");
}

#[test]
fn scene_bridge_renders_the_generation3d_preview_payload_into_a_snapshot() {
    let fixture = scene_bridge_fixture();
    let scene = scene_from_bridge_fixture(&fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);

    let expect = &fixture["expect"];
    assert_eq!(state.session.snapshot_fault, None, "the bridge must publish without faulting");
    assert_eq!(state.scene.draws.iter().count(), expect["draws"].as_u64().expect("draws") as usize, "original surface and wire meshes each retain their draw");
    let draw = state.scene.draws.iter().find(|draw| Some(draw.mesh_key.as_str()) == expect["drawMeshKey"].as_str()).expect("published solid draw");
    assert_eq!(draw.mesh_key, expect["drawMeshKey"].as_str().expect("mesh key"));
    let instance_ids: Vec<&str> = draw.instances.iter().map(|instance| instance.id.as_str()).collect();
    let expected_instance_ids: Vec<&str> = expect["instanceIds"].as_array().expect("instance ids").iter().map(|id| id.as_str().expect("instance id")).collect();
    assert_eq!(instance_ids, expected_instance_ids, "instance ids stay channel-qualified across the wire");

    let mesh = *state.scene.meshes.get(&draw.mesh_key).expect("published mesh geometry");
    let schema = mesh.schema().expect("mesh schema");
    assert_eq!(schema.indices / 3, expect["triangles"].as_u64().expect("triangles") as u32, "the bridged mesh carries the tessellated triangle count, not a placeholder box");
    assert_eq!(schema.vertices, expect["vertices"].as_u64().expect("vertices") as u32);

    for id in expect["wireOnlyMeshesRetained"].as_array().expect("wire meshes") {
        let key = id.as_str().expect("mesh id");
        let wire = state.scene.meshes.get(key).expect("original wire mesh is retained").schema().unwrap();
        assert_eq!(wire.indices, 0, "wire geometry has no synthetic triangles");
        assert!(wire.edges > 0);
        assert!(state.scene.draws.iter().any(|draw|draw.mesh_key == key));
    }
}

/// 🎯️ The delivery that arrives WITH a first framing must still publish its draws.
///
/// ⚖️ The fit now waits until the delivery and its draw rebuild are complete. It must remain ordered
/// there: framing during `sync_world3d_state` advanced `interaction_revision` while the delivery's
/// mesh items still carried the earlier revision, so the rebuild answered `WorldDrawRebuildStep::Stale`
/// and nothing ever began another — the surface
/// keeps its meshes and zero draws for the life of the document. Measured on 6118 as
/// `state-meshes=3 apply=false rebuild=false state-draws=0 draws=0` on every generation3d example in
/// both roles, with `world3d-editor` falling 75/115 → 62/115 the hour the fit lane landed
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-regressions-sweep-2026-09-15.md`).
#[test]
fn a_first_framing_never_strands_the_deliverys_own_draw_rebuild() {
    let fixture = scene_bridge_fixture();
    let mut scene = scene_from_bridge_fixture(&fixture);
    scene.world_3d.as_mut().expect("world scene").fit_json = Some(BOOT_FRAME_FIT_JSON.to_string());
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 1600.0, h: 900.0 });
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    assert_eq!(state.session.snapshot_fault, None, "a framed delivery must apply without faulting");
    assert!(state.camera.fit_applied_key.is_some(), "the delivery's own fit lane framed this document once");
    assert_eq!(state.scene.draws.iter().count(), fixture["expect"]["draws"].as_u64().expect("draws") as usize, "the framing must not strand the delivery's draw rebuild");
    assert!(state.session.interaction_revision >= state.bridge.snapshot_lease.expect("applied lease").revision, "fit advances only after the applied delivery owns the surface revision");
}

/// 🟩️ A `provisional: true` instance record — the producer's stamp from `ArtifactView::tool_run()` (tool run
/// contract §4.1 layer 1) — fills `provisional_instance_ids`, so the draw pass paints it with the provisional
/// token; records without the flag stay ordinary instances.
#[test]
fn scene_bridge_collects_the_provisional_instance_flag_the_producer_stamps() {
    let mut fixture = scene_bridge_fixture();
    let bridged: Vec<String> = fixture["expect"]["instanceIds"].as_array().expect("instance ids").iter().map(|id| id.as_str().expect("instance id").to_string()).collect();
    let provisional = bridged.first().expect("a bridged instance").clone();
    for instance in fixture["instancesJson"].as_array_mut().expect("instances") {
        if instance["id"].as_str() == Some(provisional.as_str()) {
            instance["provisional"] = serde_json::Value::Bool(true);
        }
    }
    let scene = scene_from_bridge_fixture(&fixture);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });
    assert_eq!(state.selection.provisional_instance_ids, HashSet::from([provisional]), "exactly the stamped record is provisional");
}

/// 🧭️ Only a world surface bound to a window AND holding an applied trace lane owes that window a cursor echo.
#[test]
fn world_surfaces_echo_their_trace_cursor_under_their_window_instance() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/⏯️tool-run/🧪️tests/🧫️fixtures/📼️trace-pages.json")).expect("trace pages fixture");
    let bytes: Vec<u8> = fixture["deltas"][0]["hex"].as_str().expect("delta hex").as_bytes().chunks(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex"), 16).expect("hex")).collect();
    let delta = semio_framework_tool_run::ToolRunTraceDelta::decode(&bytes).expect("fixture delta decodes");
    let lane = base64_codec::base64_url_encode(&bytes);
    let mut bound = World3dState::new("surface-bound".into(), "controller".into());
    bound.trace.tool_run_trace_window_id = Some("world-left".into());
    bound.trace.tool_run_trace.apply_lane(Some(&lane)).expect("lane applies");
    let mut unbound = World3dState::new("surface-unbound".into(), "controller".into());
    unbound.trace.tool_run_trace.apply_lane(Some(&lane)).expect("lane applies");
    let mut idle = World3dState::new("surface-idle".into(), "controller".into());
    idle.trace.tool_run_trace_window_id = Some("world-right".into());
    let echoed = world3d_tool_run_trace_cursors([&bound, &unbound, &idle]);
    let expected = semio_framework_tool_run::ToolRunTraceCursor { run: delta.identity.id.run, generation: delta.identity.generation, page: delta.next };
    assert_eq!(echoed, std::collections::HashMap::from([("world-left".to_string(), expected)]));
}

#[test]
fn scene_bridge_honours_selection_hover_camera_and_sun() {
    let fixture = scene_bridge_fixture();
    let scene = scene_from_bridge_fixture(&fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);

    let expect = &fixture["expect"];
    assert_eq!(state.selection.selected_ids, expect["selectedIds"].as_array().expect("selected").iter().map(|id| id.as_str().expect("id").to_string()).collect::<Vec<_>>());
    assert_eq!(state.selection.local_hover_id.as_deref(), expect["hoveredId"].as_str());
    let draw = state.scene.draws.iter().find(|draw| Some(draw.mesh_key.as_str()) == expect["drawMeshKey"].as_str()).expect("published solid draw");
    let instance = draw.instances.first().expect("published instance");
    assert!(instance.selected, "the scene's own selection document must paint the instance selected");

    let camera = state.camera.orbit.to_camera();
    let expected_camera = expect["cameraPosition"].as_array().expect("camera");
    for axis in 0..3 {
        assert!((f64::from(camera.position.to_array()[axis]) - expected_camera[axis].as_f64().expect("axis")).abs() < 1.0e-4, "camera axis {axis}: {:?}", camera.position);
    }

    let sun = environment_light_dir(&state.scene.environment);
    let expected_sun = expect["sunDirection"].as_array().expect("sun");
    for axis in 0..3 {
        assert!((f64::from(sun[axis]) - expected_sun[axis].as_f64().expect("axis")).abs() < 1.0e-5, "sun axis {axis}: {sun:?}");
    }
}

/// 🎯️ Aim through the fixture camera's own target, which is the centre of the prism's base face
/// — the one point guaranteed both inside the solid and inside the 45° frustum.
#[test]
fn scene_bridge_binds_the_apps_interaction_domain_for_world_picking() {
    let fixture = scene_bridge_fixture();
    let scene = scene_from_bridge_fixture(&fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    state.session.pick_bounds = bounds;

    assert_eq!(resolved_domain_id(&state.session), fixture["domainId"].as_str().expect("domain"));
    assert_eq!(resolved_domain_granularity_id(&state.session), fixture["domainGranularityId"].as_str().expect("granularity"));
    let instance_id = state.scene.draws.iter().find(|draw| Some(draw.mesh_key.as_str()) == fixture["expect"]["drawMeshKey"].as_str()).expect("published solid draw").instances.first().expect("published instance").id.clone();
    assert_eq!(resolved_item_id(&state.session, &instance_id), instance_id, "a bound app domain addresses the bare channel-qualified id — never `surfaceId/id`");
    let interaction_id = state.interaction.instance_interaction_ids.get(&instance_id).expect("the scene bridge retains the instance's topology target").clone();
    assert_eq!(interaction_id, "extrude@solid");

    let camera = state.camera.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, bounds.w, bounds.h), Vec3::ZERO, bounds.w, bounds.h).expect("camera target projects");
    let hit = pick_instance_at(&state.session, &state.camera, &state.scene, screen[0], screen[1], bounds);
    assert_eq!(hit.as_deref(), Some(instance_id.as_str()), "the bridged geometry is what a world pick actually hits");
    let select = pick_select_action(&state.session, &state.camera, &state.selection, &state.interaction, &state.scene, screen[0], screen[1], bounds, false, false).expect("pick emits an action");
    assert_eq!(select.action, "interactionSelect");
    let args = select.args.expect("pick args");
    assert_eq!(args["domainId"].as_str(), fixture["domainId"].as_str());
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert_eq!(targets[0]["granularity"].as_str(), fixture["domainGranularityId"].as_str());
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert_eq!(targets[0]["id"].as_str(), Some(interaction_id.as_str()));

    state.selection.local_hover_id = None;
    let hover = pick_hover_action(&mut state.session, &mut state.camera, &mut state.selection, &mut state.interaction, &mut state.scene, screen[0], screen[1], bounds).expect("hover emits an action");
    assert_eq!(hover.action, "interactionHover");
    let args = hover.args.expect("hover args");
    assert_eq!(args["domainId"].as_str(), fixture["domainId"].as_str());
    let targets: serde_json::Value = serde_json::from_str(args["targets"].as_str().unwrap()).unwrap();
    assert_eq!(targets[0]["id"].as_str(), Some(interaction_id.as_str()));
}
//#endregion 🌉️World3dSceneBridge

/// 🎨️ LAW: decoded paint pixels become resident only for the exact mesh generation shape that can
/// consume them; codec work remains outside the frame-owned World state.
#[test]
fn decoded_mesh_paint_publication_requires_rgba_and_matching_uvs() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut painted = triangle_mesh_oracle();
    painted.uvs = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
    state.scene.meshes.insert("painted".into(), publish_oracle_mesh(painted)).expect("painted mesh admitted");
    state.scene.mesh_versions.insert("painted".into(), 1).expect("painted revision admitted");
    let paint = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 3 }), 96);
    assert!(apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "painted", DecodedMeshPaintImage { raster: paint }));
    let resident = state.scene.mesh_paint_textures.get("painted").expect("decoded paint pixels became resident");
    assert_eq!((resident.identity.descriptor().width, resident.identity.descriptor().height), (1, 1));

    state.scene.meshes.insert("plain".into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("plain mesh admitted");
    state.scene.mesh_versions.insert("plain".into(), 1).expect("plain revision admitted");
    let plain = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 3 }), 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "plain", DecodedMeshPaintImage { raster: plain }), "a mesh without one UV per vertex cannot sample a paint map");
    let stale = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 2, uv_revision: 2, uv_count: 3 }), 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "painted", DecodedMeshPaintImage { raster: stale }), "paint from another mesh generation cannot cross the resident topology");
    let wrong_uvs = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 2 }), 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "painted", DecodedMeshPaintImage { raster: wrong_uvs }), "paint UV cardinality is exact");
    let wrong_profile = test_scene_raster_lease(1, 1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state.session, &mut state.authority, &mut state.scene, "painted", DecodedMeshPaintImage { raster: wrong_profile }), "a reference raster cannot cross the mesh-paint profile seal");
}

/// 🫧 LAW: retained transparent Standard objects use Three's transformed bounding-sphere-centre
/// projected depth and preserve producer insertion order for exact ties.
#[test]
fn transparent_standard_draws_match_the_recorded_three_projected_order() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎨️scene-shading/🔣️.json")).unwrap();
    let camera_json = &fixture["scene"]["camera"];
    let array3 = |value: &serde_json::Value| {
        let values = value.as_array().expect("three scalars");
        [values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32]
    };
    let camera = Camera3d {
        position: Vec3::new(array3(&camera_json["position"])[0], array3(&camera_json["position"])[1], array3(&camera_json["position"])[2]),
        target: Vec3::new(array3(&camera_json["target"])[0], array3(&camera_json["target"])[1], array3(&camera_json["target"])[2]),
        up: Vec3::new(array3(&camera_json["up"])[0], array3(&camera_json["up"])[1], array3(&camera_json["up"])[2]),
        fov_y: (camera_json["fov"].as_f64().unwrap() as f32).to_radians(),
        near: 0.01,
        far: 100.0,
        projection: CameraProjection3d::Perspective,
        zoom: 1.0,
    };
    let view_proj = camera.view_proj(64.0, 64.0);

    for case in fixture["depthOrderCases"].as_array().unwrap() {
        let mut state = World3dState::new(case["id"].as_str().unwrap().into(), "controller".into());
        let mut draws = Vec::new();
        for layer in case["layers"].as_array().unwrap() {
            let id = layer["id"].as_str().unwrap();
            let center = array3(&layer["localCenter"]);
            let position = array3(&layer["position"]);
            let mesh = mesh_oracle_from_buffers(
                vec![center[0] - 0.5, center[1] - 0.5, center[2], center[0] + 0.5, center[1] - 0.5, center[2], center[0], center[1] + 0.5, center[2]],
                vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
                vec![0, 1, 2],
            );
            state.scene.meshes.insert(id.into(), publish_oracle_mesh(mesh)).expect("sort mesh admitted");
            draws.push(SceneMaterialDraw3d {
                mesh_key: id.into(),
                mesh_version: 0,
                first_index: 0,
                index_count: u32::MAX,
                instances: vec![Instance3d { component_source: None,
                    id: id.into(),
                    model: Instance3d::model_from_trs(position, [0.0, 0.0, 0.0, 1.0], [1.0; 3]),
                    color: [1.0, 1.0, 1.0, layer["opacity"].as_f64().unwrap() as f32],
                    selected: false,
                    hovered: false,
                    material: Default::default(),
                }],
                material: SceneMaterialKind3d::Standard,
                translucent: true,
            });
        }
        sort_world3d_translucent_material_draws(&state.scene.meshes, view_proj, &mut draws);
        let actual: Vec<&str> = draws.iter().flat_map(|draw| draw.instances.iter().map(|instance| instance.id.as_str())).collect();
        let expected: Vec<&str> = case["expectedDrawOrder"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect();
        assert_eq!(actual, expected, "{} follows the actual Three order", case["id"].as_str().unwrap());
        retire_bridged_surface(&mut state);
    }
}

/// 🔘️ LAW: every mesh key the gumball draws is ENSURED on the GPU in the same build, and the plane it
/// draws is pinned in the mesh pool.
///
/// The defect: `append_gumball_geometry` pushed three translucent `SceneDraw3d`s keyed
/// `gumball-plane` and never called `gpu.ensure_mesh` for it — the one thing every other world draw
/// site does. On the GPU a missing mesh is a FATAL `present_step` fault ("prepared frame submit step:
/// prepared world mesh was missing"), not a skipped draw, so the first frame after ANY selection
/// killed the whole frame loop. It sat unreachable while the wgpu host dropped the reserved tool job
/// that applies a selection; the instant that job ran, the app died on the first click
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-spawn-job-effect-2026-09-13.md`).
#[test]
fn gumball_draws_ensure_the_plane_mesh_they_reference() {
    let mut lines = Vec::new();
    let mut translucent = Vec::new();
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.session.active_utility = "select".into();
    state.scene.meshes.insert(GUMBALL_PLANE_MESH.into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("plane lease admitted");
    state.scene.mesh_versions.insert(GUMBALL_PLANE_MESH.into(), 7).expect("plane version admitted");
    state.scene.meshes.insert("mesh-1".into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("body lease admitted");
    state.scene.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { component_source: None, id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: true, hovered: false, material: Default::default() }], shadow_role: Default::default() }).expect("body draw admitted");
    state.selection.selected_ids = vec!["obj-1".into()];
    append_gumball_geometry(&mut lines, &mut translucent, &mut gpu, &state.selection, &state.gumball, &state.scene, &Camera3d::default(), &state.scene.meshes, &state.scene.mesh_versions);

    assert_eq!(translucent.len(), 3, "the gumball draws one quad per axis plane");
    let requested: Vec<(String, u64)> = gpu.mesh_requests[..gpu.mesh_request_len].iter().flatten().cloned().collect();
    for draw in &translucent {
        assert!(requested.iter().any(|(key, version)| key == &draw.mesh_key && *version == draw.mesh_version), "gumball draw {}@{} was never ensured on the GPU; requested={requested:?}", draw.mesh_key, draw.mesh_version);
    }
    assert!(requested.iter().all(|(key, _)| key == GUMBALL_PLANE_MESH), "the gumball ensures nothing but its own plane: {requested:?}");
}

/// 🎥️ LAW: the diagnostics row reports the LIVE orbit — the pose the pane is looking through this
/// frame, its projection family and its framing orientation — not the wire camera the guest
/// published.
///
/// 🩸️ `dumpMeshStats` filled `camera` from `World3dScene.camera_json` and nothing else, so every
/// local camera move was invisible to a probe: a `Projection` pane switch to `Orthographic` reported
/// the delivered perspective pose, unchanged, and read as a dead control
/// (`📓️w8b-orthographic-camera-and-3d-parity.md` §7.5).
#[test]
fn the_live_camera_row_reports_the_orbit_and_not_the_wire() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.bounds = Rect::new(0.0, 0.0, 478.0, 814.0);
    let before = world3d_live_camera_json(&state.session, &state.camera, &state.selection, &state.scene);
    assert_eq!(before["projection"]["mode"]["kind"], "threePoint", "🎥️ an untouched pane opens the canonical three-point perspective mode");
    assert_eq!(before["userMoved"], serde_json::Value::Bool(false));

    let top_spec = semio_framework_ui_viewport::Viewport3dProjectionSpec { mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {}, orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top } };
    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, top_spec));
    let after = world3d_live_camera_json(&state.session, &state.camera, &state.selection, &state.scene);
    assert_eq!(after["projection"]["mode"]["kind"], "orthographic", "🎥️ the switch is visible in the row the probe reads");
    assert_eq!(after["projection"]["orientation"]["type"], "cardinal", "🎥️ and so is the plane its framing measures in");
    assert!(after["zoom"].as_f64().unwrap_or_default() > 1.0, "🎥️ a parallel pane reports its real frustum scale: {}", after["zoom"]);

    state.camera.orbit.target = Vec3::new(3.5, 0.0, 0.005);
    let moved = world3d_live_camera_json(&state.session, &state.camera, &state.selection, &state.scene);
    let target: Vec<f64> = moved["target"].as_array().expect("the row carries a target").iter().filter_map(serde_json::Value::as_f64).collect();
    assert!((target[0] - 3.5).abs() < 1e-6 && target[1].abs() < 1e-6 && (target[2] - 0.005).abs() < 1e-6, "🎥️ the row follows the orbit, not the wire: {target:?}");
}

/// 📐️ LAW: local projection selections acquire the spec without rearming content framing. React's
/// viewport owner remains mounted and `WorldProjectionSnapDriver` retains the accepted pose.
///
/// 📐️ And the wire never takes the selection back. The settle a press queues publishes
/// `setCamera`, whose payload carries no `projection` member at all, so the guest's echo always
/// arrives in the DELIVERED family — applying it whole is how the switch undid itself in one
/// round trip.
#[test]
fn a_local_projection_selection_retains_viewport_ownership() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.session.bounds = Rect::new(0.0, 0.0, 478.0, 814.0);
    state.camera.camera_user_moved = true;
    state.camera.projection_frame_key = Some(7);
    state.camera.projection_frame_zoom = Some(3.0);

    let top = semio_framework_ui_viewport::Viewport3dProjectionSpec { mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {}, orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top } };
    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, top), "📐️ the press moves the pane");
    assert!(!state.camera.projection_frame_owed, "📐️ local selection retains the viewport owner instead of invoking the content framer");
    assert!(state.camera.projection_selected, "📐️ and holds its own spec from here on, React's `externalPendingProjectionSpec`");
    assert_eq!(state.camera.projection_frame_key, Some(7));
    assert_eq!(state.camera.projection_frame_zoom, Some(3.0));
    assert_eq!(state.camera.projection_spec, top);

    assert!(!apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, top), "📐️ pressing the SAME row again changes nothing");
    let cabinet = semio_framework_ui_viewport::Viewport3dProjectionSpec { mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Oblique { variant: semio_framework_ui_viewport::Viewport3dObliqueVariant::Cabinet, angle: 45.0, depth_scale: 0.5 }, orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Free {} };
    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, cabinet), "📐️ but a row with the same family and another plane does — Cabinet after Orthographic");
    assert_eq!(state.camera.projection_spec, cabinet, "📐️ a free oblique retains every framing and shear parameter");

    let mut wired = World3dState::new("surface".into(), "controller".into());
    wired.session.bounds = Rect::new(0.0, 0.0, 478.0, 814.0);
    assert!(apply_world3d_projection_spec(&mut wired.session, &mut wired.camera, &mut wired.scene, top));
    let echo = OrbitController { projection: CameraProjection3d::Perspective, target: Vec3::new(7.0, 0.0, 0.01), ..OrbitController::default() };
    let held = OrbitController { projection: wired.camera.orbit.projection, ..echo.clone() };
    assert_eq!(held.projection, CameraProjection3d::Orthographic, "📐️ the echoed pose lands, the echoed FAMILY does not");
    assert_eq!(held.target, echo.target, "📐️ everything else the wire says still wins");
}

/// 🐟️ LAW: Top → Curvilinear keeps the accepted Top eye/target/distance/zoom while replacing the
/// projection family and FOV. This is the exact mounted React transition after initial framing.
#[test]
fn curvilinear_selection_retains_the_accepted_top_pose_and_zoom() {
    let mut state = World3dState::new("surface-curvilinear-frame".into(), "controller".into());
    state.session.bounds = Rect::new(0.0, 0.0, 400.0, 800.0);
    state.camera.orbit.projection = CameraProjection3d::Orthographic;
    state.camera.orbit.target = Vec3::new(5.4054055, 2.3406067, 1.5015);
    state.camera.orbit.distance = 19.855713;
    state.camera.orbit.zoom = 6.1791667;
    state.camera.projection_spec = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {},
        orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top },
    };
    state.camera.projection_selected = true;
    let accepted = state.camera.orbit.clone();
    let spec = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Curvilinear { fov: 120.0, strength: 1.0, mapping: semio_framework_ui_viewport::Viewport3dCurvilinearMapping::Fisheye },
        orientation: state.camera.projection_spec.orientation,
    };

    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, spec));
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    let camera = state.camera.orbit.to_camera();
    assert_eq!(camera.target.to_array(), accepted.target.to_array());
    assert_eq!(camera.position.to_array(), accepted.to_camera().position.to_array());
    assert_eq!(state.camera.orbit.distance, accepted.distance);
    assert_eq!(state.camera.orbit.zoom, accepted.zoom);
    assert!((camera.fov_y.to_degrees() - 120.0).abs() <= 2e-5);
    assert_eq!(camera.projection, CameraProjection3d::Perspective);
    assert!(!state.camera.projection_frame_owed);
}

/// 📥️ LAW: a delivered full projection spec owns one frame after its camera snapshot is accepted.
/// The bridge exposes instance bounds before the staged camera page lands, so consuming the frame any
/// earlier lets snapshot apply overwrite it. A later document-bound change stays put until another
/// external seed or explicit projection selection rearms the debt.
#[test]
fn delivered_projection_spec_frames_once_after_accepted_camera() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let spec = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {},
        orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Cardinal { view: semio_framework_ui_viewport::Viewport3dOrthographicView::Top },
    };
    let mut scene = camera_framing_scene(&fixture, "perspective", true, None);
    scene.world_3d.as_mut().expect("world scene").camera_json = serde_json::json!({
        "position": [40, -30, 20],
        "target": [0, 0, 0],
        "up": [0, 0, 1],
        "fov": 50,
        "zoom": 1,
        "projection": spec,
    })
    .to_string();
    let mut state = World3dState::new("surface-delivered-projection".into(), "controller".into());
    state.camera.camera_user_moved = true;

    drive_scene_bridge(&mut state, &scene, viewport);
    assert_eq!(state.camera.projection_spec, spec, "the typed delivered spec survives the scene bridge");
    assert!(state.camera.projection_frame_owed, "an accepted external projection seed owns one content frame");
    assert!(!state.camera.camera_user_moved, "external camera reattach releases the previous viewport owner");

    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert!(!state.camera.projection_frame_owed, "the accepted camera consumes the debt exactly once");
    let accepted = state.camera.orbit.clone();
    state.scene.instance_positions.insert("late-growth".into(), [1_000.0, 1_000.0, 1_000.0]);
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert_eq!(state.camera.orbit.target.to_array(), accepted.target.to_array());
    assert_eq!(state.camera.orbit.to_camera().position.to_array(), accepted.to_camera().position.to_array());
}

/// 📷️ An authored preview camera keeps its exact pose while its full projection spec still drives
/// matrix family/FOV. A later local projection command acquires the spec without framing content.
#[test]
fn delivered_preserve_camera_policy_cancels_frame_debt_until_local_projection_selection() {
    let fixture = camera_framing_fixture();
    let projection_fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🪟️viewport/🧪️tests/🧫️fixtures/📐️projection/🔣️.json")).unwrap();
    assert!(projection_fixture["framePolicies"].as_array().unwrap().iter().any(|row| row["value"] == "preserveCamera" && row["expectedFramesContent"] == false));
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let spec = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::Orthographic {},
        orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Free {},
    };
    let mut scene = camera_framing_scene(&fixture, "perspective", true, None);
    scene.world_3d.as_mut().expect("world scene").camera_json = serde_json::json!({
        "position": [40, -30, 20],
        "target": [123, 456, 789],
        "up": [0, 0, 1],
        "fov": 50,
        "zoom": 7,
        "projection": spec,
        "projectionFrame": "preserveCamera",
    })
    .to_string();
    let mut state = World3dState::new("surface-preserved-camera".into(), "controller".into());

    drive_scene_bridge(&mut state, &scene, viewport);
    assert_eq!(state.camera.projection_frame_policy, semio_framework_ui_viewport::Viewport3dProjectionFramePolicy::PreserveCamera);
    assert!(!state.camera.projection_frame_owed);
    let accepted = state.camera.orbit.clone();
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert_eq!(state.camera.orbit.target.to_array(), accepted.target.to_array());
    assert_eq!(state.camera.orbit.to_camera().position.to_array(), accepted.to_camera().position.to_array());
    assert_eq!(state.camera.orbit.zoom, accepted.zoom);

    let local = semio_framework_ui_viewport::Viewport3dProjectionSpec {
        mode: semio_framework_ui_viewport::Viewport3dProjectionMode::ThreePoint { fov: 50.0 },
        orientation: semio_framework_ui_viewport::Viewport3dProjectionOrientation::Free {},
    };
    assert!(apply_world3d_projection_spec(&mut state.session, &mut state.camera, &mut state.scene, local));
    assert_eq!(state.camera.projection_frame_policy, semio_framework_ui_viewport::Viewport3dProjectionFramePolicy::Content);
    assert!(!state.camera.projection_frame_owed, "a local projection selection retains the accepted authored pose");
    sync_world3d_projection_content_frame(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut state.bridge);
    assert!(!state.camera.projection_frame_owed);
    assert_eq!(state.camera.orbit.target.to_array(), accepted.target.to_array());
    assert_eq!(state.camera.orbit.to_camera().position.to_array(), accepted.to_camera().position.to_array());
}

/// 🖼️ LAW: reference identity, geometry, and visual state come from the shared Three fixture.
#[test]
fn reference_visual_identity_geometry_and_interaction_match_three() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🖼️reference-visual/🔣️.json")).expect("reference visual fixture");
    let identity = fixture["identityCases"].as_array().expect("identity cases");
    let shared_url = identity[0]["url"].as_str().expect("shared URL");
    assert_eq!(identity[1]["url"].as_str(), Some(shared_url));

    let geometry = &fixture["geometry"];
    let default_width = geometry["defaultWidthWorld"].as_f64().unwrap() as f32;
    assert_eq!(world_reference_width(&WorldReferenceRecord::default()), default_width, "the neutral fixture and WGPU use React's missing-width authority");
    let source = geometry["sourceNaturalSize"].as_array().expect("source natural size");
    let mut state = World3dState::new("surface".into(), "controller".into());
    let _ = state.scene.reference_pixels.insert(shared_url.into(), test_world_scene_raster(source[0].as_u64().unwrap() as u32, source[1].as_u64().unwrap() as u32, 0));
    for (index, row) in identity.iter().enumerate() {
        state.scene.references.push(WorldReferenceRecord {
            id: Some(row["id"].as_str().unwrap().into()),
            url: Some(shared_url.into()),
            origin: Some([-2.0 + index as f64 * 4.0, 0.0, 0.0]),
            width_world: Some(2.0),
            ..Default::default()
        });
    }
    state.scene.references.push(WorldReferenceRecord { id: Some("locked".into()), url: Some(shared_url.into()), locked: Some(true), ..Default::default() });
    state.session.interaction_revision = 11;
    let mut registry = WorldInteractionRegistryBuildCursor::new(11);
    for _ in 0..16 {
        if with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    let reference_ids: Vec<&str> = state.interaction
        .interaction_objects
        .slots
        .iter()
        .flatten()
        .filter(|entry| entry.revision == 11 && entry.kind == WorldInteractionObjectKind::Reference)
        .map(|entry| entry.id.as_str())
        .collect();
    assert_eq!(reference_ids.len(), 2, "two authored ids sharing one URL remain two interaction objects; locked stays nonselectable");
    for row in identity {
        assert!(reference_ids.contains(&row["expectedInteractionId"].as_str().unwrap()));
    }

    let width = geometry["widthWorld"].as_f64().unwrap() as f32;
    let [actual_width, actual_height] = world_reference_plane_size(&state.scene, shared_url, width);
    let expected_size = geometry["expectedPlaneSize"].as_array().unwrap();
    assert!((actual_width - expected_size[0].as_f64().unwrap() as f32).abs() < 1e-5);
    assert!((actual_height - expected_size[1].as_f64().unwrap() as f32).abs() < 1e-5);
    let origin: [f32; 3] = geometry["origin"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect::<Vec<_>>().try_into().unwrap();
    let model = Instance3d::model_from_trs(origin, [0.0, 0.0, 0.0, 1.0], [actual_width, actual_height, 1.0]);
    for (index, local) in [Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.5, -0.5, 0.0), Vec3::new(0.5, 0.5, 0.0), Vec3::new(-0.5, 0.5, 0.0)].into_iter().enumerate() {
        let actual = model.transform_point(local).to_array();
        let expected = geometry["expectedCorners"][index].as_array().unwrap();
        for axis in 0..3 {
            assert!((actual[axis] - expected[axis].as_f64().unwrap() as f32).abs() < 1e-5, "corner {index} axis {axis}");
        }
    }

    state.interaction.interaction_objects.revision = state.session.interaction_revision;
    let mut select = WorldObjectPickCursor::from_ray(state.session.interaction_revision, 12, WorldObjectPickPurpose::ReferenceSelect, Vec3::new(-2.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
    select.merge = world_merge_code(false, false, false);
    for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
        if with_world_step_context(1, |context| select.step(&state.session, &state.interaction, 12, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    let mut plan = select.finish_plan(&state.session, &state.selection, &state.interaction, 12).expect("reference select plan").expect("reference hit");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 12, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let targets = actions[0].args.as_ref().and_then(|args| args.get("targets")).and_then(|value| value.as_str()).and_then(|raw| serde_json::from_str::<Vec<serde_json::Value>>(raw).ok()).expect("reference target JSON");
    assert_eq!(targets[0]["id"], format!("{}/{}", state.session.surface_id, identity[0]["expectedInteractionId"].as_str().unwrap()));
    assert_eq!(targets[0]["granularity"], "reference");
}

#[test]
fn reference_visual_geometry_reaches_the_actual_textured_scene_pass() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🖼️reference-visual/🔣️.json")).unwrap();
    let geometry = &fixture["geometry"];
    let url = fixture["identityCases"][0]["url"].as_str().unwrap();
    let mut scene = scene_with_selection("{}");
    scene.world_3d.as_mut().unwrap().references_json = Some(serde_json::json!([{
        "id": "actual-reference-plane", "url": url, "origin": geometry["origin"],
        "widthWorld": geometry["widthWorld"], "locked": true, "hidden": false
    }]).to_string());
    let bounds = Rect::new(0.0, 0.0, 320.0, 240.0);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    let size = &geometry["sourceNaturalSize"];
    let _ = state.scene.reference_pixels.insert(url.into(), test_world_scene_raster(size[0].as_u64().unwrap() as u32, size[1].as_u64().unwrap() as u32, 0));
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = ui_wgpu::wgpu::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = ui_wgpu::wgpu::widgets::WidgetContext {
        draw: &mut draw, overlay: None, atlas: &mut atlas, icons: None, input: &mut input, theme: &theme,
        scroll_offsets: &mut scroll, collapsed_sections: &mut collapsed, open_selects: &mut selects,
        interaction_maps: None, pick_clip: None, viewport_height: bounds.h,
    };
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::World);
    let pass = draw.scene_passes.last().expect("real World3d scene pass");
    assert_eq!(pass.textured_draws.len(), 1);
    assert_eq!(pass.textured_draws[0].instances.len(), 1);
    let actual = &pass.textured_draws[0].instances[0];
    assert_eq!(actual.texture_key, url);
    for (index, local) in [Vec3::new(-0.5, -0.5, 0.0), Vec3::new(0.5, -0.5, 0.0), Vec3::new(0.5, 0.5, 0.0), Vec3::new(-0.5, 0.5, 0.0)].into_iter().enumerate() {
        let point = actual.model.transform_point(local).to_array();
        for axis in 0..3 {
            assert!((point[axis] - geometry["expectedCorners"][index][axis].as_f64().unwrap() as f32).abs() < 1e-5, "emitted corner {index} axis {axis}");
        }
    }
    retire_bridged_surface(&mut state);
}

/// 🌐️ LAW: the actual World producer publishes one retained procedural grid scalar and no
/// finite grid line vertices. The later overlay lanes remain ordinary line geometry.
#[test]
fn an_actual_world_grid_is_one_camera_projected_procedural_scalar_without_grid_lines() {
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().unwrap();world.meshes_json = "[]".into();world.instances_json = "[]".into();
    let bounds = Rect::new(0.0, 0.0, 320.0, 240.0);
    let mut state = World3dState::new("surface-grid".into(), "controller-grid".into());
    state.camera.orbit.target = Vec3::new(7.0, 11.0, 0.0);
    state.camera.orbit.distance = 59.0;
    state.camera.orbit.yaw = -0.63;
    state.camera.orbit.pitch = 0.71;
    state.scene.lod.show_grid = true;
    state.scene.lod.grid_datum = Some([3.0, 5.0, 2.0]);
    drive_scene_bridge(&mut state, &scene, bounds);
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = ui_wgpu::wgpu::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = ui_wgpu::wgpu::widgets::WidgetContext {
        draw: &mut draw, overlay: None, atlas: &mut atlas, icons: None, input: &mut input, theme: &theme,
        scroll_offsets: &mut scroll, collapsed_sections: &mut collapsed, open_selects: &mut selects,
        interaction_maps: None, pick_clip: None, viewport_height: bounds.h,
    };

    let expected_camera = state.camera.orbit.to_camera();
    let expected_step = lod_grid_step_world(scene_lod(&state.session, &state.camera, &state.scene), state.scene.lod.grid_factor).expect("positive retained LOD step");
    let expected_fade = camera_grid_fade_distance(&expected_camera, 2.0, expected_step, bounds.w, bounds.h);
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::World);
    let pass = draw.scene_passes.last().expect("actual World3d scene pass");
    let grid = pass.procedural_grid.expect("one actual retained procedural grid");
    assert!((grid.plane_z - 2.001).abs() < 1e-6, "only the rendered plane receives the datum offset");
    assert_eq!(grid.camera_plane_projection, [expected_camera.position.x, expected_camera.position.y, 2.001], "the infinite plane follows camera position rather than the orbit target");
    assert!((grid.cell_size - expected_step as f32).abs() < f32::EPSILON, "the retained cell scalar owns the actual LOD step");
    assert!((grid.fade_distance - expected_fade).abs() < f32::EPSILON, "the fade helper receives the unshifted datum and logical viewport");
    assert_eq!(grid.cell_color, [theme.text_element.r, theme.text_element.g, theme.text_element.b], "the retained grid owns the linear element token");
    let retained_line_vertices = pass.line_draws.iter().map(|draw| draw.vertices.len()).sum::<usize>();
    assert_eq!(retained_line_vertices, 0, "the grid is one procedural scalar, never a camera-target-anchored LineList");
    retire_bridged_surface(&mut state);
}

#[test]
fn isolated_world_presentation_suppresses_chrome_and_input_but_retains_frame_work() {
    let mut scene = scene_with_selection("{}");
    scene.world_3d.as_mut().expect("world fixture").presentation_json = Some(
        serde_json::to_string(&World3dPresentation {
            show_grid: false,
            show_gizmo: false,
            interactive: false,
            viewport_mask: SceneViewportMask3d::Ellipse,
            clear: World3dPresentationClear::Transparent,
            source_aspect: Some(2.0),
        })
        .expect("typed isolated presentation"),
    );
    let bounds = Rect::new(0.0, 0.0, 396.0, 196.0);
    let mut state = World3dState::new("surface-icon".into(), "controller-icon".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    state.scene.retired_draws = Some(WorldDrawRegistry::default());
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = ui_wgpu::wgpu::Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    let mut ctx = ui_wgpu::wgpu::widgets::WidgetContext {
        draw: &mut draw, overlay: None, atlas: &mut atlas, icons: None, input: &mut input, theme: &theme,
        scroll_offsets: &mut scroll, collapsed_sections: &mut collapsed, open_selects: &mut selects,
        interaction_maps: None, pick_clip: None, viewport_height: bounds.h,
    };
    render_world_3d(&scene, bounds, &mut ctx, &mut state, &mut gpu, World3dShadowProfile::IconPng);
    let pass = draw.scene_passes.last().expect("isolated world scene pass");
    assert_eq!(pass.viewport_mask, SceneViewportMask3d::Ellipse);
    assert_eq!(pass.clear_color, None, "transparent presentation leaves the accepted card in place");
    assert!(pass.procedural_grid.is_none());
    let camera = state.camera.orbit.to_camera();
    let expected = ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.camera.projection_spec, bounds.w, bounds.w / 2.0).to_cols_array();
    assert_eq!(pass.view_proj, expected, "authored 2:1 source aspect survives the 396×196 physical stretch");
    assert!(input.staged_hits().is_empty(), "noninteractive worlds publish no pointer owner");
    assert_eq!(draw.layers.iter().map(|layer| layer.ui_instances.len() + layer.overlay_ui_instances.len()).sum::<usize>(), 0, "noninteractive isolated worlds paint no gizmo or fallback clear quad");
    assert!(gpu.take_cursor_wake().expect("wake authority remains open").is_some(), "retained scene work still wakes an isolated world");
    state.scene.retired_draws = None;
    retire_bridged_surface(&mut state);
}

/// 🎨️ LAW: the retained Basic-material appearance follows authored opacity and live theme tokens.
#[test]
fn reference_visual_state_uses_authored_opacity_and_live_theme() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🖼️reference-visual/🔣️.json")).expect("reference visual fixture");
    for row in fixture["stateCases"].as_array().unwrap() {
        if row["sourceProfileId"] != "world-image" || row["hidden"].as_bool().unwrap() {
            continue;
        }
        let theme = if row["themeId"] == "semio-dark" { ui_wgpu::wgpu::Theme::dark() } else { ui_wgpu::wgpu::Theme::light() };
        let id = "reference-authored-a";
        let reference = WorldReferenceRecord {
            id: Some(id.into()),
            url: Some("shared.png".into()),
            opacity: row["baseOpacity"].as_f64(),
            locked: row["locked"].as_bool(),
            hidden: row["hidden"].as_bool(),
            ..Default::default()
        };
        let mut state = World3dState::new("surface".into(), "controller".into());
        if row["selected"].as_bool().unwrap() {
            state.selection.selected_ids.push(id.into());
        }
        if row["hovered"].as_bool().unwrap() {
            state.selection.local_hover_id = Some(format!("reference:{id}"));
        }
        let actual = world_reference_visual(&state.selection, &reference, &theme);
        let expected = &row["expected"];
        assert!((actual.content_opacity - expected["contentOpacity"].as_f64().unwrap() as f32).abs() < 1e-6, "{} content opacity", row["id"]);
        let expected_background = match expected["backgroundSemantic"].as_str() {
            Some("activeBase") => [theme.selected.r, theme.selected.g, theme.selected.b, theme.selected.a],
            Some("hoverBase") => [theme.row_hover.r, theme.row_hover.g, theme.row_hover.b, theme.row_hover.a],
            _ => [0.0; 4],
        };
        assert_eq!(actual.background, expected_background, "{} background", row["id"]);
        let expected_outline = match expected["outlineSemantic"].as_str() {
            Some("activeBase") => Some([theme.celebrate[0].r, theme.celebrate[0].g, theme.celebrate[0].b, 1.0]),
            Some("accentSecondary") => Some([theme.celebrate[1].r, theme.celebrate[1].g, theme.celebrate[1].b, 0.9]),
            _ => None,
        };
        assert_eq!(actual.outline, expected_outline, "{} outline", row["id"]);
    }
}

/// 🎯️ Native picks and marquee retain exact analytic labels through the original scene bridge.
#[test]
fn world_native_analytic_component_picks_and_marquee_preserve_original_source() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎯️analytic-component-picking/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let expected = row["expected"].as_str().unwrap();
        let oracle = format!("{}.face.{}~{}~{}~{}", row["instance"].as_str().unwrap(), row["group"].as_u64().unwrap(), row["handle"].as_str().unwrap(), row["label"].as_str().unwrap(), row["revision"].as_str().unwrap());
        assert_eq!(expected, oracle);
        assert!(row["label"].as_str().unwrap().parse::<u64>().unwrap() > 9_007_199_254_740_992);
        let mut mesh = fixture["mesh"].clone();
        mesh["componentReferences"] = serde_json::json!({"face":[row["label"]]});
        let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
        let world = scene.world_3d.as_mut().unwrap();
        world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
        world.instances_json = serde_json::json!([{"id":row["instance"],"meshId":"analytic","componentSource":{"handle":row["handle"],"revision":row["revision"]}}]).to_string();
        world.camera_json = fixture["camera"].to_string();
        let mut state = World3dState::new("surface".into(), "controller".into());
        drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 400.0, 400.0));
        state.selection.granularity = "face".into();
        state.selection.active_object_id = Some(row["instance"].as_str().unwrap().into());
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..128 {
            match with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) {
                WorldInteractionStep::Complete => break,
                WorldInteractionStep::Pending => assert!(turn < 127),
                step => panic!("analytic registry {step:?}"),
            }
        }
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection, 10, WorldComponentPickPurpose::Select, 200.0, 200.0).unwrap();
        for turn in 0..WORLD_INTERACTION_OBJECT_CAPACITY + 8 {
            match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) {
                WorldInteractionStep::Complete => break,
                WorldInteractionStep::Pending => assert!(turn < WORLD_INTERACTION_OBJECT_CAPACITY + 7),
                step => panic!("analytic pick {step:?}"),
            }
        }
        assert_eq!(cursor.best.map(|hit| hit.id), Some(0));
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        for turn in 0..128 {
            if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; }
            assert!(turn < 127);
        }
        let actions = take_actions(&mut input);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, "interactionSelect");
        let args = actions[0].args.as_ref().unwrap();
        assert_eq!(args.get("domainId").and_then(|value| value.as_str()), Some("geometry"));
        let targets: serde_json::Value = serde_json::from_str(args.get("targets").and_then(|value| value.as_str()).unwrap()).unwrap();
        assert_eq!(targets[0]["id"].as_str(), Some(expected));
        let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 11, [0.0, 0.0]);
        assert!(gesture.push([400.0, 400.0]));
        let mut marquee = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 12, gesture).unwrap();
        for turn in 0..1024 {
            match with_world_step_context(1, |context| marquee.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context)) {
                WorldInteractionStep::Complete => break,
                WorldInteractionStep::Pending => assert!(turn < 1023),
                step => panic!("analytic marquee {step:?}"),
            }
        }
        assert_eq!(marquee.results.lens[0], 1);
        let mut publish = WorldComponentMarqueePublishJob::new(12, marquee.gesture, marquee.results, WorldComponentKind::Face, false, false);
        for turn in 0..1024 {
            if with_world_step_context(1, |context| publish.step(&state.session, &state.interaction, &state.scene, 12, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; }
            assert!(turn < 1023);
        }
        let actions = take_actions(&mut input);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, "interactionSelect");
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").and_then(|value| value.as_str()).unwrap()).unwrap();
        assert_eq!(targets[0]["id"].as_str(), Some(expected));
        println!("[DEBUG] nativeAnalyticSelection pickAndMarquee={expected} originalSceneBridge=true");
    }
}

/// 🖱️ Optimistic preview reads the original encoded target text and exact analytic source.
#[test]
fn world_component_contract_preview_reads_exact_target_text() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎯️analytic-component-picking/🔣️.json")).unwrap();
    let row = &fixture["cases"][0];
    let mut mesh = fixture["mesh"].clone(); mesh["componentReferences"] = serde_json::json!({"face":[row["label"]]});
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json = serde_json::json!([{"id":row["instance"],"meshId":"analytic","componentSource":{"handle":row["handle"],"revision":row["revision"]}}]).to_string();
    let mut state = World3dState::new("surface".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 400.0, 400.0));
    let targets = serde_json::json!([{"granularity":"face","id":row["expected"]}]).to_string();
    let action = ActionDescriptor { controller_id: "controller".into(), action: "interactionSelect".into(), args: Some(semio_framework::dsl_value!({"domainId":"geometry","targets":targets,"merge":"replace","method":"pick"})) };
    apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &action);
    assert_eq!(state.selection.component_ids, vec!["0".to_string()]);
    assert_eq!(state.selection.granularity, "face");
    let mut action = action;
    action.action = "interactionHover".into();
    apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene, &action);
    assert_eq!(state.selection.hovered_component_id.as_deref(), Some("0"));
    assert_eq!(state.selection.hovered_component_object_id.as_deref(), row["instance"].as_str());
    assert_eq!(state.selection.hovered_component_mode.as_deref(), Some("face"));
    println!("[DEBUG] componentPreview originalEncodedTargets=true exactSource=true group=0 hoverAndSelection=true");
}

/// 📍️ Native vertex picking skips surface samples and resolves the original analytic point label.
#[test]
fn world_native_analytic_vertex_pick_skips_surface_samples() {
    assert_native_original_vertex_selection(false);
}

/// 🎨️ Authored face and corner attributes retain unindexed original vertices through expansion.
#[test]
fn world_native_analytic_vertex_pick_survives_authored_expansion() {
    assert_native_original_vertex_selection(true);
}

fn assert_native_original_vertex_selection(expanded: bool) {
    let mut fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎯️analytic-vertex-picking/🔣️.json")).unwrap();
    if expanded { fixture["mesh"]["attributes"] = fixture["expandedAttributes"].clone(); }
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":fixture["mesh"]}]).to_string();
    world.instances_json = serde_json::json!([{"id":fixture["instance"],"meshId":"analytic","componentSource":{"handle":fixture["handle"],"revision":fixture["revision"]}}]).to_string();
    world.camera_json = fixture["camera"].to_string();
    let mut state = World3dState::new("surface".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 400.0, 400.0));
    state.selection.granularity = "vertex".into();
    let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
    for turn in 0..128 {
        match with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) {
            WorldInteractionStep::Complete => break,
            WorldInteractionStep::Pending => assert!(turn < 127),
            step => panic!("original vertex registry {step:?}"),
        }
    }
    let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection, 10, WorldComponentPickPurpose::Select, 200.0, 200.0).unwrap();
    for turn in 0..2048 {
        match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) {
            WorldInteractionStep::Complete => break,
            WorldInteractionStep::Pending => assert!(turn < 2047),
            step => panic!("original vertex pick {step:?}"),
        }
    }
    assert_eq!(cursor.best.map(|hit| hit.id), Some(0), "surface sample at the pointer is not an original vertex");
    let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    for turn in 0..128 {
        if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; }
        assert!(turn < 127);
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
    assert_eq!(targets, serde_json::json!([{"granularity":"vertex","id":fixture["expected"]}]));
    let start: [f32; 2] = serde_json::from_value(fixture["marquee"]["start"].clone()).unwrap();
    let end: [f32; 2] = serde_json::from_value(fixture["marquee"]["end"].clone()).unwrap();
    let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 11, start);
    assert!(gesture.push(end));
    let mut marquee = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 12, gesture).unwrap();
    for turn in 0..1024 {
        match with_world_step_context(1, |context| marquee.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context)) {
            WorldInteractionStep::Complete => break,
            WorldInteractionStep::Pending => assert!(turn < 1023),
            step => panic!("original vertex marquee {step:?}"),
        }
    }
    let groups: Vec<u32> = marquee.results.pages[0][..usize::from(marquee.results.lens[0])].iter().map(|result| match result.unwrap() { WorldMarqueeResult::Component { id, .. } => id, _ => panic!("original vertex component") }).collect();
    let expected_groups: Vec<u32> = serde_json::from_value(fixture["marquee"]["groups"].clone()).unwrap();
    assert_eq!(groups, expected_groups);
    let mut publish = WorldComponentMarqueePublishJob::new(12, marquee.gesture, marquee.results, WorldComponentKind::Vertex, false, false);
    for turn in 0..1024 {
        if with_world_step_context(1, |context| publish.step(&state.session, &state.interaction, &state.scene, 12, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; }
        assert!(turn < 1023);
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
    assert_eq!(targets, serde_json::json!([{"granularity":"vertex","id":fixture["expected"]}]));
    println!("[DEBUG] nativeVertexPickAndMarquee originalLabel={} surfaceSamplesExcluded=true authoredExpansion={expanded}", fixture["label"]);
}

/// 🤝️ Equal component groups retain their original object ownership through every shared merge mode.
#[test]
fn world_component_selection_merge_preserves_remaining_object_projection() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let mut mesh = fixture["mesh"].clone();mesh["componentReferences"] = serde_json::json!({"face":[fixture["source"]["label"]]});
    let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json = serde_json::Value::Array(fixture["objects"].as_array().unwrap().iter().map(|object|serde_json::json!({"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}})).collect()).to_string();
    let mut state = World3dState::new("surface".into(),"controller".into());
    drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
    for row in fixture["steps"].as_array().unwrap() {
        let targets = serde_json::Value::Array(row["incoming"].as_array().unwrap().iter().map(|index|serde_json::json!({"granularity":"face","id":fixture["targets"][index.as_u64().unwrap() as usize]})).collect()).to_string();
        let action = ActionDescriptor { controller_id:"controller".into(),action:"interactionSelect".into(),args:Some(semio_framework::dsl_value!({"domainId":"geometry","targets":targets,"merge":row["merge"].as_str().unwrap(),"method":"pick"})) };
        apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&action);
        let expected: Vec<String> = row["selected"].as_array().unwrap().iter().map(|index|fixture["targets"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
        let expected_active = row["active"].as_u64().map(|index|fixture["objects"][index as usize].as_str().unwrap());
        assert_eq!(state.gumball.gumball_selection_ids,expected,"original encoded targets {row}");
        let objects: Vec<String> = row["selected"].as_array().unwrap().iter().map(|index|fixture["objects"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
        assert_eq!(state.selection.selected_ids,objects,"plain original Scene object projection {row}");
        assert_eq!(state.selection.component_ids,serde_json::from_value::<Vec<String>>(row["groups"].clone()).unwrap(),"active original group projection {row}");
        assert_eq!(state.selection.active_object_id.as_deref(),expected_active,"active original object {row}");
    }
    println!("[DEBUG] scopedComponentMerges modes=4 sameGroupsDifferentObjects=true exactSource=true");
}

/// 🧵️ The original Scene bridge retains pure analytic wires for vertex and edge selection.
#[test]
fn world_native_analytic_wire_preserves_original_vertex_and_edge_picking() {
    assert_native_original_wire(false);
}

/// 🧵️ Edge-only source samples remain visible without inventing selectable topology vertices.
#[test]
fn world_native_analytic_wire_preserves_edge_only_scene() {
    assert_native_original_wire(true);
}

fn assert_native_original_wire(edge_only: bool) {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json")).unwrap();
    let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"wire","data":if edge_only {&fixture["edgeOnly"]["mesh"]}else{&fixture["mesh"]}}]).to_string();
    world.instances_json = serde_json::json!([{"id":fixture["instance"],"meshId":"wire","componentSource":{"handle":fixture["handle"],"revision":fixture["revision"]}}]).to_string();
    world.camera_json = fixture["camera"].to_string();
    let mut state = World3dState::new("surface".into(),"controller".into());
    drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
    let schema = state.scene.meshes.get("wire").expect("original wire is published").schema().unwrap();
    assert_eq!((schema.vertices,schema.edges,schema.indices),(if edge_only {fixture["edgeOnly"]["expected"]["renderVertices"].as_u64().unwrap() as u32}else{4},4,0));
    if edge_only {
        assert_eq!(schema.vertex_ids,0,"render samples never become topology vertices");
        let (min,max)=state.scene.meshes.get("wire").unwrap().aabb().unwrap();
        assert_eq!(min,serde_json::from_value::<[f32;3]>(fixture["edgeOnly"]["expected"]["boundsMin"].clone()).unwrap());
        assert_eq!(max,serde_json::from_value::<[f32;3]>(fixture["edgeOnly"]["expected"]["boundsMax"].clone()).unwrap());
    }
    for row in fixture["picks"].as_array().unwrap() {
        if edge_only && row["kind"]=="vertex" {continue;}
        state.selection.granularity = row["kind"].as_str().unwrap().into();
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..128 {
            match with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<127),
                step=>panic!("original wire registry {step:?}"),
            }
        }
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection,10,WorldComponentPickPurpose::Select,row["pointer"][0].as_f64().unwrap() as f32,row["pointer"][1].as_f64().unwrap() as f32).unwrap();
        for turn in 0..2048 {
            match with_world_step_context(1,|context|cursor.step(&state.session, &state.selection, &state.interaction, &state.scene,10,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<2047),
                step=>panic!("original wire pick {step:?}"),
            }
        }
        assert_eq!(cursor.best.map(|hit|hit.id),Some(row["group"].as_u64().unwrap() as u32));
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene,10).unwrap().unwrap();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        for turn in 0..128 {
            if with_world_step_context(1,|context|publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene,&mut plan,10,&mut input,context)).unwrap()==WorldInteractionStep::Complete {break;}
            assert!(turn<127);
        }
        let actions = take_actions(&mut input);assert_eq!(actions.len(),1);
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets,serde_json::json!([{"granularity":row["kind"],"id":row["expected"]}]));
    }
    if edge_only {
        state.selection.granularity = "vertex".into();
        state.selection.selection_targets.vertex = true;
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..2048 {
            match with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<2047),
                step=>panic!("edge-only vertex registry {step:?}"),
            }
        }
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection,10,WorldComponentPickPurpose::Select,fixture["picks"][0]["pointer"][0].as_f64().unwrap() as f32,fixture["picks"][0]["pointer"][1].as_f64().unwrap() as f32).unwrap();
        for turn in 0..2048 {
            match with_world_step_context(1,|context|cursor.step(&state.session, &state.selection, &state.interaction, &state.scene,10,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<2047),
                step=>panic!("edge-only vertex pick {step:?}"),
            }
        }
        assert!(cursor.best.is_none(),"packed render endpoints are not authored topology vertices");
        for (method,points) in [
            ("rectangle",vec![[0.0,0.0],[400.0,400.0]]),
            ("lasso",vec![[0.0,0.0],[400.0,0.0],[400.0,400.0],[0.0,400.0]]),
        ] {
            state.selection.selection_method = method.into();
            assert!(world_component_marquee_cursor_ids(&state,&points).is_empty(),"{method}: packed endpoints have no topology selection");
        }
        let mut lines = Vec::new();append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay,&ui_wgpu::wgpu::Theme::default(),&state.camera.orbit.to_camera(),state.session.bounds,&mut lines);
        assert_eq!(lines.len(),fixture["edgeOnly"]["expected"]["renderVertices"].as_u64().unwrap() as usize,"no vertex glyph is synthesized for an edge endpoint");
    }
    retire_bridged_surface(&mut state);
    println!("[DEBUG] originalAnalyticWire edgeOnly={edge_only} edges=4 triangles=0 exactSourcePicks=true noSyntheticTopologyVertices=true");
}

/// 🧷️ A pure point paints through the original native primitive owner in ordinary object mode.
#[test]
fn world_native_analytic_point_paints_without_component_targeting() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/🧫️fixtures/🎯️vertex-provenance/🔣️.json")).unwrap();
    let row=fixture["cases"].as_array().unwrap().iter().find(|row|row["kind"]=="point").unwrap();
    let point=serde_json::from_value::<[f32;3]>(row["points"][0].clone()).unwrap();
    let mut scene=scene_with_selection("{}");let world=scene.world_3d.as_mut().unwrap();
    world.meshes_json=serde_json::json!([{"id":"point","data":{"positions":point,"normals":[],"indices":[],"vertexIds":[0],"edgePositions":[],"edgeIds":[]}}]).to_string();
    world.instances_json=serde_json::json!([{"id":"point@vertex#0","meshId":"point"}]).to_string();
    world.camera_json=serde_json::json!({"position":[point[0],point[1],point[2]+4.0],"target":point,"up":[0,1,0],"fov":50}).to_string();
    world.selection_json=serde_json::json!({"granularity":"object","ids":[],"showEdges":false,"targets":{"mesh":true,"vertex":false,"edge":false,"face":false}}).to_string();
    let mut state=World3dState::new("surface-point".into(),"controller".into());let bounds=Rect::new(0.0,0.0,400.0,400.0);drive_scene_bridge(&mut state,&scene,bounds);
    let schema=state.scene.meshes.get("point").unwrap().schema().unwrap();assert_eq!((schema.vertices,schema.vertex_ids,schema.indices,schema.edges),(1,1,0,0));
    let mut lines=Vec::new();append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay,&ui_wgpu::wgpu::Theme::default(),&state.camera.orbit.to_camera(),bounds,&mut lines);
    assert!(!lines.is_empty(),"an unselected pure point has a primary native visual");
    for pair in lines.chunks_exact(2) {for axis in 0..3 {assert!(((pair[0].position[axis]+pair[1].position[axis])*0.5-point[axis]).abs()<1e-5);}}
    assert!(state.selection.selected_ids.is_empty());assert_eq!(state.selection.granularity,"object");retire_bridged_surface(&mut state);
    println!("[DEBUG] originalDefaultPointPaint points=1 objectMode=true noSelection=true noSyntheticTrianglesOrEdges=true");
}

/// 🪡️ A wire is primary geometry even when surface outlines and component targeting are disabled.
#[test]
fn world_native_analytic_wire_paints_without_surface_outlines() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json")).unwrap();
    for show_edges in [true,false] {
        let mut scene=scene_with_selection("{}");let world=scene.world_3d.as_mut().unwrap();
        world.meshes_json=serde_json::json!([{"id":"wire","data":fixture["mesh"]}]).to_string();world.instances_json=serde_json::json!([{"id":fixture["instance"],"meshId":"wire"}]).to_string();world.camera_json=fixture["camera"].to_string();
        world.selection_json=serde_json::json!({"granularity":"object","ids":[],"showEdges":show_edges,"targets":{"mesh":true,"vertex":false,"edge":false,"face":false}}).to_string();
        let mut state=World3dState::new("surface-wire".into(),"controller".into());let bounds=Rect::new(0.0,0.0,400.0,400.0);drive_scene_bridge(&mut state,&scene,bounds);
        let mut lines=Vec::new();append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay,&ui_wgpu::wgpu::Theme::default(),&state.camera.orbit.to_camera(),bounds,&mut lines);
        assert_eq!(lines.iter().flat_map(|line|line.position).collect::<Vec<_>>(),serde_json::from_value::<Vec<f32>>(fixture["mesh"]["edgePositions"].clone()).unwrap(),"primary original wire showEdges={show_edges}");
        retire_bridged_surface(&mut state);
    }
    println!("[DEBUG] originalDefaultWirePaint edges=4 showSurfaceOutlinesBoth=true noSelection=true independentThree=true");
}

/// 🖌️ Equal local face groups retain their original object when the selected overlay is scanned.
#[test]
fn world_face_selection_overlay_is_scoped_to_original_object() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let mut mesh = fixture["mesh"].clone();mesh["componentReferences"] = serde_json::json!({"face":[fixture["source"]["label"]]});
    let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json = serde_json::Value::Array(fixture["objects"].as_array().unwrap().iter().map(|object|serde_json::json!({"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}})).collect()).to_string();
    let mut state = World3dState::new("surface".into(),"controller".into());
    drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
    for row in fixture["steps"].as_array().unwrap() {
        let targets = serde_json::Value::Array(row["incoming"].as_array().unwrap().iter().map(|index|serde_json::json!({"granularity":"face","id":fixture["targets"][index.as_u64().unwrap() as usize]})).collect()).to_string();
        apply_world_action_preview(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&ActionDescriptor {controller_id:"controller".into(),action:"interactionSelect".into(),args:Some(semio_framework::dsl_value!({"domainId":"geometry","targets":targets,"merge":row["merge"].as_str().unwrap(),"method":"pick"}))});
        let mut scan = WorldFaceOverlayScan::default();let mut objects = std::collections::BTreeSet::new();
        for turn in 0..4096 {
            match scan.step(&state.selection, &state.gumball, &state.scene) {
                WorldFaceOverlayScanStep::Complete=>break,
                WorldFaceOverlayScanStep::Pending=>assert!(turn<4095),
                WorldFaceOverlayScanStep::Match(hit)=>{
                    assert_eq!(hit.category,2);
                    let draw = state.scene.draws.get(usize::from(hit.draw)).unwrap();
                    objects.insert(draw.instances[usize::from(hit.instance)].id.to_string());
                }
            }
        }
        let expected: std::collections::BTreeSet<String> = row["selected"].as_array().unwrap().iter().map(|index|fixture["objects"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
        assert_eq!(objects,expected,"original selected face overlays {row}");
    }
    println!("[DEBUG] originalFaceSelectionOverlay modes=4 equalGroupObjectsIsolated=true");
}

/// 🔐️ The original analytic face scan consumes the fresh Guest source lane and refuses ambiguity.
#[test]
fn world_face_selection_overlay_validates_fresh_guest_source_targets() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let refusals: serde_json::Value = serde_json::from_str(read_static("../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🎯️analytic-component-target.json")).unwrap();
    let mut rows: Vec<serde_json::Value> = fixture["steps"].as_array().unwrap().iter().map(|step|serde_json::json!({"name":step["merge"],"targets":step["selected"].as_array().unwrap().iter().map(|index|fixture["targets"][index.as_u64().unwrap() as usize].clone()).collect::<Vec<_>>(),"labels":[fixture["source"]["label"]],"selected":step["selected"],"active":step["active"],"groups":step["groups"]})).collect();
    rows.extend(refusals["overlayRefusals"].as_array().unwrap().iter().map(|row|serde_json::json!({"name":row["name"],"targets":row["targets"],"labels":row["labels"],"selected":[],"active":0,"groups":["0"]})));
    for row in rows {
        let mut mesh=fixture["mesh"].clone();mesh["componentReferences"]=serde_json::json!({"face":row["labels"]});
        let mut scene=scene_with_selection_and_domain("{}",Some(("geometry","object")));let world=scene.world_3d.as_mut().unwrap();
        world.meshes_json=serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
        world.instances_json=serde_json::Value::Array(fixture["objects"].as_array().unwrap().iter().map(|object|serde_json::json!({"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}})).collect()).to_string();
        world.selection_json=serde_json::json!({"ids":fixture["objects"],"gumballSelectionIds":row["targets"],"granularity":"face","componentIds":row["groups"],"activeObjectId":row["active"].as_u64().map(|index|fixture["objects"][index as usize].clone())}).to_string();
        let mut state=World3dState::new("surface".into(),"controller".into());drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
        assert_eq!(state.selection.selected_ids,serde_json::from_value::<Vec<String>>(fixture["objects"].clone()).unwrap(),"Guest ids retain their plain object contract");
        let mut scan=WorldFaceOverlayScan::default();let mut objects=std::collections::BTreeSet::new();
        for turn in 0..4096 {
            match scan.step(&state.selection, &state.gumball, &state.scene) {
                WorldFaceOverlayScanStep::Complete=>break,
                WorldFaceOverlayScanStep::Pending=>assert!(turn<4095),
                WorldFaceOverlayScanStep::Match(hit)=>{assert_eq!(hit.category,2);objects.insert(state.scene.draws.get(usize::from(hit.draw)).unwrap().instances[usize::from(hit.instance)].id.to_string());}
            }
        }
        let expected: std::collections::BTreeSet<String>=row["selected"].as_array().unwrap().iter().map(|index|fixture["objects"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
        assert_eq!(objects,expected,"fresh original Guest source lane {}",row["name"]);
        retire_bridged_surface(&mut state);
    }
    println!("[DEBUG] nativeFreshGuestFaceOverlay merges=4 sourceRefusals=9 plainIdsPreserved=true");
}

/// 🔄️ Changing a nonactive exact face invalidates the original overlay without changing active projections.
#[test]
fn world_face_selection_overlay_invalidates_nonactive_source_changes() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let row=&fixture["overlayLifecycle"];
    let mut mesh=fixture["mesh"].clone();mesh["faceIds"]=serde_json::json!([0,1]);mesh["componentReferences"]=serde_json::json!({"face":row["labels"]});
    let mut scene=scene_with_selection_and_domain("{}",Some(("geometry","object")));let world=scene.world_3d.as_mut().unwrap();
    world.meshes_json=serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json=serde_json::Value::Array(fixture["objects"].as_array().unwrap().iter().map(|object|serde_json::json!({"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}})).collect()).to_string();
    let mut state=World3dState::new("surface".into(),"controller".into());drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
    let mut stale:Option<WorldFaceOverlayMeshCursor>=None;
    for step in row["steps"].as_array().unwrap() {
        let selection=serde_json::json!({"ids":fixture["objects"],"gumballSelectionIds":step["targets"].as_array().unwrap().iter().map(|index|row["targets"][index.as_u64().unwrap() as usize].clone()).collect::<Vec<_>>(),"granularity":"face","componentIds":[0],"activeObjectId":fixture["objects"][0]}).to_string();
        let revision=state.session.interaction_revision;sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&selection);
        assert_eq!(state.session.interaction_revision,revision.wrapping_add(1),"exact original source selection advances once");
        sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&selection);assert_eq!(state.session.interaction_revision,revision.wrapping_add(1),"unchanged snapshot retains its revision");
        let equivalent=serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&selection).unwrap()).unwrap();sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&equivalent);assert_eq!(state.session.interaction_revision,revision.wrapping_add(1),"equivalent original selection text preserves its owner revision");
        assert_eq!(state.selection.selected_ids,serde_json::from_value::<Vec<String>>(fixture["objects"].clone()).unwrap());assert_eq!(state.selection.component_ids,vec!["0"]);assert_eq!(state.selection.active_object_id.as_deref(),fixture["objects"][0].as_str());
        if let Some(mut cursor)=stale.take() {
            let mut retired=false;
            for _ in 0..1024 {if matches!(cursor.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay),WorldFaceOverlayMeshStep::Stale){retired=true;break;}}
            assert!(retired&&cursor.terminal_is_empty(),"old original overlay retires before publication");
        }
        let mut scan=WorldFaceOverlayScan::default();let mut groups=std::collections::BTreeMap::new();
        for turn in 0..4096 {
            match scan.step(&state.selection, &state.gumball, &state.scene) {
                WorldFaceOverlayScanStep::Complete=>break,
                WorldFaceOverlayScanStep::Pending=>assert!(turn<4095),
                WorldFaceOverlayScanStep::Match(hit)=>{let draw=state.scene.draws.get(usize::from(hit.draw)).unwrap();let instance=&draw.instances[usize::from(hit.instance)];groups.insert(instance.id.to_string(),world_mesh_component_id(*state.scene.meshes.get(&draw.mesh_key).unwrap(),Mesh3dField::FaceIds,hit.triangle));}
            }
        }
        assert_eq!(groups.values().copied().collect::<Vec<_>>(),serde_json::from_value::<Vec<u32>>(step["groups"].clone()).unwrap());
        let mut cursor=WorldFaceOverlayMeshCursor::new("surface",9000+state.session.interaction_revision,state.session.interaction_revision,state.scene.draw_generation).unwrap();
        for _ in 0..1024 {assert!(matches!(cursor.step(&state.session, &state.selection, &state.gumball, &state.scene, &state.overlay),WorldFaceOverlayMeshStep::Pending));if matches!(cursor.owner,WorldPlaceholderOwner::Writing(_)){break;}}
        assert!(matches!(cursor.owner,WorldPlaceholderOwner::Writing(_)),"the original overlay owns a physical partial mesh before supersession");stale=Some(cursor);
    }
    if let Some(mut cursor)=stale {for _ in 0..1024 {if cursor.close_step(){break;}}assert!(cursor.terminal_is_empty());}
    retire_bridged_surface(&mut state);
    println!("[DEBUG] originalOverlayLifecycle nonactiveSourceChanges=2 activeProjectionPreserved=true staleRetirement=true");
}

/// 🧽️ A fresh empty original selection snapshot clears each component and hover projection.
#[test]
fn world_scene_selection_snapshot_clears_empty_components_and_hover() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let mut state = World3dState::new("surface".into(),"controller".into());
    let original = serde_json::json!({"ids":[fixture["objects"][0]],"gumballSelectionIds":[fixture["targets"][0]],"granularity":"face","componentIds":[0],"activeObjectId":fixture["objects"][0],"hoveredComponent":{"id":0,"objectId":fixture["objects"][0],"mode":"face"}}).to_string();
    sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&original);
    assert_eq!(state.selection.component_ids,vec!["0"]);assert!(state.selection.active_object_id.is_some());assert!(state.selection.hovered_component_id.is_some());
    let empty = fixture["steps"].as_array().unwrap().last().unwrap();
    let cleared = serde_json::json!({"ids":[],"granularity":"face","componentIds":empty["groups"],"activeObjectId":empty["active"],"hoveredComponent":null}).to_string();
    sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&cleared);
    assert!(state.selection.selected_ids.is_empty());
    assert!(state.gumball.gumball_selection_ids.is_empty());
    assert_eq!(state.selection.component_ids,serde_json::from_value::<Vec<String>>(empty["groups"].clone()).unwrap());
    assert_eq!(state.selection.active_object_id,empty["active"].as_str().map(str::to_string));
    assert_eq!(state.selection.hovered_component_id,fixture["clearedHover"]["id"].as_str().map(str::to_string));
    assert_eq!(state.selection.hovered_component_object_id,fixture["clearedHover"]["object"].as_str().map(str::to_string));
    assert_eq!(state.selection.hovered_component_mode,fixture["clearedHover"]["mode"].as_str().map(str::to_string));
    println!("[DEBUG] originalSceneSelectionClear componentIdsEmpty=true activeObjectEmpty=true hoverEmpty=true");
}

/// 📍️ The published Scene pivot belongs to the existing gumball target owner and clears with it.
#[test]
fn world_native_component_gumball_retains_delivered_pivot() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let mut state=World3dState::new("surface".into(),"controller".into());
    for row in fixture["gumball"]["pivotCases"].as_array().unwrap() {
        sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,&serde_json::json!({"ids":[fixture["objects"][0]],"gumballSelectionIds":[fixture["targets"][0]],"granularity":"face","gumballTarget":row["pivot"]}).to_string());
        let expected=std::array::from_fn(|axis|row["pivot"][axis].as_f64().unwrap() as f32);
        assert_eq!(state.gumball.gumball_target,Some(expected));
    }
    sync_world3d_scene_selection(&mut state.session, &mut state.selection, &mut state.gumball, &mut state.scene,"{}");assert_eq!(state.gumball.gumball_target,None);
    println!("[DEBUG] originalGumballScenePivot cases=2 freshEmptyClears=true independentThreePointer=true");
}

/// 🛠️ Original component gumball capture and publication retain the guest source target lane.
#[test]
fn world_native_component_gumball_retains_original_source_targets() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let row = &fixture["gumball"];let object = &fixture["objects"][row["object"].as_u64().unwrap() as usize];
    let expected = serde_json::Value::Array(row["ids"].as_array().unwrap().iter().map(|index|fixture["targets"][index.as_u64().unwrap() as usize].clone()).collect());
    for pivot_case in row["pivotCases"].as_array().unwrap() {
    for phase in row["phases"].as_array().unwrap() {
        let mut mesh = fixture["mesh"].clone();mesh["componentReferences"] = serde_json::json!({"face":[fixture["source"]["label"]]});
        let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
        let world = scene.world_3d.as_mut().unwrap();
        world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
        world.instances_json = serde_json::json!([{"id":object,"meshId":"analytic","selected":true,"componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}}]).to_string();
        world.camera_json = serde_json::json!({"position":[0,0,4],"target":[0,0,0],"up":[0,1,0],"fov":50}).to_string();
        world.selection_json = serde_json::json!({"ids":[object],"granularity":row["mode"],"componentIds":[0],"activeObjectId":object,"gumballSelectionIds":expected,"gumballTarget":pivot_case["pivot"],"transformMode":"move"}).to_string();
        let mut state = World3dState::new("surface".into(),"controller".into());
        drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
        set_world3d_declared_actions(&mut state.session,&[TRANSLATE_SELECTION_ACTION_ID.into()]);
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..128 {
            match with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<127),
                step=>panic!("component gumball registry {step:?}"),
            }
        }
        let mut pick = WorldGumballPickCursor::new(&state.session, &state.selection,12,pivot_case["pointer"][0].as_f64().unwrap() as f32,pivot_case["pointer"][1].as_f64().unwrap() as f32);
        for turn in 0..2048 {
            match with_world_step_context(1,|context|pick.step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene,12,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<2047),
                step=>panic!("component gumball capture {step:?}"),
            }
        }
        let mut gesture = pick.finish(&state.session, &state.gumball,12).unwrap_or_else(|_|panic!("current component gumball capture")).expect("actual component gumball handle");
        assert_eq!(gesture.handle,GumballHandle::MoveX);
        assert_eq!([gesture.pivot.x,gesture.pivot.y,gesture.pivot.z],std::array::from_fn::<_,3,_>(|axis|pivot_case["pivot"][axis].as_f64().unwrap() as f32));
        assert_eq!(gesture.begin_update(13,pivot_case["movedPointer"][0].as_f64().unwrap() as f32,pivot_case["movedPointer"][1].as_f64().unwrap() as f32),WorldInteractionStep::Pending);
        for turn in 0..128 {
            match gesture.update_step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<127),
                step=>panic!("component gumball motion {step:?}"),
            }
        }
        assert!((gesture.translate.x-row["delta"][0].as_f64().unwrap() as f32).abs()<1e-4);
        let stage = match phase.as_str().unwrap() {"once"=>WorldGumballPhase::Once,"stream"=>WorldGumballPhase::Stream,"commit"=>WorldGumballPhase::Commit,"abort"=>{gesture.translate=Vec3::ZERO;WorldGumballPhase::Abort(WorldCancelReason::Blur)},_=>unreachable!()};
        let action = world_gumball_publish(&state,gesture,stage).expect("original prepared gumball action");
        let args = action.args.as_ref().unwrap();
        assert_eq!(args.get("mode").and_then(|value|value.as_str()),row["mode"].as_str());
        let ids: Vec<&str> = args.get("ids").unwrap().as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect();
        assert_eq!(ids,expected.as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<Vec<_>>());
        retire_bridged_surface(&mut state);
    }
    }
    println!("[DEBUG] nativeComponentGumball phases=4 pivots=2 actualCaptureAndCommit=true exactOriginalSourceTargets=true");
}

/// 🕹️ Component gumballs use the original event authority through capture, motion, release and cancellation.
#[test]
fn world_native_component_gumball_uses_original_event_authority() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let row=&fixture["gumball"];let object=&fixture["objects"][row["object"].as_u64().unwrap() as usize];
    let targets:Vec<String>=row["ids"].as_array().unwrap().iter().map(|index|fixture["targets"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
    for case in row["authorityCases"].as_array().unwrap() {
        let mut mesh=fixture["mesh"].clone();mesh["componentReferences"]=serde_json::json!({"face":[fixture["source"]["label"]]});
        let mut scene=scene_with_selection_and_domain("{}",Some(("geometry","object")));let world=scene.world_3d.as_mut().unwrap();
        world.meshes_json=serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
        world.instances_json=serde_json::json!([{"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}}]).to_string();
        world.camera_json=serde_json::json!({"position":[0,0,4],"target":[0,0,0],"up":[0,1,0],"fov":50}).to_string();
        world.selection_json=serde_json::json!({"ids":[object],"granularity":row["mode"],"componentIds":[0],"activeObjectId":object,"gumballSelectionIds":targets,"gumballTarget":row["pivot"],"transformMode":"move","gumballLiveDispatch":case["live"]}).to_string();
        let mut state=World3dState::new("surface".into(),"controller".into());drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));set_world3d_declared_actions(&mut state.session,&[TRANSLATE_SELECTION_ACTION_ID.into()]);
        let mut input=ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();let mut actions=Vec::new();
        for event in 0..4 {
            let finish=case["finish"].as_str().unwrap();if event==3&&finish=="release" {continue;}
            let point=if event==0 {&row["pointer"]} else {&row["movedPointer"]};
            let intent=WorldInteractionIntent {phase:if event==1 {WorldInteractionPhase::PointerMove} else if event==2&&finish!="release" {WorldInteractionPhase::Cancel} else {WorldInteractionPhase::PointerButton},generation:0,x:point[0].as_f64().unwrap() as f32,y:point[1].as_f64().unwrap() as f32,dx:0.0,dy:0.0,delta:0.0,button:if event==2&&finish!="release" {if finish=="blur" {WorldCancelReason::Blur as i16} else {WorldCancelReason::CaptureLost as i16}} else {0},down:event<2,shift:false,ctrl:false,alt:false,meta:false};
            let generation=enqueue_world3d_event(&mut state.selection, &mut state.authority,intent).expect("original component event admission");
            let mut complete=false;
            for _ in 0..4096 {
                match with_world_step_context(1,|context|step_world3d_interaction(&mut state,generation,&mut input,context)) {
                    WorldInteractionAuthorityStep::Pending=>{},
                    WorldInteractionAuthorityStep::Complete|WorldInteractionAuthorityStep::Idle=>{complete=true;break;},
                    step=>panic!("original component event {event} {step:?}"),
                }
            }
            assert!(complete,"each original event terminates under one-unit grants");
            if event==0 {assert!(state.authority.interaction_authority.as_ref().unwrap().gumball.is_some(),"component mode captures its original gumball");}
            actions.extend(take_actions(&mut input).into_iter().filter(|action|action.action==TRANSLATE_SELECTION_ACTION_ID));
        }
        assert_eq!(actions.len(),case["phases"].as_array().unwrap().len());
        for (action,phase) in actions.iter().zip(case["phases"].as_array().unwrap()) {
            let args=action.args.as_ref().unwrap();assert_eq!(args.get("mode").and_then(|value|value.as_str()),row["mode"].as_str());assert_eq!(args.get("phase").and_then(|value|value.as_str()),phase.as_str());
            assert_eq!(args.get("ids").unwrap().as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<Vec<_>>(),targets.iter().map(String::as_str).collect::<Vec<_>>());
        }
        begin_world3d_interaction_close(&mut state.authority);
        for _ in 0..4096 {if with_world_step_context(1,|context|close_world3d_interaction_step(&mut state.authority,&mut input,context)){break;}}
        assert!(world3d_interaction_terminal_is_empty(&state.authority));while !input.close_step().unwrap(){}assert!(input.terminal_is_empty());retire_bridged_surface(&mut state);
    }
    println!("[DEBUG] originalComponentGumballAuthority cases=4 captureMotionReleaseBlurCaptureLost=true exactSourceTargets=true boundedRetirement=true");
}

/// 🧮️ One object's source components consume original action credits rather than an arbitrary sixty-four object limit.
#[test]
fn world_native_component_gumball_captures_wide_source_selection() {
    let fixture:serde_json::Value=serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();let row=&fixture["gumball"];
    let count=row["wideSelection"]["count"].as_u64().unwrap() as usize;let first=row["wideSelection"]["labelStart"].as_str().unwrap().parse::<u64>().unwrap();let object=fixture["objects"][0].as_str().unwrap();
    let labels:Vec<String>=(0..count).map(|index|(first+index as u64).to_string()).collect();let targets:Vec<String>=labels.iter().enumerate().map(|(index,label)|format!("{object}.face.{index}~{}~{label}~{}",fixture["source"]["handle"].as_str().unwrap(),fixture["source"]["revision"].as_str().unwrap())).collect();
    let mut mesh=fixture["mesh"].clone();mesh["indices"]=serde_json::json!((0..count).flat_map(|_|[0,1,2]).collect::<Vec<_>>());mesh["faceIds"]=serde_json::json!((0..count as u32).collect::<Vec<_>>());mesh["componentReferences"]=serde_json::json!({"face":labels});
    let mut scene=scene_with_selection_and_domain("{}",Some(("geometry","object")));let world=scene.world_3d.as_mut().unwrap();world.meshes_json=serde_json::json!([{"id":"analytic","data":mesh}]).to_string();world.instances_json=serde_json::json!([{"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}}]).to_string();world.camera_json=serde_json::json!({"position":[0,0,4],"target":[0,0,0],"up":[0,1,0],"fov":50}).to_string();world.selection_json=serde_json::json!({"ids":[object],"granularity":"face","gumballSelectionIds":targets,"gumballTarget":row["pivot"],"transformMode":"move"}).to_string();
    let mut state=World3dState::new("surface".into(),"controller".into());drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));set_world3d_declared_actions(&mut state.session,&[TRANSLATE_SELECTION_ACTION_ID.into()]);
    let mut registry=WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);for _ in 0..4096 {if with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context))==WorldInteractionStep::Complete {break;}}
    let mut pick=WorldGumballPickCursor::new(&state.session, &state.selection,12,row["pointer"][0].as_f64().unwrap() as f32,row["pointer"][1].as_f64().unwrap() as f32);
    for _ in 0..16384 {if with_world_step_context(1,|context|pick.step(&state.session, &state.camera, &state.gumball, &state.interaction, &state.scene,12,context))==WorldInteractionStep::Complete {break;}}
    let mut gesture=pick.finish(&state.session, &state.gumball,12).unwrap_or_else(|_|panic!("current wide original source selection")).expect("actual wide gumball");assert_eq!(usize::from(gesture.selected_len),count);assert_eq!(usize::from(gesture.selected_bytes),targets.iter().map(String::len).sum::<usize>());
    gesture.translate=Vec3::new(1.0,0.0,0.0);let action=world_gumball_publish(&state,gesture,WorldGumballPhase::Once).expect("wide original bounded action");let args=action.args.as_ref().unwrap();assert_eq!(args.get("mode").and_then(|value|value.as_str()),Some("face"));assert_eq!(args.get("ids").unwrap().as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect::<Vec<_>>(),targets.iter().map(String::as_str).collect::<Vec<_>>());retire_bridged_surface(&mut state);
    println!("[DEBUG] originalWideComponentGumball sourceTargets={count} exactU64=true physicalByteCredits=true independentLodash=true");
}

/// 🌊️ A live component gesture keeps its captured targets across a real guest mesh/source refresh.
#[test]
fn world_native_component_gumball_survives_live_guest_source_refresh() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let row = &fixture["gumball"];
    let refresh = &row["liveRefresh"];
    let object = &fixture["objects"][row["object"].as_u64().unwrap() as usize];
    let targets: Vec<String> = row["ids"].as_array().unwrap().iter().map(|index| fixture["targets"][index.as_u64().unwrap() as usize].as_str().unwrap().to_string()).collect();
    let mut mesh = fixture["mesh"].clone();
    mesh["componentReferences"] = serde_json::json!({"face":[fixture["source"]["label"]]});
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
    world.instances_json = serde_json::json!([{"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":fixture["source"]["revision"]}}]).to_string();
    world.camera_json = refresh["camera"].to_string();
    world.selection_json = serde_json::json!({"ids":[object],"granularity":row["mode"],"componentIds":[0],"activeObjectId":object,"gumballSelectionIds":targets,"gumballTarget":row["pivot"],"transformMode":"move","gumballLiveDispatch":true}).to_string();
    let bounds = Rect::new(0.0, 0.0, refresh["viewport"][0].as_f64().unwrap() as f32, refresh["viewport"][1].as_f64().unwrap() as f32);
    let mut state = World3dState::new("surface".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    set_world3d_declared_actions(&mut state.session, &[TRANSLATE_SELECTION_ACTION_ID.into()]);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut actions = Vec::new();
    for event in 0..refresh["pointers"].as_array().unwrap().len() {
        let point = &refresh["pointers"][event];
        let intent = WorldInteractionIntent { phase: if event == 0 || event == 3 { WorldInteractionPhase::PointerButton } else { WorldInteractionPhase::PointerMove }, generation: 0, x: point[0].as_f64().unwrap() as f32, y: point[1].as_f64().unwrap() as f32, dx: 0.0, dy: 0.0, delta: 0.0, button: 0, down: event < 3, shift: false, ctrl: false, alt: false, meta: false };
        let generation = enqueue_world3d_event(&mut state.selection, &mut state.authority, intent).expect("original live-refresh event admission");
        let mut complete = false;
        for _ in 0..4096 {
            match with_world_step_context(1, |context| step_world3d_interaction(&mut state, generation, &mut input, context)) {
                WorldInteractionAuthorityStep::Pending => {}
                WorldInteractionAuthorityStep::Complete | WorldInteractionAuthorityStep::Idle => { complete = true; break; }
                step => panic!("original live-refresh event {event} stopped at {step:?}"),
            }
        }
        assert!(complete, "original live-refresh event {event} terminates under one-unit grants");
        if event == 0 { assert!(state.authority.interaction_authority.as_ref().unwrap().gumball.is_some(), "original pointer captures a real component gumball"); }
        actions.extend(take_actions(&mut input).into_iter().filter(|action| action.action == TRANSLATE_SELECTION_ACTION_ID));
        if event == refresh["refreshAfter"].as_u64().unwrap() as usize {
            assert_eq!(actions.len(), 1, "the first stream precedes the actual guest publication");
            let generation = state.scene.geometry_generation;
            let revision = state.session.interaction_revision;
            for (index, position) in mesh["positions"].as_array_mut().unwrap().iter_mut().enumerate() {
                *position = serde_json::Value::from(position.as_f64().unwrap() + refresh["positions"][event][index % 3].as_f64().unwrap());
            }
            let world = scene.world_3d.as_mut().unwrap();
            world.meshes_json = serde_json::json!([{"id":"analytic","data":mesh}]).to_string();
            world.instances_json = serde_json::json!([{"id":object,"meshId":"analytic","componentSource":{"handle":fixture["source"]["handle"],"revision":refresh["revision"]}}]).to_string();
            drive_scene_bridge(&mut state, &scene, bounds);
            assert!(state.scene.geometry_generation > generation, "a real resident mesh was retired and replaced");
            assert!(state.session.interaction_revision > revision, "the actual guest refresh advanced the original view revision");
            assert!(state.bridge.scene_bridge_retired.is_none(), "the superseded snapshot lease finished bounded retirement");
            assert_eq!(state.gumball.gumball_selection_ids, targets, "the guest preserves the captured start selection");
            assert!(state.authority.interaction_authority.as_ref().unwrap().gumball.is_some(), "guest source publication retains the live gesture");
            let revision = state.session.interaction_revision;
            sync_world3d_state(&mut state, &scene, bounds);
            assert_eq!(with_world_step_context(64, |context| step_world3d_scene_bridge(&mut state, context)), World3dSceneBridgeStep::Idle, "an unchanged guest source stages no new bridge");
            assert_eq!(state.session.interaction_revision, revision, "an unchanged guest source does not advance view authority");
        }
    }
    let expected = refresh["dispatches"].as_array().unwrap();
    assert_eq!(actions.len(), expected.len(), "the refreshed gesture streams twice and commits its remaining tail");
    let mut offset = [0.0; 3];
    for (action, expected) in actions.iter().zip(expected) {
        let args = action.args.as_ref().unwrap();
        eprintln!("[DEBUG] original live refresh actual={args:?} expected={expected}");
        assert_eq!(args.get("mode").and_then(|value| value.as_str()), row["mode"].as_str());
        assert_eq!(args.get("phase").and_then(|value| value.as_str()), expected["phase"].as_str());
        assert_eq!(args.get("ids").unwrap().as_array().unwrap().iter().map(|value| value.as_str().unwrap()).collect::<Vec<_>>(), targets.iter().map(String::as_str).collect::<Vec<_>>());
        for (axis, key) in ["dx", "dy", "dz"].iter().enumerate() {
            let delta = args.get(*key).unwrap().as_f64().unwrap();
            assert!((delta - expected[*key].as_f64().unwrap()).abs() < 1e-4, "{key} is the incremental guest effect");
            offset[axis] += delta;
        }
    }
    for (axis, value) in offset.iter().enumerate() { assert!((value - refresh["offset"][axis].as_f64().unwrap()).abs() < 1e-4); }
    begin_world3d_interaction_close(&mut state.authority);
    for _ in 0..4096 { if with_world_step_context(1, |context| close_world3d_interaction_step(&mut state.authority, &mut input, context)) { break; } }
    assert!(world3d_interaction_terminal_is_empty(&state.authority));
    while !input.close_step().unwrap() {}
    assert!(input.terminal_is_empty());
    retire_bridged_surface(&mut state);
    println!("[DEBUG] originalLiveComponentRefresh realMeshRetirement=true newSourceRevision=true pinnedStartTargets=true phases=stream/stream/commit incremental=.25/.5/.25 independentThree=true boundedClose=true");
}

/// 🖱️ Primary points and wires remain selectable and hoverable in the original object domain.
#[test]
fn world_native_primary_geometry_object_pick_uses_original_domain() {
    let wire: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json")).unwrap();
    let points: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/🧫️fixtures/🎯️vertex-provenance/🔣️.json")).unwrap();
    let point = &points["cases"].as_array().unwrap().iter().find(|row|row["kind"] == "point").unwrap()["points"][0];
    let point_mesh = serde_json::json!({"positions":point,"indices":[],"vertexIds":[0],"edgePositions":[],"edgeIds":[]});
    let point_camera = serde_json::json!({"position":[point[0],point[1],point[2].as_f64().unwrap()+4.0],"target":point,"up":[0,1,0],"fov":50});
    let pointer = serde_json::json!([200,200]);
    for (name,mesh,camera,pointer,object) in [
        ("wire",&wire["mesh"],&wire["camera"],&wire["picks"][1]["pointer"],wire["instance"].as_str().unwrap()),
        ("edge-only",&wire["edgeOnly"]["mesh"],&wire["camera"],&wire["picks"][1]["pointer"],wire["instance"].as_str().unwrap()),
        ("point",&point_mesh,&point_camera,&pointer,"point@vertex#0"),
    ] {
        let mut scene = scene_with_selection_and_domain("{}",Some(("geometry","object")));
        let world = scene.world_3d.as_mut().unwrap();
        world.meshes_json = serde_json::json!([{"id":name,"data":mesh}]).to_string();
        world.instances_json = serde_json::json!([{"id":object,"meshId":name}]).to_string();
        world.camera_json = camera.to_string();
        world.selection_json = serde_json::json!({"granularity":"object","targets":{"mesh":true,"vertex":false,"edge":false,"face":false},"showEdges":false}).to_string();
        let mut state = World3dState::new(format!("surface-{name}"),"controller".into());
        drive_scene_bridge(&mut state,&scene,Rect::new(0.0,0.0,400.0,400.0));
        let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
        for turn in 0..4096 {
            match with_world_step_context(1,|context|registry.step(&mut state.session, &mut state.interaction, &mut state.scene,context)) {
                WorldInteractionStep::Complete=>break,
                WorldInteractionStep::Pending=>assert!(turn<4095),
                step=>panic!("original primary registry {name} {step:?}"),
            }
        }
        for purpose in [WorldRayPickPurpose::Instance,WorldRayPickPurpose::Hover] {
            let mut cursor = WorldRayPickCursor::new(&state.session, &state.camera,10,purpose,pointer[0].as_f64().unwrap() as f32,pointer[1].as_f64().unwrap() as f32).unwrap();
            for turn in 0..4096 {
                match with_world_step_context(1,|context|cursor.step(&state.session, &state.selection, &state.interaction, &state.scene,10,context)) {
                    WorldInteractionStep::Complete=>break,
                    WorldInteractionStep::Pending=>assert!(turn<4095),
                    step=>panic!("original primary pick {name} {step:?}"),
                }
            }
            let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene,10).unwrap().expect("original primary plan");
            let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
            for turn in 0..256 {
                if with_world_step_context(1,|context|publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene,&mut plan,10,&mut input,context)).unwrap()==WorldInteractionStep::Complete {break;}
                assert!(turn<255);
            }
            let actions = take_actions(&mut input);assert_eq!(actions.len(),1);
            let args = actions[0].args.as_ref().unwrap();
            let targets: serde_json::Value = serde_json::from_str(args.get("targets").unwrap().as_str().unwrap()).unwrap();
            assert_eq!(targets,serde_json::json!([{"granularity":"object","id":object}]),"{name}: original primary object {purpose:?}");
            while !input.close_step().unwrap() {}
            assert!(input.terminal_is_empty());
        }
        retire_bridged_surface(&mut state);
    }
    println!("[DEBUG] originalPrimaryObjectPick cases=3 selectAndHover=true objectMode=true noSyntheticTopology=true independentThree=true");
}

/// 🪡️ Original object picking orders overlapping sloping wires by their actual closest point.
#[test]
fn world_native_overlapping_wires_pick_original_closest_depth() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let law = &fixture["gumball"]["overlappingWirePick"];
    let cases = fixture["gumball"]["raySegmentClosestCases"].as_array().unwrap();
    let rows = law["caseNames"].as_array().unwrap().iter().map(|name| cases.iter().find(|row| row["name"] == *name).unwrap()).collect::<Vec<_>>();
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::Value::Array(rows.iter().enumerate().map(|(index, row)| serde_json::json!({"id":format!("wire-{index}"),"data":{"positions":[],"indices":[],"vertexIds":[],"edgePositions":row["a"].as_array().unwrap().iter().chain(row["b"].as_array().unwrap()).collect::<Vec<_>>(),"edgeIds":[0]}})).collect()).to_string();
    world.instances_json = serde_json::Value::Array(law["objects"].as_array().unwrap().iter().enumerate().map(|(index, id)| serde_json::json!({"id":id,"meshId":format!("wire-{index}")})).collect()).to_string();
    world.camera_json = law["camera"].to_string();
    world.selection_json = serde_json::json!({"granularity":"object","targets":{"mesh":true,"vertex":false,"edge":false,"face":false}}).to_string();
    let viewport: [f32; 4] = serde_json::from_value(law["viewport"].clone()).unwrap();
    let mut state = World3dState::new("surface-overlapping-wires".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, Rect::new(viewport[0], viewport[1], viewport[2], viewport[3]));
    let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
    for turn in 0..4096 { match with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original overlapping registry {step:?}") } }
    for purpose in [WorldRayPickPurpose::Instance, WorldRayPickPurpose::Hover] {
        let mut cursor = WorldRayPickCursor::new(&state.session, &state.camera, 10, purpose, law["pointer"][0].as_f64().unwrap() as f32, law["pointer"][1].as_f64().unwrap() as f32).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original overlapping pick {step:?}") } }
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        for turn in 0..256 { if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 255); }
        let actions = take_actions(&mut input);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, if purpose == WorldRayPickPurpose::Hover { "interactionHover" } else { "interactionSelect" });
        if purpose == WorldRayPickPurpose::Instance { assert_eq!(actions[0].args.as_ref().unwrap().get("merge").and_then(|value| value.as_str()), Some("replace")); }
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":law["granularity"],"id":law["selectedObject"]}]), "original {purpose:?} chooses the actual near edge rather than its far midpoint");
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
    }
    state.selection.granularity = "edge".into();
    state.selection.selection_targets.edge = true;
    for purpose in [WorldComponentPickPurpose::Select, WorldComponentPickPurpose::Hover] {
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection, 10, purpose, law["pointer"][0].as_f64().unwrap() as f32, law["pointer"][1].as_f64().unwrap() as f32).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original overlapping component pick {step:?}") } }
        let best = cursor.best.expect("original overlapping component hit");
        assert_eq!(state.interaction.interaction_objects.resolve(best.object).unwrap().id.as_str(), law["selectedObject"].as_str().unwrap());
        assert!((best.primary - 1.0).abs() < 1e-5);
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        for turn in 0..256 { if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 255); }
        let actions = take_actions(&mut input);
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, if purpose == WorldComponentPickPurpose::Hover { "interactionHover" } else { "interactionSelect" });
        if purpose == WorldComponentPickPurpose::Select { assert_eq!(actions[0].args.as_ref().unwrap().get("merge").and_then(|value| value.as_str()), Some("replace")); }
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":"edge","id":format!("{}.edge.0", law["selectedObject"].as_str().unwrap())}]), "original component {purpose:?} publishes the actual near edge");
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
        while !cursor.close_step() {}
    }
    retire_bridged_surface(&mut state);
    eprintln!("[DEBUG] originalOverlappingWirePick instanceAndHover=true componentSelectAndHover=true actualClosestDepth=true noSyntheticVertices=true");
}

/// 🪟️ Original object and component cursors pick the visible portion of an authored clipped edge.
#[test]
fn world_native_clipped_wire_pick_preserves_original_visible_depth() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-selection-merges/🔣️.json")).unwrap();
    let law = &fixture["gumball"]["clippedWirePick"];
    let mut scene = scene_with_selection_and_domain("{}", Some(("geometry", "object")));
    let world = scene.world_3d.as_mut().unwrap();
    world.meshes_json = serde_json::json!([{"id":"clipped-wire","data":{"positions":[],"indices":[],"vertexIds":[],"edgePositions":law["a"].as_array().unwrap().iter().chain(law["b"].as_array().unwrap()).collect::<Vec<_>>(),"edgeIds":[law["edgeId"]]}}]).to_string();
    world.instances_json = serde_json::json!([{"id":law["object"],"meshId":"clipped-wire"}]).to_string();
    world.camera_json = law["camera"].to_string();
    world.selection_json = serde_json::json!({"granularity":"object","targets":{"mesh":true,"vertex":false,"edge":true,"face":false}}).to_string();
    let viewport: [f32; 4] = serde_json::from_value(law["viewport"].clone()).unwrap();
    let mut state = World3dState::new("surface-clipped-wire".into(), "controller".into());
    drive_scene_bridge(&mut state, &scene, Rect::new(viewport[0], viewport[1], viewport[2], viewport[3]));
    let camera = state.camera.orbit.to_camera();
    assert_eq!(camera.near, law["camera"]["near"].as_f64().unwrap() as f32);
    assert_eq!(camera.far, law["camera"]["far"].as_f64().unwrap() as f32);
    let mut registry = WorldInteractionRegistryBuildCursor::new(state.session.interaction_revision);
    for turn in 0..4096 { match with_world_step_context(1, |context| registry.step(&mut state.session, &mut state.interaction, &mut state.scene, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original clipped registry {step:?}") } }
    let pointer = [law["pointer"][0].as_f64().unwrap() as f32, law["pointer"][1].as_f64().unwrap() as f32];
    for purpose in [WorldRayPickPurpose::Instance, WorldRayPickPurpose::Hover] {
        let mut cursor = WorldRayPickCursor::new(&state.session, &state.camera, 10, purpose, pointer[0], pointer[1]).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original clipped object pick {step:?}") } }
        assert!((cursor.best.unwrap().distance - law["closestDepth"].as_f64().unwrap() as f32).abs() < 1e-5);
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        for turn in 0..256 { if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 255); }
        let actions = take_actions(&mut input); assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, if purpose == WorldRayPickPurpose::Hover { "interactionHover" } else { "interactionSelect" });
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":"object","id":law["object"]}]));
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
    }
    state.selection.granularity = "edge".into();
    for purpose in [WorldComponentPickPurpose::Select, WorldComponentPickPurpose::Hover] {
        let mut cursor = WorldComponentPickCursor::new(&state.session, &state.camera, &state.selection, 10, purpose, pointer[0], pointer[1]).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| cursor.step(&state.session, &state.selection, &state.interaction, &state.scene, 10, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original clipped component pick {step:?}") } }
        let best = cursor.best.unwrap(); assert_eq!(best.id, law["edgeId"].as_u64().unwrap() as u32);
        assert!((best.primary - law["closestDepth"].as_f64().unwrap() as f32).abs() < 1e-5);
        let mut plan = cursor.finish_plan(&state.session, &state.selection, &state.interaction, &state.scene, 10).unwrap().unwrap();
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        for turn in 0..256 { if with_world_step_context(1, |context| publish_world3d_plan_step(&mut state.session, &mut state.camera, &mut state.selection, &mut state.scene, &mut plan, 10, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 255); }
        let actions = take_actions(&mut input); assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].action, if purpose == WorldComponentPickPurpose::Hover { "interactionHover" } else { "interactionSelect" });
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":"edge","id":format!("{}.edge.{}", law["object"].as_str().unwrap(), law["edgeId"].as_u64().unwrap())}]));
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
        while !cursor.close_step() {}
    }
    for rectangle in law["marquees"].as_array().unwrap() {
        let point = |value: &serde_json::Value| [value[0].as_f64().unwrap() as f32, value[1].as_f64().unwrap() as f32];
        let mut gesture = WorldMarqueeGesture::new(state.session.interaction_revision, 11, point(&rectangle["points"][0]));
        assert!(gesture.push(point(&rectangle["points"][1])));
        let mut marquee = WorldMarqueePickCursor::new(&state.session, &state.camera, &state.selection, 12, gesture).unwrap();
        for turn in 0..4096 { match with_world_step_context(1, |context| marquee.step(&state.session, &state.selection, &state.interaction, &state.scene, 12, context)) { WorldInteractionStep::Complete => break, WorldInteractionStep::Pending => assert!(turn < 4095), step => panic!("original clipped marquee {step:?}") } }
        assert_eq!(marquee.results.lens[0], 1, "{} includes the visible clipped segment", rectangle["name"]);
        let mut publish = WorldComponentMarqueePublishJob::new(12, marquee.gesture, marquee.results, WorldComponentKind::Edge, false, false);
        let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
        for turn in 0..1024 { if with_world_step_context(1, |context| publish.step(&state.session, &state.interaction, &state.scene, 12, &mut input, context)).unwrap() == WorldInteractionStep::Complete { break; } assert!(turn < 1023); }
        let actions = take_actions(&mut input); assert_eq!(actions.len(), 1); assert_eq!(actions[0].action, "interactionSelect");
        let targets: serde_json::Value = serde_json::from_str(actions[0].args.as_ref().unwrap().get("targets").unwrap().as_str().unwrap()).unwrap();
        assert_eq!(targets, serde_json::json!([{"granularity":"edge","id":format!("{}.edge.{}", law["object"].as_str().unwrap(), law["edgeId"].as_u64().unwrap())}]));
        while !input.close_step().unwrap() {}
        assert!(input.terminal_is_empty());
    }
    retire_bridged_surface(&mut state);
    eprintln!("[DEBUG] originalClippedWirePick objectAndComponentSelectHover=true containedAndCrossingMarquees=true clippedVisibleDepth=true authoredEdgeId=true exactInputClosure=true");
}

/// 🎨️ Primary analytic geometry follows the original object-style and live-theme authority.
#[test]
fn world_native_primary_geometry_paint_uses_original_style_tokens() {
    let fixture: serde_json::Value = serde_json::from_str(read_static("../../../🧪️tests/🧫️fixtures/🎨️primary-geometry-style/🔣️.json")).unwrap();
    let geometry: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️component-source/🔣️.json")).unwrap();
    let mut custom = ui_wgpu::wgpu::Theme::default();
    custom.border_normal = ui_wgpu::wgpu::Rgba::new(0.13,0.21,0.34,1.0);
    custom.border_emphasized = ui_wgpu::wgpu::Rgba::new(0.55,0.34,0.21,1.0);
    custom.celebrate[0] = ui_wgpu::wgpu::Rgba::new(0.21,0.55,0.34,1.0);
    for kind in fixture["geometry"].as_array().unwrap() {
        let mesh = geometry["zeroIndex"]["accepted"].as_array().unwrap().iter().find(|row|row["kind"] == *kind).unwrap();
        for row in fixture["states"].as_array().unwrap() {
            let object = format!("primary-{}",kind.as_str().unwrap());
            let mut scene = scene_with_selection("{}");let world = scene.world_3d.as_mut().unwrap();
            world.meshes_json = serde_json::json!([{"id":"primary","data":{"positions":mesh["positions"].as_array().unwrap().iter().flat_map(|point|point.as_array().unwrap()).collect::<Vec<_>>(),"indices":[],"vertexIds":mesh["vertexIds"],"edgePositions":mesh["edges"].as_array().unwrap().iter().flat_map(|edge|edge.as_array().unwrap()).flat_map(|point|point.as_array().unwrap()).collect::<Vec<_>>(),"edgeIds":mesh["edgeIds"]}}]).to_string();
            world.instances_json = serde_json::json!([{"id":object,"meshId":"primary"}]).to_string();
            world.selection_json = serde_json::json!({"granularity":"object","showEdges":false,"ids":if row["selected"].as_bool().unwrap() {vec![object.as_str()]} else {vec![]},"hoveredId":if row["hovered"].as_bool().unwrap() {Some(object.as_str())} else {None},"targets":{"mesh":true,"vertex":false,"edge":false,"face":false}}).to_string();
            let bounds = Rect::new(0.0,0.0,400.0,400.0);let mut state = World3dState::new("surface-style".into(),"controller".into());drive_scene_bridge(&mut state,&scene,bounds);
            for theme in [ui_wgpu::wgpu::Theme::default(),custom.clone()] {
                let color = match row["line"].as_str().unwrap() {
                    "var(--border-normal-color)"=>theme.border_normal,
                    "var(--border-emphasized-color)"=>theme.border_emphasized,
                    "var(--color-primary)"=>theme.celebrate[0],
                    _=>panic!("neutral style token"),
                };
                let mut lines = Vec::new();append_component_overlays(&state.selection, &state.gumball, &state.scene, &state.overlay,&theme,&state.camera.orbit.to_camera(),bounds,&mut lines);
                assert!(!lines.is_empty());
                assert!(lines.iter().all(|line|line.color == [color.r,color.g,color.b,color.a]),"{kind}: {} follows the original primary line token",row["name"]);
            }
            retire_bridged_surface(&mut state);
        }
    }
    println!("[DEBUG] originalPrimaryGeometryStyle geometries=3 objectStates=4 themes=2 originalPalette=true noSyntheticGeometry=true");
}

/// 🧊️ Original zero-index scene leases reach actual GPU buffers without fabricated triangles.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn world_native_primary_geometry_gpu_upload_preserves_original_zero_indices() {
    let mut gpu = match semio_framework_async::block_on(ui_gpu::GpuContext::headless(1, 1)) {
        Ok(gpu) => gpu,
        Err(error) if error.starts_with("offscreen adapter:") => { println!("[DEBUG] originalPrimaryGpuAdapterUnavailable {error}"); return; }
        Err(error) => panic!("original primary GPU initialization {error}"),
    };
    let wire: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎯️analytic-wire-picking/🔣️.json")).unwrap();
    let points: serde_json::Value = serde_json::from_str(read_static("../../../../../../../../🔨️modules/🧊️3d/📐️brep/⚙️engine/🧪️tests/🧫️fixtures/🎯️vertex-provenance/🔣️.json")).unwrap();
    let point = &points["cases"].as_array().unwrap().iter().find(|row| row["kind"] == "point").unwrap()["points"][0];
    let point_mesh = serde_json::json!({"positions":point,"indices":[],"vertexIds":[0],"edgePositions":[],"edgeIds":[]});
    let mut results = Vec::new();
    for (name, mesh) in [("wire", &wire["mesh"]), ("edge-only", &wire["edgeOnly"]["mesh"]), ("point", &point_mesh)] {
        let mut scene = scene_with_selection("{}"); let world = scene.world_3d.as_mut().unwrap();
        world.meshes_json = serde_json::json!([{"id":name,"data":mesh}]).to_string();
        world.instances_json = serde_json::json!([{"id":name,"meshId":name}]).to_string();
        let mut state = World3dState::new("surface-gpu".into(), "controller".into());
        drive_scene_bridge(&mut state, &scene, Rect::new(0.0, 0.0, 400.0, 400.0));
        let lease = *state.scene.meshes.get(name).expect("original bridge published the primitive lease");
        assert_eq!(lease.schema().unwrap().indices, 0);
        let version = live_world3d_mesh_version(&state.scene, name);
        let mut result = Err("original GPU upload turn ceiling");
        for _ in 0..128 {
            match gpu.ensure_mesh_step(name, version, lease) {
                Ok(false) => {}
                Ok(true) => { result = Ok(()); break; }
                Err(error) => { result = Err(error); break; }
            }
        }
        if result.is_ok() {
            let resident = gpu.mesh_observation(name, version).unwrap();
            assert_eq!(resident.index_count, 0);
            assert_eq!(resident.index_bytes, 0, "an original primitive owns no fabricated index bytes");
        }
        for _ in 0..4096 { if gpu.close_mesh_upload_step() { break; } }
        assert!(gpu.mesh_upload_terminal_is_empty());
        for _ in 0..4096 { if gpu.retire_mesh_exact_step(name, version).unwrap() { break; } }
        assert!(gpu.mesh_observation(name, version).is_none());
        retire_bridged_surface(&mut state); results.push((name, result));
    }
    for _ in 0..4096 { if gpu.close_mesh_table_step() { break; } }
    assert!(gpu.mesh_table_terminal_is_empty());
    for (name, result) in results { assert!(result.is_ok(), "{name}: actual original GPU upload {result:?}"); }
    println!("[DEBUG] originalPrimaryGpuUpload actualDevice=true originalBridgeLeases=3 indexCount=0 fabricatedTriangles=false uploadAndResidentRetirement=true independentThree=true");
}

fn pixel(rgba: &[u8], size: u32, x: u32, y: u32) -> [u8; 4] {
    let at = ((y * size + x) * 4) as usize;
    [rgba[at], rgba[at + 1], rgba[at + 2], rgba[at + 3]]
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
