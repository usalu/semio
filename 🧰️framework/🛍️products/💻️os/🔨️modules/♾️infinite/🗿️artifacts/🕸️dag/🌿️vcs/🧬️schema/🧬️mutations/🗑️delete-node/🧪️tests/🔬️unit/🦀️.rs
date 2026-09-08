
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<DeleteNode>(1, DagMutation::DeleteNode, include_str!("../../🔣️.json"));
}
