use semio_framework_pack_json::Value;

/// 🎲️ Projects the Puzzle document onto the engine's closed `BoardSnapshot` wire schema.
pub fn puzzle2d_board_snapshot_json(snapshot: &Value) -> String {
    let mut board = snapshot.clone();
    if let Some(nodes) = board.get_mut("nodes").and_then(Value::as_array_mut) {
        for node in nodes.iter_mut().filter_map(Value::as_object_mut) {
            node.remove("anchor");
        }
    }
    if let Some(edges) = board.get_mut("edges").and_then(Value::as_array_mut) {
        for edge in edges.iter_mut().filter_map(Value::as_object_mut) {
            for field in ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y"] {
                edge.remove(field);
            }
        }
    }
    board.to_string()
}
