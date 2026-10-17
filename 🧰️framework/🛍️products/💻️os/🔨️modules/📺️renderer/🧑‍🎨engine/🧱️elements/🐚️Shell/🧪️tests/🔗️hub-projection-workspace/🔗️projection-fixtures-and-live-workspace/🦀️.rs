/// 🔗️ Projection fixtures and live workspace: the hub projection fixture records, the route overlay and summary laws, the resolution band and the live sign-in to the retained workspace.

fn shell() -> ShellState {
    ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
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

#[test]
fn hub_command_opens_the_route_overlay_without_adding_a_default_dock_tab() {
    let mut shell = shell();
    let dock = SessionCtx::of(&mut shell).default_dock();
    assert!(!PanelAnchor::ALL.into_iter().flat_map(|anchor| dock.tabs(anchor)).any(|tab| tab.id == FRAMEWORK_HUB_PANEL_ID));
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).apply_os_command("os.openHub", None)).expect("Hub command opens the workspace");
    assert!(shell.directory.hub_workspace_open);
    assert_eq!(shell.opening.uri_history.get(shell.opening.uri_index).map(String::as_str), Some("/hub"));
    assert!(shell.docs.shell_owned_panel_leaves(&shell.chrome, &shell.plugins, &shell.prefs).iter().any(|id| id == FRAMEWORK_HUB_PANEL_ID), "the overlay keeps its retained body publication");
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).handle_hub_workspace_action(semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::CLOSE_WORKSPACE, None));
    assert!(!shell.directory.hub_workspace_open);
}

#[test]
fn neutral_session_link_and_multi_document_vectors_fold_to_the_shared_summary() {
    let fixture: HubProjectionFixture = serde_json::from_str(include_str!("../../../../../🧪️tests/🧫️fixtures/🔗️hub-projection/🔣️.json")).expect("hub projection fixture");
    for case in fixture.cases {
        let summary = shell_hub_connection_summary_v1(&case.projection);
        assert_eq!(summary.state.as_str(), case.expected.state, "{} state", case.id);
        assert_eq!(summary.peer_count, case.expected.peer_count, "{} peers", case.id);
        assert_eq!(summary.document_count, case.expected.document_count, "{} documents", case.id);
    }
}

#[test]
fn status_replacement_and_document_retirement_share_one_projection() {
    let mut shell = shell();
    shell.sync.publish_hub_document_status("hub:a/one", ShellHubRemoteV1::Connecting);
    shell.sync.publish_hub_document_status("hub:a/one", ShellHubRemoteV1::Live { peer_count: 4 });
    shell.sync.publish_hub_document_status("hub:b/two", ShellHubRemoteV1::Backoff { retry_in_ms: 1000 });
    assert_eq!(shell.sync.hub_documents.len(), 2);
    let projection = ShellHubProjectionV1 {
        session: HubSessionPresence::SignedIn,
        link: HubLink::Reachable,
        documents: shell.sync.hub_documents.iter().map(|(document_key, remote)| ShellHubDocumentV1 { document_key: document_key.clone(), remote: remote.clone() }).collect(),
    };
    let summary = shell_hub_connection_summary_v1(&projection);
    assert_eq!(summary.state, HubConnectionState::Live { peer_count: 4 });
    shell.sync.retire_hub_document_status("hub:a/one");
    assert_eq!(shell.sync.hub_documents.len(), 1);
    shell.sync.retire_hub_document_status("hub:b/two");
    assert!(shell.sync.hub_documents.is_empty());
}

#[test]
fn document_projection_refuses_a_sixty_fifth_distinct_owner_but_allows_replacement() {
    let mut shell = shell();
    for index in 0..64 {
        shell.sync.publish_hub_document_status(format!("hub:a/{index}"), ShellHubRemoteV1::Detached);
    }
    shell.sync.publish_hub_document_status("hub:a/64", ShellHubRemoteV1::Connecting);
    assert_eq!(shell.sync.hub_documents.len(), 64);
    assert!(!shell.sync.hub_documents.contains_key("hub:a/64"));
    shell.sync.publish_hub_document_status("hub:a/0", ShellHubRemoteV1::Live { peer_count: 2 });
    assert_eq!(shell.sync.hub_documents.get("hub:a/0"), Some(&ShellHubRemoteV1::Live { peer_count: 2 }));
}

/// 🧩️ A hub document's open band names the resolution step it is on — asking the hub for the lease, downloading
/// the component, then its descriptor, verified — as step 1 of 3, in both tongues, and a cancelled resolution
/// settles as cancelled (ticket 26/09/23 slice WG8, hub-resolved execution targets).
#[test]
fn the_open_band_names_every_resolution_step_in_both_tongues() {
    let mut opening = ShellDocumentOpening::new("Block".into(), ShellDocumentOpenPhase::Resolving, "block".into(), "s.block.block2d@1/*#editor".into(), None, None);
    for (step, en, de) in [
        (0, "asking the hub which component runs it", "Hub wird nach der ausführenden Komponente gefragt"),
        (1, "downloading the component", "Komponente wird heruntergeladen"),
        (2, "downloading its descriptor", "Deskriptor wird heruntergeladen"),
        (3, "verified", "geprüft"),
    ] {
        opening.resolve_step.store(step, std::sync::atomic::Ordering::Release);
        let english = document_opening_banner_text(&opening, opening.started_at_ms, false);
        let german = document_opening_banner_text(&opening, opening.started_at_ms, true);
        assert_eq!(english, format!("Resolving the hub's component for Block ({en}) · 1/3 · 0 s"));
        assert_eq!(german, format!("Hub-Komponente wird ermittelt für Block ({de}) · 1/3 · 0 s"));
    }
    assert!(opening.running(), "a resolving open is still running");
    opening.phase = ShellDocumentOpenPhase::Instantiating;
    assert_eq!(document_opening_banner_text(&opening, opening.started_at_ms, false), "Starting Block · 2/3 · 0 s");
    opening.phase = ShellDocumentOpenPhase::Cancelled;
    assert!(!opening.running(), "a cancelled open only waits for its band to close");
}

/// 🔐️ One hub workspace verb, driven the way a frame drives it natively ([`drive`], which also pumps the
/// renderer I/O and worker-retirement slots). A bare `block_on` parked for ever inside a verb after a
/// live law had opened a document: sampled on two-user gate run 17, the test thread sat in
/// `block_on(handle_hub_workspace_action)` for 23 min, and eight earlier runs never exited.
fn hub_verb(shell: &mut ShellState, verb: &str, args: &[(&str, &str)]) {
    let args = (!args.is_empty()).then(|| DslValue::Object(args.iter().map(|(key, value)| ((*key).to_string(), DslValue::String((*value).to_string()))).collect()));
    #[cfg(not(target_arch = "wasm32"))]
    drive(ActionsCtx::of(&mut shell).handle_hub_workspace_action(verb, args));
    #[cfg(target_arch = "wasm32")]
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).handle_hub_workspace_action(verb, args));
    settle_hub_workspace(shell);
}

/// 🔐️ Pumps the directory lane until the hub workspace's spawned requests (sign-in, spaces) have answered — the
/// verbs only arm them now, and a frame applies the answer.
fn settle_hub_workspace(shell: &mut ShellState) {
    while !shell.directory.hub_workspace_settled() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = semio_framework_os_renderer_wgpu_host_base::pump_renderer_io_sessions(1);
            drive(ActionsCtx::of(&mut shell).pump_directory_events());
        }
        #[cfg(target_arch = "wasm32")]
        semio_framework_async::block_on(ActionsCtx::of(&mut shell).pump_directory_events());
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
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
    semio_framework_async::block_on(ActionsCtx::of(&mut shell).apply_os_command("os.openHub", None)).expect("the hub route opens");
    assert!(shell.directory.hub_workspace_open);
    assert_eq!(shell.directory.hub_workspace.presence(), HubSessionPresence::SignedOut);
    assert_eq!(shell.sync.hub_connection_state(&shell.directory), HubConnectionState::SignedOut);
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_ADDRESS, &[("value", origin.as_str())]);
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::ADD_CONNECTION, &[]);
    assert_eq!(shell.directory.hub_workspace.origin(), origin.trim_end_matches('/'));
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_EMAIL, &[("value", email.as_str())]);
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_PASSWORD, &[("value", password.as_str())]);
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SIGN_IN, &[]);
    println!("wg6-live sign-in phase={} error={:?} user={:?} display={:?}", shell.directory.hub_workspace.session.phase.as_str(), shell.directory.hub_workspace.session.error, shell.directory.hub_workspace.session.user_id, shell.directory.hub_workspace.display_name);
    assert_eq!(shell.directory.hub_workspace.session.phase, HubSessionPhase::SignedIn, "error {:?}", shell.directory.hub_workspace.session.error);
    assert!(shell.directory.hub_workspace.password_draft.is_empty(), "the password never outlives its request");
    assert!(shell.directory.hub_workspace.display_name.is_some());
    assert_eq!(shell.sync.hub_connection_state(&shell.directory), HubConnectionState::Online, "a verified session on a reachable hub with nothing attached");
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SET_SPACE_NAME, &[("value", space_name.as_str())]);
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::CREATE_SPACE, &[]);
    println!("wg6-live spaces phase={} rows={:?}", shell.directory.hub_workspace.phase.as_str(), shell.directory.hub_workspace.rows.iter().map(|row| (row.name.as_str(), row.id.as_str(), row.access.as_str())).collect::<Vec<_>>());
    assert_eq!(shell.directory.hub_workspace.phase, semio_framework_os_renderer_wgpu_hub_contracts::space_browser::SpaceBrowserPhase::Ready);
    let created = shell.directory.hub_workspace.rows.iter().find(|row| row.name == space_name).expect("the created space is listed").id.clone();
    for locale in [Locale::En, Locale::De] {
        let tree = semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::build_hub_workspace_ui(&shell.directory.hub_workspace, locale);
        let mut spaces = Vec::new();
        hub_attribute_values(&tree, "data-semio-hub-space", &mut spaces);
        let mut phases = Vec::new();
        hub_attribute_values(&tree, "data-semio-hub-phase", &mut phases);
        println!("wg6-live tree locale={locale:?} phase={phases:?} spaces={spaces:?}");
        assert!(spaces.contains(&created), "{locale:?} tree lists the created space");
        assert_eq!(phases, vec![HubSessionPhase::SignedIn.as_str().to_string()]);
    }
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::OPEN_SPACE, &[("spaceId", created.as_str())]);
    println!(
        "wg6-live open space={:?} members={:?} uri={:?}",
        shell.directory.hub_workspace.open_space_id,
        shell.directory.hub_workspace.members.iter().map(|member| (member.display_name.as_str(), member.owner)).collect::<Vec<_>>(),
        shell.opening.uri_history.get(shell.opening.uri_index)
    );
    assert_eq!(shell.directory.hub_workspace.open_space_id.as_deref(), Some(created.as_str()));
    assert!(shell.directory.hub_workspace.members.iter().any(|member| member.owner), "the creator is listed as the owner");
    hub_verb(&mut shell, semio_framework_os_renderer_wgpu_hub_workspace::hub_connection::action::SIGN_OUT, &[]);
    assert_eq!(shell.directory.hub_workspace.presence(), HubSessionPresence::SignedOut);
    assert!(shell.directory.hub_workspace.rows.is_empty());
}
