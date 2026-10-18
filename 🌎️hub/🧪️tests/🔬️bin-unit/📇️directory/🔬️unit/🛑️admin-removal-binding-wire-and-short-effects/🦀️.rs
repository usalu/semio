/// 🛑️ Admin removal, binding wire and short effects: the open hang guard, the recovery denial helper, the admin removal that revokes plan presence, the admin intent binding wire matrix and the short admin effects that retain their principal.

/// ⏳️ The single wall-clock bound every `test_state()` open in this suite answers to. Opening a hub
/// state is bounded I/O, not a rendezvous, so the bound exists only to name a genuinely wedged open
/// instead of letting nextest kill it 300 s later with no site. It replaces the per-call 5 s bounds,
/// which measured fleet scheduling latency and failed `directory_invite_redemption_admitted_fence_
/// precedes_archive` on load alone (2026-09-20).
const TEST_STATE_OPEN_HANG_GUARD: std::time::Duration = std::time::Duration::from_secs(60);

fn assert_recovery_denied(response: RawHttpResponse, expected: &serde_json::Value) {
    assert_eq!(u64::from(response.status), expected["removedStatus"].as_u64().unwrap());
    assert!(response.headers.lines().any(|line| line.eq_ignore_ascii_case("content-type: application/json")));
    let error: DocumentOpenPlanErrorV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&response.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("strict bounded error, not selected asset bytes");
    assert_eq!(error.schema, "semio.hub.document-open-plan-error/v1");
    assert_eq!(error.code, DocumentOpenPlanErrorCodeV1::Denied);
    assert_eq!(expected["removedCode"], "denied");
}

#[test]
fn admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json")).expect("admin recovery fixture");
        let expected = &fixture["expected"];
        let scope = DocumentScope::new(fixture["scope"]["spaceId"].as_str().unwrap(), fixture["scope"]["documentId"].as_str().unwrap());
        let dir = tempdir("admin-presence-target-recovery");
        std::fs::create_dir_all(&dir).expect("recovery fixture directory");
        let path = dir.join("directory.sqlite");
        let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("file directory");
        let mut state = test_state_with_directory(dir.join("db"), directory, 1024, 256).await;
        let admin_headers = authorize_test_admin(&mut state, "recovery-admin@example.com").await;
        let removed = issue_test_session(&state, "recovery-removed@example.com").await;
        let observer = issue_test_session(&state, "recovery-observer@example.com").await;
        for member in fixture["members"].as_array().unwrap() {
            let email = format!("recovery-{}@example.com", member["id"].as_str().unwrap());
            assert_eq!(member["role"], "author");
            upsert_member_for_test(&state, &scope.space_id, &email, DirectorySpaceRole::Author).await;
        }
        seed_genesis_for_document_for_test(&state, &removed.token, &scope.space_id, &scope.document_id, "admin-presence-recovery").await;
        let descriptor = state.directory.get_document_descriptor(&scope).await.unwrap().unwrap();
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
        let (plan_b, grant_b) = issue_and_exchange_document_open_plan_for_test(&state, &removed.token, &scope, "client:removed").await;
        let (plan_c, grant_c) = issue_and_exchange_document_open_plan_for_test(&state, &observer.token, &scope, "client:observer").await;
        assert_eq!(plan_b.surface.surface_id, fixture["surfaceId"].as_str().unwrap());
        assert_eq!(plan_b.surface.surface_id, plan_c.surface.surface_id);
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let root = format!("/spaces/{}/documents/{}", scope.space_id, scope.document_id);
        let url =         format!(
            "ws://{addr}/scopes/{}%2F{}/document/ws?surface={}",
            scope.space_id,
            scope.document_id,
            plan_b.surface.surface_id
        );
        let (mut b, _) = connect_async(document_socket_request(&url, &removed.token)).await.expect("member document socket");
        let (mut c, _) = connect_async(document_socket_request(&url, &observer.token)).await.expect("observer document socket");
        b.send(client_binary(&socket_hello(), Lane::Command).await).await.unwrap();
        c.send(client_binary(&socket_hello(), Lane::Command).await).await.unwrap();
        let welcome_b = match tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, b.next()).await {
            Ok(Some(Ok(WsMessage::Binary(bytes)))) => protocol::decode_server_frame(&bytes).await.expect("member welcome frame").1,
            other => panic!("the member plan socket never reached its welcome: {other:?} refused as {:?}", last_document_plan_refusal()),
        };
        assert!(matches!(&welcome_b, ServerFrame::Welcome { .. }), "member welcome: {welcome_b:?}");
        let ServerFrame::Session { actor: actor_b, color: color_b } = next_server_frame_at(&mut b, "member session frame after the sync-hello bootstrap drain").await else { panic!("member session") };
        let welcome_c = next_server_frame_at(&mut c, "observer welcome").await;
        assert!(matches!(&welcome_c, ServerFrame::Welcome { .. }), "observer welcome: {welcome_c:?}");
        let ServerFrame::Session { actor: actor_c, .. } = next_server_frame_at(&mut c, "observer session frame after the sync-hello bootstrap drain").await else { panic!("observer session") };
        let raw = presence_hex_bytes(presence_normalization_fixture()["vectors"][0]["rawPeerHex"].as_str().unwrap());
        b.send(client_binary(&ClientFrame::Presence { peer: raw.clone() }, Lane::Preview).await).await.unwrap();
        let ServerFrame::Presence { peers } = next_server_frame_at(&mut c, "observer normalized presence of the member").await else { panic!("observer normalized presence") };
        assert_eq!(peers.len(), 1);
        let peer = protocol::decode_presence_peer(&peers[0]).await.unwrap();
        assert_eq!(peer.actor, actor_b);
        assert_eq!(peer.user_id.as_deref(), Some(removed.user_id.as_str()));
        assert_eq!(peer.label.as_deref(), Some("recovery-removed@example.com"));
        assert_eq!(peer.role.as_deref(), Some("author"));
        assert_eq!(peer.color, Some(color_b));
        assert_eq!(peer.surface.as_deref(), Some(plan_b.surface.surface_id.as_str()));
        assert!(matches!(next_server_frame_at(&mut b, "member echo of its own normalized presence").await, ServerFrame::Presence { .. }));
        let intent = semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some(plan_b.surface.surface_id.clone()),
            client_instance_id: "client:recovery-target".into(),
        });
        let authorization_b = format!("Bearer {}", removed.token);
        let authorization_c = format!("Bearer {}", observer.token);
        let headers_b = [("Authorization", authorization_b.as_str()), ("Content-Type", "application/json")];
        let headers_c = [("Authorization", authorization_c.as_str()), ("Content-Type", "application/json")];
        assert_eq!(raw_http_request(addr, "POST", &format!("{root}/execution-target/manifest"), &headers_b, intent.as_bytes()).await.status, 200);
        let (_, pending_b) = issue_and_exchange_document_open_plan_for_test(&state, &removed.token, &scope, "client:pending-removed").await;
        let remove = semio_framework_pack_json::to_json_string(&AdminIntentV1::RemoveSpaceMember { request_id: "request:admin-presence-recovery".into(), space_id: scope.space_id.clone(), user_id: removed.user_id.clone() });
        let admin_authorization = admin_headers.get(axum::http::header::AUTHORIZATION).unwrap().to_str().unwrap();
        let receipt = raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", admin_authorization), ("Content-Type", "application/json")], remove.as_bytes()).await;
        assert_eq!(receipt.status, 200);
        let receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&receipt.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(receipt.state, AdminIntentStateV1::Succeeded);
        assert_eq!(u64::from(next_close_without_authority(&mut b).await), expected["removedCloseCode"].as_u64().unwrap());
        let ServerFrame::Presence { peers } = next_server_frame_at(&mut c, "observer withdrawal presence after the admin removal").await else { panic!("removed member withdrawal") };
        assert_eq!(peers.len() as u64, expected["visibleAfterRemoval"].as_u64().unwrap());
        assert!(state.presence_snapshot(&document_scope_key_v1(&scope)).peers.is_empty());
        let exchange = semio_framework_pack_json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: plan_b.receipt.clone() });
        let denied = raw_http_request(addr, "POST", &format!("{root}/socket-grants"), &headers_b, exchange.as_bytes()).await;
        assert_recovery_denied(denied, expected);
        assert!(connect_async(document_socket_request(&url, &removed.token)).await.is_err(), "removed member cannot reuse its consumed grant");
        assert!(connect_async(document_socket_request(&url, &removed.token)).await.is_err(), "removal invalidates a previously unused member grant");
        for route in ["execution-target/manifest", "execution-target/component", "execution-target/descriptor", "open-plan"] {
            let denied = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_b, intent.as_bytes()).await;
            assert_recovery_denied(denied, expected);
        }
        c.send(client_binary(&ClientFrame::Presence { peer: raw }, Lane::Preview).await).await.unwrap();
        let ServerFrame::Presence { peers } = next_server_frame_at(&mut c, "observer presence after the removal, still live").await else { panic!("observer remains live") };
        assert_eq!(peers.len(), 1);
        let observer_peer = protocol::decode_presence_peer(&peers[0]).await.unwrap();
        assert_eq!(observer_peer.actor, actor_c);
        assert_eq!(observer_peer.user_id.as_deref(), Some(observer.user_id.as_str()));
        assert_eq!(observer_peer.role.as_deref(), Some("author"));
        assert_eq!(observer_peer.surface.as_deref(), Some(plan_c.surface.surface_id.as_str()));
        assert_eq!(u64::from(raw_http_request(addr, "POST", &format!("{root}/execution-target/manifest"), &headers_c, intent.as_bytes()).await.status), expected["observerStatus"].as_u64().unwrap());
        c.close(None).await.unwrap();
        drop(c);
        drop(b);
        stop_recovery_server(state, shutdown, server).await;
        assert_eq!(expected["sqliteReopen"], true);
        let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("reopen exact file directory");
        let mut state = test_state_with_directory(dir.join("db"), directory, 1024, 256).await;
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
        assert!(state.presence_snapshot(&document_scope_key_v1(&scope)).peers.is_empty());
        assert!(state.socket_grants.inner.lock().unwrap().records.is_empty());
        assert!(state.document_open_plans.inner.lock().unwrap().records.is_empty());
        for session in [&removed, &observer] {
            assert!(state.directory.authenticate_session(&SessionCapability::parse(&session.token).unwrap()).await.unwrap().is_some(), "membership removal must not be mistaken for session loss");
        }
        assert_eq!(state.directory.get_role(&scope.space_id, &removed.user_id).await.unwrap(), None);
        assert_eq!(state.directory.get_role(&scope.space_id, &observer.user_id).await.unwrap(), Some(SpaceRole::Author));
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for route in ["execution-target/manifest", "execution-target/component", "execution-target/descriptor", "open-plan"] {
            let denied = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_b, intent.as_bytes()).await;
            assert_recovery_denied(denied, expected);
            let admitted = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_c, intent.as_bytes()).await;
            assert_eq!(u64::from(admitted.status), expected["observerStatus"].as_u64().unwrap(), "reopened observer {route}");
            match route {
                "execution-target/component" => assert_eq!(admitted.body, TEST_EXECUTION_TARGET_COMPONENT_BYTES),
                "execution-target/descriptor" => assert_eq!(admitted.body, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES),
                "execution-target/manifest" => {
                    let manifest: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&admitted.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
                    manifest.validate().unwrap();
                    assert_eq!(manifest.scope, scope);
                }
                "open-plan" => {
                    let plan: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&admitted.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
                    plan.validate(now_ms() as u64).unwrap();
                    assert_eq!(plan.scope, scope);
                }
                _ => unreachable!(),
            }
        }
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn admin_intent_binding_wire_matrix_is_exact_sorted_and_self_deduplicated() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).unwrap();
    let principal = AdminPrincipalV1 {
        user_id: "admin".into(),
        auth_session_id: "admin-session".into(),
        authorization_generation: 1,
        identity_provider: "fixture".into(),
        identity_subject_digest: [0; 32],
        expires_at_ms: i64::MAX,
        correlation_id: "binding-fixture".into(),
        peer_class: "test",
        session_kind: AuthSessionKind::DevelopmentLocal,
        device_instance_id: "admin-device".into(),
    };
    for row in fixture["bindings"].as_array().unwrap() {
        let intent: AdminIntentV1 = semio_framework_pack_json::from_json_str(row["intentJson"].as_str().unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("actual closed administrator intent wire");
        let keys = admin_intent_bindings(&principal, &intent).map(|bindings| {
            bindings
                .into_iter()
                .map(|binding| match binding {
                    SocketBindingKeyV1::User(id) => format!("user:{id}"),
                    SocketBindingKeyV1::Session(id) => format!("session:{id}"),
                    SocketBindingKeyV1::DirectorySpaceAuthority { space_id } => format!("space:{space_id}"),
                    SocketBindingKeyV1::Membership { user_id, space_id } => format!("membership:{user_id}/{space_id}"),
                    SocketBindingKeyV1::Share(id) => format!("share:{id}"),
                })
                .collect::<Vec<_>>()
        });
        assert_eq!(serde_json::to_value(&keys).unwrap(), row["keys"], "exact sorted binding union: {}", row["name"]);
    }
}

#[test]
fn admin_short_effects_retain_principal_until_their_actual_side_effect() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).expect("admin short effect fixture");
        let root = tempdir("admin-short-authority");
        std::fs::create_dir_all(&root).expect("physical directory parent");
        let path = root.join("directory.sqlite");
        let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("physical directory");
        let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state_with_directory(root.join("db"), directory, 1024, 256)).await.expect("admin effect state open deadline");
        let physical = rusqlite::Connection::open(&path).expect("independent physical directory reader");
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let email = "admin-short-authority@example.test";
        let _headers = authorize_test_admin(&mut state, email).await;
        let owner = issue_test_session(&state, "admin-short-owner@example.test").await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["shortActions"].as_array().unwrap().iter().enumerate() {
            let admin = issue_test_session(&state, email).await;
            let target = issue_test_session(&state, &format!("admin-short-target-{index}@example.test")).await;
            let space = create_space_for_test(&state, &owner.user_id, "Short Authority", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let scope = DocumentScope::new(space, format!("admin-short-document-{index}"));
            announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
            let request_id = format!("request:admin-short-{index}");
            let sync_id = format!("sync:admin-short-{index}");
            let kick = Arc::new(tokio::sync::Notify::new());
            state.session_kicks.insert(sync_id.clone(), kick.clone());
            let action = row["action"].as_str().unwrap();
            let existing_share = if action == "revoke-share" { Some(state.directory.issue_share_token(&scope, 600, "fixture:existing-share").await.expect("existing target share")) } else { None };
            let intent = match action {
                "issue-share" => AdminIntentV1::IssueDocumentShare { request_id, scope: scope.clone(), ttl_secs: 600 },
                "revoke-share" => AdminIntentV1::RevokeDocumentShare { request_id, scope: scope.clone(), share_id: existing_share.as_ref().unwrap().record.id.clone(), reason_code: "test-revoke".into() },
                "revoke-user" => AdminIntentV1::RevokeUserSessions { request_id, user_id: target.user_id.clone(), reason_code: "test-revoke".into() },
                "kick" => AdminIntentV1::KickConnection { request_id, sync_session_id: sync_id.clone(), reason_code: "test-kick".into() },
                other => panic!("unknown short action {other}"),
            };
            let principal = authenticate_admin_principal(&state, &bearer_headers(&admin.token), None).await.expect("short action principal");
            let bindings = admin_intent_bindings(&principal, &intent).expect("short effect owns an authority union");
            assert_eq!(bindings.len() as u64, row["keys"].as_u64().unwrap(), "closed short action authority keys");
            let action_first = row["first"] == "action";
            *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), action_first));
            let command = tokio::spawn({
                let authorization = format!("Bearer {}", admin.token);
                let body = semio_framework_pack_json::to_json_string(&intent);
                async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("short action pause deadline").expect("short action pause").forget();
            let mut revoke = tokio::spawn({
                let state = state.clone();
                let token = admin.token.clone();
                async move { post_session_sign_out(bearer_headers(&token), State(state), Bytes::new()).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_session_revoke_attempted.acquire()).await.expect("short action revoke attempt deadline").expect("short action revoke attempt").forget();
            if action_first {
                assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted {} must own the principal authority", row["name"]);
                for binding in bindings {
                    assert!(state.socket_binding_gates.gate(binding).try_write_owned().is_err(), "every short effect key stays owned through its physical side effect");
                }
            } else {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("winning short action revoke deadline").expect("winning short action revoke"), StatusCode::NO_CONTENT);
            }
            *gate.directory_command_pause_user.lock().unwrap() = None;
            gate.directory_command_release.add_permits(1);
            let response = tokio::time::timeout(std::time::Duration::from_secs(5), command).await.expect("short action completion deadline").expect("short action task");
            assert_eq!(response.status, 200);
            let receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&response.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("short action receipt");
            assert_eq!(receipt.state, if row["effect"] == true { AdminIntentStateV1::Succeeded } else { AdminIntentStateV1::Cancelled }, "principal authority decides {}", row["name"]);
            if action_first {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing short action revoke deadline").expect("trailing short action revoke"), StatusCode::NO_CONTENT);
            }
            let effect: i64 = match action {
                "issue-share" => physical.query_row("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(),
                "revoke-share" => physical.query_row("SELECT count(*) FROM hub_share_grant WHERE id = ?1 AND revoked_at IS NOT NULL", [existing_share.as_ref().unwrap().record.id.as_str()], |row| row.get(0)).unwrap(),
                "revoke-user" => physical.query_row("SELECT count(*) FROM hub_auth_session WHERE user_id = ?1 AND revoked_at IS NOT NULL", [target.user_id.as_str()], |row| row.get(0)).unwrap(),
                "kick" => i64::from(tokio::time::timeout(std::time::Duration::from_millis(30), kick.notified()).await.is_ok()),
                _ => unreachable!(),
            };
            assert_eq!(effect, i64::from(row["effect"].as_bool().unwrap()), "exact physical effect: {}", row["name"]);
            let audit_count: i64 = physical.query_row("SELECT count(*) FROM hub_auth_audit WHERE correlation_id = ?1", [&receipt.correlation_id], |row| row.get(0)).unwrap();
            assert_eq!(audit_count, if action == "kick" { 0 } else { effect }, "no auth side-effect audit under revoked authority");
            let secret = receipt.result.as_ref().and_then(|result| result.share_token.as_ref());
            assert_eq!(secret.is_some(), row["secret"].as_bool().unwrap(), "plaintext share result follows the exact admitted effect");
            if let Some(secret) = secret {
                let capability = semio_hub_directory_contract::ShareCapability::parse(secret).expect("minted share capability");
                assert!(state.directory.authenticate_share(&scope, &capability).await.expect("minted share authentication"));
            }
            state.session_kicks.remove(&sync_id);
        }
        drop(physical);
        stop_recovery_server(state, shutdown, server).await;
    });
}
