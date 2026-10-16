
use std::sync::Arc;
use directory::os_directory::DocumentScope;
use semio_hub_runtime::presence::{PRESENCE_LEASE_TTL_MS, PresenceLeaseSlot, PresenceLeaseTransition, document_scope_key_v1};
use semio_hub_route_testkit::{STUDIO, announce_document_for_test, artifact_document_id_for_test, bearer_headers, client_binary, document_socket_request, install_document_open_catalog_for_test, issue_and_exchange_document_open_plan_for_test, issue_test_session, lag_test_state, next_server_frame, observed_test_state, presence_hex_bytes, presence_normalization_fixture, run_socket_test, seed_author_token, seed_genesis_for_document_for_test, socket_hello, spawn_server, test_state};
use semio_hub_directory_contract::SessionCapability;
use semio_hub_ledgers::socket_grants::SocketAudienceV1;
use semio_hub_session_routes::access_grants::authenticate_document_credential;
use tokio_tungstenite::connect_async;
use semio_hub_document_routes::directory_events::directory_message_visible;
use semio_hub_runtime::support::now_ms;
use semio_hub_session_routes::directory_commands::resolve_bearer_user;
use semio_hub_directory::{AuthSessionDirectory, DocumentDirectory, EventLog, UserDirectory};
#[cfg(test)]
use protocol::{ClientFrame, Lane, ServerFrame};
#[cfg(test)]
use directory::os_directory::DirectoryStreamMessage;
#[cfg(test)]
use directory::os_spr::channel::{PRESENCE_ROSTER_MAXIMUM_BYTES, PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES, PRESENCE_ROSTER_MAXIMUM_ITEMS};
#[cfg(test)]
use axum::extract::{Path, State};
#[cfg(test)]
use semio_framework_trace::record::SERVER_SPAN_EVENTS;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_runtime::presence::TestPresenceClock;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::issue_document_socket_grant_fixture;
fn test_presence_slot(live: &str, user_id: Option<&str>, now: tokio::time::Instant) -> PresenceLeaseSlot {
    PresenceLeaseSlot {
        socket_live_id: live.into(),
        expires_at: now + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS),
        user_id: user_id.map(str::to_string),
        connected_at_ms: 1000,
        label: None,
        role: None,
        document_surface: Some("surface".into()),
        color: 0,
        principal_kind: protocol::PresencePrincipalKind::Human,
        peer: None,
    }
}

async fn presence_test_peer(pack: &[u8]) -> Vec<u8> {
    let fixture = presence_normalization_fixture();
    let mut peer = protocol::decode_presence_peer(&presence_hex_bytes(fixture["vectors"][0]["rawPeerHex"].as_str().expect("raw peer"))).await.expect("fixture peer");
    peer.presence_pack = Some(pack.to_vec());
    protocol::encode_presence_peer(&peer).await
}

async fn presence_frame_has_pack(frame: ServerFrame, pack: &[u8]) -> bool {
    let ServerFrame::Presence { peers } = frame else { return false };
    peers.len() == 1 && protocol::decode_presence_peer(&peers[0]).await.is_ok_and(|peer| peer.presence_pack.as_deref() == Some(pack))
}

#[tokio::test]
async fn presence_normalization_matches_neutral_authority_and_no_effect_rejections() {
    let state = lag_test_state(1024, 256).await;
    let fixture = presence_normalization_fixture();
    let head = state.directory.head_seq().await.expect("directory head");
    let now = tokio::time::Instant::now();
    for vector in fixture["vectors"].as_array().expect("vectors") {
        let document_id = vector["name"].as_str().expect("vector name");
        let key = document_scope_key_v1(&DocumentScope::new(STUDIO, document_id));
        let admitted = &vector["admission"];
        let actor = admitted["actor"].as_str().expect("admitted actor");
        let mut slot = test_presence_slot("live", admitted["userId"].as_str(), now);
        slot.connected_at_ms = admitted["connectedAtMs"].as_i64().expect("admitted time");
        slot.label = admitted["label"].as_str().map(str::to_string);
        slot.role = admitted["role"].as_str().map(str::to_string);
        slot.color = u8::try_from(admitted["color"].as_u64().expect("admitted color")).expect("palette index");
        slot.document_surface = admitted["surface"].as_str().map(str::to_string);
        let deadline = slot.expires_at;
        let mut receiver = state.fanout_for(&key).subscribe();
        assert_eq!(state.install_presence_slot(&key, STUDIO, document_id, actor, slot).await, PresenceLeaseTransition::NoChange);
        let raw = presence_hex_bytes(vector["rawPeerHex"].as_str().expect("raw peer"));
        let transition = state.refresh_document_presence(&key, STUDIO, document_id, actor, "live", raw.clone(), now + std::time::Duration::from_secs(1)).await;
        let map_key = (key.clone(), actor.to_string());
        if vector["expected"]["accepted"] == true {
            let expected = presence_hex_bytes(vector["expected"]["normalizedPeerHex"].as_str().expect("normalized peer"));
            assert_eq!(transition, PresenceLeaseTransition::Published, "{document_id}");
            assert_eq!(state.presence_snapshot(&key).peers, vec![expected.clone()], "{document_id}");
            assert_eq!(state.presence_snapshot(&key).actors.len(), usize::from(admitted["surface"].is_string()), "non-plan rows cannot mint a directory surface: {document_id}");
            assert!(matches!(receiver.try_recv(), Ok(ServerFrame::Presence { peers }) if peers == vec![expected]));
            assert_eq!(state.refresh_document_presence(&key, STUDIO, document_id, actor, "live", raw, now + std::time::Duration::from_secs(2)).await, PresenceLeaseTransition::NoChange);
            assert_eq!(state.presence.with(&map_key, |slot| slot.expect("slot").expires_at), deadline + std::time::Duration::from_secs(2));
        } else {
            assert_eq!(transition, PresenceLeaseTransition::Rejected, "{document_id}");
            assert!(state.presence_snapshot(&key).peers.is_empty(), "{document_id}");
            assert_eq!(state.presence.with(&map_key, |slot| slot.expect("slot").expires_at), deadline, "{document_id}");
        }
        assert!(matches!(receiver.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "no duplicate or rejected publication: {document_id}");
        assert_eq!(vector["expected"]["durableWrites"], 0);
    }
    assert_eq!(state.directory.head_seq().await.expect("directory head"), head);
}

#[test]
fn presence_normalization_socket_overwrites_identity_and_rejects_without_refresh() {
    run_socket_test(|| async {
        let mut state = lag_test_state(1024, 256).await;
        let clock = Arc::new(TestPresenceClock::new());
        state.presence_clock = Some(clock.clone());
        let token = seed_author_token(&state).await;
        let document_id = artifact_document_id_for_test("presence-normalized-socket");
        let document_id = document_id.as_str();
        seed_genesis_for_document_for_test(&state, &token, STUDIO, document_id, "presence-normalized-socket").await;
        let scope = DocumentScope::new(STUDIO, document_id);
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
        install_document_open_catalog_for_test(&mut state, &descriptor);
        let (plan, receipt) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-normalization").await;
        let session = state.directory.authenticate_session(&SessionCapability::parse(&token).expect("session capability")).await.expect("session lookup").expect("session");
        let user = state.directory.get_user(&session.user_id).await.expect("user lookup").expect("user");
        let started = now_ms();
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2F{document_id}/document/ws?surface={}", plan.surface.surface_id);
        let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("admitted socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        let ServerFrame::Session { actor, color } = next_server_frame(&mut socket).await else { panic!("session frame") };
        assert_eq!(actor, receipt.actor_id);
        let fixture = presence_normalization_fixture();
        let raw = presence_hex_bytes(fixture["vectors"][0]["rawPeerHex"].as_str().expect("raw peer"));
        let input = protocol::decode_presence_peer(&raw).await.expect("raw peer decode");
        socket.send(client_binary(&ClientFrame::Presence { peer: raw.clone() }, Lane::Preview).await).await.expect("forged peer send");
        let ServerFrame::Presence { peers } = next_server_frame(&mut socket).await else { panic!("presence frame") };
        assert_eq!(peers.len(), 1);
        let normalized = protocol::decode_presence_peer(&peers[0]).await.expect("normalized peer");
        assert_eq!(normalized.actor, actor);
        assert_eq!(normalized.user_id, Some(session.user_id));
        assert_eq!(normalized.label, Some(user.display_name));
        assert_eq!(normalized.role.as_deref(), Some("author"));
        assert_eq!(normalized.color, Some(color));
        assert_eq!(normalized.surface, Some(plan.surface.surface_id));
        assert!(normalized.connected_at_ms >= started && normalized.connected_at_ms <= now_ms());
        assert_eq!(normalized.presence_pack, input.presence_pack);
        assert_eq!(normalized.drag_ghost_json, input.drag_ghost_json);
        assert_eq!(normalized.interaction, input.interaction);
        assert_eq!(normalized.views, input.views);
        assert_eq!(normalized.ui, input.ui);
        assert_eq!(normalized.tool_run, input.tool_run);
        assert_eq!(protocol::encode_presence_peer(&normalized).await, peers[0]);
        let key = document_scope_key_v1(&scope);
        assert_eq!(state.presence_snapshot(&key).actors[0].surface, normalized.surface.as_deref().expect("plan surface"));
        let deadline = state.presence.with(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").expires_at);
        let head = state.directory.head_seq().await.expect("head before rejected input");
        clock.advance_to(5_000);
        for vector in fixture["vectors"]
            .as_array()
            .expect("vectors")
            .iter()
            .filter(|vector| !vector["expected"]["accepted"].as_bool().expect("accepted") && !vector["name"].as_str().expect("name").starts_with("authoritative-") && !vector["name"].as_str().expect("name").starts_with("normalized-"))
        {
            socket.send(client_binary(&ClientFrame::Presence { peer: presence_hex_bytes(vector["rawPeerHex"].as_str().expect("hostile peer")) }, Lane::Preview).await).await.expect("hostile send");
        }
        socket.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-fifo".into(), seq: 1, payload: vec![] }, Lane::Preview).await).await.expect("rejection fence");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq: 1, .. } if key == "presence-fifo"), "every rejected frame precedes the FIFO marker without an interim roster");
        assert_eq!(state.presence.with(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").expires_at), deadline);
        assert_eq!(state.presence_snapshot(&key).peers, peers);
        assert_eq!(state.directory.head_seq().await.expect("head after rejected input"), head);
        for (sequence, name, label_bytes) in [(2, "authoritative-field-over-limit", 1025), (3, "normalized-entry-over-limit", 1024)] {
            let vector = fixture["vectors"].as_array().expect("vectors").iter().find(|vector| vector["name"] == name).expect("authority expansion vector");
            let saved_label = state.presence.with_mut(&(key.clone(), actor.clone()), |slot| std::mem::replace(&mut slot.expect("visible slot").label, Some("l".repeat(label_bytes))));
            socket.send(client_binary(&ClientFrame::Presence { peer: presence_hex_bytes(vector["rawPeerHex"].as_str().expect("expansion raw peer")) }, Lane::Preview).await).await.expect("authority-expanded input");
            socket.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-fifo".into(), seq: sequence, payload: vec![] }, Lane::Preview).await).await.expect("expansion fence");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq, .. } if key == "presence-fifo" && seq == sequence), "authority-expanded output is rejected before publication: {name}");
            assert_eq!(state.presence.with(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").expires_at), deadline);
            assert_eq!(state.presence_snapshot(&key).peers, peers);
            state.presence.with_mut(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").label = saved_label);
        }
        socket.send(client_binary(&ClientFrame::Presence { peer: raw }, Lane::Preview).await).await.expect("identical refresh");
        socket.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-fifo".into(), seq: 4, payload: vec![] }, Lane::Preview).await).await.expect("duplicate fence");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq: 4, .. } if key == "presence-fifo"), "identical normalized peer does not republish");
        assert_eq!(state.presence.with(&(key, actor), |slot| slot.expect("visible slot").expires_at), deadline + std::time::Duration::from_secs(5));
        socket.close(None).await.expect("close socket");
    });
}

#[test]
fn presence_lease_reconnect_rejects_old_live_refresh_and_close() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        state.presence_clock = Some(Arc::new(TestPresenceClock::new()));
        let token = seed_author_token(&state).await;
        let document_id = artifact_document_id_for_test("presence-reconnect");
        let document_id = document_id.as_str();
        seed_genesis_for_document_for_test(&state, &token, STUDIO, document_id, "presence-reconnect").await;
        let scope = DocumentScope::new(STUDIO, document_id);
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
        install_document_open_catalog_for_test(&mut state, &descriptor);
        let observer_token = seed_author_token(&state).await;
        let (plan, first) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-first").await;
        let (_, second) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-second").await;
        let (_, observer_grant) = issue_and_exchange_document_open_plan_for_test(&state, &observer_token, &scope, "client:presence-observer").await;
        let (first_subject, _) = authenticate_document_credential(&state, &scope, &token).await.expect("first credential");
        let first_record = state.socket_grants.pending_document_binding(&SocketAudienceV1::Document(scope.clone()), &first_subject.binding(), now_ms()).expect("first pending record");
        assert_eq!(first_record.actor_id, first.actor_id);
        assert_eq!(first.actor_id, second.actor_id);
        assert_ne!(first.actor_id, observer_grant.actor_id);
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2F{document_id}/document/ws?surface={}", plan.surface.surface_id);
        let (mut observer, _) = connect_async(document_socket_request(&url, &observer_token)).await.expect("observer socket");
        observer.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("observer hello");
        assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Session { .. }));
        let (mut socket_a, _) = connect_async(document_socket_request(&url, &token)).await.expect("first socket");
        socket_a.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("first hello");
        assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Session { actor, .. } if actor == first.actor_id));
        socket_a.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"old-live").await }, Lane::Preview).await).await.expect("first presence");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket_a).await, b"old-live").await);
        assert!(presence_frame_has_pack(next_server_frame(&mut observer).await, b"old-live").await);
        let key = document_scope_key_v1(&scope);
        let first_live = state.presence.with(&(key.clone(), first.actor_id.clone()), |slot| slot.expect("first slot").socket_live_id.clone());

        let (mut socket_b, _) = connect_async(document_socket_request(&url, &token)).await.expect("replacement socket");
        socket_b.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("replacement hello");
        assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Session { actor, .. } if actor == second.actor_id));
        assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Presence { peers } if peers.is_empty()), "replacement removes the old visible row");
        assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Presence { peers } if peers.is_empty()));
        socket_a.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"stale-refresh").await }, Lane::Preview).await).await.expect("stale refresh");
        socket_a.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-stale-fifo".into(), seq: 1, payload: vec![] }, Lane::Preview).await).await.expect("stale owner fence");
        assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Preview { seq: 1, .. }));
        assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Preview { seq: 1, .. }));
        assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Preview { seq: 1, .. }));
        socket_b.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"current-live").await }, Lane::Preview).await).await.expect("current refresh");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket_b).await, b"current-live").await);
        assert!(presence_frame_has_pack(next_server_frame(&mut observer).await, b"current-live").await);
        socket_a.close(None).await.expect("stale socket close");
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while state.socket_grants.is_live(&first_record, &first_live) {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            }
        })
        .await
        .expect("old socket cleanup completed");
        socket_b.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"current-live-2").await }, Lane::Preview).await).await.expect("current refresh after stale close");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket_b).await, b"current-live-2").await);
        assert!(presence_frame_has_pack(next_server_frame(&mut observer).await, b"current-live-2").await);
        let snapshot = state.presence_snapshot(&key);
        assert_eq!(snapshot.actors.len(), 1);
        assert_eq!(snapshot.actors[0].surface, plan.surface.surface_id);
        assert_eq!(snapshot.actors[0].actor, second.actor_id);
        observer.close(None).await.expect("observer close");
        socket_b.close(None).await.expect("current socket close");
    });
}

/// 👥️ ticket 26/09/18 slice PR1: a lapsed beat lease means the app-owned EPHEMERALS are
/// stale, never that the human left — the row a live socket contributes stays, stripped, and
/// only `close_presence_for_live` removes it. A roster that emptied itself under an open
/// socket is exactly what C3 §3.4 measured in two browsers.
#[test]
fn presence_lease_expires_server_clocked_visibility_without_socket_close() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let clock = Arc::new(TestPresenceClock::new());
        state.presence_clock = Some(clock.clone());
        let token = seed_author_token(&state).await;
        let document_id = "presence-expiry";
        announce_document_for_test(&state, STUDIO, document_id).await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document_id.to_string())), bearer_headers(&token), State(state.clone())).await.expect("socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2F{document_id}/document/ws");
        let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("presence socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        let ServerFrame::Session { actor, color } = next_server_frame(&mut socket).await else { panic!("session frame") };
        socket.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"visible").await }, Lane::Preview).await).await.expect("visible presence");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket).await, b"visible").await);
        let key = document_scope_key_v1(&DocumentScope::new(STUDIO, document_id));
        let mut observed = state.fanout_for(&key).subscribe();
        clock.gate_ticks.store(true, std::sync::atomic::Ordering::Release);
        clock.advance_to(PRESENCE_LEASE_TTL_MS - 1);
        clock.evaluate_tick(false).await;
        assert_eq!(state.presence_snapshot(&key).peers.len(), 1, "an evaluated server tick preserves the lease immediately before its deadline");
        assert!(matches!(observed.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "the evaluated early tick publishes no expiry");
        clock.advance_to(PRESENCE_LEASE_TTL_MS);
        clock.evaluate_tick(false).await;
        let stripped = state.presence_snapshot(&key).peers;
        assert_eq!(stripped.len(), 1, "an open socket keeps its admitted row past the beat lease");
        let stripped_peer = protocol::decode_presence_peer(&stripped[0]).await.expect("stripped row decodes");
        assert_eq!(stripped_peer.actor, actor, "the stripped row is the same admitted actor");
        assert_eq!(stripped_peer.color, Some(color), "identity the hub authenticated survives the lapse");
        assert_eq!(stripped_peer.presence_pack, None, "every app-owned ephemeral is dropped");
        assert_eq!(stripped_peer.drag_ghost_json, None);
        assert_eq!(stripped_peer.interaction, None);
        assert!(stripped_peer.views.is_empty());
        assert_eq!(stripped_peer.ui, None);
        assert_eq!(stripped_peer.tool_run, None);
        assert!(matches!(observed.try_recv(), Ok(ServerFrame::Presence { peers }) if peers == stripped));
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Presence { peers } if peers == stripped), "the server tick publishes exact-deadline stripping");
        clock.advance_to(PRESENCE_LEASE_TTL_MS + 1);
        clock.evaluate_tick(true).await;
        assert_eq!(clock.tick_release.available_permits(), 0, "ungating leaves no stale release permit");
        assert_eq!(state.presence_snapshot(&key).peers, stripped, "a re-armed lease does not re-strip an already stripped row");
        assert!(matches!(observed.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "stripping publishes exactly once per lapse");
        socket.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"revived").await }, Lane::Preview).await).await.expect("live socket refresh after visibility expiry");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket).await, b"revived").await, "expiry does not close or unregister the authenticated socket");
    });
}

/// 👥️ ticket 26/09/18 slice PR1 — presence over the hub is SYMMETRIC: a socket that attaches after
/// the roster has settled starts from that roster instead of waiting for some other peer to move.
///
/// 🔁️ The joiner beats NOTHING here. Before this law a roster delta was only ever published
/// when some peer's bytes CHANGED, so the joiner stayed blind until one of them moved —
/// measured on this exact handler in `🗑️generated/pr1-before-hub-socket.txt` run A as
/// `user2:n=0 frames=0` two seconds after its socket opened, while `user1` already listed a
/// peer. That is C3 §3.4's asymmetry, read from the hub's own side.
///
/// 🤝️ And the settled peer learns of the join from the joiner's first beat, so both sockets
/// end on the SAME two-row roster with one palette index each.
///
/// 🧹️ Closing one socket removes exactly that row from the other's roster.
#[test]
fn presence_join_replays_the_settled_roster_and_close_removes_exactly_one_row() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let token = seed_author_token(&state).await;
        let document_id = artifact_document_id_for_test("presence-join-replay");
        let document_id = document_id.as_str();
        seed_genesis_for_document_for_test(&state, &token, STUDIO, document_id, "presence-join-replay").await;
        let scope = DocumentScope::new(STUDIO, document_id);
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
        install_document_open_catalog_for_test(&mut state, &descriptor);
        let joiner_token = seed_author_token(&state).await;
        let (plan, settled_grant) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-settled").await;
        let (_, joiner_grant) = issue_and_exchange_document_open_plan_for_test(&state, &joiner_token, &scope, "client:presence-joiner").await;
        assert_ne!(settled_grant.actor_id, joiner_grant.actor_id, "two humans, two admitted actors");
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2F{document_id}/document/ws?surface={}", plan.surface.surface_id);
        let (mut settled, _) = connect_async(document_socket_request(&url, &token)).await.expect("settled socket");
        settled.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("settled hello");
        assert!(matches!(next_server_frame(&mut settled).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut settled).await, ServerFrame::Session { .. }));
        settled.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"settled").await }, Lane::Preview).await).await.expect("settled beat");
        assert!(presence_frame_has_pack(next_server_frame(&mut settled).await, b"settled").await);

        let (mut joining, _) = connect_async(document_socket_request(&url, &joiner_token)).await.expect("joining socket");
        joining.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("joining hello");
        assert!(matches!(next_server_frame(&mut joining).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut joining).await, ServerFrame::Session { .. }));
        let ServerFrame::Presence { peers } = next_server_frame(&mut joining).await else { panic!("join replay frame") };
        assert_eq!(peers.len(), 1, "the joiner starts from the roster the document already had");
        let replayed = protocol::decode_presence_peer(&peers[0]).await.expect("replayed row");
        assert_eq!(replayed.actor, settled_grant.actor_id);
        assert_eq!(replayed.presence_pack.as_deref(), Some(b"settled".as_slice()));
        assert_eq!(replayed.surface.as_deref(), Some(plan.surface.surface_id.as_str()));

        joining.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"joined").await }, Lane::Preview).await).await.expect("joiner beat");
        let ServerFrame::Presence { peers: settled_view } = next_server_frame(&mut settled).await else { panic!("settled roster") };
        let ServerFrame::Presence { peers: joining_view } = next_server_frame(&mut joining).await else { panic!("joiner roster") };
        assert_eq!(settled_view, joining_view, "both sockets hold the identical roster");
        assert_eq!(settled_view.len(), 2);
        let mut colors = Vec::new();
        for row in &settled_view {
            colors.push(protocol::decode_presence_peer(row).await.expect("roster row").color.expect("admitted palette index"));
        }
        colors.sort_unstable();
        colors.dedup();
        assert_eq!(colors.len(), 2, "each admitted session holds its own palette index");

        joining.close(None).await.expect("joiner close");
        let ServerFrame::Presence { peers: after_close } = next_server_frame(&mut settled).await else { panic!("removal roster") };
        assert_eq!(after_close.len(), 1);
        assert_eq!(protocol::decode_presence_peer(&after_close[0]).await.expect("survivor row").actor, settled_grant.actor_id);
        settled.close(None).await.expect("settled close");
    });
}

/// 💓️ ticket 26/09/18 slice PR1 — the beat/eviction contract, read from the SAME file the OS
/// shell's own law reads (`📺️renderer/🧑‍🎨engine/🧪️tests/👥️scoped-presence/🟦️.tsx`), so the two
/// languages cannot drift apart on the three numbers that decide whether a live socket keeps its
/// roster row.
#[tokio::test]
async fn presence_liveness_contract_matches_the_shared_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../🧰️framework/🔨️modules/📡️replication/🧪️tests/🧫️fixtures/💓️presence-liveness-v1/🔣️.json")).expect("presence liveness fixture");
    assert_eq!(fixture["schema"], "semio.presence.liveness/v1");
    let constants = &fixture["constants"];
    let heartbeat = constants["heartbeatIntervalMs"].as_u64().expect("heartbeat interval");
    let snapshot_deadline = constants["ephemeralSnapshotDeadlineMs"].as_u64().expect("snapshot deadline");
    assert_eq!(constants["leaseTtlMs"].as_u64(), Some(PRESENCE_LEASE_TTL_MS), "the hub owns leaseTtlMs");
    let law = |name: &str| fixture["laws"].as_array().expect("laws").iter().find(|law| law["name"] == name).expect("named law").clone();
    assert!(snapshot_deadline < heartbeat, "{}", law("snapshot-deadline-fits-inside-one-beat")["statement"]);
    let tolerated = law("lease-outlives-three-beats")["missedBeatsTolerated"].as_u64().expect("tolerated beats");
    assert!(PRESENCE_LEASE_TTL_MS >= tolerated * heartbeat, "{}", law("lease-outlives-three-beats")["statement"]);
    let live_row = law("a-live-socket-is-always-a-row");
    assert_eq!(live_row["removesRow"].as_array().expect("removal causes").len(), 1, "only a socket close removes a row");
    assert_eq!(live_row["removesRow"][0], "socket-close");
    assert_eq!(live_row["stripsEphemeralsOnly"][0], "lease-lapse");
    assert_eq!(law("a-joiner-starts-from-the-settled-roster")["replayFramesPerJoin"].as_u64(), Some(1));
    assert_eq!(law("a-joiner-starts-from-the-settled-roster")["replayedWhenRosterEmpty"], serde_json::Value::Bool(false));
}

/// 📡️ A peer beat that only changes its ephemerals (pointer, camera, caret) reaches the document's sockets but is not
/// republished to every member's directory socket: the member-directory projection (actor, user, surface, colour) is
/// published when it changes — a join, a leave, the emptied roster — and never for an unchanged projection (C11: React
/// beats presence up to 10 Hz per moving human, which cost a directory frame per member socket per move).
#[tokio::test]
async fn a_peer_beat_reaches_the_document_roster_and_the_directory_projection_only_when_it_changes() {
    let state = test_state().await;
    let scope = DocumentScope::new(STUDIO, "presence-projection");
    let key = document_scope_key_v1(&scope);
    let mut roster = state.fanout_for(&key).subscribe();
    let mut directory = state.directory_service.subscribe();
    let now = tokio::time::Instant::now();
    let mut observe = |expected_roster: usize, expected_directory: usize, step: &str| {
        let mut rosters = 0;
        while let Ok(frame) = roster.try_recv() {
            rosters += usize::from(matches!(frame, ServerFrame::Presence { .. }));
        }
        let mut projections = Vec::new();
        while let Ok(message) = directory.try_recv() {
            if let DirectoryStreamMessage::Presence { document_id, actors, .. } = message {
                if document_id == scope.document_id {
                    projections.push(actors.iter().map(|actor| actor.actor.clone()).collect::<Vec<_>>());
                }
            }
        }
        assert_eq!((rosters, projections.len()), (expected_roster, expected_directory), "{step}: roster frames and directory projections {projections:?}");
        projections
    };
    assert_eq!(state.install_presence_slot(&key, STUDIO, &scope.document_id, "actor-a", test_presence_slot("live-a", Some("user-a"), now)).await, PresenceLeaseTransition::NoChange);
    assert_eq!(state.refresh_presence(&key, STUDIO, &scope.document_id, "actor-a", "live-a", b"pointer-1".to_vec(), now).await, PresenceLeaseTransition::Published);
    assert_eq!(observe(1, 1, "join"), vec![vec!["actor-a".to_string()]]);
    for (index, peer) in [b"pointer-2".to_vec(), b"pointer-3".to_vec(), b"camera-1".to_vec()].into_iter().enumerate() {
        assert_eq!(state.refresh_presence(&key, STUDIO, &scope.document_id, "actor-a", "live-a", peer, now + std::time::Duration::from_millis(100 * (index as u64 + 1))).await, PresenceLeaseTransition::Published);
    }
    observe(3, 0, "three ephemeral moves");
    assert_eq!(state.install_presence_slot(&key, STUDIO, &scope.document_id, "actor-b", test_presence_slot("live-b", Some("user-b"), now)).await, PresenceLeaseTransition::NoChange);
    assert_eq!(state.refresh_presence(&key, STUDIO, &scope.document_id, "actor-b", "live-b", b"pointer-b".to_vec(), now).await, PresenceLeaseTransition::Published);
    assert_eq!(observe(1, 1, "second join"), vec![vec!["actor-a".to_string(), "actor-b".to_string()]]);
    assert_eq!(state.close_presence_for_live(&key, STUDIO, &scope.document_id, "actor-b", "live-b").await, PresenceLeaseTransition::Published);
    assert_eq!(observe(1, 1, "leave"), vec![vec!["actor-a".to_string()]]);
    assert_eq!(state.close_presence_for_live(&key, STUDIO, &scope.document_id, "actor-a", "live-a").await, PresenceLeaseTransition::Published);
    assert_eq!(observe(1, 1, "last leave"), vec![Vec::<String>::new()]);
    assert_eq!(state.install_presence_slot(&key, STUDIO, &scope.document_id, "actor-a", test_presence_slot("live-a2", Some("user-a"), now)).await, PresenceLeaseTransition::NoChange);
    assert_eq!(state.refresh_presence(&key, STUDIO, &scope.document_id, "actor-a", "live-a2", b"pointer-1".to_vec(), now).await, PresenceLeaseTransition::Published);
    assert_eq!(observe(1, 1, "rejoin after the roster emptied"), vec![vec!["actor-a".to_string()]]);
}

#[tokio::test]
async fn presence_lease_enforces_shared_roster_bounds_and_actor_order() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/👥️presence-lease-v1/🔣️.json")).expect("presence lease fixture");
    assert_eq!(fixture["limits"]["ttlMs"].as_u64(), Some(PRESENCE_LEASE_TTL_MS));
    assert_eq!(fixture["limits"]["maximumItems"].as_u64(), Some(PRESENCE_ROSTER_MAXIMUM_ITEMS as u64));
    assert_eq!(fixture["limits"]["maximumEntryBytes"].as_u64(), Some(PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES as u64));
    assert_eq!(fixture["limits"]["maximumBytes"].as_u64(), Some(PRESENCE_ROSTER_MAXIMUM_BYTES as u64));
    let state = test_state().await;
    let ordered_scope = DocumentScope::new(STUDIO, "presence-order");
    let ordered_key = document_scope_key_v1(&ordered_scope);
    let now = tokio::time::Instant::now();
    for (actor, live, peer) in [("actor-z", "live-z", b"aaa".to_vec()), ("actor-a", "live-a", b"zzz".to_vec())] {
        assert_eq!(state.install_presence_slot(&ordered_key, STUDIO, &ordered_scope.document_id, actor, test_presence_slot(live, None, now)).await, PresenceLeaseTransition::NoChange);
        assert_eq!(state.refresh_presence(&ordered_key, STUDIO, &ordered_scope.document_id, actor, live, peer, now).await, PresenceLeaseTransition::Published);
    }
    let ordered = state.presence_snapshot(&ordered_key);
    assert_eq!(ordered.actors.iter().map(|actor| actor.actor.as_str()).collect::<Vec<_>>(), vec!["actor-a", "actor-z"]);
    assert_eq!(ordered.peers, vec![b"zzz".to_vec(), b"aaa".to_vec()], "opaque bytes do not select roster order");

    let full_scope = DocumentScope::new(STUDIO, "presence-full");
    let full_key = document_scope_key_v1(&full_scope);
    for index in 0..PRESENCE_ROSTER_MAXIMUM_ITEMS {
        let actor = format!("actor-{index:03}");
        let live = format!("live-{index:03}");
        assert_eq!(state.install_presence_slot(&full_key, STUDIO, &full_scope.document_id, &actor, test_presence_slot(&live, None, now)).await, PresenceLeaseTransition::NoChange);
        assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, &actor, &live, vec![0; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES], now).await, PresenceLeaseTransition::Published);
    }
    assert_eq!(state.presence_snapshot(&full_key).peers.len(), PRESENCE_ROSTER_MAXIMUM_ITEMS);
    assert_eq!(state.install_presence_slot(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", test_presence_slot("live-overflow", None, now)).await, PresenceLeaseTransition::NoChange);
    let deadline = state.presence.with(&(full_key.clone(), "actor-overflow".to_string()), |slot| slot.expect("overflow slot").expires_at);
    assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", "live-overflow", vec![1], now + std::time::Duration::from_secs(1)).await, PresenceLeaseTransition::Rejected);
    assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", "live-overflow", vec![1; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES + 1], now + std::time::Duration::from_secs(2)).await, PresenceLeaseTransition::Rejected);
    assert_eq!(state.presence.with(&(full_key, "actor-overflow".to_string()), |slot| slot.expect("overflow slot").expires_at), deadline, "rejection cannot refresh the lease deadline");

    let normalized_document = "presence-normalized-capacity";
    let normalized_key = document_scope_key_v1(&DocumentScope::new(STUDIO, normalized_document));
    let raw = presence_test_peer(b"canonical-capacity").await;
    let mut observed = state.fanout_for(&normalized_key).subscribe();
    for index in 0..=PRESENCE_ROSTER_MAXIMUM_ITEMS {
        let actor = format!("normalized-{index:03}");
        let live = format!("normalized-live-{index:03}");
        let slot = test_presence_slot(&live, Some("normalized-user"), now);
        let deadline = slot.expires_at;
        assert_eq!(state.install_presence_slot(&normalized_key, STUDIO, normalized_document, &actor, slot).await, PresenceLeaseTransition::NoChange);
        let before = state.presence_snapshot(&normalized_key).peers;
        let result = state.refresh_document_presence(&normalized_key, STUDIO, normalized_document, &actor, &live, raw.clone(), now + std::time::Duration::from_secs(1)).await;
        if index < PRESENCE_ROSTER_MAXIMUM_ITEMS {
            assert_eq!(result, PresenceLeaseTransition::Published);
            let ServerFrame::Presence { peers } = observed.try_recv().expect("normalized publication") else { panic!("normalized roster frame") };
            assert_eq!(peers.len(), index + 1);
            let peer = protocol::decode_presence_peer(peers.last().unwrap()).await.expect("canonical admitted row");
            assert_eq!(peer.actor, actor);
            assert_eq!(peer.user_id.as_deref(), Some("normalized-user"));
        } else {
            assert_eq!(result, PresenceLeaseTransition::Rejected);
            assert_eq!(state.presence_snapshot(&normalized_key).peers, before);
            assert_eq!(state.presence.with(&(normalized_key.clone(), actor), |slot| slot.expect("rejected normalized slot").expires_at), deadline);
            assert!(matches!(observed.try_recv(), Err(broadcast::error::TryRecvError::Empty)));
        }
    }
}

#[tokio::test]
async fn presence_lease_restart_is_empty_and_directory_presence_is_member_only() {
    let state = test_state().await;
    let key = document_scope_key_v1(&DocumentScope::new(STUDIO, "presence-restart"));
    let before = state.directory.head_seq().await.expect("directory head");
    let now = tokio::time::Instant::now();
    assert_eq!(state.install_presence_slot(&key, STUDIO, "presence-restart", "actor-a", test_presence_slot("live-a", Some("seed"), now)).await, PresenceLeaseTransition::NoChange);
    assert_eq!(state.refresh_presence(&key, STUDIO, "presence-restart", "actor-a", "live-a", b"opaque".to_vec(), now).await, PresenceLeaseTransition::Published);
    assert_eq!(state.directory.head_seq().await.expect("directory head"), before, "presence never appends a durable directory event");
    let member_token = seed_author_token(&state).await;
    let member = resolve_bearer_user(&state, Some(&member_token)).await.expect("member caller");
    let outsider_session = issue_test_session(&state, "presence-outsider@example.com").await;
    let outsider = resolve_bearer_user(&state, Some(&outsider_session.token)).await.expect("outsider caller");
    let message = DirectoryStreamMessage::Presence { space_id: STUDIO.into(), document_id: "presence-restart".into(), actors: state.presence_snapshot(&key).actors };
    assert!(directory_message_visible(&state, &message, Some(&member)).await);
    assert!(!directory_message_visible(&state, &message, Some(&outsider)).await);
    assert!(!directory_message_visible(&state, &message, None).await);
    let restarted = test_state().await;
    assert_eq!(restarted.presence.len(), 0, "a fresh hub has no server-local lease slots");
    assert!(restarted.presence_snapshot(&key).peers.is_empty());
}

/// 👥️ Every presence transition an operator can see in a roster is one structured record: a peer
/// becoming visible (`server.presence.join`), its lapsed lease stripping it to identity
/// (`server.presence.expiry`) and its socket leaving (`server.presence.leave`) — and a heartbeat that
/// changes nothing says nothing.
#[tokio::test]
async fn presence_join_expiry_and_leave_each_emit_one_record() {
    let (state, sink) = observed_test_state().await;
    let key = document_scope_key_v1(&DocumentScope::new(STUDIO, "presence-trace"));
    let now = tokio::time::Instant::now();
    assert_eq!(state.install_presence_slot(&key, STUDIO, "presence-trace", "actor-a", test_presence_slot("live-a", Some("seed"), now)).await, PresenceLeaseTransition::NoChange);
    assert_eq!(state.refresh_presence(&key, STUDIO, "presence-trace", "actor-a", "live-a", b"opaque".to_vec(), now).await, PresenceLeaseTransition::Published);
    assert_eq!(state.refresh_presence(&key, STUDIO, "presence-trace", "actor-a", "live-a", b"opaque".to_vec(), now).await, PresenceLeaseTransition::NoChange);
    let lapsed = now + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS + 1);
    assert_eq!(state.expire_presence_for_live(&key, STUDIO, "presence-trace", "actor-a", "live-a", lapsed).await, PresenceLeaseTransition::Published);
    assert_eq!(state.close_presence_for_live(&key, STUDIO, "presence-trace", "actor-a", "live-a").await, PresenceLeaseTransition::Published);
    let presence: Vec<(String, String, String, String)> = sink
        .records()
        .into_iter()
        .filter(|record| record.event.starts_with("server.presence."))
        .map(|record| (record.event, record.principal.unwrap_or_default(), record.space.unwrap_or_default(), record.artifact.unwrap_or_default()))
        .collect();
    let expected = |event: &str| (event.to_string(), "actor-a".to_string(), STUDIO.to_string(), "presence-trace".to_string());
    assert_eq!(presence, vec![expected("server.presence.join"), expected("server.presence.expiry"), expected("server.presence.leave")]);
    for event in ["server.presence.join", "server.presence.expiry", "server.presence.leave"] {
        assert!(SERVER_SPAN_EVENTS.contains(&event), "{event} is declared vocabulary");
    }
}
