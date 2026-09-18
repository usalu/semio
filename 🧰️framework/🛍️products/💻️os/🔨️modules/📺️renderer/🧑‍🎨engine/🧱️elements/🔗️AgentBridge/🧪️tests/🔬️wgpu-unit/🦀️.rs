//! 🧪️ wgpu `🔗️AgentBridge` unit tests — the anti-drift replay plus the consumer's own state rules.
//!
//! The replay is the mechanism the Rust SSOT (`💻️os/🔨️modules/🌉️mcp/🧵️bridge/🦀️.rs`) names in its own
//! module docstring: `🧫️fixtures/📨️frames.json` carries a hex encoding for every frame variant in
//! both directions, and a codec that disagrees with one byte of it has drifted. The SSOT's
//! `mod quick` replays it; the `🟦️.ts` twin replays it; this third implementation replays it too.

use super::*;

/// 🧾️ The shared, cross-implementation frame corpus — the same file the Rust SSOT and the TS twin
/// assert against, read at compile time so a moved fixture is a build error, not a skipped test.
const FRAME_FIXTURES: &str = include_str!("../../../../../../🌉️mcp/🧵️bridge/🧫️fixtures/📨️frames.json");

/// 📨️ Shell→Gateway variants this shell deliberately does not model — the four snapshot-carrying
/// frames that only a `ShellState`-mirroring shell produces (see [`ShellToGateway`]'s own doc).
const UNMODELLED_SHELL_TO_GATEWAY: [&str; 4] = ["ShellState", "ShellStatePatch", "Instances", "AppFrames"];

fn decode_hex(hex: &str) -> Vec<u8> {
    assert!(hex.len() % 2 == 0, "fixture hex must be byte-aligned: {hex}");
    (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16).expect("fixture hex digit")).collect()
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn fixtures() -> Vec<serde_json::Value> {
    serde_json::from_str::<Vec<serde_json::Value>>(FRAME_FIXTURES).expect("frame fixtures parse")
}

/// 🧾️ The corpus carries more rows than variants (two `AgentPresence`s, two `Approval`s, …), so
/// coverage is asserted over the distinct variant names, not the row count.
fn distinct_variants(covered: &[&str]) -> usize {
    let mut names: Vec<&str> = covered.to_vec();
    names.sort_unstable();
    names.dedup();
    names.len()
}

#[test]
fn every_gateway_to_shell_fixture_round_trips_through_this_codec() {
    let rows = fixtures();
    let mut seen: Vec<&str> = Vec::new();
    for row in &rows {
        if row["direction"] != "gateway_to_shell" {
            continue;
        }
        let variant = row["variant"].as_str().expect("fixture variant");
        let hex = row["hex"].as_str().expect("fixture hex");
        let bytes = decode_hex(hex);
        let frame = GatewayToShell::decode(&bytes).unwrap_or_else(|fault| panic!("{variant} did not decode: {fault:?}"));
        assert_eq!(encode_hex(&frame.encode()), hex, "{variant} re-encoded to different bytes");
        seen.push(variant);
    }
    assert_eq!(distinct_variants(&seen), 10, "the gateway→shell corpus must cover all ten tags, saw {seen:?}");
}

#[test]
fn every_modelled_shell_to_gateway_fixture_round_trips_through_this_codec() {
    let rows = fixtures();
    let mut seen: Vec<&str> = Vec::new();
    for row in &rows {
        if row["direction"] != "shell_to_gateway" {
            continue;
        }
        let variant = row["variant"].as_str().expect("fixture variant");
        if UNMODELLED_SHELL_TO_GATEWAY.contains(&variant) {
            continue;
        }
        let hex = row["hex"].as_str().expect("fixture hex");
        let bytes = decode_hex(hex);
        let frame = ShellToGateway::decode(&bytes).unwrap_or_else(|fault| panic!("{variant} did not decode: {fault:?}"));
        assert_eq!(encode_hex(&frame.encode()), hex, "{variant} re-encoded to different bytes");
        seen.push(variant);
    }
    assert_eq!(distinct_variants(&seen), 6, "Hello + ShellCommandResult + Approval + Ping + Bye + AgentMessage — the ten SSOT variants minus the unmodelled four, saw {seen:?}");
}

#[test]
fn a_truncated_or_trailing_frame_is_a_fault_not_a_panic() {
    assert_eq!(GatewayToShell::decode(&[3, 4, 0, 0, 0]), Err(BridgeFrameFault::Truncated));
    assert_eq!(GatewayToShell::decode(&[6, 0]), Err(BridgeFrameFault::TrailingBytes));
    assert_eq!(GatewayToShell::decode(&[99]), Err(BridgeFrameFault::UnknownTag(99)));
    assert_eq!(GatewayToShell::decode(&[]), Err(BridgeFrameFault::Truncated));
}

#[test]
fn a_simulated_approval_requested_frame_parks_a_pending_approval() {
    let mut state = AgentBridgeState::default();
    let frame = GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "translate selection by (1,0,0)".into() };
    state.apply_encoded_frame(&frame.encode(), 1_000.0).expect("frame decodes");
    assert!(state.has_pending_approvals());
    assert_eq!(state.pending_approvals.len(), 1);
    assert_eq!(state.pending_approvals[0].approval_id, "appr_1");
    assert_eq!(state.pending_approvals[0].requested_at_ms, 1_000.0);
}

#[test]
fn a_repeated_approval_id_replaces_rather_than_duplicates() {
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "first".into() }, 1.0);
    state.apply_frame(GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "second".into() }, 2.0);
    assert_eq!(state.pending_approvals.len(), 1);
    assert_eq!(state.pending_approvals[0].summary, "second");
}

#[test]
fn each_decision_emits_the_matching_approval_frame_and_clears_the_request() {
    for decision in ApprovalDecision::ALL {
        let mut state = AgentBridgeState::default();
        state.apply_frame(GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "s".into() }, 0.0);
        state.resolve_approval("appr_1", decision, None);
        assert!(!state.has_pending_approvals(), "{decision:?} must clear the request");
        let outbox = state.take_outbox();
        assert_eq!(outbox, vec![ShellToGateway::Approval { approval_id: "appr_1".into(), decision, note: None }]);
        let re_decoded = ShellToGateway::decode(&outbox[0].encode()).expect("emitted frame decodes");
        assert_eq!(re_decoded, outbox[0]);
    }
}

#[test]
fn a_gateway_side_resolution_withdraws_the_request_without_emitting_anything() {
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "s".into() }, 0.0);
    state.apply_frame(GatewayToShell::ApprovalResolved { approval_id: "appr_1".into(), decision: ApprovalDecision::Deny }, 1.0);
    assert!(!state.has_pending_approvals());
    assert_eq!(state.outbox_len(), 0);
}

#[test]
fn presence_frames_drive_the_presence_record() {
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: "conn_1".into(), principal: "agent:local".into() }, 0.0);
    assert_eq!(state.status, AgentBridgeStatus::Open);
    state.apply_frame(GatewayToShell::AgentPresence { active: true, label: "claude-code".into(), invocation_id: Some("inv-1".into()) }, 1.0);
    assert_eq!(state.presence, AgentBridgePresence { active: true, label: "claude-code".into(), invocation_id: Some("inv-1".into()) });
    state.apply_frame(GatewayToShell::AgentPresence { active: false, label: String::new(), invocation_id: None }, 2.0);
    assert!(!state.presence.active);
}

#[test]
fn a_welcome_clears_the_reconnect_attempt_and_a_close_raises_it() {
    let mut state = AgentBridgeState::default();
    state.note_connecting();
    assert_eq!(state.status, AgentBridgeStatus::Connecting);
    state.note_socket_closed();
    assert_eq!(state.status, AgentBridgeStatus::Reconnecting);
    assert_eq!(state.reconnect_attempt, 1);
    state.note_connecting();
    assert_eq!(state.status, AgentBridgeStatus::Reconnecting);
    state.apply_frame(GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: "c".into(), principal: "p".into() }, 0.0);
    assert_eq!(state.reconnect_attempt, 0);
}

#[test]
fn reconnect_backoff_doubles_and_saturates() {
    assert_eq!(reconnect_delay_ms(0), RECONNECT_BASE_MS);
    assert_eq!(reconnect_delay_ms(1), RECONNECT_BASE_MS);
    assert_eq!(reconnect_delay_ms(2), 2_000.0);
    assert_eq!(reconnect_delay_ms(3), 4_000.0);
    assert_eq!(reconnect_delay_ms(30), RECONNECT_MAX_MS);
}

#[test]
fn a_socket_open_announces_this_shell_kind() {
    let mut state = AgentBridgeState::default();
    state.note_socket_opened("shell-1", "agent:local", BridgeFlags::NONE);
    let frames = state.take_outbox();
    match &frames[0] {
        ShellToGateway::Hello { bridge_version, shell_kind, shell_session_id, .. } => {
            assert_eq!(*bridge_version, BRIDGE_VERSION);
            assert_eq!(*shell_kind, ShellKind::WgpuNative);
            assert_eq!(shell_session_id, "shell-1");
        }
        other => panic!("expected Hello, got {other:?}"),
    }
}

#[test]
fn an_inbound_shell_command_is_refused_rather_than_silently_dropped() {
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::ShellCommand { seq: 7, command: vec![1, 2] }, 0.0);
    let frames = state.take_outbox();
    match &frames[0] {
        ShellToGateway::ShellCommandResult { in_reply_to, ok, fault } => {
            assert_eq!(*in_reply_to, 7);
            assert!(!ok);
            assert!(fault.is_some());
        }
        other => panic!("expected ShellCommandResult, got {other:?}"),
    }
}

#[test]
fn a_bye_frame_closes_the_bridge_and_keeps_its_reason() {
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::Bye { reason: "shutdown".into() }, 0.0);
    assert_eq!(state.status, AgentBridgeStatus::Closed);
    assert_eq!(state.last_error.as_deref(), Some("shutdown"));
}

/// 💬️ LAW: a tool call becomes its own RESULT, in place — React's `updateConversationEntry`, not a
/// second row. A result whose call is not in the conversation records nothing at all rather than
/// opening a call-less row the panel cannot render.
#[test]
fn a_tool_result_settles_its_own_call_in_place_and_an_orphan_result_records_nothing() {
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::AgentToolCall { invocation_id: "inv_1".into(), tool_name: "translate".into(), arguments: "{\"dx\":1}".into() }, 0.0);
    assert_eq!(state.conversation.len(), 1);
    state.apply_frame(GatewayToShell::AgentToolResult { invocation_id: "inv_1".into(), tool_name: "translate".into(), ok: false, summary: "refused".into() }, 1.0);
    assert_eq!(state.conversation.len(), 1);
    match &state.conversation[0] {
        AgentConversationEntry::ToolCall { tool_name, arguments, state: call_state, summary, .. } => {
            assert_eq!(tool_name, "translate");
            assert_eq!(arguments, "{\"dx\":1}");
            assert_eq!(*call_state, AgentToolCallState::Failed);
            assert_eq!(summary.as_deref(), Some("refused"));
        }
        other => panic!("expected ToolCall, got {other:?}"),
    }
    state.apply_frame(GatewayToShell::AgentToolResult { invocation_id: "inv_missing".into(), tool_name: "translate".into(), ok: true, summary: "done".into() }, 2.0);
    assert_eq!(state.conversation.len(), 1);
}

/// ⏸️ LAW: an approval is ONE conversation row across its whole life — requested, then resolved in
/// place, whether the gateway resolves it or this shell's own human does.
#[test]
fn an_approval_is_one_conversation_row_from_request_to_decision() {
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::ApprovalRequested { approval_id: "appr_1".into(), summary: "translate".into() }, 0.0);
    state.resolve_approval("appr_1", ApprovalDecision::Once, None);
    assert!(!state.has_pending_approvals());
    assert_eq!(state.conversation.len(), 1);
    match &state.conversation[0] {
        AgentConversationEntry::Approval { state: approval_state, decision, .. } => {
            assert_eq!(*approval_state, AgentApprovalState::Resolved);
            assert_eq!(*decision, Some(ApprovalDecision::Once));
        }
        other => panic!("expected Approval, got {other:?}"),
    }
}

/// 💬️ LAW: a human turn is echoed the moment it is queued, under the SAME id the frame carries, and
/// blank text queues nothing. The conversation is a bounded live view, never an archive.
#[test]
fn a_human_turn_is_echoed_under_its_own_frame_id_and_the_view_stays_bounded() {
    let mut state = AgentBridgeState::default();
    assert!(!state.send_agent_message("shell-1", "   "));
    assert!(state.conversation.is_empty());
    assert!(state.send_agent_message("shell-1", "  move it  "));
    let queued = state.take_outbox();
    match &queued[0] {
        ShellToGateway::AgentMessage { message_id, text } => {
            assert_eq!(text, "move it");
            assert_eq!(state.conversation[0], AgentConversationEntry::UserMessage { id: message_id.clone(), text: "move it".into() });
        }
        other => panic!("expected AgentMessage, got {other:?}"),
    }
    for index in 0..AGENT_CONVERSATION_MAX_ENTRIES {
        assert!(state.send_agent_message("shell-1", &format!("turn-{index}")));
    }
    assert_eq!(state.conversation.len(), AGENT_CONVERSATION_MAX_ENTRIES);
    assert_eq!(state.conversation[AGENT_CONVERSATION_MAX_ENTRIES - 1], AgentConversationEntry::UserMessage { id: format!("msg_shell-1_{AGENT_CONVERSATION_MAX_ENTRIES}"), text: format!("turn-{}", AGENT_CONVERSATION_MAX_ENTRIES - 1) });
}
