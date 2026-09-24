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
    assert!(!PanelAnchor::ALL.into_iter().flat_map(|anchor| dock.tabs(anchor)).any(|tab| tab.id == crate::hub_connection::FRAMEWORK_HUB_PANEL_ID));
    semio_framework_async::block_on(shell.apply_os_command("os.openHub", None)).expect("Hub command opens the workspace");
    assert!(shell.hub_workspace_open);
    assert_eq!(shell.uri_history.get(shell.uri_index).map(String::as_str), Some("/hub"));
    assert!(shell.shell_owned_panel_leaves().iter().any(|id| id == crate::hub_connection::FRAMEWORK_HUB_PANEL_ID), "the overlay keeps its retained body publication");
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

fn hub_verb(shell: &mut ShellState, verb: &str, args: &[(&str, &str)]) {
    let args = (!args.is_empty()).then(|| DslValue::Object(args.iter().map(|(key, value)| ((*key).to_string(), DslValue::String((*value).to_string()))).collect()));
    semio_framework_async::block_on(shell.handle_hub_workspace_action(verb, args));
}

fn hub_attribute_values(node: &ui_wgpu::wgpu::UiNode, attribute: &str, found: &mut Vec<String>) {
    match node {
        ui_wgpu::wgpu::UiNode::Stack(stack) => stack.children.iter().for_each(|child| hub_attribute_values(child, attribute, found)),
        ui_wgpu::wgpu::UiNode::Text(text) => found.extend(text.data_attributes.as_ref().and_then(|attributes| attributes.get(attribute)).cloned()),
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
    assert_eq!(shell.hub_workspace.presence(), crate::hub_connection::HubSessionPresence::SignedOut);
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
    for locale in [ui_wgpu::wgpu::Locale::En, ui_wgpu::wgpu::Locale::De] {
        let tree = crate::hub_connection::build_hub_workspace_ui(&shell.hub_workspace, locale);
        let mut spaces = Vec::new();
        hub_attribute_values(&tree, "data-semio-hub-space", &mut spaces);
        let mut phases = Vec::new();
        hub_attribute_values(&tree, "data-semio-hub-phase", &mut phases);
        println!("wg6-live tree locale={locale:?} phase={phases:?} spaces={spaces:?}");
        assert!(spaces.contains(&created), "{locale:?} tree lists the created space");
        assert_eq!(phases, vec!["signedIn".to_string()]);
    }
    hub_verb(&mut shell, crate::hub_connection::action::OPEN_SPACE, &[("spaceId", created.as_str())]);
    println!("wg6-live open space={:?} members={:?} uri={:?}", shell.hub_workspace.open_space_id, shell.hub_workspace.members.iter().map(|member| (member.display_name.as_str(), member.owner)).collect::<Vec<_>>(), shell.uri_history.get(shell.uri_index));
    assert_eq!(shell.hub_workspace.open_space_id.as_deref(), Some(created.as_str()));
    assert!(shell.hub_workspace.members.iter().any(|member| member.owner), "the creator is listed as the owner");
    hub_verb(&mut shell, crate::hub_connection::action::SIGN_OUT, &[]);
    assert_eq!(shell.hub_workspace.presence(), crate::hub_connection::HubSessionPresence::SignedOut);
    assert!(shell.hub_workspace.rows.is_empty());
}
