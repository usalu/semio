/// ♻️ Rebootstrap, supersession refusal and presence corpus laws: the rebootstrap that waits for the host reseed, the refused supersession retracted as a typed refusal and the folder archive presence corpus.

/// 🛟️ A hub `RebootstrapRequired` never strands a native document again (LD item 3, WG10 item 6): the actor asks its host for
/// the pair and does not dial into a Welcome it would have to refuse; the host's reseed restarts it at the pair — unacked local
/// work handed back to the guest — and its next Hello names the pair's baseline, so the hub's None/Tail Welcome goes Live.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn a_rebootstrap_waits_for_the_hosts_reseed_and_says_hello_at_its_baseline() {
    let (bootstrap, pair) = demo_artifact_bootstrap(true).await;
    let baseline = bootstrap.baseline_frontier.clone();
    let (_, remote) = ChannelBackbone::pair("native-rebootstrap-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, mut event_rx) = broadcast::channel(32);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "demo".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url: "http://hub.local".into(), space_id: "space-reseed".into(), surface: None }], watch_external: false, actor: "rebootstrap-test".into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    let local = sample_operation_envelope("unacked-local", 5).await;
    actor.queue_test_outbox(vec![local.clone()]);
    let control = semio_framework_os_spr::RebootstrapRequired { space_id: "space-reseed".into(), document_id: "demo".into(), checkpoint_id: [0x22; 32], descriptor_hash: [0x11; 32], baseline_frontier: baseline.clone() };
    actor.inject_hub_frame(ServerFrame::RebootstrapRequired { control }).await;
    let mut asked = None;
    while let Ok(event) = event_rx.try_recv() {
        if let ArtifactEvent::RebootstrapRequired { control } = event {
            asked = Some(control);
        }
    }
    let asked = asked.expect("the actor asks its host for the pair instead of waiting for it inside a Welcome");
    assert_eq!((asked.scope.document_id.as_str(), asked.checkpoint_id.0, asked.baseline_frontier.head_edit_ordinal), ("demo", [0x22; 32], baseline.head_edit_ordinal));
    assert!(actor.bootstrap_test_state().8, "the actor waits for its host's reseed");
    let baseline_frontier = semio_framework_os_directory::ArtifactFrontier { document_id: "demo".into(), head_edit_ordinal: baseline.head_edit_ordinal, head_edit_id: baseline.head_edit_id.clone(), last_commit_seq: baseline.last_commit_seq, chain_hash: semio_framework_os_directory::ArtifactHash(baseline.chain_hash) };
    let _ = actor.handle_test_cmd(ArtifactActorMsg::Reseed { pack: pair.pack.clone(), spr: pair.spr.clone(), baseline: baseline_frontier }).await;
    let mut replayed = Vec::new();
    while let Ok(event) = event_rx.try_recv() {
        if let ArtifactEvent::RemoteMutations { envelopes } = event {
            replayed.extend(envelopes.into_iter().map(|envelope| envelope.mutation_id.0));
        }
    }
    assert_eq!(replayed, vec![local.mutation_id.0.clone()], "the unacked local operation goes back to the re-seeded guest");
    let (pack, spr, frontier, _, resume, _, _, outbox, rebootstrap) = actor.bootstrap_test_state();
    assert_eq!((pack.as_deref(), spr.as_deref(), frontier, resume, rebootstrap), (Some(pair.pack.as_slice()), Some(pair.spr.as_slice()), Some(baseline.clone()), None, false));
    assert_eq!(outbox, vec![local.mutation_id.0.clone()], "and stays queued for the hub");
    let mut socket = connect(&mut actor, "hub.v1.eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").await;
    let ClientFrame::SocketHelloV1 { frontier, .. } = receive_frame(&mut socket).await else { panic!("the first frame is the hello") };
    assert_eq!(frontier, Some(baseline.clone()), "the rebuilt socket says Hello at the pair's baseline");
    actor.inject_hub_frame(ServerFrame::Welcome { session_id: "session-reseed".into(), resume_token: "resume-reseed".into(), server_frontier: baseline.clone(), bootstrap: Bootstrap::None }).await;
    assert!(matches!(actor.bootstrap_test_state().6, RemoteState::Live { .. }), "the hub's None Welcome is accepted like a first open");
}

/// 🛟️ A hub-refused `Supersede` has no inverse (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING): the refused batch's
/// operation rolls back by its inverse, the transition is retracted — the store receives `BackboneMessage::Retract`
/// naming it, on its channel and through the document backbone alike — and the actor raises the typed
/// [`HISTORY_TRANSITION_REFUSED_CODE`] refusal naming it. No rebuild follows — the hub forces none after a refusal — and
/// the refused batch is never requeued or resent: the socket stays live and the author converges by the retraction.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn a_refused_supersession_is_retracted_as_a_typed_refusal() {
    use semio_framework_os_store_backbone::Backbone;
    let (bootstrap, _) = demo_artifact_bootstrap(true).await;
    let baseline = bootstrap.baseline_frontier.clone();
    let (mut backbone, remote) = ChannelBackbone::pair("native-refused-supersede-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, mut event_rx) = broadcast::channel(32);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "demo".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url: "http://hub.local".into(), space_id: "space-supersede".into(), surface: None }], watch_external: false, actor: "supersede-test".into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    let operation = sample_operation_envelope("supersede-edit", 5).await;
    let supersede = semio_framework_os_spr::TransitionSupersede { scope: None, inputs: vec![semio_framework_os_spr::SupersededInput { target: MutationId("ghost".into()), replacement: semio_framework_os_spr::InputReplacement::Withdrawn }] };
    let dependencies = supersede.targets();
    let transition = semio_framework_os_spr::history_transition_envelope(&semio_framework_os_spr::HistoryTransition::Supersede(supersede), &ArtifactId("demo".into()), &ActorId("supersede-test".into()), dependencies, semio_framework_os_spr::HybridLogicalTimestamp::new(1, 9));
    actor.queue_test_outbox(vec![operation.clone(), transition.clone()]);
    let receipt = "hub.v1.ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let mut socket = connect(&mut actor, receipt).await;
    assert!(matches!(receive_frame(&mut socket).await, ClientFrame::SocketHelloV1 { .. }));
    actor.inject_hub_frame(ServerFrame::Session { actor: receipt.into(), color: 2 }).await;
    let ClientFrame::Commands { batch_id, envelopes } = receive_frame(&mut socket).await else { panic!("the Session flushes the queued batch") };
    assert_eq!(envelopes.iter().map(|envelope| envelope.mutation_id.clone()).collect::<Vec<_>>(), vec![operation.mutation_id.clone(), transition.mutation_id.clone()]);
    while event_rx.try_recv().is_ok() {}

    actor.inject_hub_frame(ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: "history.unknown-target".into(), messages: Vec::new() }) }], frontier: baseline.clone() }).await;
    let (mut rolled_back, mut retracted, mut outcome, mut refusal) = (Vec::new(), Vec::new(), None, None);
    while let Ok(event) = event_rx.try_recv() {
        match event {
            ArtifactEvent::DocumentBackbone { message } => match semio_framework_os_store_backbone::decode_hot_backbone_message_exact(&message).expect("canonical backbone message") {
                BackboneMessage::Mutations { envelopes } => rolled_back.extend(decode_envelopes(&envelopes).expect("rollback batch").into_iter().map(|envelope| envelope.mutation_id.0)),
                BackboneMessage::Retract { mutation_ids } => retracted.extend(mutation_ids),
                other => panic!("unexpected document backbone message {other:?}"),
            },
            ArtifactEvent::CommandOutcome { outcome: ack, .. } => outcome = Some(ack),
            ArtifactEvent::Conflict(message) => refusal = Some(message),
            _ => {}
        }
    }
    assert_eq!(rolled_back, vec![format!("{}~undo", operation.mutation_id.0)], "only the operation rolls back; a transition has no inverse");
    assert_eq!(retracted, vec![transition.mutation_id.0.clone()], "the transition is retracted instead");
    let store_bound: Vec<BackboneMessage> = backbone.receive().await.expect("the store end receives");
    assert!(store_bound.contains(&BackboneMessage::Retract { mutation_ids: vec![transition.mutation_id.0.clone()] }), "the store's own channel carries the retraction: {store_bound:?}");
    assert!(matches!(outcome, Some(CommandAckOutcome::Rejected { .. })), "the batch's own outcome is still reported: {outcome:?}");
    let refusal = refusal.expect("a refused transition is a typed refusal the runtime can show");
    assert_eq!((refusal.code.0.as_str(), refusal.level, refusal.target.clone()), (HISTORY_TRANSITION_REFUSED_CODE, semio_framework_diagnostic::Severity::Error, vec![transition.mutation_id.0.clone()]));

    assert!(!std::iter::from_fn(|| event_rx.try_recv().ok()).any(|event| matches!(event, ArtifactEvent::RebootstrapRequired { .. })), "a refusal asks for no rebuild");
    let (_, _, _, _, _, _, _, outbox, rebootstrap) = actor.bootstrap_test_state();
    assert_eq!((outbox, rebootstrap), (Vec::<String>::new(), false), "the refused batch is never requeued and nothing rebuilds");
}

/// 🫙️ Every case of the folder-archive-presence corpus an append-only folder log can stage (its folder never returns to
/// absent after holding an archive), through the native actor over a real folder: the bootstrap read of an empty folder
/// announces `DocumentArchiveAbsent` exactly once, no later read does, and an archive whose bytes the folder is known to hold
/// is not written again. The TypeScript worker runs the whole corpus (`💻️os/🧪️tests/🧪️folder-archive-restore`).
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn the_folder_archive_presence_corpus_matches_the_native_actor() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/🧫️folder-archive-presence/🔣️.json")).expect("the corpus parses");
    let (document, schema) = ("presence-1", "s.test.root/1/*");
    let mut archives: std::collections::HashMap<&str, Vec<u8>> = std::collections::HashMap::new();
    for name in ["a", "b"] {
        let archive = semio_framework_os_spr::DocumentArchivePack { parent_pack: name.as_bytes().to_vec(), parent_spr: semio_framework_os_store_text_io::text_format::empty_document_spr(document, schema).await, members: Vec::new() };
        archives.insert(name, semio_framework_os_spr::encode_document_archive_bytes(&archive).expect("encode the archive"));
    }
    let content = |value: &serde_json::Value| value.as_str().filter(|content| *content != "absent").map(|content| archives[content].clone());
    let mut staged = 0;
    for case in corpus["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("a case name");
        let steps = case["steps"].as_array().expect("steps");
        let mut holds = content(&case["bootstrap"]).is_some();
        let mut stageable = true;
        for step in steps {
            match step.get("read") {
                Some(read) if content(read).is_none() && holds => stageable = false,
                Some(read) => holds = content(read).is_some(),
                None => holds = true,
            }
        }
        if !stageable {
            continue;
        }
        let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
        let other_writer = FolderEventLogStorage::new(dir.path().to_path_buf());
        if let Some(archive) = content(&case["bootstrap"]) {
            other_writer.write_archive(document, schema, &archive).await.expect("the folder holds an archive at the open");
        }
        let (_, remote) = ChannelBackbone::pair(name).await;
        let (_, receiver) = artifact_mailbox_pair();
        let (events, mut observed) = broadcast::channel(64);
        let mut actor = native_actor::ArtifactActor::new(
            test_pool(),
            ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: document.into(), schema: schema.into(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: false, actor: "local".into() },
            remote,
            receiver,
            events,
            Arc::new(std::sync::RwLock::new(None)),
            Arc::new(std::sync::RwLock::new(None)),
            None,
            semio_framework_async::CancelToken::root_now(),
        )
        .await;
        actor.setup_test().await;
        for step in steps {
            match (step.get("read"), step.get("write")) {
                (Some(read), _) => {
                    if let Some(wanted) = content(read) {
                        if other_writer.read_archive(document).await.expect("the folder reads").as_ref() != Some(&wanted) {
                            other_writer.write_archive(document, schema, &wanted).await.expect("another writer's archive");
                        }
                    }
                    actor.handle_test_cmd(ArtifactActorMsg::ExternalChanged).await;
                }
                (None, Some(write)) => actor.handle_test_document_archive(content(write).expect("a write names an archive")).await,
                (None, None) => panic!("{name}: a step reads or writes"),
            }
        }
        let mut absent = 0;
        while let Ok(event) = observed.try_recv() {
            if matches!(event, ArtifactEvent::DocumentArchiveAbsent) {
                absent += 1;
            }
        }
        assert_eq!((absent, actor.folder_writes_test() as u64), (case["expected"]["absent"].as_u64().expect("absent"), case["expected"]["puts"].as_u64().expect("puts")), "{name}");
        staged += 1;
    }
    assert_eq!(staged, 7, "every case that never empties a held folder is staged");
}
