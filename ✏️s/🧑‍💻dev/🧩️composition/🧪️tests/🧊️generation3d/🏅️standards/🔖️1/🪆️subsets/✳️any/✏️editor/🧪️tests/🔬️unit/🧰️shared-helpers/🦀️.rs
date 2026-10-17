/// 🧰️ Test helpers the editor law groups share, kept in the support so every target that owns a group resolves them.

/// 🌉️ Decodes one preview mesh record's `data` field (a `pack::json`/`serde_json::Value` fragment
/// off the wire) into `MeshData` through its first-party `FromValue` codec — `MeshData` no longer
/// derives `serde::Deserialize` in a non-`#[cfg(test)]` build of its own crate (`🏗️mesh-engine/🦀️.rs`'s
/// `#[cfg_attr(test, derive(Serialize, Deserialize))]` only activates inside that crate's OWN test
/// build, never for a downstream consumer like this one), so `serde_json::from_value` cannot reach
/// it here; round-trip through the wire text and `semio_framework_pack_json::from_json_str` instead, exactly like
/// production's `mesh_data_for_preview_handle` (above) does.
fn mesh_data_from_json(value: &Value) -> semio_framework_plugin::MeshData {
    semio_framework_pack_json::from_json_str(&serde_json::to_string(value).expect("mesh data json text"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("mesh data")
}

/// 🌉️ `Mesh3d` (a plain positions/normals/indices struct) no longer exists —
/// `semio_framework_ui::wgpu::kernel_3d_scene`'s mesh API is now a generation/revision-keyed write-token/lease
/// pair (`mesh3d_begin`/`mesh3d_write_vec3`/`mesh3d_seal`) meant for the shared render-owned mesh
/// arena, not for a one-off AABB check. This test only ever needed the bounding box of the raw
/// position buffer, so compute it directly instead of standing up a lease.
fn aabb_of_positions(positions: &[f32]) -> ([f32; 3], [f32; 3]) {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for vertex in positions.chunks_exact(3) {
        for axis in 0..3 {
            min[axis] = min[axis].min(vertex[axis]);
            max[axis] = max[axis].max(vertex[axis]);
        }
    }
    (min, max)
}

fn test_serial() -> crate::test_serial::TestSerialGuard {
    crate::editor_domain::editor_laws::serial_execution::lock()
}

/// 🚚️ The `nodeGraphEdit` payload one released node drag travels as — the node-graph gesture record of design §13.3
/// (the press, the moved node and its ONE relative offset) both hosts write (`🕸️NodeGraph/🟦️.tsx`, wgpu `EngineCanvas`).
pub(crate) fn node_move_operations_json(node_id: &str, dx: f64, dy: f64) -> String {
    serde_json::json!([{ "operation": "move", "gestureId": format!("node-drag:{node_id}"), "nodeIds": [node_id], "dx": dx, "dy": dy }]).to_string()
}

/// 📇️ Every action this app declares: each window kind's own roster plus the app-level roster
/// (`AppDefinition::actions`), which `semio_framework::window_kind_actions` offers to every window
/// that does not claim the id itself. The builder no longer clones app actions into window kinds.
fn declared_actions(definition: &semio_framework_plugin::AppDefinition) -> impl Iterator<Item = &semio_framework_plugin::ActionDefinition> {
    definition.window_kinds.iter().flat_map(|window| window.actions.iter()).chain(definition.actions.iter())
}

/// 🚦️ One widget's `NodeEvalStatus` discriminant as the flow window publishes it.
pub(crate) fn node_eval_status(status_json: &str, widget_id: &str) -> String {
    let status: serde_json::Value = serde_json::from_str(status_json).expect("flow status json");
    status.get(widget_id).and_then(|entry| entry.get("status")).and_then(serde_json::Value::as_str).unwrap_or("<absent>").to_string()
}
