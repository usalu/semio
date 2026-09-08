
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::dag_direct_tests::assert_leaf_contract::<ReplaceNodeKind>(9, DagMutation::ReplaceNodeKind, include_str!("../../🔣️.json"));
}
