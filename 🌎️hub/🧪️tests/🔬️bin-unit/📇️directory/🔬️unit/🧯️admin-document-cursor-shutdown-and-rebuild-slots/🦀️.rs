/// 🧯️ Admin document cursor, shutdown and rebuild slots: the principal bound document cursor, the retained short admin shutdown and the atomic admin rebuild slots.

#[test]
fn admin_document_cursor_is_principal_route_and_exact_page_bound() {
    let cursor_key = [0x5a; 32];
    let principal = AdminPrincipalV1 {
        user_id: "user:admin".into(),
        auth_session_id: "session:admin".into(),
        authorization_generation: 7,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: now_ms() + 60_000,
        correlation_id: "correlation:admin".into(),
        peer_class: "admin-rest",
        session_kind: AuthSessionKind::DevelopmentLocal,
        device_instance_id: "admin-device".into(),
    };
    let cursor = admin_cursor_encode_scoped(&cursor_key, &principal, 5, Some("space:one"), ADMIN_PAGE_MAX).expect("document cursor");
    assert_eq!(cursor.len(), 84);
    assert_eq!(admin_cursor_decode_scoped(&cursor_key, &principal, 5, Some("space:one"), Some(&cursor)), Ok(ADMIN_PAGE_MAX));
    assert_eq!(admin_cursor_decode_scoped(&cursor_key, &principal, 5, Some("space:two"), Some(&cursor)), Err(StatusCode::BAD_REQUEST));
    assert_eq!(admin_cursor_decode(&cursor_key, &principal, 2, Some(&cursor)), Err(StatusCode::BAD_REQUEST));
    let mut other_principal = principal.clone();
    other_principal.auth_session_id = "session:other".into();
    assert_eq!(admin_cursor_decode_scoped(&cursor_key, &other_principal, 5, Some("space:one"), Some(&cursor)), Err(StatusCode::BAD_REQUEST));
    assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(ADMIN_PAGE_MAX) }), Ok(ADMIN_PAGE_MAX));
    assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(0) }), Err(StatusCode::BAD_REQUEST));
    assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(ADMIN_PAGE_MAX + 1) }), Err(StatusCode::BAD_REQUEST));
}

#[tokio::test]
async fn retained_short_admin_shutdown_drains_before_bounded_abort_and_receipt_reconciliation_is_exact() {
    let cancelled = AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    };
    assert!(cancelled.request_cancel());
    assert!(!cancelled.admit_effect(), "cancellation linearized before admission refuses the effect");
    let admitted = AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    };
    assert!(admitted.admit_effect());
    assert!(!admitted.request_cancel(), "admission linearized before cancellation cannot claim rollback");
    ADMIN_SECRET_WIPE_BYTES.store(0, std::sync::atomic::Ordering::SeqCst);
    let invite = "invite.v1.selector.secret".to_string();
    let share = "share.v1.selector.secret".to_string();
    let expected_wipes = invite.len() + share.len();
    drop(AdminIntentSecretResult::Invite(invite));
    drop(AdminIntentSecretResult::Share(share));
    assert_eq!(ADMIN_SECRET_WIPE_BYTES.load(std::sync::atomic::Ordering::SeqCst), expected_wipes);

    let state = test_state().await;
    let principal = AdminPrincipalV1 {
        user_id: "user:admin".into(),
        auth_session_id: "session:admin".into(),
        authorization_generation: 1,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: now_ms() + 60_000,
        correlation_id: "correlation:retained".into(),
        peer_class: "admin-rest",
        session_kind: AuthSessionKind::DevelopmentLocal,
        device_instance_id: "admin-device".into(),
    };
    let metadata = AdminIntentMetadata { intent_kind: "delete-space", target_kind: "space", target_id: "space:one".into(), reason_code: None };
    let mut accepted = new_admin_audit_fact(&principal, "request:stale-accepted", &"11".repeat(32), "operation:stale-accepted", &metadata, "accepted", None, "accepted");
    accepted.occurred_at = now_ms() - 60_000;
    state.directory.append_admin_operation_audit(&accepted).await.expect("stale accepted audit");
    let rows = reconcile_stale_admin_acceptance(&state, state.directory.admin_operation_audit_for_request(&accepted.request_id).await.expect("stale audit read")).await.expect("stale reconciliation");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].fact.phase, "accepted", "age alone cannot prove cancellation or rollback");
    let unresolved = admin_audit_visibility(admin_audit_receipt(&rows).expect("stale accepted receipt"), false);
    assert_eq!(unresolved.state, AdminIntentStateV1::Indeterminate);
    assert_eq!(unresolved.outcome.code, "admin-effect-outcome-indeterminate");
    assert_eq!(admin_audit_visibility(admin_audit_receipt(&rows).expect("live accepted receipt"), true).state, AdminIntentStateV1::Accepted);

    let owner = issue_test_session(&state, "retained-reconcile-owner@example.test").await;
    let space = create_space_for_test(&state, &owner.user_id, "Receipt Reconciliation", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let scope = DocumentScope::new(&space, "receipt-reconciliation-document");
    announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
    let committed_digest = "22".repeat(32);
    let committed_metadata = AdminIntentMetadata { intent_kind: "issue-document-share", target_kind: "document", target_id: format!("{}/{}", scope.space_id, scope.document_id), reason_code: None };
    let mut committed = new_admin_audit_fact(&principal, "request:committed-accepted", &committed_digest, "operation:committed-accepted", &committed_metadata, "accepted", None, "accepted");
    committed.occurred_at = now_ms() - 60_000;
    state.directory.append_admin_operation_audit(&committed).await.expect("committed acceptance audit");
    let effect = NewAdminOperationEffectReceiptV1 { operation_id: committed.operation_id.clone(), intent_digest: committed.intent_digest.clone(), committed_at: now_ms(), outcome_code: "share-issued".into() };
    let AdminEffectCommitV1::Applied(issued) = state.directory.issue_share_token_as_with_admin_effect(&scope, 600, Some(&principal.user_id), &principal.correlation_id, &effect).await else {
        panic!("atomic share and effect receipt");
    };
    drop(issued);
    assert!(tokio::time::timeout(std::time::Duration::ZERO, std::future::pending::<()>()).await.is_err(), "effect commit can precede acknowledgement deadline");
    assert!(state.directory.admin_operation_effect_receipt(&committed.operation_id, &"33".repeat(32)).await.expect("mismatched receipt query").is_none());
    let rows = reconcile_stale_admin_acceptance(&state, state.directory.admin_operation_audit_for_request(&committed.request_id).await.expect("committed audit read")).await.expect("factual receipt reconciliation");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].fact.phase, "succeeded");
    assert_eq!(rows[1].fact.outcome_code, "share-issued");
    assert_eq!(rows[1].fact.event_seq_first, None);
    assert_eq!(rows[1].fact.event_seq_last, None);

    let owner = AdminOperationTaskOwner::new(std::time::Duration::from_secs(1));
    let runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    let drained = Arc::new(std::sync::atomic::AtomicBool::new(false));
    assert!(owner
        .spawn("operation:drain".into(), runtime.clone(), {
            let drained = drained.clone();
            let runtime = runtime.clone();
            async move {
                while !runtime.progress().cancel_requested {
                    tokio::task::yield_now().await;
                }
                drained.store(true, std::sync::atomic::Ordering::Release);
            }
        })
        .is_ok());
    owner.shutdown().await;
    assert!(drained.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(owner.task_count(), 0);
    assert!(owner.spawn("operation:after-close".into(), runtime.clone(), async {}).is_err(), "shutdown refuses later operation tasks");

    struct DropSignal(Option<tokio::sync::oneshot::Sender<()>>);
    impl Drop for DropSignal {
        fn drop(&mut self) {
            if let Some(signal) = self.0.take() {
                let _ = signal.send(());
            }
        }
    }
    let aborting = AdminOperationTaskOwner::new(std::time::Duration::from_millis(10));
    let abort_runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_ADMITTED),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    let (dropped_tx, dropped_rx) = tokio::sync::oneshot::channel();
    assert!(aborting
        .spawn("operation:bounded-abort".into(), abort_runtime, async move {
            let _signal = DropSignal(Some(dropped_tx));
            std::future::pending::<()>().await;
        })
        .is_ok());
    aborting.shutdown().await;
    tokio::time::timeout(std::time::Duration::from_secs(1), dropped_rx).await.expect("aborted task drop deadline").expect("aborted task dropped");
    assert_eq!(aborting.task_count(), 0);
}

#[tokio::test]
async fn admin_rebuild_slots_are_atomic_and_abort_closes_once() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
    directory.seed().await.expect("seed directory");
    let directory = Arc::new(HubDirectories::from(directory));
    let operations = Arc::new(ShardedMap::new());
    let operation_slots = Arc::new(tokio::sync::Semaphore::new(64));
    let barrier = Arc::new(tokio::sync::Barrier::new(129));
    let release = Arc::new(tokio::sync::Notify::new());
    let acquired = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut contenders = Vec::new();
    for _ in 0..128 {
        let barrier = barrier.clone();
        let release = release.clone();
        let acquired = acquired.clone();
        let slots = operation_slots.clone();
        contenders.push(tokio::spawn(async move {
            barrier.wait().await;
            if let Ok(_permit) = slots.try_acquire_owned() {
                acquired.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                release.notified().await;
            }
        }));
    }
    barrier.wait().await;
    for _ in 0..128 {
        if acquired.load(std::sync::atomic::Ordering::Acquire) == 64 {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(acquired.load(std::sync::atomic::Ordering::Acquire), 64);
    assert_eq!(operation_slots.available_permits(), 0);
    release.notify_waiters();
    for contender in contenders {
        contender.await.expect("slot contender");
    }
    assert_eq!(operation_slots.available_permits(), 64);

    let principal = AdminPrincipalV1 {
        user_id: "user:admin".into(),
        auth_session_id: "session:admin".into(),
        authorization_generation: 1,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: now_ms() + 60_000,
        correlation_id: "correlation:admin".into(),
        peer_class: "admin-rest",
        session_kind: AuthSessionKind::DevelopmentLocal,
        device_instance_id: "admin-device".into(),
    };
    let metadata = AdminIntentMetadata { intent_kind: "rebuild-directory-projections", target_kind: "directory", target_id: "directory".into(), reason_code: None };
    let request_id = "request:abort";
    let digest = "11".repeat(32);
    let operation_id = "operation:abort";
    let accepted = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "accepted", None, "accepted");
    directory.append_admin_operation_audit(&accepted).await.expect("accepted audit");
    let runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + std::time::Duration::from_secs(10),
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    operations.insert(operation_id.into(), runtime);
    let cleanup = AdminOperationCleanup { operations: operations.clone(), operation_id: operation_id.into(), _permit: operation_slots.clone().try_acquire_owned().expect("cleanup slot") };
    let task = tokio::spawn(async move {
        let _cleanup = cleanup;
        std::future::pending::<()>().await;
    });
    tokio::task::yield_now().await;
    task.abort();
    let _ = task.await;
    let mut rows = Vec::new();
    for _ in 0..128 {
        rows = directory.admin_operation_audit_for_request(request_id).await.expect("operation audit");
        if operations.get_cloned(operation_id).is_none() {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].fact.phase, "accepted");
    assert_eq!(rows[0].fact.outcome_code, "accepted");
    assert!(operations.get_cloned(operation_id).is_none());
    assert_eq!(operation_slots.available_permits(), 64);
    let later = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "succeeded", None, "late-success");
    assert_eq!(directory.append_admin_operation_audit(&later).await.expect("factual late terminal").fact.phase, "succeeded", "task abortion cannot invent rollback before the factual result is known");
}
