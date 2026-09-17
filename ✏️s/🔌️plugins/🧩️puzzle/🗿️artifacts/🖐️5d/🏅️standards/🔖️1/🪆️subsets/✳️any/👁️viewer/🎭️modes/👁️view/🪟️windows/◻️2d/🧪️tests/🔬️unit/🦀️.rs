use super::*;

#[test]
fn definition_is_a_read_only_board_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
    assert!(def.utilities.is_empty(), "a viewer window binds no utilities");
    assert!(def.actions.is_empty(), "a viewer window declares no actions");
}

#[test]
fn render_produces_a_scene_node_for_the_default_document() {
    let document = Puzzle5dSnapshot::default();
    let _node = render(&document);
}

#[test]
fn fixture_carries_one_node_per_part_and_one_edge_per_fastener() {
    let document = Puzzle5dSnapshot {
        parts: vec![Puzzle5dPart { id: "teil-ä".into(), ..Default::default() }, Puzzle5dPart { id: "teil-ß".into(), ..Default::default() }],
        fasteners: vec![Puzzle5dFastener {
            id: "kante-ß".into(),
            source: "teil-ä:g1".into(),
            target: "teil-ß:g1".into(),
            fastener_kind: None,
            gap: 0.0,
            shift: 0.0,
            rise: 0.0,
            rotation: 0.0,
            turn: 0.0,
            tilt: 0.0,
            x: 0.0,
            y: 0.0,
        }],
        ..Default::default()
    };
    let fixture: Value = serde_json::from_str(&board_fixture_json(&document)).expect("board fixture json");
    assert_eq!(fixture["nodes"].as_array().map(Vec::len), Some(2));
    assert_eq!(fixture["edges"].as_array().map(Vec::len), Some(1));
    assert_eq!(fixture["edges"][0]["source"], serde_json::json!("teil-ä:g1"));
}

#[test]
fn board_scene_is_never_interactive() {
    let scene = board_scene(&Puzzle5dSnapshot::default());
    assert!(!scene.interactive, "a viewer board never accepts input");
    assert_eq!(scene.selection_json, "[]");
    assert!(scene.hovered_id.is_none());
    assert!(!scene.selectable_nodes && !scene.selectable_edges && !scene.selectable_handles);
}
