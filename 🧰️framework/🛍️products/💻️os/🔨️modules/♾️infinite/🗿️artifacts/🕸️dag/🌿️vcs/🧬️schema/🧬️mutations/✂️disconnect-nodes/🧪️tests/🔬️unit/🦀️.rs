
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<DisconnectNodes>(13, DagMutation::DisconnectNodes, include_str!("../../🔣️.json"));
}
