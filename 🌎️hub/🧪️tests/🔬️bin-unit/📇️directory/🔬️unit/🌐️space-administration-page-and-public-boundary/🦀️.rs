// 🌐️ Space administration page and public boundary: the space administration page route laws, the public boundary routes and sockets and the admin intent wire taxonomy.

//#region 🏛️SpaceAdministration
/// 🏛️ An author receives both bounded windows, a canonical receipt over the exact response
/// bytes, and the server's own capability flags — and no credential column anywhere.
#[tokio::test]
async fn space_administration_page_v1_route_returns_the_author_windows_with_a_canonical_receipt() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-author@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Administered", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "administration-guest@example.invalid", DirectorySpaceRole::Spectator).await;
    state
        .directory_service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 })
        .await
        .expect("author invite fixture");
    let addr = spawn_server(state).await;
    let authorization = format!("Bearer {}", author.token);
    let response = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await;
    assert_eq!(response.status, 200);
    let canonical = std::str::from_utf8(&response.body).expect("administration page UTF-8").to_string();
    assert!(canonical.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES);
    let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical).expect("canonical author page");
    assert_eq!(page.space_id(), space);
    let DirectorySpaceAdministrationPageV1::Author { members, invites, capabilities, .. } = &page else { panic!("author projection") };
    assert_eq!(members.rows.len(), 2);
    assert!(members.rows.windows(2).all(|pair| pair[0].user_id < pair[1].user_id), "member rows are keyset-ordered");
    assert!(members.rows.iter().any(|row| row.owner), "the owner row is marked so removal can be disabled before dispatch");
    assert_eq!(invites.rows.len(), 1);
    assert!(capabilities.remove_member && capabilities.create_invite);
    for secret in ["selector", "secretDigest", "passwordHash", "ssoSubject", "ssoProvider", "inviteToken"] {
        assert!(!canonical.contains(secret), "administration page leaked {secret}");
    }
}

/// 🛂️ A spectator receives the member shape only: invites and capability flags are structurally
/// absent, so no renderer can mis-gate them into existence.
#[tokio::test]
async fn space_administration_page_v1_route_denies_a_spectator_the_author_windows() {
    let _watchdog = LawHangWatchdogV1::arm("space_administration_page_v1_route_denies_a_spectator_the_author_windows");
    let state = bounded_law_step("spectator law: hub state open", test_state()).await;
    let author = bounded_law_step("spectator law: author session", issue_test_session(&state, "administration-owner@example.invalid")).await;
    let spectator = bounded_law_step("spectator law: spectator session", issue_test_session(&state, "administration-spectator@example.invalid")).await;
    let outsider = bounded_law_step("spectator law: outsider session", issue_test_session(&state, "administration-outsider@example.invalid")).await;
    let space = bounded_law_step("spectator law: space creation", create_space_for_test(&state, &author.user_id, "Spectated", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private)).await;
    bounded_law_step("spectator law: spectator membership", upsert_member_for_test(&state, &space, "administration-spectator@example.invalid", DirectorySpaceRole::Spectator)).await;
    bounded_law_step(
        "spectator law: author invite fixture",
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-spectator-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 }),
    )
    .await
    .expect("author invite fixture");
    let addr = spawn_server(state).await;
    let spectator_authorization = format!("Bearer {}", spectator.token);
    let response = bounded_law_step("spectator law: spectator page read", raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", spectator_authorization.as_str())])).await;
    assert_eq!(response.status, 200);
    let canonical = std::str::from_utf8(&response.body).expect("spectator page UTF-8").to_string();
    let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical).expect("canonical member page");
    assert!(matches!(page, DirectorySpaceAdministrationPageV1::Member { .. }));
    assert!(page.capabilities().is_none());
    assert!(!canonical.contains("\"invites\"") && !canonical.contains("\"capabilities\""));
    let outsider_authorization = format!("Bearer {}", outsider.token);
    assert_eq!(bounded_law_step("spectator law: outsider page read", raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", outsider_authorization.as_str())])).await.status, 404);
    assert_eq!(bounded_law_step("spectator law: anonymous page read", raw_http_get(addr, &format!("/directory/spaces/{space}"), &[])).await.status, 404);
}

/// 🧯️ A membership removal takes effect on the very next page read: the response is a denial with
/// no page bytes at all, so no member or invite row can leak past the revocation.
#[tokio::test]
async fn space_administration_page_v1_route_denies_a_removed_member_and_leaks_no_rows() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-remover@example.invalid").await;
    let removed = issue_test_session(&state, "administration-removed@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Revoked", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "administration-removed@example.invalid", DirectorySpaceRole::Spectator).await;
    let service = state.directory_service.clone();
    let addr = spawn_server(state).await;
    let authorization = format!("Bearer {}", removed.token);
    assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await.status, 200);
    service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-remove-law", author.user_id) }, DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: removed.user_id.clone() })
        .await
        .expect("member removal");
    let denied = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await;
    assert_eq!(denied.status, 404, "a removed member cannot even enumerate the private space");
    assert!(!String::from_utf8_lossy(&denied.body).contains("administration-remover@example.invalid"));
}

/// 🛡️ The query grammar is exact and every cursor is MAC-bound: a foreign, tampered, or
/// differently shaped cursor is a 400 with no page bytes, never a page for another identity.
#[tokio::test]
async fn space_administration_page_v1_route_rejects_a_noncanonical_query_and_a_foreign_cursor() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-cursor@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Cursored", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state).await;
    let authorization = format!("Bearer {}", author.token);
    for query in ["?cursor=", "?cursor=a&cursor=b", "?cursor=%6d", "?cursor=m+1", "?section=members", "?cursor=m.7573.zz"] {
        let response = raw_http_get(addr, &format!("/directory/spaces/{space}{query}"), &[("Authorization", authorization.as_str())]).await;
        assert_eq!(response.status, 400, "query {query} must be refused before any read");
        assert!(response.body.is_empty() || !String::from_utf8_lossy(&response.body).contains("receiptSha256"));
    }
    let forged = format!("m.{}.{}", os_directory::io::binary::artifact_hash::hex_lower(b"user-forged"), "ab".repeat(32));
    let response = raw_http_get(addr, &format!("/directory/spaces/{space}?cursor={forged}"), &[("Authorization", authorization.as_str())]).await;
    assert_eq!(response.status, 400, "a cursor with a forged MAC is refused");
    assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await.status, 200);
}
//#endregion 🏛️SpaceAdministration

#[test]
fn space_public_boundary_real_routes_emit_discriminated_public_member_author_and_private_404() {
    run_socket_test(|| async {
        let state = test_state().await;
        let author = issue_test_session(&state, "public-author@example.invalid").await;
        let spectator = issue_test_session(&state, "public-spectator@example.invalid").await;
        let outsider = issue_test_session(&state, "public-outsider@example.invalid").await;
        let public_space = create_space_for_test(&state, &author.user_id, "Discoverable", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        upsert_member_for_test(&state, &public_space, "public-spectator@example.invalid", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, &public_space, "catalog-document").await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#route-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: public_space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 })
            .await
            .expect("author invite fixture");
        let private_space = create_space_for_test(&state, &author.user_id, "Private", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let addr = spawn_server(state).await;

        let anonymous = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[]).await;
        assert_eq!(anonymous.status, 200);
        let anonymous: serde_json::Value = serde_json::from_slice(&anonymous.body).expect("anonymous public detail JSON");
        assert_eq!(anonymous["access"], "public");
        assert_eq!(anonymous["schema"], DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA);
        assert_eq!(anonymous["space"]["visibility"], "public");
        assert_eq!(anonymous["documents"]["rows"][0]["documentId"], "catalog-document");
        assert!(anonymous["documents"]["rows"][0].get("descriptor").is_none());
        assert_public_projection_has_no_private_keys(&anonymous);

        let outsider_authorization = format!("Bearer {}", outsider.token);
        let public_nonmember = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", outsider_authorization.as_str())]).await;
        assert_eq!(public_nonmember.status, 200);
        let nonmember: serde_json::Value = serde_json::from_slice(&public_nonmember.body).expect("nonmember public detail");
        assert_eq!(
            public_projection_without_caller_binding(&nonmember),
            public_projection_without_caller_binding(&anonymous),
            "an authenticated nonmember reads exactly the anonymous public projection"
        );
        assert_eq!(anonymous["sessionBindingSha256"], "0".repeat(64), "the anonymous public projection is bound to no session");
        assert_eq!(anonymous["authorizationGeneration"], 0, "the anonymous public projection carries no authorization generation");
        assert_ne!(nonmember["sessionBindingSha256"], anonymous["sessionBindingSha256"], "an authenticated reader's public page is bound to that reader's own session");
        assert!(nonmember["authorizationGeneration"].as_u64().expect("nonmember authorization generation") >= 1, "an authenticated reader's public page carries that reader's generation");
        assert_public_projection_has_no_private_keys(&nonmember);
        let private_nonmember = raw_http_get(addr, &format!("/directory/spaces/{private_space}"), &[("Authorization", outsider_authorization.as_str())]).await;
        assert_eq!(private_nonmember.status, 404);

        let public_document_status = format!("/spaces/{public_space}/documents/catalog-document");
        assert_eq!(raw_http_get(addr, &public_document_status, &[]).await.status, 401, "public discovery is not document-currentness authority");
        assert_eq!(raw_http_get(addr, &public_document_status, &[("Authorization", outsider_authorization.as_str())]).await.status, 401);
        let blob = format!("/spaces/{public_space}/blobs/{}", "11".repeat(32));
        assert_eq!(raw_http_request(addr, "GET", &blob, &[], &[]).await.status, 401, "public discovery is not blob read authority");
        assert_eq!(raw_http_request(addr, "HEAD", &blob, &[], &[]).await.status, 401, "public discovery is not blob existence authority");
        assert_eq!(raw_http_request(addr, "PUT", &blob, &[], b"private").await.status, 401, "public discovery is not blob write authority");
        assert_eq!(raw_http_request(addr, "GET", &blob, &[("Authorization", outsider_authorization.as_str())], &[]).await.status, 401);
        assert_eq!(raw_http_request(addr, "HEAD", &blob, &[("Authorization", outsider_authorization.as_str())], &[]).await.status, 401);
        assert_eq!(raw_http_request(addr, "PUT", &blob, &[("Authorization", outsider_authorization.as_str())], b"private").await.status, 401);

        let spectator_authorization = format!("Bearer {}", spectator.token);
        let member = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", spectator_authorization.as_str())]).await;
        assert_eq!(member.status, 200);
        let member: serde_json::Value = serde_json::from_slice(&member.body).expect("member detail");
        assert_eq!(member["access"], "member");
        assert!(member.get("members").is_some());
        assert!(member.get("invites").is_none(), "a member page structurally omits invites");
        assert!(member.get("capabilities").is_none(), "a member page structurally omits capability flags");
        assert_eq!(member["documents"]["rows"][0]["headSeq"], 0);

        let author_authorization = format!("Bearer {}", author.token);
        let authored = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", author_authorization.as_str())]).await;
        assert_eq!(authored.status, 200);
        let authored: serde_json::Value = serde_json::from_slice(&authored.body).expect("author detail");
        assert_eq!(authored["access"], "author");
        assert_eq!(authored["space"]["role"], "author");
        assert_eq!(authored["invites"]["rows"].as_array().map(Vec::len), Some(1));
        assert_eq!(authored["capabilities"]["removeMember"], true);
        let authored_bytes = std::str::from_utf8(&raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", author_authorization.as_str())]).await.body).expect("author page UTF-8").to_string();
        assert!(!authored_bytes.contains("selector") && !authored_bytes.contains("secretDigest") && !authored_bytes.contains("passwordHash"));
        assert_eq!(DirectorySpaceAdministrationPageV1::parse_canonical_json(&authored_bytes).map(|page| page.space_id().to_string()), Ok(public_space.clone()));

        let list = raw_http_get(addr, "/directory/spaces", &[]).await;
        assert_eq!(list.status, 200);
        let list: serde_json::Value = serde_json::from_slice(&list.body).expect("public list");
        let rows = list.as_array().expect("list rows");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["access"], "public");
        assert_public_projection_has_no_private_keys(&rows[0]);
    });
}

#[test]
fn space_public_boundary_public_event_route_denies_raw_directory_events() {
    run_socket_test(|| async {
        let state = test_state().await;
        let outsider = issue_test_session(&state, "event-outsider@example.invalid").await;
        let since = state.directory.head_seq().await.expect("pre-public head");
        let owner = issue_test_session(&state, "event-owner@example.invalid").await;
        let public_space = create_space_for_test(&state, &owner.user_id, "Event Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        announce_document_for_test(&state, &public_space, "event-document").await;
        let addr = spawn_server(state).await;
        let anonymous = raw_http_get(addr, &format!("/directory/events?since={since}&limit=100"), &[]).await;
        assert_eq!(anonymous.status, 200);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&anonymous.body).expect("anonymous events"), serde_json::json!([]));
        let authorization = format!("Bearer {}", outsider.token);
        let nonmember = raw_http_get(addr, &format!("/directory/events?since={since}&limit=100"), &[("Authorization", authorization.as_str())]).await;
        assert_eq!(nonmember.status, 200);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&nonmember.body).expect("nonmember events"), serde_json::json!([]));
    });
}

#[test]
fn space_public_boundary_real_socket_denies_public_raw_events_and_member_telemetry() {
    run_socket_test(|| async {
        let state = test_state().await;
        let outsider = issue_test_session(&state, "socket-public-outsider@example.invalid").await;
        let owner = issue_test_session(&state, "socket-public-owner@example.invalid").await;
        let public_space = create_space_for_test(&state, &owner.user_id, "Socket Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let since = state.directory.head_seq().await.expect("head");
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#socket-replay-law", owner.user_id) }, DirectoryCommand::RenameSpace { space_id: public_space.clone(), name: "Socket Public Replay".into() })
            .await
            .expect("replayed raw public event");
        let receipt = issue_directory_socket_grant(bearer_headers(&outsider.token), State(state.clone()), Bytes::new()).await.expect("outsider directory grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("public outsider directory socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("credential-free directory hello");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#socket-live-law", owner.user_id) }, DirectoryCommand::RenameSpace { space_id: public_space.clone(), name: "Socket Public Live".into() })
            .await
            .expect("live raw public event");
        state.directory_service.publish(DirectoryStreamMessage::Connection {
            phase: DirectoryConnectionPhase::Opened,
            connection: ConnectionView {
                sync_session_id: "private-sync".into(),
                space_id: public_space,
                document_id: "private-document".into(),
                surface: "private-surface".into(),
                actor: "private-actor".into(),
                user_id: Some(owner.user_id),
                email: Some("socket-public-owner@example.invalid".into()),
                role: DirectorySpaceRole::Author,
                connected_at_ms: 1,
                presence_known: true,
            },
        });
        let head = state.directory.head_seq().await.expect("post-event head");
        assert!(head > since);
        state.directory_service.publish(DirectoryStreamMessage::Heartbeat { head_seq: head });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(250), socket.next()).await.is_err(), "public nonmember received a raw event, member telemetry, or global progress cursor");
    });
}

#[test]
fn admin_intent_wire_taxonomy_rejects_generic_and_unknown_commands() {
    let valid = r#"{"kind":"create-space","requestId":"request:one","name":"Studio","spaceKind":"studio","visibility":"private"}"#;
    assert!(semio_framework_pack_json::from_json_str::<AdminIntentV1>(valid, semio_framework_pack_json::JsonMemberPolicy::Reject).is_ok());
    let generic = r#"{"kind":"directory","requestId":"request:one","command":{"kind":"create-space","name":"Studio","spaceKind":"studio","visibility":"private"}}"#;
    let forbidden = r#"{"kind":"announce-document","requestId":"request:one","descriptor":{}}"#;
    let unknown = r#"{"kind":"create-space","requestId":"request:one","name":"Studio","spaceKind":"studio","visibility":"private","actor":"admin"}"#;
    assert!(semio_framework_pack_json::from_json_str::<AdminIntentV1>(generic, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    assert!(semio_framework_pack_json::from_json_str::<AdminIntentV1>(forbidden, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    assert!(semio_framework_pack_json::from_json_str::<AdminIntentV1>(unknown, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
}
