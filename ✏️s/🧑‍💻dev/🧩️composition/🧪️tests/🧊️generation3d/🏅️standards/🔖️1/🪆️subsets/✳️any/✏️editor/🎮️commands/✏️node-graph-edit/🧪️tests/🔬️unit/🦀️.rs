use super::*;
use crate::editor_domain::editor_laws::context::{app, dispatch, snapshot, Generation3dApp};
use semio_s_artifact_procedural_generation3d::editor::generation3d::Generation3dCommand;

async fn edit(app: &mut Generation3dApp, operations: serde_json::Value) {
    dispatch(app, Generation3dCommand::NodeGraphEdit(NodeGraphEdit { operations_json: operations.to_string() })).await;
}

/// 🔌️ One live wire of the current document, as the four endpoint coordinates a port-to-port drag
/// reports plus the synapse id a wire cut names: `(id, fromNode, fromPort, toNode, toPort)`. Read off
/// the document's OWN synapses rather than off a port projection, so the law needs no `FlowHost`
/// (building one inside a 2 MiB test thread overflows the stack).
fn one_live_wire(host_snapshot: &FlowHostSnapshot) -> (String, String, String, String, String) {
    let synapse = host_snapshot.synapses.first().expect("every bundled example wires at least one synapse");
    (synapse.id.clone(), synapse.from.clone(), synapse.from_port.clone(), synapse.to.clone(), synapse.to_port.clone())
}

fn wire_exists(host_snapshot: &FlowHostSnapshot, from: &str, from_port: &str, to: &str, to_port: &str) -> bool {
    host_snapshot.synapses.iter().any(|synapse| synapse.from == from && synapse.from_port == from_port && synapse.to == to && synapse.to_port == to_port)
}

/// ⚖️ LAW: `disconnect` cuts a wire and `connect` draws it again between the same two ports — the two
/// halves of node-graph wire editing, both reachable through the ONE `nodeGraphEdit` command the
/// canvas dispatches, and both asserted on the document's own synapse table.
///
/// 🐛️ `disconnect` used to fall through the `_ => {}` arm, so every wire cut was a silent no-op that
/// still spent a whole retained command (`📓️audit-user-journey-gaps-2026-09-13.md` gap #3).
#[semio_framework_async_macros::async_test]
async fn disconnect_then_connect_round_trips_one_wire() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let (before, synapse_id, from, from_port, to, to_port) = {
        let read = snapshot(&app);
        let (id, from, from_port, to, to_port) = one_live_wire(&read.host_snapshot);
        (read.host_snapshot.synapses.len(), id, from, from_port, to, to_port)
    };

    edit(&mut app, serde_json::json!([{ "operation": "disconnect", "synapseId": synapse_id }])).await;
    {
        let read = snapshot(&app);
        assert_eq!(read.host_snapshot.synapses.len(), before - 1, "disconnect must remove exactly the one wire it names");
        assert!(!wire_exists(&read.host_snapshot, &from, &from_port, &to, &to_port), "the cut wire must be gone: {synapse_id}");
    }

    edit(&mut app, serde_json::json!([{ "operation": "connect", "sourceNodeId": from, "sourcePortId": from_port, "targetNodeId": to, "targetPortId": to_port }])).await;
    {
        let read = snapshot(&app);
        assert_eq!(read.host_snapshot.synapses.len(), before, "connect must restore exactly one wire");
        assert!(wire_exists(&read.host_snapshot, &from, &from_port, &to, &to_port), "connect must re-wire {from}:{from_port} -> {to}:{to_port}");
    }
}

/// ⚖️ LAW: `move` is the node-graph gesture record of a released node drag (design §13.3): every named widget lands at
/// its BASE position plus the ONE relative offset, committed as ONE `move-nodes` leaf. It is the ONE spelling of the
/// node-move verb — the duplicate `moveMediaNode` command, which had no dispatcher anywhere in the renderer, was deleted
/// (`📓️audit-user-journey-gaps-2026-09-13.md` gap #10).
#[semio_framework_async_macros::async_test]
async fn move_offsets_the_widget_a_node_drag_names_from_its_base() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app().await;
    let (node_id, base) = {
        let read = snapshot(&app);
        let id = read.host_snapshot.widgets.iter().map(semio_s_artifact_procedural_generation3d::widget_id).find(|id| read.host_snapshot.layout.get(*id).is_some()).expect("the fixture places a widget").to_string();
        let base = read.host_snapshot.layout.get(id.as_str()).map(|layout| (layout.x, layout.y)).expect("its base position");
        (id, base)
    };

    edit(&mut app, serde_json::json!([{ "operation": "move", "gestureId": "node-drag:1", "nodeIds": [node_id], "dx": -321.0, "dy": 654.0 }])).await;

    let landed = {
        let read = snapshot(&app);
        let position = read.host_snapshot.layout.get(node_id.as_str()).unwrap_or_else(|| panic!("widget {node_id} keeps its pinned layout after a move"));
        (position.x, position.y)
    };
    assert_eq!(landed, (base.0 - 321.0, base.1 + 654.0), "move must offset the widget from where the drag started");
}

//#region 🎚️SliderGesture
/// 🎚️ The table a dragged inline slider must answer — the twin of the host lane's own
/// (`📺️renderer/🧑‍🎨engine/🧪️tests/🎚️continuous-gesture-lane`), over the same numbers.
const SLIDER_GESTURE_FIXTURE_JSON: &str = include_str!("../../../../../../../../../../../../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🎚️slider-gesture.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureFixture {
    schema: String,
    presses: Vec<SliderPressRow>,
    live_evaluations: SliderGestureLiveEvaluations,
}

#[derive(serde::Deserialize)]
struct SliderPressRow {
    row: String,
    dispatches: Vec<serde_json::Value>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SliderGestureLiveEvaluations {
    max_live_round_trips: usize,
    commits_per_press: usize,
}

/// ⚖️ LAW: N rapid values cost at most two live round trips and exactly one commit per press — the host lane's ceiling,
/// stated here over the SAME table the TypeScript twin reads so the two implementations cannot disagree about what a
/// gesture is allowed to cost. The release is the dispatch's own top-level `commit` (the framework scrub machine).
#[test]
fn a_press_spends_at_most_two_live_round_trips_and_one_commit() {
    let table: SliderGestureFixture = serde_json::from_str(SLIDER_GESTURE_FIXTURE_JSON).expect("slider gesture fixture");
    assert_eq!(table.schema, "s.procedural.generation3d.slider-gesture/v2");
    assert_eq!(table.live_evaluations.max_live_round_trips, 2, "one round trip in flight and one owed is the whole budget");
    assert_eq!(table.live_evaluations.commits_per_press, 1, "a press releases once");
    let press = table.presses.iter().find(|row| row.row == "one-press-is-one-transaction").expect("the table must carry the one-press row");
    let commits = press.dispatches.iter().filter(|dispatch_row| dispatch_row["commit"] == true).count();
    assert_eq!(commits, table.live_evaluations.commits_per_press, "exactly one dispatch of a press carries the release");
}
//#endregion 🎚️SliderGesture
