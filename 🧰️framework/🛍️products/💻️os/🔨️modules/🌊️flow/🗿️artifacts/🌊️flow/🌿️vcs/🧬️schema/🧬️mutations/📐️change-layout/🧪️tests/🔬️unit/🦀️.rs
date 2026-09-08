
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<ChangeLayout>(8, FlowMutation::ChangeLayout, include_str!("../../🔣️.json"));
}
