/// 🔑️ Directory command authority and invite redemption: the event page and receipt helpers, the directory command authority revalidation and the invite redemption laws.

async fn append_directory_page_test_events(state: &HubState, rows: &[(String, String)]) -> Vec<DirectoryEvent> {
    let events = rows
        .iter()
        .map(|(space_id, name)| semio_hub_directory_contract::NewDirectoryEvent {
            hlc: os_directory::Hlc { physical_ms: now_ms(), logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: format!("system:event-page:{space_id}") },
            space_id: Some(space_id.clone()),
            user_id: None,
            body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: space_id.clone(), name: name.clone() },
        })
        .collect::<Vec<_>>();
    state.directory.append_events(&events).await.expect("append event-page fixture")
}

fn event_page_authorization(token: &str) -> String {
    format!("Bearer {token}")
}

fn parse_directory_command_receipt_for_test(response: &RawHttpResponse, request_id: &str, command: &DirectoryCommand) -> DirectoryCommandReceiptV1 {
    let canonical = std::str::from_utf8(&response.body).expect("receipt UTF-8");
    DirectoryCommandReceiptV1::parse_canonical_json(canonical, &DirectoryCommandRequestV1::new(request_id, command.clone())).expect("canonical receipt")
}

#[tokio::test]
async fn directory_command_authority_revalidates_after_durable_revocation_before_fence() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../📇️directory/🧪️tests/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["appended"] == 0) {
        let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("authority state open deadline");
        let owner = issue_test_session(&state, "authority-owner@example.com").await;
        let author = issue_test_session(&state, "authority-author@example.com").await;
        let target = issue_test_session(&state, "authority-target@example.com").await;
        let space = create_space_for_test(&state, &owner.user_id, "Authority Race", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space, "authority-target@example.com", DirectorySpaceRole::Spectator).await;
        let gate = Arc::new(TestLiveGate::default());
        *gate.directory_command_pause_user.lock().unwrap() = Some((author.user_id.clone(), false));
        state.live_gate = Some(gate.clone());
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let command = DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: target.user_id.clone() };
        let request_id = "a00102030405060708090a0b0c0d0e0f";
        let pending = {
            let token = author.token.clone();
            let command = command.clone();
            tokio::spawn(async move { post_directory_command_for_test(addr, &token, request_id, command).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
        match row["revocation"].as_str().unwrap() {
            "demote" | "remove" => {
                let revocation = if row["revocation"] == "demote" {
                    DirectoryCommand::UpsertMember { space_id: space.clone(), email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator }
                } else {
                    DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: author.user_id.clone() }
                };
                assert_eq!(post_directory_command_for_test(addr, &owner.token, "b00102030405060708090a0b0c0d0e0f", revocation).await.status, 202);
            }
            "session" => assert_eq!(post_session_sign_out(bearer_headers(&author.token), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT),
            _ => unreachable!(),
        }
        let head = state.directory.head_seq().await.unwrap();
        gate.directory_command_release.add_permits(1);
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["id"]);
        assert!(response.body.is_empty());
        assert_eq!(state.directory.head_seq().await.unwrap(), head);
        assert_eq!(state.directory.get_role(&space, &target.user_id).await.unwrap(), Some(SpaceRole::Spectator));
        let claim = NewDirectoryCommandReceipt { actor_user_id: author.user_id.clone(), request_id: request_id.into(), command_sha256: directory_command_sha256(&command), result_kind: directory_command_result_kind(&command), claimed_at: now_ms() };
        assert!(matches!(state.directory.claim_or_read_directory_command_receipt(&claim).await.unwrap(), DirectoryCommandClaimV1::Claimed(_)), "denied command must leave no durable claim");
        state.directory.release_directory_command_receipt(&author.user_id, request_id, &claim.command_sha256).await.unwrap();
        stop_recovery_server(state, shutdown, server).await;
    }
}

#[tokio::test]
async fn directory_command_authority_holds_admitted_command_until_receipt_before_demotion() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../📇️directory/🧪️tests/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["appended"] == 1).unwrap();
    let mut state = test_state().await;
    let owner = issue_test_session(&state, "authority-owner@example.com").await;
    let author = issue_test_session(&state, "authority-author@example.com").await;
    let target = issue_test_session(&state, "authority-target@example.com").await;
    let space = create_space_for_test(&state, &owner.user_id, "Authority Order", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space, "authority-target@example.com", DirectorySpaceRole::Spectator).await;
    let gate = Arc::new(TestLiveGate::default());
    *gate.directory_command_pause_user.lock().unwrap() = Some((author.user_id.clone(), true));
    state.live_gate = Some(gate.clone());
    let head = state.directory.head_seq().await.unwrap();
    let addr = spawn_server(state.clone()).await;
    let command = DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: target.user_id.clone() };
    let pending = {
        let token = author.token.clone();
        let command = command.clone();
        tokio::spawn(async move { post_directory_command_for_test(addr, &token, "c00102030405060708090a0b0c0d0e0f", command).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
    gate.directory_command_attempted.acquire().await.unwrap().forget();
    let demotion = DirectoryCommand::UpsertMember { space_id: space.clone(), email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator };
    let revoking = {
        let token = owner.token.clone();
        tokio::spawn(async move { post_directory_command_for_test(addr, &token, "d00102030405060708090a0b0c0d0e0f", demotion).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_attempted.acquire()).await.unwrap().unwrap().forget();
    assert!(!revoking.is_finished());
    assert_eq!(state.directory.head_seq().await.unwrap(), head);
    gate.directory_command_release.add_permits(1);
    let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
    assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap());
    let receipt = parse_directory_command_receipt_for_test(&response, "c00102030405060708090a0b0c0d0e0f", &command);
    assert_eq!(receipt.outcome, DirectoryCommandOutcomeV1::Accepted);
    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(5), revoking).await.unwrap().unwrap().status, 202);
    let events = state.directory.events_since(head, 8).await.unwrap();
    assert_eq!(events.len(), 2);
    assert!(matches!(&events[0].body, os_directory::DirectoryEventBody::MemberRemoved { user_id, .. } if user_id == &target.user_id));
    assert_eq!(state.directory.get_role(&space, &target.user_id).await.unwrap(), None);
    assert_eq!(state.directory.get_role(&space, &author.user_id).await.unwrap(), Some(SpaceRole::Spectator));
}

#[tokio::test]
async fn directory_command_authority_invite_revocation_requires_the_exact_owned_space() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../📇️directory/🧪️tests/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).expect("authority fixture");
    let state = test_state().await;
    let caller = issue_test_session(&state, "invite-scope-owner@example.test").await;
    let other = issue_test_session(&state, "invite-scope-other@example.test").await;
    let owned = create_space_for_test(&state, &caller.user_id, "Owned space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let foreign = create_space_for_test(&state, &other.user_id, "Foreign space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    for (index, row) in fixture["inviteScopes"].as_array().expect("invite scope rows").iter().enumerate() {
        let issued = state.directory.issue_invite(&foreign, SpaceRole::Spectator, 600, "invite-scope-law").await.expect("foreign invite");
        let invite = issued.record.clone();
        if row["accepted"].as_bool().expect("accepted fixture") {
            let recipient = issue_test_session(&state, &format!("invite-recipient-{index}@example.test")).await;
            state.directory_service.redeem_invite(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#invite-scope", recipient.user_id) }, &issued.capability, &recipient.user_id).await.expect("accepted invite fixture");
        }
        let exact = row["commandSpace"] == row["inviteSpace"];
        let (space, token) = if exact { (&foreign, &other.token) } else { (&owned, &caller.token) };
        let head = state.directory.head_seq().await.expect("before revoke head");
        let request_id = format!("{:032x}", 1000 + index);
        let response = post_directory_command_for_test(addr, token, &request_id, DirectoryCommand::RevokeInvite { space_id: space.clone(), invite_id: invite.id.clone() }).await;
        assert_eq!(u64::from(response.status), row["status"].as_u64().expect("expected status"), "{}", row["id"]);
        let current = state.directory.list_invites(&foreign).await.expect("foreign invite after request").into_iter().find(|candidate| candidate.id == invite.id).expect("retained foreign invite");
        assert_eq!(current.revoked_at.is_some(), row["revoked"].as_bool().expect("expected revocation"));
        assert_eq!(state.directory.head_seq().await.expect("after revoke head"), head, "invite revocation changes capability authority, not membership events");
        if !exact {
            assert!(response.body.is_empty(), "cross-space response carries no foreign metadata");
        }
    }
}

/// 🎟️ Uses real space commands and HTTP redemption to preserve archive reads without restoring authorship.
#[tokio::test]
async fn directory_invite_redemption_obeys_current_space_state_and_readonly_replay() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).expect("invite state fixture");
    let state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("invite state open deadline");
    let owner = issue_test_session(&state, "invite-state-owner@example.test").await;
    let addr = spawn_server(state.clone()).await;
    for (index, row) in fixture["spaceStates"].as_array().expect("state rows").iter().enumerate() {
        let caller = issue_test_session(&state, &format!("invite-state-{index}@example.test")).await;
        let other = issue_test_session(&state, &format!("invite-state-other-{index}@example.test")).await;
        let space = create_space_for_test(&state, &owner.user_id, "Invite state", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let role = if row["inviteRole"] == "author" { SpaceRole::Author } else { SpaceRole::Spectator };
        let issued = state.directory.issue_invite(&space, role, 600, "invite-state-law").await.expect("pending invite");
        let accepted_user = if row["accepted"] == "other" { &other } else { &caller };
        let accepted = if row["accepted"] != "none" {
            state
                .directory_service
                .redeem_invite(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#invite-state", accepted_user.user_id) }, &issued.capability, &accepted_user.user_id)
                .await
                .map(|commit| vec![commit.into_event()])
                .expect("initial acceptance")
        } else {
            Vec::new()
        };
        let transition = match row["spaceState"].as_str().unwrap() {
            "archived" => Some(DirectoryCommand::ArchiveSpace { space_id: space.clone() }),
            "deleted" => Some(DirectoryCommand::DeleteSpace { space_id: space.clone() }),
            "writable" => None,
            _ => unreachable!(),
        };
        if let Some(command) = transition {
            assert_eq!(post_directory_command_for_test(addr, &owner.token, &format!("{:032x}", 2000 + index), command).await.status, 202);
        }
        let replay_grant = if row["accepted"] == "same" && row["spaceState"] != "deleted" {
            let session = resolve_bearer_user(&state, Some(&caller.token)).await.unwrap();
            let capability = SocketGrantCapability::mint().unwrap();
            let audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new(&space, "invite-replay"));
            let subject = SocketSubjectV1::Session { session_id: session.session_id, user_id: caller.user_id.clone(), authorization_generation: session.authorization_generation, role: Some(SpaceRole::Spectator), expires_at_ms: session.expires_at, session_kind: AuthSessionKind::External, device_instance_id: "device-spectator".into() };
            state.socket_grants.issue(&capability, audience.clone(), "hub.v1.invite-replay".into(), subject, now_ms(), now_ms() + 30_000).unwrap();
            Some((capability, audience))
        } else {
            None
        };
        let before = state.directory.head_seq().await.unwrap();
        let mut stream = state.directory_service.subscribe();
        let token = issued.capability.expose_once();
        let authorization = format!("Bearer {}", caller.token);
        let response = raw_http_request(addr, "POST", &format!("/directory/invites/{token}/redeem"), &[("Authorization", &authorization)], &[]).await;
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["name"]);
        if let Some((capability, audience)) = replay_grant {
            assert!(state.socket_grants.pending(&capability, &audience, now_ms()).is_ok(), "read-only replay preserves fresh admission");
        }
        let appended = state.directory.events_since(before, 16).await.unwrap();
        assert_eq!(appended.len() as u64, row["appended"].as_u64().unwrap(), "{}", row["name"]);
        assert!(appended.iter().all(|event| matches!(event.body, os_directory::DirectoryEventBody::InviteRedeemed { .. })));
        let current = state.directory.get_role(&space, &caller.user_id).await.unwrap();
        let expected_role = match row["membershipRole"].as_str().unwrap() {
            "author" => Some(SpaceRole::Author),
            "spectator" => Some(SpaceRole::Spectator),
            "none" => None,
            _ => unreachable!(),
        };
        assert_eq!(current, expected_role, "{}", row["name"]);
        let invitations = state.directory.list_invites(&space).await.unwrap();
        if row["spaceState"] == "deleted" {
            assert!(invitations.is_empty());
        } else {
            let retained = invitations.iter().find(|invite| invite.id == issued.record.id).expect("retained invitation");
            assert_eq!(retained.accepted_at.is_some(), !accepted.is_empty() || !appended.is_empty());
            if !accepted.is_empty() {
                assert_eq!(retained.accepted_event_id.as_deref(), Some(accepted[0].id.as_str()));
            }
        }
        if row["appended"] == 0 {
            assert!(matches!(stream.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "replay and denial publish no new event");
        } else {
            assert!(stream.try_recv().is_ok());
        }
        if response.status == 200 && !accepted.is_empty() {
            let expected = axum::body::to_bytes(DirectoryJson(accepted).into_response().into_body(), 1 << 20).await.unwrap();
            assert_eq!(response.body.as_slice(), expected.as_ref(), "replay returns the exact original immutable event");
        } else if response.status != 200 {
            assert!(response.body.is_empty());
        }
    }
}

/// 🧭️ Scope selection accepts only the exact stored secret and existing authenticated subject.
#[tokio::test]
async fn directory_invite_redemption_scope_hint_is_capability_bound() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("invite hint state deadline");
    let owner = issue_test_session(&state, "invite-hint-owner@example.test").await;
    let caller = issue_test_session(&state, "invite-hint-caller@example.test").await;
    let space = create_space_for_test(&state, &owner.user_id, "Invite hint", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-hint").await.unwrap();
    let other = InviteCapability::mint().unwrap();
    let other_encoded = other.expose_once();
    let wrong_secret = InviteCapability::parse(&other_encoded.replacen(other.selector(), issued.capability.selector(), 1)).unwrap();
    let head = state.directory.head_seq().await.unwrap();
    for row in fixture["vectors"].as_array().unwrap().iter().filter(|row| ["fresh-single", "wrong-selector", "wrong-secret", "actor-mismatch", "missing-user"].contains(&row["name"].as_str().unwrap())) {
        let user_id = if row["name"] == "missing-user" { "nonexistent-user" } else { &caller.user_id };
        let actor_user = if row["name"] == "actor-mismatch" { &owner.user_id } else { user_id };
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{actor_user}#hint") };
        let capability = match row["name"].as_str().unwrap() {
            "wrong-selector" => &other,
            "wrong-secret" => &wrong_secret,
            _ => &issued.capability,
        };
        let result = state.directory.invite_redemption_scope_hint(capability, &actor, user_id).await;
        if row["expected"]["outcomes"][0] == "newly-committed" {
            assert_eq!(result.unwrap().space_id(), space);
        } else {
            assert!(matches!(result, Err(DirectoryError::Unauthorized)), "{}", row["name"]);
        }
        assert_eq!(state.directory.head_seq().await.unwrap(), head, "a hint writes no event");
        assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), None, "a hint grants no membership");
        assert!(state.directory.list_invites(&space).await.unwrap().iter().all(|invite| invite.accepted_at.is_none()));
    }
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hint", caller.user_id) };
    state.directory_service.execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hint-owner", owner.user_id) }, DirectoryCommand::DeleteSpace { space_id: space }).await.unwrap();
    assert!(matches!(state.directory.invite_redemption_scope_hint(&issued.capability, &actor, &caller.user_id).await, Err(DirectoryError::Unauthorized)));
}

/// 🔒️ A paused redemption cannot spend the identity captured before a durable revocation.
#[tokio::test]
async fn directory_invite_redemption_revalidates_after_hint_before_fence() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("invite race state deadline");
    let owner = issue_test_session(&state, "invite-race-owner@example.test").await;
    let gate = Arc::new(TestLiveGate::default());
    state.live_gate = Some(gate.clone());
    let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
    for (index, row) in fixture["authorityRaces"].as_array().unwrap().iter().filter(|row| row["appended"] == 0).enumerate() {
        let caller = issue_test_session(&state, &format!("invite-race-{index}@example.test")).await;
        let space = create_space_for_test(&state, &owner.user_id, "Invite race", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-race").await.unwrap();
        *gate.directory_command_pause_user.lock().unwrap() = Some((caller.user_id.clone(), false));
        let pending = {
            let token = caller.token.clone();
            let path = format!("/directory/invites/{}/redeem", issued.capability.expose_once());
            tokio::spawn(async move { raw_http_request(addr, "POST", &path, &[("Authorization", &format!("Bearer {token}"))], &[]).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
        if row["revocation"] == "session" {
            assert_eq!(post_session_sign_out(bearer_headers(&caller.token), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT);
        } else {
            let command = if row["revocation"] == "archive" { DirectoryCommand::ArchiveSpace { space_id: space.clone() } } else { DirectoryCommand::DeleteSpace { space_id: space.clone() } };
            assert_eq!(post_directory_command_for_test(addr, &owner.token, &format!("{:032x}", 3000 + index), command).await.status, 202);
        }
        let head = state.directory.head_seq().await.unwrap();
        gate.directory_command_release.add_permits(1);
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["name"]);
        assert!(response.body.is_empty());
        assert_eq!(state.directory.head_seq().await.unwrap(), head);
        assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), None);
        assert!(state.directory.list_invites(&space).await.unwrap().iter().all(|invite| invite.accepted_at.is_none()));
    }
    shutdown.send(()).unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), server).await.unwrap().unwrap();
}

/// 🧱️ All session and space keys stay owned through one irreversible invitation event.
#[tokio::test]
async fn directory_invite_redemption_admitted_fence_precedes_archive() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let row = fixture["authorityRaces"].as_array().unwrap().iter().find(|row| row["appended"] == 1).unwrap();
    let mut state = tokio::time::timeout(TEST_STATE_OPEN_HANG_GUARD, test_state()).await.expect("invite order state deadline");
    let owner = issue_test_session(&state, "invite-order-owner@example.test").await;
    let caller = issue_test_session(&state, "invite-order-caller@example.test").await;
    let session = resolve_bearer_user(&state, Some(&caller.token)).await.unwrap();
    let space = create_space_for_test(&state, &owner.user_id, "Invite order", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-order").await.unwrap();
    let gate = Arc::new(TestLiveGate::default());
    *gate.directory_command_pause_user.lock().unwrap() = Some((caller.user_id.clone(), true));
    state.live_gate = Some(gate.clone());
    let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
    let head = state.directory.head_seq().await.unwrap();
    let pending = {
        let token = caller.token.clone();
        let path = format!("/directory/invites/{}/redeem", issued.capability.expose_once());
        tokio::spawn(async move { raw_http_request(addr, "POST", &path, &[("Authorization", &format!("Bearer {token}"))], &[]).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
    for key in [
        SocketBindingKeyV1::User(caller.user_id.clone()),
        SocketBindingKeyV1::Session(session.session_id.clone()),
        SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space.clone() },
        SocketBindingKeyV1::Membership { space_id: space.clone(), user_id: caller.user_id.clone() },
    ] {
        assert!(state.socket_binding_gates.gate(key).try_write_owned().is_err(), "the admitted redemption must own every exact authority key");
    }
    gate.directory_command_attempted.acquire().await.unwrap().forget();
    let revoking = {
        let token = owner.token.clone();
        let command = DirectoryCommand::ArchiveSpace { space_id: space.clone() };
        tokio::spawn(async move { post_directory_command_for_test(addr, &token, "f10102030405060708090a0b0c0d0e0f", command).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_attempted.acquire()).await.unwrap().unwrap().forget();
    assert!(!revoking.is_finished());
    assert_eq!(state.directory.head_seq().await.unwrap(), head);
    gate.directory_command_release.add_permits(1);
    let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
    assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap());
    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(5), revoking).await.unwrap().unwrap().status, 202);
    let events = state.directory.events_since(head, 16).await.unwrap();
    assert!(matches!(&events[0].body, os_directory::DirectoryEventBody::InviteRedeemed { user_id, role: DirectorySpaceRole::Author, .. } if user_id == &caller.user_id));
    assert!(matches!(&events.last().unwrap().body, os_directory::DirectoryEventBody::SpaceArchived { .. }));
    assert_eq!(events.iter().filter(|event| matches!(event.body, os_directory::DirectoryEventBody::InviteRedeemed { .. })).count(), row["appended"].as_u64().unwrap() as usize);
    assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), Some(SpaceRole::Spectator));
    shutdown.send(()).unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), server).await.unwrap().unwrap();
}

#[tokio::test]
async fn directory_command_authority_demotion_invalidates_only_affected_scope_once() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../📇️directory/🧪️tests/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
    let shared = SocketGrantLedgerV1::default();
    let capacity = &fixture["capacity"];
    assert_eq!(capacity["subjectLimit"].as_u64().unwrap() as usize, SOCKET_GRANT_BINDING_PENDING_CAPACITY);
    assert_eq!(capacity["ledgerLimit"].as_u64().unwrap() as usize, SOCKET_GRANT_LEDGER_CAPACITY);
    for index in 0..capacity["subjects"].as_u64().unwrap() {
        let subject = SocketSubjectV1::Session { session_id: format!("capacity-session-{index}"), user_id: format!("capacity-user-{index}"), authorization_generation: 1, role: Some(SpaceRole::Author), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: format!("capacity-device-{index}") };
        shared.issue(&SocketGrantCapability::mint().unwrap(), SocketAudienceV1::DirectoryScoped(DocumentScope::new("shared-space", "document")), format!("hub.v1.capacity-{index}"), subject, 1, 9_000).unwrap();
    }
    assert_eq!(shared.inner.lock().unwrap().records.len() as u64, capacity["accepted"].as_u64().unwrap());
    let state = test_state().await;
    let owner = issue_test_session(&state, "authority-owner@example.com").await;
    let author = issue_test_session(&state, "authority-author@example.com").await;
    let space = create_space_for_test(&state, &owner.user_id, "Authority Bindings", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let other = create_space_for_test(&state, &author.user_id, "Other Bindings", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
    let plan_fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let plan = Arc::new(document_open_plan_test_authority(&plan_fixture));
    let issue_pending = |audience: &SocketAudienceV1, actor_id: String, subject: SocketSubjectV1| -> String {
        if matches!(audience, SocketAudienceV1::Document(_)) {
            return state.socket_grants.admit_document(audience.clone(), actor_id, subject, now_ms(), now_ms() + 30_000, plan.clone()).unwrap();
        }
        let capability = SocketGrantCapability::mint().unwrap();
        state.socket_grants.issue(&capability, audience.clone(), actor_id, subject, now_ms(), now_ms() + 30_000).unwrap();
        capability.selector().to_string()
    };
    let still_pending = |selector: &str| state.socket_grants.inner.lock().unwrap().records.get(selector).is_some_and(|record| record.state == SocketGrantStateV1::Pending);
    let mut records = Vec::new();
    for row in fixture["bindings"].as_array().unwrap() {
        let user = if row["user"] == "author" { &author } else { &owner };
        let session = state.directory.authenticate_session(&SessionCapability::parse(&user.token).unwrap()).await.unwrap().unwrap();
        let scope = DocumentScope::new(if row["space"] == "changed" { &space } else { &other }, "authority-document");
        let subject = SocketSubjectV1::Session { session_id: session.id.clone(), user_id: user.user_id.clone(), authorization_generation: session.authorization_generation, role: Some(SpaceRole::Author), expires_at_ms: session.expires_at, session_kind: session.session_kind, device_instance_id: session.device_instance_id.clone() };
        let audience = match row["audience"].as_str().unwrap() {
            "document" => SocketAudienceV1::Document(scope),
            "scoped" => SocketAudienceV1::DirectoryScoped(scope),
            "global" => SocketAudienceV1::Directory { auth_session_id: session.id, authorization_generation: session.authorization_generation },
            _ => unreachable!(),
        };
        let actor_id = format!("hub.v1.{}", row["id"].as_str().unwrap());
        let live = issue_pending(&audience, actor_id.clone(), subject.clone());
        let pending = issue_pending(&audience, actor_id, subject.clone());
        let record = state.socket_grants.inner.lock().unwrap().records.get(&live).cloned().unwrap();
        let record = state.socket_grants.consume(&record, now_ms()).unwrap();
        let (live_id, notify) = state.socket_grants.register_live(&record).unwrap();
        records.push((row, pending, record, live_id, notify));
    }
    let addr = spawn_server(state.clone()).await;
    let command = DirectoryCommand::UpsertMember { space_id: space, email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator };
    let request_id = "e00102030405060708090a0b0c0d0e0f";
    assert_eq!(post_directory_command_for_test(addr, &owner.token, request_id, command.clone()).await.status, 202);
    for (row, pending, record, live_id, notify) in &records {
        let invalidated = row["invalidated"].as_bool().unwrap();
        assert_eq!(!still_pending(pending), invalidated, "pending {}", row["id"]);
        assert_eq!(!state.socket_grants.is_live(record, live_id), invalidated, "live {}", row["id"]);
        if invalidated {
            tokio::time::timeout(std::time::Duration::from_secs(1), notify.notified()).await.unwrap();
            let mut subject = record.subject.clone();
            if let SocketSubjectV1::Session { role, .. } = &mut subject {
                *role = Some(SpaceRole::Spectator);
            }
            let fresh = issue_pending(&record.audience, record.actor_id.clone(), subject);
            assert_eq!(post_directory_command_for_test(addr, &owner.token, request_id, command.clone()).await.status, 202);
            assert!(still_pending(&fresh), "receipt replay must not invalidate fresh admission");
        }
    }
}
