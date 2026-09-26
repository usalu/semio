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
const CANCELLATION_FIXTURE: &str = include_str!("../../🧫️fixtures/🛑️cancellation/🔣️.json");
const HANDSHAKE_FIXTURE: &str = include_str!("../../🧫️fixtures/🤝️handshake/🔣️.json");

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
    assert_eq!(distinct_variants(&seen), 13, "the gateway→shell corpus must cover all thirteen tags — including `AgentReply` (10), `ApprovalWithdrawn` (11) and `Refused` (12), saw {seen:?}");
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
    assert_eq!(distinct_variants(&seen), 7, "Hello + ShellCommandResult + Approval + Ping + Bye + AgentMessage + AgentCancel — the eleven SSOT variants minus the unmodelled four, saw {seen:?}");
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

/// 🪦️ Replays `🌉️mcp/🛡️policy/🧫️fixtures/🪦️approval-withdrawal.json`'s `retiredAffordances` through this
/// shell's own decoder and state: a withdrawn request leaves nothing waiting and its row says why.
#[test]
fn a_withdrawn_approval_retires_its_affordance_and_keeps_the_reason() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🌉️mcp/🛡️policy/🧫️fixtures/🪦️approval-withdrawal.json")).expect("approval withdrawal law");
    for row in law["retiredAffordances"].as_array().expect("retiredAffordances") {
        let name = row["name"].as_str().expect("name");
        let mut state = AgentBridgeState::default();
        for frame in row["frames"].as_array().expect("frames") {
            let decoded = match frame["variant"].as_str().expect("variant") {
                "approvalRequested" => GatewayToShell::ApprovalRequested { approval_id: "appr_law".into(), summary: "{}".into() },
                "approvalResolved" => GatewayToShell::ApprovalResolved { approval_id: "appr_law".into(), decision: ApprovalDecision::Deny },
                "approvalWithdrawn" => GatewayToShell::ApprovalWithdrawn {
                    approval_id: "appr_law".into(),
                    reason: match frame["reason"].as_str().expect("reason") {
                        "cancelled" => ApprovalWithdrawal::Cancelled,
                        "timed_out" => ApprovalWithdrawal::TimedOut,
                        "superseded" => ApprovalWithdrawal::Superseded,
                        other => panic!("{name}: unknown reason {other}"),
                    },
                },
                other => panic!("{name}: unexpected frame {other}"),
            };
            state.apply_encoded_frame(&decoded.encode(), 0.0).expect("frame decodes");
        }
        assert_eq!(state.pending_approvals.len() as u64, row["pending"].as_u64().expect("pending"), "{name}");
        let Some(AgentConversationEntry::Approval { state: approval, .. }) = state.conversation.iter().find(|entry| entry.id() == "appr_law") else { panic!("{name}: no approval row") };
        let (expected_state, expected_reason) = (row["state"].as_str().expect("state"), row["withdrawal"].as_str());
        let observed = match approval {
            AgentApprovalState::Pending => ("pending", None),
            AgentApprovalState::Resolved => ("resolved", None),
            AgentApprovalState::Withdrawn(ApprovalWithdrawal::Cancelled) => ("withdrawn", Some("cancelled")),
            AgentApprovalState::Withdrawn(ApprovalWithdrawal::TimedOut) => ("withdrawn", Some("timed_out")),
            AgentApprovalState::Withdrawn(ApprovalWithdrawal::Superseded) => ("withdrawn", Some("superseded")),
        };
        assert_eq!(observed, (expected_state, expected_reason), "{name}");
    }
    let de = crate::agent_approvals::approvals_withdrawal_label(ApprovalWithdrawal::TimedOut, Locale::De);
    let en = crate::agent_approvals::approvals_withdrawal_label(ApprovalWithdrawal::TimedOut, Locale::En);
    assert!(de.contains("niemand hat rechtzeitig entschieden") && en.contains("nobody decided in time"), "{en} / {de}");
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

/// 🛑️ LAW: the WGPU consumer follows the same neutral cancellation sequence as the real React
/// hook, carrying the gateway's invocation id byte-for-byte and waiting for a real result to settle.
#[test]
fn cancellation_matches_the_shared_react_lifecycle_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(CANCELLATION_FIXTURE).expect("cancellation fixture parses");
    let invocation_id = fixture["invocation"]["id"].as_str().expect("invocation id");
    let tool_name = fixture["invocation"]["toolName"].as_str().expect("tool name");
    let arguments = fixture["invocation"]["arguments"].as_str().expect("arguments");
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: "gateway-1".into(), principal: "agent:test".into() }, 0.0);
    state.apply_frame(GatewayToShell::AgentToolCall { invocation_id: invocation_id.into(), tool_name: tool_name.into(), arguments: arguments.into() }, 1.0);

    assert_eq!(state.cancel_tool_call(invocation_id), fixture["openCancellation"]["accepted"].as_bool().expect("open accepted"));
    assert_eq!(state.take_outbox(), vec![ShellToGateway::AgentCancel { invocation_id: invocation_id.into() }]);
    assert!(matches!(&state.conversation[0], AgentConversationEntry::ToolCall { id, state: AgentToolCallState::Cancelling, .. } if id == invocation_id));

    state.note_socket_closed();
    assert_eq!(state.cancel_tool_call(invocation_id), fixture["closedCancellation"]["accepted"].as_bool().expect("closed accepted"));
    assert_eq!(state.outbox_len(), fixture["closedCancellation"]["outboundCount"].as_u64().expect("closed outbound count") as usize);
    assert!(matches!(&state.conversation[0], AgentConversationEntry::ToolCall { state: AgentToolCallState::Cancelling, .. }));

    let ok = fixture["terminalResult"]["ok"].as_bool().expect("terminal ok");
    let summary = fixture["terminalResult"]["summary"].as_str().expect("terminal summary");
    state.apply_frame(GatewayToShell::AgentToolResult { invocation_id: invocation_id.into(), tool_name: tool_name.into(), ok, summary: summary.into() }, 2.0);
    assert!(matches!(&state.conversation[0], AgentConversationEntry::ToolCall { state: AgentToolCallState::Failed, summary: Some(actual), .. } if actual == summary));
}

/// ✂️ A result arriving after the bounded transcript retired its call cannot resurrect a call-less
/// row or a cancellation affordance.
#[test]
fn a_retired_cancelled_invocation_stays_retired_when_its_result_arrives() {
    let fixture: serde_json::Value = serde_json::from_str(CANCELLATION_FIXTURE).expect("cancellation fixture parses");
    let invocation_id = fixture["invocation"]["id"].as_str().expect("invocation id");
    let maximum = fixture["retirement"]["maximumEntries"].as_u64().expect("maximum entries") as usize;
    assert_eq!(maximum, AGENT_CONVERSATION_MAX_ENTRIES);
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: "gateway-1".into(), principal: "agent:test".into() }, 0.0);
    state.apply_frame(GatewayToShell::AgentToolCall { invocation_id: invocation_id.into(), tool_name: "inference_run".into(), arguments: "{}".into() }, 1.0);
    assert!(state.cancel_tool_call(invocation_id));
    for index in 0..maximum {
        assert!(state.send_agent_message("shell-1", &format!("turn-{index}")));
    }
    assert_eq!(state.conversation.len(), maximum);
    assert!(state.conversation.iter().all(|entry| entry.id() != invocation_id), "the oldest cancelled call is retired");
    state.apply_frame(GatewayToShell::AgentToolResult { invocation_id: invocation_id.into(), tool_name: "inference_run".into(), ok: false, summary: "cancelled by user".into() }, 2.0);
    assert_eq!(state.conversation.len(), maximum);
    assert_eq!(state.conversation.iter().any(|entry| entry.id() == invocation_id), fixture["retirement"]["lateResultCreatesEntry"].as_bool().expect("late result policy"));
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

//#region 🔖️DialLadder
// 🌉️ WGPU-RENDERER-REACT-PARITY packet W15e — the socket lifecycle W1j left to a future transport.
// `AgentBridgeDialer` is that lifecycle held apart from any socket, so React's own effect (dial →
// hello → 20 s ping → doubling backoff capped at 30 s) is provable without a gateway.

#[test]
fn a_bridge_config_is_admitted_only_as_a_websocket_url_with_a_proof() {
    assert!(AgentBridgeConfig::admit("ws://127.0.0.1:6300/bridge", "session.v1.selector.proof").is_ok());
    assert!(AgentBridgeConfig::admit("wss://gateway.example/bridge", "proof").is_ok());
    assert!(AgentBridgeConfig::admit("http://127.0.0.1:6300/bridge", "proof").is_err(), "an http url is not a socket url");
    assert!(AgentBridgeConfig::admit("", "proof").is_err());
    assert!(AgentBridgeConfig::admit("ws://host", "").is_err());
    assert!(AgentBridgeConfig::admit(&format!("ws://{}", "x".repeat(AGENT_BRIDGE_FIELD_MAX_BYTES)), "proof").is_err());
}

#[test]
fn the_admission_proof_never_reaches_a_debug_line() {
    let config = AgentBridgeConfig::admit("ws://127.0.0.1:6300/bridge", "session.v1.secret.proof").expect("admitted");
    let printed = format!("{config:?}");
    assert!(!printed.contains("session.v1.secret.proof"), "the proof must never be printable: {printed}");
    assert!(printed.contains("admission_proof_len"));
}

#[test]
fn the_protocol_offer_is_reacts_exact_ordered_pair() {
    let config = AgentBridgeConfig::admit("ws://127.0.0.1:6300/bridge", "session.v1.selector.proof").expect("admitted");
    assert_eq!(bridge_protocols(&config), ["semio.mcp.bridge.v1".to_string(), "session.v1.selector.proof".to_string()]);
    assert_eq!(bridge_protocols(&config)[0], BRIDGE_SUBPROTOCOL);
}

#[test]
fn discovery_is_disabled_exactly_as_reacts_own_is() {
    assert!(discover_agent_bridge_config().is_none());
}

#[test]
fn an_unconfigured_dialer_stays_disabled_and_dials_nothing() {
    let mut state = AgentBridgeState::default();
    let mut dialer = AgentBridgeDialer::default();
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Absent, 0.0), AgentBridgeDialTurn::Idle);
    assert_eq!(state.status, AgentBridgeStatus::Disabled);
    assert!(!dialer.is_armed());
}

#[test]
fn a_config_arms_the_ladder_and_the_first_turn_dials_with_the_exact_protocols() {
    let mut state = AgentBridgeState::default();
    let mut dialer = AgentBridgeDialer::default();
    let config = AgentBridgeConfig::admit("ws://127.0.0.1:6300/bridge", "proof").expect("admitted");
    assert!(dialer.set_config(Some(config), &mut state));
    assert_eq!(state.status, AgentBridgeStatus::Connecting);
    match dialer.turn(&mut state, AgentBridgeSocketState::Absent, 0.0) {
        AgentBridgeDialTurn::Dial { url, protocols } => {
            assert_eq!(url, "ws://127.0.0.1:6300/bridge");
            assert_eq!(protocols, ["semio.mcp.bridge.v1".to_string(), "proof".to_string()]);
        }
        other => panic!("expected a dial, got {other:?}"),
    }
}

#[test]
fn a_connecting_socket_is_waited_on_rather_than_dialled_again() {
    let mut state = AgentBridgeState::default();
    let mut dialer = AgentBridgeDialer::default();
    dialer.set_config(Some(AgentBridgeConfig::admit("ws://host/bridge", "proof").expect("admitted")), &mut state);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Connecting, 0.0), AgentBridgeDialTurn::Idle);
    assert_eq!(state.status, AgentBridgeStatus::Connecting);
}

#[test]
fn an_opened_socket_announces_exactly_once_and_then_pings_on_reacts_own_cadence() {
    let mut state = AgentBridgeState::default();
    let mut dialer = AgentBridgeDialer::default();
    dialer.set_config(Some(AgentBridgeConfig::admit("ws://host/bridge", "proof").expect("admitted")), &mut state);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, 1_000.0), AgentBridgeDialTurn::Announce);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, 1_001.0), AgentBridgeDialTurn::Idle, "announce happens once, not every turn");
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, 1_000.0 + PING_INTERVAL_MS - 1.0), AgentBridgeDialTurn::Idle);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, 1_000.0 + PING_INTERVAL_MS), AgentBridgeDialTurn::Ping);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, 1_000.0 + PING_INTERVAL_MS + 1.0), AgentBridgeDialTurn::Idle);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, 1_000.0 + PING_INTERVAL_MS * 2.0), AgentBridgeDialTurn::Ping);
}

#[test]
fn a_closed_socket_is_retired_and_its_redial_waits_the_backoff_react_computes() {
    let mut state = AgentBridgeState::default();
    let mut dialer = AgentBridgeDialer::default();
    dialer.set_config(Some(AgentBridgeConfig::admit("ws://host/bridge", "proof").expect("admitted")), &mut state);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, 0.0), AgentBridgeDialTurn::Announce);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Closed, 10_000.0), AgentBridgeDialTurn::Retire);
    assert_eq!(state.reconnect_attempt, 1);
    // ⏱️ attempt 1 → RECONNECT_BASE_MS, the first step of `scheduleReconnect`'s own ladder.
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Absent, 10_000.0), AgentBridgeDialTurn::Wait { until_ms: 10_000.0 + RECONNECT_BASE_MS });
    assert_eq!(state.status, AgentBridgeStatus::Reconnecting);
    assert!(matches!(dialer.turn(&mut state, AgentBridgeSocketState::Absent, 10_000.0 + RECONNECT_BASE_MS), AgentBridgeDialTurn::Dial { .. }));
}

#[test]
fn the_backoff_doubles_per_unanswered_dial_until_the_offer_is_unavailable() {
    let mut state = AgentBridgeState::default();
    let mut dialer = AgentBridgeDialer::default();
    dialer.set_config(Some(AgentBridgeConfig::admit("ws://host/bridge", "proof").expect("admitted")), &mut state);
    let mut now = 0.0;
    let mut delays = Vec::new();
    for _ in 1..BRIDGE_UNANSWERED_ATTEMPTS {
        assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Closed, now), AgentBridgeDialTurn::Retire);
        match dialer.turn(&mut state, AgentBridgeSocketState::Absent, now) {
            AgentBridgeDialTurn::Wait { until_ms } => delays.push(until_ms - now),
            other => panic!("expected a wait, got {other:?}"),
        }
        now += RECONNECT_MAX_MS;
        assert!(matches!(dialer.turn(&mut state, AgentBridgeSocketState::Absent, now), AgentBridgeDialTurn::Dial { .. }));
    }
    assert_eq!(delays, vec![RECONNECT_BASE_MS, RECONNECT_BASE_MS * 2.0]);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Closed, now), AgentBridgeDialTurn::Retire);
    assert_eq!(state.status, AgentBridgeStatus::Unavailable);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Absent, now + RECONNECT_MAX_MS * 10.0), AgentBridgeDialTurn::Idle, "a given-up offer is never dialled again");
    assert_eq!(reconnect_delay_ms(64), RECONNECT_MAX_MS, "the ladder saturates, it does not grow forever");
}

#[test]
fn a_welcome_frame_after_a_reconnect_resets_the_ladder_the_way_reacts_hook_does() {
    let mut state = AgentBridgeState::default();
    let mut dialer = AgentBridgeDialer::default();
    dialer.set_config(Some(AgentBridgeConfig::admit("ws://host/bridge", "proof").expect("admitted")), &mut state);
    dialer.turn(&mut state, AgentBridgeSocketState::Closed, 0.0);
    dialer.turn(&mut state, AgentBridgeSocketState::Closed, 0.0);
    assert_eq!(state.reconnect_attempt, 2);
    state.apply_frame(GatewayToShell::Welcome { bridge_version: BRIDGE_VERSION, connection: "c1".into(), principal: "agent:one".into() }, 0.0);
    assert_eq!(state.reconnect_attempt, 0);
    assert_eq!(state.status, AgentBridgeStatus::Open);
}

#[test]
fn a_new_config_restarts_the_ladder_from_zero_and_clearing_it_disables_the_bridge() {
    let mut state = AgentBridgeState::default();
    let mut dialer = AgentBridgeDialer::default();
    dialer.set_config(Some(AgentBridgeConfig::admit("ws://host-a/bridge", "proof-a").expect("admitted")), &mut state);
    dialer.turn(&mut state, AgentBridgeSocketState::Closed, 0.0);
    assert_eq!(state.reconnect_attempt, 1);
    assert!(dialer.set_config(Some(AgentBridgeConfig::admit("ws://host-b/bridge", "proof-b").expect("admitted")), &mut state));
    assert_eq!(state.reconnect_attempt, 0);
    assert_eq!(state.status, AgentBridgeStatus::Connecting);
    let same = dialer.config().cloned();
    assert!(!dialer.set_config(same, &mut state), "an identical config is not a restart");
    assert!(dialer.set_config(None, &mut state));
    assert_eq!(state.status, AgentBridgeStatus::Disabled);
    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Absent, 0.0), AgentBridgeDialTurn::Idle);
}
fn handshake_frame(frame: &serde_json::Value) -> GatewayToShell {
    let version = |key: &str| u16::try_from(frame[key].as_u64().expect("version")).expect("u16 version");
    match frame["variant"].as_str().expect("variant") {
        "welcome" => GatewayToShell::Welcome { bridge_version: version("bridgeVersion"), connection: frame["connection"].as_str().expect("connection").into(), principal: frame["principal"].as_str().expect("principal").into() },
        "refused" => GatewayToShell::Refused {
            reason: match frame["reason"].as_str().expect("reason") {
                "version" => BridgeRefusal::Version,
                "capacity" => BridgeRefusal::Capacity,
                other => panic!("unknown refusal {other}"),
            },
            gateway_version: version("gatewayVersion"),
        },
        other => panic!("unexpected handshake frame {other}"),
    }
}

/// 🤝️ Replays `🧫️fixtures/🤝️handshake/🔣️.json` — the scenarios React's `useAgentBridge handshake`
/// suite replays — through this twin's dialer and state: same dial count, same terminal status, same
/// named version pair, and nothing dialled after the offer is given up on.
#[test]
fn every_handshake_scenario_ends_where_reacts_hook_ends() {
    let fixture: serde_json::Value = serde_json::from_str(HANDSHAKE_FIXTURE).expect("handshake fixture");
    assert_eq!((fixture["handshakeDeadlineMs"].as_f64(), fixture["unansweredAttempts"].as_u64(), fixture["shellVersion"].as_u64()), (Some(BRIDGE_HANDSHAKE_DEADLINE_MS), Some(u64::from(BRIDGE_UNANSWERED_ATTEMPTS)), Some(u64::from(BRIDGE_VERSION))));
    for scenario in fixture["scenarios"].as_array().expect("scenarios") {
        let name = scenario["name"].as_str().expect("name");
        let mut state = AgentBridgeState::default();
        let mut dialer = AgentBridgeDialer::default();
        dialer.set_config(Some(AgentBridgeConfig::admit("ws://127.0.0.1:6300/bridge", "session.v1.handshake.proof").expect("admitted")), &mut state);
        let (mut now, mut socket, mut dials) = (0.0, AgentBridgeSocketState::Absent, 0usize);
        fn advance(state: &mut AgentBridgeState, dialer: &mut AgentBridgeDialer, socket: &mut AgentBridgeSocketState, now: &mut f64, dials: &mut usize) {
            for _ in 0..64 {
                match dialer.turn(state, *socket, *now) {
                    AgentBridgeDialTurn::Dial { .. } => {
                        *dials += 1;
                        *socket = AgentBridgeSocketState::Connecting;
                        return;
                    }
                    AgentBridgeDialTurn::Retire => *socket = AgentBridgeSocketState::Absent,
                    AgentBridgeDialTurn::Wait { until_ms } => *now = until_ms,
                    AgentBridgeDialTurn::Idle if matches!(*socket, AgentBridgeSocketState::Absent) => return,
                    _ => *now += 1.0,
                }
            }
        }
        for dial in scenario["dials"].as_array().expect("dials") {
            advance(&mut state, &mut dialer, &mut socket, &mut now, &mut dials);
            match dial.as_str() {
                Some("close") => {
                    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Closed, now), AgentBridgeDialTurn::Retire, "{name}");
                    socket = AgentBridgeSocketState::Absent;
                }
                Some("silence") => {
                    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, now), AgentBridgeDialTurn::Announce, "{name}");
                    now += BRIDGE_HANDSHAKE_DEADLINE_MS;
                    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, now), AgentBridgeDialTurn::Retire, "{name}: an unanswered socket outlives no deadline");
                    socket = AgentBridgeSocketState::Absent;
                }
                _ => {
                    assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Open, now), AgentBridgeDialTurn::Announce, "{name}");
                    socket = AgentBridgeSocketState::Open;
                    state.apply_encoded_frame(&handshake_frame(&dial["frame"]).encode(), now).expect("frame decodes");
                    if dial["thenClose"].as_bool() == Some(true) {
                        assert_eq!(dialer.turn(&mut state, AgentBridgeSocketState::Closed, now), AgentBridgeDialTurn::Retire, "{name}");
                        socket = AgentBridgeSocketState::Absent;
                    }
                }
            }
        }
        for _ in 0..16 {
            now += RECONNECT_MAX_MS;
            match dialer.turn(&mut state, socket, now) {
                AgentBridgeDialTurn::Retire => socket = AgentBridgeSocketState::Absent,
                AgentBridgeDialTurn::Dial { .. } => dials += 1,
                _ => {}
            }
        }
        assert_eq!(dials, scenario["dials"].as_array().expect("dials").len(), "{name}: dial count");
        let expected = match (scenario["status"].as_str().expect("status"), &scenario["versionMismatch"]) {
            ("unavailable", _) => AgentBridgeStatus::Unavailable,
            ("open", _) => AgentBridgeStatus::Open,
            ("incompatible", mismatch) => AgentBridgeStatus::Incompatible(AgentBridgeVersionMismatch { gateway: mismatch["gateway"].as_u64().expect("gateway") as u16, shell: mismatch["shell"].as_u64().expect("shell") as u16 }),
            (other, _) => panic!("{name}: unknown status {other}"),
        };
        assert_eq!(state.status, expected, "{name}");
    }
}
//#endregion 🔖️DialLadder

//#region 🔖️InboundShellCommands
/// 🎛️ `ui_focus` and `ui_reveal` are the gateway's whole live chrome surface, and both arrive as
/// `ShellCommand` frames. They used to be answered with one blanket "no ShellState reducer twin"
/// refusal; they are chrome actions this shell has always been able to perform, so they are queued
/// for the host and only a verb with no chrome here is refused — by name.
#[test]
fn ui_focus_and_ui_reveal_are_queued_for_the_host_while_a_chromeless_verb_is_refused_by_name() {
    let mut state = AgentBridgeState::default();
    let frame = |seq: u64, json: &str| GatewayToShell::ShellCommand { seq, command: json.as_bytes().to_vec() };
    state.apply_frame(frame(1, r#"{"type":"focusWindow","windowId":"note-composite"}"#), 0.0);
    state.apply_frame(frame(2, r#"{"type":"setPanelVisible","anchor":"right","visible":true}"#), 0.0);
    state.apply_frame(frame(3, r#"{"type":"setPanelPath","anchor":"right","path":["framework.chat"]}"#), 0.0);
    state.apply_frame(frame(4, r#"{"type":"setStorageScope","scope":"memory"}"#), 0.0);

    let refusals: Vec<_> = state
        .take_outbox()
        .into_iter()
        .filter_map(|frame| match frame {
            ShellToGateway::ShellCommandResult { in_reply_to, ok, fault } => Some((in_reply_to, ok, fault)),
            _ => None,
        })
        .collect();
    assert_eq!(refusals.len(), 1, "only the chromeless verb answers before the host runs");
    assert_eq!(refusals[0].0, 4);
    assert!(!refusals[0].1);
    assert_eq!(refusals[0].2.as_deref(), Some("this shell has no chrome for the `setStorageScope` shell command"));

    let inbound = state.take_inbound_shell_commands();
    assert_eq!(
        inbound,
        vec![InboundShellCommand::FocusWindow { seq: 1, window_id: Some("note-composite".to_string()) }, InboundShellCommand::Acknowledge { seq: 2 }, InboundShellCommand::RevealPanelTab { seq: 3, tab_id: "framework.chat".to_string() },]
    );
    assert!(state.take_inbound_shell_commands().is_empty(), "taking drains the queue");
}

/// ✅️ The acknowledgement is the HOST's to send, after the chrome actually moved — the whole point
/// of splitting take/settle rather than answering `ok` at decode time.
#[test]
fn the_host_acknowledges_an_inbound_command_only_after_it_applied_it() {
    let mut state = AgentBridgeState::default();
    state.apply_frame(GatewayToShell::ShellCommand { seq: 7, command: br#"{"type":"focusWindow","windowId":null}"#.to_vec() }, 0.0);
    assert!(state.take_outbox().is_empty(), "nothing is answered before the host applies it");
    let inbound = state.take_inbound_shell_commands();
    assert_eq!(inbound.len(), 1);
    assert_eq!(inbound[0].seq(), 7);
    assert_eq!(inbound[0], InboundShellCommand::FocusWindow { seq: 7, window_id: None });
    state.settle_shell_command(7, false, Some("no such window".to_string()));
    assert_eq!(state.take_outbox(), vec![ShellToGateway::ShellCommandResult { in_reply_to: 7, ok: false, fault: Some("no such window".to_string()) }]);
}

/// 🚨️ A malformed payload is a named refusal, never a panic and never a silent drop.
#[test]
fn a_malformed_shell_command_payload_is_refused_with_its_reason() {
    assert!(decode_inbound_shell_command(1, b"not json").expect_err("malformed").starts_with("malformed ShellCommand JSON"));
    assert_eq!(decode_inbound_shell_command(2, br#"{"anchor":"right"}"#).expect_err("typeless"), "ShellCommand carried no `type`");
    assert_eq!(decode_inbound_shell_command(3, br#"{"type":"setPanelPath","anchor":"right","path":[]}"#).expect_err("pathless"), "setPanelPath carried no panel tab id to reveal");
}
//#endregion 🔖️InboundShellCommands
