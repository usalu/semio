
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<RemoveSynapse>(5, FlowMutation::RemoveSynapse, include_str!("../../🔣️.json"));
}
