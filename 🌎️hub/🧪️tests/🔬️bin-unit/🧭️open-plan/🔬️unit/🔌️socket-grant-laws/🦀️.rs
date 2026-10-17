/// 🔌️ Laws of socket grant routes: exact replay, revoke linearization, lag and broadcast fences and directory hello.

#[test]
fn socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live() {
    run_socket_test(|| async {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-a").await;
        announce_document_for_test(&state, STUDIO, "socket-b").await;
        let nonmember = issue_test_session(&state, "socket-nonmember@example.com").await;
        let unauthorized_existing = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&nonmember.token), State(state.clone())).await.err();
        let unauthorized_missing = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-missing".to_string())), bearer_headers(&nonmember.token), State(state.clone())).await.err();
        assert_eq!(unauthorized_existing, Some(StatusCode::UNAUTHORIZED));
        assert_eq!(unauthorized_missing, unauthorized_existing, "unauthorized callers cannot enumerate descriptor existence");
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue document socket grant").0;
        assert_eq!(receipt.schema, "semio.hub.document-socket-grant/v1");
        assert_eq!(receipt.protocol, SESSION_PROTOCOL_V1);
        assert!(receipt.actor_id.starts_with("hub.v1."));

        let addr = spawn_server(state.clone()).await;
        let rejected = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-b/document/ws"), &token)).await.expect_err("cross-document grant rejected");
        assert!(matches!(rejected, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));

        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-a/document/ws");
        let (mut socket, response) = connect_async(document_socket_request(&url, &token)).await.expect("upgrade socket grant");
        assert_eq!(response.headers().get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL).and_then(|value| value.to_str().ok()), Some(SESSION_PROTOCOL_V1));
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { actor, .. } if actor == receipt.actor_id));

        let mut forged = sample_envelope("forged-actor", &WireArtifactId("socket-a".into())).await;
        forged.actor = ActorId("client-selected-forgery".into());
        socket.send(client_binary(&ClientFrame::Commands { batch_id: 77, envelopes: vec![forged] }, Lane::Command).await).await.expect("forged command");
        match next_server_frame(&mut socket).await {
            ServerFrame::Ack { batch_id: 77, stages, .. } => match &stages[0] {
                AckStage::Applied { outcome } => match outcome.as_ref() {
                    ApplyOutcome::Rejected { reason, .. } => {
                        assert_eq!(reason, "socket subject actor mismatch");
                    }
                    other => panic!("forged actor was not rejected: {other:?}"),
                },
                other => panic!("unexpected forged actor stage: {other:?}"),
            },
            other => panic!("expected forged actor ack, got {other:?}"),
        }

        let mut committed = sample_envelope("committed-once", &WireArtifactId("socket-a".into())).await;
        committed.actor = ActorId(receipt.actor_id.clone());
        let accepted = |frame: &ServerFrame, batch: u64| matches!(frame, ServerFrame::Ack { batch_id, stages, .. } if *batch_id == batch && matches!(stages.last(), Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted)));
        socket.send(client_binary(&ClientFrame::Commands { batch_id: 78, envelopes: vec![committed.clone()] }, Lane::Command).await).await.expect("first commit");
        let ack = next_server_frame(&mut socket).await;
        assert!(accepted(&ack, 78), "{ack:?}");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Commands { envelopes, .. } if envelopes[0].mutation_id == committed.mutation_id), "a first commit is relayed");
        let resend_receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("resend grant").0;
        assert_eq!(resend_receipt.actor_id, receipt.actor_id);
        let (mut resend, _) = connect_async(document_socket_request(&url, &token)).await.expect("resend socket");
        resend.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("resend hello");
        assert!(matches!(next_server_frame(&mut resend).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut resend).await, ServerFrame::Commands { envelopes, origin, .. } if envelopes[0].mutation_id == committed.mutation_id && origin.0 == HUB_CATCH_UP_ORIGIN), "the reconnect is caught up with the committed edit, under the hub's declared catch-up origin");
        assert!(matches!(next_server_frame(&mut resend).await, ServerFrame::Session { .. }));
        resend.send(client_binary(&ClientFrame::Commands { batch_id: 79, envelopes: vec![committed.clone()] }, Lane::Command).await).await.expect("resend after reconnect");
        let ack = next_server_frame(&mut resend).await;
        assert!(accepted(&ack, 79), "an idempotent resend after reconnect is acknowledged: {ack:?}");
        resend.send(client_binary(&ClientFrame::PreviewPublish { key: "relay-fence".into(), seq: 79, payload: vec![] }, Lane::Preview).await).await.expect("relay fence");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq: 79, .. } if key == "relay-fence"), "an idempotent resend is never relayed to peers as a new commit");
        let committed_frontier = state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, "socket-a"))).await.expect("document handle").frontier().await.expect("frontier");
        assert_eq!(committed_frontier.commit_seq, 1, "the resend did not commit twice");
        resend.close(None).await.expect("close resend socket");

        let legacy_receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue legacy-carrier rejection grant").0;
        let (mut legacy, _) = connect_async(document_socket_request(&url, &token)).await.expect("legacy rejection socket");
        legacy.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("initial socket hello");
        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Commands { origin, .. } if origin.0 == HUB_CATCH_UP_ORIGIN && origin.0 != legacy_receipt.actor_id), "a joiner behind the head is caught up before Session, never under its own actor");
        let echo_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧫️fixtures/document-echo-suppression-v1/🔣️.json")).expect("echo suppression fixture");
        assert_eq!(echo_fixture["hubCatchUpOrigin"].as_str(), Some(HUB_CATCH_UP_ORIGIN), "the hub's catch-up origin is the fixture's declared one");
        assert_eq!(semio_hub_testing::hub_source("🏗️bootstrap/🔌️document-socket/🦀️.rs").matches("ActorId(HUB_CATCH_UP_ORIGIN.into())").count(), 2, "the hello tail and the FrontierAdvertise catch-up both carry the declared catch-up origin");
        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Session { .. }));
        legacy.send(WsMessage::Binary(vec![0, 0].into())).await.expect("legacy tag-zero frame");
        assert_eq!(next_close_code(&mut legacy, false).await, 4401, "v1 rejects the legacy actor/token carrier after upgrade");

        let replay = connect_async(document_socket_request(&url, &token)).await.expect_err("consumed grant replay rejected");
        assert!(matches!(replay, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));
        let pending = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue pending grant").0;
        assert_eq!(pending.actor_id, receipt.actor_id, "session-derived actor is stable across grants");

        assert_eq!(post_session_sign_out(bearer_headers(&token), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT);
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "successful durable revoke immediately invalidates a live socket");
        let revoked_pending = connect_async(document_socket_request(&url, &token)).await.expect_err("pending grant invalidated by revoke");
        assert!(matches!(revoked_pending, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));
    });
}

#[test]
fn socket_grant_revoke_and_welcome_have_a_bounded_binding_linearization() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-linearized").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-linearized".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-linearized/document/ws");
        let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_before_welcome.acquire()).await.expect("pre-welcome gate deadline").expect("pre-welcome gate");
        let mut revoke = tokio::spawn({
            let state = state.clone();
            let token = token.clone();
            async move { post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await }
        });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "revoke waits while Welcome owns the binding linearization");
        gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }), "Welcome linearizes before the waiting revoke");
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_after_welcome.acquire()).await.expect("post-Welcome boundary deadline").expect("post-Welcome boundary");
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("bounded revoke completion").expect("revoke task"), StatusCode::NO_CONTENT);
        gate.socket_bootstrap_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "a revoke winning after Welcome suppresses bootstrap and Session authority");
    });
}

#[test]
fn socket_grant_revoke_before_lag_authorization_reads_no_private_control() {
    run_socket_test(|| async {
        let mut state = test_state_with_capacity(1024, 1).await;
        let live_gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(live_gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-lag-revoke").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-lag-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-lag-revoke/document/ws");
        let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
        live_gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
        live_gate.socket_bootstrap_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
        let fanout = state.fanout_for(&document_scope_key_v1(&DocumentScope::new(STUDIO, "socket-lag-revoke")));
        fanout.send(ServerFrame::Presence { peers: vec![b"first".to_vec()] }).expect("first fanout");
        fanout.send(ServerFrame::Presence { peers: vec![b"second".to_vec()] }).expect("second fanout");
        live_gate.document_release.add_permits(1);
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_lag_received.acquire()).await.expect("lag boundary deadline").expect("lag boundary");
        assert_eq!(post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await, StatusCode::NO_CONTENT);
        live_gate.socket_lag_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "revoked lag path discloses no control frame");
        assert_eq!(live_gate.socket_rebootstrap_read.available_permits(), 0, "revoked lag path never enters the private checkpoint/control read");
    });
}

#[test]
fn socket_grant_revoke_before_broadcast_authorization_suppresses_frame() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let live_gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(live_gate.clone());
        live_gate.socket_broadcast_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-broadcast-revoke").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-broadcast-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-broadcast-revoke/document/ws");
        let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
        live_gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
        live_gate.socket_bootstrap_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
        let fanout = state.fanout_for(&document_scope_key_v1(&DocumentScope::new(STUDIO, "socket-broadcast-revoke")));
        fanout.send(ServerFrame::Presence { peers: vec![b"private-presence".to_vec()] }).expect("fanout");
        live_gate.document_release.add_permits(1);
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_broadcast_received.acquire()).await.expect("broadcast boundary deadline").expect("broadcast boundary");
        assert_eq!(post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await, StatusCode::NO_CONTENT);
        live_gate.socket_broadcast_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "a broadcast received before a winning revoke is never disclosed afterward");
    });
}

#[test]
fn socket_grant_directory_route_uses_credential_free_hello_and_revokes_live() {
    run_socket_test(|| async {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&token), State(state.clone()), Bytes::new()).await.expect("issue directory socket grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("directory socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("credential-free hello");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#directory-socket-law".into() }, DirectoryCommand::RenameSpace { space_id: STUDIO.into(), name: "Socket Grant Studio".into() })
            .await
            .expect("member-visible directory event");
        assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if event.space_id.as_deref() == Some(STUDIO)));
        assert_eq!(post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await, StatusCode::NO_CONTENT);
        assert_eq!(next_close_code(&mut socket, false).await, 4401);
    });
}
