//! 🧰️ Test-only host surface: the step context fixture and the close-driving `Drop` for `World3dState` (feature `testkit`).

use semio_os_infinite_world_bridge::step_world3d_scene_bridge_close;
use crate::World3dState;
use semio_os_infinite_world_model::with_world_step_context;
use semio_os_infinite_world_model::world_mesh_ownership_grant;
use semio_os_infinite_world_scene::TerrainMeshOracleData;
use ui_scene::math::{mesh3d_allocate_step, mesh3d_begin, mesh3d_seal, mesh3d_write_edge, mesh3d_write_u32, mesh3d_write_vec3, Mesh3dField, Mesh3dLease, Mesh3dSchema};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

impl Drop for World3dState {
    fn drop(&mut self) {
        for _ in 0..16_384 {
            if !with_world_step_context(1,|context|step_world3d_scene_bridge_close(&mut self.bridge,context)) { return; }
        }
        assert!(std::thread::panicking(), "test fixture retained an original scene bridge owner after its close ceiling");
    }
}

/// 🌉️ Drains a finished bridge cursor still parked in its close phase, because a new payload stages nothing while one is parked.
pub fn drain_scene_bridge_close(state: &mut World3dState) {
    use semio_os_infinite_world_bridge::{World3dSceneBridgePhase, World3dSceneBridgeStep};
    for turn in 0..4_096 {
        let parked = state.bridge.scene_bridge.as_ref().is_some_and(|cursor| cursor.phase == World3dSceneBridgePhase::Close) || state.bridge.scene_bridge_close.is_some();
        if !parked {
            return;
        }
        assert_ne!(with_world_step_context(64, |context| crate::step_world3d_scene_bridge(state, context)), World3dSceneBridgeStep::Fault, "parked bridge close faulted on turn {turn}");
    }
    panic!("the parked scene bridge closes within its turn ceiling");
}

/// 🌉️ Drives the staged bridge to its sealed lease, then the snapshot apply ladder to `Complete`.
pub fn drive_scene_bridge(state: &mut World3dState, scene: &semio_ui_declaration::ui::UiComponentSceneNode, bounds: semio_ui_theme::geometry::Rect) {
    use semio_os_infinite_world_bridge::{world3d_draw_rebuild_terminal_is_empty, World3dSceneBridgeStep, World3dSnapshotApplyStep};
    use semio_os_infinite_world_interaction::step_world3d_draw_rebuild;
    use semio_os_infinite_world_model::WorldDrawRebuildStep;
    drain_scene_bridge_close(state);
    crate::sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        match with_world_step_context(64, |context| crate::step_world3d_scene_bridge(state, context)) {
            World3dSceneBridgeStep::Complete => break,
            World3dSceneBridgeStep::Pending => {}
            step => panic!("scene bridge stopped at {step:?} on turn {turn} (fault {:?})", state.session.snapshot_fault),
        }
        assert!(turn < 4_095, "scene bridge did not seal within its turn ceiling");
    }
    crate::sync_world3d_state(state, scene, bounds);
    for turn in 0..4_096 {
        assert_ne!(with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context)), WorldDrawRebuildStep::Fault, "draw rebuild faulted on turn {turn}");
        match with_world_step_context(64, |context| crate::step_world3d_snapshot(state, context)) {
            World3dSnapshotApplyStep::Complete | World3dSnapshotApplyStep::Idle => break,
            World3dSnapshotApplyStep::Pending => {}
            step => panic!("snapshot apply stopped at {step:?} on turn {turn} (fault {:?})", state.session.snapshot_fault),
        }
        assert!(turn < 4_095, "snapshot apply did not complete within its turn ceiling");
    }
    for turn in 0..4_096 {
        let draw_step = with_world_step_context(64, |context| step_world3d_draw_rebuild(&mut state.session, &mut state.authority, &mut state.scene, context));
        assert_ne!(draw_step, WorldDrawRebuildStep::Fault, "draw retirement faulted on turn {turn}");
        let bridge_step = with_world_step_context(64, |context| crate::step_world3d_scene_bridge(state, context));
        assert_ne!(bridge_step, World3dSceneBridgeStep::Fault, "bridge retirement faulted on turn {turn}");
        if world3d_draw_rebuild_terminal_is_empty(&state.scene) && state.bridge.scene_bridge_retired.is_none() && state.bridge.scene_bridge_close.is_none() && state.bridge.scene_bridge.is_none() {
            break;
        }
        assert!(turn < 4_095, "scene bridge retirement did not complete within its turn ceiling");
    }
    crate::apply_runtime_draw_flags(&mut state.selection, &mut state.interaction, &mut state.scene);
}

/// 🧊️ Walks one claimed mesh authority to its allocated pages.
pub fn allocate_oracle_mesh(token: ui_scene::math::Mesh3dWriteToken) {
    loop {
        let grant = world_mesh_ownership_grant(0, ui_scene::math::mesh3d_allocate_capacity_byte_demand(token).unwrap(), 0, 1);
        let step = mesh3d_allocate_step(token, grant).unwrap();
        assert!(step.progress.fits(grant));
        if step.complete {
            break;
        }
    }
}

pub fn mesh_oracle_from_buffers(positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32>) -> TerrainMeshOracleData {
    TerrainMeshOracleData { positions, normals, indices, face_ids: Vec::new(), vertex_ids: Vec::new(), edge_positions: Vec::new(), edge_ids: Vec::new(), uvs: Vec::new(), colors: Vec::new() }
}

pub fn publish_oracle_mesh(data: TerrainMeshOracleData) -> Mesh3dLease {
    publish_oracle_mesh_at_revision(data, 0)
}

/// 🧊️ An oracle mesh stamped with a chosen interaction revision — what
/// `publish_world3d_asset_mesh_lease` witnesses before it stores a decoded GLB under its url's id.
pub fn publish_oracle_mesh_at_revision(data: TerrainMeshOracleData, revision: u64) -> Mesh3dLease {
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

