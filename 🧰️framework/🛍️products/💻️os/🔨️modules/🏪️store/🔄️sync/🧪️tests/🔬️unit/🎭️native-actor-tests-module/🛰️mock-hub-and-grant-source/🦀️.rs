    // 🛰️ Mock hub and grant source: the mock hub with its socket grant source, the host configuration, the frontier and the mock hub connection loop.

//#region 🧪️Actor
    //#region 🔖️MockHub
    /// 🧪️ A minimal in-process semio_hub speaking the real, binary `crate::os_spr::wire::ClientFrame`/
    /// `ServerFrame` protocol, so the semio_hub endpoint is exercised end-to-end without linking a real
    /// `db`-backed semio_hub (that's CW6's job — this mock never touches `db`). Ordinal-indexed log,
    /// mirroring `db_sync`'s replica-catch-up shape (`Hello.frontier` -> filtered backlog ->
    /// `Welcome` then a follow-up `Commands`), but with a placeholder `chain_hash`/`resume_token`
    /// (this mock has no durable log to derive a real chain hash from).
    struct MockHub {
        log: Arc<Mutex<Vec<(u64, MutationEnvelope)>>>,
        broadcast: tokio_broadcast::Sender<ServerFrame>,
        connections: AtomicUsize,
        session_gate: Option<Arc<tokio::sync::Semaphore>>,
        trace: Arc<std::sync::Mutex<Vec<&'static str>>>,
    }

    impl MockHub {
        fn record(&self, event: &'static str) {
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(event);
        }

        fn trace(&self) -> Vec<&'static str> {
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
        }
    }

    struct MockHubSocketGrantSource {
        hub_origin: String,
        actor_id: String,
        plan_window_ms: u64,
        trace: Arc<std::sync::Mutex<Vec<&'static str>>>,
    }

    impl semio_framework_os_directory_client::HubSocketGrantSource for MockHubSocketGrantSource {
        fn admit_document_socket(
            &self,
            _ctx: &semio_framework_async::OperationContext,
            space_id: &str,
            document_id: &str,
            expectation: &semio_framework_os_directory_client::DocumentSocketExpectationV1,
            _client_instance_id: &str,
            _timeout_ms: u64,
        ) -> Result<semio_framework_os_directory_client::DocumentSocketAdmissionV1, semio_framework_os_directory_client::DirectoryClientError> {
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("grant.entered");
            let expires_at_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).expect("system clock after epoch").as_millis().saturating_add(u128::from(self.plan_window_ms)).min(i64::MAX as u128) as i64;
            let admission = semio_framework_os_directory_client::DocumentSocketAdmissionV1 {
                socket: semio_framework_os_directory_client::DocumentSocketGrantReceiptV1 {
                    schema: "semio.hub.document-socket-grant/v1".into(),
                    protocol: "semio.session.v1".into(),
                    actor_id: self.actor_id.clone(),
                    expires_at_ms,
                },
                authority: semio_framework_os_directory_client::DocumentSocketAuthorityV1 {
                    admitted_lease: expectation.lease.clone(),
                    hub_origin: self.hub_origin.clone(),
                    browser_actor: semio_framework_os_directory::DocumentOpenBrowserActorV1::None,
                    expires_at_unix_ms: u64::try_from(expires_at_ms).expect("positive mock expiry"),
                    scope: semio_framework_os_directory::DocumentScope::new(space_id, document_id),
                    descriptor_digest_v1: "4".repeat(64),
                    catalog: semio_framework_os_directory::DocumentOpenCatalogV1 { generation_id: "5".repeat(64) },
                    package: semio_framework_os_directory::DocumentOpenPackageV1 {
                        plugin_id: "mock-plugin".into(),
                        package_id: "semio:mock-plugin".into(),
                        version: "1.0.0".into(),
                        component_sha256: "6".repeat(64),
                        component_blake3: "7".repeat(64),
                        descriptor_byte_sha256: "8".repeat(64),
                        execution_protocol: semio_framework_os_directory::DocumentExecutionProtocolV1 { app_channel_version: semio_framework_os_spr::CHANNEL_VERSION },
                        component_receipt: semio_framework_plugin_component_receipt::ComponentReceiptV1 {
                            schema: semio_framework_plugin_component_receipt::COMPONENT_RECEIPT_SCHEMA.into(),
                            plugin_id: "mock-plugin".into(),
                            package_id: "semio:mock-plugin".into(),
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
                    artifact: semio_framework_os_directory::DocumentOpenArtifactV1 { kind: "mock.document".into(), schema: expectation.artifact_schema.clone(), pack_schema_hash: "1".repeat(64) },
                    parent_dialect: semio_framework_os_directory::DocumentOpenParentDialectV1 { artifact_kind: "mock.document".into(), standard: "1".into(), subset: "*".into() },
                    pack_schema_hash: expectation.pack_schema_hash,
                    surface: semio_framework_os_directory::DocumentOpenSurfaceV1 {
                        surface_id: expectation.requested_surface_id.clone().unwrap_or_else(|| "mock.surface".into()),
                        app_id: "mock.app".into(),
                        window_kind_id: "mock.window".into(),
                        role: semio_framework_os_directory::DocumentOpenSurfaceRoleV1::Editor,
                        renderer_target: semio_framework_os_directory::DocumentOpenRendererTargetV1::Wgpu,
                    },
                    grant: semio_framework_os_directory::DocumentOpenGrantV1 { read: true, write: true, observe: true },
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
                    revalidation: semio_framework_os_directory::DocumentOpenRevalidationV1 { directory_revision: 1, membership_generation: 1, session_generation: Some(1), share_generation: None },
                },
            };
            self.trace.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push("grant.returned");
            Ok(admission)
        }
    }

    fn configure_mock_hub(host: &ArtifactHost, hub_origin: &str, actor_fill: char, hub: &MockHub) {
        configure_mock_hub_with_plan_window(host, hub_origin, actor_fill, hub, 60_000);
    }

    /// ⏳️ [`configure_mock_hub`] with the admission plan's exchange window the grant source answers.
    fn configure_mock_hub_with_plan_window(host: &ArtifactHost, hub_origin: &str, actor_fill: char, hub: &MockHub, plan_window_ms: u64) {
        host.set_local_hub_credential(Arc::new(semio_framework_os_directory_client::LocalHubCredential::test(hub_origin, &format!("session.v1.{}.{}", actor_fill.to_string().repeat(32), actor_fill.to_string().repeat(64)))));
        host.set_hub_socket_grant_source(Arc::new(MockHubSocketGrantSource { hub_origin: hub_origin.into(), actor_id: format!("hub.v1.{}", actor_fill.to_string().repeat(64)), plan_window_ms, trace: hub.trace.clone() }));
    }

    async fn mock_frontier(ordinal: u64) -> RuntimeFrontierSummary {
        RuntimeFrontierSummary { document_id: ArtifactId("mock".to_string()), head_edit_ordinal: ordinal, head_edit_id: format!("edit-{ordinal}"), last_commit_seq: ordinal, chain_hash: [0u8; 32] }
    }

    async fn spawn_mock_hub() -> (std::net::SocketAddr, Arc<MockHub>) {
        spawn_mock_hub_with_session_gate(false).await
    }

    /// 🎭️ Framework document WS (C4b): client offers `semio.session.v1, <token>`;
    /// server selects the protocol token only (mirrors gateway `.protocols([SESSION_PROTOCOL_V1])`).
    ///
    /// Session actor is derived from the session credential (last segment of
    /// `session.v1.<selector>.<secret>` → `hub.v1.<secret>`), never from a
    /// caller-supplied `?actor=` query — mirrors the real hub binding.
    async fn spawn_mock_hub_with_session_gate(gated: bool) -> (std::net::SocketAddr, Arc<MockHub>) {
        ensure_demo_codec_registered().await;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let (broadcast, _rx) = tokio_broadcast::channel(256);
        let semio_hub = Arc::new(MockHub { log: Arc::new(Mutex::new(Vec::new())), broadcast, connections: AtomicUsize::new(0), session_gate: gated.then(|| Arc::new(tokio::sync::Semaphore::new(0))), trace: Arc::new(std::sync::Mutex::new(Vec::new())) });
        let accept_hub = semio_hub.clone();
        tokio::spawn(async move {
            loop {
                let Ok((stream, _)) = listener.accept().await else { break };
                let conn_hub = accept_hub.clone();
                conn_hub.record("tcp.accepted");
                tokio::spawn(async move {
                    let socket_actor = std::sync::Arc::new(std::sync::Mutex::new(None::<String>));
                    let socket_actor_for_hdr = socket_actor.clone();
                    let accepted = tokio_tungstenite::accept_hdr_async(
                        stream,
                        move |request: &tokio_tungstenite::tungstenite::handshake::server::Request,
                         mut response: tokio_tungstenite::tungstenite::handshake::server::Response|
                         -> Result<_, tokio_tungstenite::tungstenite::handshake::server::ErrorResponse> {
                            const SESSION_PROTOCOL_V1: &str = "semio.session.v1";
                            let offered = request
                                .headers()
                                .get("Sec-WebSocket-Protocol")
                                .and_then(|value| value.to_str().ok())
                                .unwrap_or("");
                            let selected = offered.split(',').next().map(str::trim).filter(|token| *token == SESSION_PROTOCOL_V1);
                            let Some(selected) = selected else {
                                return Err(tokio_tungstenite::tungstenite::http::Response::builder().status(400).body(None).expect("static refuse"));
                            };
                            response.headers_mut().insert("Sec-WebSocket-Protocol", selected.parse().expect("static protocol header"));
                            let actor = offered.split_once(", ").and_then(|(_, token)| {
                                let secret = token.rsplit('.').next()?;
                                (!secret.is_empty()).then(|| format!("hub.v1.{secret}"))
                            });
                            *socket_actor_for_hdr.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = actor;
                            Ok(response)
                        },
                    )
                    .await;
                    match accepted {
                        Ok(ws) => {
                            conn_hub.record("websocket.accepted");
                            let actor = match socket_actor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                                Some(actor) => {
                                    let _ = conn_hub.connections.fetch_add(1, Ordering::SeqCst);
                                    actor
                                }
                                None => {
                                    let n = conn_hub.connections.fetch_add(1, Ordering::SeqCst);
                                    let fill = if n == 0 { 'a' } else { 'b' };
                                    format!("hub.v1.{}", fill.to_string().repeat(64))
                                }
                            };
                            mock_hub_connection(ws, conn_hub, actor).await;
                        }
                        Err(_) => conn_hub.record("websocket.refused"),
                    }
                });
            }
        });
        (addr, semio_hub)
    }

    /// 🤝️ Expect Hello first.
    ///
    /// 👻️ Best-effort fan-out on the uncredited preview lane — this mock
    /// semio_hub doesn't track per-connection actor identity beyond `Hello`, so
    /// it stamps a fixed sentinel origin (fine for the round-trip test
    /// this drives, which only asserts the *other* peer receives it).
    async fn mock_hub_connection(ws: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>, semio_hub: Arc<MockHub>, actor: String) {
        let (mut write, mut read) = ws.split();
        let requested_ordinal = match read.next().await {
            Some(Ok(WsMessage::Binary(bytes))) => match decode_client_frame(&bytes).await {
                Ok((_, ClientFrame::SocketHelloV1 { frontier, .. })) => {
                    semio_hub.record("client.socket-hello");
                    frontier.map_or(0, |frontier| frontier.head_edit_ordinal)
                }
                Ok(_) => {
                    semio_hub.record("client.unexpected-first-frame");
                    return;
                }
                Err(_) => {
                    semio_hub.record("client.malformed-first-frame");
                    return;
                }
            },
            Some(Ok(_)) => {
                semio_hub.record("client.nonbinary-first-frame");
                return;
            }
            Some(Err(_)) => {
                semio_hub.record("client.first-frame-error");
                return;
            }
            None => {
                semio_hub.record("client.closed-before-hello");
                return;
            }
        };
        let (frontier, backlog) = {
            let log = semio_hub.log.lock().await;
            let ordinal = log.last().map_or(0, |(ordinal, _)| *ordinal);
            let backlog: Vec<MutationEnvelope> = log.iter().filter(|(ordinal, _)| *ordinal > requested_ordinal).map(|(_, envelope)| envelope.clone()).collect();
            (mock_frontier(ordinal).await, backlog)
        };
        if let Some(gate) = &semio_hub.session_gate {
            semio_hub.record("server.session-gate-waiting");
            gate.acquire().await.expect("mock session gate remains live").forget();
            semio_hub.record("server.session-gate-released");
        }
        let welcome = ServerFrame::Welcome { session_id: "mock-session".to_string(), resume_token: "mock-resume".to_string(), server_frontier: frontier.clone(), bootstrap: Bootstrap::Tail };
        if write.send(WsMessage::Binary(encode_server_frame(&welcome, Lane::Command).await.into())).await.is_err() {
            semio_hub.record("server.welcome-send-failed");
            return;
        }
        semio_hub.record("server.welcome-sent");
        let session = ServerFrame::Session { actor, color: 1 };
        if write.send(WsMessage::Binary(encode_server_frame(&session, Lane::Command).await.into())).await.is_err() {
            semio_hub.record("server.session-send-failed");
            return;
        }
        semio_hub.record("server.session-sent");
        let commands = ServerFrame::Commands { envelopes: backlog, origin: ActorId("semio_hub-backlog".into()), frontier: frontier.clone() };
        if write.send(WsMessage::Binary(encode_server_frame(&commands, Lane::Command).await.into())).await.is_err() {
            semio_hub.record("server.commands-send-failed");
            return;
        }
        semio_hub.record("server.commands-sent");
        let mut broadcast_rx = semio_hub.broadcast.subscribe();
        loop {
            tokio::select! {
                incoming = read.next() => {
                    match incoming {
                        Some(Ok(WsMessage::Binary(bytes))) => {
                            match decode_client_frame(&bytes).await {
                                Ok((_, ClientFrame::Commands { batch_id, envelopes })) => {
                                    semio_hub.record("client.commands");
                                    let mut assigned_frontier = frontier.clone();
                                    for envelope in envelopes {
                                        let (ordinal, origin) = {
                                            let mut log = semio_hub.log.lock().await;
                                            let next = log.last().map_or(0, |(ordinal, _)| *ordinal) + 1;
                                            log.push((next, envelope.clone()));
                                            (next, envelope.actor.clone())
                                        };
                                        assigned_frontier = mock_frontier(ordinal).await;
                                        let _ = semio_hub.broadcast.send(ServerFrame::Commands { envelopes: vec![envelope], origin, frontier: assigned_frontier.clone() });
                                    }
                                    let ack = ServerFrame::Ack { batch_id, stages: vec![AckStage::Applied { outcome: Box::new(ApplyOutcome::Accepted) }], frontier: assigned_frontier };
                                    let _ = write.send(WsMessage::Binary(encode_server_frame(&ack, Lane::Command).await.into())).await;
                                }
                                Ok((_, ClientFrame::PreviewPublish { key, seq, payload })) => {
                                    semio_hub.record("client.preview-publish");
                                    let _ = semio_hub.broadcast.send(ServerFrame::Preview { actor: ActorId("mock-semio_hub-peer".into()), key, seq, payload });
                                }
                                Ok((_, ClientFrame::Bye)) => semio_hub.record("client.bye"),
                                Ok(_) => semio_hub.record("client.other"),
                                Err(_) => semio_hub.record("client.malformed-frame"),
                            }
                        }
                        Some(Ok(WsMessage::Close(_))) | None | Some(Err(_)) => break,
                        Some(Ok(_)) => {}
                    }
                }
                frame = broadcast_rx.recv() => {
                    match frame {
                        Ok(frame) => {
                            if write.send(WsMessage::Binary(encode_server_frame(&frame, Lane::Command).await.into())).await.is_err() {
                                break;
                            }
                        }
                        Err(tokio_broadcast::error::RecvError::Lagged(_)) => {}
                        Err(tokio_broadcast::error::RecvError::Closed) => break,
                    }
                }
            }
        }
    }
    //#endregion 🔖️MockHub
//#endregion 🧪️Actor
