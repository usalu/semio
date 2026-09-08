
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<RenameNode>(2, DagMutation::RenameNode, include_str!("../../🔣️.json"));
}
