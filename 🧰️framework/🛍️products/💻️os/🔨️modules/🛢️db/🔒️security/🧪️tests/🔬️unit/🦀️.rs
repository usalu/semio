
use super::*;

async fn doc(id: &str) -> protocol::ArtifactId {
    protocol::ArtifactId(id.to_string())
}
async fn actor(id: &str) -> protocol::ActorId {
    protocol::ActorId(id.to_string())
}
async fn op(id: &str) -> protocol::MutationId {
    protocol::MutationId(id.to_string())
}
async fn principal(role: &str) -> Principal {
    Principal::new(actor("alice").await, TenantId::from("tenant-1"), vec![role.to_string()])
}

//#region 🔖️Identity
#[semio_framework_async_macros::async_test]
async fn check_tenant_allows_matching_and_rejects_mismatched() {
    let p = principal("editor").await;
    assert!(check_tenant(&p, &TenantId::from("tenant-1")).is_ok());
    assert!(matches!(check_tenant(&p, &TenantId::from("tenant-2")), Err(DbError::Unauthorized(_))));
}

#[semio_framework_async_macros::async_test]
async fn principal_has_role_matches_exactly() {
    let p = Principal::new(actor("alice").await, TenantId::from("t1"), vec!["editor".to_string(), "viewer".to_string()]);
    assert!(p.has_role("editor"));
    assert!(!p.has_role("admin"));
}
//#endregion 🔖️Identity

//#region 🔖️Scope
#[semio_framework_async_macros::async_test]
async fn scope_segments_nest_under_document() {
    assert_eq!(AuthzScope::Database.segments(), vec!["db"]);
    assert_eq!(AuthzScope::Document { document: doc("doc-1").await }.segments(), vec!["db", "document", "doc-1"]);
    assert_eq!(AuthzScope::CommandKind { document: doc("doc-1").await, kind: "edit".to_string() }.segments(), vec!["db", "document", "doc-1", "kind", "edit"]);
    assert_eq!(AuthzScope::Field { document: doc("doc-1").await, object_id: "obj-1".to_string(), field: "name".to_string() }.segments(), vec!["db", "document", "doc-1", "object", "obj-1", "field", "name"]);
}

#[semio_framework_async_macros::async_test]
async fn scope_document_extracts_owning_document_or_none() {
    assert_eq!(AuthzScope::Database.document().await, None);
    assert_eq!(AuthzScope::Historical { document: doc("doc-1").await }.document().await, Some(&doc("doc-1").await));
    assert_eq!(AuthzScope::Preview { document: doc("doc-1").await }.document().await, Some(&doc("doc-1").await));
}
//#endregion 🔖️Scope

//#region 🔖️Policy
#[semio_framework_async_macros::async_test]
async fn pattern_matches_exact_wildcard_and_trailing_double_star() {
    let seg = |s: &[&str]| s.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let pat = |s: &[&str]| s.iter().map(|x| x.to_string()).collect::<Vec<_>>();

    assert!(pattern_matches(&pat(&["db", "document", "doc-1"]), &seg(&["db", "document", "doc-1"])));
    assert!(!pattern_matches(&pat(&["db", "document", "doc-2"]), &seg(&["db", "document", "doc-1"])));
    assert!(pattern_matches(&pat(&["db", "document", "*", "kind", "edit"]), &seg(&["db", "document", "doc-1", "kind", "edit"])));
    assert!(pattern_matches(&pat(&["db", "document", "doc-1", "**"]), &seg(&["db", "document", "doc-1", "field", "x"])));
    assert!(pattern_matches(&pat(&["db", "document", "doc-1", "**"]), &seg(&["db", "document", "doc-1"])));
    assert!(!pattern_matches(&pat(&["db", "document", "doc-1", "kind", "edit"]), &seg(&["db", "document", "doc-1", "kind"])));
    assert!(!pattern_matches(&pat(&["db", "document", "doc-1"]), &seg(&["db", "document", "doc-1", "kind", "edit"])));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_default_denies_with_no_matching_grant() {
    let policy = RoleBasedPolicy::new();
    let decision = policy.evaluate(&principal("editor").await, &AuthzScope::Document { document: doc("doc-1").await }, Action::Read).await;
    assert!(!decision.is_allowed());
}

#[semio_framework_async_macros::async_test]
async fn evaluate_allows_on_matching_role_pattern_and_action() {
    let policy = RoleBasedPolicy::new().with_grant(Grant::allow("editor", &["db", "document", "*", "**"], &[Action::Read, Action::Write]));
    let decision = policy.evaluate(&principal("editor").await, &AuthzScope::CommandKind { document: doc("doc-1").await, kind: "edit".to_string() }, Action::Write).await;
    assert!(decision.is_allowed());

    let wrong_role = policy.evaluate(&principal("viewer").await, &AuthzScope::Document { document: doc("doc-1").await }, Action::Read).await;
    assert!(!wrong_role.is_allowed());
}

#[semio_framework_async_macros::async_test]
async fn evaluate_explicit_deny_always_wins_over_allow() {
    let policy = RoleBasedPolicy::new().with_grant(Grant::allow("editor", &["db", "document", "doc-1", "**"], &[Action::Read])).with_grant(Grant::deny("editor", &["db", "document", "doc-1", "object", "secret", "**"], &[Action::Read]));
    let allowed = policy.evaluate(&principal("editor").await, &AuthzScope::Object { document: doc("doc-1").await, object_id: "public".to_string() }, Action::Read).await;
    assert!(allowed.is_allowed());

    let denied = policy.evaluate(&principal("editor").await, &AuthzScope::Field { document: doc("doc-1").await, object_id: "secret".to_string(), field: "value".to_string() }, Action::Read).await;
    assert!(!denied.is_allowed());
}

#[semio_framework_async_macros::async_test]
async fn decision_into_result_maps_deny_to_unauthorized() {
    assert!(Decision::Allow.into_result().await.is_ok());
    let err = Decision::Deny { reason: "nope".to_string() }.into_result().await.unwrap_err();
    assert!(matches!(err, DbError::Unauthorized(reason) if reason == "nope"));
}
//#endregion 🔖️Policy

//#region 🔖️SpaceGrants
#[semio_framework_async_macros::async_test]
async fn space_grants_studio_allows_author_write_and_spectator_read_only() {
    let policy = RoleBasedPolicy::new();
    let policy = space_grants("space-1", "studio").await.into_iter().fold(policy, RoleBasedPolicy::with_grant);
    let scope = AuthzScope::CommandKind { document: doc("space-1:doc-1").await, kind: "edit".to_string() };

    assert!(policy.evaluate(&principal("author").await, &scope, Action::Write).await.is_allowed());
    assert!(policy.evaluate(&principal("author").await, &scope, Action::Read).await.is_allowed());
    assert!(policy.evaluate(&principal("spectator").await, &scope, Action::Read).await.is_allowed());
    assert!(!policy.evaluate(&principal("spectator").await, &scope, Action::Write).await.is_allowed());
}

#[semio_framework_async_macros::async_test]
async fn space_grants_archive_denies_author_write_even_though_allow_also_matches() {
    let policy = space_grants("space-1", "archive").await.into_iter().fold(RoleBasedPolicy::new(), RoleBasedPolicy::with_grant);
    let scope = AuthzScope::CommandKind { document: doc("space-1:doc-1").await, kind: "edit".to_string() };

    assert!(!policy.evaluate(&principal("author").await, &scope, Action::Write).await.is_allowed(), "deny must win over the author allow grant");
    assert!(policy.evaluate(&principal("author").await, &scope, Action::Read).await.is_allowed(), "archive still permits reads");
}
//#endregion 🔖️SpaceGrants

//#region 🔖️Signing
struct FixedSigner {
    signature: Vec<u8>,
}
impl protocol::Signer for FixedSigner {
    async fn scheme(&self) -> &str {
        "test-scheme"
    }
    async fn key_id(&self) -> &str {
        "test-key"
    }
    async fn sign(&self, _message: &[u8; 32]) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(self.signature.clone())
    }
}
struct FailingSigner;
impl protocol::Signer for FailingSigner {
    async fn scheme(&self) -> &str {
        "test-scheme"
    }
    async fn key_id(&self) -> &str {
        "test-key"
    }
    async fn sign(&self, _message: &[u8; 32]) -> Result<Vec<u8>, protocol::ProtocolError> {
        Err(protocol::ProtocolError::LimitExceeded("too big"))
    }
}
struct ExactVerifier {
    expected: Vec<u8>,
}
impl protocol::SignatureVerifier for ExactVerifier {
    async fn verify(&self, _scheme: &str, _key_id: &str, _message: &[u8; 32], signature: &[u8]) -> Result<bool, protocol::ProtocolError> {
        Ok(signature == self.expected.as_slice())
    }
}

#[semio_framework_async_macros::async_test]
async fn sign_then_verify_round_trips() {
    let signer = FixedSigner { signature: vec![1, 2, 3, 4] };
    let message = [7u8; 32];
    let signature = sign_message(&signer, &message).await.unwrap();
    assert_eq!(signature.scheme, "test-scheme");
    assert_eq!(signature.bytes, vec![1, 2, 3, 4]);

    let verifier = ExactVerifier { expected: vec![1, 2, 3, 4] };
    assert!(verify_signature(&verifier, &signature, &message).await.is_ok());
}

#[semio_framework_async_macros::async_test]
async fn verify_rejects_mismatched_signature_without_panicking() {
    let signature = Signature { scheme: "test-scheme".to_string(), key_id: "test-key".to_string(), bytes: vec![9, 9, 9] };
    let verifier = ExactVerifier { expected: vec![1, 2, 3] };
    let err = verify_signature(&verifier, &signature, &[0u8; 32]).await.unwrap_err();
    assert!(matches!(err, DbError::Unauthorized(_)));
}

#[semio_framework_async_macros::async_test]
async fn sign_message_maps_protocol_error_by_category() {
    let err = sign_message(&FailingSigner, &[0u8; 32]).await.unwrap_err();
    assert_eq!(err, DbError::LimitExceeded("too big"));
}
//#endregion 🔖️Signing

//#region 🔖️Replay
#[semio_framework_async_macros::async_test]
async fn replay_guard_rejects_duplicate_operation_within_window() {
    let mut guard = ReplayGuard::new(1_000, 16);
    let a = actor("alice").await;
    let o = op("op-1").await;
    assert!(guard.check_and_record(&a, &o, 0).is_ok());
    let err = guard.check_and_record(&a, &o, 500).unwrap_err();
    assert!(matches!(err, DbError::Conflict(_)));
}

#[semio_framework_async_macros::async_test]
async fn replay_guard_allows_same_operation_id_after_window_expires() {
    let mut guard = ReplayGuard::new(1_000, 16);
    let a = actor("alice").await;
    let o = op("op-1").await;
    assert!(guard.check_and_record(&a, &o, 0).is_ok());
    assert!(guard.check_and_record(&a, &o, 2_000).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn replay_guard_is_isolated_per_actor() {
    let mut guard = ReplayGuard::new(1_000, 16);
    let o = op("op-1").await;
    assert!(guard.check_and_record(&actor("alice").await, &o, 0).is_ok());
    assert!(guard.check_and_record(&actor("bob").await, &o, 0).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn replay_guard_evicts_oldest_beyond_capacity_bounding_memory() {
    let mut guard = ReplayGuard::new(1_000_000, 2);
    let a = actor("alice").await;
    assert!(guard.check_and_record(&a, &op("op-1").await, 0).is_ok());
    assert!(guard.check_and_record(&a, &op("op-2").await, 0).is_ok());
    assert!(guard.check_and_record(&a, &op("op-3").await, 0).is_ok());
    assert!(guard.check_and_record(&a, &op("op-1").await, 0).is_ok(), "op-1 should have been evicted to bound memory");
    assert!(guard.check_and_record(&a, &op("op-3").await, 0).is_err(), "op-3 is still within capacity and must still be caught");
}
//#endregion 🔖️Replay

//#region 🔖️Budget
#[semio_framework_async_macros::async_test]
async fn budget_registry_exhausts_then_refills_over_time() {
    let mut budgets = BudgetRegistry::new(2, 1);
    assert!(budgets.try_consume("alice", 1, 0).is_ok());
    assert!(budgets.try_consume("alice", 1, 0).is_ok());
    assert!(matches!(budgets.try_consume("alice", 1, 0), Err(DbError::LimitExceeded(_))));
    assert!(budgets.try_consume("alice", 1, 1_000).is_ok());
}

#[semio_framework_async_macros::async_test]
async fn budget_registry_keys_are_independent() {
    let mut budgets = BudgetRegistry::new(1, 1);
    assert!(budgets.try_consume("alice", 1, 0).is_ok());
    assert!(budgets.try_consume("bob", 1, 0).is_ok());
    assert!(budgets.try_consume("alice", 1, 0).is_err());
}
//#endregion 🔖️Budget

//#region 🔖️Redaction
#[semio_framework_async_macros::async_test]
async fn redact_fields_hides_denied_nested_field_and_keeps_allowed_siblings() {
    let policy = RoleBasedPolicy::new().with_grant(Grant::allow("viewer", &["db", "document", "doc-1", "object", "obj-1", "field", "**"], &[Action::Read])).with_grant(Grant::deny(
        "viewer",
        &["db", "document", "doc-1", "object", "obj-1", "field", "ssn"],
        &[Action::Read],
    ));
    let value = serde_json::json!({"name": "Ada", "ssn": "123-45-6789", "address": {"city": "Zurich"}});

    let redacted = redact_fields(&policy, &principal("viewer").await, &doc("doc-1").await, "obj-1", &value).await;

    assert_eq!(redacted["name"], serde_json::json!("Ada"));
    assert_eq!(redacted["ssn"], serde_json::json!({"$redacted": true}));
    assert_eq!(redacted["address"]["city"], serde_json::json!("Zurich"));
}

#[semio_framework_async_macros::async_test]
async fn redact_fields_does_not_recurse_into_a_denied_subtree() {
    let policy = RoleBasedPolicy::new().with_grant(Grant::deny("viewer", &["db", "document", "doc-1", "object", "obj-1", "field", "secret"], &[Action::Read]));
    let value = serde_json::json!({"secret": {"nested": "value"}});
    let redacted = redact_fields(&policy, &principal("viewer").await, &doc("doc-1").await, "obj-1", &value).await;
    assert_eq!(redacted["secret"], serde_json::json!({"$redacted": true}));
}

#[semio_framework_async_macros::async_test]
async fn redact_fields_top_level_object_itself_is_never_field_checked() {
    let policy = RoleBasedPolicy::new();
    let value = serde_json::json!({"a": 1});
    let redacted = redact_fields(&policy, &principal("viewer").await, &doc("doc-1").await, "obj-1", &value).await;
    assert_eq!(redacted["a"], serde_json::json!({"$redacted": true}));
}

#[semio_framework_async_macros::async_test]
async fn redact_fields_beyond_depth_ceiling_is_conservatively_redacted() {
    let policy = RoleBasedPolicy::new().with_grant(Grant::allow("viewer", &["db", "document", "doc-1", "object", "obj-1", "field", "**"], &[Action::Read]));
    let mut value = serde_json::json!("leaf");
    for _ in 0..(MAX_REDACT_DEPTH + 5) {
        value = serde_json::json!({ "n": value });
    }
    let redacted = redact_fields(&policy, &principal("viewer").await, &doc("doc-1").await, "obj-1", &value).await;

    let mut cursor = &redacted;
    for _ in 0..MAX_REDACT_DEPTH {
        cursor = &cursor["n"];
    }
    assert_ne!(*cursor, serde_json::json!({"$redacted": true}), "the ceiling must not fire before it is reached");
    cursor = &cursor["n"];
    assert_eq!(*cursor, serde_json::json!({"$redacted": true}), "the ceiling must fire once depth exceeds MAX_REDACT_DEPTH");
}
//#endregion 🔖️Redaction

//#region 🔖️Audit
struct RecordingEmit {
    events: std::sync::Mutex<Vec<EmitEvent>>,
}
impl Emit for RecordingEmit {
    async fn emit(&self, event: EmitEvent) {
        self.events.lock().unwrap().push(event);
    }
}

#[semio_framework_async_macros::async_test]
async fn audit_decision_emits_named_event_with_reason_on_deny() {
    let sink = RecordingEmit { events: std::sync::Mutex::new(Vec::new()) };
    let decision = Decision::Deny { reason: "no grant".to_string() };
    audit_decision(&sink, &principal("editor").await, &AuthzScope::Document { document: doc("doc-1").await }, Action::Read, &decision).await;
    let events = sink.events.lock().unwrap();
    assert_eq!(events[0].name, "security.authz_denied");
    assert_eq!(events[0].document, Some(ArtifactId::from("doc-1")));
}
//#endregion 🔖️Audit

//#region 🔖️Gate
#[semio_framework_async_macros::async_test]
async fn security_gate_admit_command_enforces_authz_then_budget_then_replay() {
    let policy = RoleBasedPolicy::new().with_grant(Grant::allow("editor", &["db", "document", "*", "**"], &[Action::Write]));
    let sink = std::sync::Arc::new(RecordingEmit { events: std::sync::Mutex::new(Vec::new()) });
    let gate = SecurityGate::new(policy, ReplayGuard::new(10_000, 16), BudgetRegistry::new(1, 1), sink.clone());
    let editor = principal("editor").await;

    assert!(gate.admit_command(&editor, &TenantId::from("tenant-1"), &doc("doc-1").await, "edit", &actor("alice").await, &op("op-1").await, 0).await.is_ok());

    let budget_err = gate.admit_command(&editor, &TenantId::from("tenant-1"), &doc("doc-1").await, "edit", &actor("alice").await, &op("op-2").await, 0).await.unwrap_err();
    assert!(matches!(budget_err, DbError::LimitExceeded(_)));

    let gate2 = SecurityGate::new(RoleBasedPolicy::new().with_grant(Grant::allow("editor", &["db", "document", "*", "**"], &[Action::Write])), ReplayGuard::new(10_000, 16), BudgetRegistry::new(10, 1), sink);
    assert!(gate2.admit_command(&editor, &TenantId::from("tenant-1"), &doc("doc-1").await, "edit", &actor("alice").await, &op("op-1").await, 0).await.is_ok());
    let replay_err = gate2.admit_command(&editor, &TenantId::from("tenant-1"), &doc("doc-1").await, "edit", &actor("alice").await, &op("op-1").await, 100).await.unwrap_err();
    assert!(matches!(replay_err, DbError::Conflict(_)));
}

#[semio_framework_async_macros::async_test]
async fn security_gate_admit_command_rejects_cross_tenant_before_authz() {
    let policy = RoleBasedPolicy::new().with_grant(Grant::allow("editor", &["db", "**"], &[Action::Write]));
    let sink = std::sync::Arc::new(RecordingEmit { events: std::sync::Mutex::new(Vec::new()) });
    let gate = SecurityGate::new(policy, ReplayGuard::new(10_000, 16), BudgetRegistry::new(10, 1), sink);
    let editor = principal("editor").await;
    let err = gate.admit_command(&editor, &TenantId::from("tenant-2"), &doc("doc-1").await, "edit", &actor("alice").await, &op("op-1").await, 0).await.unwrap_err();
    assert!(matches!(err, DbError::Unauthorized(_)));
}

#[semio_framework_async_macros::async_test]
async fn security_gate_redact_forwards_to_policy() {
    let policy = RoleBasedPolicy::new().with_grant(Grant::allow("editor", &["db", "document", "doc-1", "object", "obj-1", "field", "**"], &[Action::Read])).with_grant(Grant::deny(
        "editor",
        &["db", "document", "doc-1", "object", "obj-1", "field", "secret"],
        &[Action::Read],
    ));
    let sink = std::sync::Arc::new(RecordingEmit { events: std::sync::Mutex::new(Vec::new()) });
    let gate = SecurityGate::new(policy, ReplayGuard::new(1_000, 16), BudgetRegistry::new(10, 1), sink);
    let value = serde_json::json!({"secret": "hidden", "open": "visible"});
    let redacted = gate.redact(&principal("editor").await, &doc("doc-1").await, "obj-1", &value).await;
    assert_eq!(redacted["secret"], serde_json::json!({"$redacted": true}));
    assert_eq!(redacted["open"], serde_json::json!("visible"));
}
//#endregion 🔖️Gate
