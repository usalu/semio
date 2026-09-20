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
        documents: shell
            .hub_documents
            .iter()
            .map(|(document_key, remote)| ShellHubDocumentV1 { document_key: document_key.clone(), remote: remote.clone() })
            .collect(),
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
