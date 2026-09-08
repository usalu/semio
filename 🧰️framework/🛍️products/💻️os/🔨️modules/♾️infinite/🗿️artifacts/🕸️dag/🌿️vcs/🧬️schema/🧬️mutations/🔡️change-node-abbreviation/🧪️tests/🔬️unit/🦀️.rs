
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<ChangeNodeAbbreviation>(7, DagMutation::ChangeNodeAbbreviation, include_str!("../../🔣️.json"));
}
