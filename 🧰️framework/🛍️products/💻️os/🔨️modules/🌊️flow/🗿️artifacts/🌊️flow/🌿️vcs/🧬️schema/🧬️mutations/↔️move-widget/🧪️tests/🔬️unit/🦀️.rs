
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<MoveWidget>(2, FlowMutation::MoveWidget, include_str!("../../🔣️.json"));
}
