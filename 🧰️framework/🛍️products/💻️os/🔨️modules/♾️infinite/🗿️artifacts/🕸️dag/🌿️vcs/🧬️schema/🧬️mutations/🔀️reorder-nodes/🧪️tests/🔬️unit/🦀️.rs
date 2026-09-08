
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<ReorderNodes>(11, DagMutation::ReorderNodes, include_str!("../../🔣️.json"));
}
