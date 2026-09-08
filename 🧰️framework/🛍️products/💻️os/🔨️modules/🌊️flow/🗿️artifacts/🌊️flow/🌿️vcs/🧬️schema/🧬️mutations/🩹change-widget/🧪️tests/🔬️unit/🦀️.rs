
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<ChangeWidget>(3, FlowMutation::ChangeWidget, include_str!("../../🔣️.json"));
}
