/// 📡️ Reprojection, blur, import and context menu: reprojection announcements, host window blur, cancellable import tasks and disabled context menu reasons.

//#region 📡️HistoryReprojectionStatus
/// 🖌️ Walks the reprojection band's retained step to completion; answers the opportunities it took and the hits it staged.
fn paint_reprojection_band(shell: &mut ShellState) -> (usize, Vec<HitTarget<ActionDescriptor>>) {
    let mut cursor = ShellChromeChildCursor::default();
    let (mut overlay, mut atlas, mut input, theme) = (DrawList::default(), FontAtlas::builtin(), InputState::<ActionDescriptor>::default(), Theme::light());
    for step in 0..100_000 {
        if OverlaysViewCtx::of(&mut shell).render_history_reprojection_band_step(&mut cursor, &mut overlay, &mut atlas, &mut input, &theme) {
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
    let fixture: Value = serde_json::from_str(semio_framework_os_renderer_wgpu_test_support::repo_text("🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/🧫️history-reprojection/🔣️.json")).expect("the shared history-reprojection fixture parses");
    let theme = Theme::light();
    let mut shell = session_shell();
    assert!(OverlaysViewCtx::of(&mut shell).history_reprojection_accessibility_node(1).is_none() && paint_reprojection_band(&mut shell).0 == 0, "nothing replays, nothing is announced or painted");
    let cases = fixture["cases"].as_array().expect("cases");
    assert!(!cases.is_empty());
    for case in cases {
        let name = case["name"].as_str().expect("name");
        let reprojection: HistoryReprojection = serde_json::from_value(case["reprojection"].clone()).unwrap_or_else(|error| panic!("{name}: a kernel HistoryReprojection: {error}"));
        let paused = case["paused"].as_bool().expect("paused");
        let running = !paused && reprojection.total > 0;
        shell.history.observe_history_reprojection(Some(&reprojection));
        for locale in ["en", "de"] {
            shell.prefs.locale_id = locale.into();
            let message = format!("{}: {}", case["title"][locale].as_str().expect("title"), case["text"][locale].as_str().expect("text"));
            let (steps, hits) = paint_reprojection_band(&mut shell);
            let nodes = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&hits);
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
            let (_, plan) = shell.prefs.history_reprojection_band_plan_for(&theme, &shell.chrome, &shell.history).expect("a band");
            assert_eq!((plan.lines.iter().map(|line| line.text.as_str()).collect::<Vec<_>>().join(" "), plan.progress.is_some()), (message.clone(), running), "{name} {locale}: its lines read the message, a track while it replays");
            assert!(steps > CHROME_BAND_FRAME_STEPS, "{name} {locale}: the band paints its frame and its lines");
            let expected = match (paused, running) {
                (true, _) => Some(("shell.history.reprojection.rerun", semio_framework::history_edit::HISTORY_EDIT_RERUN_ACTION_ID)),
                (false, true) => Some(("shell.history.reprojection.cancel-replay", semio_framework::history_edit::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID)),
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
    shell.prefs.locale_id = "en".into();
    shell.history.observe_history_reprojection(Some(&HistoryReprojection { done: 3, total: 12, processed: None, kind: semio_framework::kernel::HistoryReprojectionKind::Step, paused: false, fault: None }));
    let editing = session("editing a mutation");
    SessionCtx::of(&mut shell).observe_history_time_travel(Some(&editing));
    let session_band = shell.prefs.time_travel_band_plan_for(&editing, &theme, &shell.chrome).band;
    let (_, plan) = shell.prefs.history_reprojection_band_plan_for(&theme, &shell.chrome, &shell.history).expect("still announced");
    assert!(plan.band.y + plan.band.h <= session_band.y && plan.control.is_none(), "stacked above the session band, which owns the controls: {plan:?} over {session_band:?}");
    assert!(paint_reprojection_band(&mut shell).1.is_empty(), "no control while a session is open");
    assert!(PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&[]).iter().any(|node| node.key == HISTORY_REPROJECTION_STATUS_ID) && PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&[]).iter().any(|node| node.key == TIME_TRAVEL_BAND_STATUS_ID), "both live nodes while a session is open");
    shell.history.observe_history_reprojection(None);
    assert!(OverlaysViewCtx::of(&mut shell).history_reprojection_accessibility_node(1).is_none() && paint_reprojection_band(&mut shell).0 == 0, "adopted: nothing announced or painted any more");
}
//#endregion 📡️HistoryReprojectionStatus

//#region 🫥️HostWindowBlur
/// ⚖️ LAW (React's pane `onBlur` when the page loses focus; S3-SPATIAL N9): a host window blur — native `Focused(false)`, the
/// browser's `host-window-blur` door — arms exactly one `hostEvent{windowId: <active pane>, kind: blur}` on the session's
/// controller at the shell's next drain, keeps the settle pump owed until then, and nothing more after it.
#[test]
fn a_host_window_blur_blurs_the_active_pane_for_its_program_once() {
    let mut shell = session_shell();
    assert!(!WindowActionsCtx::of(&mut shell).arm_host_window_blur(), "no blur, nothing armed");
    note_host_window_blur();
    assert!(PointerCtx::of(&mut shell).settle_pump_pending(), "a pending blur is work the settle pump owes");
    assert!(WindowActionsCtx::of(&mut shell).arm_host_window_blur(), "the drain takes the blur");
    let controller_id = shell.docs.shell_command_controller_id(&shell.plugins).expect("the session's controller");
    let armed: Vec<(String, String, Value)> = shell.settle.deferred_actions.iter().filter(|action| action.action == semio_framework::HOST_EVENT_ACTION_ID).map(|action| (action.controller_id.clone(), action.action.clone(), args_of(action))).collect();
    assert_eq!(armed, [(controller_id, semio_framework::HOST_EVENT_ACTION_ID.to_string(), serde_json::json!({ "windowId": "main", "kind": semio_framework::HOST_EVENT_KIND_BLUR }))], "one blur for the active pane");
    assert!(!WindowActionsCtx::of(&mut shell).arm_host_window_blur(), "a blur is taken once");
}
//#endregion 🫥️HostWindowBlur

//#region 📥️ImportTransfer
/// 🧾️ A shell whose session app declares `importAbort`, holding one picked import of two files (`a.obj` of two chunks'
/// worth of payload, `b.obj` of one).
fn importing_shell() -> (ShellState, u64) {
    let mut shell = session_shell();
    shell.docs.session.as_mut().expect("session").app.actions.push(semio_framework::ActionDefinition::new(IMPORT_ABORT_ACTION_ID, LocalizedLabel::native("Abort import", "Import abbrechen"), semio_framework::ActionKind::Mutation, IconName::X));
    let chunk = semio_framework::kernel::import_payload_chunks("x").len();
    assert_eq!(chunk, 1, "a tiny file is one chunk");
    let opened = vec![OpenedFile { name: "a.obj".into(), contents: "a".repeat(400_000) }, OpenedFile { name: "b.obj".into(), contents: "b".into() }];
    assert!(shell.opening.begin_import_transfer(PickedImport { controller_id: "test.controller".into(), import_action: "importFile".into(), args: None, opened, multiple: true }, &shell.docs));
    let id = shell.opening.import_transfers.front().expect("the import").id;
    (shell, id)
}

/// ⚖️ LAW (React's `importOpenedFilesV1` + `documentTransferTasksV1` on wgpu): a picked import is ONE running task in the Task
/// Manager — its files, lane and owner, a progress bar of the chunks delivered and "Cancel <files>" (en/de) — and keeps the
/// settle pump owed; a Cancel before any chunk reached the guest drops it with the `import-cancelled` notice and no
/// `importAbort`, a Cancel after one arms the app's `importAbort {}` exactly once, and "No task is running." returns.
#[test]
fn a_picked_import_is_a_cancellable_task_that_frees_a_started_import() {
    let (mut shell, id) = importing_shell();
    let transfer = shell.opening.import_transfers.front().expect("the import").clone();
    assert!(transfer.total > 2 && transfer.delivered == 0 && transfer.file == "a.obj, b.obj", "every file's chunks wait in order: {transfer:?}");
    assert!(PointerCtx::of(&mut shell).settle_pump_pending(), "a waiting import is work the settle pump owes");
    let body = serde_json::to_value(shell.opening.build_task_manager_ui(&shell.prefs)).expect("the Task Manager body serializes");
    let text = body.to_string();
    for expected in ["Running tasks", "a.obj, b.obj · Document import · test · Running", &format!("os.task-manager.progress.documentTransfer:{id}"), &format!("os.task-manager.cancel.documentTransfer:{id}"), "Cancel a.obj, b.obj", "cancelImportTransfer"] {
        assert!(text.contains(expected), "the Task Manager shows {expected:?}: {text}");
    }
    shell.prefs.locale_id = "de".into();
    let german = serde_json::to_value(shell.opening.build_task_manager_ui(&shell.prefs)).expect("serializes").to_string();
    assert!(german.contains("Laufende Aufgaben") && german.contains("a.obj, b.obj abbrechen") && german.contains("Dokumentimport"), "{german}");
    shell.prefs.locale_id = "en".into();

    assert!(SessionCtx::of(&mut shell).cancel_import_transfer(id));
    assert!(shell.opening.import_transfers.is_empty() && !shell.settle.deferred_actions.iter().any(|action| action.action == IMPORT_ABORT_ACTION_ID), "nothing reached the guest, nothing to free");
    let notice = shell.chrome.transient_notice().expect("told");
    assert_eq!((notice.message.as_str(), notice.code.as_deref()), ("Import of “a.obj” cancelled.", Some("shell.documentTransfer.import-cancelled")));
    assert!(serde_json::to_value(shell.opening.build_task_manager_ui(&shell.prefs)).expect("serializes").to_string().contains("No task is running."));

    let (mut shell, id) = importing_shell();
    shell.opening.import_transfers.front_mut().expect("the import").delivered = 1;
    assert!(SessionCtx::of(&mut shell).cancel_import_transfer(id));
    let aborts: Vec<(String, Value)> = shell.settle.deferred_actions.iter().filter(|action| action.action == IMPORT_ABORT_ACTION_ID).map(|action| (action.controller_id.clone(), args_of(action))).collect();
    assert_eq!(aborts, [("test.controller".to_string(), serde_json::json!({}))], "a started import is freed once, without a reason");
    assert!(!SessionCtx::of(&mut shell).cancel_import_transfer(id), "a cancelled import is gone");
}

/// ⚖️ LAW (`RequestMediaFrames` host cancel): the frames one decoded video yields — each `frameAction`, then `doneAction` —
/// are ONE running "Video frames" task in the Task Manager (en/de) that keeps the settle pump owed and dispatches in order; a
/// Cancel after a frame reached the guest drops the rest (the `doneAction` never fires) and arms the app's `importAbort {}`
/// once, telling the person by the video's name; an empty answer (no pick) starts nothing.
#[test]
fn a_decoded_video_is_a_cancellable_task_that_frees_a_started_stream() {
    let (mut shell, _) = importing_shell();
    shell.opening.import_transfers.clear();
    assert!(!shell.opening.begin_media_frames_transfer("test.controller".into(), Vec::new(), &shell.docs), "no pick, no task");
    let frame = |action: &str, index: usize| ActionDescriptor { controller_id: "test.controller".into(), action: action.into(), args: semio_framework_os_renderer_wgpu_host_base::action_args_json!({ "name": "clip.mp4", "index": index }) };
    assert!(shell.opening.begin_media_frames_transfer("test.controller".into(), vec![frame("importVideoFramePayload", 0), frame("importVideoFramePayload", 1), frame("importVideoDone", 2)], &shell.docs));
    let transfer = shell.opening.import_transfers.front().expect("the stream").clone();
    assert_eq!((transfer.lane, transfer.file.as_str(), transfer.total, transfer.chunks.back().map(|last| last.action.as_str())), (TransferLane::VideoFrames, "clip.mp4", 3, Some("importVideoDone")), "every frame, then done, waits in order: {transfer:?}");
    assert!(PointerCtx::of(&mut shell).settle_pump_pending(), "a waiting stream is work the settle pump owes");
    let text = serde_json::to_value(shell.opening.build_task_manager_ui(&shell.prefs)).expect("serializes").to_string();
    for expected in ["clip.mp4 · Video frames · test · Running", &format!("os.task-manager.cancel.documentTransfer:{}", transfer.id), "Cancel clip.mp4"] {
        assert!(text.contains(expected), "the Task Manager shows {expected:?}: {text}");
    }
    shell.prefs.locale_id = "de".into();
    assert!(serde_json::to_value(shell.opening.build_task_manager_ui(&shell.prefs)).expect("serializes").to_string().contains("clip.mp4 · Videobilder · test · Läuft"));
    shell.prefs.locale_id = "en".into();
    shell.opening.import_transfers.front_mut().expect("the stream").delivered = 1;
    assert!(SessionCtx::of(&mut shell).cancel_import_transfer(transfer.id));
    assert!(shell.opening.import_transfers.is_empty() && !shell.settle.deferred_actions.iter().any(|action| action.action == "importVideoDone"), "the rest of the stream is dropped");
    assert_eq!(shell.settle.deferred_actions.iter().filter(|action| action.action == IMPORT_ABORT_ACTION_ID).count(), 1, "a started stream is freed once");
    assert_eq!(shell.chrome.transient_notice().map(|notice| notice.message.clone()).as_deref(), Some("Import of “clip.mp4” cancelled."));
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
    let paste = shell_context_menu_item_from_spec(semio_framework_ui_layout::layout::ContextMenuItemSpec { id: "menu.paste".into(), label: Some("Paste".into()), action: Some("paste".into()), ..Default::default() }.disabled_because("The clipboard is empty".into()), "ctrl", false);
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
    shell.input.context_menu = Some(ContextMenuState { items: items.clone(), active: vec![2], ..Default::default() });
    assert!(matches!(shell.input.context_menu_handle_key(semio_framework_ui_wgpu_input::KeyAction::Enter), ContextMenuKeyOutcome::Ignored), "Enter on a disabled row fires nothing");
    assert!(shell.input.context_menu.is_some(), "and keeps the menu open");

    let menu = shell.input.context_menu.clone().expect("open");
    let (mut draw, mut atlas, icons, theme) = (DrawList::default(), FontAtlas::builtin(), IconAtlas::default(), Theme::default());
    let mut input = InputState::<ActionDescriptor>::default();
    render_context_menu_level(&mut draw, &mut atlas, &icons, &mut input, &theme, &menu, &menu.items, &[], 0.0, 0.0, 800.0, 600.0);
    let hits = input.staged_hits().to_vec();
    let paste_hit = hits.iter().find(|hit| hit.control_id.as_deref() == Some("menu.paste")).expect("the disabled row is a hit target");
    assert!(paste_hit.event.is_none(), "that fires nothing");
    let nodes = PanelsViewCtx::of(&mut shell).chrome_accessibility_nodes(&hits);
    let node = |key: &str| nodes.iter().find(|node| node.key == key).unwrap_or_else(|| panic!("`{key}` is projected"));
    let disabled = node("menu.paste");
    assert_eq!((disabled.role.as_str(), disabled.label.as_deref(), disabled.description.as_deref(), disabled.disabled, disabled.actionable, disabled.focusable), ("menuitem", Some("Paste"), Some("The clipboard is empty"), true, false, true));
    let enabled = node("menu.copy");
    assert_eq!((enabled.label.as_deref(), enabled.description.as_deref(), enabled.disabled, enabled.actionable), (Some("Copy"), None, false, true));

    let consumed = semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_shell_hit(paste_hit, &InputState::<ActionDescriptor>::default())).expect("activating a disabled row never errors");
    assert!(consumed && shell.input.context_menu.is_some() && shell.settle.deferred_actions.iter().all(|action| action.action != "paste"), "its activation fires nothing and keeps the menu open");
}
//#endregion 🍔️ContextMenuReason
