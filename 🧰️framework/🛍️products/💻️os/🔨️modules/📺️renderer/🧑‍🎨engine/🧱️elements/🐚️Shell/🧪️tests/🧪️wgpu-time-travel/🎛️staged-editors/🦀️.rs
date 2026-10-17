/// 🎛️ Staged editors: every editor kind in the Actions form and in a chrome dialog, axis splicing, number facets and keyboard and pointer staging.

//#region 🎛️StagedEditors
fn option(value: &str, en: &str, de: &str) -> ActionArgOption {
    ActionArgOption { value: value.into(), label: LocalizedLabel::native(en, de) }
}

/// 🎛️ One argument of every editor kind the input descriptors produce.
fn editor_defs() -> Vec<ActionArgDef> {
    let mut stepper = ActionArgDef { schema: ArgSchema::number(Some(0.0), Some(10.0), Some(1.0), true), ..ActionArgDef::number("count", LocalizedLabel::native("Count", "Anzahl")) };
    stepper.presentation = Some(ArgPresentation::Stepper);
    let mut slider = ActionArgDef::slider("angle", LocalizedLabel::native("Angle", "Winkel"), 0.0, 90.0);
    if let ArgSchema::Number { snaps, step, .. } = &mut slider.schema {
        *snaps = vec![0.0, 45.0, 90.0];
        *step = Some(5.0);
    }
    let mut segmented = ActionArgDef::select("mode", LocalizedLabel::native("Mode", "Modus"), vec![option("add", "Add", "Hinzufügen"), option("cut", "Cut", "Schneiden")]);
    segmented.presentation = Some(ArgPresentation::Segmented);
    let vector = ActionArgDef::vector("offset", LocalizedLabel::native("Offset", "Versatz"), 3);
    let reference = ActionArgDef {
        schema: ArgSchema::Reference { kinds: vec!["node".into()], domain: Some("vortex".into()), granularity: Some("node".into()), many: true, min_items: None, max_items: None, id_type: semio_framework::ReferenceIdType::String },
        ..ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele"))
    };
    vec![stepper, slider, segmented, vector, reference]
}

fn selection() -> HashMap<String, DomainSelection> {
    HashMap::from([("vortex".to_string(), DomainSelection { granularity: "node".into(), ids: vec!["n1".into(), "n2".into()], anchor_id: None })])
}

fn args_of(descriptor: &ActionDescriptor) -> Value {
    descriptor.args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null)
}

/// ⚖️ LAW: the Actions form renders every editor kind as its own accessible control — a stepper, a slider with its
/// detents, a segmented choice of pressed buttons, one labelled field per vector axis, and a reference list of removable
/// chips with a "use current selection" button that stages the live selection — each part sending the stage verb.
#[test]
fn the_actions_form_renders_every_editor_kind() {
    let defs = editor_defs();
    let row = |index: usize, value: Option<Value>| staged_action_arg_row("main", "move", &defs[index], value.as_ref(), &selection(), Terminology::Native, Locale::En);
    let Some(UiControlNode::NumberStepper(stepper)) = row(0, Some(serde_json::json!(3.0))).control else { panic!("a stepper") };
    assert_eq!((stepper.value, stepper.step, stepper.min, stepper.max, stepper.on_absolute.action.as_str(), stepper.on_delta.action.is_empty()), (3.0, 1.0, Some(0.0), Some(10.0), "stageActionArg", true));
    let Some(UiControlNode::Slider(slider)) = row(1, None).control else { panic!("a slider") };
    assert_eq!((slider.snaps.clone(), slider.step, slider.value), (vec![0.0, 45.0, 90.0], 5.0, 0.0), "the detents reach the slider and paint as ticks");
    let segmented = row(2, Some(serde_json::json!("cut")));
    let segments = segmented.items.expect("one row per option");
    assert_eq!(segmented.default_open, Some(true));
    let pressed: Vec<(String, bool, Value)> = segments
        .iter()
        .map(|item| {
            let Some(UiControlNode::Toggle(toggle)) = &item.control else { panic!("a pressed button") };
            (toggle.id.clone(), toggle.presence.selected, args_of(&toggle.on_change)["option"].clone())
        })
        .collect();
    assert_eq!(pressed, [("mode.option.add".to_string(), false, serde_json::json!("add")), ("mode.option.cut".to_string(), true, serde_json::json!("cut"))]);
    let axes = row(3, Some(serde_json::json!([1.0, 2.0]))).items.expect("one row per axis");
    let axes: Vec<(Option<String>, String, Value)> = axes
        .iter()
        .map(|item| {
            let Some(UiControlNode::Input(field)) = &item.control else { panic!("a number field") };
            (field.accessibility_label.as_ref().map(|label| label.as_str().to_string()), field.value.clone(), args_of(&field.on_change))
        })
        .collect();
    assert_eq!(axes.iter().map(|(name, value, _)| (name.clone().unwrap_or_default(), value.clone())).collect::<Vec<_>>(), [("Offset x".to_string(), "1".to_string()), ("Offset y".to_string(), "2".to_string()), ("Offset z".to_string(), "0".to_string())]);
    assert_eq!((axes[2].2["index"].clone(), axes[2].2["dims"].clone(), axes[2].2["tuple"].clone()), (serde_json::json!(2), serde_json::json!(3), serde_json::json!([1.0, 2.0, 0.0])));
    let references = row(4, Some(serde_json::json!(["a"]))).items.expect("chips and the selection button");
    let buttons: Vec<(String, String, Value, bool)> = references
        .iter()
        .filter_map(|item| match &item.control {
            Some(UiControlNode::Button(button)) => Some((button.id.clone().unwrap_or_default(), button.label.as_str().to_string(), args_of(&button.action)["value"].clone(), button.presence.state == semio_framework_ui_layout::layout::UiState::Disabled)),
            _ => None,
        })
        .collect();
    assert_eq!(buttons, [("targets.remove.a".to_string(), "Remove a".to_string(), serde_json::json!([]), false), ("targets.useSelection".to_string(), "Use current selection".to_string(), serde_json::json!(["n1", "n2"]), false)]);
    let empty = staged_action_arg_row("main", "move", &defs[4], None, &HashMap::new(), Terminology::Native, Locale::De).items.expect("rows");
    assert_eq!(empty.iter().map(|item| item.label.as_str().to_string()).collect::<Vec<_>>(), ["Nichts ausgewählt", "Aktuelle Auswahl verwenden"]);
    assert!(matches!(&empty[1].control, Some(UiControlNode::Button(button)) if button.presence.state == semio_framework_ui_layout::layout::UiState::Disabled), "nothing selected, nothing to use");
}

/// ⚖️ LAW: a stage dispatch stages a segmented option, splices a vector axis into the staged tuple (else the painted
/// one), or takes the editor's own value; "use current selection" reads the argument's domain at its granularity.
#[test]
fn staging_splices_axes_and_reads_the_selection_domain() {
    assert_eq!(staged_arg_value(&serde_json::json!({ "option": "cut", "value": false }), None), serde_json::json!("cut"));
    assert_eq!(staged_arg_value(&serde_json::json!({ "index": 1, "dims": 3, "tuple": [0.0, 0.0, 0.0], "value": "7" }), Some(&serde_json::json!([1.0, 2.0, 3.0]))), serde_json::json!([1.0, 7.0, 3.0]));
    assert_eq!(staged_arg_value(&serde_json::json!({ "index": 0, "dims": 2, "tuple": [4.0, 5.0], "value": 9.0 }), None), serde_json::json!([9.0, 5.0]));
    assert_eq!(staged_arg_value(&serde_json::json!({ "value": ["n1"] }), None), serde_json::json!(["n1"]));
    assert_eq!(reference_selection_ids(&selection(), Some("vortex"), Some("node")), ["n1", "n2"]);
    assert!(reference_selection_ids(&selection(), Some("vortex"), Some("region")).is_empty(), "another granularity is not this argument's");
    assert_eq!(reference_selection_ids(&selection(), None, None), ["n1", "n2"], "the one domain that selects anything");
    let mut shell = session_shell();
    let stage = |args: Value| ActionDescriptor { controller_id: "framework".into(), action: "stageActionArg".into(), args: Some(DslValue::from(args)) };
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(stage(serde_json::json!({ "window": "main", "action": "move", "arg": "offset", "index": 1, "dims": 3, "tuple": [0.0, 0.0, 0.0], "value": 4.0 })))).expect("stages");
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).dispatch_action(stage(serde_json::json!({ "window": "main", "action": "move", "arg": "offset", "index": 2, "dims": 3, "tuple": [0.0, 0.0, 0.0], "value": 5.0 })))).expect("stages");
    assert_eq!(shell.commands.staged_map_for("main", "move").get("offset"), Some(&serde_json::json!([0.0, 4.0, 5.0])), "the second axis keeps the first");
}

/// 🎚️ One row of the shared `🛂️manifest/🧫️fixtures/🧫️number-facets` corpus as the facets a staged row carries (absent = none
/// or the default).
fn corpus_number_facets(value: &Value) -> Option<semio_framework::ActionArgNumberFacets> {
    let facets = value.as_object()?;
    let number = |key: &str| facets.get(key).and_then(Value::as_f64);
    let text = |key: &str| facets.get(key).and_then(Value::as_str).map(ToOwned::to_owned);
    Some(semio_framework::ActionArgNumberFacets {
        min: number("min"),
        max: number("max"),
        step: number("step"),
        appearance: serde_json::from_value(facets["appearance"].clone()).expect("corpus appearance"),
        scale: serde_json::from_value(facets["scale"].clone()).expect("corpus scale"),
        unit: text("unit"),
        display_unit: text("displayUnit"),
        display_factor: number("displayFactor"),
        precision: facets.get("precision").and_then(Value::as_u64).map(|precision| u16::try_from(precision).expect("corpus precision")),
        snaps: facets["snaps"].as_array().expect("corpus snaps").iter().map(|snap| snap.as_f64().expect("corpus snap")).collect(),
        limits: serde_json::from_value(facets["limits"].clone()).expect("corpus limits"),
    })
}

/// ⚖️ LAW: every staged number row carries exactly the facets the shared corpus derives (`ActionArgDef::number_facets`, the
/// mapping the time-travel editor shares) — a slider's or dial's look, axis, travel, step, units, display factor,
/// precision, detents and hard limits with their localized refusals; a stepper's, a number field's and every vector axis'
/// key range, step, precision, detents, shown unit, factor and limits — a staged value beyond a soft travel stays as
/// staged, and an input that is no number carries no limits anywhere.
#[test]
fn every_staged_number_row_carries_the_shared_corpus_facets() {
    let corpus: Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🧫️number-facets/🔣️.json")).expect("the shared number-facets corpus parses");
    let cases = corpus["cases"].as_array().expect("cases");
    let mut kinds = std::collections::BTreeSet::new();
    for case in cases {
        let name = case["name"].as_str().expect("case name");
        let def: ActionArgDef = serde_json::from_value(case["def"].clone()).unwrap_or_else(|error| panic!("{name}: the descriptor decodes: {error}"));
        let locale = if case["locale"] == "de" { Locale::De } else { Locale::En };
        let row = staged_action_arg_row("main", "act", &def, None, &HashMap::new(), Terminology::Native, locale);
        let Some(expected) = corpus_number_facets(&case["facets"]) else {
            kinds.insert("none");
            let controls = row.control.iter().chain(row.items.iter().flatten().filter_map(|item| item.control.as_ref()));
            assert!(controls.into_iter().all(|control| match control {
                UiControlNode::Slider(slider) => slider.limits.is_none(),
                UiControlNode::NumberStepper(stepper) => stepper.limits.is_none(),
                UiControlNode::Input(field) => field.limits.is_none() && field.display_factor.is_none(),
                _ => true,
            }), "{name}: no number facets");
            continue;
        };
        let input = |field: &UiInputNode| (field.min, field.max, field.step, field.precision, field.snaps.clone(), field.display_factor, field.limits.clone());
        let expected_input = (expected.min, expected.max, expected.step, expected.precision, expected.snaps.clone(), expected.display_factor, Some(expected.limits.clone()));
        match (def.control(), &row.control) {
            (control @ (semio_framework::ActionArgControl::Slider { .. } | semio_framework::ActionArgControl::Dial { .. }), Some(UiControlNode::Slider(slider))) => {
                kinds.insert(if matches!(control, semio_framework::ActionArgControl::Dial { .. }) { "dial" } else { "slider" });
                assert_eq!(
                    (slider.min, slider.max, slider.step, slider.appearance, slider.scale, slider.unit.clone(), slider.display_unit.clone(), slider.display_factor, slider.precision, slider.snaps.clone(), slider.limits.clone()),
                    (expected.min.unwrap_or(0.0), expected.max.unwrap_or(0.0), expected.step.unwrap_or(1.0), expected.appearance, expected.scale, expected.unit.clone(), expected.display_unit.clone(), expected.display_factor, expected.precision, expected.snaps.clone(), Some(expected.limits.clone())),
                    "{name}"
                );
            }
            (semio_framework::ActionArgControl::Stepper { .. }, Some(UiControlNode::NumberStepper(stepper))) => {
                kinds.insert("stepper");
                assert_eq!(
                    (stepper.min, stepper.max, stepper.step, stepper.precision, stepper.snaps.clone(), stepper.unit.clone(), stepper.display_factor, stepper.limits.clone()),
                    (expected.min, expected.max, expected.step.unwrap_or(1.0), expected.precision, expected.snaps.clone(), expected.shown_unit().map(ToOwned::to_owned), expected.display_factor, Some(expected.limits.clone())),
                    "{name}"
                );
            }
            (semio_framework::ActionArgControl::Number { .. }, Some(UiControlNode::Input(field))) => {
                kinds.insert("number");
                assert_eq!(input(field), expected_input, "{name}");
            }
            (semio_framework::ActionArgControl::Vector { dims, .. }, None) => {
                kinds.insert("vector");
                let axes = row.items.as_ref().expect("one row per axis");
                assert_eq!(axes.len(), dims as usize, "{name}");
                for axis in axes {
                    let Some(UiControlNode::Input(field)) = &axis.control else { panic!("{name}: an axis is a number field") };
                    assert_eq!(input(field), expected_input, "{name}: {}", axis.id);
                }
            }
            (control, node) => panic!("{name}: {control:?} rendered as {node:?}"),
        }
    }
    assert!(["dial", "slider", "stepper", "number", "vector", "none"].iter().all(|kind| kinds.contains(kind)), "the corpus reaches every number control and a non-number: {kinds:?}");
    let log = cases.iter().find(|case| case["name"] == "log-slider-soft-travel-inside-an-exclusive-floor").expect("the log slider case");
    let def: ActionArgDef = serde_json::from_value(log["def"].clone()).expect("the log slider decodes");
    let Some(UiControlNode::Slider(slider)) = staged_action_arg_row("main", "act", &def, Some(&serde_json::json!(20.0)), &HashMap::new(), Terminology::Native, Locale::En).control else { panic!("a slider") };
    assert_eq!((slider.value, slider.max, slider.limits.as_ref().is_some_and(|limits| limits.crossed(20.0).is_none())), (20.0, 10.0, true), "a value the limits admit beyond the soft travel stays as staged");
}

fn kinds_dialog() -> DialogDefinition {
    DialogDefinition { args: editor_defs(), ..DialogDefinition::new("editors", LocalizedLabel::native("Edit", "Bearbeiten"), semio_framework::ActionRef::new("apply")) }
}

fn press(shell: &mut ShellState, action: ui_wgpu::wgpu::KeyAction) {
    let mut input = InputState::<ActionDescriptor>::default();
    PointerCtx::of(&mut shell).handle_keyboard(action, &PointerModifiers::default(), &mut input);
}

fn focus(shell: &mut ShellState, stop: ChromeDialogStop) {
    shell.chrome.chrome_build.dialog_stack.last_mut().expect("an open dialog").focus_stop(stop);
}

fn staged(shell: &ShellState, id: &str) -> Option<Value> {
    shell.chrome.chrome_build.dialog_stack.last().and_then(|request| request.effective().get(id).map(dsl_value_as_json))
}

/// ⚖️ LAW: a chrome dialog stages every editor kind by keyboard alone — a stepper steps inside its bounds, a slider
/// steps, jumps between its detents and to its ends, a segmented field moves between its options, each vector axis is
/// its own labelled field, and a reference field stages the live selection whose chips are focus stops of their own
/// that remove themselves — and the submit carries all of it.
#[test]
fn a_chrome_dialog_stages_every_editor_kind_by_keyboard() {
    let mut shell = session_shell();
    shell.input.interaction_selection = selection();
    let request = ChromeDialogRequest::from_definition("test", &kinds_dialog(), None, Terminology::Native, Locale::En);
    assert_eq!(request.fields.iter().map(|field| field.label.as_str()).collect::<Vec<_>>(), ["Count", "Angle", "Mode", "Offset x", "Offset y", "Offset z", "Targets"]);
    shell.chrome.chrome_build.open_dialog(request);

    focus(&mut shell, ChromeDialogStop::Field(0));
    for action in [ui_wgpu::wgpu::KeyAction::ArrowUp, ui_wgpu::wgpu::KeyAction::ArrowUp, ui_wgpu::wgpu::KeyAction::ArrowUp, ui_wgpu::wgpu::KeyAction::ArrowDown] {
        press(&mut shell, action);
    }
    assert_eq!(staged(&shell, "count"), Some(serde_json::json!(2.0)));

    focus(&mut shell, ChromeDialogStop::Field(1));
    let mut walk = Vec::new();
    for action in [ui_wgpu::wgpu::KeyAction::ArrowRight, ui_wgpu::wgpu::KeyAction::PageUp, ui_wgpu::wgpu::KeyAction::PageUp, ui_wgpu::wgpu::KeyAction::Home, ui_wgpu::wgpu::KeyAction::End, ui_wgpu::wgpu::KeyAction::PageDown] {
        press(&mut shell, action);
        walk.push(staged(&shell, "angle").and_then(|value| value.as_f64()).unwrap_or(f64::NAN));
    }
    assert_eq!(walk, [5.0, 45.0, 90.0, 0.0, 90.0, 45.0], "step, detent, detent, start, end, detent");

    focus(&mut shell, ChromeDialogStop::Field(2));
    press(&mut shell, ui_wgpu::wgpu::KeyAction::ArrowRight);
    press(&mut shell, ui_wgpu::wgpu::KeyAction::ArrowRight);
    assert_eq!(staged(&shell, "mode"), Some(serde_json::json!("cut")));

    focus(&mut shell, ChromeDialogStop::Field(4));
    press(&mut shell, ui_wgpu::wgpu::KeyAction::Char("7".into()));
    assert_eq!(staged(&shell, "offset"), Some(serde_json::json!([0.0, 7.0, 0.0])));

    focus(&mut shell, ChromeDialogStop::Field(6));
    press(&mut shell, ui_wgpu::wgpu::KeyAction::Enter);
    assert_eq!(staged(&shell, "targets"), Some(serde_json::json!(["n1", "n2"])), "Enter on a reference field uses the current selection");
    let stops = shell.chrome.chrome_build.dialog_stack.last().expect("dialog").stops();
    assert!(stops.contains(&ChromeDialogStop::Chip(6, 0)) && stops.contains(&ChromeDialogStop::Chip(6, 1)), "each chip is a focus stop");
    press(&mut shell, ui_wgpu::wgpu::KeyAction::Tab);
    assert_eq!(shell.chrome.chrome_build.dialog_stack.last().map(ChromeDialogRequest::focused), Some(ChromeDialogStop::Chip(6, 0)));
    press(&mut shell, ui_wgpu::wgpu::KeyAction::Backspace);
    assert_eq!(staged(&shell, "targets"), Some(serde_json::json!(["n2"])));
    assert_eq!(shell.chrome.chrome_build.dialog_stack.last().map(ChromeDialogRequest::focused), Some(ChromeDialogStop::Chip(6, 0)), "focus stays on the chip that took its place");

    focus(&mut shell, ChromeDialogStop::Confirm);
    press(&mut shell, ui_wgpu::wgpu::KeyAction::Enter);
    let submit = shell.settle.deferred_actions.pop().expect("the submit dispatched");
    assert_eq!(submit.action, "apply");
    assert_eq!(args_of(&submit), serde_json::json!({ "count": 2.0, "angle": 45.0, "mode": "cut", "offset": [0.0, 7.0, 0.0], "targets": ["n2"] }));
}

/// ⚖️ LAW: the dialog paints each editor accessibly — a slider with one tick per detent announcing its range and value,
/// spin buttons for numbers and axes, pressed segments, named stepper buttons and removable chips — and a pointer picks a
/// segment, steps a stepper and moves a slider through the shared detent law.
#[test]
fn a_chrome_dialog_paints_each_editor_accessibly_and_answers_the_pointer() {
    let mut shell = session_shell();
    shell.input.interaction_selection = selection();
    let mut request = ChromeDialogRequest::from_definition("test", &kinds_dialog(), None, Terminology::Native, Locale::De);
    request.use_selection(6, &selection());
    let theme = Theme::light();
    let ops = chrome_dialog_paint_ops(&request, 1440.0, 900.0, &theme, &mut FontAtlas::builtin());
    let semantics = |key: &str| {
        ops.iter()
            .find_map(|op| match op {
                ChromeDialogPaintOp::Hit { control_id, semantics, label, .. } if control_id == key => Some((label.clone(), semantics.clone())),
                _ => None,
            })
            .unwrap_or_else(|| panic!("{key} paints a hit"))
    };
    let (label, slider) = semantics("shell.dialog.editors.field.angle");
    assert_eq!((label.as_str(), slider.role, slider.value_range, slider.value_text.as_deref()), ("Winkel", Some("slider"), Some([0.0, 90.0, 0.0]), Some("0")));
    assert_eq!(semantics("shell.dialog.editors.field.count").1.role, Some("spinbutton"));
    assert_eq!(semantics("shell.dialog.editors.field.offset.y").0, "Versatz y");
    assert_eq!(semantics("shell.dialog.editors.field.targets.chip.n1").0, "n1 entfernen");
    assert_eq!(semantics("shell.dialog.editors.field.targets").0, "Ziele: Aktuelle Auswahl verwenden");
    let ticks = ops.iter().filter(|op| matches!(op, ChromeDialogPaintOp::Fill { rect, color } if rect.w == 2.0 && *color == theme.text_muted)).count();
    assert_eq!(ticks, 3, "one tick per detent");
    let segments: Vec<(String, bool)> = ops.iter().filter_map(|op| if let ChromeDialogPaintOp::Segment { label, selected, .. } = op { Some((label.clone(), *selected)) } else { None }).collect();
    assert_eq!(segments, [("Modus: Hinzufügen".to_string(), false), ("Modus: Schneiden".to_string(), false)]);
    let nudges: Vec<String> = ops.iter().filter_map(|op| if let ChromeDialogPaintOp::Nudge { label, .. } = op { Some(label.clone()) } else { None }).collect();
    assert_eq!(nudges, ["Verringern Anzahl", "Erhöhen Anzahl"]);

    let dialog = ops.iter().find_map(|op| if let ChromeDialogPaintOp::Modal(rect) = op { Some(*rect) } else { None }).expect("the dialog box");
    let centre = |rect: Rect| (rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
    let cut = ops.iter().find_map(|op| if let ChromeDialogPaintOp::Segment { rect, option: 1, .. } = op { Some(*rect) } else { None }).expect("the second segment");
    let increase = ops.iter().find_map(|op| if let ChromeDialogPaintOp::Nudge { rect, sign, .. } = op { (*sign > 0.0).then_some(*rect) } else { None }).expect("the increase button");
    let rail = ops.iter().find_map(|op| if let ChromeDialogPaintOp::Hit { rect, kind: HitKind::Slider, .. } = op { Some(*rect) } else { None }).expect("the slider rail");
    shell.chrome.chrome_build.open_dialog(request);
    for (x, y) in [centre(cut), centre(increase), (rail.x + rail.w * 0.49, rail.y + rail.h * 0.5)] {
        shell.input.resolve_chrome_dialog_click(&ops, dialog, x, y, &mut shell.chrome, &mut shell.settle);
    }
    assert_eq!(staged(&shell, "mode"), Some(serde_json::json!("cut")));
    assert_eq!(staged(&shell, "count"), Some(serde_json::json!(1.0)));
    assert_eq!(staged(&shell, "angle"), Some(serde_json::json!(45.0)), "the pointer lands on the nearby detent");
    assert!(shell.chrome.chrome_build.dialog_open(), "editing a field never closes the dialog");
}
//#endregion 🎛️StagedEditors
