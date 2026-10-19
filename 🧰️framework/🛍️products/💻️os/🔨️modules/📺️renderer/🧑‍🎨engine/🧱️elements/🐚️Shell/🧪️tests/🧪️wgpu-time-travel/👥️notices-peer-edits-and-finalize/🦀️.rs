/// 👥️ Notices, peer edits and finalize: transient notice projection, the shared peer edit corpus, the roster badge and the keyboard-only finalize prompt.

//#region 🧯️NoticeProjection
/// ⚖️ LAW (React's `[data-semio-transient-notice]`, fixture `🧫️wgpu-transient-notice`): every showing transient notice
/// is projected as the one polite status node `shell.notice`, named by its localized message and described by its code —
/// beside the chrome, and over an open modal too — and nothing once dismissed or past its deadline.
#[test]
fn every_transient_notice_is_a_polite_status_named_by_its_message_and_described_by_its_code() {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🧯️wgpu-transient-notice/🔣️.json")).expect("the transient notice fixture parses");
    let expected = &fixture["node"];
    let notice_node = |shell: &ShellState| PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&[]).into_iter().filter(|node| node.key == expected["key"].as_str().expect("key")).collect::<Vec<_>>();
    for notice in fixture["notices"].as_array().expect("notices") {
        let mut shell = session_shell();
        shell.prefs.locale_id = notice["locale"].as_str().expect("locale").into();
        let severity = match notice["severity"].as_str().expect("severity") {
            "info" => semio_framework::Severity::Info,
            "warning" => semio_framework::Severity::Warning,
            _ => semio_framework::Severity::Error,
        };
        let message = notice["message"].as_str().expect("message");
        assert!(notice_node(&shell).is_empty(), "no notice, no node");
        shell.chrome.show_transient_notice(message, severity, notice["code"].as_str());
        let nodes = notice_node(&shell);
        assert_eq!(nodes.len(), 1, "{message}: one status node");
        let node = &nodes[0];
        assert_eq!((node.role.as_str(), node.live.as_str(), node.focusable, node.actionable), (expected["role"].as_str().expect("role"), expected["live"].as_str().expect("live"), false, false), "{message}");
        assert_eq!((node.label.as_deref(), node.description.as_deref()), (Some(message), notice["code"].as_str()), "{message}: named by the message, described by the code");
        shell.docs.session.as_mut().expect("session").app.dialogs.push(finalize_dialog());
        SessionCtx::of(&mut shell).observe_history_time_travel(Some(&session("the finalize prompt can go back")));
        open_finalize_prompt(&mut shell, "Alternative");
        assert_eq!(notice_node(&shell).len(), 1, "{message}: still told while a modal dialog is open");
        shell.chrome.chrome_build.transient_notice.as_mut().expect("showing").shown_at_ms -= 5_000.0;
        assert!(notice_node(&shell).is_empty(), "{message}: nothing past the 4 s deadline");
    }
}
//#endregion 🧯️NoticeProjection

//#region 👥️PeerHistoryEdits
fn peers_corpus() -> Value {
    serde_json::from_str(include_str!("../../../../🛠️ShellHelpers/🧪️tests/🧫️fixtures/🧫️time-travel-peers/🔣️.json")).expect("the shared time-travel peers corpus parses")
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
    shell.history.history_entries = peer_rows(&corpus);
    shell.directory.presence_surface = Some("s".into());
    shell.directory.presence_peers = case["peers"].as_array().expect("peers").iter().map(|peer| corpus_peer(peer, "s")).collect();
    shell.directory.presence_peers.push(corpus_peer(&serde_json::json!({ "actor": "actor-far", "label": "Far", "historyEdit": { "mutationId": "m-drag", "stage": "editing", "drafts": 1 } }), "elsewhere"));
    shell.prefs.locale_id = "de".into();
    let rows = PanelsViewCtx::of(&mut shell).footer_presence_rows();
    assert_eq!(rows.iter().map(|row| (row.actor.as_str(), row.activity.as_ref().map(|activity| activity.badge.as_str()))).collect::<Vec<_>>(), [("actor-ada", Some("⏪")), ("actor-cy", None), ("actor-bo", Some("⏪"))]);
    assert_eq!(semio_framework_ui_wgpu_light::presence_bar::presence_bar_chip_text(&rows, None, Locale::De), "Ada ⏪ · Cy · Bo ⏪");
    let announced = PanelsViewCtx::of(&mut shell).footer_status_chips().into_iter().find(|(key, _)| *key == "s-presence-peers").expect("the roster is announced").1;
    assert_eq!(announced, "Ada (Ada bearbeitet Skalieren im Verlauf) · Cy · Bo (Bo bearbeitet Drehen im Verlauf)");
    let expected: Vec<(String, String)> = case["expect"]["de"]["notes"].as_array().expect("notes").iter().map(|note| (note["key"].as_str().expect("key").into(), note["text"].as_str().expect("text").into())).collect();
    assert_eq!(shell.directory.peer_time_travel_presence(&shell.chrome, &shell.history, &shell.prefs).notes, expected, "a peer on another surface notes nothing");
}
//#endregion 👥️PeerHistoryEdits

//#region 🗨️FinalizePrompt
fn finalize_dialog() -> DialogDefinition {
    let fixture: Value = serde_json::from_str(include_str!("../../../../../../../../../../🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🧫️dialog-choices/🔣️.json")).expect("dialog-choices fixture");
    serde_json::from_value(fixture["dialog"].clone()).expect("the finalize prompt")
}

/// 🗨️ Opens the framework-injected finalize prompt exactly as the runtime does: `Effect::OpenDialog` seeded with the
/// localized default name and the session generation (a context arg no field declares).
fn open_finalize_prompt(shell: &mut ShellState, name: &str) {
    let controller = shell.docs.session.as_ref().map(|session| session.app.controller_id.clone()).expect("a session");
    let generation = shell.history.history_time_travel().map(|status| status.generation).expect("a live session");
    WindowActionsCtx::of(&mut shell).queue_host_effects(&controller, vec![semio_framework::kernel::Effect::OpenDialog { req: semio_framework::kernel::RequestId(1), dialog_id: "finalizeHistoryEdit".into(), args: Some(DslValue::from(serde_json::json!({ "name": name, "generation": generation }))) }]);
    assert!(shell.chrome.chrome_build.dialog_open(), "the prompt opened from the session's dialogs");
}

fn key(shell: &mut ShellState, input: &mut InputState<ActionDescriptor>, action: semio_framework_ui_wgpu_input::KeyAction) {
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_keyboard_async(action, &PointerModifiers::default(), input)).expect("the key routes");
}

/// ⚖️ LAW: keyboard alone finalizes through the real async router — the open prompt owns every key ahead of the shell's
/// own rungs: typing edits the seeded name and Enter submits a new alternative; Tab to the destructive choice and Enter
/// overwrites, needing no name; Escape goes back. Every dispatch carries the session generation; no path discards or
/// exits.
#[test]
fn the_finalize_prompt_is_operated_by_keyboard_alone() {
    let mut shell = session_shell();
    shell.docs.session.as_mut().expect("session").app.dialogs.push(finalize_dialog());
    let choosing = session("the finalize prompt can go back");
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&choosing));
    let mut input = InputState::<ActionDescriptor>::default();

    open_finalize_prompt(&mut shell, "Edited history");
    assert_eq!(shell.chrome.chrome_build.dialog_stack.last().map(|request| request.fields[0].draft.clone()), Some("Edited history".to_string()), "the runtime's default name seeds the field");
    key(&mut shell, &mut input, semio_framework_ui_wgpu_input::KeyAction::Char("!".into()));
    key(&mut shell, &mut input, semio_framework_ui_wgpu_input::KeyAction::Enter);
    assert!(!shell.chrome.chrome_build.dialog_open());
    let submit = shell.settle.deferred_actions.pop().expect("the submit dispatched");
    assert_eq!((submit.controller_id.as_str(), submit.action.as_str()), ("test", "historyEditCommit"));
    assert_eq!(serde_json::to_value(submit.args.expect("args")).expect("wire"), serde_json::json!({ "generation": choosing.generation, "name": "Edited history!" }));

    open_finalize_prompt(&mut shell, "Edited history");
    for _ in 0.."Edited history".len() {
        key(&mut shell, &mut input, semio_framework_ui_wgpu_input::KeyAction::Backspace);
    }
    key(&mut shell, &mut input, semio_framework_ui_wgpu_input::KeyAction::Enter);
    assert!(shell.chrome.chrome_build.dialog_open(), "a cleared name gates the new alternative");
    key(&mut shell, &mut input, semio_framework_ui_wgpu_input::KeyAction::Tab);
    key(&mut shell, &mut input, semio_framework_ui_wgpu_input::KeyAction::Tab);
    key(&mut shell, &mut input, semio_framework_ui_wgpu_input::KeyAction::Enter);
    let overwrite = shell.settle.deferred_actions.pop().expect("the overwrite choice dispatched without a name");
    assert_eq!(serde_json::to_value(overwrite.args.expect("args")).expect("wire"), serde_json::json!({ "generation": choosing.generation, "choice": "overwrite" }));

    open_finalize_prompt(&mut shell, "Edited history");
    key(&mut shell, &mut input, semio_framework_ui_wgpu_input::KeyAction::Escape);
    assert!(!shell.chrome.chrome_build.dialog_open());
    let back = shell.settle.deferred_actions.pop().expect("Escape answered");
    assert_eq!((back.action.as_str(), back.args), ("historyEditBack", None), "Escape goes back to reviewing — it never discards");
    assert!(!shell.settle.deferred_actions.iter().any(|action| matches!(action.action.as_str(), "historyEditDiscard" | "historyEditExit")));
}
//#endregion 🗨️FinalizePrompt
