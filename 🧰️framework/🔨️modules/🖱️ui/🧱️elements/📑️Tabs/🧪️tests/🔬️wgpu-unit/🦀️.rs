//! ⌨️ `Tabs`' roving-focus key table and its declarative tree, against React's `moveTabFocus`
//! (`🧱️elements/📑️Tabs/🟦️.tsx:163-181`) — ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W2k.

use super::*;

fn rows() -> Vec<TabRow> {
    vec![TabRow::new("a", "Alpha"), TabRow::disabled("b", "Beta"), TabRow::new("c", "Gamma")]
}

fn action() -> ActionDescriptor {
    ActionDescriptor { controller_id: "admin".into(), action: "setTab".into(), args: None }
}

#[test]
fn the_step_pair_follows_the_orientation_and_mirrors_only_the_horizontal_one() {
    assert_eq!(tabs_step_keys(TabsOrientation::Horizontal, FlowInline::Ltr), ("ArrowLeft", "ArrowRight"));
    assert_eq!(tabs_step_keys(TabsOrientation::Horizontal, FlowInline::Rtl), ("ArrowRight", "ArrowLeft"));
    assert_eq!(tabs_step_keys(TabsOrientation::Vertical, FlowInline::Ltr), ("ArrowUp", "ArrowDown"));
    assert_eq!(tabs_step_keys(TabsOrientation::Vertical, FlowInline::Rtl), ("ArrowUp", "ArrowDown"), "a vertical tablist carries no flow direction");
}

#[test]
fn stepping_skips_disabled_triggers_and_wraps_at_both_ends() {
    let rows = rows();
    let next = |from: usize, key: &str| tabs_key(&rows, from, key, TabsOrientation::Horizontal, TabsActivationMode::Automatic, FlowInline::Ltr);
    assert_eq!(next(0, "ArrowRight"), TabsKey::Activate(2), "index 1 is disabled, so React's enabled-only query skips it");
    assert_eq!(next(2, "ArrowRight"), TabsKey::Activate(0), "forward wraps");
    assert_eq!(next(0, "ArrowLeft"), TabsKey::Activate(2), "backward wraps");
    assert_eq!(next(2, "Home"), TabsKey::Activate(0));
    assert_eq!(next(0, "End"), TabsKey::Activate(2), "End is the last ENABLED trigger, not the last row");
}

#[test]
fn an_rtl_tablist_steps_the_other_way_for_the_same_key() {
    let rows = rows();
    assert_eq!(tabs_key(&rows, 0, "ArrowRight", TabsOrientation::Horizontal, TabsActivationMode::Automatic, FlowInline::Rtl), TabsKey::Activate(2), "under rtl ArrowRight is the PREVIOUS key, which wraps from 0 to the last enabled");
    assert_eq!(tabs_key(&rows, 0, "ArrowLeft", TabsOrientation::Horizontal, TabsActivationMode::Automatic, FlowInline::Rtl), TabsKey::Activate(2));
}

#[test]
fn manual_activation_moves_focus_without_selecting_and_selects_on_enter_or_space() {
    let rows = rows();
    assert_eq!(tabs_key(&rows, 0, "ArrowRight", TabsOrientation::Horizontal, TabsActivationMode::Manual, FlowInline::Ltr), TabsKey::Focus(2));
    assert_eq!(tabs_key(&rows, 2, "Enter", TabsOrientation::Horizontal, TabsActivationMode::Manual, FlowInline::Ltr), TabsKey::Activate(2));
    assert_eq!(tabs_key(&rows, 2, " ", TabsOrientation::Horizontal, TabsActivationMode::Manual, FlowInline::Ltr), TabsKey::Activate(2));
    assert_eq!(tabs_key(&rows, 1, "Enter", TabsOrientation::Horizontal, TabsActivationMode::Manual, FlowInline::Ltr), TabsKey::Ignored, "a disabled trigger cannot be activated");
    assert_eq!(tabs_key(&rows, 0, "Enter", TabsOrientation::Horizontal, TabsActivationMode::Automatic, FlowInline::Ltr), TabsKey::Ignored, "under Automatic, Enter is not a tablist key at all");
}

#[test]
fn a_tablist_with_no_enabled_trigger_or_an_unclaimed_key_reports_ignored() {
    let all_disabled = vec![TabRow::disabled("a", "Alpha")];
    assert_eq!(tabs_key(&all_disabled, 0, "ArrowRight", TabsOrientation::Horizontal, TabsActivationMode::Automatic, FlowInline::Ltr), TabsKey::Ignored);
    assert_eq!(tabs_key(&[], 0, "ArrowRight", TabsOrientation::Horizontal, TabsActivationMode::Automatic, FlowInline::Ltr), TabsKey::Ignored);
    assert_eq!(tabs_key(&rows(), 0, "ArrowUp", TabsOrientation::Horizontal, TabsActivationMode::Automatic, FlowInline::Ltr), TabsKey::Ignored, "a horizontal tablist does not claim the vertical pair");
    assert_eq!(tabs_key(&rows(), 1, "ArrowRight", TabsOrientation::Horizontal, TabsActivationMode::Automatic, FlowInline::Ltr), TabsKey::Ignored, "focus parked on a disabled trigger is not in the enabled sequence");
}

#[test]
fn the_built_tree_carries_one_trigger_per_row_with_the_selected_and_disabled_presence() {
    let tree = build_tabs("admin.tabs", &rows(), "c", TabsOrientation::Horizontal, None, action());
    let UiNode::Stack(root) = &tree else { panic!("a tablist is a Stack, got {tree:?}") };
    assert_eq!(root.direction, "column", "the list and the panel stack vertically");
    assert_eq!(root.children.len(), 2, "the tablist plus exactly ONE panel — React renders no inactive TabsContent");
    let UiNode::Stack(list) = &root.children[0] else { panic!("the first child is the tablist") };
    assert_eq!(list.direction, "row");
    assert_eq!(list.children.len(), 3);
    let presences: Vec<(UiState, bool)> = list
        .children
        .iter()
        .map(|child| {
            let UiNode::Button(button) = child else { panic!("a trigger is a Button") };
            (button.presence.state, button.presence.selected)
        })
        .collect();
    assert_eq!(presences, vec![(UiState::Normal, false), (UiState::Disabled, false), (UiState::Normal, true)], "React's `data-state=\"active\"` is presence.selected and its `disabled` is UiState::Disabled");
    let UiNode::Button(first) = &list.children[0] else { panic!("a trigger is a Button") };
    assert_eq!(first.id.as_deref(), Some("admin.tabs.trigger.a"));
    assert_eq!(first.action.controller_id, "admin");
    assert_eq!(first.action.args, Some(dsl::DslValue::Object(vec![("value".to_string(), dsl::DslValue::String("a".to_string()))])), "a click carries its own value, so one verb serves every trigger");
}

#[test]
fn a_vertical_tablist_stacks_its_triggers_the_other_way() {
    let tree = build_tabs("t", &rows(), "a", TabsOrientation::Vertical, None, action());
    let UiNode::Stack(root) = &tree else { panic!("a tablist is a Stack") };
    let UiNode::Stack(list) = &root.children[0] else { panic!("the first child is the tablist") };
    assert_eq!(list.direction, "column");
}
