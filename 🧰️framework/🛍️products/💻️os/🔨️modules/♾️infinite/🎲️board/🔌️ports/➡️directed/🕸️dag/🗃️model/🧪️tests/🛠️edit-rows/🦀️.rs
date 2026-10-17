//! 🧪️ dag-model tests (🛠️edit-rows): compiled against the `testkit` surface of the board crates.

use semio_framework_os_infinite_board_schema::field;
use semio_framework_os_infinite_board_dag_model::{dag_graph_edit_row_strings, dag_graph_edit_rows_json, DagGraphEdit, DagPortSide};
use semio_framework_test_fixtures::fixture_text;
use serde_json::Value;
use semio_framework_os_infinite_directed_presentation::*;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

fn fixture() -> Value {
    serde_json::from_str(fixture_text!("../../../../../../../../../../../🔨️modules/🛠️tool-machine/🧪️tests/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json")).expect("node-graph edit rows fixture")
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
