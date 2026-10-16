//! 🧾️ Direct leaf contract of every DAG mutation: the committed descriptor, wire witness, codecs and exact inverse of each leaf.
use super::dag_direct_tests::{assert_leaf_contract, read_vcs_fixture};
use super::{ChangeNodeAbbreviation, ChangeNodeIcon, ChangeNodeName, ChangeNodeOperatorKind, ConnectNodes, CreateNode, DagMutation, DeleteNode, DisconnectNodes, MoveNode, RenameNode, ReorderNodes, ReplaceNodeKind, ReplaceNodeProperties, ResizeNode};

#[test]
fn direct_leaf_contract_create_node() {
    assert_leaf_contract::<CreateNode>(0, DagMutation::CreateNode, &read_vcs_fixture("🧬️schema/🧬️mutations/➕️create-node/🔣️.json"));
}

#[test]
fn direct_leaf_contract_delete_node() {
    assert_leaf_contract::<DeleteNode>(1, DagMutation::DeleteNode, &read_vcs_fixture("🧬️schema/🧬️mutations/🗑️delete-node/🔣️.json"));
}

#[test]
fn direct_leaf_contract_rename_node() {
    assert_leaf_contract::<RenameNode>(2, DagMutation::RenameNode, &read_vcs_fixture("🧬️schema/🧬️mutations/✏️rename-node/🔣️.json"));
}

#[test]
fn direct_leaf_contract_change_node_name() {
    assert_leaf_contract::<ChangeNodeName>(3, DagMutation::ChangeNodeName, &read_vcs_fixture("🧬️schema/🧬️mutations/🔤️change-node-name/🔣️.json"));
}

#[test]
fn direct_leaf_contract_move_node() {
    assert_leaf_contract::<MoveNode>(4, DagMutation::MoveNode, &read_vcs_fixture("🧬️schema/🧬️mutations/↔️move-node/🔣️.json"));
}

#[test]
fn direct_leaf_contract_resize_node() {
    assert_leaf_contract::<ResizeNode>(5, DagMutation::ResizeNode, &read_vcs_fixture("🧬️schema/🧬️mutations/📐️resize-node/🔣️.json"));
}

#[test]
fn direct_leaf_contract_change_node_icon() {
    assert_leaf_contract::<ChangeNodeIcon>(6, DagMutation::ChangeNodeIcon, &read_vcs_fixture("🧬️schema/🧬️mutations/🖼️change-node-icon/🔣️.json"));
}

#[test]
fn direct_leaf_contract_change_node_abbreviation() {
    assert_leaf_contract::<ChangeNodeAbbreviation>(7, DagMutation::ChangeNodeAbbreviation, &read_vcs_fixture("🧬️schema/🧬️mutations/🔡️change-node-abbreviation/🔣️.json"));
}

#[test]
fn direct_leaf_contract_change_node_operator_kind() {
    assert_leaf_contract::<ChangeNodeOperatorKind>(8, DagMutation::ChangeNodeOperatorKind, &read_vcs_fixture("🧬️schema/🧬️mutations/🧮️change-node-operator-kind/🔣️.json"));
}

#[test]
fn direct_leaf_contract_replace_node_kind() {
    assert_leaf_contract::<ReplaceNodeKind>(9, DagMutation::ReplaceNodeKind, &read_vcs_fixture("🧬️schema/🧬️mutations/🔁️replace-node-kind/🔣️.json"));
}

#[test]
fn direct_leaf_contract_replace_node_properties() {
    assert_leaf_contract::<ReplaceNodeProperties>(10, DagMutation::ReplaceNodeProperties, &read_vcs_fixture("🧬️schema/🧬️mutations/🗃️replace-node-properties/🔣️.json"));
}

#[test]
fn direct_leaf_contract_reorder_nodes() {
    assert_leaf_contract::<ReorderNodes>(11, DagMutation::ReorderNodes, &read_vcs_fixture("🧬️schema/🧬️mutations/🔀️reorder-nodes/🔣️.json"));
}

#[test]
fn direct_leaf_contract_connect_nodes() {
    assert_leaf_contract::<ConnectNodes>(12, DagMutation::ConnectNodes, &read_vcs_fixture("🧬️schema/🧬️mutations/🔗️connect-nodes/🔣️.json"));
}

#[test]
fn direct_leaf_contract_disconnect_nodes() {
    assert_leaf_contract::<DisconnectNodes>(13, DagMutation::DisconnectNodes, &read_vcs_fixture("🧬️schema/🧬️mutations/✂️disconnect-nodes/🔣️.json"));
}
