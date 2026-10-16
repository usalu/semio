//! 🧪️ dag-host tests (🛠️edit-rows): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_schema::refusal;
use semio_framework_os_infinite_board_dag_model::{computation_node_height, computation_node_width, DAG_GRAPH_EDIT_CAPACITY, dag_graph_edit_rows_json, DagGraphEdit, DagJournalRefusal, DagPortSide};
use semio_framework_os_infinite_board_dag_host::DagHost;
use serde_json::Value;
use semio_framework_os_infinite_directed_presentation::*;

/// 🔢️ `value` with every number as `f64`, so an integral JSON number and its float spelling compare equal.
fn normalized(value: &Value) -> Value {
    match value {
        Value::Number(number) => Value::from(number.as_f64().expect("finite number")),
        Value::Array(items) => Value::Array(items.iter().map(normalized).collect()),
        Value::Object(fields) => Value::Object(fields.iter().map(|(key, value)| (key.clone(), normalized(value))).collect()),
        other => other.clone(),
    }
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
    let baseline = host.graph.node_positions();
    host.graph.journal_moves_since("node-drag:1", &baseline).expect("unchanged gesture fits the journal");
    assert!(host.graph.take_graph_edits().is_empty(), "nothing moved");
    let moved = [("a", 10.0, 5.0), ("b", 110.0, 5.0), ("c", 200.0, -40.0)];
    for (id, x, y) in moved {
        let node = host.graph.host_snapshot.nodes.iter_mut().find(|node| node.id == id).expect("node");
        (node.x, node.y) = (x, y);
    }
    let without_d: Vec<(String, f64, f64)> = baseline.into_iter().filter(|(id, _, _)| id != "d").collect();
    host.graph.host_snapshot.nodes.iter_mut().find(|node| node.id == "d").expect("d").x = 999.0;
    host.graph.journal_moves_since("node-drag:2", &without_d).expect("moved gesture fits the journal");
    assert_eq!(
        host.graph.take_graph_edits(),
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
    host.graph.journal_port_insert("a".into(), DagPortSide::Input, 2);
    host.graph.journal_port_insert("a".into(), DagPortSide::Input, 2);
    assert_eq!(host.graph.take_journal_refusal(), None, "a duplicate is accepted and dropped");
    assert_eq!(host.graph.take_graph_edits(), vec![DagGraphEdit::InsertPort { node_id: "a".into(), side: DagPortSide::Input, index: 2 }]);
}

/// 🪜️ `count` nodes in a row, each to be moved by its own offset — what an align of `count` nodes journals.
fn spread_and_move(count: usize) -> (DagHost, Vec<(String, f64, f64)>) {
    let mut host = host((0..count).map(|index| node(&format!("n{index}"), index as f64 * 100.0, 0.0)).collect());
    let baseline = host.graph.node_positions();
    for (index, node) in host.graph.host_snapshot.nodes.iter_mut().enumerate() {
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
    fits.graph.journal_moves_since("node-drag:7", &baseline).expect("200 rows fit one dispatch");
    let edits = fits.graph.take_graph_edits();
    assert_eq!(edits.len(), 200, "one row per distinct offset");
    let answer: Value = serde_json::from_str(&dag_graph_edit_rows_json(&edits, None)).expect("rows json");
    assert_eq!(answer["operations"].as_array().map(Vec::len), Some(200));
    let (mut oversized, baseline) = spread_and_move(DAG_GRAPH_EDIT_CAPACITY);
    oversized.graph.journal_port_insert("n0".into(), DagPortSide::Input, 0);
    let refusal = DagJournalRefusal { rows: DAG_GRAPH_EDIT_CAPACITY + 1, limit: DAG_GRAPH_EDIT_CAPACITY };
    assert_eq!(oversized.graph.journal_moves_since("node-drag:8", &baseline), Err(refusal));
    assert!(oversized.graph.take_graph_edits().is_empty(), "a refused gesture journals nothing");
    assert_eq!(oversized.graph.take_journal_refusal(), Some(refusal), "the refusal is answered once");
    assert_eq!(oversized.graph.take_journal_refusal(), None);
    oversized.graph.restore_node_positions(&baseline);
    assert_eq!(oversized.graph.node_positions(), baseline, "the refused gesture leaves every node where it stood");
    let refused: Value = serde_json::from_str(&dag_graph_edit_rows_json(&[], Some(refusal))).expect("refusal json");
    assert_eq!(normalized(&refused), normalized(&serde_json::json!({ "operations": [], "refused": { "rows": 257, "limit": 256 } })));
}
