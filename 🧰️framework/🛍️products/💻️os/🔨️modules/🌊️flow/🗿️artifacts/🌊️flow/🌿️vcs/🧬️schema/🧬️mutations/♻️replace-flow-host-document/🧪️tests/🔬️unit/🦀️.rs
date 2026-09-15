
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<ReplaceFlowHostDocument>(9, FlowMutation::ReplaceFlowHostDocument, include_str!("../../🔣️.json"));
}
