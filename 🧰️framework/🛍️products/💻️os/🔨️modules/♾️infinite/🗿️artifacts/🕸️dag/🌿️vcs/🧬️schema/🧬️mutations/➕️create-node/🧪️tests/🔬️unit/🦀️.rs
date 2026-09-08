
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<CreateNode>(0, DagMutation::CreateNode, include_str!("../../🔣️.json"));
}
