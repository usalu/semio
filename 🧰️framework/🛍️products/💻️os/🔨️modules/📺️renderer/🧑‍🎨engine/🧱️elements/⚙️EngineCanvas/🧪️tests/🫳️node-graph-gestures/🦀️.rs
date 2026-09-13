//! 🫳️ What the five node-graph pointer gestures cost and what they dispatch, driven through the REAL
//! wgpu entry points (`node_graph_pointer_{down,move,up}_into`) over the REAL `FlowHost` built from
//! generation3d's own `hexagonal-mushroom-column` payload — the same graph the 6118 serve boots.
//!
//! Oracle: `🧑‍🎨engine/🧫️fixtures/🫳️node-graph-gestures/🔣️.json`. TypeScript twin:
//! `🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts`.
//!
//! Every coordinate below is DERIVED from the live host rather than authored: a node's draggable body
//! is the first point in its rect that `DagScreenHit::is_draggable_body` claims (its own rect minus
//! its inline widgets and connector dots), and a port's grab point is the published connector centre.
//! Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane `wgpu-node-graph-gestures`.

use super::node_graph_attach_tests::{drop_engine_surface, entity_screen_rect, flow_window_scene};
use super::*;
use ui_wgpu::wgpu::{InputState, SurfaceKind};

const GESTURE_LAW: &str = include_str!("../../../../🧫️fixtures/🫳️node-graph-gestures/🔣️.json");

fn law() -> Value {
    serde_json::from_str(GESTURE_LAW).expect("the gesture oracle parses")
}

fn law_case(name: &str) -> Value {
    law().get("cases").and_then(Value::as_array).expect("cases").iter().find(|case| case.get("name").and_then(Value::as_str) == Some(name)).cloned().unwrap_or_else(|| panic!("{name} is declared in the oracle"))
}

/// 🧪️ One attached surface, live for the length of one law.
fn attach(surface_id: &str, bounds: Rect) -> UiComponentSceneNode {
    drop_engine_surface(surface_id);
    let scene = flow_window_scene(surface_id);
    assert_eq!(scene.component_kind, SurfaceKind::NodeGraph, "the law drives a node-graph surface");
    assert!(sync_node_graph_scene(&scene, "law-window", bounds, Theme::default().panel), "the scene attaches its flow engine");
    scene
}

/// 🩺️ The host's own classification of a surface point — the ONE the path discriminator reads.
fn screen_hit(surface_id: &str, bounds: Rect, x: f32, y: f32) -> flow::dag::DagScreenHit {
    ENGINE_SURFACES.with(|cell| {
        let map = cell.borrow();
        let Some(NodeGraphEngine::Flow(host)) = map.get(surface_id).and_then(|entry| entry.node_graph.as_ref()) else { panic!("live flow host") };
        host.dag.screen_hit((x - bounds.x) as f64, (y - bounds.y) as f64)
    })
}

/// 🫳️ The first point of a node's own rect a press would select and drag from. Scanned rather than
/// authored: which part of a node is body and which is inline widget is the engine's answer, not this
/// law's guess.
fn draggable_body_point(surface_id: &str, node_id: &str, bounds: Rect) -> (f32, f32) {
    let rect = entity_screen_rect(surface_id, "node", node_id);
    for row in 0..24 {
        for column in 0..24 {
            let x = bounds.x + (rect[0] + rect[2] * (0.08 + 0.84 * f64::from(column) / 23.0)) as f32;
            let y = bounds.y + (rect[1] + rect[3] * (0.04 + 0.92 * f64::from(row) / 23.0)) as f32;
            let hit = screen_hit(surface_id, bounds, x, y);
            if hit.is_draggable_body() && hit.node_id.as_deref() == Some(node_id) {
                return (x, y);
            }
        }
    }
    panic!("{node_id} has a draggable body region");
}

/// 🔌️ The published centre of one channel's connector — the one rect `entity_screen_json` reports for
/// `"handle"`, which is also the one the pointer grabs.
fn handle_point(surface_id: &str, channel_id: &str, bounds: Rect) -> (f32, f32) {
    let rect = entity_screen_rect(surface_id, "handle", channel_id);
    (bounds.x + (rect[0] + rect[2] * 0.5) as f32, bounds.y + (rect[1] + rect[3] * 0.5) as f32)
}

/// 🫳️ One whole press-drag-release through the production entry points on ONE input state.
fn drag_gesture(surface_id: &str, controller_id: &str, bounds: Rect, from: (f32, f32), to: (f32, f32)) -> Vec<ActionDescriptor> {
    let mut input = InputState::<ActionDescriptor>::default();
    let mut actions = Vec::new();
    node_graph_pointer_down_into(surface_id, controller_id, bounds, from.0, from.1, 0, false, false, false, false, &mut input).expect("the press is admitted");
    actions.extend(crate::collect_fixture_actions(&mut input));
    for step in 1..=6 {
        let x = from.0 + (to.0 - from.0) * step as f32 / 6.0;
        let y = from.1 + (to.1 - from.1) * step as f32 / 6.0;
        node_graph_pointer_move_into(surface_id, controller_id, bounds, x, y, false, false, false, &mut input).expect("every intermediate move is admitted");
        actions.extend(crate::collect_fixture_actions(&mut input));
    }
    node_graph_pointer_up_into(surface_id, controller_id, bounds, to.0, to.1, false, false, false, &mut input).expect("the release is admitted");
    actions.extend(crate::collect_fixture_actions(&mut input));
    actions
}

/// 🔗️ The `nodeGraphEdit` sub-operations one gesture dispatched, as plain rows.
fn edit_operations(actions: &[ActionDescriptor]) -> Vec<Value> {
    actions
        .iter()
        .filter(|action| action.action == "nodeGraphEdit")
        .flat_map(|action| {
            let json = Value::from(action.args.clone().unwrap_or(dsl::DslValue::Null));
            json.get("operations").and_then(Value::as_array).cloned().unwrap_or_default()
        })
        .collect()
}

fn selection_targets(actions: &[ActionDescriptor]) -> Option<String> {
    actions.iter().rev().find(|action| action.action == "interactionSelect").map(|action| node_graph_attach_tests::action_fields(action).into_iter().find(|(key, _)| key == "targets").map(|(_, value)| value).unwrap_or_default())
}

#[test]
fn a_hover_that_changes_nothing_publishes_nothing_and_never_exhausts_the_action_queue() {
    let _serialized = engine_surface_law_guard();
    let case = law_case("an-unchanged-hover-is-free");
    let repeats = case["gesture"]["repeats"].as_u64().expect("repeats") as usize;
    let surface_id = "node-graph-gesture-idle-hover";
    let bounds = Rect { x: 0.0, y: 0.0, w: 966.0, h: 836.0 };
    let scene = attach(surface_id, bounds);
    let (x, y) = draggable_body_point(surface_id, case["gesture"]["over"].as_str().expect("over"), bounds);

    let mut input = InputState::<ActionDescriptor>::default();
    node_graph_pointer_move_into(surface_id, &scene.controller_id, bounds, x, y, false, false, false, &mut input).expect("the first move is admitted");
    let first = crate::collect_fixture_actions(&mut input);
    assert!(
        first.len() <= case["expectedFirstMoveActionsAtMost"].as_u64().expect("bound") as usize,
        "the first hover over a node publishes at most the three interaction actions, got {:?}",
        first.iter().map(|action| action.action.as_str()).collect::<Vec<_>>()
    );

    for turn in 0..repeats {
        node_graph_pointer_move_into(surface_id, &scene.controller_id, bounds, x, y, false, false, false, &mut input).unwrap_or_else(|fault| panic!("move {turn} of an unchanged hover must not fault, got {fault:?}"));
        let published = crate::collect_fixture_actions(&mut input);
        assert!(published.is_empty(), "move {turn} changed nothing and must publish nothing, got {:?}", published.iter().map(|action| action.action.as_str()).collect::<Vec<_>>());
    }
    drop_engine_surface(surface_id);
}

#[test]
fn a_dense_sweep_across_the_whole_surface_survives_an_action_queue_that_is_never_drained() {
    let _serialized = engine_surface_law_guard();
    let case = law_case("a-dense-sweep-survives-an-undrained-queue");
    let columns = case["gesture"]["columns"].as_u64().expect("columns") as usize;
    let rows = case["gesture"]["rows"].as_u64().expect("rows") as usize;
    let surface_id = "node-graph-gesture-sweep";
    let bounds = Rect { x: 0.0, y: 0.0, w: 966.0, h: 836.0 };
    let scene = attach(surface_id, bounds);

    let mut input = InputState::<ActionDescriptor>::default();
    let mut published = 0usize;
    for row in 0..rows {
        for column in 0..columns {
            let x = bounds.x + bounds.w * (column as f32 + 0.5) / columns as f32;
            let y = bounds.y + bounds.h * (row as f32 + 0.5) / rows as f32;
            node_graph_pointer_move_into(surface_id, &scene.controller_id, bounds, x, y, false, false, false, &mut input).unwrap_or_else(|fault| panic!("the sweep faulted at ({x}, {y}) with {fault:?} — a hover is not a mutation"));
            published += 1;
        }
    }
    assert_eq!(published, columns * rows, "every swept point was admitted");
    assert!(
        crate::collect_fixture_actions(&mut input).len() < ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY,
        "a {columns}x{rows} sweep of an undrained queue stays inside {} items",
        ui_wgpu::wgpu::action::ACTION_QUEUE_ITEM_CAPACITY
    );
    drop_engine_surface(surface_id);
}

#[test]
fn a_press_on_a_node_body_selects_that_node_and_a_released_drag_publishes_its_move() {
    let _serialized = engine_surface_law_guard();
    let select_case = law_case("a-body-press-selects-that-node");
    let drag_case = law_case("a-released-drag-publishes-move");
    let click_case = law_case("a-click-is-not-a-move");
    let node_id = select_case["gesture"]["over"].as_str().expect("over");
    let surface_id = "node-graph-gesture-drag";
    let bounds = Rect { x: 0.0, y: 0.0, w: 966.0, h: 836.0 };
    let scene = attach(surface_id, bounds);
    let body = draggable_body_point(surface_id, node_id, bounds);

    let mut input = InputState::<ActionDescriptor>::default();
    node_graph_pointer_down_into(surface_id, &scene.controller_id, bounds, body.0, body.1, 0, false, false, false, false, &mut input).expect("the press is admitted");
    let pressed = crate::collect_fixture_actions(&mut input);
    let expected_selection = serde_json::to_string(&select_case["expectedSelection"].as_array().expect("selection").iter().map(|id| json!({ "granularity": "node", "id": id })).collect::<Vec<_>>()).expect("targets");
    assert_eq!(selection_targets(&pressed).as_deref(), Some(expected_selection.as_str()), "a press on {node_id}'s draggable body selects exactly that node");
    node_graph_pointer_up_into(surface_id, &scene.controller_id, bounds, body.0, body.1, false, false, false, &mut input).expect("the release is admitted");
    let clicked = crate::collect_fixture_actions(&mut input);
    assert_eq!(edit_operations(&clicked), click_case["expectedEdits"].as_array().cloned().unwrap_or_default(), "a click that ended where it started is not a move");

    let before = entity_screen_rect(surface_id, "node", node_id);
    let dx = drag_case["gesture"]["dx"].as_f64().expect("dx") as f32;
    let dy = drag_case["gesture"]["dy"].as_f64().expect("dy") as f32;
    let dragged = drag_gesture(surface_id, &scene.controller_id, bounds, body, (body.0 + dx, body.1 + dy));
    let operations = edit_operations(&dragged);
    let moves: Vec<&Value> = operations.iter().filter(|operation| operation.get("operation").and_then(Value::as_str) == Some("move")).collect();
    assert_eq!(moves.len(), 1, "a one-node drag publishes exactly one move, got {operations:?}");
    assert_eq!(moves[0].get("nodeId").and_then(Value::as_str), Some(node_id));
    for field in law()["operationFields"]["move"].as_array().expect("fields").iter().filter_map(Value::as_str) {
        assert!(moves[0].get(field).is_some(), "the move operation carries {field}");
    }
    let after = entity_screen_rect(surface_id, "node", node_id);
    assert!((after[0] - before[0]).abs() > 1.0 || (after[1] - before[1]).abs() > 1.0, "the dragged node moved on screen, from {before:?} to {after:?}");
    drop_engine_surface(surface_id);
}

#[test]
fn an_output_to_input_drag_publishes_connect() {
    let _serialized = engine_surface_law_guard();
    let connect_case = law_case("a-port-drag-publishes-connect");
    let surface_id = "node-graph-gesture-wire";
    let bounds = Rect { x: 0.0, y: 0.0, w: 966.0, h: 836.0 };
    let scene = attach(surface_id, bounds);

    let from = handle_point(surface_id, connect_case["gesture"]["from"].as_str().expect("from"), bounds);
    let to = handle_point(surface_id, connect_case["gesture"]["to"].as_str().expect("to"), bounds);
    assert!(screen_hit(surface_id, bounds, from.0, from.1).handle, "the published source connector is grabbable");
    assert!(screen_hit(surface_id, bounds, to.0, to.1).handle, "the published target connector is grabbable");
    let wired = drag_gesture(surface_id, &scene.controller_id, bounds, from, to);
    let connects: Vec<Value> = edit_operations(&wired).into_iter().filter(|operation| operation.get("operation").and_then(Value::as_str) == Some("connect")).collect();
    assert_eq!(connects.len(), 1, "one output-to-input drag draws one wire, got {:?}", edit_operations(&wired));
    let expected = connect_case["expectedEdits"].as_array().expect("edits")[0].clone();
    for (key, value) in expected.as_object().expect("object") {
        assert_eq!(connects[0].get(key), Some(value), "the connect operation's {key}");
    }
    for field in law()["operationFields"]["connect"].as_array().expect("fields").iter().filter_map(Value::as_str) {
        assert!(connects[0].get(field).is_some(), "the connect operation carries {field}");
    }
    drop_engine_surface(surface_id);
}

#[test]
fn detaching_a_wired_input_from_its_port_publishes_disconnect() {
    let _serialized = engine_surface_law_guard();
    let cut_case = law_case("a-detached-wire-publishes-disconnect");
    let channel = cut_case["gesture"]["from"].as_str().expect("from");
    let surface_id = "node-graph-gesture-cut";
    let bounds = Rect { x: 0.0, y: 0.0, w: 966.0, h: 836.0 };
    let scene = attach(surface_id, bounds);

    let from = handle_point(surface_id, channel, bounds);
    let synapse = cut_case["expectedEdits"].as_array().expect("edits")[0]["synapseId"].as_str().expect("synapseId");
    let wired_ids = |label: &str| {
        ENGINE_SURFACES.with(|cell| {
            let map = cell.borrow();
            let Some(NodeGraphEngine::Flow(host)) = map.get(surface_id).and_then(|entry| entry.node_graph.as_ref()) else { panic!("live flow host for {label}") };
            host.dag.fixture.edges.iter().map(|edge| edge.id.clone()).collect::<Vec<_>>()
        })
    };
    assert!(wired_ids("before").iter().any(|id| id == synapse), "{synapse} is wired before the cut");
    let cut = drag_gesture(surface_id, &scene.controller_id, bounds, from, (bounds.x + bounds.w - 12.0, bounds.y + bounds.h - 12.0));
    let disconnects: Vec<Value> = edit_operations(&cut).into_iter().filter(|operation| operation.get("operation").and_then(Value::as_str) == Some("disconnect")).collect();
    assert_eq!(disconnects.len(), 1, "dragging {channel} off its port cuts exactly one wire, got {:?}", edit_operations(&cut));
    assert_eq!(disconnects[0].get("synapseId"), cut_case["expectedEdits"].as_array().expect("edits")[0].get("synapseId"), "the cut names the synapse the guest knows");
    assert!(!wired_ids("after").iter().any(|id| id == synapse), "the cut wire left the host's own graph too");
    drop_engine_surface(surface_id);
}

#[test]
fn a_minimap_press_publishes_the_camera_it_moved() {
    let _serialized = engine_surface_law_guard();
    let case = law_case("a-minimap-click-moves-the-camera");
    let surface_id = "node-graph-gesture-minimap";
    let bounds = Rect { x: 0.0, y: 0.0, w: 966.0, h: 836.0 };
    let scene = attach(surface_id, bounds);

    let camera = |label: &str| {
        ENGINE_SURFACES.with(|cell| {
            let map = cell.borrow();
            let Some(NodeGraphEngine::Flow(host)) = map.get(surface_id).and_then(|entry| entry.node_graph.as_ref()) else { panic!("live flow host for {label}") };
            [host.dag.fixture.camera.x, host.dag.fixture.camera.y]
        })
    };
    let mut point = None;
    for row in 0..40 {
        for column in 0..40 {
            let x = bounds.x + bounds.w - 4.0 - bounds.w * 0.34 * column as f32 / 39.0;
            let y = bounds.y + bounds.h - 4.0 - bounds.h * 0.34 * row as f32 / 39.0;
            let hit = screen_hit(surface_id, bounds, x, y);
            if hit.minimap && !hit.minimap_viewport {
                point = Some((x, y));
                break;
            }
        }
        if point.is_some() {
            break;
        }
    }
    let (x, y) = point.expect("the minimap panel has a point outside its own viewport rectangle — WHICH part that is depends on the camera, so it is searched for rather than authored");

    let before = camera("before");
    let mut input = InputState::<ActionDescriptor>::default();
    node_graph_pointer_down_into(surface_id, &scene.controller_id, bounds, x, y, 0, false, false, false, false, &mut input).expect("the minimap press is admitted");
    node_graph_pointer_up_into(surface_id, &scene.controller_id, bounds, x, y, false, false, false, &mut input).expect("the minimap release is admitted");
    let actions = crate::collect_fixture_actions(&mut input);
    let after = camera("after");

    assert_eq!(case["expectedCameraMoves"].as_bool(), Some(true), "the oracle declares this click navigates");
    assert!((after[0] - before[0]).abs() > 0.5 || (after[1] - before[1]).abs() > 0.5, "a minimap click outside the viewport rectangle re-centres the camera, {before:?} -> {after:?}");
    let viewport = actions.iter().rev().find(|action| action.action == "nodeGraphViewport").expect("the moved camera is published");
    assert_eq!(
        viewport.args,
        semio_framework::optional_json_to_dsl(Some(json!({
            "surfaceId": surface_id,
            "viewport": { "x": after[0], "y": after[1], "zoom": ENGINE_SURFACES.with(|cell| { let map = cell.borrow(); let Some(NodeGraphEngine::Flow(host)) = map.get(surface_id).and_then(|entry| entry.node_graph.as_ref()) else { panic!("live flow host") }; host.dag.fixture.camera.zoom }) },
        }))),
        "the published viewport is exactly the camera the minimap committed"
    );
    drop_engine_surface(surface_id);
}

#[test]
fn a_drag_that_crosses_a_port_keeps_the_bounded_path_and_still_publishes_its_move() {
    let _serialized = engine_surface_law_guard();
    let case = law_case("a-drag-that-crosses-a-port-still-publishes-its-move");
    let node_id = case["gesture"]["over"].as_str().expect("over");
    let channel = case["gesture"]["crosses"].as_str().expect("crosses");
    let surface_id = "node-graph-gesture-cross";
    let bounds = Rect { x: 0.0, y: 0.0, w: 966.0, h: 836.0 };
    let scene = attach(surface_id, bounds);
    let body = draggable_body_point(surface_id, node_id, bounds);
    let across = handle_point(surface_id, channel, bounds);
    assert!(screen_hit(surface_id, bounds, across.0, across.1).handle, "the crossed point really is a connector — a plain body point would prove nothing");

    let mut input = InputState::<ActionDescriptor>::default();
    let mut actions = Vec::new();
    let dx = case["gesture"]["dx"].as_f64().expect("dx") as f32;
    let dy = case["gesture"]["dy"].as_f64().expect("dy") as f32;
    node_graph_pointer_down_into(surface_id, &scene.controller_id, bounds, body.0, body.1, 0, false, false, false, false, &mut input).expect("the press is admitted");
    actions.extend(crate::collect_fixture_actions(&mut input));
    node_graph_pointer_move_into(surface_id, &scene.controller_id, bounds, across.0, across.1, false, false, false, &mut input).expect("the move across the connector is admitted");
    actions.extend(crate::collect_fixture_actions(&mut input));
    node_graph_pointer_up_into(surface_id, &scene.controller_id, bounds, body.0 + dx, body.1 + dy, false, false, false, &mut input).expect("the release is admitted");
    actions.extend(crate::collect_fixture_actions(&mut input));

    let moves: Vec<Value> = edit_operations(&actions).into_iter().filter(|operation| operation.get("operation").and_then(Value::as_str) == Some("move")).collect();
    assert_eq!(moves.len(), 1, "the drag survived crossing {channel} and published its move, got {:?}", edit_operations(&actions));
    assert_eq!(moves[0].get("nodeId").and_then(Value::as_str), Some(node_id));
    drop_engine_surface(surface_id);
}
