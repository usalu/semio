/// 🔍️ Detail LOD, variadic, wheel and minimap: detail LOD body hovers and drags, variadic plus hits, wheel zoom pinning, pointer pan, the minimap widget laws and the forced LOD paint.

#[test]
fn detail_lod_non_channel_body_hovers_and_selects_node() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_move_screen(sx, sy, false, false, false);
    assert!(hovered_node_id(&host.viewport, &host.graph).as_deref() == Some("combine"));
    assert!(host.graph.engine.hover.is_some());
    assert!(!host.graph.engine.selection.node_ids.contains(&host.graph.node_id_for_widget_id("combine").expect("combine node id")));
    let divider_x = computation_column_divider_x(&combine).expect("divider");
    let (_, header_top, _, header_bottom) = channel_row_bounds(&combine, 0);
    let title_probe = canvas::Point::new(divider_x, (header_top + header_bottom) * 0.5);
    let (body_sx, body_sy) = world_to_screen_px(&host, title_probe);
    host.pointer_move_screen(body_sx, body_sy, false, false, false);
    assert!(hovered_node_id(&host.viewport, &host.graph).as_deref() == Some("combine"));
    assert!(host.graph.engine.hover.is_some());
    host.pointer_down(body_sx, body_sy, false);
    assert!(host.graph.selected_node_ids().contains(&"combine".to_string()));
    host.pointer_up(body_sx, body_sy);
}

#[test]
fn visible_handle_lod_row_center_does_not_start_edge_draw() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    let scale = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "scale").expect("scale");
    let port_idx = scale.outputs().iter().position(|p| p.id == "out").expect("port out");
    let (x0, y0, x1, y1) = output_port_row_hit_bounds(scale, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "scale@out");
    assert!((row_center.x - handle.x).abs() > 4.0, "row center should sit away from the painted handle anchor");
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_down(sx, sy, false);
    assert!(!matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }), "visible handles require anchor hit for wire draw");
    let (hsx, hsy) = world_to_screen_px(&host, handle);
    host.pointer_down(hsx, hsy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DrawEdge { .. }));
}

#[test]
fn detail_lod_channel_row_drags_node_without_prior_selection() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("micro");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "b").expect("port b");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let row_center = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let handle = handle_world(&host, "combine@b");
    assert!((row_center.x - handle.x).abs() > 4.0);
    let (sx, sy) = world_to_screen_px(&host, row_center);
    host.pointer_down(sx, sy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DragNode { .. }));
    assert!(host.graph.selected_node_ids().contains(&"combine".to_string()));
}

#[test]
fn detail_lod_title_row_drags_node() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("detail");
    let combine = host.graph.host_snapshot.nodes.iter().find(|n| n.id == "combine").expect("combine").clone();
    let port_idx = combine.inputs().iter().position(|p| p.id == "a").expect("port a");
    let (x0, y0, x1, y1) = input_port_row_hit_bounds(&combine, port_idx).expect("row bounds");
    let title_probe = canvas::Point::new((x0 + x1) * 0.5, (y0 + y1) * 0.5);
    let (sx, sy) = world_to_screen_px(&host, title_probe);
    host.pointer_down(sx, sy, false);
    assert!(matches!(host.graph.engine.interaction, InteractionMode::DragNode { .. }));
}

#[test]
fn variadic_plus_hit_maps_insert_index() {
    let inputs = vec![IoPortSpec { id: "0".into(), label: "0".into(), ..Default::default() }, IoPortSpec { id: "1".into(), label: "1".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width("dictionary.merge", &inputs, &outputs);
    let height = computation_node_height(2, 1, true, false);
    let host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
        nodes: vec![DagNodeSpec::computation("merge".into(), "Merge", "Merge", "emoji:🔀️".into(), inputs, outputs, true, false, 0.0, 0.0, width, height)],
        edges: vec![],
    });
    let positions = variadic_input_insert_positions(&host.graph.host_snapshot.nodes[0]);
    assert_eq!(positions.len(), 1);
    let (_, px, py) = positions[0];
    let hit = host.graph.port_insert_hit(px, py, 2.0).expect("hit");
    assert_eq!(hit.0, DagPortSide::Input);
    assert_eq!(hit.1, "merge");
    assert_eq!(hit.2, 2);
    assert!(host.graph.port_insert_hit(px, py, 1.0).is_none());
}

#[test]
fn variadic_output_plus_hit_maps_insert_index() {
    let outputs = vec![IoPortSpec { id: "0".into(), label: "i".into(), ..Default::default() }];
    let inputs = vec![IoPortSpec { id: "list".into(), label: "list".into(), ..Default::default() }, IoPortSpec { id: "index".into(), label: "index".into(), ..Default::default() }];
    let width = computation_node_width("list.get", &inputs, &outputs);
    let height = computation_node_height(2, 1, false, true);
    let host = DagHost::from_host_snapshot_without_layout(DagHostSnapshot {
        schema: "dag.hostDocument".into(),
        camera: DagCamera { x: 0.0, y: 0.0, zoom: 2.0 },
        nodes: vec![DagNodeSpec::computation("get".into(), "Get", "Get", "emoji:📋️".into(), inputs, outputs, false, true, 0.0, 0.0, width, height)],
        edges: vec![],
    });
    let positions = variadic_output_insert_positions(&host.graph.host_snapshot.nodes[0]);
    assert_eq!(positions.len(), 1);
    let (_, px, py) = positions[0];
    let hit = host.graph.port_insert_hit(px, py, 2.0).expect("hit");
    assert_eq!(hit.0, DagPortSide::Output);
    assert_eq!(hit.1, "get");
    assert_eq!(hit.2, 1);
}

#[test]
fn wheel_zoom_pins_draw_lod_until_gesture_ends() {
    let mut host = DagHost::default_demo();
    host.graph.host_snapshot.camera.zoom = 1.2;
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Normal);
    dag_set_wheel_zoom_active(&mut host.viewport, &mut host.graph, true);
    host.graph.host_snapshot.camera.zoom = 0.45;
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Normal);
    dag_set_wheel_zoom_active(&mut host.viewport, &mut host.graph, false);
    assert_eq!(draw_lod_for_frame(&host.viewport, &host.graph), DagDrawLod::Overview);
}

#[test]
fn pointer_pan_gesture_moves_camera() {
    let mut host = DagHost::default_demo();
    host.graph.set_camera(0.0, 0.0, 2.0);
    host.pointer_down_screen(100.0, 100.0, 1, false, false, false, true);
    host.pointer_move_screen(150.0, 100.0, false, false, false);
    assert!((host.graph.host_snapshot.camera.x - -25.0).abs() < 1e-6);
}

#[test]
fn minimap_widget_label_overlay_includes_rect_when_visible() {
    let mut host = DagHost::default_demo();
    host.minimap.set_minimap_widget_visible(true);
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_camera(500.0, 400.0, 3.0);
    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let minimap = raw.get("minimapWidget").and_then(|v| v.as_object()).expect("minimap widget json");
    assert!(minimap.get("width").and_then(|v| v.as_f64()).unwrap_or(0.0) > 0.0);
}

#[test]
fn minimap_widget_click_repositions_camera() {
    let mut host = DagHost::default_demo();
    host.minimap.set_minimap_widget_visible(true);
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_camera(500.0, 400.0, 3.0);
    let raw: Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let minimap = raw.get("minimapWidget").expect("minimap");
    let x = minimap["x"].as_f64().unwrap() + minimap["width"].as_f64().unwrap() * 0.5;
    let y = minimap["y"].as_f64().unwrap() + minimap["height"].as_f64().unwrap() * 0.5;
    let before_x = host.graph.host_snapshot.camera.x;
    host.pointer_down_screen(x, y, 0, false, false, false, false);
    host.pointer_up_screen(x, y, false, false, false);
    assert!((host.graph.host_snapshot.camera.x - before_x).abs() > 1.0);
}

#[test]
fn minimap_widget_paints_without_panic() {
    let mut host = DagHost::default_demo();
    host.minimap.set_minimap_widget_visible(true);
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_camera(120.0, 80.0, 0.75);
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
}

#[test]
fn minimap_widget_panel_uses_square_corners() {
    let mut host = DagHost::default_demo();
    host.minimap.set_minimap_widget_visible(true);
    host.viewport.set_viewport(1280, 800, 1.0);
    host.graph.set_camera(-5000.0, -5000.0, 0.75);
    assert!(!host.graph.minimap_camera_fully_shows_content(&host.graph.minimap_widget_content_bounds().unwrap(), 1280, 800));
    let layout = minimap_widget_layout(&host.graph, &host.minimap, 1280, 800).expect("minimap layout");
    let (px0, py0, px1, py1) = layout.panel;
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    let encoded: serde_json::Value =
        serde_json::from_str(&canvas::draw_list::scene_draw_list_json(&scene, canvas::draw_list::DrawListOptions::default())).expect("draw list");
    let epsilon = 0.5;
    let panel_shape = |shape: &serde_json::Value| -> bool {
        if shape[0].as_str() != Some("r") {
            return false;
        }
        let coords = shape.as_array().expect("rect coords");
        assert_eq!(coords.len(), 5, "flat rect tuple");
        let nums = coords[1..].iter().map(|v| v.as_f64().expect("coord")).collect::<Vec<_>>();
        (nums[0] - px0).abs() < epsilon && (nums[1] - py0).abs() < epsilon && (nums[2] - px1).abs() < epsilon && (nums[3] - py1).abs() < epsilon
    };
    let panel_commands = encoded["commands"]
        .as_array()
        .expect("commands")
        .iter()
        .filter(|command| {
            let shape = match command[0].as_str() {
                Some("f") => &command[4],
                Some("s") => &command[8],
                _ => return false,
            };
            panel_shape(shape)
        })
        .count();
    assert_eq!(panel_commands, 2, "minimap panel must fill and stroke as axis-aligned rects, not rounded rects");
}

/// 🏷️ A captioned tier DELEGATES its captions to the overlay (`node_caption_delegated_to_js_overlay`),
/// so "still labelled at a forced low zoom" is a property of the overlay rows, not of the scene's
/// path count — which is node chrome only, two paths per node. This used to assert `> 12` paths and
/// measured 10 whichever string the tier served (recorded in
/// `26/09/09/PROCEDURAL-3D-END-TO-END/📓️node-graph-camera-fit-labels-2026-09-12.md`).
#[test]
fn dag_paint_scene_keeps_labels_when_lod_forced_at_low_zoom() {
    let mut host = DagHost::default_demo();
    host.viewport.set_viewport(1280, 800, 1.0);
    host.viewport.set_automatic_lod(false);
    host.viewport.set_forced_draw_lod_label("compact");
    host.graph.host_snapshot.camera.zoom = 0.25;
    let mut scene = canvas::Scene::new();
    host.paint_scene(&mut scene, 1280, 800, 1.0);
    assert!(scene.path_count() >= host.graph.host_snapshot.nodes.len() * 2, "a forced compact LOD must still paint every node's chrome");
    let state: semio_framework_pack_json::Value = semio_framework_pack_json::parse(&host.label_overlay_paint_state_json().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let rows = state["labels"].as_array().expect("label overlay rows");
    assert_eq!(rows.len(), host.graph.host_snapshot.nodes.len(), "a forced compact LOD must caption every node");
    for (row, node) in rows.iter().zip(&host.graph.host_snapshot.nodes) {
        assert_eq!(row["text"], semio_framework_pack_json::Value::from(node.name.as_str()), "a caption is the node's NAME at every captioned tier");
    }
}
