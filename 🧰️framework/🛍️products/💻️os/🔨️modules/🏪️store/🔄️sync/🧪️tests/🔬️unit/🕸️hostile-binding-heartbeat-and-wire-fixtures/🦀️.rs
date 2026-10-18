// 🕸️ Hostile binding, heartbeat and wire fixtures: the hub url derivation, the hostile hub binding, the actor heartbeat stamps, the rollback envelope, the wire fixtures shared with typescript and the presence heartbeat laws.

//#region 🧪️Helpers
#[tokio::test]
async fn hub_ws_url_derives_ws_endpoint_from_remote_uri() {
    assert_eq!(hub_ws_url("remote://host:6070", "studio-1", "doc-1", None).await, "ws://host:6070/scopes/studio-1%2Fdoc-1/document/ws");
    assert_eq!(hub_ws_url("https://semio_hub.example.com", "studio-1", "doc-2", None).await, "wss://semio_hub.example.com/scopes/studio-1%2Fdoc-2/document/ws");
    assert_eq!(hub_ws_url("ws://127.0.0.1:5000/prefix", "studio-1", "d", None).await, "ws://127.0.0.1:5000/scopes/studio-1%2Fd/document/ws");
    assert_eq!(
        hub_ws_url("remote://host:6070", "studio /東京?", "doc#ä", Some("s.space.home@1/*#editor")).await,
        "ws://host:6070/scopes/studio%20%2F%E6%9D%B1%E4%BA%AC%3F%2Fdoc%23%C3%A4/document/ws?surface=s.space.home%401%2F%2A%23editor",
        "ticket 26/08/16/HUB-SPACES-…: surface travels out of band as ?surface= on the document WS URL"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::test]
async fn hostile_hub_binding_cannot_receive_a_credential_bound_document_grant() {
    use semio_framework_os_directory_client::{DirectoryClientError, DocumentSocketAdmissionV1, DocumentSocketAuthorityV1, DocumentSocketGrantReceiptV1, HubSocketGrantSource, LocalHubCredential};
    use semio_framework_os_directory::{DocumentOpenArtifactV1, DocumentOpenCatalogV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenParentDialectV1, DocumentOpenRendererTargetV1, DocumentOpenRevalidationV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1, DocumentScope};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct BoundSource {
        admissions: AtomicUsize,
        trusted_origin: String,
    }

    impl HubSocketGrantSource for BoundSource {
        fn admit_document_socket(
            &self,
            _ctx: &semio_framework_async::OperationContext,
            space_id: &str,
            document_id: &str,
            expectation: &semio_framework_os_directory_client::DocumentSocketExpectationV1,
            _client_instance_id: &str,
            _timeout_ms: u64,
        ) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
            self.admissions.fetch_add(1, Ordering::SeqCst);
            Ok(DocumentSocketAdmissionV1 {
                socket: DocumentSocketGrantReceiptV1 {
                    schema: "semio.hub.document-socket-grant/v1".into(),
                    protocol: "semio.session.v1".into(),
                    actor_id: format!("hub.v1.{}", "3".repeat(64)),
                    expires_at_ms: i64::MAX,
                },
                authority: DocumentSocketAuthorityV1 {
                    admitted_lease: expectation.lease.clone(),
                    hub_origin: self.trusted_origin.clone(),
                    browser_actor: semio_framework_os_directory::DocumentOpenBrowserActorV1::None,
                    expires_at_unix_ms: u64::try_from(i64::MAX).expect("positive max"),
                    scope: DocumentScope::new(space_id, document_id),
                    descriptor_digest_v1: "4".repeat(64),
                    catalog: DocumentOpenCatalogV1 { generation_id: "5".repeat(64) },
                    package: DocumentOpenPackageV1 {
                        plugin_id: "trusted-plugin".into(),
                        package_id: "semio:trusted-plugin".into(),
                        version: "1.0.0".into(),
                        component_sha256: "6".repeat(64),
                        component_blake3: "7".repeat(64),
                        descriptor_byte_sha256: "8".repeat(64),
                        execution_protocol: semio_framework_os_directory::DocumentExecutionProtocolV1 { app_channel_version: semio_framework_os_spr::CHANNEL_VERSION },
                        component_receipt: semio_framework_plugin_component_receipt::ComponentReceiptV1 {
                            schema: semio_framework_plugin_component_receipt::COMPONENT_RECEIPT_SCHEMA.into(),
                            plugin_id: "trusted-plugin".into(),
                            package_id: "semio:trusted-plugin".into(),
                            profile: semio_framework_plugin_component_receipt::ComponentProfileV1::Release,
                            target: semio_framework_plugin_component_receipt::COMPONENT_TARGET.into(),
                            wasm_sha256: "6".repeat(64),
                            wasm_bytes: 1024,
                            semantic_descriptor_sha256: "8".repeat(64),
                            toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)".into(),
                            source_tree_digest: "3".repeat(64),
                            provenance_sha256: "4".repeat(64),
                        },
                    },
                    artifact: DocumentOpenArtifactV1 { kind: "trusted.document".into(), schema: expectation.artifact_schema.clone(), pack_schema_hash: "1".repeat(64) },
                    parent_dialect: DocumentOpenParentDialectV1 { artifact_kind: "trusted.document".into(), standard: "1".into(), subset: "*".into() },
                    pack_schema_hash: [0x11; 32],
                    surface: DocumentOpenSurfaceV1 {
                        surface_id: "trusted.surface".into(),
                        app_id: "trusted.app".into(),
                        window_kind_id: "trusted.window".into(),
                        role: DocumentOpenSurfaceRoleV1::Editor,
                        renderer_target: DocumentOpenRendererTargetV1::Wgpu,
                    },
                    grant: DocumentOpenGrantV1 { read: true, write: true, observe: true },
                    checkpoint: semio_framework_os_directory::DocumentOpenCheckpointV1 {
                        checkpoint_id: "9".repeat(64),
                        descriptor_digest_v1: "4".repeat(64),
                        baseline_frontier: semio_framework_os_directory::ArtifactFrontier {
                            document_id: document_id.to_string(),
                            head_edit_ordinal: 0,
                            head_edit_id: String::new(),
                            last_commit_seq: 0,
                            chain_hash: semio_framework_os_directory::ArtifactHash::new([0; 32]),
                        },
                        aggregate_sha256: "a".repeat(64),
                    },
                    revalidation: DocumentOpenRevalidationV1 { directory_revision: 1, membership_generation: 1, session_generation: Some(1), share_generation: None },
                },
            })
        }
    }

    ensure_demo_codec_registered().await;
    let hostile = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("hostile listener");
    let hostile_origin = format!("http://{}", hostile.local_addr().expect("hostile address"));
    let source = Arc::new(BoundSource { admissions: AtomicUsize::new(0), trusted_origin: "http://127.0.0.1:1".into() });
    let (_, remote) = ChannelBackbone::pair("hostile-binding-test").await;
    let (_, receiver) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(8);
    let mut actor = native_actor::ArtifactActor::new(
        test_pool(),
        ArtifactActorConfig { actor_identity_grant: semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}, 
            document_id: "document".into(),
            schema: "demo/v1".into(),
            bindings: vec![PersistenceBinding::Hub { base_url: hostile_origin, space_id: "space".into(), surface: Some("trusted.surface".into()) }],
            watch_external: false,
            actor: "local-untrusted".into(),
        },
        remote,
        receiver,
        events,
        Arc::new(std::sync::RwLock::new(Some(Arc::new(LocalHubCredential::test("http://127.0.0.1:1", &format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64))))))),
        Arc::new(std::sync::RwLock::new(Some(source.clone()))),
        None,
        semio_framework_async::CancelToken::root_now(),
    )
    .await;
    actor.run_test_connect_attempt().await;
    assert_eq!(source.admissions.load(Ordering::SeqCst), 1);
    assert!(tokio::time::timeout(std::time::Duration::from_millis(50), hostile.accept()).await.is_err(), "hostile binding receives no dial or grant header");
    assert_eq!(actor.socket_epoch_test_state(), (None, false, 0, Vec::new()));
}

/// 🎨️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4: both
/// actors call `stamp_session` on every outbound `PresenceHeartbeat` right before
/// `presence_to_bytes` — shells never fill `color`/`surface` themselves. This proves the pure
/// helper both actors call: sets both fields from the actor's own `session_color`/`hub_surface`
/// state, overwriting whatever the shell handed in, and clears `surface` to `None` when the
/// document has no hub binding (folder-only) even if `session_color` is somehow set.
#[semio_framework_async_macros::async_test]
async fn actor_stamps_session_color_and_surface_on_outbound_heartbeat() {
    let mut peer = PresencePeer {
        actor: "actor-1".into(),
        connected_at_ms: 1000,
        label: None,
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: None,
        color: Some(99),
        surface: Some("shell-should-never-set-this".into()),
        views: Vec::new(),
        ui: None,
        tool_run: None,
        principal_kind: None, active_tool: None, history_edit: None,
        typing: Vec::new(),
    };
    stamp_session(&mut peer, Some(7), Some("s.space.home@1/*#editor")).await;
    assert_eq!(peer.color, Some(7));
    assert_eq!(peer.surface.as_deref(), Some("s.space.home@1/*#editor"));

    stamp_session(&mut peer, None, None).await;
    assert_eq!(peer.color, None, "no session color yet (folder-only document, or hub not yet assigned one)");
    assert_eq!(peer.surface, None, "folder-only document carries no surface");
}
//#endregion 🧪️Helpers

//#region 🧪️WireBridge
// 🎯️ W6: `wire_bridge_round_trips_identity_and_diff_through_protocol_causal` is DELETED — the
// local/wire bridge it tested (`to_wire_envelope`/`from_wire_envelope`) no longer exists; local
// and wire envelopes are the same `crate::os_spr::MutationEnvelope` type now, an identity the type
// system enforces, not something a round-trip test needs to prove.
#[semio_framework_async_macros::async_test]
async fn rollback_envelope_synthesizes_an_undo_from_the_original_inverse() {
    let envelope = sample_operation_envelope("edit-1", 5).await;
    let rollback = rollback_envelope(&envelope).await.expect("a domain operation rolls back by its inverse");
    assert_eq!(rollback.dependencies, vec![envelope.mutation_id.clone()], "the undo depends on the operation it undoes");
    assert_eq!(rollback.diff.payload, envelope.inverse.payload, "the undo's forward diff IS the original's inverse");
    assert_ne!(rollback.mutation_id, envelope.mutation_id, "the undo gets its own operation id");
}

/// 🎬️ Compares nineteen committed wire specimens with Rust encoding and decoding without
/// modifying them. The TypeScript replication suite consumes the same semantic-owner files
/// under `🧰️framework/🔨️modules/📡️replication/🧪️tests/🧫️fixtures/📡️wire`.
/// The current socket hello round-trips in memory; the committed obsolete hello must reject.
#[semio_framework_async_macros::async_test]
async fn wire_fixtures_stay_byte_identical_across_rust_and_ts() {
    let fixtures_dir = fixture_path!("../../../../../../../../../🔨️modules/📡️replication/🧪️tests/🧫️fixtures/📡️wire");

    async fn check_client(dir: &std::path::Path, name: &str, frame: &ClientFrame, lane: Lane) {
        let bytes = std::fs::read(dir.join(name)).unwrap_or_else(|error| panic!("read {name}: {error}"));
        assert_eq!(encode_client_frame(frame, lane).await, bytes, "{name} committed bytes");
        let (decoded_lane, decoded) = semio_framework_os_spr::decode_client_frame(&bytes).await.unwrap_or_else(|error| panic!("decode {name}: {error}"));
        assert_eq!(decoded_lane, lane, "{name} lane round trip");
        assert_eq!(&decoded, frame, "{name} frame round trip");
    }

    async fn check_server(dir: &std::path::Path, name: &str, frame: &ServerFrame, lane: Lane) {
        let bytes = std::fs::read(dir.join(name)).unwrap_or_else(|error| panic!("read {name}: {error}"));
        assert_eq!(semio_framework_os_spr::encode_server_frame(frame, lane).await, bytes, "{name} committed bytes");
        let (decoded_lane, decoded) = decode_server_frame(&bytes).await.unwrap_or_else(|error| panic!("decode {name}: {error}"));
        assert_eq!(decoded_lane, lane, "{name} lane round trip");
        assert_eq!(&decoded, frame, "{name} frame round trip");
    }

    let frontier = RuntimeFrontierSummary { document_id: ArtifactId("doc-1".to_string()), head_edit_ordinal: 1, head_edit_id: "op-1".to_string(), last_commit_seq: 1, chain_hash: [9u8; 32] };
    let wire_envelope = MutationEnvelope {
        mutation_id: MutationId("op-1".to_string()),
        document_id: ArtifactId("doc-1".to_string()),
        actor: ActorId("actor-1".into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: semio_framework_os_spr::ArtifactDiff { schema: semio_framework_os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
        inverse: semio_framework_os_spr::InverseMutation { schema: semio_framework_os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 0 }).expect("encode demo op") },
        timestamp: semio_framework_os_spr::HybridLogicalTimestamp { actor: 42, physical_ms: 1000, logical: 0 },
        transaction: None, verb: None, line: None,
    };

    let hello = ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: "demo/v1".to_string(), pack_schema_hash: [7u8; 32], resume_token: None, frontier: None };
    let hello_bytes = encode_client_frame(&hello, Lane::Command).await;
    let (hello_lane, decoded_hello) = semio_framework_os_spr::decode_client_frame(&hello_bytes).await.expect("decode current socket hello");
    assert_eq!(hello_lane, Lane::Command, "current socket hello lane round trip");
    assert_eq!(decoded_hello, hello, "current socket hello frame round trip");
    let rejected_hello = std::fs::read(fixtures_dir.join("🚫️legacy-client-hello-rejected/💾️.bin")).expect("read rejected obsolete hello");
    assert!(semio_framework_os_spr::decode_client_frame(&rejected_hello).await.is_err(), "obsolete hello must reject");
    check_client(&fixtures_dir, "🕹️client-commands/💾️.bin", &ClientFrame::Commands { batch_id: 1, envelopes: vec![wire_envelope.clone()] }, Lane::Command).await;
    check_client(&fixtures_dir, "🚩️client-frontier-advertise/💾️.bin", &ClientFrame::FrontierAdvertise { frontier: frontier.clone() }, Lane::Command).await;
    check_client(&fixtures_dir, "📣️client-preview-publish/💾️.bin", &ClientFrame::PreviewPublish { key: "cursor".to_string(), seq: 3, payload: vec![1, 2, 3] }, Lane::Preview).await;
    check_client(&fixtures_dir, "🙋️client-presence/💾️.bin", &ClientFrame::Presence { peer: presence_to_bytes(&sample_presence_peer_with_interaction().await).await }, Lane::Preview).await;
    check_client(&fixtures_dir, "🎟️client-credit-grant/💾️.bin", &ClientFrame::CreditGrant { n: 16 }, Lane::Command).await;
    check_client(&fixtures_dir, "👋️client-bye/💾️.bin", &ClientFrame::Bye, Lane::Command).await;

    check_server(&fixtures_dir, "🔗️server-welcome-tail/💾️.bin", &ServerFrame::Welcome { session_id: "session-1".to_string(), resume_token: "resume-1".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail }, Lane::Command).await;
    check_server(
        &fixtures_dir,
        "📸️server-welcome-snapshot-inline/💾️.bin",
        &ServerFrame::Welcome { session_id: "session-2".to_string(), resume_token: "resume-2".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Snapshot { pack_hash: [3u8; 32], inline: Some(vec![9, 9, 9]) } },
        Lane::Command,
    )
    .await;
    check_server(&fixtures_dir, "🧩️server-snapshot-chunk/💾️.bin", &ServerFrame::SnapshotChunk { seq: 0, bytes: semio_framework_os_spr::SnapshotChunkBytes::try_from_slice(&[1, 2, 3, 4]).unwrap() }, Lane::Command).await;
    check_server(&fixtures_dir, "🏁️server-snapshot-done/💾️.bin", &ServerFrame::SnapshotDone { seq_count: 4 }, Lane::Command).await;
    check_server(&fixtures_dir, "🎮️server-commands/💾️.bin", &ServerFrame::Commands { envelopes: vec![wire_envelope], origin: ActorId("actor-1".into()), frontier: frontier.clone() }, Lane::Command).await;
    check_server(
        &fixtures_dir,
        "✅️server-ack-accepted/💾️.bin",
        &ServerFrame::Ack { batch_id: 1, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: frontier.clone() },
        Lane::Command,
    )
    .await;
    check_server(
        &fixtures_dir,
        "🔀️server-ack-transformed/💾️.bin",
        &ServerFrame::Ack {
            batch_id: 2,
            stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Transformed { envelope: Box::new(sample_wire_envelope_for_fixtures().await) }) }],
            frontier: frontier.clone(),
        },
        Lane::Command,
    )
    .await;
    check_server(
        &fixtures_dir,
        "⛔️server-ack-rejected/💾️.bin",
        &ServerFrame::Ack { batch_id: 3, stages: vec![AckStage::Received, AckStage::Persisted, AckStage::Applied { outcome: Box::new(ApplyOutcome::Rejected { reason: "conflict".to_string(), messages: vec![1, 2, 3] }) }], frontier: frontier.clone() },
        Lane::Command,
    )
    .await;
    check_server(&fixtures_dir, "👁️server-preview/💾️.bin", &ServerFrame::Preview { actor: ActorId("actor-1".into()), key: "cursor".to_string(), seq: 3, payload: vec![5, 6] }, Lane::Preview).await;
    check_server(&fixtures_dir, "👥️server-presence/💾️.bin", &ServerFrame::Presence { peers: vec![b"{\"id\":\"a\"}".to_vec(), presence_to_bytes(&sample_presence_peer_with_interaction().await).await] }, Lane::Preview).await;
    check_server(&fixtures_dir, "🎫️server-credit-grant/💾️.bin", &ServerFrame::CreditGrant { n: 32 }, Lane::Command).await;
    check_server(&fixtures_dir, "🚨️server-error/💾️.bin", &ServerFrame::Error { code: "rejected".to_string(), message: "bad batch".to_string() }, Lane::Command).await;
    check_server(&fixtures_dir, "🪪️server-session/💾️.bin", &ServerFrame::Session { actor: "actor-1".to_string(), color: 5 }, Lane::Command).await;
}

/// 🧸️ A second, distinct `MutationEnvelope` for `🔀️server-ack-transformed/💾️.bin`'s
/// `ApplyOutcome::Transformed` payload — must differ from the primary `wire_envelope` fixture so
/// the vitest canary can assert it decodes as its own value, not an accidental copy.
async fn sample_wire_envelope_for_fixtures() -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: MutationId("op-2".to_string()),
        document_id: ArtifactId("doc-1".to_string()),
        actor: ActorId("actor-2".into()),
        dependencies: vec![MutationId("op-1".to_string())],
        observed: None,
        target: Vec::new(),
        diff: semio_framework_os_spr::ArtifactDiff { schema: semio_framework_os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 6 }).expect("encode demo op") },
        inverse: semio_framework_os_spr::InverseMutation { schema: semio_framework_os_spr::SchemaId("demo/v1".to_string()), payload: OpBinary::encode_op(&DemoMutation::SetN { n: 5 }).expect("encode demo op") },
        timestamp: semio_framework_os_spr::HybridLogicalTimestamp { actor: 42, physical_ms: 1001, logical: 0 },
        transaction: None, verb: None, line: None,
    }
}

/// 🕹️ A `PresencePeer` whose `interaction` carries THREE domains (one selection-only, one
/// hover-only, one with both), TWO `views` (one Orbit with a pointer, one Canvas), a `color` +
/// `surface`, and a `ui` — `🙋️client-presence/💾️.bin`/`👥️server-presence/💾️.bin` match this so
/// the TS vitest twin exercises every `PresencePeer` v3 flag bit (§C7.1) with a realistic payload.
async fn sample_presence_peer_with_interaction() -> PresencePeer {
    PresencePeer {
        actor: "actor-1".to_string(),
        connected_at_ms: 1_700_000_000_000,
        label: Some("Ada".to_string()),
        presence_pack: None,
        user_id: Some("user-9".to_string()),
        role: Some("owner".to_string()),
        drag_ghost_json: None,
        interaction: Some(semio_framework_os_spr::PresenceInteraction {
            app_id: "space".to_string(),
            domains: vec![
                semio_framework_os_spr::PresenceDomain { domain: "outline".to_string(), granularity: "task".to_string(), selected: vec!["t1".to_string(), "t2".to_string()], hovered: vec![] },
                semio_framework_os_spr::PresenceDomain { domain: "board".to_string(), granularity: "card".to_string(), selected: vec![], hovered: vec!["c1".to_string()] },
                semio_framework_os_spr::PresenceDomain { domain: "canvas".to_string(), granularity: "node".to_string(), selected: vec!["n9".to_string()], hovered: vec!["n9".to_string(), "n10".to_string()] },
            ],
        }),
        color: Some(5),
        surface: Some("s.space.home@1/*#editor".to_string()),
        views: vec![
            semio_framework_os_spr::PresenceWindowView {
                window_id: "w1".to_string(),
                space: "world".to_string(),
                kind: semio_framework_os_spr::PresenceViewKind::Orbit { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], up: [0.0, 1.0, 0.0], fov: 45.0 },
                size: [1024.0, 768.0],
                pointer: Some([0.5, 0.5, 0.5]),
                ray_origin: Some([1.0, 2.0, 3.0]),
            },
            semio_framework_os_spr::PresenceWindowView { window_id: "w2".to_string(), space: "canvas".to_string(), kind: semio_framework_os_spr::PresenceViewKind::Canvas { x: 12.5, y: -4.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None, ray_origin: None },
        ],
        ui: Some(semio_framework_os_spr::PresenceUi { hovered_path: Some("row[2]#t1".to_string()), focused_path: None, pressed_path: None }),
        tool_run: None,
        principal_kind: None, active_tool: None, history_edit: None,
        typing: Vec::new(),
    }
}
//#endregion 🧪️WireBridge

// 🎯️ ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.4:
// `assemble_presence_interaction` and its `🧪️PresenceInteraction` tests MOVED to
// `crate::os_spr::wire`'s `🔖️PresenceInteraction` region (`assemble_presence_interaction_tests`
// module there) alongside the function itself — see this file's `🔖️WireBridge` region for the
// pointer left behind.

//#region 🧪️PresenceInteraction
#[semio_framework_async_macros::async_test]
async fn presence_heartbeat_producer_publishes_immediately_then_coalesces_to_latest() {
    let mut producer = PresenceHeartbeatProducer::new(100);
    let mut first = sample_presence_peer_with_interaction().await;
    first.views = vec![semio_framework_os_spr::PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: semio_framework_os_spr::PresenceViewKind::Canvas { x: 1.0, y: 2.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None, ray_origin: None }];
    assert_eq!(producer.offer(1_000, first.clone()), Some(first));

    let mut intermediate = sample_presence_peer_with_interaction().await;
    intermediate.views = vec![semio_framework_os_spr::PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: semio_framework_os_spr::PresenceViewKind::Canvas { x: 3.0, y: 4.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None, ray_origin: None }];
    assert_eq!(producer.offer(1_040, intermediate), None);

    let mut latest = sample_presence_peer_with_interaction().await;
    latest.views = vec![semio_framework_os_spr::PresenceWindowView { window_id: "w1".into(), space: "canvas".into(), kind: semio_framework_os_spr::PresenceViewKind::Canvas { x: 5.0, y: 6.0, zoom: 1.0 }, size: [800.0, 600.0], pointer: None, ray_origin: None }];
    assert_eq!(producer.offer(1_099, latest.clone()), None);
    assert_eq!(producer.pending(), Some(&latest));
    assert_eq!(producer.offer(1_100, latest.clone()), Some(latest));
    assert!(producer.pending().is_none());
}

#[semio_framework_async_macros::async_test]
async fn presence_tool_run_summary_is_byte_credited_and_last_writer_wins() {
    let run = |state, completed, total| semio_framework_os_spr::PresenceToolRun { tool_id: "fill".into(), state, stage: 1, completed, total };
    let bare = sample_presence_peer_with_interaction().await;
    let mut indeterminate = bare.clone();
    indeterminate.tool_run = Some(run(semio_framework_os_spr::PresenceToolRunState::Starting, 0, None));
    let mut determinate = bare.clone();
    determinate.tool_run = Some(run(semio_framework_os_spr::PresenceToolRunState::Running, 42, Some(100)));
    let bytes = |peer: &PresencePeer| artifact_actor_message_bytes(&ArtifactActorMsg::PresenceHeartbeat { peer: Box::new(peer.clone()) }).expect("presence fits the mailbox");
    assert!(bytes(&bare) < bytes(&indeterminate) && bytes(&indeterminate) < bytes(&determinate), "tool run id, counters and total contribute byte credit");
    assert_eq!(semio_framework_os_spr::decode_presence_peer(&presence_to_bytes(&determinate).await).await.expect("tool run presence decodes"), determinate);

    let mut producer = PresenceHeartbeatProducer::new(100);
    assert_eq!(producer.offer(1_000, indeterminate.clone()), Some(indeterminate));
    assert_eq!(producer.offer(1_050, determinate.clone()), None);
    let mut finalized = determinate.clone();
    finalized.tool_run = Some(run(semio_framework_os_spr::PresenceToolRunState::Finalized, 100, Some(100)));
    assert_eq!(producer.offer(1_080, finalized.clone()), None);
    assert_eq!(producer.pending(), Some(&finalized), "the newest tool run summary replaces pending progress");
    assert_eq!(producer.offer(1_100, finalized.clone()), Some(finalized));
}

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn artifact_host_presence_heartbeat_owns_cadence_per_document() {
    let host = ArtifactHost::new(test_pool());
    let (cmd_tx, cmd_rx) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(1);
    let runner = native_actor::retained_turn_fixtures::fixture_runner_handle(host.core().pool.clone(), 1, cmd_rx.close_handle());
    host.core().insert_document(ArtifactDocumentKey::local("doc"), OpenDocument { generation: 1, cancel: semio_framework_async::CancelToken::root_now(), cmd_tx, events, presence: PresenceHeartbeatProducer::default(), runner });

    let first = sample_presence_peer_with_interaction().await;
    assert!(host.presence_heartbeat("doc", 500, first.clone()));
    assert!(matches!(cmd_rx.try_recv(), Some(ArtifactActorMsg::PresenceHeartbeat { peer }) if *peer == first));

    let mut latest = sample_presence_peer_with_interaction().await;
    latest.ui = Some(semio_framework_os_spr::PresenceUi { hovered_path: Some("row[0]#changed".into()), focused_path: None, pressed_path: None });
    assert!(!host.presence_heartbeat("doc", 550, latest.clone()));
    assert!(cmd_rx.try_recv().is_none(), "sub-interval offer must not publish");
    assert!(host.presence_heartbeat("doc", 600, latest.clone()));
    assert!(matches!(cmd_rx.try_recv(), Some(ArtifactActorMsg::PresenceHeartbeat { peer }) if *peer == latest));
    assert!(!host.presence_heartbeat("missing", 700, sample_presence_peer_with_interaction().await));
}

/// ⏯️ The presence assembly carries the guest's tool run summary (`AppFrame::Ephemeral.tool_run`) into the local
/// peer: an observed summary wins over the renderer's, its absence clears a stale one, and `completed` never
/// exceeds `total`, so every published peer decodes.
#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn artifact_host_presence_heartbeat_stamps_the_observed_tool_run_summary() {
    let host = ArtifactHost::new(test_pool());
    let (cmd_tx, cmd_rx) = artifact_mailbox_pair();
    let (events, _) = broadcast::channel(1);
    let runner = native_actor::retained_turn_fixtures::fixture_runner_handle(host.core().pool.clone(), 1, cmd_rx.close_handle());
    let key = ArtifactDocumentKey::local("doc");
    host.core().insert_document(key.clone(), OpenDocument { generation: 1, cancel: semio_framework_async::CancelToken::root_now(), cmd_tx, events, presence: PresenceHeartbeatProducer::new(100), runner });
    let run = |state, completed, total| semio_framework_os_spr::PresenceToolRun { tool_id: "fill".into(), state, stage: 0, completed, total };

    let mut renderer_peer = sample_presence_peer_with_interaction().await;
    renderer_peer.tool_run = Some(run(semio_framework_os_spr::PresenceToolRunState::Running, 9, Some(4)));
    assert!(host.presence_heartbeat_key(&key, 1_000, renderer_peer.clone()));
    let Some(ArtifactActorMsg::PresenceHeartbeat { peer }) = cmd_rx.try_recv() else { panic!("first heartbeat publishes") };
    assert_eq!(peer.tool_run.as_ref().map(|tool_run| (tool_run.completed, tool_run.total)), Some((4, Some(4))), "an unobserved renderer summary is still clamped");

    assert!(host.observe_presence_tool_run_key(&key, Some(run(semio_framework_os_spr::PresenceToolRunState::Complete, 120, Some(100)))));
    assert!(host.presence_heartbeat_key(&key, 1_100, sample_presence_peer_with_interaction().await));
    let Some(ArtifactActorMsg::PresenceHeartbeat { peer }) = cmd_rx.try_recv() else { panic!("second heartbeat publishes") };
    assert_eq!(peer.tool_run, Some(run(semio_framework_os_spr::PresenceToolRunState::Complete, 100, Some(100))), "the observed summary is stamped, completed clamped to total");
    assert_eq!(semio_framework_os_spr::decode_presence_peer(&presence_to_bytes(&peer).await).await.expect("the stamped peer decodes"), *peer);

    assert!(host.observe_presence_tool_run_key(&key, None));
    assert!(host.presence_heartbeat_key(&key, 1_200, renderer_peer));
    let Some(ArtifactActorMsg::PresenceHeartbeat { peer }) = cmd_rx.try_recv() else { panic!("third heartbeat publishes") };
    assert!(peer.tool_run.is_none(), "a dismissed run clears the renderer's stale summary");
    assert!(!host.observe_presence_tool_run_key(&ArtifactDocumentKey::local("missing"), None));
}
//#endregion 🧪️PresenceInteraction

//#region 🧪️Helpers

#[cfg(not(target_arch = "wasm32"))]
#[semio_framework_async_macros::async_test]
async fn op_envelope_from_stored_edit_round_trips_through_ingest() {
    let edit = semio_framework_os_spr::HistoryEdit { line: None,
        id: "ext-1".into(),
        actor: Some("peer".into()),
        started_at: "0".into(),
        finished_at: None, verb: None,
        ops: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 42 }.encode_op().expect("encode")) }],
        inverse: vec![semio_framework_os_spr::OpPayload { text: None, binary: Some(DemoMutation::SetN { n: 0 }.encode_op().expect("encode")) }],
        meta: None, lane: None,
    };
    let envelopes = envelopes_from_history_edit(&edit, "demo", "demo/v1", semio_framework_value::RetainedCloneGrant {maximum_items:1024,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:64}).await.expect("envelopes from history edit");
    assert_eq!(envelopes.len(), 1, "single-op edit yields one envelope");
    assert_eq!(envelopes[0].mutation_id.0, "ext-1#0", "meta-less fallback: edit id # op index");
    let recovered = <DemoMutation as OpBinary>::decode_op(&envelopes[0].diff.payload).expect("decode op");
    assert_eq!(recovered, DemoMutation::SetN { n: 42 });
}
//#endregion 🧪️Helpers
