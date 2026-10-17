/// 🧭️ Native guest journey: the shared guest journey fixture and the hubless mount, undo, redo, copy and cancellable open laws.

/// ⏯️ The one guest journey both native laws drive, read from the shared language-agnostic fixture.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct NativeGuestJourney {
    plugin_id: String,
    artifact_ref: String,
    app_id: String,
    schema: String,
    verb: String,
    undo: String,
    redo: String,
    clipboard: Vec<String>,
    expected_edits_after_verb: usize,
    expected_applied_after_undo: usize,
    expected_applied_after_redo: usize,
    expected_edits_after_clipboard: usize,
}

#[cfg(not(target_arch = "wasm32"))]
fn native_guest_journey() -> NativeGuestJourney {
    serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/⏯️native-guest-journey/🔣️.json")).expect("native guest journey fixture")
}

/// ⏯️ Mounts the fixture's staged guest in one native wgpu shell with no hub, authors the fixture's
/// verb and asserts the mount and the edit: the relay switches to the guest's app, every surface its
/// session opens reaches the retained registry (no `Surface unavailable` fault), the session binds the
/// document it opened (a refused binding used to pass silently as a debug line), and the verb settles
/// and lands exactly once in the guest's own ledger. Answers the shell for a law that continues.
#[cfg(not(target_arch = "wasm32"))]
fn native_guest_authored(journey: &NativeGuestJourney) -> ShellState {
    let modules = std::path::PathBuf::from(live_env("SEMIO_PLUGIN_MODULES"));
    let variant = live_env("SEMIO_PLUGIN");
    let plugins = drive(semio_framework_os_renderer_wgpu_program_bridge::load_wasm_plugins(&variant, &modules)).expect("the staged native runtime loads");
    assert!(plugins.iter().any(|entry| entry.plugin_id == journey.plugin_id), "the staged runtime carries {}", journey.plugin_id);
    let mut shell = ShellState::new(plugins, variant, semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let document_id = format!("native-guest-journey-{}", chrome_now_ms() as u64);
    let started = std::time::Instant::now();
    shell_command(
        &mut shell,
        "os.open-artifact",
        &[("artifactRef", journey.artifact_ref.as_str()), ("pluginId", journey.plugin_id.as_str()), ("appId", journey.app_id.as_str()), ("documentId", document_id.as_str()), ("schema", journey.schema.as_str())],
    );
    let command = started.elapsed();
    let frames = settle_document_opening(&mut shell);
    let session = shell.docs.session.as_ref().map(|session| (session.plugin_id.clone(), session.app.id.clone(), session.instance_id));
    println!("native-guest-journey open command={command:?} frames={frames:?} session={session:?} error={:?}", shell.chrome.error);
    assert!(shell.opening.document_opening.is_none(), "the open settled and cleared its band: {:?}", shell.opening.document_opening.as_ref().map(|opening| &opening.phase));
    assert_eq!(session.as_ref().map(|(plugin, app, _)| (plugin.as_str(), app.as_str())), Some((journey.plugin_id.as_str(), journey.app_id.as_str())), "the relay mounted the guest's app");
    assert_eq!(shell.chrome.error, None, "every surface the session opened reached the retained registry");
    assert_eq!(shell.sync.sync_channel.as_ref().map(|channel| channel.document_id.as_str()), Some(document_id.as_str()), "the relay bound the session to the document it opened");

    let before = applied_edits(&mut shell);
    let (edit, latency) = author_edit(&mut shell, &journey.verb);
    let after = applied_edits(&mut shell);
    println!("native-guest-journey verb={} outcome={edit:?} latency={latency:?} ledger {before:?} -> {after:?}", journey.verb);
    assert_eq!(edit, Ok(()), "the authored verb settles");
    assert_eq!(applied_count(&after, &journey.verb), applied_count(&before, &journey.verb) + journey.expected_edits_after_verb);
    shell
}

/// ⏯️ One native wgpu shell mounts the staged guest on the native kernel thread with no hub at all and
/// authors one edit (see [`native_guest_authored`]). This is the kernel turn loop's contract with a
/// real owned-interpreter guest — the B1 defect of ticket 26/09/18 slice G7w (the first post-boot turn
/// never returned, then every authored edit spun `MoreWork` until the settle budget) reproduced
/// without a hub, so it can never hide behind a hub-side failure again.
///
/// 🔌️ `#[ignore]`d: needs the fixture's native runtime staged at `SEMIO_PLUGIN_MODULES` for
/// `SEMIO_PLUGIN`; the renderer package's `native-guest-journey-check` verb provides both.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native guest runtime; see this test's own doc comment"]
fn a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub() {
    let _ = native_guest_authored(&native_guest_journey());
}

/// 🧮️ How many of `ledger`'s edits are `verb`'s and applied.
#[cfg(not(target_arch = "wasm32"))]
fn applied_count(ledger: &[(String, bool)], verb: &str) -> usize {
    ledger.iter().filter(|(action, applied)| action == verb && *applied).count()
}

/// ⏪️ [`native_guest_authored`], then the fixture's undo, asserted to settle and to revert exactly the
/// authored edit. Answers the shell for a law that continues.
#[cfg(not(target_arch = "wasm32"))]
fn native_guest_undone(journey: &NativeGuestJourney) -> ShellState {
    let mut shell = native_guest_authored(journey);
    let (undo, latency) = author_edit(&mut shell, &journey.undo);
    let undone = applied_edits(&mut shell);
    println!("native-guest-journey undo outcome={undo:?} latency={latency:?} ledger {undone:?}");
    assert_eq!(undo, Ok(()), "undo settles");
    assert_eq!(applied_count(&undone, &journey.verb), journey.expected_applied_after_undo, "undo reverted exactly the authored edit");
    shell
}

/// ⏪️ The journey's edit, then its undo. Undo is a framework reserved tool verb: the guest admits it and
/// spawns a `semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND` job, so this law holds the native
/// kernel to starting that job live, stepping it to its end and settling the guest's completion turn
/// — the one reserved-job mechanism the React host drives in `driveReservedToolJob` (ticket 26/09/23
/// slice WG8, §1.4).
///
/// 🔌️ `#[ignore]`d for the same staged runtime as [`a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native guest runtime; see this test's own doc comment"]
fn a_native_guest_undoes_its_authored_edit_without_a_hub() {
    let _ = native_guest_undone(&native_guest_journey());
}

/// ⏩️ The journey's edit and undo, then the fixture's redo: a second reserved tool job on the same
/// instance, which settles and applies exactly the undone edit again.
///
/// 🔌️ `#[ignore]`d for the same staged runtime as [`a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native guest runtime; see this test's own doc comment"]
fn a_native_guest_redoes_its_undone_edit_without_a_hub() {
    let journey = native_guest_journey();
    let mut shell = native_guest_undone(&journey);
    let (redo, latency) = author_edit(&mut shell, &journey.redo);
    let redone = applied_edits(&mut shell);
    println!("native-guest-journey redo outcome={redo:?} latency={latency:?} ledger {redone:?}");
    assert_eq!(redo, Ok(()), "redo settles");
    assert_eq!(applied_count(&redone, &journey.verb), journey.expected_applied_after_redo, "redo applied exactly the undone edit again");
}

/// 📋️ The journey's edit, then the fixture's selection and clipboard verbs in order (select all, copy,
/// paste): each is a framework reserved verb — the selection one a spawned reserved tool job, the
/// clipboard ones run inside the guest's turn — and each settles, leaving the edit ledger exactly as the
/// fixture declares.
///
/// 🔌️ `#[ignore]`d for the same staged runtime as [`a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native guest runtime; see this test's own doc comment"]
fn a_native_guest_copies_and_pastes_its_selection_without_a_hub() {
    let journey = native_guest_journey();
    let mut shell = native_guest_authored(&journey);
    let before = applied_edits(&mut shell);
    for verb in &journey.clipboard {
        let (outcome, latency) = author_edit(&mut shell, verb);
        println!("native-guest-journey clipboard verb={verb} outcome={outcome:?} latency={latency:?}");
        assert_eq!(outcome, Ok(()), "{verb} settles");
    }
    let after = applied_edits(&mut shell);
    println!("native-guest-journey clipboard ledger {before:?} -> {after:?}");
    assert_eq!(after.len(), before.len() + journey.expected_edits_after_clipboard, "the clipboard verbs left the declared edits");
    assert_eq!(applied_count(&after, &journey.verb), applied_count(&before, &journey.verb), "the authored edit stays applied");
}

/// 🖼️ Opening a document never holds the shell across a guest turn: the relay only starts the open,
/// every frame while the guest instantiates and loads the document stays short, and the band's cancel
/// stops an open whose app instance is still being created — the instance is destroyed and the session
/// stays as it was. Before, the relay held the shell (and so the frame build) for the whole open: 17 s
/// in a debug build (ticket 26/09/23 slice WG8). The frames that then render the new session are
/// measured and reported; each spends the guest's own render turns.
///
/// 🔌️ `#[ignore]`d for the same staged runtime as [`a_native_guest_mounts_and_settles_an_authored_edit_without_a_hub`].
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a staged native guest runtime; see this test's own doc comment"]
fn a_native_guest_open_keeps_the_frame_loop_painting_and_is_cancellable() {
    let journey = native_guest_journey();
    let modules = std::path::PathBuf::from(live_env("SEMIO_PLUGIN_MODULES"));
    let variant = live_env("SEMIO_PLUGIN");
    let plugins = drive(semio_framework_os_renderer_wgpu_program_bridge::load_wasm_plugins(&variant, &modules)).expect("the staged native runtime loads");
    let mut shell = ShellState::new(plugins, variant, semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    let open = |shell: &mut ShellState, document_id: &str| {
        let started = std::time::Instant::now();
        shell_command(shell, "os.open-artifact", &[("artifactRef", journey.artifact_ref.as_str()), ("pluginId", journey.plugin_id.as_str()), ("appId", journey.app_id.as_str()), ("documentId", document_id), ("schema", journey.schema.as_str())]);
        started.elapsed()
    };

    let command = open(&mut shell, "native-guest-open-cancelled");
    assert_eq!(shell.opening.document_opening.as_ref().map(|opening| opening.phase.clone()), Some(ShellDocumentOpenPhase::Instantiating), "the relay only starts the open");
    shell.opening.cancel_document_opening();
    let cancelled = settle_document_opening(&mut shell);
    println!("native-guest-open cancel command={command:?} frames={cancelled:?} phase={:?}", shell.opening.document_opening.as_ref().map(|opening| &opening.phase));
    assert_eq!(shell.opening.document_opening.as_ref().map(|opening| opening.phase.clone()), Some(ShellDocumentOpenPhase::Cancelled), "the cancel settled the open");
    assert!(shell.docs.session.is_none() && shell.sync.sync_channel.is_none(), "a cancelled open mounts nothing");
    shell.opening.cancel_document_opening();
    assert!(shell.opening.document_opening.is_none(), "closing the settled band clears it");

    let command = open(&mut shell, "native-guest-open-painted");
    let frames = settle_document_opening(&mut shell);
    println!("native-guest-open command={command:?} frames={frames:?}");
    assert!(command < std::time::Duration::from_millis(500), "the relay started the open without a guest turn: {command:?}");
    assert!(frames.opening > 1 && frames.opening_longest < std::time::Duration::from_millis(500), "every frame stayed short while the guest opened: {frames:?}");
    assert_eq!(shell.sync.sync_channel.as_ref().map(|channel| channel.document_id.as_str()), Some("native-guest-open-painted"));
}
