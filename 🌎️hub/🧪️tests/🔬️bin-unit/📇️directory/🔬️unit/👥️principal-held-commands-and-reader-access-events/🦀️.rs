/// 👥️ Principal-held commands and reader access events: the admin directory commands that hold the exact principal, the global message bindings and the membership events that tell a reader its access changed.

#[test]
fn admin_directory_commands_hold_exact_principal_without_confusing_space_role() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).expect("admin authority fixture");
        let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("admin authority state open deadline");
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let email = "directory-admin-authority@example.test";
        let _headers = authorize_test_admin(&mut state, email).await;
        let owner = issue_test_session(&state, "directory-admin-target-owner@example.test").await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["vectors"].as_array().unwrap().iter().enumerate() {
            let admin = issue_test_session(&state, email).await;
            let headers = bearer_headers(&admin.token);
            let principal = authenticate_admin_principal(&state, &headers, None).await.expect("configured administrator");
            let space = create_space_for_test(&state, &owner.user_id, "Admin Target", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            upsert_member_for_test(&state, &space, email, DirectorySpaceRole::Author).await;
            let request_id = format!("request:admin-directory-authority-{index}");
            let name = format!("Admin Mutation {index}");
            let create = row["command"] == "create";
            let command_first = row["first"] == "command";
            let intent = if create {
                AdminIntentV1::CreateSpace { request_id: request_id.clone(), name: name.clone(), space_kind: os_directory::DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }
            } else {
                AdminIntentV1::RenameSpace { request_id: request_id.clone(), space_id: space.clone(), name: name.clone() }
            };
            let target = if create { admin_create_space_id(&request_id) } else { space.clone() };
            let before = state.directory.head_seq().await.expect("admin head before intent");
            *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), command_first));
            let mut command = tokio::spawn({
                let authorization = format!("Bearer {}", admin.token);
                let body = semio_framework_pack_json::to_json_string(&intent);
                async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("admin pause deadline").expect("admin pause").forget();
            let mut trailing_revoke = None;
            if row["transition"] == "session" {
                let mut revoke = tokio::spawn({
                    let state = state.clone();
                    let token = admin.token.clone();
                    async move { post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await }
                });
                tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_session_revoke_attempted.acquire()).await.expect("admin revoke attempt deadline").expect("admin revoke attempt").forget();
                if command_first {
                    assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted admin command retains the exact session");
                    assert!(state.socket_binding_gates.gate(SocketBindingKeyV1::User(principal.user_id.clone())).try_write_owned().is_err(), "admitted admin command also retains user-wide revocation authority");
                    trailing_revoke = Some(revoke);
                } else {
                    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("admin revoke deadline").expect("admin revoke"), StatusCode::NO_CONTENT);
                }
            } else {
                upsert_member_for_test(&state, &space, email, DirectorySpaceRole::Spectator).await;
                assert_eq!(state.directory.get_role(&space, &admin.user_id).await.unwrap(), Some(SpaceRole::Spectator));
            }
            *gate.directory_command_pause_user.lock().unwrap() = None;
            gate.directory_command_release.add_permits(1);
            let response = tokio::time::timeout(std::time::Duration::from_secs(5), &mut command).await.expect("admin intent completion deadline").expect("admin intent task");
            assert_eq!(response.status, 200);
            let receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&response.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("admin authority receipt");
            let expected = if row["mutated"] == true { AdminIntentStateV1::Succeeded } else { AdminIntentStateV1::Cancelled };
            assert_eq!(receipt.state, expected, "exact principal, not ordinary role, decides {}", row["name"]);
            if let Some(revoke) = trailing_revoke {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing admin revoke deadline").expect("trailing admin revoke"), StatusCode::NO_CONTENT);
            }
            let events = state.directory.events_since(before, 50).await.expect("admin authority durable events");
            let own_events: Vec<_> = events.iter().filter(|event| event.actor == principal.event_actor()).collect();
            assert_eq!(
                own_events.len(),
                if row["mutated"] == true {
                    if create {
                        2
                    } else {
                        1
                    }
                } else {
                    0
                },
                "no mutation escapes a revoked principal"
            );
            let persisted = state.directory.get_space(&target).await.expect("admin target projection");
            assert_eq!(persisted.as_ref().is_some_and(|space| space.name == name), row["mutated"].as_bool().unwrap());
            if row["mutated"] == false {
                assert!(receipt.event_seq_first.is_none() && receipt.event_seq_last.is_none());
            }
        }

        let self_session = issue_test_session(&state, email).await;
        let authorization = format!("Bearer {}", self_session.token);
        let body = semio_framework_pack_json::to_json_string(&AdminIntentV1::RevokeUserSessions { request_id: "request:admin-directory-self-revoke".into(), user_id: self_session.user_id.clone(), reason_code: "test-self-revoke".into() });
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()))
            .await
            .expect("self-revocation cannot nest an administrator User fence");
        assert_eq!(response.status, 200);
        let receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&response.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("self-revocation receipt");
        assert_eq!(receipt.state, AdminIntentStateV1::Succeeded);
        let capability = SessionCapability::parse(&self_session.token).unwrap();
        assert!(state.directory.authenticate_session(&capability).await.unwrap().is_none());
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn directory_global_message_bindings_decode_wire_without_indexing_unrelated_memberships() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🌐️directory-message-authority-v1/🔣️.json")).expect("message authority fixture");
    let ledger = SocketGrantLedgerV1::default();
    let capability = SocketGrantCapability::mint().expect("global capability");
    let audience = SocketAudienceV1::Directory { auth_session_id: "session".into(), authorization_generation: 1 };
    let subject = SocketSubjectV1::Session { session_id: "session".into(), user_id: "recipient".into(), authorization_generation: 1, role: None, expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-recipient".into() };
    ledger.issue(&capability, audience.clone(), "hub.v1.global".into(), subject, 1, 9_000).expect("global issue");
    let pending = ledger.pending(&capability, &audience, 2).expect("pending global");
    let record = ledger.consume(&pending, 3).expect("consumed global");
    let (live_id, _) = ledger.register_live(&record).expect("global lease");
    let principal_bindings = vec![SocketBindingKeyV1::User("recipient".into()), SocketBindingKeyV1::Session("session".into())];
    for row in fixture["messages"].as_array().unwrap() {
        let message: DirectoryStreamMessage = semio_framework_pack_json::from_json_str(row["messageJson"].as_str().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("wire message {}: {error:?}", row["name"]));
        assert_eq!(directory_stream_message_space(&message), row["spaceId"].as_str(), "wire-derived space");
        let bindings = directory_message_bindings(&record, &message);
        assert_eq!(bindings.len() as u64, row["keys"].as_u64().unwrap(), "complete transient union");
        let mut expected = principal_bindings.clone();
        if let Some(space_id) = row["spaceId"].as_str() {
            expected.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_owned() });
            expected.push(SocketBindingKeyV1::Membership { user_id: "recipient".into(), space_id: space_id.to_owned() });
        }
        assert_eq!(bindings, expected, "sorted exact principal and recipient membership union");
        assert_eq!(record.bindings(), principal_bindings, "transient keys never become permanent global indices");
        let mut scoped = record.clone();
        scoped.audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new("scoped", "doc"));
        assert_eq!(directory_message_bindings(&scoped, &message), scoped.bindings(), "scoped audiences do not borrow unrelated scopes");
    }
    ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "recipient".into(), space_id: "space-a".into() });
    ledger.invalidate_binding(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: "space-a".into() });
    assert!(ledger.is_live(&record, &live_id), "space A revocation cannot invalidate a global lease for B");
    ledger.invalidate_binding(SocketBindingKeyV1::Session("session".into()));
    assert!(!ledger.is_live(&record, &live_id), "principal revocation remains terminal");
}

/// 🔑️ Law over the shared `🔑️directory-access-changed-v1` fixture: exactly the membership events naming the reader owe it
/// an `access-changed` frame — a grant or redemption only for a space this socket never delivered the creation of, every
/// removal — and each frame borrows no space authority (it is addressed to its reader alone).
#[test]
fn a_membership_event_naming_the_reader_owes_it_exactly_the_fixture_access_change() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔑️access-changed-v1/🔣️.json")).expect("access-changed fixture");
    let reader = fixture["readerUserId"].as_str().expect("fixture reader");
    for row in fixture["cases"].as_array().expect("fixture cases") {
        let message: DirectoryStreamMessage = semio_framework_pack_json::from_json_str(&row["message"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("fixture message {}: {error:?}", row["id"]));
        let created = row["createdOnSocket"].as_array().expect("created spaces").iter().map(|space| space.as_str().expect("space id").to_owned()).collect::<std::collections::BTreeSet<_>>();
        let expected = (!row["frame"].is_null()).then(|| semio_framework_pack_json::from_json_str::<DirectoryStreamMessage>(&row["frame"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("fixture frame"));
        let owed = directory_access_change_for_reader(reader, &created, &message);
        assert_eq!(owed, expected, "{}", row["id"]);
        if let Some(frame) = owed {
            assert_eq!(directory_stream_message_space(&frame), None, "an access change borrows no space authority: {}", row["id"]);
            assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&frame)).expect("frame JSON"), row["frame"], "wire shape: {}", row["id"]);
        }
    }
}

/// 🔑️ Live on a global directory socket: a reader added to a space created before its socket is told its access
/// changed right after the membership event (so its Home re-reads the space's earlier events); a space it creates on
/// that socket is not an access change (it read the space from its first event); its removal is.
#[test]
fn a_reader_added_to_an_older_space_is_told_its_access_changed_and_its_own_new_space_is_not() {
    run_socket_test(|| async {
        let state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("access-changed state open deadline");
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let owner = issue_test_session(&state, "access-owner@example.test").await;
        let reader = issue_test_session(&state, "access-reader@example.test").await;
        let older = create_space_for_test(&state, &owner.user_id, "Older", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&reader.token), State(state.clone()), Bytes::new()).await.expect("reader grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let (mut socket, _) = connect_async(socket_request(&format!("ws://{addr}/directory/socket/v1?since={since}"), &receipt.grant)).await.expect("reader socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("reader hello");
        upsert_member_for_test(&state, &older, "access-reader@example.test", DirectorySpaceRole::Author).await;
        assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { space_id, user_id, .. } if *space_id == older && *user_id == reader.user_id)), "the membership event itself");
        assert_eq!(next_directory_message(&mut socket).await, DirectoryStreamMessage::AccessChanged { space_id: older.clone(), change: os_directory::DirectoryAccessChange::Granted }, "then the reader's access change");
        let own = create_space_for_test(&state, &reader.user_id, "Own", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let mut own_frames = Vec::new();
        loop {
            let message = next_directory_message(&mut socket).await;
            let settled = matches!(&message, DirectoryStreamMessage::Event { event } if matches!(&event.body, os_directory::DirectoryEventBody::MemberUpserted { space_id, .. } if *space_id == own));
            own_frames.push(message);
            if settled {
                break;
            }
        }
        assert!(own_frames.iter().any(|message| matches!(message, DirectoryStreamMessage::Event { event } if matches!(&event.body, os_directory::DirectoryEventBody::SpaceCreated { space_id, .. } if *space_id == own))), "the creator reads its space from its first event: {own_frames:?}");
        state.directory_service.execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#test", owner.user_id) }, DirectoryCommand::RemoveMember { space_id: older.clone(), user_id: reader.user_id.clone() }).await.expect("remove reader");
        assert_eq!(next_directory_message(&mut socket).await, DirectoryStreamMessage::AccessChanged { space_id: older.clone(), change: os_directory::DirectoryAccessChange::Revoked }, "no AccessChanged for its own space; then its removal from the older one");
        socket.close(None).await.expect("close reader socket");
        stop_recovery_server(state, shutdown, server).await;
    });
}

/// 🗑️ Live on a global directory socket: a member of a space its owner deletes is told its access was revoked (its own
/// `member.removed` of the deletion), so its Home re-reads the directory from the origin without the space.
#[test]
fn a_member_of_a_deleted_space_is_told_its_access_was_revoked() {
    run_socket_test(|| async {
        let state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("deleted-space state open deadline");
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let owner = issue_test_session(&state, "deleted-owner@example.test").await;
        let reader = issue_test_session(&state, "deleted-reader@example.test").await;
        let doomed = create_space_for_test(&state, &owner.user_id, "Doomed", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &doomed, "deleted-reader@example.test", DirectorySpaceRole::Spectator).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&reader.token), State(state.clone()), Bytes::new()).await.expect("reader grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let (mut socket, _) = connect_async(socket_request(&format!("ws://{addr}/directory/socket/v1?since={since}"), &receipt.grant)).await.expect("reader socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("reader hello");
        state.directory_service.execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#test", owner.user_id) }, DirectoryCommand::DeleteSpace { space_id: doomed.clone() }).await.expect("delete space");
        assert_eq!(next_directory_message(&mut socket).await, DirectoryStreamMessage::AccessChanged { space_id: doomed.clone(), change: os_directory::DirectoryAccessChange::Revoked }, "the deletion revokes the member's access");
        socket.close(None).await.expect("close reader socket");
        stop_recovery_server(state, shutdown, server).await;
    });
}
