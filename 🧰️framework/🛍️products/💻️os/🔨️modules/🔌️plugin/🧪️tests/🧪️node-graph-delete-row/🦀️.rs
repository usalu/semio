//! 🗑️ Node-graph context-menu delete laws (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §13.3): the delete row
//! names the selection it deletes by id as the ONE `delete {nodeIds, synapseIds}` row of the shared node-graph edit
//! vocabulary — never the ambient `deleteSelection` row the language-agnostic fixture refuses — and an empty selection offers
//! no delete at all.
//! @see 🧰️framework/🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json

use super::*;
use semio_framework_tool_machine::{node_graph_edit_rows, NodeGraphEditRow};

const NODE_GRAPH_EDIT_ROWS_FIXTURE_JSON: &str = include_str!("../../../../../../🔨️modules/🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json");

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

/// ⚖️ LAW: via `nodeGraphEdit` the menu row carries the selection's node and edge ids as one shared `delete` row; the direct
/// verb carries no row; an empty selection offers nothing; and the ambient row stays refused by the shared decoder.
#[test]
fn a_menu_delete_names_its_selection_as_one_delete_row() {
    let spec = node_graph_delete_selection_spec("Delete selection", false, &ids(&["a", "b"]), &ids(&["s1"]), NodeGraphDeleteDispatch::ViaNodeGraphEdit).expect("a selection offers delete");
    assert_eq!((spec.id.as_str(), spec.action.as_deref(), spec.destructive), ("delete-selection", Some("nodeGraphEdit"), Some(true)));
    let args = spec.args.expect("the delete row");
    assert_eq!(node_graph_edit_rows(&args), Ok(vec![NodeGraphEditRow::Delete { node_ids: ids(&["a", "b"]), synapse_ids: ids(&["s1"]) }]));
    let edges_only = node_graph_delete_selection_spec("Delete selection", true, &[], &ids(&["s1", "s2"]), NodeGraphDeleteDispatch::ViaNodeGraphEdit).expect("edges alone offer delete");
    assert_eq!(node_graph_edit_rows(&edges_only.args.expect("the delete row")), Ok(vec![NodeGraphEditRow::Delete { node_ids: Vec::new(), synapse_ids: ids(&["s1", "s2"]) }]));
    let direct = node_graph_delete_selection_spec("Delete selection", false, &ids(&["a"]), &[], NodeGraphDeleteDispatch::Direct).expect("a selection offers delete");
    assert_eq!((direct.action.as_deref(), direct.args.is_none()), (Some("deleteSelection"), true));
    for dispatch in [NodeGraphDeleteDispatch::ViaNodeGraphEdit, NodeGraphDeleteDispatch::Direct] {
        assert!(node_graph_delete_selection_spec("Delete selection", false, &[], &[], dispatch).is_none(), "an empty selection offers no delete");
    }
    let fixture: serde_json::Value = serde_json::from_str(NODE_GRAPH_EDIT_ROWS_FIXTURE_JSON).expect("node-graph edit rows fixture");
    let ambient = fixture["refused"].as_array().expect("refused rows").iter().find(|case| case["id"] == "ambient-selection-delete").expect("the ambient delete row");
    assert!(node_graph_edit_rows(&DslValue::from(&serde_json::json!({ "operations": [ambient["row"].clone()] }))).is_err(), "the ambient row is refused");
}
