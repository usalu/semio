/// 🔌️ Socket delivery, visibility and public projection: the global socket delivery and revocation, the revoke after admission, the membership visibility and the public projection helpers.

#[test]
fn directory_global_socket_delivery_and_revocation_share_one_transient_authority_order() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🌐️directory-message-authority-v1/🔣️.json")).expect("message authority fixture");
        let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("global directory state open deadline");
        let gate = Arc::new(TestLiveGate::default());
        gate.socket_directory_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        state.live_gate = Some(gate.clone());
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["vectors"].as_array().unwrap().iter().enumerate() {
            let owner_email = format!("global-owner-{index}@example.test");
            let recipient_email = format!("global-recipient-{index}@example.test");
            let owner = issue_test_session(&state, &owner_email).await;
            let recipient = issue_test_session(&state, &recipient_email).await;
            let space_a = create_space_for_test(&state, &owner.user_id, "A", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let space_b = create_space_for_test(&state, &owner.user_id, "B", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            for space in [&space_a, &space_b] {
                upsert_member_for_test(&state, space, &recipient_email, DirectorySpaceRole::Spectator).await;
            }
            let receipt = issue_directory_socket_grant(bearer_headers(&recipient.token), State(state.clone()), Bytes::new()).await.expect("global grant").0;
            let since = state.directory.head_seq().await.expect("global directory head");
            let url = format!("ws://{addr}/directory/socket/v1?since={since}");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("global directory socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("global socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_directory_admitted.acquire()).await.expect("global admission deadline").expect("global admission").forget();
            gate.socket_directory_release.add_permits(1);
            let delivery_first = row["first"] == "send";
            *gate.socket_global_send_pause.lock().expect("global pause lock") = Some((recipient.user_id.clone(), if delivery_first { 2 } else { 1 }));
            let event_a = state
                .directory_service
                .execute(DirectoryActor { kind: DirectoryActorKind::System, id: "system:global-message-authority".into() }, DirectoryCommand::RenameSpace { space_id: space_a.clone(), name: format!("A-{index}") })
                .await
                .expect("durable A event")
                .0
                .into_iter()
                .next()
                .expect("A event");
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_global_send_admitted.acquire()).await.expect("message pause deadline").expect("message pause").forget();
            let membership = row["revocation"] == "membership";
            let mut revoke = tokio::spawn({
                let state = state.clone();
                let token = if membership { owner.token.clone() } else { recipient.token.clone() };
                let command = DirectoryCommand::RemoveMember { space_id: space_a.clone(), user_id: recipient.user_id.clone() };
                async move {
                    if membership {
                        post_directory_command_for_test(addr, &token, "c00102030405060708090a0b0c0d0e0f", command).await.status
                    } else {
                        post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await.as_u16()
                    }
                }
            });
            let attempted = if membership { &gate.directory_command_attempted } else { &gate.socket_session_revoke_attempted };
            tokio::time::timeout(std::time::Duration::from_secs(5), attempted.acquire()).await.expect("revocation fence attempt deadline").expect("revocation fence attempt").forget();
            if delivery_first {
                assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted delivery must own the same authority keys as revocation: {}", row["name"]);
            } else {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("winning revocation deadline").expect("winning revocation"), if membership { 202 } else { 204 });
            }
            *gate.socket_global_send_pause.lock().expect("clear global pause") = None;
            gate.socket_global_send_release.add_permits(1);
            if row["a"] == true {
                assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if *event == event_a), "one exact A event wins before revocation");
            }
            if delivery_first {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing revocation deadline").expect("trailing revocation"), if membership { 202 } else { 204 });
            }
            if row["b"] == true {
                assert_eq!(
                    next_directory_message(&mut socket).await,
                    DirectoryStreamMessage::AccessChanged { space_id: space_a.clone(), change: os_directory::DirectoryAccessChange::Revoked },
                    "the removed reader is told its access to A moved before any later frame: {}",
                    row["name"]
                );
                let event_b = state
                    .directory_service
                    .execute(DirectoryActor { kind: DirectoryActorKind::System, id: "system:global-message-authority".into() }, DirectoryCommand::RenameSpace { space_id: space_b.clone(), name: format!("B-{index}") })
                    .await
                    .expect("durable B event")
                    .0
                    .into_iter()
                    .next()
                    .expect("B event");
                assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if *event == event_b), "removed A membership neither leaks A nor closes unrelated B");
                socket.close(None).await.expect("close unaffected global socket");
            } else {
                assert_eq!(u64::from(next_close_code(&mut socket, false).await), row["close"].as_u64().unwrap(), "revoked principal closes without a later message");
            }
        }
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn socket_directory_revoke_after_admission_suppresses_replay_without_deadlock() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        gate.socket_directory_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let token = seed_author_token(&state).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&token), State(state.clone()), Bytes::new()).await.expect("issue directory grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/socket/v1?since=0");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("directory socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_directory_admitted.acquire()).await.expect("directory admission deadline").expect("directory admission");
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), post_session_sign_out(bearer_headers(&token), State(state), Bytes::new())).await.expect("bounded revoke"), StatusCode::NO_CONTENT,);
        gate.socket_directory_release.add_permits(1);
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "no replay text crosses a winning revoke");
    });
}

#[tokio::test]
async fn socket_directory_visibility_requires_membership_even_for_public_spaces() {
    let state = test_state().await;
    let token = seed_author_token(&state).await;
    let capability = SessionCapability::parse(&token).expect("session capability");
    let session = state.directory.authenticate_session(&capability).await.expect("authenticate session").expect("active session");
    let other = issue_test_session(&state, "public-owner@example.com").await;
    let public_space = create_space_for_test(&state, &other.user_id, "Public Other", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
    let event = state.directory.events_since(0, 100).await.expect("directory events").into_iter().find(|event| event.space_id.as_deref() == Some(public_space.as_str())).expect("public-space event");
    let audience = SocketAudienceV1::Directory { auth_session_id: session.id.clone(), authorization_generation: session.authorization_generation };
    let record = SocketGrantRecordV1 {
        selector: "visibility".into(),
        key: SocketGrantKeyV1::Capability([0; 32]),
        audience,
        actor_id: "hub.v1.visibility".into(),
        subject: SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: None, expires_at_ms: session.expires_at, session_kind: session.session_kind, device_instance_id: session.device_instance_id },
        document_plan: None,
        issued_at_ms: session.issued_at,
        expires_at_ms: session.expires_at,
        state: SocketGrantStateV1::Consumed,
    };
    assert_eq!(socket_directory_membership_visibility(&state, &record, &DirectoryStreamMessage::Event { event: Box::new(event) }).await, SocketBindingValidityV1::Unauthorized);
}

fn assert_public_projection_has_no_private_keys(value: &serde_json::Value) {
    const FORBIDDEN: &[&str] = &[
        "ownerUserId",
        "role",
        "activeConnections",
        "connections",
        "presence",
        "members",
        "invites",
        "email",
        "userId",
        "displayName",
        "actor",
        "hlc",
        "cursor",
        "headSeq",
        "commitSeq",
        "epoch",
        "bootstrapVersion",
        "bootstrapFrontier",
        "bootstrapSnapshotHash",
        "checkpointId",
        "storageKey",
    ];
    match value {
        serde_json::Value::Object(entries) => {
            for (key, value) in entries {
                assert!(!FORBIDDEN.contains(&key.as_str()), "public projection disclosed forbidden key {key}");
                assert_public_projection_has_no_private_keys(value);
            }
        }
        serde_json::Value::Array(entries) => entries.iter().for_each(assert_public_projection_has_no_private_keys),
        _ => {}
    }
}

/// 🪞️ Strips the three fields `DirectorySpaceAdministrationPageV1` binds to its individual caller —
/// `sessionBindingSha256`, `authorizationGeneration` and `receiptSha256` — so two readers of the same
/// public space can be compared on the projection they are meant to share.
fn public_projection_without_caller_binding(value: &serde_json::Value) -> serde_json::Value {
    let mut stripped = value.clone();
    let entries = stripped.as_object_mut().expect("administration page object");
    for key in ["sessionBindingSha256", "authorizationGeneration", "receiptSha256"] {
        assert!(entries.remove(key).is_some(), "administration page carries its caller binding field {key}");
    }
    stripped
}
