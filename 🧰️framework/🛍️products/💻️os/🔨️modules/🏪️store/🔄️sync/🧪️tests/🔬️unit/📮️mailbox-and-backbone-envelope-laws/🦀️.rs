/// 📮️ Mailbox and backbone envelope laws: the folder event log root refusal, the document backbone envelope helpers, the opening attempt wire owner and the artifact mailbox item cap, byte cap, wake storm, stale send and nested identifier laws.

#[tokio::test]
async fn folder_event_log_refuses_the_repository_root() {
    let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
    std::fs::write(dir.path().join("📋️project.json"), "{}\n").expect("build marker");
    std::fs::write(dir.path().join("package.json"), r#"{"workspaces":[]}"#).expect("workspace declaration");
    let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
    let error = storage.write("doc", "schema", b"pack", b"spr").await.expect_err("repository root");
    assert!(error.to_string().contains("repository root"), "{error}");
    assert!(!dir.path().join(".semio").join("events.semio").exists());
}

fn test_pool() -> Arc<semio_framework_async::WorkerPool> {
    static POOL: std::sync::OnceLock<Arc<semio_framework_async::WorkerPool>> = std::sync::OnceLock::new();
    POOL.get_or_init(|| Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)))).clone()
}

fn document_backbone_envelope(id: &str, document_id: &str) -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: MutationId(id.into()),
        document_id: ArtifactId(document_id.into()),
        actor: ActorId("actor-a".into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: semio_framework_os_spr::ArtifactDiff { schema: semio_framework_os_spr::SchemaId("demo/v1".into()), payload: vec![1] },
        inverse: semio_framework_os_spr::InverseMutation { schema: semio_framework_os_spr::SchemaId("demo/v1".into()), payload: vec![2] },
        timestamp: semio_framework_os_spr::HybridLogicalTimestamp { actor: 3, physical_ms: 9_007_199_254_740_992, logical: 5 },
        transaction: None, verb: None, line: None,
    }
}

fn document_backbone_message(envelopes: &[MutationEnvelope]) -> Vec<u8> {
    BackboneMessage::from_envelopes(envelopes).encode_op().expect("canonical document backbone message")
}

fn document_backbone_event_envelopes(event: &ArtifactEvent) -> Option<Vec<MutationEnvelope>> {
    let ArtifactEvent::DocumentBackbone { message } = event else { return None };
    decode_document_backbone_message_exact(message).ok()
}

#[semio_framework_async_macros::async_test]
async fn document_opening_attempt_wire_preserves_outer_owner_without_widening_actor_messages() {
    let attempt = "11111111-1111-4111-8111-111111111111";
    let request = backbone_worker_wire::BackboneWorkerRequest::Open { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, document_id: "document-a".into(), client_instance_id: Some(attempt.into()), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: Some(true), actor: "actor-a".into() };
    let wire = backbone_worker_wire::encode_request(&request);
    let decoded = backbone_worker_wire::decode_request(&wire).await.expect("decode exact opening owner");
    assert!(matches!(decoded, backbone_worker_wire::BackboneWorkerRequest::Open { client_instance_id: Some(owner), .. } if owner == attempt));

    let response = backbone_worker_wire::BackboneWorkerResponse::Event { document_id: "document-a".into(), client_instance_id: attempt.into(), event: ArtifactEvent::Status(ArtifactSyncStatus::default()) };
    let wire = backbone_worker_wire::encode_response(&response);
    let decoded = backbone_worker_wire::decode_response(&wire).await.expect("decode exact event owner");
    assert!(matches!(decoded, backbone_worker_wire::BackboneWorkerResponse::Event { client_instance_id: owner, .. } if owner == attempt));
}

#[test]
fn artifact_mailbox_item_cap_plus_one_returns_exact_owner_and_preserves_fifo() {
    let (sender, receiver) = artifact_mailbox_pair();
    for seq in 0..ARTIFACT_MAILBOX_ITEMS as u64 {
        sender.send(ArtifactActorMsg::PublishPreview { key: format!("key-{seq}"), seq, payload: vec![seq as u8] }).expect("admit fixed mailbox owner");
    }
    let rejected = sender.send(ArtifactActorMsg::PublishPreview { key: "cap-plus-one".into(), seq: 256, payload: vec![7, 8] }).expect_err("item cap + 1 must reject");
    assert!(matches!(rejected.into_message(), ArtifactActorMsg::PublishPreview { key, seq: 256, payload } if key == "cap-plus-one" && payload == vec![7, 8]));
    for seq in 0..ARTIFACT_MAILBOX_ITEMS as u64 {
        assert!(matches!(receiver.try_recv(), Some(ArtifactActorMsg::PublishPreview { seq: actual, .. }) if actual == seq));
    }
    assert!(receiver.try_recv().is_none());
}

#[test]
fn artifact_mailbox_byte_cap_and_plus_one_preflight_before_mutation() {
    let exact_payload = vec![3; ARTIFACT_MAILBOX_BYTES - 17];
    let (exact_sender, exact_receiver) = artifact_mailbox_pair();
    exact_sender.send(ArtifactActorMsg::PublishPreview { key: String::new(), seq: 1, payload: exact_payload }).expect("exact byte cap admits");
    assert_eq!(exact_sender.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).bytes, ARTIFACT_MAILBOX_BYTES);
    assert!(matches!(exact_receiver.try_recv(), Some(ArtifactActorMsg::PublishPreview { seq: 1, .. })));

    let (overflow_sender, overflow_receiver) = artifact_mailbox_pair();
    let rejected = overflow_sender.send(ArtifactActorMsg::PublishPreview { key: String::new(), seq: 2, payload: vec![9; ARTIFACT_MAILBOX_BYTES - 16] }).expect_err("byte cap + 1 must reject");
    assert!(matches!(rejected.into_message(), ArtifactActorMsg::PublishPreview { seq: 2, payload, .. } if payload.len() == ARTIFACT_MAILBOX_BYTES - 16));
    assert!(overflow_receiver.try_recv().is_none(), "byte rejection cannot mutate FIFO state");
}

#[test]
fn document_backbone_mailbox_and_retention_are_exact_bounded_and_terminal() {
    let envelope = document_backbone_envelope("mutation-a", "document-a");
    let message = document_backbone_message(std::slice::from_ref(&envelope));
    let (sender, receiver) = artifact_mailbox_pair();
    sender.send(ArtifactActorMsg::DocumentBackbone { message: message.clone() }).expect("canonical mutation message is admitted");
    assert!(matches!(receiver.try_recv(), Some(ArtifactActorMsg::DocumentBackbone { message: actual }) if actual == message));
    let ack = BackboneMessage::Ack { op_ids: Vec::new() }.encode_op().expect("ack encodes");
    assert!(matches!(sender.send(ArtifactActorMsg::DocumentBackbone { message: ack }), Err(ArtifactMailboxSendError::Bytes { .. })), "host ingress refuses a Store acknowledgment");
    let member = BackboneMessage::Member { owner: String::new(), slot: "content".into(), child_id: "child-a".into(), envelopes: encode_envelopes(std::slice::from_ref(&envelope)) }.encode_op().expect("member message encodes");
    let rejected = sender.send(ArtifactActorMsg::DocumentBackbone { message: member.clone() }).expect_err("a root document actor cannot flatten a member lane");
    assert!(matches!(rejected.into_message(), ArtifactActorMsg::DocumentBackbone { message: returned } if returned == member), "member refusal returns the exact encoded message to its caller");
    assert!(receiver.try_recv().is_none(), "member refusal leaves root document ingress empty");

    let mut retention = DocumentBackboneRetentionV1::default();
    retention.retain(message.len(), std::slice::from_ref(&envelope)).expect("first exact owner is retained");
    assert_eq!((retention.bytes, retention.messages, retention.entries.len()), (message.len(), 1, 1));
    assert!(retention.retain(message.len(), std::slice::from_ref(&envelope)).is_err(), "duplicate mutation identity cannot replace its owner");
    retention.release(std::slice::from_ref(&envelope));
    assert_eq!((retention.bytes, retention.messages, retention.entries.len()), (0, 0, 0));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn artifact_mailbox_wake_storm_coalesces_until_fifo_becomes_empty() {
    let (sender, receiver) = artifact_mailbox_pair();
    let wakes = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = wakes.clone();
    receiver.set_wake(Arc::new(move || {
        observed.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    }));
    for _ in 0..ARTIFACT_MAILBOX_ITEMS {
        sender.send(ArtifactActorMsg::ExternalChanged).expect("wake-storm owner admitted");
    }
    assert_eq!(wakes.load(std::sync::atomic::Ordering::Acquire), 1);
    while receiver.try_recv().is_some() {}
    sender.send(ArtifactActorMsg::Detach).expect("new readiness edge admitted");
    assert_eq!(wakes.load(std::sync::atomic::Ordering::Acquire), 2);
}

#[test]
fn artifact_mailbox_stale_late_send_hands_back_exact_owner_and_interrupted_close_drains_one_per_grant() {
    let (sender, receiver) = artifact_mailbox_pair();
    for seq in 0..3 {
        sender.send(ArtifactActorMsg::PublishPreview { key: "close".into(), seq, payload: vec![seq as u8] }).expect("close fixture admission");
    }
    let close = receiver.close_handle();
    close.close();
    let late = sender.send(ArtifactActorMsg::PublishPreview { key: "late".into(), seq: 99, payload: vec![4, 5, 6] }).expect_err("late generation rejects");
    assert!(matches!(late, ArtifactMailboxSendError::Stale { message: ArtifactActorMsg::PublishPreview { key, seq: 99, payload } } if key == "late" && payload == vec![4, 5, 6]));
    for remaining in [2, 1, 0] {
        assert!(close.close_one());
        assert_eq!(close.authority.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).len, remaining);
    }
    assert!(!close.close_one());
}

#[semio_framework_async_macros::async_test]
async fn artifact_mailbox_nested_identifier_bytes_and_backbone_one_pop_preserve_ownership_order() {
    use semio_framework_os_store_backbone::Backbone;
    let nested = sample_presence_peer_with_interaction().await;
    let bare = PresencePeer {
        actor: nested.actor.clone(),
        connected_at_ms: nested.connected_at_ms,
        label: None,
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: None,
        color: None,
        surface: None,
        views: Vec::new(),
        ui: None,
        tool_run: None,
        principal_kind: None, active_tool: None, history_edit: None,
        typing: Vec::new(),
    };
    let nested_bytes = artifact_actor_message_bytes(&ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(nested) }).expect("nested message fits");
    let bare_bytes = artifact_actor_message_bytes(&ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(bare) }).expect("bare message fits");
    assert!(nested_bytes > bare_bytes, "all nested identifiers and collections contribute byte credit");

    let (mut channel, remote) = ChannelBackbone::pair("store-sync-one-pop").await;
    channel.send(BackboneMessage::Ack { op_ids: vec!["first".into()] }).await.expect("first backbone owner");
    channel.send(BackboneMessage::Ack { op_ids: vec!["second".into()] }).await.expect("second backbone owner");
    assert!(matches!(remote.try_pop_front().expect("first opportunity"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["first"]));
    assert!(matches!(remote.try_pop_front().expect("second opportunity"), Some(BackboneMessage::Ack { op_ids }) if op_ids == vec!["second"]));
    assert!(remote.try_pop_front().expect("idle opportunity").is_none());
}
