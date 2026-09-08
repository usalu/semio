
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<MoveSynapse>(6, FlowMutation::MoveSynapse, include_str!("../../🔣️.json"));
}
