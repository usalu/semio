use super::DagNodeGraphEditOp;
use super::*;
use crate::editor::dag::commands::{connect_media_ports, disconnect, patch_dag_nodes};
use crate::editor::dag::unit_tests::context;
use crate::editor::dag::unit_tests::context::DagApp;
use crate::editor::dag::DagCommand;
use semio_framework::kernel::HistoryEntry;
use semio_framework_plugin::artifact_app_laws::{meta, settle_history_verb, settle_registered_typed_operation};
use semio_framework_plugin::PluginApp;
use serde_json::json;
use crate::{DagNodeKind, DagScene};

fn scene(app: &DagApp) -> DagScene {
    ::semio_framework_async::poll::resolve_ready(context::live_scene(app))
}

/// 🧪️ `nodeGraphEdit` batches id-keyed sub-edits: a `connect` row adds an edge, and a `delete` row removes exactly the
/// nodes it names together with every wire they hold — never an ambient selection.
#[semio_framework_async_macros::async_test]
async fn node_graph_edit_connects_then_deletes_the_named_node() {
    let mut app = context::new_app().await;
    let (source_id, target_id) = {
        let nodes = scene(&app).nodes;
        (nodes[0].id.clone(), nodes[1].id.clone())
    };
    let edges_before = scene(&app).edges.len();
    context::dispatch(
        &mut app,
        DagCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![DagNodeGraphEditOp::Connect { source_node_id: source_id.clone(), source_port_id: "out".into(), target_node_id: target_id, target_port_id: "in".into() }] }),
    )
    .await;
    assert!(scene(&app).edges.len() >= edges_before, "connect either adds an edge or is a safe no-op (e.g. a cycle)");
    let nodes_before = scene(&app).nodes.len();
    context::dispatch(&mut app, DagCommand::NodeGraphEdit(NodeGraphEdit { operations: vec![DagNodeGraphEditOp::Delete { node_ids: vec![source_id.clone()], synapse_ids: Vec::new() }] })).await;
    let projection = scene(&app);
    assert_eq!(projection.nodes.len(), nodes_before - 1);
    assert!(!projection.edges.iter().any(|edge| edge.source.starts_with(&format!("{source_id}@")) || edge.target.starts_with(&format!("{source_id}@"))), "the deleted node's wires go with it");
    context::close(&mut app);
}

/// 🧪️ A mounted wrapper never answers `InvocationResult.mutations`, so the "unknown edge is a
/// no-op" half is proved on the projected document instead of on the invocation answer.
#[semio_framework_async_macros::async_test]
async fn disconnect_removes_a_known_edge_and_is_a_no_op_for_an_unknown_one() {
    let mut app = context::new_app().await;
    let edge_id = scene(&app).edges.first().map(|edge| edge.id.clone());
    if let Some(edge_id) = edge_id {
        let edges_before = scene(&app).edges.len();
        context::dispatch(&mut app, DagCommand::Disconnect(disconnect::Disconnect { edge_id })).await;
        assert_eq!(scene(&app).edges.len(), edges_before - 1);
    }
    let before = scene(&app);
    context::dispatch(&mut app, DagCommand::Disconnect(disconnect::Disconnect { edge_id: "nonexistent".into() })).await;
    assert_eq!(scene(&app), before, "disconnecting an unknown edge leaves the document untouched");
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn connect_media_ports_adds_an_edge_between_two_nodes() {
    let mut app = context::new_app().await;
    let (source_id, target_id) = {
        let nodes = scene(&app).nodes;
        (nodes[0].id.clone(), nodes[1].id.clone())
    };
    let edges_before = scene(&app).edges.len();
    context::dispatch(&mut app, DagCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: source_id, source_port_id: "out".into(), target_node_id: target_id, target_port_id: "in".into() })).await;
    assert!(scene(&app).edges.len() >= edges_before);
    context::close(&mut app);
}

//#region ✋️GestureLaws
/// 🕹️ One `nodeGraphEdit` exactly as a host sends it (`operations` plus a press's top-level `gesture`/`commit`/`abort`),
/// settled through the registered ladder; a host cancel carries no value and is settled by the runtime itself.
async fn send(app: &mut DagApp, verb: &str, args: serde_json::Value) {
    let action_meta = meta("local");
    let aborting = args.get("abort").is_some();
    let args: semio_framework_value::DslValue = args.into();
    app.handle_action(verb, Some(&args), &action_meta).await.unwrap_or_else(|fault| panic!("{verb} admitted: {fault:?}"));
    if !aborting {
        settle_registered_typed_operation(app, action_meta.instance_id).await.expect("the dispatch settles");
    }
}

/// 🧾️ Every applied history row whose mutations land in the composed `content` member (design §12), oldest first.
async fn edit_rows(app: &mut DagApp) -> Vec<HistoryEntry> {
    let mut rows: Vec<HistoryEntry> = PluginApp::history_snapshot(app).await.expect("history").upserts.into_iter().filter(|entry| entry.applied && entry.mutations.iter().any(|mutation| mutation.store.is_some())).collect();
    rows.sort_by_key(|entry| entry.seq);
    rows
}

fn position(app: &DagApp, id: &str) -> (f64, f64) {
    scene(app).nodes.into_iter().find(|node| node.id == id).map(|node| (node.x, node.y)).expect("node")
}

fn slider_value(app: &DagApp, id: &str) -> f64 {
    match scene(app).nodes.into_iter().find(|node| node.id == id).map(|node| node.kind) {
        Some(DagNodeKind::Slider { value, .. }) => value,
        other => panic!("{id} is no slider: {other:?}"),
    }
}

fn english(entry: &HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string()
}

fn german(entry: &HistoryEntry) -> String {
    entry.label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De).to_string()
}

/// ⚖️ LAW: a released node drag (the node-graph gesture record, as both hosts write it) is ONE composed-child edit, one row
/// stamped with its transaction (tool `<appId>#nodeGraphEdit`), whose op is the RELATIVE graph `drag-nodes` leaf; every
/// dragged node lands at its base position plus the offset, and one undo puts them back.
#[semio_framework_async_macros::async_test]
async fn a_node_drag_record_is_one_transaction_of_one_relative_move() {
    let mut app = context::new_app().await;
    let ids: Vec<String> = scene(&app).nodes.iter().take(2).map(|node| node.id.clone()).collect();
    let bases: Vec<(f64, f64)> = ids.iter().map(|id| position(&app, id)).collect();
    let before = edit_rows(&mut app).await.len();
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": ids, "dx": 40.0, "dy": -12.5 }] })).await;
    for (id, (x, y)) in ids.iter().zip(&bases) {
        assert_eq!(position(&app, id), (x + 40.0, y - 12.5));
    }
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 1, "one drag, one row: {rows:?}");
    let transaction = rows[0].transaction.as_ref().expect("the row is keyed by its tool transaction");
    assert!(transaction.id.starts_with("tx-") && transaction.tool == "s.dag.dag@1/*#editor#nodeGraphEdit", "{transaction:?}");
    assert_eq!(rows[0].mutations.len(), 1, "one relative leaf for the whole selection");
    assert!(rows[0].op_lines.iter().all(|line| line.starts_with("dragNodes")), "{:?}", rows[0].op_lines);
    assert_eq!(english(&rows[0]), "Drag 2 nodes by (40, -12.5)");
    assert_eq!(german(&rows[0]), "2 Knoten um (40; -12,5) ziehen");
    settle_history_verb(&mut app, "undo", meta("local").instance_id).await;
    for (id, base) in ids.iter().zip(&bases) {
        assert_eq!(position(&app, id), *base, "one undo restores the drag");
    }
    context::close(&mut app);
}

/// ⚖️ LAW: a drag that moves nothing leaves zero trace; two drags are two transactions; a palette drop
/// (`moveMediaNode`) is the same relative leaf from the node's base position, under its own tool.
#[semio_framework_async_macros::async_test]
async fn zero_trace_two_transactions_and_the_palette_drop() {
    let mut app = context::new_app().await;
    let id = scene(&app).nodes[0].id.clone();
    let (x, y) = position(&app, &id);
    let before = edit_rows(&mut app).await.len();
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:0", "nodeIds": [id], "dx": 0.0, "dy": 0.0 }] })).await;
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:0", "nodeIds": ["ghost"], "dx": 10.0, "dy": 0.0 }] })).await;
    assert_eq!(edit_rows(&mut app).await.len(), before, "nothing moved, nothing recorded");
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": [id], "dx": 10.0, "dy": 0.0 }] })).await;
    send(&mut app, "moveMediaNode", json!({ "nodeId": id, "x": x + 30.0, "y": y + 80.0 })).await;
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 2, "{rows:?}");
    assert_ne!(rows[0].transaction.as_ref().expect("first").id, rows[1].transaction.as_ref().expect("second").id, "two moves are two transactions");
    assert_eq!(rows[1].transaction.as_ref().expect("drop").tool, "s.dag.dag@1/*#editor#moveMediaNode");
    assert_eq!(english(&rows[1]), "Drag 1 node by (20, 80)", "the drop is the offset from the base position after the first drag");
    assert_eq!(position(&app, &id), (x + 30.0, y + 80.0));
    context::close(&mut app);
}

/// ⚖️ LAW: a slider press on the canvas overlay keeps its ticks provisional and lands as ONE composed-child edit with ONE
/// transaction of the absolute `set-node-property` leaf; a second press is a second transaction; a cancelled press leaves
/// zero trace.
#[semio_framework_async_macros::async_test]
async fn a_slider_press_is_one_transaction_and_a_cancel_is_zero_trace() {
    let mut app = context::new_app().await;
    let base = slider_value(&app, "slider");
    let before = edit_rows(&mut app).await.len();
    for (value, commit) in [(6.0, false), (7.0, false), (7.5, true)] {
        send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "setSlider", "widgetId": "slider", "value": value }], "gesture": "press-1", "commit": commit })).await;
        assert_eq!(slider_value(&app, "slider"), if commit { value } else { base }, "a tick stays provisional");
    }
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "setSlider", "widgetId": "slider", "value": 2.0 }], "gesture": "press-2", "commit": true })).await;
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "setSlider", "widgetId": "slider", "value": 9.0 }], "gesture": "press-3", "commit": false })).await;
    send(&mut app, "nodeGraphEdit", json!({ "operations": [], "gesture": "press-3", "abort": "blur" })).await;
    assert_eq!(slider_value(&app, "slider"), 2.0, "the cancelled press left no value");
    let rows = edit_rows(&mut app).await;
    let rows = &rows[before..];
    assert_eq!(rows.len(), 2, "two released presses, two rows: {rows:?}");
    assert!(rows.iter().all(|entry| entry.op_lines.first().is_some_and(|line| line.starts_with("setNodeProperty"))), "{rows:?}");
    let transactions: std::collections::BTreeSet<&str> = rows.iter().map(|entry| entry.transaction.as_ref().expect("every press is a transaction").id.as_str()).collect();
    assert_eq!(transactions.len(), 2);
    assert_eq!(english(&rows[0]), "Set \"value\" of node \"slider\" to 7.5");
    assert_eq!(german(&rows[0]), "\"value\" von Knoten \"slider\" auf 7,5 setzen");
    context::close(&mut app);
}

/// ⚖️ LAW: the inspector's slider fields and name field are absolute child leaves — `set-node-property` per addressed
/// slider field (plus `resize-node` only when the widget refits) and `change-node-label` per renamed node — never a
/// whole-node replace.
#[semio_framework_async_macros::async_test]
async fn the_inspector_patches_are_absolute_leaves() {
    let mut app = context::new_app().await;
    let before = edit_rows(&mut app).await.len();
    context::dispatch(&mut app, DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec!["slider".into()], field: "max".into(), value: "20".into() })).await;
    context::dispatch(&mut app, DagCommand::PatchDagNodes(patch_dag_nodes::PatchDagNodes { node_ids: vec!["slider".into()], field: "name".into(), value: "Volume".into() })).await;
    let rows = edit_rows(&mut app).await;
    let lines: Vec<&String> = rows[before..].iter().flat_map(|entry| entry.op_lines.iter()).collect();
    assert!(lines.first().is_some_and(|line| line.starts_with("setNodeProperty")) && lines.last().is_some_and(|line| line.starts_with("changeNodeLabel")), "{lines:?}");
    assert!(lines.iter().all(|line| line.starts_with("setNodeProperty") || line.starts_with("resizeNode") || line.starts_with("changeNodeLabel")), "{lines:?}");
    context::close(&mut app);
}

async fn history_edit(app: &mut DagApp, verb: &str, args: serde_json::Value) {
    let args: semio_framework_value::DslValue = args.into();
    let result = app.handle_action(verb, Some(&args), &meta("local")).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));
    assert!(result.output.get("rejected").is_none(), "{verb} was refused: {:?}", result.output);
}

async fn time_travel_stage(app: &mut DagApp) -> Option<semio_framework::kernel::HistoryTimeTravelStage> {
    app.history_snapshot().await.expect("history").time_travel.map(|status| status.stage)
}

async fn pump_time_travel(app: &mut DagApp, done: impl Fn(Option<semio_framework::kernel::HistoryTimeTravelStage>) -> bool) {
    for _ in 0..10_000 {
        if done(time_travel_stage(app).await) {
            return;
        }
        app.advance_typed_operation_publication().await.expect("a driver turn");
        while app.take_typed_operation_ui_progress().is_some() {}
    }
    panic!("the history edit never settled: {:?}", time_travel_stage(app).await);
}

/// ⚖️ LAW: time travel edits the drag's offset, never the tool: superseding the first drag's `dx` replays the later drag
/// of the same node on the new base, and the overwritten head equals a fresh run of the edited drags.
#[semio_framework_async_macros::async_test]
async fn editing_a_drag_offset_in_history_replays_downstream() {
    let mut app = context::new_app().await;
    let id = scene(&app).nodes[0].id.clone();
    let (x, y) = position(&app, &id);
    let store = format!("content/{}", app.snapshot().expect("snapshot").content.child_id);
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": [id], "dx": 10.0, "dy": 0.0 }] })).await;
    send(&mut app, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:2", "nodeIds": [id], "dx": 0.0, "dy": 7.0 }] })).await;
    let rows = edit_rows(&mut app).await;
    let first = rows[rows.len() - 2].mutations.first().map(|mutation| mutation.mutation_id.clone()).expect("the first drag's mutation");
    history_edit(&mut app, "historyEditBegin", json!({ "mutationId": first, "store": store })).await;
    history_edit(&mut app, "historyEditInput", json!({ "path": "/dx", "value": 55.0 })).await;
    history_edit(&mut app, "historyEditAccept", json!({})).await;
    pump_time_travel(&mut app, |stage| stage != Some(semio_framework::kernel::HistoryTimeTravelStage::Replaying)).await;
    assert_eq!(time_travel_stage(&mut app).await, Some(semio_framework::kernel::HistoryTimeTravelStage::Reviewing));
    assert_eq!(position(&app, &id), (x + 10.0, y + 7.0), "reviewing never touches the committed document");
    history_edit(&mut app, "historyEditFinalize", json!({})).await;
    history_edit(&mut app, "historyEditCommit", json!({ "choice": "overwrite" })).await;
    pump_time_travel(&mut app, |stage| stage.is_none()).await;
    assert_eq!(position(&app, &id), (x + 55.0, y + 7.0), "the edited offset replays and the later drag still applies on top");
    let mut fresh = context::new_app().await;
    send(&mut fresh, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:3", "nodeIds": [id], "dx": 55.0, "dy": 0.0 }] })).await;
    send(&mut fresh, "nodeGraphEdit", json!({ "operations": [{ "operation": "move", "gestureId": "node-drag:4", "nodeIds": [id], "dx": 0.0, "dy": 7.0 }] })).await;
    assert_eq!(scene(&app).nodes, scene(&fresh).nodes, "the edited log equals a fresh run of the edited drags");
    context::close(&mut app);
    context::close(&mut fresh);
}

/// ⚖️ LAW: the host wire decodes by name — an unknown operation or a malformed gesture record is refused, never guessed.
#[test]
fn the_host_wire_decodes_by_name() {
    let decode = |value: serde_json::Value| NodeGraphEdit::from_action_args(Some(&semio_framework_value::DslValue::from(value)));
    let decoded = decode(json!({ "operations": [{ "operation": "move", "gestureId": "g", "nodeIds": ["a"], "dx": 1.0, "dy": 2.0 }, { "operation": "setSlider", "widgetId": "s", "value": 3.0 }], "gesture": "press", "commit": true })).expect("host rows decode");
    assert_eq!(decoded.operations, vec![DagNodeGraphEditOp::Move { gesture_id: "g".into(), node_ids: vec!["a".into()], dx: 1.0, dy: 2.0 }, DagNodeGraphEditOp::SetSlider { widget_id: "s".into(), value: 3.0 }]);
    assert!(decode(json!({ "operations": [{ "operation": "move", "gestureId": "g", "nodeIds": ["a"], "dx": 1.0 }] })).is_err(), "a gesture record misses dy");
    assert!(decode(json!({ "operations": [{ "operation": "teleport" }] })).is_err());
    assert_eq!(decode(json!({ "operations": [], "gesture": "press", "abort": "blur" })).expect("an abort carries no rows").operations, Vec::new());
}
/// ⚖️ LAW: the guest decodes the renderer's committed node-graph row vocabulary — every accepted row, and only those — and
/// maps each to its typed row (`insertPort` sides by name, `delete` by its node and wire ids).
#[test]
fn the_renderer_row_fixture_decodes_exactly() {
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS).expect("the row fixture parses");
    let batch = |row: &serde_json::Value| NodeGraphEdit::from_action_args(Some(&semio_framework_value::DslValue::from(json!({ "operations": [row] }))));
    for case in fixture["accepted"].as_array().expect("accepted rows") {
        assert!(batch(&case["row"]).is_ok(), "accepted row {} refused", case["id"]);
    }
    for case in fixture["refused"].as_array().expect("refused rows") {
        assert!(batch(&case["row"]).is_err(), "refused row {} decoded", case["id"]);
    }
    let decoded = NodeGraphEdit::from_action_args(Some(&semio_framework_value::DslValue::from(json!({ "operations": [
        { "operation": "insertPort", "nodeId": "add", "side": "output", "index": 0 },
        { "operation": "delete", "nodeIds": ["add"], "synapseIds": ["s1"] }
    ] })))).expect("rows decode");
    assert_eq!(decoded.operations, vec![DagNodeGraphEditOp::InsertPort { node_id: "add".into(), side: "output".into(), index: 0 }, DagNodeGraphEditOp::Delete { node_ids: vec!["add".into()], synapse_ids: vec!["s1".into()] }]);
}

/// 🧾️ The framework's committed node-graph row vocabulary (schema `🧰️framework/🔨️modules/🛠️tool-machine/🧬️schema/🔣️node-graph-edit-rows`).
const NODE_GRAPH_EDIT_ROWS: &str = include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");
//#endregion ✋️GestureLaws
