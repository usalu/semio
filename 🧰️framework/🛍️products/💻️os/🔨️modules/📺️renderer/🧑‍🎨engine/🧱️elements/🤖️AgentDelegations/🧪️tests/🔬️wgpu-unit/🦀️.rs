//! 🧪️ Laws of the wgpu agent-delegations twin: the shared vectors
//! (`📇️directory/🤖️delegations/🧪️tests/🧫️fixtures/📐️agent-delegation-contract-vectors.json`) and the shared label/control
//! contract (`📇️directory/🤖️delegations/🔣️.json`) are the same files the React twin's vitest suite reads.

use super::*;
use semio_ui_declaration::ui::UiButtonNode;

const VECTORS: &str = include_str!("../../../../../../../../../🔨️modules/📇️directory/🤖️delegations/🧪️tests/🧫️fixtures/📐️agent-delegation-contract-vectors.json");

fn vectors() -> serde_json::Value {
    serde_json::from_str(VECTORS).expect("the shared vectors are valid JSON")
}

fn list(value: &serde_json::Value) -> Vec<serde_json::Value> {
    value.as_array().cloned().unwrap_or_default()
}

fn walk<'a>(node: &'a UiNode, out: &mut Vec<&'a UiNode>) {
    out.push(node);
    if let UiNode::Stack(stack) = node {
        for child in &stack.children {
            walk(child, out);
        }
    }
}

fn strings(node: &UiNode) -> Vec<String> {
    let mut nodes = Vec::new();
    walk(node, &mut nodes);
    nodes
        .into_iter()
        .filter_map(|node| match node {
            UiNode::Text(text) => Some(text.value.as_str().to_string()),
            UiNode::Button(button) => Some(button.label.as_str().to_string()),
            UiNode::Input(input) => Some(input.value.clone()),
            _ => None,
        })
        .collect()
}

fn buttons(node: &UiNode) -> Vec<&UiButtonNode> {
    let mut nodes = Vec::new();
    walk(node, &mut nodes);
    nodes.into_iter().filter_map(|node| if let UiNode::Button(button) = node { Some(button) } else { None }).collect()
}

fn button_with<'a>(node: &'a UiNode, id: &str) -> Option<&'a UiButtonNode> {
    buttons(node).into_iter().find(|button| button.id.as_deref() == Some(id))
}

fn summary(id: &str, created: i64, expires: i64, revoked: bool) -> AgentDelegationSummary {
    AgentDelegationSummary { delegation_id: id.into(), agent_label: format!("agent {id}"), space_id: "space-a".into(), audience: AgentAudience::Edit, created_at_ms: created, expires_at_ms: expires, revoked, last_used_at_ms: None }
}

fn receipt() -> AgentDelegationReceipt {
    let source = vectors()["credential"]["receipt"].clone();
    parse_agent_delegation_receipt(&serde_json::json!({
        "schema": "semio.hub.auth.agent-delegation-receipt/v1",
        "delegationId": source["delegationId"], "agentPrincipalId": source["agentPrincipalId"], "agentLabel": source["agentLabel"],
        "spaceId": source["spaceId"], "audience": source["audience"], "expiresAtMs": source["expiresAtMs"], "token": source["token"]
    }).to_string())
    .unwrap()
}

fn facts(can_delegate: bool) -> AgentSurfaceFacts<'static> {
    AgentSurfaceFacts { signed_in: true, space_id: Some("space-a"), can_delegate }
}

#[test]
fn both_locales_publish_the_same_keys_with_no_english_left_in_german() {
    let labels = &contract().labels;
    let mut en: Vec<&String> = labels["en"].keys().collect();
    let mut de: Vec<&String> = labels["de"].keys().collect();
    en.sort();
    de.sort();
    assert_eq!(en, de);
    for key in en {
        assert!(!labels["en"][key].is_empty() && !labels["de"][key].is_empty(), "{key}");
    }
    for key in ["title", "description", "create", "revokeConfirmBody"] {
        assert_ne!(agent_label(Locale::En, key), agent_label(Locale::De, key), "{key}");
    }
    assert_eq!(agent_label_with(Locale::En, "revoke", &[("name", "Ada")]), "Withdraw Ada");
    assert_eq!(agent_label_with(Locale::De, "created", &[("when", "x")]), "Erstellt x");
}

#[test]
fn the_controls_keep_the_react_ids_and_icons() {
    for (name, id, icon) in [("create", "os.hub.agent.create", "cpu"), ("download", "os.hub.agent.download", "download"), ("mcpInstall", "os.hub.agent.mcpInstall", "link"), ("mcpCopy", "os.hub.agent.mcpCopy", "copy")] {
        assert_eq!((contract().controls[name].id.as_str(), contract().controls[name].icon.as_str()), (id, icon));
    }
    assert_eq!(contract().fields["name"], "os.hub.agent.name");
    assert_eq!(contract().fields["list"], "os.hub.agent.list");
    assert_eq!(agent_ttl_choices_secs(), &[86_400, 604_800, 2_592_000]);
}

#[test]
fn every_instant_vector_renders_the_locale_free_utc_text() {
    for row in list(&vectors()["instants"]) {
        assert_eq!(agent_delegation_instant(row["ms"].as_i64()), row["text"].as_str().unwrap());
    }
}

#[test]
fn the_ordering_vector_sorts_live_then_expired_then_revoked_newest_first() {
    let source = &vectors()["ordering"];
    let summaries: Vec<AgentDelegationSummary> = parse_agent_delegation_list(&serde_json::json!({ "delegations": source["summaries"] }).to_string());
    let rows = agent_delegation_rows(&summaries, source["nowMs"].as_i64().unwrap());
    let expected = list(&source["order"]);
    assert_eq!(rows.len(), expected.len());
    for (row, expected) in rows.iter().zip(expected) {
        assert_eq!((row.summary.delegation_id.as_str(), row.state.as_str(), agent_delegation_revocable(row)), (expected["delegationId"].as_str().unwrap(), expected["state"].as_str().unwrap(), expected["revocable"].as_bool().unwrap()));
    }
}

#[test]
fn every_create_body_vector_builds_the_exact_body_or_names_its_refusal() {
    for row in list(&vectors()["createBodies"]) {
        let audience = AgentAudience::parse(row["audience"].as_str().unwrap()).unwrap();
        let built = create_agent_delegation_body(row["spaceId"].as_str().unwrap(), row["agentLabel"].as_str().unwrap(), audience, row["ttlSecs"].as_u64().unwrap());
        match row["body"].as_str() {
            Some(body) => assert_eq!(built.as_deref(), Ok(body), "{}", row["id"]),
            None => assert_eq!(built, Err(row["error"].as_str().unwrap()), "{}", row["id"]),
        }
    }
}

#[test]
fn every_receipt_vector_is_read_or_refused() {
    for row in list(&vectors()["receipts"]) {
        let parsed = parse_agent_delegation_receipt(row["body"].as_str().unwrap());
        assert_eq!(parsed.is_ok(), !row["receipt"].is_null(), "{}", row["id"]);
        if let (Ok(parsed), false) = (parsed, row["receipt"].is_null()) {
            assert_eq!(parsed.delegation_id, row["receipt"]["delegationId"].as_str().unwrap());
            assert_eq!(parsed.token, row["receipt"]["token"].as_str().unwrap());
        }
    }
}

#[test]
fn the_credential_vector_fixes_the_file_name_bytes_and_command() {
    let source = &vectors()["credential"];
    let receipt = receipt();
    let file = agent_credential_file(&receipt, source["hubOrigin"].as_str().unwrap()).unwrap();
    assert_eq!(file.file_name, source["fileName"].as_str().unwrap());
    assert_eq!(file.contents, source["contents"].as_str().unwrap());
    assert_eq!(agent_credential_command(&receipt, source["hubOrigin"].as_str().unwrap()), source["command"].as_str().unwrap());
    let mut empty = receipt;
    empty.agent_label = source["emptySlugFileName"]["agentLabel"].as_str().unwrap().into();
    assert_eq!(agent_credential_file_name(&empty), source["emptySlugFileName"]["fileName"].as_str().unwrap());
    assert!(agent_credential_file(&empty, "").is_err());
}

#[test]
fn every_error_vector_maps_to_its_closed_code() {
    for row in list(&vectors()["errors"]) {
        assert_eq!(agent_error_from_response(row["status"].as_u64().unwrap() as u16, row["body"].as_str().unwrap()).as_str(), row["code"].as_str().unwrap());
    }
}

#[test]
fn every_list_and_path_vector_holds() {
    for row in list(&vectors()["lists"]) {
        let parsed = parse_agent_delegation_list(row["body"].as_str().unwrap());
        let expected = list(&row["delegations"]);
        assert_eq!(parsed.len(), expected.len(), "{}", row["id"]);
        for (got, want) in parsed.iter().zip(expected) {
            assert_eq!((got.delegation_id.as_str(), got.revoked, got.last_used_at_ms), (want["delegationId"].as_str().unwrap(), want["revoked"].as_bool().unwrap(), want["lastUsedAtMs"].as_i64()));
        }
    }
    for row in list(&vectors()["paths"]["list"]) {
        assert_eq!(agent_delegation_list_path(row["spaceId"].as_str().unwrap()).unwrap(), row["path"].as_str().unwrap());
    }
    for row in list(&vectors()["paths"]["revoke"]) {
        assert_eq!(agent_delegation_revoke_path(row["delegationId"].as_str().unwrap()).unwrap(), row["path"].as_str().unwrap());
    }
    assert!(agent_delegation_list_path("").is_err() && agent_delegation_revoke_path("a\u{7}b").is_err());
}

#[test]
fn the_one_time_credential_lives_only_until_dismissed_and_the_token_is_shown_only_on_the_save_fallback() {
    let mut state = HubAgentState::default();
    state.minted(receipt(), "http://127.0.0.1:7800").unwrap();
    let token = receipt().token;
    let tree = build_agent_delegations_ui(&state, facts(true), Locale::En, 126);
    assert!(strings(&tree).iter().any(|text| text.starts_with("semio-os-mcp stdio")), "the command is shown");
    assert!(!strings(&tree).iter().any(|text| text.contains(&token)), "the token is not painted before a save is attempted");
    assert!(button_with(&tree, "os.hub.agent.download").is_some());
    state.download_settled(&receipt().delegation_id, false);
    let tree = build_agent_delegations_ui(&state, facts(true), Locale::En, 126);
    assert!(strings(&tree).iter().any(|text| text.contains(&token)), "no file door: the exact file text is shown to copy");
    assert!(strings(&tree).iter().any(|text| text == agent_label(Locale::En, "downloadFailed")));
    assert!(state.begin_install("http://127.0.0.1:7800").is_some());
    state.install_settled(&receipt().delegation_id, "http://127.0.0.1:7800", Some(404), "");
    assert!(strings(&build_agent_delegations_ui(&state, facts(true), Locale::En, 126)).iter().any(|text| text == agent_label(Locale::En, "mcpUnavailable")));
    state.dismiss_credential();
    let tree = build_agent_delegations_ui(&state, facts(true), Locale::De, 126);
    assert!(!strings(&tree).iter().any(|text| text.contains(&token)) && button_with(&tree, "os.hub.agent.download").is_none());
}

#[test]
fn the_form_is_offered_only_to_an_author_of_an_open_space_and_submits_only_with_a_name() {
    let mut state = HubAgentState::default();
    let tree = build_agent_delegations_ui(&state, facts(true), Locale::En, 126);
    assert_eq!(button_with(&tree, "os.hub.agent.create").unwrap().presence.state, semio_ui_declaration::ui::UiState::Disabled, "no name: Create is disabled");
    state.name_draft = "  Claude ".into();
    assert_eq!(button_with(&build_agent_delegations_ui(&state, facts(true), Locale::En, 126), "os.hub.agent.create").unwrap().presence.state, semio_ui_declaration::ui::UiState::Normal);
    let spectator = build_agent_delegations_ui(&state, facts(false), Locale::En, 126);
    assert!(button_with(&spectator, "os.hub.agent.create").is_none() && strings(&spectator).iter().any(|text| text == agent_label(Locale::En, "notAuthor")));
    let signed_out = build_agent_delegations_ui(&state, AgentSurfaceFacts { signed_in: false, space_id: Some("space-a"), can_delegate: true }, Locale::En, 126);
    assert!(strings(&signed_out).iter().any(|text| text == agent_label(Locale::En, "signedOut")) && button_with(&signed_out, "os.hub.agent.create").is_none());
    let no_space = build_agent_delegations_ui(&state, AgentSurfaceFacts { signed_in: true, space_id: None, can_delegate: true }, Locale::En, 126);
    assert!(strings(&no_space).iter().any(|text| text == agent_label(Locale::En, "noSpace")));
}

#[test]
fn withdrawal_is_a_two_step_inline_confirmation_offered_only_for_non_revoked_rows() {
    let mut state = HubAgentState::default();
    state.begin_load("space-a");
    state.loaded(&[summary("d-1", 100, 9000, false), summary("d-2", 90, 9000, true)], 1000);
    let tree = build_agent_delegations_ui(&state, facts(true), Locale::En, 126);
    assert!(button_with(&tree, "os.hub.agent.revoke.d-1").is_some() && button_with(&tree, "os.hub.agent.revoke.d-2").is_none(), "a revoked row is already withdrawn");
    state.ask_revoke("d-2");
    assert_eq!(state.revoke_confirm, None);
    state.ask_revoke("d-1");
    let tree = build_agent_delegations_ui(&state, facts(true), Locale::En, 126);
    assert!(button_with(&tree, "os.hub.agent.revoke.d-1").is_none() && button_with(&tree, "os.hub.agent.revokeConfirm.d-1").is_some() && button_with(&tree, "os.hub.agent.revokeCancel.d-1").is_some());
    state.cancel_revoke();
    assert_eq!(state.confirm_revoke(), None);
    state.ask_revoke("d-1");
    assert_eq!(state.confirm_revoke().as_deref(), Some("d-1"));
}

#[test]
fn another_spaces_rows_are_never_shown_under_the_new_space_and_errors_name_a_closed_cause() {
    let mut state = HubAgentState::default();
    state.begin_load("space-a");
    state.loaded(&[summary("d-1", 1, 9000, false)], 10);
    state.begin_load("space-b");
    assert!(state.rows.is_empty() && state.phase == AgentPhase::Loading);
    state.failed(AgentErrorCode::RateLimited);
    let tree = build_agent_delegations_ui(&state, facts(true), Locale::En, 126);
    assert!(strings(&tree).iter().any(|text| text == agent_label(Locale::En, "errorRateLimited")) && strings(&tree).iter().any(|text| text == agent_label(Locale::En, "failed")));
}

#[test]
fn rows_past_the_node_budget_are_counted_not_silently_dropped() {
    let mut state = HubAgentState::default();
    state.begin_load("space-a");
    let summaries: Vec<AgentDelegationSummary> = (0..12).map(|index| summary(&format!("d-{index:02}"), index, 9000, false)).collect();
    state.loaded(&summaries, 10);
    let tree = build_agent_delegations_ui(&state, facts(true), Locale::En, 30);
    assert!(node_count(&tree) <= 30, "{}", node_count(&tree));
    assert!(strings(&tree).iter().any(|text| text.starts_with('+')), "the hidden count is named");
}

const MCP_CONFIG: &str = include_str!("../../../../../../📇️directory/🤖️delegations/🧪️tests/🧫️fixtures/🔌️mcp-client-config.json");

fn mcp_case_receipt(case: &serde_json::Value) -> AgentDelegationReceipt {
    let source = &case["receipt"];
    parse_agent_delegation_receipt(&serde_json::json!({
        "schema": "semio.hub.auth.agent-delegation-receipt/v1",
        "delegationId": source["delegationId"], "agentPrincipalId": source["agentPrincipalId"], "agentLabel": source["agentLabel"],
        "spaceId": source["spaceId"], "audience": source["audience"], "expiresAtMs": source["expiresAtMs"], "token": source["token"]
    }).to_string())
    .unwrap()
}

#[test]
fn the_mcp_client_configuration_replays_the_shared_law() {
    let fixture: serde_json::Value = serde_json::from_str(MCP_CONFIG).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let receipt = mcp_case_receipt(case);
        let origin = case["hubOrigin"].as_str().unwrap();
        assert_eq!(agent_credential_install_file_name(&receipt.delegation_id).unwrap(), case["fileName"].as_str().unwrap());
        let installed = parse_agent_credential_install_receipt(&case["installed"].to_string()).unwrap();
        let config = agent_mcp_client_config(&receipt, origin, &installed).unwrap();
        assert_eq!(serde_json::from_str::<serde_json::Value>(&config).unwrap(), case["expected"], "{}", case["name"]);
        assert!(config.ends_with("}\n") && !config.contains(&receipt.token), "the token never appears in the configuration");
    }
    for refusal in fixture["refusals"].as_array().unwrap() {
        assert_eq!(agent_credential_install_file_name(refusal["delegationId"].as_str().unwrap()), Err(refusal["error"].as_str().unwrap()), "{}", refusal["name"]);
    }
}

#[test]
fn an_install_answer_settles_the_block_like_react_and_only_for_its_own_delegation() {
    let fixture: serde_json::Value = serde_json::from_str(MCP_CONFIG).unwrap();
    let case = &fixture["cases"][0];
    let (receipt, origin) = (mcp_case_receipt(case), case["hubOrigin"].as_str().unwrap());
    let mut state = HubAgentState::default();
    state.minted(receipt.clone(), origin).unwrap();
    let (id, body) = state.begin_install(origin).expect("an idle block installs");
    assert!(body.contains(&contract().schemas["installRequest"]) && body.contains("\"contents\""));
    assert!(state.begin_install(origin).is_none(), "an installing block does not start a second install");
    state.install_settled("another-delegation", origin, Some(200), &case["installed"].to_string());
    assert_eq!(state.credential.as_ref().unwrap().mcp, AgentMcpPhase::Installing, "an answer for another credential is ignored");
    state.install_settled(&id, origin, Some(200), &case["installed"].to_string());
    assert_eq!(state.credential.as_ref().unwrap().mcp, AgentMcpPhase::Ready);
    let tree = build_agent_delegations_ui(&state, facts(true), Locale::En, 126);
    assert!(button_with(&tree, "os.hub.agent.mcpCopy").is_some() && strings(&tree).iter().any(|text| text == agent_label(Locale::En, "mcpReady")));
    state.mcp_copied(&id);
    assert_eq!(state.credential.as_ref().unwrap().mcp, AgentMcpPhase::Copied);
    for (status, expected) in [(Some(404), AgentMcpPhase::Unavailable), (Some(405), AgentMcpPhase::Unavailable), (Some(500), AgentMcpPhase::Failed), (None, AgentMcpPhase::Failed)] {
        let mut state = HubAgentState::default();
        state.minted(receipt.clone(), origin).unwrap();
        state.begin_install(origin);
        state.install_settled(&id, origin, status, "");
        assert_eq!(state.credential.as_ref().unwrap().mcp, expected, "{status:?}");
    }
}

#[test]
fn a_download_answer_marks_the_credential_saved_or_failed() {
    let mut state = HubAgentState::default();
    state.minted(receipt(), "http://127.0.0.1:7800").unwrap();
    let (name, contents) = state.download_file().expect("a shown credential has a file");
    assert_eq!((name.as_str(), contents.as_str()), (state.credential.as_ref().unwrap().file.file_name.as_str(), state.credential.as_ref().unwrap().file.contents.as_str()));
    state.download_settled(&receipt().delegation_id, true);
    assert_eq!(state.credential.as_ref().unwrap().save, AgentSave::Saved);
    assert!(strings(&build_agent_delegations_ui(&state, facts(true), Locale::De, 126)).iter().any(|text| text == agent_label(Locale::De, "downloaded")));
}

#[test]
fn audience_and_expiry_are_select_components_with_the_react_options() {
    let tree = build_agent_delegations_ui(&HubAgentState::default(), facts(true), Locale::En, 126);
    let mut nodes = Vec::new();
    walk(&tree, &mut nodes);
    let selects: Vec<_> = nodes.into_iter().filter_map(|node| if let UiNode::Select(select) = node { Some(select) } else { None }).collect();
    assert_eq!(selects.iter().map(|select| select.id.as_str()).collect::<Vec<_>>(), ["os.hub.agent.audience", "os.hub.agent.expiry"]);
    assert_eq!(selects[0].items.iter().map(|item| item.value.as_str()).collect::<Vec<_>>(), ["read", "edit"]);
    assert_eq!(selects[0].value, "edit");
    assert_eq!(selects[1].items.iter().map(|item| item.value.as_str()).collect::<Vec<_>>(), ["86400", "604800", "2592000"]);
    assert_eq!(selects[1].value, "604800");
    assert_eq!(selects[1].items[1].label.as_str(), agent_label(Locale::En, "expiryWeek"));
    assert!(selects.iter().all(|select| select.on_change.action == "hubAgentSetAudience" || select.on_change.action == "hubAgentSetExpiry"));
}
