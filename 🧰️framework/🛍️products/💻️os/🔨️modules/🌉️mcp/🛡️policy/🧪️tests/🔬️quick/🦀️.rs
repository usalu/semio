
use super::*;
use crate::catalog::{CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, ToolExposure};
use semio_framework::manifest::{CapabilityEffects, CapabilityExecution, CapabilityPolicy};

fn capability(id: &str, scopes: &[&str], approval: ApprovalMode, destructive: bool) -> CapabilityDefinition {
    CapabilityDefinition {
        id: CapabilityRef(id.to_string()),
        version: 1,
        owner: CapabilityOwner::Plugin { plugin_id: "cad".into(), app_id: Some("editor".into()), window_kind_id: Some("viewport".into()), mode_id: None },
        kind: CapabilityKind::Mutation,
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
    assert!(principal.grants(&kernel::CapabilityId("documents.write".into())));
    assert!(principal.grants(&kernel::CapabilityId("jobs.spawn".into())));
    assert!(!principal.grants(&kernel::CapabilityId("shell.raw".into())));
}

#[test]
fn ui_raw_control_expands_to_shell_raw() {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["ui.raw-control".to_string()], None);
    assert!(principal.grants(&kernel::CapabilityId("shell.raw".into())));
}

#[test]
fn an_unknown_alias_passes_through_as_a_literal_capability_id() {
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["documents.write".to_string()], None);
    assert!(principal.grants(&kernel::CapabilityId("documents.write".into())));
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
    let capability = capability("cad.editor.translateSelection", &["documents.write"], ApprovalMode::Never, false);
    let error = engine.authorize_scopes(&principal, &capability).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::PermissionDenied);
}

#[test]
fn authorize_scopes_allows_when_every_scope_is_granted() {
    let engine = PolicyEngine::new(Arc::new(HandleTable::new()), AutoApprovePolicy::Never);
    let principal = AgentPrincipal::from_scope_names("agent:local", "local", &["artifact.write".to_string()], None);
    let capability = capability("cad.editor.translateSelection", &["documents.write"], ApprovalMode::Never, false);
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
