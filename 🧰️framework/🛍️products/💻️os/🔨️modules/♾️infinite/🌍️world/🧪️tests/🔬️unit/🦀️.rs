use super::*;
use ui_wgpu::wgpu::{SurfaceKind, UiComponentSceneNode, UiPresence, World3dScene, mesh3d_write_edge};

fn test_scene_raster_lease(width: u32, height: u32, profile: SceneRasterProfile, mesh: Option<SceneRasterMeshSeal>, fill: u8) -> SceneRasterLease {
    static REVISION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let revision = REVISION.fetch_add(1, std::sync::atomic::Ordering::Relaxed).max(1);
    let descriptor = SceneRasterDescriptor { width, height, source_digest: [revision, revision.rotate_left(17)], source_revision: revision, profile, mesh };
    let pool = world_scene_raster_pool();
    let mut writer = None;
    for _ in 0..=ui_wgpu::wgpu::SCENE_RASTER_POOL_SLOTS {
        match pool.begin(descriptor, revision, SceneRasterWriteMode::Moved) {
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

pub(super) fn take_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority live") {
        actions.push(action.into_descriptor().expect("bounded action materializes"));
    }
    actions
}

fn triangle_mesh_oracle() -> LegacyMeshOracleData {
    mesh_oracle_from_buffers(vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])
}

pub(super) fn mesh_oracle_from_buffers(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> LegacyMeshOracleData {
    LegacyMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors: Vec::new() }
}

pub(super) fn publish_oracle_mesh(data: LegacyMeshOracleData) -> Mesh3dLease {
    publish_oracle_mesh_at_revision(data, 0)
}

/// 🧊️ An oracle mesh stamped with a chosen interaction revision — what
/// `publish_world3d_asset_mesh_lease` witnesses before it stores a decoded GLB under its url's id.
pub(super) fn publish_oracle_mesh_at_revision(data: LegacyMeshOracleData, revision: u64) -> Mesh3dLease {
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
    };
    let generation = GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let token = mesh3d_begin(generation, revision, schema).expect("oracle mesh claim");
    while !mesh3d_allocate_step(token).expect("oracle mesh page allocation") {}
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

fn assert_send<T: Send>() {}

pub(super) fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
    let mut sequence = 0;
    let mut context = semio_framework_job::StepContext::new(
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(1),
        semio_framework_job::StepBudget::new(fuel, u64::MAX),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut sequence,
    );
    step(&mut context)
}

#[test]
fn cursor_wake_coalesces_duplicates_and_rearms_after_exact_take() {
    let wake = WorldCursorWakeAuthority::new();
    let mut first_frame = World3dBuildContext::new(wake.clone());
    first_frame.request_cursor_wake();
    first_frame.request_cursor_wake();
    let first = first_frame.take_cursor_wake().unwrap().expect("first frame wake token");
    assert_eq!(first_frame.take_cursor_wake().unwrap(), None, "one frame carries one exact token");
    for _ in 0..128 {
        assert_eq!(wake.request().unwrap().generation(), first.generation(), "a pending wake storm coalesces onto one retained generation");
    }

    let mut consecutive_frame = World3dBuildContext::new(wake.clone());
    consecutive_frame.request_cursor_wake();
    let duplicate = consecutive_frame.take_cursor_wake().unwrap().expect("pending wake coalesces across frames");
    assert_eq!(duplicate.generation(), first.generation());
    assert!(wake.acknowledge(&first));
    assert!(!wake.acknowledge(&duplicate), "duplicate acknowledgement cannot consume another generation");

    let mut resumed_frame = World3dBuildContext::new(wake.clone());
    resumed_frame.request_cursor_wake();
    let resumed = resumed_frame.take_cursor_wake().unwrap().expect("resumed work receives another token");
    assert!(resumed.generation() > first.generation(), "consecutive presented frames retain an ABA-distinguishable generation");
    assert!(!wake.acknowledge(&duplicate), "stale token cannot consume the newer pending generation");
    assert_eq!(wake.pending_generation(), Some(resumed.generation()));
    assert!(wake.acknowledge(&resumed));
    assert_eq!(wake.pending_generation(), None);

    let pending = wake.request().expect("close fixture pending wake");
    assert_eq!(wake.pending_generation(), Some(pending.generation()));
    assert!(!wake.close_step(), "first close grant retires only the pending token scalar");
    assert!(!wake.close_step(), "second close grant retires only the generation scalar");
    assert!(!wake.close_step(), "third close grant retires only the acknowledgement scalar");
    assert!(wake.close_step());
    assert!(wake.terminal_is_empty());
    assert_eq!(wake.request(), Err(WorldCursorWakeFault::Closed));
}

#[test]
fn live_renderer_retains_generation_wake_and_rejects_recreation_erasure_loss_and_duplicate_consumption() {
    const TOKEN_FIELD: &str = "cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>";
    const HOST_TOKEN_FIELD: &str = "cursor_wake_requested: Option<infinite_world::world::WorldCursorWakeToken>";

    fn region<'a>(source: &'a str, start: &str, end: &str) -> Option<&'a str> {
        let start = source.find(start)?;
        let end = source[start..].find(end)?.checked_add(start)?;
        Some(&source[start..end])
    }

    fn replace_region(source: &str, start: &str, end: &str, from: &str, to: &str) -> Option<String> {
        let start = source.find(start)?;
        let end = source[start..].find(end)?.checked_add(start)?;
        let owned = source[start..end].replacen(from, to, 1);
        let mut output = String::with_capacity(source.len().saturating_add(to.len()).saturating_sub(from.len()));
        output.push_str(&source[..start]);
        output.push_str(&owned);
        output.push_str(&source[end..]);
        Some(output)
    }

    fn exact(glue: &str, host: &str, native: &str, browser: &str) -> bool {
        let typed_handoffs = [
            ("AppFrameBuild", "pub(crate) struct AppFrameBuild {", "struct AppFrameAfterChrome {"),
            ("AppFrameAfterChrome", "struct AppFrameAfterChrome {", "struct FrameBuildCursor {"),
            ("FrameBuildCursor", "struct FrameBuildCursor {", "struct FrameWheelCursor {"),
            ("AppFramePresentation", "pub(crate) struct AppFramePresentation {", "impl AppFrameBuild {"),
            ("AppFramePreparation", "pub(crate) struct AppFramePreparation {", "impl AppFramePreparation {"),
            ("AppPresentStep::Complete", "pub(crate) enum AppPresentStep {", "impl AppPresenter {"),
        ];
        let host_handoffs = [("OsHost", "pub struct OsHost {", "struct OsHostRetirementState {"), ("OsHostRetirementState", "struct OsHostRetirementState {", "pub(crate) struct OsHostRetirement {")];
        glue.contains("world_cursor_wake: infinite_world::world::WorldCursorWakeAuthority")
            && glue.contains("World3dBuildContext::new(runtime.world_cursor_wake_authority())")
            && glue.matches(TOKEN_FIELD).count() == typed_handoffs.len()
            && typed_handoffs.iter().all(|(_, start, end)| region(glue, start, end).is_some_and(|handoff| handoff.matches(TOKEN_FIELD).count() == 1))
            && region(glue, "pub(crate) enum AppPresentStep {", "impl AppPresenter {").is_some_and(|handoff| {
                handoff.contains("Complete { generation: semio_framework_trace::Generation, cursor: SemioCursor, fullscreen: Option<bool>, cursor_wake: Option<infinite_world::world::WorldCursorWakeToken> }")
                    && handoff.matches(TOKEN_FIELD).count() == 1
            })
            && region(glue, "pub(crate) struct AppPresenter {", "#[derive(Clone, Copy, Debug, PartialEq, Eq)]").is_some_and(|presenter| presenter.matches("pending: Option<AppPresentCursor>").count() == 1)
            && region(glue, "struct AppPresentCursor {", "pub(crate) enum AppPresentStep {").is_some_and(|presenter| presenter.matches("frame: AppFramePresentation").count() == 1)
            && !glue.contains("cursor_wake: bool")
            && !glue.contains("cursor_wake: Option<bool>")
            && !glue.contains("request_frame: bool")
            && glue.contains("fn close_world_cursor_wake_step(&self) -> bool")
            && glue.contains("pub(crate) fn close_cursor_wake_step(&mut self) -> bool")
            && !glue.contains("World3dBuildContext::default()")
            && host.matches(HOST_TOKEN_FIELD).count() == host_handoffs.len()
            && host_handoffs.iter().all(|(_, start, end)| region(host, start, end).is_some_and(|handoff| handoff.matches(HOST_TOKEN_FIELD).count() == 1))
            && !host.contains("cursor_wake_requested: bool")
            && !host.contains("cursor_wake_requested: Option<bool>")
            && host.contains("token.generation() > pending.generation()")
            && host.contains("if self.cursor_wake_requested.take().is_some()")
            && native.contains("self.runtime.acknowledge_world_cursor_wake(&token)")
            && native.contains("self.retain_cursor_wake_directive(token)")
            && native.matches("host.take_cursor_wake_directive()").count() == 1
            && browser.contains("host.take_cursor_wake_directive().is_some()")
    }

    let glue = include_str!("../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
    let host = include_str!("../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs");
    let native = include_str!("../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs");
    let browser = include_str!("../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs");
    assert!(exact(glue, host, native, browser));
    assert!(!exact(&glue.replace("World3dBuildContext::new(runtime.world_cursor_wake_authority())", "World3dBuildContext::default()"), host, native, browser));
    for (name, start, end) in [
        ("AppFrameBuild", "pub(crate) struct AppFrameBuild {", "struct AppFrameAfterChrome {"),
        ("AppFrameAfterChrome", "struct AppFrameAfterChrome {", "struct FrameBuildCursor {"),
        ("FrameBuildCursor", "struct FrameBuildCursor {", "struct FrameWheelCursor {"),
        ("AppFramePresentation", "pub(crate) struct AppFramePresentation {", "impl AppFrameBuild {"),
        ("AppFramePreparation", "pub(crate) struct AppFramePreparation {", "impl AppFramePreparation {"),
        ("AppPresentStep::Complete", "pub(crate) enum AppPresentStep {", "impl AppPresenter {"),
    ] {
        let erased = replace_region(glue, start, end, TOKEN_FIELD, "request_frame: bool").expect("enumerated typed wake handoff");
        assert!(!exact(&erased, host, native, browser), "erasing {name} must violate the exact typed-token handoff census");
    }
    let erased_all = glue.replace(TOKEN_FIELD, "request_frame: bool");
    assert!(!exact(&erased_all, host, native, browser), "erasing every internal typed wake handoff must be rejected");
    let extra_bool = glue.replacen("pub(crate) struct AppFrameBuild {", "pub(crate) struct AppFrameBuild {\n    request_frame: bool,", 1);
    assert!(!exact(&extra_bool, host, native, browser), "an extra internal boolean wake channel must be rejected");
    let presenter_erased = glue.replacen("frame: AppFramePresentation", "request_frame: bool", 1);
    assert!(!exact(&presenter_erased, host, native, browser), "presenter ownership must retain the typed presentation handoff");
    let pending_presenter_erased = glue.replacen("pending: Option<AppPresentCursor>", "request_frame: bool", 1);
    assert!(!exact(&pending_presenter_erased, host, native, browser), "AppPresenter must retain its typed pending cursor");
    for (name, start, end) in [("OsHost", "pub struct OsHost {", "struct OsHostRetirementState {"), ("OsHostRetirementState", "struct OsHostRetirementState {", "pub(crate) struct OsHostRetirement {")] {
        let erased = replace_region(host, start, end, HOST_TOKEN_FIELD, "cursor_wake_requested: bool").expect("enumerated host token field");
        assert!(!exact(glue, &erased, native, browser), "erasing {name}'s retained host token must be rejected");
    }
    assert!(!exact(glue, host, &native.replace("self.runtime.acknowledge_world_cursor_wake(&token)", "true"), browser));
    assert!(!exact(glue, host, &native.replace("self.retain_cursor_wake_directive(token)", "drop(token)"), browser));
    assert!(!exact(glue, &host.replace("token.generation() > pending.generation()", "true"), native, browser));
    assert!(!exact(glue, &host.replace("if self.cursor_wake_requested.take().is_some()", "if false"), native, browser));
    assert!(!exact(glue, host, native, &browser.replace("host.take_cursor_wake_directive().is_some()", "host.cursor_wake_requested.is_some()")));
}

#[test]
fn terrain_visibility_requires_the_complete_family_marker() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let tile = (3, 4, 5);
    assert!(!terrain_family_visible(&state, tile));
    state.terrain_built_tiles.insert(tile);
    assert!(terrain_family_visible(&state, tile));
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

    let mut cursor = WorldTerrainMeshCursor::new("surface", (3, 4, 5), payload(), 50, 9, 9).expect("terrain cursor");
    let mut published = 0;
    let mut turns = 0;
    loop {
        turns += 1;
        assert!(turns < 512);
        match cursor.step(9, 9) {
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
                while !mesh3d_close_step(lease).unwrap() {}
            }
            WorldTerrainMeshStep::Complete(tile) => {
                assert_eq!(tile, (3, 4, 5));
                assert_eq!(published, 1, "a tile publishes exactly one mesh");
                break;
            }
            WorldTerrainMeshStep::Fault => panic!("valid terrain cursor faulted"),
        }
    }
    assert!(cursor.terminal_is_empty());

    let mut interrupted = WorldTerrainMeshCursor::new("surface", (3, 4, 5), payload(), 70, 11, 11).expect("terrain cursor");
    assert!(matches!(interrupted.step(11, 11), WorldTerrainMeshStep::Pending));
    let mut close_turns = 0;
    while !interrupted.close_step() || !interrupted.terminal_is_empty() {
        close_turns += 1;
        assert!(close_turns < 64);
    }
    assert!(interrupted.terminal_is_empty());
    assert!(WorldTerrainMeshCursor::new("surface", (0, 0, 0), TerrainTileMeshPayload { positions: vec![0.0, 1.0], normals: Vec::new(), indices: vec![0, 1, 2], uvs: Vec::new() }, 80, 1, 1).is_err());
}

fn face_overlay_test_mesh_with_faces(generation: u64, revision: u64, face_ids: &[u32]) -> Mesh3dLease {
    let triangles = u32::try_from(face_ids.len()).expect("fixture triangle count");
    let vertices = triangles * 3;
    let schema = Mesh3dSchema { vertices, indices: vertices, face_ids: triangles, vertex_ids: 0, edges: 0, edge_ids: 0, uvs: 0, colors: 0 };
    let token = mesh3d_begin(generation, revision, schema).expect("face overlay fixture claim");
    while !mesh3d_allocate_step(token).expect("face overlay fixture page") {}
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
    state.interaction_revision = 7;
    state.draw_generation = 11;
    state.granularity = "face".into();
    state.component_ids.push("12".into());
    let source = face_overlay_test_mesh(400, 7, 12);
    publish_world3d_mesh_lease(&mut state, "source".into(), source).unwrap();
    let version = *state.mesh_versions.get("source").expect("source mesh version");
    state.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();

    let mut cursor = WorldFaceOverlayMeshCursor::new("surface", 500, 7, 11).unwrap();
    let mut published = None;
    for _ in 0..128 {
        match cursor.step(&state) {
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
    while !mesh3d_close_step(published).unwrap() {}

    let mut stale = WorldFaceOverlayMeshCursor::new("surface", 600, 7, 11).unwrap();
    assert!(matches!(stale.step(&state), WorldFaceOverlayMeshStep::Pending));
    state.interaction_revision = 8;
    let mut terminal = false;
    for _ in 0..32 {
        if matches!(stale.step(&state), WorldFaceOverlayMeshStep::Stale) {
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
    state.interaction_revision = 7;
    state.draw_generation = 11;
    state.granularity = "face".into();
    state.marquee_preview_ids.push("10".into());
    state.component_ids.push("12".into());
    state.hovered_component_id = Some("11".into());
    state.hovered_component_object_id = Some("object".into());
    state.hovered_component_mode = Some("face".into());
    let source = face_overlay_test_mesh_with_faces(750, 7, &[10, 11, 12]);
    publish_world3d_mesh_lease(&mut state, "source".into(), source).unwrap();
    let version = *state.mesh_versions.get("source").expect("source mesh version");
    state.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() }).unwrap();
    state.face_overlay_build = Some(WorldFaceOverlayMeshCursor::new("surface", 800, 7, 11).unwrap());

    let mut staged_seen = false;
    for _ in 0..1_024 {
        step_component_face_overlay_build(&mut state);
        if state.meshes.contains_key("component-face-overlay:surface:800:0") && state.face_overlay_generation.is_none() {
            staged_seen = true;
        }
        if state.face_overlay_generation == Some(800) {
            break;
        }
    }
    assert!(staged_seen, "a completed bucket remains invisible while the family transaction is partial");
    assert_eq!(state.face_overlay_generation, Some(800));
    assert_eq!(state.face_overlay_colors, [Some([1.0, 0.85, 0.35, 0.36]), Some([0.35, 0.75, 1.0, 0.48]), Some([0.35, 0.75, 1.0, 0.62])]);
    for index in 0..3 {
        assert!(state.meshes.contains_key(&format!("component-face-overlay:surface:800:{index}")), "every nonempty category publishes before family visibility");
    }

    state.face_overlay_build = Some(WorldFaceOverlayMeshCursor::new("surface", 900, 7, 11).unwrap());
    for _ in 0..1_024 {
        step_component_face_overlay_build(&mut state);
        if state.meshes.contains_key("component-face-overlay:surface:900:0") {
            break;
        }
    }
    assert_eq!(state.face_overlay_generation, Some(800), "superseding partial family never changes the visible generation");
    state.interaction_revision = 8;
    for _ in 0..64 {
        step_component_face_overlay_build(&mut state);
        if state.face_overlay_build.is_none() {
            break;
        }
    }
    assert_eq!(state.face_overlay_generation, Some(800), "stale family retirement preserves the last complete publication");
    assert!(state.meshes.contains_key("component-face-overlay:surface:900:0"), "stale partial publication remains owned by the generation-qualified registry until bounded retirement");
}

#[test]
fn world_flat_plan_enforces_exact_item_and_byte_caps() {
    let mut bytes = WorldInteractionPlan::new(1, 1);
    assert!(bytes.push_string(&"x".repeat(WORLD_INTERACTION_BYTE_CAPACITY)).is_some());
    assert!(bytes.push_string("x").is_none());
    assert!(bytes.faulted);

    let mut items = WorldInteractionPlan::new(1, 1);
    let action = WorldFlatAction { kind: WorldFlatActionKind::Camera, strings: [None; 8], numbers: [0.0; 10] };
    for _ in 0..WORLD_INTERACTION_ITEM_CAPACITY {
        assert!(items.push_action(action));
    }
    assert!(!items.push_action(action));
    assert!(items.faulted);
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
    assert_eq!(mesh3d_begin(99, 0, Mesh3dSchema::triangle_mesh(0, 0)), Err(ui_wgpu::wgpu::Mesh3dFault::Schema));
}

#[test]
fn world_object_registry_enforces_capacity_revision_and_aba() {
    let mut registry = WorldInteractionObjectRegistry::default();
    let first = registry.admit(1, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [0.0; 8]).expect("first object token");
    assert_eq!(registry.admit(1, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [0.0; 8]), Some(first));
    let replacement = registry.admit(1, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [1.0; 8]).expect("same-revision replacement");
    assert_ne!(replacement, first);
    assert!(registry.resolve(first).is_none());
    let next_revision = registry.admit(2, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [1.0; 8]).expect("new revision replacement");
    assert_ne!(next_revision, replacement);
    assert!(registry.resolve(replacement).is_none());
    for index in 1..WORLD_INTERACTION_OBJECT_CAPACITY {
        assert!(registry.admit(2, WorldInteractionObjectKind::Instance, &format!("object-{index}"), None, Mat4::identity(), [index as f32; 8]).is_some());
    }
    assert!(registry.admit(2, WorldInteractionObjectKind::Instance, "object-overflow", None, Mat4::identity(), [0.0; 8]).is_none());
    assert!(registry.faulted);

    let mut oversized = WorldInteractionObjectRegistry::default();
    assert!(oversized.admit(1, WorldInteractionObjectKind::Reference, &"x".repeat(WORLD_INTERACTION_ID_BYTE_CAPACITY + 1), None, Mat4::identity(), [0.0; 8]).is_none());
    assert!(oversized.faulted);
}

#[test]
fn world_object_registry_build_is_one_owner_per_turn_and_interruptible() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_revision = 7;
    let mesh = publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]));
    store_mesh(&mut state, "mesh".into(), mesh);
    let mesh_version = *state.mesh_versions.get("mesh").expect("mesh version");
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { id: "instance".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.vortices.push(WorldVortexRecord { full_id: "vortex".into(), position: Some([1.0, 2.0, 3.0]), radius: Some(0.5), ..Default::default() });
    state.references.push(WorldReferenceRecord { url: Some("reference".into()), origin: Some([4.0, 5.0, 6.0]), width_world: Some(2.0), hidden: Some(false), ..Default::default() });
    state.reference_pixels.insert("reference".into(), test_world_scene_raster(2, 1, 0));

    let mut cursor = WorldInteractionRegistryBuildCursor::new(7);
    assert_eq!(with_world_step_context(0, |context| cursor.step(&mut state, context)), WorldInteractionStep::Pending);
    let mut turns = 0;
    while with_world_step_context(1, |context| cursor.step(&mut state, context)) == WorldInteractionStep::Pending {
        turns += 1;
        assert!(turns < 16);
    }
    assert!(state.interaction_objects.terminal_for_revision(7));
    let live: Vec<_> = state.interaction_objects.slots.iter().flatten().filter(|slot| slot.revision == 7).collect();
    assert_eq!(live.len(), 3);
    assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Instance && slot.id.as_str() == "instance"));
    assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Vortex && slot.id.as_str() == "vortex"));
    assert!(live.iter().any(|slot| slot.kind == WorldInteractionObjectKind::Reference && slot.id.as_str() == "reference"));

    let token = state
        .interaction_objects
        .slots
        .iter()
        .enumerate()
        .find_map(|(index, slot)| slot.as_ref().filter(|slot| slot.kind == WorldInteractionObjectKind::Reference).map(|slot| WorldInteractionObjectToken { slot: index as u16, generation: slot.generation, revision: slot.revision }))
        .expect("reference token");
    assert_eq!(state.interaction_objects.resolve(token).map(|slot| slot.values[4]), Some(1.0));

    let mut interrupted = WorldInteractionRegistryBuildCursor::new(8);
    assert!(with_world_step_context(1, |context| interrupted.close_step(context)));
    assert_eq!(interrupted.phase, WorldInteractionRegistryBuildPhase::Complete);
}

#[test]
fn world_vortex_and_reference_pick_cursors_resume_revalidate_and_close() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_revision = 3;
    state.interaction_objects.revision = 3;
    let vortex = state.interaction_objects.admit(3, WorldInteractionObjectKind::Vortex, "vortex", None, Mat4::identity(), [0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.5, 0.0]).expect("vortex token");
    state.interaction_objects.admit(3, WorldInteractionObjectKind::Reference, "reference", None, Mat4::identity(), [0.0, 0.0, 0.0, 2.0, 1.0, 0.0, 0.0, 0.0]).expect("reference token");

    let mut vortex_cursor = WorldObjectPickCursor::from_ray(3, 8, WorldObjectPickPurpose::VortexSelect, Vec3::new(0.0, -2.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(with_world_step_context(0, |context| vortex_cursor.step(&state, 8, context)), WorldInteractionStep::Pending);
    for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
        if with_world_step_context(1, |context| vortex_cursor.step(&state, 8, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    assert_eq!(vortex_cursor.best.map(|(token, _)| token), Some(vortex));
    let mut plan = vortex_cursor.finish_plan(&state, 8).expect("vortex plan").expect("vortex hit");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);

    state.interaction_objects.revision = state.interaction_revision;
    let mut reference_cursor = WorldObjectPickCursor::from_ray(state.interaction_revision, 9, WorldObjectPickPurpose::ReferenceHover, Vec3::new(0.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
    for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
        if with_world_step_context(1, |context| reference_cursor.step(&state, 9, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    assert!(reference_cursor.best.is_some());
    state.interaction_revision = state.interaction_revision.wrapping_add(1);
    assert!(matches!(reference_cursor.finish_plan(&state, 9), Err(WorldInteractionStep::Stale)));
    assert!(!reference_cursor.close_step());
    assert!(reference_cursor.close_step());
}

#[test]
fn world_component_cursor_steps_one_topology_item_and_rejects_aba() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_revision = 5;
    state.interaction_objects.revision = 5;
    state.granularity = "face".into();
    let mesh = publish_oracle_mesh(triangle_mesh_oracle());
    store_mesh(&mut state, "mesh".into(), mesh);
    let mesh_token = state
        .interaction_meshes
        .slots
        .iter()
        .enumerate()
        .find_map(|(index, slot)| slot.as_ref().filter(|slot| slot.id.as_str() == "mesh").map(|slot| WorldInteractionMeshToken { slot: index as u16, generation: slot.generation }))
        .expect("mesh token");
    let object = state.interaction_objects.admit(5, WorldInteractionObjectKind::Instance, "object", Some(mesh_token), Mat4::identity(), [0.0; 8]).expect("object token");
    let mut cursor = WorldComponentPickCursor {
        revision: 5,
        generation: 10,
        purpose: WorldComponentPickPurpose::Select,
        kind: WorldComponentKind::Face,
        local_x: 0.0,
        local_y: 0.0,
        viewport: Rect { x: 0.0, y: 0.0, w: 100.0, h: 100.0 },
        view_projection: Mat4::identity(),
        origin: Vec3::new(0.0, 0.0, 2.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
        slot: object.slot,
        current: None,
        topology: 0,
        merge: 0,
        best: None,
        complete: false,
    };
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 10, context)), WorldInteractionStep::Pending);
    assert_eq!(cursor.current, Some(object));
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 10, context)), WorldInteractionStep::Pending);
    assert_eq!(cursor.topology, 1);
    assert_eq!(cursor.best.map(|hit| hit.id), Some(0));
    cursor.current = None;
    cursor.slot = WORLD_INTERACTION_OBJECT_CAPACITY as u16;
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 10, context)), WorldInteractionStep::Pending);
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 10, context)), WorldInteractionStep::Complete);
    let mut plan = cursor.finish_plan(&state, 10).expect("component plan").expect("component hit");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert_eq!(take_actions(&mut input).len(), 1);

    state.interaction_objects.revision = state.interaction_revision;
    let mut replacement_model = Mat4::identity();
    replacement_model.cols[3][0] = 1.0;
    let replacement = state.interaction_objects.admit(state.interaction_revision, WorldInteractionObjectKind::Instance, "object", Some(mesh_token), replacement_model, [0.0; 8]).expect("replacement object token");
    assert_ne!(replacement, object);
    cursor.revision = state.interaction_revision;
    cursor.complete = true;
    cursor.best = Some(WorldComponentHit { object, id: 0, primary: 1.0, secondary: 0.0 });
    assert!(matches!(cursor.finish_plan(&state, 10), Err(WorldInteractionStep::Stale)));
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
    state.interaction_revision = 4;
    state.interaction_objects.revision = 4;
    state.hovered_vortex_id = Some("vortex".into());
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut authority = WorldInteractionAuthority::default();
    authority.next_generation = 4;
    authority.queue.push(WorldInteractionIntent::pointer_button(0.0, 0.0, true, 2, &PointerModifiers::default())).unwrap();
    authority.queue.slots[0].as_mut().unwrap().generation = 1;
    authority.queue.push(WorldInteractionIntent::pointer_move(10.0, 0.0, 10.0, 0.0, true, 2, &PointerModifiers::default())).unwrap();
    authority.queue.slots[1].as_mut().unwrap().generation = 2;
    authority.queue.push(WorldInteractionIntent::pointer_button(10.0, 0.0, false, 2, &PointerModifiers::default())).unwrap();
    authority.queue.slots[2].as_mut().unwrap().generation = 3;
    state.interaction_authority = Some(authority);
    state.interaction_objects.revision = state.interaction_revision;
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
#[test]
fn an_intent_aimed_outside_the_surface_retires_instead_of_faulting_the_frame() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.bounds = Rect { x: 536.0, y: 54.0, w: 1061.0, h: 913.0 };
    state.interaction_objects.revision = state.interaction_revision;
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut authority = WorldInteractionAuthority::default();
    authority.next_generation = 2;
    authority.queue.push(WorldInteractionIntent::pointer_move(534.4, 500.0, -34.0, -453.0, false, 0, &PointerModifiers::default())).unwrap();
    authority.queue.slots[0].as_mut().unwrap().generation = 1;
    state.interaction_authority = Some(authority);
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Complete, "🖱️ a hover the pane no longer covers is retired");
    assert_eq!(world3d_interaction_front_generation(&state), None, "🖱️ and it leaves the queue rather than blocking it");
    assert!(!state.interaction_authority.as_ref().expect("authority").faulted, "🖱️ the authority stays usable for the next gesture");
    assert!(take_actions(&mut input).is_empty());

    // 🖱️ The button no CAMERA gesture claims still answers, and never faults — a middle press used to
    // fall through to the unclaimed-intent fault, which killed the page on the first middle-click of
    // any world surface. What it answers is the selection React publishes for any button (packet
    // W13c's `plan_world3d_non_primary_press_pick`, through R3F's `onPointerMissed`); the release
    // retires as before.
    let mut authority = WorldInteractionAuthority::default();
    authority.next_generation = 3;
    authority.queue.push(WorldInteractionIntent::pointer_button(1000.0, 400.0, true, 1, &PointerModifiers::default())).unwrap();
    authority.queue.slots[0].as_mut().unwrap().generation = 1;
    authority.queue.push(WorldInteractionIntent::pointer_button(1000.0, 400.0, false, 1, &PointerModifiers::default())).unwrap();
    authority.queue.slots[1].as_mut().unwrap().generation = 2;
    state.interaction_authority = Some(authority);
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
    assert!(!state.interaction_authority.as_ref().expect("authority").faulted);
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), ["interactionSelect"], "🖱️ the press publishes the selection React's own `handleEmptyClick` publishes for any button");
}

#[test]
fn world_marquee_overlay_points_reads_authority_gesture() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut gesture = WorldMarqueeGesture::new(1, 1, [10.0, 20.0]);
    assert!(gesture.push([100.0, 80.0]));
    state.interaction_authority.as_mut().unwrap().marquee = Some(gesture);
    assert!(!state.marquee_active);
    let points = world_marquee_overlay_points(&state).expect("overlay points");
    assert_eq!(points, vec![[10.0, 20.0], [100.0, 80.0]]);
}

#[test]
fn world_marquee_gesture_has_fixed_points_generation_and_cursorized_close() {
    let mut gesture = WorldMarqueeGesture::new(7, 11, [0.0, 0.0]);
    for index in 1..WORLD_INTERACTION_MARQUEE_POINT_CAPACITY {
        assert!(gesture.push([index as f32, 0.0]));
    }
    assert!(!gesture.push([999.0, 0.0]));
    assert!(!gesture.is_click([10.0, 0.0]));
    let mut turns = 0;
    while !gesture.close_step() {
        turns += 1;
        assert!(turns <= WORLD_INTERACTION_MARQUEE_POINT_CAPACITY);
    }
    assert_eq!(turns, WORLD_INTERACTION_MARQUEE_POINT_CAPACITY);
    assert_eq!(gesture.len, 0);
    assert!(gesture.points.iter().all(Option::is_none));
}

#[test]
fn world_marquee_click_retires_points_before_exact_release_retry() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_objects.revision = state.interaction_revision;
    let modifiers = PointerModifiers::default();
    let mut down = WorldInteractionIntent::pointer_button(0.0, 0.0, true, 0, &modifiers);
    down.generation = 1;
    let mut up = WorldInteractionIntent::pointer_button(1.0, 1.0, false, 0, &modifiers);
    up.generation = 2;
    state.interaction_authority.as_mut().unwrap().queue.push(down).unwrap();
    state.interaction_authority.as_mut().unwrap().queue.push(up).unwrap();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Complete);
    assert_eq!(state.interaction_authority.as_ref().unwrap().marquee.as_ref().map(|gesture| gesture.len), Some(1));
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Pending);
    assert!(state.interaction_authority.as_ref().unwrap().marquee.as_ref().is_some_and(|gesture| gesture.retiring));
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Stale);
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Stale);
    assert!(state.interaction_authority.as_ref().unwrap().marquee.is_none());
    assert_eq!(world3d_interaction_front_generation(&state), Some(2));
}

#[test]
fn world_marquee_result_pages_admit_exact_capacity_and_retire_one_target_per_grant() {
    let token = WorldInteractionObjectToken { slot: 0, generation: 1, revision: 1 };
    let mut pages = WorldMarqueeResultPages::default();
    for _ in 0..WORLD_MARQUEE_RESULT_PAGE_CAPACITY * WORLD_MARQUEE_RESULT_PAGE_COUNT {
        assert!(pages.push(WorldMarqueeResult::Object(token), 1));
    }
    assert_eq!(pages.page_len as usize, WORLD_MARQUEE_RESULT_PAGE_COUNT);
    assert!(!pages.push(WorldMarqueeResult::Object(token), 1));
    let mut turns = 0;
    while !pages.close_step() {
        turns += 1;
        assert!(turns <= WORLD_MARQUEE_RESULT_PAGE_CAPACITY * WORLD_MARQUEE_RESULT_PAGE_COUNT + WORLD_MARQUEE_RESULT_PAGE_COUNT);
    }
    assert_eq!(turns, WORLD_MARQUEE_RESULT_PAGE_CAPACITY * WORLD_MARQUEE_RESULT_PAGE_COUNT + WORLD_MARQUEE_RESULT_PAGE_COUNT);
    assert_eq!(pages.page_len, 0);
}

#[test]
fn world_marquee_pages_build_one_target_per_grant_and_publish_atomically_fifo() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_revision = 6;
    state.interaction_objects.revision = 6;
    let mut results = WorldMarqueeResultPages::default();
    for index in 0..=WORLD_MARQUEE_RESULT_PAGE_CAPACITY {
        let id = format!("target-{index:03}");
        let token = state.interaction_objects.admit(6, WorldInteractionObjectKind::Instance, &id, None, Mat4::identity(), [0.0; 8]).expect("marquee result token");
        assert!(results.push(WorldMarqueeResult::Object(token), id.len()));
    }
    let mut gesture = WorldMarqueeGesture::new(6, 7, [0.0, 0.0]);
    assert!(gesture.push([100.0, 100.0]));
    let mut job = WorldMarqueePublishJob::new(8, gesture, results, false, false);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    loop {
        turns += 1;
        let step = with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).expect("bounded marquee page step");
        if step == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 400);
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 2);
    // 🎯️ `targets` is a JSON-encoded STRING, not a structured array: the framework's own decoder is
    // `parse_interaction_targets`, which reads the arg with `DslValue::as_str`
    // (`💻️os/🔨️modules/🔌️plugin/🦀️.rs`), and the manifest declares it `ActionArgDef::text`. An array
    // makes `as_str` answer `None` and the guest faults the job — ticket
    // 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-world3d-interaction-2026-09-13.md`.
    let page_targets = |action: &ActionDescriptor| {
        let raw = action.args.as_ref().and_then(|args| args.get("targets")).and_then(|value| value.as_str()).expect("targets is JSON text");
        serde_json::from_str::<Vec<serde_json::Value>>(raw).expect("targets decodes as the framework does")
    };
    assert_eq!(page_targets(&actions[0]).len(), WORLD_MARQUEE_RESULT_PAGE_CAPACITY);
    assert_eq!(page_targets(&actions[1]).len(), 1);
    assert_eq!(page_targets(&actions[0])[0]["granularity"], resolved_domain_granularity_id(&state), "a marquee target uses the scene's declared interaction granularity");
    assert_eq!(job.results.page_len, 0);
    assert_eq!(job.gesture.len, 0);
}

#[test]
fn world_marquee_page_claim_saturation_preserves_all_results_for_exact_retry() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_revision = 6;
    state.interaction_objects.revision = 6;
    let id = "target";
    let token = state.interaction_objects.admit(6, WorldInteractionObjectKind::Instance, id, None, Mat4::identity(), [0.0; 8]).expect("target token");
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
    assert!(matches!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)));
    assert!(job.prepared.is_none());
    assert_eq!(job.results.lens[0], 1);
    input.release_action_claim(blockers.pop().expect("retry claim")).expect("release retry claim");
    assert_eq!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
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
/// (`🎬️scene/📐️math/🦀️.rs:1631`, `:1647`) and the oracle falls through to the instance-id arm, which
/// is not a component census at all.
fn component_triangle_mesh_oracle() -> LegacyMeshOracleData {
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

fn world_marquee_geometry_fixture_from(data: LegacyMeshOracleData, instance_count: usize) -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.pick_bounds = state.bounds;
    state.interaction_revision = 2;
    let mesh = publish_oracle_mesh(data);
    store_mesh(&mut state, "mesh".into(), mesh);
    let mesh_version = *state.mesh_versions.get("mesh").expect("mesh version");
    let instances = (0..instance_count).map(|index| Instance3d { id: format!("object-{index:03}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }).collect();
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances, shadow_role: Default::default() });
    let mut registry = WorldInteractionRegistryBuildCursor::new(2);
    let mut turns = 0;
    while with_world_step_context(1, |context| registry.step(&mut state, context)) == WorldInteractionStep::Pending {
        turns += 1;
        assert!(turns < instance_count + 16);
    }
    assert!(state.interaction_objects.terminal_for_revision(2));
    state
}

fn world_marquee_cursor_ids(state: &World3dState, points: &[[f32; 2]]) -> Vec<String> {
    let mut gesture = WorldMarqueeGesture::new(state.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(state, 8, gesture).expect("marquee cursor");
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| cursor.step(state, 8, context)) == WorldInteractionStep::Complete {
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
            ids.push(state.interaction_objects.resolve(token).expect("live result token").id.as_str().to_owned());
        }
    }
    ids
}

#[test]
fn world_marquee_mesh_cursor_matches_legacy_window_crossing_disjoint_and_degenerate_cases() {
    let state = world_marquee_geometry_fixture(1);
    let viewport = render_pick_viewport(&state);
    let view_projection = state.orbit.to_camera().view_proj(800.0, 800.0);
    let mesh = state.meshes.get("mesh").unwrap();
    let projected: Vec<_> = (0..3).map(|index| ui_wgpu::wgpu::project_point(view_projection, world_mesh_vertex(*mesh, index).unwrap(), viewport.w, viewport.h).unwrap()).collect();
    let min_x = projected.iter().map(|point| point[0]).fold(f32::INFINITY, f32::min);
    let max_x = projected.iter().map(|point| point[0]).fold(f32::NEG_INFINITY, f32::max);
    let min_y = projected.iter().map(|point| point[1]).fold(f32::INFINITY, f32::min);
    let max_y = projected.iter().map(|point| point[1]).fold(f32::NEG_INFINITY, f32::max);
    let cases = [[[min_x - 2.0, min_y - 2.0], [max_x + 2.0, max_y + 2.0]], [[(min_x + max_x) * 0.5, min_y - 2.0], [min_x - 2.0, (min_y + max_y) * 0.5]], [[0.0, 0.0], [2.0, 2.0]], [[3.0, 3.0], [3.0, 3.0]]];
    for points in cases {
        let crossing = marquee_is_crossing_from_path(&points, false);
        let (meshes, draws) = legacy_geometry_fixture(&state);
        let legacy = screen_select_instances(&meshes, &draws, view_projection, viewport.w, viewport.h, &points, true, crossing);
        let retained = world_marquee_cursor_ids(&state, &points);
        assert_eq!(retained, legacy, "points={points:?}");
    }
}

#[test]
fn world_marquee_mesh_cursor_preserves_legacy_multi_page_draw_order() {
    let state = world_marquee_geometry_fixture(WORLD_MARQUEE_RESULT_PAGE_CAPACITY + 1);
    let points = [[0.0, 0.0], [400.0, 400.0]];
    let view_projection = state.orbit.to_camera().view_proj(800.0, 800.0);
    let (meshes, draws) = legacy_geometry_fixture(&state);
    let legacy = screen_select_instances(&meshes, &draws, view_projection, 400.0, 400.0, &points, true, false);
    let retained = world_marquee_cursor_ids(&state, &points);
    assert_eq!(retained, legacy);
    assert_eq!(retained.len(), WORLD_MARQUEE_RESULT_PAGE_CAPACITY + 1);
}

#[test]
fn world_marquee_lasso_edge_cursor_matches_legacy_and_rejects_object_aba() {
    let mut state = world_marquee_geometry_fixture(1);
    state.selection_method = "lasso".into();
    let viewport = render_pick_viewport(&state);
    let view_projection = state.orbit.to_camera().view_proj(800.0, 800.0);
    let points = [[0.0, 0.0], [400.0, 0.0], [400.0, 400.0], [0.0, 400.0]];
    let (meshes, draws) = legacy_geometry_fixture(&state);
    let legacy = screen_select_instances(&meshes, &draws, view_projection, viewport.w, viewport.h, &points, false, marquee_is_crossing_from_path(&points, true));
    assert_eq!(world_marquee_cursor_ids(&state, &points), legacy);

    let mut gesture = WorldMarqueeGesture::new(state.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(&state, 8, gesture).unwrap();
    while cursor.current.is_none() {
        assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 8, context)), WorldInteractionStep::Pending);
    }
    let token = cursor.current.unwrap();
    let entry = *state.interaction_objects.resolve(token).unwrap();
    let mut replacement = entry.model;
    replacement.cols[3][0] += 1.0;
    let next = state.interaction_objects.admit(state.interaction_revision, WorldInteractionObjectKind::Instance, entry.id.as_str(), entry.mesh, replacement, entry.values).expect("replacement token");
    assert_ne!(next, token);
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 8, context)), WorldInteractionStep::Stale);
    let mut turns = 0;
    while !cursor.close_step() {
        turns += 1;
        assert!(turns < WORLD_INTERACTION_MARQUEE_POINT_CAPACITY + 4);
    }
    assert_eq!(cursor.gesture.len, 0);
}

fn world_component_marquee_cursor_ids(state: &World3dState, points: &[[f32; 2]]) -> Vec<u32> {
    let mut gesture = WorldMarqueeGesture::new(state.interaction_revision, 7, points[0]);
    for point in points.iter().skip(1) {
        assert!(gesture.push(*point));
    }
    let mut cursor = WorldMarqueePickCursor::new(state, 8, gesture).expect("component marquee cursor");
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| cursor.step(state, 8, context)) == WorldInteractionStep::Complete {
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
    let viewport = render_pick_viewport(&state);
    let view_projection = state.orbit.to_camera().view_proj(800.0, 800.0);
    let points = [[0.0, 0.0], [400.0, 400.0]];
    for granularity in ["vertex", "edge", "face"] {
        state.granularity = granularity.into();
        let (meshes, draws) = legacy_geometry_fixture(&state);
        let mut legacy: Vec<u32> = screen_select_components(&meshes, &draws, view_projection, viewport.w, viewport.h, &points, true, granularity, None, false).into_iter().map(|id| id.parse().expect("numeric component id")).collect();
        legacy.sort_unstable();
        assert!(!legacy.is_empty(), "the oracle must answer a component census, not an empty one: granularity={granularity}");
        assert_eq!(world_component_marquee_cursor_ids(&state, &points), legacy, "granularity={granularity}");
    }
}

#[test]
fn world_component_marquee_publish_merges_before_one_atomic_set_selection() {
    let mut state = world_marquee_geometry_fixture(1);
    state.granularity = "vertex".into();
    state.component_ids = vec!["7".into()];
    let object = state.interaction_objects.instance_order[0].expect("instance token");
    let mut results = WorldMarqueeResultPages::default();
    assert!(results.push(WorldMarqueeResult::Component { object, id: 8 }, 0));
    let mut gesture = WorldMarqueeGesture::new(state.interaction_revision, 7, [0.0, 0.0]);
    assert!(gesture.push([400.0, 400.0]));
    let mut job = WorldComponentMarqueePublishJob::new(8, gesture, results, WorldComponentKind::Vertex, true, false);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    loop {
        turns += 1;
        if with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).expect("component marquee step") == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 128);
    }
    let events = take_actions(&mut input);
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0].args.as_ref().and_then(|args| args.get("ids")), Some(dsl::DslValue::Array(ids)) if ids == &vec![dsl::DslValue::int(7), dsl::DslValue::int(8)]));
    assert_eq!(job.results.page_len, 0);
    assert_eq!(job.gesture.len, 0);
}

#[test]
fn world_component_marquee_capacity_plus_one_fails_closed_and_retires() {
    let token = WorldInteractionObjectToken { slot: 0, generation: 1, revision: 1 };
    let mut results = WorldMarqueeResultPages::default();
    for id in 0..WORLD_COMPONENT_MARQUEE_CAPACITY as u32 {
        assert!(results.push(WorldMarqueeResult::Component { object: token, id }, 0));
    }
    assert!(!results.push(WorldMarqueeResult::Component { object: token, id: WORLD_COMPONENT_MARQUEE_CAPACITY as u32 }, 0));
    let mut turns = 0;
    while !results.close_step() {
        turns += 1;
        assert!(turns <= WORLD_COMPONENT_MARQUEE_CAPACITY + 1);
    }
    assert_eq!(results.page_len, 0);
}

#[test]
fn world_gumball_cursor_caps_selected_tokens_and_closes_one_owner_per_grant() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_revision = 9;
    state.interaction_objects.revision = 9;
    for index in 0..=WORLD_GUMBALL_SELECTED_CAPACITY {
        let mut model = Mat4::identity();
        model.cols[3][0] = index as f32;
        assert!(state.interaction_objects.admit(9, WorldInteractionObjectKind::Instance, &format!("selected-{index}"), None, model, [0.0, 0.0, 1.0, index as f32, 0.0, 0.0, 0.0, 0.0]).is_some());
    }
    let mut cursor = WorldGumballPickCursor::new(&state, 12, 0.0, 0.0);
    for _ in 0..WORLD_INTERACTION_OBJECT_CAPACITY + 1 {
        let step = with_world_step_context(1, |context| cursor.step(&state, 12, context));
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
    state.interaction_revision = 3;
    state.interaction_objects.revision = 3;
    state.bounds = Rect { x: 0.0, y: 0.0, w: 100.0, h: 100.0 };
    let token = state.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, Mat4::identity(), [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]).expect("selected token");
    let mut selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
    selected[0] = Some(token);
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
        selected,
        selected_len: 1,
        selected_bytes: "selected".len() as u16,
        validation: 0,
        pending: None,
    };
    assert_eq!(gesture.begin_update(6, 50.0, 50.0), WorldInteractionStep::Pending);
    assert_eq!(gesture.update_step(&state), WorldInteractionStep::Pending);
    let mut replacement = Mat4::identity();
    replacement.cols[3][0] = 1.0;
    let next = state.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, replacement, [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]).expect("replacement token");
    assert_ne!(next, token);
    gesture.validation = 0;
    assert_eq!(gesture.update_step(&state), WorldInteractionStep::Stale);
    assert!(!gesture.close_step());
    assert!(!gesture.close_step());
    assert!(gesture.close_step());
}

fn world_gumball_commit_host_snapshot() -> (World3dState, WorldGumballGesture) {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_revision = 3;
    state.interaction_objects.revision = 3;
    let token = state.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, Mat4::identity(), [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]).expect("selected token");
    let mut selected = Box::new([None; WORLD_GUMBALL_SELECTED_CAPACITY]);
    selected[0] = Some(token);
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
        selected,
        selected_len: 1,
        selected_bytes: "selected".len() as u16,
        validation: 0,
        pending: None,
    };
    (state, gesture)
}

#[test]
fn world_gumball_commit_builds_one_flat_node_per_grant_then_retires_tokens() {
    let (state, gesture) = world_gumball_commit_host_snapshot();
    let mut job = WorldGumballCommitJob::new(8, gesture);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(0, |context| job.step(&state, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    let mut turns = 0;
    loop {
        turns += 1;
        let step = with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).expect("bounded gumball commit");
        if step == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 32);
    }
    assert!(job.terminal_is_empty());
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "translateSelection");
    // 🪟️ A transform verb is window-owned: it addresses its own window instance rather than
    // whatever has focus — see `WorldGumballCommitJob::step`'s `windowId` stage.
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("windowId")).and_then(|value| value.as_str()), Some(state.surface_id.as_str()));
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("surfaceId")).and_then(|value| value.as_str()), Some(state.surface_id.as_str()));
    assert!(turns > 8);
}

#[test]
fn world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority() {
    let (mut state, gesture) = world_gumball_commit_host_snapshot();
    let mut job = WorldGumballCommitJob::new(8, gesture);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut blockers = Vec::new();
    while let Ok(claim) = input.claim_action(1) {
        blockers.push(claim);
    }
    assert!(matches!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)), Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits)));
    assert!(job.claim.is_none());
    input.release_action_claim(blockers.pop().expect("retry credit")).expect("release retry credit");
    assert_eq!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(job.claim.is_some());
    assert_eq!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(job.draft.is_some());
    let mut replacement = Mat4::identity();
    replacement.cols[3][0] = 1.0;
    state.interaction_objects.admit(3, WorldInteractionObjectKind::Instance, "selected", None, replacement, [0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0]).expect("ABA replacement");
    // 🪟️ Five staged nodes precede the first selected-id resolve: `{`, `surfaceId`, `windowId`,
    // `mode`, `ids[`.
    for _ in 0..5 {
        let _ = with_world_step_context(1, |context| job.step(&state, 8, &mut input, context));
    }
    assert!(matches!(with_world_step_context(1, |context| job.step(&state, 8, &mut input, context)), Err(ui_wgpu::wgpu::BoundedActionFault::Structure)));
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
    state.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    let source = Mat4::identity();
    let preview = retained_gumball_preview_model(&state, 0, 0, source);
    assert_eq!(source.cols[3], [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(preview.cols[3], [2.0, 0.0, 0.0, 1.0]);
    let unmatched = retained_gumball_preview_model(&state, 0, 1, source);
    assert_eq!(unmatched.cols, source.cols);
}

fn world_brush_commit_host_snapshot(target: String) -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_revision = 4;
    state.brush_preview = Some(WorldBrushPreviewRecord {
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
    let mut job = WorldBrushCommitJob::new(&state, 9).unwrap().expect("brush job");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    loop {
        turns += 1;
        let step = with_world_step_context(1, |context| job.step(&state, 9, &mut input, context)).expect("bounded brush step");
        if step == WorldInteractionStep::Complete {
            break;
        }
        assert!(turns < 64);
    }
    assert!(turns > 20);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "addBrushObject");
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("targetVortexFullId")).and_then(dsl::DslValue::as_str).map(str::len), Some(WORLD_BRUSH_COPY_CHUNK_BYTES * 2 + 1));
}

#[test]
fn world_brush_validation_detects_same_length_replacement_and_close_releases_draft_claim() {
    let mut state = world_brush_commit_host_snapshot("first".into());
    let mut stale = WorldBrushCommitJob::new(&state, 9).unwrap().expect("brush job");
    while !stale.validating {
        assert_eq!(with_world_step_context(1, |context| stale.step(&state, 9, &mut ui_wgpu::wgpu::InputState::default(), context)).unwrap(), WorldInteractionStep::Pending);
    }
    state.brush_preview.as_mut().unwrap().target_vortex_full_id = Some("other".into());
    assert!(matches!(with_world_step_context(1, |context| stale.step(&state, 9, &mut ui_wgpu::wgpu::InputState::default(), context)), Err(ui_wgpu::wgpu::BoundedActionFault::Structure)));

    let state = world_brush_commit_host_snapshot("target".into());
    let mut interrupted = WorldBrushCommitJob::new(&state, 10).unwrap().expect("brush job");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    while !interrupted.complete {
        let _ = with_world_step_context(1, |context| interrupted.step(&state, 10, &mut input, context));
    }
    assert_eq!(with_world_step_context(1, |context| interrupted.step(&state, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(interrupted.claim.is_some());
    assert_eq!(with_world_step_context(1, |context| interrupted.step(&state, 10, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert!(interrupted.draft.is_some());
    assert!(!interrupted.close_step(&mut input));
    assert!(!interrupted.close_step(&mut input));
    assert!(interrupted.close_step(&mut input));
    assert!(take_actions(&mut input).is_empty());
}

fn world_intent(generation: u64, delta: f32) -> WorldInteractionIntent {
    WorldInteractionIntent { phase: WorldInteractionPhase::Wheel, generation, x: 0.0, y: 0.0, dx: 0.0, dy: 0.0, delta, button: 0, down: false, shift: false, ctrl: false, alt: false, meta: false }
}

#[test]
fn world_intent_queue_retains_exact_fifo_owner_on_saturation_and_closes_one_per_turn() {
    let mut queue = WorldInteractionIntentQueue::default();
    for index in 0..WORLD_INTERACTION_INTENT_CAPACITY {
        queue.push(world_intent(index as u64, index as f32)).expect("fixed intent slot");
    }
    let rejected = queue.push(world_intent(99, 99.0)).expect_err("capacity plus one retains intent");
    assert_eq!(rejected.generation, 99);
    assert_eq!(queue.front().map(|intent| intent.generation), Some(0));
    assert!(!queue.retire_front(1));
    assert_eq!(queue.front().map(|intent| intent.generation), Some(0));
    assert!(queue.retire_front(0));
    assert_eq!(queue.front().map(|intent| intent.generation), Some(1));
    queue.begin_close();
    let mut turns = 0;
    while !queue.close_step() {
        turns += 1;
        assert!(turns < WORLD_INTERACTION_INTENT_CAPACITY);
    }
    assert_eq!(turns, WORLD_INTERACTION_INTENT_CAPACITY - 1);
    assert!(queue.terminal_is_empty());
    assert_eq!(queue.push(rejected).expect_err("closed authority retains late intent").generation, 99);
}

#[test]
fn world_wheel_plan_revalidates_before_mutation_and_publishes_flat_action() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let original_distance = state.orbit.distance;
    let mut stale = plan_world3d_wheel(&state, 7, 20.0).expect("bounded wheel plan");
    state.interaction_revision = state.interaction_revision.wrapping_add(1);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let stale_step = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut stale, 7, &mut input, context)).unwrap();
    assert_eq!(stale_step, WorldInteractionStep::Stale);
    assert_eq!(state.orbit.distance, original_distance);
    assert!(take_actions(&mut input).is_empty());

    let mut plan = plan_world3d_wheel(&state, 8, 200.0).expect("bounded wheel plan");
    let pending = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 8, &mut input, context)).unwrap();
    assert_eq!(pending, WorldInteractionStep::Pending);
    assert_ne!(state.orbit.distance, original_distance);
    // 🧭️ A navigation step moves the orbit and publishes NOTHING: the wire report is the debounced
    // settle's, one per gesture (see `WorldCameraSync`).
    assert!(take_actions(&mut input).is_empty());
    let complete = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 8, &mut input, context)).unwrap();
    assert_eq!(complete, WorldInteractionStep::Complete);
    assert!(plan.terminal_is_empty());

    let mut settle = plan_world3d_camera_settle(&state, 9).expect("a moved camera owes a settle");
    let mut turns = 0;
    while with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut settle, 9, &mut input, context)).unwrap() != WorldInteractionStep::Complete {
        turns += 1;
        assert!(turns < 8, "a bounded settle terminates");
    }
    let actions = take_actions(&mut input);
    assert_eq!(actions.iter().map(|action| action.action.as_str()).collect::<Vec<_>>(), vec!["noteWorldNavigation", "setCamera"]);
    assert_eq!(actions[0].args.as_ref().and_then(|args| args.get("gestures")).and_then(dsl::DslValue::as_array).map(|ids| ids.iter().filter_map(dsl::DslValue::as_str).collect::<Vec<_>>()), Some(vec!["zoom"]));
}

#[test]
fn world_drag_and_paint_plans_reserve_before_exact_mutation() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let original_target = state.orbit.target;
    let modifiers = PointerModifiers { shift: false, ctrl: false, alt: false, meta: false };
    let mut drag = plan_world3d_drag(&state, 3, 8.0, -4.0, 1, &modifiers).expect("middle drag plan");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let step = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut drag, 3, &mut input, context)).unwrap();
    assert_eq!(step, WorldInteractionStep::Pending);
    assert_ne!(state.orbit.target, original_target);
    assert!(take_actions(&mut input).is_empty(), "a pan step moves the orbit and owes the settle, it never publishes per move");

    state.interaction_mode = "paint".into();
    let mut begin = plan_world3d_paint_stroke(&state, 4, true, 0).expect("paint begin plan");
    let zero = with_world_step_context(0, |context| publish_world3d_plan_step(&mut state, &mut begin, 4, &mut input, context)).unwrap();
    assert_eq!(zero, WorldInteractionStep::Pending);
    assert!(!state.paint_stroke_active);
    let published = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut begin, 4, &mut input, context)).unwrap();
    assert_eq!(published, WorldInteractionStep::Pending);
    assert!(state.paint_stroke_active);
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["paintStrokeBegin"]);

    let mut end = plan_world3d_paint_stroke(&state, 5, false, 0).expect("paint end plan");
    state.interaction_revision = state.interaction_revision.wrapping_add(1);
    let stale = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut end, 5, &mut input, context)).unwrap();
    assert_eq!(stale, WorldInteractionStep::Stale);
    assert!(state.paint_stroke_active);
    assert!(take_actions(&mut input).is_empty());
}

fn world_pick_fixture() -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut data = triangle_mesh_oracle();
    data.uvs = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
    store_mesh(&mut state, "mesh".into(), publish_oracle_mesh(data));
    let mesh_version = *state.mesh_versions.get("mesh").expect("mesh version");
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state
}

#[test]
fn world_ray_pick_cursor_advances_one_triangle_or_boundary_per_grant() {
    let state = world_pick_fixture();
    let mut cursor = WorldRayPickCursor {
        revision: state.interaction_revision,
        generation: 9,
        purpose: WorldRayPickPurpose::Paint,
        origin: Vec3::new(0.0, 0.0, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
        draw: 0,
        instance: 0,
        triangle: 0,
        mesh: None,
        mesh_probe: 0,
        merge: 0,
        best: None,
        pick_target: 0,
        best_target: None,
        complete: false,
        faulted: false,
    };
    let zero = with_world_step_context(0, |context| cursor.step(&state, 9, context));
    assert_eq!(zero, WorldInteractionStep::Pending);
    assert_eq!(cursor.triangle, 0);
    let first = with_world_step_context(1, |context| cursor.step(&state, 9, context));
    assert_eq!(first, WorldInteractionStep::Pending);
    assert_eq!(cursor.triangle, 0);
    assert!(cursor.mesh.is_some());
    let triangle = with_world_step_context(1, |context| cursor.step(&state, 9, context));
    assert_eq!(triangle, WorldInteractionStep::Pending);
    assert_eq!(cursor.triangle, 1);
    assert!(cursor.best.is_some());
    for _ in 0..3 {
        let _ = with_world_step_context(1, |context| cursor.step(&state, 9, context));
    }
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 9, context)), WorldInteractionStep::Complete);
    let mut plan = cursor.finish_plan(&state, 9).expect("live cursor").expect("paint hit");
    let mut state = state;
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 9, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["paintAt"]);
}

#[test]
fn world_ray_pick_cursor_stale_and_interrupted_close_do_not_publish() {
    let mut state = world_pick_fixture();
    let mut cursor = WorldRayPickCursor {
        revision: state.interaction_revision,
        generation: 12,
        purpose: WorldRayPickPurpose::Surface,
        origin: Vec3::new(0.0, 0.0, 1.0),
        direction: Vec3::new(0.0, 0.0, -1.0),
        draw: 0,
        instance: 0,
        triangle: 0,
        mesh: None,
        mesh_probe: 0,
        merge: 0,
        best: None,
        pick_target: 0,
        best_target: None,
        complete: false,
        faulted: false,
    };
    let _ = with_world_step_context(1, |context| cursor.step(&state, 12, context));
    let _ = with_world_step_context(1, |context| cursor.step(&state, 12, context));
    state.interaction_revision = state.interaction_revision.wrapping_add(1);
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 12, context)), WorldInteractionStep::Stale);
    assert_eq!(cursor.finish_plan(&state, 12).err().expect("stale cursor"), WorldInteractionStep::Stale);
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
    state.interaction_objects.revision = state.interaction_revision;
    let original_distance = state.orbit.distance;
    enqueue_world3d_intent(&mut state, world_intent(1, 200.0)).expect("wheel intent");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)) != WorldInteractionAuthorityStep::Complete {
        turns += 1;
        assert!(turns < 8, "a wheel intent settles in a bounded number of steps");
    }
    assert_ne!(state.orbit.distance, original_distance);
    assert!(take_actions(&mut input).is_empty(), "a navigation step publishes nothing");

    let settle = world3d_interaction_front_generation(&state).expect("a drained queue still owes the settle");
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
    enqueue_world3d_intent(&mut state, world_intent(1, 10.0)).expect("active intent");
    enqueue_world3d_intent(&mut state, world_intent(2, 20.0)).expect("queued intent");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let _ = with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context));
    begin_world3d_interaction_close(&mut state);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert!(!with_world_step_context(0, |context| close_world3d_interaction_step(&mut state, &mut input, context)));
    assert!(!world3d_interaction_terminal_is_empty(&state));
    let mut turns = 0;
    while !with_world_step_context(1, |context| close_world3d_interaction_step(&mut state, &mut input, context)) {
        turns += 1;
        assert!(turns < 8);
    }
    assert!(world3d_interaction_terminal_is_empty(&state));
    assert!(take_actions(&mut input).is_empty());
}

#[test]
fn world_saturation_owner_blocks_new_ingress_until_exact_fifo_transfer() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_objects.revision = state.interaction_revision;
    for generation in 1..=WORLD_INTERACTION_INTENT_CAPACITY as u64 + 1 {
        enqueue_world3d_intent(&mut state, world_intent(generation, generation as f32)).expect("queue or retained saturation owner");
    }
    let rejected_generation = WORLD_INTERACTION_INTENT_CAPACITY as u64 + 2;
    assert_eq!(enqueue_world3d_intent(&mut state, world_intent(rejected_generation, rejected_generation as f32)).expect_err("blocked owner seals ingress").generation, rejected_generation);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut turns = 0;
    while state.interaction_authority.as_ref().and_then(|authority| authority.queue.front()).map(|intent| intent.generation) == Some(1) {
        let _ = with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context));
        turns += 1;
        assert!(turns < 16, "the front intent is answered in a bounded number of turns");
    }
    assert!(take_actions(&mut input).is_empty(), "a navigation intent moves the orbit and owes the settle, it publishes nothing");
    // 🗃️ The blocked owner transfers on the FIRST turn that finds a freed slot, whatever else that
    // turn's authority owes first (a moved camera re-dirties the object registry, and its rebuild is
    // itself several turns).
    let mut turns = 0;
    while state.interaction_authority.as_ref().is_some_and(|authority| authority.blocked.is_some()) {
        assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Pending);
        turns += 1;
        assert!(turns < 16, "a freed slot admits the retained saturation owner");
    }
    let authority = state.interaction_authority.as_mut().expect("authority");
    for generation in 2..=WORLD_INTERACTION_INTENT_CAPACITY as u64 + 1 {
        assert_eq!(authority.queue.front().map(|intent| intent.generation), Some(generation));
        assert!(authority.queue.retire_front(generation));
    }
    assert!(authority.queue.front().is_none());
}

#[test]
fn renderer_world_consumers_use_only_fixed_intent_ingress() {
    let glue = include_str!("../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
    let shell = include_str!("../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs");
    for source in [glue, shell] {
        assert!(source.contains("enqueue_world3d_event"));
        assert!(!source.contains("handle_world3d_pointer_button"));
        assert!(!source.contains("handle_world3d_pointer_move"));
        assert!(!source.contains("handle_world3d_paint_actions"));
        assert!(!source.contains("handle_world3d_pointer_drag"));
        assert!(!source.contains("handle_world3d_wheel"));
        assert!(!source.contains(concat!("let mut world_actions", " = Vec")));
    }
    let world = include_str!("../../🦀️.rs");
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_pointer_button")));
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_pointer_move")));
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_paint_actions")));
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_pointer_drag")));
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_wheel")));
}

/// 📐️ The world reference underlay is the only raster producer this lane publishes, and a producer it
/// appends UNBOUND is refused by the prepared render job with `raster producer generation is stale` —
/// the silent refusal that held the wgpu shell at exactly one presented frame per boot
/// (`📓️w7b-presenter-one-frame-per-boot.md` §1).
#[test]
fn an_appended_world_scene_raster_keeps_its_exact_pool_lease() {
    use semio_framework_job::InteractiveJob;
    let mut resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let lease = test_scene_raster_lease(1, 1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, 1);
    let identity = lease.identity();
    resources.ensure_world_plane_texture("plan", lease);
    let mut input = ui_wgpu::wgpu::PreparedRenderInput::try_new(1, 2, ui_wgpu::wgpu::DrawList::default(), None, 0.0).ok().expect("prepared input admitted");
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert!(matches!(input.uploads.get(0), Some(PreparedRenderUpload::SceneRaster { lease, .. }) if lease.identity() == identity));

    let mut input = ui_wgpu::wgpu::PreparedRenderInput::try_new(1, 2, ui_wgpu::wgpu::DrawList::default(), None, 0.0).ok().expect("second prepared input admitted");
    let mut resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    resources.ensure_world_plane_texture("plan", test_scene_raster_lease(1, 1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, 1));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    let mut job = ui_wgpu::wgpu::PreparedRenderJob::try_new(input).ok().expect("prepared job admitted");
    let mut preview = 0;
    let outcome = semio_framework_job::drive_step(
        &mut job,
        "world.prepare",
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(2),
        semio_framework_job::InteractiveStage::BackgroundStep,
        semio_framework_job::StepBudget::new(4, u64::MAX),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut preview,
        &mut None,
    );
    assert!(matches!(outcome, semio_framework_job::StepOutcome::Yield), "a bound producer keeps the preparation alive");
    assert_eq!(job.fault(), None, "the prepared job never refuses this lane's own producer");
    job.begin_close();
    while !matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {}
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
    state.reference_pixels.insert("/plan.jpg".into(), test_world_scene_raster(4, 2, 7)).expect("reference pixels admitted");
    for render in 0..4 {
        let offered = reference_underlay_upload(&state, "/plan.jpg").expect("every render offers the payload");
        assert_eq!(offered.identity().descriptor().byte_len(), Some(32), "🖼️ render {render}");
    }
    assert_eq!(state.reference_pixels.get("/plan.jpg").and_then(|raster| raster.identity.descriptor().byte_len()), Some(32), "🖼️ and the surface keeps the immutable identity while the pool owns pixels");
    assert_eq!(reference_image_aspect(&state, "/plan.jpg"), 2.0, "🖼️ the dimensions answer the aspect every frame reads");
    assert!(reference_underlay_upload(&state, "/missing.jpg").is_none());

    let pool = world_scene_raster_pool();
    let over_budget = (ui_wgpu::wgpu::SCENE_RASTER_ITEM_BYTES / 4 + 1) as u32;
    let descriptor = SceneRasterDescriptor { width: over_budget, height: 1, source_digest: [91, 92], source_revision: 91, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
    assert!(matches!(pool.begin(descriptor, 91, SceneRasterWriteMode::Moved), SceneRasterBegin::Refused(_)), "🧯️ the full-quality pool refuses item +1 before it owns an allocation");
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
        state.references.push(WorldReferenceRecord { url: Some(url.clone()), origin: Some([0.0; 3]), width_world: Some(2.0), hidden: Some(false), ..Default::default() });
        let descriptor = SceneRasterDescriptor { width: 2, height: 1, source_digest: [revision, revision.rotate_left(17)], source_revision: revision, profile: SceneRasterProfile::ReferenceImageMapNoColorSpace, mesh: None };
        let lease = publish(&pool, descriptor, revision, revision as u8);
        let identity = lease.identity();
        state.reference_pixels.insert(url.clone(), WorldSceneRaster { identity, pending: Mutex::new(Some(lease)) }).expect("reference identity");
        let upload = world_raster_upload_in_pool(&state, &url, &url, false, &pool).expect("pending CPU upload");
        let mut producer = World3dBuildContext::new(WorldCursorWakeAuthority::new());
        producer.ensure_world_plane_texture(&url, upload);
        let mut input = ui_wgpu::wgpu::PreparedRenderInput::try_new(revision, revision, ui_wgpu::wgpu::DrawList::default(), None, 0.0).ok().expect("prepared reference input");
        assert!(matches!(producer.append_step(&mut input), Ok(false)), "lease enters actual prepared upload");
        assert!(matches!(producer.append_step(&mut input), Ok(false)), "producer retires its exact key");
        assert!(matches!(producer.append_step(&mut input), Ok(true)), "producer reaches terminal");
        let PreparedRenderUpload::SceneRaster { key, lease } = input.uploads.get(0).expect("actual producer publishes the scene lease") else { panic!("scene raster upload") };
        assert_eq!(key, &url);
        gpu.push(lease.release_witness().expect("CPU witness").commit_gpu(&url).expect("GPU commit"));
        assert!(world_reference_raster_ready_in_pool(&state, &url, &pool));
        identities.push(identity);
        drop(input);
        let mut retired = false;
        for _ in 0..64 {
            if ui_wgpu::wgpu::PreparedRenderInput::close_abandoned_step() {
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
    assert!(!world_reference_raster_ready_in_pool(&state, "/reference-1.png", &pool), "GPU loss plus CPU eviction makes the exact source requestable again");
    let request = reserve_world3d_asset_request(&mut state, WorldAssetRequestKind::ReferenceImage, "/reference-1.png").expect("lost source is requestable");
    let owner = take_next_world3d_asset(&mut state).expect("lost source reaches actual fetch ownership");
    assert_eq!(owner.token(), request);
    assert_eq!(owner.url(), "/reference-1.png");
    assert_eq!(owner.kind(), WorldAssetRequestKind::ReferenceImage);
    return_world3d_asset(&mut state, owner).unwrap_or_else(|_| panic!("source owner returns for bounded close"));

    let descriptor = identities[0].descriptor();
    let replacement = publish(&pool, descriptor, 100, 1);
    assert!(apply_decoded_reference_raster(&mut state, "/reference-1.png", replacement));
    assert!(world_reference_raster_ready_in_pool(&state, "/reference-1.png", &pool));
    assert_eq!(reference_image_aspect(&state, "/reference-1.png"), 2.0, "recovery preserves the decoded natural dimensions");
    state.asset_io.begin_close();
    while !state.asset_io.close_step() {}
    assert!(state.asset_io.terminal_is_empty());
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
    let mut input = ui_wgpu::wgpu::PreparedRenderInput::try_new(1, 2, ui_wgpu::wgpu::DrawList::default(), None, 0.0).ok().expect("prepared input admitted");
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
}

#[test]
fn world_orbit_view_gizmo_placement_matches_react_bottom_right_insets() {
    // 🧭️ React's own `resolveSceneGizmoViewportPlacement` oracle — the block margin clears the folded
    // projection pane so the cube sits ABOVE the pane's bottom chip row, never on it.
    assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 1280.0, h: 720.0 }), (32.0, 58.0));
    assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 120.0, h: 160.0 }), (32.0, 53.0));
    assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 40.0, h: 48.0 }), (22.0, 22.0));
}

#[test]
fn world_orbit_view_gizmo_preserves_label_free_hit_targets() {
    let tips = gizmo::orbit_view_gizmo_tips(&Camera3d::default(), Rect { x: 0.0, y: 0.0, w: 1280.0, h: 720.0 });
    assert_eq!(tips.len(), 15);
    assert_eq!(tips.iter().filter(|tip| tip.prominent).count(), 8);
    assert!(tips.iter().all(|tip| tip.pick_radius >= 7.0));
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
            chunking_json: None,
            environment_json: None,
            frame_json: None,
            fit_json: None,
            terrain_json: None,
            points_json: None,
            status_json: None,
            tool_run_trace: None,
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
    assert_eq!(state.references.len(), 1);
    assert_eq!(state.references[0].url.as_deref(), Some("/infinite-assets/plan.jpg"));
    assert_eq!(state.references[0].width_world, Some(50.0));
    assert!(!state.references[0].hidden.unwrap_or(true));
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
    assert_eq!(state.engagement_preview.len(), 4);
    assert_eq!(state.engagement_preview[0].kind.as_deref(), Some("point"));
    assert_eq!(state.engagement_preview[0].position, Some([1.0, 2.0, 0.0]));
    assert_eq!(state.engagement_preview[1].from, Some([0.0, 0.0, 0.0]));
    assert_eq!(state.engagement_preview[2].corner_b, Some([2.0, 3.0, 0.0]));
    assert_eq!(state.engagement_preview[2].height, Some(4.0));
    assert_eq!(state.engagement_preview[3].axis, Some([0.0, 0.0, 2.0]));
}

#[test]
fn an_absent_or_broken_engagement_preview_lane_leaves_the_state_empty() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(None), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.engagement_preview.is_empty());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some("not json")), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert!(state.engagement_preview.is_empty());
}

/// 🖍️ 3 axis crosses (2 vertices each) + 1 segment + 12 box edges + 1 handle = 6 + 2 + 24 + 2 = 34
/// line vertices, and the box wireframe extrudes UP from `cornerA.z` by `height`.
#[test]
fn the_engagement_preview_lane_paints_world_lines() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_engagement_preview(Some(ENGAGEMENT_PREVIEW_JSON)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let mut lines = Vec::new();
    append_engagement_preview_lines(&state, &mut lines, [1.0, 0.0, 0.0, 1.0]);
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
    append_engagement_preview_lines(&state, &mut lines, [0.0, 0.0, 0.0, 1.0]);
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
    assert_eq!(state.engagement_preview.len(), 5);
    let mut lines = Vec::new();
    append_engagement_preview_lines(&state, &mut lines, [0.0, 0.0, 0.0, 1.0]);
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
    state.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.pick_bounds = state.bounds;
    state.active_utility = "select".into();
    state.bound_domain_id = Some("cad".into());
    state.bound_domain_granularity_id = Some("object".into());
    state.pick_targets = world_pick_targets_from_json(pick_targets_json);
    state
}

fn pick_target_ray(state: &World3dState) -> (Vec3, Vec3) {
    let camera = state.orbit.to_camera();
    camera.ray_from_screen(200.0, 200.0, 400.0, 400.0)
}

/// 🧲️ LAW (packet W14g item 1): the `pickTargets` lane is READ, bounded, hit-tested with React's own
/// `targetRayScore` ordering, and dispatched as `interactionSelect`/`interactionHover` at the
/// target's own granularity.
///
/// ⚖️ Four things at once, because they are one mechanism: a lane that parses but never hit-tests is
/// exactly the gap this packet closed (`📓️w2f-cad-spatial-editor-wgpu.md` §5.1 — the overlay could
/// be SHOWN and not picked).
#[test]
fn the_pick_target_lane_hit_tests_finest_first_skips_unselectable_rows_and_dispatches_react_payloads() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_pick_targets(Some(PICK_TARGETS_JSON)), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.pick_targets.len(), 3, "the lane is parsed off the scene, not only off a hand-built state");

    let mut state = pick_target_state(PICK_TARGETS_JSON);
    let (origin, direction) = pick_target_ray(&state);
    let scores: Vec<Option<f32>> = state.pick_targets.iter().map(|target| world_pick_target_ray_score(target, origin, direction)).collect();
    assert!(scores.iter().all(Option::is_some), "the ray through the origin crosses every padded box");
    let vertex = scores[1].expect("vertex score");
    let object = scores[0].expect("object score");
    assert!(object < vertex, "the solid's front face is nearer than its center vertex ({object} < {vertex})");
    state.pick_targets[0].points = vec![[-0.04; 3], [0.04; 3]];
    let tied_object = world_pick_target_ray_score(&state.pick_targets[0], origin, direction).expect("coincident padded object score");
    assert!(vertex < tied_object, "React's spatialPickPriority makes the finest kind win at coincident padded bounds ({vertex} < {tied_object})");

    // 🖱️ A real click through the retained authority — the pick target beats the instance under it.
    let modifiers = PointerModifiers::default();
    enqueue_world3d_event(&mut state, WorldInteractionIntent::pointer_button(200.0, 200.0, true, 0, &modifiers)).expect("press admitted");
    enqueue_world3d_event(&mut state, WorldInteractionIntent::pointer_button(200.0, 200.0, false, 0, &modifiers)).expect("release admitted");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut published: Vec<ActionDescriptor> = Vec::new();
    let mut turns = 0;
    while let Some(generation) = world3d_interaction_front_generation(&state) {
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
    assert_eq!(args.get("domainId").and_then(dsl::DslValue::as_str), Some("cad"), "the bound domain, not the world fallback");
    // 🎯️ `targets` is JSON TEXT on the wire (see `push_interaction_targets`), never a nested array.
    let targets = args.get("targets").and_then(dsl::DslValue::as_str).expect("targets is JSON text");
    assert!(targets.contains(r#""granularity":"vertex""#), "the WINNING target's own kind is the granularity: {targets}");
    assert!(targets.contains(r#""id":"v-1""#), "the kernel entity id, not the instance id: {targets}");
    assert!(!targets.contains("f-locked"), "an unselectable (hidden/locked) row is rendered and never hit: {targets}");
}

/// 🧲️ LAW: the lane is BOUNDED on the wire — the retained set and every row's polyline are capped at
/// parse time, so no pointer move can carry an unbounded `Vec` behind it.
#[test]
fn the_pick_target_lane_is_capped_at_parse_time_however_large_it_arrives() {
    let rows: Vec<String> = (0..WORLD_PICK_TARGET_CAPACITY + 64)
        .map(|index| {
            let points: Vec<String> = (0..WORLD_PICK_TARGET_POINT_CAPACITY + 32).map(|sample| format!("[{sample}.0,0.0,0.0]")).collect();
            format!(r#"{{"kind":"edge","id":"e-{index}","point":[0,0,0],"points":[{}]}}"#, points.join(","))
        })
        .collect();
    let parsed = world_pick_targets_from_json(&format!("[{}]", rows.join(",")));
    assert_eq!(parsed.len(), WORLD_PICK_TARGET_CAPACITY);
    assert!(parsed.iter().all(|target| target.points.len() == WORLD_PICK_TARGET_POINT_CAPACITY));
    assert!(world_pick_targets_from_json("not json").is_empty(), "a broken lane leaves the retained set empty, never faults");
}

/// 🪪️ LAW: hover keys match across the kernel/pick ALIASES (`solid:` ↔ `object:`, `shell:` → `face:`,
/// `wire:` → `edge:`, `anchor:` → `vertex:`) — the framework twin of the CAD engine's
/// `spatial_hover_key_aliases`, and what stops a guest echo from re-publishing the same hover.
#[test]
fn pick_hover_keys_match_across_the_kernel_and_pick_aliases() {
    assert!(world_pick_keys_match(Some("object:s1"), Some("solid:s1")));
    assert!(world_pick_keys_match(Some("solid:s1"), Some("object:s1")));
    assert!(world_pick_keys_match(Some("shell:h1"), Some("face:h1")));
    assert!(world_pick_keys_match(Some("wire:w1"), Some("edge:w1")));
    assert!(world_pick_keys_match(Some("anchor:a1"), Some("vertex:a1")));
    assert!(!world_pick_keys_match(Some("object:s1"), Some("object:s2")));
    assert!(!world_pick_keys_match(None, Some("object:s1")));
    assert!(!world_pick_keys_match(Some(""), Some("")));
}
//#endregion 🧲️PickTargetLane

//#region 🌫️InstanceStyleWire
/// 🌫️ LAW (packet W14g items 3 + 4): the instance lane carries the per-instance OPACITY a locked
/// object is dimmed by, and the `highlighted`/`disabled`/`celebrating` flags that reach the three
/// `MESH_STYLE_PAINT` rows nothing published before.
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
    assert!(state.disabled_instance_ids.contains("obj-1"));
    assert!(state.highlighted_instance_ids.contains("obj-2"));
    assert!(state.celebrating_instance_ids.contains("obj-3"));
    let alpha = state.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == "obj-1").map(|instance| instance.color[3]).expect("the locked instance reaches a draw");
    assert!((alpha - 0.35).abs() < 1e-3, "WORLD_LOCKED_OPACITY_SCALE multiplies the authored alpha, got {alpha}");
    let opaque = state.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == "obj-2").map(|instance| instance.color[3]).expect("obj-2 reaches a draw");
    assert!((opaque - 1.0).abs() < 1e-3, "an instance with no opacity keeps its authored alpha, got {opaque}");
    // 🎨️ Each flag resolves to its OWN row of React's table, not to the two rows that existed here.
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
        let retained = state.draws.iter().flat_map(|draw| draw.instances.iter()).find(|instance| instance.id == id).expect("bridged instance retained");
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
#[test]
fn the_gumball_offers_the_authored_config_then_the_transform_mode_fallback_intersected_with_the_plane() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(
        &mut state,
        &scene_with_selection(r#"{"transformMode":"transform","gumballConfig":{"moveAxes":true,"movePlanes":true,"rotate":true,"scaleAxes":false,"scalePlanes":false,"scaleUniform":false}}"#),
        Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 },
    );
    let config = world3d_gumball_config(&state);
    assert!(config.admits(GumballHandle::MoveX) && config.admits(GumballHandle::MoveXY) && config.admits(GumballHandle::RotateZ));
    assert!(!config.admits(GumballHandle::ScaleX), "an authored config that disables scaling hides AND unpicks the scale handles");

    // 🎛️ No authored config → React's `gumballConfigForTransformMode`, whose `rotate` arm offers the
    // rings ONLY. This host used to admit the three move axes unconditionally in every mode.
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(r#"{"transformMode":"rotateSelection"}"#), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let config = world3d_gumball_config(&state);
    assert!(config.admits(GumballHandle::RotateX));
    assert!(!config.admits(GumballHandle::MoveX), "React's rotate mode offers no translation handle");
    assert!(!config.admits(GumballHandle::ScaleZ));

    // 📐️ A Top pane's `xy` drafting plane keeps the in-plane move/scale pair and the NORMAL rotation.
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene_with_selection(r#"{"transformMode":"transform","gumballConfig":{"plane":"xy"}}"#), Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    let config = world3d_gumball_config(&state);
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
#[test]
fn vertex_markers_hold_their_pixel_size_at_every_camera_distance() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    let viewport = state.bounds;
    let near = {
        let mut orbit = state.orbit.clone();
        orbit.distance = 4.0;
        orbit.to_camera()
    };
    let far = {
        let mut orbit = state.orbit.clone();
        orbit.distance = 400.0;
        orbit.to_camera()
    };
    let centre = Vec3::new(0.0, 0.0, 0.0);
    let near_half = vertex_marker_half_extent(&near, viewport, centre, VERTEX_BASE_SCALE);
    let far_half = vertex_marker_half_extent(&far, viewport, centre, VERTEX_BASE_SCALE);
    assert!(far_half > near_half * 10.0, "a marker 100x further away spans ~100x more world units to keep its pixels ({near_half} → {far_half})");
    let near_pixels = near_half / world_units_per_pixel(&near, viewport, near.position.sub(centre).length());
    assert!((near_pixels - WORLD_VERTEX_DOT_PX * 0.5).abs() < 1e-3, "the plain dot is React's 6 px, got {}", near_pixels * 2.0);
    let mark = vertex_marker_half_extent(&near, viewport, centre, VERTEX_HOVER_SCALE);
    assert!(mark > near_half, "a hovered/selected marker is React's larger 11 px mark");
    // 🩸️ The pre-W14g cross was `VERTEX_BASE_SCALE * 0.15` = 0.0075 world units, at ANY distance.
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
    assert!(state.selection_targets.vertex);
    assert!(!state.selection_targets.edge);
    assert_eq!(state.active_object_id.as_deref(), Some("obj-1"));
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
    let seed = state.orbit.clone();
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.orbit.distance, seed.distance, "a fit lane without an extent or geometry has nothing to frame");
    sync_world3d_state(&mut state, &scene_with_fit(Some(BOOT_FRAME_FIT_JSON)), bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    assert_eq!([state.orbit.target.x, state.orbit.target.y, state.orbit.target.z], [1.0, 1.0, 1.0], "the framed camera looks at the delivered box centre");
    let camera = state.orbit.to_camera();
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
    let framed = state.orbit.clone();
    state.orbit.orbit(40.0, 12.0);
    state.orbit.zoom(-120.0);
    state.camera_user_moved = true;
    let moved = state.orbit.clone();
    state.scene_bridge_digest = None;
    drive_scene_bridge(&mut state, &scene, bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.orbit.yaw, moved.yaw, "a second delivery of the same document re-framed a camera the user had moved");
    assert_eq!(state.orbit.distance, moved.distance);
    assert_ne!(moved.yaw, framed.yaw, "the gesture must actually have moved the camera for this law to say anything");
    let next_document = BOOT_FRAME_FIT_JSON.replace("\"revision\":7", "\"revision\":8");
    sync_world3d_state(&mut state, &scene_with_fit(Some(&next_document)), bounds);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Applied);
    assert_eq!(state.orbit.yaw, moved.yaw, "framing a new document keeps the look direction the user chose");
    assert_eq!([state.orbit.target.x, state.orbit.target.y, state.orbit.target.z], [1.0, 1.0, 1.0]);
    assert!((state.orbit.distance - framed.distance).abs() < 1e-3, "the new document is framed again, at the fitting distance");
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
    assert_eq!(state.granularity, "vertex");
    assert_eq!(state.component_ids, vec!["1".to_string(), "2".to_string()]);
    assert_eq!(state.hovered_component_id.as_deref(), Some("3"));
    assert_eq!(state.hovered_component_object_id.as_deref(), Some("obj-1"));
    assert_eq!(state.hovered_component_mode.as_deref(), Some("vertex"));
    assert!(state.show_edges);
}

#[test]
fn append_component_vertex_spheres_render_base_vertices() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "vertex".into();
    state.selection_targets.vertex = true;
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let instances = append_component_vertex_spheres(&mut state);
    assert_eq!(instances.len(), 0);

    let mut lines = Vec::new();
    append_component_overlays(&state, &state.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert_eq!(lines.len(), 28); // 4 from edges + 24 from 4 vertex crosses
}

/// 🔘️ The viewport the component-overlay laws measure vertex markers against — their size is now in
/// SCREEN PIXELS (`vertex_marker_half_extent`), so every one of them needs a camera and a rect.
const COMPONENT_OVERLAY_VIEWPORT: Rect = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };

#[test]
fn append_component_overlays_highlights_only_hovered_edge() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "edge".into();
    state.selection_targets.edge = true;
    state.hovered_component_id = Some("5".into());
    state.hovered_component_object_id = Some("obj-1".into());
    state.hovered_component_mode = Some("edge".into());
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let mut lines = Vec::new();
    append_component_overlays(&state, &state.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(lines.len() >= 2);
    assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
}

#[test]
fn append_component_overlays_highlights_selected_edge() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "edge".into();
    state.component_ids = vec!["6".into()];
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let mut lines = Vec::new();
    append_component_overlays(&state, &state.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(lines.len() >= 2);
    assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
}

#[test]
fn component_mode_does_not_apply_mesh_instance_hover() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "face".into();
    state.hovered_component_object_id = Some("obj-1".into());
    state.local_hover_id = None;
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    apply_runtime_draw_flags(&mut state);
    assert!(!state.draws[0].instances[0].hovered);
}

#[test]
fn runtime_draw_flags_clear_stale_instance_selected_when_selection_is_empty() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selected_ids.clear();
    state.local_hover_id = None;
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: true, hovered: true, material: Default::default() }], shadow_role: Default::default() });
    apply_runtime_draw_flags(&mut state);
    assert!(!state.draws[0].instances[0].selected, "empty selection must clear a stale instancesJson selected bit");
    assert!(!state.draws[0].instances[0].hovered, "empty hover must clear a stale instancesJson hovered bit");
}

/// 🎯️ The component-pick merge speaks the ONE `MergeMode` vocabulary (`🕹️interaction/🧬️schema/🔣️.json`,
/// fixture `🎯️merge-modes.json`) — ticket 26/09/09/PROCEDURAL-3D-END-TO-END deleted this file's
/// private `add`/`remove`/`toggle` spelling, which is unrepresentable now the mode is decoded.
#[test]
fn merge_u32_ids_speaks_the_one_schema_merge_vocabulary() {
    assert_eq!(merge_u32_ids(&["1".into()], &["2".into()], MergeMode::Additive), vec![1, 2]);
    assert_eq!(merge_u32_ids(&["1".into(), "2".into()], &["2".into(), "3".into()], MergeMode::Invertive), vec![1, 3]);
    assert_eq!(merge_u32_ids(&["1".into(), "2".into()], &["2".into()], MergeMode::Subtractive), vec![1]);
    assert_eq!(merge_u32_ids(&["1".into()], &["2".into()], MergeMode::Replace), vec![2]);
    for word in ["add", "remove", "toggle"] {
        assert!(MergeMode::from_wire_label(word).is_none(), "the deleted word '{word}' must decode to nothing");
    }
}

#[test]
fn pick_select_emits_numeric_world_pick_id() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "vertex".into();
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(inner.w, inner.h), Vec3::ZERO, inner.w, inner.h).expect("vertex projects");
    let action = pick_select_action(&state, screen[0], screen[1], inner, false, false).expect("pick action");
    assert_eq!(action.action, "worldPick");
    let args = action.args.expect("args");
    assert_eq!(args["id"].as_f64(), Some(1.0));
}

#[test]
fn marquee_preview_respects_pick_bounds_offset() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "vertex".into();
    state.active_object_id = Some("obj-1".into());
    state.meshes.insert("mesh-1".into(), topology_mesh());
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 100.0, y: 50.0, w: 400.0, h: 400.0 };
    state.pick_bounds = inner;
    state.marquee_points = vec![[110.0, 60.0], [490.0, 450.0]];
    update_marquee_preview(&mut state, inner);
    assert!(!state.marquee_preview_ids.is_empty(), "preview ids: {:?}", state.marquee_preview_ids);
}

#[test]
fn marquee_crossing_includes_partial_overlap_window_does_not() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "mesh".into();
    state.meshes.insert("mesh-1".into(), topology_mesh());
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
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
    state.marquee_points = vec![[inner.x + min_x, inner.y + min_y], [inner.x + center_x, inner.y + center_y]];
    update_marquee_preview(&mut state, inner);
    assert!(state.marquee_preview_ids.is_empty(), "window marquee should not select partially enclosed mesh");
    state.marquee_points = vec![[inner.x + center_x, inner.y + min_y], [inner.x + min_x, inner.y + center_y]];
    update_marquee_preview(&mut state, inner);
    assert!(!state.marquee_preview_ids.is_empty(), "crossing marquee should select partially enclosed mesh");
}

#[test]
fn marquee_component_mode_emits_set_selection_with_numeric_ids() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "vertex".into();
    state.component_ids = vec!["1".into()];
    state.marquee_points = vec![[10.0, 10.0], [390.0, 390.0]];
    state.meshes.insert("mesh-1".into(), topology_mesh());
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let action = marquee_select_action(&mut state, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, true, false).expect("marquee action");
    assert_eq!(action.action, "setSelection");
    let args = action.args.expect("args");
    assert_eq!(args["mode"], json!("vertex"));
    assert!(args["ids"].as_array().is_some());
}

#[test]
fn click_release_routes_to_pick_select_instead_of_empty_marquee() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.active_utility = "select".into();
    state.granularity = "mesh".into();
    state.marquee_active = true;
    state.marquee_points = vec![[120.0, 140.0]];
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let action = handle_world3d_pointer_button(&mut state, 120.0, 140.0, false, 0, &PointerModifiers::default()).expect("click should pick");
    assert_eq!(action.action, "worldPick");
    assert!(!state.marquee_active);
}

#[test]
fn marquee_face_preview_and_overlay_use_logical_face_ids() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "face".into();
    state.active_object_id = Some("obj-1".into());
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = bounds;
    state.pick_bounds = bounds;
    state.marquee_points = vec![[390.0, 10.0], [10.0, 390.0]];
    update_marquee_preview(&mut state, bounds);
    assert!(state.marquee_preview_ids.iter().any(|id| id == "10" || id == "11"), "preview ids: {:?}", state.marquee_preview_ids);
    let mut lines = Vec::new();
    append_component_overlays(&state, &state.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(!lines.is_empty(), "face marquee preview should draw triangle edge lines");
}

#[test]
fn hovered_component_preserved_when_selection_json_omits_hover_field() {
    let selection = r#"{"granularity":"face","componentIds":[10]}"#;
    let scene = scene_with_selection(selection);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "face".into();
    state.hovered_component_id = Some("11".into());
    state.hovered_component_object_id = Some("obj-1".into());
    state.hovered_component_mode = Some("face".into());
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.hovered_component_id.as_deref(), Some("11"));
}

#[test]
fn apply_world_action_preview_updates_component_hover_and_selection() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    apply_world_action_preview(
        &mut state,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "setHover".into(),
            args: action_args(json!({
                "objectId": "obj-1",
                "mode": "vertex",
                "id": 2,
            })),
        },
    );
    assert_eq!(state.hovered_component_id.as_deref(), Some("2"));
    assert_eq!(state.hovered_component_mode.as_deref(), Some("vertex"));
    assert!(state.local_hover_id.is_none());

    apply_world_action_preview(
        &mut state,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "worldPick".into(),
            args: action_args(json!({
                "granularity": "vertex",
                "id": 4,
                "merge": "replace",
            })),
        },
    );
    assert_eq!(state.component_ids, vec!["4".to_string()]);
    assert_eq!(state.granularity, "vertex");
}

#[test]
fn preview_survives_sync_when_scene_json_unchanged() {
    let selection = r#"{"granularity":"vertex","componentIds":[1],"hoveredComponent":{"objectId":"obj-1","mode":"vertex","id":2}}"#;
    let scene = scene_with_selection(selection);
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    apply_world_action_preview(
        &mut state,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "worldPick".into(),
            args: action_args(json!({
                "granularity": "vertex",
                "id": 5,
                "merge": "replace",
            })),
        },
    );
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.component_ids, vec!["5".to_string()]);
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
    let actual = typed.orbit.to_camera();
    assert_eq!(actual.position, expected.position);
    assert_eq!(actual.target, expected.target);
    assert_eq!(actual.up, expected.up);
    assert_eq!(actual.fov_y, expected.fov_y);
    assert_eq!(typed.snapshot_lease, Some(lease));

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
    state.mesh_source_urls.insert("mesh".into(), "/mesh/🧊️refused.glb".into());
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances: Vec::new(), shadow_role: Default::default() }).expect("one scene draw");
    state.references.push(WorldReferenceRecord { url: Some("/plan.jpg".into()), origin: Some([0.0; 3]), width_world: Some(1.0), hidden: Some(false), ..Default::default() });
    let offered = |state: &World3dState| -> (Vec<String>, Vec<String>) {
        let meshes = state.draws.iter().filter(|draw| !state.meshes.contains_key(&draw.mesh_key)).filter_map(|draw| state.mesh_source_urls.get(&draw.mesh_key).cloned()).filter(|url| !state.asset_url_misses.contains(url)).collect();
        let references = state.references.iter().filter_map(|reference| reference.url.clone()).filter(|url| !world_reference_raster_ready(state, url)).filter(|url| !state.asset_url_misses.contains(url)).collect();
        (meshes, references)
    };
    assert_eq!(offered(&state), (vec!["/mesh/🧊️refused.glb".to_string()], vec!["/plan.jpg".to_string()]), "both loops offer an un-refused url");
    assert!(!world3d_asset_url_missed(&state, "/mesh/🧊️refused.glb"));
    state.pending_glb_urls.insert("/mesh/🧊️refused.glb".into());
    state.pending_image_urls.insert("/plan.jpg".into());

    mark_world3d_asset_miss(&mut state, "/mesh/🧊️refused.glb");
    mark_world3d_asset_miss(&mut state, "/plan.jpg");
    assert!(world3d_asset_url_missed(&state, "/mesh/🧊️refused.glb"));
    assert!(world3d_asset_url_missed(&state, "/plan.jpg"));
    assert!(!world3d_asset_url_missed(&state, "/mesh/🧊️other.glb"));
    assert!(!state.pending_glb_urls.contains("/mesh/🧊️refused.glb"));
    assert!(!state.pending_image_urls.contains("/plan.jpg"));
    assert_eq!(offered(&state), (Vec::new(), Vec::new()), "neither loop offers a refused url again");
}

#[test]
fn dynamic_world_owners_retire_one_nested_owner_per_grant_to_terminal_empty() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mesh = publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0; 3 * 128], vec![0.0; 3 * 128], vec![0; 3 * 64]));
    state.meshes.insert("mesh".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances: (0..32).map(|index| Instance3d { id: format!("instance-{index}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }).collect(), shadow_role: Default::default() });
    state.reference_pixels.insert("reference".into(), test_world_scene_raster(64, 64, 0));
    let paint = test_scene_raster_lease(64, 64, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 3 }), 0);
    state.mesh_paint_textures.insert("paint".into(), WorldSceneRaster { identity: paint.identity(), pending: Mutex::new(Some(paint)) });
    assert!(begin_world3d_dynamic_retirement(&mut state));
    assert!(!begin_world3d_dynamic_retirement(&mut state));
    assert!(!with_world_step_context(0, |context| step_world3d_dynamic_retirement(&mut state, context)));
    let mut turns = 0;
    while !with_world_step_context(1, |context| step_world3d_dynamic_retirement(&mut state, context)) {
        turns += 1;
        assert!(turns < 16);
    }
    assert!(turns >= 8, "each registry transition and opaque owner transfer consumes a distinct grant");
    assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
}

#[test]
fn dynamic_registry_returns_capacity_and_identifier_owners_exactly() {
    let mut registry = WorldDynamicRegistry::<u32, 2>::default();
    assert!(registry.insert("a".into(), 1).is_ok());
    assert!(registry.insert("b".into(), 2).is_ok());
    let rejected = registry.insert("c".into(), 3).expect_err("capacity owner");
    assert_eq!(rejected.fault, WorldDynamicFault::RegistryCapacity);
    assert_eq!((rejected.id.as_str(), rejected.value), ("c", 3));
    let rejected = registry.insert("x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY + 1), 4).expect_err("identifier owner");
    assert_eq!(rejected.fault, WorldDynamicFault::IdCapacity);
    assert_eq!(rejected.value, 4);
    assert_eq!(registry.len(), 2);
}

#[test]
fn dynamic_registry_replacement_invalidates_aba_token_and_retains_previous_owner() {
    let mut registry = WorldDynamicRegistry::<u32, 1>::default();
    let (first, _) = registry.insert("mesh".into(), 1).unwrap();
    let (second, previous) = registry.insert("mesh".into(), 2).unwrap();
    let previous = previous.expect("replacement returns previous owner");
    assert_eq!((previous.id.as_str(), previous.value), ("mesh", 1));
    assert_eq!(registry.remove_token(first).unwrap_err(), WorldDynamicFault::StaleToken);
    assert_eq!(registry.remove_token(second).unwrap().value, 2);
    assert!(registry.is_empty());
}

#[test]
fn opaque_quarantine_saturation_returns_the_exact_rejected_owner() {
    let mut quarantine = WorldOpaqueQuarantine::<1>::default();
    let first = WorldOpaqueOwner::Draw(WorldDynamicEntry { id: "first".into(), epoch: 1, value: SceneDraw3d { mesh_key: "first-mesh".into(), mesh_version: 1, instances: Vec::new(), shadow_role: Default::default() } });
    assert!(quarantine.admit(first).is_ok());
    let second = WorldOpaqueOwner::Draw(WorldDynamicEntry { id: "second".into(), epoch: 2, value: SceneDraw3d { mesh_key: "second-mesh".into(), mesh_version: 2, instances: Vec::new(), shadow_role: Default::default() } });
    let rejected = quarantine.admit(second).expect_err("quarantine returns saturated owner");
    let WorldOpaqueOwner::Draw(rejected) = rejected;
    assert_eq!((rejected.id.as_str(), rejected.epoch, rejected.value.mesh_key.as_str(), rejected.value.mesh_version), ("second", 2, "second-mesh", 2));
    assert_eq!(quarantine.saturated, 1);
    assert!(quarantine.take_one().is_some());
    assert!(quarantine.take_one().is_none());
}

#[test]
fn dynamic_close_is_interruptible_and_rejects_late_insertions() {
    let mut registry = WorldDynamicRegistry::<u32, 2>::default();
    registry.insert("a".into(), 1).unwrap();
    registry.insert("b".into(), 2).unwrap();
    registry.begin_close();
    assert_eq!(registry.take_one().expect("one owner per close grant").value, 1);
    assert_eq!(registry.len(), 1);
    let rejected = registry.insert("late".into(), 3).expect_err("closing registry rejects late owner");
    assert_eq!((rejected.fault, rejected.id.as_str(), rejected.value), (WorldDynamicFault::Closing, "late", 3));
    assert_eq!(registry.take_one().expect("resumed close owner").value, 2);
    assert!(registry.is_empty());
}

#[test]
fn draw_registry_returns_the_exact_instance_capacity_owner() {
    let mut draws = WorldDrawRegistry::default();
    let instances = (0..=WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY).map(|index| Instance3d { id: format!("instance-{index}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }).collect();
    let rejected = draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances, shadow_role: Default::default() }).expect_err("draw instance capacity owner");
    assert_eq!(rejected.fault, WorldDynamicFault::InstanceCapacity);
    assert_eq!(rejected.value.instances.len(), WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY + 1);
    assert!(draws.is_empty());
}

fn draw_fixture_instance(id: &str) -> Instance3d {
    Instance3d { id: id.into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }
}

fn admit_draw_fixture_instance(state: &mut World3dState, draw: u16, id: &str) -> Result<(), WorldDynamicFault> {
    world3d_draw_rebuild_admit_instance(state, draw, id, Mat4::identity(), [1.0; 4], false, false)
}

fn draw_fixture_bytes(draws: &[(&str, &[&str])]) -> u32 {
    draws.iter().map(|(mesh, instances)| mesh.len() + size_of::<SceneDraw3d>() + instances.iter().map(|id| id.len() + size_of::<Instance3d>()).sum::<usize>()).sum::<usize>() as u32
}

#[test]
fn retained_draw_rebuild_preserves_mixed_group_and_instance_fifo_then_swaps_atomically() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let bytes = draw_fixture_bytes(&[("mesh-a", &["a-0", "a-1"]), ("mesh-b", &["b-0"])]);
    begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 2, instance_count: 3, byte_count: bytes }).unwrap();
    world3d_draw_rebuild_admit_draw(&mut state, "mesh-a", 1, 2).unwrap();
    world3d_draw_rebuild_admit_draw(&mut state, "mesh-b", 2, 1).unwrap();
    admit_draw_fixture_instance(&mut state, 0, "a-0").unwrap();
    admit_draw_fixture_instance(&mut state, 0, "a-1").unwrap();
    admit_draw_fixture_instance(&mut state, 1, "b-0").unwrap();
    world3d_draw_rebuild_seal(&mut state).unwrap();
    assert!(state.draws.is_empty(), "sealed drafts never publish partially");
    let mut turns = 0;
    while with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state, context)) == WorldDrawRebuildStep::Pending {
        turns += 1;
        assert!(turns < 16);
    }
    assert_eq!(state.draws.iter().map(|draw| draw.mesh_key.as_str()).collect::<Vec<_>>(), vec!["mesh-a", "mesh-b"]);
    assert_eq!(state.draws[0].instances.iter().map(|instance| instance.id.as_str()).collect::<Vec<_>>(), vec!["a-0", "a-1"]);
    assert_eq!(state.draws[1].instances[0].id, "b-0");
    assert_eq!(state.draw_generation, 1);
}

#[test]
fn retained_draw_rebuild_rejects_byte_and_identifier_plus_one_before_owner_publication() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 0, byte_count: 0 }).unwrap();
    let rejected = world3d_draw_rebuild_admit_draw(&mut state, "mesh", 1, 0).expect_err("byte +1 leaves borrowed mesh owner at producer");
    assert_eq!(rejected, WorldDynamicFault::ByteCapacity);
    while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state, context)) {}
    let long = "x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY + 1);
    begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 0, byte_count: WORLD_DYNAMIC_DRAW_BYTE_CAPACITY as u32 }).unwrap();
    let rejected = world3d_draw_rebuild_admit_draw(&mut state, &long, 1, 0).expect_err("ID +1 leaves borrowed mesh owner at producer");
    assert_eq!(rejected, WorldDynamicFault::IdCapacity);
    while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state, context)) {}
}

#[test]
fn retained_draw_rebuild_rejects_aggregate_instance_plus_one_before_draft_ownership() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let result = begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: (WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY + 1) as u32, byte_count: WORLD_DYNAMIC_DRAW_BYTE_CAPACITY as u32 });
    assert_eq!(result, Err(WorldDynamicFault::InstanceCapacity));
    assert!(world3d_draw_rebuild_terminal_is_empty(&state));
}

#[test]
fn retained_draw_rebuild_stale_and_interrupted_close_never_publish() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let bytes = draw_fixture_bytes(&[("mesh", &["one", "two"])]);
    begin_world3d_draw_rebuild(&mut state, WorldDrawRebuildDescriptor { generation: 1, revision: 0, draw_count: 1, instance_count: 2, byte_count: bytes }).unwrap();
    world3d_draw_rebuild_admit_draw(&mut state, "mesh", 1, 2).unwrap();
    admit_draw_fixture_instance(&mut state, 0, "one").unwrap();
    admit_draw_fixture_instance(&mut state, 0, "two").unwrap();
    world3d_draw_rebuild_seal(&mut state).unwrap();
    state.interaction_revision = 1;
    assert_eq!(with_world_step_context(1, |context| step_world3d_draw_rebuild(&mut state, context)), WorldDrawRebuildStep::Stale);
    assert!(!with_world_step_context(0, |context| close_world3d_draw_rebuild_step(&mut state, context)));
    let mut turns = 0;
    while !with_world_step_context(1, |context| close_world3d_draw_rebuild_step(&mut state, context)) {
        turns += 1;
        assert!(turns < 8);
    }
    assert!(state.draws.is_empty());
    assert!(world3d_draw_rebuild_terminal_is_empty(&state));
}

#[test]
fn production_dynamic_owners_have_no_hash_map_vec_or_direct_pixel_mutation_bypass() {
    let source = include_str!("../../🦀️.rs");
    let production = source.split("#[cfg(test)]\nmod tests").next().expect("production source");
    for forbidden in [
        concat!("pub meshes: HashMap<String, Mesh", "3d>"),
        "pub draws: Vec<SceneDraw3d>",
        "reference_pixels: HashMap<String, (u32, u32, Vec<u8>)>",
        "mesh_paint_textures: HashMap<String, (u32, u32, Vec<u8>)>",
        "state.reference_pixels.insert(",
        "state.mesh_paint_textures.insert(",
        "state.draws =",
        "fn rebuild_instance_draws(state:",
    ] {
        assert!(!production.contains(forbidden), "production dynamic owner bypass returned: {forbidden}");
    }
    assert!(production.contains("state.meshes.plan_insert(&id)"), "mesh publication remains centralized at the observed-slot replacement authority");
    assert!(production.contains("step_world3d_dynamic_retirement"), "World3d exposes the one-grant retained close pump");
    assert!(production.contains("struct WorldPlaceholderMeshCursor"));
    assert!(production.contains("mesh3d_allocate_step(self.token()?)"));
    assert!(production.contains("struct WorldFaceOverlayMeshCursor"));
    assert!(production.contains("self.owner = WorldPlaceholderOwner::Writing(mesh3d_begin("));
    let face_route = production.split("fn append_component_face_translucent_overlays").nth(1).and_then(|source| source.split("fn selection_centroid").next()).expect("face overlay production route");
    for forbidden in [concat!("Mesh", "3d::from_buffers"), "FaceOverlayBucket", "HashSet<String>", "Vec<f32>"] {
        assert!(!face_route.contains(forbidden), "face overlay recursive/contiguous constructor returned: {forbidden}");
    }
    let mounted_root = include_str!("../../../🦀️.rs");
    assert!(mounted_root.contains("pub use crate::world::*;"));
    assert!(!mounted_root.contains(concat!("Mesh", "3d")));
}

#[test]
fn pick_viewport_uses_render_bounds_not_pick_clip_offset() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "vertex".into();
    state.active_object_id = Some("obj-1".into());
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let bounds = Rect { x: 0.0, y: 50.0, w: 400.0, h: 400.0 };
    let clip = Rect { x: 0.0, y: 100.0, w: 400.0, h: 400.0 };
    state.bounds = bounds;
    state.pick_bounds = clip;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(bounds.w, bounds.h), Vec3::ZERO, bounds.w, bounds.h).expect("vertex projects");
    let global_x = bounds.x + screen[0];
    let global_y = bounds.y + screen[1];
    let picked = pick_component_at(&state, global_x, global_y, bounds).expect("vertex pick respects render viewport");
    assert_eq!(picked.1, "1");
}

#[test]
fn append_component_face_overlay_lines_include_hovered_face() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "face".into();
    state.hovered_component_mode = Some("face".into());
    state.hovered_component_id = Some("10".into());
    state.hovered_component_object_id = Some("obj-1".into());
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let mut lines = Vec::new();
    append_component_overlays(&state, &state.orbit.to_camera(), COMPONENT_OVERLAY_VIEWPORT, &mut lines);
    assert!(lines.len() >= 6, "hovered face should emit triangle edge lines, got {}", lines.len());
}

#[test]
fn pick_component_at_face_mode_uses_ray_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "face".into();
    state.active_object_id = Some("obj-1".into());
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let mesh_ref = state.meshes.get("mesh-1").expect("mesh");
    let tri = world_mesh_triangle(*mesh_ref, 0).expect("triangle");
    let centroid = mesh_vertex(*mesh_ref, tri[0]).expect("first vertex").add(mesh_vertex(*mesh_ref, tri[1]).expect("second vertex")).add(mesh_vertex(*mesh_ref, tri[2]).expect("third vertex")).scale(1.0 / 3.0);
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(inner.w, inner.h), centroid, inner.w, inner.h).expect("face centroid projects");
    let picked = pick_component_at(&state, screen[0], screen[1], inner).expect("face pick");
    assert_eq!(picked.0, "face");
    assert_eq!(picked.2, "obj-1");
}

#[test]
fn pick_component_at_edge_mode_uses_ray_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "edge".into();
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let edge = state.meshes.get("mesh-1").expect("mesh").edge(0).expect("edge");
    let a = Vec3::new(edge[0][0], edge[0][1], edge[0][2]);
    let b = Vec3::new(edge[1][0], edge[1][1], edge[1][2]);
    let mid = a.add(b).scale(0.5);
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(inner.w, inner.h), mid, inner.w, inner.h).expect("edge midpoint projects");
    let picked = pick_component_at(&state, screen[0], screen[1], inner).expect("edge pick");
    assert_eq!(picked.0, "edge");
    assert_eq!(picked.2, "obj-1");
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

#[test]
fn mesh_pool_release_clears_at_zero_refcount() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.mesh_pool.acquire("mesh-1".into());
    assert!(state.mesh_pool.release(&"mesh-1".into()));
    assert!(!state.mesh_pool.contains(&"mesh-1".to_string()));
    state.mesh_pool.acquire("mesh-1".into());
    state.mesh_pool.acquire("mesh-1".into());
    assert!(!state.mesh_pool.release(&"mesh-1".into()));
    assert!(state.mesh_pool.contains(&"mesh-1".to_string()));
    assert!(state.mesh_pool.release(&"mesh-1".into()));
    assert!(!state.mesh_pool.contains(&"mesh-1".to_string()));
}

#[test]
fn resolve_physical_mesh_id_picks_closest_lod_url() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.mesh_lod_catalog.insert("tower".into(), vec![WorldMeshLodEntry { lod: 1.0, url: "https://example.com/tower-high.glb".into() }, WorldMeshLodEntry { lod: 100.0, url: "https://example.com/tower-low.glb".into() }]);
    let detailed = resolve_physical_mesh_id(&state, "tower", 2.0);
    let coarse = resolve_physical_mesh_id(&state, "tower", 200.0);
    assert_eq!(detailed, "mesh:tower-high");
    assert_eq!(coarse, "mesh:tower-low");
}

//#region GlbAssetTests
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
    let plan = image::DynamicImage::new_rgba8(2275, 2560);
    let bounded = bounded_reference_image(plan);
    assert_eq!((bounded.width(), bounded.height()), (2275, 2560));
    assert_eq!(bounded.width() as usize * bounded.height() as usize * 4, 23_296_000);
    assert!(23_296_000 <= WORLD_REFERENCE_TEXTURE_BYTES);
    let small = image::DynamicImage::new_rgba8(64, 48);
    let bounded = bounded_reference_image(small);
    assert_eq!((bounded.width(), bounded.height()), (64, 48));
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

fn publish_retained_draw_fixture(state: &mut World3dState, vector: &serde_json::Value) {
    let mesh = vector["mesh"].as_str().unwrap();
    let instances = vector["instances"].as_array().unwrap();
    let ids: Vec<_> = instances.iter().map(|instance| instance["id"].as_str().unwrap()).collect();
    begin_world3d_draw_rebuild(state, WorldDrawRebuildDescriptor { generation: state.draw_generation + 1, revision: state.interaction_revision, draw_count: 1, instance_count: instances.len() as u32, byte_count: draw_fixture_bytes(&[(mesh, &ids)]) })
        .unwrap();
    world3d_draw_rebuild_admit_draw(state, mesh, vector["version"].as_u64().unwrap(), instances.len() as u16).unwrap();
    for instance in instances {
        let color = std::array::from_fn(|index| instance["color"][index].as_f64().unwrap() as f32);
        world3d_draw_rebuild_admit_instance(state, 0, instance["id"].as_str().unwrap(), Mat4::identity(), color, false, false).unwrap();
    }
    world3d_draw_rebuild_seal(state).unwrap();
    for _ in 0..16 {
        match with_world_step_context(1, |context| step_world3d_draw_rebuild(state, context)) {
            WorldDrawRebuildStep::Complete => return,
            WorldDrawRebuildStep::Pending => {}
            other => panic!("draw fixture failed: {other:?}"),
        }
    }
    panic!("draw fixture exceeded its bounded turn count");
}

#[test]
fn retained_draw_rebuild_keeps_url_backed_asset_authority() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️draws.json")).unwrap();
    let vector = &fixture["draws"][1];
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let token = reserve_world3d_asset_request(&mut state, WorldAssetRequestKind::Glb, vector["url"].as_str().unwrap()).unwrap();
    publish_retained_draw_fixture(&mut state, vector);
    assert!(!state.meshes.contains_key(vector["mesh"].as_str().unwrap()));
    assert!(state.placeholder_build.is_none(), "retained draws never replace an admitted asset request with a primitive");
    let mut owner = take_next_world3d_asset(&mut state).expect("exact retained asset request");
    assert_eq!(owner.token(), token);
    assert_eq!(owner.url(), vector["url"].as_str().unwrap());
    owner.begin_close();
    return_world3d_asset(&mut state, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state) {}
    assert!(state.asset_io.terminal_is_empty());
    assert!(begin_world3d_dynamic_retirement(&mut state));
    while !with_world_step_context(1, |context| step_world3d_dynamic_retirement(&mut state, context)) {}
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
    assert!(begin_world3d_dynamic_retirement(state));
    for _ in 0..4_096 {
        if with_world_step_context(1, |context| step_world3d_dynamic_retirement(state, context)) {
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

    assert_eq!(state.snapshot_fault, None);
    let drawn: Vec<(String, Vec<String>)> = state.draws.iter().map(|draw| (draw.mesh_key.clone(), draw.instances.iter().map(|instance| instance.id.clone()).collect())).collect();
    assert_eq!(drawn, vec![("box".to_string(), vec!["obj-box".to_string()]), (URL_MESH_ID.to_string(), vec!["obj-left".to_string()])], "both the kind mesh and the url mesh keep their draw and their instances");

    assert!(state.meshes.contains_key("box"), "a built-in kind resolves through the placeholder ladder both renderers own");
    assert!(!state.meshes.contains_key(URL_MESH_ID), "a url mesh is owed by the asset pipeline and must NOT be stood in for: a placeholder suppresses its own fetch forever");
    assert_eq!(state.mesh_source_urls.get(URL_MESH_ID).map(String::as_str), Some(URL_MESH_URL), "the bridge binds the mesh id to the url the fetch loop reads");
    retire_bridged_surface(&mut state);
}

/// 🎨️ LAW: geometry provenance selects React's standard-material pair inside one delivered scene:
/// inline geometry remains `0/1`, while the adjacent GLB consumes the environment override.
#[test]
fn one_scene_resolves_distinct_inline_and_glb_standard_materials() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️scene-shading/🔣️.json")).unwrap();
    let mut scene = url_mesh_wire_scene();
    scene.world_3d.as_mut().unwrap().environment_json = Some(serde_json::json!({ "material": fixture["mixedMaterialScope"]["environment"] }).to_string());
    let mut state = World3dState::new("surface-material".into(), "controller-material".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    let environment = environment_scene_material(&state.environment);
    assert_eq!(world3d_standard_material_for_mesh(&state, "box", environment), [0.0, 1.0]);
    assert_eq!(world3d_standard_material_for_mesh(&state, URL_MESH_ID, environment), [0.35, 0.58]);
    assert_eq!(world3d_standard_material_for_mesh(&state, URL_MESH_ID, SceneMaterial3d::default()), [0.0, 1.0]);
    retire_bridged_surface(&mut state);
}

/// 🥽️📡️ The url lane end to end inside the surface: the draw offers its url exactly once, the
/// decoded mesh lands under the wire's own id, and the instance draws from then on.
#[test]
fn a_url_declared_mesh_reserves_one_glb_fetch_and_draws_once_its_mesh_lands() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    offer_missing_mesh_fetches(&mut state);
    offer_missing_mesh_fetches(&mut state);
    assert!(state.pending_glb_urls.contains(URL_MESH_URL), "the live predicate that keeps a per-frame offer idempotent");
    let mut owner = take_next_world3d_asset(&mut state).expect("exactly one admitted GLB request");
    assert_eq!(owner.url(), URL_MESH_URL);
    assert_eq!(owner.kind(), WorldAssetRequestKind::Glb);
    assert!(take_next_world3d_asset(&mut state).is_none(), "a re-offered url never doubles its request");

    let mesh = publish_oracle_mesh_at_revision(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]), state.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state, URL_MESH_URL, mesh).expect("the decoded GLB publishes under the wire's mesh id");
    let resident = *state.meshes.get(URL_MESH_ID).expect("the fetched GLB is resident under the id the wire named");
    let schema = resident.schema().expect("resident mesh schema");
    assert_eq!((schema.vertices, schema.indices), (3, 3), "the resident mesh carries real positions and indices");
    assert!(!state.pending_glb_urls.contains(URL_MESH_URL), "a landed mesh stops being pending");

    offer_missing_mesh_fetches(&mut state);
    assert!(take_next_world3d_asset(&mut state).is_none(), "a resident mesh is never re-fetched");
    let draw = state.draws.iter().find(|draw| draw.mesh_key == URL_MESH_ID).expect("the url mesh keeps its draw");
    assert_eq!(draw.instances.len(), 1, "the instance draws now that its mesh is resident");

    owner.begin_close();
    return_world3d_asset(&mut state, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state) {}
    retire_bridged_surface(&mut state);
}

/// 🗑️ A refused GLB is a miss, not an endless retry.
#[test]
fn a_refused_mesh_url_is_never_offered_again_by_its_surface() {
    let scene = url_mesh_wire_scene();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });

    offer_missing_mesh_fetches(&mut state);
    let mut owner = take_next_world3d_asset(&mut state).expect("admitted GLB request");
    owner.begin_close();
    return_world3d_asset(&mut state, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state) {}

    mark_world3d_asset_miss(&mut state, URL_MESH_URL);
    assert!(!state.pending_glb_urls.contains(URL_MESH_URL), "a refusal clears the pending mark it replaces");
    offer_missing_mesh_fetches(&mut state);
    assert!(take_next_world3d_asset(&mut state).is_none(), "a refused url is never offered again");
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

    assert_eq!(ghost_mesh_id(&state, Some(URL_MESH_URL)), "box", "a pending url ghosts as the shared primitive");
    assert_eq!(ghost_mesh_id(&state, None), "box");
    assert_eq!(ghost_mesh_id(&state, Some("/mesh/🧊️unrelated.glb")), "mesh:🧊️unrelated", "a url the scene never named keeps its own lazy ghost key");

    let mesh = publish_oracle_mesh_at_revision(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]), state.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state, URL_MESH_URL, mesh).expect("the decoded GLB publishes");
    assert_eq!(ghost_mesh_id(&state, Some(URL_MESH_URL)), URL_MESH_ID, "once the mesh lands the ghost draws the real thing");
    retire_bridged_surface(&mut state);
}

//#endregion GlbAssetTests

//#region 🥽️BrushMeshAnnounceTests
/// 🧫️ React's measured `registerBrushMesh` rows and the arg shapes it sends, from the 2026-09-18
/// side-by-side run (`🗑️generated/w11a-parity-run-14/steps.json`).
const BRUSH_MESH_REGISTRATION: &str = include_str!("../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🥽️brush-mesh-registration.json");

/// 🔒️ The announcement registry is process-wide by design (React's is a module singleton), so the
/// laws below take it one at a time whatever `--test-threads` says.
static BRUSH_MESH_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn brush_mesh_fixture() -> serde_json::Value {
    serde_json::from_str(BRUSH_MESH_REGISTRATION).expect("brush mesh registration fixture parses")
}

/// 🧫️ A mesh whose payload spans SEVERAL pages, so the run's positions-then-indices split falls
/// inside a page the way a real GLB's does: 400 vertices (1 200 position values) plus 900 indices is
/// 2 100 values, three pages at [`WORLD_BRUSH_MESH_PAGE_VALUES`].
fn brush_mesh_oracle() -> LegacyMeshOracleData {
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
    let mesh = publish_oracle_mesh_at_revision(brush_mesh_oracle(), state.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state, URL_MESH_URL, mesh).expect("the decoded GLB publishes under the wire's mesh id");
    state
}

fn brush_mesh_text(action: &ActionDescriptor, key: &str) -> Option<String> {
    action.args.as_ref().and_then(|args| args.get(key)).and_then(dsl::DslValue::as_str).map(str::to_string)
}

fn brush_mesh_number(action: &ActionDescriptor, key: &str) -> Option<u64> {
    action.args.as_ref().and_then(|args| args.get(key)).and_then(dsl::DslValue::as_f64).map(|value| value as u64)
}

fn brush_mesh_carries(action: &ActionDescriptor, key: &str) -> bool {
    action.args.as_ref().and_then(|args| args.get(key)).is_some()
}

fn drain_brush_mesh_run(state: &mut World3dState) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    for _ in 0..(WORLD_BRUSH_MESH_MAX_PAGES as usize + 2) {
        let Some(action) = step_world3d_brush_mesh_announce(state) else { break };
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
    assert!(step_world3d_brush_mesh_announce(&mut first).is_none(), "an announced identity is never re-announced at the same revision");
    assert!(!world3d_brush_mesh_announce_pending(&first), "and the surface owes the guest nothing more");

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
    assert!(!world3d_brush_mesh_announce_pending(&state));

    sync_world3d_brush_mesh_feedback(&mut state, Some(r#"{"meshResidency":7,"meshReuploadUrls":[]}"#));
    assert!(!world3d_brush_mesh_announce_pending(&state), "a climbing residency is the guest confirming what it holds, not news");

    sync_world3d_brush_mesh_feedback(&mut state, Some(&format!(r#"{{"meshResidency":7,"meshReuploadUrls":["{URL_MESH_URL}"]}}"#)));
    assert!(world3d_brush_mesh_announce_pending(&state), "a standing request is an identity the guest cannot serve");
    let reupload = drain_brush_mesh_run(&mut state);
    assert!(reupload.iter().any(|action| brush_mesh_carries(action, "positionsB64")), "a claimed request takes the PAGE path, never the id-only one");

    sync_world3d_brush_mesh_feedback(&mut state, Some(r#"{"meshResidency":0,"meshReuploadUrls":[]}"#));
    assert!(world3d_brush_mesh_announce_pending(&state), "a residency below the high-water mark is proof of a restarted guest");
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
            assert!(step_world3d_brush_mesh_announce(&mut state).is_none(), "and no duplicate follows in the same frame");
            retire_bridged_surface(&mut state);
        }
    }
    reset_world3d_brush_mesh_registry();
}
//#endregion 🥽️BrushMeshAnnounceTests

/// 📐️ LAW: the grid draws React's ONE band at `lodGridStepWorld`'s exact spacing, is sized by the
/// camera's own fade radius rather than a fixed square, stays inside the far plane, and fades to
/// zero alpha at its rim (`📓️w7b-presenter-one-frame-per-boot.md` §5,
/// `📓️w8b-orthographic-camera-and-3d-parity.md` §3).
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

    let step = ui_wgpu::wgpu::lod_grid_step_world(2.0, 10.0).expect("React's helper answers a step") as f32;
    let mut offsets: Vec<f32> = lines.iter().map(|vertex| vertex.position[1]).collect();
    offsets.sort_by(|left, right| left.partial_cmp(right).expect("finite grid coordinates"));
    offsets.dedup_by(|left, right| (*left - *right).abs() < 1e-4);
    for pair in offsets.windows(2) {
        let gap = pair[1] - pair[0];
        assert!((gap / step - (gap / step).round()).abs() < 1e-3, "every row lands on React's band multiple, never on a clamped division");
    }
    assert!(lines.iter().any(|vertex| vertex.color[3] == 0.0), "the rim fades out instead of ending on a hard edge");

    // 🔀️ The same law under a plan view: a parallel camera's fade radius comes from its zoomed
    // pixel extent, so a `Top` pane gets a grid of its own instead of the perspective one's.
    let parallel = ui_wgpu::wgpu::Camera3d { projection: ui_wgpu::wgpu::CameraProjection3d::Orthographic, zoom: 12.0, position: Vec3::new(0.0, 0.0, 9.45), up: Vec3::new(0.0, 1.0, 0.0), ..camera.clone() };
    let mut parallel_lines = Vec::new();
    append_lod_grid_lines(&mut parallel_lines, 2.0, 10.0, Vec3::ZERO, &parallel, viewport, [0.5, 0.5, 0.5, 1.0]);
    assert!(!parallel_lines.is_empty(), "a plan pane draws a grid too");
    let parallel_half = parallel_lines.iter().flat_map(|vertex| [vertex.position[0].abs(), vertex.position[1].abs()]).fold(0.0_f32, f32::max);
    assert!(parallel_half <= camera.far * 0.25 + 1e-3);
    assert!(parallel_lines.len() <= 8 * (WORLD_GRID_MAX_DIVISIONS as usize + 1), "and never submits more than the division ceiling allows");
}

//#region EnvironmentTests
#[test]
fn environment_clear_color_uses_opaque_background() {
    let environment = WorldEnvironmentRecord { background: Some("#112233".into()), ..Default::default() };
    let theme_clear = Rgba::new(0.0, 0.0, 0.0, 1.0);
    let clear = environment_clear_color(&environment, theme_clear);
    // 🎨️ Linear, not raw sRGB: the world pass renders into an sRGB view, so a wire colour is
    // linearized on the way in exactly as React's `new Color(hex)` linearizes it.
    assert!((clear.r - srgb_to_linear(0x11 as f32 / 255.0)).abs() < 1e-4);
    assert!((clear.g - srgb_to_linear(0x22 as f32 / 255.0)).abs() < 1e-4);
    assert!((clear.b - srgb_to_linear(0x33 as f32 / 255.0)).abs() < 1e-4);
}

#[test]
fn environment_clear_color_falls_back_when_transparent_or_absent() {
    let theme_clear = Rgba::new(0.1, 0.2, 0.3, 1.0);
    let transparent = WorldEnvironmentRecord { background: Some("transparent".into()), ..Default::default() };
    let absent = WorldEnvironmentRecord::default();
    assert_eq!(environment_clear_color(&transparent, theme_clear).r, theme_clear.r);
    assert_eq!(environment_clear_color(&absent, theme_clear).r, theme_clear.r);
}

#[test]
fn environment_light_dir_uses_sun_direction_only_when_enabled() {
    let disabled = WorldEnvironmentRecord { sun: Some(WorldEnvironmentSunRecord { enabled: Some(false), azimuth: Some(90.0), elevation: Some(0.0), ..Default::default() }), ..Default::default() };
    assert_eq!(environment_light_dir(&disabled), [0.4, 0.6, 0.8]);

    let enabled = WorldEnvironmentRecord { sun: Some(WorldEnvironmentSunRecord { enabled: Some(true), azimuth: Some(90.0), elevation: Some(0.0), ..Default::default() }), ..Default::default() };
    let dir = environment_light_dir(&enabled);
    // azimuth=90, elevation=0 -> pure +Y direction (cos(0)*cos(90)=~0, cos(0)*sin(90)=1, sin(0)=0).
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
    assert!(!environment_scene_shadow(&environment, environment_light_dir(&environment), World3dShadowProfile::Unshadowed).enabled);
}

#[test]
fn neutral_shadow_fixture_maps_to_world_draw_roles_and_exact_profiles() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌑️scene-shadow-parity/🔣️.json")).unwrap();
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
fn world_environment_reaches_the_actual_scene_pass() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌞️scene-lighting/🔣️.json")).unwrap();
    let shadow_fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌑️scene-shadows/🔣️.json")).unwrap();
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🌑️scene-shadow-parity/🔣️.json")).unwrap();
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
    state.meshes.insert("shadow-mesh".into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("mesh lease admitted");
    state.mesh_versions.insert("shadow-mesh".into(), 7).expect("mesh version admitted");
    state
        .draws
        .push(SceneDraw3d {
            mesh_key: "shadow-mesh".into(),
            mesh_version: 7,
            instances: vec![
                Instance3d { id: "offscreen-caster".into(), model: Instance3d::model_from_trs([-7.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [0.25, 0.25, 0.25]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() },
                Instance3d { id: "visible-receiver".into(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [0.25, 0.25, 0.25]), color: [1.0; 4], selected: false, hovered: false, material: Default::default() },
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
    assert!(state.mesh_pool.contains(&"shadow-mesh".to_string()), "the offscreen-only caster participates in retained mesh ownership");
    retire_bridged_surface(&mut state);
}

#[test]
fn retained_draw_rebuild_preserves_prepared_material_colors_from_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️draws.json")).unwrap();
    for vector in fixture["draws"].as_array().unwrap() {
        let mut state = World3dState::new("surface-1".into(), "controller-1".into());
        publish_retained_draw_fixture(&mut state, vector);
        let expected = vector["instances"].as_array().unwrap();
        assert_eq!(state.draws[0].mesh_key, vector["mesh"].as_str().unwrap());
        assert_eq!(state.draws[0].mesh_version, vector["version"].as_u64().unwrap());
        assert_eq!(state.draws[0].instances.len(), expected.len());
        for (actual, expected) in state.draws[0].instances.iter().zip(expected) {
            let color: [f32; 4] = serde_json::from_value(expected["color"].clone()).unwrap();
            assert_eq!(actual.id, expected["id"].as_str().unwrap());
            assert_eq!(actual.color, color);
        }
        assert!(begin_world3d_dynamic_retirement(&mut state));
        while !with_world_step_context(1, |context| step_world3d_dynamic_retirement(&mut state, context)) {}
        assert!(world3d_dynamic_retirement_terminal_is_empty(&state));
    }
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

#[test]
fn build_terrain_tile_mesh_oracle_colours_every_vertex_along_the_continuous_ramp() {
    // Two triangles, 6 verts (no sharing, to keep each triangle's average elevation exact):
    // triangle 0 is flat at elevation ratio 0.0, triangle 1 is flat at elevation ratio 1.0.
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

#[test]
fn sync_terrain_state_queues_fetch_for_uncached_tile_and_builds_after_upload() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 9.7382, project_origin_lat: 52.3759, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    apply_terrain_style_if_changed_state(&mut state);
    let camera = Camera3d { position: Vec3::new(0.0, 0.0, 300.0), target: Vec3::ZERO, up: Vec3::new(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0, projection: ui_wgpu::wgpu::CameraProjection3d::Perspective, zoom: 1.0 };
    let (tile_draws, evicted) = sync_terrain_state(&mut state, &camera);
    assert!(tile_draws.is_empty(), "no elevation data uploaded yet, nothing to draw");
    assert!(evicted.is_empty(), "nothing was cached yet, nothing to evict");
    assert!(!state.pending_terrain_tile_urls.is_empty(), "an uncached visible tile should be queued for byte-fetch");

    // 📡️ The queue is not a to-do list nobody reads: an uncached visible tile is ADMITTED into the
    // bounded world-asset pipeline, so the renderer host's generic fetch pump drains it exactly as it
    // drains GLB meshes and reference images. Before this wiring the queue had no consumer outside a
    // unit test and terrain never loaded in production at all.
    let owner = take_next_world3d_asset(&mut state).expect("a visible uncached DEM tile is admitted as a fetch request");
    let (z, x, y) = match owner.kind() {
        WorldAssetRequestKind::Terrain { z, x, y } => (z, x, y),
        other => panic!("a DEM tile is admitted as a Terrain request, got {other:?}"),
    };
    assert_eq!(owner.url(), terrain_tile_url("/dem/{z}/{x}/{y}.png", z, x, y));
    return_world3d_asset(&mut state, owner).expect("the admitted owner returns to its claim");
    let value = (100.0_f64 + 32768.0).round() as i64;
    let r = ((value >> 8) & 0xff) as u8;
    let g = (value - ((r as i64) << 8)).clamp(0, 255) as u8;
    let mut image = image::RgbaImage::new(256, 256);
    for pixel in image.pixels_mut() {
        *pixel = image::Rgba([r, g, 0, 255]);
    }
    let mut bytes = Vec::new();
    image::DynamicImage::ImageRgba8(image).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
    assert!(apply_world3d_terrain_tile_bytes(&mut state, z, x, y, &bytes), "fetched DEM bytes reach the terrain session");
    assert!(!state.pending_terrain_tile_urls.contains_key(&terrain_tile_url("/dem/{z}/{x}/{y}.png", z, x, y)), "an applied tile leaves the pending set");

    // 🕸️ The tile mesh is built by a BOUNDED cursor — ONE scalar write per turn — so the draw appears
    // many turns after the bytes land, not on the very next frame. The turns are driven directly
    // (`step_world_terrain_mesh`) rather than through whole `sync_terrain_state` frames, which would
    // re-serialize every visible tile's mesh JSON per step.
    sync_terrain_state(&mut state, &camera);
    assert!(state.terrain_build.is_some(), "an uploaded tile opens its mesh-build cursor");
    let mut turns = 0;
    while state.terrain_build.is_some() {
        step_world_terrain_mesh(&mut state);
        turns += 1;
        assert!(turns < 1_000_000, "an uploaded tile converges to its vertex-coloured mesh");
    }
    let (tile_draws_after_upload, _) = sync_terrain_state(&mut state, &camera);
    assert_eq!(tile_draws_after_upload.len(), 1, "one draw per tile — the ramp rides the mesh's own vertex colours, not ten flat bands");
}

#[test]
fn apply_terrain_style_if_changed_state_purges_stale_meshes_on_origin_change() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 0.0, project_origin_lat: 0.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    assert!(apply_terrain_style_if_changed_state(&mut state).is_empty(), "first application has nothing to purge");
    let mesh_key = terrain_tile_mesh_key(&state.surface_id, 10, 1, 2);
    store_mesh(&mut state, mesh_key, publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])));
    state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 5.0, project_origin_lat: 5.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    let purged = apply_terrain_style_if_changed_state(&mut state);
    assert_eq!(purged.len(), 1, "an origin change should purge the previously-cached terrain mesh");
    assert!(state.meshes.is_empty());
}
//#endregion TerrainTests

//#region BrushPreviewTests
#[test]
fn brush_preview_mesh_id_falls_back_to_box_without_mesh_url() {
    assert_eq!(brush_preview_mesh_id(None), "box");
    assert_eq!(brush_preview_mesh_id(Some("/assets/tower.glb")), mesh_id_from_url("/assets/tower.glb"));
}
//#endregion BrushPreviewTests

//#region ContextMenuTests
#[test]
fn resolve_world_context_menu_target_prioritizes_vortex_over_object_over_reference() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.local_hover_id = Some("reference:site-plan.png".into());
    assert_eq!(resolve_world_context_menu_target(&state), Some(("reference", "site-plan.png".to_string())));

    state.hovered_component_mode = Some("face".into());
    state.hovered_component_object_id = Some("obj-1".into());
    assert_eq!(resolve_world_context_menu_target(&state), Some(("object", "obj-1".to_string())));

    state.hovered_vortex_id = Some("vortex-1".into());
    assert_eq!(resolve_world_context_menu_target(&state), Some(("vortex", "vortex-1".to_string())));
}

#[test]
fn resolve_world_context_menu_target_is_none_without_any_hover() {
    let state = World3dState::new("surface-1".into(), "controller-1".into());
    assert_eq!(resolve_world_context_menu_target(&state), None);
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
    state.bounds = inner;
    state.pick_bounds = inner;
    state.hovered_vortex_id = Some("vortex-1".into());
    handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
    let action = handle_world3d_pointer_button(&mut state, 200.0, 200.0, false, 2, &PointerModifiers::default()).expect("a right release reports the camera");
    assert_eq!(action.action, "setCamera");
    assert!(action.args.expect("args").get("camera").is_some(), "the report carries the pose, never a menu target");
}

#[test]
fn right_drag_reports_the_camera_like_every_other_release() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    state.hovered_vortex_id = Some("vortex-1".into());
    handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
    let action = handle_world3d_pointer_button(&mut state, 260.0, 260.0, false, 2, &PointerModifiers::default()).expect("right release should still sync camera");
    assert_eq!(action.action, "setCamera", "a right-drag reports the camera, and no renderer opens a menu from a drag");
}
//#endregion ContextMenuTests

//#region 🔖️WorldInteractionVerbs
#[test]
fn world_interaction_definition_declares_path_delimited_item_domain() {
    let def = world_interaction_definition();
    assert_eq!(def.id, WORLD_INTERACTION_DOMAIN_ID);
    assert_eq!(def.granularities.iter().map(|granularity| granularity.id.clone()).collect::<Vec<_>>(), vec!["surface".to_string(), "item".to_string()]);
    assert!(matches!(def.hierarchy, HierarchyProvider::PathDelimited { ref delimiter } if delimiter == "/"));
    assert!(def.selection.methods.contains(&SelectionMethod::Pick));
    assert!(def.selection.methods.contains(&SelectionMethod::Rectangle));
    assert!(def.selection.merges.contains(&MergeMode::Additive));
}

#[test]
fn pick_select_emits_batched_interaction_select_for_plain_object_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    // 🕹️ default `granularity` ("object") is neither component-mode nor "mesh" — this is the
    // plain `world`-domain item pick path (see `pick_select_action`'s final fallback branch).
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(inner.w, inner.h), Vec3::ZERO, inner.w, inner.h).expect("object projects");
    let action = pick_select_action(&state, screen[0], screen[1], inner, true, false).expect("pick action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!(WORLD_INTERACTION_DOMAIN_ID));
    assert_eq!(args["method"], json!("pick"));
    assert_eq!(args["merge"], json!("additive"), "shift modifier maps to the canonical MergeMode label");
    let targets = args["targets"].as_array().expect("targets array");
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0]["granularity"], json!(WORLD_ITEM_GRANULARITY_ID));
    assert_eq!(targets[0]["id"], json!("surface-1/obj-1"), "item target id is surfaceId/objectId (PathDelimited)");
}

#[test]
fn marquee_select_emits_batched_interaction_select_with_rectangle_method() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.meshes.insert("mesh-1".into(), topology_mesh());
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.marquee_points = vec![[0.0, 0.0], [400.0, 400.0]];
    let action = marquee_select_action(&mut state, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, false, true).expect("marquee action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    assert_eq!(args["method"], json!("rectangle"));
    assert_eq!(args["merge"], json!("invertive"), "ctrl modifier maps to the canonical MergeMode label");
    assert!(state.marquee_points.is_empty(), "marquee is consumed after gathering targets");
}

#[test]
fn pick_hover_emits_interaction_hover_and_clears_when_nothing_hit() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(inner.w, inner.h), Vec3::ZERO, inner.w, inner.h).expect("object projects");
    let action = pick_hover_action(&mut state, screen[0], screen[1], inner).expect("hover action");
    assert_eq!(action.action, "interactionHover");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!(WORLD_INTERACTION_DOMAIN_ID));
    assert_eq!(args["channel"], json!("pointer"));
    let targets = args["targets"].as_array().expect("targets array");
    assert_eq!(targets[0]["id"], json!("surface-1/obj-1"));

    // 🖱️ Moving off the instance clears — empty `targets` is `next_hover`'s clear signal.
    let action = pick_hover_action(&mut state, 5.0, 5.0, inner).expect("clear action");
    assert_eq!(action.action, "interactionHover");
    let args = action.args.expect("args");
    assert!(args["targets"].as_array().expect("targets array").is_empty());
}

#[test]
fn apply_world_action_preview_applies_interaction_select_and_hover_for_world_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    apply_world_action_preview(
        &mut state,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: action_args(json!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "targets": [{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-1" }],
                "merge": "replace",
                "method": "pick",
            })),
        },
    );
    assert_eq!(state.selected_ids, vec!["obj-1".to_string()], "surfaceId/ prefix is stripped for this surface");

    apply_world_action_preview(
        &mut state,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionHover".into(),
            args: action_args(json!({
                "domainId": WORLD_INTERACTION_DOMAIN_ID,
                "channel": "pointer",
                "targets": [{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-2" }],
            })),
        },
    );
    assert_eq!(state.local_hover_id.as_deref(), Some("obj-2"));

    apply_world_action_preview(&mut state, &ActionDescriptor { controller_id: "controller-1".into(), action: "interactionHover".into(), args: action_args(json!({ "domainId": WORLD_INTERACTION_DOMAIN_ID, "channel": "pointer", "targets": [] })) });
    assert!(state.local_hover_id.is_none(), "empty targets clears hover");
}

#[test]
fn pick_select_emits_bare_id_into_bound_app_domain_when_window_binds_one() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.bound_domain_id = Some("cad".into());
    state.bound_domain_granularity_id = Some("object".into());
    state.meshes.insert("mesh-1".into(), topology_mesh());
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(inner.w, inner.h), Vec3::ZERO, inner.w, inner.h).expect("object projects");
    let action = pick_select_action(&state, screen[0], screen[1], inner, false, false).expect("pick action");
    assert_eq!(action.action, "interactionSelect");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!("cad"), "targets the window's bound app domain, not the OS `world` fallback");
    let targets = args["targets"].as_array().expect("targets array");
    assert_eq!(targets[0]["granularity"], json!("object"), "uses the bound domain's own granularity, not `item`");
    assert_eq!(targets[0]["id"], json!("obj-1"), "bare id — a bound domain is single-surface-scoped, no surfaceId/ prefix");
}

#[test]
fn pick_hover_emits_bare_id_into_bound_app_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.bound_domain_id = Some("cad".into());
    state.bound_domain_granularity_id = Some("object".into());
    state.meshes.insert("mesh-1".into(), topology_mesh());
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(inner.w, inner.h), Vec3::ZERO, inner.w, inner.h).expect("object projects");
    let action = pick_hover_action(&mut state, screen[0], screen[1], inner).expect("hover action");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!("cad"));
    let targets = args["targets"].as_array().expect("targets array");
    assert_eq!(targets[0]["granularity"], json!("object"));
    assert_eq!(targets[0]["id"], json!("obj-1"));
}

#[test]
fn marquee_select_emits_bare_ids_into_bound_app_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.bound_domain_id = Some("features".into());
    state.bound_domain_granularity_id = Some("pin".into());
    state.meshes.insert("mesh-1".into(), topology_mesh());
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() });
    state.marquee_points = vec![[0.0, 0.0], [400.0, 400.0]];
    let action = marquee_select_action(&mut state, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 }, false, false).expect("marquee action");
    let args = action.args.expect("args");
    assert_eq!(args["domainId"], json!("features"));
    let targets = args["targets"].as_array().expect("targets array");
    assert_eq!(targets[0]["granularity"], json!("pin"));
    assert_eq!(targets[0]["id"], json!("obj-1"));
}

#[test]
fn apply_world_action_preview_respects_bound_app_domain_and_ignores_other_domains() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.bound_domain_id = Some("cad".into());
    state.bound_domain_granularity_id = Some("object".into());
    // 🚫️ An action for the OS `world` fallback domain must NOT apply once this window is bound to
    // its own app domain — otherwise the same click could ever light up two selection universes.
    apply_world_action_preview(
        &mut state,
        &ActionDescriptor {
            controller_id: "controller-1".into(),
            action: "interactionSelect".into(),
            args: action_args(json!({ "domainId": WORLD_INTERACTION_DOMAIN_ID, "targets": [{ "granularity": WORLD_ITEM_GRANULARITY_ID, "id": "surface-1/obj-1" }], "merge": "replace", "method": "pick" })),
        },
    );
    assert!(state.selected_ids.is_empty(), "an unbound-domain action must not apply once this window binds its own domain");

    apply_world_action_preview(
        &mut state,
        &ActionDescriptor { controller_id: "controller-1".into(), action: "interactionSelect".into(), args: action_args(json!({ "domainId": "cad", "targets": [{ "granularity": "object", "id": "obj-1" }], "merge": "replace", "method": "pick" })) },
    );
    assert_eq!(state.selected_ids, vec!["obj-1".to_string()], "bare id applies as-is — no surfaceId/ stripping for a bound domain");
}

#[test]
fn sync_world3d_state_captures_scene_bound_domain() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let scene = scene_with_selection_and_domain("{}", Some(("cad", "object")));
    sync_world3d_state(&mut state, &scene, Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 });
    assert_eq!(state.bound_domain_id.as_deref(), Some("cad"));
    assert_eq!(state.bound_domain_granularity_id.as_deref(), Some("object"));
    assert_eq!(resolved_domain_id(&state), "cad");
    assert_eq!(resolved_domain_granularity_id(&state), "object");
}
//#endregion 🔖️WorldInteractionVerbs

//#region 🌉️World3dSceneBridge
/// 🌉️ The committed generation3d preview payload this lane drives the bridge with. Produced by the
/// real pipeline (`parse_dsl` → `FlowHost::evaluate` → `tessellate_geometry(0.05)` →
/// `preview_payload_from_eval`) in `s.procedural.generation3d`'s own example-geometry harness, whose
/// `parry3d` oracle independently confirms the same solid's volume and extent — see the fixture's
/// `provenance` block, and the twin assertion in that harness that keeps this file from rotting.
const SCENE_BRIDGE_FIXTURE: &str = include_str!("../../🧫️fixtures/🌉️scene-bridge/🔣️.json");

pub(super) fn scene_bridge_fixture() -> serde_json::Value {
    serde_json::from_str(SCENE_BRIDGE_FIXTURE).expect("scene bridge fixture parses")
}

/// 🎬️ Builds the exact `World3dScene` the generation3d preview window publishes for the fixture.
pub(super) fn scene_from_bridge_fixture(fixture: &serde_json::Value) -> UiComponentSceneNode {
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
pub(super) fn drive_scene_bridge(state: &mut World3dState, scene: &UiComponentSceneNode, bounds: Rect) {
    sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_scene_bridge(state, context)) {
            World3dSceneBridgeStep::Complete => break,
            World3dSceneBridgeStep::Pending => {}
            step => panic!("scene bridge stopped at {step:?} on turn {turn} (fault {:?})", state.snapshot_fault),
        }
        assert!(turn < 4_095, "scene bridge did not seal within its turn ceiling");
    }
    sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        assert_ne!(with_world_step_context(64, |context| step_world3d_draw_rebuild(state, context)), WorldDrawRebuildStep::Fault, "draw rebuild faulted on turn {turn}");
        match with_world_step_context(64, |context| step_world3d_snapshot(state, context)) {
            World3dSnapshotApplyStep::Complete | World3dSnapshotApplyStep::Idle => break,
            World3dSnapshotApplyStep::Pending => {}
            step => panic!("snapshot apply stopped at {step:?} on turn {turn} (fault {:?})", state.snapshot_fault),
        }
        assert!(turn < 4_095, "snapshot apply did not complete within its turn ceiling");
    }
    for turn in 0..4_096 {
        let draw_step = with_world_step_context(64, |context| step_world3d_draw_rebuild(state, context));
        assert_ne!(draw_step, WorldDrawRebuildStep::Fault, "draw retirement faulted on turn {turn}");
        let bridge_step = with_world_step_context(64, |context| step_world3d_scene_bridge(state, context));
        assert_ne!(bridge_step, World3dSceneBridgeStep::Fault, "bridge retirement faulted on turn {turn}");
        if world3d_draw_rebuild_terminal_is_empty(state) && state.scene_bridge_retired.is_none() {
            break;
        }
        assert!(turn < 4_095, "scene bridge retirement did not complete within its turn ceiling");
    }
}

const CAMERA_FRAMING_FIXTURE: &str = include_str!("../../../../../../../🔨️modules/🖱️ui/🧪️fixtures/🎥️world3d-camera-framing/🔣️.json");

fn camera_framing_fixture() -> serde_json::Value {
    serde_json::from_str(CAMERA_FRAMING_FIXTURE).expect("camera framing fixture parses")
}

fn camera_framing_scene(fixture: &serde_json::Value, camera: &str, geometry: bool, revision: Option<u64>) -> UiComponentSceneNode {
    let mut scene = scene_with_selection("{}");
    let world = scene.world_3d.as_mut().expect("world scene");
    world.meshes_json = if geometry { serde_json::Value::Array(vec![fixture["mesh"].clone()]).to_string() } else { "[]".into() };
    world.instances_json = if geometry { fixture["instances"].to_string() } else { "[]".into() };
    world.camera_json = fixture["cameras"][camera].to_string();
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
    let camera = state.orbit.to_camera();
    let actual_position = [camera.position.x, camera.position.y, camera.position.z];
    let actual_target = [camera.target.x, camera.target.y, camera.target.z];
    for axis in 0..3 {
        assert!((actual_position[axis] - expected["position"][axis].as_f64().expect("position") as f32).abs() < 1e-4, "position axis {axis}: {actual_position:?}");
        assert!((actual_target[axis] - expected["target"][axis].as_f64().expect("target") as f32).abs() < 1e-5, "target axis {axis}: {actual_target:?}");
    }
    assert!((state.orbit.distance - expected["distance"].as_f64().expect("distance") as f32).abs() < 1e-4);
    assert!((state.orbit.zoom - expected["zoom"].as_f64().expect("zoom") as f32).abs() < 1e-5);
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
        (World3dState::new("surface-top".into(), "controller".into()), "orthographicTop", "orthographicTopAfterRawContent"),
        (World3dState::new("surface-perspective".into(), "controller".into()), "perspective", "perspective"),
    ];
    for (state, camera, _) in &mut panes {
        drive_scene_bridge(state, &camera_framing_scene(&fixture, camera, false, None), viewport);
        assert_eq!(step_world3d_camera_fit(state), World3dCameraFitStep::Idle);
        assert!(!world3d_cursor_work_pending(state), "an exactly witnessed empty generation sleeps");
    }
    for (state, camera, _) in &mut panes {
        drive_scene_bridge(state, &camera_framing_scene(&fixture, camera, true, None), viewport);
        assert!(world3d_cursor_work_pending(state), "a sealed draw generation re-arms its own fit lane");
    }

    let ceiling = fixture["instances"].as_array().unwrap().len() * 8 + 2;
    let mut applied = [false; 2];
    for turn in 0..ceiling {
        for (index, (state, _, _)) in panes.iter_mut().enumerate() {
            if !applied[index] {
                applied[index] = step_world3d_camera_fit(state) == World3dCameraFitStep::Applied;
            }
        }
        if applied.into_iter().all(|complete| complete) {
            break;
        }
        assert!(panes.iter().any(|(state, _, _)| world3d_cursor_work_pending(state)), "turn {turn}: unfinished retained camera work owns the next frame");
    }
    assert!(applied.into_iter().all(|complete| complete), "both visible panes finish inside one exact corner walk");
    for (state, _, expected) in &panes {
        assert_camera_fit(state, &fixture["expect"][expected]);
        assert!(!world3d_cursor_work_pending(state), "an applied fit releases its wake");
    }
}

/// 📐️ A dock template lands after the producer camera lease and before auto-fit. Its look
/// direction owns the fitted eye, while the measured geometry owns target and distance.
#[test]
fn initial_projection_template_seeds_the_actual_delivered_camera_before_fit() {
    let fixture = camera_framing_fixture();
    let viewport = Rect::new(0.0, 0.0, fixture["viewport"][0].as_f64().unwrap() as f32, fixture["viewport"][1].as_f64().unwrap() as f32);
    let mut state = World3dState::new("surface-template".into(), "controller".into());
    let direction = [0.75, -0.75, 0.55];
    let up = [0.0, 0.0, 1.0];
    assert!(!apply_world3d_initial_projection_seed(&mut state, CameraProjection3d::Perspective, ui_wgpu::wgpu::WorldProjectionOrientation::Free, false, direction, up), "a template never races ahead of the delivered camera lease");

    drive_scene_bridge(&mut state, &camera_framing_scene(&fixture, "perspective", true, None), viewport);
    assert!(apply_world3d_initial_projection_seed(&mut state, CameraProjection3d::Perspective, ui_wgpu::wgpu::WorldProjectionOrientation::Free, false, direction, up));
    assert!(!apply_world3d_initial_projection_seed(&mut state, CameraProjection3d::Perspective, ui_wgpu::wgpu::WorldProjectionOrientation::Free, false, direction, up), "the same pane consumes its initial template once");
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
    assert_eq!(state.instance_positions.get("obj-left"), Some(&[2.0, 0.0, 0.0]), "the bridge retains React's raw pre-asset position for projection framing");
    sync_world3d_state(&mut state, &scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| step_world3d_snapshot(&mut state, context)) {
            World3dSnapshotApplyStep::Complete => break,
            World3dSnapshotApplyStep::Pending => {}
            step => panic!("url snapshot stopped at {step:?} on turn {turn}"),
        }
        assert!(turn < 4_095, "url snapshot did not apply inside its fixture ceiling");
    }
    assert!(state.draw_rebuild.is_some(), "the applied delivery retains its sealed rebuild for the next transaction");
    assert!(apply_world3d_initial_projection_seed(&mut state, CameraProjection3d::Orthographic, ui_wgpu::wgpu::WorldProjectionOrientation::Cardinal(ui_wgpu::wgpu::WorldCardinalView::Top), false, [0.0, 0.0, 1.0], [0.0, 1.0, 0.0],));
    sync_world3d_state(&mut state, &scene, bounds);
    assert!(!state.projection_frame_owed, "the exact Top document sync consumes its content-frame request");
    assert!((state.orbit.target.x - 2.0).abs() < 1e-5, "the pre-asset Top frame targets the raw instance position");
    assert!(state.draw_rebuild.is_some(), "the following document sync keeps the camera-only revision on the same delivery");
    for turn in 0..64 {
        match with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state, context)) {
            WorldDrawRebuildStep::Complete => break,
            WorldDrawRebuildStep::Pending => {}
            step => panic!("camera-only seed invalidated the delivered rebuild at {step:?} on turn {turn}"),
        }
        assert!(turn < 63, "url draw rebuild did not publish inside its exact fixture ceiling");
    }
    let draw = state.draws.iter().find(|draw| draw.mesh_key == URL_MESH_ID).expect("url draw survives the template seed");
    assert_eq!(draw.instances.iter().map(|instance| instance.id.as_str()).collect::<Vec<_>>(), ["obj-left"]);

    offer_missing_mesh_fetches(&mut state);
    offer_missing_mesh_fetches(&mut state);
    let mut owner = take_next_world3d_asset(&mut state).expect("the surviving draw admits one GLB request");
    assert_eq!((owner.kind(), owner.url()), (WorldAssetRequestKind::Glb, URL_MESH_URL));
    assert!(take_next_world3d_asset(&mut state).is_none(), "the delivered url is admitted exactly once");
    let mesh = publish_oracle_mesh_at_revision(mesh_oracle_from_buffers(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2]), state.interaction_revision);
    publish_world3d_asset_mesh_lease(&mut state, URL_MESH_URL, mesh).expect("the requested GLB publishes into the delivered draw");
    drive_camera_fit(&mut state);
    assert!(state.fit_applied_key.is_some(), "the loaded mesh generation completes the retained auto-fit");
    assert!(state.orbit.target.x > 2.0 && state.orbit.target.y > 0.0, "the fit measures the translated loaded mesh rather than a placeholder origin");

    owner.begin_close();
    return_world3d_asset(&mut state, owner).unwrap();
    while retire_cancelled_world3d_asset_step(&mut state) {}
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
    let seed = state.orbit.clone();
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.fit_applied_key, None, "an empty group does not consume React's fit key");
    assert_eq!(state.orbit.target, seed.target, "no placeholder origin is framed");

    let loaded = camera_framing_scene(&fixture, "perspective", true, None);
    drive_scene_bridge(&mut state, &loaded, viewport);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Pending);
    state.geometry_generation = state.geometry_generation.wrapping_add(1);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Stale, "a residency change retires the partial measurement");
    drive_camera_fit(&mut state);
    assert_camera_fit(&state, &fixture["expect"]["perspective"]);
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
    state.orbit.orbit(0.4, -0.2);
    state.camera_user_moved = true;
    let moved = state.orbit.clone();
    sync_world3d_state(&mut state, &scene, viewport);
    assert_eq!(step_world3d_camera_fit(&mut state), World3dCameraFitStep::Idle);
    assert_eq!(state.orbit.yaw, moved.yaw, "same owner never takes back a user orbit");

    let next_revision = camera_framing_scene(&fixture, "perspective", true, Some(fixture["fit"]["revision"].as_u64().unwrap() + 1));
    sync_world3d_state(&mut state, &next_revision, viewport);
    assert!(drive_camera_fit(&mut state) >= fixture["instances"].as_array().unwrap().len() * 8, "a new revision remeasures resident geometry through the bounded cursor");
    assert_eq!(state.orbit.yaw, moved.yaw, "reframing preserves the user's look direction");
    state.orbit.orbit(-0.3, 0.1);
    state.camera_user_moved = true;

    let mut next_seed_fixture = fixture.clone();
    next_seed_fixture["cameras"]["perspective"]["position"][0] = serde_json::Value::from(12.0);
    let next_seed = camera_framing_scene(&next_seed_fixture, "perspective", true, Some(fixture["fit"]["revision"].as_u64().unwrap() + 1));
    drive_scene_bridge(&mut state, &next_seed, viewport);
    drive_camera_fit(&mut state);
    assert!(state.fit_applied_key.is_some(), "an external camera seed changes the fit key and owns a fresh frame");
}

#[test]
fn scene_bridge_renders_the_generation3d_preview_payload_into_a_snapshot() {
    let fixture = scene_bridge_fixture();
    let scene = scene_from_bridge_fixture(&fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);

    let expect = &fixture["expect"];
    assert_eq!(state.snapshot_fault, None, "the bridge must publish without faulting");
    assert_eq!(state.draws.iter().count(), expect["draws"].as_u64().expect("draws") as usize, "one draw per mesh that actually carries triangles");
    let draw = state.draws.get(0).expect("published draw");
    assert_eq!(draw.mesh_key, expect["drawMeshKey"].as_str().expect("mesh key"));
    let instance_ids: Vec<&str> = draw.instances.iter().map(|instance| instance.id.as_str()).collect();
    let expected_instance_ids: Vec<&str> = expect["instanceIds"].as_array().expect("instance ids").iter().map(|id| id.as_str().expect("instance id")).collect();
    assert_eq!(instance_ids, expected_instance_ids, "instance ids stay channel-qualified across the wire");

    let mesh = *state.meshes.get(&draw.mesh_key).expect("published mesh geometry");
    let schema = mesh.schema().expect("mesh schema");
    assert_eq!(schema.indices / 3, expect["triangles"].as_u64().expect("triangles") as u32, "the bridged mesh carries the tessellated triangle count, not a placeholder box");
    assert_eq!(schema.vertices, expect["vertices"].as_u64().expect("vertices") as u32);

    for id in expect["wireOnlyMeshesDropped"].as_array().expect("dropped meshes") {
        assert!(!state.meshes.contains_key(id.as_str().expect("mesh id")), "a wire-only preview mesh carries no triangles and must not become a draw");
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
    assert_eq!(state.snapshot_fault, None, "a framed delivery must apply without faulting");
    assert!(state.fit_applied_key.is_some(), "the delivery's own fit lane framed this document once");
    assert_eq!(state.draws.iter().count(), fixture["expect"]["draws"].as_u64().expect("draws") as usize, "the framing must not strand the delivery's draw rebuild");
    assert!(state.interaction_revision >= state.snapshot_lease.expect("applied lease").revision, "fit advances only after the applied delivery owns the surface revision");
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
    assert_eq!(state.provisional_instance_ids, HashSet::from([provisional]), "exactly the stamped record is provisional");
}

/// 🧭️ Only a world surface bound to a window AND holding an applied trace lane owes that window a cursor echo.
#[test]
fn world_surfaces_echo_their_trace_cursor_under_their_window_instance() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/⏯️tool-run/🧫️fixtures/📼️trace-pages.json")).expect("trace pages fixture");
    let bytes: Vec<u8> = fixture["deltas"][0]["hex"].as_str().expect("delta hex").as_bytes().chunks(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex"), 16).expect("hex")).collect();
    let delta = semio_framework_tool_run::ToolRunTraceDelta::decode(&bytes).expect("fixture delta decodes");
    let lane = base64_codec::base64_url_encode(&bytes);
    let mut bound = World3dState::new("surface-bound".into(), "controller".into());
    bound.tool_run_trace_window_id = Some("world-left".into());
    bound.tool_run_trace.apply_lane(Some(&lane)).expect("lane applies");
    let mut unbound = World3dState::new("surface-unbound".into(), "controller".into());
    unbound.tool_run_trace.apply_lane(Some(&lane)).expect("lane applies");
    let mut idle = World3dState::new("surface-idle".into(), "controller".into());
    idle.tool_run_trace_window_id = Some("world-right".into());
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
    apply_runtime_draw_flags(&mut state);

    let expect = &fixture["expect"];
    assert_eq!(state.selected_ids, expect["selectedIds"].as_array().expect("selected").iter().map(|id| id.as_str().expect("id").to_string()).collect::<Vec<_>>());
    assert_eq!(state.local_hover_id.as_deref(), expect["hoveredId"].as_str());
    let draw = state.draws.get(0).expect("published draw");
    let instance = draw.instances.first().expect("published instance");
    assert!(instance.selected, "the scene's own selection document must paint the instance selected");

    let camera = state.orbit.to_camera();
    let expected_camera = expect["cameraPosition"].as_array().expect("camera");
    for axis in 0..3 {
        assert!((f64::from(camera.position.to_array()[axis]) - expected_camera[axis].as_f64().expect("axis")).abs() < 1.0e-4, "camera axis {axis}: {:?}", camera.position);
    }

    let sun = environment_light_dir(&state.environment);
    let expected_sun = expect["sunDirection"].as_array().expect("sun");
    for axis in 0..3 {
        assert!((f64::from(sun[axis]) - expected_sun[axis].as_f64().expect("axis")).abs() < 1.0e-5, "sun axis {axis}: {sun:?}");
    }
}

#[test]
fn scene_bridge_binds_the_apps_interaction_domain_for_world_picking() {
    let fixture = scene_bridge_fixture();
    let scene = scene_from_bridge_fixture(&fixture);
    let bounds = Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 };
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    drive_scene_bridge(&mut state, &scene, bounds);
    state.pick_bounds = bounds;

    assert_eq!(resolved_domain_id(&state), fixture["domainId"].as_str().expect("domain"));
    assert_eq!(resolved_domain_granularity_id(&state), fixture["domainGranularityId"].as_str().expect("granularity"));
    let instance_id = state.draws.get(0).expect("published draw").instances.first().expect("published instance").id.clone();
    assert_eq!(resolved_item_id(&state, &instance_id), instance_id, "a bound app domain addresses the bare channel-qualified id — never `surfaceId/id`");
    let interaction_id = state.instance_interaction_ids.get(&instance_id).expect("the scene bridge retains the instance's topology target").clone();
    assert_eq!(interaction_id, "extrude@solid");

    // 🎯️ Aim through the fixture camera's own target, which is the centre of the prism's base face
    // — the one point guaranteed both inside the solid and inside the 45° frustum.
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(bounds.w, bounds.h), Vec3::ZERO, bounds.w, bounds.h).expect("camera target projects");
    let hit = pick_instance_at(&state, screen[0], screen[1], bounds);
    assert_eq!(hit.as_deref(), Some(instance_id.as_str()), "the bridged geometry is what a world pick actually hits");
    let select = pick_select_action(&state, screen[0], screen[1], bounds, false, false).expect("pick emits an action");
    assert_eq!(select.action, "interactionSelect");
    let args = select.args.expect("pick args");
    assert_eq!(args["domainId"].as_str(), fixture["domainId"].as_str());
    assert_eq!(args["targets"][0]["granularity"].as_str(), fixture["domainGranularityId"].as_str());
    assert_eq!(args["targets"][0]["id"].as_str(), Some(interaction_id.as_str()));

    state.local_hover_id = None;
    let hover = pick_hover_action(&mut state, screen[0], screen[1], bounds).expect("hover emits an action");
    assert_eq!(hover.action, "interactionHover");
    let args = hover.args.expect("hover args");
    assert_eq!(args["domainId"].as_str(), fixture["domainId"].as_str());
    assert_eq!(args["targets"][0]["id"].as_str(), Some(interaction_id.as_str()));
}
//#endregion 🌉️World3dSceneBridge

/// 🧱️ The `boxed_fixed_slots` law for this module's fixed slot tables, against the one committed
/// budget every implementation of it reads (`the committed fixed-slot fixture`).
///
/// Asserts the measured shape of each table (capacity, one slot's bytes, the owner's own bytes)
/// against that record, that each owner is smaller than the table it owns — the structural proof the
/// slots are heap-first rather than an inline `[T; N]` field — and then constructs them on a thread
/// holding only the fixture's `boundedThreadStackBytes`. `Builder::stack_size` overrides
/// `RUST_MIN_STACK`, so the repo runner's 128 MiB floor cannot hide a re-inflated frame here.
#[test]
fn world_interaction_object_slot_table_is_heap_first_and_fits_a_bounded_thread_stack() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json")).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "infinite::world")
        .map(|table| {
            semio_framework_async::FixedSlotTableBudget::new(
                table["owner"].as_str().expect("owner"),
                table["capacity"].as_u64().expect("capacity") as usize,
                table["elementSizeBytes"].as_u64().expect("element bytes") as usize,
                table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize,
            )
        })
        .collect();
    let measured = vec![semio_framework_async::FixedSlotTableBudget::new("world::WorldInteractionObjectRegistry", WORLD_INTERACTION_OBJECT_CAPACITY, size_of::<Option<WorldInteractionObjectSlot>>(), size_of::<WorldInteractionObjectRegistry>())];
    semio_framework_async::assert_fixed_slot_tables(
        "infinite::world",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
            drop(WorldInteractionObjectRegistry::default());
        },
    );
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
    let draw = |key: &str, version: u64| SceneDraw3d { mesh_key: key.into(), mesh_version: version, instances: vec![Instance3d { id: key.into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false, material: Default::default() }], shadow_role: Default::default() };
    let mut draws = vec![draw("resident", 3), draw("resident", 4), draw("ghost-still-landing", 0)];
    retain_ensured_world_draws(&gpu, &mut draws);

    assert_eq!(draws.len(), 1, "only the draw whose exact identity this frame uploads survives");
    assert_eq!((draws[0].mesh_key.as_str(), draws[0].mesh_version), ("resident", 3), "and it is the ensured one, not a stale version of the same key");
    assert!(gpu.has_mesh_request("resident", 3) && !gpu.has_mesh_request("resident", 4), "the predicate is exact on BOTH key and version");
}

/// 🧷️ LAW: every scene pass the world publishes runs its mesh-backed draw lists through an exact
/// residency filter first. The filter is the structural guarantee that a sixth
/// ghost site cannot reintroduce the fatal submit fault.
#[test]
fn the_scene_pass_is_published_behind_the_residency_filter() {
    let world = include_str!("../../🦀️.rs");
    assert_eq!(world.matches("ctx.draw.push_scene_pass(ScenePass3d {").count(), 1, "the world publishes exactly one scene pass");
    assert!(
        world.contains("retain_ensured_world_draws(gpu, &mut shadow_draws);\n    retain_ensured_world_draws(gpu, &mut culled_draws);\n    retain_ensured_world_draws(gpu, &mut translucent_draws);\n    retain_ensured_world_material_draws(gpu, &mut material_draws);\n    ctx.draw.push_scene_pass(ScenePass3d {"),
        "the caster, standard color and material draw lists are filtered immediately before the pass is pushed"
    );
}

/// 🎨️ LAW: decoded paint pixels become resident only for the exact mesh generation shape that can
/// consume them; codec work remains outside the frame-owned World state.
#[test]
fn decoded_mesh_paint_publication_requires_rgba_and_matching_uvs() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mut painted = triangle_mesh_oracle();
    painted.uvs = vec![0.0, 0.0, 1.0, 0.0, 0.5, 1.0];
    state.meshes.insert("painted".into(), publish_oracle_mesh(painted)).expect("painted mesh admitted");
    state.mesh_versions.insert("painted".into(), 1).expect("painted revision admitted");
    let paint = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 3 }), 96);
    assert!(apply_decoded_mesh_paint_image(&mut state, "painted", DecodedMeshPaintImage { raster: paint }));
    let resident = state.mesh_paint_textures.get("painted").expect("decoded paint pixels became resident");
    assert_eq!((resident.identity.descriptor().width, resident.identity.descriptor().height), (1, 1));

    state.meshes.insert("plain".into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("plain mesh admitted");
    state.mesh_versions.insert("plain".into(), 1).expect("plain revision admitted");
    let plain = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 3 }), 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state, "plain", DecodedMeshPaintImage { raster: plain }), "a mesh without one UV per vertex cannot sample a paint map");
    let stale = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 2, uv_revision: 2, uv_count: 3 }), 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state, "painted", DecodedMeshPaintImage { raster: stale }), "paint from another mesh generation cannot cross the resident topology");
    let wrong_uvs = test_scene_raster_lease(1, 1, SceneRasterProfile::MeshPaintMapNoColorSpace, Some(SceneRasterMeshSeal { mesh_revision: 1, uv_revision: 1, uv_count: 2 }), 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state, "painted", DecodedMeshPaintImage { raster: wrong_uvs }), "paint UV cardinality is exact");
    let wrong_profile = test_scene_raster_lease(1, 1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, 1);
    assert!(!apply_decoded_mesh_paint_image(&mut state, "painted", DecodedMeshPaintImage { raster: wrong_profile }), "a reference raster cannot cross the mesh-paint profile seal");
}

/// 🫧 LAW: retained transparent Standard objects use Three's transformed bounding-sphere-centre
/// projected depth and preserve producer insertion order for exact ties.
#[test]
fn transparent_standard_draws_match_the_recorded_three_projected_order() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎨️scene-shading/🔣️.json")).unwrap();
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
            state.meshes.insert(id.into(), publish_oracle_mesh(mesh)).expect("sort mesh admitted");
            draws.push(SceneMaterialDraw3d {
                mesh_key: id.into(),
                mesh_version: 0,
                instances: vec![Instance3d {
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
        sort_world3d_translucent_material_draws(&state.meshes, view_proj, &mut draws);
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
    state.active_utility = "select".into();
    state.meshes.insert(GUMBALL_PLANE_MESH.into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("plane lease admitted");
    state.mesh_versions.insert(GUMBALL_PLANE_MESH.into(), 7).expect("plane version admitted");
    state.meshes.insert("mesh-1".into(), publish_oracle_mesh(triangle_mesh_oracle())).expect("body lease admitted");
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: true, hovered: false, material: Default::default() }], shadow_role: Default::default() }).expect("body draw admitted");
    state.selected_ids = vec!["obj-1".into()];
    append_gumball_geometry(&mut lines, &mut translucent, &mut gpu, &state, &Camera3d::default(), &state.meshes, &state.mesh_versions);

    assert_eq!(translucent.len(), 3, "the gumball draws one quad per axis plane");
    let requested: Vec<(String, u64)> = gpu.mesh_requests[..gpu.mesh_request_len].iter().flatten().cloned().collect();
    for draw in &translucent {
        assert!(requested.iter().any(|(key, version)| key == &draw.mesh_key && *version == draw.mesh_version), "gumball draw {}@{} was never ensured on the GPU; requested={requested:?}", draw.mesh_key, draw.mesh_version);
    }
    assert!(requested.iter().all(|(key, _)| key == GUMBALL_PLANE_MESH), "the gumball ensures nothing but its own plane: {requested:?}");
}

/// 🔘️ LAW: the gumball plane is PINNED in the mesh pool. It belongs to no document draw, so every
/// `sync_mesh_pool` would otherwise see it as stale and evict the mesh the very next frame draws.
#[test]
fn the_gumball_plane_is_pinned_against_pool_eviction() {
    let world = include_str!("../../🦀️.rs");
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
    let theme = ui_wgpu::wgpu::Theme::light();
    let rgba = |color: ui_wgpu::wgpu::Rgba| [color.r, color.g, color.b, color.a];
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

    let instance = Instance3d { id: "one".into(), model: Instance3d::model_from_trs([0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0]), color: [0.1, 0.2, 0.3, 1.0], selected: false, hovered: false, material: Default::default() };
    assert_eq!(world3d_style_paint(&theme, MeshStyleState::default(), false, instance.clone()).color, [0.1, 0.2, 0.3, 1.0], "🎨️ a neutral authored instance keeps its producer colour");
    let painted = world3d_style_paint(&theme, MeshStyleState { disabled: true, ..MeshStyleState::default() }, false, instance);
    assert_eq!(painted.color[3], 0.45, "🎨️ and a styled one takes the row's opacity into its alpha");
}

/// 🎨️ LAW: neutral color provenance remains retained until paint, so appearance changes recolor
/// only semantic defaults; React's neutral/disabled vertex mode and every static active emissive row
/// are carried through the packed instance material instead of inferred from interaction flags.
#[test]
fn retained_color_provenance_and_static_material_policy_match_react() {
    let light = ui_wgpu::wgpu::Theme::light();
    let dark = ui_wgpu::wgpu::Theme::dark();
    let instance = |source| Instance3d {
        id: "fixture".into(),
        model: Mat4::identity(),
        color: [0.25, 0.5, 0.75, 0.8],
        selected: false,
        hovered: false,
        material: ui_wgpu::wgpu::SceneInstanceMaterial3d { color_source: source, ..Default::default() },
    };
    let semantic = |theme: &ui_wgpu::wgpu::Theme| world3d_style_paint(theme, MeshStyleState::default(), false, instance(ui_wgpu::wgpu::SceneColorSource3d::SemanticNeutral));
    assert_eq!(semantic(&light).color, [light.panel.r, light.panel.g, light.panel.b, light.panel.a * 0.8]);
    assert_eq!(semantic(&dark).color, [dark.panel.r, dark.panel.g, dark.panel.b, dark.panel.a * 0.8]);
    for source in [ui_wgpu::wgpu::SceneColorSource3d::Authored, ui_wgpu::wgpu::SceneColorSource3d::Environment] {
        assert_eq!(world3d_style_paint(&dark, MeshStyleState::default(), false, instance(source)).color, [0.25, 0.5, 0.75, 0.8]);
    }

    let neutral_vertex = world3d_style_paint(&light, MeshStyleState::default(), true, instance(ui_wgpu::wgpu::SceneColorSource3d::Authored));
    assert_eq!(neutral_vertex.color, [1.0, 1.0, 1.0, 0.8]);
    assert!(neutral_vertex.material.preserve_vertex_color);
    let disabled_vertex = world3d_style_paint(&light, MeshStyleState { disabled: true, ..Default::default() }, true, instance(ui_wgpu::wgpu::SceneColorSource3d::Authored));
    assert_eq!(&disabled_vertex.color[..3], &[1.0, 1.0, 1.0]);
    assert!((disabled_vertex.color[3] - 0.36).abs() <= f32::EPSILON);
    assert!(disabled_vertex.material.preserve_vertex_color);
    for (style, expected) in [
        (MeshStyleState { selected: true, ..Default::default() }, 0.35),
        (MeshStyleState { hovered: true, ..Default::default() }, 0.08),
        (MeshStyleState { highlighted: true, ..Default::default() }, 0.2),
        (MeshStyleState { provisional: true, ..Default::default() }, 0.2),
    ] {
        let painted = world3d_style_paint(&light, style, true, instance(ui_wgpu::wgpu::SceneColorSource3d::SemanticNeutral));
        assert!(!painted.material.preserve_vertex_color);
        assert_eq!(painted.material.emissive_intensity, expected);
    }
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
    state.bounds = Rect::new(0.0, 0.0, 478.0, 814.0);
    let before = world3d_live_camera_json(&state);
    assert_eq!(before["projection"]["mode"]["kind"], "perspective", "🎥️ an untouched pane opens perspective");
    assert_eq!(before["userMoved"], serde_json::Value::Bool(false));

    assert!(apply_world3d_projection_spec(&mut state, CameraProjection3d::Orthographic, ui_wgpu::wgpu::WorldProjectionOrientation::Cardinal(ui_wgpu::wgpu::WorldCardinalView::Top), false));
    let after = world3d_live_camera_json(&state);
    assert_eq!(after["projection"]["mode"]["kind"], "orthographic", "🎥️ the switch is visible in the row the probe reads");
    assert_eq!(after["projection"]["orientation"], "Cardinal(Top)", "🎥️ and so is the plane its framing measures in");
    assert!(after["zoom"].as_f64().unwrap_or_default() > 1.0, "🎥️ a parallel pane reports its real frustum scale: {}", after["zoom"]);

    state.orbit.target = Vec3::new(3.5, 0.0, 0.005);
    let moved = world3d_live_camera_json(&state);
    let target: Vec<f64> = moved["target"].as_array().expect("the row carries a target").iter().filter_map(serde_json::Value::as_f64).collect();
    assert!((target[0] - 3.5).abs() < 1e-6 && target[1].abs() < 1e-6 && (target[2] - 0.005).abs() < 1e-6, "🎥️ the row follows the orbit, not the wire: {target:?}");
}

/// 📐️ LAW: selecting a projection template re-arms the pane's framing. React's
/// `handleProjectionKindChange` hands the pending spec to `seedPendingWorldProjectionCamera`, which
/// re-runs `frameWorldProjectionPose` (`🌐️World3dHost/🟦️.tsx`).
///
/// 🩸️ `apply_world3d_projection` arms `camera_user_moved`, which is the very gate
/// `sync_world3d_projection_content_frame` refuses on — so a pane switched to `Orthographic` took
/// the parallel frustum and kept the perspective pane's zoom forever.
#[test]
fn a_projection_selection_rearms_the_panes_framing() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.bounds = Rect::new(0.0, 0.0, 478.0, 814.0);
    state.camera_user_moved = true;
    state.projection_frame_key = Some(7);
    state.projection_frame_zoom = Some(3.0);

    let top = ui_wgpu::wgpu::WorldProjectionOrientation::Cardinal(ui_wgpu::wgpu::WorldCardinalView::Top);
    assert!(apply_world3d_projection_spec(&mut state, CameraProjection3d::Orthographic, top, false), "📐️ the press moves the pane");
    assert!(state.projection_frame_owed, "📐️ and owes one framing, immune to the ownership latch the settle it queues arms");
    assert!(state.projection_selected, "📐️ and holds its own spec from here on, React's `externalPendingProjectionSpec`");
    assert_eq!(state.projection_frame_key, None, "📐️ and forgets the extent it last framed");
    assert_eq!(state.projection_frame_zoom, None);
    assert_eq!(state.projection_orientation, top);

    assert!(!apply_world3d_projection_spec(&mut state, CameraProjection3d::Orthographic, top, false), "📐️ pressing the SAME row again changes nothing");
    assert!(apply_world3d_projection_spec(&mut state, CameraProjection3d::Orthographic, ui_wgpu::wgpu::WorldProjectionOrientation::Free, true), "📐️ but a row with the same family and another plane does — Cabinet after Orthographic");
    assert!(state.projection_oblique_off_axis, "📐️ a free oblique frames in (x, z), which is a framing input, not a camera one");

    // 📐️ And the wire never takes the selection back. The settle a press queues publishes
    // `setCamera`, whose payload carries no `projection` member at all, so the guest's echo always
    // arrives in the DELIVERED family — applying it whole is how the switch undid itself in one
    // round trip.
    let mut wired = World3dState::new("surface".into(), "controller".into());
    wired.bounds = Rect::new(0.0, 0.0, 478.0, 814.0);
    assert!(apply_world3d_projection_spec(&mut wired, CameraProjection3d::Orthographic, top, false));
    let echo = OrbitController { projection: CameraProjection3d::Perspective, target: Vec3::new(7.0, 0.0, 0.01), ..OrbitController::default() };
    let held = OrbitController { projection: wired.orbit.projection, ..echo.clone() };
    assert_eq!(held.projection, CameraProjection3d::Orthographic, "📐️ the echoed pose lands, the echoed FAMILY does not");
    assert_eq!(held.target, echo.target, "📐️ everything else the wire says still wins");
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
#[test]
fn the_grid_never_ends_on_a_hard_edge_however_far_the_camera_is() {
    let viewport = Rect { x: 0.0, y: 0.0, w: 956.0, h: 814.0 };
    let base = OrbitController::default().to_camera();
    let step = ui_wgpu::wgpu::lod_grid_step_world(2.0, 10.0).expect("React's helper answers a step") as f32;
    for distance in [12.0_f32, 400.0, 20_000.0, 500_000.0] {
        let camera = ui_wgpu::wgpu::Camera3d { position: Vec3::new(0.0, 0.0, distance), target: Vec3::ZERO, ..base.clone() };
        let mut lines = Vec::new();
        append_lod_grid_lines(&mut lines, 2.0, 10.0, Vec3::ZERO, &camera, viewport, [0.5, 0.5, 0.5, 1.0]);
        assert!(!lines.is_empty(), "🌐️ every camera gets a grid at {distance}");
        assert!(lines.len() <= 8 * (WORLD_GRID_MAX_DIVISIONS as usize + 1), "🌐️ inside the vertex budget at {distance}: {}", lines.len());
        assert!(lines.iter().any(|vertex| vertex.color[3] == 0.0), "🌐️ the rim reaches alpha 0 at {distance}, so the ceiling is invisible");

        // 🌐️ And the ceiling is never spent on the SPACING: every band keeps React's own
        // `lodGridStepWorld` step however far the camera is.
        let mut offsets: Vec<f32> = lines.iter().map(|vertex| vertex.position[1]).collect();
        offsets.sort_by(|left, right| left.partial_cmp(right).expect("finite grid coordinates"));
        offsets.dedup_by(|left, right| (*left - *right).abs() < 1e-4);
        let gap = offsets.windows(2).map(|pair| pair[1] - pair[0]).fold(f32::INFINITY, f32::min);
        assert!((gap - step).abs() < 1e-2, "🌐️ the band keeps React's {step}-unit spacing at {distance}, it read {gap}");
    }
}

/// 🖼️ LAW: reference identity, geometry, and visual state come from the shared Three fixture.
#[test]
fn reference_visual_identity_geometry_and_interaction_match_three() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖼️reference-visual/🔣️.json")).expect("reference visual fixture");
    let identity = fixture["identityCases"].as_array().expect("identity cases");
    let shared_url = identity[0]["url"].as_str().expect("shared URL");
    assert_eq!(identity[1]["url"].as_str(), Some(shared_url));

    let geometry = &fixture["geometry"];
    let source = geometry["sourceNaturalSize"].as_array().expect("source natural size");
    let mut state = World3dState::new("surface".into(), "controller".into());
    let _ = state.reference_pixels.insert(shared_url.into(), test_world_scene_raster(source[0].as_u64().unwrap() as u32, source[1].as_u64().unwrap() as u32, 0));
    for (index, row) in identity.iter().enumerate() {
        state.references.push(WorldReferenceRecord {
            id: Some(row["id"].as_str().unwrap().into()),
            url: Some(shared_url.into()),
            origin: Some([-2.0 + index as f64 * 4.0, 0.0, 0.0]),
            width_world: Some(2.0),
            ..Default::default()
        });
    }
    state.references.push(WorldReferenceRecord { id: Some("locked".into()), url: Some(shared_url.into()), locked: Some(true), ..Default::default() });
    state.interaction_revision = 11;
    let mut registry = WorldInteractionRegistryBuildCursor::new(11);
    for _ in 0..16 {
        if with_world_step_context(1, |context| registry.step(&mut state, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    let reference_ids: Vec<&str> = state
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
    let [actual_width, actual_height] = world_reference_plane_size(&state, shared_url, width);
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

    state.interaction_objects.revision = state.interaction_revision;
    let mut select = WorldObjectPickCursor::from_ray(state.interaction_revision, 12, WorldObjectPickPurpose::ReferenceSelect, Vec3::new(-2.0, 0.0, 2.0), Vec3::new(0.0, 0.0, -1.0));
    select.merge = world_merge_code(false, false, false);
    for _ in 0..=WORLD_INTERACTION_OBJECT_CAPACITY {
        if with_world_step_context(1, |context| select.step(&state, 12, context)) == WorldInteractionStep::Complete {
            break;
        }
    }
    let mut plan = select.finish_plan(&state, 12).expect("reference select plan").expect("reference hit");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 12, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    let targets = actions[0].args.as_ref().and_then(|args| args.get("targets")).and_then(|value| value.as_str()).and_then(|raw| serde_json::from_str::<Vec<serde_json::Value>>(raw).ok()).expect("reference target JSON");
    assert_eq!(targets[0]["id"], format!("{}/{}", state.surface_id, identity[0]["expectedInteractionId"].as_str().unwrap()));
    assert_eq!(targets[0]["granularity"], "reference");
}


#[test]
fn reference_visual_geometry_reaches_the_actual_textured_scene_pass() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖼️reference-visual/🔣️.json")).unwrap();
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
    let _ = state.reference_pixels.insert(url.into(), test_world_scene_raster(size[0].as_u64().unwrap() as u32, size[1].as_u64().unwrap() as u32, 0));
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
    println!("[DEBUG] actual reference scene pass retained the texture identity and Three-derived plane corners");
}

/// 🌐️ LAW: the actual World producer publishes one retained procedural grid scalar and no
/// finite grid line vertices. The later overlay lanes remain ordinary line geometry.
#[test]
fn an_actual_world_grid_is_one_camera_projected_procedural_scalar_without_grid_lines() {
    let scene = scene_with_selection("{}");
    let bounds = Rect::new(0.0, 0.0, 320.0, 240.0);
    let mut state = World3dState::new("surface-grid".into(), "controller-grid".into());
    state.orbit.target = Vec3::new(7.0, 11.0, 0.0);
    state.orbit.distance = 59.0;
    state.orbit.yaw = -0.63;
    state.orbit.pitch = 0.71;
    state.lod.show_grid = true;
    state.lod.grid_datum = Some([3.0, 5.0, 2.0]);
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

    let expected_camera = state.orbit.to_camera();
    let expected_step = lod_grid_step_world(scene_lod(&state), state.lod.grid_factor).expect("positive retained LOD step");
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

/// 🎨️ LAW: the retained Basic-material appearance follows authored opacity and live theme tokens.
#[test]
fn reference_visual_state_uses_authored_opacity_and_live_theme() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🖼️reference-visual/🔣️.json")).expect("reference visual fixture");
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
            state.selected_ids.push(id.into());
        }
        if row["hovered"].as_bool().unwrap() {
            state.local_hover_id = Some(format!("reference:{id}"));
        }
        let actual = world_reference_visual(&state, &reference, &theme);
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
