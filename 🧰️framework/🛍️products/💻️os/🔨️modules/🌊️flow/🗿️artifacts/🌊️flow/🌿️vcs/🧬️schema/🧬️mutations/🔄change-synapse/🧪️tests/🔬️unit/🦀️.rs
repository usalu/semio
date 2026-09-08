
use super::*;
#[test]
fn direct_leaf_contract() {
    super::super::super::flow_direct_tests::assert_leaf_contract::<ChangeSynapse>(7, FlowMutation::ChangeSynapse, include_str!("../../🔣️.json"));
}
