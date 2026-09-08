
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<ChangeNodeOperatorKind>(8, DagMutation::ChangeNodeOperatorKind, include_str!("../../🔣️.json"));
}
