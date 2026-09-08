
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<MoveNode>(4, DagMutation::MoveNode, include_str!("../../🔣️.json"));
}
