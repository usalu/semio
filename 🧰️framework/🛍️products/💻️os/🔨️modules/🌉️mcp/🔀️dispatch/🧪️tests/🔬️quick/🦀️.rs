
use super::*;

use crate::audit::InMemoryAuditSink;
use crate::catalog::{CapabilityAudience, CapabilityDefinition, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, Catalog, ToolExposure, compile};
use crate::source_builders;
use semio_framework::manifest::kernel;
use semio_framework::manifest::{ApprovalMode, CapabilityEffects, CapabilityExecution, CapabilityPolicy, ResourceSelector};
use semio_framework::{Locale, Terminology};

fn synthetic_capability(id: &str, scopes: &[&str], approval: ApprovalMode, destructive: bool) -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Mutation,
        audience: CapabilityAudience::Agent,
        title: id.to_string(),
        description: String::new(),
        artifact_kind: None,
        use_when: Vec::new(),
        input_schema: serde_json::json!({"type": "object"}),
        output_schema: serde_json::json!({"type": "object"}),
        effects: CapabilityEffects { destructive, writes: vec![ResourceSelector::new("artifact:{self}")], ..Default::default() },
        policy: CapabilityPolicy { scopes: scopes.iter().map(|scope| kernel::CapabilityId(scope.to_string())).collect(), approval },
        execution: CapabilityExecution::default(),
        exposure: ToolExposure::CatalogOnly,
        presentation: CapabilityPresentation { icon_id: None, category: None, keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn single_capability_catalog(capability: CapabilityDefinition) -> Catalog {
    Catalog { hash: "test".into(), entries: vec![capability] }
}

fn two_capability_catalog(a: CapabilityDefinition, b: CapabilityDefinition) -> Catalog {
    let mut entries = vec![a, b];
    entries.sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));
    Catalog { hash: "test".into(), entries }
}

fn real_fixture_catalog() -> Catalog {
    compile(&source_builders::note_and_cad_source(), Locale::En, Terminology::Native).expect("fixture catalog compiles")
}

// 🔀️ dedyn-fw-os-misc: returns `Arc<AuditSinks>` (was `Arc<InMemoryAuditSink>`) — ONE shared
// `Arc`, same instance `ActionAdapter` holds, so events it records via the trait method are
// visible through this same handle. `InMemoryAuditSink::events()` is inherent (not part of the
// `AuditSink` trait), so a caller wanting it back matches the `AuditSinks::InMemory` variant —
// see `assert_events` below.
fn harness(auto_approve: AutoApprovePolicy) -> (ActionAdapter, MockArtifactChannel, Arc<HandleTable>, Arc<AuditSinks>) {
    let channel = MockArtifactChannel::new();
    let handles = Arc::new(HandleTable::new());
    let idempotency = Arc::new(IdempotencyStore::new());
    let audit = Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new()));
    let adapter = ActionAdapter::new(Box::new(ArtifactChannels::Mock(channel.clone())), handles.clone(), idempotency, audit.clone(), auto_approve, ClientInfo { name: "test".into(), version: "0".into() });
    (adapter, channel, handles, audit)
}

#[derive(Default)]
struct RecordingHistoryUndoPort {
    members: Mutex<Vec<HubInferenceApprovalUndoMemberV1>>,
}

impl HistoryUndoPort for RecordingHistoryUndoPort {
    fn undo_hub_inference_approval(&self, member: &HubInferenceApprovalUndoMemberV1) -> Result<(), GatewayError> {
        self.members.lock().expect("history members lock poisoned").push(member.clone());
        Ok(())
    }
}

// 🔀️ dedyn-fw-os-misc: extracts the recorded events from a harness `Arc<AuditSinks>` known (by
// every caller in this test module) to be the `InMemory` variant.
fn assert_events(audit: &AuditSinks) -> Vec<AgentAuditEvent> {
    match audit {
        AuditSinks::InMemory(sink) => sink.events(),
        AuditSinks::File(_) => panic!("test harness always constructs AuditSinks::InMemory"),
    }
}

fn principal(scopes: &[&str]) -> AgentPrincipal {
    AgentPrincipal::from_scope_names("agent:test", "test agent", &scopes.iter().map(|scope| scope.to_string()).collect::<Vec<_>>(), None)
}

//#region 🔖️PreviewVsCommit
#[test]
fn preview_ops_and_the_ops_actually_committed_are_the_same_bytes() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);

    let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
    let preview_ops_count = prepared.preview["opsCount"]["document"].as_u64().unwrap();

    let report = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap();
    assert_eq!(report.status, InvocationStatus::Succeeded);

    // The exact PureCommand op payload (preview) and the exact TransactionPrepare op payload
    // (commit) sent over the wire must be byte-identical — asserted on the recorded frame log, not
    // on internal state. `MockArtifactChannel::handle` derives its `PureCommand` Emit bytes
    // deterministically from `(capability_id, input)`, so reconstructing the expected bytes here
    // (rather than reading them back off a response, which the command-only frame log does not
    // carry) is exact, not approximate.
    let log = channel.frame_log();
    assert!(log.iter().any(|(_, command)| matches!(command, AppCommand::PureCommand { .. })), "a PureCommand was sent during preview");
    let expected_op = serde_json::to_vec(&serde_json::json!({ "capabilityId": "cad.editor.translateSelection", "input": {"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]} })).unwrap();
    let prepare_ops: Vec<Vec<u8>> = log
        .iter()
        .find_map(|(_, command)| match command {
            AppCommand::TransactionPrepare { ops, .. } => Some(ops.document.clone()),
            _ => None,
        })
        .expect("a TransactionPrepare was sent");
    assert_eq!(prepare_ops.len() as u64, preview_ops_count);
    assert_eq!(prepare_ops, vec![expected_op], "the op bytes committed must be byte-identical to what PureCommand emitted during preview");
}
//#endregion 🔖️PreviewVsCommit

//#region 🔖️RevisionConflict
#[test]
fn stale_expected_revision_is_a_revision_conflict_with_no_mutation_sent() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);

    let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();

    // Simulate a concurrent edit landing between prepare and invoke.
    channel.bump_generation(0);
    let before_invoke = channel.frame_log().len();

    let stale = RevisionStamp { artifact_id: "mock-artifact-0".into(), head_edit_id: "edit-0".into(), cursor: "gen-0".into() };
    let error = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), expected_revision: Some(stale), ..Default::default() }, 0, 1).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::RevisionConflict);

    let log = channel.frame_log();
    // Only a ReadHistory (a pure read) may have been sent after the staleness became detectable —
    // no TransactionPrepare/TransactionCommit anywhere in the whole log.
    assert!(!log.iter().any(|(_, command)| matches!(command, AppCommand::TransactionPrepare { .. } | AppCommand::TransactionCommit { .. })), "a mutation command was sent despite a stale expectedRevision: {log:?}");
    assert!(log.len() > before_invoke, "invoke must have issued at least the ReadHistory recheck");
}
//#endregion 🔖️RevisionConflict

//#region 🔖️Idempotency
#[test]
fn idempotent_replay_performs_exactly_one_mutation() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);

    let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
    let request = InvokeRequest { prepared_handle: Some(prepared.prepared_handle), idempotency_key: Some("key-1".into()), ..Default::default() };

    let first = adapter.invoke(&catalog, &principal, &session, request.clone(), 0, 1).unwrap();
    assert!(!first.replayed);
    let commits_after_first = channel.frame_log().iter().filter(|(_, command)| matches!(command, AppCommand::TransactionCommit { .. })).count();
    assert_eq!(commits_after_first, 1);

    // A replay with a DIFFERENT prepared handle in the request would normally re-resolve, but the
    // idempotency key alone must short-circuit before any channel command is sent a second time.
    let second = adapter.invoke(&catalog, &principal, &session, InvokeRequest { idempotency_key: Some("key-1".into()), ..request }, 0, 2).unwrap();
    assert!(second.replayed);
    assert_eq!(second.invocation_id, first.invocation_id);
    let commits_after_second = channel.frame_log().iter().filter(|(_, command)| matches!(command, AppCommand::TransactionCommit { .. })).count();
    assert_eq!(commits_after_second, 1, "a replayed idempotency key must not perform a second mutation");
}
//#endregion 🔖️Idempotency

//#region 🔖️UndoRoundTrip
#[test]
fn undo_token_round_trips_through_history_undo_and_redo() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);

    let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
    let report = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap();
    let undo_token = report.undo_token.expect("a committed invocation mints an undo token");

    let undo_report = adapter.history_undo(&session, &undo_token, 2).unwrap();
    assert_eq!(undo_report.members, 1);
    assert!(undo_report.warnings.is_empty());
    assert!(channel.frame_log().iter().any(|(_, command)| matches!(command, AppCommand::TransactionUndo { .. })));

    let redo_report = adapter.history_redo(&session, &undo_token, 3).unwrap();
    assert_eq!(redo_report.members, 1);
    assert!(channel.frame_log().iter().any(|(_, command)| matches!(command, AppCommand::TransactionRedo { .. })));
}

#[test]
fn hub_gis_approval_history_uses_the_private_port_and_never_stores_inverse_bytes() {
    let (adapter, _channel, handles, _audit) = harness(AutoApprovePolicy::Never);
    let port = Arc::new(RecordingHistoryUndoPort::default());
    adapter.bind_history_undo_port(port.clone());
    let session = SessionHandle::new("sess_owner");
    let scope = semio_framework_os_kernel::os_directory::DocumentScope::new("space-a", "document-a");
    let expected_current = semio_framework_os_kernel::os_directory::EditedArtifactFrontierV1 { document_id: scope.document_id.clone(), head_edit_ordinal: 1, head_edit_id: "edit-a".into(), last_commit_seq: 1, chain_sha256: "11".repeat(32) };
    let token = adapter
        .retain_hub_inference_approval_undo(&session, "https://hub.invalid", &scope, "inference/gis-map", &semio_framework_os_kernel::os_directory::GisMapApprovalUndoHandleV1 { target_id: "22".repeat(16), expected_current: expected_current.clone() }, 7)
        .expect("Hub receipt mints one private undo token");
    let retained = handles.resolve(&token, &session, 8).expect("owner resolves its token");
    let encoded = serde_json::to_string(&retained.payload).expect("payload");
    assert!(encoded.contains("hub-inference-approval") && !encoded.contains("inverse") && !encoded.contains("mutation"), "{encoded}");
    let report = adapter.history_undo(&session, &token, 9).expect("history routes through the Hub port");
    assert_eq!((report.members, report.warnings.len()), (1, 0));
    let observed = port.members.lock().expect("history members lock poisoned");
    assert_eq!(observed.len(), 1);
    assert_eq!((observed[0].space_id.as_str(), observed[0].document_id.as_str(), observed[0].target_id.as_str()), ("space-a", "document-a", "22222222222222222222222222222222"));
    assert_eq!(observed[0].expected_current, expected_current);
    drop(observed);
    let redo = adapter.history_redo(&session, &token, 10).expect_err("remote durable undo cannot be replayed as a fabricated redo");
    assert_eq!(redo.code, GatewayErrorCode::SideEffectRejected);
    assert_eq!(port.members.lock().expect("history members lock poisoned").len(), 1);
    assert_eq!(adapter.history_undo(&SessionHandle::new("sess_foreign"), &token, 11).expect_err("foreign session cannot resolve token").code, GatewayErrorCode::PermissionDenied);
}
//#endregion 🔖️UndoRoundTrip

//#region 🔖️ApprovalGateBlocksThenProceeds
#[test]
fn approval_gate_blocks_a_destructive_capability_without_approval_and_proceeds_with_it() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let capability = synthetic_capability("gateway.destructiveThing", &[], ApprovalMode::Always, true);
    let catalog = single_capability_catalog(capability);
    let session = SessionHandle::new("sess_1");
    let principal = principal(&[]);

    let prepared = adapter.prepare(&catalog, &principal, &session, "gateway.destructiveThing", serde_json::json!({}), 0, 0).unwrap();

    let blocked = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle.clone()), ..Default::default() }, 0, 1).unwrap_err();
    assert_eq!(blocked.code, GatewayErrorCode::ApprovalRequired);
    let approval_handle = blocked.details["approvalHandle"].as_str().unwrap().to_string();

    let approved_handle = adapter.resolve_approval(&session, &approval_handle, true, 2).unwrap();
    let succeeded = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), approval_handle: Some(approved_handle), ..Default::default() }, 0, 3).unwrap();
    assert_eq!(succeeded.status, InvocationStatus::Succeeded);
}
//#endregion 🔖️ApprovalGateBlocksThenProceeds

//#region 🔖️ScopeDenialAudited
#[test]
fn a_capability_whose_scopes_exceed_the_principals_is_permission_denied_and_audited() {
    let (adapter, _channel, _handles, audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&[]); // no scopes granted at all

    let error = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::PermissionDenied);

    let events = assert_events(&audit);
    assert!(events.iter().any(|event| matches!(&event.decision, AuditDecision::Denied { code } if *code == GatewayErrorCode::PermissionDenied) && event.capability == "cad.editor.translateSelection"));
}
//#endregion 🔖️ScopeDenialAudited

//#region 🔖️Cancel
#[test]
fn cancel_drops_a_prepared_handle() {
    let (adapter, _channel, handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);

    let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
    assert_eq!(handles.len(), 1);
    adapter.cancel(&session, &prepared.prepared_handle, 1).unwrap();
    assert_eq!(handles.len(), 0);

    let error = adapter.cancel(&session, &prepared.prepared_handle, 2).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}
//#endregion 🔖️Cancel

//#region 🔖️InstanceBusy
#[test]
fn instance_busy_retries_then_precondition_failed() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);

    // Occupy instance 0 with an externally-pending transaction that never clears.
    let _ = channel
        .clone()
        .exchange(0, vec![AppCommand::TransactionPrepare { txn_id: "external".into(), ops: PreparedOps::default(), label: "external".into(), origin: MutationOrigin::Agent { principal: "someone-else".into(), invocation_id: "x".into() } }]);

    let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
    let error = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::PreconditionFailed);

    let attempts = channel.frame_log().iter().filter(|(_, command)| matches!(command, AppCommand::TransactionPrepare { .. })).count();
    assert_eq!(attempts as u32, INSTANCE_BUSY_MAX_ATTEMPTS + 1, "one external prepare plus every retry attempt");
}
//#endregion 🔖️InstanceBusy

//#region 🔖️GenerationMismatch
#[test]
fn a_concurrent_edit_between_prepare_and_commit_is_a_revision_conflict() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);

    let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
    // Force the LOW-LEVEL commit to see a stale base_generation without going through the upfront
    // expectedRevision recheck (which used the freshly re-read current revision as `expected`) —
    // bump AFTER our own ReadHistory recheck would run, by scripting a commit-time fault directly.
    channel.force_commit_fault(0, Fault { code: "transaction.generation-mismatch".into(), message: "base generation stale".into() });

    let error = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::RevisionConflict);
}
//#endregion 🔖️GenerationMismatch

//#region 🔖️BudgetExceeded
#[test]
fn budget_exceeded_fault_maps_to_budget_exceeded_code() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);

    let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
    channel.force_budget_exceeded(0);
    let error = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::BudgetExceeded);
    assert!(error.retryable);
}
//#endregion 🔖️BudgetExceeded

//#region 🔖️FaultMapping
#[test]
fn every_fault_code_maps_to_the_right_gateway_error_code() {
    let cases = [
        ("viewer.read-only", GatewayErrorCode::PermissionDenied),
        ("capability-denied", GatewayErrorCode::PermissionDenied),
        ("mutation.rejected", GatewayErrorCode::SideEffectRejected),
        ("transaction.generation-mismatch", GatewayErrorCode::RevisionConflict),
        ("transaction.instance-busy", GatewayErrorCode::PreconditionFailed),
        ("budget.exceeded", GatewayErrorCode::BudgetExceeded),
        ("some.unrecognised.code", GatewayErrorCode::Internal),
    ];
    for (code, expected) in cases {
        let mapped = map_fault(&Fault { code: code.to_string(), message: "x".into() });
        assert_eq!(mapped.code, expected, "fault code {code} mapped to {:?}, expected {:?}", mapped.code, expected);
    }
}
//#endregion 🔖️FaultMapping

//#region 🔖️SagaCompensation
#[test]
fn saga_commits_in_reverse_discovery_order_and_compensates_on_failure() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let capability_a = synthetic_capability("gateway.memberA", &[], ApprovalMode::Never, false);
    let capability_b = synthetic_capability("gateway.memberB", &[], ApprovalMode::Never, false);
    let catalog = two_capability_catalog(capability_a, capability_b);
    let session = SessionHandle::new("sess_1");
    let principal = principal(&[]);

    let prepared_a = adapter.prepare(&catalog, &principal, &session, "gateway.memberA", serde_json::json!({}), 0, 0).unwrap();
    let prepared_b = adapter.prepare(&catalog, &principal, &session, "gateway.memberB", serde_json::json!({}), 1, 0).unwrap();

    // Member A (instance 0) will fail its commit; its undo (compensation) must be attempted after
    // member B (instance 1, committed first since commit runs in REVERSE discovery order) succeeds.
    channel.force_commit_fault(0, Fault { code: "mutation.rejected".into(), message: "A rejected".into() });

    let saga_handle = adapter.transaction_begin(&session, &[prepared_a.prepared_handle, prepared_b.prepared_handle], 1).unwrap();
    let error = adapter.transaction_commit(&principal, &session, &saga_handle, 2).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::SideEffectRejected);

    let log = channel.frame_log();
    let commit_order: Vec<u32> = log.iter().filter_map(|(instance, command)| matches!(command, AppCommand::TransactionCommit { .. }).then_some(*instance)).collect();
    assert_eq!(commit_order, vec![1, 0], "commit must run in reverse discovery order (B=instance 1 before A=instance 0)");
    assert!(log.iter().any(|(instance, command)| *instance == 1 && matches!(command, AppCommand::TransactionUndo { .. })), "member B (already committed) must be compensated via TransactionUndo");
}

#[test]
fn compensation_failure_itself_is_reported_as_compensation_failed() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let capability_a = synthetic_capability("gateway.memberA", &[], ApprovalMode::Never, false);
    let capability_b = synthetic_capability("gateway.memberB", &[], ApprovalMode::Never, false);
    let catalog = two_capability_catalog(capability_a, capability_b);
    let session = SessionHandle::new("sess_1");
    let principal = principal(&[]);

    let prepared_a = adapter.prepare(&catalog, &principal, &session, "gateway.memberA", serde_json::json!({}), 0, 0).unwrap();
    let prepared_b = adapter.prepare(&catalog, &principal, &session, "gateway.memberB", serde_json::json!({}), 1, 0).unwrap();

    channel.force_commit_fault(0, Fault { code: "mutation.rejected".into(), message: "A rejected".into() });
    channel.force_undo_fails(1); // compensating the already-committed member B also fails

    let saga_handle = adapter.transaction_begin(&session, &[prepared_a.prepared_handle, prepared_b.prepared_handle], 1).unwrap();
    let error = adapter.transaction_commit(&principal, &session, &saga_handle, 2).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::CompensationFailed);
}
//#endregion 🔖️SagaCompensation

//#region 🔖️Misc
#[test]
fn unknown_capability_id_is_not_found() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);
    let error = adapter.prepare(&catalog, &principal, &session, "no.such.capability", serde_json::json!({}), 0, 0).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}

/// 🐛️ post-unblock fix: this test originally sent `{}` and asserted `INPUT_INVALID`, on the wrong
/// assumption that translateSelection's `dx`/`dy`/`dz`/`objectIds` args are required. They are
/// not — `🧫️fixtures/🦀️.rs`'s `string_array_arg`/`number_arg` helpers never call
/// `ActionArgDef::required()`, so the compiled `input_schema` has no `"required"` array at all,
/// and `{}` is genuinely schema-valid (confirmed directly by the repo-owned validator built from
/// this exact capability's `input_schema` reports `is_valid(&json!({}))  == true`). The test
/// encoded the wrong expectation, not a defect in `prepare`'s validation — `prepare` itself was
/// already correctly invoking the validator (confirmed: the same validator correctly rejects a
/// wrong-typed `dx` and an `additionalProperties:false`-violating unknown field). Fixed by
/// asserting against genuinely-invalid input instead of an incorrectly-assumed-required field.
#[test]
fn invalid_input_against_the_capabilitys_schema_is_input_invalid() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);
    // `dx` is declared `ArgSchema::Number` — a string value violates the schema's `type: "number"`.
    let error = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": "not a number"}), 0, 0).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::InputInvalid);
}

/// 🐛️ post-unblock fix, second half: the schema's `additionalProperties: false` is the OTHER real
/// enforcement point translateSelection's own args never exercise (none of them are required) —
/// an unrecognised field must still be rejected.
#[test]
fn an_unrecognised_field_against_the_capabilitys_schema_is_input_invalid() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);
    let error = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"notARealArg": 1}), 0, 0).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::InputInvalid);
}

/// ✅️ The flip side of the fix above, made explicit rather than left implicit: `{}` genuinely IS
/// valid input for translateSelection (no arg is required in this fixture), so `prepare` must
/// succeed for it — pinning down the exact behaviour the two tests above now correctly assume.
#[test]
fn empty_input_is_valid_for_a_capability_with_no_required_args() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = real_fixture_catalog();
    let session = SessionHandle::new("sess_1");
    let principal = principal(&["artifact.write"]);
    adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({}), 0, 0).expect("no arg is required, so {} must be accepted");
}

#[test]
fn transaction_begin_requires_at_least_one_prepared_handle() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let session = SessionHandle::new("sess_1");
    let error = adapter.transaction_begin(&session, &[], 0).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::InputInvalid);
}
//#endregion 🔖️Misc

//#region 💡️Inference
/// 💡️ The general inference route reaches the channel as a REAL `AppCommand::Infer` carrying every
/// identity field the router needs, and the guest's own answer comes straight back out — the
/// contract `🏠️workspace`'s `PluginArtifactChannel` implements against `ArtifactInferenceRouter`.
#[test]
fn run_inference_sends_one_infer_command_and_returns_the_guest_result() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let command = InferCommand {
        plugin_id: "wfc".into(),
        artifact_kind: "s.wfc.wfc3d".into(),
        inference_schema: "s.wfc.wfc3d.solve".into(),
        revision: 4,
        generation: 0,
        cancellation_id: "cancel-1".into(),
        work_units: 64,
        canonical_payload: b"{\"seed\":7}".to_vec(),
        artifact_id: String::new(),
        artifact_document: None,
        cancel: crate::actions::InferenceCancel::default(),
    };
    let outcome = adapter.run_inference(3, command.clone()).expect("the mock channel answers a real Inferred frame");
    assert_eq!(outcome.inference_schema, "s.wfc.wfc3d.solve");
    assert!(outcome.complete);
    assert_eq!(outcome.payload, b"{\"seed\":7}".to_vec());
    assert_eq!(channel.frame_log(), vec![(3, AppCommand::Infer(command))], "exactly one Infer command, on the instance the caller named");
}

/// 💡️ A stale base generation is the guest/store's own rejection, mapped through the SAME
/// `map_fault` table every mutation fault travels — never swallowed, never retried silently.
#[test]
fn run_inference_maps_a_stale_generation_to_a_revision_conflict() {
    let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    channel.bump_generation(0);
    let command = InferCommand { plugin_id: "gis".into(), artifact_kind: "s.gis.gismap".into(), inference_schema: "s.gis.gismap.inference".into(), generation: 0, work_units: 1, ..InferCommand::default() };
    let error = adapter.run_inference(0, command).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::RevisionConflict);
}
//#endregion 💡️Inference

//#region 🔖️UnboundChannelAndResolvableApproval
// 🎫️ slice M4 (audit §6 P0.2 + P0.3). Two things that were silently wrong in the shipped binary:
// a gateway with no `--folder`/`--hub` ran the whole mutation protocol against a scripted mock with
// no signal to the caller, and a parked approval could never be decided by anyone.
fn unbound_harness() -> ActionAdapter {
    ActionAdapter::new(
        Box::new(ArtifactChannels::Unbound(UnboundArtifactChannel)),
        Arc::new(HandleTable::new()),
        Arc::new(IdempotencyStore::new()),
        Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())),
        AutoApprovePolicy::Never,
        ClientInfo { name: "test".into(), version: "0".into() },
    )
}

#[test]
fn an_unbound_gateway_answers_a_typed_retryable_error_instead_of_a_scripted_commit() {
    let adapter = unbound_harness();
    let catalog = single_capability_catalog(synthetic_capability("gateway.harmlessThing", &[], ApprovalMode::Never, false));
    let session = SessionHandle::new("sess_unbound");
    let principal = principal(&[]);

    let error = adapter.prepare(&catalog, &principal, &session, "gateway.harmlessThing", serde_json::json!({}), 0, 0).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable, "an unbound workspace is a binding gap the caller can close, not a permanent failure");
    assert!(error.message.contains("--folder") && error.message.contains("--hub"), "the error names both bindings: {}", error.message);

    let invoked = adapter.invoke(&catalog, &principal, &session, InvokeRequest { capability_id: Some("gateway.harmlessThing".into()), ..Default::default() }, 0, 1).unwrap_err();
    assert_eq!(invoked.code, GatewayErrorCode::PluginUnavailable);
}

#[test]
fn the_unbound_channel_never_answers_a_frame_for_any_command() {
    let mut channel = UnboundArtifactChannel;
    for command in [AppCommand::ReadHistory, AppCommand::TransactionCommit { txn_id: "txn".into() }, AppCommand::TransactionUndo { group_id: "grp".into() }] {
        let fault = channel.exchange(0, vec![command]).expect_err("an unbound channel has nothing to answer with");
        assert_eq!(fault.code, WORKSPACE_UNBOUND_FAULT_CODE);
    }
}

#[test]
fn with_no_approval_coordinator_bound_the_error_says_nobody_could_be_asked() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    let catalog = single_capability_catalog(synthetic_capability("gateway.destructiveThing", &[], ApprovalMode::Always, true));
    let session = SessionHandle::new("sess_nochannel");
    let principal = principal(&[]);
    let blocked = adapter.invoke(&catalog, &principal, &session, InvokeRequest { capability_id: Some("gateway.destructiveThing".into()), ..Default::default() }, 0, 1).unwrap_err();
    assert_eq!(blocked.code, GatewayErrorCode::ApprovalRequired);
    assert!(blocked.details["approvalHandle"].as_str().is_some());
    assert!(blocked.details["channels"].is_object(), "the caller is told which lanes were tried: {}", blocked.details);
}

#[test]
fn a_bound_coordinator_whose_human_says_yes_lets_the_invocation_commit_in_one_call() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    adapter.bind_approval_coordinator(Arc::new(ApprovalCoordinator::new(Some(accepting_elicitation_slot()), None)));
    let catalog = single_capability_catalog(synthetic_capability("gateway.destructiveThing", &[], ApprovalMode::Always, true));
    let session = SessionHandle::new("sess_yes");
    let principal = principal(&[]);
    let report = adapter
        .invoke(&catalog, &principal, &session, InvokeRequest { capability_id: Some("gateway.destructiveThing".into()), ..Default::default() }, 0, 1)
        .expect("a human yes resolves the gate inside the same invocation");
    assert_eq!(report.status, InvocationStatus::Succeeded);
}

#[test]
fn a_bound_coordinator_whose_human_says_no_denies_rather_than_asking_again() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
    adapter.bind_approval_coordinator(Arc::new(ApprovalCoordinator::new(Some(declining_elicitation_slot()), None)));
    let catalog = single_capability_catalog(synthetic_capability("gateway.destructiveThing", &[], ApprovalMode::Always, true));
    let session = SessionHandle::new("sess_no");
    let principal = principal(&[]);
    let denied = adapter.invoke(&catalog, &principal, &session, InvokeRequest { capability_id: Some("gateway.destructiveThing".into()), ..Default::default() }, 0, 1).unwrap_err();
    assert_eq!(denied.code, GatewayErrorCode::PermissionDenied);
    assert_eq!(denied.details["channel"], "elicitation");
}

#[test]
fn auto_approve_all_needs_no_human_at_all() {
    let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::All);
    let catalog = single_capability_catalog(synthetic_capability("gateway.destructiveThing", &[], ApprovalMode::Always, true));
    let session = SessionHandle::new("sess_auto");
    let principal = principal(&[]);
    let report = adapter
        .invoke(&catalog, &principal, &session, InvokeRequest { capability_id: Some("gateway.destructiveThing".into()), ..Default::default() }, 0, 1)
        .expect("--auto-approve all is the launch-time human decision");
    assert_eq!(report.status, InvocationStatus::Succeeded);
}

fn elicitation_slot_answering(response_line: &str) -> crate::transport::ElicitationSlot {
    let features = Arc::new(crate::protocol::ClientFeatures::default());
    features.record(Some(&serde_json::json!({ "elicitation": {} })));
    let lines = Arc::new(crate::transport::StdioLines::new(Box::new(std::io::Cursor::new(response_line.as_bytes().to_vec())), Box::new(Vec::new())));
    let slot: crate::transport::ElicitationSlot = Arc::new(std::sync::OnceLock::new());
    let _ = slot.set(Arc::new(crate::transport::ElicitationChannel::new(lines, features)));
    slot
}

fn accepting_elicitation_slot() -> crate::transport::ElicitationSlot {
    elicitation_slot_answering("{\"jsonrpc\":\"2.0\",\"id\":\"semio-elicit-1\",\"result\":{\"action\":\"accept\",\"content\":{\"approve\":true}}}\n")
}

fn declining_elicitation_slot() -> crate::transport::ElicitationSlot {
    elicitation_slot_answering("{\"jsonrpc\":\"2.0\",\"id\":\"semio-elicit-1\",\"result\":{\"action\":\"decline\"}}\n")
}
//#endregion 🔖️UnboundChannelAndResolvableApproval
