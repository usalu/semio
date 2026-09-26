//! 🧪️ Laws of the wgpu hub workspace and of the footer-pill fold (ticket 26/09/18, slice WG6 —
//! G8 items WG-6 and WG-5).
//!
//! 🧮️ The fold half is the Rust twin of `hubConnectionSummaryV1`; its three precedence laws are
//! stated in `📓️u1-progress-cancel-and-connection-status.md` §8 precisely so the two
//! implementations can be compared law by law rather than by eye.
//!
//! 🌲️ The surface half asserts the RETAINED TREE, not pixels: the wgpu accessibility projection is
//! a walk of exactly this tree, so a law over the tree is a law over what an assistive technology
//! announces.

use super::*;
use crate::hub_sign_in::{hub_session_initial_state, parse_hub_connection_book, HubSessionEvent, HubSignInErrorCode, LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1};
use crate::space_browser::{SpaceAccess, SpaceBrowserPhase};

fn workspace() -> HubWorkspaceState {
    HubWorkspaceState::new(parse_hub_connection_book(None, "http://127.0.0.1:7777", Locale::En))
}

fn signed_in(mut state: HubWorkspaceState) -> HubWorkspaceState {
    state.session = crate::hub_sign_in::reduce_hub_session(&state.session, &HubSessionEvent::Minted { user_id: "u1".into(), expires_at_ms: None });
    state.phase = SpaceBrowserPhase::Ready;
    state
}

fn row(id: &str, access: SpaceAccess) -> SpaceRow {
    SpaceRow {
        id: id.to_string(),
        name: format!("Space {id}"),
        kind: semio_framework_os_kernel::os_directory::DirectorySpaceKind::Studio,
        visibility: semio_framework_os_kernel::os_directory::DirectorySpaceVisibility::Private,
        access,
        role: Some(DirectorySpaceRole::Author),
        member_count: 2,
        document_count: 1,
        active_connections: 1,
        updated_at_ms: 10,
    }
}

fn texts(node: &UiNode, out: &mut Vec<String>) {
    match node {
        UiNode::Text(text) => out.push(text.value.as_str().to_string()),
        UiNode::Button(button) => out.push(button.label.as_str().to_string()),
        UiNode::Input(input) => {
            out.push(input.value.clone());
            if let Some(placeholder) = input.placeholder.as_ref() {
                out.push(placeholder.as_str().to_string());
            }
        }
        UiNode::Stack(stack) => {
            for child in &stack.children {
                texts(child, out);
            }
        }
        _ => {}
    }
}

fn control_ids(node: &UiNode, out: &mut Vec<String>) {
    match node {
        UiNode::Button(button) => out.push(button.id.clone().unwrap_or_default()),
        UiNode::Input(input) => out.push(input.id.clone()),
        UiNode::Stack(stack) => {
            out.push(stack.id.clone().unwrap_or_default());
            for child in &stack.children {
                control_ids(child, out);
            }
        }
        _ => {}
    }
}

fn enabled_buttons(node: &UiNode, out: &mut Vec<(String, bool)>) {
    match node {
        UiNode::Button(button) => out.push((button.id.clone().unwrap_or_default(), button.presence.state != UiState::Disabled)),
        UiNode::Stack(stack) => {
            for child in &stack.children {
                enabled_buttons(child, out);
            }
        }
        _ => {}
    }
}

fn input_contracts(node: &UiNode, out: &mut Vec<(String, String, bool)>) {
    match node {
        UiNode::Input(input) => out.push((input.id.clone(), input.input_kind.clone(), input.commit.is_none())),
        UiNode::Stack(stack) => {
            for child in &stack.children {
                input_contracts(child, out);
            }
        }
        _ => {}
    }
}

fn attributes(node: &UiNode, out: &mut Vec<(String, String)>) {
    match node {
        UiNode::Text(text) => {
            if let Some(map) = text.data_attributes.as_ref() {
                for (key, value) in map {
                    out.push((key.clone(), value.clone()));
                }
            }
        }
        UiNode::Stack(stack) => {
            for child in &stack.children {
                attributes(child, out);
            }
        }
        _ => {}
    }
}

fn collected<T>(node: &UiNode, walk: fn(&UiNode, &mut Vec<T>)) -> Vec<T> {
    let mut out = Vec::new();
    walk(node, &mut out);
    out
}

//#region 📶️FoldLaws
#[test]
fn signed_out_outranks_every_transport_state_and_is_always_an_entry_point() {
    let summary = hub_connection_summary(&[HubDocumentRemote::Live { peer_count: 9 }], HubSessionPresence::SignedOut, HubLink::Reachable);
    assert_eq!(summary.state, HubConnectionState::SignedOut);
    assert!(summary.actionable, "signed out is the one state that must offer a way in");
}

#[test]
fn one_live_document_means_the_hub_is_reachable_however_many_others_are_detached() {
    let statuses = [HubDocumentRemote::Detached, HubDocumentRemote::Backoff, HubDocumentRemote::Live { peer_count: 2 }, HubDocumentRemote::Connecting];
    assert_eq!(hub_connection_summary(&statuses, HubSessionPresence::SignedIn, HubLink::Unreachable).state, HubConnectionState::Live { peer_count: 2 });
}

#[test]
fn the_peer_count_is_the_max_over_live_documents_and_never_a_sum() {
    let statuses = [HubDocumentRemote::Live { peer_count: 3 }, HubDocumentRemote::Live { peer_count: 1 }, HubDocumentRemote::Live { peer_count: 2 }];
    assert_eq!(hub_connection_summary(&statuses, HubSessionPresence::SignedIn, HubLink::Reachable).state, HubConnectionState::Live { peer_count: 3 }, "a sum would double-count a peer with two documents open, which no reader could interpret");
}

#[test]
fn a_document_still_dialling_outranks_one_already_in_backoff() {
    assert_eq!(hub_connection_summary(&[HubDocumentRemote::Backoff, HubDocumentRemote::Connecting], HubSessionPresence::SignedIn, HubLink::Reachable).state, HubConnectionState::Connecting);
    assert_eq!(hub_connection_summary(&[HubDocumentRemote::Backoff], HubSessionPresence::SignedIn, HubLink::Reachable).state, HubConnectionState::Reconnecting);
}

#[test]
fn an_unreachable_link_is_a_shortage_that_outranks_a_dialling_document() {
    assert_eq!(hub_connection_summary(&[HubDocumentRemote::Connecting], HubSessionPresence::SignedIn, HubLink::Unreachable).state, HubConnectionState::Reconnecting);
    assert_eq!(hub_connection_summary(&[HubDocumentRemote::Detached], HubSessionPresence::SignedIn, HubLink::Verifying).state, HubConnectionState::Connecting);
}

#[test]
fn a_reachable_link_with_nothing_live_is_online_whatever_is_detached() {
    assert_eq!(hub_connection_summary(&[HubDocumentRemote::Detached, HubDocumentRemote::Detached], HubSessionPresence::SignedIn, HubLink::Reachable).state, HubConnectionState::Online);
    assert_eq!(hub_connection_summary(&[], HubSessionPresence::SignedIn, HubLink::Reachable).state, HubConnectionState::Online);
}

#[test]
fn a_shell_with_no_hub_at_all_is_local_and_offers_no_button() {
    let summary = hub_connection_summary(&[HubDocumentRemote::Live { peer_count: 1 }], HubSessionPresence::None, HubLink::Reachable);
    assert_eq!(summary.state, HubConnectionState::Local);
    assert!(!summary.actionable, "`none` is deliberately not `signedOut`: only the latter is actionable");
}

#[test]
fn every_pill_state_carries_an_icon_and_its_own_wire_spelling() {
    let states = [HubConnectionState::Local, HubConnectionState::SignedOut, HubConnectionState::Live { peer_count: 1 }, HubConnectionState::Online, HubConnectionState::Connecting, HubConnectionState::Reconnecting];
    let mut spellings: Vec<&str> = states.iter().map(|state| state.as_str()).collect();
    spellings.sort_unstable();
    spellings.dedup();
    assert_eq!(spellings.len(), states.len(), "icon plus text, never colour alone: the text tells `live` from `online`, which share React's cloud icon");
    assert!(states.iter().all(|state| !state.icon_id().is_empty()));
}
//#endregion 📶️FoldLaws

//#region 🌲️SurfaceLaws
#[test]
fn a_fresh_workspace_addresses_the_selected_hubs_own_origin_and_not_the_pages() {
    let state = workspace();
    assert_eq!(state.origin(), "http://127.0.0.1:7777");
    assert_eq!(state.session.connection_id, LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1);
    assert_eq!(state.presence(), HubSessionPresence::SignedOut);
}

#[test]
fn the_signed_out_surface_offers_a_credential_form_and_states_that_local_work_continues() {
    let tree = build_hub_workspace_ui(&workspace(), Locale::En);
    let ids = collected(&tree, control_ids);
    assert!(ids.iter().any(|id| id == FRAMEWORK_HUB_PANEL_ID));
    assert!(ids.iter().any(|id| id == HUB_EMAIL_INPUT_ID));
    assert!(ids.iter().any(|id| id == HUB_PASSWORD_INPUT_ID));
    let announced = collected(&tree, attributes);
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-local-only" && value == "true"), "the local-only statement is present in every phase");
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-phase" && value == "signed-out"));
}

#[test]
fn every_workspace_draft_is_live_and_the_secret_field_keeps_password_semantics() {
    let inputs = collected(&build_hub_workspace_ui(&workspace(), Locale::En), input_contracts);
    assert!(inputs.iter().all(|(_, _, live)| *live), "workspace drafts must update on change rather than wait for a blur commit");
    assert_eq!(inputs.iter().find(|(id, _, _)| id == HUB_PASSWORD_INPUT_ID).map(|(_, kind, _)| kind.as_str()), Some("password"));
}

#[test]
fn a_selected_remote_hub_has_a_targetable_forget_control() {
    let mut state = workspace();
    let connection = crate::hub_sign_in::HubConnection { id: "remote-hub".into(), kind: crate::hub_sign_in::HubConnectionKind::Remote, label: "Remote hub".into(), origin: "https://hub.example.org".into(), last_user_id: None };
    state.book = crate::hub_sign_in::select_hub_connection(&crate::hub_sign_in::upsert_hub_connection(&state.book, connection), "remote-hub");
    assert!(collected(&build_hub_workspace_ui(&state, Locale::En), control_ids).iter().any(|id| id == &format!("{HUB_SIGN_IN_FORM_ID}.forget")));
}

#[test]
fn the_submit_control_is_disabled_until_both_credential_fields_are_admissible() {
    let mut state = workspace();
    let disabled = collected(&build_hub_workspace_ui(&state, Locale::En), enabled_buttons);
    let submit = format!("{HUB_SIGN_IN_FORM_ID}.submit");
    assert_eq!(disabled.iter().find(|(id, _)| id == &submit).map(|(_, enabled)| *enabled), Some(false));
    state.email_draft = "ada@example.org".into();
    state.password_draft = "short".into();
    let still = collected(&build_hub_workspace_ui(&state, Locale::En), enabled_buttons);
    assert_eq!(still.iter().find(|(id, _)| id == &submit).map(|(_, enabled)| *enabled), Some(false), "a password the hub would refuse never costs a round trip");
    state.password_draft = "correct horse".into();
    let ready = collected(&build_hub_workspace_ui(&state, Locale::En), enabled_buttons);
    assert_eq!(ready.iter().find(|(id, _)| id == &submit).map(|(_, enabled)| *enabled), Some(true));
}

#[test]
fn a_hub_that_refuses_password_sign_in_renders_no_password_field_at_all() {
    let mut state = workspace();
    state.session = crate::hub_sign_in::reduce_hub_session(&state.session, &HubSessionEvent::Failed { code: HubSignInErrorCode::PasswordSignInDisabled, retry_after_seconds: None });
    let ids = collected(&build_hub_workspace_ui(&state, Locale::En), control_ids);
    assert!(!ids.iter().any(|id| id == HUB_PASSWORD_INPUT_ID), "the field is structurally absent rather than offered and then refused");
    let announced = collected(&build_hub_workspace_ui(&state, Locale::En), attributes);
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-error" && value == "password-sign-in-disabled"));
}

#[test]
fn a_rate_limited_denial_is_announced_with_its_countdown_substituted() {
    let mut state = workspace();
    state.session = crate::hub_sign_in::reduce_hub_session(&state.session, &HubSessionEvent::Failed { code: HubSignInErrorCode::RateLimited, retry_after_seconds: Some(45) });
    let strings = collected(&build_hub_workspace_ui(&state, Locale::En), texts);
    assert!(strings.iter().any(|value| value.contains("45")), "{strings:?}");
    assert!(!strings.iter().any(|value| value.contains("{{seconds}}")));
}

#[test]
fn the_signed_in_surface_replaces_the_form_with_a_sign_out_control() {
    let state = signed_in(workspace());
    let ids = collected(&build_hub_workspace_ui(&state, Locale::En), control_ids);
    assert!(!ids.iter().any(|id| id == HUB_EMAIL_INPUT_ID));
    assert!(ids.iter().any(|id| id == &format!("{HUB_SIGN_IN_FORM_ID}.sign-out")));
    assert!(ids.iter().any(|id| id == HUB_SPACES_LIST_ID), "a signed-in shell always shows the spaces section, even while empty");
}

#[test]
fn every_space_row_is_addressable_by_id_and_carries_its_access_class() {
    let mut state = signed_in(workspace());
    state.rows = vec![row("a1", SpaceAccess::Author), row("m1", SpaceAccess::Member)];
    let tree = build_hub_workspace_ui(&state, Locale::En);
    let announced = collected(&tree, attributes);
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-space" && value == "a1"));
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-space-access" && value == "member"));
    let ids = collected(&tree, control_ids);
    assert!(ids.iter().any(|id| id == &format!("{HUB_SPACES_LIST_ID}.open.a1")));
    assert!(ids.iter().any(|id| id == &format!("{HUB_SPACES_LIST_ID}.invite.a1")), "an author may issue an invitation");
    assert!(!ids.iter().any(|id| id == &format!("{HUB_SPACES_LIST_ID}.invite.m1")), "a member may not");
}

/// 🔢️ Every node a tree carries, the way the shell's panel projection spends one retained record per node.
fn node_count(node: &UiNode) -> usize {
    match node {
        UiNode::Stack(stack) => 1 + stack.children.iter().map(node_count).sum::<usize>(),
        _ => 1,
    }
}

/// 🪟️ A user with many spaces still gets a workspace that fits one retained panel document: the first
/// `HUB_WORKSPACE_VISIBLE_SPACE_ROWS` rows the search leaves, the open space wherever it sorts, and a line naming
/// how many more the search reaches — in both tongues; a search that leaves few rows shows them all.
#[test]
fn many_spaces_fit_one_panel_document_with_the_open_space_and_a_count_of_the_rest() {
    let mut state = signed_in(workspace());
    state.rows = (0..64).map(|index| row(&format!("s{index:02}"), SpaceAccess::Author)).collect();
    state.open_space_id = Some("s63".into());
    for locale in [Locale::En, Locale::De] {
        let tree = build_hub_workspace_ui(&state, locale);
        assert!(node_count(&tree) <= ui_contract::UI_DOCUMENT_NODES, "{locale:?}: {} nodes", node_count(&tree));
        let spaces: Vec<String> = collected(&tree, attributes).into_iter().filter(|(key, _)| key == "data-semio-hub-space").map(|(_, value)| value).collect();
        assert_eq!(spaces.len(), HUB_WORKSPACE_VISIBLE_SPACE_ROWS + 1, "{locale:?}: the first rows plus the open one");
        assert!(spaces.contains(&"s63".to_string()), "{locale:?}: the open space stays reachable");
        let hidden = collected(&tree, attributes).into_iter().find(|(key, _)| key == "data-semio-hub-spaces-hidden").map(|(_, value)| value);
        assert_eq!(hidden.as_deref(), Some("55"), "{locale:?}: 64 − 8 shown − the open one");
    }
    state.search_draft = "s6".into();
    let narrowed = build_hub_workspace_ui(&state, Locale::En);
    assert!(collected(&narrowed, attributes).iter().all(|(key, _)| key != "data-semio-hub-spaces-hidden"), "a narrow search shows every match");
}

#[test]
fn a_stale_hub_keeps_every_row_rendered_and_openable() {
    let mut state = signed_in(workspace());
    state.phase = SpaceBrowserPhase::Stale;
    state.rows = vec![row("a1", SpaceAccess::Author)];
    let tree = build_hub_workspace_ui(&state, Locale::En);
    let open = format!("{HUB_SPACES_LIST_ID}.open.a1");
    let buttons = collected(&tree, enabled_buttons);
    assert_eq!(buttons.iter().find(|(id, _)| id == &open).map(|(_, enabled)| *enabled), Some(true));
    let announced = collected(&tree, attributes);
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-spaces-phase" && value == "stale"));
}

#[test]
fn a_loading_hub_renders_its_rows_but_refuses_to_open_one() {
    let mut state = signed_in(workspace());
    state.phase = SpaceBrowserPhase::Loading;
    state.rows = vec![row("a1", SpaceAccess::Author)];
    let buttons = collected(&build_hub_workspace_ui(&state, Locale::En), enabled_buttons);
    let open = format!("{HUB_SPACES_LIST_ID}.open.a1");
    assert_eq!(buttons.iter().find(|(id, _)| id == &open).map(|(_, enabled)| *enabled), Some(false));
}

#[test]
fn the_search_field_narrows_the_rendered_rows() {
    let mut state = signed_in(workspace());
    state.rows = vec![row("a1", SpaceAccess::Author), row("b2", SpaceAccess::Author)];
    state.search_draft = "b2".into();
    let announced = collected(&build_hub_workspace_ui(&state, Locale::En), attributes);
    let spaces: Vec<&String> = announced.iter().filter(|(key, _)| key == "data-semio-hub-space").map(|(_, value)| value).collect();
    assert_eq!(spaces, vec!["b2"]);
}

#[test]
fn a_one_shot_invitation_link_is_rendered_with_a_copy_and_a_discard_control() {
    let mut state = signed_in(workspace());
    state.rows = vec![row("a1", SpaceAccess::Author)];
    state.invite_capability = Some("abcDEF012".into());
    assert!(!collected(&build_hub_workspace_ui(&state, Locale::En), control_ids).iter().any(|id| id == &format!("{HUB_SPACES_LIST_ID}.copy-invite")), "a renderer with no clipboard door paints no copy control");
    state.clipboard_available = true;
    let tree = build_hub_workspace_ui(&state, Locale::En);
    let strings = collected(&tree, texts);
    assert!(strings.iter().any(|value| value.contains("#semio-invite=abcDEF012")), "{strings:?}");
    let ids = collected(&tree, control_ids);
    assert!(ids.iter().any(|id| id == &format!("{HUB_SPACES_LIST_ID}.copy-invite")));
    assert!(ids.iter().any(|id| id == &format!("{HUB_SPACES_LIST_ID}.discard-invite")));
}

#[test]
fn the_redeem_control_refuses_a_capability_it_could_not_parse() {
    let mut state = signed_in(workspace());
    let redeem = format!("{HUB_SPACES_LIST_ID}.redeem");
    state.invite_draft = "abc DEF".into();
    let refused = collected(&build_hub_workspace_ui(&state, Locale::En), enabled_buttons);
    assert_eq!(refused.iter().find(|(id, _)| id == &redeem).map(|(_, enabled)| *enabled), Some(false));
    state.invite_draft = "https://hub.example.org/#semio-invite=abcDEF012".into();
    let accepted = collected(&build_hub_workspace_ui(&state, Locale::En), enabled_buttons);
    assert_eq!(accepted.iter().find(|(id, _)| id == &redeem).map(|(_, enabled)| *enabled), Some(true), "pasting a whole link is what humans actually do");
}

#[test]
fn a_redemption_denial_names_its_cause_rather_than_a_status() {
    let mut state = signed_in(workspace());
    state.redemption_error = Some(InviteRedemptionErrorCode::AlreadyMember);
    let announced = collected(&build_hub_workspace_ui(&state, Locale::En), attributes);
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-redemption-error" && value == "already-member"));
}

#[test]
fn the_roster_only_appears_once_a_space_is_open_and_names_who_is_here() {
    let mut state = signed_in(workspace());
    state.rows = vec![row("a1", SpaceAccess::Author)];
    assert!(!collected(&build_hub_workspace_ui(&state, Locale::En), control_ids).iter().any(|id| id == HUB_MEMBERS_LIST_ID));
    state.open_space_id = Some("a1".into());
    state.members = vec![SpaceMemberPresence { user_id: "u1".into(), display_name: "Ada".into(), role: DirectorySpaceRole::Author, owner: true, online: true }];
    let tree = build_hub_workspace_ui(&state, Locale::En);
    let announced = collected(&tree, attributes);
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-member" && value == "u1"));
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-member-online" && value == "true"));
    assert!(announced.iter().any(|(key, value)| key == "data-semio-hub-space-current" && value == "true"));
}

#[test]
fn the_whole_surface_renders_in_german_with_no_english_leaking_through() {
    let mut state = signed_in(workspace());
    state.rows = vec![row("a1", SpaceAccess::Author)];
    state.open_space_id = Some("a1".into());
    state.members = vec![SpaceMemberPresence { user_id: "u1".into(), display_name: "Ada".into(), role: DirectorySpaceRole::Author, owner: false, online: false }];
    let english = collected(&build_hub_workspace_ui(&state, Locale::En), texts);
    let german = collected(&build_hub_workspace_ui(&state, Locale::De), texts);
    assert_eq!(english.len(), german.len(), "the two languages must render the same tree shape");
    let differences = english.iter().zip(&german).filter(|(left, right)| left != right).count();
    assert!(differences >= 8, "only {differences} strings changed between the two locales: {english:?}");
}

#[test]
fn the_minted_capability_never_reaches_the_retained_tree() {
    let mut state = signed_in(workspace());
    state.password_draft = "correct horse".into();
    state.rows = vec![row("a1", SpaceAccess::Author)];
    let strings = collected(&build_hub_workspace_ui(&state, Locale::En), texts);
    assert!(!strings.iter().any(|value| value.contains("session.v1.")), "no session capability may be painted: {strings:?}");
    assert!(!strings.iter().any(|value| value == "correct horse"), "the signed-in surface renders no password field, so no draft can leak into it");
}

#[test]
fn every_verb_the_surface_dispatches_is_one_of_the_declared_hub_actions() {
    let mut state = signed_in(workspace());
    state.rows = vec![row("a1", SpaceAccess::Author)];
    state.invite_capability = Some("abcDEF012".into());
    state.clipboard_available = true;
    state.open_space_id = Some("a1".into());
    let declared = [
        action::CLOSE_WORKSPACE,
        action::REFRESH_SPACES,
        action::SELECT_CONNECTION,
        action::ADD_CONNECTION,
        action::FORGET_CONNECTION,
        action::SET_EMAIL,
        action::SET_PASSWORD,
        action::SET_ADDRESS,
        action::SET_SEARCH,
        action::SET_SPACE_NAME,
        action::SET_INVITE_TEXT,
        action::SIGN_IN,
        action::CANCEL_SIGN_IN,
        action::SIGN_OUT,
        action::OPEN_SPACE,
        action::CREATE_SPACE,
        action::CREATE_INVITE,
        action::COPY_INVITE_LINK,
        action::DISCARD_INVITE_LINK,
        action::REDEEM_INVITE,
        action::SELECT_ARTIFACT_KIND,
        action::SET_ARTIFACT_NAME,
        action::CREATE_ARTIFACT,
        action::CANCEL_ARTIFACT_CREATION,
        action::OPEN_CREATED_ARTIFACT,
    ];
    let mut seen = Vec::new();
    verbs(&build_hub_workspace_ui(&state, Locale::En), &mut seen);
    assert!(!seen.is_empty());
    for verb in &seen {
        assert!(declared.contains(&verb.as_str()), "the surface dispatches an undeclared verb {verb}");
    }
    for required in [action::SIGN_OUT, action::OPEN_SPACE, action::CREATE_INVITE, action::REDEEM_INVITE, action::COPY_INVITE_LINK] {
        assert!(seen.iter().any(|verb| verb == required), "{required} is declared but never reachable");
    }
}

fn verbs(node: &UiNode, out: &mut Vec<String>) {
    match node {
        UiNode::Button(button) => {
            out.push(button.action.action.clone());
            assert_eq!(button.action.controller_id, "framework");
        }
        UiNode::Input(input) => {
            out.push(input.on_change.action.clone());
            if let Some(submit) = input.on_submit.as_ref() {
                out.push(submit.action.clone());
            }
        }
        UiNode::Stack(stack) => {
            for child in &stack.children {
                verbs(child, out);
            }
        }
        _ => {}
    }
}

#[test]
fn a_signed_out_workspace_that_never_loaded_a_row_shows_only_the_sign_in_section() {
    let state = workspace();
    let ids = collected(&build_hub_workspace_ui(&state, Locale::En), control_ids);
    assert!(!ids.iter().any(|id| id == HUB_SPACES_LIST_ID), "there is nothing honest to list before a session exists");
    assert!(!ids.iter().any(|id| id == HUB_MEMBERS_LIST_ID));
    assert_eq!(hub_session_initial_state(LOCAL_BOOTSTRAP_HUB_CONNECTION_ID_V1), state.session);
}
//#endregion 🌲️SurfaceLaws

//#region 🔁️RepublishLaws
// 🔁️ Laws for the defect WGr measured in a live browser on 2026-09-20: the hub panel painted a
// stale enabled-state, so `Add a hub` stayed disabled after its own address had been typed and only
// woke up when an unrelated verb happened to run. The root fix is one `refresh_ui` at the tail of
// `handle_hub_workspace_action` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `//#region 🔐️HubWorkspaceLane`),
// which covers EVERY hub verb rather than the one that was noticed. These laws pin the half that is
// provable without a shell: the tree really is a pure function of the drafts, so a republish after
// any draft write is both necessary and sufficient. Each law changes exactly ONE field and asserts
// the control it governs — no second verb, no nudge.

fn control_enabled(state: &HubWorkspaceState, id: &str) -> bool {
    collected(&build_hub_workspace_ui(state, Locale::En), enabled_buttons).into_iter().find(|(button, _)| button == id).map(|(_, enabled)| enabled).unwrap_or_else(|| panic!("{id} is not painted at all"))
}

#[test]
fn an_address_draft_alone_decides_the_add_control() {
    let mut state = workspace();
    let add = format!("{HUB_SIGN_IN_FORM_ID}.add");
    assert!(!control_enabled(&state, &add), "an empty address has nothing to add");
    state.address_draft = "http://127.0.0.1:7501".into();
    assert!(control_enabled(&state, &add), "the typed address is the ONLY thing that changed, and it must be enough");
    state.address_draft = "   ".into();
    assert!(!control_enabled(&state, &add), "whitespace is not an origin");
}

#[test]
fn credentials_alone_decide_the_sign_in_control() {
    let mut state = workspace();
    let submit = format!("{HUB_SIGN_IN_FORM_ID}.submit");
    assert!(!control_enabled(&state, &submit));
    state.email_draft = "user1@semio.dev".into();
    assert!(!control_enabled(&state, &submit), "an email with no password is half a credential");
    state.password_draft = "collab e2e first human phrase".into();
    assert!(control_enabled(&state, &submit), "both drafts are present and nothing else changed");
    state.email_draft = "user1-at-semio.dev".into();
    assert!(!control_enabled(&state, &submit), "the email admission runs before the round trip, not after it");
}

#[test]
fn every_draft_field_is_read_by_the_tree_so_a_republish_is_never_wasted() {
    let base = build_hub_workspace_ui(&workspace(), Locale::En);
    for (name, mutate) in [
        ("address", (|state: &mut HubWorkspaceState| state.address_draft = "http://127.0.0.1:7501".into()) as fn(&mut HubWorkspaceState)),
        ("email", |state: &mut HubWorkspaceState| state.email_draft = "user1@semio.dev".into()),
        ("password", |state: &mut HubWorkspaceState| state.password_draft = "collab e2e first human phrase".into()),
    ] {
        let mut state = workspace();
        mutate(&mut state);
        assert_ne!(base, build_hub_workspace_ui(&state, Locale::En), "the {name} draft changes the tree, so the lane owes a republish after writing it");
    }
}
//#endregion 🔁️RepublishLaws

//#region 🚪️BrowserDoorLaws
// 🚪️ Laws for the second live finding: the wgpu shell's sign-in submit produced no request and no
// error row. The transport itself is real — `📇️directory-door/🦀️.rs`'s browser arm puts every hop
// through the page's `directory-http` door — so what these laws pin is the contract on both sides of
// that door and the fact that a refusal is always something the human can READ, in both tongues.

#[test]
fn the_browser_door_carries_the_same_mint_route_au3_proved_live() {
    let credential = HubSignInCredential { email: "user1@semio.dev".into(), password: "collab e2e first human phrase".into(), device_instance_id: "wgr-probe".into(), client_class: crate::hub_sign_in::HubSignInClientClass::Browser };
    let body = hub_session_mint_request_json(&credential).expect("a well-formed credential seals");
    let url = format!("http://127.0.0.1:7501{}", HUB_SESSION_MINT_PATH_V1);
    let request = crate::directory_door::encode_directory_door_request(HttpMethod::Post, &url, None, Some(body.as_bytes())).expect("the door encodes a POST");
    let parsed: serde_json::Value = serde_json::from_str(&request).expect("the door speaks JSON");
    assert_eq!(parsed["op"], crate::directory_door::DIRECTORY_DOOR_OP);
    assert_eq!(parsed["method"], "POST");
    assert_eq!(parsed["url"], url);
    assert!(parsed["url"].as_str().is_some_and(|url| url.ends_with("/auth/sessions")), "the route is AU1's own constant, not a second spelling");
    assert!(parsed["bearer"].is_null(), "a mint carries no bearer — it is what mints one");
    let sent: serde_json::Value = serde_json::from_str(parsed["body"].as_str().expect("the body crosses as text")).expect("the body is the mint request");
    assert_eq!(sent["schema"], "semio.hub.auth.credential-sign-in/v1");
    assert_eq!(sent["email"], "user1@semio.dev");
    assert_eq!(sent["clientClass"], "browser");
}

#[test]
fn a_door_answer_becomes_a_response_and_a_door_error_becomes_a_transport_error() {
    let ok = crate::directory_door::decode_directory_door_response(r#"{"status":200,"body":"{\"token\":\"t\"}"}"#).expect("a status answer decodes");
    assert_eq!(ok.status, 200);
    assert_eq!(String::from_utf8_lossy(&ok.body), "{\"token\":\"t\"}");
    assert!(crate::directory_door::decode_directory_door_response(r#"{"error":"fetch failed"}"#).is_err(), "a page-side failure must not be read as a 0-status success");
    assert!(crate::directory_door::decode_directory_door_response(r#"{}"#).is_err(), "an answer with no status is unreadable, not empty");
}

#[test]
fn every_sign_in_refusal_is_a_readable_row_in_both_tongues() {
    for code in [HubSignInErrorCode::Unreachable, HubSignInErrorCode::InvalidCredentials, HubSignInErrorCode::InvalidResponse, HubSignInErrorCode::MalformedRequest, HubSignInErrorCode::Cancelled] {
        let mut state = workspace();
        state.session.error = Some(code);
        for locale in [Locale::En, Locale::De] {
            let tree = build_hub_workspace_ui(&state, locale);
            let tagged = collected(&tree, attributes);
            assert!(tagged.iter().any(|(key, value)| key == "data-semio-hub-error" && value == code.as_str()), "{code:?} must reach the surface as a row an assistive technology can announce, not only as shell state",);
        }
        let english = hub_sign_in_error_text(Locale::En, code, None);
        let german = hub_sign_in_error_text(Locale::De, code, None);
        assert!(!english.trim().is_empty() && !german.trim().is_empty(), "{code:?} has an untranslated column");
        assert_ne!(english, german, "{code:?} paints the same string in both tongues, which is an untranslated key rather than a translation");
    }
}
//#endregion 🚪️BrowserDoorLaws

//#region 🌱️ArtifactCreationDoorLaws
/// 🧫️ React's language-agnostic creation-progress fixture: both locales' texts and the per-phase
/// cases React's `ArtifactCreationProgressNotice` is held to (`🔬️artifact-creation-ready-opening`).
fn creation_progress_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🔣️.json")).expect("artifact creation progress fixture")
}

/// 🗂️ The one-kind catalog of React's catalog-authority fixture, as the hub would serve it.
fn fixture_catalog() -> SpaceArtifactCreationCatalogV1 {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🏛️ShellHost/🧫️fixtures/🌱️artifact-creation/🪪️catalog-authority/🔣️.json")).expect("catalog authority fixture");
    let catalog = &fixture["catalog"];
    SpaceArtifactCreationCatalogV1 {
        schema: "semio.hub.space-artifact-creation-catalog/v1".into(),
        space_id: catalog["spaceId"].as_str().expect("space").into(),
        catalog_generation_id: catalog["catalogGenerationId"].as_str().expect("generation").into(),
        kinds: vec![serde_json::from_str(catalog["member"].as_str().expect("member")).expect("catalog member kind")],
    }
}

fn phase_of(name: &str) -> SpaceArtifactCreationPhaseV1 {
    match name {
        "accepted" => SpaceArtifactCreationPhaseV1::Accepted,
        "preparing" => SpaceArtifactCreationPhaseV1::Preparing,
        "ready" => SpaceArtifactCreationPhaseV1::Ready,
        "indeterminate" => SpaceArtifactCreationPhaseV1::Indeterminate,
        "failed" => SpaceArtifactCreationPhaseV1::Failed,
        "cancelled" => SpaceArtifactCreationPhaseV1::Cancelled,
        other => panic!("unknown phase {other}"),
    }
}

fn locale_of(name: &str) -> Locale {
    if name == "de" { Locale::De } else { Locale::En }
}

/// 🌱️ A signed-in workspace on the fixture's open space with its catalog ready.
fn creation_door() -> HubWorkspaceState {
    let mut state = signed_in(workspace());
    let catalog = fixture_catalog();
    state.open_space_id = Some(catalog.space_id.clone());
    state.creation = HubArtifactCreationState { catalog_phase: HubArtifactCatalogPhase::Ready, catalog: Some(catalog), kind_id: None, name_draft: String::new(), next_request_id: "1".repeat(32), operation: None };
    state
}

/// 🗣️ Every text the wgpu door paints is byte-identical to React's in both locales — the fixture is
/// the one owner React's own law is held to (`deepEqual(fixture.locales, ARTIFACT_CREATION_PROGRESS_TEXT_V1)`).
#[test]
fn the_creation_door_speaks_the_fixture_texts_in_both_tongues() {
    let fixture = creation_progress_fixture();
    for (locale_name, locale) in [("en", Locale::En), ("de", Locale::De)] {
        let words = &fixture["locales"][locale_name];
        for phase in ["accepted", "preparing", "ready", "indeterminate", "failed", "cancelled"] {
            assert_eq!(hub_artifact_creation_phase_text(phase_of(phase), locale), words["phases"][phase].as_str().expect("phase text"), "{locale_name} {phase}");
        }
        for (label, path) in [
            (HubArtifactCreationLabel::Heading, "/heading"),
            (HubArtifactCreationLabel::Cancel, "/cancel"),
            (HubArtifactCreationLabel::Cancelling, "/cancelling"),
            (HubArtifactCreationLabel::Opening, "/opening/active"),
            (HubArtifactCreationLabel::OpeningFailed, "/opening/failed"),
            (HubArtifactCreationLabel::OpenRetry, "/opening/retry"),
            (HubArtifactCreationLabel::CatalogLoading, "/catalog/loading"),
            (HubArtifactCreationLabel::CatalogReady, "/catalog/ready"),
            (HubArtifactCreationLabel::CatalogUnavailable, "/catalog/unavailable"),
        ] {
            assert_eq!(hub_artifact_creation_label(label, locale), words.pointer(path).and_then(serde_json::Value::as_str).expect("fixture text"), "{locale_name} {path}");
        }
    }
}

/// 🚦️ Each fixture case, painted by the wgpu door: the phase line and its role, Cancel exactly while
/// the case is cancellable, the cancelling line once asked, the opening line while it opens and the
/// retry control exactly when it is retryable.
#[test]
fn every_fixture_creation_phase_paints_its_role_and_controls() {
    let fixture = creation_progress_fixture();
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("id");
        let locale = locale_of(case["locale"].as_str().expect("locale"));
        let words = &fixture["locales"][case["locale"].as_str().expect("locale")];
        let phase = phase_of(case["phase"].as_str().expect("phase"));
        let mut state = creation_door();
        let catalog = state.creation.catalog.clone().expect("catalog");
        let kind = &catalog.kinds[0];
        let intent = SpaceArtifactCreateV1 { schema: "semio.hub.space-artifact-create/v1".into(), request_id: "2".repeat(32), expected_catalog_generation_id: catalog.catalog_generation_id.clone(), kind_id: kind.kind_id.clone(), name: "Shared Map".into() };
        let ready = (phase == SpaceArtifactCreationPhaseV1::Ready).then(|| SpaceArtifactCreationReadyV1 {
            artifact_id: format!("artifact-{}", "4".repeat(32)),
            kind_id: kind.kind_id.clone(),
            artifact_schema: kind.schema.clone(),
            parent_dialect: kind.dialect.clone(),
        });
        let opening = match case["openingDisposition"].as_str().expect("opening") {
            "opening" => HubArtifactOpening::Opening,
            "failed" => HubArtifactOpening::Failed,
            _ => HubArtifactOpening::Idle,
        };
        state.creation.operation = Some(HubArtifactCreation { intent, space_id: catalog.space_id.clone(), phase, submitted: true, cancel_requested: case["cancelRequested"].as_bool().expect("cancel"), cancel_sent: false, ready, opening, last_answered_at_ms: 0, polls: 0, next_poll_at_ms: 0 });
        let tree = build_hub_workspace_ui(&state, locale);
        let painted = collected(&tree, texts);
        let buttons = collected(&tree, enabled_buttons);
        let tagged = collected(&tree, attributes);
        assert!(painted.iter().any(|text| text == words["phases"][case["phase"].as_str().expect("phase")].as_str().expect("phase text")), "{id}: phase line");
        assert!(tagged.iter().any(|(key, value)| key == "data-semio-hub-artifact-creation-role" && value == case["role"].as_str().expect("role")), "{id}: role");
        let cancel = format!("{HUB_ARTIFACT_CREATION_ID}.cancel");
        assert_eq!(buttons.iter().any(|(button, enabled)| *button == cancel && *enabled), case["cancellable"].as_bool().expect("cancellable"), "{id}: cancel control");
        assert_eq!(painted.iter().any(|text| text == words["opening"]["active"].as_str().expect("opening")), case["opens"].as_bool().expect("opens"), "{id}: opening line");
        let retry = format!("{HUB_ARTIFACT_CREATION_ID}.open");
        assert_eq!(buttons.iter().any(|(button, _)| *button == retry), case["retryable"].as_bool().expect("retryable"), "{id}: retry control");
        if case["cancelRequested"].as_bool().expect("cancel") && !hub_artifact_creation_terminal(phase) {
            assert!(painted.iter().any(|text| text == words["cancelling"].as_str().expect("cancelling")), "{id}: cancelling line");
        }
    }
}

/// 📚️ The catalog line of each fixture catalog case, and whether the door offers any choice; a
/// `Ready` answer carrying no valid kind is shown as unavailable, never as an empty ready chooser.
#[test]
fn every_fixture_catalog_phase_paints_its_line_and_choices() {
    let fixture = creation_progress_fixture();
    for case in fixture["catalogCases"].as_array().expect("catalog cases") {
        let id = case["id"].as_str().expect("id");
        let locale_name = case["locale"].as_str().expect("locale");
        let mut state = creation_door();
        state.creation.catalog_phase = match case["phase"].as_str().expect("phase") {
            "loading" => HubArtifactCatalogPhase::Loading,
            "ready" => HubArtifactCatalogPhase::Ready,
            _ => HubArtifactCatalogPhase::Unavailable,
        };
        if id.starts_with("catalog-empty-invalid") {
            state.creation.catalog.as_mut().expect("catalog").kinds.clear();
        }
        let tree = build_hub_workspace_ui(&state, locale_of(locale_name));
        let painted = collected(&tree, texts);
        let effective = case["effectivePhase"].as_str().expect("effective");
        assert!(painted.iter().any(|text| text == fixture["locales"][locale_name]["catalog"][effective].as_str().expect("catalog text")), "{id}: catalog line");
        let tagged = collected(&tree, attributes);
        assert!(tagged.iter().any(|(key, value)| key == "data-semio-hub-artifact-catalog-role" && value == case["role"].as_str().expect("role")), "{id}: catalog role");
        let choices = collected(&tree, control_ids).iter().filter(|control| control.starts_with(&format!("{HUB_ARTIFACT_CREATION_ID}.kind."))).count();
        assert_eq!(choices > 0, case["hasChoices"].as_bool().expect("choices"), "{id}: choices");
    }
}

/// 📥️ The sealed intent names exactly the chosen catalog row, the catalog generation it was chosen
/// from, the trimmed name and the door's pre-minted key; no choice, a blank name or a creation still
/// in flight seals nothing.
#[test]
fn the_door_seals_one_intent_for_a_chosen_kind_and_a_valid_name() {
    let mut state = creation_door();
    state.creation.name_draft = "  Shared Map ".into();
    assert_eq!(hub_artifact_creation_intent(&state), None, "no kind chosen yet");
    let kind_id = state.creation.catalog.as_ref().expect("catalog").kinds[0].kind_id.clone();
    state.creation.kind_id = Some(kind_id.clone());
    let intent = hub_artifact_creation_intent(&state).expect("a chosen kind and a name seal an intent");
    assert_eq!(
        (intent.request_id.as_str(), intent.expected_catalog_generation_id.as_str(), intent.kind_id.as_str(), intent.name.as_str()),
        ("1".repeat(32).as_str(), fixture_catalog().catalog_generation_id.as_str(), kind_id.as_str(), "Shared Map")
    );
    assert!(intent.validate());
    let buttons = collected(&build_hub_workspace_ui(&state, Locale::En), enabled_buttons);
    assert!(buttons.contains(&(format!("{HUB_ARTIFACT_CREATION_ID}.create"), true)));
    state.creation.operation = Some(HubArtifactCreation { intent, space_id: "space-a".into(), phase: SpaceArtifactCreationPhaseV1::Preparing, submitted: true, cancel_requested: false, cancel_sent: false, ready: None, opening: HubArtifactOpening::Idle, last_answered_at_ms: 0, polls: 0, next_poll_at_ms: 0 });
    assert_eq!(hub_artifact_creation_intent(&state), None, "one creation at a time");
    state.creation.operation = None;
    state.creation.name_draft = "   ".into();
    assert_eq!(hub_artifact_creation_intent(&state), None, "a blank name seals nothing");
}
//#endregion 🌱️ArtifactCreationDoorLaws

/// ⏱️ The door follows a creation exactly as the browser does: every poll delay and the unreachable bound
/// come from the shared contract (`🏪️store/👷️worker/🌱️creation-polling/🔣️.json`), recomputed here from its
/// raw numbers — a slow hub that still answers is waited for, never concluded.
#[test]
fn the_creation_door_follows_the_shared_polling_contract() {
    let contract: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🏪️store/👷️worker/🌱️creation-polling/🔣️.json")).expect("contract");
    let (initial, maximum, bound) = (contract["pollInitialMs"].as_u64().unwrap(), contract["pollMaxMs"].as_u64().unwrap(), contract["unreachableBoundMs"].as_u64().unwrap());
    let mut expected = initial;
    for attempt in 0..40 {
        assert_eq!(space_artifact_creation_poll_delay_ms(attempt), expected.min(maximum), "attempt {attempt}");
        expected = expected.saturating_mul(2).min(maximum);
    }
    assert!(!space_artifact_creation_unreachable(1_000, 1_000 + bound - 1), "an answer inside the bound keeps the creation followed");
    assert!(space_artifact_creation_unreachable(1_000, 1_000 + bound), "a hub silent for the whole bound is unreachable");
    assert!(!space_artifact_creation_unreachable(1_000, 500), "a clock read before the answer is never unreachable");
}
