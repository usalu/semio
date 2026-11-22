// 🔘️ Segmented select routing laws: a click picks the segment under the pointer and the inline arrows, Home and End move the radio choice, with no popup — the fixture's `ToggleGroup` rules through the live event router.

fn segmented_select_ui(value: &str) -> UiNode {
    let UiNode::Select(mut select) = select_ui("seg", value) else { unreachable!() };
    select.appearance = ui_contract::SelectAppearance::Segmented;
    select.items = ["a", "b", "c"].iter().map(|key| UiSelectItem { value: (*key).into(), label: Label::data(key.to_uppercase()) }).collect();
    UiNode::Select(select)
}

fn change_payloads(commands: &[UiCommand]) -> Vec<String> {
    commands.iter().filter_map(|cmd| if let UiCommand::App { intent, .. } = cmd { intent.payload().map(|payload| format!("{payload:?}")) } else { None }).collect()
}

#[test]
fn segmented_select_keys_follow_the_toggle_group_fixture_through_the_router() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/🔘️segmented-select/🔣️.json")).expect("segmented select fixture");
    for case in fixture["keys"].as_array().unwrap().iter().filter(|case| case["disabled"].as_array().unwrap().is_empty() && !case["rtl"].as_bool().unwrap()) {
        let values = ["a", "b", "c"];
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let select = leaf(&mut tree, Some(root), 1, segmented_select_ui(values[case["current"].as_u64().unwrap() as usize]), (0.0, 0.0, 99.0, 30.0));
        let mut router = EventRouter::new("main");
        router.focus.set_focus(&mut tree, Some(select), true);
        let commands = router.dispatch(&mut tree, root, &key(case["key"].as_str().unwrap()));
        let fired = change_payloads(&commands);
        match case["next"].as_u64() {
            Some(next) => assert!(fired.len() == 1 && fired[0].contains(values[next as usize]), "{case}: {fired:?}"),
            None => assert!(fired.is_empty(), "{case}: {fired:?}"),
        }
        assert!(!tree.node(select).unwrap().state.open, "a segmented select never opens a popup: {case}");
    }
}

#[test]
fn clicking_a_segment_picks_it_and_clicking_the_chosen_one_changes_nothing() {
    let click = |value: &str, x: f32| {
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
        let select = leaf(&mut tree, Some(root), 1, segmented_select_ui(value), (0.0, 0.0, 99.0, 30.0));
        let mut router = EventRouter::new("main");
        let mut commands = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
        commands.extend(router.dispatch(&mut tree, root, &UiEvent::PointerUp { x, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() }));
        assert!(!tree.node(select).unwrap().state.open, "a click never opens a popup on a segmented select");
        change_payloads(&commands)
    };
    let picked = click("a", 80.0);
    assert!(picked.len() == 1 && picked[0].contains('c'), "the third segment is picked: {picked:?}");
    assert!(click("c", 80.0).is_empty(), "the chosen segment cannot be pressed off");
}
