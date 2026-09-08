
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<ReplaceNodeProperties>(10, DagMutation::ReplaceNodeProperties, include_str!("../../🔣️.json"));
}
