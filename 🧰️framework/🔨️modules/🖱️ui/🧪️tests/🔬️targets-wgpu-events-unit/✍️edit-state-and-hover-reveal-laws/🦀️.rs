// ✍️ Edit state and hover reveal laws: the text edit state, caret, selection and clipboard routing and the hover reveal.

//#region 🔖️EditStateTests
#[test]
fn focusing_an_input_seeds_edit_state_from_its_value_and_blur_clears_it() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let input = leaf(&mut tree, Some(root), 1, input_ui("name", "hello"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(tree.node(input).unwrap().state.edit, Some(EditState { text: "hello".into(), caret: 5, anchor: 5, composition: None, scroll_x: 0.0 }));

    // clicking empty space blurs.
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 190.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(tree.node(input).unwrap().state.edit, None, "blur must relinquish the buffer so the declarative value governs again");
}

/// 📌️ A number field's page keys follow the shared keyboard law — the adjacent detent, else ten steps — in its
/// edit buffer, a committed detent stays off the step ladder, and a text field ignores them.
#[test]
fn a_number_fields_page_keys_jump_between_its_detents() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    let UiNode::Input(mut number) = input_ui("offset", "1.30") else { unreachable!() };
    (number.input_kind, number.step, number.precision, number.snaps) = ("number".into(), Some(0.5), Some(2), vec![0.0, 2.25]);
    leaf(&mut tree, Some(root), 1, UiNode::Input(number.clone()), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let input = router.focus.focused.expect("the field takes focus");
    let text = |tree: &UiTree| tree.node(input).and_then(|node| node.state.edit.as_ref()).map(|edit| edit.text.clone());
    let mut walk = Vec::new();
    for pressed in ["PageUp", "PageUp", "PageDown", "PageDown", "PageDown"] {
        router.dispatch(&mut tree, root, &key(pressed));
        walk.push(text(&tree).expect("the buffer stays open"));
    }
    assert_eq!(walk, ["2.25", "7.00", "2.25", "0.00", "-5.00"], "detent, ten steps, detent, detent, ten steps");
    assert_eq!(constrain_number_field(2.25, &number), 2.25, "a committed detent is kept off the ladder");
    assert_eq!(constrain_number_field(1.6, &number), 1.5);
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let field = router.focus.focused.expect("the text field takes focus");
    router.dispatch(&mut tree, root, &key("PageUp"));
    assert_eq!(tree.node(field).and_then(|node| node.state.edit.as_ref()).map(|edit| edit.text.as_str()), Some("abc"));
}

#[test]
fn arrow_keys_move_the_caret_and_backspace_deletes_the_previous_char() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let input = router.focus.focused.unwrap();

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers::default() });
    assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().caret, 2);

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "ArrowLeft".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
    assert_eq!((edit.anchor, edit.caret), (2, 1), "shift+arrow extends the selection instead of collapsing it");

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Backspace".into(), modifiers: EventModifiers::default() });
    let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
    assert_eq!(edit.text, "ac", "backspace over a selection deletes the selected range");
    assert_eq!((edit.anchor, edit.caret), (1, 1));
}

#[test]
fn character_insertion_replaces_the_selection() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", "abc"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let input = router.focus.focused.unwrap();

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "xyz".into() });

    let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
    assert_eq!(edit.text, "xyz");
    assert_eq!((edit.anchor, edit.caret), (3, 3));
}

#[test]
fn copy_over_a_selection_emits_a_clipboard_command_without_mutating_the_buffer() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", "hello"), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let input = router.focus.focused.unwrap();

    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "Home".into(), modifiers: EventModifiers::default() });
    router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "End".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "c".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } });

    assert!(commands.iter().any(|cmd| matches!(cmd, UiCommand::ClipboardCopy { text, .. } if text == "hello")));
    assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().text, "hello", "copy must not mutate the buffer");
}

#[test]
fn ime_commit_inserts_the_composed_text_and_clears_composition() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 200.0));
    leaf(&mut tree, Some(root), 1, input_ui("name", ""), (0.0, 0.0, 100.0, 20.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 10.0, y: 10.0, button: PointerButton::Primary, modifiers: Default::default() });
    let input = router.focus.focused.unwrap();

    router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Start));
    router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Update { text: "ねこ".into(), cursor: 2 }));
    assert_eq!(tree.node(input).unwrap().state.edit.as_ref().unwrap().composition.as_deref(), Some("ねこ"));

    router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Commit { text: "ねこ".into() }));
    let edit = tree.node(input).unwrap().state.edit.clone().unwrap();
    assert_eq!(edit.text, "ねこ");
    assert_eq!(edit.composition, None);
}

/// 🔢 The retained NumberStepper consumes the same language-neutral focus/edit/key/hold law as
/// the actual React Stepper oracle, including the browser-owned selection, clipboard and IME paths.
#[test]
fn number_stepper_editing_and_hold_repeat_match_the_neutral_react_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⌨️number-stepper-editing/🔣️.json")).expect("NumberStepper editing fixture");
    let uniform = &fixture["uniform"];
    let number = |value: &serde_json::Value| value.as_f64().expect("fixture number");
    let min = number(&uniform["min"]);
    let max = number(&uniform["max"]);
    let step = number(&uniform["step"]);
    let app_payloads = |commands: &[UiCommand]| {
        commands
            .iter()
            .filter_map(|command| match command {
                UiCommand::App { intent, .. } => Some((intent.trigger, intent.payload().map(without_scrub_press))),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let scalar = |field: &str, value: f64| Some(DslValue::Object(vec![(field.into(), DslValue::float(value))]));

    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let stepper = leaf(&mut tree, Some(root), 1, number_stepper_ui(number(&uniform["value"]), min, max, step, true, false), (0.0, 0.0, 120.0, 24.0));
    let mut router = EventRouter::new("main");
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 60.0, y: 12.0, button: PointerButton::Primary, modifiers: Default::default() });
    let focused = tree.node(stepper).unwrap().state.edit.as_ref().unwrap();
    assert_eq!(focused.text, uniform["focusDisplay"].as_str().unwrap());
    assert_eq!((focused.anchor, focused.caret), (uniform["focusCaret"].as_u64().unwrap() as usize, uniform["focusCaret"].as_u64().unwrap() as usize));

    let select_all = UiEvent::KeyDown { key: "a".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } };
    assert!(app_payloads(&router.dispatch(&mut tree, root, &select_all)).is_empty(), "selection changes do not dispatch values");
    let copied = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "c".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } });
    assert!(copied.iter().any(|command| matches!(command, UiCommand::ClipboardCopy { text, .. } if text == uniform["focusDisplay"].as_str().unwrap())));
    assert!(app_payloads(&copied).is_empty());
    let cut = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: "x".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } });
    assert!(cut.iter().any(|command| matches!(command, UiCommand::ClipboardCut { text, .. } if text == uniform["focusDisplay"].as_str().unwrap())));
    assert!(app_payloads(&cut).is_empty(), "an empty numeric edit is not a value");
    assert_eq!(tree.node(stepper).unwrap().state.edit.as_ref().unwrap().text, "");

    let input = fixture["validEdit"]["input"].as_str().unwrap();
    let edited = router.dispatch(&mut tree, root, &UiEvent::Paste { text: input.into() });
    let expected = number(&fixture["validEdit"]["expectedValue"]);
    assert_eq!(app_payloads(&edited), vec![(Trigger::Change, scalar("value", expected))]);
    assert_eq!(tree.node(stepper).unwrap().state.edit.as_ref().unwrap().text, fixture["validEdit"]["expectedDisplay"].as_str().unwrap());
    router.dispatch(&mut tree, root, &select_all);
    let refused = router.dispatch(&mut tree, root, &UiEvent::Paste { text: fixture["refusedEdit"]["input"].as_str().unwrap().into() });
    assert!(app_payloads(&refused).is_empty(), "a typed value beyond the hard bound is refused, never clamped");
    let state = &tree.node(stepper).unwrap().state;
    assert_eq!((state.edit.as_ref().unwrap().text.as_str(), state.number_refusal.as_deref()), (fixture["refusedEdit"]["expectedDisplay"].as_str().unwrap(), Some("")), "the refused draft is kept with its refusal");

    router.dispatch(&mut tree, root, &select_all);
    assert!(app_payloads(&router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Start))).is_empty());
    assert!(app_payloads(&router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Update { text: "4".into(), cursor: 1 }))).is_empty());
    assert_eq!(tree.node(stepper).unwrap().state.edit.as_ref().unwrap().composition.as_deref(), Some("4"));
    let composed = router.dispatch(&mut tree, root, &UiEvent::Ime(ImeEvent::Commit { text: "4".into() }));
    assert_eq!(app_payloads(&composed), vec![(Trigger::Change, scalar("value", 4.0))]);
    assert_eq!(tree.node(stepper).unwrap().state.edit.as_ref().unwrap().text, "4");
    assert!(tree.node(stepper).unwrap().state.edit.as_ref().unwrap().composition.is_none());
    assert!(app_payloads(&router.dispatch(&mut tree, root, &key("ArrowLeft"))).is_empty(), "caret motion alone does not dispatch");
    assert_eq!(app_payloads(&router.dispatch(&mut tree, root, &key("Enter"))), vec![(Trigger::Change, scalar("value", 4.0))], "Enter blurs the editor and releases its typed press");
    assert!(tree.node(stepper).unwrap().state.edit.is_none());

    let mixed = leaf(&mut tree, Some(root), 2, number_stepper_ui(number(&fixture["mixed"]["value"]), number(&fixture["mixed"]["min"]), number(&fixture["mixed"]["max"]), number(&fixture["mixed"]["step"]), false, false), (0.0, 30.0, 120.0, 24.0));
    router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 60.0, y: 42.0, button: PointerButton::Primary, modifiers: Default::default() });
    let mixed_edit = tree.node(mixed).unwrap().state.edit.as_ref().unwrap();
    assert_eq!(mixed_edit.text, fixture["mixed"]["focusDisplay"].as_str().unwrap());
    assert_eq!(mixed_edit.caret, fixture["mixed"]["focusCaret"].as_u64().unwrap() as usize);

    for key_case in fixture["boundedKeys"].as_array().unwrap() {
        let mut keyed_tree = UiTree::new();
        let keyed_root = leaf(&mut keyed_tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
        let delta = key_case["binding"].as_str().unwrap() == "delta";
        let keyed = leaf(&mut keyed_tree, Some(keyed_root), 1, number_stepper_ui(number(&key_case["value"]), min, max, step, true, delta), (0.0, 0.0, 120.0, 24.0));
        let mut keyed_router = EventRouter::new("main");
        keyed_router.dispatch(&mut keyed_tree, keyed_root, &UiEvent::PointerDown { x: 60.0, y: 12.0, button: PointerButton::Primary, modifiers: Default::default() });
        let commands = keyed_router.dispatch(&mut keyed_tree, keyed_root, &key(key_case["key"].as_str().unwrap()));
        let field = key_case["expectedField"].as_str().unwrap();
        let expected_value = number(&key_case["expectedActionValue"]);
        let trigger = if field == "delta" { Trigger::Delta } else { Trigger::Change };
        assert_eq!(app_payloads(&commands), vec![(trigger, scalar(field, expected_value))], "{}", key_case["id"].as_str().unwrap());
        assert_eq!(keyed_tree.node(keyed).unwrap().state.edit.as_ref().unwrap().text, key_case["expectedDisplay"].as_str().unwrap());
    }

    let pointer_case = &fixture["boundedPointer"];
    let mut bounded_tree = UiTree::new();
    let bounded_root = leaf(&mut bounded_tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let bounded = leaf(&mut bounded_tree, Some(bounded_root), 1, number_stepper_ui(number(&pointer_case["value"]), min, max, step, true, false), (0.0, 0.0, 120.0, 24.0));
    let mut bounded_router = EventRouter::new("main");
    let blocked = bounded_router.dispatch(&mut bounded_tree, bounded_root, &UiEvent::PointerDown { x: 108.0, y: 12.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(app_payloads(&blocked).len(), pointer_case["actionCount"].as_u64().unwrap() as usize);
    assert!(bounded_tree.node(bounded).unwrap().state.edit.is_none(), "a disabled side button never focuses or starts an edit");

    let hold = &fixture["hold"];
    let mut held_tree = UiTree::new();
    let held_root = leaf(&mut held_tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let held = leaf(&mut held_tree, Some(held_root), 1, number_stepper_ui(number(&hold["start"]), min, max, step, true, false), (0.0, 0.0, 120.0, 24.0));
    let mut held_router = EventRouter::new("main");
    let clock_origin = number(&hold["clockOriginSeconds"]);
    assert!(!held_router.advance_clock(&mut held_tree, clock_origin).changed);
    let pressed = held_router.dispatch(&mut held_tree, held_root, &UiEvent::PointerDown { x: 108.0, y: 12.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(app_payloads(&pressed), vec![(Trigger::Change, scalar("value", number(&hold["atDelay"])))]);
    assert_eq!(held_tree.node(held).unwrap().state.edit.as_ref().unwrap().text, hold["atDelay"].as_i64().unwrap().to_string());
    let tooltip_deadline = clock_origin + f64::from(TOOLTIP_DWELL_SECONDS);
    let first_deadline = clock_origin + (number(&hold["delayMs"]) + number(&hold["intervalMs"])) / 1000.0;
    assert_eq!(held_router.next_clock_deadline(), Some(tooltip_deadline), "the hovered control's tooltip dwell is earlier than its first held repeat");
    let at_delay = held_router.advance_clock(&mut held_tree, clock_origin + number(&hold["delayMs"]) / 1000.0);
    assert!(at_delay.changed, "the exact tooltip dwell changes presented hover state before the first repeat");
    assert_eq!(at_delay.tooltip, TooltipStep::Reveal(held));
    assert_eq!(at_delay.next_deadline, Some(first_deadline));
    assert!(app_payloads(&at_delay.commands).is_empty());
    assert_eq!(held_tree.node(held).unwrap().state.edit.as_ref().unwrap().text, hold["atDelay"].as_i64().unwrap().to_string());
    let first_repeat = held_router.advance_clock(&mut held_tree, first_deadline);
    assert!(first_repeat.changed, "the first repeat changes retained edit state exactly at its deadline");
    assert_eq!(first_repeat.next_deadline, Some(first_deadline + number(&hold["intervalMs"]) / 1000.0));
    assert!(app_payloads(&first_repeat.commands).is_empty());
    assert_eq!(held_tree.node(held).unwrap().state.edit.as_ref().unwrap().text, hold["atFirstInterval"].as_i64().unwrap().to_string());
    let second_repeat = held_router.advance_clock(&mut held_tree, first_deadline + number(&hold["intervalMs"]) / 1000.0);
    assert!(second_repeat.changed);
    assert!(app_payloads(&second_repeat.commands).is_empty());
    assert_eq!(held_tree.node(held).unwrap().state.edit.as_ref().unwrap().text, hold["atSecondInterval"].as_i64().unwrap().to_string());
    held_router.dispatch(&mut held_tree, held_root, &UiEvent::PointerMove { x: 60.0, y: 12.0, modifiers: Default::default() });
    held_router.advance_clock(&mut held_tree, clock_origin + 2.0);
    assert_eq!(held_tree.node(held).unwrap().state.edit.as_ref().unwrap().text, hold["atSecondInterval"].as_i64().unwrap().to_string(), "leaving the held side cancels local repeat");
}
//#endregion 🔖️EditStateTests

//#region 🔖️HoverRevealTests
#[test]
fn hovering_a_leaf_marks_its_whole_ancestor_chain_hovered_and_clearing_hover_clears_it_all() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let row = leaf(&mut tree, Some(root), 1, stack_ui(), (0.0, 0.0, 100.0, 100.0));
    let label = leaf(&mut tree, Some(row), 1, text_ui("item"), (0.0, 0.0, 50.0, 20.0));
    let mut router = EventRouter::new("main");

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 10.0, y: 10.0, modifiers: Default::default() });
    assert!(tree.node(label).unwrap().flags.contains(NodeFlags::HOVERED));
    assert!(tree.node(row).unwrap().flags.contains(NodeFlags::HOVERED), "an ancestor Stack row must observe hover too, for paint's reveal-on-hover");
    assert!(tree.node(root).unwrap().flags.contains(NodeFlags::HOVERED));

    router.dispatch(&mut tree, root, &UiEvent::PointerMove { x: 500.0, y: 500.0, modifiers: Default::default() });
    assert!(!tree.node(label).unwrap().flags.contains(NodeFlags::HOVERED));
    assert!(!tree.node(row).unwrap().flags.contains(NodeFlags::HOVERED));
    assert!(!tree.node(root).unwrap().flags.contains(NodeFlags::HOVERED));
}
//#endregion 🔖️HoverRevealTests
