//! 🔬️ Direct-leaf contract of every Flow mutation payload: descriptor, codecs, unknown and missing fields, inverse replay.

use super::flow_direct_tests;
use crate::*;

//#region 🧪️Tests
#[test]
fn add_widget_direct_leaf_contract() {
    flow_direct_tests::assert_leaf_contract::<AddWidget>(0, FlowMutation::AddWidget, include_str!("../../🧬️schema/🧬️mutations/➕️add-widget/🔣️.json"));
}
//#endregion 🧪️Tests

//#region 🧪️Tests
#[test]
fn remove_widget_direct_leaf_contract() {
    flow_direct_tests::assert_leaf_contract::<RemoveWidget>(1, FlowMutation::RemoveWidget, include_str!("../../🧬️schema/🧬️mutations/🗑️remove-widget/🔣️.json"));
}
//#endregion 🧪️Tests

//#region 🧪️Tests
#[test]
fn move_widget_direct_leaf_contract() {
    flow_direct_tests::assert_leaf_contract::<MoveWidget>(2, FlowMutation::MoveWidget, include_str!("../../🧬️schema/🧬️mutations/↔️move-widget/🔣️.json"));
}
//#endregion 🧪️Tests

//#region 🧪️Tests
#[test]
fn change_widget_direct_leaf_contract() {
    flow_direct_tests::assert_leaf_contract::<ChangeWidget>(3, FlowMutation::ChangeWidget, include_str!("../../🧬️schema/🧬️mutations/🩹change-widget/🔣️.json"));
}
//#endregion 🧪️Tests

//#region 🧪️Tests
#[test]
fn add_synapse_direct_leaf_contract() {
    flow_direct_tests::assert_leaf_contract::<AddSynapse>(4, FlowMutation::AddSynapse, include_str!("../../🧬️schema/🧬️mutations/🔗️add-synapse/🔣️.json"));
}
//#endregion 🧪️Tests

//#region 🧪️Tests
#[test]
fn remove_synapse_direct_leaf_contract() {
    flow_direct_tests::assert_leaf_contract::<RemoveSynapse>(5, FlowMutation::RemoveSynapse, include_str!("../../🧬️schema/🧬️mutations/✂️remove-synapse/🔣️.json"));
}
//#endregion 🧪️Tests

//#region 🧪️Tests
#[test]
fn move_synapse_direct_leaf_contract() {
    flow_direct_tests::assert_leaf_contract::<MoveSynapse>(6, FlowMutation::MoveSynapse, include_str!("../../🧬️schema/🧬️mutations/🔀️move-synapse/🔣️.json"));
}
//#endregion 🧪️Tests

//#region 🧪️Tests
#[test]
fn change_synapse_direct_leaf_contract() {
    flow_direct_tests::assert_leaf_contract::<ChangeSynapse>(7, FlowMutation::ChangeSynapse, include_str!("../../🧬️schema/🧬️mutations/🔄change-synapse/🔣️.json"));
}
//#endregion 🧪️Tests

//#region 🧪️Tests
#[test]
fn change_layout_direct_leaf_contract() {
    flow_direct_tests::assert_leaf_contract::<ChangeLayout>(8, FlowMutation::ChangeLayout, include_str!("../../🧬️schema/🧬️mutations/📐️change-layout/🔣️.json"));
}
//#endregion 🧪️Tests
