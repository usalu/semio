
use super::*;

fn row(name: &str, payload: Value) -> BoardEventRow {
    BoardEventRow { name: name.to_string(), payload }
}

fn typed_kind(name: &str) -> infinite_canvas::BoardEventKind {
    use infinite_canvas::BoardEventKind;
    match name {
        "camera" => BoardEventKind::Camera,
        "nodeMove" => BoardEventKind::NodeMove,
        "nodeDragEnd" => BoardEventKind::NodeDragEnd,
        "select" => BoardEventKind::Select,
        "preselect" => BoardEventKind::Preselect,
        "preselectCancel" => BoardEventKind::PreselectCancel,
        "brushPreview" => BoardEventKind::BrushPreview,
        "brushCandidates" => BoardEventKind::BrushCandidates,
        "brushPlace" => BoardEventKind::BrushPlace,
        "edgeCreate" => BoardEventKind::EdgeCreate,
        "edgeDelete" => BoardEventKind::EdgeDelete,
        "nodeDelete" => BoardEventKind::NodeDelete,
        _ => panic!("fixture kind {name}"),
    }
}

fn typed_coalesce(rows: &[BoardEventRow]) -> CoalescedBoardEvents {
    let mut queue = infinite_canvas::BoardEventQueue::default();
    for row in rows {
        let payload = serde_json::to_string(&row.payload).unwrap();
        let key = (row.name == "nodeMove").then(|| row.payload.get("id").and_then(Value::as_str)).flatten();
        queue.push(infinite_canvas::BoardOwnedEvent::from_payload(typed_kind(&row.name), &payload, key).unwrap()).unwrap();
    }
    coalesce_owned_board_events(&queue).unwrap()
}

#[test]
fn typed_coalescer_matches_legacy_fifo_and_flush_semantics() {
    let rows = vec![
        row("camera", json!({ "x": 1 })),
        row("nodeMove", json!({ "id": "a", "x": 1 })),
        row("preselect", json!({ "ids": ["a"] })),
        row("nodeMove", json!({ "id": "b", "x": 2 })),
        row("nodeMove", json!({ "id": "a", "x": 3 })),
        row("camera", json!({ "x": 2 })),
        row("select", json!({ "ids": ["a"] })),
    ];
    let legacy = coalesce_board2d_events(&rows);
    let typed = typed_coalesce(&rows);
    assert_eq!(typed.flush_now, legacy.flush_now);
    assert_eq!(serde_json::from_str::<Value>(&typed.events_json).unwrap(), serde_json::from_str::<Value>(&legacy.events_json).unwrap());

    let drag_end = vec![row("nodeMove", json!({ "id": "a", "x": 1 })), row("nodeDragEnd", json!({ "moves": [{ "id": "a", "x": 1 }] }))];
    assert_eq!(serde_json::from_str::<Value>(&typed_coalesce(&drag_end).events_json).unwrap(), serde_json::from_str::<Value>(&coalesce_board2d_events(&drag_end).events_json).unwrap());
}

#[test]
fn coalesce_drops_transient_events() {
    let rows = vec![row("preselect", json!({})), row("brushPreview", json!({})), row("select", json!({ "ids": ["a"] }))];
    let result = coalesce_board2d_events(&rows);
    assert!(result.flush_now);
    let parsed: Vec<Value> = serde_json::from_str(&result.events_json).unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0]["name"], "select");
}

#[test]
fn coalesce_keeps_only_latest_camera() {
    let rows = vec![row("camera", json!({ "x": 1 })), row("camera", json!({ "x": 2 }))];
    let result = coalesce_board2d_events(&rows);
    assert!(!result.flush_now);
    let parsed: Vec<Value> = serde_json::from_str(&result.events_json).unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0]["payload"]["x"], 2);
}

#[test]
fn coalesce_collapses_node_move_to_one_row_per_id_preserving_order() {
    let rows = vec![row("nodeMove", json!({ "id": "a", "x": 1 })), row("nodeMove", json!({ "id": "b", "x": 2 })), row("nodeMove", json!({ "id": "a", "x": 3 }))];
    let result = coalesce_board2d_events(&rows);
    let parsed: Vec<Value> = serde_json::from_str(&result.events_json).unwrap();
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0]["payload"]["id"], "a");
    assert_eq!(parsed[0]["payload"]["x"], 3);
    assert_eq!(parsed[1]["payload"]["id"], "b");
}

#[test]
fn coalesce_drops_node_move_entirely_when_drag_end_follows() {
    let rows = vec![row("nodeMove", json!({ "id": "a", "x": 1 })), row("nodeDragEnd", json!({ "moves": [] }))];
    let result = coalesce_board2d_events(&rows);
    let parsed: Vec<Value> = serde_json::from_str(&result.events_json).unwrap();
    assert_eq!(parsed.len(), 1);
    assert_eq!(parsed[0]["name"], "nodeDragEnd");
}

#[test]
fn coalesce_flags_flush_now_for_edge_and_brush_events() {
    for name in ["preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"] {
        let result = coalesce_board2d_events(&[row(name, json!({}))]);
        assert!(result.flush_now, "{name} should flush immediately");
    }
}

#[test]
fn coalesce_empty_input_produces_empty_array_and_no_flush() {
    let result = coalesce_board2d_events(&[]);
    assert!(!result.flush_now);
    assert_eq!(result.events_json, "[]");
}
