/// ⏪️ Band law and chords: the shared band corpus, the polite announcement, control captions and reserved actions, and the remappable time-travel chords.

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
            if let Some(processed) = status.processed {
            }
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
    assert!(OverlaysViewCtx::of(&mut shell).time_travel_status_accessibility_node(1).is_none(), "no session, nothing announced");
    for (name, progress_expected, busy) in [("editing a mutation", false, false), ("editing prepares a history preview with visible localized progress", true, false), ("a running replay", true, true), ("an error downstream", false, false), ("finalizing offers", false, true)] {
        let status = session(name);
        shell.prefs.locale_id = "de".into();
        SessionCtx::of(&mut shell).observe_history_time_travel(Some(&status));
        let node = OverlaysViewCtx::of(&mut shell).time_travel_status_accessibility_node(1).expect("an open session announces itself");
        assert_eq!((node.role.as_str(), node.busy, node.value_now, node.value_max), ("status", busy, None, None), "{name}: one stable status");
        let lines = time_travel_band_lines(&status, Terminology::Native, Locale::De);
        assert_eq!(node.label, Some(lines.message()), "{name}");
        assert_eq!(node.live, ui_contract::liveness_name(ui_contract::Liveness::Polite));
        let progress = OverlaysViewCtx::of(&mut shell).time_travel_progress_accessibility_node(2);
        assert_eq!(progress.is_some(), progress_expected, "{name}: a progress bar while preparation or replay with a known total runs");
        if let Some(progress) = progress {
            assert_eq!((progress.key.as_str(), progress.role.as_str(), progress.depth, progress.value_min, progress.value_now, progress.value_max), (TIME_TRAVEL_BAND_PROGRESS_ID, "progressbar", 1, Some(0.0), Some(f64::from(status.done.unwrap_or(0))), status.total.map(f64::from)), "{name}");
            assert_eq!((progress.label.clone(), progress.value_text.clone()), (lines.progress.clone(), lines.progress.clone()), "{name}: named and valued by the progress line");
            assert_eq!(progress.live, ui_contract::liveness_name(ui_contract::Liveness::Off), "{name}: the status is the one live region");
        }
        let keys: Vec<String> = OverlaysViewCtx::of(&mut shell).band_accessibility_nodes(1).into_iter().map(|node| node.key).collect();
        assert_eq!(keys, if progress_expected { vec![TIME_TRAVEL_BAND_STATUS_ID.to_string(), TIME_TRAVEL_BAND_PROGRESS_ID.to_string()] } else { vec![TIME_TRAVEL_BAND_STATUS_ID.to_string()] }, "{name}: the progress follows its status");
    }
    SessionCtx::of(&mut shell).observe_history_time_travel(None);
    assert!(shell.history.history_time_travel().is_none(), "an absent status closes the session");
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
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&editing));
    let accept = |shell: &ShellState| shell.prefs.time_travel_band_plan_for(&editing, &Theme::light(), &shell.chrome).buttons.into_iter().find(|button| button.control.verb == TimeTravelVerb::Accept).expect("Accept is offered").label;
    let terse = accept(&shell);
    shell.chrome.chrome_build.driver.label_tier = ui_wgpu::wgpu::UiDriverLabelTier::Beginner;
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
    assert!(shell.docs.time_travel_shortcut_action(TimeTravelVerb::Exit, &shell.chrome, &shell.history).is_none(), "no session, no verb");
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
        SessionCtx::of(&mut shell).observe_history_time_travel(Some(&status));
        let expected = expected.map(|action| ("test".to_string(), action.to_string(), Some(serde_json::json!({ "generation": status.generation }))));
        assert_eq!(shell.docs.time_travel_shortcut_action(verb, &shell.chrome, &shell.history).map(|action| (action.controller_id, action.action, action.args.as_ref().map(dsl_value_as_json))), expected, "{name} / {verb:?}");
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
    let mut shell = host_test_shell();
    shell.plugins.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("host fixture guest program").install_fixture_action(record_action);
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("editing a mutation")));
    let mut input = InputState::<ActionDescriptor>::default();
    let (action, modifiers) = chord("alt+enter");
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(action, &modifiers, &mut input)).expect("the chord routes");
    assert!(dispatched().iter().any(|action| action.contains("historyEditAccept")), "the chord reached the guest: {:?}", dispatched());
    assert_eq!(shell.history.history_time_travel().map(|status| status.stage), Some(HistoryTimeTravelStage::Replaying), "the dispatch reply's patch moved the band");
    let before = dispatched().len();
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(ui_wgpu::wgpu::KeyAction::Escape, &PointerModifiers::default(), &mut input)).expect("Escape routes");
    assert!(!dispatched()[before..].iter().any(|action| action.contains("historyEdit")), "Escape dispatched no history-edit verb: {:?}", &dispatched()[before..]);
    assert_eq!(shell.history.history_time_travel().map(|status| status.stage), Some(HistoryTimeTravelStage::Replaying));
}
//#endregion ⌨️Chords
