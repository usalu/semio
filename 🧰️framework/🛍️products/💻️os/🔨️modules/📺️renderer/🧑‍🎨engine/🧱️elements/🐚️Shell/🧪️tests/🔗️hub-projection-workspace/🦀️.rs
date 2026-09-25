use super::*;
use serde::Deserialize;

fn shell() -> ShellState {
    ShellState::new(Vec::new(), String::new())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HubProjectionFixture {
    cases: Vec<HubProjectionCase>,
}

#[derive(Deserialize)]
struct HubProjectionCase {
    id: String,
    projection: ShellHubProjectionV1,
    expected: HubProjectionExpected,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HubProjectionExpected {
    state: String,
    peer_count: usize,
    document_count: usize,
}

fn state_name(state: ShellHubConnectionState) -> &'static str {
    match state {
        ShellHubConnectionState::SignedOut => "signedOut",
        ShellHubConnectionState::Live(_) => "live",
        ShellHubConnectionState::Connecting => "connecting",
        ShellHubConnectionState::Reconnecting => "reconnecting",
        ShellHubConnectionState::Offline => "offline",
    }
}

#[test]
fn hub_command_opens_the_route_overlay_without_adding_a_default_dock_tab() {
    let mut shell = shell();
    let dock = shell.default_dock();
    assert!(!PanelAnchor::ALL.into_iter().flat_map(|anchor| dock.tabs(anchor)).any(|tab| tab.id == FRAMEWORK_HUB_PANEL_ID));
    semio_framework_async::block_on(shell.apply_os_command("os.openHub", None)).expect("Hub command opens the workspace");
    assert!(shell.hub_workspace_open);
    assert_eq!(shell.uri_history.get(shell.uri_index).map(String::as_str), Some("/hub"));
    assert!(shell.shell_owned_panel_leaves().iter().any(|id| id == FRAMEWORK_HUB_PANEL_ID), "the overlay keeps its retained body publication");
    semio_framework_async::block_on(shell.handle_hub_workspace_action(crate::hub_connection::action::CLOSE_WORKSPACE, None));
    assert!(!shell.hub_workspace_open);
}

#[test]
fn neutral_authority_and_multi_document_vectors_fold_to_the_shared_summary() {
    let fixture: HubProjectionFixture = serde_json::from_str(include_str!("../../../../🧫️fixtures/🔗️hub-projection/🔣️.json")).expect("hub projection fixture");
    for case in fixture.cases {
        let summary = shell_hub_connection_summary_v1(&case.projection);
        assert_eq!(state_name(summary.state), case.expected.state, "{} state", case.id);
        assert_eq!(summary.peer_count, case.expected.peer_count, "{} peers", case.id);
        assert_eq!(summary.document_count, case.expected.document_count, "{} documents", case.id);
        if let ShellHubConnectionState::Live(count) = summary.state {
            assert_eq!(count, summary.peer_count, "{} live state and summary disagree", case.id);
        }
    }
}

#[test]
fn status_replacement_and_document_retirement_share_one_projection() {
    let mut shell = shell();
    shell.publish_hub_document_status("hub:a/one", ShellHubRemoteV1::Connecting);
    shell.publish_hub_document_status("hub:a/one", ShellHubRemoteV1::Live { peer_count: 4 });
    shell.publish_hub_document_status("hub:b/two", ShellHubRemoteV1::Backoff { retry_in_ms: 1000 });
    assert_eq!(shell.hub_documents.len(), 2);
    let projection = ShellHubProjectionV1 {
        authority: ShellHubAuthorityV1::VerifiedSession { authorization_generation: 1 },
        documents: shell.hub_documents.iter().map(|(document_key, remote)| ShellHubDocumentV1 { document_key: document_key.clone(), remote: remote.clone() }).collect(),
    };
    let summary = shell_hub_connection_summary_v1(&projection);
    assert_eq!(summary.state, ShellHubConnectionState::Live(4));
    shell.retire_hub_document_status("hub:a/one");
    assert_eq!(shell.hub_documents.len(), 1);
    shell.retire_hub_document_status("hub:b/two");
    assert!(shell.hub_documents.is_empty());
}

#[test]
fn document_projection_refuses_a_sixty_fifth_distinct_owner_but_allows_replacement() {
    let mut shell = shell();
    for index in 0..64 {
        shell.publish_hub_document_status(format!("hub:a/{index}"), ShellHubRemoteV1::Detached);
    }
    shell.publish_hub_document_status("hub:a/64", ShellHubRemoteV1::Connecting);
    assert_eq!(shell.hub_documents.len(), 64);
    assert!(!shell.hub_documents.contains_key("hub:a/64"));
    shell.publish_hub_document_status("hub:a/0", ShellHubRemoteV1::Live { peer_count: 2 });
    assert_eq!(shell.hub_documents.get("hub:a/0"), Some(&ShellHubRemoteV1::Live { peer_count: 2 }));
}

/// 🔐️ One hub workspace verb, driven the way a frame drives it natively ([`drive`], which also pumps the
/// renderer I/O and worker-retirement slots). A bare `block_on` parked for ever inside a verb after a
/// live law had opened a document: sampled on two-user gate run 17, the test thread sat in
/// `block_on(handle_hub_workspace_action)` for 23 min, and eight earlier runs never exited.
fn hub_verb(shell: &mut ShellState, verb: &str, args: &[(&str, &str)]) {
    let args = (!args.is_empty()).then(|| DslValue::Object(args.iter().map(|(key, value)| ((*key).to_string(), DslValue::String((*value).to_string()))).collect()));
    #[cfg(not(target_arch = "wasm32"))]
    drive(shell.handle_hub_workspace_action(verb, args));
    #[cfg(target_arch = "wasm32")]
    semio_framework_async::block_on(shell.handle_hub_workspace_action(verb, args));
}

fn hub_attribute_values(node: &UiNode, attribute: &str, found: &mut Vec<String>) {
    match node {
        UiNode::Stack(stack) => stack.children.iter().for_each(|child| hub_attribute_values(child, attribute, found)),
        UiNode::Text(text) => found.extend(text.data_attributes.as_ref().and_then(|attributes| attributes.get(attribute)).cloned()),
        _ => {}
    }
}

/// 🤝️ The wgpu shell's whole hub journey against a REAL hub, through the shell's own lane and its
/// own native `DirectoryTransport` — no stub anywhere: `/hub` opens the workspace, a typed origin
/// becomes the selected connection, a credential sign-in mints a session, a sealed `create-space`
/// command lands, and the authoritative space list reaches the retained `UiNode` tree the
/// accessibility mirror projects (`data-semio-hub-space`).
///
/// 🔌️ `#[ignore]`d because it needs a live hub with credential sign-in enabled; boot one with the
/// `os-hub:live-sign-in-check` recipe (`OS_HUB_CREDENTIAL_SIGN_IN=1 bun 🌎️hub/🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts --hold`)
/// and run with `SEMIO_HUB_LIVE_ORIGIN`, `SEMIO_HUB_LIVE_EMAIL`, `SEMIO_HUB_LIVE_PASSWORD` set and
/// `-- --ignored`.
#[test]
#[ignore = "needs a live hub at SEMIO_HUB_LIVE_ORIGIN; see this test's own doc comment"]
fn a_live_hub_signs_in_and_its_spaces_reach_the_retained_workspace() {
    let origin = std::env::var("SEMIO_HUB_LIVE_ORIGIN").expect("SEMIO_HUB_LIVE_ORIGIN");
    let email = std::env::var("SEMIO_HUB_LIVE_EMAIL").expect("SEMIO_HUB_LIVE_EMAIL");
    let password = std::env::var("SEMIO_HUB_LIVE_PASSWORD").expect("SEMIO_HUB_LIVE_PASSWORD");
    let space_name = format!("wg6 live {}", chrome_now_ms() as u64);
    let mut shell = shell();
    semio_framework_async::block_on(shell.apply_os_command("os.openHub", None)).expect("the hub route opens");
    assert!(shell.hub_workspace_open);
    assert_eq!(shell.hub_workspace.presence(), HubSessionPresence::SignedOut);
    assert_eq!(shell.hub_connection_state(), ShellHubConnectionState::SignedOut);
    hub_verb(&mut shell, crate::hub_connection::action::SET_ADDRESS, &[("value", origin.as_str())]);
    hub_verb(&mut shell, crate::hub_connection::action::ADD_CONNECTION, &[]);
    assert_eq!(shell.hub_workspace.origin(), origin.trim_end_matches('/'));
    hub_verb(&mut shell, crate::hub_connection::action::SET_EMAIL, &[("value", email.as_str())]);
    hub_verb(&mut shell, crate::hub_connection::action::SET_PASSWORD, &[("value", password.as_str())]);
    hub_verb(&mut shell, crate::hub_connection::action::SIGN_IN, &[]);
    println!("wg6-live sign-in phase={} error={:?} user={:?} display={:?}", shell.hub_workspace.session.phase.as_str(), shell.hub_workspace.session.error, shell.hub_workspace.session.user_id, shell.hub_workspace.display_name);
    assert_eq!(shell.hub_workspace.session.phase, HubSessionPhase::SignedIn, "error {:?}", shell.hub_workspace.session.error);
    assert!(shell.hub_workspace.password_draft.is_empty(), "the password never outlives its request");
    assert!(shell.hub_workspace.display_name.is_some());
    assert_ne!(shell.hub_connection_state(), ShellHubConnectionState::SignedOut);
    hub_verb(&mut shell, crate::hub_connection::action::SET_SPACE_NAME, &[("value", space_name.as_str())]);
    hub_verb(&mut shell, crate::hub_connection::action::CREATE_SPACE, &[]);
    println!("wg6-live spaces phase={} rows={:?}", shell.hub_workspace.phase.as_str(), shell.hub_workspace.rows.iter().map(|row| (row.name.as_str(), row.id.as_str(), row.access.as_str())).collect::<Vec<_>>());
    assert_eq!(shell.hub_workspace.phase, crate::space_browser::SpaceBrowserPhase::Ready);
    let created = shell.hub_workspace.rows.iter().find(|row| row.name == space_name).expect("the created space is listed").id.clone();
    for locale in [Locale::En, Locale::De] {
        let tree = crate::hub_connection::build_hub_workspace_ui(&shell.hub_workspace, locale);
        let mut spaces = Vec::new();
        hub_attribute_values(&tree, "data-semio-hub-space", &mut spaces);
        let mut phases = Vec::new();
        hub_attribute_values(&tree, "data-semio-hub-phase", &mut phases);
        println!("wg6-live tree locale={locale:?} phase={phases:?} spaces={spaces:?}");
        assert!(spaces.contains(&created), "{locale:?} tree lists the created space");
        assert_eq!(phases, vec![HubSessionPhase::SignedIn.as_str().to_string()]);
    }
    hub_verb(&mut shell, crate::hub_connection::action::OPEN_SPACE, &[("spaceId", created.as_str())]);
    println!("wg6-live open space={:?} members={:?} uri={:?}", shell.hub_workspace.open_space_id, shell.hub_workspace.members.iter().map(|member| (member.display_name.as_str(), member.owner)).collect::<Vec<_>>(), shell.uri_history.get(shell.uri_index));
    assert_eq!(shell.hub_workspace.open_space_id.as_deref(), Some(created.as_str()));
    assert!(shell.hub_workspace.members.iter().any(|member| member.owner), "the creator is listed as the owner");
    hub_verb(&mut shell, crate::hub_connection::action::SIGN_OUT, &[]);
    assert_eq!(shell.hub_workspace.presence(), HubSessionPresence::SignedOut);
    assert!(shell.hub_workspace.rows.is_empty());
}

/// ✂️ A loopback TCP relay in front of the hub whose every live connection can be cut at once,
/// then healed: the second actor reaches the hub ONLY through it, so severing it is a real
/// network loss on that actor's sockets — the document socket and the directory transport alike —
/// while the first actor's own connections stay untouched.
#[cfg(not(target_arch = "wasm32"))]
struct SeverableRelay {
    origin: String,
    live: std::sync::Arc<std::sync::Mutex<Vec<std::net::TcpStream>>>,
    open: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SeverableRelay {
    fn start(upstream_origin: &str) -> Self {
        let upstream = upstream_origin.trim_start_matches("http://").trim_end_matches('/').to_string();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("relay binds a loopback port");
        let origin = format!("http://{}", listener.local_addr().expect("relay address"));
        let live = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let open = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
        let (accepted, admitting) = (live.clone(), open.clone());
        std::thread::spawn(move || {
            for inbound in listener.incoming().flatten() {
                let outbound = match std::net::TcpStream::connect(&upstream) {
                    Ok(outbound) if admitting.load(std::sync::atomic::Ordering::SeqCst) => outbound,
                    _ => {
                        let _ = inbound.shutdown(std::net::Shutdown::Both);
                        continue;
                    }
                };
                let mut retained = accepted.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                retained.extend([inbound.try_clone(), outbound.try_clone()].into_iter().flatten());
                drop(retained);
                let pipe = |mut from: std::net::TcpStream, mut to: std::net::TcpStream| {
                    std::thread::spawn(move || {
                        let _ = std::io::copy(&mut from, &mut to);
                        let _ = to.shutdown(std::net::Shutdown::Write);
                    });
                };
                if let (Ok(from), Ok(to)) = (inbound.try_clone(), outbound.try_clone()) {
                    pipe(from, to);
                }
                pipe(outbound, inbound);
            }
        });
        Self { origin, live, open }
    }

    fn sever(&self) -> usize {
        self.open.store(false, std::sync::atomic::Ordering::SeqCst);
        let streams = std::mem::take(&mut *self.live.lock().unwrap_or_else(std::sync::PoisonError::into_inner));
        streams.iter().for_each(|stream| {
            let _ = stream.shutdown(std::net::Shutdown::Both);
        });
        streams.len()
    }

    fn heal(&self) {
        self.open.store(true, std::sync::atomic::Ordering::SeqCst);
    }
}

/// 📒️ One named step of the two-user journey and what the shells' own state showed for it.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct CollaborationLedger {
    steps: Vec<(&'static str, bool, String)>,
}

#[cfg(not(target_arch = "wasm32"))]
impl CollaborationLedger {
    fn record(&mut self, step: &'static str, passed: bool, detail: String) -> bool {
        println!("g7w-live step={step} {} {detail}", if passed { "PASS" } else { "FAIL" });
        self.steps.push((step, passed, detail));
        passed
    }
}

/// 🔂️ Polls one shell future to completion while pumping what the GPU present loop pumps in the
/// real binary (`present_step_inner`): retained renderer I/O sessions and worker-job retirements.
/// Without it a headless caller parks forever on the first `run_renderer_io` (the native runtime
/// manifest read inside `load_wasm_plugins`), because nothing else drives those slots.
#[cfg(not(target_arch = "wasm32"))]
fn drive<F: std::future::Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        if let std::task::Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
        let advanced = crate::pump_renderer_io_sessions(1);
        let _ = semio_framework_job::pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        if advanced == 0 {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn live_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{key} is required; see the two-user law's doc comment"))
}

/// 🔑️ Signs one shell in through its own hub lane, clicking Sign in a second time exactly like a
/// human would when the first attempt failed (a debug hub's password hash can outlast the lane's
/// 5 s command deadline under load); answers the first attempt's refusal, if any.
#[cfg(not(target_arch = "wasm32"))]
fn sign_in_live(shell: &mut ShellState, origin: &str, email: &str, password: &str) -> Option<String> {
    drive(shell.apply_os_command("os.openHub", None)).expect("the hub route opens");
    hub_verb(shell, crate::hub_connection::action::SET_ADDRESS, &[("value", origin)]);
    hub_verb(shell, crate::hub_connection::action::ADD_CONNECTION, &[]);
    let mut first_refusal = None;
    for _ in 0..2 {
        hub_verb(shell, crate::hub_connection::action::SET_EMAIL, &[("value", email)]);
        hub_verb(shell, crate::hub_connection::action::SET_PASSWORD, &[("value", password)]);
        hub_verb(shell, crate::hub_connection::action::SIGN_IN, &[]);
        if shell.hub_workspace.session.phase == HubSessionPhase::SignedIn {
            break;
        }
        first_refusal.get_or_insert_with(|| format!("{:?}", shell.hub_workspace.session.error));
    }
    first_refusal
}

#[cfg(not(target_arch = "wasm32"))]
fn shell_command(shell: &mut ShellState, action_id: &str, args: &[(&str, &str)]) {
    let args = DslValue::Object(args.iter().map(|(key, value)| ((*key).to_string(), DslValue::String((*value).to_string()))).collect());
    drive(shell.handle_replay_shell_command(action_id, Some(&args)));
}

#[cfg(not(target_arch = "wasm32"))]
fn remote_of(shell: &ShellState) -> String {
    shell.sync_status.as_ref().map_or_else(|| "none".to_string(), |status| format!("{:?}", status.remote))
}

#[cfg(not(target_arch = "wasm32"))]
fn is_live(shell: &ShellState) -> bool {
    matches!(shell.sync_status.as_ref().map(|status| &status.remote), Some(RemoteState::Live { .. }))
}

/// 🖼️ What one painted frame does for an open document, headless, answering how many renderer I/O
/// sessions it advanced: the present loop's renderer I/O and
/// worker-retirement slots (`present_step_inner`; a detached request such as the open's app instance
/// waits on them), the frame pump (`pump_sync_events`: directory lane, auto check-in, the frame-pumped
/// open, document actor events), the chrome walk's presence phase, which arms the maintenance lane's
/// heartbeat step (`ShellChromeFramePhase::Presence` → `advance_presence_preview_step`), and one settle
/// step when the settle lane is owed one. Without the presence half a headless shell never beat, so the
/// hub saw no peer and dropped each idle document socket after its presence lease (two-user gate run 13).
#[cfg(not(target_arch = "wasm32"))]
fn frame_pump(shell: &mut ShellState) -> usize {
    let advanced = crate::pump_renderer_io_sessions(1);
    let _ = semio_framework_job::pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    drive(shell.pump_sync_events());
    shell.request_presence_preview();
    if shell.chrome_present.maintenance.presence_requested {
        shell.advance_presence_preview_step();
    }
    if shell.settle_pump_pending() {
        drive(shell.settle_pump_step());
    }
    advanced
}

/// ⏱️ How one frame-pumped open went, frame by frame: the frames while its steps were out and the
/// frames that then rendered what it owed, each with its count and its longest single frame.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Default)]
struct OpenFrames {
    opening: usize,
    opening_longest: std::time::Duration,
    rendering: usize,
    rendering_longest: std::time::Duration,
    elapsed: std::time::Duration,
}

/// 🚪️ Pumps frames ([`frame_pump`]) until the shell's frame-pumped document open settles and the
/// settle lane has rendered what it owed, timing every frame. Like the present loop, it idles only when
/// no renderer I/O session advanced. A settled failure or cancellation is left for the caller to assert.
#[cfg(not(target_arch = "wasm32"))]
fn settle_document_opening(shell: &mut ShellState) -> OpenFrames {
    let started = std::time::Instant::now();
    let mut frames = OpenFrames::default();
    while shell.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) && started.elapsed() < std::time::Duration::from_secs(180) {
        let frame = std::time::Instant::now();
        let advanced = frame_pump(shell);
        frames.opening += 1;
        frames.opening_longest = frames.opening_longest.max(frame.elapsed());
        if advanced == 0 {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    while shell.settle_pump_pending() && started.elapsed() < std::time::Duration::from_secs(180) {
        let frame = std::time::Instant::now();
        let _ = frame_pump(shell);
        frames.rendering += 1;
        frames.rendering_longest = frames.rendering_longest.max(frame.elapsed());
    }
    frames.elapsed = started.elapsed();
    frames
}

/// 🔁️ Drives both shells' frames ([`frame_pump`]) until `done` holds or the budget ends, recording
/// every distinct remote state either shell passed through.
#[cfg(not(target_arch = "wasm32"))]
fn pump_pair(a: &mut ShellState, b: &mut ShellState, budget: std::time::Duration, trail: &mut Vec<String>, done: impl Fn(&ShellState, &ShellState) -> bool) -> Option<std::time::Duration> {
    let started = std::time::Instant::now();
    while started.elapsed() < budget {
        let _ = frame_pump(a);
        let _ = frame_pump(b);
        let observed = format!("A={} B={}", remote_of(a), remote_of(b));
        if trail.last() != Some(&observed) {
            trail.push(observed);
        }
        if done(a, b) {
            return Some(started.elapsed());
        }
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    None
}

#[cfg(not(target_arch = "wasm32"))]
fn applied_edits(shell: &mut ShellState) -> Vec<(String, bool)> {
    drive(shell.refresh_history_snapshot());
    shell.history_entries.values().filter(|entry| entry.kind == "mutation").map(|entry| (entry.action_id.clone(), entry.applied)).collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn author_edit(shell: &mut ShellState, action: &str) -> (Result<(), String>, std::time::Duration) {
    let controller_id = shell.session.as_ref().map(|session| session.app.controller_id.clone()).unwrap_or_default();
    let started = std::time::Instant::now();
    let outcome = drive(shell.dispatch_action(ActionDescriptor { controller_id, action: action.to_string(), args: None }));
    (outcome, started.elapsed())
}

#[cfg(not(target_arch = "wasm32"))]
fn online_members(shell: &mut ShellState, space_id: &str) -> Vec<(String, bool)> {
    drive(shell.reload_hub_members(space_id));
    shell.hub_workspace.members.iter().map(|member| (member.display_name.clone(), member.online)).collect()
}

/// 🤝️ Two native wgpu shells, two hub users, one hub document — the whole collaboration journey
/// through each shell's own lanes and state, no browser and no stub: both sign in, the first
/// creates a space and seats the second as an author, both open the space and the same block2d
/// document through the `os.open-artifact` relay (the guest runs natively on the kernel thread,
/// the document actor dials the hub's document socket), presence must show both, each actor's
/// edit must reach the other's ledger, undo is per actor, and severing the second actor's network
/// must neither freeze its shell nor lose its offline edit. "Not frozen" is measured against the same
/// shell's own online edit (at most twice its latency), so the bound holds for a debug interpreter too.
///
/// 📒️ Every step is recorded (PASS/FAIL + what the shells showed) before the law asserts the
/// whole ledger, so one run is the per-step table even when an early step fails.
///
/// 🔌️ `#[ignore]`d: needs a live credential-sign-in hub at `SEMIO_HUB_LIVE_ORIGIN` with the two
/// principals `SEMIO_HUB_LIVE_EMAIL`/`_PASSWORD` and `SEMIO_HUB_LIVE_PEER_EMAIL`/`_PASSWORD`, plus
/// a native runtime staged for `block2d` at `SEMIO_PLUGIN_MODULES`; the renderer package's
/// `hub-live-collaboration-check` verb provides all of it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
#[ignore = "needs a live hub, two principals and a staged native block2d runtime; see this test's own doc comment"]
fn two_live_wgpu_shells_collaborate_on_one_hub_document() {
    let journey = native_guest_journey();
    let origin = live_env("SEMIO_HUB_LIVE_ORIGIN");
    let (a_email, a_password) = (live_env("SEMIO_HUB_LIVE_EMAIL"), live_env("SEMIO_HUB_LIVE_PASSWORD"));
    let (b_email, b_password) = (live_env("SEMIO_HUB_LIVE_PEER_EMAIL"), live_env("SEMIO_HUB_LIVE_PEER_PASSWORD"));
    let modules = std::path::PathBuf::from(live_env("SEMIO_PLUGIN_MODULES"));
    let variant = live_env("SEMIO_PLUGIN");
    let relay = SeverableRelay::start(&origin);
    let mut ledger = CollaborationLedger::default();
    let plugins = drive(crate::program_bridge::load_wasm_plugins(&variant, &modules)).expect("the staged native runtime loads");
    assert!(plugins.iter().any(|entry| entry.plugin_id == journey.plugin_id.as_str()), "the staged runtime carries {}", journey.plugin_id);
    let mut a = ShellState::new(plugins.clone(), variant.clone());
    let mut b = ShellState::new(plugins, variant);

    let a_retry = sign_in_live(&mut a, &origin, &a_email, &a_password);
    let b_retry = sign_in_live(&mut b, &relay.origin, &b_email, &b_password);
    let (a_user, b_user) = (a.hub_workspace.session.user_id.clone(), b.hub_workspace.session.user_id.clone());
    ledger.record(
        "1-sign-in",
        a.hub_workspace.session.phase == HubSessionPhase::SignedIn && b.hub_workspace.session.phase == HubSessionPhase::SignedIn && a_user.is_some() && a_user != b_user,
        format!(
            "A={} {:?} {:?} error={:?} retried-after={a_retry:?} B={} {:?} {:?} error={:?} retried-after={b_retry:?} (B via relay {})",
            a.hub_workspace.session.phase.as_str(),
            a_user,
            a.hub_workspace.display_name,
            a.hub_workspace.session.error,
            b.hub_workspace.session.phase.as_str(),
            b_user,
            b.hub_workspace.display_name,
            b.hub_workspace.session.error,
            relay.origin
        ),
    );

    let space_name = format!("g7w collaboration {}", chrome_now_ms() as u64);
    hub_verb(&mut a, crate::hub_connection::action::SET_SPACE_NAME, &[("value", space_name.as_str())]);
    hub_verb(&mut a, crate::hub_connection::action::CREATE_SPACE, &[]);
    let space_id = a.hub_workspace.rows.iter().find(|row| row.name == space_name).map(|row| row.id.clone()).unwrap_or_default();
    shell_command(&mut a, "os.directory.upsert-member", &[("spaceId", space_id.as_str()), ("email", b_email.as_str()), ("role", "author")]);
    hub_verb(&mut b, crate::hub_connection::action::REFRESH_SPACES, &[]);
    let b_row = b.hub_workspace.rows.iter().find(|row| row.id == space_id).map(|row| (row.name.clone(), row.role));
    ledger.record("2-same-space", !space_id.is_empty() && b_row.as_ref().is_some_and(|(_, role)| *role == Some(DirectorySpaceRole::Author)), format!("space={space_id} B sees {b_row:?}"));

    hub_verb(&mut a, crate::hub_connection::action::OPEN_SPACE, &[("spaceId", space_id.as_str())]);
    hub_verb(&mut b, crate::hub_connection::action::OPEN_SPACE, &[("spaceId", space_id.as_str())]);
    let (a_roster, b_roster) = (online_members(&mut a, &space_id), online_members(&mut b, &space_id));
    ledger.record("3-roster", a_roster.len() == 2 && b_roster.len() == 2, format!("A roster={a_roster:?} B roster={b_roster:?}"));

    let kind_id = a.hub_workspace.creation.catalog.as_ref().and_then(|catalog| catalog.kinds.iter().find(|kind| kind.schema == journey.schema.as_str())).map(|kind| kind.kind_id.clone()).unwrap_or_default();
    hub_verb(&mut a, crate::hub_connection::action::SELECT_ARTIFACT_KIND, &[("kindId", kind_id.as_str())]);
    hub_verb(&mut a, crate::hub_connection::action::SET_ARTIFACT_NAME, &[("value", "Shared board")]);
    hub_verb(&mut a, crate::hub_connection::action::CREATE_ARTIFACT, &[]);
    let creation_started = std::time::Instant::now();
    let mut creation_trail = Vec::new();
    while creation_started.elapsed() < std::time::Duration::from_secs(150) {
        let observed = a.hub_workspace.creation.operation.as_ref().map(|operation| (crate::hub_connection::hub_artifact_creation_phase_str(operation.phase), operation.opening));
        if creation_trail.last() != Some(&observed) {
            creation_trail.push(observed);
        }
        if a.hub_workspace.creation.operation.as_ref().is_some_and(|operation| operation.opening != HubArtifactOpening::Idle && operation.opening != HubArtifactOpening::Opening) {
            break;
        }
        drive(a.pump_sync_events());
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    let created = a.hub_workspace.creation.operation.as_ref().and_then(|operation| operation.ready.clone());
    ledger.record(
        "4a-create-through-the-door",
        created.is_some() && a.hub_workspace.creation.operation.as_ref().is_some_and(|operation| operation.opening == HubArtifactOpening::Opened),
        format!("catalog={} kind={kind_id:?} trail={creation_trail:?} created={created:?} after={:?}", a.hub_workspace.creation.catalog_phase.as_str(), creation_started.elapsed()),
    );
    let document_id = created.as_ref().map(|ready| ready.artifact_id.clone()).unwrap_or_default();
    let open_args = [("artifactRef", journey.artifact_ref.as_str()), ("pluginId", journey.plugin_id.as_str()), ("appId", journey.app_id.as_str()), ("documentId", document_id.as_str()), ("schema", journey.schema.as_str()), ("spaceId", space_id.as_str())];
    shell_command(&mut b, "os.open-artifact", &open_args);
    let b_open = settle_document_opening(&mut b);
    println!("g7w-live B open frames={b_open:?}");
    let session_of = |shell: &ShellState| shell.session.as_ref().map(|session| (session.plugin_id.clone(), session.app.id.clone(), session.instance_id));
    ledger.record(
        "4-guest-mounted",
        [&a, &b].iter().all(|shell| shell.session.as_ref().is_some_and(|session| session.plugin_id == journey.plugin_id.as_str() && session.app.id == journey.app_id.as_str())),
        format!("A session={:?} error={:?} B session={:?} error={:?}", session_of(&a), a.error, session_of(&b), b.error),
    );
    ledger.record(
        "5-hub-binding",
        a.sync_channel.is_some() && b.sync_channel.is_some() && a.presence_surface.is_some() && b.presence_surface.is_some(),
        format!("A uri={:?} surface={:?} B uri={:?} surface={:?}", a.sync_backbone_uri, a.presence_surface, b.sync_backbone_uri, b.presence_surface),
    );

    let mut trail = Vec::new();
    let live_after = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(30), &mut trail, |a, b| is_live(a) && is_live(b));
    let codec = match drive(store_sync::os_store::document_kind_codec(journey.schema.as_str())) {
        Ok(Some(codec)) => drive(codec.pack_schema_hash()).ok(),
        _ => None,
    };
    let socket_live = ledger.record("6-document-socket-live", live_after.is_some(), format!("after={live_after:?} trail={trail:?} native codec for {}={codec:?} hub_documents A={:?} B={:?}", journey.schema, a.hub_documents, b.hub_documents));

    let presence_after = socket_live.then(|| pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |a, b| a.presence_peers.len() >= 2 && b.presence_peers.len() >= 2)).flatten();
    let (a_online, b_online) = (online_members(&mut a, &space_id), online_members(&mut b, &space_id));
    ledger.record(
        "7-presence-both",
        presence_after.is_some() && a_online.iter().all(|(_, online)| *online) && b_online.iter().all(|(_, online)| *online),
        format!("after={presence_after:?} A peers={} roster={a_online:?} B peers={} roster={b_online:?}", a.presence_peers.len(), b.presence_peers.len()),
    );

    let a_before = applied_edits(&mut a).len();
    let (a_edit, a_latency) = author_edit(&mut a, journey.verb.as_str());
    let a_ledger = applied_edits(&mut a);
    ledger.record("8-a-authors", a_edit.is_ok() && a_ledger.len() == a_before + 1, format!("outcome={a_edit:?} latency={a_latency:?} A ledger {a_before}->{:?}", a_ledger));

    let b_before = applied_edits(&mut b).len();
    let b_ingest = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |_, _| false);
    let b_seen = applied_edits(&mut b);
    ledger.record("9-b-ingests", socket_live && b_seen.len() > b_before, format!("pumped={b_ingest:?} B ledger {b_before}->{b_seen:?} B remote={}", remote_of(&b)));

    let a_seen_before = applied_edits(&mut a).len();
    let (b_edit, b_latency) = author_edit(&mut b, journey.verb.as_str());
    let _ = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |_, _| false);
    let a_seen = applied_edits(&mut a);
    ledger.record("10-b-authors-a-ingests", b_edit.is_ok() && socket_live && a_seen.len() > a_seen_before, format!("B outcome={b_edit:?} latency={b_latency:?} A ledger {a_seen_before}->{a_seen:?}"));

    let (undo, _) = author_edit(&mut a, journey.undo.as_str());
    let _ = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |_, _| false);
    let (a_after_undo, b_after_undo) = (applied_edits(&mut a), applied_edits(&mut b));
    let a_own_reverted = a_after_undo.iter().filter(|(action, applied)| action == journey.verb.as_str() && !applied).count() == 1;
    ledger.record("11-per-actor-undo", undo.is_ok() && a_own_reverted && socket_live, format!("undo={undo:?} A ledger={a_after_undo:?} B ledger={b_after_undo:?}"));

    let severed = relay.sever();
    let (offline_edit, offline_latency) = author_edit(&mut b, journey.verb.as_str());
    let offline_pump = std::time::Instant::now();
    let _ = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(3), &mut trail, |_, _| false);
    let pump_elapsed = offline_pump.elapsed();
    hub_verb(&mut b, crate::hub_connection::action::REFRESH_SPACES, &[]);
    let offline_phase = b.hub_workspace.phase.as_str();
    let offline_remote = remote_of(&b);
    relay.heal();
    hub_verb(&mut b, crate::hub_connection::action::REFRESH_SPACES, &[]);
    let healed_phase = b.hub_workspace.phase.as_str();
    let a_before_heal = applied_edits(&mut a).len();
    let relive = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(40), &mut trail, |a, b| is_live(a) && is_live(b));
    let _ = pump_pair(&mut a, &mut b, std::time::Duration::from_secs(10), &mut trail, |_, _| false);
    let a_after_heal = applied_edits(&mut a);
    ledger.record(
        "12-connection-loss",
        offline_edit.is_ok() && offline_latency <= b_latency.saturating_mul(2) && pump_elapsed < std::time::Duration::from_secs(5) && offline_phase == crate::space_browser::SpaceBrowserPhase::Stale.as_str() && healed_phase == crate::space_browser::SpaceBrowserPhase::Ready.as_str() && relive.is_some() && a_after_heal.len() > a_before_heal,
        format!("severed={severed} B offline edit={offline_edit:?} latency={offline_latency:?} pump={pump_elapsed:?} B spaces offline={offline_phase} healed={healed_phase} B remote offline={offline_remote} relive={relive:?} A ledger {a_before_heal}->{a_after_heal:?}"),
    );

    println!("g7w-live trail={trail:?}");
    let failed: Vec<&str> = ledger.steps.iter().filter(|(_, passed, _)| !passed).map(|(step, _, _)| *step).collect();
    hub_verb(&mut a, crate::hub_connection::action::SIGN_OUT, &[]);
    hub_verb(&mut b, crate::hub_connection::action::SIGN_OUT, &[]);
    assert!(failed.is_empty(), "two-user collaboration steps failed: {failed:?}");
}

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
    serde_json::from_str(include_str!("../../../../🧫️fixtures/⏯️native-guest-journey/🔣️.json")).expect("native guest journey fixture")
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
    let plugins = drive(crate::program_bridge::load_wasm_plugins(&variant, &modules)).expect("the staged native runtime loads");
    assert!(plugins.iter().any(|entry| entry.plugin_id == journey.plugin_id), "the staged runtime carries {}", journey.plugin_id);
    let mut shell = ShellState::new(plugins, variant);
    let document_id = format!("native-guest-journey-{}", chrome_now_ms() as u64);
    let started = std::time::Instant::now();
    shell_command(&mut shell, "os.open-artifact", &[("artifactRef", journey.artifact_ref.as_str()), ("pluginId", journey.plugin_id.as_str()), ("appId", journey.app_id.as_str()), ("documentId", document_id.as_str()), ("schema", journey.schema.as_str())]);
    let command = started.elapsed();
    let frames = settle_document_opening(&mut shell);
    let session = shell.session.as_ref().map(|session| (session.plugin_id.clone(), session.app.id.clone(), session.instance_id));
    println!("native-guest-journey open command={command:?} frames={frames:?} session={session:?} error={:?}", shell.error);
    assert!(shell.document_opening.is_none(), "the open settled and cleared its band: {:?}", shell.document_opening.as_ref().map(|opening| &opening.phase));
    assert_eq!(session.as_ref().map(|(plugin, app, _)| (plugin.as_str(), app.as_str())), Some((journey.plugin_id.as_str(), journey.app_id.as_str())), "the relay mounted the guest's app");
    assert_eq!(shell.error, None, "every surface the session opened reached the retained registry");
    assert_eq!(shell.sync_channel.as_ref().map(|channel| channel.document_id.as_str()), Some(document_id.as_str()), "the relay bound the session to the document it opened");

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
    let plugins = drive(crate::program_bridge::load_wasm_plugins(&variant, &modules)).expect("the staged native runtime loads");
    let mut shell = ShellState::new(plugins, variant);
    let open = |shell: &mut ShellState, document_id: &str| {
        let started = std::time::Instant::now();
        shell_command(shell, "os.open-artifact", &[("artifactRef", journey.artifact_ref.as_str()), ("pluginId", journey.plugin_id.as_str()), ("appId", journey.app_id.as_str()), ("documentId", document_id), ("schema", journey.schema.as_str())]);
        started.elapsed()
    };

    let command = open(&mut shell, "native-guest-open-cancelled");
    assert_eq!(shell.document_opening.as_ref().map(|opening| opening.phase.clone()), Some(ShellDocumentOpenPhase::Instantiating), "the relay only starts the open");
    shell.cancel_document_opening();
    let cancelled = settle_document_opening(&mut shell);
    println!("native-guest-open cancel command={command:?} frames={cancelled:?} phase={:?}", shell.document_opening.as_ref().map(|opening| &opening.phase));
    assert_eq!(shell.document_opening.as_ref().map(|opening| opening.phase.clone()), Some(ShellDocumentOpenPhase::Cancelled), "the cancel settled the open");
    assert!(shell.session.is_none() && shell.sync_channel.is_none(), "a cancelled open mounts nothing");
    shell.cancel_document_opening();
    assert!(shell.document_opening.is_none(), "closing the settled band clears it");

    let command = open(&mut shell, "native-guest-open-painted");
    let frames = settle_document_opening(&mut shell);
    println!("native-guest-open command={command:?} frames={frames:?}");
    assert!(command < std::time::Duration::from_millis(500), "the relay started the open without a guest turn: {command:?}");
    assert!(frames.opening > 1 && frames.opening_longest < std::time::Duration::from_millis(500), "every frame stayed short while the guest opened: {frames:?}");
    assert_eq!(shell.sync_channel.as_ref().map(|channel| channel.document_id.as_str()), Some("native-guest-open-painted"));
}

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
    assert_eq!(shell.hub_workspace.session.phase, HubSessionPhase::SignedIn, "first refusal {refusal:?}");
    let space_name = format!("wg8 door {}", chrome_now_ms() as u64);
    hub_verb(&mut shell, crate::hub_connection::action::SET_SPACE_NAME, &[("value", space_name.as_str())]);
    hub_verb(&mut shell, crate::hub_connection::action::CREATE_SPACE, &[]);
    let space_id = shell.hub_workspace.rows.iter().find(|row| row.name == space_name).map(|row| row.id.clone()).expect("the created space is listed");
    hub_verb(&mut shell, crate::hub_connection::action::OPEN_SPACE, &[("spaceId", space_id.as_str())]);
    let creation = &shell.hub_workspace.creation;
    let kinds: Vec<(String, String, String)> = creation.catalog.iter().flat_map(|catalog| catalog.kinds.iter()).map(|kind| (kind.kind_id.clone(), kind.label.en.clone(), kind.label.de.clone())).collect();
    println!("wg8-door catalog={} kinds={kinds:?}", creation.catalog_phase.as_str());
    assert_eq!(creation.catalog_phase, HubArtifactCatalogPhase::Ready);
    let kind_id = kinds.first().map(|(kind_id, _, _)| kind_id.clone()).expect("the catalog offers a kind");
    hub_verb(&mut shell, crate::hub_connection::action::SELECT_ARTIFACT_KIND, &[("kindId", kind_id.as_str())]);
    hub_verb(&mut shell, crate::hub_connection::action::SET_ARTIFACT_NAME, &[("value", "Door artifact")]);
    let started = std::time::Instant::now();
    hub_verb(&mut shell, crate::hub_connection::action::CREATE_ARTIFACT, &[]);
    let mut trail = Vec::new();
    while started.elapsed() < std::time::Duration::from_secs(150) {
        let observed = shell.hub_workspace.creation.operation.as_ref().map(|operation| (crate::hub_connection::hub_artifact_creation_phase_str(operation.phase), operation.opening));
        if trail.last() != Some(&observed) {
            trail.push(observed);
        }
        if shell.hub_workspace.creation.operation.as_ref().is_some_and(|operation| hub_artifact_creation_terminal(operation.phase) && operation.opening != HubArtifactOpening::Idle && operation.opening != HubArtifactOpening::Opening) {
            break;
        }
        drive(shell.pump_sync_events());
        std::thread::sleep(std::time::Duration::from_millis(25));
    }
    let operation = shell.hub_workspace.creation.operation.clone().expect("the door holds its creation");
    println!("wg8-door kind={kind_id} trail={trail:?} after={:?} ready={:?}", started.elapsed(), operation.ready);
    for locale in [Locale::En, Locale::De] {
        let tree = crate::hub_connection::build_hub_workspace_ui(&shell.hub_workspace, locale);
        let mut phases = Vec::new();
        hub_attribute_values(&tree, "data-semio-hub-artifact-creation-phase", &mut phases);
        println!("wg8-door tree locale={locale:?} phase={phases:?}");
    }
    assert_eq!(operation.phase, SpaceArtifactCreationPhaseV1::Ready);
    assert!(operation.ready.as_ref().is_some_and(|ready| ready.kind_id == kind_id && ready.artifact_id.starts_with("artifact-")));
    hub_verb(&mut shell, crate::hub_connection::action::SIGN_OUT, &[]);
}
