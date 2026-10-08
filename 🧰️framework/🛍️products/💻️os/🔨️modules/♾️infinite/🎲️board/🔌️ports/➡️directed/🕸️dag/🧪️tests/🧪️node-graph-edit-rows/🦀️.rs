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
    let encoded: Value = serde_json::from_str(&dag_graph_edit_rows_json(&edits, None)).expect("rows json");
    assert_eq!(normalized(&encoded), normalized(&serde_json::json!({ "operations": journalled })));
    assert_eq!(dag_graph_edit_rows_json(&[], None), r#"{"operations":[]}"#, "an empty journal is no row");
}

fn node(id: &str, x: f64, y: f64) -> DagNodeSpec {
    let inputs = vec![IoPortSpec { id: "in".into(), label: "in".into(), ..Default::default() }];
    let outputs = vec![IoPortSpec { id: "out".into(), label: "out".into(), ..Default::default() }];
    let width = computation_node_width(id, &inputs, &outputs);
    let height = computation_node_height(inputs.len(), outputs.len(), false, false);
    DagNodeSpec::computation(id.into(), id, id, "emoji:🔢️".into(), inputs, outputs, false, false, x, y, width, height)
}

fn host(nodes: Vec<DagNodeSpec>) -> DagHost {
    DagHost::from_host_snapshot_without_layout(DagHostSnapshot { schema: "dag.hostDocument".into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes, edges: vec![] })
}

/// ⚖️ LAW: what moved since a press is one `move` record per distinct offset, the nodes of each in node order; a node that
/// stayed and a node the baseline does not know are not narrated, and a press that moved nothing journals nothing.
#[test]
fn moves_since_a_press_are_one_record_per_distinct_offset() {
    let mut host = host(vec![node("a", 0.0, 0.0), node("b", 100.0, 0.0), node("c", 200.0, 0.0), node("d", 300.0, 0.0)]);
    let baseline = host.node_positions();
    host.journal_moves_since("node-drag:1", &baseline).expect("unchanged gesture fits the journal");
    assert!(host.take_graph_edits().is_empty(), "nothing moved");
    let moved = [("a", 10.0, 5.0), ("b", 110.0, 5.0), ("c", 200.0, -40.0)];
    for (id, x, y) in moved {
        let node = host.host_snapshot.nodes.iter_mut().find(|node| node.id == id).expect("node");
        (node.x, node.y) = (x, y);
    }
    let without_d: Vec<(String, f64, f64)> = baseline.into_iter().filter(|(id, _, _)| id != "d").collect();
    host.host_snapshot.nodes.iter_mut().find(|node| node.id == "d").expect("d").x = 999.0;
    host.journal_moves_since("node-drag:2", &without_d).expect("moved gesture fits the journal");
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
    host.journal_port_insert("a".into(), DagPortSide::Input, 2);
    host.journal_port_insert("a".into(), DagPortSide::Input, 2);
    assert_eq!(host.take_journal_refusal(), None, "a duplicate is accepted and dropped");
    assert_eq!(host.take_graph_edits(), vec![DagGraphEdit::InsertPort { node_id: "a".into(), side: DagPortSide::Input, index: 2 }]);
}

/// 🪜️ `count` nodes in a row, each to be moved by its own offset — what an align of `count` nodes journals.
fn spread_and_move(count: usize) -> (DagHost, Vec<(String, f64, f64)>) {
    let mut host = host((0..count).map(|index| node(&format!("n{index}"), index as f64 * 100.0, 0.0)).collect());
    let baseline = host.node_positions();
    for (index, node) in host.host_snapshot.nodes.iter_mut().enumerate() {
        node.y = index as f64 + 1.0;
    }
    (host, baseline)
}

/// ⚖️ LAW (audit F1): a 200-node align — 200 distinct offsets — journals whole as 200 `move` rows in one answer; a gesture
/// whose rows outgrow one dispatch ([`DAG_GRAPH_EDIT_CAPACITY`]) is refused whole: nothing is journalled (not even the
/// press's port row), the refusal is answered, and restoring the baseline puts every node back — no row is ever dropped.
#[test]
fn a_two_hundred_node_gesture_journals_whole_and_an_oversized_one_is_refused_whole() {
    let (mut fits, baseline) = spread_and_move(200);
    fits.journal_moves_since("node-drag:7", &baseline).expect("200 rows fit one dispatch");
    let edits = fits.take_graph_edits();
    assert_eq!(edits.len(), 200, "one row per distinct offset");
    let answer: Value = serde_json::from_str(&dag_graph_edit_rows_json(&edits, None)).expect("rows json");
    assert_eq!(answer["operations"].as_array().map(Vec::len), Some(200));
    let (mut oversized, baseline) = spread_and_move(DAG_GRAPH_EDIT_CAPACITY);
    oversized.journal_port_insert("n0".into(), DagPortSide::Input, 0);
    let refusal = DagJournalRefusal { rows: DAG_GRAPH_EDIT_CAPACITY + 1, limit: DAG_GRAPH_EDIT_CAPACITY };
    assert_eq!(oversized.journal_moves_since("node-drag:8", &baseline), Err(refusal));
    assert!(oversized.take_graph_edits().is_empty(), "a refused gesture journals nothing");
    assert_eq!(oversized.take_journal_refusal(), Some(refusal), "the refusal is answered once");
    assert_eq!(oversized.take_journal_refusal(), None);
    oversized.restore_node_positions(&baseline);
    assert_eq!(oversized.node_positions(), baseline, "the refused gesture leaves every node where it stood");
    let refused: Value = serde_json::from_str(&dag_graph_edit_rows_json(&[], Some(refusal))).expect("refusal json");
    assert_eq!(normalized(&refused), normalized(&serde_json::json!({ "operations": [], "refused": { "rows": 257, "limit": 256 } })));
}

/// ⚖️ LAW (audit F2): the string census a bounded writer prices its credits from names every field and text value the row
/// encoder writes, in writing order.
#[test]
fn the_row_string_census_names_every_written_field_and_text() {
    let edits = vec![
        DagGraphEdit::Move { gesture_id: "node-drag:1".into(), node_ids: vec!["a".into(), "b".into()], dx: 1.0, dy: 2.0 },
        DagGraphEdit::InsertPort { node_id: "a".into(), side: DagPortSide::Output, index: 3 },
    ];
    assert_eq!(dag_graph_edit_row_strings(&edits), vec!["operation", "move", "gestureId", "node-drag:1", "nodeIds", "a", "b", "dx", "dy", "operation", "insertPort", "nodeId", "a", "side", "output", "index"]);
}
