
use super::*;
use ui_wgpu::wgpu::{SurfaceKind, UiComponentSceneNode, UiPresence, World3dScene, mesh3d_write_edge};

fn take_actions(input: &mut ui_wgpu::wgpu::InputState<ActionDescriptor>) -> Vec<ActionDescriptor> {
    let mut actions = Vec::new();
    while let Some(action) = input.take_action_step().expect("action authority live") {
        actions.push(action.into_descriptor().expect("bounded action materializes"));
    }
    actions
}

fn triangle_mesh_oracle() -> LegacyMeshOracleData {
    mesh_oracle_from_buffers(vec![-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0], vec![0, 1, 2])
}

fn mesh_oracle_from_buffers(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> LegacyMeshOracleData {
    LegacyMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors: Vec::new() }
}

fn publish_oracle_mesh(data: LegacyMeshOracleData) -> Mesh3dLease {
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
    let token = mesh3d_begin(generation, 0, schema).expect("oracle mesh claim");
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

fn with_world_step_context<T>(fuel: u64, step: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> T) -> T {
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
    const HOST_TOKEN_FIELD: &str = "cursor_wake_requested: Option<crate::infinite_world::world::WorldCursorWakeToken>";

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
            ("AppFrameAfterChrome", "struct AppFrameAfterChrome {", "struct FrameWheelCursor {"),
            ("AppFramePresentation", "pub(crate) struct AppFramePresentation {", "impl AppFrameBuild {"),
            ("AppFramePreparation", "pub(crate) struct AppFramePreparation {", "impl AppFramePreparation {"),
            ("AppPresentStep::Complete", "pub(crate) enum AppPresentStep {", "impl AppPresenter {"),
        ];
        let host_handoffs = [("OsHost", "pub struct OsHost {", "pub(crate) struct OsHostRetirement {"), ("OsHostRetirement", "pub(crate) struct OsHostRetirement {", "impl OsHost {")];
        glue.contains("world_cursor_wake: infinite_world::world::WorldCursorWakeAuthority")
            && glue.contains("World3dBuildContext::new(runtime.world_cursor_wake_authority())")
            && glue.matches(TOKEN_FIELD).count() == typed_handoffs.len()
            && typed_handoffs.iter().all(|(_, start, end)| region(glue, start, end).is_some_and(|handoff| handoff.matches(TOKEN_FIELD).count() == 1))
            && region(glue, "pub(crate) enum AppPresentStep {", "impl AppPresenter {")
                .is_some_and(|handoff| handoff.contains("Complete { fullscreen: Option<bool>, cursor_wake: Option<infinite_world::world::WorldCursorWakeToken> }") && handoff.matches(TOKEN_FIELD).count() == 1)
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
        ("AppFrameAfterChrome", "struct AppFrameAfterChrome {", "struct FrameWheelCursor {"),
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
    for (name, start, end) in [("OsHost", "pub struct OsHost {", "pub(crate) struct OsHostRetirement {"), ("OsHostRetirement", "pub(crate) struct OsHostRetirement {", "impl OsHost {")] {
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
fn terrain_writer_matches_legacy_bands_and_closes_interrupted_authority() {
    fn payload() -> TerrainTileMeshPayload {
        TerrainTileMeshPayload {
            positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 3.0, 0.0, 0.0, 2.0, 1.0, 0.0],
            normals: [0.0, 0.0, 1.0].repeat(6),
            indices: vec![0, 1, 2, 3, 4, 5],
            uvs: vec![0.0, 0.05, 0.0, 0.05, 0.0, 0.05, 0.0, 0.95, 0.0, 0.95, 0.0, 0.95],
        }
    }

    let mut cursor = WorldTerrainMeshCursor::new("surface", 3, 4, 5, payload(), 50, 9, 9).expect("terrain cursor");
    let mut turns = 0;
    loop {
        turns += 1;
        assert!(turns < 512);
        match cursor.step(9, 9) {
            WorldTerrainMeshStep::Pending => {}
            WorldTerrainMeshStep::Ready(key, lease) => {
                let band = key.rsplit(':').next().and_then(|value| value.parse::<usize>().ok()).expect("band key");
                let legacy = build_terrain_band_mesh(&payload(), band, TERRAIN_COLOR_BANDS).expect("published band has legacy geometry");
                let schema = lease.schema().expect("terrain lease schema");
                assert_eq!(schema.vertices as usize * 3, legacy.positions.len());
                assert_eq!(schema.indices as usize, legacy.indices.len());
                for item in 0..schema.vertices {
                    assert_eq!(lease.vec3(Mesh3dField::Positions, item).unwrap(), legacy.positions[item as usize * 3..item as usize * 3 + 3]);
                    assert_eq!(lease.vec3(Mesh3dField::Normals, item).unwrap(), legacy.normals[item as usize * 3..item as usize * 3 + 3]);
                }
                for item in 0..schema.indices {
                    assert_eq!(lease.u32(Mesh3dField::Indices, item).unwrap(), legacy.indices[item as usize]);
                }
                mesh3d_begin_close(lease).unwrap();
                while !mesh3d_close_step(lease).unwrap() {}
            }
            WorldTerrainMeshStep::Complete(tile) => {
                assert_eq!(tile, (3, 4, 5));
                break;
            }
            WorldTerrainMeshStep::Fault => panic!("valid terrain cursor faulted"),
        }
    }
    assert!(cursor.terminal_is_empty());

    let mut interrupted = WorldTerrainMeshCursor::new("surface", 3, 4, 5, payload(), 70, 11, 11).expect("terrain cursor");
    assert!(matches!(interrupted.step(11, 11), WorldTerrainMeshStep::Pending));
    let mut close_turns = 0;
    while !interrupted.close_step() || !interrupted.terminal_is_empty() {
        close_turns += 1;
        assert!(close_turns < 64);
    }
    assert!(interrupted.terminal_is_empty());
    assert!(WorldTerrainMeshCursor::new("surface", 0, 0, 0, TerrainTileMeshPayload { positions: vec![0.0, 1.0], normals: Vec::new(), indices: vec![0, 1, 2], uvs: Vec::new() }, 80, 1, 1).is_err());
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
    state.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }] }).unwrap();

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
    state.draws.push(SceneDraw3d { mesh_key: "source".into(), mesh_version: version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }] }).unwrap();
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { id: "instance".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }] });
    state.vortices.push(WorldVortexRecord { full_id: "vortex".into(), position: Some([1.0, 2.0, 3.0]), radius: Some(0.5), ..Default::default() });
    state.references.push(WorldReferenceRecord { url: Some("reference".into()), origin: Some([4.0, 5.0, 6.0]), width_world: Some(2.0), hidden: Some(false) });
    state.reference_pixels.insert("reference".into(), (2, 1, Vec::new()));

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

#[test]
fn world_context_menu_cursor_is_revisioned_and_right_drag_suppresses_publication() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.interaction_revision = 4;
    state.interaction_objects.revision = 4;
    state.hovered_vortex_id = Some("vortex".into());
    let token = state.interaction_objects.admit(4, WorldInteractionObjectKind::Vortex, "vortex", None, Mat4::identity(), [0.0; 8]).expect("vortex token");
    let mut cursor = WorldContextMenuCursor::new(&state, 11, 12.0, 13.0).expect("context cursor");
    cursor.slot = token.slot;
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 11, context)), WorldInteractionStep::Pending);
    assert_eq!(cursor.target, Some(token));
    assert_eq!(with_world_step_context(1, |context| cursor.step(&state, 11, context)), WorldInteractionStep::Complete);
    let mut plan = cursor.finish_plan(&state, 11).expect("context plan").expect("context target");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 11, &mut input, context)).unwrap(), WorldInteractionStep::Pending);
    assert_eq!(take_actions(&mut input).len(), 1);

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
fn world_marquee_pages_build_one_target_field_per_grant_and_publish_atomically_fifo() {
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
    assert!(matches!(actions[0].args.as_ref().and_then(|args| args.get("targets")), Some(dsl::DslValue::Array(values)) if values.len() == WORLD_MARQUEE_RESULT_PAGE_CAPACITY));
    assert!(matches!(actions[1].args.as_ref().and_then(|args| args.get("targets")), Some(dsl::DslValue::Array(values)) if values.len() == 1));
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

fn world_marquee_geometry_fixture(instance_count: usize) -> World3dState {
    let mut state = World3dState::new("surface".into(), "controller".into());
    state.bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.pick_bounds = state.bounds;
    state.interaction_revision = 2;
    let mesh = publish_oracle_mesh(triangle_mesh_oracle());
    store_mesh(&mut state, "mesh".into(), mesh);
    let mesh_version = *state.mesh_versions.get("mesh").expect("mesh version");
    let instances = (0..instance_count).map(|index| Instance3d { id: format!("object-{index:03}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }).collect();
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances });
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
    let view_projection = state.orbit.to_camera().view_proj(1.0);
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
    let view_projection = state.orbit.to_camera().view_proj(1.0);
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
    let view_projection = state.orbit.to_camera().view_proj(1.0);
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
    let mut state = world_marquee_geometry_fixture(1);
    let viewport = render_pick_viewport(&state);
    let view_projection = state.orbit.to_camera().view_proj(1.0);
    let points = [[0.0, 0.0], [400.0, 400.0]];
    for granularity in ["vertex", "edge", "face"] {
        state.granularity = granularity.into();
        let (meshes, draws) = legacy_geometry_fixture(&state);
        let mut legacy: Vec<u32> = screen_select_components(&meshes, &draws, view_projection, viewport.w, viewport.h, &points, true, granularity, None, false).into_iter().map(|id| id.parse().expect("numeric component id")).collect();
        legacy.sort_unstable();
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

fn world_gumball_commit_fixture() -> (World3dState, WorldGumballGesture) {
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
    let (state, gesture) = world_gumball_commit_fixture();
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
    assert!(turns > 8);
}

#[test]
fn world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority() {
    let (mut state, gesture) = world_gumball_commit_fixture();
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
    for _ in 0..4 {
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
    let (mut state, gesture) = world_gumball_commit_fixture();
    state.interaction_authority.as_mut().unwrap().gumball = Some(gesture);
    let source = Mat4::identity();
    let preview = retained_gumball_preview_model(&state, 0, 0, source);
    assert_eq!(source.cols[3], [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(preview.cols[3], [2.0, 0.0, 0.0, 1.0]);
    let unmatched = retained_gumball_preview_model(&state, 0, 1, source);
    assert_eq!(unmatched.cols, source.cols);
}

fn world_brush_commit_fixture(target: String) -> World3dState {
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
    let state = world_brush_commit_fixture("v".repeat(WORLD_BRUSH_COPY_CHUNK_BYTES * 2 + 1));
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
    let mut state = world_brush_commit_fixture("first".into());
    let mut stale = WorldBrushCommitJob::new(&state, 9).unwrap().expect("brush job");
    while !stale.validating {
        assert_eq!(with_world_step_context(1, |context| stale.step(&state, 9, &mut ui_wgpu::wgpu::InputState::default(), context)).unwrap(), WorldInteractionStep::Pending);
    }
    state.brush_preview.as_mut().unwrap().target_vortex_full_id = Some("other".into());
    assert!(matches!(with_world_step_context(1, |context| stale.step(&state, 9, &mut ui_wgpu::wgpu::InputState::default(), context)), Err(ui_wgpu::wgpu::BoundedActionFault::Structure)));

    let state = world_brush_commit_fixture("target".into());
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

    let mut plan = plan_world3d_wheel(&state, 8, 20.0).expect("bounded wheel plan");
    let pending = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 8, &mut input, context)).unwrap();
    assert_eq!(pending, WorldInteractionStep::Pending);
    assert_ne!(state.orbit.distance, original_distance);
    let actions = take_actions(&mut input);
    assert_eq!(actions.len(), 1);
    assert_eq!(actions[0].action, "setCamera");
    let complete = with_world_step_context(1, |context| publish_world3d_plan_step(&mut state, &mut plan, 8, &mut input, context)).unwrap();
    assert_eq!(complete, WorldInteractionStep::Complete);
    assert!(plan.terminal_is_empty());
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
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["setCamera"]);

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
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }] });
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

#[test]
fn world_authority_retains_front_plan_across_output_saturation_and_retries_in_order() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let original_distance = state.orbit.distance;
    enqueue_world3d_intent(&mut state, world_intent(1, 10.0)).expect("wheel intent");
    enqueue_world3d_intent(&mut state, world_intent(2, 20.0)).expect("second wheel intent");
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Pending);
    let mut claims = Vec::new();
    while let Ok(claim) = input.claim_action(1) {
        claims.push(claim);
    }
    assert!(!claims.is_empty());
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::OutputBlocked);
    assert_eq!(state.orbit.distance, original_distance);
    let released = claims.pop().expect("one retry credit");
    input.release_action_claim(released).expect("release retry credit");
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Pending);
    assert_ne!(state.orbit.distance, original_distance);
    for claim in claims {
        input.release_action_claim(claim).expect("release retained credit");
    }
    assert_eq!(take_actions(&mut input).into_iter().map(|action| action.action).collect::<Vec<_>>(), vec!["setCamera"]);
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context)), WorldInteractionAuthorityStep::Complete);
    assert_eq!(state.interaction_authority.as_ref().and_then(|authority| authority.queue.front()).map(|intent| intent.generation), Some(2));
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
    for generation in 1..=WORLD_INTERACTION_INTENT_CAPACITY as u64 + 1 {
        enqueue_world3d_intent(&mut state, world_intent(generation, generation as f32)).expect("queue or retained saturation owner");
    }
    let rejected_generation = WORLD_INTERACTION_INTENT_CAPACITY as u64 + 2;
    assert_eq!(enqueue_world3d_intent(&mut state, world_intent(rejected_generation, rejected_generation as f32)).expect_err("blocked owner seals ingress").generation, rejected_generation);
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    for _ in 0..3 {
        let _ = with_world_step_context(1, |context| step_world3d_interaction(&mut state, 1, &mut input, context));
    }
    assert_eq!(take_actions(&mut input).len(), 1);
    assert_eq!(with_world_step_context(1, |context| step_world3d_interaction(&mut state, 2, &mut input, context)), WorldInteractionAuthorityStep::Pending);
    let authority = state.interaction_authority.as_mut().expect("authority");
    assert!(authority.blocked.is_none());
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

#[test]
fn prepared_world_resources_are_send_and_deduplicate_uploads() {
    assert_send::<World3dBuildContext>();
    let mut resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let mesh = publish_oracle_mesh(triangle_mesh_oracle());
    resources.ensure_mesh("mesh", 3, mesh);
    resources.ensure_mesh("mesh", 3, mesh);
    resources.ensure_world_plane_texture("image", &[1, 2, 3, 4], 1, 1);
    resources.ensure_world_plane_texture("image", &[9, 9, 9, 9], 1, 1);
    resources.evict_mesh("stale");
    let mut input = ui_wgpu::wgpu::PreparedRenderInput::try_new(1, 2, ui_wgpu::wgpu::DrawList::default(), None, 0.0).ok().expect("prepared input admitted");
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert_eq!(resources.append_step(&mut input).ok(), Some(true));
    assert_eq!(input.uploads.len(), 2);
    assert_eq!(input.evictions.len(), 1);
    assert_eq!(input.evictions.get(0), Some(&PreparedRenderEviction::Mesh { key: "stale".into() }));
}

#[test]
fn world_orbit_view_gizmo_placement_matches_react_bottom_right_insets() {
    assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 1280.0, h: 720.0 }), (32.0, 32.0));
    assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 120.0, h: 160.0 }), (32.0, 32.0));
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
            selection_json: selection_json.into(),
            vortices_json: None,
            attractions_json: None,
            target_volumes_json: None,
            references_json: None,
            brush_preview_json: None,
            interaction_json: None,
            engagement_preview_json: None,
            lod_json: None,
            chunking_json: None,
            environment_json: None,
            frame_json: None,
            fit_json: None,
            terrain_json: None,
            points_json: None,
            status_json: None,
            domain_id: domain.map(|(id, _)| id.to_string()),
            domain_granularity_id: domain.map(|(_, granularity)| granularity.to_string()),
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let instances = append_component_vertex_spheres(&mut state);
    assert_eq!(instances.len(), 0);

    let mut lines = Vec::new();
    append_component_overlays(&state, &mut lines);
    assert_eq!(lines.len(), 28); // 4 from edges + 24 from 4 vertex crosses
}

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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let mut lines = Vec::new();
    append_component_overlays(&state, &mut lines);
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let mut lines = Vec::new();
    append_component_overlays(&state, &mut lines);
    assert!(lines.len() >= 2);
    assert!(lines.iter().any(|vertex| vertex.color[2] > 0.9));
}

#[test]
fn component_mode_does_not_apply_mesh_instance_hover() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "face".into();
    state.hovered_component_object_id = Some("obj-1".into());
    state.local_hover_id = None;
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    apply_runtime_draw_flags(&mut state);
    assert!(!state.draws[0].instances[0].hovered);
}

#[test]
fn runtime_draw_flags_clear_stale_instance_selected_when_selection_is_empty() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.selected_ids.clear();
    state.local_hover_id = None;
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: true, hovered: true }] });
    apply_runtime_draw_flags(&mut state);
    assert!(!state.draws[0].instances[0].selected, "empty selection must clear a stale instancesJson selected bit");
    assert!(!state.draws[0].instances[0].hovered, "empty hover must clear a stale instancesJson hovered bit");
}

#[test]
fn merge_u32_ids_supports_add_and_toggle() {
    assert_eq!(merge_u32_ids(&["1".into()], &["2".into()], "add"), vec![1, 2]);
    assert_eq!(merge_u32_ids(&["1".into(), "2".into()], &["2".into(), "3".into()], "toggle"), vec![1, 3]);
}

#[test]
fn pick_select_emits_numeric_world_pick_id() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "vertex".into();
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("vertex projects");
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let view_proj = camera.view_proj(1.0);
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let bounds = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = bounds;
    state.pick_bounds = bounds;
    state.marquee_points = vec![[390.0, 10.0], [10.0, 390.0]];
    update_marquee_preview(&mut state, bounds);
    assert!(state.marquee_preview_ids.iter().any(|id| id == "10" || id == "11"), "preview ids: {:?}", state.marquee_preview_ids);
    let mut lines = Vec::new();
    append_component_overlays(&state, &mut lines);
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
    page.push_item(World3dSnapshotItem { numbers: [4.0, 4.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 45.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], number_len: 10, ..Default::default() }).unwrap();
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
    let expected = OrbitController::from_camera(&Camera3d { position: vector("position"), target: vector("target"), up: vector("up"), fov_y: oracle["fov"].as_f64().unwrap() as f32 * std::f32::consts::PI / 180.0, near: 0.1, far: 1000.0 }).to_camera();
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

#[test]
fn dynamic_world_owners_retire_one_nested_owner_per_grant_to_terminal_empty() {
    let mut state = World3dState::new("surface".into(), "controller".into());
    let mesh = publish_oracle_mesh(mesh_oracle_from_buffers(vec![0.0; 3 * 128], vec![0.0; 3 * 128], vec![0; 3 * 64]));
    state.meshes.insert("mesh".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances: (0..32).map(|index| Instance3d { id: format!("instance-{index}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }).collect() });
    state.reference_pixels.insert("reference".into(), (64, 64, vec![0; 64 * 64 * 4]));
    state.mesh_paint_textures.insert("paint".into(), (64, 64, vec![0; 64 * 64 * 4]));
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
    let first = WorldOpaqueOwner::ReferencePixels(WorldDynamicEntry { id: "first".into(), epoch: 1, value: (1, 1, vec![1]) });
    assert!(quarantine.admit(first).is_ok());
    let second = WorldOpaqueOwner::PaintPixels(WorldDynamicEntry { id: "second".into(), epoch: 2, value: (1, 1, vec![2]) });
    let rejected = quarantine.admit(second).expect_err("quarantine returns saturated owner");
    let WorldOpaqueOwner::PaintPixels(rejected) = rejected else { panic!("exact paint owner") };
    assert_eq!((rejected.id.as_str(), rejected.epoch, rejected.value.2.as_slice()), ("second", 2, &[2][..]));
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
    let instances = (0..=WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY).map(|index| Instance3d { id: format!("instance-{index}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }).collect();
    let rejected = draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances }).expect_err("draw instance capacity owner");
    assert_eq!(rejected.fault, WorldDynamicFault::InstanceCapacity);
    assert_eq!(rejected.value.instances.len(), WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY + 1);
    assert!(draws.is_empty());
}

fn draw_fixture_instance(id: &str) -> Instance3d {
    Instance3d { id: id.into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let bounds = Rect { x: 0.0, y: 50.0, w: 400.0, h: 400.0 };
    let clip = Rect { x: 0.0, y: 100.0, w: 400.0, h: 400.0 };
    state.bounds = bounds;
    state.pick_bounds = clip;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, bounds.w, bounds.h).expect("vertex projects");
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let mut lines = Vec::new();
    append_component_overlays(&state, &mut lines);
    assert!(lines.len() >= 6, "hovered face should emit triangle edge lines, got {}", lines.len());
}

#[test]
fn pick_component_at_face_mode_uses_ray_pick() {
    let mesh = topology_mesh();
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.granularity = "face".into();
    state.active_object_id = Some("obj-1".into());
    state.meshes.insert("mesh-1".into(), mesh);
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let mesh_ref = state.meshes.get("mesh-1").expect("mesh");
    let tri = world_mesh_triangle(*mesh_ref, 0).expect("triangle");
    let centroid = mesh_vertex(*mesh_ref, tri[0]).expect("first vertex").add(mesh_vertex(*mesh_ref, tri[1]).expect("second vertex")).add(mesh_vertex(*mesh_ref, tri[2]).expect("third vertex")).scale(1.0 / 3.0);
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), centroid, inner.w, inner.h).expect("face centroid projects");
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let edge = state.meshes.get("mesh-1").expect("mesh").edge(0).expect("edge");
    let a = Vec3::new(edge[0][0], edge[0][1], edge[0][2]);
    let b = Vec3::new(edge[1][0], edge[1][1], edge[1][2]);
    let mid = a.add(b).scale(0.5);
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), mid, inner.w, inner.h).expect("edge midpoint projects");
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
    assert!(state.mesh_pool.release("mesh-1".into()));
    assert!(!state.mesh_pool.contains(&"mesh-1".to_string()));
    state.mesh_pool.acquire("mesh-1".into());
    state.mesh_pool.acquire("mesh-1".into());
    assert!(!state.mesh_pool.release("mesh-1".into()));
    assert!(state.mesh_pool.contains(&"mesh-1".to_string()));
    assert!(state.mesh_pool.release("mesh-1".into()));
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️draws.json")).unwrap();
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

//#endregion GlbAssetTests

#[test]
fn lod_grid_lines_generate_for_near_camera() {
    let mut lines = Vec::new();
    append_lod_grid_lines(&mut lines, 2.0, 10.0, Vec3::ZERO, [0.5, 0.5, 0.5, 1.0]);
    assert!(!lines.is_empty());
}

//#region EnvironmentTests
#[test]
fn environment_clear_color_uses_opaque_background() {
    let environment = WorldEnvironmentRecord { background: Some("#112233".into()), ..Default::default() };
    let theme_clear = Rgba::new(0.0, 0.0, 0.0, 1.0);
    let clear = environment_clear_color(&environment, theme_clear);
    assert!((clear.r - (0x11 as f32 / 255.0)).abs() < 1e-3);
    assert!((clear.g - (0x22 as f32 / 255.0)).abs() < 1e-3);
    assert!((clear.b - (0x33 as f32 / 255.0)).abs() < 1e-3);
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
fn retained_draw_rebuild_preserves_prepared_material_colors_from_the_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🔣️draws.json")).unwrap();
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
fn build_terrain_band_mesh_buckets_by_average_elevation() {
    // Two triangles, 6 verts (no sharing, to keep each triangle's average elevation exact):
    // triangle 0 is flat at elevation ratio 0.0, triangle 1 is flat at elevation ratio 1.0.
    let mesh = TerrainTileMeshPayload {
        positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 10.0, 3.0, 0.0, 10.0, 2.0, 1.0, 10.0],
        normals: vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        indices: vec![0, 1, 2, 3, 4, 5],
        uvs: vec![0.5, 0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 1.0, 0.5, 1.0, 0.5, 1.0],
    };
    let low_band = build_terrain_band_mesh(&mesh, 0, TERRAIN_COLOR_BANDS);
    assert!(low_band.is_some(), "triangle 0 (all-zero elevation) should fall in band 0");
    let high_band = build_terrain_band_mesh(&mesh, TERRAIN_COLOR_BANDS - 1, TERRAIN_COLOR_BANDS);
    assert!(high_band.is_some(), "triangle 1 (all-one elevation) should fall in the top band");
    let empty_band = build_terrain_band_mesh(&mesh, 5, TERRAIN_COLOR_BANDS);
    assert!(empty_band.is_none(), "no triangle should land in a middle band for this fixture");
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
    let camera = Camera3d { position: Vec3::new(0.0, 0.0, 300.0), target: Vec3::ZERO, up: Vec3::new(0.0, 0.0, 1.0), fov_y: 45.0_f32.to_radians(), near: 0.1, far: 1000.0 };
    let (band_draws, evicted) = sync_terrain_state(&mut state, &camera);
    assert!(band_draws.is_empty(), "no elevation data uploaded yet, nothing to draw");
    assert!(evicted.is_empty(), "nothing was cached yet, nothing to evict");
    assert!(!state.pending_terrain_tile_urls.is_empty(), "an uncached visible tile should be queued for byte-fetch");

    let (_, &(z, x, y)) = state.pending_terrain_tile_urls.iter().next().expect("a pending tile");
    let value = (100.0_f64 + 32768.0).round() as i64;
    let r = ((value >> 8) & 0xff) as u8;
    let g = (value - ((r as i64) << 8)).clamp(0, 255) as u8;
    let mut image = image::RgbaImage::new(256, 256);
    for pixel in image.pixels_mut() {
        *pixel = image::Rgba([r, g, 0, 255]);
    }
    let mut bytes = Vec::new();
    image::DynamicImage::ImageRgba8(image).write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png).expect("encode png");
    assert!(state.terrain_session.upload_elevation_tile(z, x, y, &bytes));

    let (band_draws_after_upload, _) = sync_terrain_state(&mut state, &camera);
    assert!(!band_draws_after_upload.is_empty(), "an uploaded tile should produce at least one banded draw");
}

#[test]
fn apply_terrain_style_if_changed_state_purges_stale_meshes_on_origin_change() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    state.terrain_style = Some(WorldTerrainStyle { tile_url_template: "/dem/{z}/{x}/{y}.png".into(), project_origin_lon: 0.0, project_origin_lat: 0.0, exaggeration: 1.0, color_ramp: "hypsometric".into(), min_zoom: 6, max_zoom: 14 });
    assert!(apply_terrain_style_if_changed_state(&mut state).is_empty(), "first application has nothing to purge");
    let mesh_key = terrain_band_mesh_key(&state.surface_id, 10, 1, 2, 0);
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

#[test]
fn right_click_dispatches_context_menu_at_for_hovered_vortex() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    state.hovered_vortex_id = Some("vortex-1".into());
    handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
    let action = handle_world3d_pointer_button(&mut state, 200.0, 200.0, false, 2, &PointerModifiers::default()).expect("right click should dispatch");
    assert_eq!(action.action, "contextMenuAt");
    let args = action.args.expect("args");
    assert_eq!(args["kind"], json!("vortex"));
    assert_eq!(args["id"], json!("vortex-1"));
}

#[test]
fn right_drag_does_not_dispatch_context_menu_even_with_a_hovered_vortex() {
    let mut state = World3dState::new("surface-1".into(), "controller-1".into());
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    state.hovered_vortex_id = Some("vortex-1".into());
    handle_world3d_pointer_button(&mut state, 200.0, 200.0, true, 2, &PointerModifiers::default());
    let action = handle_world3d_pointer_button(&mut state, 260.0, 260.0, false, 2, &PointerModifiers::default()).expect("right release should still sync camera");
    assert_eq!(action.action, "setCamera", "a right-drag should fall back to the orbit camera sync, not open a context menu");
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("object projects");
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("object projects");
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("object projects");
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
    let inner = Rect { x: 0.0, y: 0.0, w: 400.0, h: 400.0 };
    state.bounds = inner;
    state.pick_bounds = inner;
    let camera = state.orbit.to_camera();
    let screen = ui_wgpu::wgpu::project_point(camera.view_proj(1.0), Vec3::ZERO, inner.w, inner.h).expect("object projects");
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
    state.draws.push(SceneDraw3d { mesh_key: "mesh-1".into(), mesh_version: 0, instances: vec![Instance3d { id: "obj-1".into(), model: Mat4::identity(), color: [1.0, 1.0, 1.0, 1.0], selected: false, hovered: false }] });
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
