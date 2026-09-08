
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<ChangeNodeName>(3, DagMutation::ChangeNodeName, include_str!("../../🔣️.json"));
}
