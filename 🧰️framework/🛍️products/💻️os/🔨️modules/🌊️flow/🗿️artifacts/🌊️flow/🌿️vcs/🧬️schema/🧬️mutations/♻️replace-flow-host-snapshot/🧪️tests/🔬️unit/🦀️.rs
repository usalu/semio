
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<ReplaceFlowHostSnapshot>(9, FlowMutation::ReplaceFlowHostSnapshot, include_str!("../../🔣️.json"));
}
