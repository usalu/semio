
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<ChangeNodeIcon>(6, DagMutation::ChangeNodeIcon, include_str!("../../🔣️.json"));
}
