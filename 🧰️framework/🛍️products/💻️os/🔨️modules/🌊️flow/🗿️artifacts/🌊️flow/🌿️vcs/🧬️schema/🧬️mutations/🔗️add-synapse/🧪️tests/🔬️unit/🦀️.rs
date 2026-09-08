
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<AddSynapse>(4, FlowMutation::AddSynapse, include_str!("../../🔣️.json"));
}
