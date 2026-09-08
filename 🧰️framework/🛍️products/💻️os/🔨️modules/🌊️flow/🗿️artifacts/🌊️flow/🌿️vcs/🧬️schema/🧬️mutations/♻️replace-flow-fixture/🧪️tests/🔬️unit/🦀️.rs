
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<ReplaceFlowFixture>(9, FlowMutation::ReplaceFlowFixture, include_str!("../../🔣️.json"));
}
