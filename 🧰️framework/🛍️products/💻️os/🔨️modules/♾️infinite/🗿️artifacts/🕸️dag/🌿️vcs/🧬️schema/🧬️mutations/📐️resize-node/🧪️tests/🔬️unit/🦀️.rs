
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<ResizeNode>(5, DagMutation::ResizeNode, include_str!("../../🔣️.json"));
}
