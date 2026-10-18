/// 🤝️ Hub hello, bootstrap and receive laws: the bootstrap frontier identity, the hub actor hello at the canonical pair baseline, the terminal connection failure, the concurrent supersessions, the native bootstrap and the received envelope materialization.

#[test]
fn bootstrap_frontier_identity_rejects_same_ordinals_with_wrong_authenticated_head() {
    let required = bootstrap_frontier("demo", 9, "edit-9", 4, 0x44);
    let mut wrong_head = required.clone();
    wrong_head.head_edit_id = "edit-other".into();
    let mut wrong_chain = required.clone();
    wrong_chain.chain_hash = [0x55; 32];
    assert!(!frontier_reaches(&wrong_head, &required));
    assert!(!frontier_reaches(&wrong_chain, &required));
    assert!(frontier_reaches(&required, &required));
}

/// 🔌️ Dials `actor` into a loopback hub socket and answers the hub's end of it.
#[cfg(not(target_arch = "wasm32"))]
async fn connect(actor: &mut native_actor::ArtifactActor, receipt_actor: &str) -> tokio_tungstenite::WebSocketStream<tokio::net::TcpStream> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind test socket");
    let url = format!("ws://{}", listener.local_addr().expect("test socket address"));
    let accepted = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("accept test socket");
        tokio_tungstenite::accept_async(stream).await.expect("upgrade test socket")
    });
    actor.connect_test_socket(&url, receipt_actor).await;
    accepted.await.expect("test socket task")
}

/// 📨️ The next client frame the hub's end of a loopback socket reads.
#[cfg(not(target_arch = "wasm32"))]
async fn receive_frame(socket: &mut tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) -> ClientFrame {
    use futures::StreamExt;
    let message = tokio::time::timeout(std::time::Duration::from_secs(1), socket.next()).await.expect("client frame deadline").expect("client frame owner").expect("client frame");
    let tokio_tungstenite::tungstenite::Message::Binary(bytes) = message else { panic!("expected binary client frame") };
    semio_framework_os_spr::decode_client_frame(&bytes).await.expect("decode client frame").1
}

/// 🪢️ A hub document's actor seeded with the canonical checkpoint pair its shell verified starts AT the pair: the pair
/// and its history are the actor's, no transfer is in flight, and its first `SocketHelloV1` names the pair's baseline,
/// so the hub tails only what followed it — the one seed of every shell (ticket 26/09/23 slice WG10).
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn a_seeded_hub_actor_says_hello_at_the_canonical_pair_baseline() {
    let (bootstrap, pair) = demo_artifact_bootstrap(true).await;
    let baseline = bootstrap.baseline_frontier.clone();
    let (_, remote) = ChannelBackbone::pair("native-seed-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(8);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "seed-test".into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    let blob = |bytes: &[u8]| semio_framework_os_directory::PublishedArtifactBlob { sha256: semio_framework_os_directory::ArtifactHash(semio_framework_hash::Sha256::digest(bytes)), byte_length: bytes.len() as u64 };
    actor
        .seed_hub_document(semio_framework_os_directory::CanonicalCheckpointPairV1 {
            scope: semio_framework_os_directory::DocumentScope::new("space-seed", "demo"),
            descriptor_digest_v1: semio_framework_os_directory::ArtifactHash([0x11; 32]),
            active_checkpoint_id: semio_framework_os_directory::ArtifactHash([0x22; 32]),
            baseline_frontier: semio_framework_os_directory::ArtifactFrontier { document_id: "demo".into(), head_edit_ordinal: baseline.head_edit_ordinal, head_edit_id: baseline.head_edit_id.clone(), last_commit_seq: baseline.last_commit_seq, chain_hash: semio_framework_os_directory::ArtifactHash(baseline.chain_hash) },
            pack: blob(&pair.pack),
            spr: blob(&pair.spr),
            aggregate_sha256: semio_framework_os_directory::ArtifactHash(semio_framework_os_spr::artifact_bootstrap_aggregate_hash(&pair.pack, &pair.spr)),
            pack_bytes: pair.pack.clone(),
            spr_bytes: pair.spr.clone(),
        })
        .await;
    let (pack, spr, frontier, pending_required, resume, pending_resume, remote_state, outbox, _) = actor.bootstrap_test_state();
    assert_eq!((pack.as_deref(), spr.as_deref()), (Some(pair.pack.as_slice()), Some(pair.spr.as_slice())));
    assert_eq!(frontier, Some(baseline.clone()), "the actor's hub frontier is the pair's baseline");
    assert_eq!((pending_required, resume, pending_resume, outbox.len()), (None, None, None, 0), "a seed is no transfer in flight");
    assert!(!matches!(remote_state, RemoteState::Live { .. }));
    let mut socket = connect(&mut actor, "hub.v1.dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd").await;
    let ClientFrame::SocketHelloV1 { frontier, resume_token, .. } = receive_frame(&mut socket).await else { panic!("the first frame is the hello") };
    assert_eq!((frontier, resume_token), (Some(baseline), None), "the hello asks the hub only for what followed the pair");
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn native_terminal_connection_failure_clears_receipt_actor_before_reissue() {
    use futures::StreamExt;

    let (_, remote) = ChannelBackbone::pair("native-actor-epoch-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(8);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "local-only".into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    let stale = "hub.v1.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    let fresh = "hub.v1.bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    actor.install_test_socket_actor(stale);
    actor.inject_hub_frame(ServerFrame::Session { actor: stale.into(), color: 3 }).await;
    assert_eq!(actor.socket_epoch_test_state(), (Some(stale.into()), true, 0, Vec::new()));
    actor.fail_test_bootstrap().await;
    assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, Vec::new()));
    let first = sample_operation_envelope("after-bootstrap-failure", 1).await;
    let first_actor = first.actor.0.as_str().to_owned();
    actor.relay_test_envelope(first).await;
    assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, vec![first_actor.clone()]));

    actor.install_test_socket_actor(stale);
    actor.inject_hub_frame(ServerFrame::Session { actor: stale.into(), color: 4 }).await;
    actor.fail_test_connection().await;
    assert_eq!(actor.socket_epoch_test_state().0, None);
    assert!(!actor.socket_epoch_test_state().1);
    let second = sample_operation_envelope("after-eof", 2).await;
    let second_actor = second.actor.0.as_str().to_owned();
    actor.relay_test_envelope(second).await;
    assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, vec![first_actor.clone(), second_actor.clone()]));

    let mut socket = connect(&mut actor, fresh).await;
    assert!(matches!(receive_frame(&mut socket).await, ClientFrame::SocketHelloV1 { .. }));
    let before_session = sample_operation_envelope("before-fresh-session", 3).await;
    let before_session_actor = before_session.actor.0.as_str().to_owned();
    actor.relay_test_envelope(before_session).await;
    assert_eq!(actor.socket_epoch_test_state(), (Some(fresh.into()), false, 0, vec![first_actor, second_actor, before_session_actor]));
    assert!(tokio::time::timeout(std::time::Duration::from_millis(30), socket.next()).await.is_err(), "queued mutations cannot cross the socket before Session confirms the receipt actor");
    actor.inject_hub_frame(ServerFrame::Session { actor: fresh.into(), color: 5 }).await;
    let first_batch = receive_frame(&mut socket).await;
    let ClientFrame::Commands { batch_id, envelopes } = first_batch else { panic!("fresh Session must flush one command batch") };
    assert_eq!(envelopes.len(), 3);
    assert!(envelopes.iter().all(|envelope| envelope.actor.0.as_str() == fresh));
    assert!(tokio::time::timeout(std::time::Duration::from_millis(30), socket.next()).await.is_err(), "each queued mutation is sent exactly once after Session");
    assert_eq!(actor.socket_epoch_test_state(), (Some(fresh.into()), true, 1, Vec::new()));
    actor.inject_hub_frame(ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: bootstrap_frontier("demo", 3, "after-session", 3, 0x66) }).await;
    assert_eq!(actor.socket_epoch_test_state(), (Some(fresh.into()), true, 0, Vec::new()));

    actor.fail_test_connection().await;
    let reconnected = "hub.v1.cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
    let queued_during_reconnect = sample_operation_envelope("during-reconnect", 4).await;
    actor.relay_test_envelope(queued_during_reconnect).await;
    let mut reconnected_socket = connect(&mut actor, reconnected).await;
    assert!(matches!(receive_frame(&mut reconnected_socket).await, ClientFrame::SocketHelloV1 { .. }));
    assert!(tokio::time::timeout(std::time::Duration::from_millis(30), reconnected_socket.next()).await.is_err(), "reconnect cannot flush before its own Session");
    actor.inject_hub_frame(ServerFrame::Session { actor: reconnected.into(), color: 6 }).await;
    let ClientFrame::Commands { envelopes, .. } = receive_frame(&mut reconnected_socket).await else { panic!("reconnect Session must flush queued mutation") };
    assert_eq!(envelopes.len(), 1);
    assert_eq!(envelopes[0].actor.0.as_str(), reconnected);
    assert!(tokio::time::timeout(std::time::Duration::from_millis(30), reconnected_socket.next()).await.is_err(), "reconnect flush is exactly once");
    assert_eq!(actor.socket_epoch_test_state(), (Some(reconnected.into()), true, 1, Vec::new()));
}

/// 🛰️ A native document actor dialled into its own loopback hub socket as `socket_actor`, the receipt confirmed.
#[cfg(not(target_arch = "wasm32"))]
async fn confirmed_hub_actor(name: &str, socket_actor: &str) -> (native_actor::ArtifactActor, tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
    let (_, remote) = ChannelBackbone::pair(name).await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(8);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "authored-stamps".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: name.into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    let mut socket = connect(&mut actor, socket_actor).await;
    assert!(matches!(receive_frame(&mut socket).await, ClientFrame::SocketHelloV1 { .. }));
    actor.inject_hub_frame(ServerFrame::Session { actor: socket_actor.into(), color: 1 }).await;
    (actor, socket)
}

/// ⏰️ Author stamps are canonical: an authored envelope crosses the hub socket with its own `(hlc, id)` — only the
/// actor is bound to the socket subject — so two replicas whose concurrent supersessions of one operation reach the hub
/// in the opposite order of their authoring fold the same winner, the last by authored `(hlc, id)`. A send-time clock
/// would let each author keep its own supersession (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, audit F-M5).
#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn concurrent_supersessions_of_one_operation_converge_through_the_hub_socket() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
    const LEFT: &str = "hub.v1.1111111111111111111111111111111111111111111111111111111111111111";
    const RIGHT: &str = "hub.v1.2222222222222222222222222222222222222222222222222222222222222222";
    let genesis = || create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "authored-stamps", DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let mut left = semio_framework_os_store_test_support::plain_test_store(genesis()).await;
    let mut right = semio_framework_os_store_test_support::plain_test_store(genesis()).await;
    let mut shared = sample_operation_envelope("shared-edit", 1).await;
    shared.document_id = ArtifactId("authored-stamps".into());
    shared.actor = ActorId("peer".into());
    shared.timestamp = semio_framework_os_spr::HybridLogicalTimestamp { actor: 5, physical_ms: 1 << 50, logical: 0 };
    for store in [&mut left, &mut right] {
        store.ingest_remote(shared.clone()).await.expect("both replicas ingest the shared edit");
    }
    let target = shared.mutation_id.clone();
    let supersede = |n: i32| ArtifactCommand::Supersede { scope: None, inputs: vec![semio_framework_os_store_space_schema::SupersedeInput { target: target.clone(), replacement: Some(DemoMutation::SetN { n }) }] };
    left.dispatch(supersede(7), &mut identity).await.expect("left supersedes");
    right.dispatch(supersede(9), &mut identity).await.expect("right supersedes concurrently");
    let authored = |store: &semio_framework_os_store_engine::lifted_store::ArtifactStore<DemoSnapshot, DemoMutation>| store.envelope().transitions.last().cloned().expect("an authored supersession");
    let (left_authored, right_authored) = (authored(&left), authored(&right));
    let (winner, value) = if (left_authored.timestamp.cmp_key(), &left_authored.mutation_id.0) > (right_authored.timestamp.cmp_key(), &right_authored.mutation_id.0) { (&left_authored, 7) } else { (&right_authored, 9) };
    let (mut right_actor, mut right_socket) = confirmed_hub_actor("right", RIGHT).await;
    right_actor.relay_test_envelope(right_authored.clone()).await;
    let ClientFrame::Commands { envelopes: right_wire, .. } = receive_frame(&mut right_socket).await else { panic!("the right replica relays one command batch") };
    let (mut left_actor, mut left_socket) = confirmed_hub_actor("left", LEFT).await;
    left_actor.relay_test_envelope(left_authored.clone()).await;
    let ClientFrame::Commands { envelopes: left_wire, .. } = receive_frame(&mut left_socket).await else { panic!("the left replica relays one command batch") };
    for (wire, authored, socket_actor) in [(&left_wire[0], &left_authored, LEFT), (&right_wire[0], &right_authored, RIGHT)] {
        assert_eq!((&wire.mutation_id, wire.timestamp), (&authored.mutation_id, authored.timestamp), "the wire carries the authored identity and clock");
        assert_eq!(wire.actor.0.as_str(), socket_actor, "only the actor is bound to the socket subject");
    }
    left.ingest_remote(right_wire[0].clone()).await.expect("left ingests the right supersession from the hub");
    right.ingest_remote(left_wire[0].clone()).await.expect("right ingests the left supersession from the hub");
    for store in [&left, &right] {
        assert_eq!(store.supersessions()[&target].transition_id, winner.mutation_id.0, "every replica folds the same last supersession");
        assert_eq!(store.snapshot().expect("snapshot"), DemoSnapshot { n: value });
    }
    semio_framework_os_store_test_support::close_plain_test_store(&mut left);
    semio_framework_os_store_test_support::close_plain_test_store(&mut right);
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn native_bootstrap_commits_pair_before_failed_local_replay_then_restarts_without_duplicate() {
    use semio_framework_os_store_backbone::Backbone;
    let (bootstrap, pair) = demo_artifact_bootstrap(true).await;
    let required = bootstrap.required_tail_frontier.clone();
    let baseline = bootstrap.baseline_frontier.clone();
    let (mut channel, remote) = ChannelBackbone::pair("native-bootstrap-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, mut event_rx) = broadcast::channel(32);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "actor-bootstrap-test".into() },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(None)),
        Arc::new(std::sync::RwLock::new(None)),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    let local = sample_operation_envelope("pending-local", 8).await;
    actor.queue_test_outbox(vec![local.clone(), local.clone()]);
    actor.inject_bootstrap_local_replay_failure();
    let welcome =
        |bootstrap: ArtifactBootstrap| ServerFrame::Welcome { session_id: "session-bootstrap".into(), resume_token: "resume-bootstrap".into(), server_frontier: required.clone(), bootstrap: Bootstrap::ArtifactBootstrap(Box::new(bootstrap)) };

    actor.inject_hub_frame(welcome(bootstrap.clone())).await;
    let first = channel.receive().await.expect("baseline queue");
    assert_eq!(first.len(), 1, "failed replay queues only the committed baseline");
    assert!(matches!(&first[0], BackboneMessage::Genesis { pack } if pack == &pair.pack), "the baseline is the hub document's genesis followed by its (here: no) events");
    let (pack, spr, frontier, pending_required, resume, pending_resume, remote_state, outbox, _rebootstrap) = actor.bootstrap_test_state();
    assert_eq!(pack.as_deref(), Some(pair.pack.as_slice()));
    assert_eq!(spr.as_deref(), Some(pair.spr.as_slice()));
    assert_eq!(frontier, Some(baseline.clone()));
    assert_eq!(pending_required, Some(required.clone()));
    assert_eq!(resume, None);
    assert_eq!(pending_resume.as_deref(), Some("resume-bootstrap"));
    assert!(!matches!(remote_state, RemoteState::Live { .. }));
    assert_eq!(outbox, vec![local.mutation_id.0.clone()], "failure preserves one deduplicated local owner");
    assert!(matches!(event_rx.try_recv(), Ok(ArtifactEvent::BootstrapProgress { .. })));

    actor.inject_hub_frame(ServerFrame::Presence { peers: Vec::new() }).await;
    assert!(!matches!(actor.bootstrap_test_state().6, RemoteState::Live { .. }), "presence cannot bypass authenticated catch-up");
    actor.inject_hub_frame(welcome(bootstrap)).await;
    let restarted = channel.receive().await.expect("restart queue");
    assert_eq!(restarted.iter().filter(|message| matches!(message, BackboneMessage::Genesis { .. })).count(), 1);
    let replayed: Vec<MutationEnvelope> = restarted
        .iter()
        .filter_map(|message| match message {
            BackboneMessage::Mutations { envelopes } => Some(decode_envelopes(envelopes).expect("decode replay")),
            _ => None,
        })
        .flatten()
        .collect();
    assert_eq!(replayed.iter().map(|envelope| &envelope.mutation_id).collect::<Vec<_>>(), vec![&local.mutation_id], "restart performs one successful local replay");

    let mut wrong = required.clone();
    wrong.head_edit_id = "edit-wrong".into();
    wrong.chain_hash = [0x55; 32];
    actor.inject_hub_frame(ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-bootstrap-test".into()), frontier: wrong }).await;
    assert!(!matches!(actor.bootstrap_test_state().6, RemoteState::Live { .. }), "same ordinals cannot authenticate a different chain");
    actor.inject_hub_frame(ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-bootstrap-test".into()), frontier: required.clone() }).await;
    let (_, _, frontier, pending_required, resume, _, remote_state, outbox, _) = actor.bootstrap_test_state();
    assert_eq!(frontier, Some(required));
    assert_eq!(pending_required, None);
    assert_eq!(resume.as_deref(), Some("resume-bootstrap"));
    assert!(matches!(remote_state, RemoteState::Live { .. }));
    assert_eq!(outbox, vec![local.mutation_id.0], "offline hub retains the exact local owner after semantic replay");
    assert!(channel.receive().await.expect("no duplicate store replay").is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn native_inline_and_chunked_bootstrap_install_the_same_typed_pair_after_cancelled_restart() {
    use semio_framework_os_store_backbone::Backbone;
    async fn actor_pair(uri: &str) -> (native_actor::ArtifactActor, ChannelBackbone) {
        let (channel, remote) = ChannelBackbone::pair(uri).await;
        let (_, receiver) = artifact_mailbox_pair();
        let (events, _) = broadcast::channel(32);
        let actor = native_actor::ArtifactActor::new(
            test_pool(),
            ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "actor-bootstrap-test".into() },
            remote,
            receiver,
            events,
            Arc::new(std::sync::RwLock::new(None)),
            Arc::new(std::sync::RwLock::new(None)),
            None,
            semio_framework_async::CancelToken::root_now(),
        )
        .await;
        (actor, channel)
    }
    let (inline_bootstrap, pair) = demo_artifact_bootstrap(true).await;
    let (mut chunked_bootstrap, _) = demo_artifact_bootstrap(false).await;
    let required = inline_bootstrap.required_tail_frontier.clone();
    let welcome =
        |bootstrap: ArtifactBootstrap| ServerFrame::Welcome { session_id: "session-bootstrap".into(), resume_token: "resume-bootstrap".into(), server_frontier: required.clone(), bootstrap: Bootstrap::ArtifactBootstrap(Box::new(bootstrap)) };

    let (mut inline_actor, mut inline_channel) = actor_pair("native-bootstrap-inline").await;
    inline_actor.inject_hub_frame(welcome(inline_bootstrap)).await;
    let inline_messages = inline_channel.receive().await.expect("inline messages");
    assert!(matches!(&inline_messages[..], [BackboneMessage::Genesis { pack }] if pack == &pair.pack));

    let mut combined = pair.pack.clone();
    combined.extend_from_slice(&pair.spr);
    let chunk_size = combined.len().div_ceil(3);
    let chunks: Vec<Vec<u8>> = combined.chunks(chunk_size).map(<[u8]>::to_vec).collect();
    assert_eq!(chunks.len(), 3);
    chunked_bootstrap.chunk_count = chunks.len() as u32;
    let descriptor_hash = chunked_bootstrap.descriptor_hash;
    let (mut chunked_actor, mut chunked_channel) = actor_pair("native-bootstrap-chunked").await;
    chunked_actor.inject_hub_frame(welcome(chunked_bootstrap.clone())).await;
    chunked_actor.inject_hub_frame(ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index: 0, bytes: semio_framework_os_spr::ArtifactBootstrapChunkBytes::try_from_slice(&chunks[0]).expect("bounded chunk") }).await;
    chunked_actor.cancel_test_bootstrap();
    assert!(chunked_channel.receive().await.expect("cancelled staging").is_empty(), "cancellation commits no partial pair");

    chunked_actor.inject_hub_frame(welcome(chunked_bootstrap)).await;
    for (index, chunk) in chunks.iter().enumerate() {
        chunked_actor.inject_hub_frame(ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index: index as u32, bytes: semio_framework_os_spr::ArtifactBootstrapChunkBytes::try_from_slice(chunk).expect("bounded chunk") }).await;
        if index + 1 < chunks.len() {
            assert!(chunked_channel.receive().await.expect("staged chunk").is_empty(), "chunks stay invisible before done");
        }
    }
    chunked_actor.inject_hub_frame(ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count: chunks.len() as u32 }).await;
    let chunked_messages = chunked_channel.receive().await.expect("chunked messages");
    assert_eq!(chunked_messages, inline_messages, "inline and chunked replace with byte-identical typed pairs");
    chunked_actor.inject_hub_frame(ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-bootstrap-test".into()), frontier: required.clone() }).await;
    let (actual_pack, actual_spr, frontier, pending_required, resume, _, remote_state, _, _) = chunked_actor.bootstrap_test_state();
    assert_eq!(actual_pack, Some(pair.pack));
    assert_eq!(actual_spr, Some(pair.spr));
    assert_eq!(frontier, Some(required));
    assert_eq!(pending_required, None);
    assert_eq!(resume.as_deref(), Some("resume-bootstrap"));
    assert!(matches!(remote_state, RemoteState::Live { .. }));
}

/// 🚏️ Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice C1 — the document socket's bootstrap
/// ORDER as a law, not just its frame shapes. A hub that relayed a command tail before the snapshot it
/// rebases on would hand the client envelopes against a document it has never seen, so `on_hub_frame`
/// refuses a `Commands` frame while an artifact bootstrap is still in flight: nothing is installed, the
/// refusal is emitted as an `artifactBootstrap` conflict, and the connection drops to reconnect rather
/// than diverging in silence. The same frame after `ArtifactBootstrapDone` is accepted, which is what
/// makes this an ordering law and not a blanket rejection.
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn bootstrap_sequence_refuses_a_command_tail_that_arrives_before_the_snapshot() {
    use semio_framework_os_store_backbone::Backbone;
    async fn ordered_actor(uri: &str) -> (native_actor::ArtifactActor, ChannelBackbone, broadcast::Receiver<ArtifactEvent>) {
        let (channel, remote) = ChannelBackbone::pair(uri).await;
        let (_, receiver) = artifact_mailbox_pair();
        let (events, event_receiver) = broadcast::channel(32);
        let actor = native_actor::ArtifactActor::new(
            test_pool(),
            ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "demo".into(), schema: "demo/v1".into(), bindings: Vec::new(), watch_external: false, actor: "actor-order-test".into() },
            remote,
            receiver,
            events,
            Arc::new(std::sync::RwLock::new(None)),
            Arc::new(std::sync::RwLock::new(None)),
            None,
            semio_framework_async::CancelToken::root_now(),
        )
        .await;
        (actor, channel, event_receiver)
    }

    let (_, pair) = demo_artifact_bootstrap(true).await;
    let (mut bootstrap, _) = demo_artifact_bootstrap(false).await;
    let required = bootstrap.required_tail_frontier.clone();
    let mut combined = pair.pack.clone();
    combined.extend_from_slice(&pair.spr);
    let chunk_size = combined.len().div_ceil(3);
    let chunks: Vec<Vec<u8>> = combined.chunks(chunk_size).map(<[u8]>::to_vec).collect();
    bootstrap.chunk_count = chunks.len() as u32;
    let descriptor_hash = bootstrap.descriptor_hash;
    let welcome = ServerFrame::Welcome {
        session_id: "session-order".into(),
        resume_token: "resume-order".into(),
        server_frontier: required.clone(),
        bootstrap: Bootstrap::ArtifactBootstrap(Box::new(bootstrap.clone())),
    };
    let tail = || ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor-order-test".into()), frontier: required.clone() };

    let (mut early, _early_channel, mut early_events) = ordered_actor("native-bootstrap-order-early").await;
    early.inject_hub_frame(welcome).await;
    early.inject_hub_frame(ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index: 0, bytes: semio_framework_os_spr::ArtifactBootstrapChunkBytes::try_from_slice(&chunks[0]).expect("bounded chunk") }).await;
    early.inject_hub_frame(tail()).await;
    let (early_pack, early_spr, early_frontier, _, _, _, early_remote_state, _, _) = early.bootstrap_test_state();
    assert_eq!(early_pack, None, "a tail before the snapshot installs nothing");
    assert_eq!(early_spr, None, "a tail before the snapshot installs nothing");
    assert_eq!(early_frontier, None, "and never advances the client's view of the server frontier");
    assert!(!matches!(early_remote_state, RemoteState::Live { .. }), "an out-of-order bootstrap must not read as a live session");
    let mut refusal = None;
    while let Ok(event) = early_events.try_recv() {
        if let ArtifactEvent::Conflict(message) = event {
            refusal = Some(message);
        }
    }
    let refusal = refusal.expect("the refusal is emitted, never swallowed");
    assert_eq!(refusal.code.0, "artifactBootstrap");
    assert!(refusal.message.contains("tail arrived before artifact bootstrap completion"), "the refusal names the order it enforced: {}", refusal.message);

    let (mut ordered, mut ordered_channel, _ordered_events) = ordered_actor("native-bootstrap-order-ok").await;
    ordered
        .inject_hub_frame(ServerFrame::Welcome {
            session_id: "session-order".into(),
            resume_token: "resume-order".into(),
            server_frontier: required.clone(),
            bootstrap: Bootstrap::ArtifactBootstrap(Box::new(bootstrap)),
        })
        .await;
    for (index, chunk) in chunks.iter().enumerate() {
        ordered.inject_hub_frame(ServerFrame::ArtifactBootstrapChunk { descriptor_hash, index: index as u32, bytes: semio_framework_os_spr::ArtifactBootstrapChunkBytes::try_from_slice(chunk).expect("bounded chunk") }).await;
    }
    ordered.inject_hub_frame(ServerFrame::ArtifactBootstrapDone { descriptor_hash, chunk_count: chunks.len() as u32 }).await;
    let _ = ordered_channel.receive().await.expect("installed messages");
    ordered.inject_hub_frame(tail()).await;
    let (ordered_pack, ordered_spr, ordered_frontier, ordered_pending, _, _, ordered_remote_state, _, _) = ordered.bootstrap_test_state();
    assert_eq!(ordered_pack, Some(pair.pack), "the same tail AFTER the snapshot is accepted");
    assert_eq!(ordered_spr, Some(pair.spr));
    assert_eq!(ordered_frontier, Some(required));
    assert_eq!(ordered_pending, None);
    assert!(matches!(ordered_remote_state, RemoteState::Live { .. }));
}

async fn sample_operation_envelope(edit_id: &str, n: i32) -> MutationEnvelope {
    let edit = Edit { line: None,
        id: edit_id.into(),
        actor: None,
        forwards: vec![DemoMutation::SetN { n }],
        inverse: vec![DemoMutation::SetN { n: 0 }].into(),
        mutation_meta: Vec::new(), verb: None,
        sequence_number: 1,
        started_at: "0".into(),
        finished_at: None,
    };
    let document_id = ArtifactId("demo".to_string());
    let schema = semio_framework_os_spr::SchemaId("demo/v1".to_string());
    let mut envelopes = semio_framework_os_spr::mutation_envelope_from_edit::<DemoSnapshot, DemoMutation>(&edit, &document_id, &schema).expect("operation envelope");
    envelopes.pop().expect("exactly one op envelope for a single-op edit")
}

/// 🪪️ Admits the fixture caller's independent byte ceiling and observes the original cumulative identity operation.
async fn receive_under_fixture_identity(session: &mut SyncSession<DemoSnapshot, DemoMutation>, envelope: MutationEnvelope) -> Result<(), SyncError> {
    const CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut observed = 0;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
        assert!(progress.owned_bytes <= CEILING);
        observed = progress.owned_bytes;
        true
    };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(CEILING, &mut observer).expect("declared fixture receive identity authority");
    let result = session.receive(envelope, &mut identity).await;
    drop(identity.pause().expect("original fixture receive identity receipt"));
    drop(observer);
    result
}

//#region 🧪️SyncSession
#[semio_framework_async_macros::async_test]
async fn receive_materializes_remote_envelope_into_the_edit_timeline() {
    let envelope: semio_framework_os_store_space_schema::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let store = semio_framework_os_store_test_support::plain_test_store(envelope).await;
    let mut session = SyncSession::new(store).await;
    receive_under_fixture_identity(&mut session, sample_operation_envelope("edit-1", 5).await).await.expect("receive");
    assert_eq!(session.store.snapshot().expect("snapshot").n, 5);
    assert_eq!(session.store.envelope().vcs.edits.len(), 1);
    semio_framework_os_store_test_support::close_plain_test_store(&mut session.store);
}

#[tokio::test]
async fn receive_buffers_out_of_order_envelopes_until_dependencies_arrive() {
    let envelope: semio_framework_os_store_space_schema::ArtifactEnvelope<DemoSnapshot, DemoMutation> = create_document_envelope("demo/v1", "demo", DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let store = semio_framework_os_store_test_support::plain_test_store(envelope).await;
    let mut session = SyncSession::new(store).await;
    let first = sample_operation_envelope("edit-1", 5).await;
    let mut second = sample_operation_envelope("edit-2", 9).await;
    second.dependencies = vec![first.mutation_id.clone()];
    receive_under_fixture_identity(&mut session, second).await.expect("receive second first");
    assert_eq!(session.store.envelope().vcs.edits.len(), 0, "buffered until edit-1 arrives");
    receive_under_fixture_identity(&mut session, first).await.expect("receive first");
    assert_eq!(session.store.envelope().vcs.edits.len(), 2, "both edits now applied");
    assert_eq!(session.store.snapshot().expect("snapshot").n, 9);
    semio_framework_os_store_test_support::close_plain_test_store(&mut session.store);
}
//#endregion 🧪️SyncSession
