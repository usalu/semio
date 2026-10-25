//! 🧪️ scene tests (🔬️unit): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{World3dBuildContext, WORLD_INTERACTION_ID_BYTE_CAPACITY, WORLD_INTERACTION_MESH_CAPACITY, world_mesh_ownership_grant, world_scene_raster_pool, WorldCursorWakeAuthority, WorldDynamicFault, WorldEnvironmentRecord, WorldEnvironmentShadowRecord, WorldEnvironmentSunRecord, WorldInteractionMeshRegistry};
use semio_framework_os_infinite_world_components::{placeholder_triangle, placeholder_triangle_normal, TerrainTileMeshPayload, WorldPlaceholderKind, WorldPlaceholderMeshCursor, WorldPlaceholderMeshStep};
use semio_framework_os_infinite_world_scene::{append_lod_grid_lines, bounded_reference_image, chunk_center, chunk_distance_visible, chunk_key_indices, collect_world3d_asset_bytes, decode_reference_image_bytes, environment_light_dir, GUMBALL_PLANE_MESH, hypsometric_color, mesh_style_paint, MeshStyleKind, MeshStyleState, parse_color, resolve_mesh_style, retain_ensured_world_draws, terrain_tile_url, world3d_style_paint, WORLD_ASSET_REQUEST_CAPACITY, WORLD_ASSET_RESPONSE_BYTE_CAPACITY, WORLD_ASSET_RESPONSE_PAGE_BYTES, WORLD_GRID_MAX_DIVISIONS, WorldAssetFault, WorldAssetIoAuthority, WorldAssetMetadataId, WorldAssetRequestKind, WorldAssetResponsePage, WorldTerrainMeshCursor, WorldTerrainMeshPhase, WorldTerrainMeshStep};
use semio_framework_test_fixtures::fixture_text;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_infinite_world_scene::{build_terrain_tile_mesh_oracle, placeholder_mesh, TerrainMeshOracleData};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_os_infinite_world_scene::{environment_scene_shadow, world3d_shadow_role, World3dShadowGeometry, World3dShadowProfile, WORLD_REFERENCE_TEXTURE_BYTES};
use ui_scene::math::{Camera3d, CameraProjection3d, Instance3d, lod_grid_step_world, Mat4, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_seal, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec3, mesh3d_write_vec4, Mesh3dField, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, OrbitController, SceneColorSource3d, SceneDraw3d, Vec3};
use semio_framework_ui_prepared::PreparedRenderEviction;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_theme::{Rgba, Theme};
use semio_framework_ui_raster_owner::{SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterProfile, SceneRasterWriteMode};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))] use ui_scene::math::{ICON_SHADOW_MAP_SIZE, SceneInstanceMaterial3d, WORLD_SHADOW_MAP_SIZE};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(test)] use semio_framework_ui_raster_owner::SceneRasterMeshSeal;
use ui_styling::metrics::tool_run;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 🌳️ Reads one world crate root and splices every split-out child (a `#[path]` module re-exported with a glob) in at its mount, so the text keeps the item order of the unsplit file.
fn read_world_tree(path_from_manifest: &str) -> String {
    let directory = path_from_manifest.trim_end_matches("🦀️.rs");
    let lines: Vec<&str> = fixture_text!(path_from_manifest).lines().collect();
    let mut text = String::new();
    for (index, line) in lines.iter().enumerate() {
        text.push_str(line);
        text.push('\n');
        if index >= 2 && line.contains("use ") && line.ends_with("::*;") && lines[index - 2].starts_with("#[path = \"") {
            let child = lines[index - 2].trim_start_matches("#[path = \"").trim_end_matches("\"]");
            text.push_str(&read_world_tree(&format!("{directory}{child}")));
        }
    }
    text
}

#[allow(non_snake_case)]
fn WORLD_SOURCE() -> &'static str {
    let joined: String = ["../../../🗃️model/🦀️.rs", "../../../🧩️components/🦀️.rs", "../../../🧲️cursors/🦀️.rs", "../../🦀️.rs", "../../../🌉️bridge/🦀️.rs", "../../../🧵️interaction/🦀️.rs", "../../../🎛️host/🦀️.rs"].iter().map(|root| read_world_tree(root)).collect();
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

fn assert_send<T: Send>() {}

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
fn placeholder_writer_matches_legacy_geometry_and_closes_interrupted_authority() {
    for (name, kind) in [("box", WorldPlaceholderKind::Box), ("plane", WorldPlaceholderKind::Plane), ("cylinder", WorldPlaceholderKind::Cylinder), ("cone", WorldPlaceholderKind::Cone), ("vortex-marker", WorldPlaceholderKind::Icosphere)] {
        let legacy = placeholder_mesh(name);
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        for triangle in 0..kind.triangles() {
            let vertices = placeholder_triangle(kind, triangle);
            let normal = placeholder_triangle_normal(vertices);
            for vertex in vertices {
                positions.extend_from_slice(&vertex);
                normals.extend_from_slice(&normal);
                indices.push(indices.len() as u32);
            }
        }
        assert_eq!(positions, legacy.positions);
        assert_eq!(indices, legacy.indices);
        for (actual, expected) in normals.iter().zip(&legacy.normals) {
            assert!((actual - expected).abs() <= 1e-6, "{name} normal mismatch: {actual} != {expected}");
        }
    }

    let mut interrupted = WorldPlaceholderMeshCursor::new("interrupted", WorldPlaceholderKind::Icosphere, 41, 7).expect("placeholder authority");
    assert!(matches!(interrupted.step(), WorldPlaceholderMeshStep::Pending));
    assert!(matches!(interrupted.step(), WorldPlaceholderMeshStep::Pending));
    let mut turns = 0;
    while !interrupted.close_step() || !interrupted.terminal_is_empty() {
        turns += 1;
        assert!(turns < 64);
    }
    assert!(interrupted.terminal_is_empty());
}

#[test]
fn terrain_writer_matches_the_vertex_coloured_tile_oracle_and_closes_interrupted_authority() {
    fn payload() -> TerrainTileMeshPayload {
        TerrainTileMeshPayload {
            positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 3.0, 0.0, 0.0, 2.0, 1.0, 0.0],
            normals: [0.0, 0.0, 1.0].repeat(6),
            indices: vec![0, 1, 2, 3, 4, 5],
            uvs: vec![0.0, 0.05, 0.0, 0.05, 0.0, 0.05, 0.0, 0.95, 0.0, 0.95, 0.0, 0.95],
        }
    }

    let mut cursor = terrain_test_admit("surface", (3, 4, 5), payload(), 50, 9, 9).expect("terrain cursor");
    let mut published = 0;
    let mut turns = 0;
    loop {
        turns += 1;
        assert!(turns < 512);
        match terrain_test_step(&mut cursor,9,9) {
            WorldTerrainMeshStep::Pending => {}
            WorldTerrainMeshStep::Ready(key, lease) => {
                published += 1;
                assert_eq!(key, "terrain:surface:3:4:5", "one vertex-coloured mesh per TILE, never one per colour band");
                let legacy = build_terrain_tile_mesh_oracle(&payload()).expect("published tile has oracle geometry");
                let schema = lease.schema().expect("terrain lease schema");
                assert_eq!(schema.vertices as usize * 3, legacy.positions.len());
                assert_eq!(schema.indices as usize, legacy.indices.len());
                assert_eq!(schema.colors, schema.vertices, "every terrain vertex carries its own hypsometric colour");
                for item in 0..schema.vertices {
                    assert_eq!(lease.vec3(Mesh3dField::Positions, item).unwrap(), legacy.positions[item as usize * 3..item as usize * 3 + 3]);
                    assert_eq!(lease.vec3(Mesh3dField::Normals, item).unwrap(), legacy.normals[item as usize * 3..item as usize * 3 + 3]);
                    assert_eq!(lease.vec4(Mesh3dField::Colors, item).unwrap(), legacy.colors[item as usize * 4..item as usize * 4 + 4]);
                }
                for item in 0..schema.indices {
                    assert_eq!(lease.u32(Mesh3dField::Indices, item).unwrap(), legacy.indices[item as usize]);
                }
                mesh3d_begin_close(lease).unwrap();
                close_oracle_mesh(lease);
            }
            WorldTerrainMeshStep::Complete(tile) => {
                assert_eq!(tile, (3, 4, 5));
                assert_eq!(published, 1, "a tile publishes exactly one mesh");
                break;
            }
            WorldTerrainMeshStep::Fault(fault) => panic!("valid terrain cursor faulted: {fault:?}"),
        }
    }
    assert!(cursor.terminal_is_empty());

    let mut interrupted = terrain_test_admit("surface", (3, 4, 5), payload(), 70, 11, 11).expect("terrain cursor");
    assert!(matches!(terrain_test_step(&mut interrupted,11,11), WorldTerrainMeshStep::Pending));
    let mut close_turns = 0;
    while !terrain_test_close(&mut interrupted) || !interrupted.terminal_is_empty() {
        close_turns += 1;
        assert!(close_turns < 64);
    }
    assert!(interrupted.terminal_is_empty());
    assert!(terrain_test_admit("surface", (0, 0, 0), TerrainTileMeshPayload { positions: vec![0.0, 1.0], normals: Vec::new(), indices: vec![0, 1, 2], uvs: Vec::new() }, 80, 1, 1).is_err());
}

fn allocate_oracle_mesh(token:Mesh3dWriteToken) {
    loop {
        let grant=world_mesh_ownership_grant(0,ui_scene::math::mesh3d_allocate_capacity_byte_demand(token).unwrap(),0,1);
        let step=mesh3d_allocate_step(token,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
}

fn close_oracle_mesh(lease:Mesh3dLease) {
    loop {
        let demand=ui_scene::math::mesh3d_close_demands(lease,4096).unwrap();
        let grant=world_mesh_ownership_grant(4096,demand.capacity_bytes,demand.release_bytes,demand.depth);
        let step=mesh3d_close_step(lease,grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete{break}
    }
}

#[test]
fn world_mesh_registry_enforces_fixed_capacity_id_topology_and_aba() {
    let mesh = publish_oracle_mesh(triangle_mesh_oracle());
    let mut registry = WorldInteractionMeshRegistry::default();
    let first = registry.admit("mesh-0", 1, mesh).expect("first mesh token");
    assert_eq!(registry.admit("mesh-0", 1, mesh), Some(first));
    let replacement = registry.admit("mesh-0", 2, mesh).expect("replacement token");
    assert_ne!(replacement, first);
    assert!(registry.resolve(first).is_none());
    assert_eq!(registry.resolve(replacement).map(|slot| slot.version), Some(2));
    for index in 1..WORLD_INTERACTION_MESH_CAPACITY {
        assert!(registry.admit(&format!("mesh-{index}"), 1, mesh).is_some());
    }
    assert!(registry.admit("mesh-overflow", 1, mesh).is_none());
    assert!(registry.faulted);

    let mut oversized = WorldInteractionMeshRegistry::default();
    assert!(oversized.admit(&"x".repeat(WORLD_INTERACTION_ID_BYTE_CAPACITY + 1), 1, mesh).is_none());
    assert!(oversized.faulted);
    let grant=world_mesh_ownership_grant(0,ui_scene::math::mesh3d_begin_capacity_byte_demand(),0,1);
    assert_eq!(mesh3d_begin(99,0,Mesh3dSchema::triangle_mesh(0,0),grant),Err(ui_scene::math::Mesh3dFault::Schema));
}

#[test]
fn prepared_world_resources_are_send_and_deduplicate_uploads() {
    assert_send::<World3dBuildContext>();
    let mut resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mesh = publish_oracle_mesh(triangle_mesh_oracle());
    resources.ensure_mesh("mesh", 3, mesh);
    resources.ensure_mesh("mesh", 3, mesh);
    resources.ensure_world_plane_texture("image", test_scene_raster_lease(1, 1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, 1));
    resources.ensure_world_plane_texture("image", test_scene_raster_lease(1, 1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, 9));
    resources.evict_mesh("stale");
    let mut input = semio_framework_ui_prepared::PreparedRenderInput::try_new(1, 2, semio_framework_ui_draw_contract::DrawList::default(), None, 0.0).ok().expect("prepared input admitted");
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(true));
    assert_eq!(input.uploads.len(), 2, "one mesh and one lease-backed raster upload survive key dedupe");
    assert_eq!(input.raster_producers.len(), 0);
    assert_eq!(input.evictions.len(), 1);
    assert_eq!(input.evictions.get(0), Some(&PreparedRenderEviction::Mesh { key: "stale".into() }));
    for _ in 0..4096 {
        let grant = semio_framework_job::RetainedCloneGrant { maximum_copy_bytes: 16 * 1024, ..input.next_close_demands(16 * 1024).expect("prepared input close demand") };
        if matches!(input.close_step(grant).expect("prepared input close step"), semio_framework_job::InteractiveJobCloseStep::Complete { .. }) {
            break;
        }
    }
    assert!(input.terminal_is_empty());
}

#[test]
fn chunk_key_indices_buckets_negative_coordinates() {
    assert_eq!(chunk_key_indices([-10.0, 5.0, 0.0], 256.0), (-1, 0, 0));
    assert_eq!(chunk_key_indices([300.0, 300.0, 0.0], 256.0), (1, 1, 0));
}

#[test]
fn chunk_distance_visible_uses_hysteresis() {
    let center = chunk_center((0, 0, 0), 256.0);
    let cam_near = Vec3::new(0.0, 0.0, 0.0);
    assert!(chunk_distance_visible(cam_near, center, 256.0, 8000.0, false));
    let cam_far = Vec3::new(8400.0, 128.0, 0.0);
    assert!(!chunk_distance_visible(cam_far, center, 256.0, 8000.0, false));
    assert!(chunk_distance_visible(cam_far, center, 256.0, 8000.0, true));
}

//#region GlbAssetTests
#[test]
fn asset_request_cancellation_is_exact_and_cannot_cancel_a_reused_slot() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/📤️asset-cancellation/🔣️.json")).unwrap();
    for stage in fixture["stages"].as_array().unwrap() {
        let mut lane = WorldAssetIoAuthority::default();
        let first = lane.reserve_request(1, 0, WorldAssetRequestKind::Glb, fixture["requests"][0].as_str().unwrap()).unwrap();
        let unrelated = lane.reserve_request(2, 0, WorldAssetRequestKind::Glb, fixture["requests"][1].as_str().unwrap()).unwrap();
        let owner = (stage == "fetching").then(|| lane.take_next().unwrap());
        assert!(lane.cancel_request(first));
        assert_eq!(lane.cancel_request(first), fixture["expected"]["duplicateCancelAccepted"].as_bool().unwrap());
        assert_eq!(lane.cancellation_requested(first), fixture["expected"]["cancelled"][0].as_bool().unwrap());
        assert_eq!(lane.cancellation_requested(unrelated), fixture["expected"]["cancelled"][1].as_bool().unwrap());
        if let Some(owner) = owner {
            lane.return_owner(owner).unwrap();
        }
        while lane.retire_cancelled_step() {}
        let replacement = lane.reserve_request(3, 0, WorldAssetRequestKind::Glb, fixture["requests"][2].as_str().unwrap()).unwrap();
        assert_eq!(first.slot, replacement.slot);
        assert_eq!(lane.cancel_request(first), fixture["expected"]["staleCancelAccepted"].as_bool().unwrap());
        assert_eq!(lane.cancellation_requested(replacement), fixture["expected"]["replacementCancelled"].as_bool().unwrap());
        lane.begin_close();
        while !lane.close_step() {}
        assert!(lane.terminal_is_empty());
    }
}

#[test]
fn asset_authority_rejects_request_and_byte_capacity_plus_one_before_string_ownership() {
    let mut lane = WorldAssetIoAuthority::default();
    for index in 0..WORLD_ASSET_REQUEST_CAPACITY {
        lane.reserve(1, 1, WorldAssetRequestKind::Glb, &format!("asset-{index}"), 1).unwrap();
    }
    assert_eq!(lane.reserve(1, 1, WorldAssetRequestKind::Glb, "overflow", 1), Err(WorldAssetFault::ItemCapacity));
    lane.begin_close();
    while !lane.close_step() {}
    assert!(lane.terminal_is_empty());

    let mut lane = WorldAssetIoAuthority::default();
    assert_eq!(lane.reserve(1, 1, WorldAssetRequestKind::Glb, "large", WORLD_ASSET_RESPONSE_BYTE_CAPACITY + 1), Err(WorldAssetFault::ByteCapacity));
    assert!(lane.terminal_is_empty());
}

#[test]
fn asset_authority_reopens_after_terminal_close_so_later_reserves_succeed() {
    let mut lane = WorldAssetIoAuthority::default();
    lane.reserve_request(1, 1, WorldAssetRequestKind::Glb, "first").unwrap();
    lane.begin_close();
    while !lane.close_step() {}
    assert!(lane.terminal_is_empty());
    lane.reserve_request(2, 1, WorldAssetRequestKind::MapTile {
        surface: WorldAssetMetadataId::try_from_str("map-a").unwrap(),
        key: WorldAssetMetadataId::try_from_str("0/0/0").unwrap(),
        vector: false,
        z: 0,
        x: 0,
        y: 0,
    }, "https://tiles/0/0/0.png").unwrap();
    lane.cancel_map_tiles_for_surface("map-a");
    while lane.retire_cancelled_step() {}
    assert!(lane.terminal_is_empty());
    lane.reserve_request(3, 1, WorldAssetRequestKind::Glb, "after-cancel").unwrap();
}

#[test]
fn asset_response_rejects_page_plus_one_and_retires_partial_stream_one_page_per_step() {
    let bytes = vec![7; WORLD_ASSET_RESPONSE_PAGE_BYTES + 1];
    assert_eq!(WorldAssetResponsePage::try_from_owned(bytes).expect_err("page +1 exact owner").len(), WORLD_ASSET_RESPONSE_PAGE_BYTES + 1);

    let mut lane = WorldAssetIoAuthority::default();
    let token = lane.reserve_request(2, 3, WorldAssetRequestKind::ReferenceImage, "image").unwrap();
    let mut owner = lane.take_next().unwrap();
    lane.reserve_response(&mut owner, WORLD_ASSET_RESPONSE_PAGE_BYTES * 2).unwrap();
    owner.push_page(WorldAssetResponsePage::try_from_owned(vec![1; WORLD_ASSET_RESPONSE_PAGE_BYTES]).unwrap()).unwrap();
    owner.push_page(WorldAssetResponsePage::try_from_owned(vec![2; 7]).unwrap()).unwrap();
    lane.return_owner(owner).unwrap();
    lane.begin_close();
    assert!(!lane.close_step(), "first grant retires one response page");
    assert!(!lane.close_step(), "second grant retires the second response page");
    while !lane.close_step() {}
    assert!(lane.terminal_is_empty());
    let _ = token;
}

#[test]
fn asset_decode_resume_and_stale_generation_keep_claimed_pages_until_terminal_return() {
    let mut lane = WorldAssetIoAuthority::default();
    let token = lane.reserve(4, 9, WorldAssetRequestKind::Glb, "mesh", 16).unwrap();
    let mut fetch = lane.take_next().unwrap();
    fetch.push_page(WorldAssetResponsePage::try_from_owned(vec![1; 8]).unwrap()).unwrap();
    fetch.push_page(WorldAssetResponsePage::try_from_owned(vec![2; 8]).unwrap()).unwrap();
    fetch.seal().unwrap();
    lane.return_owner(fetch).unwrap();
    assert!(matches!(lane.take_completed(token, 5, 9), Err(WorldAssetFault::Stale)));
    let mut decode = lane.take_completed(token, 4, 9).unwrap();
    assert_eq!(decode.take_decode_page().unwrap().unwrap().bytes(), &[1; 8]);
    lane.return_owner(decode).unwrap();
    let mut decode = lane.take_completed(token, 4, 9).unwrap();
    assert_eq!(decode.take_decode_page().unwrap().unwrap().bytes(), &[2; 8]);
    assert!(decode.take_decode_page().unwrap().is_none());
    decode.begin_close();
    while !decode.close_step() {}
    lane.finish(decode).unwrap();
    assert!(lane.terminal_is_empty());
}

#[test]
fn asset_unknown_length_releases_unused_aggregate_credit_at_seal() {
    let mut lane = WorldAssetIoAuthority::default();
    let token = lane.reserve_request(7, 11, WorldAssetRequestKind::Glb, "chunked").unwrap();
    let mut owner = lane.take_next().unwrap();
    lane.reserve_response(&mut owner, WORLD_ASSET_RESPONSE_BYTE_CAPACITY).unwrap();
    owner.push_page(WorldAssetResponsePage::try_from_owned(vec![3; 7]).unwrap()).unwrap();
    lane.seal_response(&mut owner).unwrap();
    assert_eq!(lane.reserved_bytes, 7);
    lane.return_owner(owner).unwrap();
    let mut owner = lane.take_completed(token, 7, 11).unwrap();
    assert_eq!(owner.take_decode_page().unwrap().unwrap().bytes(), &[3; 7]);
    owner.begin_close();
    while !owner.close_step() {}
    lane.finish(owner).unwrap();
    assert!(lane.terminal_is_empty());
}

#[test]
fn asset_completed_cursor_advances_one_fixed_slot_per_grant_and_hands_back_exact_owner() {
    let mut lane = WorldAssetIoAuthority::default();
    let token = lane.reserve_request(8, 12, WorldAssetRequestKind::Glb, "cursor.glb").unwrap();
    let mut fetch = lane.take_next().unwrap();
    assert!(lane.take_next_completed_step().is_none(), "the first grant observes the in-flight slot without scanning ahead");
    lane.reserve_response(&mut fetch, 12).unwrap();
    fetch.push_page(WorldAssetResponsePage::try_from_owned(b"glTF\x02\0\0\0\x0c\0\0\0".to_vec()).unwrap()).unwrap();
    lane.seal_response(&mut fetch).unwrap();
    lane.return_owner(fetch).unwrap();
    let mut decode = (0..WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| lane.take_next_completed_step()).expect("the retained cursor reaches the completed slot");
    assert_eq!(decode.token(), token);
    assert_eq!(decode.decode_page().unwrap().unwrap().bytes(), b"glTF\x02\0\0\0\x0c\0\0\0");
    decode.advance_decode_page().unwrap();
    assert!(decode.decode_page().unwrap().is_none());
    decode.rewind_decode_pages().unwrap();
    assert!(decode.decode_page().unwrap().is_some());
    decode.begin_close();
    while !decode.close_step() {}
    lane.finish(decode).unwrap();
    assert!(lane.terminal_is_empty());
}

/// 🖼️ A reference underlay preserves the source's natural dimensions inside the schema-owned 64 MiB
/// straight-RGBA credit. The puzzle3d playground's plan is 2275×2560, or 23 296 000 bytes.
#[test]
fn a_reference_underlay_preserves_its_natural_full_quality_dimensions() {
    let plan = semio_framework_pixels::RasterImage::new(2275, 2560);
    let bounded = bounded_reference_image(plan);
    assert_eq!((bounded.width, bounded.height), (2275, 2560));
    assert_eq!(bounded.width as usize * bounded.height as usize * 4, 23_296_000);
    assert!(23_296_000 <= WORLD_REFERENCE_TEXTURE_BYTES);
    let small = semio_framework_pixels::RasterImage::new(64, 48);
    let bounded = bounded_reference_image(small);
    assert_eq!((bounded.width, bounded.height), (64, 48));
}

/// 📥️ `collect_world3d_asset_bytes` answers EVERY sealed page exactly once. It used to re-read page
/// zero forever (`while let Some(page) = owner.decode_page()?` with no advance), which on a
/// multi-page response grew the payload until `RawVec` panicked with `capacity overflow` and took the
/// frame Worker down with it — the second half of the puzzle3d wgpu boot fault.
#[test]
fn collecting_a_multi_page_asset_response_answers_each_page_once() {
    let mut lane = WorldAssetIoAuthority::default();
    lane.reserve_request(3, 4, WorldAssetRequestKind::ReferenceImage, "plan.jpg").unwrap();
    let mut fetch = lane.take_next().unwrap();
    lane.reserve_response(&mut fetch, 7).unwrap();
    for page in [vec![1u8; 4], vec![2u8; 2], vec![3u8; 1]] {
        fetch.push_page(WorldAssetResponsePage::try_from_owned(page).unwrap()).unwrap();
    }
    lane.seal_response(&mut fetch).unwrap();
    lane.return_owner(fetch).unwrap();
    let mut decode = (0..WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| lane.take_next_completed_step()).expect("completed response");
    assert_eq!(collect_world3d_asset_bytes(&mut decode).unwrap(), vec![1, 1, 1, 1, 2, 2, 3]);
    assert_eq!(collect_world3d_asset_bytes(&mut decode).unwrap(), vec![1, 1, 1, 1, 2, 2, 3], "a second collect reads the same sealed pages, never the cursor's leftovers");
    assert_eq!(decode.decode_page().unwrap().map(|page| page.bytes().to_vec()), Some(vec![1, 1, 1, 1]), "the cursor is left rewound for the next decoder");
    decode.begin_close();
    while !decode.close_step() {}
    lane.finish(decode).unwrap();
    assert!(lane.terminal_is_empty());
}

/// 📐️ LAW: the grid draws React's ONE band at `lodGridStepWorld`'s exact spacing, is sized by the
/// camera's own fade radius rather than a fixed square, stays inside the far plane, and fades to
/// zero alpha at its rim (`📓️w7b-presenter-one-frame-per-boot.md` §5,
/// `📓️w8b-orthographic-camera-and-3d-parity.md` §3).
///
/// 🔀️ The same law under a plan view: a parallel camera's fade radius comes from its zoomed
/// pixel extent, so a `Top` pane gets a grid of its own instead of the perspective one's.
#[test]
fn lod_grid_lines_keep_reacts_band_spacing_and_fade_inside_the_far_plane() {
    let viewport = Rect { x: 0.0, y: 0.0, w: 956.0, h: 814.0 };
    let camera = OrbitController::default().to_camera();
    let mut lines = Vec::new();
    append_lod_grid_lines(&mut lines, 2.0, 10.0, Vec3::ZERO, &camera, viewport, [0.5, 0.5, 0.5, 1.0]);
    assert!(!lines.is_empty(), "a near camera still gets a grid");

    let half = lines.iter().flat_map(|vertex| [vertex.position[0].abs(), vertex.position[1].abs()]).fold(0.0_f32, f32::max);
    assert!(half <= camera.far * 0.25 + 1e-3, "the whole grid sits inside the camera's far plane");
    assert!(half < 12_000.0 * 0.5, "and is nowhere near the fixed square it used to draw");

    let step = ui_scene::math::lod_grid_step_world(2.0, 10.0).expect("React's helper answers a step") as f32;
    let mut offsets: Vec<f32> = lines.iter().map(|vertex| vertex.position[1]).collect();
    offsets.sort_by(|left, right| left.partial_cmp(right).expect("finite grid coordinates"));
    offsets.dedup_by(|left, right| (*left - *right).abs() < 1e-4);
    for pair in offsets.windows(2) {
        let gap = pair[1] - pair[0];
        assert!((gap / step - (gap / step).round()).abs() < 1e-3, "every row lands on React's band multiple, never on a clamped division");
    }
    assert!(lines.iter().any(|vertex| vertex.color[3] == 0.0), "the rim fades out instead of ending on a hard edge");

    let parallel = ui_scene::math::Camera3d { projection: ui_scene::math::CameraProjection3d::Orthographic, zoom: 12.0, position: Vec3::new(0.0, 0.0, 9.45), up: Vec3::new(0.0, 1.0, 0.0), ..camera.clone() };
    let mut parallel_lines = Vec::new();
    append_lod_grid_lines(&mut parallel_lines, 2.0, 10.0, Vec3::ZERO, &parallel, viewport, [0.5, 0.5, 0.5, 1.0]);
    assert!(!parallel_lines.is_empty(), "a plan pane draws a grid too");
    let parallel_half = parallel_lines.iter().flat_map(|vertex| [vertex.position[0].abs(), vertex.position[1].abs()]).fold(0.0_f32, f32::max);
    assert!(parallel_half <= camera.far * 0.25 + 1e-3);
    assert!(parallel_lines.len() <= 8 * (WORLD_GRID_MAX_DIVISIONS as usize + 1), "and never submits more than the division ceiling allows");
}

/// ☀️ Azimuth=90, elevation=0 -> pure +Y direction (cos(0)*cos(90)=~0, cos(0)*sin(90)=1, sin(0)=0).
#[test]
fn environment_light_dir_uses_sun_direction_only_when_enabled() {
    let disabled = WorldEnvironmentRecord { sun: Some(WorldEnvironmentSunRecord { enabled: Some(false), azimuth: Some(90.0), elevation: Some(0.0), ..Default::default() }), ..Default::default() };
    assert_eq!(environment_light_dir(&disabled), [0.4, 0.6, 0.8]);

    let enabled = WorldEnvironmentRecord { sun: Some(WorldEnvironmentSunRecord { enabled: Some(true), azimuth: Some(90.0), elevation: Some(0.0), ..Default::default() }), ..Default::default() };
    let dir = environment_light_dir(&enabled);
    assert!(dir[0].abs() < 1e-3);
    assert!((dir[1] - 1.0).abs() < 1e-3);
    assert!(dir[2].abs() < 1e-3);
}

#[test]
fn environment_shadow_requires_the_configured_sun_and_ignores_react_dead_controls() {
    let mut environment = WorldEnvironmentRecord {
        sun: Some(WorldEnvironmentSunRecord { enabled: Some(false), azimuth: Some(120.0), elevation: Some(25.0), ..Default::default() }),
        shadow: Some(WorldEnvironmentShadowRecord { enabled: Some(true), opacity: Some(1.5), softness: Some(7.0) }),
        ..Default::default()
    };
    let light_dir = environment_light_dir(&environment);
    let disabled = environment_scene_shadow(&environment, light_dir, World3dShadowProfile::World);
    assert!(!disabled.enabled);
    assert_eq!(disabled.map_size, WORLD_SHADOW_MAP_SIZE);

    environment.sun.as_mut().unwrap().enabled = Some(true);
    let enabled = environment_scene_shadow(&environment, environment_light_dir(&environment), World3dShadowProfile::World);
    assert!(enabled.enabled);
    assert_eq!(enabled.map_size, WORLD_SHADOW_MAP_SIZE);
    let icon = environment_scene_shadow(&environment, environment_light_dir(&environment), World3dShadowProfile::IconPng);
    assert_eq!(icon.map_size, ICON_SHADOW_MAP_SIZE);
    assert!(!environment_scene_shadow(&environment, environment_light_dir(&environment), World3dShadowProfile::IconSvg).enabled);
    assert!(!environment_scene_shadow(&environment, environment_light_dir(&environment), World3dShadowProfile::Unshadowed).enabled);
}

#[test]
fn neutral_shadow_fixture_maps_to_world_draw_roles_and_exact_profiles() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🌑️scene-shadow-parity/🔣️.json")).unwrap();
    assert_eq!(World3dShadowProfile::World.map_size(), fixture["profiles"]["world"]["mapSize"].as_u64().unwrap() as u32);
    assert_eq!(World3dShadowProfile::IconPng.map_size(), fixture["profiles"]["iconPng"]["mapSize"].as_u64().unwrap() as u32);
    for row in fixture["roles"].as_array().unwrap() {
        let geometry = match row["kind"].as_str().unwrap() {
            "terrain" => World3dShadowGeometry::Terrain,
            "paint" => World3dShadowGeometry::Paint,
            "glb" | "icon" => World3dShadowGeometry::Glb,
            kind => panic!("unknown neutral shadow kind {kind}"),
        };
        let enabled = row["kind"] != "icon" || row["materialPresent"] == true;
        let role = world3d_shadow_role(geometry, enabled);
        assert_eq!(role.casts, row["casts"].as_bool().unwrap(), "{} caster role", row["kind"]);
        assert_eq!(role.receives, row["receives"].as_bool().unwrap(), "{} receiver role", row["kind"]);
    }
    assert!(!fixture["profiles"]["world"]["consumesOpacity"].as_bool().unwrap());
    assert!(!fixture["profiles"]["world"]["consumesSoftness"].as_bool().unwrap());
}

#[test]
fn svg_icon_profile_is_flat_lit_while_world_and_png_remain_raster_pbr() {
    assert_eq!(World3dShadowProfile::IconSvg.render_profile(), ui_scene::math::SceneRenderProfile3d::SvgFlatLit);
    for profile in [World3dShadowProfile::World, World3dShadowProfile::IconPng, World3dShadowProfile::Unshadowed] {
        assert_eq!(profile.render_profile(), ui_scene::math::SceneRenderProfile3d::RasterPbr);
    }
}

#[test]
fn svg_icon_flat_lighting_matches_the_neutral_three_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🎨️icon-svg-lighting/🔣️.json")).unwrap();
    let to_srgb = |value: f32| if value < 0.0031308 { value * 12.92 } else { 1.055 * value.powf(0.41666) - 0.055 };
    for row in fixture["cases"].as_array().unwrap() {
        let vector = |field: &str| Vec3::new(row[field][0].as_f64().unwrap() as f32, row[field][1].as_f64().unwrap() as f32, row[field][2].as_f64().unwrap() as f32).normalize();
        let normal = vector("normal");
        let light = vector("lightDirection");
        let ambient = parse_color(row["ambient"]["color"].as_str().unwrap());
        let sun = parse_color(row["sun"]["color"].as_str().unwrap());
        let base = parse_color(row["material"]["color"].as_str().unwrap());
        let emissive = parse_color(row["material"]["emissive"].as_str().unwrap());
        let amount = normal.dot(light).max(0.0) * row["sun"]["intensity"].as_f64().unwrap() as f32;
        let rgb: [u8; 3] = std::array::from_fn(|channel| (to_srgb(((ambient[channel] + sun[channel] * amount) * base[channel] + emissive[channel]).clamp(0.0, 1.0)) * 255.0).round() as u8);
        assert_eq!(format!("rgb({},{},{})", rgb[0], rgb[1], rgb[2]), row["expectedFill"], "{}", row["id"]);
    }
    assert_eq!(fixture["cases"][0]["expectedFill"], fixture["cases"][1]["expectedFill"], "ambient intensity, metalness, roughness, and emissive intensity are intentionally ignored by SVGRenderer");
}

//#endregion EnvironmentTests

//#region TerrainTests
#[test]
fn hypsometric_color_matches_reference_stops() {
    let low = hypsometric_color(0.0);
    assert!((low[0] - 0x4b as f32 / 255.0).abs() < 1e-3);
    let peak = hypsometric_color(1.0);
    assert!((peak[0] - 1.0).abs() < 1e-3);
    assert!((peak[1] - 1.0).abs() < 1e-3);
}

/// 🏔️ Two triangles, 6 verts (no sharing, to keep each triangle's average elevation exact):
/// triangle 0 is flat at elevation ratio 0.0, triangle 1 is flat at elevation ratio 1.0.
#[test]
fn build_terrain_tile_mesh_oracle_colours_every_vertex_along_the_continuous_ramp() {
    let mesh = TerrainTileMeshPayload {
        positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 10.0, 3.0, 0.0, 10.0, 2.0, 1.0, 10.0],
        normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        indices: vec![0, 1, 2, 3, 4, 5],
        uvs: vec![0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0],
    };
    let tile = build_terrain_tile_mesh_oracle(&mesh).expect("a tile with triangles builds");
    assert_eq!(tile.positions.len(), 18, "the tile is de-indexed into one triangle soup, not split per band");
    assert_eq!(tile.colors.len(), 24, "one RGBA per vertex");
    assert_eq!(&tile.colors[..4], &hypsometric_color(0.0), "the ground-level vertex takes the ramp's low stop");
    assert_eq!(&tile.colors[12..16], &hypsometric_color(1.0), "the peak vertex takes the ramp's white stop");
    assert_ne!(&tile.colors[..4], &tile.colors[12..16], "the ramp is continuous, not one flat colour per tile");
}

#[test]
fn terrain_tile_url_substitutes_z_x_y() {
    assert_eq!(terrain_tile_url("/dem/{z}/{x}/{y}.png", 12, 34, 56), "/dem/12/34/56.png");
}

/// 🧷️ LAW: a world draw is published only when THIS frame's uploads carry the exact mesh identity it
/// names — the source half of the prepared-frame residency law.
///
/// 🩸️ On the GPU a prepared world draw is answered by `mesh_store.get_versioned` at SUBMIT, hundreds
/// of host steps after the build. Five ghost/stand-in sites (`brush_preview`,
/// `catalogue_drop_preview`, the tool-run trace, the vortex arrows, the vertex markers) pushed their
/// `SceneDraw3d` unconditionally while calling `ensure_mesh` only `if let Some(mesh) =
/// state.meshes.get(..)` — and every one of them reaches for a lease `begin_world_placeholder_mesh`
/// is still admitting. The puzzle3d journey died on the engagement chip with `prepared frame submit
/// step: prepared world mesh was missing` (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w11a-prepared-world-mesh-missing.md`).
#[test]
fn a_world_draw_without_this_frames_upload_is_never_published() {
    let mut gpu = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    gpu.ensure_mesh("resident", 3, publish_oracle_mesh(triangle_mesh_oracle()));
    let draw = |key: &str, version: u64| SceneDraw3d { mesh_key: key.into(), mesh_version: version, instances: vec![Instance3d { component_source: None, id: key.into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() };
    let mut draws = vec![draw("resident", 3), draw("resident", 4), draw("ghost-still-landing", 0)];
    retain_ensured_world_draws(&gpu, &mut draws);

    assert_eq!(draws.len(), 1, "only the draw whose exact identity this frame uploads survives");
    assert_eq!((draws[0].mesh_key.as_str(), draws[0].mesh_version), ("resident", 3), "and it is the ensured one, not a stale version of the same key");
    assert!(gpu.has_mesh_request("resident", 3) && !gpu.has_mesh_request("resident", 4), "the predicate is exact on BOTH key and version");
}

/// 🔘️ LAW: the gumball plane is PINNED in the mesh pool. It belongs to no document draw, so every
/// `sync_mesh_pool` would otherwise see it as stale and evict the mesh the very next frame draws.
#[test]
fn the_gumball_plane_is_pinned_against_pool_eviction() {
    let world = WORLD_SOURCE();
    let pinned = world.split("const PINNED: &[&str] = &[").nth(1).expect("sync_mesh_pool declares a PINNED list").split("];").next().expect("PINNED list closes");
    assert!(pinned.contains("GUMBALL_PLANE_MESH"), "the gumball plane must be pinned: {pinned}");
    assert_eq!(GUMBALL_PLANE_MESH, "gumball-plane");
}

/// 🎨️ LAW: the whole of React's `MESH_STYLE_PAINT` lives in ONE table with React's own numbers, and
/// a row is chosen by React's own priority ladder (`resolveMeshStyle`, `🌐️World3dHost/🟦️.tsx`:
/// `disabled → provisional → celebrated → selected → highlighted → hovered → neutral`).
///
/// 🩸️ Two of the seven rows existed, as an inline `if/else if`: `selected` and `hovered`.
/// `provisional` was painted by a second, unrelated rule that substituted `theme.accent` for React's
/// `--color-secondary`; `celebrated`, `highlighted` and `disabled` had no wgpu paint at all, so a
/// disabled instance painted opaque where React paints it at 0.45
/// (`📓️w8b-orthographic-camera-and-3d-parity.md` §7.4).
#[test]
fn the_mesh_style_table_is_reacts_whole_paint_table() {
    let theme = semio_framework_ui_theme::Theme::light();
    let rgba = |color: semio_framework_ui_theme::Rgba| [color.r, color.g, color.b, color.a];
    let (primary, secondary, tertiary) = (rgba(theme.celebrate[0]), rgba(theme.celebrate[1]), rgba(theme.celebrate[2]));

    let neutral = mesh_style_paint(&theme, MeshStyleKind::Neutral);
    assert_eq!((neutral.fill, neutral.line, neutral.emissive_intensity, neutral.opacity), (rgba(theme.panel), rgba(theme.border_normal), 0.0, 1.0));
    let hovered = mesh_style_paint(&theme, MeshStyleKind::Hovered);
    assert_eq!((hovered.fill, hovered.line, hovered.emissive_intensity, hovered.opacity), (rgba(theme.row_hover), rgba(theme.border_emphasized), 0.08, 1.0));
    let selected = mesh_style_paint(&theme, MeshStyleKind::Selected);
    assert_eq!((selected.fill, selected.line, selected.emissive_intensity, selected.opacity), (primary, primary, 0.35, 1.0));
    let highlighted = mesh_style_paint(&theme, MeshStyleKind::Highlighted);
    assert_eq!((highlighted.fill, highlighted.line, highlighted.emissive_intensity, highlighted.opacity), (secondary, secondary, 0.2, 1.0));

    let provisional = mesh_style_paint(&theme, MeshStyleKind::Provisional);
    assert_eq!(provisional.fill, secondary, "🎨️ React's provisional fill is `--color-secondary`, not this theme's accent");
    assert_eq!((provisional.emissive_intensity, provisional.opacity), (0.2, ui_styling::metrics::tool_run::PROVISIONAL_OPACITY as f32));

    let celebrated = mesh_style_paint(&theme, MeshStyleKind::Celebrated);
    assert_eq!((celebrated.fill, celebrated.emissive_intensity, celebrated.opacity), (primary, 0.55, 1.0));
    assert_eq!(celebrated.conic, Some([primary, secondary, tertiary]), "🎉️ the celebrated row carries React's whole conic triad, primary → secondary → tertiary");
    assert!(MeshStyleKind::Neutral != MeshStyleKind::Celebrated && mesh_style_paint(&theme, MeshStyleKind::Selected).conic.is_none(), "🎉️ and it is the ONLY row that does");

    let disabled = mesh_style_paint(&theme, MeshStyleKind::Disabled);
    assert_eq!((disabled.line, disabled.emissive_intensity, disabled.opacity), (rgba(theme.text_muted), 0.0, 0.45), "🎨️ React's disabled row is 0.45 opaque");
    assert_eq!(disabled.fill, ui_styling::color::oklab_mix(rgba(theme.text_muted), rgba(theme.panel), 0.45), "🎨️ and its fill is React's `color-mix(in oklab, muted-foreground 55%, panel)`");
    assert!(disabled.fill != rgba(theme.text_muted) && disabled.fill != rgba(theme.panel), "🎨️ a real mix, not one of its ends: {:?}", disabled.fill);

    let all = MeshStyleState { disabled: true, provisional: true, celebrating: true, selected: true, highlighted: true, hovered: true };
    assert_eq!(resolve_mesh_style(all), MeshStyleKind::Disabled, "🎨️ disabled outranks everything");
    assert_eq!(resolve_mesh_style(MeshStyleState { disabled: false, ..all }), MeshStyleKind::Provisional);
    assert_eq!(resolve_mesh_style(MeshStyleState { disabled: false, provisional: false, ..all }), MeshStyleKind::Celebrated);
    assert_eq!(resolve_mesh_style(MeshStyleState { disabled: false, provisional: false, celebrating: false, ..all }), MeshStyleKind::Selected);
    assert_eq!(resolve_mesh_style(MeshStyleState { selected: false, hovered: true, highlighted: true, ..MeshStyleState::default() }), MeshStyleKind::Highlighted);
    assert_eq!(resolve_mesh_style(MeshStyleState { hovered: true, ..MeshStyleState::default() }), MeshStyleKind::Hovered);
    assert_eq!(resolve_mesh_style(MeshStyleState::default()), MeshStyleKind::Neutral);

    let instance = Instance3d { component_source: None, id: "one".into(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), color: [0.1, 0.2, 0.3, 1.0], selected: false, hovered: false, material: Default::default() };
    assert_eq!(world3d_style_paint(&theme, MeshStyleState::default(), false, instance.clone()).color, [0.1, 0.2, 0.3, 1.0], "🎨️ a neutral authored instance keeps its producer colour");
    let painted = world3d_style_paint(&theme, MeshStyleState { disabled: true, ..MeshStyleState::default() }, false, instance);
    assert_eq!(painted.color[3], 0.45, "🎨️ and a styled one takes the row's opacity into its alpha");
}

/// 🎨️ LAW: neutral color provenance remains retained until paint, so appearance changes recolor
/// only semantic defaults; React's neutral/disabled vertex mode and every static active emissive row
/// are carried through the packed instance material instead of inferred from interaction flags.
#[test]
fn retained_color_provenance_and_static_material_policy_match_react() {
    let light = semio_framework_ui_theme::Theme::light();
    let dark = semio_framework_ui_theme::Theme::dark();
    let instance = |source| Instance3d { component_source: None,
        id: "fixture".into(),
        model: Mat4::identity(),
        color: [0.25, 0.5, 0.75, 0.8],
        selected: false,
        hovered: false,
        material: ui_scene::math::SceneInstanceMaterial3d { color_source: source, ..Default::default() },
    };
    let semantic = |theme: &semio_framework_ui_theme::Theme| world3d_style_paint(theme, MeshStyleState::default(), false, instance(ui_scene::math::SceneColorSource3d::SemanticNeutral));
    assert_eq!(semantic(&light).color, [light.panel.r, light.panel.g, light.panel.b, light.panel.a * 0.8]);
    assert_eq!(semantic(&dark).color, [dark.panel.r, dark.panel.g, dark.panel.b, dark.panel.a * 0.8]);
    for source in [ui_scene::math::SceneColorSource3d::Authored, ui_scene::math::SceneColorSource3d::Environment] {
        assert_eq!(world3d_style_paint(&dark, MeshStyleState::default(), false, instance(source)).color, [0.25, 0.5, 0.75, 0.8]);
    }

    let neutral_vertex = world3d_style_paint(&light, MeshStyleState::default(), true, instance(ui_scene::math::SceneColorSource3d::Authored));
    assert_eq!(neutral_vertex.color, [1.0, 1.0, 1.0, 0.8]);
    assert!(neutral_vertex.material.preserve_vertex_color);
    let disabled_vertex = world3d_style_paint(&light, MeshStyleState { disabled: true, ..Default::default() }, true, instance(ui_scene::math::SceneColorSource3d::Authored));
    assert_eq!(&disabled_vertex.color[..3], &[1.0, 1.0, 1.0]);
    assert!((disabled_vertex.color[3] - 0.36).abs() <= f32::EPSILON);
    assert!(disabled_vertex.material.preserve_vertex_color);
    for (style, expected) in [
        (MeshStyleState { selected: true, ..Default::default() }, 0.35),
        (MeshStyleState { hovered: true, ..Default::default() }, 0.08),
        (MeshStyleState { highlighted: true, ..Default::default() }, 0.2),
        (MeshStyleState { provisional: true, ..Default::default() }, 0.2),
    ] {
        let painted = world3d_style_paint(&light, style, true, instance(ui_scene::math::SceneColorSource3d::SemanticNeutral));
        assert!(!painted.material.preserve_vertex_color);
        assert_eq!(painted.material.emissive_intensity, expected);
    }
}

/// 🌐️ LAW: the grid never ends on a HARD edge, at any camera. React's drei `Grid` is `infiniteGrid`
/// with `followCamera`, so it has no edge at all; this is real line geometry with a
/// [`WORLD_GRID_MAX_DIVISIONS`] vertex ceiling, and the ceiling is only invisible while the fade
/// curve is clamped WITH the radius — the outermost line has to reach alpha 0.
///
/// 🩸️ Spending the ceiling on the STEP instead, so the geometry covers the whole
/// `camera_grid_fade_distance`, was tried and reverted: that distance is React's `visibleRadius × 32`
/// FADE parameter, not a draw extent, so at the puzzle 3d boot camera it coarsened the band from
/// 10 world units to 10 000 and painted four giant cells across a pane where React paints a 10-unit
/// grid (`📓️w9b-projection-pane-framing-grid-materials.md` §3).
///
/// 🌐️ And the ceiling is never spent on the SPACING: every band keeps React's own
/// `lodGridStepWorld` step however far the camera is.
#[test]
fn the_grid_never_ends_on_a_hard_edge_however_far_the_camera_is() {
    let viewport = Rect { x: 0.0, y: 0.0, w: 956.0, h: 814.0 };
    let base = OrbitController::default().to_camera();
    let step = ui_scene::math::lod_grid_step_world(2.0, 10.0).expect("React's helper answers a step") as f32;
    for distance in [12.0_f32, 400.0, 20_000.0, 500_000.0] {
        let camera = ui_scene::math::Camera3d { position: Vec3::new(0.0, 0.0, distance), target: Vec3::ZERO, ..base.clone() };
        let mut lines = Vec::new();
        append_lod_grid_lines(&mut lines, 2.0, 10.0, Vec3::ZERO, &camera, viewport, [0.5, 0.5, 0.5, 1.0]);
        assert!(!lines.is_empty(), "🌐️ every camera gets a grid at {distance}");
        assert!(lines.len() <= 8 * (WORLD_GRID_MAX_DIVISIONS as usize + 1), "🌐️ inside the vertex budget at {distance}: {}", lines.len());
        assert!(lines.iter().any(|vertex| vertex.color[3] == 0.0), "🌐️ the rim reaches alpha 0 at {distance}, so the ceiling is invisible");

        let mut offsets: Vec<f32> = lines.iter().map(|vertex| vertex.position[1]).collect();
        offsets.sort_by(|left, right| left.partial_cmp(right).expect("finite grid coordinates"));
        offsets.dedup_by(|left, right| (*left - *right).abs() < 1e-4);
        let gap = offsets.windows(2).map(|pair| pair[1] - pair[0]).fold(f32::INFINITY, f32::min);
        assert!((gap - step).abs() < 1e-2, "🌐️ the band keeps React's {step}-unit spacing at {distance}, it read {gap}");
    }
}

/// 🧭️ Native reference decode applies the same EXIF orientation that browser `createImageBitmap`
/// applies before the generation-owned pixels cross the World publication boundary.
#[test]
fn reference_decode_applies_exif_orientation_before_publication() {
    use image::ImageEncoder as _;

    let source = image::RgbImage::from_fn(3, 2, |x, y| image::Rgb([(x * 70 + y * 13) as u8, (y * 100 + x * 9) as u8, (x * 30 + y * 40) as u8]));
    let mut normal = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut normal, 100)
        .write_image(source.as_raw(), source.width(), source.height(), image::ExtendedColorType::Rgb8)
        .expect("encode asymmetric orientation fixture");
    let unrotated = decode_reference_image_bytes(&normal).expect("normal JPEG decodes");
    assert_eq!((unrotated.width, unrotated.height), (3, 2), "orientation 1 preserves intrinsic dimensions");

    let exif6 = [0xff, 0xe1, 0, 34, 69, 120, 105, 102, 0, 0, 73, 73, 42, 0, 8, 0, 0, 0, 1, 0, 18, 1, 3, 0, 1, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0];
    let mut oriented = Vec::with_capacity(normal.len() + exif6.len());
    oriented.extend_from_slice(&normal[..2]);
    oriented.extend_from_slice(&exif6);
    oriented.extend_from_slice(&normal[2..]);
    let rotated = decode_reference_image_bytes(&oriented).expect("EXIF-6 JPEG decodes");
    assert_eq!((rotated.width, rotated.height), (2, 3), "orientation 6 rotates source dimensions clockwise");
    for y in 0..unrotated.height as usize {
        for x in 0..unrotated.width as usize {
            let source_offset = (y * unrotated.width as usize + x) * 4;
            let target_x = unrotated.height as usize - 1 - y;
            let target_y = x;
            let target_offset = (target_y * rotated.width as usize + target_x) * 4;
            assert_eq!(&rotated.pixels[target_offset..target_offset + 4], &unrotated.pixels[source_offset..source_offset + 4], "orientation 6 preserves the decoded corner mapping");
        }
    }
}

fn distance(left: [u8; 4], right: [u8; 4]) -> i32 {
    (0..3).map(|channel| (i32::from(left[channel]) - i32::from(right[channel])).abs()).sum()
}

#[test]
fn terrain_mesh_fault_preserves_busy_and_semantic_owner_refusals() {
    for fault in [ui_scene::math::Mesh3dFault::Busy, ui_scene::math::Mesh3dFault::Closing, ui_scene::math::Mesh3dFault::Schema, ui_scene::math::Mesh3dFault::Stale] {
        assert_eq!(WorldTerrainMeshCursor::mesh_fault(fault), WorldDynamicFault::Mesh(fault));
    }
}

#[test]
fn terrain_mesh_zero_grant_retains_original_payload_and_phase() {
    let payload = TerrainTileMeshPayload { positions: vec![0.0; 9], normals: vec![0.0; 9], indices: vec![0,1,2], uvs: vec![0.0; 6] };
    let pointer = payload.positions.as_ptr();
    let capacity = payload.positions.capacity();
    let mut cursor = terrain_test_admit("surface", (3,4,5), payload, 90, 9, 9).expect("original cursor");
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items:0, maximum_copy_bytes:0, maximum_capacity_bytes:0, maximum_release_bytes:0, maximum_depth:0 };
    let (step, progress) = cursor.step_live(grant).expect("zero grant pauses");
    assert!(matches!(step, WorldTerrainMeshStep::Pending));
    assert_eq!(progress, semio_framework_value::retained_clone::RetainedCloneProgress::default());
    assert_eq!(cursor.phase, WorldTerrainMeshPhase::Begin);
    assert_eq!(cursor.payload.positions.as_ptr(), pointer);
    assert_eq!(cursor.payload.positions.capacity(), capacity);
    assert_eq!(cursor.payload.positions.len(), 9);
    let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:4096, maximum_capacity_bytes:65536, maximum_release_bytes:65536, maximum_depth:64 };
    for _ in 0..512 {
        let step = cursor.close_step(grant).expect("finite original source close");
        assert!(step.progress.fits(grant));
        if step.complete { break; }
    }
    assert!(cursor.terminal_is_empty());
}

fn terrain_test_grant()->semio_framework_value::retained_clone::RetainedCloneGrant {semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:16*1024*1024,maximum_depth:64}}

fn terrain_test_admit(surface:&str,tile:(u32,u32,u32),payload:TerrainTileMeshPayload,generation:u64,revision:u64,terrain_revision:u64)->Result<WorldTerrainMeshCursor,WorldDynamicFault> {
    WorldTerrainMeshCursor::new(surface,tile,payload,generation,revision,terrain_revision,terrain_test_grant()).map(|(cursor,progress)|{assert!(progress.fits(terrain_test_grant()));cursor}).map_err(|(fault,_)|fault)
}

fn terrain_test_step(cursor:&mut WorldTerrainMeshCursor,revision:u64,terrain_revision:u64)->WorldTerrainMeshStep {
    with_world_step_context(1,|context|{let (step,progress)=cursor.step(revision,terrain_revision,terrain_test_grant(),context).expect("typed finite terrain turn");assert!(progress.fits(terrain_test_grant()));step})
}

fn terrain_test_close(cursor:&mut WorldTerrainMeshCursor)->bool {
    match cursor.close_step(terrain_test_grant()) {
        Ok(step)=>{assert!(step.progress.fits(terrain_test_grant()));step.complete}
        Err(WorldDynamicFault::Mesh(ui_scene::math::Mesh3dFault::Busy))=>false,
        Err(fault)=>panic!("semantic terrain owner refusal: {fault:?}"),
    }
}

#[test]
fn terrain_constructor_refusal_preserves_original_payload_pointer_and_capacity() {
    let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
    let payload=TerrainTileMeshPayload{positions:vec![0.0;9],normals:vec![0.0;9],indices:vec![0,1,2],uvs:vec![0.0;6]};
    let pointer=payload.positions.as_ptr();let capacity=payload.positions.capacity();
    let Err((fault,payload))=WorldTerrainMeshCursor::new("surface",(0,0,0),payload,91,1,1,grant) else {panic!("unfunded ID must refuse")};
    assert_eq!(fault,WorldDynamicFault::Ownership(semio_framework_value::ValueRefusalKind::OwnershipLimit));
    assert_eq!(payload.positions.as_ptr(),pointer);assert_eq!(payload.positions.capacity(),capacity);assert_eq!(payload.positions.len(),9);
}

#[test]
fn terrain_owner_yield_and_copy_1_3_64_keep_the_original_write_cursor() {
    for maximum_copy_bytes in [1,3,64] {
        let payload=TerrainTileMeshPayload{positions:vec![0.0;9],normals:vec![0.0;9],indices:vec![0,1,2],uvs:vec![0.0;6]};
        let mut cursor=terrain_test_admit("surface",(0,0,0),payload,92,1,1).expect("funded original payload");
        let initial=cursor.payload.positions.as_ptr();
        with_world_step_context(0,|context|{let (step,progress)=cursor.step(1,1,terrain_test_grant(),context).unwrap();assert!(matches!(step,WorldTerrainMeshStep::Pending));assert_eq!(progress,Default::default());});
        assert_eq!(cursor.phase,WorldTerrainMeshPhase::Begin);assert_eq!(cursor.payload.positions.as_ptr(),initial);
        for _ in 0..512 {if cursor.phase==WorldTerrainMeshPhase::Positions {break;}terrain_test_step(&mut cursor,1,1);}
        assert_eq!(cursor.phase,WorldTerrainMeshPhase::Positions);
        let grant=semio_framework_value::retained_clone::RetainedCloneGrant {maximum_copy_bytes,..terrain_test_grant()};
        let (step,progress)=cursor.step_live(grant).expect("caller byte budget");
        assert!(matches!(step,WorldTerrainMeshStep::Pending));assert!(progress.fits(grant));
        assert_eq!(cursor.vertex,if maximum_copy_bytes<12 {0} else {1});
        for _ in 0..512 {if terrain_test_close(&mut cursor) {break;}}
        assert!(cursor.terminal_is_empty());
    }
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}

#[test]
fn original_asset_page_ordinals_retain_gaps_until_complete_seal(){
    let mut lane=WorldAssetIoAuthority::default();let token=lane.reserve(41,9,WorldAssetRequestKind::Glb,"original-order",3).unwrap();let mut fetch=lane.take_next().unwrap();
    for index in[2u16,1,0]{fetch.push_page_at(index,WorldAssetResponsePage::try_from_owned(vec![index as u8]).unwrap()).unwrap();if index!=0{assert_eq!(fetch.seal(),Err(WorldAssetFault::Incomplete));}}
    let duplicate=WorldAssetResponsePage::try_from_owned(vec![7]).unwrap();assert_eq!(fetch.push_page_at(1,duplicate).unwrap_err().bytes(),&[7]);fetch.seal().unwrap();
    for index in 0..3{let page=fetch.take_decode_page().unwrap().unwrap();assert_eq!(page.bytes(),&[index]);}assert!(fetch.take_decode_page().unwrap().is_none());fetch.begin_close();while !fetch.close_step(){}lane.finish(fetch).unwrap();assert!(lane.terminal_is_empty());let _=token;
}
