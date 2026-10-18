/// 📝️ Manual LOD, notes and preview tree: manual LOD pinning, slider height fit, note editing and overlays, the node content LOD matrix, the preview tree toggle and the paint smoke.

#[test]
fn manual_lod_pins_draw_tier_until_automatic_restored() {
    let mut host = DagHost::default_demo();
    host.graph.host_snapshot.camera.zoom = 1.0;
    assert_eq!(dag_draw_lod_label(&host.viewport, &host.graph), "normal");
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("minimap");
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Minimap);
    host.graph.host_snapshot.camera.zoom = 5.0;
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Minimap);
    host.viewport.set_automatic_lod(true);
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Micro);
}

#[test]
fn fit_note_sizes_keeps_slider_height() {
    let mut host = DagHost::from_host_snapshot(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "note".into(),
            name: "Note".into(),
            abbreviation: "Note".into(),
            icon: "emoji:📝️".into(),
            x: 0.0,
            y: 0.0,
            width: 80.0,
            height: 80.0,
            kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.graph.fit_note_sizes();
    assert_eq!(host.graph.host_snapshot.nodes[0].height, DAG_CHANNEL_ROW_HEIGHT);
    let DagNodeKind::Note { text, .. } = &mut host.graph.host_snapshot.nodes[0].kind else {
        panic!("expected note");
    };
    *text = "a much longer note body".into();
    host.graph.fit_note_sizes();
    assert_eq!(host.graph.host_snapshot.nodes[0].height, DAG_CHANNEL_ROW_HEIGHT);
    assert_eq!(host.graph.host_snapshot.nodes[0].width, DAG_COMPONENT_WIDTH);
}

#[test]
fn begin_note_edit_inserts_and_backspaces_text() {
    let mut host = DagHost::from_host_snapshot(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "note".into(),
            name: "Note".into(),
            abbreviation: "Note".into(),
            icon: "emoji:📝️".into(),
            x: 0.0,
            y: 0.0,
            width: note_widget_size("hi").0,
            height: note_widget_size("hi").1,
            kind: DagNodeKind::Note { text: "hi".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    let origin_x = note_text_origin_x(&host.graph.host_snapshot.nodes[0]);
    assert!(begin_note_edit(&mut host.graph, &mut host.notes, "note", origin_x + 100.0, 0.0));
    assert_eq!(host.notes.editing_note_id(), Some("note"));
    assert!(note_insert_text(&mut host.graph, &mut host.notes, "!"));
    {
        let DagNodeKind::Note { text, .. } = &host.graph.host_snapshot.nodes[0].kind else {
            panic!("expected note");
        };
        assert_eq!(text, "hi!");
    }
    assert!(note_backspace(&mut host.graph, &mut host.notes));
    {
        let DagNodeKind::Note { text, .. } = &host.graph.host_snapshot.nodes[0].kind else {
            panic!("expected note");
        };
        assert_eq!(text, "hi");
    }
    host.notes.note_commit_edit();
    assert_eq!(host.notes.editing_note_id(), None);
}

#[test]
fn note_label_overlay_skips_title_and_ports() {
    let mut host = DagHost::from_host_snapshot(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "note".into(),
            name: "Note".into(),
            abbreviation: "Note".into(),
            icon: "emoji:📝️".into(),
            x: 0.0,
            y: 0.0,
            width: note_widget_size("hello").0,
            height: note_widget_size("hello").1,
            kind: DagNodeKind::Note { text: "hello".into(), output: IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.viewport.set_viewport(800, 600, 1.0);
    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let labels = raw["labels"].as_array().expect("labels");
    assert!(labels.iter().all(|row| row["text"] != "Note" && row["text"] != "out"));
}

#[test]
fn dag_draw_lod_node_content_matrix() {
    assert!(!DagDrawLod::Minimap.node_icon_visible());
    assert_eq!(DagDrawLod::Minimap.node_label(), DagNodeLabel::None);
    assert!(DagDrawLod::Overview.node_icon_visible());
    assert_eq!(DagDrawLod::Overview.node_label(), DagNodeLabel::None);
    assert!(!DagDrawLod::Compact.node_icon_visible());
    assert_eq!(DagDrawLod::Compact.node_label(), DagNodeLabel::Name);
    assert!(!DagDrawLod::Normal.node_icon_visible());
    assert_eq!(DagDrawLod::Normal.node_label(), DagNodeLabel::Name);
    assert!(!DagDrawLod::Detail.node_icon_visible());
    assert_eq!(DagDrawLod::Detail.node_label(), DagNodeLabel::Name);
    assert!(!DagDrawLod::Micro.node_icon_visible());
    assert_eq!(DagDrawLod::Micro.node_label(), DagNodeLabel::Name);
    let computation = DagNodeSpec::computation(
        "add".into(),
        "Add",
        "Add",
        "emoji:➕️".into(),
        vec![IoPortSpec { id: "a".into(), label: "a".into(), ..Default::default() }],
        vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }],
        false,
        false,
        0.0,
        0.0,
        120.0,
        56.0,
    );
    assert!(DagHost::should_paint_node_lod_icon(&computation, DagDrawLod::Overview));
    assert!(!DagHost::should_paint_node_lod_icon(&computation, DagDrawLod::Detail));
    assert!(!DagHost::should_paint_node_lod_icon(&computation, DagDrawLod::Micro));
}

#[test]
fn preview_tree_toggle_expands_and_resizes() {
    let json = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::object([("alpha".to_string(), semio_framework_pack_json::object([("beta".to_string(), Value::from(1))])), ("gamma".to_string(), Value::from("x"))]));
    let mut host = DagHost::from_host_snapshot(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 },
        nodes: vec![DagNodeSpec {
            id: "preview".into(),
            name: "Preview".into(),
            abbreviation: "Preview".into(),
            icon: "emoji:👁️".into(),
            x: 0.0,
            y: 0.0,
            width: 80.0,
            height: 80.0,
            kind: DagNodeKind::Preview { content: DagPreviewContent::Tree { json: json.clone() }, expanded: DagExpandedPaths::new(), input: IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() } },
            ..Default::default()
        }],
        edges: vec![],
    });
    host.viewport.set_viewport(800, 600, 1.0);
    let collapsed_h = host.graph.host_snapshot.nodes[0].height;
    let layouts = preview_tree_row_layouts(&host.graph.host_snapshot.nodes[0], &json, &DagExpandedPaths::new());
    let row = layouts.iter().find(|entry| entry.path == "alpha").expect("alpha row");
    let (x0, y0, x1, y1) = row.row_rect;
    let world_x = x0 + (x1 - x0) * 0.75;
    let world_y = (y0 + y1) * 0.5;
    use canvas::Point;
    use canvas::camera::{Camera as CanvasCamera, Viewport, world_to_screen};
    let cam = CanvasCamera { x: host.graph.host_snapshot.camera.x, y: host.graph.host_snapshot.camera.y, zoom: host.graph.host_snapshot.camera.zoom };
    let viewport = Viewport { width: 800, height: 600, dpr: 1.0 };
    let screen = world_to_screen(&cam, &viewport, Point::new(world_x, world_y));
    host.pointer_down(screen.x, screen.y, false);
    let expanded_h = host.graph.host_snapshot.nodes[0].height;
    assert!(expanded_h > collapsed_h);
    let DagNodeKind::Preview { expanded, .. } = &host.graph.host_snapshot.nodes[0].kind else {
        panic!("preview kind");
    };
    assert!(expanded.contains("alpha"));
}

#[test]
fn dag_paint_scene_smoke_at_overview_and_micro_zoom() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    let mut scene = canvas::Scene::new();
    host.graph.host_snapshot.camera.zoom = 0.3;
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    host.graph.host_snapshot.camera.zoom = 5.0;
    host.paint_scene(&mut scene, 1280, 800, 1.0);
}
