// 🪞️ Mirrored number and text keyboard laws: the mirrored number law and the text keyboard law.

//#region 🪞️MirroredNumberLawTests
/// 🧾️ The value payloads a command list dispatches, without their press keys.
fn dispatched_values(commands: &[UiCommand]) -> Vec<DslValue> {
    commands
        .iter()
        .filter_map(|command| match command {
            UiCommand::App { intent, .. } => intent.payload().map(without_scrub_press),
            _ => None,
        })
        .collect()
}

/// 🧮️ The payload `{value}` of one dispatched number.
fn value_payload(number: f64) -> DslValue {
    DslValue::Object(vec![("value".into(), DslValue::float(number))])
}

/// 📓️ A numeric control's draft and its refusal, as `(edit text, refusal)`.
fn draft_state(tree: &UiTree, id: NodeId) -> (Option<String>, Option<String>) {
    let state = &tree.node(id).expect("live control").state;
    (state.edit.as_ref().map(|edit| edit.text.clone()), state.number_refusal.clone())
}

/// 🚧️ A hard bound whose refusal names it.
fn named_bound(value: f64, refusal: &str) -> ui_contract::UiNumberBound {
    ui_contract::UiNumberBound { value, exclusive: false, refusal: Some(ui_contract::Label(ui_contract::UiText::try_from_str(refusal).expect("bounded refusal"))) }
}

/// 🎹️ Every `fieldKeys` row of `🧫️number-controls`: the law key a physical key names on a stepper or a number field — the
/// rows React's `uiNumberFieldKey` answers.
#[test]
fn every_field_key_row_names_the_law_key_the_shared_corpus_declares() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/🧪️tests/🧫️fixtures/🧫️number-controls/🔣️.json")).expect("number-controls fixture");
    let rows = fixture["fieldKeys"].as_array().expect("field key rows");
    assert!(!rows.is_empty(), "the corpus declares field keys");
    for row in rows {
        let named = number_field_key(row["key"].as_str().expect("key"), row["shift"].as_bool().expect("shift"), row["min"].as_f64(), row["max"].as_f64());
        let expected = row["expected"].as_object().map(|expected| {
            let key = match expected["key"].as_str().expect("law key") {
                "increment" => ui_contract::SliderKey::Increment,
                "decrement" => ui_contract::SliderKey::Decrement,
                "pageUp" => ui_contract::SliderKey::PageUp,
                "pageDown" => ui_contract::SliderKey::PageDown,
                "home" => ui_contract::SliderKey::Home,
                _ => ui_contract::SliderKey::End,
            };
            (key, expected["large"].as_bool().expect("large"))
        });
        assert_eq!(named, expected, "{}", row["case"]);
    }
}

/// 🪜️ Every keyboard-law row of `🧫️number-controls` a stepper can be asked (Home and End only toward a bound it has), pressed
/// as the physical key the retained canvas receives: the stepper reports the law's value — the rows React's `Stepper` answers.
#[test]
fn stepper_keys_answer_the_shared_keyboard_law_rows() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/🧪️tests/🧫️fixtures/🧫️number-controls/🔣️.json")).expect("number-controls fixture");
    let mut asked = 0;
    for row in fixture["keys"].as_array().expect("key rows") {
        let number = |field: &str| row[field].as_f64();
        let (min, max) = (number("min"), number("max"));
        let large = row["large"].as_bool().expect("large");
        let (pressed, shift) = match row["key"].as_str().expect("key") {
            "increment" => ("ArrowUp", large),
            "decrement" => ("ArrowDown", large),
            "pageUp" => ("PageUp", false),
            "pageDown" => ("PageDown", false),
            "home" if min.is_some() => ("Home", false),
            "end" if max.is_some() => ("End", false),
            _ => continue,
        };
        let node = UiNode::NumberStepper(UiNumberStepperNode {
            id: "stepper".into(),
            value: number("current").expect("current"),
            step: number("step").expect("step"),
            uniform: true,
            min,
            max,
            precision: row["precision"].as_u64().map(|precision| precision as u16),
            snaps: row["snaps"].as_array().expect("snaps").iter().map(|snap| snap.as_f64().expect("snap")).collect(),
            display_factor: number("factor"),
            on_absolute: ActionDescriptor { controller_id: "ctrl".into(), action: "absolute".into(), args: None },
            on_delta: ActionDescriptor { controller_id: "ctrl".into(), action: String::new(), args: None },
            ..Default::default()
        });
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
        let stepper = leaf(&mut tree, Some(root), 1, node, (0.0, 0.0, 120.0, 24.0));
        let mut router = EventRouter::new("main");
        router.focus.set_focus(&mut tree, Some(stepper), true);
        let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: pressed.into(), modifiers: EventModifiers { shift, ..Default::default() } });
        assert_eq!(dispatched_values(&commands), vec![value_payload(number("expected").expect("expected"))], "{}", row["case"]);
        assert_eq!(draft_state(&tree, stepper).1, None, "{}: a law key leaves no refusal", row["case"]);
        asked += 1;
    }
    assert!(asked > 0, "the corpus asks a stepper at least one key");
}

/// 🪞️ A value an assistive technology writes into a stepper's mirror takes the path a typed keystroke takes on the canvas —
/// one implementation: an admitted text is dispatched exactly, a text beyond a hard bound is refused naming the bound with the
/// draft kept as typed and nothing dispatched (never clamped), and a law key then starts from the declared value with the
/// refusal gone.
#[test]
fn a_mirrored_stepper_value_takes_the_typed_path_of_the_canvas() {
    let stepper_node = || {
        let UiNode::NumberStepper(mut stepper) = number_stepper_ui(2.0, 0.0, 5.0, 1.0, true, false) else { unreachable!() };
        stepper.limits = Some(ui_contract::UiNumberLimits { min: Some(named_bound(0.0, "Must be at least 0")), max: Some(named_bound(5.0, "Must be at most 5")) });
        UiNode::NumberStepper(stepper)
    };
    let mut mirrored_tree = UiTree::new();
    let mirrored_root = leaf(&mut mirrored_tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let mirrored = leaf(&mut mirrored_tree, Some(mirrored_root), 1, stepper_node(), (0.0, 0.0, 120.0, 24.0));
    let mut mirror = EventRouter::new("main");
    let mut typed_tree = UiTree::new();
    let typed_root = leaf(&mut typed_tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let typed = leaf(&mut typed_tree, Some(typed_root), 1, stepper_node(), (0.0, 0.0, 120.0, 24.0));
    let mut canvas = EventRouter::new("main");
    canvas.focus.set_focus(&mut typed_tree, Some(typed), true);
    let select_all = UiEvent::KeyDown { key: "a".into(), modifiers: EventModifiers { ctrl: true, ..Default::default() } };
    for (text, admitted, refusal) in [("4", Some(4.0), None), ("-1", None, Some("Must be at least 0")), ("9", None, Some("Must be at most 5")), ("3", Some(3.0), None)] {
        let through_mirror = mirror.dispatch_accessibility(&mut mirrored_tree, mirrored, &AccessibilityUiEvent::Value(text.into()));
        canvas.dispatch(&mut typed_tree, typed_root, &select_all);
        let through_canvas = canvas.dispatch(&mut typed_tree, typed_root, &UiEvent::Paste { text: text.into() });
        assert_eq!(dispatched_values(&through_mirror), dispatched_values(&through_canvas), "{text}: the mirror and the canvas dispatch the same");
        assert_eq!(dispatched_values(&through_mirror), admitted.map(value_payload).into_iter().collect::<Vec<_>>(), "{text}: an admitted text is dispatched exactly, a refused one never");
        assert_eq!(draft_state(&mirrored_tree, mirrored), draft_state(&typed_tree, typed), "{text}: the same draft and refusal on both paths");
        assert_eq!(draft_state(&mirrored_tree, mirrored), (Some(text.to_string()), refusal.map(str::to_string)), "{text}: the draft is kept as typed and a refusal names its bound");
    }
    assert!(dispatched_values(&mirror.dispatch_accessibility(&mut mirrored_tree, mirrored, &AccessibilityUiEvent::Value("-1".into()))).is_empty());
    let home = mirror.dispatch(&mut mirrored_tree, mirrored_root, &key("Home"));
    assert_eq!(dispatched_values(&home), vec![value_payload(0.0)], "Home reaches the hard minimum");
    assert_eq!(draft_state(&mirrored_tree, mirrored), (Some("0".to_string()), None), "the law's value is shown and the refusal is gone");
    let large = mirror.dispatch(&mut mirrored_tree, mirrored_root, &UiEvent::KeyDown { key: "ArrowUp".into(), modifiers: EventModifiers { shift: true, ..Default::default() } });
    assert_eq!(dispatched_values(&large), vec![value_payload(5.0)], "a large arrow walks ten rungs and stops at the bound");
    let caret = mirror.dispatch(&mut mirrored_tree, mirrored_root, &key("ArrowLeft"));
    assert!(dispatched_values(&caret).is_empty(), "an inline arrow stays with the caret");
}

/// 🎚️ A slider's mirror reads display units and never clamps: a value written into the slider commits through the typed
/// readout law (a detent kept exactly, a value beyond the soft travel admitted while the hard limits admit it), one beyond the
/// limits is refused naming the bound with the draft kept, and the always-addressable `<key>::editor` readout drafts on a
/// value and commits on Enter.
#[test]
fn a_mirrored_slider_value_takes_the_typed_readout_path() {
    use std::f64::consts::{FRAC_PI_2, PI};
    let degrees = 180.0 / PI;
    let step = PI / 180.0;
    let UiNode::Slider(mut dial) = slider_ui("angle", 0.0) else { unreachable!() };
    (dial.min, dial.max, dial.step) = (-FRAC_PI_2, FRAC_PI_2, step);
    dial.display_factor = Some(degrees);
    dial.snaps = vec![-FRAC_PI_2, 0.0, FRAC_PI_2];
    dial.limits = Some(ui_contract::UiNumberLimits { min: Some(named_bound(-PI, "Must be at least -180 °")), max: Some(named_bound(PI, "Must be at most 180 °")) });
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let slider = leaf(&mut tree, Some(root), 1, UiNode::Slider(dial), (0.0, 0.0, 120.0, 24.0));
    let mut router = EventRouter::new("main");

    let detent = router.dispatch_accessibility(&mut tree, slider, &AccessibilityUiEvent::Value("90".into()));
    assert_eq!(dispatched_values(&detent), vec![value_payload(FRAC_PI_2)], "90 ° is the detent's exact stored value, never 90 radians clamped");
    assert_eq!(draft_state(&tree, slider), (None, None));
    let beyond_travel = router.dispatch_accessibility(&mut tree, slider, &AccessibilityUiEvent::Value("135".into()));
    assert_eq!(dispatched_values(&beyond_travel), vec![value_payload(135.0 / degrees)], "a value beyond the soft travel is admitted while the limits admit it");
    let refused = router.dispatch_accessibility(&mut tree, slider, &AccessibilityUiEvent::Value("270".into()));
    assert!(dispatched_values(&refused).is_empty(), "a value beyond a hard limit is never dispatched");
    assert_eq!(draft_state(&tree, slider), (Some("270".to_string()), Some("Must be at most 180 °".to_string())), "the draft is kept and the refusal names the bound");
    assert_eq!(tree.node(slider).unwrap().state.slider_draft_value, Some(135.0 / degrees), "the value stays where it was");

    let drafted = router.dispatch_accessibility_slider_editor(&mut tree, slider, &AccessibilityUiEvent::Value("45".into()));
    assert!(dispatched_values(&drafted).is_empty(), "the readout drafts without committing");
    assert_eq!(draft_state(&tree, slider), (Some("45".to_string()), None));
    let committed = router.dispatch(&mut tree, root, &key("Enter"));
    assert_eq!(dispatched_values(&committed), vec![value_payload(constrain_number_input(45.0 / degrees, Some(-FRAC_PI_2), Some(FRAC_PI_2), Some(step)))], "Enter commits the readout through the slider's law");
    assert_eq!(draft_state(&tree, slider), (None, None));

    let opened = router.dispatch_accessibility_slider_editor(&mut tree, slider, &AccessibilityUiEvent::Focus);
    assert!(dispatched_values(&opened).is_empty());
    assert_eq!(draft_state(&tree, slider).0.as_deref(), Some("45"), "a focus opens the readout draft on the shown value");

    let UiNode::Slider(mut factor) = slider_ui("factor", 1.0) else { unreachable!() };
    (factor.min, factor.max, factor.step) = (0.1, 10.0, 0.01);
    factor.precision = Some(2);
    factor.limits = Some(ui_contract::UiNumberLimits { min: Some(ui_contract::UiNumberBound { exclusive: true, ..named_bound(0.0, "Must be greater than 0") }), max: None });
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let slider = leaf(&mut tree, Some(root), 1, UiNode::Slider(factor), (0.0, 0.0, 120.0, 24.0));
    let mut router = EventRouter::new("main");
    for refused in ["0", "-5"] {
        assert!(dispatched_values(&router.dispatch_accessibility_slider_editor(&mut tree, slider, &AccessibilityUiEvent::Value(refused.into()))).is_empty());
        assert!(dispatched_values(&router.dispatch(&mut tree, root, &key("Enter"))).is_empty(), "{refused}: an excluded bound refuses itself and everything beyond it");
        assert_eq!(draft_state(&tree, slider), (Some(refused.to_string()), Some("Must be greater than 0".to_string())), "{refused}: the readout keeps the draft and names the bound");
    }
    assert!(dispatched_values(&router.dispatch_accessibility_slider_editor(&mut tree, slider, &AccessibilityUiEvent::Value("20".into()))).is_empty());
    assert_eq!(dispatched_values(&router.dispatch(&mut tree, root, &key("Enter"))), vec![value_payload(20.0)], "beyond the soft travel and inside the limits: committed exactly");
}
//#endregion 🪞️MirroredNumberLawTests

//#region 🖊️TextKeyboardLawTests
/// 🧷️ A draft-holding text field of `kind` whose `Commit` binding is live.
fn draft_field(kind: &str, value: &str) -> UiNode {
    let UiNode::Input(mut field) = input_ui("notes", value) else { unreachable!() };
    field.input_kind = kind.into();
    field.commit = Some("blur".into());
    UiNode::Input(field)
}

/// 🖊️ Every row of the shared `🧫️text-controls` corpus, pressed as the physical key the retained canvas receives on a focused
/// draft-holding field: `newline` inserts a line break at the caret and dispatches nothing, `commit` dispatches the draft
/// without leaving the field, `revert` drops the draft back to the published value, and a key that is not the field's changes
/// nothing — the rows React's text fields answer.
#[test]
fn text_fields_answer_the_shared_text_keyboard_law_rows() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧬️contract/🧪️tests/🧫️fixtures/🧫️text-controls/🔣️.json")).expect("text-controls fixture");
    let rows = fixture["keys"].as_array().expect("key rows");
    assert!(rows.len() >= 15, "single-line, multi-line and number rows");
    for row in rows {
        let case = row["case"].as_str().expect("case");
        let kind = row["kind"].as_str().expect("kind");
        let flag = |name: &str| row[name].as_bool().expect("modifier");
        let modifiers = EventModifiers { ctrl: flag("primary"), shift: flag("shift"), alt: flag("alt"), ..Default::default() };
        let pressed = if row["key"] == "enter" { "Enter" } else { "Escape" };
        let expected = match row["action"].as_str() {
            Some("newline") => Some(ui_contract::TextInputKeyAction::Newline),
            Some("commit") => Some(ui_contract::TextInputKeyAction::Commit),
            Some("revert") => Some(ui_contract::TextInputKeyAction::Revert),
            _ => None,
        };
        assert_eq!(text_field_key(kind, pressed, modifiers), expected, "{case}: the law's answer");
        let published = if kind == "number" { "7" } else { "first" };
        let typed = if kind == "number" { "5" } else { "!" };
        let mut tree = UiTree::new();
        let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
        let field = leaf(&mut tree, Some(root), 1, draft_field(kind, published), (0.0, 0.0, 120.0, 24.0));
        let mut router = EventRouter::new("main");
        router.focus.set_focus(&mut tree, Some(field), true);
        assert!(dispatched_values(&router.dispatch(&mut tree, root, &UiEvent::TextInput { text: typed.into() })).is_empty(), "{case}: a draft-holding field dispatches nothing while typing");
        let drafted = format!("{published}{typed}");
        let commands = router.dispatch(&mut tree, root, &UiEvent::KeyDown { key: pressed.into(), modifiers });
        let draft = tree.node(field).and_then(|node| node.state.edit.as_ref()).map(|edit| edit.text.clone());
        match expected {
            Some(ui_contract::TextInputKeyAction::Newline) => {
                assert!(dispatched_values(&commands).is_empty(), "{case}: a line break is the field's own editing");
                assert_eq!(draft.as_deref(), Some(format!("{drafted}\n").as_str()), "{case}: the line break lands at the caret");
            }
            Some(ui_contract::TextInputKeyAction::Commit) => {
                let committed = if kind == "number" { DslValue::float(75.0) } else { DslValue::String(drafted.clone()) };
                assert_eq!(dispatched_values(&commands), vec![DslValue::Object(vec![("value".into(), committed)])], "{case}: the draft is committed without leaving the field");
                assert_eq!((draft.as_deref(), router.focus.focused), (Some(drafted.as_str()), Some(field)), "{case}: the field keeps its text and focus");
            }
            Some(ui_contract::TextInputKeyAction::Revert) => {
                assert!(dispatched_values(&commands).is_empty(), "{case}: a revert dispatches nothing");
                assert_eq!((draft.as_deref(), router.focus.focused), (Some(published), Some(field)), "{case}: the published value is shown again and the field keeps focus");
            }
            None => {
                assert!(dispatched_values(&commands).is_empty(), "{case}: a key that is not the field's dispatches nothing");
                assert_eq!(draft.as_deref(), Some(drafted.as_str()), "{case}: and leaves the draft alone");
            }
        }
    }
}

/// 📜️ A multi-line draft commits whole — every line — when focus leaves, and its single-row field shows the line that
/// holds the caret with the caret's place inside that line.
#[test]
fn a_multi_line_draft_commits_whole_and_shows_the_carets_line() {
    let mut tree = UiTree::new();
    let root = leaf(&mut tree, None, 0, stack_ui(), (0.0, 0.0, 200.0, 100.0));
    let field = leaf(&mut tree, Some(root), 1, draft_field("longText", "one"), (0.0, 0.0, 120.0, 24.0));
    let mut router = EventRouter::new("main");
    router.focus.set_focus(&mut tree, Some(field), true);
    router.dispatch(&mut tree, root, &key("Enter"));
    router.dispatch(&mut tree, root, &UiEvent::TextInput { text: "two".into() });
    let edit = tree.node(field).and_then(|node| node.state.edit.clone()).expect("an open draft");
    assert_eq!((edit.text.as_str(), edit.caret), ("one\ntwo", 7));
    assert_eq!(semio_framework_ui_wgpu_paint::text_line_view(&edit.text, Some(edit.caret)), ("two", 3), "the caret's line and its place in it");
    assert_eq!(semio_framework_ui_wgpu_paint::text_line_view(&edit.text, Some(2)), ("one", 2));
    assert_eq!(semio_framework_ui_wgpu_paint::text_line_view(&edit.text, None), ("one", 0), "without a caret the first line");
    assert_eq!(semio_framework_ui_wgpu_paint::text_line_view("", Some(0)), ("", 0));
    let blurred = router.dispatch(&mut tree, root, &UiEvent::PointerDown { x: 190.0, y: 90.0, button: PointerButton::Primary, modifiers: Default::default() });
    assert_eq!(dispatched_values(&blurred), vec![DslValue::Object(vec![("value".into(), DslValue::String("one\ntwo".into()))])], "leaving the field commits every line");
}
//#endregion 🖊️TextKeyboardLawTests
