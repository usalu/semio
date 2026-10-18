    // 🔁️ Two host convergence, reconnect and fixture replay laws: the two hosts converging through the hub, the raw backbone message, the admission plan window, the reconnect catch up, the detach drain, the command ack, the preview round trip and the fixture replay.

//#region 🧪️Actor
    /// 🔬️ Two stores, each behind its own ArtifactHost, converge through a semio_hub on edits AND
    /// structural history: every step A authors (apply, undo, checkpoint, redo) reaches B only as
    /// events relayed by the hub, and B's fold lands on exactly A's position after each one.
    #[tokio::test]
    async fn two_hosts_converge_through_hub() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        ensure_demo_codec_registered().await;
        let (addr, hub) = spawn_mock_hub_with_session_gate(true).await;
        let base_url = format!("ws://{addr}");
        let config = |actor: &str| ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
            document_id: "shared".into(),
            schema: "demo/v1".into(),
            bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
            watch_external: false,
            actor: actor.into(),
        };
        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a = host_a.open(config("A")).await;
        let key_a = channels_a.document_key.clone();
        let mut events_a = host_a.subscribe_key(&key_a).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await;
        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b.open(config("B")).await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        let mut store_a = semio_framework_os_store_test_support::plain_test_store(demo_envelope("shared").await).await;
        store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
        let mut store_b = semio_framework_os_store_test_support::plain_test_store(demo_envelope("shared").await).await;
        store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");
        let position = |store: &ArtifactStore<DemoSnapshot, DemoMutation>| (store.snapshot().expect("snapshot"), store.applied_edit_ids().len(), store.redo_edit_ids().len(), store.current_checkpoint_id().map(str::to_string));

        let steps: Vec<ArtifactCommand<DemoMutation>> = vec![
            ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 7 }], transaction: None },
            ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 9 }], transaction: None },
            ArtifactCommand::Undo,
            ArtifactCommand::CommitCheckpoint { message: Some("hub checkpoint".into()), authors: Vec::new() },
            ArtifactCommand::Redo,
        ];
        for command in steps {
            store_a.dispatch(command, &mut identity).await.expect("a authors the step");
            channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");
            wait_for_mock_hub_event("B DocumentBackbone", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
            store_b.tick().await.expect("b folds the relayed events");
            assert_eq!(position(&store_b), position(&store_a), "B sits exactly where A sits after every hub-relayed step");
        }
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 9, "the redo reinstated A's second edit on B too");

        semio_framework_os_store_test_support::close_plain_test_store(&mut store_a);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_b);
        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    #[tokio::test]
    async fn raw_document_backbone_reaches_hub_once_and_returns_one_canonical_event() {
        ensure_demo_codec_registered().await;
        let (addr, hub) = spawn_mock_hub_with_session_gate(true).await;
        let base_url = format!("ws://{addr}");
        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a = host_a
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "raw-shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let key_a = channels_a.document_key.clone();
        let mut events_a = host_a.subscribe_key(&key_a).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        assert!(matches!(wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await, ArtifactEvent::Session { .. }));
        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "raw-shared".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "B".into(),
            })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        assert!(matches!(wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await, ArtifactEvent::Session { .. }));

        let original = document_backbone_envelope("raw-mutation", "raw-shared");
        channels_a.cmd_tx.send(ArtifactActorMsg::DocumentBackbone { message: document_backbone_message(std::slice::from_ref(&original)) }).expect("exact raw owner admitted");
        let event = wait_for_mock_hub_event("B DocumentBackbone", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        let delivered = document_backbone_event_envelopes(&event).expect("canonical raw event");
        assert_eq!(delivered.len(), 1);
        assert_eq!(delivered[0].mutation_id, original.mutation_id);
        assert_eq!(delivered[0].diff, original.diff);
        assert_eq!(delivered[0].inverse, original.inverse);
        assert_ne!(delivered[0].actor, original.actor, "Hub authority stamps its actor without rewriting opaque mutation fields");
        assert!(matches!(wait_for_mock_hub_event("A CommandOutcome", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await, ArtifactEvent::CommandOutcome { outcome: CommandAckOutcome::Accepted, .. }));
        while let Ok(next) = events_b.try_recv() {
            assert!(!matches!(next, ArtifactEvent::RemoteMutations { .. }), "one server Commands frame cannot emit a duplicate legacy mutation event");
        }

        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    /// 🔌️ A live document socket outlives its admission plan's exchange window: the plan's expiry bounds when its grant
    /// may be exchanged (the hub's `DOCUMENT_OPEN_PLAN_MAX_TTL_MS`, 30 s), never the socket, which the hub re-proves every
    /// second. With a 3 s window, an edit sent 3.5 s after Session goes out on the SAME socket and is accepted — the React
    /// worker's contract since 09-22 (ticket 26/09/23 session 14c: both Rust actors re-planned every live socket at it).
    #[tokio::test]
    async fn a_live_socket_outlives_its_admission_plan_window() {
        ensure_demo_codec_registered().await;
        let (addr, hub) = spawn_mock_hub_with_session_gate(true).await;
        let base_url = format!("ws://{addr}");
        let host = ArtifactHost::new(test_pool());
        configure_mock_hub_with_plan_window(&host, &base_url, 'a', &hub, 3_000);
        let channels = host
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "plan-window".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let key = channels.document_key.clone();
        let mut events = host.subscribe_key(&key).await;
        hub.session_gate.as_ref().expect("gated mock").add_permits(1);
        assert!(matches!(wait_for_mock_hub_event("Session", &hub, &mut events, |event| matches!(event, ArtifactEvent::Session { .. })).await, ArtifactEvent::Session { .. }));
        tokio::time::sleep(std::time::Duration::from_millis(3_500)).await;
        let edit = document_backbone_envelope("after-the-plan-window", "plan-window");
        channels.cmd_tx.send(ArtifactActorMsg::DocumentBackbone { message: document_backbone_message(std::slice::from_ref(&edit)) }).expect("edit after the plan window");
        assert!(matches!(wait_for_mock_hub_event("CommandOutcome", &hub, &mut events, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await, ArtifactEvent::CommandOutcome { outcome: CommandAckOutcome::Accepted, .. }));
        assert_eq!(hub.connections.load(Ordering::SeqCst), 1, "the edit went out on the socket admitted inside the plan window");
        host.close_key(&key);
    }

    // 🔬️ Reconnect with `since` catch-up: after A appends operations while B is offline, B reconnects and
    // its Welcome backlog carries only the operations it missed.
    /// 📬️ A applies two operations while nobody else is connected.
    ///
    /// B connects fresh (since_version 0) and its Welcome backlog replays both operations.
    #[tokio::test]
    async fn reconnect_since_catch_up_replays_backlog() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        let (addr, hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a = host_a
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "catchup".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let mut events_a = host_a.subscribe_key(&channels_a.document_key).await;
        let mut store_a = semio_framework_os_store_test_support::plain_test_store(demo_envelope("catchup").await).await;
        let key_a = channels_a.document_key.clone();
        store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
        wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        for n in [3, 4] {
            store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n }], transaction: None }, &mut identity).await.expect("apply on a");
            channels_a.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake a");
            tokio::time::sleep(Duration::from_millis(80)).await;
        }

        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "catchup".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "B".into() })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        let mut store_b = semio_framework_os_store_test_support::plain_test_store(demo_envelope("catchup").await).await;
        store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");
        wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        assert_eq!(document_backbone_event_envelopes(&event).expect("exact backlog event").len(), 2, "backlog replays both missed operations");
        store_b.tick().await.expect("tick b");
        assert_eq!(store_b.envelope().vcs.edits.len(), 2, "B caught up on the full backlog");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 4);

        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_a);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_b);
    }

    // 🔬️ Detach drains the outbox: an operation applied right before close still reaches the semio_hub (and B).
    /// 🚿️ Observer B stays connected to witness A's last operation.
    ///
    /// Immediately close A without waiting for the poll tick: Detach must flush the outbox first.
    #[tokio::test]
    async fn detach_drains_pending_outbound_operations() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        let (addr, hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "drain".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "B".into(),
            })
            .await;
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        let mut store_b = semio_framework_os_store_test_support::plain_test_store(demo_envelope("drain").await).await;
        store_b.attach_backbone(Backbones::Channel(channels_b.channel_backbone)).await.expect("attach b");
        wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a =
            host_a.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "drain".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "A".into() }).await;
        let mut events_a = host_a.subscribe_key(&channels_a.document_key).await;
        let mut store_a = semio_framework_os_store_test_support::plain_test_store(demo_envelope("drain").await).await;
        let key_a = channels_a.document_key.clone();
        store_a.attach_backbone(Backbones::Channel(channels_a.channel_backbone)).await.expect("attach a");
        wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        store_a.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 5 }], transaction: None }, &mut identity).await.expect("apply on a");
        host_a.close_key(&key_a);

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::DocumentBackbone { .. })).await;
        assert_eq!(document_backbone_event_envelopes(&event).expect("exact drain event").len(), 1, "the operation applied before detach was not lost");
        store_b.tick().await.expect("tick b");
        assert_eq!(store_b.snapshot().expect("snapshot b").n, 5);
        host_b.close_key(&key_b);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_b);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store_a);
    }

    // 🔬️ The mock semio_hub always Acks `Accepted` — confirms the new `ServerFrame::Ack` ->
    // `ArtifactEvent::CommandOutcome` wiring actually fires (not just that it compiles).
    #[tokio::test]
    async fn command_outcome_accepted_fires_after_hub_ack() {
    const IDENTITY_CEILING:usize=201*semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;
    let mut identity_observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=IDENTITY_CEILING);true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(IDENTITY_CEILING,&mut identity_observer).expect("declared original Store test identity");
        let (addr, hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");
        let host = ArtifactHost::new(test_pool());
        configure_mock_hub(&host, &base_url, 'a', &hub);
        let channels =
            host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "outcome".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "A".into() }).await;
        let key = channels.document_key.clone();
        let mut events = host.subscribe_key(&key).await;
        let mut store = semio_framework_os_store_test_support::plain_test_store(demo_envelope("outcome").await).await;
        store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");
        wait_for_mock_hub_event("A Session", &hub, &mut events, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        store.dispatch(ArtifactCommand::Apply { mutations: vec![DemoMutation::SetN { n: 1 }], transaction: None }, &mut identity).await.expect("apply");
        channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: Vec::new() }).expect("wake");

        let event = wait_for_event(&mut events, |event| matches!(event, ArtifactEvent::CommandOutcome { .. })).await;
        match event {
            ArtifactEvent::CommandOutcome { outcome, .. } => assert_eq!(outcome, CommandAckOutcome::Accepted),
            other => panic!("expected CommandOutcome, got {other:?}"),
        }
        host.close_key(&key);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store);
    }

    // 🔬️ `SyncSession::publish_preview` -> `ClientFrame::PreviewPublish` -> the mock semio_hub's
    // preview-lane fan-out -> `ServerFrame::Preview` -> `ArtifactEvent::Preview` on another peer.
    #[tokio::test]
    async fn publish_preview_round_trips_through_hub() {
        let (addr, hub) = spawn_mock_hub().await;
        let base_url = format!("ws://{addr}");

        let host_a = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_a, &base_url, 'a', &hub);
        let channels_a = host_a
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
                document_id: "preview".into(),
                schema: "demo/v1".into(),
                bindings: vec![PersistenceBinding::Hub { base_url: base_url.clone(), space_id: "studio-1".into(), surface: None }],
                watch_external: false,
                actor: "A".into(),
            })
            .await;
        let mut events_a = host_a.subscribe_key(&channels_a.document_key).await;

        let host_b = ArtifactHost::new(test_pool());
        configure_mock_hub(&host_b, &base_url, 'b', &hub);
        let channels_b = host_b
            .open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: "preview".into(), schema: "demo/v1".into(), bindings: vec![PersistenceBinding::Hub { base_url, space_id: "studio-1".into(), surface: None }], watch_external: false, actor: "B".into() })
            .await;
        let key_a = channels_a.document_key.clone();
        let key_b = channels_b.document_key.clone();
        let mut events_b = host_b.subscribe_key(&key_b).await;
        wait_for_mock_hub_event("A Session", &hub, &mut events_a, |event| matches!(event, ArtifactEvent::Session { .. })).await;
        wait_for_mock_hub_event("B Session", &hub, &mut events_b, |event| matches!(event, ArtifactEvent::Session { .. })).await;

        channels_a.cmd_tx.send(ArtifactActorMsg::PublishPreview { key: "cursor".into(), seq: 1, payload: vec![1, 2, 3] }).expect("publish preview");

        let event = wait_for_event(&mut events_b, |event| matches!(event, ArtifactEvent::Preview { .. })).await;
        match event {
            ArtifactEvent::Preview { key, seq, payload, .. } => {
                assert_eq!(key, "cursor");
                assert_eq!(seq, 1);
                assert_eq!(payload, vec![1, 2, 3]);
            }
            other => panic!("expected Preview, got {other:?}"),
        }
        host_a.close_key(&key_a);
        host_b.close_key(&key_b);
    }

    // 🔬️ Shared fixtures replay: each fixture's inbound stimuli produce the expected ArtifactEvent
    // sequence and final timeline. The same fixtures drive WS-E's vitest harness against the TS twin.
    #[tokio::test]
    async fn fixtures_replay_matches_expected_events() {
        let fixtures_dir = fixture_path!("../../../../🧫️fixtures");
        let fixtures = load_fixtures(&fixtures_dir).await;
        assert!(!fixtures.is_empty(), "expected fixtures in {fixtures_dir:?}");
        for fixture in fixtures {
            replay_fixture(&fixture).await;
        }
    }

    /// 🎞️ Lockstep: apply each stimulus, then wait for its paired expected event before the next
    /// (removes any write/poke race). Folder-replayable fixtures pair inbound 1:1 with events.
    async fn replay_fixture(fixture: &ActorFixture) {
        ensure_demo_codec_registered().await;
        let codec = semio_framework_os_store_codec_registry::document_codec(&fixture.schema).await.expect("codec registry available").unwrap_or_else(|| panic!("no codec registered for fixture schema {:?}", fixture.schema));
        let dir = semio_framework_os_store_test_support::tempdir().expect("tempdir");
        let host = ArtifactHost::new(test_pool());
        let channels =
            host.open(ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64},  document_id: fixture.document_id.clone(), schema: fixture.schema.clone(), bindings: vec![PersistenceBinding::Folder { path: dir.path().to_path_buf() }], watch_external: true, actor: "local".into() }).await;
        let mut events = host.subscribe(&fixture.document_id).await;
        let mut store = semio_framework_os_store_test_support::plain_test_store(create_document_envelope::<DemoSnapshot, DemoMutation>(&fixture.schema, &fixture.document_id, DemoSnapshot { n: 0 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).await;
        store.attach_backbone(Backbones::Channel(channels.channel_backbone)).await.expect("attach");
        let storage = FolderEventLogStorage::new(dir.path().to_path_buf());
        wait_until(&format!("seed snapshot for {} on disk", fixture.document_id), || async { storage.read_archive(&fixture.document_id).await.expect("read archive").is_some() }).await;

        assert_eq!(fixture.inbound.len(), fixture.expected_events.len(), "fixture {} must pair each inbound stimulus with one expected event", fixture.name);
        let mut observed: Vec<String> = Vec::new();
        for (inbound, expected) in fixture.inbound.iter().zip(fixture.expected_events.iter()) {
            match inbound {
                FixtureInbound::ExternalEdits { ops_text } => {
                    let archive_bytes = storage.read_archive(&fixture.document_id).await.expect("read archive").expect("some archive");
                    let mut archive = semio_framework_os_spr::decode_document_archive_bytes(&archive_bytes).await.expect("decode current archive");
                    let parsed = semio_framework_os_spr::parse_ops_text(ops_text).unwrap_or_else(|error| panic!("fixture {} parse_ops_text: {error}", fixture.name));
                    let mut new_edits: Vec<semio_framework_os_spr::HistoryEdit> = Vec::new();
                    for edit in parsed.edits {
                        let mut ops: Vec<semio_framework_os_spr::OpPayload> = Vec::new();
                        for op in &edit.ops {
                            let text = op.text.as_deref().unwrap_or_else(|| panic!("fixture {} op line has no text", fixture.name));
                            let concrete = DemoMutation::parse_op(text).unwrap_or_else(|error| panic!("fixture {} parse_op {text:?}: {error}", fixture.name));
                            ops.push(semio_framework_os_spr::OpPayload { text: None, binary: Some(concrete.encode_op().expect("encode demo op")) });
                        }
                        new_edits.push(semio_framework_os_spr::HistoryEdit { ops, meta: None, ..edit });
                    }
                    archive.parent_spr = semio_framework_os_store_text_io::text_format::append_history_events_to_spr(&archive.parent_spr, &new_edits, &[]).await.expect("append fixture edits");
                    let archive_bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).expect("encode fixture archive");
                    storage.write_archive(&fixture.document_id, &fixture.schema, &archive_bytes).await.expect("write archive");
                    channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                }
                FixtureInbound::ReplaceDocument { dsl_text, ops_text } => {
                    let (pack_files, _dsl_mirror) = (codec.compile_dsl)(dsl_text, ops_text).await.unwrap_or_else(|error| panic!("fixture {} compile_dsl: {error}", fixture.name));
                    let archive = semio_framework_os_spr::DocumentArchivePack { parent_pack: pack_files.pack, parent_spr: pack_files.spr, members: Vec::new() };
                    let archive_bytes = semio_framework_os_spr::encode_document_archive_bytes(&archive).expect("encode replacement archive");
                    storage.write_archive(&fixture.document_id, &fixture.schema, &archive_bytes).await.expect("replace archive write");
                    channels.cmd_tx.send(ArtifactActorMsg::ExternalChanged).expect("poke");
                }
                FixtureInbound::HubFrame { .. } => {
                    panic!("fixture {} uses a HubFrame stimulus not supported by the Rust harness", fixture.name);
                }
            }
            let event = wait_for_event(&mut events, |event| document_event_tag(event) == expected.as_str()).await;
            observed.push(document_event_tag(&event).to_string());
            store.tick().await.expect("tick");
        }
        assert_eq!(&observed, &fixture.expected_events, "fixture {} event sequence", fixture.name);
        let timeline_ids: Vec<String> = store.envelope().vcs.edits.iter().map(|edit| edit.id.clone()).collect();
        for expected_id in &fixture.expected_edit_ids {
            assert!(timeline_ids.contains(expected_id), "fixture {} expected edit id {expected_id} in timeline {timeline_ids:?}", fixture.name);
        }
        host.close(&fixture.document_id);
        semio_framework_os_store_test_support::close_plain_test_store(&mut store);
    }

    // 🚫️async: E1-adjacent — pure match with no suspension point, consumed by
    // `wait_for_event`'s sync `FnMut(&ArtifactEvent) -> bool` predicate bound — see R9.
    fn document_event_tag(event: &ArtifactEvent) -> &'static str {
        match event {
            ArtifactEvent::RemoteMutations { .. } => "remoteMutations",
            ArtifactEvent::DocumentBackbone { .. } => "documentBackbone",
            ArtifactEvent::DocumentArchiveReplaced { .. } => "documentArchiveReplaced",
            ArtifactEvent::DocumentArchiveAbsent => "documentArchiveAbsent",
            ArtifactEvent::BootstrapProgress { .. } => "bootstrapProgress",
            ArtifactEvent::Status(_) => "status",
            ArtifactEvent::Presence { .. } => "presence",
            ArtifactEvent::Session { .. } => "session",
            ArtifactEvent::Preview { .. } => "preview",
            ArtifactEvent::CommandOutcome { .. } => "commandOutcome",
            ArtifactEvent::Conflict(_) => "conflict",
            ArtifactEvent::RebootstrapRequired { .. } => "rebootstrapRequired",
        }
    }
//#endregion 🧪️Actor
