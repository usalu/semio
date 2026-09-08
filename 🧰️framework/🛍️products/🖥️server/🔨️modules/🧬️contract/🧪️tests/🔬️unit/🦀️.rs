
use super::*;

fn envelope() -> CommandEnvelope {
    CommandEnvelope {
        command_id: CommandId("cmd-1".into()),
        kind: "artifact.mutate".into(),
        version: 1,
        target: ActorKey { tenant: TenantId("t1".into()), kind: "artifact".into(), id: "doc-1".into() },
        scope: Scope("space-1".into()),
        principal: Principal::User { id: "alice".into() },
        session: Some(SessionId("s1".into())),
        device: Some(DeviceId("d1".into())),
        payload: vec![1, 2, 3],
        causal_frontier: None,
        client_hlc: HybridLogicalClock { millis: 7, counter: 2 },
        expected_revision: Some(Revision(4)),
        idempotency_key: Some(IdempotencyKey("k1".into())),
        capability_proof: None,
        trace: TraceContext::default(),
    }
}

//#region 🔖️Command
#[test]
fn command_envelope_round_trips_through_json() {
    let original = envelope();
    let text = serde_json::to_string(&original).unwrap();
    assert_eq!(serde_json::from_str::<CommandEnvelope>(&text).unwrap(), original);
}

#[test]
fn command_outcome_variants_are_tagged_by_status() {
    let receipt = CommandReceipt { command_id: CommandId("cmd-1".into()), actor: envelope().target, revision: Revision(5), accepted_at: HybridLogicalClock::default() };
    let accepted = CommandOutcome::Accepted { receipt: receipt.clone(), events: vec![], frontier: None };
    assert!(serde_json::to_string(&accepted).unwrap().contains("\"status\":\"accepted\""));
    let rejected = CommandOutcome::Rejected { receipt, reason: Rejection::Invalid { detail: "no".into() }, notices: vec![] };
    assert!(serde_json::to_string(&rejected).unwrap().contains("\"status\":\"rejected\""));
}
//#endregion 🔖️Command

//#region 🔖️Query
#[test]
fn query_consistency_round_trips_every_variant() {
    for consistency in [QueryConsistency::Local, QueryConsistency::Authority] {
        let text = serde_json::to_string(&consistency).unwrap();
        assert_eq!(serde_json::from_str::<QueryConsistency>(&text).unwrap(), consistency);
    }
}
//#endregion 🔖️Query

//#region 🔖️Module
#[test]
fn unknown_command_kinds_default_to_authority_required() {
    let instance = ServerInstanceDefinition {
        id: "hub".into(),
        version: "0.1.0".into(),
        modules: vec![ModuleManifest { id: "documents".into(), commands: vec![CommandDescriptor { kind: "artifact.mutate".into(), version: 1, actor_kind: "artifact".into(), offline: OfflinePolicy::Optimistic }], ..Default::default() }],
    };
    assert_eq!(instance.offline_policy("artifact.mutate"), OfflinePolicy::Optimistic);
    assert_eq!(instance.offline_policy("directory.inviteMember"), OfflinePolicy::AuthorityRequired);
}
//#endregion 🔖️Module

//#region 🔖️Policy
#[test]
fn deny_is_not_allowed() {
    assert!(PolicyDecision::Allow.is_allowed());
    assert!(!PolicyDecision::Deny { reason: "nope".into() }.is_allowed());
}
//#endregion 🔖️Policy
