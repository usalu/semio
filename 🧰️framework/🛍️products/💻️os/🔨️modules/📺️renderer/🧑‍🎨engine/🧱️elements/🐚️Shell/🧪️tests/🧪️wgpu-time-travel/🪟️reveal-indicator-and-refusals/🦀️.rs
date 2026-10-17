/// 🪟️ Reveal, indicator and refusals: History tab reveal, phone panel, reply status, operation publications, localized history refusals and lane refusals.

//#region 🪟️RevealAndIndicator
/// ⚖️ LAW: the edge into a session reveals the History tab once, later statuses of the same session leave a closed
/// panel closed, and every pane's indicator is a named note under its own window id that no pane chip claims.
#[test]
fn a_session_reveals_the_history_tab_and_each_pane_wears_an_indicator() {
    let mut shell = session_shell();
    SessionCtx::of(&mut shell).sync_dock_tabs();
    let anchor = shell.dock.dock_tabs.locate(FRAMEWORK_PANEL_TAB_HISTORY_ID).map(|(anchor, _)| anchor).expect("the History tab is docked");
    shell.dock.anchor_state_mut(anchor).visible = false;
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("editing a mutation")));
    assert!(shell.dock.anchor_open(anchor), "the session start opens the History panel");
    assert_eq!(shell.dock.anchor_state(anchor).active_tab(), Some(FRAMEWORK_PANEL_TAB_HISTORY_ID));
    shell.dock.anchor_state_mut(anchor).visible = false;
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("a running replay")));
    assert!(!shell.dock.anchor_open(anchor), "only the edge into a session reveals");

    let indicator = time_travel_indicator_control_id("side");
    assert!(indicator.starts_with("framework.window.") && indicator.ends_with(".timeTravel.indicator"));
    assert!(shell.dock.window_pane_chip_target(&indicator, &shell.docs).is_none(), "no pane chip claims the indicator");
    assert_eq!((time_travel_indicator_caption(Locale::En), time_travel_indicator_caption(Locale::De)), ("History editing", "Verlaufsbearbeitung"));
}

/// ⚖️ LAW (phone width): below the mobile breakpoint, where no anchor paints, the edge into a session opens the one
/// mobile panel on the History tab that holds the editor; on a desktop the mobile panel stays shut.
#[test]
fn on_a_phone_the_session_start_opens_the_mobile_panel_on_history() {
    let mut shell = session_shell();
    SessionCtx::of(&mut shell).sync_dock_tabs();
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("editing a mutation")));
    assert!(!shell.dock.mobile_panel_visible, "a desktop reveals the anchor, not the mobile panel");
    let mut phone = session_shell();
    phone.chrome.screen_w = 375.0;
    phone.chrome.screen_h = 812.0;
    SessionCtx::of(&mut phone).sync_dock_tabs();
    assert!(phone.chrome.mobile_panel_active() && !phone.dock.mobile_panel_visible);
    SessionCtx::of(&mut phone).observe_history_time_travel(Some(&session("editing a mutation")));
    assert!(phone.dock.mobile_panel_visible, "the mobile panel opens");
    assert_eq!(phone.dock.mobile_panel_path.last().map(String::as_str), Some(FRAMEWORK_PANEL_TAB_HISTORY_ID), "on the History tab");
}

/// ⚖️ LAW: every non-stale dispatch reply carries the session status (absent = closed); a stale reply changes nothing;
/// rows fold by their edit so a re-emitted edit replaces its row.
#[test]
fn replies_carry_the_status_and_rows_fold_by_their_edit() {
    let mut shell = session_shell();
    let open = HistoryPatch { cursor: 5, time_travel: Some(session("editing a mutation")), ..Default::default() };
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).observe_invocation_history(Some(&open)));
    assert_eq!(shell.history.history_time_travel().map(|status| status.stage), Some(HistoryTimeTravelStage::Editing));
    let stale = HistoryPatch { cursor: 4, ..Default::default() };
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).observe_invocation_history(Some(&stale)));
    assert!(shell.history.history_time_travel().is_some(), "a stale reply is ignored");
    let closed = HistoryPatch { cursor: 6, ..Default::default() };
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).observe_invocation_history(Some(&closed)));
    assert!(shell.history.history_time_travel().is_none(), "a reply without a status closes the band");

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
    let mut shell = host_test_shell();
    shell.plugins.plugins.iter_mut().find(|program| program.plugin_id == "space").expect("host fixture guest program").install_fixture_publications(queued_publications);
    PUBLISHED.with(|queue| queue.borrow_mut().clear());
    shell.settle.owed_refresh_scope = UiDirtyScope::None;
    shell
}

/// ⚖️ LAW (React's `subscribeOperationProgress` lane): the progress patches the guest pushed on uncorrelated
/// `Invocation` frames move the band between two dispatches, oldest first through the reply's stale guard — a patch
/// older than the one already folded rolls nothing back, an empty take is no change, and progress that carries a patch
/// alone re-renders nothing.
#[test]
fn unsolicited_progress_patches_move_the_band_between_dispatches() {
    let mut shell = publication_shell();
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).observe_invocation_history(Some(&HistoryPatch { cursor: 5, time_travel: Some(session("editing a mutation")), ..Default::default() })));
    assert!(!semio_framework_async::block_on(ActionsCtx::of(&mut shell).drain_operation_publications()), "nothing queued is no change");
    let replaying = session("a running replay");
    PUBLISHED.with(|queue| queue.borrow_mut().extend([progress_patch(HistoryPatch { cursor: 5, time_travel: Some(replaying.clone()), ..Default::default() }), progress_patch(HistoryPatch { cursor: 4, ..Default::default() })]));
    assert!(semio_framework_async::block_on(ActionsCtx::of(&mut shell).drain_operation_publications()), "the queued progress moved the band");
    assert_eq!(shell.history.history_time_travel(), Some(&replaying), "the stale patch after it closed nothing");
    assert!(shell.settle.owed_refresh_scope.asks_for_nothing(), "a patch without a scope on a progress frame re-renders nothing");
    assert!(PUBLISHED.with(|queue| queue.borrow().is_empty()), "the drain took the queue");
    assert!(!semio_framework_async::block_on(ActionsCtx::of(&mut shell).drain_operation_publications()));
}

/// ⚖️ LAW (live fault F11; React's `subscribeOperationCompletions` pass): a typed operation that ends AFTER the host
/// call that started it — an example load of a few hundred steps, the replay behind an accepted history edit — reaches
/// the shell on the publication lane alone. With no further dispatch its progress scope and its completion are owed to
/// the settle lane as ONE refresh (the board's body and the History body whose row the completion minted), the row
/// itself is folded into the projection, and the settle pump has work.
#[test]
fn a_typed_operation_that_ends_after_its_host_call_refreshes_and_publishes_its_rows() {
    let mut shell = publication_shell();
    let before = history_rows_oldest_first(&shell.history.history_entries).len();
    let board = UiDirtyScope::Partial { window_bodies: vec!["board".into()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false };
    let row = HistoryEntry { seq: 41, action_id: "setActiveExample".into(), label: LocalizedLabel::native("Load example", "Beispiel laden"), kind: "command".into(), applied: true, ..Default::default() };
    PUBLISHED.with(|queue| {
        queue.borrow_mut().extend([
            OperationPublication { completed: false, ui_scope: board.clone(), history_patch: None, fault: None, resync: false },
            OperationPublication { completed: true, ui_scope: UiDirtyScope::None, history_patch: Some(HistoryPatch { cursor: 41, upserts: vec![row], ..Default::default() }), fault: None, resync: false },
        ])
    });
    assert!(semio_framework_async::block_on(ActionsCtx::of(&mut shell).drain_operation_publications()), "the lane alone changed the shell");
    let rows = history_rows_oldest_first(&shell.history.history_entries);
    assert_eq!(rows.len(), before + 1, "the completion's row is in the projection");
    assert!(rows.iter().any(|entry| entry.key() == "seq:41"));
    assert!(shell.settle.owed_refresh_scope.wants_window_body("board"), "the progress scope is owed: {:?}", shell.settle.owed_refresh_scope);
    assert!(shell.settle.owed_refresh_scope.wants_panel_body(semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY), "the History body owes the minted row: {:?}", shell.settle.owed_refresh_scope);
    assert!(!shell.settle.owed_refresh_scope.wants_window_body("another"), "one merged pass, never a full one");
    assert!(PointerCtx::of(&mut shell).settle_pump_pending(), "the settle lane refreshes without a dispatch");
    assert!(shell.chrome.transient_notice().is_none(), "a completion is told by its rows, not by a notice");
}

/// ⚖️ LAW (live fault F11): a typed-operation drain that stopped is never silence — its fault reaches the person
/// through the dispatch-fault funnel as an error notice, and an answer of the bridge this host cannot read is told the
/// same way and re-reads everything it may have hidden.
#[test]
fn a_refused_completion_is_told_as_a_notice() {
    let mut shell = publication_shell();
    PUBLISHED.with(|queue| queue.borrow_mut().push(OperationPublication { completed: false, ui_scope: UiDirtyScope::None, history_patch: None, fault: Some("browser-actor-publication: noncanonical bytes".into()), resync: false }));
    assert!(semio_framework_async::block_on(ActionsCtx::of(&mut shell).drain_operation_publications()), "a told fault repaints");
    let notice = shell.chrome.transient_notice().expect("the refused completion is told");
    assert!(notice.message.contains("noncanonical bytes"), "the notice names what was refused: {}", notice.message);
    assert!(shell.settle.owed_refresh_scope.asks_for_nothing(), "a fault alone re-renders nothing");
    let unreadable = semio_framework_os_renderer_wgpu_program_bridge::operation_publications_from_json("{\"completed\":true}");
    assert_eq!(unreadable.len(), 1);
    assert!(unreadable[0].fault.as_deref().is_some_and(|fault| fault.starts_with("wgpu-bridge.operation-publication.unreadable")) && unreadable[0].resync, "an unreadable answer is a fault and a re-read: {unreadable:?}");
    let mangled = semio_framework_os_renderer_wgpu_program_bridge::operation_publications_from_json("[{\"completed\":true,\"historyPatch\":{\"cursor\":\"seven\"}}]");
    assert!(mangled[0].fault.as_deref().is_some_and(|fault| fault.contains("historyPatch")) && mangled[0].resync, "an unreadable entry names its member: {mangled:?}");
    assert_eq!(operation_publication_refresh(&mangled[0]), UiDirtyScope::Full);
}

/// ⚖️ LAW (shared corpus `🧫️operation-publication`, the bridge's own producer reads the same rows): every publication
/// the browser bridge hands over as JSON is read back exactly, owes exactly the refresh its row names, and carries a
/// fault exactly when the row says it is told.
#[test]
fn every_operation_publication_owes_what_the_shared_corpus_says() {
    let corpus: Value = serde_json::from_str(include_str!("../../../../🛠️ShellHelpers/🧪️tests/🧫️fixtures/🧫️operation-publication/🔣️.json")).expect("the shared operation-publication corpus parses");
    assert_eq!(corpus["historyBodyKey"].as_str(), Some(semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY));
    let rows = corpus["rows"].as_array().expect("the corpus rows");
    assert!(rows.len() >= 7, "the corpus covers completion, progress, fault and resync");
    for row in rows {
        let name = row["name"].as_str().expect("name");
        let read = semio_framework_os_renderer_wgpu_program_bridge::operation_publications_from_json(&Value::Array(vec![row["publication"].clone()]).to_string());
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
    shell.prefs.locale_id = "de".into();
    shell.prefs.note_dispatch_fault("history.unknown-target: edit-9#0", &mut shell.chrome);
    let notice = shell.chrome.transient_notice().expect("a banner is showing");
    assert_eq!((notice.severity, notice.code.as_deref()), (semio_framework::Severity::Error, Some("history.unknown-target")));
    assert_eq!(notice.message, "Verlaufsbearbeitung abgelehnt: Die bearbeitete Mutation existiert nicht mehr.");
}

/// ⚖️ LAW (shared with React's `commandRejectionNoticeV1`): every row of the command-rejection notice corpus — the one
/// typed `CommandRejectionV1` every producer answers — is told in both locales exactly as the corpus says, with its
/// notice code and severity, from the rejection's `code` and its messages' codes alone; the ten closed codes are all
/// covered.
#[test]
fn every_command_rejection_is_told_from_its_codes_in_both_locales() {
    let corpus: Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🧫️command-rejection/🔣️.json")).expect("the shared command-rejection corpus parses");
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
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🧫️history-notices/🔣️.json")).expect("the kernel's history-notices fixture parses");
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
            shell.prefs.locale_id = locale_id.into();
            shell.chrome.chrome_build.refused_guest_fault = Some(refused.clone());
            shell.prefs.note_dispatch_fault(&format!("{code}: refused"), &mut shell.chrome);
            let told: Vec<(Option<String>, Option<String>)> = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&[]).into_iter().filter(|node| node.key == TRANSIENT_NOTICE_STATUS_ID).map(|node| (node.label, node.description)).collect();
            assert_eq!(told, [(Some(text.clone()), Some(code.to_string()))], "{code} ({tongue}): the mirror tells the notice by its message and its code");
        }
    }
    assert_eq!(classify_dispatch_fault_notice("history.full: refused at 64", None, Terminology::Native, Locale::En).2, None, "no count is ever read from a fault message");
}
//#endregion 🛑️HistoryRefusals
