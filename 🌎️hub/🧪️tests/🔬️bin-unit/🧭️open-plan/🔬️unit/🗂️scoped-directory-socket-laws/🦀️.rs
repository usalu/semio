/// 🗂️ Laws of the scoped directory socket: in-flight commands, membership ledger, message matching, scope substitution and removal.

/// 🚦️ Using an authority never waits on another use of it: while a member's document command is in
/// flight — holding its principal, membership and space bindings, waiting for the document's writer —
/// the space owner still adds a newcomer at once and the member's own directory socket still delivers
/// that event at once. Every use held its bindings exclusively, so one slow write in a space stalled
/// every directory command and every directory delivery of that space behind it (C10: a member's Home
/// never listed a space's new documents for 60–120 s).
#[test]
fn a_command_in_flight_never_stalls_the_spaces_directory_commands_or_deliveries() {
    run_socket_test(|| async {
        let state = test_state().await;
        let owner = issue_test_session(&state, "fence-owner@example.com").await;
        let member = issue_test_session(&state, "fence-member@example.com").await;
        issue_test_session(&state, "fence-newcomer@example.com").await;
        let space_id = create_space_for_test(&state, &owner.user_id, "Fence space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space_id, "fence-member@example.com", DirectorySpaceRole::Author).await;
        announce_document_for_test(&state, &space_id, "fence-doc").await;
        let addr = spawn_server(state.clone()).await;
        let grant = issue_document_socket_grant_fixture(Path((space_id.clone(), "fence-doc".to_string())), bearer_headers(&member.token), State(state.clone())).await.expect("member document grant").0;
        let (mut document, _) = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{space_id}%2Ffence-doc/document/ws"), &member.token)).await.expect("member document socket");
        document.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("document hello");
        assert!(matches!(next_server_frame(&mut document).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut document).await, ServerFrame::Session { .. }));
        let directory_grant = issue_directory_socket_grant(bearer_headers(&member.token), State(state.clone()), Bytes::new()).await.expect("member directory grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let (mut directory, _) = connect_async(socket_request(&format!("ws://{addr}/directory/socket/v1?since={since}"), &directory_grant.grant)).await.expect("member directory socket");
        directory.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("directory hello");

        let scope = DocumentScope::new(space_id.clone(), "fence-doc");
        let writer = state.socket_binding_gates.document_write(&scope).lock_owned().await;
        let mut command = sample_envelope("fence-command", &WireArtifactId("fence-doc".into())).await;
        command.actor = ActorId(grant.actor_id.clone());
        document.send(client_binary(&ClientFrame::Commands { batch_id: 31, envelopes: vec![command] }, Lane::Command).await).await.expect("in-flight command");
        let membership = SocketBindingKeyV1::Membership { user_id: member.user_id.clone(), space_id: space_id.clone() };
        let admitted = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            while state.socket_binding_gates.gate(membership.clone()).try_write_owned().is_ok() {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            }
        })
        .await;
        assert!(admitted.is_ok(), "the member's command is in flight, holding its membership");

        let upserted_at = std::time::Instant::now();
        let upsert = DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "fence-newcomer@example.com".into(), role: DirectorySpaceRole::Spectator };
        let response = post_directory_command_for_test(addr, &owner.token, "0123456789abcdef0123456789abcdef", upsert).await;
        assert_eq!(response.status, 202, "the owner's command is not fenced by the member's write: {}", String::from_utf8_lossy(&response.body));
        assert!(matches!(next_directory_message(&mut directory).await, DirectoryStreamMessage::Event { event } if matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { space_id: upserted, .. } if *upserted == space_id)), "the member's directory socket delivers the space's event while its own command is in flight");
        assert!(upserted_at.elapsed() < std::time::Duration::from_secs(2), "neither waited on the in-flight write: {:?}", upserted_at.elapsed());

        drop(writer);
        match next_server_frame(&mut document).await {
            ServerFrame::Ack { batch_id: 31, stages, .. } => assert!(matches!(stages.last(), Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted)), "{stages:?}"),
            other => panic!("expected the in-flight command's Ack, got {other:?}"),
        }
    });
}

#[tokio::test]
async fn scoped_directory_socket_ledger_indexes_and_invalidates_exact_membership() {
    let ledger = SocketGrantLedgerV1::default();
    let scope = DocumentScope::new("space-a", "document-a");
    let audience = SocketAudienceV1::DirectoryScoped(scope.clone());
    let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Spectator), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-a".into() };
    let capability = SocketGrantCapability::mint().expect("scoped capability");
    ledger.issue(&capability, audience.clone(), "hub.v1.scoped".into(), subject.clone(), 1, 9_000).expect("scoped issue");
    let pending = ledger.pending(&capability, &audience, 2).expect("pending scoped grant");
    assert_eq!(
        pending.bindings(),
        vec![
            SocketBindingKeyV1::User("user-a".into()),
            SocketBindingKeyV1::Session("session-a".into()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: "space-a".into() },
            SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() },
        ]
    );
    let consumed = ledger.consume(&pending, 3).expect("consume scoped grant");
    let (live_id, notify) = ledger.register_live(&consumed).expect("register scoped live lease");
    assert!(ledger.is_live(&consumed, &live_id));
    ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() });
    assert!(!ledger.is_live(&consumed, &live_id));
    tokio::time::timeout(std::time::Duration::from_millis(50), notify.notified()).await.expect("membership invalidation notifies once");

    let later = SocketGrantCapability::mint().expect("later scoped capability");
    ledger.issue(&later, audience.clone(), "hub.v1.later".into(), subject, 4, 9_000).expect("later issue");
    ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() });
    assert!(ledger.pending(&later, &audience, 5).is_err(), "membership invalidation also removes pending grants");
}

#[test]
fn scoped_directory_socket_message_matching_is_body_exact_and_removal_private() {
    let scope = DocumentScope::new("space-a", "document-a");
    let event = |body: os_directory::DirectoryEventBody| DirectoryStreamMessage::Event {
        event: Box::new(DirectoryEvent {
            seq: 1,
            id: "event-a".into(),
            hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:test".into() },
            space_id: Some("space-a".into()),
            user_id: None,
            body,
            recorded_at_ms: 1,
        }),
    };
    assert!(directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::DocumentAnnounced { descriptor: document_descriptor_for_test("space-a", "document-a") })));
    assert!(!directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::DocumentAnnounced { descriptor: document_descriptor_for_test("space-a", "document-b") })));
    assert!(!directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::MemberRemoved { space_id: "space-a".into(), user_id: "user-a".into() })));
    assert!(!directory_message_matches_scope(&scope, &DirectoryStreamMessage::Heartbeat { head_seq: 99 }));
    assert!(directory_message_matches_scope(&scope, &DirectoryStreamMessage::Presence { space_id: "space-a".into(), document_id: "document-a".into(), actors: Vec::new() }));
    assert!(!directory_message_matches_scope(&scope, &DirectoryStreamMessage::Presence { space_id: "space-a".into(), document_id: "document-b".into(), actors: Vec::new() }));
}

#[test]
fn scoped_directory_socket_route_rejects_scope_substitution_and_rest_removal_closes_without_event() {
    run_socket_test(|| async {
        let state = test_state().await;
        let owner_token = seed_author_token(&state).await;
        let member = issue_test_session(&state, "scoped-member@example.com").await;
        upsert_member_for_test(&state, STUDIO, "scoped-member@example.com", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, STUDIO, "scoped-document-a").await;
        announce_document_for_test(&state, STUDIO, "scoped-document-b").await;
        let unaffected = issue_test_session(&state, "scoped-unaffected@example.com").await;
        let unaffected_space = create_space_for_test(&state, &unaffected.user_id, "Scoped Unaffected", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let unaffected_document = artifact_document_id_for_test("scoped-unaffected-document");
        seed_genesis_for_document_for_test(&state, &unaffected.token, &unaffected_space, &unaffected_document, "scoped-unaffected-document").await;
        let addr = spawn_server(state.clone()).await;
        let authorization = format!("Bearer {}", member.token);
        let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-document-a/socket-grants");
        let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(issued.status, 200);
        let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("scoped grant JSON");
        let grant = receipt["grant"].as_str().expect("scoped grant");
        let substituted = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-document-b/socket/v1?since=0");
        let error = connect_async(socket_request(&substituted, grant)).await.expect_err("scope substitution rejected before upgrade");
        assert!(matches!(error, tokio_tungstenite::tungstenite::Error::Http(response) if response.status() == StatusCode::UNAUTHORIZED));

        let since = state.directory.head_seq().await.expect("directory head");
        let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-document-a/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, grant)).await.expect("exact scoped socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("scoped socket hello");
        let unaffected_authorization = format!("Bearer {}", unaffected.token);
        let unaffected_issue_path = format!("/directory/spaces/{unaffected_space}/documents/{unaffected_document}/socket-grants");
        let unaffected_issued = raw_http_request(addr, "POST", &unaffected_issue_path, &[("Authorization", &unaffected_authorization)], &[]).await;
        assert_eq!(unaffected_issued.status, 200);
        let unaffected_receipt: serde_json::Value = serde_json::from_slice(&unaffected_issued.body).expect("unaffected scoped grant JSON");
        let unaffected_grant = unaffected_receipt["grant"].as_str().expect("unaffected scoped grant");
        let unaffected_url = format!("ws://{addr}/directory/spaces/{unaffected_space}/documents/{unaffected_document}/socket/v1?since={since}");
        let (mut unaffected_socket, _) = connect_async(socket_request(&unaffected_url, unaffected_grant)).await.expect("unaffected scoped socket");
        unaffected_socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("unaffected scoped hello");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let pending = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(pending.status, 200);
        let pending_receipt: serde_json::Value = serde_json::from_slice(&pending.body).expect("pending scoped grant JSON");
        let pending_grant = pending_receipt["grant"].as_str().expect("pending scoped grant").to_string();
        announce_document_for_test(&state, STUDIO, "scoped-document-c").await;
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), socket.next()).await.is_err(), "same-space foreign document never serializes");

        let command = DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id: member.user_id.clone() };
        let removed = post_directory_command_for_test(addr, &owner_token, "5c0ded0c0ffee0bad0f00d0dead0beef", command).await;
        assert_eq!(removed.status, 202);
        let removed_body = String::from_utf8(removed.body).expect("remove response UTF-8");
        assert!(removed_body.contains("member.removed"));
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "durable removal invalidates the scoped lease without exposing its event");
        let unaffected_published = publish_checkpoint_for_test(&state, &unaffected_space, &unaffected_document).await;
        let unaffected_delivered = next_directory_message(&mut unaffected_socket).await;
        assert!(
            matches!(
                &unaffected_delivered,
                DirectoryStreamMessage::Event { event }
                    if matches!(event.body, os_directory::DirectoryEventBody::ArtifactCheckpointPublished { ref checkpoint }
                        if checkpoint.scope == DocumentScope::new(&unaffected_space, &unaffected_document) && checkpoint.checkpoint_id == unaffected_published.checkpoint_id)
            ),
            "another user's exact scoped subscription remains live: {unaffected_delivered:?}"
        );

        for stale in [grant.to_string(), pending_grant] {
            let error = connect_async(socket_request(&url, &stale)).await.expect_err("consumed or pending pre-removal grant remains invalid");
            assert!(matches!(error, tokio_tungstenite::tungstenite::Error::Http(response) if response.status() == StatusCode::UNAUTHORIZED));
        }

        let denied = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(denied.status, 401, "removed member cannot reacquire the scoped grant");
    });
}

#[test]
fn scoped_directory_socket_admin_removal_uses_the_same_membership_fence() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let mut admin_headers = authorize_test_admin(&mut state, "scoped-admin@example.com").await;
        admin_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let member = issue_test_session(&state, "scoped-admin-target@example.com").await;
        upsert_member_for_test(&state, STUDIO, "scoped-admin-target@example.com", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, STUDIO, "scoped-admin-document").await;
        let addr = spawn_server(state.clone()).await;
        let authorization = format!("Bearer {}", member.token);
        let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-admin-document/socket-grants");
        let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(issued.status, 200);
        let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("scoped grant JSON");
        let grant = receipt["grant"].as_str().expect("scoped grant");
        let since = state.directory.head_seq().await.expect("directory head");
        let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-admin-document/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, grant)).await.expect("admin target scoped socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("scoped socket hello");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let intent = AdminIntentV1::RemoveSpaceMember { request_id: "request:scoped-admin-removal".into(), space_id: STUDIO.into(), user_id: member.user_id };
        let body = Bytes::from(semio_framework_pack_json::to_json_string(&intent));
        let (status, receipt) = admin_intents(admin_headers, loopback_peer(), State(state), body).await.expect("admin removal response");
        assert_eq!(status, StatusCode::OK);
        assert_eq!(receipt.0.state, AdminIntentStateV1::Succeeded);
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "admin removal uses the same no-event membership fence");
    });
}
