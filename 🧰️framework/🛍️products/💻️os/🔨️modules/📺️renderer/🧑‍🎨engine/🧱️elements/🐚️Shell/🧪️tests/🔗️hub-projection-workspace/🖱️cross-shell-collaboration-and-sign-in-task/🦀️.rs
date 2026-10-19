/// 🖱️ Cross-shell collaboration and sign-in task: native and React collaboration, shared cursors, the long guest turn and the cancellable sign-in verb.

/// 🤝️ One native wgpu user and one React `s` user co-edit ONE hub document (the fixture's block2d kind):
/// after [`CrossShellMeeting::meet`], each user's edit reaches the other's ledger, each undoes only their own
/// edit, and B's same-document reload converges (A's later edit reaches the reloaded B). Every step is
/// recorded before the law asserts the whole ledger.
///
/// 🔌️ `#[ignore]`d: needs a live hub (`SEMIO_HUB_LIVE_ORIGIN`, principal A `SEMIO_HUB_LIVE_EMAIL`/`_PASSWORD`,
/// B's `SEMIO_HUB_LIVE_PEER_EMAIL`), a staged native runtime (`SEMIO_PLUGIN_MODULES`/`SEMIO_PLUGIN`) and the
/// browser driver (`CROSS_MODE=edits`) sharing `SEMIO_CROSS_SHELL_DIR`.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a live hub, a React s serve and its browser driver; see this test's own doc comment"]
fn a_native_and_a_react_user_collaborate_on_one_hub_document() {
    let journey = native_guest_journey();
    let mut meeting = CrossShellMeeting::meet(journey.schema.as_str());
    let CrossShellMeeting { a, handshake, ledger, .. } = &mut meeting;

    let before = applied_edits(a).len();
    let (edit, latency) = author_edit(a, journey.verb.as_str());
    let a_ledger = applied_edits(a);
    handshake.publish("edit", serde_json::json!({ "verb": journey.verb, "ok": edit.is_ok(), "ledger": a_ledger.len() }));
    let react_ingested = handshake.await_react(a, "ingested", std::time::Duration::from_secs(120));
    ledger.record(
        "5-a-edits-b-ingests",
        edit.is_ok() && a_ledger.len() == before + 1 && react_ingested.as_ref().is_some_and(|value| value["ingested"] == true),
        format!("A outcome={edit:?} latency={latency:?} ledger {before}->{}; B {react_ingested:?}", a_ledger.len()),
    );

    let remote_before = applied_edits(a).iter().filter(|(action, _)| action == "apply").count();
    let react_edit = handshake.await_react(a, "edit", std::time::Duration::from_secs(120));
    let a_ingests = pump_until_ledger(a, std::time::Duration::from_secs(30), |ledger| ledger.iter().filter(|(action, _)| action == "apply").count() > remote_before);
    let a_after_b = applied_edits(a);
    handshake.publish("ingested", serde_json::json!({ "ingested": a_ingests.is_some(), "ledger": a_after_b.len() }));
    ledger.record("6-b-edits-a-ingests", react_edit.as_ref().is_some_and(|value| value["ok"] == true) && a_ingests.is_some(), format!("B {react_edit:?}; A ingested after {a_ingests:?} ledger {remote_before}->{a_after_b:?}"));

    let (undo, _) = author_edit(a, journey.undo.as_str());
    let a_undone = applied_edits(a);
    let own_reverted = a_undone.iter().filter(|(action, applied)| action == journey.verb.as_str() && !applied).count() == 1;
    handshake.publish("undo", serde_json::json!({ "ok": undo.is_ok(), "ownReverted": own_reverted, "ledger": a_undone.iter().map(|(action, applied)| format!("{action}:{applied}")).collect::<Vec<_>>() }));
    let react_undo = handshake.await_react(a, "undo", std::time::Duration::from_secs(120));
    let a_after_b_undo = pump_until_ledger(a, std::time::Duration::from_secs(20), |ledger| ledger.iter().any(|(action, applied)| action == "apply" && !applied));
    let a_final = applied_edits(a);
    ledger.record(
        "7-each-undoes-own",
        undo.is_ok() && own_reverted && react_undo.as_ref().is_some_and(|value| value["ok"] == true && value["ownReverted"] == true) && a_after_b_undo.is_some(),
        format!("A undo={undo:?} own-reverted={own_reverted}; B {react_undo:?}; A sees B's undo after {a_after_b_undo:?} ledger={a_final:?}"),
    );

    let react_reloaded = handshake.await_react(a, "reloaded", std::time::Duration::from_secs(600));
    let (after_reload, _) = author_edit(a, journey.verb.as_str());
    handshake.publish("after-reload-edit", serde_json::json!({ "ok": after_reload.is_ok(), "ledger": applied_edits(a).len() }));
    let react_converged = handshake.await_react(a, "converged", std::time::Duration::from_secs(120));
    ledger.record(
        "8-reload-converges",
        react_reloaded.as_ref().is_some_and(|value| value["live"] == true) && after_reload.is_ok() && react_converged.as_ref().is_some_and(|value| value["converged"] == true),
        format!("B reloaded {react_reloaded:?}; A edit {after_reload:?}; B {react_converged:?}"),
    );
    meeting.finish();
}

/// 🖱️ One native wgpu user and one React `s` user point at ONE hub board ([`CROSS_SHELL_BOARD_SCHEMA`]):
/// after [`CrossShellMeeting::meet`], A's pointer over its painted board travels as a canvas-presence window
/// view on the one presence wire and React paints A's cursor over the same window; B moves its mouse over
/// React's board canvas and A's verified roster carries B's view, which A's chrome turns into a peer cursor
/// over its own board (`board_peer_overlays`).
///
/// 🔌️ `#[ignore]`d: the same live inputs as [`a_native_and_a_react_user_collaborate_on_one_hub_document`], with
/// the browser driver in `CROSS_MODE=cursors`.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a live hub, a React s serve and its browser driver; see this test's own doc comment"]
fn a_native_and_a_react_user_see_each_others_cursor_on_one_hub_board() {
    let mut meeting = CrossShellMeeting::meet(CROSS_SHELL_BOARD_SCHEMA);
    let CrossShellMeeting { a, handshake, ledger, b_actor, boards, .. } = &mut meeting;

    let board = a.surfaces.board2d_states.iter().next().map(|(_, surface)| (surface.window_id.clone(), surface.bounds));
    if let Some((_, bounds)) = board.as_ref() {
        a.directory.presence_pointer = Some((bounds.x + bounds.w * 0.75, bounds.y + bounds.h * 0.25));
    }
    let (views, _) = a.directory.board_presence_views(&a.surfaces);
    let _ = pump_until(a, std::time::Duration::from_secs(2), |_| false);
    handshake.publish(
        "cursor",
        serde_json::json!({ "board": board.as_ref().map(|(window, _)| window), "views": views.iter().map(|view| serde_json::json!({ "windowId": view.window_id, "space": view.space, "pointer": view.pointer })).collect::<Vec<_>>() }),
    );
    let react_cursor = handshake.await_react(a, "cursor", std::time::Duration::from_secs(120));
    let b_actor = b_actor.clone();
    let native_sees = pump_until(a, std::time::Duration::from_secs(20), |shell| shell.directory.board_peer_overlays(&shell.surfaces).iter().any(|(_, overlays)| overlays.cursors.iter().any(|cursor| cursor.actor == b_actor)));
    let a_overlays: Vec<String> = a.directory.board_peer_overlays(&a.surfaces).iter().flat_map(|(_, overlays)| overlays.cursors.iter().map(|cursor| format!("{}@{:?} chip={}", cursor.actor, cursor.at, cursor.chip))).collect();
    let b_views: Vec<String> = a.directory.presence_peers.iter().filter(|peer| peer.actor == b_actor).flat_map(|peer| peer.views.iter().map(|view| format!("{}:{}:{:?}", view.window_id, view.space, view.pointer))).collect();
    ledger.record(
        "5-cursors-both-ways",
        native_sees.is_some() && react_cursor.as_ref().is_some_and(|value| value["seesNativeCursor"] == true),
        format!("A boards={boards:?} views={views:?} B sees A's cursor: {react_cursor:?}; A sees B's cursor after {native_sees:?}: {a_overlays:?} (B's views {b_views:?})"),
    );
    handshake.publish("cursor-seen", serde_json::json!({ "seesReactCursor": native_sees.is_some(), "overlays": a_overlays }));
    meeting.finish();
}

/// ⚖️ A long guest turn shares the native kernel and only its owner's cancel ends it (ticket 26/09/23 WG8,
/// coordinator-approved design): the hub-resolved puzzle component's `InstanceOpen` — longer than the old 30 s
/// wall-clock kill in a debug host under load (cross-shell run 10) — completes while the staged block2d app keeps
/// answering history reads between its slices, and a second open dropped by its owner mid-turn is retired.
///
/// 🔌️ `#[ignore]`d: needs the staged native runtime (`SEMIO_PLUGIN_MODULES`/`SEMIO_PLUGIN`, the journey fixture's
/// app answers) and the long-opening component (`SEMIO_LONG_TURN_PLUGIN`, `SEMIO_LONG_TURN_APP`,
/// `SEMIO_LONG_TURN_COMPONENT`, `SEMIO_LONG_TURN_DESCRIPTOR`, `SEMIO_LONG_TURN_RECEIPT`).
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native runtime and a long-opening component; see this test's own doc comment"]
fn a_long_guest_turn_shares_the_kernel_and_only_its_owners_cancel_ends_it() {
    let journey = native_guest_journey();
    let modules = std::path::PathBuf::from(live_env("SEMIO_PLUGIN_MODULES"));
    let plugins = drive(semio_framework_os_renderer_wgpu_program_bridge::load_wasm_plugins(&live_env("SEMIO_PLUGIN"), &modules)).expect("the staged native runtime loads");
    let answering_program = plugins.iter().find(|entry| entry.plugin_id == journey.plugin_id).cloned().expect("the staged runtime carries the answering app");
    let answering = drive(answering_program.create_app(&journey.app_id)).expect("the answering app opens");
    let long = drive(semio_framework_os_renderer_wgpu_program_bridge::load_resolved_program(
        &live_env("SEMIO_LONG_TURN_PLUGIN"),
        std::path::Path::new(&live_env("SEMIO_LONG_TURN_COMPONENT")),
        std::path::Path::new(&live_env("SEMIO_LONG_TURN_DESCRIPTOR")),
        &semio_framework_os_kernel::os_component_receipt::ComponentReceiptV1::decode_canonical(&std::fs::read(live_env("SEMIO_LONG_TURN_RECEIPT")).expect("the long-opening component's receipt")).expect("a canonical shipped receipt"),
    ))
    .expect("the long-opening component loads");
    let long_app = live_env("SEMIO_LONG_TURN_APP");
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    let pump = || {
        let _ = semio_framework_os_renderer_wgpu_host_base::pump_renderer_io_sessions(1);
        let _ = semio_framework_job_worker::pump_worker_job_retirements(1, 1, semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES);
    };

    let started = std::time::Instant::now();
    let mut opening = Box::pin(long.create_app(&long_app));
    let mut answers: Vec<std::time::Duration> = Vec::new();
    let mut answer = None;
    let opened = loop {
        pump();
        if let std::task::Poll::Ready(result) = std::future::Future::poll(opening.as_mut(), &mut context) {
            break result;
        }
        let (asked, reading) = answer.get_or_insert_with(|| (std::time::Instant::now(), Box::pin(answering_program.read_history(answering))));
        if let std::task::Poll::Ready(read) = std::future::Future::poll(reading.as_mut(), &mut context) {
            read.expect("the answering app answers while the long turn is in flight");
            answers.push(asked.elapsed());
            answer = None;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    };
    let open_took = started.elapsed();
    drop(answer);
    let longest_answer = answers.iter().max().copied().unwrap_or_default();
    println!("fair-turns long open took={open_took:?} result={opened:?} answers-while-open={} longest-answer={longest_answer:?}", answers.len());
    let long_instance = opened.expect("the long InstanceOpen completes: no wall-clock limit ends a progressing turn");
    assert!(open_took < std::time::Duration::from_secs(2) || !answers.is_empty(), "another app answered between the long turn's slices ({open_took:?} open, no answer)");

    let cancelled_before = semio_framework_os_renderer_wgpu_kernel_runtime::CANCELLED_KERNEL_TURNS.load(std::sync::atomic::Ordering::Relaxed);
    let cancelled_open_started = std::time::Instant::now();
    let mut cancelled_open = Box::pin(long.create_app(&long_app));
    let mut finished_before_cancel = false;
    while cancelled_open_started.elapsed() < std::time::Duration::from_millis(800) {
        pump();
        if std::future::Future::poll(cancelled_open.as_mut(), &mut context).is_ready() {
            finished_before_cancel = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    drop(cancelled_open);
    let answered_after_cancel = std::time::Instant::now();
    let mut reading = Box::pin(answering_program.read_history(answering));
    let read = loop {
        pump();
        if let std::task::Poll::Ready(read) = std::future::Future::poll(reading.as_mut(), &mut context) {
            break read;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    };
    let cancelled_after = semio_framework_os_renderer_wgpu_kernel_runtime::CANCELLED_KERNEL_TURNS.load(std::sync::atomic::Ordering::Relaxed);
    println!("fair-turns cancel finished-before-cancel={finished_before_cancel} cancelled-turns={cancelled_before}->{cancelled_after} next-answer-after={:?} ok={}", answered_after_cancel.elapsed(), read.is_ok());
    read.expect("the kernel keeps answering after a cancelled turn");
    assert!(finished_before_cancel || cancelled_after == cancelled_before + 1, "dropping the open mid-turn retired exactly that turn");
    long.destroy_app(long_instance);
    answering_program.destroy_app(answering);
}

/// 📒️ Pumps one shell's frames ([`frame_pump`]) and re-reads its edit ledger ([`applied_edits`]) every half second
/// until `done` holds for the ledger or the budget ends — the ledger is a history snapshot, not frame state.
#[cfg(not(target_arch = "wasm32"))]
fn pump_until_ledger(shell: &mut ShellState, budget: std::time::Duration, done: impl Fn(&[(String, bool)]) -> bool) -> Option<std::time::Duration> {
    let started = std::time::Instant::now();
    let mut next_read = started;
    while started.elapsed() < budget {
        let _ = frame_pump(shell);
        if std::time::Instant::now() >= next_read {
            if done(&applied_edits(shell)) {
                return Some(started.elapsed());
            }
            next_read = std::time::Instant::now() + std::time::Duration::from_millis(500);
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    None
}

/// ⏳️ Pumps one shell's frames ([`frame_pump`]) until `done` holds or the budget ends.
#[cfg(not(target_arch = "wasm32"))]
fn pump_until(shell: &mut ShellState, budget: std::time::Duration, done: impl Fn(&ShellState) -> bool) -> Option<std::time::Duration> {
    let started = std::time::Instant::now();
    while started.elapsed() < budget {
        let _ = frame_pump(shell);
        if done(shell) {
            return Some(started.elapsed());
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    None
}


/// 🔐️ LAW (ticket 26/09/23 session 12, runs s12g/s12h): the sign-in and the spaces read never hold the frame's
/// interaction state. The verbs only ARM a [`ShellHubTask`] and return — so the chrome shows `Signing in…` with its
/// Cancel while the hub hashes — the directory pump applies the answer, and Cancel ends the request: a cancelled
/// sign-in's late answer is never applied.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_sign_in_verb_arms_a_task_and_returns_while_the_hub_is_still_answering() {
    let source = semio_framework_os_renderer_wgpu_test_support::shell_source();
    let dispatch = source.split("fn handle_hub_workspace_action<'a>(").nth(1).expect("the hub verb dispatch exists");
    let dispatch = &dispatch[..dispatch.find("\n    }\n").expect("the dispatch closes")];
    assert!(dispatch.contains("hub_action::SIGN_IN => self.start_hub_sign_in(),"), "SIGN_IN arms and returns");
    assert!(dispatch.contains("hub_action::REFRESH_SPACES => self.start_hub_spaces_reload(),"), "REFRESH_SPACES arms and returns");
    assert!(!source.contains("run_hub_sign_in_turn") && !source.contains("reload_hub_spaces().await"), "no hub leg is awaited inside a verb");

    let mut shell = shell();
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_EMAIL, &[("value", "user1@semio.dev")]);
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_PASSWORD, &[("value", "correct horse battery staple")]);
    let (sender, receiver) = std::sync::mpsc::channel();
    shell.directory.hub_workspace.session = reduce_hub_session(&shell.directory.hub_workspace.session, &HubSessionEvent::Submit);
    shell.directory.hub_sign_in_task = Some(ShellHubTask { receiver, cancel: CancelToken::root_now(), task: None });
    assert_eq!(shell.directory.hub_workspace.session.phase, HubSessionPhase::SigningIn);
    for locale in [Locale::En, Locale::De] {
        let tree = serde_json::to_string(&semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::build_hub_workspace_ui(&shell.directory.hub_workspace, locale)).expect("hub tree json");
        assert!(tree.contains("framework.hub.sign-in.cancel"), "a running sign-in offers Cancel ({locale:?})");
    }
    assert!(!shell.sync.poll_hub_workspace_tasks(&mut shell.directory), "nothing is applied before the hub answers");
    sender.send(ShellHubSignInAnswer::Failed { code: HubSignInErrorCode::InvalidCredentials, retry_after_seconds: None }).expect("the task answers");
    assert!(shell.sync.poll_hub_workspace_tasks(&mut shell.directory), "the pump applies the answer");
    assert!(shell.directory.hub_workspace_settled());
    assert_eq!(shell.directory.hub_workspace.session.error, Some(HubSignInErrorCode::InvalidCredentials));

    let (late, receiver) = std::sync::mpsc::channel();
    shell.directory.hub_workspace.session = reduce_hub_session(&shell.directory.hub_workspace.session, &HubSessionEvent::Submit);
    shell.directory.hub_sign_in_task = Some(ShellHubTask { receiver, cancel: CancelToken::root_now(), task: None });
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::CANCEL_SIGN_IN, &[]);
    assert!(shell.directory.hub_workspace_settled(), "Cancel ends the request");
    let _ = late.send(ShellHubSignInAnswer::Unverified);
    assert!(!shell.sync.poll_hub_workspace_tasks(&mut shell.directory), "a cancelled sign-in's late answer is never applied");
    assert_eq!(shell.directory.hub_workspace.session.error, Some(HubSignInErrorCode::Cancelled));
}
