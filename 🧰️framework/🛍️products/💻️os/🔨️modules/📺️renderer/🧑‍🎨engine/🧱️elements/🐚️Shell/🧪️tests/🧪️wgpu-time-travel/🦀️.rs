//! 🧪️ Laws of the wgpu shell's time-travel chrome and its staged editors (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING,
//! packet W2-C).
//!
//! The band law is language-agnostic and shared with the React shell: `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band`
//! (validated by Ajv against its schema and the kernel's `HistoryTimeTravel` schema in the React suite) is read here per
//! session and locale — the band's lines, its controls with the refusal that disables one, the dispatch each sends,
//! the chords and the window indicator. The rest pins the wgpu behaviour around it: the keyboard (remappable chords,
//! Escape never discards), the reveal of the History tab, the indicator, the history-edit refusals, the finalize
//! prompt operated by keyboard alone, and every staged editor kind in the Actions form and in a chrome dialog.

use super::*;
use semio_framework::kernel::{HistoryEntry, HistoryPatch, HistoryTimeTravel, HistoryTimeTravelStage, InvocationResult};
use semio_framework::{ActionArgDef, ActionArgOption, ArgPresentation, ArgSchema, DialogDefinition, DomainSelection};

//#region 🧰️Harness
fn corpus() -> Value {
    serde_json::from_str(include_str!("../../../🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json")).expect("the shared time-travel band corpus parses")
}

/// 🧾️ The kernel session of the corpus case whose name starts with `name`.
fn session(name: &str) -> HistoryTimeTravel {
    let corpus = corpus();
    let case = corpus["cases"].as_array().expect("cases").iter().find(|case| case["name"].as_str().is_some_and(|case| case.starts_with(name))).unwrap_or_else(|| panic!("a corpus case named {name}…")).clone();
    serde_json::from_value(case["session"].clone()).unwrap_or_else(|error| panic!("{name}: a kernel HistoryTimeTravel: {error}"))
}

fn verb_name(verb: TimeTravelVerb) -> &'static str {
    match verb {
        TimeTravelVerb::Accept => "accept",
        TimeTravelVerb::Discard => "discard",
        TimeTravelVerb::CancelReplay => "cancelReplay",
        TimeTravelVerb::Rerun => "rerun",
        TimeTravelVerb::Finalize => "finalize",
        TimeTravelVerb::Back => "back",
        TimeTravelVerb::Exit => "exit",
    }
}

/// 🔑️ React's label key of the refusal that disables a control.
fn refusal_key(label: TimeTravelLabel) -> String {
    format!("ui.timeTravel.refusal.{}", label.key().trim_start_matches("refusal").to_lowercase())
}

const ALL_VERBS: [TimeTravelVerb; 7] = [TimeTravelVerb::Accept, TimeTravelVerb::Discard, TimeTravelVerb::CancelReplay, TimeTravelVerb::Rerun, TimeTravelVerb::Finalize, TimeTravelVerb::Back, TimeTravelVerb::Exit];

fn chord(chord: &str) -> (ui_wgpu::wgpu::KeyAction, PointerModifiers) {
    let mut modifiers = PointerModifiers::default();
    let mut key = ui_wgpu::wgpu::KeyAction::Escape;
    for token in chord.split('+') {
        match token {
            "alt" => modifiers.alt = true,
            "shift" => modifiers.shift = true,
            "ctrl" => modifiers.ctrl = true,
            "meta" | "mod" => modifiers.meta = true,
            "enter" => key = ui_wgpu::wgpu::KeyAction::Enter,
            "backspace" => key = ui_wgpu::wgpu::KeyAction::Backspace,
            "escape" => key = ui_wgpu::wgpu::KeyAction::Escape,
            other => key = ui_wgpu::wgpu::KeyAction::Char(other.to_string()),
        }
    }
    (key, modifiers)
}

/// 🧪️ A shell on the command-registry test app, with the framework History tab the plugin runtime injects.
fn session_shell() -> ShellState {
    let mut app = command_registry_tests::test_app(Vec::new(), Vec::new());
    app.panel_tabs.push(PanelTabDefinition {
        kind: semio_framework::PanelTabKind::App(FRAMEWORK_PANEL_TAB_HISTORY_ID.into()),
        label: LocalizedLabel::native("History", "Verlauf"),
        group: PanelGroup::Settings,
        body_key: Some("framework.body.history".into()),
        children: vec![],
    });
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.dock.root = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("main"), DockStackTab::new("side")], active: "main".into() };
    shell.dock.active_window_id = Some("main".into());
    shell.active_window_id = Some("main".into());
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app, view_state: ViewModel::default() });
    shell.screen_w = 1280.0;
    shell.screen_h = 720.0;
    shell
}

/// 🖌️ Walks the band's retained step to completion and answers the hits it staged.
fn paint_band(shell: &mut ShellState) -> Vec<HitTarget<ActionDescriptor>> {
    let mut cursor = ShellChromeChildCursor::default();
    let (mut overlay, mut atlas, mut input, theme) = (DrawList::default(), FontAtlas::builtin(), InputState::<ActionDescriptor>::default(), Theme::light());
    for _ in 0..100_000 {
        if shell.render_time_travel_band_step(&mut cursor, &mut overlay, &mut atlas, &mut input, &theme) {
            return input.staged_hits().to_vec();
        }
    }
    panic!("the time-travel band step never completed");
}

fn hit_events(hits: &[HitTarget<ActionDescriptor>]) -> Vec<(String, Option<String>)> {
    hits.iter().filter_map(|hit| Some((hit.control_id.clone()?, hit.event.as_ref().map(|event| event.action.clone())))).collect()
}

thread_local! {
    static DISPATCHED: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// 🎬️ A guest that records every dispatched action and answers each with the corpus's replaying session.
fn record_action(_instance_id: u32, action_json: &str, _view_state: &ViewModel) -> Result<InvocationResult, String> {
    DISPATCHED.with(|log| log.borrow_mut().push(action_json.to_string()));
    Ok(InvocationResult {
        output: DslValue::Null,
        mutations: Vec::new(),
        inverse_group: semio_framework::kernel::UndoGroup { invocation_id: semio_framework::kernel::InvocationId(String::new()), mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() },
        diagnostics: Vec::new(),
        requested_effects: Vec::new(),
        events: Vec::new(),
        ui_scope: UiDirtyScope::default(),
        history_patch: Some(HistoryPatch { cursor: 9, time_travel: Some(session("a running replay")), ..Default::default() }),
    })
}

fn dispatched() -> Vec<String> {
    DISPATCHED.with(|log| log.borrow().clone())
}
//#endregion 🧰️Harness

//#region ⏪️BandLaw
/// ⚖️ LAW (shared with React): for every corpus session and locale the band carries exactly the corpus's lines (a
/// review reads the session's own `review`, never a missing report), the stage offers exactly its controls in order with
/// the refusal that disables one (Replay again only when `rerunnable`), each control dispatches its reserved verb stamped
/// with the session generation, and every window's indicator is named as the corpus says.
#[test]
fn the_shared_band_corpus_holds_on_wgpu() {
    let corpus = corpus();
    let terminology = Terminology::parse(corpus["axes"]["terminology"].as_str().expect("terminology")).expect("a shell terminology");
    let controller = corpus["controllerId"].as_str().expect("controller");
    let cases = corpus["cases"].as_array().expect("cases");
    assert!(cases.len() >= 11, "every stage, every review and the fault variants");
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let status: HistoryTimeTravel = serde_json::from_value(case["session"].clone()).expect("a kernel session");
        for locale in Locale::ALL {
            let lines = time_travel_band_lines(&status, terminology, locale);
            let expected = &case["text"][locale.as_str()];
            let actual = serde_json::json!({ "stage": lines.stage, "target": lines.target, "progress": lines.progress, "review": lines.review, "outcome": lines.outcome, "fault": lines.fault, "accepted": lines.accepted });
            assert_eq!(&actual, expected, "{name} / {}", locale.as_str());
            assert_eq!(time_travel_indicator_text(&status, terminology, locale), case["indicator"][locale.as_str()].as_str().expect("indicator"), "{name} / {}", locale.as_str());
        }
        let controls: Vec<Value> = time_travel_band_controls(&status).into_iter().map(|control| serde_json::json!({ "control": verb_name(control.verb), "action": control.verb.action_id(), "disabledBy": control.disabled_by.map(refusal_key) })).collect();
        assert_eq!(Value::from(controls), case["controls"], "{name}");
        for control in time_travel_band_controls(&status) {
            let action = control.verb.action(controller, &status);
            assert_eq!((action.controller_id.as_str(), action.args.as_ref().map(dsl_value_as_json)), (controller, Some(serde_json::json!({ "generation": status.generation }))), "{name}: the dispatch carries the generation it was shown");
        }
    }
}

/// ⚖️ LAW: the band's live node speaks exactly the painted message, as a progress bar while a replay with a known total
/// runs, politely, and busy while the runtime replays or finalizes; no session announces nothing.
#[test]
fn the_band_announces_its_message_politely() {
    let mut shell = ShellState::new(Vec::new(), String::new());
    assert!(shell.time_travel_status_accessibility_node(1).is_none(), "no session, nothing announced");
    for (name, role, busy) in [("editing a mutation", "status", false), ("a running replay", "progressbar", true), ("an error downstream", "status", false), ("finalizing offers", "status", true)] {
        let status = session(name);
        shell.locale_id = "de".into();
        shell.observe_history_time_travel(Some(&status));
        let node = shell.time_travel_status_accessibility_node(1).expect("an open session announces itself");
        assert_eq!((node.role.as_str(), node.busy), (role, busy), "{name}");
        assert_eq!(node.label, Some(time_travel_band_lines(&status, Terminology::default(), Locale::De).message()), "{name}");
        assert_eq!(node.live, ui_contract::liveness_name(ui_contract::Liveness::Polite));
        if role == "progressbar" {
            assert_eq!((node.value_now, node.value_max), (Some(3.0), Some(8.0)));
        }
    }
    shell.observe_history_time_travel(None);
    assert!(shell.history_time_travel().is_none(), "an absent status closes the session");
    assert_eq!(time_travel_band_lines(&session("an error downstream"), Terminology::default(), Locale::En).message(), "Reviewing the edited history · Errors must be fixed or withdrawn before finalizing · Worst outcome: Error · Accepted changes: 2");
}

/// ⚖️ LAW: the captions are React's `ui.timeTravel.<control>` labels in both locales, and every control dispatches its
/// reserved `historyEdit*` action.
#[test]
fn every_control_reads_reacts_caption_and_names_its_reserved_action() {
    let captions = [
        (TimeTravelVerb::Accept, "Accept draft", "Entwurf übernehmen"),
        (TimeTravelVerb::Discard, "Discard draft", "Entwurf verwerfen"),
        (TimeTravelVerb::CancelReplay, "Cancel replay", "Neuanwendung abbrechen"),
        (TimeTravelVerb::Rerun, "Replay again", "Erneut anwenden"),
        (TimeTravelVerb::Finalize, "Finalize…", "Abschließen…"),
        (TimeTravelVerb::Back, "Back", "Zurück"),
        (TimeTravelVerb::Exit, "Exit time travel", "Zeitreise beenden"),
    ];
    for (verb, en, de) in captions {
        assert_eq!((verb.label(Locale::En), verb.label(Locale::De)), (en, de));
        assert!(verb.action_id().starts_with("historyEdit"));
    }
    assert_eq!(ALL_VERBS.len(), captions.len());
}
//#endregion ⏪️BandLaw

//#region ⌨️Chords
/// ⚖️ LAW: the corpus's three remappable chords are shell shortcut rows resolving to their verbs through the one table,
/// run on the async funnel, are never an app's to shadow, follow a user override, and no modifier combination turns
/// Escape into a time-travel verb.
#[test]
fn the_time_travel_chords_are_remappable_rows_and_escape_is_never_one() {
    let corpus = corpus();
    for verb in TimeTravelVerb::SHORTCUTS {
        let row = &corpus["chords"][verb_name(verb)];
        let (id, keys) = (row["controlId"].as_str().expect("control id"), row["keys"].as_str().expect("keys"));
        assert!(SHELL_SHORTCUT_ROWS.contains(&(id, keys)), "{id} is a shell shortcut row");
        assert_eq!(verb.control_id(), id, "the band control and the chord share one id");
        assert_eq!(shell_shortcut_for_control_id(id), Some(ShellShortcut::TimeTravel(verb)));
        assert!(ShellShortcut::TimeTravel(verb).is_async());
        for chord_text in keys.split(',') {
            let (action, modifiers) = chord(chord_text);
            assert_eq!(shell_shortcut_for(&action, &modifiers), Some(ShellShortcut::TimeTravel(verb)), "{chord_text}");
            assert!(is_reserved_shell_chord(&action, &modifiers), "{chord_text} is never an app's to shadow");
        }
    }
    for mask in 0..16u8 {
        let modifiers = PointerModifiers { shift: mask & 1 != 0, alt: mask & 2 != 0, ctrl: mask & 4 != 0, meta: mask & 8 != 0, ..PointerModifiers::default() };
        assert!(!matches!(shell_shortcut_for(&ui_wgpu::wgpu::KeyAction::Escape, &modifiers), Some(ShellShortcut::TimeTravel(_))), "Escape never discards a history edit");
    }
    let remapped = shell_shortcut_table(&HashMap::from([("ui.timeTravel.accept".to_string(), "ctrl+shift+y".to_string())]));
    let (action, modifiers) = chord("ctrl+shift+y");
    assert_eq!(shell_shortcut_for_in(&remapped, &action, &modifiers), Some(ShellShortcut::TimeTravel(TimeTravelVerb::Accept)), "a user override wins");
    let (action, modifiers) = chord("alt+enter");
    assert_eq!(shell_shortcut_for_in(&remapped, &action, &modifiers), None, "and the default chord is released");
}

/// ⚖️ LAW: a chord dispatches only a control the stage offers and enables, on the session controller with the
/// generation, and never while a retained text field keeps the keys.
#[test]
fn a_chord_dispatches_only_what_the_stage_offers() {
    let mut shell = session_shell();
    assert!(shell.time_travel_shortcut_action(TimeTravelVerb::Exit).is_none(), "no session, no verb");
    for (name, verb, expected) in [
        ("editing a mutation", TimeTravelVerb::Accept, Some("historyEditAccept")),
        ("editing a mutation", TimeTravelVerb::Discard, Some("historyEditDiscard")),
        ("editing a mutation", TimeTravelVerb::Exit, Some("historyEditExit")),
        ("a running replay", TimeTravelVerb::Accept, None),
        ("a running replay", TimeTravelVerb::Exit, Some("historyEditExit")),
        ("an error downstream", TimeTravelVerb::Discard, None),
        ("the finalize prompt can go back", TimeTravelVerb::Exit, Some("historyEditExit")),
        ("finalizing offers", TimeTravelVerb::Exit, None),
    ] {
        let status = session(name);
        shell.observe_history_time_travel(Some(&status));
        let expected = expected.map(|action| ("test".to_string(), action.to_string(), Some(serde_json::json!({ "generation": status.generation }))));
        assert_eq!(shell.time_travel_shortcut_action(verb).map(|action| (action.controller_id, action.action, action.args.as_ref().map(dsl_value_as_json))), expected, "{name} / {verb:?}");
    }
    assert!(time_travel_chord_yields_to(Some(RetainedNodeFocusKind::Input)), "a focused text or number field keeps its keys");
    assert!(!time_travel_chord_yields_to(Some(RetainedNodeFocusKind::Slider)));
    assert!(!time_travel_chord_yields_to(None));
}

/// ⚖️ LAW: through the real async keyboard router, `alt+enter` accepts the draft on the guest, the reply's patch moves
/// the band on, and a bare Escape reaches no history-edit verb at all.
#[test]
fn the_keyboard_router_accepts_by_chord_and_escape_discards_nothing() {
    DISPATCHED.with(|log| log.borrow_mut().clear());
    let mut shell = panel_anchor_model_tests::host_test_shell();
    shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("host fixture guest program").install_fixture_action(record_action);
    shell.observe_history_time_travel(Some(&session("editing a mutation")));
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = chord("alt+enter");
    semio_framework_async::block_on(shell.handle_keyboard_async(action, &modifiers, &mut input)).expect("the chord routes");
    assert!(dispatched().iter().any(|action| action.contains("historyEditAccept")), "the chord reached the guest: {:?}", dispatched());
    assert_eq!(shell.history_time_travel().map(|status| status.stage), Some(HistoryTimeTravelStage::Replaying), "the dispatch reply's patch moved the band");
    let before = dispatched().len();
    semio_framework_async::block_on(shell.handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input)).expect("Escape routes");
    assert!(!dispatched()[before..].iter().any(|action| action.contains("historyEdit")), "Escape dispatched no history-edit verb: {:?}", &dispatched()[before..]);
    assert_eq!(shell.history_time_travel().map(|status| status.stage), Some(HistoryTimeTravelStage::Replaying));
}
//#endregion ⌨️Chords

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
    assert!((plan.band.y + plan.band.h - (720.0 - theme.footer_height - TIME_TRAVEL_BAND_GAP)).abs() < 0.001);
    assert!((plan.band.x + plan.band.w * 0.5 - 640.0).abs() < 0.001, "horizontally centred");
    assert_eq!(plan.buttons.iter().map(|button| button.control.verb).collect::<Vec<_>>(), [TimeTravelVerb::Rerun, TimeTravelVerb::Finalize, TimeTravelVerb::Exit]);
    assert!(plan.buttons.windows(2).all(|pair| pair[0].rect.x + pair[0].rect.w <= pair[1].rect.x));
    assert!(plan.buttons[2].rect.x + plan.buttons[2].rect.w <= plan.band.x + plan.band.w);
    assert_eq!(plan.lines.len(), 1, "a desktop band is one row");
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
        let message = time_travel_band_lines(status, Terminology::default(), Locale::De).message();
        let compact = time_travel_band_plan(status, message.clone(), buttons(status), width, 812.0, &theme);
        assert!(compact.band.x >= 0.0 && compact.band.w <= width * 0.9 + 0.001 && compact.band.x + compact.band.w <= width, "{width}: within 90 % of the viewport");
        assert!((compact.band.y + compact.band.h - (812.0 - theme.footer_height - TIME_TRAVEL_BAND_GAP)).abs() < 0.001, "{width}: still just above the footer");
        assert!(compact.lines.len() > 1, "{width}: the message wraps: {:?}", compact.lines);
        let words = |text: &str| text.split_whitespace().filter(|word| *word != "·").map(str::to_string).collect::<Vec<_>>();
        assert_eq!(words(&compact.lines.iter().map(|line| line.text.as_str()).collect::<Vec<_>>().join(" ")), words(&message), "{width}: every word, in order, none lost");
        assert!(compact.lines.iter().all(|line| !line.text.starts_with(" · ") && !line.text.ends_with(" · ")), "{width}: a line never starts or ends on the separator");
        assert!(compact.lines.iter().all(|line| inside(compact.band, line.rect)) && compact.buttons.iter().all(|button| inside(compact.band, button.rect)), "{width}: everything inside the band");
        let last_line = compact.lines.last().expect("a line");
        assert!(compact.buttons.iter().all(|button| button.rect.y >= last_line.rect.y + last_line.rect.h - 0.001), "{width}: the buttons flow below the message");
        assert!(compact.buttons.iter().enumerate().all(|(index, a)| compact.buttons[index + 1..].iter().all(|b| apart(a.rect, b.rect))), "{width}: no two buttons overlap");
        if let Some((track, _)) = compact.progress {
            assert!(inside(compact.band, track) && track.y + track.h >= compact.band.y + compact.band.h - TIME_TRAVEL_PROGRESS_TRACK - 1.001, "{width}: the track runs along the lower edge");
        }
    }
}

/// ⚖️ LAW: the painted band registers one hit per control — an enabled one carrying its dispatch on the session
/// controller, named in the active locale with its chord; a disabled Finalize dispatching nothing, announced disabled
/// and described by the refusal — and no control at all under a modal dialog.
#[test]
fn the_band_registers_each_control_with_its_dispatch_name_and_state() {
    let mut shell = session_shell();
    shell.locale_id = "de".into();
    shell.observe_history_time_travel(Some(&session("editing a mutation")));
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
    let nodes = shell.chrome_accessibility_nodes(&hits);
    let accept = nodes.iter().find(|node| node.key == "ui.timeTravel.accept").expect("Accept is announced");
    assert_eq!((accept.role.as_str(), accept.label.as_deref(), accept.disabled), ("button", Some("Entwurf übernehmen"), false));
    assert_eq!(accept.shortcut, Some(format_keybinding_shortcut("alt+enter")), "aria-keyshortcuts comes from the remappable row");
    let band = nodes.iter().find(|node| node.key == TIME_TRAVEL_BAND_STATUS_ID).expect("the band's live node");
    assert_eq!(band.label.as_deref(), Some("Mutation wird bearbeitet · Bearbeitet: Auswahl ziehen"));

    shell.observe_history_time_travel(Some(&session("an error downstream")));
    let hits = paint_band(&mut shell);
    assert_eq!(
        hit_events(&hits),
        [("shell.time-travel.rerun".to_string(), None), ("shell.time-travel.finalize".to_string(), None), ("ui.timeTravel.exit".to_string(), Some("historyEditExit".to_string()))],
        "a blocked Finalize and an unavailable replay stay visible and dispatch nothing"
    );
    let finalize = shell.chrome_accessibility_nodes(&hits).into_iter().find(|node| node.key == "shell.time-travel.finalize").expect("Finalize is announced");
    assert_eq!((finalize.disabled, finalize.description.as_deref()), (true, Some("Blockiert: zuerst die offene Änderung oder die Fehler auflösen")));

    shell.chrome_build.open_dialog(ChromeDialogRequest::confirm("finalizeHistoryEdit", "Finish", "historyEditCommit"));
    assert!(paint_band(&mut shell).is_empty(), "the finalize prompt owns the pointer; the band under its veil registers nothing");
}
//#endregion 📐️Band

//#region 🎯️Focus
fn focus_corpus() -> Value {
    serde_json::from_str(include_str!("../../../🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-focus/🔣️.json")).expect("the shared time-travel focus corpus parses")
}

fn focus_name(focus: Option<TimeTravelFocus>) -> Value {
    match focus {
        None => Value::Null,
        Some(TimeTravelFocus::Editor) => "editor".into(),
        Some(TimeTravelFocus::NextProblem) => "nextProblem".into(),
        Some(TimeTravelFocus::Dialog) => "dialog".into(),
        Some(TimeTravelFocus::Control(verb)) => verb_name(verb).into(),
    }
}

/// ⚖️ LAW (shared with React): for every edge of the focus corpus — another session, another stage, another edited
/// mutation, and the edges that must leave focus alone (a draft edit, replay progress, the commit, a closed session) —
/// the wgpu shell names exactly the corpus's focus target, and resolves the editor and Next problem through the corpus's
/// keys, settle time and band fallbacks.
#[test]
fn the_shared_focus_corpus_holds_on_wgpu() {
    let corpus = focus_corpus();
    assert_eq!(corpus["settleMs"].as_f64(), Some(TIME_TRAVEL_FOCUS_SETTLE_MS));
    assert_eq!((corpus["keys"]["editorInputs"].as_str(), corpus["keys"]["editorAccept"].as_str(), corpus["keys"]["nextProblem"].as_str()), (Some(TIME_TRAVEL_EDITOR_INPUTS_KEY), Some(TIME_TRAVEL_EDITOR_ACCEPT_KEY), Some(TIME_TRAVEL_NEXT_PROBLEM_KEY)));
    for (focus, key) in [(TimeTravelFocus::Editor, "editor"), (TimeTravelFocus::NextProblem, "nextProblem")] {
        assert_eq!(focus_name(focus.fallback().map(TimeTravelFocus::Control)), corpus["fallbacks"][key], "{key}");
    }
    let cases = corpus["cases"].as_array().expect("cases");
    assert!(cases.len() >= 15, "every stage edge and every edge that keeps focus");
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let status = |key: &str| (!case[key].is_null()).then(|| serde_json::from_value::<HistoryTimeTravel>(case[key].clone()).unwrap_or_else(|error| panic!("{name}: a kernel session: {error}")));
        let (previous, next) = (status("previous"), status("next"));
        assert_eq!(focus_name(time_travel_focus_target(previous.as_ref(), next.as_ref())), case["focus"], "{name}");
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

/// ⚖️ LAW: in a history body's projection the editor target is the first enabled control under the inputs section — never
/// its row, never a control outside the section — matched bare or surface-qualified; with no usable input it is the
/// editor's Accept button; Next problem is its own button; a band control or the prompt is no body node.
#[test]
fn the_editor_target_is_the_first_enabled_input_control_else_accept() {
    let surface = "framework.panel.history";
    let mut nodes = vec![
        projected("#0", "tree", 0, false),
        projected(&format!("{surface}/framework.history.editor"), "group", 1, false),
        projected(&format!("{surface}/framework.history.editor.accept.row"), "treeitem", 2, true),
        projected(&format!("{surface}/framework.history.editor.accept"), "button", 3, true),
        projected(&format!("{surface}/{TIME_TRAVEL_EDITOR_INPUTS_KEY}"), "group", 1, false),
        projected(&format!("{surface}/framework.history.editor.input.dx.row"), "treeitem", 2, true),
        projected(&format!("{surface}/framework.history.editor.input.dx"), "spinbutton", 3, true),
        projected(&format!("{surface}/framework.history.editor.input.targets.useSelection"), "button", 3, true),
        projected(&format!("{surface}/{TIME_TRAVEL_NEXT_PROBLEM_KEY}"), "button", 2, true),
    ];
    assert_eq!(time_travel_focus_node(TimeTravelFocus::Editor, &nodes), Some(6), "the first input control, past its row");
    nodes[6].disabled = true;
    assert_eq!(time_travel_focus_node(TimeTravelFocus::Editor, &nodes), Some(7), "a disabled control is passed over");
    nodes[7].disabled = true;
    assert_eq!(time_travel_focus_node(TimeTravelFocus::Editor, &nodes), Some(3), "no usable input: Accept");
    assert_eq!(time_travel_focus_node(TimeTravelFocus::NextProblem, &nodes), Some(8), "the section ends at the next shallower node");
    let bare: Vec<_> = nodes.iter().cloned().map(|mut node| {
        node.key = node.key.rsplit_once('/').map_or(node.key.clone(), |(_, key)| key.to_string());
        node
    }).collect();
    assert_eq!(time_travel_focus_node(TimeTravelFocus::NextProblem, &bare), Some(8), "a bare key matches too");
    assert_eq!(time_travel_focus_node(TimeTravelFocus::NextProblem, &nodes[..8]), None, "an absent button is no target");
    assert_eq!((time_travel_focus_node(TimeTravelFocus::Dialog, &nodes), time_travel_focus_node(TimeTravelFocus::Control(TimeTravelVerb::Exit), &nodes)), (None, None));
}

/// 🖌️ Publishes the band's hits the way a presented frame does and resolves the waiting focus against them.
fn present_band(shell: &mut ShellState) -> InputState<ActionDescriptor> {
    let mut input = InputState::<ActionDescriptor>::default();
    for hit in paint_band(shell) {
        input.register_hit(hit);
    }
    input.publish_hits();
    shell.resolve_time_travel_focus(&mut input);
    input
}

fn focused_chrome(shell: &ShellState, input: &InputState<ActionDescriptor>) -> Vec<String> {
    shell.chrome_accessibility_nodes(input.hits()).into_iter().filter(|node| node.focused).map(|node| node.key).collect()
}

/// ⚖️ LAW: each edge moves keyboard focus once its target is on screen — Cancel while a replay runs, Finalize on a ready
/// review, the prompt while choosing — and the chrome projection the ARIA mirror reads marks exactly that control
/// focused; replay progress leaves it alone; an editor that never shows falls back to the band's Accept after the settle
/// time; a closed session drops a waiting focus.
#[test]
fn each_session_edge_focuses_its_band_control_or_the_prompt_and_a_missing_editor_falls_back_to_accept() {
    let mut shell = session_shell();
    shell.session.as_mut().expect("session").app.dialogs.push(finalize_dialog());
    shell.observe_history_time_travel(Some(&session("editing a mutation")));
    assert_eq!(shell.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Editor));
    let input = present_band(&mut shell);
    assert_eq!(shell.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Editor), "no history body on screen yet: still waiting");
    assert!(focused_chrome(&shell, &input).is_empty());
    shell.time_travel_focus = Some((TimeTravelFocus::Editor, 0.0));
    present_band(&mut shell);
    assert_eq!(shell.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Control(TimeTravelVerb::Accept)), "past the settle time the band's Accept stands in");
    let input = present_band(&mut shell);
    assert_eq!((shell.time_travel_focus, focused_chrome(&shell, &input)), (None, vec!["ui.timeTravel.accept".to_string()]));

    let mut replaying = session("a running replay");
    shell.observe_history_time_travel(Some(&replaying));
    let input = present_band(&mut shell);
    assert_eq!(focused_chrome(&shell, &input), ["shell.time-travel.cancel-replay"]);
    replaying.done = Some(4);
    shell.observe_history_time_travel(Some(&replaying));
    assert_eq!(shell.time_travel_focus, None, "progress inside the replay moves nothing");

    shell.observe_history_time_travel(Some(&session("a ready review")));
    let input = present_band(&mut shell);
    assert_eq!(focused_chrome(&shell, &input), ["shell.time-travel.finalize"]);

    let choosing = session("the finalize prompt can go back");
    shell.observe_history_time_travel(Some(&choosing));
    assert_eq!(shell.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Dialog));
    present_band(&mut shell);
    assert!(shell.time_travel_focus.is_some(), "the prompt is not open yet");
    open_finalize_prompt(&mut shell, "Alternative");
    let input = present_band(&mut shell);
    assert_eq!(shell.time_travel_focus, None, "the open prompt holds focus");
    assert!(input.hits().is_empty() && shell.chrome_build.dialog_open(), "under the prompt the band registers nothing; the prompt's own first stop is focused (its projection law lives with the prompt)");

    shell.observe_history_time_travel(Some(&session("a ready review")));
    shell.observe_history_time_travel(None);
    assert_eq!(shell.time_travel_focus, None, "a closed session drops a waiting focus");
}

/// ⚖️ LAW (focus reaches the ARIA mirror): beginning an edit gives the first input control of the history body's draft
/// editor the keyboard focus on the next presented frame — the control the mirror then focuses, not its row — and hands
/// that window the keyboard; nothing is left waiting.
#[test]
fn beginning_an_edit_focuses_the_first_editor_input_in_the_published_projection() {
    use super::shell_input_tests::tree_pointer_record;
    const SURFACE: &str = "framework.history.focus-law";
    let component = |wire: Value| serde_json::from_value::<ui_contract::Component>(wire).expect("history body component wire");
    let records = vec![
        tree_pointer_record(1, "#0", component(serde_json::json!({ "type": "tree" })), &[2, 4], None),
        tree_pointer_record(2, "framework.history.editor", component(serde_json::json!({ "type": "treeSection", "label": "Drag selection", "defaultOpen": true })), &[3], None),
        tree_pointer_record(3, "framework.history.editor.accept.row", component(serde_json::json!({ "type": "treeItem", "label": "Accept" })), &[5], None),
        tree_pointer_record(5, TIME_TRAVEL_EDITOR_ACCEPT_KEY, component(serde_json::json!({ "type": "button", "label": "Accept" })), &[], Some("historyEditAccept")),
        tree_pointer_record(4, TIME_TRAVEL_EDITOR_INPUTS_KEY, component(serde_json::json!({ "type": "treeSection", "label": "Inputs", "defaultOpen": true })), &[6], None),
        tree_pointer_record(6, "framework.history.editor.input.dx.row", component(serde_json::json!({ "type": "treeItem", "label": "Offset x" })), &[7], None),
        tree_pointer_record(7, "framework.history.editor.input.dx", component(serde_json::json!({ "type": "numberStepper", "value": 2.0, "step": 1.0, "uniform": false })), &[], None),
    ];
    let mut shell = session_shell();
    let document = shell.publish_surface_records(SURFACE, records).expect("the history body publishes");
    shell.observe_history_time_travel(Some(&session("editing a mutation")));
    let (mut draw, mut atlas, icons, theme) = (DrawList::default(), FontAtlas::builtin(), IconAtlas::default(), Theme::default());
    let mut input = InputState::<ActionDescriptor>::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = std::mem::take(&mut shell.world3d_states);
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let body = Rect::new(0.0, 0.0, 420.0, 640.0);
    crate::interpreter::begin_accessibility_visible_documents();
    let mut cursor = UiDocumentFrameCursor::default();
    let painted = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
        let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
        let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: SURFACE };
        render_ui_document_step(&mut cursor, &document, body, &mut ctx, SURFACE, "test", ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts)
    });
    assert!(painted, "the history body painted within its opportunity ceiling");
    shell.register_retained_body_hits(SURFACE, body, &mut input);
    shell.world3d_states = world3d_states;
    shell.publish_retained_hit_registry(&mut input);
    assert_eq!(shell.time_travel_focus, None, "the editor reached the screen and took the focus");
    let nodes = crate::interpreter::published_accessibility_nodes_for_test(SURFACE);
    let focused: Vec<_> = nodes.iter().filter(|node| node.focused).map(|node| (node.key.clone(), node.role.clone())).collect();
    assert_eq!(focused.len(), 1, "one focused node: {focused:?}");
    assert!(projected_key_is(&focused[0].0, "framework.history.editor.input.dx") && focused[0].1 == "spinbutton", "the stepper itself, not its row: {focused:?}");
    assert_eq!(shell.chrome_build.focused_retained_surface.as_deref(), Some(SURFACE), "the history body holds the keyboard");
}
//#endregion 🎯️Focus

//#region 🪟️RevealAndIndicator
/// ⚖️ LAW: the edge into a session reveals the History tab once, later statuses of the same session leave a closed
/// panel closed, and every pane's indicator is a named note under its own window id that no pane chip claims.
#[test]
fn a_session_reveals_the_history_tab_and_each_pane_wears_an_indicator() {
    let mut shell = session_shell();
    shell.sync_dock_tabs();
    let anchor = shell.dock_tabs.locate(FRAMEWORK_PANEL_TAB_HISTORY_ID).map(|(anchor, _)| anchor).expect("the History tab is docked");
    shell.anchor_state_mut(anchor).visible = false;
    shell.observe_history_time_travel(Some(&session("editing a mutation")));
    assert!(shell.anchor_open(anchor), "the session start opens the History panel");
    assert_eq!(shell.anchor_state(anchor).active_tab(), Some(FRAMEWORK_PANEL_TAB_HISTORY_ID));
    shell.anchor_state_mut(anchor).visible = false;
    shell.observe_history_time_travel(Some(&session("a running replay")));
    assert!(!shell.anchor_open(anchor), "only the edge into a session reveals");

    let indicator = time_travel_indicator_control_id("side");
    assert!(indicator.starts_with("framework.window.") && indicator.ends_with(".timeTravel.indicator"));
    assert!(shell.window_pane_chip_target(&indicator).is_none(), "no pane chip claims the indicator");
    assert_eq!((time_travel_indicator_caption(Locale::En), time_travel_indicator_caption(Locale::De)), ("Time travel", "Zeitreise"));
}

/// ⚖️ LAW (phone width): below the mobile breakpoint, where no anchor paints, the edge into a session opens the one
/// mobile panel on the History tab that holds the editor; on a desktop the mobile panel stays shut.
#[test]
fn on_a_phone_the_session_start_opens_the_mobile_panel_on_history() {
    let mut shell = session_shell();
    shell.sync_dock_tabs();
    shell.observe_history_time_travel(Some(&session("editing a mutation")));
    assert!(!shell.mobile_panel_visible, "a desktop reveals the anchor, not the mobile panel");
    let mut phone = session_shell();
    phone.screen_w = 375.0;
    phone.screen_h = 812.0;
    phone.sync_dock_tabs();
    assert!(phone.mobile_panel_active() && !phone.mobile_panel_visible);
    phone.observe_history_time_travel(Some(&session("editing a mutation")));
    assert!(phone.mobile_panel_visible, "the mobile panel opens");
    assert_eq!(phone.mobile_panel_path.last().map(String::as_str), Some(FRAMEWORK_PANEL_TAB_HISTORY_ID), "on the History tab");
}

/// ⚖️ LAW: every non-stale dispatch reply carries the session status (absent = closed); a stale reply changes nothing;
/// rows fold by their edit so a re-emitted edit replaces its row.
#[test]
fn replies_carry_the_status_and_rows_fold_by_their_edit() {
    let mut shell = session_shell();
    let open = HistoryPatch { cursor: 5, time_travel: Some(session("editing a mutation")), ..Default::default() };
    semio_framework_async::block_on(shell.observe_invocation_history(Some(&open)));
    assert_eq!(shell.history_time_travel().map(|status| status.stage), Some(HistoryTimeTravelStage::Editing));
    let stale = HistoryPatch { cursor: 4, ..Default::default() };
    semio_framework_async::block_on(shell.observe_invocation_history(Some(&stale)));
    assert!(shell.history_time_travel().is_some(), "a stale reply is ignored");
    let closed = HistoryPatch { cursor: 6, ..Default::default() };
    semio_framework_async::block_on(shell.observe_invocation_history(Some(&closed)));
    assert!(shell.history_time_travel().is_none(), "a reply without a status closes the band");

    let row = |seq: u64, edit: Option<&str>| HistoryEntry { seq, edit_id: edit.map(str::to_string), action_id: "apply".into(), label: LocalizedLabel::native("Apply", "Anwenden"), kind: "mutation".into(), applied: true, ..Default::default() };
    let mut entries = BTreeMap::new();
    let mut cursor = 0u64;
    assert!(fold_history_patch(&mut entries, &mut cursor, &HistoryPatch { cursor: 1, upserts: vec![row(1, Some("e1")), row(2, None)], ..Default::default() }, false));
    assert!(fold_history_patch(&mut entries, &mut cursor, &HistoryPatch { cursor: 2, upserts: vec![row(7, Some("e1"))], ..Default::default() }, false));
    assert_eq!(entries.len(), 2, "the re-emitted edit replaced its row");
    assert_eq!(history_rows_oldest_first(&entries).into_iter().map(|entry| entry.key()).collect::<Vec<_>>(), ["seq:2", "edit:e1"], "oldest first by seq");
    assert_eq!(uncommitted_edit_count(&entries), 2);
}

thread_local! {
    static PROGRESS: std::cell::RefCell<Vec<HistoryPatch>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// 📶️ A guest bridge whose uncorrelated progress frames are whatever the law queued, taken once.
fn queued_progress(_instance_id: u32) -> Vec<HistoryPatch> {
    PROGRESS.with(|queue| std::mem::take(&mut *queue.borrow_mut()))
}

/// ⚖️ LAW (React's `subscribeOperationProgress` lane): the progress patches the guest pushed on uncorrelated
/// `Invocation` frames move the band between two dispatches, oldest first through the reply's stale guard — a patch
/// older than the one already folded rolls nothing back, and an empty take is no change.
#[test]
fn unsolicited_progress_patches_move_the_band_between_dispatches() {
    let mut shell = panel_anchor_model_tests::host_test_shell();
    shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("host fixture guest program").install_fixture_progress(queued_progress);
    semio_framework_async::block_on(shell.observe_invocation_history(Some(&HistoryPatch { cursor: 5, time_travel: Some(session("editing a mutation")), ..Default::default() })));
    assert!(!semio_framework_async::block_on(shell.drain_progress_history_patches()), "nothing queued is no change");
    let replaying = session("a running replay");
    PROGRESS.with(|queue| queue.borrow_mut().extend([HistoryPatch { cursor: 5, time_travel: Some(replaying.clone()), ..Default::default() }, HistoryPatch { cursor: 4, ..Default::default() }]));
    assert!(semio_framework_async::block_on(shell.drain_progress_history_patches()), "the queued progress moved the band");
    assert_eq!(shell.history_time_travel(), Some(&replaying), "the stale patch after it closed nothing");
    assert!(PROGRESS.with(|queue| queue.borrow().is_empty()), "the drain took the queue");
    assert!(!semio_framework_async::block_on(shell.drain_progress_history_patches()));
}
//#endregion 🪟️RevealAndIndicator

//#region 🛑️HistoryRefusals
/// ⚖️ LAW: every history-edit refusal of the shared corpus (`refusals`: the hub's `history.*`, and every `timeTravel.*`
/// code the session, its hosting runtime and its replay driver answer — exactly the framework's `TIME_TRAVEL_CODE_LABELS`)
/// reads its copy in both locales wherever a fault surfaces — the band's fault line and a dispatch fault (a notice
/// carrying the code at the corpus severity) — and nothing else is mistaken for one.
#[test]
fn history_refusals_are_localized_notices_carrying_their_code() {
    let corpus = corpus();
    let refusals: Vec<(String, String, String, semio_framework::Severity)> = corpus["refusals"]
        .as_array()
        .expect("the corpus's history-edit refusals")
        .iter()
        .map(|row| {
            let severity = match row["severity"].as_str().expect("severity") {
                "error" => semio_framework::Severity::Error,
                "warning" => semio_framework::Severity::Warning,
                other => panic!("a history-edit refusal is an error or a warning, not {other}"),
            };
            (row["code"].as_str().expect("code").to_string(), row["text"]["en"].as_str().expect("en").to_string(), row["text"]["de"].as_str().expect("de").to_string(), severity)
        })
        .collect();
    let time_travel_codes: std::collections::BTreeSet<&str> = refusals.iter().map(|(code, ..)| code.as_str()).filter(|code| code.starts_with("timeTravel.")).collect();
    assert_eq!(time_travel_codes, semio_framework_time_travel::TIME_TRAVEL_CODE_LABELS.iter().map(|(code, _)| *code).collect(), "the corpus names every timeTravel code the framework answers, and no other");
    assert_eq!(refusals.len(), time_travel_codes.len() + 3, "the hub's three beside them");
    for (code, en, de, severity) in refusals {
        let (code, en, de) = (code.as_str(), en.as_str(), de.as_str());
        for (locale, text) in [(Locale::En, en), (Locale::De, de)] {
            assert_eq!(history_refusal_notice(code, locale), Some((code, text, severity)));
            assert_eq!(history_refusal_notice(&format!("hub refused batch 4: {code}"), locale), None, "{code}: prose never names a refusal");
            for fault in [format!("{code}: edit-9#0"), format!("handle_action failed: {code}: stale"), format!("app.command.rejected: refused — mutation.clamped: region; {code}: step [t-1]")] {
                assert_eq!(history_refusal_of_fault(&fault, locale), Some((code, text, severity)), "{fault}");
                let (message, notice_severity, notice_code) = classify_dispatch_fault_notice(&fault, locale);
                assert_eq!((message.as_str(), notice_severity, notice_code), (text, severity, Some(code)), "{fault}");
            }
            let status = HistoryTimeTravel { fault: Some(code.to_string()), ..session("editing a mutation") };
            assert_eq!(time_travel_band_lines(&status, Terminology::default(), locale).fault.as_deref(), Some(text), "{code}: the band's fault line");
        }
    }
    assert!(history_refusal_notice("app.command.rejected: conflicting edit", Locale::En).is_none());
    assert!(history_refusal_of_fault("app.command.rejected: the history.transition-refusedness of it", Locale::En).is_none(), "a code inside a word is no code");
    let mut shell = ShellState::new(Vec::new(), String::new());
    shell.locale_id = "de".into();
    shell.note_dispatch_fault("history.unknown-target: edit-9#0");
    let notice = shell.transient_notice().expect("a banner is showing");
    assert_eq!((notice.severity, notice.code.as_deref()), (semio_framework::Severity::Error, Some("history.unknown-target")));
    assert_eq!(notice.message, "Verlaufsbearbeitung abgelehnt: Die bearbeitete Mutation existiert nicht mehr.");
}

/// ⚖️ LAW (shared with React's `commandRejectionNoticeV1`): every row of the command-rejection notice corpus — the one
/// typed `CommandRejectionV1` every producer answers — is told in both locales exactly as the corpus says, with its
/// notice code and severity, from the rejection's `code` and its messages' codes alone; the ten closed codes are all
/// covered.
#[test]
fn every_command_rejection_is_told_from_its_codes_in_both_locales() {
    let corpus: Value = serde_json::from_str(include_str!("../../../🛠️ShellHelpers/🧫️fixtures/🧫️command-rejection/🔣️.json")).expect("the shared command-rejection corpus parses");
    let rows = corpus["rows"].as_array().expect("the corpus rows");
    let mut codes = std::collections::BTreeSet::new();
    for row in rows {
        let name = row["name"].as_str().expect("name");
        let rejection = row["rejection"].to_string();
        let store_sync::sync::CommandAckOutcome::Rejected { code, messages, .. } = dsl::os_pack::json::from_json_str::<store_sync::sync::CommandAckOutcome>(&rejection).unwrap_or_else(|error| panic!("{name}: a CommandRejectionV1: {error}")) else {
            panic!("{name}: a rejected outcome")
        };
        codes.insert(row["rejection"]["code"].as_str().expect("code").to_string());
        for (locale, tongue) in [(Locale::En, "en"), (Locale::De, "de")] {
            let (notice_code, text, severity) = command_rejection_notice(code, &messages, locale);
            let kind = match severity {
                semio_framework::Severity::Info => "info",
                semio_framework::Severity::Warning => "warning",
                semio_framework::Severity::Error => "error",
                semio_framework::Severity::Fatal => "fatal",
            };
            assert_eq!((notice_code, text.as_str(), kind), (row["code"].as_str().expect("notice code"), row["notice"][tongue].as_str().expect("notice text"), row["kind"].as_str().expect("kind")), "{name} ({tongue})");
        }
    }
    assert_eq!(codes.len(), 10, "the corpus covers every closed rejection code: {codes:?}");
}
//#endregion 🛑️HistoryRefusals

//#region 🧯️NoticeProjection
/// ⚖️ LAW (React's `[data-semio-transient-notice]`, fixture `🧫️wgpu-transient-notice`): every showing transient notice
/// is projected as the one polite status node `shell.notice`, named by its localized message and described by its code —
/// beside the chrome, and over an open modal too — and nothing once dismissed or past its deadline.
#[test]
fn every_transient_notice_is_a_polite_status_named_by_its_message_and_described_by_its_code() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🧯️wgpu-transient-notice/🔣️.json")).expect("the transient notice fixture parses");
    let expected = &fixture["node"];
    let notice_node = |shell: &ShellState| shell.chrome_accessibility_nodes(&[]).into_iter().filter(|node| node.key == expected["key"].as_str().expect("key")).collect::<Vec<_>>();
    for notice in fixture["notices"].as_array().expect("notices") {
        let mut shell = session_shell();
        shell.locale_id = notice["locale"].as_str().expect("locale").into();
        let severity = match notice["severity"].as_str().expect("severity") {
            "info" => semio_framework::Severity::Info,
            "warning" => semio_framework::Severity::Warning,
            _ => semio_framework::Severity::Error,
        };
        let message = notice["message"].as_str().expect("message");
        assert!(notice_node(&shell).is_empty(), "no notice, no node");
        shell.show_transient_notice(message, severity, notice["code"].as_str());
        let nodes = notice_node(&shell);
        assert_eq!(nodes.len(), 1, "{message}: one status node");
        let node = &nodes[0];
        assert_eq!((node.role.as_str(), node.live.as_str(), node.focusable, node.actionable), (expected["role"].as_str().expect("role"), expected["live"].as_str().expect("live"), false, false), "{message}");
        assert_eq!((node.label.as_deref(), node.description.as_deref()), (Some(message), notice["code"].as_str()), "{message}: named by the message, described by the code");
        shell.session.as_mut().expect("session").app.dialogs.push(finalize_dialog());
        shell.observe_history_time_travel(Some(&session("the finalize prompt can go back")));
        open_finalize_prompt(&mut shell, "Alternative");
        assert_eq!(notice_node(&shell).len(), 1, "{message}: still told while a modal dialog is open");
        shell.chrome_build.transient_notice.as_mut().expect("showing").shown_at_ms -= 5_000.0;
        assert!(notice_node(&shell).is_empty(), "{message}: nothing past the 4 s deadline");
    }
}
//#endregion 🧯️NoticeProjection

//#region 👥️PeerHistoryEdits
fn peers_corpus() -> Value {
    serde_json::from_str(include_str!("../../../🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-peers/🔣️.json")).expect("the shared time-travel peers corpus parses")
}

/// 🧾️ The corpus's history rows as this replica's own folded history.
fn peer_rows(corpus: &Value) -> BTreeMap<String, HistoryEntry> {
    corpus["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| {
            let mutations = row["mutations"]
                .as_array()
                .expect("mutations")
                .iter()
                .enumerate()
                .map(|(index, mutation)| semio_framework::kernel::HistoryMutationEntry {
                    mutation_id: mutation["mutationId"].as_str().expect("mutationId").into(),
                    position: index as u32,
                    op_index: index as u32,
                    label: LocalizedLabel::native(mutation["label"]["en"].as_str().expect("en"), mutation["label"]["de"].as_str().expect("de")),
                    worst: None,
                    messages: Vec::new(),
                    superseded: false,
                    withdrawn: false,
                    editable: true,
                    pending: false,
                    edited: false,
                    introduced: false,
                    store: None,
                })
                .collect();
            let entry = HistoryEntry { seq: row["seq"].as_u64().expect("seq"), edit_id: row["editId"].as_str().map(str::to_string), action_id: "apply".into(), label: LocalizedLabel::native("Apply", "Anwenden"), kind: "mutation".into(), applied: true, mutations, ..Default::default() };
            (entry.key(), entry)
        })
        .collect()
}

/// 👤️ One corpus peer on `surface` as the hub admits it.
fn corpus_peer(peer: &Value, surface: &str) -> PresencePeer {
    let history_edit = peer.get("historyEdit").map(|edit| store_sync::os_spr::PresenceHistoryEdit {
        mutation_id: edit["mutationId"].as_str().expect("mutationId").into(),
        stage: store_sync::os_spr::PresenceHistoryEditStage::ALL.into_iter().find(|stage| stage.wire_name() == edit["stage"].as_str().expect("stage")).expect("a wire stage"),
        drafts: edit["drafts"].as_u64().expect("drafts") as u32,
    });
    let actor = peer["actor"].as_str().expect("actor");
    PresencePeer {
        actor: actor.into(),
        connected_at_ms: 1,
        label: peer["label"].as_str().map(str::to_string),
        presence_pack: None,
        user_id: Some(format!("user:{actor}")),
        role: Some("member".into()),
        drag_ghost_json: None,
        interaction: None,
        color: Some(1),
        surface: Some(surface.into()),
        views: Vec::new(),
        ui: None,
        tool_run: None,
        principal_kind: None,
        active_tool: None,
        history_edit,
        typing: Vec::new(),
    }
}

/// ⚖️ LAW (shared with React's `timeTravelPeerPresenceV1`): for every corpus case and locale, the peers' open history
/// edits are labelled from this replica's own rows exactly as the corpus says — each editing peer's roster activity
/// (text and badge) in peer order, and the history body's notes by node key, sorted, one key's lines joined in peer
/// order; a peer without a history edit has neither.
#[test]
fn the_shared_peer_history_edit_corpus_holds_on_wgpu() {
    let corpus = peers_corpus();
    let entries = peer_rows(&corpus);
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let peers: Vec<PresencePeer> = case["peers"].as_array().expect("peers").iter().map(|peer| corpus_peer(peer, "s")).collect();
        for (locale, tongue) in [(Locale::En, "en"), (Locale::De, "de")] {
            let editing = peers.iter().filter_map(|peer| Some((peer.actor.as_str(), peer.label.as_deref().unwrap_or(peer.actor.as_str()), peer.history_edit.as_ref()?.mutation_id.as_str())));
            let presence = time_travel_peer_presence(editing, &entries, Terminology::default(), locale);
            let expect = &case["expect"][tongue];
            let chips: Vec<(String, String, String)> = expect["chips"].as_array().expect("chips").iter().map(|chip| (chip["actor"].as_str().expect("actor").into(), chip["text"].as_str().expect("text").into(), chip["badge"].as_str().expect("badge").into())).collect();
            let notes: Vec<(String, String)> = expect["notes"].as_array().expect("notes").iter().map(|note| (note["key"].as_str().expect("key").into(), note["text"].as_str().expect("text").into())).collect();
            assert_eq!(presence.activities.iter().map(|(actor, activity)| (actor.clone(), activity.text.clone(), activity.badge.clone())).collect::<Vec<_>>(), chips, "{name} ({tongue}): chips");
            assert_eq!(presence.notes, notes, "{name} ({tongue}): notes");
        }
    }
}

/// ⚖️ LAW: the shell's roster shows exactly the attached surface's editing peers — the footer chip paints the badge after
/// the name and is announced with the activity text, and the history body's notes are what the shell hands the
/// retained UI; a peer on another surface is neither shown nor noted.
#[test]
fn the_roster_badges_and_announces_an_editing_peer_and_notes_its_rows() {
    let corpus = peers_corpus();
    let case = corpus["cases"].as_array().expect("cases").iter().find(|case| case["name"].as_str().is_some_and(|name| name.starts_with("two peers"))).expect("the two-peer case").clone();
    let mut shell = session_shell();
    shell.history_entries = peer_rows(&corpus);
    shell.presence_surface = Some("s".into());
    shell.presence_peers = case["peers"].as_array().expect("peers").iter().map(|peer| corpus_peer(peer, "s")).collect();
    shell.presence_peers.push(corpus_peer(&serde_json::json!({ "actor": "actor-far", "label": "Far", "historyEdit": { "mutationId": "m-drag", "stage": "editing", "drafts": 1 } }), "elsewhere"));
    shell.locale_id = "de".into();
    let rows = shell.footer_presence_rows();
    assert_eq!(rows.iter().map(|row| (row.actor.as_str(), row.activity.as_ref().map(|activity| activity.badge.as_str()))).collect::<Vec<_>>(), [("actor-ada", Some("⏪")), ("actor-cy", None), ("actor-bo", Some("⏪"))]);
    assert_eq!(ui_wgpu::wgpu::presence_bar_chip_text(&rows, None, Locale::De), "Ada ⏪ · Cy · Bo ⏪");
    let announced = shell.footer_status_chips().into_iter().find(|(key, _)| *key == "s-presence-peers").expect("the roster is announced").1;
    assert_eq!(announced, "Ada (Ada bearbeitet Skalieren in der Zeitreise) · Cy · Bo (Bo bearbeitet Drehen in der Zeitreise)");
    let expected: Vec<(String, String)> = case["expect"]["de"]["notes"].as_array().expect("notes").iter().map(|note| (note["key"].as_str().expect("key").into(), note["text"].as_str().expect("text").into())).collect();
    assert_eq!(shell.peer_time_travel_presence().notes, expected, "a peer on another surface notes nothing");
}
//#endregion 👥️PeerHistoryEdits

//#region 🗨️FinalizePrompt
fn finalize_dialog() -> DialogDefinition {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🧫️dialog-choices/🔣️.json")).expect("dialog-choices fixture");
    serde_json::from_value(fixture["dialog"].clone()).expect("the finalize prompt")
}

/// 🗨️ Opens the framework-injected finalize prompt exactly as the runtime does: `Effect::OpenDialog` seeded with the
/// localized default name and the session generation (a context arg no field declares).
fn open_finalize_prompt(shell: &mut ShellState, name: &str) {
    let controller = shell.session.as_ref().map(|session| session.app.controller_id.clone()).expect("a session");
    let generation = shell.history_time_travel().map(|status| status.generation).expect("a live session");
    shell.queue_host_effects(&controller, vec![semio_framework::kernel::Effect::OpenDialog { req: semio_framework::kernel::RequestId(1), dialog_id: "finalizeHistoryEdit".into(), args: Some(DslValue::from(serde_json::json!({ "name": name, "generation": generation }))) }]);
    assert!(shell.chrome_build.dialog_open(), "the prompt opened from the session's dialogs");
}

fn key(shell: &mut ShellState, input: &mut InputState<ActionDescriptor>, action: ui_wgpu::wgpu::KeyAction) {
    semio_framework_async::block_on(shell.handle_keyboard_async(action, &PointerModifiers::default(), input)).expect("the key routes");
}

/// ⚖️ LAW: keyboard alone finalizes through the real async router — the open prompt owns every key ahead of the shell's
/// own rungs: typing edits the seeded name and Enter submits a new alternative; Tab to the destructive choice and Enter
/// overwrites, needing no name; Escape goes back. Every dispatch carries the session generation; no path discards or
/// exits.
#[test]
fn the_finalize_prompt_is_operated_by_keyboard_alone() {
    let mut shell = session_shell();
    shell.session.as_mut().expect("session").app.dialogs.push(finalize_dialog());
    let choosing = session("the finalize prompt can go back");
    shell.observe_history_time_travel(Some(&choosing));
    let mut input = InputState::<ActionDescriptor>::default();

    open_finalize_prompt(&mut shell, "Edited history");
    assert_eq!(shell.chrome_build.dialog_stack.last().map(|request| request.fields[0].draft.clone()), Some("Edited history".to_string()), "the runtime's default name seeds the field");
    key(&mut shell, &mut input, ui_wgpu::wgpu::KeyAction::Char("!".into()));
    key(&mut shell, &mut input, ui_wgpu::wgpu::KeyAction::Enter);
    assert!(!shell.chrome_build.dialog_open());
    let submit = shell.deferred_actions.pop().expect("the submit dispatched");
    assert_eq!((submit.controller_id.as_str(), submit.action.as_str()), ("test", "historyEditCommit"));
    assert_eq!(serde_json::to_value(submit.args.expect("args")).expect("wire"), serde_json::json!({ "generation": choosing.generation, "name": "Edited history!" }));

    open_finalize_prompt(&mut shell, "Edited history");
    for _ in 0.."Edited history".len() {
        key(&mut shell, &mut input, ui_wgpu::wgpu::KeyAction::Backspace);
    }
    key(&mut shell, &mut input, ui_wgpu::wgpu::KeyAction::Enter);
    assert!(shell.chrome_build.dialog_open(), "a cleared name gates the new alternative");
    key(&mut shell, &mut input, ui_wgpu::wgpu::KeyAction::Tab);
    key(&mut shell, &mut input, ui_wgpu::wgpu::KeyAction::Tab);
    key(&mut shell, &mut input, ui_wgpu::wgpu::KeyAction::Enter);
    let overwrite = shell.deferred_actions.pop().expect("the overwrite choice dispatched without a name");
    assert_eq!(serde_json::to_value(overwrite.args.expect("args")).expect("wire"), serde_json::json!({ "generation": choosing.generation, "choice": "overwrite" }));

    open_finalize_prompt(&mut shell, "Edited history");
    key(&mut shell, &mut input, ui_wgpu::wgpu::KeyAction::Escape);
    assert!(!shell.chrome_build.dialog_open());
    let back = shell.deferred_actions.pop().expect("Escape answered");
    assert_eq!((back.action.as_str(), back.args), ("historyEditBack", None), "Escape goes back to reviewing — it never discards");
    assert!(!shell.deferred_actions.iter().any(|action| matches!(action.action.as_str(), "historyEditDiscard" | "historyEditExit")));
}
//#endregion 🗨️FinalizePrompt

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
    let row = |index: usize, value: Option<Value>| staged_action_arg_row("main", "move", &defs[index], value.as_ref(), &selection(), Terminology::default(), Locale::En);
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
            Some(UiControlNode::Button(button)) => Some((button.id.clone().unwrap_or_default(), button.label.as_str().to_string(), args_of(&button.action)["value"].clone(), button.presence.state == ui_wgpu::wgpu::component::ui::UiState::Disabled)),
            _ => None,
        })
        .collect();
    assert_eq!(buttons, [("targets.remove.a".to_string(), "Remove a".to_string(), serde_json::json!([]), false), ("targets.useSelection".to_string(), "Use current selection".to_string(), serde_json::json!(["n1", "n2"]), false)]);
    let empty = staged_action_arg_row("main", "move", &defs[4], None, &HashMap::new(), Terminology::default(), Locale::De).items.expect("rows");
    assert_eq!(empty.iter().map(|item| item.label.as_str().to_string()).collect::<Vec<_>>(), ["Nichts ausgewählt", "Aktuelle Auswahl verwenden"]);
    assert!(matches!(&empty[1].control, Some(UiControlNode::Button(button)) if button.presence.state == ui_wgpu::wgpu::component::ui::UiState::Disabled), "nothing selected, nothing to use");
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
    semio_framework_async::block_on(shell.dispatch_action(stage(serde_json::json!({ "window": "main", "action": "move", "arg": "offset", "index": 1, "dims": 3, "tuple": [0.0, 0.0, 0.0], "value": 4.0 })))).expect("stages");
    semio_framework_async::block_on(shell.dispatch_action(stage(serde_json::json!({ "window": "main", "action": "move", "arg": "offset", "index": 2, "dims": 3, "tuple": [0.0, 0.0, 0.0], "value": 5.0 })))).expect("stages");
    assert_eq!(shell.staged_map_for("main", "move").get("offset"), Some(&serde_json::json!([0.0, 4.0, 5.0])), "the second axis keeps the first");
}

fn kinds_dialog() -> DialogDefinition {
    DialogDefinition { args: editor_defs(), ..DialogDefinition::new("editors", LocalizedLabel::native("Edit", "Bearbeiten"), semio_framework::ActionRef::new("apply")) }
}

fn press(shell: &mut ShellState, action: ui_wgpu::wgpu::KeyAction) {
    let mut input = InputState::<ActionDescriptor>::default();
    shell.handle_keyboard(action, &PointerModifiers::default(), &mut input);
}

fn focus(shell: &mut ShellState, stop: ChromeDialogStop) {
    shell.chrome_build.dialog_stack.last_mut().expect("an open dialog").focus_stop(stop);
}

fn staged(shell: &ShellState, id: &str) -> Option<Value> {
    shell.chrome_build.dialog_stack.last().and_then(|request| request.effective().get(id).map(dsl_value_as_json))
}

/// ⚖️ LAW: a chrome dialog stages every editor kind by keyboard alone — a stepper steps inside its bounds, a slider
/// steps, jumps between its detents and to its ends, a segmented field moves between its options, each vector axis is
/// its own labelled field, and a reference field stages the live selection whose chips are focus stops of their own
/// that remove themselves — and the submit carries all of it.
#[test]
fn a_chrome_dialog_stages_every_editor_kind_by_keyboard() {
    let mut shell = session_shell();
    shell.interaction_selection = selection();
    let request = ChromeDialogRequest::from_definition("test", &kinds_dialog(), None, Terminology::default(), Locale::En);
    assert_eq!(request.fields.iter().map(|field| field.label.as_str()).collect::<Vec<_>>(), ["Count", "Angle", "Mode", "Offset x", "Offset y", "Offset z", "Targets"]);
    shell.chrome_build.open_dialog(request);

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
    let stops = shell.chrome_build.dialog_stack.last().expect("dialog").stops();
    assert!(stops.contains(&ChromeDialogStop::Chip(6, 0)) && stops.contains(&ChromeDialogStop::Chip(6, 1)), "each chip is a focus stop");
    press(&mut shell, ui_wgpu::wgpu::KeyAction::Tab);
    assert_eq!(shell.chrome_build.dialog_stack.last().map(ChromeDialogRequest::focused), Some(ChromeDialogStop::Chip(6, 0)));
    press(&mut shell, ui_wgpu::wgpu::KeyAction::Backspace);
    assert_eq!(staged(&shell, "targets"), Some(serde_json::json!(["n2"])));
    assert_eq!(shell.chrome_build.dialog_stack.last().map(ChromeDialogRequest::focused), Some(ChromeDialogStop::Chip(6, 0)), "focus stays on the chip that took its place");

    focus(&mut shell, ChromeDialogStop::Confirm);
    press(&mut shell, ui_wgpu::wgpu::KeyAction::Enter);
    let submit = shell.deferred_actions.pop().expect("the submit dispatched");
    assert_eq!(submit.action, "apply");
    assert_eq!(args_of(&submit), serde_json::json!({ "count": 2.0, "angle": 45.0, "mode": "cut", "offset": [0.0, 7.0, 0.0], "targets": ["n2"] }));
}

/// ⚖️ LAW: the dialog paints each editor accessibly — a slider with one tick per detent announcing its range and value,
/// spin buttons for numbers and axes, pressed segments, named stepper buttons and removable chips — and a pointer picks a
/// segment, steps a stepper and moves a slider through the shared detent law.
#[test]
fn a_chrome_dialog_paints_each_editor_accessibly_and_answers_the_pointer() {
    let mut shell = session_shell();
    shell.interaction_selection = selection();
    let mut request = ChromeDialogRequest::from_definition("test", &kinds_dialog(), None, Terminology::default(), Locale::De);
    request.use_selection(6, &selection());
    let theme = Theme::light();
    let ops = ShellState::chrome_dialog_paint_ops(&request, 1440.0, 900.0, &theme);
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
    shell.chrome_build.open_dialog(request);
    for (x, y) in [centre(cut), centre(increase), (rail.x + rail.w * 0.49, rail.y + rail.h * 0.5)] {
        shell.resolve_chrome_dialog_click(&ops, dialog, x, y);
    }
    assert_eq!(staged(&shell, "mode"), Some(serde_json::json!("cut")));
    assert_eq!(staged(&shell, "count"), Some(serde_json::json!(1.0)));
    assert_eq!(staged(&shell, "angle"), Some(serde_json::json!(45.0)), "the pointer lands on the nearby detent");
    assert!(shell.chrome_build.dialog_open(), "editing a field never closes the dialog");
}
//#endregion 🎛️StagedEditors
