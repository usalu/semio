/// 🧮️ Cursors, projection and selection laws: the cursor grant, selected node and edge cursors, bounded interaction projection, selection hover and dimming, handle hover, idle move hover and the delete selected laws.

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

fn cursor_grant() -> DagSelectionTextGrant {
    DagSelectionTextGrant { fuel: 1, now_milliseconds: 1, deadline_milliseconds: 8, cancelled: false, interrupted: false }
}

#[test]
fn selected_nodes_cursor_censuses_and_emits_one_byte_per_grant() {
    let vectors: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🎯️selected-json.json")).unwrap();
    for case in vectors["cases"].as_array().unwrap() {
        let ids: Vec<String> = case["ids"].as_array().unwrap().iter().map(|id|id.as_str().unwrap().into()).collect();
        let fixture = DagHostSnapshot { schema: "dag.hostDocument".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: ids.iter().map(|id|DagNodeSpec { id: id.clone(), ..Default::default() }).collect(), edges: vec![] };
        let mut host = DagHost::from_host_snapshot_without_layout(fixture);
        host.graph.set_selection(&ids);
        assert_eq!(host.graph.selected_node_ids(), ids);
        let expected = semio_framework_pack_json::to_json_string(&host.graph.selected_node_ids()).into_bytes();
        assert_eq!(expected, serde_json::to_vec(&host.graph.selected_node_ids()).unwrap());
        let mut cursor = DagSelectionJsonCursor::default();
        let mut rejected = cursor_grant();
        rejected.fuel = 0;
        assert_eq!(cursor.step(&host, rejected), Err(DagSelectionTextFault::NoFuel));
        let mut output = Vec::new();
        let mut census = None;
        loop {
            match cursor.step(&host, cursor_grant()).unwrap() {
                DagSelectionTextStep::Census { bytes } => census = Some(bytes),
                DagSelectionTextStep::Byte(byte) => output.push(byte),
                DagSelectionTextStep::Complete => break,
                DagSelectionTextStep::Progress { .. } => {}
            }
        }
        assert_eq!(census, Some(expected.len()));
        assert_eq!(output, expected);
        assert_eq!(cursor.step(&host, cursor_grant()), Ok(DagSelectionTextStep::Complete));
    }
}

#[test]
fn selected_edges_cursor_matches_direct_encode() {
    let mut host = DagHost::default_demo();
    let edge = host.graph.host_snapshot.edges.first().expect("demo edge").id.clone();
    host.graph.set_selection_domains(&DagSelectionDomains { edges: vec![edge], ..Default::default() });
    let expected = semio_framework_pack_json::to_json_string(&host.graph.selected_edge_ids()).into_bytes();
    let mut cursor = DagSelectionJsonCursor::edges();
    let mut output = Vec::new();
    let mut census = None;
    loop {
        match cursor.step(&host, cursor_grant()).unwrap() {
            DagSelectionTextStep::Census { bytes } => census = Some(bytes),
            DagSelectionTextStep::Byte(byte) => output.push(byte),
            DagSelectionTextStep::Complete => break,
            DagSelectionTextStep::Progress { .. } => {}
        }
    }
    assert_eq!(census, Some(expected.len()));
    assert_eq!(output, expected);
}

#[test]
fn bounded_interaction_projection_rejects_node_and_identifier_overflow() {
    let mut fixture = DagHostSnapshot::default();
    fixture.nodes = (0..=DAG_INTERACTION_NODE_CAPACITY).map(|index| DagNodeSpec { id: format!("node-{index}"), ..Default::default() }).collect();
    let host = DagHost::from_host_snapshot_without_layout(fixture);
    assert_eq!(host.graph.bounded_interaction_projection(0).unwrap_err(), DagInteractionPlanFault::NodeCredits);

    let mut fixture = DagHostSnapshot::default();
    fixture.nodes = vec![DagNodeSpec { id: "x".repeat(16 * 1024 + 1), ..Default::default() }];
    let host = DagHost::from_host_snapshot_without_layout(fixture);
    assert_eq!(host.graph.bounded_interaction_projection(0).unwrap_err(), DagInteractionPlanFault::StringCredits);
}

#[test]
fn dag_selection_hover_and_dimmed_map_widget_ids() {
    let fixture = DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("a".into(), "A", "A", "emoji:🔷️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 0.0, 0.0, 160.0, 24.0),
            DagNodeSpec::computation(
                "b".into(),
                "B",
                "B",
                "emoji:🔷️".into(),
                vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
                false,
                false,
                200.0,
                0.0,
                160.0,
                24.0,
            ),
        ],
        edges: vec![],
    };
    let mut host = DagHost::from_host_snapshot(fixture);
    host.graph.set_selection(&["a".to_string()]);
    assert_eq!(host.graph.selected_node_ids(), vec!["a"]);
    host.graph.set_hover(Some("b"));
    assert_eq!(hovered_node_id(&host.viewport, &host.graph).as_deref(), Some("b"));
    set_dimmed(&mut host.graph, &mut host.paint, &["a".to_string()]);
    assert_eq!(dimmed_node_ids(&host.graph, &host.paint), vec!["a"]);
}

#[test]
fn handle_hover_does_not_hover_parent_node() {
    let fixture = DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec::computation(
            "merge".into(),
            "Merge",
            "M",
            "emoji:🔀️".into(),
            vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }],
            vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
            false,
            false,
            0.0,
            0.0,
            160.0,
            48.0,
        )],
        edges: vec![],
    };
    let mut host = DagHost::from_host_snapshot(fixture);
    host.viewport.set_viewport(800, 600, 1.0);
    let (handle_center, node_id) = {
        let snap = host.graph.engine.render_snapshot();
        let (hid, center, _) = snap.handles.first().expect("input handle");
        let node_id = host.graph.engine.handles.get(hid).expect("handle").node_id;
        (*center, node_id)
    };
    let (sx, sy) = world_to_screen_px(&host, handle_center);
    host.pointer_move_screen(sx, sy, false, false, false);
    let hover = host.graph.engine.hover.expect("handle hover");
    assert!(host.graph.engine.handles.contains_key(&hover));
    assert!(hovered_node_id(&host.viewport, &host.graph).is_none());
    assert!(!host.graph.is_node_hovered(node_id));
    host.pointer_move_screen(0.0, 0.0, false, false, false);
    assert!(host.graph.engine.hover.is_none());
}

#[test]
fn idle_pointer_move_updates_hover() {
    use canvas::Point;
    use canvas::camera::{Camera, Viewport, world_to_screen};

    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(800, 600, 1.0);
    let camera = Camera { x: host.graph.host_snapshot.camera.x, y: host.graph.host_snapshot.camera.y, zoom: host.graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let slider = world_to_screen(&camera, &viewport, Point::new(-400.0, -40.0));
    host.pointer_move_screen(slider.x, slider.y, false, false, false);
    assert_eq!(hovered_node_id(&host.viewport, &host.graph).as_deref(), Some("slider"));
    host.pointer_move_screen(8.0, 8.0, false, false, false);
    assert!(hovered_node_id(&host.viewport, &host.graph).is_none());
}

#[test]
fn dag_host_delete_selected_preserves_remaining_positions() {
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("a".into(), "A", "A", "emoji:🔷️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 100.0, 200.0, 160.0, 56.0),
            DagNodeSpec::computation(
                "b".into(),
                "B",
                "B",
                "emoji:🔷️".into(),
                vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
                false,
                false,
                400.0,
                500.0,
                160.0,
                56.0,
            ),
            DagNodeSpec::computation("c".into(), "C", "C", "emoji:🔷️".into(), vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }], vec![], false, false, 700.0, 300.0, 160.0, 56.0),
        ],
        edges: vec![DagHostSnapshotEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }, DagHostSnapshotEdge { id: "e2".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() }],
    });
    host.graph.set_selection(&["b".to_string()]);
    host.graph.delete_selected();
    let a = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "a").expect("a");
    let c = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "c").expect("c");
    assert!((a.x - 100.0).abs() < 0.01);
    assert!((a.y - 200.0).abs() < 0.01);
    assert!((c.x - 700.0).abs() < 0.01);
    assert!((c.y - 300.0).abs() < 0.01);
    assert!(host.graph.host_snapshot.nodes.iter().all(|n| n.id != "b"));
}

#[test]
fn dag_host_delete_selected_removes_edge_only_selection() {
    let mut host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![
            DagNodeSpec::computation("a".into(), "A", "A", "emoji:🔷️".into(), vec![], vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }], false, false, 100.0, 200.0, 160.0, 56.0),
            DagNodeSpec::computation(
                "b".into(),
                "B",
                "B",
                "emoji:🔷️".into(),
                vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }],
                vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
                false,
                false,
                400.0,
                500.0,
                160.0,
                56.0,
            ),
            DagNodeSpec::computation("c".into(), "C", "C", "emoji:🔷️".into(), vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }], vec![], false, false, 700.0, 300.0, 160.0, 56.0),
        ],
        edges: vec![DagHostSnapshotEdge { id: "e1".into(), source: "a@out".into(), target: "b@in".into(), ..Default::default() }, DagHostSnapshotEdge { id: "e2".into(), source: "b@out".into(), target: "c@in".into(), ..Default::default() }],
    });
    let edge_id = *host.graph.engine.edges.keys().next().expect("edge");
    host.graph.engine.selection.edge_ids.insert(edge_id);
    assert!(host.graph.has_selection());
    assert_eq!(host.graph.host_snapshot.edges.len(), 2);
    host.graph.delete_selected();
    assert_eq!(host.graph.host_snapshot.edges.len(), 1);
    assert_eq!(host.graph.host_snapshot.nodes.len(), 3);
    assert!(!host.graph.has_selection());
}
