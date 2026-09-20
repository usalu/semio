
use super::*;
use crate::catalog::{CapabilityAudience, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, ToolExposure};
use semio_framework::manifest::{CapabilityEffects, CapabilityExecution, CapabilityPolicy};

fn capability(id: &str, scopes: &[&str], approval: ApprovalMode, destructive: bool) -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: "cad".into(), label: None, app_id: Some("editor".into()), window_kind_id: Some("viewport".into()), mode_id: None },
        kind: CapabilityKind::Mutation,
        audience: CapabilityAudience::Agent,
        title: id.to_string(),
        description: String::new(),
        artifact_kind: None,
        use_when: Vec::new(),
        input_schema: serde_json::json!({"type": "object"}),
        output_schema: serde_json::json!({"type": "object"}),
        effects: CapabilityEffects { destructive, writes: vec![semio_framework::manifest::ResourceSelector::new("artifact:{self}")], ..Default::default() },
        policy: CapabilityPolicy { scopes: scopes.iter().map(|scope| kernel::CapabilityId(scope.to_string())).collect(), approval },
        execution: CapabilityExecution::default(),
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: None, keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

//#region 🔖️ScopeExpansion
#[test]
fn artifact_write_expands_to_documents_write_and_jobs_spawn() {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["artifact.write".to_string()], None);
    assert!(principal.grants(&kernel::CapabilityId("artifacts.write".into())));
    assert!(principal.grants(&kernel::CapabilityId("jobs.spawn".into())));
    assert!(!principal.grants(&kernel::CapabilityId("shell.raw".into())));
}

#[test]
fn legacy_documents_read_aliases_expand_to_artifacts_read() {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["documents.read".to_string()], None);
    assert!(principal.grants(&kernel::CapabilityId("artifacts.read".into())));
    assert!(!principal.grants(&kernel::CapabilityId("artifacts.write".into())));
}

#[test]
fn legacy_documents_write_aliases_expand_like_artifact_write() {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["documents.write".to_string()], None);
    assert!(principal.grants(&kernel::CapabilityId("artifacts.write".into())));
    assert!(principal.grants(&kernel::CapabilityId("jobs.spawn".into())));
}

#[test]
fn ui_raw_control_expands_to_shell_raw() {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["ui.raw-control".to_string()], None);
    assert!(principal.grants(&kernel::CapabilityId("shell.raw".into())));
}

#[test]
fn an_unknown_alias_passes_through_as_a_literal_capability_id() {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["artifacts.write".to_string()], None);
    assert!(principal.grants(&kernel::CapabilityId("artifacts.write".into())));
}

#[test]
fn wildcard_family_grant_covers_any_concrete_member() {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["host.filesystem.read".to_string()], None);
    assert!(principal.grants(&kernel::CapabilityId("fs.read:/tmp/workspace".into())));
    assert!(!principal.grants(&kernel::CapabilityId("fs.write:/tmp/workspace".into())));
}
//#endregion 🔖️ScopeExpansion

//#region 🔖️ScopeEnforcement
#[test]
fn authorize_scopes_denies_when_a_required_scope_is_missing() {
    let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
    let capability = capability("cad.editor.translateSelection", &["artifacts.write"], ApprovalMode::Never, false);
    let error = engine.authorize_scopes(&principal, &capability).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::PermissionDenied);
}

#[test]
fn authorize_scopes_allows_when_every_scope_is_granted() {
    let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["artifact.write".to_string()], None);
    let capability = capability("cad.editor.translateSelection", &["artifacts.write"], ApprovalMode::Never, false);
    assert!(engine.authorize_scopes(&principal, &capability).is_ok());
}
//#endregion 🔖️ScopeEnforcement

//#region 🔖️ApprovalGate
#[test]
fn never_approval_mode_proceeds_without_any_gate() {
    let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
    let capability = capability("cad.editor.translateSelection", &[], ApprovalMode::Never, true);
    let session = SessionHandle::new("sess_1");
    assert_eq!(engine.gate_approval(&principal, &capability, serde_json::json!({}), None, &session, 0), ApprovalGate::Proceed);
}

#[test]
fn a_destructive_capability_under_when_destructive_requires_approval_then_proceeds_once_resolved() {
    let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
    let capability = capability("cad.editor.deleteSelection", &[], ApprovalMode::WhenDestructive, true);
    let session = SessionHandle::new("sess_1");

    let first = engine.gate_approval(&principal, &capability, serde_json::json!({"opsCount": 1}), None, &session, 0);
    let handle = match first {
        ApprovalGate::Required { approval_handle } => approval_handle,
        ApprovalGate::Proceed => panic!("a destructive WhenDestructive capability must require approval"),
    };

    // not yet decided: resubmitting the same (undecided) handle must still be Required.
    let still_pending = engine.gate_approval(&principal, &capability, serde_json::json!({}), Some(&handle), &session, 1);
    assert_ne!(still_pending, ApprovalGate::Proceed);

    let approved_handle = engine.resolve_approval(&session, &handle, true, 2).unwrap();
    let proceeds = engine.gate_approval(&principal, &capability, serde_json::json!({}), Some(&approved_handle), &session, 3);
    assert_eq!(proceeds, ApprovalGate::Proceed);
}

#[test]
fn a_denied_approval_never_lets_the_gate_proceed() {
    let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
    let capability = capability("cad.editor.deleteSelection", &[], ApprovalMode::Always, false);
    let session = SessionHandle::new("sess_1");
    let first = match engine.gate_approval(&principal, &capability, serde_json::json!({}), None, &session, 0) {
        ApprovalGate::Required { approval_handle } => approval_handle,
        ApprovalGate::Proceed => panic!("Always must require approval"),
    };
    let denied_handle = engine.resolve_approval(&session, &first, false, 1).unwrap();
    let gate = engine.gate_approval(&principal, &capability, serde_json::json!({}), Some(&denied_handle), &session, 2);
    assert_ne!(gate, ApprovalGate::Proceed);
}

#[test]
fn auto_approve_all_waives_the_gate_entirely() {
    let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::All);
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &[], None);
    let capability = capability("cad.editor.deleteSelection", &[], ApprovalMode::Always, true);
    let session = SessionHandle::new("sess_1");
    assert_eq!(engine.gate_approval(&principal, &capability, serde_json::json!({}), None, &session, 0), ApprovalGate::Proceed);
}

#[test]
fn auto_approve_parses_the_three_frozen_values_and_nothing_else() {
    assert_eq!(AutoApprovePolicy::parse("never"), Some(AutoApprovePolicy::Never));
    assert_eq!(AutoApprovePolicy::parse("readonly"), Some(AutoApprovePolicy::ReadonlyOnly));
    assert_eq!(AutoApprovePolicy::parse("all"), Some(AutoApprovePolicy::All));
    assert_eq!(AutoApprovePolicy::parse("sometimes"), None);
    assert_eq!(AutoApprovePolicy::default(), AutoApprovePolicy::Never);
}
//#endregion 🔖️ApprovalGate

//#region 🔖️ApprovalCoordinator
// 🎫️ ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END slice M4 (audit §6 P0.2): the approval gate
// used to be unresolvable in EVERY configuration — no elicitation was ever sent, no bridge frame was
// ever published, no tool could decide a handle, and `--auto-approve` had no flag. These tests drive
// the real chain end to end: the real `ElicitationChannel` over real in-memory streams, and the real
// `BridgeHandle` with a real registered connection.

use crate::bridge::{ApprovalDecision, BridgeHandle, GatewayToShell, ShellToGateway};
use crate::protocol::ClientFeatures;
use crate::transport::{ElicitationChannel, ElicitationSlot, StdioLines};
use std::sync::OnceLock;

fn approval_request<'a>(handle: &'a str, diff: &'a serde_json::Value) -> ApprovalRequest<'a> {
    ApprovalRequest {
        approval_handle: handle,
        capability_id: "cad.editor.deleteSelection",
        capability_title: "Delete Selection",
        capability_description: "Removes every currently selected element from the drawing.",
        artifact_kind: Some("s.cad.cad"),
        principal_id: "agent:local",
        diff_summary: diff,
    }
}

/// 🙋 An elicitation slot whose client already answered `response_line`, with `elicitation` either
/// advertised or not.
fn elicitation_slot(response_line: &str, advertised: bool) -> ElicitationSlot {
    let features = Arc::new(ClientFeatures::default());
    features.record(advertised.then(|| serde_json::json!({ "elicitation": {} })).as_ref());
    let lines = Arc::new(StdioLines::new(Box::new(std::io::Cursor::new(response_line.as_bytes().to_vec())), Box::new(Vec::new())));
    let slot: ElicitationSlot = Arc::new(OnceLock::new());
    let _ = slot.set(Arc::new(ElicitationChannel::new(lines, features)));
    slot
}

#[test]
fn an_elicitation_accept_approves_and_names_its_channel() {
    let coordinator = ApprovalCoordinator::new(Some(elicitation_slot("{\"jsonrpc\":\"2.0\",\"id\":\"semio-elicit-1\",\"result\":{\"action\":\"accept\",\"content\":{\"approve\":true}}}\n", true)), None);
    let diff = serde_json::json!({ "opsCount": 1 });
    assert_eq!(coordinator.resolve(&approval_request("appr_1", &diff)), ApprovalResolution::Approved { channel: ApprovalChannel::Elicitation });
}

#[test]
fn an_elicitation_decline_denies_and_never_falls_through_to_another_lane() {
    let coordinator = ApprovalCoordinator::new(Some(elicitation_slot("{\"jsonrpc\":\"2.0\",\"id\":\"semio-elicit-1\",\"result\":{\"action\":\"decline\"}}\n", true)), None);
    let diff = serde_json::json!({});
    match coordinator.resolve(&approval_request("appr_2", &diff)) {
        ApprovalResolution::Denied { channel, .. } => assert_eq!(channel, ApprovalChannel::Elicitation),
        other => panic!("a declined elicitation must deny, got {other:?}"),
    }
}

#[test]
fn an_elicitation_cancel_denies_rather_than_approving_by_default() {
    let coordinator = ApprovalCoordinator::new(Some(elicitation_slot("{\"jsonrpc\":\"2.0\",\"id\":\"semio-elicit-1\",\"result\":{\"action\":\"cancel\"}}\n", true)), None);
    let diff = serde_json::json!({});
    match coordinator.resolve(&approval_request("appr_3", &diff)) {
        ApprovalResolution::Denied { channel, .. } => assert_eq!(channel, ApprovalChannel::Elicitation),
        other => panic!("a cancelled elicitation must deny, got {other:?}"),
    }
}

#[test]
fn an_accept_without_an_approve_field_is_a_denial_not_an_approval() {
    let coordinator = ApprovalCoordinator::new(Some(elicitation_slot("{\"jsonrpc\":\"2.0\",\"id\":\"semio-elicit-1\",\"result\":{\"action\":\"accept\",\"content\":{}}}\n", true)), None);
    let diff = serde_json::json!({});
    assert!(matches!(coordinator.resolve(&approval_request("appr_4", &diff)), ApprovalResolution::Denied { .. }));
}

#[test]
fn a_client_that_never_advertised_elicitation_is_not_asked_at_all() {
    let coordinator = ApprovalCoordinator::new(Some(elicitation_slot("{\"jsonrpc\":\"2.0\",\"id\":\"semio-elicit-1\",\"result\":{\"action\":\"accept\",\"content\":{\"approve\":true}}}\n", false)), None);
    let diff = serde_json::json!({});
    match coordinator.resolve(&approval_request("appr_5", &diff)) {
        ApprovalResolution::Unreachable { details } => assert_eq!(details["channels"]["elicitation"], "the connected client did not advertise capabilities.elicitation"),
        other => panic!("an unadvertised client must not be asked, got {other:?}"),
    }
}

#[test]
fn a_client_that_closes_mid_elicitation_is_unreachable_never_approved() {
    let coordinator = ApprovalCoordinator::new(Some(elicitation_slot("", true)), None);
    let diff = serde_json::json!({});
    match coordinator.resolve(&approval_request("appr_6", &diff)) {
        ApprovalResolution::Unreachable { details } => assert_eq!(details["channels"]["elicitation"], "the client closed the connection while the elicitation was pending"),
        other => panic!("EOF mid-elicitation must never approve, got {other:?}"),
    }
}

// 🎫️ slice M7: a client whose human walks away closes the elicitation lane the same way a silent
// shell closes the bridge lane — a named, typed, non-approving outcome.
#[test]
fn a_client_that_never_answers_its_elicitation_times_out_into_the_same_typed_outcome_as_a_silent_shell() {
    let (keep_open, gate) = std::sync::mpsc::channel::<u8>();
    struct SilentClient {
        gate: std::sync::mpsc::Receiver<u8>,
    }
    impl std::io::Read for SilentClient {
        fn read(&mut self, _buffer: &mut [u8]) -> std::io::Result<usize> {
            let _ = self.gate.recv();
            Ok(0)
        }
    }
    let features = Arc::new(ClientFeatures::default());
    features.record(Some(&serde_json::json!({ "elicitation": {} })));
    let lines = Arc::new(StdioLines::new(Box::new(std::io::BufReader::new(SilentClient { gate })), Box::new(Vec::new())));
    let slot: ElicitationSlot = Arc::new(OnceLock::new());
    let _ = slot.set(Arc::new(ElicitationChannel::new(lines, features).with_deadline(60, Box::new(crate::transport::SystemElicitationClock::default()))));
    let coordinator = ApprovalCoordinator::new(Some(slot), None);
    let diff = serde_json::json!({});
    match coordinator.resolve(&approval_request("appr_timeout", &diff)) {
        ApprovalResolution::Unreachable { details } => {
            assert_eq!(details["channels"]["elicitation"], "the connected client did not answer the elicitation in time");
            assert!(details["remedy"].as_str().expect("remedy").contains("--auto-approve"));
        }
        other => panic!("a silent client must never approve, got {other:?}"),
    }
    drop(keep_open);
}

#[test]
fn with_no_lane_at_all_the_answer_names_both_closed_lanes_and_the_remedy() {
    let coordinator = ApprovalCoordinator::new(None, None);
    let diff = serde_json::json!({});
    match coordinator.resolve(&approval_request("appr_7", &diff)) {
        ApprovalResolution::Unreachable { details } => {
            assert_eq!(details["approvalHandle"], "appr_7");
            assert_eq!(details["channels"]["elicitation"], "no server-initiated request channel on this transport");
            assert_eq!(details["channels"]["shell"], "this gateway is serving no /bridge — no OS shell can be asked");
            assert!(details["remedy"].as_str().expect("remedy").contains("--auto-approve"));
        }
        other => panic!("nothing attached must be Unreachable, got {other:?}"),
    }
}

/// 🌉️ A live bridge with one registered connection, plus the receiver that would be the shell's
/// socket — kept alive so the outbox is never closed under the coordinator.
fn live_bridge() -> (Arc<BridgeHandle>, crate::bridge::ShellConnectionId, impl FnMut() -> Option<GatewayToShell>) {
    let bridge = Arc::new(BridgeHandle::new());
    let (connection, mut receiver) = bridge.register();
    (bridge, connection, move || receiver.try_recv())
}

#[test]
fn a_bridge_with_no_shell_attached_is_unreachable_and_says_so() {
    let bridge = Arc::new(BridgeHandle::new());
    let slot: crate::ui::BridgeSlot = Arc::new(OnceLock::new());
    let _ = slot.set(bridge);
    let coordinator = ApprovalCoordinator::new(None, Some(slot));
    let diff = serde_json::json!({});
    match coordinator.resolve(&approval_request("appr_8", &diff)) {
        ApprovalResolution::Unreachable { details } => assert_eq!(details["channels"]["shell"], "a /bridge is running but no OS shell is attached to it"),
        other => panic!("an empty bridge must be Unreachable, got {other:?}"),
    }
}

#[test]
fn the_shell_lane_publishes_a_structured_request_and_honours_the_humans_yes() {
    let (bridge, connection, mut next_frame) = live_bridge();
    let slot: crate::ui::BridgeSlot = Arc::new(OnceLock::new());
    let _ = slot.set(bridge.clone());
    bridge.record(connection, ShellToGateway::Approval { approval_id: "appr_9".to_string(), decision: ApprovalDecision::Once, note: None });
    let coordinator = ApprovalCoordinator::new(None, Some(slot)).with_shell_timeout_ms(8_000);
    let diff = serde_json::json!({ "opsCount": 3 });
    assert_eq!(coordinator.resolve(&approval_request("appr_9", &diff)), ApprovalResolution::Approved { channel: ApprovalChannel::Shell });

    let published = next_frame().expect("the shell lane publishes an ApprovalRequested frame");
    match published {
        GatewayToShell::ApprovalRequested { approval_id, summary } => {
            assert_eq!(approval_id, "appr_9");
            let parsed: serde_json::Value = serde_json::from_str(&summary).expect("the summary is the structured shape 🤖️AgentApprovals parses");
            assert_eq!(parsed["capabilityId"], "cad.editor.deleteSelection");
            assert_eq!(parsed["requestedBy"], "agent:local");
            assert_eq!(parsed["risk"], "high");
            // 🧾️ WHO / WHAT / HOW LONG — the three things a human cannot decide without, and the
            // reason a shell-lane approval is an affordance rather than a JSON blob on screen.
            assert_eq!(parsed["capabilityTitle"], "Delete Selection");
            assert_eq!(parsed["description"], "Removes every currently selected element from the drawing.");
            assert_eq!(parsed["artifactKind"], "s.cad.cad");
            assert_eq!(parsed["timeoutMs"], 8_000);
        }
        other => panic!("expected ApprovalRequested, got {other:?}"),
    }
}

#[test]
fn the_shell_lane_carries_a_humans_deny_back_as_a_denial() {
    let (bridge, connection, _next_frame) = live_bridge();
    let slot: crate::ui::BridgeSlot = Arc::new(OnceLock::new());
    let _ = slot.set(bridge.clone());
    bridge.record(connection, ShellToGateway::Approval { approval_id: "appr_10".to_string(), decision: ApprovalDecision::Deny, note: Some("not on my document".to_string()) });
    let coordinator = ApprovalCoordinator::new(None, Some(slot)).with_shell_timeout_ms(2_000);
    let diff = serde_json::json!({});
    match coordinator.resolve(&approval_request("appr_10", &diff)) {
        ApprovalResolution::Denied { channel, note } => {
            assert_eq!(channel, ApprovalChannel::Shell);
            assert_eq!(note.as_deref(), Some("not on my document"));
        }
        other => panic!("a human's Deny must deny, got {other:?}"),
    }
}

#[test]
fn a_shell_that_never_answers_times_out_into_unreachable_never_into_approved() {
    let (bridge, _connection, _next_frame) = live_bridge();
    let slot: crate::ui::BridgeSlot = Arc::new(OnceLock::new());
    let _ = slot.set(bridge);
    let coordinator = ApprovalCoordinator::new(None, Some(slot)).with_shell_timeout_ms(120);
    let diff = serde_json::json!({});
    let started = std::time::Instant::now();
    match coordinator.resolve(&approval_request("appr_11", &diff)) {
        ApprovalResolution::Unreachable { details } => assert_eq!(details["channels"]["shell"], "the attached OS shell did not answer the approval request in time"),
        other => panic!("a silent shell must time out into Unreachable, got {other:?}"),
    }
    assert!(started.elapsed() >= std::time::Duration::from_millis(120), "the timeout is real wall-clock, not an immediate give-up");
}

#[test]
fn a_stale_decision_for_another_handle_is_never_mistaken_for_this_one() {
    let (bridge, connection, _next_frame) = live_bridge();
    let slot: crate::ui::BridgeSlot = Arc::new(OnceLock::new());
    let _ = slot.set(bridge.clone());
    bridge.record(connection, ShellToGateway::Approval { approval_id: "appr_other".to_string(), decision: ApprovalDecision::Session, note: None });
    let coordinator = ApprovalCoordinator::new(None, Some(slot)).with_shell_timeout_ms(120);
    let diff = serde_json::json!({});
    assert!(matches!(coordinator.resolve(&approval_request("appr_12", &diff)), ApprovalResolution::Unreachable { .. }));
}
//#endregion 🔖️ApprovalCoordinator
