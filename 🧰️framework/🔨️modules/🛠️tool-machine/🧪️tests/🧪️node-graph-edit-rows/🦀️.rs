//! 🔬️ The shared `nodeGraphEdit` row decoder against the framework's committed row vocabulary fixture: every accepted row
//! decodes to its typed record, every refused row (whole fixtures, ambient selections, absolute moves, extra fields, empty
//! or repeated ids, text numbers, unknown sides, non-integer indices, empty deletes, unknown operations) is refused.

use super::*;
use serde_json::Value;

const ROWS: &str = include_str!("../../🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

fn dsl(value: &Value) -> protocol::DslValue {
    match value {
        Value::Null => protocol::DslValue::Null,
        Value::Bool(flag) => protocol::DslValue::Bool(*flag),
        Value::Number(number) => protocol::DslValue::float(number.as_f64().expect("fixture numbers are finite")),
        Value::String(text) => protocol::DslValue::String(text.clone()),
        Value::Array(items) => protocol::DslValue::Array(items.iter().map(dsl).collect()),
        Value::Object(entries) => protocol::DslValue::object(entries.iter().map(|(key, value)| (key.clone(), dsl(value)))),
    }
}

fn cases(group: &str) -> Vec<(String, protocol::DslValue)> {
    let fixture: Value = serde_json::from_str(ROWS).expect("the row fixture parses");
    fixture[group].as_array().expect("a case group").iter().map(|case| (case["id"].as_str().expect("case id").to_string(), dsl(&case["row"]))).collect()
}

/// ✅️ Every accepted row decodes, to exactly the record the row names.
#[test]
fn every_accepted_row_decodes_to_its_record() {
    let decoded: Vec<(String, NodeGraphEditRow)> = cases("accepted").into_iter().map(|(id, row)| (id.clone(), NodeGraphEditRow::from_row(&row).unwrap_or_else(|reason| panic!("accepted row {id} refused: {reason}")))).collect();
    let by_id = |id: &str| decoded.iter().find(|(case, _)| case == id).map(|(_, row)| row.clone()).expect("fixture case");
    assert_eq!(by_id("connect"), NodeGraphEditRow::Connect { source_node_id: "slider".into(), source_port_id: "number".into(), target_node_id: "add".into(), target_port_id: "b".into() });
    assert_eq!(by_id("disconnect"), NodeGraphEditRow::Disconnect { synapse_id: "s1".into() });
    assert_eq!(by_id("move"), NodeGraphEditRow::Move(NodeDragRecord { gesture_id: "node-drag:3".into(), node_ids: vec!["add".into(), "slider".into()], dx: 284.5, dy: -48.0 }));
    assert_eq!(by_id("set-slider"), NodeGraphEditRow::SetSlider { widget_id: "slider".into(), value: 6.5 });
    assert_eq!(by_id("insert-input-port"), NodeGraphEditRow::InsertPort { node_id: "add".into(), side: NodePortSide::Input, index: 2 });
    assert_eq!(by_id("insert-output-port"), NodeGraphEditRow::InsertPort { node_id: "add".into(), side: NodePortSide::Output, index: 0 });
    assert_eq!(by_id("delete-nodes-and-wires"), NodeGraphEditRow::Delete { node_ids: vec!["add".into()], synapse_ids: vec!["s1".into()] });
    assert_eq!(by_id("delete-wires-only"), NodeGraphEditRow::Delete { node_ids: Vec::new(), synapse_ids: vec!["s1".into(), "s2".into()] });
}

/// 🚫️ Every refused row is refused, and so is any batch that carries it.
#[test]
fn every_refused_row_is_refused_alone_and_in_a_batch() {
    for (id, row) in cases("refused") {
        assert!(NodeGraphEditRow::from_row(&row).is_err(), "refused row {id} decoded");
        let accepted = cases("accepted").into_iter().map(|(_, row)| row);
        let batch = protocol::DslValue::object([("operations".to_string(), protocol::DslValue::Array(accepted.chain([row]).collect()))]);
        assert!(node_graph_edit_rows(&batch).is_err(), "a batch carrying refused row {id} decoded");
    }
}

/// 🧾️ The arguments are closed: the rows plus the scrub press fields decode (an empty abort batch too); a foreign root
/// field, a missing `operations` or more than [`NODE_GRAPH_EDIT_MAX_ROWS`] rows refuse the batch.
#[test]
fn the_arguments_are_a_closed_bounded_batch() {
    let rows: Vec<protocol::DslValue> = cases("accepted").into_iter().map(|(_, row)| row).collect();
    let args = |fields: Vec<(&str, protocol::DslValue)>| protocol::DslValue::object(fields.into_iter().map(|(key, value)| (key.to_string(), value)));
    assert_eq!(node_graph_edit_rows(&args(vec![("operations", protocol::DslValue::Array(rows.clone()))])).expect("accepted batch").len(), rows.len());
    let abort = args(vec![("operations", protocol::DslValue::Array(Vec::new())), (SCRUB_GESTURE_ARG, protocol::DslValue::String("slider:7".into())), (SCRUB_ABORT_ARG, protocol::DslValue::String("blur".into()))]);
    assert_eq!(node_graph_edit_rows(&abort).expect("an abort batch"), Vec::new());
    assert!(node_graph_edit_rows(&args(vec![("operations", protocol::DslValue::Array(rows.clone())), ("hostSnapshotChanged", protocol::DslValue::Bool(true))])).is_err());
    assert!(node_graph_edit_rows(&args(vec![(SCRUB_GESTURE_ARG, protocol::DslValue::String("slider:7".into()))])).is_err());
    let flood = vec![rows[1].clone(); NODE_GRAPH_EDIT_MAX_ROWS + 1];
    assert!(node_graph_edit_rows(&args(vec![("operations", protocol::DslValue::Array(flood))])).is_err());
}

/// 🛠️ `node_drag_emit` is the ONE emission of a released drag: with an authoring seed ONE transaction of the leaves under the
/// tool `<app>#<verb>`, without one the leaves plainly, with no leaves nothing; `authoring_clock` carries its logical tick.
#[test]
fn a_node_drag_emits_one_transaction_plain_leaves_or_nothing() {
    match node_drag_emit("s.test@1/*#editor", "nodeGraphEdit", "seed-a", "node-drag:1", vec![1_u8, 2]) {
        NodeDragEmit::Commit(transaction, leaves) => {
            assert!(transaction.id.starts_with("tx-"), "{transaction:?}");
            assert_eq!(transaction.tool, "s.test@1/*#editor#nodeGraphEdit");
            assert_eq!(leaves, vec![1, 2]);
        }
        other => panic!("a seeded drag is one transaction: {other:?}"),
    }
    assert_eq!(node_drag_emit("s.test@1/*#editor", "nodeGraphEdit", "", "node-drag:2", vec![3_u8]), NodeDragEmit::Plain(vec![3]));
    assert_eq!(node_drag_emit::<u8>("s.test@1/*#editor", "nodeGraphEdit", "seed-a", "node-drag:3", Vec::new()), NodeDragEmit::Nothing);
    assert_eq!((authoring_clock(7).actor, authoring_clock(7).logical), (0, 7));
}
