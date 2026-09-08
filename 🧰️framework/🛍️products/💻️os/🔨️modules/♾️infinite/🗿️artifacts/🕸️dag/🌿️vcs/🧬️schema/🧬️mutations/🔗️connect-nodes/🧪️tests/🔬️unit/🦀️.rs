
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<ConnectNodes>(12, DagMutation::ConnectNodes, include_str!("../../🔣️.json"));
}
