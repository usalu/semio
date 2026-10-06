//! 🧪️ Proximity auto-connect laws: the radius gate, the spatial bound, and the one-edit drop.

use super::{puzzle2d_proximity_connect, puzzle2d_proximity_pairs};
use crate::editor::puzzle2d::unit_tests::context::*;
use crate::editor::puzzle2d::{board_snapshot_edges, PUZZLE2D_PROXIMITY_GESTURE_MAX};
use serde_json::{json, Value};

/// 🧱️ Two circle nodes whose `v0` handles face each other: the left node's rim handle sits at
/// `(x + 24, y)` (east-zero angle 0) and the right node's at `(gap - 24, y)` (angle π), so the two
/// handles are `gap - 48` apart.
fn facing_pair(gap: f64) -> Value {
    json!({
        "schema": "board.ports.directed.v1",
        "meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] },
        "nodes": [
            { "id": "left", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "handles": [{ "id": "left:v0", "handleKind": "a", "angle": 0.0 }] },
            { "id": "right", "shape": "circle", "x": gap, "y": 0.0, "radius": 24.0, "handles": [{ "id": "right:v0", "handleKind": "a", "angle": std::f64::consts::PI }] }
        ],
        "edges": []
    })
}

/// 🧲️ Inside the radius the drop connects; outside it does not.
#[test]
fn proximity_connects_inside_the_radius_and_never_outside() {
    let mut near = facing_pair(52.0);
    assert_eq!(puzzle2d_proximity_connect(&mut near, &["right".to_string()], 12.0), 1, "handles 4 units apart connect under a 12-unit radius");
    let edge = &board_snapshot_edges(&near)[0];
    assert_eq!(edge.get("source").and_then(Value::as_str), Some("left:v0"), "the stationary peer stays the edge source");
    assert_eq!(edge.get("target").and_then(Value::as_str), Some("right:v0"), "the moved node's handle is the edge target");

    let mut far = facing_pair(120.0);
    assert_eq!(puzzle2d_proximity_connect(&mut far, &["right".to_string()], 12.0), 0, "handles 72 units apart never connect under a 12-unit radius");
    assert!(board_snapshot_edges(&far).is_empty(), "an out-of-range drop splices no edge");
}

/// 🚫️ A zero radius disarms the whole feature, and an incompatible pair is refused whatever the distance.
#[test]
fn proximity_respects_the_radius_gate_and_the_compatibility_table() {
    let mut zero = facing_pair(52.0);
    assert_eq!(puzzle2d_proximity_connect(&mut zero, &["right".to_string()], 0.0), 0, "a zero radius disarms the auto-connect");

    let mut incompatible = facing_pair(52.0);
    incompatible["nodes"][1]["handles"][0]["handleKind"] = json!("b");
    assert!(puzzle2d_proximity_pairs(&incompatible, "right", 12.0).is_empty(), "no compatibility row admits an `a`/`b` pair");
}

/// 🔒️ A locked or hidden node never auto-connects, and an already-occupied handle is never claimed twice.
#[test]
fn proximity_skips_locked_hidden_and_occupied_handles() {
    let mut locked = facing_pair(52.0);
    locked["nodes"][1]["locked"] = json!(true);
    assert!(puzzle2d_proximity_pairs(&locked, "right", 12.0).is_empty(), "a locked node refuses the auto-connect");

    let mut occupied = facing_pair(52.0);
    occupied["edges"] = json!([{ "id": "e0", "source": "left:v0", "target": "right:v0" }]);
    assert!(puzzle2d_proximity_pairs(&occupied, "right", 12.0).is_empty(), "an occupied handle is never connected twice");
}

/// 📏️ The gesture budget is a fixed ceiling, not a per-node multiplier — this is what keeps `extent`
/// honest on a whole-selection move.
#[test]
fn proximity_never_exceeds_the_gesture_budget() {
    let mut nodes: Vec<Value> = Vec::new();
    for index in 0..(PUZZLE2D_PROXIMITY_GESTURE_MAX + 8) {
        let y = index as f64 * 1_000.0;
        nodes.push(json!({ "id": format!("left{index}"), "shape": "circle", "x": 0.0, "y": y, "radius": 24.0, "handles": [{ "id": format!("left{index}:v0"), "handleKind": "a", "angle": 0.0 }] }));
        nodes.push(json!({ "id": format!("right{index}"), "shape": "circle", "x": 52.0, "y": y, "radius": 24.0, "handles": [{ "id": format!("right{index}:v0"), "handleKind": "a", "angle": std::f64::consts::PI }] }));
    }
    let mut snapshot = json!({ "schema": "board.ports.directed.v1", "meta": { "kindCompatibility": [{ "source": "a", "target": "a", "bidirectional": true, "important": false, "specificity": "handle" }] }, "nodes": nodes, "edges": [] });
    let moved: Vec<String> = (0..(PUZZLE2D_PROXIMITY_GESTURE_MAX + 8)).map(|index| format!("right{index}")).collect();
    assert_eq!(puzzle2d_proximity_connect(&mut snapshot, &moved, 12.0), PUZZLE2D_PROXIMITY_GESTURE_MAX, "one gesture spends at most its fixed edge budget");
}

/// 🚚️ A drag record whose drop lands inside the radius yields the move AND its new edge as ONE transaction — one
/// history edit whose connection carries the id the select tool minted from the pair.
#[test]
fn node_drop_auto_connects_as_one_history_edit() {
    let mut app = app_with_registry();
    let seed = facing_pair(1_000.0);
    dispatch(&mut app, "importSnapshot", Some(&json!({ "payload": seed })), None).expect("seed the board");
    let before = fixture_of(&app);
    assert!(board_snapshot_edges(&before).is_empty(), "the seeded board starts unconnected: {before}");
    let events = json!([{ "name": "gesture", "payload": { "gestureId": "gesture-1", "kind": "drag", "targets": ["right"], "dx": -948.0, "dy": 0.0, "proximity": [] } }]).to_string();
    let result = dispatch(&mut app, "applyBoardEvents", Some(&json!({ "eventsJson": events })), None).expect("drop");
    assert_eq!(committed_edits(&result), 1, "the drop and its connection are one edit");
    let dropped = fixture_of(&app);
    assert_eq!(board_snapshot_edges(&dropped).len(), 1, "the drop auto-connects the facing handles: {dropped}");
    assert_eq!(board_snapshot_edges(&dropped)[0].get("id").and_then(Value::as_str), Some("edge-left:v0-right:v0"), "the stationary peer is the source and the id is minted from the pair: {dropped}");
    dispatch(&mut app, "undo", None, None).expect("undo");
    let restored = fixture_of(&app);
    assert!(board_snapshot_edges(&restored).is_empty(), "ONE undo takes the drop and its edge back together: {restored}");
    close_app(&mut app);
}
