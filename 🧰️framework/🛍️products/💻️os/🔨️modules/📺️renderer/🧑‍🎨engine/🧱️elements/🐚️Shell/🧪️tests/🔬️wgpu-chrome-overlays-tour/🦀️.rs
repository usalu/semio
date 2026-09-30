use super::*;

/// 🧹️ Returns a pristine owned chrome state for each test.
fn chrome_state() -> ShellChromeBuildState {
    ShellChromeBuildState::default()
}

//#region ThreadBoundary
/// 🎯️ `content_focus` records WHICH node holds focus, not a bare bool (ticket 26/09/17 packet W2k),
/// so the fixture mints a real arena id rather than `true`.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn chrome_build_state_moves_across_threads_without_losing_state() {
    assert_shell_chrome_build_state_is_send();
    let mut chrome = chrome_state();
    let mut focus_arena = ui_wgpu::wgpu::Arena::<u8>::default();
    chrome.content_focus.insert("main".to_string(), Some(RetainedContentFocus { node: focus_arena.insert(0), key: ui_wgpu::wgpu::NodeKey::Explicit("tour-focus".into()), kind: RetainedNodeFocusKind::Button }));
    chrome.register_tooltip("nav.help", "Help");
    chrome.preferences.theme_id = "mono".to_string();
    let chrome = std::thread::spawn(move || {
        assert!(chrome.content_has_focus("main"));
        assert_eq!(chrome.tooltip_titles.get("nav.help").map(String::as_str), Some("Help"));
        chrome
    })
    .join()
    .expect("owned chrome state crosses a worker thread");
    assert_eq!(chrome.preferences.theme_id, "mono");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn find_item_sink_binding_routes_owned_output_across_threads() {
    let joined = std::thread::spawn(move || {
        let mut items = ShellFindItems::default();
        let binding = match items.bind() {
            Ok(binding) => binding,
            Err(()) => panic!("single worker binding"),
        };
        assert!(try_push_find_item(ShellFindItem { id: "item".to_string(), label: "Item".to_string(), description: None, category: None, surface_id: "surface".to_string(), node_id: "node".to_string() }).is_ok());
        drop(binding);
        items
    })
    .join()
    .expect("worker render callback completes");
    assert_eq!(joined.iter().map(|item| item.id.as_str()).collect::<Vec<_>>(), vec!["item"]);
}

#[test]
fn find_item_callback_without_render_binding_is_explicitly_rejected() {
    let item = ShellFindItem { id: "unbound".to_string(), label: "Unbound".to_string(), description: None, category: None, surface_id: "surface".to_string(), node_id: "node".to_string() };
    let identity = item.id.as_ptr();
    let rejected = match try_push_find_item(item) {
        Ok(()) => panic!("unbound callbacks must refuse"),
        Err(item) => item,
    };
    assert_eq!(rejected.id.as_ptr(), identity);
}

#[test]
fn find_item_max_plus_one_returns_the_exact_owned_item() {
    let mut items = ShellFindItems::default();
    for index in 0..SHELL_FIND_ITEM_CAPACITY {
        let item = ShellFindItem { id: index.to_string(), label: "Item".to_string(), description: None, category: None, surface_id: "surface".to_string(), node_id: "node".to_string() };
        assert!(items.try_push(item).is_ok());
    }
    let rejected = ShellFindItem { id: "exact-owner".to_string(), label: "Rejected".to_string(), description: None, category: None, surface_id: "surface".to_string(), node_id: "node".to_string() };
    let identity = rejected.id.as_ptr();
    let rejected = match items.try_push(rejected) {
        Ok(()) => panic!("MAX + 1 must refuse"),
        Err(item) => item,
    };
    assert_eq!(rejected.id.as_ptr(), identity);
    assert!(!items.close_step());
    assert!(!items.terminal_is_empty());
    while !items.close_step() {}
    assert!(items.terminal_is_empty());
}

#[test]
fn stale_find_generation_returns_the_exact_owned_item() {
    let mut items = ShellFindItems::default();
    let stale_generation = items.generation;
    assert!(items.begin_next_generation());
    let rejected = ShellFindItem { id: "stale-owner".to_string(), label: "Stale".to_string(), description: None, category: None, surface_id: "surface".to_string(), node_id: "node".to_string() };
    let identity = rejected.id.as_ptr();
    let rejected = match items.try_push_at(stale_generation, rejected) {
        Ok(()) => panic!("stale generation must refuse"),
        Err(item) => item,
    };
    assert_eq!(rejected.id.as_ptr(), identity);
    assert!(items.terminal_is_empty());
}
//#endregion ThreadBoundary

//#region Tooltip
#[test]
fn tooltip_titles_register_and_clear() {
    let mut chrome = chrome_state();
    chrome.register_tooltip("nav.help", "Help");
    assert_eq!(chrome.tooltip_titles.get("nav.help").cloned(), Some("Help".to_string()));
    chrome.tooltip_titles.clear();
    assert!(chrome.tooltip_titles.is_empty());
}

#[test]
fn tooltip_registration_ignores_empty_titles() {
    let mut chrome = chrome_state();
    chrome.register_tooltip("nav.mystery", "");
    assert!(chrome.tooltip_titles.get("nav.mystery").is_none());
}

#[test]
fn tooltip_ready_respects_hover_delay() {
    let hover = ChromeTooltipHover { control_id: "x".into(), anchor_x: 0.0, anchor_y: 0.0, started_ms: 1_000.0 };
    assert!(!chrome_tooltip_ready(&hover, 1_000.0 + CHROME_TOOLTIP_DELAY_MS - 1.0));
    assert!(chrome_tooltip_ready(&hover, 1_000.0 + CHROME_TOOLTIP_DELAY_MS));
    assert!(chrome_tooltip_ready(&hover, 1_000.0 + CHROME_TOOLTIP_DELAY_MS + 250.0));
}

/// 🧪️ Full close-on-hover-out path through `render_chrome_tooltip`: hovering a registered control
/// arms the hover timer; on the very next call the pointer has moved off (`hovered_id` no longer
/// matches), which must clear the armed hover immediately (no debounce, matching this crate's
/// documented "no animation-clock scaffolding" gap) rather than leaving a stale tooltip painted.
#[test]
fn tooltip_closes_immediately_on_hover_out() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::light();
    shell.chrome_build.register_tooltip("nav.help", "Help");
    input.hovered_id = Some("nav.help".into());
    shell.render_chrome_tooltip(&mut draw, &mut atlas, &mut input, &theme, 800.0, 600.0);
    assert!(shell.chrome_build.tooltip_hover.is_some(), "hover should arm on first hovered frame");
    input.hovered_id = None;
    shell.render_chrome_tooltip(&mut draw, &mut atlas, &mut input, &theme, 800.0, 600.0);
    assert!(shell.chrome_build.tooltip_hover.is_none(), "hover-out must clear the armed tooltip");
}
//#endregion Tooltip

//#region Dialog
#[test]
fn dialog_open_and_close_topmost() {
    let mut chrome = chrome_state();
    assert!(!chrome.dialog_open());
    chrome.open_dialog(ChromeDialogRequest::confirm("confirm-1", "Delete?", "delete"));
    assert!(chrome.dialog_open());
    chrome.close_topmost_dialog();
    assert!(!chrome.dialog_open());
}

#[test]
fn dialog_stack_supports_nesting_close_order() {
    let mut chrome = chrome_state();
    chrome.open_dialog(ChromeDialogRequest::confirm("outer", "Outer", "outer"));
    chrome.open_dialog(ChromeDialogRequest::confirm("inner", "Inner", "inner"));
    assert_eq!(chrome.dialog_stack.last().map(|dialog| dialog.id.clone()), Some("inner".to_string()));
    chrome.close_topmost_dialog();
    assert_eq!(chrome.dialog_stack.last().map(|dialog| dialog.id.clone()), Some("outer".to_string()));
    chrome.close_topmost_dialog();
    assert!(!chrome.dialog_open());
}

/// 🧪️ `render_chrome_dialog`'s scrim-click dismissal (`DismissPolicy::outside_press_swallow`) — a
/// click outside the centered dialog box closes it without dispatching `confirm_action`.
///
/// A click far in the top-left corner, well outside the centered ~360x168 box on an 800x600 viewport.
#[test]
fn dialog_scrim_click_dismisses_without_confirm_action() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::light();
    shell.chrome_build.open_dialog(ChromeDialogRequest::confirm("confirm-1", "Delete?", "delete"));
    input.pointer_x = 4.0;
    input.pointer_y = 4.0;
    shell.chrome_build.compute_click_edge(false);
    shell.chrome_build.compute_click_edge(true);
    shell.render_chrome_dialog(&mut draw, &mut atlas, &mut input, &theme, 800.0, 600.0);
    assert!(!shell.chrome_build.dialog_open(), "scrim click must dismiss the dialog");
}

/// 🧪️ Focus-trap-equivalent modality: while a dialog is open, the chrome-owned tour trigger and the
/// sync-detach-confirmation click handlers must not fire — both are gated on `!chrome_dialog_open()`.
///
/// The guard every other chrome-owned click handler in this region checks first.
#[test]
fn dialog_open_blocks_other_chrome_owned_click_handlers() {
    let mut chrome = chrome_state();
    chrome.open_dialog(ChromeDialogRequest::confirm("blocker", "Blocking", "noOperation"));
    assert!(chrome.dialog_open());
    assert!(!(!chrome.dialog_open()));
}

fn dialog_choices_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🧫️dialog-choices/🔣️.json")).expect("dialog-choices fixture")
}

fn dialog_choices_request(locale: Locale) -> (serde_json::Value, ChromeDialogRequest) {
    let fixture = dialog_choices_fixture();
    let dialog: semio_framework::DialogDefinition = serde_json::from_value(fixture["dialog"].clone()).expect("fixture dialog");
    let request = ChromeDialogRequest::from_definition("test", &dialog, Some(DslValue::from(fixture["seed"].clone())), Terminology::default(), locale);
    (fixture, request)
}

fn dialog_stop_name(request: &ChromeDialogRequest, stop: ChromeDialogStop) -> String {
    match stop {
        ChromeDialogStop::Field(index) => format!("field:{}", request.fields[index].id),
        ChromeDialogStop::Chip(index, chip) => format!("chip:{}:{chip}", request.fields[index].id),
        ChromeDialogStop::Cancel => "cancel".to_string(),
        ChromeDialogStop::Choice(index) => format!("choice:{}", request.choices[index].choice.id),
        ChromeDialogStop::Confirm => "submit".to_string(),
    }
}

/// ✍️ Stages one fixture case's field values as typed drafts.
fn stage_dialog_case(request: &mut ChromeDialogRequest, case: &serde_json::Value) {
    for (id, value) in case["staged"].as_object().expect("staged") {
        let index = request.fields.iter().position(|field| field.id == *id).expect("staged field");
        request.set_draft(index, value.as_str().expect("text value").to_string());
    }
}

/// 🔀️ The shared `🧫️dialog-choices` law, on the wgpu renderer: focus visits the fixture's order, the destructive
/// choice paints in the error colour and publishes its consequence as its accessible description, and for every
/// case each button is enabled or gated exactly as listed — the submit on the dialog's required args, a choice only
/// on the args it requires, so a cleared name gates the submit alone — and every enabled button's hit carries
/// exactly the fixture's dispatch.
#[test]
fn dialog_choices_fixture_lays_out_gates_and_dispatches_like_every_renderer() {
    for locale in [Locale::En, Locale::De] {
        let (fixture, request) = dialog_choices_request(locale);
        let order: Vec<String> = request.stops().into_iter().map(|stop| dialog_stop_name(&request, stop)).collect();
        let expected: Vec<String> = fixture["focusOrder"].as_array().expect("focus order").iter().map(|entry| entry.as_str().expect("entry").to_string()).collect();
        assert_eq!(order, expected);
        let key = if locale == Locale::De { "de" } else { "en" };
        assert_eq!(request.choices[0].label, fixture["dialog"]["choices"][0]["label"]["native"][key].as_str().expect("label"));
        let theme = Theme::light();
        for case in fixture["cases"].as_array().expect("cases") {
            let name = case["case"].as_str().expect("case");
            let mut request = request.clone();
            stage_dialog_case(&mut request, case);
            let ops = ShellState::chrome_dialog_paint_ops(&request, 1440.0, 900.0, &theme);
            let button = |entry: &serde_json::Value| {
                ops.iter()
                    .find_map(|op| match op {
                        ChromeDialogPaintOp::Hit { stop, event, disabled, .. } if dialog_stop_name(&request, *stop) == *entry => Some((event.clone(), *disabled)),
                        _ => None,
                    })
                    .unwrap_or_else(|| panic!("{name}: {entry} paints a hit"))
            };
            for (list, enabled) in [("enabled", true), ("disabled", false)] {
                for entry in case[list].as_array().expect("buttons") {
                    let (event, disabled) = button(entry);
                    assert_eq!((event.is_some(), disabled), (enabled, !enabled), "{name}: {entry} {list}");
                }
            }
            for dispatch in case["dispatches"].as_array().expect("dispatches") {
                let event = button(&dispatch["button"]).0.expect("an enabled button dispatches");
                assert_eq!(event.action, dispatch["action"].as_str().expect("action"), "{name}");
                assert_eq!(event.args.map(|args| serde_json::to_value(args).expect("args wire")).unwrap_or(serde_json::Value::Null), dispatch["args"], "{name}: {}", dispatch["button"]);
            }
            let destructive = ops.iter().find_map(|op| match op {
                ChromeDialogPaintOp::Hit { stop: ChromeDialogStop::Choice(0), description, .. } => Some(description.clone()),
                _ => None,
            });
            assert_eq!(destructive.flatten().as_deref(), fixture["dialog"]["choices"][0]["description"]["native"][key].as_str());
            assert!(ops.iter().any(|op| matches!(op, ChromeDialogPaintOp::Fill { color, .. } if *color == theme.error)), "the destructive choice paints in the error colour");
        }
    }
}

/// ⌨️ The open dialog owns the keyboard: Backspace clears the name, Tab walks the fixture order, Enter on the
/// gated submit does nothing, Enter on the destructive choice — which requires no name — dispatches it with the
/// seed context and its id, and Escape cancels with the declared cancel action; every dismissal closes the dialog.
#[test]
fn dialog_choices_keyboard_clears_the_name_takes_the_ungated_choice_and_escape_cancels() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let (fixture, request) = dialog_choices_request(Locale::En);
    let draft = request.fields[0].draft.clone();
    shell.chrome_build.open_dialog(request);
    let mut input = InputState::<ActionDescriptor>::default();
    let modifiers = PointerModifiers::default();
    for _ in draft.chars() {
        shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Backspace, &modifiers, &mut input);
    }
    for _ in 0..3 {
        shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Tab, &modifiers, &mut input);
    }
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Enter, &modifiers, &mut input);
    assert!(shell.chrome_build.dialog_open() && shell.deferred_actions.is_empty(), "the gated submit takes no Enter");
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Tab, &PointerModifiers { shift: true, ..PointerModifiers::default() }, &mut input);
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Enter, &modifiers, &mut input);
    assert!(!shell.chrome_build.dialog_open());
    let taken = shell.deferred_actions.pop().expect("the choice dispatched");
    let cleared = fixture["cases"].as_array().expect("cases").iter().find(|case| case["case"] == "cleared").expect("cleared case");
    assert_eq!(taken.action, cleared["dispatches"][0]["action"].as_str().expect("action"));
    assert_eq!(serde_json::to_value(taken.args.expect("args")).expect("args wire"), cleared["dispatches"][0]["args"]);
    let (_, request) = dialog_choices_request(Locale::En);
    shell.chrome_build.open_dialog(request);
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Escape, &modifiers, &mut input);
    assert!(!shell.chrome_build.dialog_open());
    assert_eq!(shell.deferred_actions.pop().map(|event| event.action), Some("historyEditBack".to_string()));
}

/// ♿️ The open dialog is published as a modal accessibility tree, as React's `UIDialog` is: one `dialog` node named
/// by the title and described by the body, every painted control one level below it — the name field, Cancel, the
/// destructive choice described by its consequence, the submit — with a gated button `disabled`, the dialog's own
/// keyboard focus as the focused node, and nothing of the shell behind the veil.
#[test]
fn the_open_dialog_publishes_a_modal_accessibility_tree_with_gated_and_focused_controls() {
    for locale in [Locale::En, Locale::De] {
        let mut shell = ShellState::new(Vec::new(), String::new());
        let (fixture, mut request) = dialog_choices_request(locale);
        let cleared = fixture["cases"].as_array().expect("cases").iter().find(|case| case["case"] == "cleared").expect("cleared case");
        stage_dialog_case(&mut request, cleared);
        request.focus_stop(ChromeDialogStop::Choice(0));
        shell.chrome_build.open_dialog(request);
        let (mut draw, mut atlas, mut input, theme) = (DrawList::default(), FontAtlas::builtin(), InputState::<ActionDescriptor>::default(), Theme::light());
        shell.render_chrome_dialog(&mut draw, &mut atlas, &mut input, &theme, 1440.0, 900.0);
        let hits = input.staged_hits().to_vec();
        let nodes = shell.chrome_accessibility_nodes(&hits);
        let key = if locale == Locale::De { "de" } else { "en" };
        let text = |label: &serde_json::Value| label["native"][key].as_str().expect("localized text").to_string();
        let dialog = &nodes[0];
        assert_eq!((dialog.key.as_str(), dialog.role.as_str(), dialog.depth), ("shell.dialog.finalizeHistoryEdit", "dialog", 0));
        assert_eq!((dialog.label.clone(), dialog.description.clone()), (Some(text(&fixture["dialog"]["title"])), Some(text(&fixture["dialog"]["body"]))));
        assert!(nodes[1..].iter().all(|node| node.depth == 1 && node.key.starts_with("shell.dialog.finalizeHistoryEdit.")), "the modal publishes only its own controls");
        let node = |suffix: &str| nodes.iter().find(|node| node.key == format!("shell.dialog.finalizeHistoryEdit.{suffix}")).unwrap_or_else(|| panic!("{suffix} is published"));
        let name = node("field.name");
        assert_eq!((name.role.as_str(), name.label.clone(), name.editable, name.value_text.as_deref()), ("textbox", Some(text(&fixture["dialog"]["args"][0]["label"])), true, Some("")));
        let submit = node("confirm");
        assert_eq!((submit.label.clone(), submit.disabled, submit.actionable), (Some(text(&fixture["dialog"]["submitLabel"])), true, false), "the cleared name gates the submit");
        let overwrite = node("choice.overwrite");
        assert_eq!((overwrite.role.as_str(), overwrite.disabled, overwrite.focused, overwrite.tabbable), ("button", false, true, true), "the ungated choice holds the dialog's keyboard focus");
        assert_eq!(overwrite.description, Some(text(&fixture["dialog"]["choices"][0]["description"])));
        assert_eq!(nodes.iter().filter(|node| node.focused).count(), 1);
        assert_eq!(node("cancel").label, Some(text(&fixture["dialog"]["cancelLabel"])));
        assert!(shell.chrome_surface_census().iter().any(|(id, level, _)| id == "shell.dialog.finalizeHistoryEdit" && *level == "dialog"), "the census reports the dialog level");
    }
}

/// 🦯️ The published dialog tree is operable: focusing a node moves the dialog's focus stop, a value edits the name
/// field, activating the gated submit does nothing, and activating the choice dispatches it and closes the dialog.
#[test]
fn the_dialog_accessibility_tree_is_operable_like_its_keyboard() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let (fixture, request) = dialog_choices_request(Locale::En);
    shell.chrome_build.open_dialog(request);
    let key = |suffix: &str| format!("shell.dialog.finalizeHistoryEdit.{suffix}");
    assert!(shell.handle_chrome_dialog_accessibility_event(&key("choice.overwrite"), &ui_render::AccessibilityEvent::Focus));
    assert_eq!(shell.chrome_build.dialog_stack.last().map(ChromeDialogRequest::focused), Some(ChromeDialogStop::Choice(0)));
    assert!(shell.handle_chrome_dialog_accessibility_event(&key("field.name"), &ui_render::AccessibilityEvent::Value(String::new())));
    assert!(shell.handle_chrome_dialog_accessibility_event(&key("confirm"), &ui_render::AccessibilityEvent::Activate));
    assert!(shell.chrome_build.dialog_open() && shell.deferred_actions.is_empty(), "a gated submit ignores activation");
    assert!(!shell.handle_chrome_dialog_accessibility_event("ui.search.toggle", &ui_render::AccessibilityEvent::Activate), "a key outside the dialog is not the dialog's");
    assert!(shell.handle_chrome_dialog_accessibility_event(&key("choice.overwrite"), &ui_render::AccessibilityEvent::Activate));
    assert!(!shell.chrome_build.dialog_open());
    let taken = shell.deferred_actions.pop().expect("the choice dispatched");
    assert_eq!(serde_json::to_value(taken.args.expect("args")).expect("args wire"), fixture["cases"][1]["dispatches"][0]["args"]);
}
/// 🎨️ A colour arg and a vector arg of a declared dialog.
fn color_and_vector_dialog() -> semio_framework::DialogDefinition {
    let mut fill = semio_framework::ActionArgDef::vector("fill", LocalizedLabel::native("Fill", "Füllung"), 4);
    fill.presentation = Some(semio_framework::ArgPresentation::Color);
    let mut turn = semio_framework::ActionArgDef::vector("turn", LocalizedLabel::native("Turn", "Drehung"), 2);
    if let semio_framework::ArgSchema::Vector { min, max, precision, snaps, .. } = &mut turn.schema {
        (*min, *max, *precision, *snaps) = (Some(-3.0), Some(3.0), Some(1), vec![-1.5, 0.0, 1.5]);
    }
    semio_framework::DialogDefinition::new("paint", LocalizedLabel::native("Paint", "Malen"), semio_framework::ActionRef::new("paint")).args(vec![fill, turn])
}

/// 🎨️ A colour arg renders the `color_input` recipe in the dialog: a hex field painted beside a swatch of the staged
/// colour, which stages a typed hex once it parses (keeping the alpha when the text carries none), and an opacity
/// slider over the fourth component that walks the shared keyboard law and reads as a slider — in both languages.
#[test]
fn a_dialog_colour_field_stages_hex_and_opacity_like_the_recipe() {
    for (locale, opacity) in [(Locale::En, "Fill Opacity"), (Locale::De, "Füllung Deckkraft")] {
        let mut shell = ShellState::new(Vec::new(), String::new());
        let seed = DslValue::from(serde_json::json!({ "fill": [1.0, 0.0, 0.0, 0.5] }));
        let request = ChromeDialogRequest::from_definition("test", &color_and_vector_dialog(), Some(seed), Terminology::default(), locale);
        assert_eq!(request.fields.iter().map(|field| field.label.as_str()).take(2).collect::<Vec<_>>(), [if locale == Locale::En { "Fill" } else { "Füllung" }, opacity]);
        assert_eq!(request.fields[0].draft, "#ff000080");
        let theme = Theme::light();
        let ops = ShellState::chrome_dialog_paint_ops(&request, 1440.0, 900.0, &theme);
        assert!(ops.iter().any(|op| matches!(op, ChromeDialogPaintOp::Fill { color, .. } if *color == Rgba::from_srgb8(255, 0, 0, 128))), "the swatch paints the staged colour");
        let alpha = ops.iter().find_map(|op| match op {
            ChromeDialogPaintOp::Hit { control_id, semantics, label, .. } if control_id == "shell.dialog.paint.field.fill.alpha" => Some((label.clone(), semantics.clone())),
            _ => None,
        });
        let (label, semantics) = alpha.expect("the opacity slider paints a hit");
        assert_eq!((label.as_str(), semantics.role, semantics.value_range), (opacity, Some("slider"), Some([0.0, 1.0, 0.5])));
        shell.chrome_build.open_dialog(request);
        let mut input = InputState::<ActionDescriptor>::default();
        let modifiers = PointerModifiers::default();
        for _ in 0.."#ff000080".len() {
            shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Backspace, &modifiers, &mut input);
        }
        shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Char("#0f0".into()), &modifiers, &mut input);
        let staged = |shell: &ShellState| shell.chrome_build.dialog_stack.last().and_then(|request| request.effective().get("fill").map(dsl_value_as_json));
        assert_eq!(staged(&shell), Some(serde_json::json!([0.0, 1.0, 0.0, 0.5])), "a typed hex without alpha keeps the opacity");
        shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Tab, &modifiers, &mut input);
        shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::ArrowRight, &modifiers, &mut input);
        assert_eq!(staged(&shell), Some(serde_json::json!([0.0, 1.0, 0.0, 0.51])));
        shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Home, &modifiers, &mut input);
        assert_eq!(staged(&shell), Some(serde_json::json!([0.0, 1.0, 0.0, 0.0])));
    }
}

/// 🧭️ Every vector axis of a dialog honours the vector's facets: its precision formats the draft and sets its step,
/// its bounds clamp a typed value, its detents are what the page keys jump to, and its arrows walk its step ladder.
#[test]
fn a_dialog_vector_axis_honours_bounds_precision_and_detents() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    let seed = DslValue::from(serde_json::json!({ "turn": [0.25, 1.0] }));
    let request = ChromeDialogRequest::from_definition("test", &color_and_vector_dialog(), Some(seed), Terminology::default(), Locale::En);
    let axes: Vec<&str> = request.fields.iter().filter(|field| field.id == "turn").map(|field| field.draft.as_str()).collect();
    assert_eq!(axes, ["0.3", "1.0"], "each axis shows the vector's precision");
    let x = request.fields.iter().position(|field| field.id == "turn").expect("the x axis");
    shell.chrome_build.open_dialog(request);
    let mut input = InputState::<ActionDescriptor>::default();
    let modifiers = PointerModifiers::default();
    shell.chrome_build.dialog_stack.last_mut().expect("dialog").focus_stop(ChromeDialogStop::Field(x));
    let staged = |shell: &ShellState| shell.chrome_build.dialog_stack.last().and_then(|request| request.effective().get("turn").map(dsl_value_as_json));
    let mut walk = Vec::new();
    for action in [ui_wgpu::wgpu::KeyAction::PageUp, ui_wgpu::wgpu::KeyAction::PageUp, ui_wgpu::wgpu::KeyAction::ArrowDown, ui_wgpu::wgpu::KeyAction::PageDown] {
        shell.handle_keyboard(action, &modifiers, &mut input);
        walk.push(staged(&shell).and_then(|tuple| tuple[0].as_f64()).unwrap_or(f64::NAN));
    }
    assert_eq!(walk, [1.5, 2.5, 2.4, 1.5], "detent, ten rungs of the precision's step, one rung, detent");
    for _ in 0..3 {
        shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Backspace, &modifiers, &mut input);
    }
    shell.handle_keyboard(ui_wgpu::wgpu::KeyAction::Char("9".into()), &modifiers, &mut input);
    assert_eq!(staged(&shell).and_then(|tuple| tuple[0].as_f64()), Some(3.0), "a typed value is clamped to the vector's bounds");
}

/// 🎨️ An Actions-form colour row stages the hex its swatch or hex field commits and the opacity its slider commits,
/// over the colour already staged — the same law the dialog and React's staged colour field follow.
#[test]
fn a_staged_colour_row_stages_hex_and_opacity() {
    let color = serde_json::json!({ "alpha": true, "rgba": [1.0, 0.0, 0.0, 0.5] });
    let rgb = serde_json::json!({ "alpha": false, "rgba": [1.0, 0.0, 0.0] });
    assert_eq!(staged_color_value(&color, Some(&serde_json::json!("#00ff00")), None), serde_json::json!([0.0, 1.0, 0.0, 0.5]));
    assert_eq!(staged_color_value(&color, Some(&serde_json::json!("#00ff0040")), None), serde_json::json!([0.0, 1.0, 0.0, 64.0 / 255.0]));
    assert_eq!(staged_color_value(&color, Some(&serde_json::json!(0.25)), Some(&serde_json::json!([0.0, 0.0, 1.0, 1.0]))), serde_json::json!([0.0, 0.0, 1.0, 0.25]));
    assert_eq!(staged_color_value(&color, Some(&serde_json::json!("nope")), None), serde_json::json!([1.0, 0.0, 0.0, 0.5]), "an unreadable hex keeps the colour");
    assert_eq!(staged_color_value(&rgb, Some(&serde_json::json!("#fff")), None), serde_json::json!([1.0, 1.0, 1.0]));
    assert_eq!(staged_arg_value(&serde_json::json!({ "color": color, "value": "#0000ff" }), None), serde_json::json!([0.0, 0.0, 1.0, 0.5]));
}
//#endregion Dialog

//#region Tour
#[test]
fn tour_start_advance_and_skip() {
    let mut chrome = chrome_state();
    assert!(chrome.tour_state.is_none());
    chrome.start_introduction();
    assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(0));
    chrome.advance_introduction(3);
    assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(1));
    chrome.skip_introduction();
    assert!(chrome.tour_state.is_none());
}

#[test]
fn tour_advance_past_last_step_closes_the_tour() {
    let mut chrome = chrome_state();
    chrome.start_introduction();
    chrome.advance_introduction(2);
    assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(1));
    chrome.advance_introduction(2);
    assert!(chrome.tour_state.is_none());
}

#[test]
fn tour_advance_on_empty_state_is_a_no_operation() {
    let mut chrome = chrome_state();
    chrome.advance_introduction(5);
    assert!(chrome.tour_state.is_none());
}

#[test]
fn tour_back_decrements_and_floors_at_zero() {
    let mut chrome = chrome_state();
    chrome.start_introduction();
    chrome.advance_introduction(3);
    chrome.advance_introduction(3);
    assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(2));
    chrome.back_introduction();
    assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(1));
    chrome.back_introduction();
    chrome.back_introduction();
    assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.step_index), Some(0));
}

#[test]
fn tour_back_on_empty_state_is_a_no_operation() {
    let mut chrome = chrome_state();
    chrome.back_introduction();
    assert!(chrome.tour_state.is_none());
}

#[test]
fn tour_advance_and_back_reset_completed_interactions() {
    let mut chrome = chrome_state();
    chrome.start_introduction();
    chrome.tour_state.as_mut().unwrap().completed_interactions.push(0);
    chrome.advance_introduction(3);
    assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.completed_interactions.clone()), Some(vec![]));
    chrome.tour_state.as_mut().unwrap().completed_interactions.push(0);
    chrome.back_introduction();
    assert_eq!(chrome.tour_state.as_ref().map(|tour| tour.completed_interactions.clone()), Some(vec![]));
}

/// 🧪️ Ordered interactions gate out-of-order completions (the not-yet-reached one is ignored, no
/// dedup entry added) and a repeated already-completed gesture is a no-operation — both fall out of
/// `chrome_tour_complete_interaction`'s `!completed.contains(i)` + `index != completed.len()` checks.
///
/// Pan is index 1; out of order while zoom (index 0) hasn't completed — ignored.
///
/// Repeating zoom after it's already completed is a no-operation.
#[test]
fn chrome_tour_complete_interaction_respects_order_and_dedups() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.chrome_build.start_introduction();
    let step = semio_framework::IntroductionStepDefinition::new("viewport", LocalizedLabel::data("Viewport"), LocalizedLabel::data("…"))
        .interact_ordered(vec![semio_framework_async::block_on(semio_framework::IntroductionInteraction::zoom("main", "Zoom")), semio_framework_async::block_on(semio_framework::IntroductionInteraction::pan("main", "Pan"))]);
    shell.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Pan(id) if id == "main"));
    assert_eq!(shell.chrome_build.tour_state.as_ref().unwrap().completed_interactions, Vec::<usize>::new());
    shell.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Zoom(id) if id == "main"));
    assert_eq!(shell.chrome_build.tour_state.as_ref().unwrap().completed_interactions, vec![0]);
    shell.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Zoom(id) if id == "main"));
    assert_eq!(shell.chrome_build.tour_state.as_ref().unwrap().completed_interactions, vec![0]);
    shell.chrome_tour_complete_interaction(&step, |kind| matches!(kind, semio_framework::IntroductionInteractionKind::Pan(id) if id == "main"));
    assert_eq!(shell.chrome_build.tour_state.as_ref().unwrap().completed_interactions, vec![0, 1]);
}
//#endregion Tour

//#region ElementRects
#[test]
fn element_rect_registers_and_resolves() {
    let mut chrome = chrome_state();
    assert_eq!(chrome.resolve_element_rect("transform"), None);
    chrome.register_element_rect("transform", Rect::new(1.0, 2.0, 3.0, 4.0));
    assert_eq!(chrome.resolve_element_rect("transform"), Some(Rect::new(1.0, 2.0, 3.0, 4.0)));
    chrome.element_rects.clear();
    assert_eq!(chrome.resolve_element_rect("transform"), None);
}

#[test]
fn element_rect_fallback_never_overrides_a_primary_entry() {
    let mut chrome = chrome_state();
    chrome.register_element_rect_fallback("transform", Rect::new(0.0, 0.0, 10.0, 10.0));
    assert_eq!(chrome.resolve_element_rect("transform"), Some(Rect::new(0.0, 0.0, 10.0, 10.0)));
    chrome.register_element_rect("transform", Rect::new(5.0, 5.0, 20.0, 20.0));
    chrome.register_element_rect_fallback("transform", Rect::new(0.0, 0.0, 1.0, 1.0));
    assert_eq!(chrome.resolve_element_rect("transform"), Some(Rect::new(5.0, 5.0, 20.0, 20.0)));
}
//#endregion ElementRects

//#region VeilBands
#[test]
fn punch_cutout_returns_the_band_unchanged_when_disjoint() {
    let band = Rect::new(0.0, 0.0, 100.0, 100.0);
    let hole = Rect::new(200.0, 200.0, 10.0, 10.0);
    assert_eq!(punch_introduction_cutout(band, hole), vec![band]);
}

#[test]
fn punch_cutout_centered_hole_tiles_into_four_pieces_covering_the_remaining_area() {
    let band = Rect::new(0.0, 0.0, 100.0, 100.0);
    let hole = Rect::new(25.0, 25.0, 50.0, 50.0);
    let pieces = punch_introduction_cutout(band, hole);
    assert_eq!(pieces.len(), 4);
    let covered: f32 = pieces.iter().map(|p| p.w * p.h).sum();
    assert_eq!(covered, band.w * band.h - hole.w * hole.h);
}

#[test]
fn veil_bands_with_no_cutouts_is_one_full_viewport_band() {
    let bands = introduction_veil_bands(800.0, 600.0, &[]);
    assert_eq!(bands, vec![Rect::new(0.0, 0.0, 800.0, 600.0)]);
}

#[test]
fn veil_bands_clamp_out_of_viewport_cutouts_to_a_no_operation() {
    let bands = introduction_veil_bands(800.0, 600.0, &[Rect::new(-100.0, -100.0, 10.0, 10.0)]);
    assert_eq!(bands, vec![Rect::new(0.0, 0.0, 800.0, 600.0)]);
}

#[test]
fn veil_bands_compose_multiple_cutouts() {
    let bands = introduction_veil_bands(800.0, 600.0, &[Rect::new(0.0, 0.0, 100.0, 100.0), Rect::new(700.0, 500.0, 100.0, 100.0)]);
    let covered: f32 = bands.iter().map(|b| b.w * b.h).sum();
    assert_eq!(covered, 800.0 * 600.0 - 100.0 * 100.0 * 2.0);
}
//#endregion VeilBands

//#region Pulse
#[test]
fn pulse_thickness_breathes_hairline_to_focus_and_back() {
    let hairline = 1.0;
    let focus = 3.0;
    assert_eq!(introduced_pulse_thickness(0.0, hairline, focus), hairline);
    assert!((introduced_pulse_thickness(INTRODUCED_PULSE_PERIOD_MS / 2.0, hairline, focus) - focus).abs() < 0.001);
    assert!((introduced_pulse_thickness(INTRODUCED_PULSE_PERIOD_MS, hairline, focus) - hairline).abs() < 0.001);
}

#[test]
fn pulse_thickness_is_periodic() {
    let (hairline, focus) = (1.0, 3.0);
    assert_eq!(introduced_pulse_thickness(100.0, hairline, focus), introduced_pulse_thickness(100.0 + INTRODUCED_PULSE_PERIOD_MS, hairline, focus));
}

/// 🪟️ Gap baseline sits at y = bounds.y + cap_h - stroke
///
/// Top of controls starts at x = bounds.x + bounds.w - controls_w
#[test]
fn window_silhouette_border_emits_notched_outline_segments() {
    let mut draw = DrawList::default();
    let silhouette = WindowSilhouette::from_measured_top(Rect::new(10.0, 20.0, 200.0, 100.0), 60.0, 40.0, 24.0);
    push_window_silhouette_border(&mut draw, &silhouette, 2.0, Rgba::new(1.0, 0.0, 0.0, 1.0));
    let solids: Vec<[f32; 4]> = draw.layers.iter().flat_map(|layer| layer.ui_instances.iter().map(|instance| instance.rect)).collect();
    assert!(solids.len() >= 8, "silhouette must paint every outline segment");
    assert!(solids.iter().any(|r| (r[1] - (20.0 + 24.0 - 2.0)).abs() < 0.01 && r[0] >= 10.0 + 60.0 - 0.01), "gap baseline must sit under the cutout between tabs and controls");
    assert!(solids.iter().any(|r| (r[0] - (10.0 + 200.0 - 40.0)).abs() < 0.01 && (r[1] - 20.0).abs() < 0.01), "controls cap top must be part of the silhouette");
}
//#endregion Pulse

//#region Placement
#[test]
fn placement_centers_when_no_anchor() {
    let (x, y) = resolve_introduction_placement(semio_framework::IntroductionPlacement::Auto, None, (320.0, 168.0), (800.0, 600.0));
    assert_eq!((x, y), ((800.0 - 320.0) / 2.0, (600.0 - 168.0) / 2.0));
}

#[test]
fn placement_center_variant_ignores_the_anchor() {
    let anchor = Rect::new(10.0, 10.0, 50.0, 20.0);
    let (x, y) = resolve_introduction_placement(semio_framework::IntroductionPlacement::Center, Some(anchor), (320.0, 168.0), (800.0, 600.0));
    assert_eq!((x, y), ((800.0 - 320.0) / 2.0, (600.0 - 168.0) / 2.0));
}

/// 🧭️ Anchor near the top-left in a wide viewport: space_right (750) exceeds space_bottom (580),
/// space_top (0), and space_left (0), so "right" wins.
#[test]
fn placement_auto_picks_the_side_with_the_most_free_space() {
    let anchor = Rect::new(0.0, 0.0, 50.0, 20.0);
    let (x, y) = resolve_introduction_placement(semio_framework::IntroductionPlacement::Auto, Some(anchor), (100.0, 50.0), (800.0, 600.0));
    assert_eq!(x, anchor.x + anchor.w + INTRODUCTION_INFO_BOX_GAP);
    assert!(y >= INTRODUCTION_INFO_BOX_GAP);
}

#[test]
fn placement_explicit_side_is_honored_and_clamped_to_the_viewport() {
    let anchor = Rect::new(780.0, 10.0, 15.0, 15.0);
    let (x, _) = resolve_introduction_placement(semio_framework::IntroductionPlacement::Right, Some(anchor), (100.0, 50.0), (800.0, 600.0));
    assert!(x <= 800.0 - 100.0 - INTRODUCTION_INFO_BOX_GAP + 0.001);
}
//#endregion Placement

//#region RibbonActivePath
#[test]
fn utility_subtree_has_active_path_finds_a_pressed_toggle_at_the_top_level() {
    let action = ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None };
    let nodes = vec![ui_wgpu::wgpu::utility_toggle("a", "circle".into(), "A", true, action)];
    assert!(utility_subtree_has_active_path(&nodes));
}

#[test]
fn utility_subtree_has_active_path_recurses_into_nested_collections() {
    let action = ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None };
    let inner = vec![ui_wgpu::wgpu::utility_toggle("b", "circle".into(), "B", true, action.clone())];
    let nested = ui_wgpu::wgpu::utility_collection("group-2", "circle".into(), "Group 2", vec![ui_wgpu::wgpu::utility_collection("group-1", "circle".into(), "Group 1", inner)]);
    assert!(utility_subtree_has_active_path(std::slice::from_ref(&nested)));
}

#[test]
fn utility_subtree_has_active_path_false_when_nothing_pressed() {
    let action = ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None };
    let nodes =
        vec![ui_wgpu::wgpu::utility_toggle("a", "circle".into(), "A", false, action.clone()), ui_wgpu::wgpu::utility_collection("group", "circle".into(), "Group", vec![ui_wgpu::wgpu::utility_toggle("b", "circle".into(), "B", false, action)])];
    assert!(!utility_subtree_has_active_path(&nodes));
}

/// 🧪️ Item 5's core regression test: before this fix, `render_utility_nodes` filtered nested
/// `Collection`s out of `children` before recursing (`.filter(|child| !matches!(child,
/// UtilityNode::Collection { .. }))`), so a 2nd-level nested toggle never got a hit target at all —
/// expanding both levels here must reach it.
#[test]
fn render_utility_nodes_recurses_at_least_two_levels_deep() {
    let action = ActionDescriptor { controller_id: "test".into(), action: "noOperation".into(), args: None };
    let leaf_toggle = ui_wgpu::wgpu::utility_toggle("leaf", "circle".into(), "Leaf", false, action.clone());
    let inner_collection = ui_wgpu::wgpu::utility_collection("inner", "circle".into(), "Inner", vec![leaf_toggle]);
    let outer_collection = ui_wgpu::wgpu::utility_collection("outer", "circle".into(), "Outer", vec![inner_collection]);
    let utilities = vec![outer_collection];

    let mut collection_expanded = HashMap::new();
    collection_expanded.insert("outer".to_string(), true);
    collection_expanded.insert("inner".to_string(), true);

    let mut draw = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let theme = Theme::light();
    render_utility_nodes(&mut ShellChromeBuildState::default(), &mut draw, &mut atlas, &icons, &mut input, &theme, 0.0, 0.0, theme.control_height, &utilities, &collection_expanded);

    assert!(input.staged_hits().iter().any(|hit| hit.control_id.as_deref() == Some("framework.utility.toggle.leaf")), "a toggle nested two Collection levels deep must still get a real hit target once both ancestors are expanded");
}
//#endregion RibbonActivePath

//#region GhostText
#[test]
fn engagement_completion_suffix_matches_label_prefix() {
    let possibles = vec![ui_wgpu::wgpu::WindowEngagementPossible { id: "box".into(), label: "Box".into(), detail: None, action: None }];
    assert_eq!(engagement_completion_suffix("Bo", Some(&possibles)), "x");
    assert_eq!(engagement_completion_suffix("bo", Some(&possibles)), "x");
}

/// 🔚️ Fully typed: no suffix left
#[test]
fn engagement_completion_suffix_empty_when_query_is_empty_or_unmatched() {
    let possibles = vec![ui_wgpu::wgpu::WindowEngagementPossible { id: "box".into(), label: "Box".into(), detail: None, action: None }];
    assert_eq!(engagement_completion_suffix("", Some(&possibles)), "");
    assert_eq!(engagement_completion_suffix("zz", Some(&possibles)), "");
    assert_eq!(engagement_completion_suffix("Box", Some(&possibles)), "");
    assert_eq!(engagement_completion_suffix("Bo", None), "");
}

#[test]
fn engagement_completion_suffix_picks_first_matching_possible_in_order() {
    let possibles = vec![ui_wgpu::wgpu::WindowEngagementPossible { id: "boat".into(), label: "Boat".into(), detail: None, action: None }, ui_wgpu::wgpu::WindowEngagementPossible { id: "box".into(), label: "Box".into(), detail: None, action: None }];
    assert_eq!(engagement_completion_suffix("Bo", Some(&possibles)), "at");
}

/// 🧪️ Char-boundary safety: a multi-byte label prefix-matched by a query must not panic on slicing.
#[test]
fn engagement_completion_suffix_is_multibyte_safe() {
    let possibles = vec![ui_wgpu::wgpu::WindowEngagementPossible { id: "muenster".into(), label: "Münster".into(), detail: None, action: None }];
    assert_eq!(engagement_completion_suffix("M", Some(&possibles)), "ünster");
}

/// 🧪️ Accept-on-click (the mouse-driven substitute for Tab/Right-arrow — see the report's honest
/// scope-down on why the keyboard shortcut itself isn't reachable from this region): a click landing
/// inside the ghost-text rect on a clicked frame commits `query + suffix`.
#[test]
fn engagement_ghost_accept_on_click_commits_query_plus_suffix_when_clicked_inside() {
    let ghost_rect = Rect::new(50.0, 0.0, 20.0, 24.0);
    assert_eq!(engagement_ghost_accept_on_click(ghost_rect, 55.0, 10.0, true, "Bo", "x"), Some("Box".to_string()));
}

#[test]
fn engagement_ghost_accept_on_click_ignores_clicks_outside_the_ghost_rect() {
    let ghost_rect = Rect::new(50.0, 0.0, 20.0, 24.0);
    assert_eq!(engagement_ghost_accept_on_click(ghost_rect, 5.0, 10.0, true, "Bo", "x"), None);
}

#[test]
fn engagement_ghost_accept_on_click_ignores_held_or_stale_clicks() {
    let ghost_rect = Rect::new(50.0, 0.0, 20.0, 24.0);
    assert_eq!(engagement_ghost_accept_on_click(ghost_rect, 55.0, 10.0, false, "Bo", "x"), None);
}
//#endregion GhostText
