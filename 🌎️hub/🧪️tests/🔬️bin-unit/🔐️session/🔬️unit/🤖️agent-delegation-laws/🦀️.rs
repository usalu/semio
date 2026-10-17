/// 🎫️ Laws of agent delegation: minted sessions, revocation, withdrawn delegations, audience bounds and vigilant concurrent edits.

//#region 🔖️CredentialSignIn
//#region 🔖️AgentDelegation
/// 🤖️ ticket 26/09/18 slice M6b — the end-to-end law M6 §9 gap 1 named as missing: delegate → agent
/// session → use → revoke → refused, plus the session cascade, against the real routes on a real
/// listener. Everything below is one HTTP conversation; nothing reaches into the directory to set up
/// a state a client could not have reached.
const AGENT_LABEL: &str = "Drafting agent";

fn agent_delegation_body(space_id: &str, audience: &str, ttl_secs: i64) -> Vec<u8> {
    serde_json::json!({ "schema": "semio.hub.auth.agent-delegation-create/v1", "spaceId": space_id, "agentLabel": AGENT_LABEL, "audience": audience, "ttlSecs": ttl_secs }).to_string().into_bytes()
}

fn agent_session_body(audience: &str) -> Vec<u8> {
    serde_json::json!({ "schema": "semio.hub.auth.agent-session/v1", "audience": audience, "agentInstanceId": "mcp.test.1" }).to_string().into_bytes()
}

/// 🔬️ The whole delegated-agent lifecycle over HTTP: a signed-in author mints a delegation, the
/// delegation is exchanged for a session whose kind is `agent` and whose principal is NOT the
/// delegating human's, that session authenticates an ordinary hub route with no special case, the
/// human revokes the delegation, and afterwards BOTH the already-minted session and any further
/// exchange are refused. The three durable facts land in the same log credential sign-in writes.
///
/// 1. delegate — the token is shown exactly once, and the receipt names the agent's own principal.
///
/// 2. the listing shows it, never its token, and reports it as not yet used.
///
/// 3. exchange — an ordinary session capability, whose kind is `agent`.
///
/// 4. use — the agent authenticates an ordinary route with no special case, and reports as an agent.
///
/// 4b. an agent can never delegate onward.
///
/// 4c. the listing now reports a last use, derived from the session it minted.
///
/// 5. revoke.
///
/// 6. refused — the already-minted session dies with the delegation (the cascade), and a further
///    exchange is refused with the cause a caller that proved the secret is entitled to know.
///
/// 7. every step is a durable fact in the same log credential sign-in writes.
#[tokio::test]
async fn an_agent_delegation_mints_a_session_that_works_until_it_is_revoked() {
    let state = test_state().await;
    let human = issue_test_session(&state, "delegator@example.com").await;
    let space_id = create_space_for_test(&state, &human.user_id, "Agent space", os_directory::DirectorySpaceKind::Atelier, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    let bearer = format!("Bearer {}", human.token);

    let created = raw_http_request(addr, "POST", "/auth/agent-delegations", &[("content-type", "application/json"), ("authorization", bearer.as_str())], &agent_delegation_body(&space_id, "edit", 3_600)).await;
    assert_eq!(created.status, 201, "{}", String::from_utf8_lossy(&created.body));
    assert!(created.headers.to_lowercase().contains("cache-control: no-store"));
    let receipt = json_body(&created);
    let delegation_id = receipt["delegationId"].as_str().expect("delegation id").to_string();
    let delegation_token = receipt["token"].as_str().expect("delegation token").to_string();
    assert_eq!(receipt["agentPrincipalId"].as_str(), Some(format!("agent:{delegation_id}").as_str()));
    assert_eq!(receipt["agentLabel"].as_str(), Some(AGENT_LABEL));
    assert!(delegation_token.starts_with("delegation.v1.") && delegation_token.len() == 111, "{delegation_token}");
    assert_ne!(receipt["agentPrincipalId"].as_str(), Some(format!("user:{}", human.user_id).as_str()), "an agent is never its delegating human");

    let listed = raw_http_get(addr, &format!("/auth/agent-delegations?space={space_id}"), &[("authorization", bearer.as_str())]).await;
    assert_eq!(listed.status, 200);
    let listing = json_body(&listed);
    assert_eq!(listing["delegations"].as_array().map(Vec::len), Some(1));
    let row = &listing["delegations"][0];
    assert_eq!(row["delegationId"].as_str(), Some(delegation_id.as_str()));
    assert_eq!(row["lastUsedAtMs"], serde_json::Value::Null, "a delegation that has never been exchanged reports no last use");
    assert!(!String::from_utf8_lossy(&listed.body).contains(&delegation_token), "a listing never carries the token");

    let delegation_bearer = format!("Bearer {delegation_token}");
    let minted = raw_http_request(addr, "POST", "/auth/agent-sessions", &[("content-type", "application/json"), ("authorization", delegation_bearer.as_str())], &agent_session_body("edit")).await;
    assert_eq!(minted.status, 200, "{}", String::from_utf8_lossy(&minted.body));
    let session = json_body(&minted);
    let agent_token = session["token"].as_str().expect("agent session token").to_string();
    assert_eq!(session["agentPrincipalId"].as_str(), Some(format!("agent:{delegation_id}").as_str()));
    assert_eq!(session["spaceId"].as_str(), Some(space_id.as_str()));
    assert_eq!(session["audience"].as_str(), Some("edit"));

    let agent_bearer = format!("Bearer {agent_token}");
    let me = raw_http_get(addr, "/auth/sessions/me", &[("authorization", agent_bearer.as_str())]).await;
    assert_eq!(me.status, 200, "{}", String::from_utf8_lossy(&me.body));
    let authority = json_body(&me);
    assert_eq!(authority["sessionKind"].as_str(), Some("agent"), "the agent's session names itself as one: {authority}");
    assert_eq!(authority["userId"].as_str(), Some(human.user_id.as_str()), "the agent acts under the delegating human's membership");

    let onward = raw_http_request(addr, "POST", "/auth/agent-delegations", &[("content-type", "application/json"), ("authorization", agent_bearer.as_str())], &agent_delegation_body(&space_id, "edit", 3_600)).await;
    assert_eq!(onward.status, 403);
    assert_eq!(json_body(&onward)["error"].as_str(), Some("forbidden"));

    let used = json_body(&raw_http_get(addr, &format!("/auth/agent-delegations?space={space_id}"), &[("authorization", bearer.as_str())]).await);
    assert!(used["delegations"][0]["lastUsedAtMs"].as_i64().is_some_and(|at| at > 0), "an exchanged delegation reports when it was last used: {used}");

    let revoked = raw_http_request(addr, "POST", &format!("/auth/agent-delegations/{delegation_id}/revoke"), &[("authorization", bearer.as_str())], &[]).await;
    assert_eq!(revoked.status, 204);

    assert_eq!(raw_http_get(addr, "/auth/sessions/me", &[("authorization", agent_bearer.as_str())]).await.status, 401, "revoking the delegation revokes every session it minted");
    let re_exchange = raw_http_request(addr, "POST", "/auth/agent-sessions", &[("content-type", "application/json"), ("authorization", delegation_bearer.as_str())], &agent_session_body("edit")).await;
    assert_eq!(re_exchange.status, 403);
    assert_eq!(json_body(&re_exchange)["error"].as_str(), Some("delegation-revoked"));
    assert_eq!(json_body(&raw_http_get(addr, &format!("/auth/agent-delegations?space={space_id}"), &[("authorization", bearer.as_str())]).await)["delegations"][0]["revoked"].as_bool(), Some(true), "a withdrawn delegation stays visible, marked");

    let audit = state.directory.list_auth_audit(64, 0).await.expect("auth audit");
    for event_kind in ["agent-delegated", "agent-session-issued", "agent-delegation-revoked", "session-revoked"] {
        assert!(audit.iter().any(|fact| fact.event_kind == event_kind), "the log carries `{event_kind}`");
    }
    assert!(audit.iter().any(|fact| fact.event_kind == "agent-session-issued" && fact.peer_class == "agent"), "an agent's session is journalled as an agent's");
}

/// 🤖️ Revoking a delegation ends the agent at once, never at its session's expiry: the agent's open
/// document socket closes `4401`, a human on the same document sees the agent's row leave the roster,
/// and the agent's next request is `401` — the same path a signed-out session takes.
#[test]
fn revoking_a_delegation_closes_the_agents_open_document_socket_and_roster_row() {
    run_socket_test(|| async {
        let state = test_state().await;
        let human = issue_test_session(&state, "revoking-human@example.com").await;
        let space_id = create_space_for_test(&state, &human.user_id, "Revoked agent space", os_directory::DirectorySpaceKind::Atelier, DirectorySpaceVisibility::Private).await;
        announce_document_for_test(&state, &space_id, "agent-doc").await;
        let addr = spawn_server(state.clone()).await;
        let bearer = format!("Bearer {}", human.token);
        let receipt = json_body(&raw_http_request(addr, "POST", "/auth/agent-delegations", &[("content-type", "application/json"), ("authorization", bearer.as_str())], &agent_delegation_body(&space_id, "edit", 3_600)).await);
        let delegation_id = receipt["delegationId"].as_str().expect("delegation id").to_string();
        let delegation_bearer = format!("Bearer {}", receipt["token"].as_str().expect("delegation token"));
        let minted = raw_http_request(addr, "POST", "/auth/agent-sessions", &[("content-type", "application/json"), ("authorization", delegation_bearer.as_str())], &agent_session_body("edit")).await;
        assert_eq!(minted.status, 200, "{}", String::from_utf8_lossy(&minted.body));
        let agent_token = json_body(&minted)["token"].as_str().expect("agent session token").to_string();
        let agent_bearer = format!("Bearer {agent_token}");

        let agent_grant = issue_document_socket_grant_fixture(Path((space_id.clone(), "agent-doc".to_string())), bearer_headers(&agent_token), State(state.clone())).await.expect("agent socket grant").0;
        issue_document_socket_grant_fixture(Path((space_id.clone(), "agent-doc".to_string())), bearer_headers(&human.token), State(state.clone())).await.expect("human socket grant");
        let url = format!("ws://{addr}/scopes/{space_id}%2Fagent-doc/document/ws");
        let (mut agent, _) = connect_async(document_socket_request(&url, &agent_token)).await.expect("agent document socket");
        let (mut human_socket, _) = connect_async(document_socket_request(&url, &human.token)).await.expect("human document socket");
        for socket in [&mut agent, &mut human_socket] {
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            assert!(matches!(next_server_frame_at(socket, "welcome").await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame_at(socket, "session").await, ServerFrame::Session { .. }));
        }
        let raw = presence_hex_bytes(presence_normalization_fixture()["vectors"][0]["rawPeerHex"].as_str().unwrap());
        agent.send(client_binary(&ClientFrame::Presence { peer: raw }, Lane::Preview).await).await.expect("agent presence");
        let ServerFrame::Presence { peers } = next_server_frame_at(&mut human_socket, "the human sees the agent's presence").await else { panic!("agent presence") };
        let mut roster = Vec::with_capacity(peers.len());
        for peer in &peers {
            roster.push(protocol::decode_presence_peer(peer).await.expect("presence peer").actor);
        }
        assert_eq!(roster, vec![agent_grant.actor_id.clone()], "the agent is its own roster row");

        let revoked_at = std::time::Instant::now();
        let revoked = raw_http_request(addr, "POST", &format!("/auth/agent-delegations/{delegation_id}/revoke"), &[("authorization", bearer.as_str())], &[]).await;
        assert_eq!(revoked.status, 204);
        assert_eq!(next_close_code(&mut agent, false).await, 4401, "the agent's open document socket closes on revocation");
        let ServerFrame::Presence { peers } = next_server_frame_at(&mut human_socket, "the human sees the agent leave").await else { panic!("agent withdrawal") };
        let mut roster = Vec::with_capacity(peers.len());
        for peer in &peers {
            roster.push(protocol::decode_presence_peer(peer).await.expect("presence peer").actor);
        }
        assert!(!roster.contains(&agent_grant.actor_id), "the revoked agent left the roster: {roster:?}");
        assert!(revoked_at.elapsed() < std::time::Duration::from_secs(5), "revocation reached the socket and the roster promptly: {:?}", revoked_at.elapsed());
        assert_eq!(raw_http_get(addr, "/auth/sessions/me", &[("authorization", agent_bearer.as_str())]).await.status, 401, "the agent's next request is refused");
        assert!(connect_async(document_socket_request(&url, &agent_token)).await.is_err(), "the revoked agent cannot open the document again");
    });
}

/// 🤖️ A delegation withdrawn in either order against the agent's edit has one outcome (G10 S4: on 7800 a connected
/// agent was refused in one run and still edited in the other). Withdrawn while an admitted edit is in flight: that
/// edit finishes first and is accepted — it precedes the withdrawal —, the `DELETE` answers only after it, and the
/// socket closes `4401`. Withdrawn first: the agent's next edit on its still-open socket is never accepted, its socket
/// closes `4401`, the document's head does not move, and its next request is `401`.
#[test]
fn a_withdrawn_delegation_admits_no_agent_edit_after_it_in_either_order() {
    run_socket_test(|| async {
        async fn delegated_agent(state: &HubState, addr: SocketAddr, human: &TestIssuedSession, space_id: &str, document: &str) -> (String, String, String, tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>) {
            let bearer = format!("Bearer {}", human.token);
            let receipt = json_body(&raw_http_request(addr, "POST", "/auth/agent-delegations", &[("content-type", "application/json"), ("authorization", bearer.as_str())], &agent_delegation_body(space_id, "edit", 3_600)).await);
            let delegation_id = receipt["delegationId"].as_str().expect("delegation id").to_string();
            let delegation_bearer = format!("Bearer {}", receipt["token"].as_str().expect("delegation token"));
            let minted = raw_http_request(addr, "POST", "/auth/agent-sessions", &[("content-type", "application/json"), ("authorization", delegation_bearer.as_str())], &agent_session_body("edit")).await;
            assert_eq!(minted.status, 200, "{}", String::from_utf8_lossy(&minted.body));
            let agent_token = json_body(&minted)["token"].as_str().expect("agent session token").to_string();
            let grant = issue_document_socket_grant_fixture(Path((space_id.to_string(), document.to_string())), bearer_headers(&agent_token), State(state.clone())).await.expect("agent socket grant").0;
            let (mut socket, _) = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{space_id}%2F{document}/document/ws"), &agent_token)).await.expect("agent document socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("agent hello");
            assert!(matches!(next_server_frame_at(&mut socket, "agent welcome").await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame_at(&mut socket, "agent session").await, ServerFrame::Session { .. }));
            (delegation_id, agent_token, grant.actor_id, socket)
        }
        let state = test_state().await;
        let human = issue_test_session(&state, "withdrawing-human@example.com").await;
        let space_id = create_space_for_test(&state, &human.user_id, "Withdrawn agent space", os_directory::DirectorySpaceKind::Atelier, DirectorySpaceVisibility::Private).await;
        announce_document_for_test(&state, &space_id, "withdraw-in-flight").await;
        announce_document_for_test(&state, &space_id, "withdraw-first").await;
        let addr = spawn_server(state.clone()).await;
        let bearer = format!("Bearer {}", human.token);

        let (delegation_id, agent_token, actor, mut agent) = delegated_agent(&state, addr, &human, &space_id, "withdraw-in-flight").await;
        let scope = DocumentScope::new(space_id.clone(), "withdraw-in-flight");
        let writer = state.socket_binding_gates.document_write(&scope).lock_owned().await;
        let mut edit = sample_envelope("withdraw-in-flight-edit", &WireArtifactId("withdraw-in-flight".into())).await;
        edit.actor = ActorId(actor.clone());
        agent.send(client_binary(&ClientFrame::Commands { batch_id: 41, envelopes: vec![edit] }, Lane::Command).await).await.expect("in-flight agent edit");
        let session = state.directory.authenticate_session(&SessionCapability::parse(&agent_token).expect("agent capability")).await.expect("agent session read").expect("agent session");
        let admitted = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            while state.socket_binding_gates.gate(SocketBindingKeyV1::Session(session.id.clone())).try_write_owned().is_ok() {
                tokio::time::sleep(std::time::Duration::from_millis(5)).await;
            }
        })
        .await;
        assert!(admitted.is_ok(), "the agent's edit is admitted and in flight, holding its session binding");
        let withdrawal = tokio::spawn({
            let bearer = bearer.clone();
            let path = format!("/auth/agent-delegations/{delegation_id}/revoke");
            async move { raw_http_request(addr, "POST", &path, &[("authorization", bearer.as_str())], &[]).await.status }
        });
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        assert!(!withdrawal.is_finished(), "the withdrawal waits for the edit the agent had already admitted");
        drop(writer);
        match next_server_frame_at(&mut agent, "in-flight edit ack").await {
            ServerFrame::Ack { batch_id: 41, stages, .. } => assert!(matches!(stages.last(), Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted)), "the admitted edit precedes the withdrawal: {stages:?}"),
            other => panic!("expected the admitted edit's Ack, got {other:?}"),
        }
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(5), withdrawal).await.expect("withdrawal deadline").expect("withdrawal task"), 204);
        assert_eq!(next_close_code(&mut agent, false).await, 4401, "the withdrawn agent's socket closes");
        assert_eq!(raw_http_get(addr, "/auth/sessions/me", &[("authorization", format!("Bearer {agent_token}").as_str())]).await.status, 401);

        let (delegation_id, agent_token, actor, mut agent) = delegated_agent(&state, addr, &human, &space_id, "withdraw-first").await;
        let db_id = db_artifact_id(&DocumentScope::new(space_id.clone(), "withdraw-first"));
        let before = state.db.document(&db_id).await.expect("withdraw-first handle").frontier().await.expect("withdraw-first frontier").commit_seq;
        assert_eq!(raw_http_request(addr, "POST", &format!("/auth/agent-delegations/{delegation_id}/revoke"), &[("authorization", bearer.as_str())], &[]).await.status, 204);
        let mut edit = sample_envelope("withdraw-first-edit", &WireArtifactId("withdraw-first".into())).await;
        edit.actor = ActorId(actor);
        let _ = agent.send(client_binary(&ClientFrame::Commands { batch_id: 42, envelopes: vec![edit] }, Lane::Command).await).await;
        let accepted = tokio::time::timeout(std::time::Duration::from_secs(3), async {
            loop {
                match agent.next().await {
                    Some(Ok(WsMessage::Binary(bytes))) => {
                        if let Ok((_, ServerFrame::Ack { batch_id: 42, stages, .. })) = protocol::decode_server_frame(&bytes).await {
                            return stages.iter().any(|stage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted)));
                        }
                    }
                    Some(Ok(WsMessage::Close(_))) | None | Some(Err(_)) => return false,
                    Some(Ok(_)) => {}
                }
            }
        })
        .await
        .unwrap_or(false);
        assert!(!accepted, "an edit sent after the withdrawal is never accepted");
        let after = state.db.document(&db_id).await.expect("withdraw-first handle").frontier().await.expect("withdraw-first frontier").commit_seq;
        assert_eq!(after, before, "the withdrawn agent's edit left the document unchanged");
        assert_eq!(raw_http_get(addr, "/auth/sessions/me", &[("authorization", format!("Bearer {agent_token}").as_str())]).await.status, 401);
    });
}

/// 🎚️ LAW (G11 P0, session 13): an agent session holds at most its delegation's audience, and only in its delegation's
/// one space, whatever its delegating author may do. On 7800 a `read` delegation's agent edited a hub note (the relay
/// acknowledged, head 0 → 1): the hub resolved every agent session to its human's membership role. A `read` agent now
/// resolves as a spectator — its edit on the document socket is never accepted and the head does not move —, an `edit`
/// agent as an author whose edit is accepted (the negative law), neither holds any role in another space of the same
/// human, and a directory socket binding reports the same capped role. The declared policy grants an agent only its
/// agent role (`agent-reader`/`agent-editor`): never the owner's or an author's administration — no rename, invite,
/// member change or archive, however much its human may do.
#[test]
fn an_agent_session_holds_at_most_its_delegations_audience_in_its_delegations_space() {
    run_socket_test(|| async {
        async fn agent(addr: SocketAddr, human: &TestIssuedSession, space_id: &str, audience: &str) -> String {
            let bearer = format!("Bearer {}", human.token);
            let receipt = json_body(&raw_http_request(addr, "POST", "/auth/agent-delegations", &[("content-type", "application/json"), ("authorization", bearer.as_str())], &agent_delegation_body(space_id, audience, 3_600)).await);
            let delegation_bearer = format!("Bearer {}", receipt["token"].as_str().expect("delegation token"));
            let minted = raw_http_request(addr, "POST", "/auth/agent-sessions", &[("content-type", "application/json"), ("authorization", delegation_bearer.as_str())], &agent_session_body(audience)).await;
            assert_eq!(minted.status, 200, "{}", String::from_utf8_lossy(&minted.body));
            json_body(&minted)["token"].as_str().expect("agent session token").to_string()
        }
        async fn edit_accepted(state: &HubState, addr: SocketAddr, space_id: &str, document: &str, token: &str, batch_id: u64) -> (bool, u64, u64) {
            let db_id = db_artifact_id(&DocumentScope::new(space_id.to_string(), document));
            let actor = issue_document_socket_grant_fixture(Path((space_id.to_string(), document.to_string())), bearer_headers(token), State(state.clone())).await.expect("agent socket grant").0.actor_id;
            let (mut socket, _) = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{space_id}%2F{document}/document/ws"), token)).await.expect("agent document socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("agent hello");
            assert!(matches!(next_server_frame_at(&mut socket, "agent welcome").await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame_at(&mut socket, "agent session").await, ServerFrame::Session { .. }));
            let before = state.db.document(&db_id).await.expect("document handle").frontier().await.expect("frontier").commit_seq;
            let mut edit = sample_envelope(&format!("{document}-agent-edit"), &WireArtifactId(document.into())).await;
            edit.actor = ActorId(actor);
            let _ = socket.send(client_binary(&ClientFrame::Commands { batch_id, envelopes: vec![edit] }, Lane::Command).await).await;
            let accepted = tokio::time::timeout(std::time::Duration::from_secs(5), async {
                loop {
                    match socket.next().await {
                        Some(Ok(WsMessage::Binary(bytes))) => {
                            if let Ok((_, ServerFrame::Ack { batch_id: acked, stages, .. })) = protocol::decode_server_frame(&bytes).await {
                                if acked == batch_id {
                                    return stages.iter().any(|stage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted)));
                                }
                            }
                        }
                        Some(Ok(WsMessage::Close(_))) | None | Some(Err(_)) => return false,
                        Some(Ok(_)) => {}
                    }
                }
            })
            .await
            .unwrap_or(false);
            let after = state.db.document(&db_id).await.expect("document handle").frontier().await.expect("frontier").commit_seq;
            (accepted, before, after)
        }
        let state = test_state().await;
        let human = issue_test_session(&state, "audience-human@example.com").await;
        let space_id = create_space_for_test(&state, &human.user_id, "Audience space", os_directory::DirectorySpaceKind::Atelier, DirectorySpaceVisibility::Private).await;
        let other_space = create_space_for_test(&state, &human.user_id, "Other space", os_directory::DirectorySpaceKind::Atelier, DirectorySpaceVisibility::Private).await;
        for document in ["read-agent-doc", "edit-agent-doc"] {
            announce_document_for_test(&state, &space_id, document).await;
        }
        announce_document_for_test(&state, &other_space, "other-doc").await;
        let addr = spawn_server(state.clone()).await;
        let reader = agent(addr, &human, &space_id, "read").await;
        let editor = agent(addr, &human, &space_id, "edit").await;

        for (who, token, role) in [("read", &reader, SpaceRole::Spectator), ("edit", &editor, SpaceRole::Author)] {
            let session = state.directory.authenticate_session(&SessionCapability::parse(token).expect("agent capability")).await.expect("agent session read").expect("agent session");
            assert_eq!(state.directory.principal_role(&space_id, session.principal()).await.expect("principal role"), Some(role), "{who}: its delegation's audience in its space");
            assert_eq!(state.directory.principal_role(&other_space, session.principal()).await.expect("principal role"), None, "{who}: nothing in another space of its human");
            assert!(
                matches!(state.directory.socket_session_binding(&session.id, &session.user_id, session.authorization_generation, Some(&space_id), now_ms()).await, Ok(SocketSessionBindingStatus::Active { role: Some(bound), .. }) if bound == role),
                "{who}: a socket binding reports the same capped role"
            );
            assert_eq!(state.directory.socket_session_binding(&session.id, &session.user_id, session.authorization_generation, Some(&other_space), now_ms()).await.expect("binding"), SocketSessionBindingStatus::MembershipLost, "{who}");
            for command in [
                DirectoryCommand::ArchiveSpace { space_id: space_id.clone() },
                DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Renamed by an agent".into() },
                DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: "invited-by-agent@example.com".into(), role: DirectorySpaceRole::Author },
                DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Author, ttl_secs: 3_600 },
            ] {
                assert_eq!(authorize_directory_command(&state, session.principal(), false, &command).await, Err(StatusCode::FORBIDDEN), "{who}: no administration: {command:?}");
            }
            assert!(issue_document_socket_grant_fixture(Path((other_space.clone(), "other-doc".to_string())), bearer_headers(token), State(state.clone())).await.is_err(), "{who}: no document of another space");
        }
        let human_session = state.directory.authenticate_session(&SessionCapability::parse(&human.token).expect("human capability")).await.expect("human session read").expect("human session");
        for command in [DirectoryCommand::ArchiveSpace { space_id: space_id.clone() }, DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Renamed by its owner".into() }] {
            assert_eq!(authorize_directory_command(&state, human_session.principal(), false, &command).await, Ok(()), "the owner itself keeps its authority: {command:?}");
        }

        let (accepted, before, after) = edit_accepted(&state, addr, &space_id, "read-agent-doc", &reader, 61).await;
        assert!(!accepted, "a read delegation's edit is never accepted");
        assert_eq!(after, before, "a read delegation's edit left the document unchanged");
        let (accepted, before, after) = edit_accepted(&state, addr, &space_id, "edit-agent-doc", &editor, 62).await;
        assert!(accepted, "an edit delegation's edit is accepted");
        assert!(after > before, "an edit delegation's edit moved the head: {before} -> {after}");
    });
}

/// ⚔️ Two authors write the same part of one opaque document over the hub's document sockets (fixture
/// `⚔️vigilant-concurrent-edit-v1`): a write whose author had not observed the other author's latest write to that
/// part is concurrent with it — `Vigilant` refuses it with the typed `mutation.clamped` warning, `Normal` accepts it
/// and reports it, a write that observed it is accepted. No write carries a dependency: `observed` is advisory.
#[test]
fn a_vigilant_hub_refuses_a_same_target_write_authored_without_observing_the_other_authors_latest() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/⚔️vigilant-concurrent-edit-v1/🔣️.json")).expect("vigilant fixture");
        for (index, row) in fixture["rows"].as_array().expect("rows").iter().enumerate() {
            let name = row["name"].as_str().expect("name");
            let mut state = test_state().await;
            state.merge_policy = match row["policy"].as_str().expect("policy") {
                "vigilant" => protocol::MergePolicy::Vigilant,
                "normal" => protocol::MergePolicy::Normal,
                other => panic!("undeclared policy {other}"),
            };
            let document = format!("vigilant-{index}");
            announce_document_for_test(&state, STUDIO, &document).await;
            let (alice, bob) = (seed_author_token(&state).await, seed_author_token(&state).await);
            let alice_actor = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document.clone())), bearer_headers(&alice), State(state.clone())).await.expect("alice grant").0.actor_id;
            let bob_actor = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document.clone())), bearer_headers(&bob), State(state.clone())).await.expect("bob grant").0.actor_id;
            assert_ne!(alice_actor, bob_actor, "{name}: two sessions are two authors");
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/scopes/{STUDIO}%2F{document}/document/ws");
            let (mut alice_socket, _) = connect_async(document_socket_request(&url, &alice)).await.expect("alice socket");
            let (mut bob_socket, _) = connect_async(document_socket_request(&url, &bob)).await.expect("bob socket");
            for socket in [&mut alice_socket, &mut bob_socket] {
                socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello");
                assert!(matches!(next_server_frame_at(socket, "vigilant welcome").await, ServerFrame::Welcome { .. }));
                assert!(matches!(next_server_frame_at(socket, "vigilant session").await, ServerFrame::Session { .. }));
            }
            let write = |id: &str, actor: &str, observed: Option<&str>| MutationEnvelope {
                mutation_id: protocol::MutationId(id.to_string()),
                document_id: WireArtifactId(document.clone()),
                actor: ActorId(actor.to_string()),
                dependencies: Vec::new(),
                observed: observed.map(|observed| protocol::MutationId(observed.to_string())),
                target: vec!["text".to_string()],
                diff: protocol::ArtifactDiff { schema: protocol::SchemaId("test.v1".into()), payload: id.as_bytes().to_vec() },
                inverse: protocol::InverseMutation { schema: protocol::SchemaId("test.v1".into()), payload: Vec::new() },
                timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
                transaction: None,
                verb: None,
                line: None,
            };
            let mut outcomes = Vec::new();
            for (author, batch, envelope) in [(0usize, 1u64, write("base", &alice_actor, None)), (1, 1, write("first", &bob_actor, Some("base"))), (0, 2, write("second", &alice_actor, row["secondObserves"].as_str()))] {
                let socket = if author == 0 { &mut alice_socket } else { &mut bob_socket };
                socket.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: vec![envelope] }, Lane::Command).await).await.expect("command");
                loop {
                    if let ServerFrame::Ack { batch_id, stages, .. } = next_server_frame_at(socket, "vigilant ack").await {
                        if batch_id == batch {
                            outcomes.push(stages.last().cloned().expect("applied stage"));
                            break;
                        }
                    }
                }
            }
            let accepted = |stage: &AckStage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted));
            assert!(accepted(&outcomes[0]) && accepted(&outcomes[1]), "{name}: the base and the observed first write are accepted: {outcomes:?}");
            match row["second"].as_str().expect("second") {
                "accepted" => assert!(accepted(&outcomes[2]), "{name}: {outcomes:?}"),
                "refused" => match &outcomes[2] {
                    AckStage::Applied { outcome } => match outcome.as_ref() {
                        ApplyOutcome::Rejected { messages, .. } => {
                            let messages: serde_json::Value = serde_json::from_slice(messages).expect("rejection messages are one JSON MutationMessage array");
                            assert!(messages.as_array().expect("message array").iter().any(|message| message["code"] == row["code"] && message["level"] == row["level"]), "{name}: {messages}");
                        }
                        other => panic!("{name}: expected a typed refusal, got {other:?}"),
                    },
                    other => panic!("{name}: expected an applied stage, got {other:?}"),
                },
                other => panic!("undeclared outcome {other}"),
            }
        }
    });
}

/// 🔬️ The refusals a delegation exchange must be unable to distinguish, and the one it must. A
/// wrong secret, an unknown selector and an audience WIDER than the one delegated are one
/// `invalid-delegation`, so a delegation id cannot be probed for existence and an agent cannot
/// promote itself; a narrower audience is admitted; and a delegation belonging to another human is
/// `403` rather than `404`, so a listing cannot be walked either.
#[tokio::test]
async fn an_agent_can_never_widen_its_own_audience_or_probe_another_humans_delegation() {
    let state = test_state().await;
    let human = issue_test_session(&state, "narrow@example.com").await;
    let stranger = issue_test_session(&state, "stranger@example.com").await;
    let space_id = create_space_for_test(&state, &human.user_id, "Narrow space", os_directory::DirectorySpaceKind::Atelier, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    let bearer = format!("Bearer {}", human.token);

    let receipt = json_body(&raw_http_request(addr, "POST", "/auth/agent-delegations", &[("content-type", "application/json"), ("authorization", bearer.as_str())], &agent_delegation_body(&space_id, "read", 3_600)).await);
    let delegation_id = receipt["delegationId"].as_str().expect("delegation id").to_string();
    let token = receipt["token"].as_str().expect("token").to_string();
    let delegation_bearer = format!("Bearer {token}");

    let widened = raw_http_request(addr, "POST", "/auth/agent-sessions", &[("content-type", "application/json"), ("authorization", delegation_bearer.as_str())], &agent_session_body("edit")).await;
    assert_eq!(widened.status, 401);
    assert_eq!(json_body(&widened)["error"].as_str(), Some("invalid-delegation"));

    let mut wrong_secret = token.clone();
    wrong_secret.replace_range(47..48, if token.as_bytes()[47] == b'a' { "b" } else { "a" });
    let forged = raw_http_request(addr, "POST", "/auth/agent-sessions", &[("content-type", "application/json"), ("authorization", format!("Bearer {wrong_secret}").as_str())], &agent_session_body("read")).await;
    assert_eq!(forged.status, 401);
    assert_eq!(json_body(&forged)["error"].as_str(), Some("invalid-delegation"), "a wrong secret and an unknown delegation are one refusal");

    let unknown = raw_http_request(addr, "POST", "/auth/agent-sessions", &[("content-type", "application/json"), ("authorization", format!("Bearer delegation.v1.{}.{}", "0".repeat(32), "0".repeat(64)).as_str())], &agent_session_body("read")).await;
    assert_eq!(unknown.status, 401);
    assert_eq!(json_body(&unknown)["error"].as_str(), Some("invalid-delegation"));

    assert_eq!(raw_http_request(addr, "POST", "/auth/agent-sessions", &[("content-type", "application/json"), ("authorization", delegation_bearer.as_str())], &agent_session_body("read")).await.status, 200, "the audience that WAS delegated still mints");

    let stranger_bearer = format!("Bearer {}", stranger.token);
    let stolen = raw_http_request(addr, "POST", &format!("/auth/agent-delegations/{delegation_id}/revoke"), &[("authorization", stranger_bearer.as_str())], &[]).await;
    assert_eq!(stolen.status, 403, "another human's delegation is forbidden, never not-found");
    let stranger_listing = json_body(&raw_http_get(addr, &format!("/auth/agent-delegations?space={space_id}"), &[("authorization", stranger_bearer.as_str())]).await);
    assert_eq!(stranger_listing["delegations"].as_array().map(Vec::len), Some(0), "a listing is per delegating human, not per space");
}

/// 🔬️ A spectator cannot manufacture an editing principal out of a read-only membership, and a
/// non-member cannot delegate into a space at all: both are `403` before anything is minted.
#[tokio::test]
async fn only_an_author_of_the_space_can_delegate_to_an_agent() {
    let watchdog = LawHangWatchdogV1::arm("only_an_author_of_the_space_can_delegate_to_an_agent");
    watchdog.at("hub state open");
    let state = bounded_law_step("author delegation law: hub state open", test_state()).await;
    watchdog.at("owner session");
    let owner = bounded_law_step("author delegation law: owner session", issue_test_session(&state, "owner@example.com")).await;
    watchdog.at("spectator session");
    let spectator = bounded_law_step("author delegation law: spectator session", issue_test_session(&state, "spectator@example.com")).await;
    watchdog.at("outsider session");
    let outsider = bounded_law_step("author delegation law: outsider session", issue_test_session(&state, "outsider@example.com")).await;
    watchdog.at("space creation");
    let space_id =
        bounded_law_step("author delegation law: space creation", create_space_for_test(&state, &owner.user_id, "Author space", os_directory::DirectorySpaceKind::Atelier, DirectorySpaceVisibility::Private)).await;
    watchdog.at("spectator membership");
    bounded_law_step("author delegation law: spectator membership", upsert_member_for_test(&state, &space_id, "spectator@example.com", DirectorySpaceRole::Spectator)).await;
    watchdog.at("server spawn");
    let addr = spawn_server(state.clone()).await;

    for (who, token) in [("spectator", &spectator.token), ("outsider", &outsider.token)] {
        watchdog.at("refused delegation request");
        let refused = bounded_http_request(addr, "POST", "/auth/agent-delegations", &[("content-type", "application/json"), ("authorization", format!("Bearer {token}").as_str())], &agent_delegation_body(&space_id, "edit", 3_600)).await;
        assert_eq!(refused.status, 403, "{who}");
        assert_eq!(json_body(&refused)["error"].as_str(), Some("forbidden"), "{who}");
    }
    watchdog.at("delegation listing");
    let listing = bounded_law_step("author delegation law: delegation listing", state.directory.list_agent_delegations(&space_id, &spectator.user_id, 16)).await.expect("listing");
    assert_eq!(listing.len(), 0, "a refused delegation is never written");
    watchdog.at("body complete, tearing down the hub state and its worker pool");
}
//#endregion 🔖️AgentDelegation
//#endregion 🔖️CredentialSignIn
