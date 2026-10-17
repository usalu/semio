/// 📒️ Laws of the admin operation audit, bounded overviews, the shared writer guard and event log replay.

// 🌱️ Every test in this module creates a space owned by `user_actor("u-owner")`; `decide`'s
// `CreateSpace` arm (correctly) never mints the owner's `hub_user` row itself — it only has an
// actor id, no email, so it cannot self-heal a missing user the way `UpsertMember` can. In
// production the owner's `hub_user` row must predate `create-space`; the trusted identity
// completion boundary provisions it before issuing a session. This fixture reproduces that precondition by
// appending a bare `user.created` event for `"u-owner"` under a `System` actor, mirroring
// `SqliteDirectory::seed`'s own pattern one file over.
async fn fresh_dir() -> Arc<HubDirectories> {
    let dir = SqliteDirectory::connect(":memory:").await.expect("connect");
    let seed_actor = DirectoryActor { kind: DirectoryActorKind::System, id: "system:test-seed".into() };
    let mut clock = HubClock::new();
    let events = vec![new_event(&mut clock, &seed_actor, None, Some("u-owner".into()), DirectoryEventBody::UserCreated { user_id: "u-owner".into(), email: "u-owner@example.com".into(), display_name: "Owner".into() })];
    dir.append_events(&events).await.expect("seed owner user");
    Arc::new(HubDirectories::from(dir))
}

fn admin_audit_fact(request_id: &str, digest_byte: &str, phase: &str, outcome: &str) -> NewAdminOperationAuditRecord {
    NewAdminOperationAuditRecord {
        request_id: request_id.into(),
        intent_digest: digest_byte.repeat(64),
        operation_id: format!("operation:{request_id}"),
        occurred_at: 1,
        phase: phase.into(),
        intent_kind: "delete-space".into(),
        target_kind: "space".into(),
        target_id: "space:one".into(),
        principal_user_id: "user:admin".into(),
        principal_session_id: "session:admin".into(),
        principal_generation: 7,
        correlation_id: "correlation:admin".into(),
        event_seq_first: None,
        event_seq_last: None,
        outcome_code: outcome.into(),
        reason_code: None,
    }
}

#[tokio::test]
async fn admin_operation_audit_concurrent_first_writer_is_idempotent_and_first_terminal_wins() {
    let directory = fresh_dir().await;
    let accepted = admin_audit_fact("request:race", "1", "accepted", "accepted");
    let barrier = Arc::new(tokio::sync::Barrier::new(33));
    let mut writers = Vec::new();
    for _ in 0..32 {
        let directory = directory.clone();
        let accepted = accepted.clone();
        let barrier = barrier.clone();
        writers.push(tokio::spawn(async move {
            barrier.wait().await;
            directory.append_admin_operation_audit(&accepted).await
        }));
    }
    barrier.wait().await;
    let mut sequences = BTreeSet::new();
    for writer in writers {
        sequences.insert(writer.await.expect("race writer").expect("idempotent append").sequence);
    }
    assert_eq!(sequences.len(), 1);
    assert_eq!(directory.admin_operation_audit_for_request("request:race").await.expect("race audit").len(), 1);
    let collision = admin_audit_fact("request:race", "2", "accepted", "accepted");
    assert!(matches!(directory.append_admin_operation_audit(&collision).await, Err(DirectoryError::Conflict(_))));

    let cancelled = admin_audit_fact("request:race", "1", "cancelled", "operator-cancelled");
    let failed = admin_audit_fact("request:race", "1", "failed", "late-failure");
    assert_eq!(directory.append_admin_operation_audit(&cancelled).await.expect("cancel terminal").fact.phase, "cancelled");
    assert!(matches!(directory.append_admin_operation_audit(&failed).await, Err(DirectoryError::Conflict(_))), "a different later terminal cannot masquerade as the winning cancellation");
    assert_eq!(directory.append_admin_operation_audit(&cancelled).await.expect("idempotent winning terminal").fact.phase, "cancelled");
    let operation_rows = directory.admin_operation_audit_for_operation("operation:request:race").await.expect("operation-id status lookup");
    assert_eq!(operation_rows.len(), 2);
    assert!(operation_rows.iter().all(|row| row.fact.request_id == "request:race"));
    let rows = directory.list_admin_operation_audit(0, ADMIN_PAGE_MAX).await.expect("bounded audit page");
    assert_eq!(rows.len(), 2);
    assert!(directory.list_admin_operation_audit(0, ADMIN_PAGE_MAX + 1).await.is_err());
}

#[tokio::test]
async fn admin_bounded_overview_space_and_document_projections_enforce_exact_page_boundary() {
    let directory = fresh_dir().await;
    let service = DirectoryService::new(directory.clone(), 64);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    for index in 0..=ADMIN_PAGE_MAX {
        service.execute(owner.clone(), DirectoryCommand::AnnounceDocument { descriptor: Box::new(descriptor(&space_id, &format!("document:{index:03}"))) }).await.expect("announce bounded-page document");
    }

    assert_eq!(directory.admin_overview_counts().await.expect("constant-space overview counts"), AdminDirectoryOverviewCounts { spaces: 1, users: 1, connections: 0 },);
    let spaces = directory.list_admin_space_summaries_page(None, 0, ADMIN_PAGE_FETCH_MAX).await.expect("bounded space summary page");
    assert_eq!(spaces.len(), 1);
    assert_eq!(spaces[0].member_count, 1);
    assert_eq!(spaces[0].document_count, u64::try_from(ADMIN_PAGE_FETCH_MAX).expect("page maximum fits u64"));
    assert_eq!(spaces[0].active_connections, 0);
    assert_eq!(directory.list_admin_space_summaries_page(Some(&space_id), 0, 1).await.expect("exact space summary").len(), 1);
    assert_eq!(directory.list_admin_space_members_page(&space_id, 0, ADMIN_PAGE_FETCH_MAX).await.expect("bounded member page").len(), 1);
    assert!(directory.list_admin_space_summaries_page(None, 0, 0).await.is_err());
    assert!(directory.list_admin_space_summaries_page(None, 0, ADMIN_PAGE_FETCH_MAX + 1).await.is_err());
    assert!(directory.list_admin_space_members_page(&space_id, 0, 0).await.is_err());
    assert!(directory.list_admin_space_members_page(&space_id, 0, ADMIN_PAGE_FETCH_MAX + 1).await.is_err());
    let first = directory.list_document_descriptors_page(None, 0, ADMIN_PAGE_FETCH_MAX).await.expect("page plus continuation probe");
    assert_eq!(first.len(), ADMIN_PAGE_FETCH_MAX);
    assert_eq!(first.first().expect("first descriptor").document_id, "document:000");
    assert_eq!(first.last().expect("continuation descriptor").document_id, "document:100");
    let continuation = directory.list_document_descriptors_page(Some(&space_id), ADMIN_PAGE_MAX, ADMIN_PAGE_FETCH_MAX).await.expect("scoped continuation page");
    assert_eq!(continuation.len(), 1);
    assert_eq!(continuation[0].document_id, "document:100");
    assert!(directory.list_document_descriptors_page(None, 0, 0).await.is_err());
    assert!(directory.list_document_descriptors_page(None, 0, ADMIN_PAGE_FETCH_MAX + 1).await.is_err());
}

async fn create_space(service: &DirectoryService, owner: &DirectoryActor, kind: DirectorySpaceKind) -> String {
    let (events, _) = service.execute(owner.clone(), DirectoryCommand::CreateSpace { name: "Space".into(), space_kind: kind, visibility: DirectorySpaceVisibility::Private }).await.expect("create-space");
    events[0].space_id.clone().expect("space id on space.created")
}

/// 📣️ A committed page cannot be overtaken on the live channel while it still owns the writer guard.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn directory_append_and_live_broadcast_share_one_writer_guard_and_projection_order() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📣️ordered-append-broadcast-v1/🔣️.json")).expect("language-neutral ordered publication fixture");
    assert_eq!(fixture["schema"], "semio.hub.directory.ordered-append-broadcast/v1");
    assert_eq!(fixture["cases"].as_array().expect("ordered publication cases").len(), 4);

    let dir = fresh_dir().await;
    let service = Arc::new(DirectoryService::new(dir.clone(), 16));
    let owner = user_actor("u-owner");
    let space_id = create_space(service.as_ref(), &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "ordered@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("seed ordered member");
    let member_id = dir.list_members(&space_id).await.expect("seeded member projection").into_iter().find(|(user, _)| user.email == "ordered@example.com").expect("ordered member").0.id;
    let since = dir.head_seq().await.expect("head before concurrent publication");
    let mut receiver = service.subscribe();
    let fence = service.arm_publication_test_fence();

    let first_service = service.clone();
    let first_owner = owner.clone();
    let first_space = space_id.clone();
    let first =
        tokio::spawn(async move { first_service.execute(first_owner, DirectoryCommand::UpsertMember { space_id: first_space, email: "ordered@example.com".into(), role: DirectorySpaceRole::Author }).await.expect("first concurrent member update").0 });
    tokio::time::timeout(std::time::Duration::from_secs(1), fence.reached.notified()).await.expect("first committed page reached publication fence");

    let second_service = service.clone();
    let second_owner = owner.clone();
    let second_space = space_id.clone();
    let second = tokio::spawn(async move {
        second_service.execute(second_owner, DirectoryCommand::UpsertMember { space_id: second_space, email: "ordered@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("second concurrent member update").0
    });
    assert!(tokio::time::timeout(std::time::Duration::from_millis(50), receiver.recv()).await.is_err(), "a later writer cannot append or broadcast while the committed first page is fenced");
    fence.release.notify_one();

    let first_events = first.await.expect("first writer task");
    let second_events = second.await.expect("second writer task");
    assert_eq!(first_events.len(), 1);
    assert_eq!(second_events.len(), 1);
    assert_eq!([first_events[0].seq, second_events[0].seq], [since + 1, since + 2]);
    let mut observed = Vec::new();
    for _ in 0..2 {
        match tokio::time::timeout(std::time::Duration::from_secs(1), receiver.recv()).await.expect("ordered live event deadline").expect("ordered live event") {
            DirectoryStreamMessage::Event { event } => observed.push(event.seq),
            _ => panic!("directory writer emitted a non-event message"),
        }
    }
    assert_eq!(observed, vec![since + 1, since + 2]);
    assert_eq!(dir.events_since(since, 2).await.expect("durable concurrent event page").iter().map(|event| event.seq).collect::<Vec<_>>(), observed);
    assert_eq!(dir.list_members(&space_id).await.expect("final member projection").into_iter().find(|(user, _)| user.id == member_id).expect("final ordered member").1, SpaceRole::Spectator,);
}

// 🔬️ Replaying the whole log through `rebuild_projections` reproduces the exact same
// projections a live command stream built, and `events_since(0)` is dense `1..=head_seq()`.
#[tokio::test]
async fn event_log_replay_matches_projections() {
    let dir = fresh_dir().await;
    let service = DirectoryService::new(dir.clone(), 64);
    let owner = user_actor("u-owner");
    let space_id = create_space(&service, &owner, DirectorySpaceKind::Studio).await;
    service.execute(owner.clone(), DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "member@example.com".into(), role: DirectorySpaceRole::Spectator }).await.expect("upsert-member");
    service.execute(owner.clone(), DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Renamed".into() }).await.expect("rename-space");

    let head = dir.head_seq().await.expect("head seq");
    let seqs: Vec<u64> = dir.events_since(0, 100).await.expect("events since").iter().map(|event| event.seq).collect();
    assert_eq!(seqs, (1..=head).collect::<Vec<_>>(), "seq is dense 1..n");

    let spaces_before = dir.list_spaces(100, 0).await.expect("list spaces");
    let members_before = dir.list_members(&space_id).await.expect("list members");
    let users_before = dir.list_users(100, 0).await.expect("list users");

    let replayed = dir.rebuild_projections().await.expect("rebuild");
    assert_eq!(replayed, head);
    assert_eq!(dir.list_spaces(100, 0).await.expect("list spaces"), spaces_before);
    assert_eq!(dir.list_members(&space_id).await.expect("list members"), members_before);
    assert_eq!(dir.list_users(100, 0).await.expect("list users"), users_before);
}
