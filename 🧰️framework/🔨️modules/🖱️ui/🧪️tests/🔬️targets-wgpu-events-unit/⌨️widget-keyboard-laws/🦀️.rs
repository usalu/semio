// ⌨️ Widget keyboard laws: the keyboard handling of every widget kind.

//#region ⌨️WidgetKeyboardTests
// ⌨️ Keyboard parity for the controls a plain `<button>`'s Enter/Space does not cover — ticket
// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W1o. The per-key decision tables themselves are unit
// tested with the element (`🧱️elements/🔽️Select/🧪️tests/🔬️wgpu-select-keyboard`); these assert the
// ROUTING: that a focused control claims the key, mutates the retained state, and emits (or
// withholds) a command.

fn slider_ui(id: &str, value: f64) -> UiNode {
    UiNode::Slider(UiSliderNode { id: id.into(), value, min: 0.0, max: 10.0, step: 1.0, unit: None, snaps: Vec::new(), on_change: action(), presence: UiPresence::default(), menu: None, ..Default::default() })
}

#[test]
fn slider_pointer_mapping_is_confined_to_the_shared_track_cell() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎚️slider-presentation/🔣️.json")).unwrap();
    for pointer in law["pointer"].as_array().unwrap() {
        let case = law["cases"].as_array().unwrap().iter().find(|case| case["id"] == pointer["case"]).unwrap();
        let bounds = Rect::new(case["bounds"][0].as_f64().unwrap() as f32, case["bounds"][1].as_f64().unwrap() as f32, case["bounds"][2].as_f64().unwrap() as f32, case["bounds"][3].as_f64().unwrap() as f32);
        let node = Node::new(NodeKey::Explicit("slider".into()), WidgetSpec(slider_ui("slider", case["value"].as_f64().unwrap())));
        let fired = pointer_commit_action(
            &node,
            bounds,
            pointer["point"][0].as_f64().unwrap() as f32,
            pointer["point"][1].as_f64().unwrap() as f32,
            if case["inline"] == "rtl" { FlowInline::Rtl } else { FlowInline::Ltr },
            1.0,
            0.0,
            crate::wgpu::theme::Theme::default().gap_standard,
        );
        match pointer["emits"].as_f64() {
            Some(expected) => assert!((fired.and_then(|fired| fired.input).and_then(|value| value.as_f64()).unwrap() - expected).abs() < 0.000_001, "{}", pointer["case"]),
            None => assert!(fired.is_none(), "{} value cell must be inert", pointer["case"]),
        }
    }
}

#[test]
fn slider_external_unit_sibling_is_inert_and_never_enters_track_value_mapping() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎚️slider-presentation/🔣️.json")).unwrap();
    let unit = &law["unit"];
    let placement = unit["placements"].as_array().unwrap().iter().find(|placement| placement["kind"] == "standalone").unwrap();
    let bounds = Rect::new(placement["bounds"][0].as_f64().unwrap() as f32, placement["bounds"][1].as_f64().unwrap() as f32, placement["bounds"][2].as_f64().unwrap() as f32, placement["bounds"][3].as_f64().unwrap() as f32);
    let suffix_width = 40.0;
    let gap = unit["gap"].as_f64().unwrap() as f32;
    let mut slider = slider_ui("slider.unit", unit["value"].as_f64().unwrap());
    let UiNode::Slider(slider_spec) = &mut slider else { unreachable!() };
    slider_spec.unit = Some(unit["unit"].as_str().unwrap().to_string());
    let (value, min, max) = (slider_spec.value, slider_spec.min, slider_spec.max);
    let node = Node::new(NodeKey::Explicit("slider.unit".into()), WidgetSpec(slider));
    let presentation = slider_control_presentation(bounds, value, min, max, Some(suffix_width), gap, FlowInline::Ltr);
    let unit_cell = presentation.unit_cell.unwrap();
    let fired = pointer_commit_action(&node, bounds, unit_cell.x + unit_cell.w * 0.5, unit_cell.y + unit_cell.h * 0.5, FlowInline::Ltr, 1.0, suffix_width, gap);
    assert!(fired.is_none(), "the visible value-unit sibling is outside the Slider pointer target");
}

fn toggle_ui(id: &str, on: bool) -> UiNode {
    let mut presence = UiPresence::default();
    presence.selected = on;
    UiNode::Toggle(UiToggleNode { appearance: ui_contract::ToggleAppearance::Button, id: id.into(), icon_id: IconName::CircleDot, text: None, on_change: action(), presence, menu: None })
}

fn key(key: &str) -> UiEvent {
    UiEvent::KeyDown { key: key.into(), modifiers: EventModifiers::default() }
}

/// ⬇️ `ArrowDown` on a closed, focused `Select` opens the popup highlighting the SELECTED row, the
/// way React's `SelectTrigger` opens with intent `"selected"`.
#[test]
fn arrow_down_opens_a_focused_select_on_its_selected_row() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "b"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);

    router.dispatch(&mut tree, root, &key("ArrowDown"));

    assert!(tree.node(select).unwrap().state.open, "ArrowDown opens the popup");
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1), "the highlight starts on the selected row");
}

/// ⬆️ `ArrowUp` opens on the LAST row; arrowing then wraps, and `Escape` clears both bits.
#[test]
fn arrow_up_opens_on_the_last_row_and_arrowing_wraps() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);

    router.dispatch(&mut tree, root, &key("ArrowUp"));
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1), "ArrowUp opens on the last row");

    router.dispatch(&mut tree, root, &key("ArrowDown"));
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(0), "the highlight wraps past the last row");

    router.dispatch(&mut tree, root, &key("Escape"));
    assert!(!tree.node(select).unwrap().state.open);
    assert_eq!(tree.node(select).unwrap().state.highlighted, None, "a dismissed popup keeps no stale highlight");
}

/// ⏎️ `Enter` over the open popup commits the highlighted row's value and closes it.
#[test]
fn enter_commits_the_highlighted_row_and_closes_the_popup() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);
    router.dispatch(&mut tree, root, &key("ArrowDown"));
    router.dispatch(&mut tree, root, &key("ArrowDown"));
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1));

    let commands = router.dispatch(&mut tree, root, &key("Enter"));

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { .. })), "committing a row dispatches the select's own change action");
    assert!(!tree.node(select).unwrap().state.open, "committing closes the popup");
}

/// 🔤️ A printable key opens the popup on the first matching row, and a second key extends the same
/// query rather than restarting it.
#[test]
fn typing_jumps_to_the_matching_row_and_extends_the_query() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);

    router.dispatch(&mut tree, root, &key("b"));
    assert!(tree.node(select).unwrap().state.open, "typing opens a closed select");
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1), "the query lands on the row whose label starts with it");

    router.dispatch(&mut tree, root, &key("a"));
    assert_eq!(tree.node(select).unwrap().state.highlighted, Some(1), "\"ba\" matches nothing, so the highlight holds instead of jumping to \"A\"");
}

/// ↹️ `Tab` closes the popup AND still moves focus — React's `SelectContent` closes without
/// restoring focus so the browser's own tab move lands.
#[test]
fn tab_closes_an_open_select_and_still_moves_focus() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let select = leaf(&mut tree, Some(root), 1, select_ui("sel", "a"), (0.0, 0.0, 100.0, 30.0));
    let next = leaf(&mut tree, Some(root), 2, button_ui("after"), (0.0, 40.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);
    router.dispatch(&mut tree, root, &key("ArrowDown"));

    router.dispatch(&mut tree, root, &key("Tab"));

    assert!(!tree.node(select).unwrap().state.open, "Tab dismisses the popup");
    assert_eq!(router.focus.focused, Some(next), "and the focus move still happens");
}

/// 🚫️ A disabled `Select` ignores every key, exactly as a `disabled` trigger does in the DOM.
#[test]
fn a_disabled_select_ignores_keyboard_input() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let mut disabled = select_ui("sel", "a");
    if let UiNode::Select(node) = &mut disabled {
        node.presence.state = UiState::Disabled;
    }
    let select = leaf(&mut tree, Some(root), 1, disabled, (0.0, 0.0, 100.0, 30.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(select), true);

    router.dispatch(&mut tree, root, &key("ArrowDown"));

    assert!(!tree.node(select).unwrap().state.open);
}

/// 🎚️ A focused `Slider` steps on the arrow keys, jumps to its ends on `Home`/`End`, moves ten
/// steps on `PageUp`/`PageDown`, and clamps — React's own `Slider` key handler.
#[test]
fn a_focused_slider_steps_jumps_and_clamps_on_the_arrow_keys() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let slider = leaf(&mut tree, Some(root), 1, slider_ui("s", 5.0), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(slider), true);

    for pressed in ["ArrowRight", "ArrowUp", "ArrowLeft", "ArrowDown", "Home", "End", "PageUp", "PageDown"] {
        let commands = router.dispatch(&mut tree, root, &key(pressed));
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { .. })), "{pressed} must commit a slider value");
    }
    let ignored = router.dispatch(&mut tree, root, &key("F5"));
    assert!(ignored.iter().all(|cmd| !matches!(cmd, UiCommand::App { .. })), "an unrelated key must not move the slider");
}

/// 🎚️ The values themselves: one step per arrow, ten per page key, clamped to the track's ends.
#[test]
fn slider_key_values_match_reacts_step_multiplier_and_clamp() {
    let UiNode::Slider(slider) = slider_ui("s", 5.0) else { panic!("slider") };
    assert_eq!(slider_key_value(&slider, "ArrowRight", false), Some(6.0));
    assert_eq!(slider_key_value(&slider, "ArrowLeft", false), Some(4.0));
    assert_eq!(slider_key_value(&slider, "ArrowUp", false), Some(6.0));
    assert_eq!(slider_key_value(&slider, "ArrowDown", false), Some(4.0));
    assert_eq!(slider_key_value(&slider, "Home", false), Some(0.0));
    assert_eq!(slider_key_value(&slider, "End", false), Some(10.0));
    assert_eq!(slider_key_value(&slider, "PageUp", false), Some(10.0), "ten steps from 5 clamps at the max");
    assert_eq!(slider_key_value(&slider, "PageDown", false), Some(0.0));
    assert_eq!(slider_key_value(&slider, "ArrowRight", true), Some(10.0), "a Shift chord moves ten steps too");
    assert_eq!(slider_key_value(&slider, "Enter", false), None, "a key the slider does not own is never swallowed");
}

/// ⌨️ Every bounded (slider) keyboard-law row of `🧫️number-controls`, pressed as the physical key the retained
/// canvas receives — the same rows the contract twins and React's `Slider` answer.
#[test]
fn slider_key_values_answer_the_shared_keyboard_law_rows() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧬️contract/🧪️tests/🧫️fixtures/🧫️number-controls/🔣️.json")).expect("number-controls fixture");
    for row in fixture["keys"].as_array().expect("key rows").iter().filter(|row| !row["min"].is_null() && !row["max"].is_null() && row["step"].as_f64().is_some_and(|step| step > 0.0)) {
        let number = |field: &str| row[field].as_f64().expect("fixture number");
        let UiNode::Slider(mut slider) = slider_ui("s", number("current")) else { unreachable!() };
        (slider.min, slider.max, slider.step) = (number("min"), number("max"), number("step"));
        (slider.precision, slider.display_factor) = (row["precision"].as_u64().map(|precision| precision as u16), row["factor"].as_f64());
        slider.snaps = row["snaps"].as_array().expect("snaps").iter().map(|snap| snap.as_f64().expect("snap")).collect();
        let pressed = match row["key"].as_str().expect("key") {
            "increment" => "ArrowRight",
            "decrement" => "ArrowLeft",
            "pageUp" => "PageUp",
            "pageDown" => "PageDown",
            "home" => "Home",
            _ => "End",
        };
        assert_eq!(slider_key_value(&slider, pressed, row["large"].as_bool().expect("large")), Some(number("expected")), "{}", row["case"]);
    }
}

/// 🎚️ The retained canvas consumes the same readout editing vectors as the mounted React Slider:
/// only a double-click in the readout opens its numeric buffer, Enter validates/snaps once, and
/// the local controlled draft drives later keys while an older declaration is still visible.
#[test]
fn slider_readout_editing_matches_the_neutral_react_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⌨️slider-readout-editing/🔣️.json")).expect("Slider editing fixture");
    let range = &fixture["slider"];
    let double_click = &fixture["doubleClick"];
    let number = |value: &serde_json::Value| value.as_f64().expect("fixture number");
    let make_slider = || {
        let UiNode::Slider(mut slider) = slider_ui("slider.edit", number(&range["value"])) else { unreachable!() };
        slider.min = number(&range["min"]);
        slider.max = number(&range["max"]);
        slider.step = number(&range["step"]);
        UiNode::Slider(slider)
    };
    let payloads = |commands: &[UiCommand]| {
        commands
            .iter()
            .filter_map(|command| match command {
                UiCommand::App { intent, .. } => intent.payload().map(without_scrub_press),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let pointer = |down: bool| {
        if down {
            UiEvent::PointerDown { x: 110.0, y: 12.0, button: PointerButton::Primary, modifiers: Default::default() }
        } else {
            UiEvent::PointerUp { x: 110.0, y: 12.0, button: PointerButton::Primary, modifiers: Default::default() }
        }
    };

    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let slider = leaf(&mut tree, Some(root), 1, make_slider(), (0.0, 0.0, 120.0, 24.0));
    let mut router = EventRouter::new("main");
    let clock_origin = number(&double_click["clockOriginSeconds"]);
    router.advance_clock(&mut tree, clock_origin);
    router.dispatch(&mut tree, root, &pointer(true));
    router.dispatch(&mut tree, root, &pointer(false));
    assert!(tree.node(slider).unwrap().state.edit.is_none(), "one readout click does not edit");
    router.advance_clock(&mut tree, clock_origin + number(&double_click["intervalMs"]) / 1000.0);
    router.dispatch(&mut tree, root, &pointer(true));
    router.dispatch(&mut tree, root, &pointer(false));
    assert_eq!(tree.node(slider).unwrap().state.edit.as_ref().unwrap().text, "2", "the second click seeds the displayed value at the caret");

    let mut expired_tree = UiTree::new();
    let expired_root = leaf(&mut expired_tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let expired_slider = leaf(&mut expired_tree, Some(expired_root), 1, make_slider(), (0.0, 0.0, 120.0, 24.0));
    let mut expired_router = EventRouter::new("main");
    expired_router.advance_clock(&mut expired_tree, clock_origin);
    for event in [pointer(true), pointer(false)] {
        expired_router.dispatch(&mut expired_tree, expired_root, &event);
    }
    expired_router.advance_clock(&mut expired_tree, clock_origin + number(&double_click["timeoutMs"]) / 1000.0 + 0.001);
    for event in [pointer(true), pointer(false)] {
        expired_router.dispatch(&mut expired_tree, expired_root, &event);
    }
    assert!(expired_tree.node(expired_slider).unwrap().state.edit.is_none(), "a second click after the neutral timeout starts a fresh pair even after long monotonic uptime");

    let accessible_edit = router.dispatch_accessibility_slider_editor(&mut tree, slider, &AccessibilityUiEvent::Value(fixture["enter"]["input"].as_str().unwrap().into()));
    assert!(payloads(&accessible_edit).is_empty(), "the mirrored spinbutton edits the same draft without committing before Enter");
    assert_eq!(tree.node(slider).unwrap().state.edit.as_ref().unwrap().text, fixture["enter"]["input"].as_str().unwrap());
    assert!(payloads(&router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "a".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } })).is_empty());
    assert!(payloads(&router.dispatch(&mut tree, root, &UiEvent::TextInput { text: fixture["enter"]["input"].as_str().unwrap().into() })).is_empty(), "typing only updates the native input draft");
    let committed = router.dispatch(&mut tree, root, &key("Enter"));
    assert_eq!(payloads(&committed), vec![DslValue::Object(vec![("value".into(), DslValue::float(number(&fixture["enter"]["expectedValue"])))])]);
    assert!(tree.node(slider).unwrap().state.edit.is_none());
    assert_eq!(tree.node(slider).unwrap().state.slider_draft_value, Some(number(&fixture["enter"]["expectedValue"])), "the controlled local draft survives the stale declaration");

    router.focus.set_focus(&mut tree, Some(slider), true);
    let stepped = router.dispatch(&mut tree, root, &key("ArrowRight"));
    assert_eq!(payloads(&stepped), vec![DslValue::Object(vec![("value".into(), DslValue::float(5.0))])], "keyboard stepping starts from the local draft, not the stale declared 2");
    assert_eq!(tree.node(slider).unwrap().state.slider_draft_value, Some(5.0));

    for rejected in fixture["rejected"].as_array().unwrap() {
        let mut rejected_tree = UiTree::new();
        let rejected_root = leaf(&mut rejected_tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
        let rejected_slider = leaf(&mut rejected_tree, Some(rejected_root), 1, make_slider(), (0.0, 0.0, 120.0, 24.0));
        let mut rejected_router = EventRouter::new("main");
        rejected_router.advance_clock(&mut rejected_tree, clock_origin);
        for event in [pointer(true), pointer(false)] {
            rejected_router.dispatch(&mut rejected_tree, rejected_root, &event);
        }
        rejected_router.advance_clock(&mut rejected_tree, clock_origin + number(&double_click["intervalMs"]) / 1000.0);
        for event in [pointer(true), pointer(false)] {
            rejected_router.dispatch(&mut rejected_tree, rejected_root, &event);
        }
        rejected_router.dispatch(&mut rejected_tree, rejected_root, &UiEvent::KeyDown { key: "a".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } });
        rejected_router.dispatch(&mut rejected_tree, rejected_root, &UiEvent::TextInput { text: rejected["input"].as_str().unwrap().into() });
        let commands = match rejected["exit"].as_str().unwrap() {
            "blur" => rejected_router.dispatch(&mut rejected_tree, rejected_root, &UiEvent::PointerDown { x: 180.0, y: 80.0, button: PointerButton::Primary, modifiers: Default::default() }),
            key_name => rejected_router.dispatch(&mut rejected_tree, rejected_root, &key(key_name)),
        };
        assert_eq!(payloads(&commands).len(), rejected["actionCount"].as_u64().unwrap() as usize, "{}", rejected["id"]);
        let state = &rejected_tree.node(rejected_slider).unwrap().state;
        if rejected["editing"].as_bool().unwrap() {
            assert_eq!((state.edit.as_ref().map(|edit| edit.text.as_str()), state.number_refusal.as_deref()), (rejected["expectedDisplay"].as_str(), Some("")), "{} keeps its refused draft", rejected["id"]);
        } else {
            assert!(state.edit.is_none(), "{} exits editing", rejected["id"]);
        }
        assert_eq!(rejected_tree.node(rejected_slider).unwrap().state.slider_draft_value, None, "{} preserves the declared value", rejected["id"]);
    }

    for key_case in fixture["keyboard"].as_array().unwrap() {
        let UiNode::Slider(slider) = make_slider() else { unreachable!() };
        assert_eq!(slider_key_value(&slider, key_case["key"].as_str().unwrap(), false), Some(number(&key_case["expectedValue"])), "{}", key_case["key"]);
    }

    let mut reconciled = UiTree::new();
    reconciled.apply_tree(&make_slider());
    let root = reconciled.root.unwrap();
    reconciled.node_mut(root).unwrap().state.slider_draft_value = Some(number(&fixture["enter"]["expectedValue"]));
    reconciled.apply_tree(&make_slider());
    assert_eq!(reconciled.node(root).unwrap().state.slider_draft_value, Some(number(&fixture["enter"]["expectedValue"])), "a stale controlled declaration does not roll back local echo");
    let UiNode::Slider(mut caught_up) = make_slider() else { unreachable!() };
    caught_up.value = number(&fixture["enter"]["expectedValue"]);
    reconciled.apply_tree(&UiNode::Slider(caught_up));
    assert_eq!(reconciled.node(root).unwrap().state.slider_draft_value, None, "the draft retires when the declaration catches up");
}

/// 🔀️ A focused `Toggle` flips on `Enter`/`Space` — React renders it as a `<button>`, so both keys
/// click it; wgpu used to answer neither.
#[test]
fn a_focused_toggle_flips_on_enter_and_space() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let toggle = leaf(&mut tree, Some(root), 1, toggle_ui("t", false), (0.0, 0.0, 40.0, 20.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(toggle), true);

    for pressed in ["Enter", " "] {
        let commands = router.dispatch(&mut tree, root, &key(pressed));
        assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::App { .. })), "{pressed} must fire the toggle's change action");
    }
}
//#endregion ⌨️WidgetKeyboardTests
