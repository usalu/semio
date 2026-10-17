/// 🏗️ Creation door and cross-shell meeting: the live artifact creation door, the cross-shell file handshake, the window painter and the cross-shell meeting driver.

/// 🌱️ One native wgpu shell creates a hub-bound artifact through its own creation door against a
/// REAL hub: sign in, create and open a space, the door loads the space's selected current catalog,
/// the first creatable kind is chosen and named, Create submits the sealed intent, the frame pump
/// polls the hub's receipts, and the creation reaches `Ready` with a hub-minted artifact id — every
/// step through the shell's own lanes and state (ticket 26/09/23 slice WG8, G-P1-3). Whether the
/// ready artifact then opens depends on the kind's guest being staged natively, so the law reports the
/// opening and asserts only the creation.
///
/// 🔌️ `#[ignore]`d: needs a live credential-sign-in hub with a ready trusted catalog at
/// `SEMIO_HUB_LIVE_ORIGIN` and one principal at `SEMIO_HUB_LIVE_EMAIL`/`_PASSWORD`.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a live hub with a ready trusted catalog; see this test's own doc comment"]
fn a_live_hub_artifact_is_created_through_the_wgpu_creation_door() {
    let origin = live_env("SEMIO_HUB_LIVE_ORIGIN");
    let mut shell = shell();
    let refusal = sign_in_live(&mut shell, &origin, &live_env("SEMIO_HUB_LIVE_EMAIL"), &live_env("SEMIO_HUB_LIVE_PASSWORD"));
    assert_eq!(shell.directory.hub_workspace.session.phase, HubSessionPhase::SignedIn, "first refusal {refusal:?}");
    let space_name = format!("wg8 door {}", chrome_now_ms() as u64);
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_SPACE_NAME, &[("value", space_name.as_str())]);
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::CREATE_SPACE, &[]);
    let space_id = shell.directory.hub_workspace.rows.iter().find(|row| row.name == space_name).map(|row| row.id.clone()).expect("the created space is listed");
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::OPEN_SPACE, &[("spaceId", space_id.as_str())]);
    let creation = &shell.directory.hub_workspace.creation;
    let kinds: Vec<(String, String, String)> = creation.catalog.iter().flat_map(|catalog| catalog.kinds.iter()).map(|kind| (kind.kind_id.clone(), kind.label.en.clone(), kind.label.de.clone())).collect();
    println!("wg8-door catalog={} kinds={kinds:?}", creation.catalog_phase.as_str());
    assert_eq!(creation.catalog_phase, HubArtifactCatalogPhase::Ready);
    let kind_id = kinds.first().map(|(kind_id, _, _)| kind_id.clone()).expect("the catalog offers a kind");
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SELECT_ARTIFACT_KIND, &[("kindId", kind_id.as_str())]);
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_ARTIFACT_NAME, &[("value", "Door artifact")]);
    let started = std::time::Instant::now();
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::CREATE_ARTIFACT, &[]);
    let mut trail = Vec::new();
    while started.elapsed() < std::time::Duration::from_secs(150) {
        let observed = shell.directory.hub_workspace.creation.operation.as_ref().map(|operation| (semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::hub_artifact_creation_phase_str(operation.phase), operation.opening));
        if trail.last() != Some(&observed) {
            trail.push(observed);
        }
        if shell.directory.hub_workspace.creation.operation.as_ref().is_some_and(|operation| hub_artifact_creation_terminal(operation.phase) && operation.opening != HubArtifactOpening::Idle && operation.opening != HubArtifactOpening::Opening) {
            break;
        }
        drive(PointerCtx::of(&mut shell).pump_sync_events());
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    let operation = shell.directory.hub_workspace.creation.operation.clone().expect("the door holds its creation");
    println!("wg8-door kind={kind_id} trail={trail:?} after={:?} ready={:?}", started.elapsed(), operation.ready);
    for locale in [Locale::En, Locale::De] {
        let tree = semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::build_hub_workspace_ui(&shell.directory.hub_workspace, locale);
        let mut phases = Vec::new();
        hub_attribute_values(&tree, "data-semio-hub-artifact-creation-phase", &mut phases);
        println!("wg8-door tree locale={locale:?} phase={phases:?}");
    }
    assert_eq!(operation.phase, SpaceArtifactCreationPhaseV1::Ready);
    assert!(operation.ready.as_ref().is_some_and(|ready| ready.kind_id == kind_id && ready.artifact_id.starts_with("artifact-")));
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SIGN_OUT, &[]);
}

/// 🌉️ The file handshake between the native half of the cross-shell journey and its browser driver
/// (`SEMIO_CROSS_SHELL_DIR`): each side publishes one JSON observation per step (`native-<step>.json`,
/// `react-<step>.json`, written whole through a rename), and waits for the other side's step while its own
/// frames keep running ([`frame_pump`]) — a waiting native user still beats, ingests and paints.
#[cfg(not(target_arch = "wasm32"))]
struct CrossShellHandshake {
    dir: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl CrossShellHandshake {
    fn publish(&self, step: &str, value: Value) {
        let target = self.dir.join(format!("native-{step}.json"));
        let staged = self.dir.join(format!(".native-{step}.json"));
        std::fs::write(&staged, serde_json::to_vec_pretty(&value).expect("handshake step encodes")).expect("handshake step is written");
        std::fs::rename(&staged, &target).expect("handshake step is published");
        println!("cross-shell native {step} {value}");
    }

    fn await_react(&self, shell: &mut ShellState, step: &str, budget: std::time::Duration) -> Option<Value> {
        let path = self.dir.join(format!("react-{step}.json"));
        let started = std::time::Instant::now();
        while started.elapsed() < budget {
            if let Ok(bytes) = std::fs::read(&path) {
                let value: Value = serde_json::from_slice(&bytes).expect("react handshake step decodes");
                println!("cross-shell react {step} after {:?} {value}", started.elapsed());
                return Some(value);
            }
            let _ = frame_pump(shell);
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
        println!("cross-shell react {step} never arrived within {budget:?}");
        None
    }
}

/// 🖼️ Paints every window body of the mounted session once, headless, the way a presented frame paints
/// it (the shell input laws' own painter), so the engine surfaces a window carries (the board) register and
/// mirror into the shell's pointer-state maps exactly as they would under a GPU. Answers the painted boards.
#[cfg(not(target_arch = "wasm32"))]
fn paint_session_windows(shell: &mut ShellState, body: Rect) -> Vec<String> {
    let _serialized = semio_framework_os_renderer_wgpu_engine_registry::engine_surface_law_guard();
    let controller = shell.docs.session.as_ref().map(|session| session.app.controller_id.clone()).unwrap_or_default();
    let mut windows: Vec<String> = shell.docs.window_ui.keys().cloned().collect();
    windows.sort();
    shell.dock.dock_window_plan = windows.iter().map(|window_id| (window_id.clone(), body)).collect();
    let documents: Vec<(String, UiDocumentLease)> = windows.iter().filter_map(|window_id| shell.docs.window_ui.remove(window_id).map(|document| (window_id.clone(), document))).collect();
    let rows: Vec<(&str, &str, &UiDocumentLease, Rect)> = documents.iter().map(|(window_id, document)| (window_id.as_str(), controller.as_str(), document, body)).collect();
    let _input = paint_component_pointer_documents(shell, &rows);
    drop(rows);
    shell.docs.window_ui.extend(documents);
    SessionCtx::of(&mut shell).sync_engine_surface_states();
    let boards: Vec<String> = shell.surfaces.board2d_states.iter().map(|(_, surface)| surface.window_id.clone()).collect();
    println!("cross-shell native painted windows={windows:?} boards={boards:?}");
    boards
}

/// 🧩️ The canvas kind the cross-shell cursor law opens: puzzle2d's editor window `2d-overview` is a
/// `SurfaceKind::Board2d` pane on both shells, so each user has a board to point at and to paint the other's
/// cursor over (block2d's only window is a summary surface with no board).
#[cfg(not(target_arch = "wasm32"))]
const CROSS_SHELL_BOARD_SCHEMA: &str = "board.ports.directed.v1";

/// 🤝️ Steps 1–4 of the cross-shell journey (audit s12 P1-4, ticket 26/09/23 slice WG8), shared by both
/// cross-shell laws: the native wgpu user A signs in, creates a space, seats the React user B as an author,
/// creates an artifact of `schema` through its door and opens it (the hub-resolved component), paints its
/// windows; B opens the same artifact from its Space index in a real browser on a real `s` serve (driven by
/// `wp-wg8/cross-shell.mjs` through [`CrossShellHandshake`]); then each sees the other in its roster.
#[cfg(not(target_arch = "wasm32"))]
struct CrossShellMeeting {
    a: ShellState,
    handshake: CrossShellHandshake,
    ledger: CollaborationLedger,
    b_actor: String,
    boards: Vec<String>,
}

#[cfg(not(target_arch = "wasm32"))]
impl CrossShellMeeting {
    fn meet(schema: &str) -> Self {
        let origin = live_env("SEMIO_HUB_LIVE_ORIGIN");
        let (a_email, a_password) = (live_env("SEMIO_HUB_LIVE_EMAIL"), live_env("SEMIO_HUB_LIVE_PASSWORD"));
        let b_email = live_env("SEMIO_HUB_LIVE_PEER_EMAIL");
        let handshake = CrossShellHandshake { dir: std::path::PathBuf::from(live_env("SEMIO_CROSS_SHELL_DIR")) };
        let modules = std::path::PathBuf::from(live_env("SEMIO_PLUGIN_MODULES"));
        let variant = live_env("SEMIO_PLUGIN");
        let mut ledger = CollaborationLedger::default();
        let plugins = drive(semio_framework_os_renderer_wgpu_program_bridge::load_wasm_plugins(&variant, &modules)).expect("the staged native runtime loads");
        let mut a = ShellState::new(plugins, variant, semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);

        let retry = sign_in_live(&mut a, &origin, &a_email, &a_password);
        let a_user = a.directory.hub_workspace.session.user_id.clone();
        ledger.record("1-a-signs-in", a.directory.hub_workspace.session.phase == HubSessionPhase::SignedIn, format!("A={} {a_user:?} retried-after={retry:?}", a.directory.hub_workspace.session.phase.as_str()));

        let space_name = format!("wg8 cross-shell {}", chrome_now_ms() as u64);
        hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_SPACE_NAME, &[("value", space_name.as_str())]);
        hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::CREATE_SPACE, &[]);
        let space_id = a.directory.hub_workspace.rows.iter().find(|row| row.name == space_name).map(|row| row.id.clone()).unwrap_or_default();
        shell_command(&mut a, "os.directory.upsert-member", &[("spaceId", space_id.as_str()), ("email", b_email.as_str()), ("role", "author")]);
        hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::OPEN_SPACE, &[("spaceId", space_id.as_str())]);
        let kind_id = a.directory.hub_workspace.creation.catalog.as_ref().and_then(|catalog| catalog.kinds.iter().find(|kind| kind.schema == schema)).map(|kind| kind.kind_id.clone()).unwrap_or_default();
        hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SELECT_ARTIFACT_KIND, &[("kindId", kind_id.as_str())]);
        hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_ARTIFACT_NAME, &[("value", "Cross-shell board")]);
        hub_verb(&mut a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::CREATE_ARTIFACT, &[]);
        let creation_started = std::time::Instant::now();
        let mut creation_trail = Vec::new();
        while creation_started.elapsed() < std::time::Duration::from_secs(600) && !a.directory.hub_workspace.creation.operation.as_ref().is_some_and(|operation| operation.opening != HubArtifactOpening::Idle && operation.opening != HubArtifactOpening::Opening)
        {
            let observed = a.directory.hub_workspace.creation.operation.as_ref().map(|operation| (semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::hub_artifact_creation_phase_str(operation.phase), operation.opening));
            if creation_trail.last() != Some(&observed) {
                creation_trail.push(observed);
            }
            let _ = frame_pump(&mut a);
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
        let a_open = settle_document_opening(&mut a);
        println!(
            "cross-shell native creation catalog={} trail={creation_trail:?} after={:?} open={a_open:?} phase={:?} error={:?}",
            a.directory.hub_workspace.creation.catalog_phase.as_str(),
            creation_started.elapsed(),
            a.opening.document_opening.as_ref().map(|opening| &opening.phase),
            a.chrome.error
        );
        let document_id = a.directory.hub_workspace.creation.operation.as_ref().and_then(|operation| operation.ready.as_ref()).map(|ready| ready.artifact_id.clone()).unwrap_or_default();
        let live_after = pump_until(&mut a, std::time::Duration::from_secs(30), is_live);
        ledger.record(
            "2-a-creates-and-opens",
            !space_id.is_empty() && !document_id.is_empty() && live_after.is_some(),
            format!("space={space_id} schema={schema} kind={kind_id:?} document={document_id} opening={:?} live-after={live_after:?} remote={}", a.directory.hub_workspace.creation.operation.as_ref().map(|operation| operation.opening), remote_of(&a)),
        );
        let boards = paint_session_windows(&mut a, Rect::new(0.0, 40.0, 1200.0, 760.0));
        let a_actor = a.directory.presence_self.as_ref().map(|(actor, _)| actor.clone()).unwrap_or_default();
        handshake.publish("open", serde_json::json!({ "spaceId": space_id, "documentId": document_id, "schema": schema, "actor": a_actor, "userId": a_user, "boards": boards, "live": live_after.is_some() }));

        let react_open = handshake.await_react(&mut a, "open", std::time::Duration::from_secs(600));
        ledger.record("3-b-opens-in-react", react_open.as_ref().is_some_and(|value| value["live"] == true), format!("{react_open:?}"));

        let is_b = |peer: &PresencePeer, a_actor: &str| peer.actor != a_actor && peer.user_id.is_some() && peer.user_id != a_user;
        let presence_after = pump_until(&mut a, std::time::Duration::from_secs(30), |shell| shell.directory.presence_peers.iter().any(|peer| is_b(peer, &a_actor)));
        let b_peer = a.directory.presence_peers.iter().find(|peer| is_b(peer, &a_actor)).cloned();
        let b_actor = b_peer.as_ref().map(|peer| peer.actor.clone()).unwrap_or_default();
        handshake.publish("presence", serde_json::json!({ "seesReact": b_peer.is_some(), "reactActor": b_actor, "peers": a.directory.presence_peers.iter().map(|peer| serde_json::json!({ "actor": peer.actor, "userId": peer.user_id, "label": peer.label, "color": peer.color, "views": peer.views.len() })).collect::<Vec<_>>() }));
        let react_presence = handshake.await_react(&mut a, "presence", std::time::Duration::from_secs(120));
        ledger.record(
            "4-presence-both-ways",
            presence_after.is_some() && react_presence.as_ref().is_some_and(|value| value["seesNative"] == true),
            format!("A sees B after {presence_after:?} as {:?}; B sees A: {react_presence:?}", b_peer.as_ref().map(|peer| (&peer.actor, &peer.label, peer.color))),
        );
        Self { a, handshake, ledger, b_actor, boards }
    }

    /// 🏁️ Tells the driver the journey ended, signs A out and asserts every recorded step.
    fn finish(mut self) {
        let failed: Vec<&str> = self.ledger.steps.iter().filter(|(_, passed, _)| !passed).map(|(step, _, _)| *step).collect();
        self.handshake.publish("done", serde_json::json!({ "failed": failed }));
        hub_verb(&mut self.a, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SIGN_OUT, &[]);
        assert!(failed.is_empty(), "cross-shell steps failed: {failed:?}");
    }
}
