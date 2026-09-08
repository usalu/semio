
use super::*;

fn session(id: &str) -> SessionHandle {
    SessionHandle::new(id.to_string())
}

//#region 🔖️MintResolve
#[test]
fn mint_then_resolve_by_the_owning_session_succeeds() {
    let table = HandleTable::new();
    let owner = session("sess_a");
    let id = table.mint(HandleKind::Prepared, owner.clone(), Attachment::Capability { capability_id: "cad.viewport.translateSelection".into() }, serde_json::json!({"dx": 1.0}), 1_000);
    assert!(id.starts_with("prep_"));
    let record = table.resolve(&id, &owner, 1_500).unwrap();
    assert_eq!(record.kind, HandleKind::Prepared);
    assert_eq!(record.payload["dx"], 1.0);
}

#[test]
fn every_kind_mints_with_its_frozen_prefix() {
    let table = HandleTable::new();
    let owner = session("sess_a");
    let expectations = [(HandleKind::Session, "sess_"), (HandleKind::Prepared, "prep_"), (HandleKind::Transaction, "txn_"), (HandleKind::Undo, "undo_"), (HandleKind::Job, "job_"), (HandleKind::Approval, "appr_"), (HandleKind::Continuation, "cont_")];
    for (kind, prefix) in expectations {
        let id = table.mint(kind, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
        assert!(id.starts_with(prefix), "{kind:?} minted `{id}`, expected prefix `{prefix}`");
    }
}
//#endregion 🔖️MintResolve

//#region 🔖️AuthorizationSecurity
/// 🔐️ THE security property: possession of the handle string is not enough — a session that never
/// minted/owns it must be refused even though the handle is real and unexpired.
#[test]
fn cross_session_resolve_is_permission_denied_not_a_leak() {
    let table = HandleTable::new();
    let owner = session("sess_owner");
    let thief = session("sess_thief");
    let id = table.mint(HandleKind::Transaction, owner, Attachment::None, serde_json::Value::Null, 0);
    let error = table.resolve(&id, &thief, 100).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::PermissionDenied);
}

#[test]
fn resolving_an_unknown_handle_is_not_found() {
    let table = HandleTable::new();
    let error = table.resolve("prep_does_not_exist", &session("sess_a"), 0).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}
//#endregion 🔖️AuthorizationSecurity

//#region 🔖️Expiry
#[test]
fn a_handle_past_its_ttl_is_not_found_and_removed() {
    let table = HandleTable::new();
    let owner = session("sess_a");
    let id = table.mint(HandleKind::Continuation, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
    let past_expiry = HandleKind::Continuation.default_ttl_ms() + 1;
    let error = table.resolve(&id, &owner, past_expiry).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::NotFound);
    assert_eq!(table.len(), 0, "lazily-discovered expiry must remove the record");
}

#[test]
fn session_handle_ttl_slides_forward_on_every_resolve() {
    let table = HandleTable::new();
    let owner = session("sess_a");
    let id = table.mint(HandleKind::Session, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
    let almost_expired = HandleKind::Session.default_ttl_ms() - 1;
    table.resolve(&id, &owner, almost_expired).unwrap();
    // had the TTL not slid forward on the resolve above, this would now be past the original expiry
    let would_have_been_expired = HandleKind::Session.default_ttl_ms() + 1;
    table.resolve(&id, &owner, would_have_been_expired).unwrap();
}

#[test]
fn gc_expired_removes_only_expired_records_and_reports_the_count() {
    let table = HandleTable::new();
    let owner = session("sess_a");
    table.mint(HandleKind::Continuation, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
    table.mint(HandleKind::Undo, owner, Attachment::None, serde_json::Value::Null, 0);
    let removed = table.gc_expired(HandleKind::Continuation.default_ttl_ms() + 1);
    assert_eq!(removed, 1);
    assert_eq!(table.len(), 1);
}

#[test]
fn mark_terminal_extends_a_job_handle_by_one_hour_from_now() {
    let table = HandleTable::new();
    let owner = session("sess_a");
    let id = table.mint(HandleKind::Job, owner.clone(), Attachment::None, serde_json::Value::Null, 0);
    table.mark_terminal(&id, 10_000_000).unwrap();
    let record = table.resolve(&id, &owner, 10_000_000 + HandleKind::Job.default_ttl_ms() - 1).unwrap();
    assert_eq!(record.kind, HandleKind::Job);
}

#[test]
fn mark_terminal_rejects_a_non_job_handle() {
    let table = HandleTable::new();
    let id = table.mint(HandleKind::Prepared, session("sess_a"), Attachment::None, serde_json::Value::Null, 0);
    let error = table.mark_terminal(&id, 0).unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::InputInvalid);
}
//#endregion 🔖️Expiry

//#region 🔖️Idempotency
#[test]
fn idempotent_replay_reuses_the_cached_report_and_flags_it() {
    let store = IdempotencyStore::new();
    let calls = std::cell::Cell::new(0);
    let make_report = || {
        calls.set(calls.get() + 1);
        InvocationReport {
            invocation_id: "inv-1".into(),
            capability_id: "cad.viewport.translateSelection".into(),
            status: crate::schema::InvocationStatus::Succeeded,
            affected_resources: vec![],
            revision_before: None,
            revision_after: None,
            diff_uri: None,
            warnings: vec![],
            undo_token: None,
            postconditions: vec![],
            replayed: false,
        }
    };
    let first = store.get_or_insert_with("agent:local", "key-1", 0, make_report);
    assert!(!first.replayed);
    let second = store.get_or_insert_with("agent:local", "key-1", 1_000, make_report);
    assert!(second.replayed, "a replay within the TTL window must be flagged");
    assert_eq!(second.invocation_id, "inv-1");
    assert_eq!(calls.get(), 1, "compute must run exactly once for a cached key");
}

#[test]
fn different_idempotency_keys_never_collide() {
    let store = IdempotencyStore::new();
    let a = store.get_or_insert_with("agent:local", "key-a", 0, || InvocationReport {
        invocation_id: "inv-a".into(),
        capability_id: "x".into(),
        status: crate::schema::InvocationStatus::Succeeded,
        affected_resources: vec![],
        revision_before: None,
        revision_after: None,
        diff_uri: None,
        warnings: vec![],
        undo_token: None,
        postconditions: vec![],
        replayed: false,
    });
    let b = store.get_or_insert_with("agent:local", "key-b", 0, || InvocationReport {
        invocation_id: "inv-b".into(),
        capability_id: "x".into(),
        status: crate::schema::InvocationStatus::Succeeded,
        affected_resources: vec![],
        revision_before: None,
        revision_after: None,
        diff_uri: None,
        warnings: vec![],
        undo_token: None,
        postconditions: vec![],
        replayed: false,
    });
    assert_ne!(a.invocation_id, b.invocation_id);
    assert_eq!(store.len(), 2);
}

#[test]
fn expired_idempotency_entry_recomputes() {
    let store = IdempotencyStore::new();
    let calls = std::rc::Rc::new(std::cell::Cell::new(0));
    fn report_with(calls: &std::rc::Rc<std::cell::Cell<i32>>, id: &'static str) -> InvocationReport {
        calls.set(calls.get() + 1);
        InvocationReport {
            invocation_id: id.into(),
            capability_id: "x".into(),
            status: crate::schema::InvocationStatus::Succeeded,
            affected_resources: vec![],
            revision_before: None,
            revision_after: None,
            diff_uri: None,
            warnings: vec![],
            undo_token: None,
            postconditions: vec![],
            replayed: false,
        }
    }
    let first = store.get_or_insert_with("agent:local", "key-1", 0, || report_with(&calls, "inv-1"));
    assert!(!first.replayed);
    let after_ttl = IDEMPOTENCY_TTL_MS + 1;
    let second = store.get_or_insert_with("agent:local", "key-1", after_ttl, || report_with(&calls, "inv-2"));
    assert!(!second.replayed, "past the TTL, this must recompute rather than replay");
    assert_eq!(second.invocation_id, "inv-2");
    assert_eq!(calls.get(), 2);
}
//#endregion 🔖️Idempotency
