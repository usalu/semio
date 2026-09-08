
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<RemoveWidget>(1, FlowMutation::RemoveWidget, include_str!("../../🔣️.json"));
}
