
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<AddWidget>(0, FlowMutation::AddWidget, include_str!("../../🔣️.json"));
}
