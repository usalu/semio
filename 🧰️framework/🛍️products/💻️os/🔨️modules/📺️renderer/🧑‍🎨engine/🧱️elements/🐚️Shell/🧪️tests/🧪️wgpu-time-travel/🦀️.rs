//! 🧪️ Laws of the wgpu shell's time-travel chrome and its staged editors (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING,
//! packet W2-C).
//!
//! The band law is language-agnostic and shared with the React shell: `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band`
//! (validated by Ajv against its schema and the kernel's `HistoryTimeTravel` schema in the React suite) is read here per
//! session and locale — the band's lines, its controls with the refusal that disables one, the dispatch each sends,
//! the chords and the window indicator. The rest pins the wgpu behaviour around it: the keyboard (remappable chords,
//! Escape never discards), the reveal of the History tab, the indicator, the history-edit refusals, the finalize
//! prompt operated by keyboard alone, every staged editor kind in the Actions form and in a chrome dialog, and the
//! history body the guest's REAL producer builds (paged transaction rows, Edit refused with its reason).

use super::*;
use crate::program_bridge::OperationPublication;
use semio_framework::kernel::{HistoryEntry, HistoryPatch, HistoryReprojection, HistoryTimeTravel, HistoryTimeTravelStage, InvocationResult};
use semio_framework::{ActionArgDef, ActionArgOption, ArgPresentation, ArgSchema, DialogDefinition, DomainSelection};
use semio_framework_plugin::app::time_travel::{history_row_window_path, TimeTravelInputRow, TimeTravelList, TimeTravelListItem};
use semio_framework_plugin::app::{ui_history_panel, CommandView, HistoryMutationPage, HistoryMutationPages, HistoryView, MutationView, TimeTravelEditorPanel, TimeTravelPanel};

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
        TimeTravelVerb::NextProblem => "nextProblem",
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

const ALL_VERBS: [TimeTravelVerb; 8] = [TimeTravelVerb::NextProblem, TimeTravelVerb::Accept, TimeTravelVerb::Discard, TimeTravelVerb::CancelReplay, TimeTravelVerb::Rerun, TimeTravelVerb::Finalize, TimeTravelVerb::Back, TimeTravelVerb::Exit];

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
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    shell.dock.root = crate::dock::DockNode::Stack { windows: vec![DockStackTab::new("main"), DockStackTab::new("side")], active: "main".into() };
    shell.dock.active_window_id = Some("main".into());
    shell.active_window_id = Some("main".into());
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 1, app, view_state: ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) });
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

/// 🎨️ Paints `document` in `surface` to completion with the stepped paint every retained body uses, then registers and
/// publishes its hits, so the surface's accessibility projection is the one the mirror reads; answers the staged input.
fn paint_retained_body(shell: &mut ShellState, surface: &str, document: &UiDocumentLease, body: Rect) -> InputState<ActionDescriptor> {
    let (mut draw, mut atlas, icons, theme) = (DrawList::default(), FontAtlas::builtin(), IconAtlas::default(), Theme::default());
    let mut input = InputState::<ActionDescriptor>::default();
    let (mut scroll, mut collapsed, mut selects) = (HashMap::new(), HashMap::new(), HashMap::new());
    let mut world3d_states = std::mem::take(&mut shell.world3d_states);
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    crate::interpreter::begin_accessibility_visible_documents();
    let mut cursor = UiDocumentFrameCursor::default();
    let painted = (0..SHELL_WINDOW_PAINT_OPPORTUNITIES.min(1 << 20)).any(|_| {
        let mut ctx = framework_widget_context(&mut draw, None, &mut atlas, Some(&icons), &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, body.h);
        let mut hosts = crate::scenes::SceneEngineHosts { chrome_labels: SceneChromeLabels::english(), world3d_states: &mut world3d_states, world_resources: &mut world_resources, window_id: surface };
        render_ui_document_step(&mut cursor, document, body, &mut ctx, surface, "test", ui_wgpu::wgpu::UiDriverDrag::Handle, &mut hosts)
    });
    assert!(painted, "{surface} painted within its opportunity ceiling");
    if surface == FRAMEWORK_PANEL_TAB_HISTORY_ID {
        shell.register_retained_panel_scroll_region(surface, body, &mut input);
    }
    shell.register_retained_body_hits(surface, body, &mut input);
    shell.world3d_states = world3d_states;
    shell.publish_retained_hit_registry(&mut input);
    input
}

/// 🧹️ Closes a law's retained surface and retires every document it published there.
fn close_retained_body(surface: &str, documents: Vec<UiDocumentLease>) {
    if crate::interpreter::request_ui_document_close(surface) {
        while crate::interpreter::ui_document_close_pending_for(surface) {
            assert!(crate::interpreter::close_ui_document_one(), "each admitted close opportunity advances");
        }
    }
    for mut document in documents {
        while !document.close_read_step_with_grant(1, 4096).expect("the law releases its captured document reader").complete {}
    }
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
        let controls: Vec<Value> = time_travel_band_controls(&status)
            .into_iter()
            .map(|control| {
                let mut row = serde_json::json!({ "control": verb_name(control.verb), "controlId": control.verb.control_id(), "action": control.verb.action_id(), "disabledBy": control.disabled_by.map(refusal_key) });
                if control.verb == TimeTravelVerb::NextProblem {
                    row["args"] = control.verb.action(controller, &status).args.as_ref().map(dsl_value_as_json).unwrap_or(Value::Null);
                }
                row
            })
            .collect();
        assert_eq!(Value::from(controls), case["controls"], "{name}");
        for control in time_travel_band_controls(&status).into_iter().filter(|control| control.verb != TimeTravelVerb::NextProblem) {
            let action = control.verb.action(controller, &status);
            assert_eq!((action.controller_id.as_str(), action.args.as_ref().map(dsl_value_as_json)), (controller, Some(serde_json::json!({ "generation": status.generation }))), "{name}: the dispatch carries the generation it was shown");
        }
    }
}

/// ⚖️ LAW: the band's live node speaks exactly the painted message as ONE polite status in every stage — its role never
/// flips — busy while the runtime replays or finalizes; preparation or replay with a known total adds its own progress bar under it,
/// named and valued by the progress line; no session announces nothing.
#[test]
fn the_band_announces_its_message_politely() {
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    assert!(shell.time_travel_status_accessibility_node(1).is_none(), "no session, nothing announced");
    for (name, progress_expected, busy) in [("editing a mutation", false, false), ("editing prepares a history preview with visible localized progress", true, false), ("a running replay", true, true), ("an error downstream", false, false), ("finalizing offers", false, true)] {
        let status = session(name);
        shell.locale_id = "de".into();
        shell.observe_history_time_travel(Some(&status));
        let node = shell.time_travel_status_accessibility_node(1).expect("an open session announces itself");
        assert_eq!((node.role.as_str(), node.busy, node.value_now, node.value_max), ("status", busy, None, None), "{name}: one stable status");
        let lines = time_travel_band_lines(&status, Terminology::Native, Locale::De);
        assert_eq!(node.label, Some(lines.message()), "{name}");
        assert_eq!(node.live, ui_contract::liveness_name(ui_contract::Liveness::Polite));
        let progress = shell.time_travel_progress_accessibility_node(2);
        assert_eq!(progress.is_some(), progress_expected, "{name}: a progress bar while preparation or replay with a known total runs");
        if let Some(progress) = progress {
            assert_eq!((progress.key.as_str(), progress.role.as_str(), progress.depth, progress.value_min, progress.value_now, progress.value_max), (TIME_TRAVEL_BAND_PROGRESS_ID, "progressbar", 1, Some(0.0), Some(f64::from(status.done.unwrap_or(0))), status.total.map(f64::from)), "{name}");
            assert_eq!((progress.label.clone(), progress.value_text.clone()), (lines.progress.clone(), lines.progress.clone()), "{name}: named and valued by the progress line");
            assert_eq!(progress.live, ui_contract::liveness_name(ui_contract::Liveness::Off), "{name}: the status is the one live region");
        }
        let keys: Vec<String> = shell.band_accessibility_nodes(1).into_iter().map(|node| node.key).collect();
        assert_eq!(keys, if progress_expected { vec![TIME_TRAVEL_BAND_STATUS_ID.to_string(), TIME_TRAVEL_BAND_PROGRESS_ID.to_string()] } else { vec![TIME_TRAVEL_BAND_STATUS_ID.to_string()] }, "{name}: the progress follows its status");
    }
    shell.observe_history_time_travel(None);
    assert!(shell.history_time_travel().is_none(), "an absent status closes the session");
    assert_eq!(time_travel_band_lines(&session("an error downstream"), Terminology::Native, Locale::En).message(), "Reviewing the edited history · Errors must be fixed or withdrawn before finalizing · Worst outcome: Error · Accepted changes: 2");
}

/// ⚖️ LAW: the captions are React's `ui.timeTravel.<control>` labels in both locales, and every control dispatches its
/// reserved `historyEdit*` action.
#[test]
fn every_control_reads_reacts_caption_and_names_its_reserved_action() {
    let corpus = corpus();
    for verb in ALL_VERBS {
        let label = &corpus["labels"][verb.label_key()]["normal"];
        assert_eq!((Some(verb.label(Locale::En)), Some(verb.label(Locale::De))), (label["en"].as_str(), label["de"].as_str()), "{}: the corpus caption", verb.label_key());
        assert!(!verb.label(Locale::En).is_empty() && !verb.label(Locale::De).is_empty(), "{}: a caption in both languages", verb.label_key());
        assert!(verb.action_id().starts_with("historyEdit"));
    }
    for (key, tiers) in corpus["labels"].as_object().expect("the corpus's labels") {
        assert_eq!((Some(band_label(key, Locale::En)), Some(band_label(key, Locale::De))), (tiers["normal"]["en"].as_str(), tiers["normal"]["de"].as_str()), "{key}: read from the corpus");
        for locale in Locale::ALL {
            assert_eq!(Some(band_label(key, BandTongue { locale, beginner: true })), tiers["beginner"][locale.as_str()].as_str(), "{key}: the beginner tier in {}", locale.as_str());
            assert!(!band_label(key, BandTongue { locale, beginner: true }).is_empty(), "{key}: no tier is empty");
        }
    }
    let mut shell = session_shell();
    let editing = session("editing a mutation");
    shell.observe_history_time_travel(Some(&editing));
    let accept = |shell: &ShellState| shell.time_travel_band_plan_for(&editing, &Theme::light()).buttons.into_iter().find(|button| button.control.verb == TimeTravelVerb::Accept).expect("Accept is offered").label;
    let terse = accept(&shell);
    shell.chrome_build.driver.label_tier = ui_wgpu::wgpu::UiDriverLabelTier::Beginner;
    let explaining = accept(&shell);
    let caption = &corpus["labels"]["ui.timeTravel.accept"];
    assert!(terse.contains(caption["normal"]["en"].as_str().expect("normal")) && explaining.contains(caption["beginner"]["en"].as_str().expect("beginner")) && terse != explaining, "the band reads the driver's label tier: {terse:?} / {explaining:?}");
    assert_eq!(band_label("ui.timeTravel.unknown", Locale::En), BAND_LABEL_MISSING, "a key the corpus lacks never reads empty");
    let stages = [HistoryTimeTravelStage::Editing, HistoryTimeTravelStage::Replaying, HistoryTimeTravelStage::Reviewing, HistoryTimeTravelStage::Choosing, HistoryTimeTravelStage::Finalizing].map(time_travel_stage_key);
    let reviews = [HistoryTimeTravelReview::NoChanges, HistoryTimeTravelReview::NeedsReplay, HistoryTimeTravelReview::Blocked, HistoryTimeTravelReview::Ready].map(time_travel_review_key);
    let severities = ["ui.mutation.level.info", "ui.mutation.level.warning", "ui.mutation.level.error", "ui.mutation.level.fatal"];
    let mut read: Vec<String> = ALL_VERBS.iter().map(|verb| verb.label_key().to_string()).collect();
    read.extend(stages.iter().chain(reviews.iter()).chain(severities.iter()).chain(BAND_FIXED_LABEL_KEYS.iter()).map(|key| key.to_string()));
    read.extend(corpus["refusals"].as_array().expect("refusals").iter().map(|row| row["label"].as_str().expect("a refusal names its label").to_string()));
    for key in &read {
        for locale in Locale::ALL {
            for beginner in [false, true] {
                assert!(band_label_in_catalog(key, BandTongue { locale, beginner }).is_some(), "{key}: the corpus carries every key the shell reads ({} / beginner {beginner})", locale.as_str());
            }
        }
    }
    for (severity, key) in [(semio_framework::Severity::Info, severities[0]), (semio_framework::Severity::Warning, severities[1]), (semio_framework::Severity::Error, severities[2]), (semio_framework::Severity::Fatal, severities[3])] {
        assert_eq!(Some(time_travel_severity_text(severity, Locale::En)), band_label_in_catalog(key, Locale::En), "{key}: the severity word is that corpus label");
    }
    let silent: Vec<&str> = corpus["refusals"].as_array().expect("refusals").iter().filter(|row| row["silent"] == true).filter_map(|row| row["code"].as_str()).collect();
    assert!(!silent.is_empty() && silent.iter().all(|code| history_refusal_is_silent(code)) && !history_refusal_is_silent("timeTravel.blocked"), "exactly the corpus's silent refusals stay silent: {silent:?}");
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
        (shell.screen_w, shell.screen_h) = (width, height);
        assert_eq!(shell.subfooter_height(&theme), 0.0, "{width}: no band, no row");
        let open = shell.body_rect(&theme);
        assert!((open.y + open.h - (height - theme.footer_height)).abs() < 0.001, "{width}: the body ends at the footer");
        let status = session("a ready review");
        shell.observe_history_time_travel(Some(&status));
        let band = shell.time_travel_band_plan_for(&status, &theme).band;
        let reserved = shell.subfooter_height(&theme);
        assert!((band.y + band.h - height).abs() < 0.001 && (reserved - band.h).abs() < 0.001 && reserved > 0.0, "{width}: the band fills the reserved row: {band:?} / {reserved}");
        let body = shell.body_rect(&theme);
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
        shell.sync_dock_tabs();
        shell.observe_history_time_travel(from.as_ref());
        let anchor = shell.dock_tabs.locate(FRAMEWORK_PANEL_TAB_HISTORY_ID).map(|(anchor, _)| anchor).expect("the History tab is docked");
        shell.anchor_state_mut(anchor).visible = false;
        shell.time_travel_focus = None;
        shell.observe_history_time_travel(to.as_ref());
        assert_eq!(shell.anchor_open(anchor), transition.reveal, "{name}: the History panel opens exactly on a reveal");
        assert_eq!(shell.time_travel_focus.map(|(focus, _)| focus), transition.focus, "{name}: the focus waits for the screen");
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
    shell.resolve_time_travel_focus(&mut input);
    input
}

fn focused_chrome(shell: &ShellState, input: &InputState<ActionDescriptor>) -> Vec<String> {
    shell.chrome_accessibility_nodes(input.hits()).into_iter().filter(|node| node.focused).map(|node| node.key).collect()
}

/// ⚖️ LAW: a replay or a review moves keyboard focus onto the band's live status node — the node the ARIA mirror then
/// focuses, never by Tab — and replay progress leaves it alone; the finalize prompt takes focus once it is open; an
/// editor that never reaches the screen is given up after the settle time, as React gives up after its frames; a person
/// typing in a field elsewhere keeps their focus; a closed session drops a waiting focus.
#[test]
fn a_replay_and_a_review_focus_the_band_the_prompt_takes_focus_and_typing_elsewhere_keeps_it() {
    let mut shell = session_shell();
    shell.session.as_mut().expect("session").app.dialogs.push(finalize_dialog());
    shell.observe_history_time_travel(Some(&session("editing a mutation")));
    assert_eq!(shell.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Editor));
    present_band(&mut shell);
    assert_eq!(shell.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Editor), "no history body on screen yet: still waiting");
    shell.time_travel_focus = Some((TimeTravelFocus::Editor, 0.0));
    let input = present_band(&mut shell);
    assert_eq!((shell.time_travel_focus, focused_chrome(&shell, &input)), (None, Vec::<String>::new()), "past the settle time the move is given up");

    let mut replaying = session("a running replay");
    shell.observe_history_time_travel(Some(&replaying));
    let input = present_band(&mut shell);
    assert_eq!(focused_chrome(&shell, &input), [TIME_TRAVEL_BAND_STATUS_ID]);
    let band = shell.chrome_accessibility_nodes(input.hits()).into_iter().find(|node| node.key == TIME_TRAVEL_BAND_STATUS_ID).expect("the band's live node");
    assert!(band.focusable && !band.tabbable && band.focused, "focused programmatically, never a Tab stop");
    shell.accessibility_focused_control_id = Some("ui.timeTravel.exit".into());
    replaying.done = Some(4);
    shell.observe_history_time_travel(Some(&replaying));
    assert_eq!(shell.time_travel_focus, None, "progress inside the replay moves nothing");

    shell.observe_history_time_travel(Some(&session("an error downstream")));
    let input = present_band(&mut shell);
    assert_eq!(focused_chrome(&shell, &input), [TIME_TRAVEL_BAND_STATUS_ID], "the review lands on the band that names what blocks");

    shell.observe_history_time_travel(Some(&session("the finalize prompt can go back")));
    assert_eq!(shell.time_travel_focus.map(|(focus, _)| focus), Some(TimeTravelFocus::Dialog));
    present_band(&mut shell);
    assert!(shell.time_travel_focus.is_some(), "the prompt is not open yet");
    open_finalize_prompt(&mut shell, "Alternative");
    let input = present_band(&mut shell);
    assert!(shell.time_travel_focus.is_none() && input.hits().is_empty() && shell.chrome_build.dialog_open(), "the open prompt holds focus; under it the band registers nothing");

    let mut typing = session_shell();
    typing.observe_history_time_travel(Some(&session("a running replay")));
    let mut input = InputState::<ActionDescriptor>::default();
    input.focus_input_owned("ui.search.input".to_string(), "dra".to_string());
    typing.resolve_time_travel_focus(&mut input);
    assert_eq!((typing.time_travel_focus, typing.accessibility_focused_control_id.clone()), (None, None), "a person typing elsewhere keeps their focus");

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
        tree_pointer_record(3, "framework.history.editor.accept.row", component(serde_json::json!({ "type": "treeItem", "label": "Accept", "icon": "check" })), &[5], None),
        tree_pointer_record(5, TIME_TRAVEL_EDITOR_ACCEPT_KEY, component(serde_json::json!({ "type": "button", "label": "Accept", "icon": "check" })), &[], Some("historyEditAccept")),
        tree_pointer_record(4, TIME_TRAVEL_EDITOR_INPUTS_KEY, component(serde_json::json!({ "type": "treeSection", "label": "Inputs", "defaultOpen": true })), &[6], None),
        tree_pointer_record(6, "framework.history.editor.input.dx.row", component(serde_json::json!({ "type": "treeItem", "label": "Offset x", "icon": "move" })), &[7], None),
        tree_pointer_record(7, "framework.history.editor.input.dx", component(serde_json::json!({ "type": "numberStepper", "value": 2.0, "step": 1.0, "uniform": false })), &[], None),
    ];
    let mut shell = session_shell();
    let document = shell.publish_surface_records(SURFACE, records).expect("the history body publishes");
    shell.observe_history_time_travel(Some(&session("editing a mutation")));
    let _ = paint_retained_body(&mut shell, SURFACE, &document, Rect::new(0.0, 0.0, 420.0, 640.0));
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
    assert_eq!((time_travel_indicator_caption(Locale::En), time_travel_indicator_caption(Locale::De)), ("History editing", "Verlaufsbearbeitung"));
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
    static PUBLISHED: std::cell::RefCell<Vec<OperationPublication>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// 📶️ A guest bridge whose publication lane is whatever the law queued, taken once.
fn queued_publications(_instance_id: u32) -> Vec<OperationPublication> {
    PUBLISHED.with(|queue| std::mem::take(&mut *queue.borrow_mut()))
}

/// 🎞️ A UI-progress publication that carries a history patch and no scope.
fn progress_patch(patch: HistoryPatch) -> OperationPublication {
    OperationPublication { completed: false, ui_scope: UiDirtyScope::None, history_patch: Some(patch), fault: None, resync: false }
}

/// 🏛️ The host fixture shell with its guest program's publication lane under the law's hand and nothing owed.
fn publication_shell() -> ShellState {
    let mut shell = panel_anchor_model_tests::host_test_shell();
    shell.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("host fixture guest program").install_fixture_publications(queued_publications);
    PUBLISHED.with(|queue| queue.borrow_mut().clear());
    shell.owed_refresh_scope = UiDirtyScope::None;
    shell
}

/// ⚖️ LAW (React's `subscribeOperationProgress` lane): the progress patches the guest pushed on uncorrelated
/// `Invocation` frames move the band between two dispatches, oldest first through the reply's stale guard — a patch
/// older than the one already folded rolls nothing back, an empty take is no change, and progress that carries a patch
/// alone re-renders nothing.
#[test]
fn unsolicited_progress_patches_move_the_band_between_dispatches() {
    let mut shell = publication_shell();
    semio_framework_async::block_on(shell.observe_invocation_history(Some(&HistoryPatch { cursor: 5, time_travel: Some(session("editing a mutation")), ..Default::default() })));
    assert!(!semio_framework_async::block_on(shell.drain_operation_publications()), "nothing queued is no change");
    let replaying = session("a running replay");
    PUBLISHED.with(|queue| queue.borrow_mut().extend([progress_patch(HistoryPatch { cursor: 5, time_travel: Some(replaying.clone()), ..Default::default() }), progress_patch(HistoryPatch { cursor: 4, ..Default::default() })]));
    assert!(semio_framework_async::block_on(shell.drain_operation_publications()), "the queued progress moved the band");
    assert_eq!(shell.history_time_travel(), Some(&replaying), "the stale patch after it closed nothing");
    assert!(shell.owed_refresh_scope.asks_for_nothing(), "a patch without a scope on a progress frame re-renders nothing");
    assert!(PUBLISHED.with(|queue| queue.borrow().is_empty()), "the drain took the queue");
    assert!(!semio_framework_async::block_on(shell.drain_operation_publications()));
}

/// ⚖️ LAW (live fault F11; React's `subscribeOperationCompletions` pass): a typed operation that ends AFTER the host
/// call that started it — an example load of a few hundred steps, the replay behind an accepted history edit — reaches
/// the shell on the publication lane alone. With no further dispatch its progress scope and its completion are owed to
/// the settle lane as ONE refresh (the board's body and the History body whose row the completion minted), the row
/// itself is folded into the projection, and the settle pump has work.
#[test]
fn a_typed_operation_that_ends_after_its_host_call_refreshes_and_publishes_its_rows() {
    let mut shell = publication_shell();
    let before = history_rows_oldest_first(&shell.history_entries).len();
    let board = UiDirtyScope::Partial { window_bodies: vec!["board".into()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false };
    let row = HistoryEntry { seq: 41, action_id: "setActiveExample".into(), label: LocalizedLabel::native("Load example", "Beispiel laden"), kind: "command".into(), applied: true, ..Default::default() };
    PUBLISHED.with(|queue| {
        queue.borrow_mut().extend([
            OperationPublication { completed: false, ui_scope: board.clone(), history_patch: None, fault: None, resync: false },
            OperationPublication { completed: true, ui_scope: UiDirtyScope::None, history_patch: Some(HistoryPatch { cursor: 41, upserts: vec![row], ..Default::default() }), fault: None, resync: false },
        ])
    });
    assert!(semio_framework_async::block_on(shell.drain_operation_publications()), "the lane alone changed the shell");
    let rows = history_rows_oldest_first(&shell.history_entries);
    assert_eq!(rows.len(), before + 1, "the completion's row is in the projection");
    assert!(rows.iter().any(|entry| entry.key() == "seq:41"));
    assert!(shell.owed_refresh_scope.wants_window_body("board"), "the progress scope is owed: {:?}", shell.owed_refresh_scope);
    assert!(shell.owed_refresh_scope.wants_panel_body(ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY), "the History body owes the minted row: {:?}", shell.owed_refresh_scope);
    assert!(!shell.owed_refresh_scope.wants_window_body("another"), "one merged pass, never a full one");
    assert!(shell.settle_pump_pending(), "the settle lane refreshes without a dispatch");
    assert!(shell.transient_notice().is_none(), "a completion is told by its rows, not by a notice");
}

/// ⚖️ LAW (live fault F11): a typed-operation drain that stopped is never silence — its fault reaches the person
/// through the dispatch-fault funnel as an error notice, and an answer of the bridge this host cannot read is told the
/// same way and re-reads everything it may have hidden.
#[test]
fn a_refused_completion_is_told_as_a_notice() {
    let mut shell = publication_shell();
    PUBLISHED.with(|queue| queue.borrow_mut().push(OperationPublication { completed: false, ui_scope: UiDirtyScope::None, history_patch: None, fault: Some("browser-actor-publication: noncanonical bytes".into()), resync: false }));
    assert!(semio_framework_async::block_on(shell.drain_operation_publications()), "a told fault repaints");
    let notice = shell.transient_notice().expect("the refused completion is told");
    assert!(notice.message.contains("noncanonical bytes"), "the notice names what was refused: {}", notice.message);
    assert!(shell.owed_refresh_scope.asks_for_nothing(), "a fault alone re-renders nothing");
    let unreadable = crate::program_bridge::operation_publications_from_json("{\"completed\":true}");
    assert_eq!(unreadable.len(), 1);
    assert!(unreadable[0].fault.as_deref().is_some_and(|fault| fault.starts_with("wgpu-bridge.operation-publication.unreadable")) && unreadable[0].resync, "an unreadable answer is a fault and a re-read: {unreadable:?}");
    let mangled = crate::program_bridge::operation_publications_from_json("[{\"completed\":true,\"historyPatch\":{\"cursor\":\"seven\"}}]");
    assert!(mangled[0].fault.as_deref().is_some_and(|fault| fault.contains("historyPatch")) && mangled[0].resync, "an unreadable entry names its member: {mangled:?}");
    assert_eq!(operation_publication_refresh(&mangled[0]), UiDirtyScope::Full);
}

/// ⚖️ LAW (shared corpus `🧫️operation-publication`, the bridge's own producer reads the same rows): every publication
/// the browser bridge hands over as JSON is read back exactly, owes exactly the refresh its row names, and carries a
/// fault exactly when the row says it is told.
#[test]
fn every_operation_publication_owes_what_the_shared_corpus_says() {
    let corpus: Value = serde_json::from_str(include_str!("../../../🛠️ShellHelpers/🧫️fixtures/🧫️operation-publication/🔣️.json")).expect("the shared operation-publication corpus parses");
    assert_eq!(corpus["historyBodyKey"].as_str(), Some(ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY));
    let rows = corpus["rows"].as_array().expect("the corpus rows");
    assert!(rows.len() >= 7, "the corpus covers completion, progress, fault and resync");
    for row in rows {
        let name = row["name"].as_str().expect("name");
        let read = crate::program_bridge::operation_publications_from_json(&Value::Array(vec![row["publication"].clone()]).to_string());
        assert_eq!(read.len(), 1, "{name}");
        let publication = &read[0];
        assert_eq!(publication.completed, row["publication"]["completed"].as_bool().expect("completed"), "{name}");
        assert_eq!(publication.history_patch.is_some(), row["publication"].get("historyPatch").is_some(), "{name}: its patch is read");
        assert_eq!(publication.fault.is_some(), row["told"].as_bool().expect("told"), "{name}: told exactly when it carries a fault");
        let refresh = semio_framework_pack_json::from_json_str::<UiDirtyScope>(&row["refresh"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{name}: refresh scope: {error}"));
        assert_eq!(operation_publication_refresh(publication), refresh, "{name}");
        assert_eq!(row["queued"].as_bool().expect("queued"), publication.fault.is_some() || publication.history_patch.is_some() || publication.resync || !publication.ui_scope.asks_for_nothing(), "{name}: the bridge queues exactly what says something");
    }
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
                let (message, notice_severity, notice_code) = classify_dispatch_fault_notice(&fault, None, Terminology::Native, locale);
                assert_eq!((message.as_str(), notice_severity, notice_code.as_deref()), (text, severity, Some(code)), "{fault}");
            }
            let status = HistoryTimeTravel { fault: Some(code.to_string()), ..session("editing a mutation") };
            assert_eq!(time_travel_band_lines(&status, Terminology::Native, locale).fault.as_deref(), Some(text), "{code}: the band's fault line");
        }
    }
    assert!(history_refusal_notice("app.command.rejected: conflicting edit", Locale::En).is_none());
    assert!(history_refusal_of_fault("app.command.rejected: the history.transition-refusedness of it", Locale::En).is_none(), "a code inside a word is no code");
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
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
        let store_sync::sync::CommandAckOutcome::Rejected { code, messages, .. } = semio_framework_pack_json::from_json_str::<store_sync::sync::CommandAckOutcome>(&rejection, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{name}: a CommandRejectionV1: {error}")) else {
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

/// ⚖️ LAW (the kernel's `🧫️history-notices`, the one copy every shell shows; design §20.12): a history-lane refusal — an open
/// tool recording, an ended one, a full history with its count, a history step still replaying (`history.replaying`, gap N17) —
/// reaching the shell as a refused guest dispatch, also behind the bridge's prefix, is a warning notice in both locales carrying
/// its code, `{n}` from the structured `Fault.params` alone; the ARIA mirror's `shell.notice` names it by the message and
/// describes it by the code. A dispatch-fault string without the structured refusal behind it is never read for a count.
#[test]
fn every_history_lane_refusal_is_a_notice_carrying_its_code() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧫️history-notices/🔣️.json")).expect("the kernel's history-notices fixture parses");
    let notices = fixture["notices"].as_array().expect("notices");
    assert_eq!(notices.iter().map(|notice| notice["code"].as_str().expect("code")).collect::<Vec<_>>(), semio_framework::kernel::HISTORY_NOTICE_LABELS.iter().map(|(code, _, _)| *code).collect::<Vec<_>>(), "the fixture is the kernel's table");
    for notice in notices {
        let code = notice["code"].as_str().expect("code");
        let refused = RefusedGuestFault { fault: semio_framework::Fault::new(semio_framework::FaultOrigin::Module, code.to_string(), "refused").with_param("n", "64"), notices: Vec::new() };
        for (locale, locale_id, tongue) in [(Locale::En, "en", "en"), (Locale::De, "de", "de")] {
            let text = notice[tongue].as_str().expect("text").replace("{n}", "64");
            for fault in [format!("{code}: the edit history holds at most 64 edits and is full"), format!("handle_action failed: {code}: refused")] {
                assert_eq!(classify_dispatch_fault_notice(&fault, Some(&refused), Terminology::Native, locale), (text.clone(), semio_framework::Severity::Warning, Some(code.to_string())), "{fault}");
            }
            let mut shell = session_shell();
            shell.locale_id = locale_id.into();
            shell.chrome_build.refused_guest_fault = Some(refused.clone());
            shell.note_dispatch_fault(&format!("{code}: refused"));
            let told: Vec<(Option<String>, Option<String>)> = shell.chrome_accessibility_nodes(&[]).into_iter().filter(|node| node.key == TRANSIENT_NOTICE_STATUS_ID).map(|node| (node.label, node.description)).collect();
            assert_eq!(told, [(Some(text.clone()), Some(code.to_string()))], "{code} ({tongue}): the mirror tells the notice by its message and its code");
        }
    }
    assert_eq!(classify_dispatch_fault_notice("history.full: refused at 64", None, Terminology::Native, Locale::En).2, None, "no count is ever read from a fault message");
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
                    withdrawable: true,
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
            let presence = time_travel_peer_presence(editing, &entries, Terminology::Native, locale);
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
    assert_eq!(announced, "Ada (Ada bearbeitet Skalieren im Verlauf) · Cy · Bo (Bo bearbeitet Drehen im Verlauf)");
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
            Some(UiControlNode::Button(button)) => Some((button.id.clone().unwrap_or_default(), button.label.as_str().to_string(), args_of(&button.action)["value"].clone(), button.presence.state == ui_wgpu::wgpu::component::ui::UiState::Disabled)),
            _ => None,
        })
        .collect();
    assert_eq!(buttons, [("targets.remove.a".to_string(), "Remove a".to_string(), serde_json::json!([]), false), ("targets.useSelection".to_string(), "Use current selection".to_string(), serde_json::json!(["n1", "n2"]), false)]);
    let empty = staged_action_arg_row("main", "move", &defs[4], None, &HashMap::new(), Terminology::Native, Locale::De).items.expect("rows");
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
    let corpus: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🧫️number-facets/🔣️.json")).expect("the shared number-facets corpus parses");
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
    let request = ChromeDialogRequest::from_definition("test", &kinds_dialog(), None, Terminology::Native, Locale::En);
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
    let mut request = ChromeDialogRequest::from_definition("test", &kinds_dialog(), None, Terminology::Native, Locale::De);
    request.use_selection(6, &selection());
    let theme = Theme::light();
    let ops = ShellState::chrome_dialog_paint_ops(&request, 1440.0, 900.0, &theme, &mut FontAtlas::builtin());
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

//#region 📚️HistoryBody
/// 🔢️ Operations of the law's one drag transaction, and how many of them its history row projects.
const HISTORY_BODY_OPERATIONS: u32 = 120;
const HISTORY_BODY_PROJECTED: u32 = 3;

/// 🧱️ Publishes the guest producer through the same bounded, parent/key reconciler as its runtime.
fn publish_history_body(current: &mut Option<semio_framework_ui_runtime::SurfaceReconciler>, built: ui_contract::BuiltNode, generation: u64) -> UiDocumentLease {
    use semio_framework_ui_runtime::{ComponentTree, SurfaceReconcileJob, SurfaceReconcileJobStep, SurfaceReconciler};
    let surface = SurfaceId::try_from(FRAMEWORK_PANEL_TAB_HISTORY_ID).expect("History surface");
    let mut job = SurfaceReconcileJob::try_new(current.take().unwrap_or_else(|| SurfaceReconciler::new(surface)), ComponentTree { root: built }, generation).expect("history producer admission");
    let mut sequence = 0;
    for _ in 0..100_000 {
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us, &mut sequence,
        );
        match job.drive_one(&mut context) {
            SurfaceReconcileJobStep::Ready => break,
            SurfaceReconcileJobStep::Fault => panic!("history producer refused: {:?}", job.fault()),
            SurfaceReconcileJobStep::MoreWork => {}
        }
    }
    let mut patch = None;
    assert_eq!(job.take_ready_into(current, &mut patch, SurfaceReconcileJob::required_ready_transfer_bytes()), Ok(true), "history producer completes within its bound");
    if let Some(mut patch) = patch { while !patch.close_step() {} }
    let mut terminal = job.into_terminal();
    while !terminal.close_step() {}
    let mut document = None;
    assert_eq!(current.as_ref().expect("published reconciler").capture_document(&mut document, usize::MAX), Ok(true));
    document.expect("canonical history document")
}

/// 🧹️ Retires the fixture publisher after its renderer and document readers release their roots.
fn close_history_producer(current: &mut Option<semio_framework_ui_runtime::SurfaceReconciler>) {
    if let Some(current) = current.take() {
        let mut terminal = semio_framework_ui_runtime::SurfaceReconcileTerminal::try_from_reconciler(current, 1000).expect("publisher close admission");
        while !terminal.close_step() {}
    }
}

/// ✏️ Operation `index` of the drag transaction, editable.
fn drag_mutation(index: u32) -> MutationView {
    MutationView { mutation_id: format!("m-{index}"), position: 0, op_index: index, label: LocalizedLabel::native("Drag selection", "Auswahl ziehen"), worst: None, messages: Vec::new(), superseded: false, withdrawn: false, editable: true, withdrawable: true, store: None }
}

/// ⏳️ A running replay: `Begin` is refused (`Illegal`), so every Edit is disabled with its reason.
fn replaying_panel() -> TimeTravelPanel {
    TimeTravelPanel {
        status: session("a running replay"),
        store: None,
        stage: semio_framework_time_travel::TimeTravelStage::Replaying,
        pending_after: None,
        finalize_refusal: None,
        review: None,
        rerun_refusal: None,
        begin_refusal: Some(semio_framework_time_travel::TimeTravelRefusal::Illegal),
        next_problem: None,
        editor: None,
        outcomes: Default::default(),
        edited: Default::default(),
        accepted: Default::default(),
    }
}

/// 📋️ A session editing `m-1` whose draft holds a full list (`/points`: two items, `minItems` = `maxItems` = 2) and a
/// two-chip reference list (`/targets`, `minItems` 1, `n1` labelled "Corner"/"Ecke") — the N2 list rows.
fn list_editor_panel() -> TimeTravelPanel {
    let row = |pointer: &str, input: ActionArgDef, value: Value, list: Option<TimeTravelList>, item: Option<TimeTravelListItem>| TimeTravelInputRow { pointer: pointer.into(), label: input.label.clone(), input, value: DslValue::from(value), list, item };
    let point = |index: usize, value: f64| row(&format!("/points/{index}"), ActionArgDef::number("point", LocalizedLabel::native("Point", "Punkt")), serde_json::json!(value), None, Some(TimeTravelListItem { index, removable: false, min: Some(2) }));
    let targets = ActionArgDef {
        schema: ArgSchema::Reference { kinds: vec!["node".into()], domain: Some("vortex".into()), granularity: Some("node".into()), many: true, min_items: Some(1), max_items: None, id_type: semio_framework::ReferenceIdType::String },
        ..ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele"))
    };
    TimeTravelPanel {
        status: session("editing a mutation"),
        stage: semio_framework_time_travel::TimeTravelStage::Editing,
        begin_refusal: None,
        editor: Some(TimeTravelEditorPanel {
            target: "m-1".into(),
            label: LocalizedLabel::native("Drag selection", "Auswahl ziehen"),
            rows: vec![
                row("/points", ActionArgDef::text("points", LocalizedLabel::native("Points", "Punkte")), Value::Null, Some(TimeTravelList { len: 2, addable: false, max: Some(2) }), None),
                point(0, 1.0),
                point(1, 2.0),
                row("/targets", targets, serde_json::json!(["n1", "n2"]), None, None),
            ],
            editable: true,
            inputs_refused: None,
            withdrawn: false,
            outcome: Vec::new(),
            refused: None,
            changed: false,
            reference_labels: std::collections::BTreeMap::from([("n1".to_string(), LocalizedLabel::native("Corner", "Ecke"))]),
        }),
        ..replaying_panel()
    }
}

/// 📚️ The history body the guest publishes, built by its REAL producer `ui_history_panel`: one transaction row of
/// [`HISTORY_BODY_OPERATIONS`] operations ([`HISTORY_BODY_PROJECTED`] projected), the host's window `(offset, rows)` over that
/// row with the store's page past the projection, the session `panel` when a history edit is open, and the history step
/// still replaying (`reprojection`).
fn history_body(current: &mut Option<semio_framework_ui_runtime::SurfaceReconciler>, window: Option<(u32, u32)>, panel: Option<TimeTravelPanel>, reprojection: Option<HistoryReprojection>, locale: Locale, generation: u64) -> UiDocumentLease {
    let history = HistoryView {
        commands: vec![CommandView {
            seq: 1,
            action_id: "drag".into(),
            label: LocalizedLabel::native("Drag selection", "Auswahl ziehen"),
            kind: semio_framework::ActionKind::Mutation,
            timestamp: "2026-10-02T10:00:00Z".into(),
            edit_id: Some("e-1".into()),
            child_edit_ids: Vec::new(),
            transition_id: None,
            author: None,
            op_lines: Vec::new(),
            op_count: HISTORY_BODY_OPERATIONS as usize,
            applied: true,
            revertible: false,
            count: 1,
            inverse: None,
            transaction: None,
            mutations: (0..HISTORY_BODY_PROJECTED).map(drag_mutation).collect(),
        }],
        ..HistoryView::empty()
    };
    let mut pages = HistoryMutationPages::new();
    let mut view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    if let Some((offset, rows)) = window {
        let start = offset.min(HISTORY_BODY_OPERATIONS.saturating_sub(rows));
        let paged = start.max(HISTORY_BODY_PROJECTED);
        pages.insert(1, HistoryMutationPage { from: (paged - HISTORY_BODY_PROJECTED) as usize, rows: (paged..(start + rows).min(HISTORY_BODY_OPERATIONS)).map(drag_mutation).collect() });
        view.tree_windows = vec![semio_framework::TreeWindowRequest { body_key: ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY.into(), node_key: history_row_window_path(1), open: Some(true), offset, rows }];
    }
    let built = semio_framework_async::block_on(ui_history_panel(&history, panel.as_ref(), reprojection.as_ref(), &pages, "s.test.history", locale, false, false, &view)).expect("the guest's history body assembles");
    let viewport: Value = serde_json::from_str(include_str!("../../../../../../🔌️plugin/🧫️fixtures/history-panel-command-window/🔣️.json")).expect("the neutral history viewport fixture");
    assert_eq!(serde_json::to_value(&built.layout).expect("history viewport layout"), viewport["rootLayout"]);
    publish_history_body(current, built, generation)
}

/// 🔑️ Reads the canonical identity a retained document assigns to an authored key.
fn history_document_node_id(document: &UiDocumentLease, key: &str) -> ui_contract::UiNodeId {
    let read = document.try_read().expect("history document read");
    (0..read.len()).find_map(|ordinal| read.node_at(ordinal).filter(|record| record.key.as_str() == key).map(|record| record.id)).expect("history document contains key")
}

/// ♿️ The History surface's mirror node whose key names `key`.
fn history_mirror_node(key: &str) -> Option<ui_contract::AccessibilityProjectionNode> {
    crate::interpreter::published_accessibility_nodes_for_test(FRAMEWORK_PANEL_TAB_HISTORY_ID).into_iter().find(|node| projected_key_is(&node.key, key))
}

/// 📜️ The mutation rows the History surface's mirror shows, in reading order.
fn history_mirror_mutations() -> Vec<String> {
    crate::interpreter::published_accessibility_nodes_for_test(FRAMEWORK_PANEL_TAB_HISTORY_ID)
        .into_iter()
        .filter_map(|node| node.key.rsplit('/').next().and_then(|key| key.strip_prefix("framework.history.mutation.")).filter(|id| !id.contains("::")).map(str::to_owned))
        .collect()
}

/// 🛞️ Paints `document` and wheels the History body down over its rows, a few rows a notch, until the mutation row `wanted`
/// is projected — the person scrolling to a window the tree-window observer asked for.
fn wheel_history_window_into_view(shell: &mut ShellState, surface: &str, document: &UiDocumentLease, body: Rect, wanted: &str) {
    let mut input = paint_retained_body(shell, surface, document, body);
    for _ in 0..256 {
        if history_mirror_mutations().iter().any(|id| id == wanted) {
            return;
        }
        let (x, y, row_h) = input.hits().iter().find(|hit| hit.kind == HitKind::TreeItem && body.contains(hit.rect.x + 1.0, hit.rect.y + 1.0)).map_or((body.x + body.w * 0.5, body.y + body.h * 0.5, 24.0), |hit| (hit.rect.x + hit.rect.w * 0.5, hit.rect.y + hit.rect.h * 0.5, hit.rect.h));
        assert!(shell.handle_pointer_wheel(x, y, 0.0, row_h * 3.0, &mut input), "the History body owns the wheel");
        input = paint_retained_body(shell, surface, document, body);
    }
    panic!("`{wanted}` never scrolled into the History body: windows={:?} shown={:?}", crate::interpreter::tree_window_measures(surface), history_mirror_mutations());
}

/// 🖱️ Activates one History mirror node the way the ARIA mirror's click does; answers what it dispatched.
fn activate_history_mirror_node(shell: &mut ShellState, key: &str) -> Vec<ActionDescriptor> {
    let node = history_mirror_node(key).unwrap_or_else(|| panic!("`{key}` is in the mirror"));
    let target = crate::interpreter::published_accessibility_target_for_test(FRAMEWORK_PANEL_TAB_HISTORY_ID, &node.key).expect("a mirror node is an accessibility target");
    let mut input = InputState::<ActionDescriptor>::default();
    semio_framework_async::block_on(shell.handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("the activation is handled");
    crate::collect_fixture_actions(&mut input)
}

/// ⚖️ LAW (gaps N1 + N15 on wgpu, through the guest's REAL producer and the ARIA mirror): a closed transaction row of 120
/// operations announces that it folds open (its window's total) and opens by the mirror's activation; the wgpu tree-window
/// observer then asks the guest for that row by the SAME window path the guest reads
/// (`framework.history.commands␟framework.history.entry.1`), and the slice it answers renders past the projected
/// mutations; a window at 100, wheeled into view (the mirror projects what the body shows), shows exactly operations 100–107.
/// While a replay runs, every Edit is an
/// aria-disabled button that tells the reason ("Not possible right now" / "Derzeit nicht möglich", in its name or its
/// description) and activating it or its row begins nothing; with Begin allowed the same button begins an edit of its mutation.
#[test]
fn the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason() {
    let mut producer = None;
    let surface = FRAMEWORK_PANEL_TAB_HISTORY_ID;
    let body = Rect::new(0.0, 0.0, 420.0, 640.0);
    let row = "framework.history.entry.1";
    let mut shell = session_shell();
    let mut documents = vec![history_body(&mut producer, None, None, None, Locale::En, 1)];
    let _ = paint_retained_body(&mut shell, surface, &documents[0], body);
    let closed = history_mirror_node(row).expect("the transaction row is in the mirror");
    assert_eq!(closed.expanded, Some(false), "a closed row of {HISTORY_BODY_OPERATIONS} operations announces it folds open, never a leaf: {closed:?}");
    assert!(history_mirror_mutations().is_empty(), "a closed row materialises no mutation");

    let _ = activate_history_mirror_node(&mut shell, row);
    let _ = paint_retained_body(&mut shell, surface, &documents[0], body);
    assert_eq!(history_mirror_node(row).and_then(|node| node.expanded), Some(true), "the mirror's activation opens the row");
    shell.observe_tree_windows(surface);
    let (requests, _) = shell.tree_windows.view_state_fields();
    let path = history_row_window_path(1);
    let asked = requests.iter().find(|request| request.body_key == ui_wgpu::wgpu::FRAMEWORK_HISTORY_BODY_KEY && request.node_key == path).cloned().unwrap_or_else(|| panic!("the opened row is asked for by the guest's window path {path:?}: {requests:?}"));
    assert!(asked.open == Some(true) && asked.rows > HISTORY_BODY_PROJECTED, "the request opens the row over more than its projection: {asked:?}");

    documents.push(history_body(&mut producer, Some((asked.offset, asked.rows)), None, None, Locale::En, 2));
    let _ = paint_retained_body(&mut shell, surface, &documents[1], body);
    let shown = history_mirror_mutations();
    assert!(shown.len() > HISTORY_BODY_PROJECTED as usize, "the answered window renders past the projected mutations, paged from the store: {shown:?}");
    assert_eq!(shown, (0..shown.len() as u32).map(|index| format!("m-{index}")).collect::<Vec<_>>(), "in op order from the first");

    for (generation, locale, edit_word, reason) in [(3, Locale::En, "Edit", "Not possible right now"), (4, Locale::De, "Bearbeiten", "Derzeit nicht möglich")] {
        documents.push(history_body(&mut producer, Some((100, 8)), Some(replaying_panel()), None, locale, generation));
        assert_eq!(history_document_node_id(documents.last().expect("published"), row), history_document_node_id(&documents[1], row), "an inserted editor retains the opened transaction's canonical identity");
        wheel_history_window_into_view(&mut shell, surface, documents.last().expect("published"), body, "m-100");
        assert_eq!(history_mirror_mutations(), (100..108).map(|index| format!("m-{index}")).collect::<Vec<_>>(), "{locale:?}: the scrolled window shows exactly the operations it asked for");
        for index in 100..108 {
            let edit = history_mirror_node(&format!("framework.history.mutation.m-{index}::row-action::0")).unwrap_or_else(|| panic!("{locale:?}: m-{index}'s Edit is in the mirror"));
            assert!(edit.role == "button" && edit.disabled && !edit.actionable, "{locale:?}: a refused Edit is an aria-disabled button: {edit:?}");
            let told = [edit.label.as_deref(), edit.description.as_deref()].into_iter().flatten().collect::<Vec<_>>().join(" · ");
            assert!(told.contains(edit_word) && told.contains(reason), "{locale:?}: it is named Edit and tells the reason: {told:?}");
        }
        let begun = [activate_history_mirror_node(&mut shell, "framework.history.mutation.m-100::row-action::0"), activate_history_mirror_node(&mut shell, "framework.history.mutation.m-100")].concat();
        assert!(!begun.iter().any(|action| action.action == semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID), "{locale:?}: neither the refused Edit nor its row begins an edit: {begun:?}");
    }

    documents.push(history_body(&mut producer, Some((100, 8)), None, None, Locale::En, 5));
    wheel_history_window_into_view(&mut shell, surface, documents.last().expect("published"), body, "m-100");
    let edit = history_mirror_node("framework.history.mutation.m-100::row-action::0").expect("m-100's Edit is in the mirror");
    assert!(!edit.disabled && edit.actionable && edit.label.as_deref().is_some_and(|label| label.starts_with("Edit:")), "Begin allowed: Edit is an enabled button: {edit:?}");
    let begun = activate_history_mirror_node(&mut shell, "framework.history.mutation.m-100::row-action::0");
    assert_eq!(begun.iter().map(|action| (action.action.as_str(), args_of(action)["mutationId"].clone())).collect::<Vec<_>>(), [(semio_framework::HISTORY_EDIT_BEGIN_ACTION_ID, serde_json::json!("m-100"))], "and begins an edit of its own mutation");
    close_retained_body(surface, documents);
    close_history_producer(&mut producer);
    eprintln!("[DEBUG] WGPU history paging: closed/opened windows, EN/DE refused actions, enabled target dispatch, and exact reader/producer retirement verified");
}

/// ⚖️ LAW (gap N2 on wgpu, through the guest's REAL producer and the ARIA mirror): a full list's "Add item" is a disabled
/// button and its row names the count and the ceiling; an item at the floor keeps a disabled "Remove item" whose row names
/// the floor; a reference list above its floor shows one removable chip per id, named by the entity's label (else its
/// id), and a chip's activation drafts `historyEditInput{path: "/targets/<i>", edit: "remove"}` — in English and German;
/// the disabled controls dispatch nothing.
#[test]
fn the_guest_editor_offers_list_and_chip_edits_within_their_bounds() {
    let mut producer = None;
    let surface = FRAMEWORK_PANEL_TAB_HISTORY_ID;
    let body = Rect::new(0.0, 0.0, 420.0, 900.0);
    let mut shell = session_shell();
    let mut documents = Vec::new();
    for (generation, locale, add, count, remove, floor, chips) in [
        (1, Locale::En, "Add item", "Items: 2 · Maximum 2 items", "Remove item", "Minimum 2 items", ["Remove Corner", "Remove n2"]),
        (2, Locale::De, "Element hinzufügen", "Elemente: 2 · Höchstens 2 Einträge", "Element entfernen", "Mindestens 2 Einträge", ["Entfernen Ecke", "Entfernen n2"]),
    ] {
        documents.push(history_body(&mut producer, None, Some(list_editor_panel()), None, locale, generation));
        let _ = paint_retained_body(&mut shell, surface, documents.last().expect("published"), body);
        let node = |key: &str| history_mirror_node(key).unwrap_or_else(|| panic!("{locale:?}: `{key}` is in the mirror"));
        let add_button = node("framework.history.editor.input.points.add");
        assert!(add_button.role == "button" && add_button.disabled && add_button.label.as_deref() == Some(add), "{locale:?}: a full list's Add item is disabled: {add_button:?}");
        assert!(node("framework.history.editor.input.points.row").description.as_deref().is_some_and(|description| description.contains(count)), "{locale:?}: the list row names its count and ceiling: {:?}", node("framework.history.editor.input.points.row"));
        for index in 0..2 {
            let button = node(&format!("framework.history.editor.input.points.{index}.remove"));
            assert!(button.disabled && button.label.as_deref() == Some(remove), "{locale:?}: an item at the floor keeps a disabled Remove item: {button:?}");
            assert_eq!(node(&format!("framework.history.editor.input.points.{index}.remove.row")).description.as_deref(), Some(floor), "{locale:?}: its row names the floor");
        }
        for (index, chip) in chips.iter().enumerate() {
            let button = node(&format!("framework.history.editor.input.targets.chip.{index}"));
            assert!(button.role == "button" && !button.disabled && button.actionable && button.label.as_deref() == Some(*chip), "{locale:?}: a removable chip named by its entity: {button:?}");
        }
        assert!(activate_history_mirror_node(&mut shell, "framework.history.editor.input.points.add").is_empty(), "{locale:?}: the disabled Add item dispatches nothing");
        let removed = activate_history_mirror_node(&mut shell, "framework.history.editor.input.targets.chip.0");
        assert_eq!(
            removed.iter().map(|action| (action.action.as_str(), args_of(action)["path"].clone(), args_of(action)["edit"].clone())).collect::<Vec<_>>(),
            [(semio_framework::HISTORY_EDIT_INPUT_ACTION_ID, serde_json::json!("/targets/0"), serde_json::json!(semio_framework::HISTORY_EDIT_INPUT_REMOVE))],
            "{locale:?}: a chip drafts the removal of its own item"
        );
    }
    close_retained_body(surface, documents);
    close_history_producer(&mut producer);
}

/// ⚖️ LAW (gap N17 + the stepped document load on wgpu, the REAL producer): a history change replaying is the body's
/// `framework.history.reprojection` section by its `kind` — this replica's own history step ("History step", "Replaying
/// history: 12 of 400 mutations") and a whole-document load ("Document load", "Loading document: 12 of 400") — with an
/// enabled Cancel replay that sends `historyEditCancelReplay` without a session generation (the runtime drops the step or
/// the load with zero trace), and a refused step reads "History step refused: <reason>" with no control, in English and
/// German.
#[test]
fn a_replaying_history_step_or_document_load_shows_its_progress_and_cancels_in_the_mirror() {
    let mut producer = None;
    use semio_framework::kernel::HistoryReprojectionKind as Kind;
    let surface = FRAMEWORK_PANEL_TAB_HISTORY_ID;
    let body = Rect::new(0.0, 0.0, 420.0, 640.0);
    let mut shell = session_shell();
    let mut documents = Vec::new();
    let replaying = |kind: Kind| HistoryReprojection { done: 12, total: 400, processed: None, kind, paused: false, fault: None };
    let cases = [
        (Locale::En, Kind::Step, "History step", "Replaying history: 12 of 400 mutations", "Cancel replay"),
        (Locale::De, Kind::Step, "Verlaufsschritt", "Verlauf wird neu angewendet: 12 von 400 Mutationen", "Neuanwendung abbrechen"),
        (Locale::En, Kind::Load, "Document load", "Loading document: 12 of 400", "Cancel replay"),
        (Locale::De, Kind::Load, "Dokument laden", "Dokument wird geladen: 12 von 400", "Neuanwendung abbrechen"),
    ];
    for (generation, (locale, kind, title, progress, cancel)) in (1..).zip(cases) {
        documents.push(history_body(&mut producer, None, None, Some(replaying(kind)), locale, generation));
        let _ = paint_retained_body(&mut shell, surface, documents.last().expect("published"), body);
        let node = |key: &str| history_mirror_node(key).unwrap_or_else(|| panic!("{locale:?} {kind:?}: `{key}` is in the mirror"));
        assert_eq!(node("framework.history.reprojection").label.as_deref(), Some(title), "{locale:?} {kind:?}: the section names what replays");
        assert_eq!(node("framework.history.reprojection.status").label.as_deref(), Some(progress), "{locale:?} {kind:?}: its progress in words");
        let button = node("framework.history.reprojection.cancelReplay");
        assert!(button.role == "button" && button.actionable && !button.disabled && button.label.as_deref() == Some(cancel), "{locale:?} {kind:?}: Cancel replay is an enabled button: {button:?}");
        let sent = activate_history_mirror_node(&mut shell, "framework.history.reprojection.cancelReplay");
        assert_eq!(sent.iter().map(|action| (action.action.as_str(), args_of(action).get("generation").cloned())).collect::<Vec<_>>(), [(semio_framework::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, None)], "{locale:?} {kind:?}: it cancels, addressing no session");
    }
    for (generation, locale, refusal) in [(10, Locale::En, "History step refused: "), (11, Locale::De, "Verlaufsschritt abgelehnt: ")] {
        documents.push(history_body(&mut producer, None, None, Some(HistoryReprojection { done: 0, total: 0, processed: None, kind: Kind::Step, paused: false, fault: Some("timeTravel.blocked".into()) }), locale, generation));
        let _ = paint_retained_body(&mut shell, surface, documents.last().expect("published"), body);
        let status = history_mirror_node("framework.history.reprojection.status").and_then(|node| node.label).unwrap_or_default();
        assert!(status.starts_with(refusal) && status.len() > refusal.len(), "{locale:?}: a refused step names its reason: {status:?}");
        assert!(history_mirror_node("framework.history.reprojection.cancelReplay").is_none(), "{locale:?}: a refused step offers no control");
    }
    close_retained_body(surface, documents);
    close_history_producer(&mut producer);
}

/// ⚖️ LAW (probe readiness, `verify time-travel --renderer wgpu` `scrollHistory`): a panel's retained body registers its whole
/// content as the scroll region `framework.panel.history.scroll` beneath its row hits, owned by the History surface, so a wheel
/// over the gap below the last row scrolls the body (React's overflow container) and `dumpChrome` names the region by kind and
/// owner.
#[test]
fn a_panel_body_is_one_scroll_region_under_its_rows() {
    let mut producer = None;
    let surface = FRAMEWORK_PANEL_TAB_HISTORY_ID;
    let body = Rect::new(0.0, 0.0, 420.0, 640.0);
    let mut shell = session_shell();
    let documents = vec![history_body(&mut producer, None, None, None, Locale::En, 1)];
    let mut input = paint_retained_body(&mut shell, surface, &documents[0], body);
    let region = format!("{surface}.scroll");
    let hits = input.hits().to_vec();
    let index = hits.iter().position(|hit| hit.control_id.as_deref() == Some(region.as_str())).expect("the panel's scroll region is registered");
    assert!(hits[index].kind == HitKind::ScrollRegion && hits[index].rect == body, "the whole content rect: {:?}", hits[index]);
    assert!(hits[index + 1..].iter().any(|hit| hit.kind == HitKind::TreeItem), "its rows register above it");
    assert_eq!(shell.retained_hit_windows.get(&region).map(|(owner, _)| owner.as_str()), Some(surface), "owned by the History surface");
    let gap = (body.x + body.w * 0.5, body.y + body.h - 4.0);
    assert_eq!(input.hit_at(gap.0, gap.1).and_then(|hit| hit.control_id.clone()).as_deref(), Some(region.as_str()), "the gap below the rows is the region");
    assert!(shell.handle_pointer_wheel(gap.0, gap.1, 0.0, 40.0, &mut input), "a wheel over the gap scrolls the body");
    close_retained_body(surface, documents);
    close_history_producer(&mut producer);
}
//#endregion 📚️HistoryBody

//#region 📡️HistoryReprojectionStatus
/// 🖌️ Walks the reprojection band's retained step to completion; answers the opportunities it took and the hits it staged.
fn paint_reprojection_band(shell: &mut ShellState) -> (usize, Vec<HitTarget<ActionDescriptor>>) {
    let mut cursor = ShellChromeChildCursor::default();
    let (mut overlay, mut atlas, mut input, theme) = (DrawList::default(), FontAtlas::builtin(), InputState::<ActionDescriptor>::default(), Theme::light());
    for step in 0..100_000 {
        if shell.render_history_reprojection_band_step(&mut cursor, &mut overlay, &mut atlas, &mut input, &theme) {
            return (step, input.staged_hits().to_vec());
        }
    }
    panic!("the reprojection band step never completed");
}

/// ⚖️ LAW (audit W1E-3 on wgpu, the kernel's one status copy `🎠️kernel/🧫️fixtures/🧫️history-reprojection`, React's
/// `[data-semio-history-reprojection]`): every fixture case is announced OUTSIDE the History panel by the chrome node
/// `shell.history.reprojection`, named `<title>: <text>` in English and German — a progress bar over done/total while it replays,
/// a polite status while paused or refused, never announcing a refusal's raw code — and painted as one bottom band whose lines
/// read that message with its replay track; with no session open it offers Cancel replay while it replays and Replay again while
/// a remote change is paused (no generation), nothing for a refusal; stacked above an open session's band, which then owns the
/// controls; without a reprojection nothing is announced or painted.
#[test]
fn every_history_reprojection_is_announced_outside_the_history_panel() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🎠️kernel/🧫️fixtures/🧫️history-reprojection/🔣️.json")).expect("the shared history-reprojection fixture parses");
    let theme = Theme::light();
    let mut shell = session_shell();
    assert!(shell.history_reprojection_accessibility_node(1).is_none() && paint_reprojection_band(&mut shell).0 == 0, "nothing replays, nothing is announced or painted");
    let cases = fixture["cases"].as_array().expect("cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let reprojection: HistoryReprojection = serde_json::from_value(case["reprojection"].clone()).unwrap_or_else(|error| panic!("{name}: a kernel HistoryReprojection: {error}"));
        let paused = case["paused"].as_bool().expect("paused");
        let running = !paused && reprojection.total > 0;
        shell.observe_history_reprojection(Some(&reprojection));
        for locale in ["en", "de"] {
            shell.locale_id = locale.into();
            let message = format!("{}: {}", case["title"][locale].as_str().expect("title"), case["text"][locale].as_str().expect("text"));
            let (steps, hits) = paint_reprojection_band(&mut shell);
            let nodes = shell.chrome_accessibility_nodes(&hits);
            let node = nodes.iter().find(|node| node.key == HISTORY_REPROJECTION_STATUS_ID).unwrap_or_else(|| panic!("{name} {locale}: announced"));
            assert_eq!(node.label.as_deref(), Some(message.as_str()), "{name} {locale}: named by the kernel's title and status line");
            assert_eq!(node.live, ui_contract::liveness_name(ui_contract::Liveness::Polite), "{name} {locale}: politely");
            assert!(node.description.is_none() && case["fault"].as_str().map_or(true, |code| !message.contains(code)), "{name} {locale}: a refusal's raw code is never announced: {node:?}");
            assert_eq!((node.role.as_str(), node.busy, node.value_now), ("status", running, None), "{name} {locale}: one stable status, busy while it replays");
            let progress = nodes.iter().find(|node| node.key == HISTORY_REPROJECTION_PROGRESS_ID);
            assert_eq!(progress.is_some(), running, "{name} {locale}: a progress bar only while it replays");
            if let Some(progress) = progress {
                assert_eq!((progress.role.as_str(), progress.value_min, progress.value_now, progress.value_max), ("progressbar", Some(0.0), Some(f64::from(reprojection.done)), Some(f64::from(reprojection.total))), "{name} {locale}");
                assert_eq!((progress.label.as_deref(), progress.value_text.as_deref()), (case["title"][locale].as_str(), case["text"][locale].as_str()), "{name} {locale}: named by the title, valued by the status line");
            }
            let (_, plan) = shell.history_reprojection_band_plan_for(&theme).expect("a band");
            assert_eq!((plan.lines.iter().map(|line| line.text.as_str()).collect::<Vec<_>>().join(" "), plan.progress.is_some()), (message.clone(), running), "{name} {locale}: its lines read the message, a track while it replays");
            assert!(steps > CHROME_BAND_FRAME_STEPS, "{name} {locale}: the band paints its frame and its lines");
            let expected = match (paused, running) {
                (true, _) => Some(("shell.history.reprojection.rerun", semio_framework::HISTORY_EDIT_RERUN_ACTION_ID)),
                (false, true) => Some(("shell.history.reprojection.cancel-replay", semio_framework::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID)),
                (false, false) => None,
            };
            let controls: Vec<(String, String, Option<Value>)> = hits.iter().map(|hit| (hit.control_id.clone().unwrap_or_default(), hit.event.as_ref().map(|event| event.action.clone()).unwrap_or_default(), hit.event.as_ref().and_then(|event| event.args.as_ref()).map(dsl_value_as_json))).collect();
            assert_eq!(controls, expected.map(|(control, action)| (control.to_string(), action.to_string(), None)).into_iter().collect::<Vec<_>>(), "{name} {locale}: the control a person can use, addressing no session");
            if let Some((control, _)) = expected {
                let button = nodes.iter().find(|node| node.key == control).unwrap_or_else(|| panic!("{name} {locale}: {control} is announced"));
                assert!(button.role == "button" && button.actionable && !button.disabled, "{name} {locale}: an enabled button: {button:?}");
            }
        }
    }
    shell.locale_id = "en".into();
    shell.observe_history_reprojection(Some(&HistoryReprojection { done: 3, total: 12, processed: None, kind: semio_framework::kernel::HistoryReprojectionKind::Step, paused: false, fault: None }));
    let editing = session("editing a mutation");
    shell.observe_history_time_travel(Some(&editing));
    let session_band = shell.time_travel_band_plan_for(&editing, &theme).band;
    let (_, plan) = shell.history_reprojection_band_plan_for(&theme).expect("still announced");
    assert!(plan.band.y + plan.band.h <= session_band.y && plan.control.is_none(), "stacked above the session band, which owns the controls: {plan:?} over {session_band:?}");
    assert!(paint_reprojection_band(&mut shell).1.is_empty(), "no control while a session is open");
    assert!(shell.chrome_accessibility_nodes(&[]).iter().any(|node| node.key == HISTORY_REPROJECTION_STATUS_ID) && shell.chrome_accessibility_nodes(&[]).iter().any(|node| node.key == TIME_TRAVEL_BAND_STATUS_ID), "both live nodes while a session is open");
    shell.observe_history_reprojection(None);
    assert!(shell.history_reprojection_accessibility_node(1).is_none() && paint_reprojection_band(&mut shell).0 == 0, "adopted: nothing announced or painted any more");
}
//#endregion 📡️HistoryReprojectionStatus

//#region 🫥️HostWindowBlur
/// ⚖️ LAW (React's pane `onBlur` when the page loses focus; S3-SPATIAL N9): a host window blur — native `Focused(false)`, the
/// browser's `host-window-blur` door — arms exactly one `hostEvent{windowId: <active pane>, kind: blur}` on the session's
/// controller at the shell's next drain, keeps the settle pump owed until then, and nothing more after it.
#[test]
fn a_host_window_blur_blurs_the_active_pane_for_its_program_once() {
    let mut shell = session_shell();
    assert!(!shell.arm_host_window_blur(), "no blur, nothing armed");
    note_host_window_blur();
    assert!(shell.settle_pump_pending(), "a pending blur is work the settle pump owes");
    assert!(shell.arm_host_window_blur(), "the drain takes the blur");
    let controller_id = shell.shell_command_controller_id().expect("the session's controller");
    let armed: Vec<(String, String, Value)> = shell.deferred_actions.iter().filter(|action| action.action == semio_framework::HOST_EVENT_ACTION_ID).map(|action| (action.controller_id.clone(), action.action.clone(), args_of(action))).collect();
    assert_eq!(armed, [(controller_id, semio_framework::HOST_EVENT_ACTION_ID.to_string(), serde_json::json!({ "windowId": "main", "kind": semio_framework::HOST_EVENT_KIND_BLUR }))], "one blur for the active pane");
    assert!(!shell.arm_host_window_blur(), "a blur is taken once");
}
//#endregion 🫥️HostWindowBlur

//#region 📥️ImportTransfer
/// 🧾️ A shell whose session app declares `importAbort`, holding one picked import of two files (`a.obj` of two chunks'
/// worth of payload, `b.obj` of one).
fn importing_shell() -> (ShellState, u64) {
    let mut shell = session_shell();
    shell.session.as_mut().expect("session").app.actions.push(semio_framework::ActionDefinition::new(IMPORT_ABORT_ACTION_ID, LocalizedLabel::native("Abort import", "Import abbrechen"), semio_framework::ActionKind::Mutation, IconName::X));
    let chunk = semio_framework::kernel::import_payload_chunks("x").len();
    assert_eq!(chunk, 1, "a tiny file is one chunk");
    let opened = vec![OpenedFile { name: "a.obj".into(), contents: "a".repeat(400_000) }, OpenedFile { name: "b.obj".into(), contents: "b".into() }];
    assert!(shell.begin_import_transfer(PickedImport { controller_id: "test.controller".into(), import_action: "importFile".into(), args: None, opened, multiple: true }));
    let id = shell.import_transfers.front().expect("the import").id;
    (shell, id)
}

/// ⚖️ LAW (React's `importOpenedFilesV1` + `documentTransferTasksV1` on wgpu): a picked import is ONE running task in the Task
/// Manager — its files, lane and owner, a progress bar of the chunks delivered and "Cancel <files>" (en/de) — and keeps the
/// settle pump owed; a Cancel before any chunk reached the guest drops it with the `import-cancelled` notice and no
/// `importAbort`, a Cancel after one arms the app's `importAbort {}` exactly once, and "No task is running." returns.
#[test]
fn a_picked_import_is_a_cancellable_task_that_frees_a_started_import() {
    let (mut shell, id) = importing_shell();
    let transfer = shell.import_transfers.front().expect("the import").clone();
    assert!(transfer.total > 2 && transfer.delivered == 0 && transfer.file == "a.obj, b.obj", "every file's chunks wait in order: {transfer:?}");
    assert!(shell.settle_pump_pending(), "a waiting import is work the settle pump owes");
    let body = serde_json::to_value(shell.build_task_manager_ui()).expect("the Task Manager body serializes");
    let text = body.to_string();
    for expected in ["Running tasks", "a.obj, b.obj · Document import · test · Running", &format!("os.task-manager.progress.documentTransfer:{id}"), &format!("os.task-manager.cancel.documentTransfer:{id}"), "Cancel a.obj, b.obj", "cancelImportTransfer"] {
        assert!(text.contains(expected), "the Task Manager shows {expected:?}: {text}");
    }
    shell.locale_id = "de".into();
    let german = serde_json::to_value(shell.build_task_manager_ui()).expect("serializes").to_string();
    assert!(german.contains("Laufende Aufgaben") && german.contains("a.obj, b.obj abbrechen") && german.contains("Dokumentimport"), "{german}");
    shell.locale_id = "en".into();

    assert!(shell.cancel_import_transfer(id));
    assert!(shell.import_transfers.is_empty() && !shell.deferred_actions.iter().any(|action| action.action == IMPORT_ABORT_ACTION_ID), "nothing reached the guest, nothing to free");
    let notice = shell.transient_notice().expect("told");
    assert_eq!((notice.message.as_str(), notice.code.as_deref()), ("Import of “a.obj” cancelled.", Some("shell.documentTransfer.import-cancelled")));
    assert!(serde_json::to_value(shell.build_task_manager_ui()).expect("serializes").to_string().contains("No task is running."));

    let (mut shell, id) = importing_shell();
    shell.import_transfers.front_mut().expect("the import").delivered = 1;
    assert!(shell.cancel_import_transfer(id));
    let aborts: Vec<(String, Value)> = shell.deferred_actions.iter().filter(|action| action.action == IMPORT_ABORT_ACTION_ID).map(|action| (action.controller_id.clone(), args_of(action))).collect();
    assert_eq!(aborts, [("test.controller".to_string(), serde_json::json!({}))], "a started import is freed once, without a reason");
    assert!(!shell.cancel_import_transfer(id), "a cancelled import is gone");
}

/// ⚖️ LAW (`RequestMediaFrames` host cancel): the frames one decoded video yields — each `frameAction`, then `doneAction` —
/// are ONE running "Video frames" task in the Task Manager (en/de) that keeps the settle pump owed and dispatches in order; a
/// Cancel after a frame reached the guest drops the rest (the `doneAction` never fires) and arms the app's `importAbort {}`
/// once, telling the person by the video's name; an empty answer (no pick) starts nothing.
#[test]
fn a_decoded_video_is_a_cancellable_task_that_frees_a_started_stream() {
    let (mut shell, _) = importing_shell();
    shell.import_transfers.clear();
    assert!(!shell.begin_media_frames_transfer("test.controller".into(), Vec::new()), "no pick, no task");
    let frame = |action: &str, index: usize| ActionDescriptor { controller_id: "test.controller".into(), action: action.into(), args: crate::action_args_json!({ "name": "clip.mp4", "index": index }) };
    assert!(shell.begin_media_frames_transfer("test.controller".into(), vec![frame("importVideoFramePayload", 0), frame("importVideoFramePayload", 1), frame("importVideoDone", 2)]));
    let transfer = shell.import_transfers.front().expect("the stream").clone();
    assert_eq!((transfer.lane, transfer.file.as_str(), transfer.total, transfer.chunks.back().map(|last| last.action.as_str())), (TransferLane::VideoFrames, "clip.mp4", 3, Some("importVideoDone")), "every frame, then done, waits in order: {transfer:?}");
    assert!(shell.settle_pump_pending(), "a waiting stream is work the settle pump owes");
    let text = serde_json::to_value(shell.build_task_manager_ui()).expect("serializes").to_string();
    for expected in ["clip.mp4 · Video frames · test · Running", &format!("os.task-manager.cancel.documentTransfer:{}", transfer.id), "Cancel clip.mp4"] {
        assert!(text.contains(expected), "the Task Manager shows {expected:?}: {text}");
    }
    shell.locale_id = "de".into();
    assert!(serde_json::to_value(shell.build_task_manager_ui()).expect("serializes").to_string().contains("clip.mp4 · Videobilder · test · Läuft"));
    shell.locale_id = "en".into();
    shell.import_transfers.front_mut().expect("the stream").delivered = 1;
    assert!(shell.cancel_import_transfer(transfer.id));
    assert!(shell.import_transfers.is_empty() && !shell.deferred_actions.iter().any(|action| action.action == "importVideoDone"), "the rest of the stream is dropped");
    assert_eq!(shell.deferred_actions.iter().filter(|action| action.action == IMPORT_ABORT_ACTION_ID).count(), 1, "a started stream is freed once");
    assert_eq!(shell.transient_notice().map(|notice| notice.message.clone()).as_deref(), Some("Import of “clip.mp4” cancelled."));
}
//#endregion 📥️ImportTransfer

//#region 🍔️ContextMenuReason
/// ⚖️ LAW (F7 on wgpu — React's menu focusable-when-disabled, the `RowAction::disabled_because` contract): a disabled
/// context-menu row carries its producer reason from the spec, stays reachable by the arrow keys (only separators are
/// skipped; digit ordinals still count enabled rows), paints its reason beside its label, and is projected as a focusable
/// `menuitem` that is `disabled`, not actionable and described by its reason; Enter and its activation fire nothing and keep
/// the menu open, while an enabled row stays actionable.
#[test]
fn a_disabled_context_menu_row_is_reachable_and_tells_its_reason() {
    let paste = shell_context_menu_item_from_spec(ui_wgpu::wgpu::ContextMenuItemSpec { id: "menu.paste".into(), label: Some("Paste".into()), action: Some("paste".into()), ..Default::default() }.disabled_because("The clipboard is empty".into()), "ctrl", false);
    assert_eq!((paste.disabled, paste.reason.as_deref(), paste.painted_label()), (true, Some("The clipboard is empty"), "Paste \u{b7} The clipboard is empty".to_string()));
    let action = |verb: &str| Some(ActionDescriptor { controller_id: "ctrl".into(), action: verb.into(), args: None });
    let items = vec![
        ContextMenuItem { id: "menu.copy".into(), label: "Copy".into(), action: action("copy"), ..Default::default() },
        ContextMenuItem { id: "menu.rule".into(), separator: true, ..Default::default() },
        paste,
        ContextMenuItem { id: "menu.cut".into(), label: "Cut".into(), action: action("cut"), ..Default::default() },
    ];
    assert_eq!(context_menu_move_active(&items, &[0], true), vec![2], "the arrow skips the separator and reaches the disabled row");
    assert_eq!(context_menu_move_active(&items, &[2], true), vec![3]);
    assert_eq!(context_menu_path_for_ordinal(&items, &[], 2), Some(vec![3]), "ordinals count enabled rows only");

    let mut shell = session_shell();
    shell.context_menu = Some(ContextMenuState { items: items.clone(), active: vec![2], ..Default::default() });
    assert!(matches!(shell.context_menu_handle_key(ui_wgpu::wgpu::KeyAction::Enter), ContextMenuKeyOutcome::Ignored), "Enter on a disabled row fires nothing");
    assert!(shell.context_menu.is_some(), "and keeps the menu open");

    let menu = shell.context_menu.clone().expect("open");
    let (mut draw, mut atlas, icons, theme) = (DrawList::default(), FontAtlas::builtin(), IconAtlas::default(), Theme::default());
    let mut input = InputState::<ActionDescriptor>::default();
    ShellState::render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, 800.0, 600.0);
    let hits = input.staged_hits().to_vec();
    let paste_hit = hits.iter().find(|hit| hit.control_id.as_deref() == Some("menu.paste")).expect("the disabled row is a hit target");
    assert!(paste_hit.event.is_none(), "that fires nothing");
    let nodes = shell.chrome_accessibility_nodes(&hits);
    let node = |key: &str| nodes.iter().find(|node| node.key == key).unwrap_or_else(|| panic!("`{key}` is projected"));
    let disabled = node("menu.paste");
    assert_eq!((disabled.role.as_str(), disabled.label.as_deref(), disabled.description.as_deref(), disabled.disabled, disabled.actionable, disabled.focusable), ("menuitem", Some("Paste"), Some("The clipboard is empty"), true, false, true));
    let enabled = node("menu.copy");
    assert_eq!((enabled.label.as_deref(), enabled.description.as_deref(), enabled.disabled, enabled.actionable), (Some("Copy"), None, false, true));

    let consumed = semio_framework_async::block_on(shell.handle_shell_hit(paste_hit, &InputState::<ActionDescriptor>::default())).expect("activating a disabled row never errors");
    assert!(consumed && shell.context_menu.is_some() && shell.deferred_actions.iter().all(|action| action.action != "paste"), "its activation fires nothing and keeps the menu open");
}
//#endregion 🍔️ContextMenuReason
