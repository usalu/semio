/// 🖱️ Gesture journal and canvas slider hits: the screen points, the gesture journal admission, the no-change and drag answers, the auto layout and the canvas slider hits.

/// 📐️ A world point projected through the host's own camera — the projection every screen pointer
/// method is addressed in.
fn world_screen_point(host: &FlowHost, wx: f64, wy: f64) -> (f64, f64) {
    let cam = Camera { x: host.host_snapshot.camera.x, y: host.host_snapshot.camera.y, zoom: host.host_snapshot.camera.zoom };
    let viewport = Viewport { width: host.viewport_w, height: host.viewport_h, dpr: host.viewport_dpr };
    let screen = world_to_screen(&cam, &viewport, Point::new(wx, wy));
    (screen.x, screen.y)
}

/// 🫥️ A world point no node's rect covers — where a press grabs nothing at all.
fn empty_canvas_world_point(host: &FlowHost) -> (f64, f64) {
    let point = (0.0, 260.0);
    for node in &host.dag.graph.host_snapshot.nodes {
        let covered = (point.0 - node.x).abs() <= node.width * 0.5 + 8.0 && (point.1 - node.y).abs() <= node.height * 0.5 + 8.0;
        assert!(!covered, "{} covers the point this law needs empty", node.id);
    }
    point
}

/// 🧭️ Uses a neutral admission fixture and independent JSON reader to verify the canonical journal caller.
#[test]
fn snapshot_io_prerequisite_flow_gesture_journal_admits_or_restores_all_moves() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🚦️journal-admission.json")).unwrap();
    assert_eq!(fixture["capacity"].as_u64().unwrap() as usize, dag::DAG_GRAPH_EDIT_CAPACITY);
    for case in fixture["cases"].as_array().unwrap() {
        let mut host = host_with_test_fixture();
        host.begin_gesture();
        let before = host.host_snapshot.clone();
        let pending = case["pendingRows"].as_u64().unwrap() as usize;
        host.dag.graph.journal_moves("existing", (0..pending).map(|index| (format!("pending-{index}"), index as f64 + 10.0, 0.0))).unwrap();
        for movement in fixture["moves"].as_array().unwrap() {
            let id = movement["id"].as_str().unwrap();
            let mut landed = host.host_snapshot.layout.get(id).unwrap().clone();
            landed.x += movement["dx"].as_f64().unwrap();
            landed.y += movement["dy"].as_f64().unwrap();
            host.host_snapshot.layout.insert(id.to_string(), landed);
        }
        host.commit_gesture_history();
        let answer: serde_json::Value = serde_json::from_str(&host.take_graph_edits_json()).unwrap();
        let admitted = case["admitted"].as_bool().unwrap();
        assert_eq!(answer["operations"].as_array().unwrap().len(), pending + if admitted { 2 } else { 0 });
        if admitted {
            assert!(answer.get("refused").is_none());
            for movement in fixture["moves"].as_array().unwrap() {
                let id = movement["id"].as_str().unwrap();
                assert_eq!(host.host_snapshot.layout.get(id).unwrap().x, before.layout.get(id).unwrap().x + movement["dx"].as_f64().unwrap());
            }
        } else {
            assert_eq!(answer["refused"]["rows"].as_u64(), Some(257));
            assert_eq!(answer["refused"]["limit"].as_u64(), Some(256));
            assert_eq!(host.host_snapshot.layout, before.layout);
            assert!(host.history_store.is_none());
            for node in &host.dag.graph.host_snapshot.nodes {
                let layout = before.layout.get(&node.id).unwrap();
                assert_eq!((node.x, node.y), (layout.x, layout.y));
            }
        }
        before.retire_cold();
        host.retire_cold();
    }
}

fn gesture_answer(host: &mut FlowHost) -> Vec<serde_json::Value> {
    let answer: serde_json::Value = serde_json::from_str(&host.take_graph_edits_json()).expect("gesture answer json");
    assert_eq!(answer.as_object().expect("gesture object").len(), 1, "a gesture answers only its canonical operation journal");
    answer["operations"].as_array().expect("operations array").clone()
}

/// 🪶 LAW: a pointer gesture that changed nothing answers with nothing — no narrow operation AND no
/// fixture commit — so a renderer reading that answer dispatches nothing at all.
///
/// 🩸️ The renderer read an empty `operations` as "fall back to the whole-fixture commit", so EVERY
/// plain click on the graph canvas dispatched a `setFixture` `nodeGraphEdit`: a retained command per
/// click, and — since a landed document mutation owes every attached preview a fresh evaluation — a
/// `flowEvalTick` pair on a shell nobody touched. Measured as the `keyboard-verbs` `baseline` row's
/// `invoked: ["nodeGraphEdit","flowEvalTick","flowEvalTick"]` on an idle, fully settled preview
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md`).
#[test]
fn a_gesture_that_changed_nothing_answers_no_operations_and_no_fixture_commit() {
    let mut host = host_with_test_fixture();
    host.set_viewport(800, 600, 1.0);
    let (wx, wy) = empty_canvas_world_point(&host);
    let (sx, sy) = world_screen_point(&host, wx, wy);
    let positions_before: Vec<(String, f64, f64)> = host.dag.graph.host_snapshot.nodes.iter().map(|node| (node.id.clone(), node.x, node.y)).collect();
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_up_screen(sx, sy, false, false, false);
    let operations = gesture_answer(&mut host);
    assert!(operations.is_empty(), "a click that wired nothing journals no operation: {operations:?}");
    let positions_after: Vec<(String, f64, f64)> = host.dag.graph.host_snapshot.nodes.iter().map(|node| (node.id.clone(), node.x, node.y)).collect();
    assert_eq!(positions_after, positions_before, "the click must not have moved the graph either");
    host.retire_cold();
}

/// 🫳️ LAW's partner: a node drag is narrated as ONE node-graph gesture record — the moved node and its relative offset,
/// the record the wgpu bounded path writes — and owes NO fixture commit, so the guest commits it as relative leaves
/// instead of adopting the whole fixture (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, design §12, §13.3).
/// A slider drag journals its defining `setSlider` row with the landed widget value.
#[test]
fn a_drag_that_moved_a_node_answers_its_move_rows_and_no_fixture_commit() {
    let mut host = host_with_test_fixture();
    host.set_viewport(800, 600, 1.0);
    let node = host.dag.graph.host_snapshot.nodes.iter().find(|node| node.id == "add").expect("add node").clone();
    let before = host.host_snapshot.layout.get("add").expect("add layout").clone();
    let (sx, sy) = world_screen_point(&host, node.x, node.y - node.height * 0.25);
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 60.0, sy + 40.0, false, false, false);
    host.pointer_up_screen(sx + 60.0, sy + 40.0, false, false, false);
    let answer: serde_json::Value = serde_json::from_str(&host.take_graph_edits_json()).expect("gesture answer json");
    let landed = host.host_snapshot.layout.get("add").expect("add layout").clone();
    let operations = answer["operations"].as_array().expect("operations");
    assert_eq!(operations.len(), 1, "one drag is one record: {operations:?}");
    let record = &operations[0];
    assert_eq!((record["operation"].as_str(), record["nodeIds"].clone()), (Some("move"), serde_json::json!(["add"])), "the record names the dragged node: {record}");
    assert!(record["gestureId"].as_str().is_some_and(|id| id.starts_with("node-drag:")), "the record names its press: {record}");
    assert_eq!((record["dx"].as_f64(), record["dy"].as_f64()), (Some(landed.x - before.x), Some(landed.y - before.y)), "the record carries the relative offset: {record}");
    assert_eq!(answer.as_object().expect("gesture object").len(), 1, "a narrated drag answers only its operation journal");
    assert!(landed.x > before.x && landed.y > before.y, "the node landed where the pointer took it: {landed:?} from {before:?}");
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 80.0, sy, false, false, false);
    host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    let operations = gesture_answer(&mut host);
    let value = host.host_snapshot.widgets.iter().find_map(|widget| match widget {
        Widget::InputSlider { id, value, .. } if id.as_str() == "slider" => Some(*value),
        _ => None,
    }).expect("landed slider value");
    assert_eq!(operations, vec![serde_json::json!({ "operation": "setSlider", "widgetId": "slider", "value": value })], "a slider drag journals exactly its landed value");
    host.retire_cold();
}

#[test]
fn default_fixture_does_not_auto_layout() {
    let host = host_with_test_fixture();
    let slider = host.host_snapshot.layout.get("slider").expect("slider");
    let add = host.host_snapshot.layout.get("add").expect("add");
    let preview = host.host_snapshot.layout.get("preview").expect("preview");
    assert_eq!(slider.x, 0.0);
    assert_eq!(add.x, 200.0);
    assert_eq!(preview.x, 400.0);
    host.retire_cold();
}

#[test]
fn canvas_slider_hit_adjusts_value_playground_viewport() {
    let mut host = host_with_test_fixture();
    host.set_viewport(1259, 706, 1.0);
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 90.0, sy, false, false, false);
    host.pointer_up_screen(sx + 90.0, sy, false, false, false);
    let slider = host
        .host_snapshot
        .widgets
        .iter()
        .find_map(|w| match w {
            Widget::InputSlider { id, value, .. } if id.as_str() == "slider" => Some(*value),
            _ => None,
        })
        .unwrap();
    assert!(slider > 3.0);
    host.retire_cold();
}

#[test]
fn canvas_slider_hit_adjusts_value() {
    let mut host = host_with_test_fixture();
    host.set_viewport(800, 600, 1.0);
    let (sx, sy) = widget_slider_track_screen_point(&host, "slider");
    host.pointer_down_screen(sx, sy, 0, false, false, false, false);
    host.pointer_move_screen(sx + 80.0, sy, false, false, false);
    host.pointer_up_screen(sx + 80.0, sy, false, false, false);
    let slider = host
        .host_snapshot
        .widgets
        .iter()
        .find_map(|w| match w {
            Widget::InputSlider { id, value, .. } if id.as_str() == "slider" => Some(*value),
            _ => None,
        })
        .unwrap();
    assert!(slider > 3.0);
    host.retire_cold();
}

#[test]
fn reorganize_overwrites_saved_layout_left_to_right() {
    let mut host = host_with_test_fixture();
    host.host_snapshot.layout.insert("slider".into(), WidgetLayout { x: -900.0, y: -900.0 });
    host.host_snapshot.layout.insert("add".into(), WidgetLayout { x: -900.0, y: -900.0 });
    host.host_snapshot.layout.insert("preview".into(), WidgetLayout { x: -900.0, y: -900.0 });
    host.rebuild_dag();
    let mut progress=|_|true;let mut control=semio_framework_os_infinite::board::schema::layout::LayoutControl::new(100_000_000,&mut progress);host.reorganize(&semio_framework_os_infinite::board::schema::layout::DagLayoutOptions::default(),&mut control).unwrap();
    let slider = host.host_snapshot.layout.get("slider").expect("slider layout");
    let add = host.host_snapshot.layout.get("add").expect("add layout");
    let preview = host.host_snapshot.layout.get("preview").expect("preview layout");
    assert!(add.x > slider.x);
    assert!(preview.x > add.x);
    host.retire_cold();
}
