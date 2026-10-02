//! 🔗️ LAW (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §13.3): the shared DAG journal encodes every graph edit
//! as exactly the `nodeGraphEdit` row the language-agnostic fixture accepts — the rows React and wgpu both dispatch — and
//! narrates what a gesture or an align moved as one record per distinct offset.
//!
//! Oracle: `🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json` (schema `🧬️schema/🔣️node-graph-edit-rows`); its
//! TypeScript twin is `📺️renderer/🧑‍🎨engine/🧪️tests/🧪️node-graph-edit-rows/🟦️.ts`.

use super::*;
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json")).expect("node-graph edit rows fixture")
}

/// 🔢️ `value` with every number as `f64`, so an integral JSON number and its float spelling compare equal.
fn normalized(value: &Value) -> Value {
    match value {
        Value::Number(number) => Value::from(number.as_f64().expect("finite number")),
        Value::Array(items) => Value::Array(items.iter().map(normalized).collect()),
        Value::Object(fields) => Value::Object(fields.iter().map(|(key, value)| (key.clone(), normalized(value))).collect()),
        other => other.clone(),
    }
}

/// 🔗️ The journal edit a fixture row narrates; `None` for a row no journal writes (`delete` is a verb's row).
fn journal_edit(row: &Value) -> Option<DagGraphEdit> {
    let text = |field: &str| row[field].as_str().unwrap_or_else(|| panic!("{field}")).to_string();
    Some(match row["operation"].as_str()? {
        "connect" => DagGraphEdit::Connect { source_node_id: text("sourceNodeId"), source_port_id: text("sourcePortId"), target_node_id: text("targetNodeId"), target_port_id: text("targetPortId") },
        "disconnect" => DagGraphEdit::Disconnect { synapse_id: text("synapseId") },
        "move" => DagGraphEdit::Move { gesture_id: text("gestureId"), node_ids: row["nodeIds"].as_array()?.iter().filter_map(|id| id.as_str().map(str::to_string)).collect(), dx: row["dx"].as_f64()?, dy: row["dy"].as_f64()? },
        "setSlider" => DagGraphEdit::SetSlider { node_id: text("widgetId"), value: row["value"].as_f64()? },
        "insertPort" => DagGraphEdit::InsertPort { node_id: text("nodeId"), side: if text("side") == "input" { DagPortSide::Input } else { DagPortSide::Output }, index: usize::try_from(row["index"].as_u64()?).ok()? },
        _ => return None,
    })
}

/// ⚖️ LAW: every journal edit encodes as the very row the fixture accepts, in order, under `operations`.
#[test]
fn every_journal_edit_encodes_as_the_row_the_fixture_accepts() {
    let fixture = fixture();
    let journalled: Vec<Value> = fixture["accepted"].as_array().expect("accepted rows").iter().map(|case| case["row"].clone()).filter(|row| journal_edit(row).is_some()).collect();
    assert_eq!(journalled.len(), 6, "connect, disconnect, move, setSlider and both port sides");
    let edits: Vec<DagGraphEdit> = journalled.iter().filter_map(journal_edit).collect();
    let encoded: Value = serde_json::from_str(&dag_graph_edit_rows_json(edits)).expect("rows json");
    assert_eq!(normalized(&encoded), normalized(&serde_json::json!({ "operations": journalled })));
    assert_eq!(dag_graph_edit_rows_json(Vec::new()), r#"{"operations":[]}"#, "an empty journal is no row");
}

fn node(id: &str, x: f64, y: f64) -> DagNodeSpec {
    let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width(id, &inputs, &outputs);
    let height = computation_node_height(inputs.len(), outputs.len(), false, false);
    DagNodeSpec::computation(id.into(), id, id, "emoji:🔢️".into(), inputs, outputs, false, false, x, y, width, height)
}

fn host(nodes: Vec<DagNodeSpec>) -> DagHost {
    DagHost::from_host_snapshot_without_layout(DagHostSnapshot { schema: "dag.host_snapshot".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes, edges: vec![] })
}

/// ⚖️ LAW: what moved since a press is one `move` record per distinct offset, the nodes of each in node order; a node that
/// stayed and a node the baseline does not know are not narrated, and a press that moved nothing journals nothing.
#[test]
fn moves_since_a_press_are_one_record_per_distinct_offset() {
    let mut host = host(vec![node("a", 0.0, 0.0), node("b", 100.0, 0.0), node("c", 200.0, 0.0), node("d", 300.0, 0.0)]);
    let baseline = host.node_positions();
    assert!(host.journal_moves_since("node-drag:1", &baseline) && host.take_graph_edits().is_empty(), "nothing moved");
    let moved = [("a", 10.0, 5.0), ("b", 110.0, 5.0), ("c", 200.0, -40.0)];
    for (id, x, y) in moved {
        let node = host.host_snapshot.nodes.iter_mut().find(|node| node.id == id).expect("node");
        (node.x, node.y) = (x, y);
    }
    let without_d: Vec<(String, f64, f64)> = baseline.into_iter().filter(|(id, _, _)| id != "d").collect();
    host.host_snapshot.nodes.iter_mut().find(|node| node.id == "d").expect("d").x = 999.0;
    assert!(host.journal_moves_since("node-drag:2", &without_d));
    assert_eq!(
        host.take_graph_edits(),
        vec![
            DagGraphEdit::Move { gesture_id: "node-drag:2".into(), node_ids: vec!["a".into(), "b".into()], dx: 10.0, dy: 5.0 },
            DagGraphEdit::Move { gesture_id: "node-drag:2".into(), node_ids: vec!["c".into()], dx: 0.0, dy: -40.0 },
        ]
    );
}

/// ⚖️ LAW: a port the embedding host inserted is journalled once, by node, side and index.
#[test]
fn an_inserted_port_is_journalled_once() {
    let mut host = host(vec![node("a", 0.0, 0.0)]);
    assert!(host.journal_port_insert("a".into(), DagPortSide::Input, 2));
    assert!(host.journal_port_insert("a".into(), DagPortSide::Input, 2), "a duplicate is accepted and dropped");
    assert_eq!(host.take_graph_edits(), vec![DagGraphEdit::InsertPort { node_id: "a".into(), side: DagPortSide::Input, index: 2 }]);
}
