/// 📐️ Band layout and focus: the band above the footer, the subfooter reserve, control registration, shared transitions, the editor target and keyboard focus.

//#region 📐️Band
/// ⚖️ LAW: the band floats bottom-centre just above the footer, its buttons right-aligned in stage order after the
/// message on one row, the replay track fills its share, and on a phone-width viewport it wraps compact like React's
/// `max-w-[90vw] flex-wrap` band: within 90 % of the viewport, the message over whole-line segments, every button below
/// it inside the band, nothing overlapping, the track still along its lower edge.
#[test]
fn the_band_lays_out_above_the_footer_with_its_buttons_in_stage_order() {
    let theme = Theme::light();
    let buttons = |status: &HistoryTimeTravel| time_travel_band_controls(status).into_iter().map(|control| (control, control.verb.label(Locale::En).to_string())).collect::<Vec<_>>();
    let reviewing = session("a ready review");
    let plan = time_travel_band_plan(&reviewing, "Reviewing the edited history".into(), buttons(&reviewing), 1280.0, 720.0, &theme);
    assert!((plan.band.y + plan.band.h - 720.0).abs() < 0.001, "the band is the shell's last row, under the footer");
    assert!((plan.band.x + plan.band.w * 0.5 - 640.0).abs() < 0.001, "horizontally centred");
    assert_eq!(plan.buttons.iter().map(|button| button.control.verb).collect::<Vec<_>>(), [TimeTravelVerb::Rerun, TimeTravelVerb::Finalize, TimeTravelVerb::Exit]);
    assert!(plan.buttons.windows(2).all(|pair| pair[0].rect.x + pair[0].rect.w <= pair[1].rect.x));
    assert!(plan.buttons[2].rect.x + plan.buttons[2].rect.w <= plan.band.x + plan.band.w);
    assert_eq!(plan.lines.len(), 1, "a desktop band is one row");
    assert!(plan.buttons.iter().all(|button| button.rect.w >= theme.size_large() && button.rect.h >= theme.size_large()), "every control is a `size-large` target: {:?}", plan.buttons);
    let mut warned = reviewing.clone();
    warned.worst = Some(semio_framework::Severity::Warning);
    for status in [&reviewing, &warned] {
        assert_eq!(time_travel_band_tone(status, &theme).1.a, 1.0, "the band is an opaque surface in every tone");
    }
    assert!(plan.lines[0].rect.x + plan.lines[0].rect.w <= plan.buttons[0].rect.x);
    assert!(plan.progress.is_none());
    let replaying = session("a running replay");
    let plan = time_travel_band_plan(&replaying, "Replaying".into(), buttons(&replaying), 1280.0, 720.0, &theme);
    let (track, share) = plan.progress.expect("a replay with a known total paints its track");
    assert!((share - 3.0 / 8.0).abs() < 0.0001, "3 of 8");
    assert!(track.x >= plan.band.x && track.x + track.w <= plan.band.x + plan.band.w);
    let inside = |outer: Rect, inner: Rect| inner.x >= outer.x - 0.001 && inner.y >= outer.y - 0.001 && inner.x + inner.w <= outer.x + outer.w + 0.001 && inner.y + inner.h <= outer.y + outer.h + 0.001;
    let apart = |a: Rect, b: Rect| a.x + a.w <= b.x + 0.001 || b.x + b.w <= a.x + 0.001 || a.y + a.h <= b.y + 0.001 || b.y + b.h <= a.y + 0.001;
    for (status, width) in [(&reviewing, 375.0_f32), (&replaying, 375.0), (&reviewing, 320.0)] {
        let message = time_travel_band_lines(status, Terminology::Native, Locale::De).message();
        let compact = time_travel_band_plan(status, message.clone(), buttons(status), width, 812.0, &theme);
        assert!(compact.band.x >= 0.0 && compact.band.w <= width * 0.9 + 0.001 && compact.band.x + compact.band.w <= width, "{width}: within 90 % of the viewport");
        assert!((compact.band.y + compact.band.h - 812.0).abs() < 0.001, "{width}: still the last row");
        assert!(compact.lines.len() > 1, "{width}: the message wraps: {:?}", compact.lines);
        let words = |text: &str| text.split_whitespace().filter(|word| *word != "·").map(str::to_string).collect::<Vec<_>>();
        assert_eq!(words(&compact.lines.iter().map(|line| line.text.as_str()).collect::<Vec<_>>().join(" ")), words(&message), "{width}: every word, in order, none lost");
        assert!(compact.lines.iter().all(|line| !line.text.starts_with(" · ") && !line.text.ends_with(" · ")), "{width}: a line never starts or ends on the separator");
        assert!(compact.lines.iter().all(|line| inside(compact.band, line.rect)) && compact.buttons.iter().all(|button| inside(compact.band, button.rect)), "{width}: everything inside the band");
        let last_line = compact.lines.last().expect("a line");
        assert!(compact.buttons.iter().all(|button| button.rect.y >= last_line.rect.y + last_line.rect.h - 0.001), "{width}: the buttons flow below the message");
        assert!(compact.buttons.iter().enumerate().all(|(index, a)| compact.buttons[index + 1..].iter().all(|b| apart(a.rect, b.rect))), "{width}: no two buttons overlap");
        assert!(compact.buttons.iter().all(|button| button.rect.w >= theme.size_large() && button.rect.h >= theme.size_large()), "{width}: every control is a `size-large` target");
        if let Some((track, _)) = compact.progress {
            assert!(inside(compact.band, track) && track.y + track.h >= compact.band.y + compact.band.h - TIME_TRAVEL_PROGRESS_TRACK - 1.001, "{width}: the track runs along the lower edge");
        }
    }
}

/// ⚖️ LAW (React's `subfooter`): the bands are the shell's last layout row, under the footer — while a session is open the
/// body and the footer end above the band, so nothing is covered (on a phone-width viewport either), and with no band the
/// row is empty.
#[test]
fn the_bands_reserve_the_subfooter_row_under_the_footer() {
    let theme = Theme::light();
    for (width, height) in [(1280.0_f32, 720.0_f32), (375.0, 812.0)] {
        let mut shell = session_shell();
        (shell.chrome.screen_w, shell.chrome.screen_h) = (width, height);
        assert_eq!(SessionCtx::of(&mut shell).subfooter_height(&theme), 0.0, "{width}: no band, no row");
        let open = SessionCtx::of(&mut shell).body_rect(&theme);
        assert!((open.y + open.h - (height - theme.footer_height)).abs() < 0.001, "{width}: the body ends at the footer");
        let status = session("a ready review");
        SessionCtx::of(&mut shell).observe_history_time_travel(Some(&status));
        let band = shell.prefs.time_travel_band_plan_for(&status, &theme, &shell.chrome).band;
        let reserved = SessionCtx::of(&mut shell).subfooter_height(&theme);
        assert!((band.y + band.h - height).abs() < 0.001 && (reserved - band.h).abs() < 0.001 && reserved > 0.0, "{width}: the band fills the reserved row: {band:?} / {reserved}");
        let body = SessionCtx::of(&mut shell).body_rect(&theme);
        assert!((body.y + body.h + theme.footer_height - band.y).abs() < 0.001, "{width}: body, footer, band — nothing overlaps");
        assert!((open.h - body.h - reserved).abs() < 0.001, "{width}: the body gives exactly the reserved row");
    }
}

/// ⚖️ LAW: the painted band registers one hit per control — an enabled one carrying its dispatch on the session
/// controller, named in the active locale with its chord; a disabled Finalize dispatching nothing, announced disabled
/// and described by the refusal — and no control at all under a modal dialog.
#[test]
fn the_band_registers_each_control_with_its_dispatch_name_and_state() {
    let mut shell = session_shell();
    shell.prefs.locale_id = "de".into();
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("editing a mutation")));
    let hits = paint_band(&mut shell);
    assert_eq!(
        hit_events(&hits),
        [
            ("ui.timeTravel.accept".to_string(), Some("historyEditAccept".to_string())),
            ("ui.timeTravel.discard".to_string(), Some("historyEditDiscard".to_string())),
            ("ui.timeTravel.exit".to_string(), Some("historyEditExit".to_string())),
        ]
    );
    assert!(hits.iter().filter_map(|hit| hit.event.as_ref()).all(|event| event.controller_id == "test" && event.args.as_ref().map(dsl_value_as_json) == Some(serde_json::json!({ "generation": 3 }))));
    let nodes = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&hits);
    let accept = nodes.iter().find(|node| node.key == "ui.timeTravel.accept").expect("Accept is announced");
    assert_eq!((accept.role.as_str(), accept.label.as_deref(), accept.disabled), ("button", Some("Entwurf übernehmen"), false));
    assert_eq!(accept.shortcut, Some(format_keybinding_shortcut("alt+enter")), "aria-keyshortcuts comes from the remappable row");
    let band = nodes.iter().find(|node| node.key == TIME_TRAVEL_BAND_STATUS_ID).expect("the band's live node");
    assert_eq!(band.label.as_deref(), Some("Mutation wird bearbeitet · Bearbeitet: Auswahl ziehen"));

    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("an error downstream")));
    let hits = paint_band(&mut shell);
    assert_eq!(
        hit_events(&hits),
        [("shell.time-travel.rerun".to_string(), None), ("shell.time-travel.finalize".to_string(), None), ("ui.timeTravel.exit".to_string(), Some("historyEditExit".to_string()))],
        "a blocked Finalize and an unavailable replay stay visible and dispatch nothing"
    );
    let finalize = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&hits).into_iter().find(|node| node.key == "shell.time-travel.finalize").expect("Finalize is announced");
    assert_eq!((finalize.disabled, finalize.description.as_deref()), (true, Some("Blockiert: zuerst die offene Änderung oder die Fehler auflösen")));

    shell.chrome.chrome_build.open_dialog(ChromeDialogRequest::confirm("finalizeHistoryEdit", "Finish", "historyEditCommit"));
    assert!(paint_band(&mut shell).is_empty(), "the finalize prompt owns the pointer; the band under its veil registers nothing");
}
//#endregion 📐️Band

//#region 🎯️Focus
fn focus_name(focus: Option<TimeTravelFocus>) -> Value {
    match focus {
        None => Value::Null,
        Some(TimeTravelFocus::Editor) => "editor".into(),
        Some(TimeTravelFocus::Band) => "band".into(),
        Some(TimeTravelFocus::Dialog) => "dialog".into(),
    }
}

/// ⚖️ LAW (shared with React's `timeTravelTransitionV1`): for every row of the band corpus's `transitions` — a new
/// session, another stage, another edited mutation, and the changes that must move nothing (a progress step, a draft
/// edit, the commit, the close) — the wgpu shell answers exactly the corpus's `reveal` and `focus`; and observing the
/// change reveals the History panel exactly when the corpus says so.
#[test]
fn the_shared_band_transitions_hold_on_wgpu() {
    let corpus = corpus();
    let transitions = corpus["transitions"].as_array().expect("the band corpus's transitions");
    assert!(transitions.len() >= 13, "every edge and every change that moves nothing");
    for row in transitions {
        let name = row["name"].as_str().expect("name");
        let status = |key: &str| (!row[key].is_null()).then(|| serde_json::from_value::<HistoryTimeTravel>(row[key].clone()).unwrap_or_else(|error| panic!("{name}: a kernel session: {error}")));
        let (from, to) = (status("from"), status("to"));
        let transition = time_travel_transition(from.as_ref(), to.as_ref());
        assert_eq!((Value::Bool(transition.reveal), focus_name(transition.focus), transition.scroll_to.clone().map_or(Value::Null, Value::String)), (row["reveal"].clone(), row["focus"].clone(), row["scrollTo"].clone()), "{name}");
        let mut shell = session_shell();
        SessionCtx::of(&mut shell).sync_dock_tabs();
        SessionCtx::of(&mut shell).observe_history_time_travel(from.as_ref());
        let anchor = shell.dock.dock_tabs.locate(FRAMEWORK_PANEL_TAB_HISTORY_ID).map(|(anchor, _)| anchor).expect("the History tab is docked");
        shell.dock.anchor_state_mut(anchor).visible = false;
        shell.history.time_travel_focus = None;
        SessionCtx::of(&mut shell).observe_history_time_travel(to.as_ref());
        assert_eq!(shell.dock.anchor_open(anchor), transition.reveal, "{name}: the History panel opens exactly on a reveal");
        assert_eq!(shell.history.time_travel_focus.map(|(focus, _)| focus), transition.focus, "{name}: the focus waits for the screen");
    }
}

fn projected(key: &str, role: &str, depth: usize, focusable: bool) -> ui_contract::AccessibilityProjectionNode {
    let mut node = chrome_status_accessibility_node(0, key, key.to_string());
    node.role = role.to_string();
    node.depth = depth;
    node.focusable = focusable;
    node.live = ui_contract::liveness_name(ui_contract::Liveness::Off).to_string();
    node
}

/// ⚖️ LAW (React's `timeTravelFocusElementV1("editor")`): in a history body's projection the editor target is the first
/// enabled control under the inputs section — never its row, never a control outside the section — matched bare or
/// surface-qualified; with no usable input it is the editor's Accept button; no editor on screen is no target.
#[test]
fn the_editor_target_is_the_first_enabled_input_control_else_accept() {
    let surface = "framework.panel.history";
    let mut nodes = vec![
        projected("#0", "tree", 0, false),
        projected(&format!("{surface}/framework.history.editor"), "group", 1, false),
        projected(&format!("{surface}/framework.history.editor.accept.row"), "treeitem", 2, true),
        projected(&format!("{surface}/{TIME_TRAVEL_EDITOR_ACCEPT_KEY}"), "button", 3, true),
        projected(&format!("{surface}/{TIME_TRAVEL_EDITOR_INPUTS_KEY}"), "group", 1, false),
        projected(&format!("{surface}/framework.history.editor.input.dx.row"), "treeitem", 2, true),
        projected(&format!("{surface}/framework.history.editor.input.dx"), "spinbutton", 3, true),
        projected(&format!("{surface}/framework.history.editor.input.targets.useSelection"), "button", 3, true),
        projected(&format!("{surface}/framework.history.timeTravel.nextProblem"), "button", 1, true),
    ];
    assert_eq!(time_travel_editor_focus_node(&nodes), Some(6), "the first input control, past its row");
    nodes[6].disabled = true;
    assert_eq!(time_travel_editor_focus_node(&nodes), Some(7), "a disabled control is passed over");
    nodes[7].disabled = true;
    assert_eq!(time_travel_editor_focus_node(&nodes), Some(3), "no usable input: Accept, never a control after the section");
    let bare: Vec<_> = nodes.iter().cloned().map(|mut node| {
        node.key = node.key.rsplit_once('/').map_or(node.key.clone(), |(_, key)| key.to_string());
        node
    }).collect();
    assert_eq!(time_travel_editor_focus_node(&bare), Some(3), "bare keys match too");
    assert_eq!(time_travel_editor_focus_node(&nodes[..1]), None, "no editor, no target");
}

/// 🖌️ Publishes the band's hits the way a presented frame does and resolves the waiting focus against them.
fn present_band(shell: &mut ShellState) -> InputState<ActionDescriptor> {
    let mut input = InputState::<ActionDescriptor>::default();
    for hit in paint_band(shell) {
        input.register_hit(hit);
    }
    input.publish_hits();
    shell.input.resolve_time_travel_focus(&mut input, &mut shell.chrome, &mut shell.history);
    input
}

fn focused_chrome(shell: &ShellState, input: &InputState<ActionDescriptor>) -> Vec<String> {
    PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits()).into_iter().filter(|node| node.focused).map(|node| node.key).collect()
}

/// ⚖️ LAW: a replay or a review moves keyboard focus onto the band's live status node — the node the ARIA mirror then
/// focuses, never by Tab — and replay progress leaves it alone; the finalize prompt takes focus once it is open; an
/// editor that never reaches the screen is given up after the settle time, as React gives up after its frames; a person
/// typing in a field elsewhere keeps their focus; a closed session drops a waiting focus.
#[test]
fn a_replay_and_a_review_focus_the_band_the_prompt_takes_focus_and_typing_elsewhere_keeps_it() {
    let mut shell = session_shell();
    shell.docs.session.as_mut().expect("session").app.dialogs.push(finalize_dialog());
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("editing a mutation")));
    assert_eq!(shell.history.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Editor));
    present_band(&mut shell);
    assert_eq!(shell.history.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Editor), "no history body on screen yet: still waiting");
    shell.history.time_travel_focus = Some((TimeTravelFocus::Editor, 0.0));
    let input = present_band(&mut shell);
    assert_eq!((shell.history.time_travel_focus, focused_chrome(&shell, &input)), (None, Vec::<String>::new()), "past the settle time the move is given up");

    let mut replaying = session("a running replay");
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&replaying));
    let input = present_band(&mut shell);
    assert_eq!(focused_chrome(&shell, &input), [TIME_TRAVEL_BAND_STATUS_ID]);
    let band = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(input.hits()).into_iter().find(|node| node.key == TIME_TRAVEL_BAND_STATUS_ID).expect("the band's live node");
    assert!(band.focusable && !band.tabbable && band.focused, "focused programmatically, never a Tab stop");
    shell.input.accessibility_focused_control_id = Some("ui.timeTravel.exit".into());
    replaying.done = Some(4);
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&replaying));
    assert_eq!(shell.history.time_travel_focus, None, "progress inside the replay moves nothing");

    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("an error downstream")));
    let input = present_band(&mut shell);
    assert_eq!(focused_chrome(&shell, &input), [TIME_TRAVEL_BAND_STATUS_ID], "the review lands on the band that names what blocks");

    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("the finalize prompt can go back")));
    assert_eq!(shell.history.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Dialog));
    present_band(&mut shell);
    assert!(shell.history.time_travel_focus.is_some(), "the prompt is not open yet");
    open_finalize_prompt(&mut shell, "Alternative");
    let input = present_band(&mut shell);
    assert!(shell.history.time_travel_focus.is_none() && input.hits().is_empty() && shell.chrome.chrome_build.dialog_open(), "the open prompt holds focus; under it the band registers nothing");

    let mut typing = session_shell();
    SessionCtx::of(&mut typing).observe_history_time_travel(Some(&session("a running replay")));
    let mut input = InputState::<ActionDescriptor>::default();
    input.focus_input_owned("ui.search.input".to_string(), "dra".to_string());
    typing.input.resolve_time_travel_focus(&mut input, &mut typing.chrome, &mut typing.history);
    assert_eq!((typing.history.time_travel_focus, typing.input.accessibility_focused_control_id.clone()), (None, None), "a person typing elsewhere keeps their focus");

    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("a ready review")));
    SessionCtx::of(&mut shell).observe_history_time_travel(None);
    assert_eq!(shell.history.time_travel_focus, None, "a closed session drops a waiting focus");
}

/// ⚖️ LAW (focus reaches the ARIA mirror): beginning an edit gives the first input control of the history body's draft
/// editor the keyboard focus on the next presented frame — the control the mirror then focuses, not its row — and hands
/// that window the keyboard; nothing is left waiting.
#[test]
fn beginning_an_edit_focuses_the_first_editor_input_in_the_published_projection() {
        const SURFACE: &str = "framework.history.focus-law";
    let component = |wire: Value| serde_json::from_value::<ui_contract::Component>(wire).expect("history body component wire");
    let records = vec![
        tree_pointer_record(1, "#0", component(serde_json::json!({ "type": "tree" })), &[2, 4], None),
        tree_pointer_record(2, "framework.history.editor", component(serde_json::json!({ "type": "treeSection", "label": "Drag selection", "defaultOpen": true })), &[3], None),
        tree_pointer_record(3, "framework.history.editor.accept.row", component(serde_json::json!({ "type": "treeItem", "label": "Accept", "icon": "check" })), &[5], None),
        tree_pointer_record(5, TIME_TRAVEL_EDITOR_ACCEPT_KEY, component(serde_json::json!({ "type": "button", "label": "Accept", "icon": "check" })), &[], Some("historyEditAccept")),
        tree_pointer_record(4, TIME_TRAVEL_EDITOR_INPUTS_KEY, component(serde_json::json!({ "type": "treeSection", "label": "Inputs", "defaultOpen": true })), &[6], None),
        tree_pointer_record(6, "framework.history.editor.input.dx.row", component(serde_json::json!({ "type": "treeItem", "label": "Offset x", "icon": "move" })), &[7], None),
        tree_pointer_record(7, "framework.history.editor.input.dx", component(serde_json::json!({ "type": "numberStepper", "value": 2.0, "step": 1.0, "uniform": false })), &[], None),
    ];
    let mut shell = session_shell();
    let document = shell.windows.publish_surface_records(SURFACE, records).expect("the history body publishes");
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("editing a mutation")));
    let _ = paint_retained_body(&mut shell, SURFACE, &document, Rect::new(0.0, 0.0, 420.0, 640.0));
    assert_eq!(shell.history.time_travel_focus, None, "the editor reached the screen and took the focus");
    let nodes = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_nodes_for_test(SURFACE);
    let focused: Vec<_> = nodes.iter().filter(|node| node.focused).map(|node| (node.key.clone(), node.role.clone())).collect();
    assert_eq!(focused.len(), 1, "one focused node: {focused:?}");
    assert!(projected_key_is(&focused[0].0, "framework.history.editor.input.dx") && focused[0].1 == "spinbutton", "the stepper itself, not its row: {focused:?}");
    assert_eq!(shell.chrome.chrome_build.focused_retained_surface.as_deref(), Some(SURFACE), "the history body holds the keyboard");
}
//#endregion 🎯️Focus
