//! 🔁️ The replay and live seam, the session relay, the server builder and the document socket identity of the gateway unit laws. See [`super`].

use super::*;

//#region 🔖️EventStream
#[tokio::test]
async fn the_replay_live_seam_has_no_gap_and_no_duplicate() {
    let state = state().await;
    let actor = actor("c1");
    state.store.write().await.append_events(&actor, &[event(&actor, 1), event(&actor, 2), event(&actor, 3)], &[]).await.unwrap();

    let mut live = state.fanout.subscribe(&stream_lane(&actor));
    state.fanout.publish(&stream_lane(&actor), serde_json::to_vec(&event(&actor, 3)).unwrap());
    state.fanout.publish(&stream_lane(&actor), serde_json::to_vec(&event(&actor, 4)).unwrap());

    let (mut sink, stream) = unbounded::<Message>();
    let pump = pump_events(&state, &actor, 0, &mut live, &mut sink);
    assert!(tokio::time::timeout(Duration::from_millis(120), pump).await.is_err());
    drop(sink);

    let delivered: Vec<u64> = stream
        .collect::<Vec<Message>>()
        .await
        .iter()
        .filter_map(|message| match message {
            Message::Text(text) => serde_json::from_str::<EventRecord>(text.as_str()).ok(),
            _ => None,
        })
        .map(|record| record.seq)
        .collect();
    assert_eq!(delivered, vec![1, 2, 3, 4]);
}

#[tokio::test]
async fn a_resuming_subscriber_skips_what_it_already_holds() {
    let state = state().await;
    let actor = actor("c1");
    state.store.write().await.append_events(&actor, &[event(&actor, 1), event(&actor, 2)], &[]).await.unwrap();
    let mut live = state.fanout.subscribe(&stream_lane(&actor));
    let (mut sink, stream) = unbounded::<Message>();
    let pump = pump_events(&state, &actor, 1, &mut live, &mut sink);
    assert!(tokio::time::timeout(Duration::from_millis(80), pump).await.is_err());
    drop(sink);
    assert_eq!(stream.collect::<Vec<Message>>().await.len(), 1);
}

#[tokio::test]
async fn history_is_read_while_the_bus_is_held_by_a_turn() {
    let state = state().await;
    let actor = actor("c1");
    state.store.write().await.append_events(&actor, &[event(&actor, 1), event(&actor, 2)], &[]).await.unwrap();
    let turns = state.authority.lock().await;
    let replayed = tokio::time::timeout(Duration::from_secs(5), state.replay_events(&actor, 0)).await.expect("a reader never waits for the bus").expect("read");
    assert_eq!(replayed.len(), 2);
    drop(turns);
}

#[test]
fn the_seam_admits_every_sequence_once() {
    let actor = actor("c1");
    let mut seam = EventSeam::new(0);
    assert!(seam.admit(&event(&actor, 1)));
    assert!(seam.admit(&event(&actor, 2)));
    assert!(!seam.admit(&event(&actor, 2)));
    assert!(!seam.admit(&event(&actor, 1)));
    assert!(seam.admit(&event(&actor, 3)));
    assert_eq!(seam.delivered(), 3);
}
//#endregion 🔖️EventStream

//#region 🔖️Relay
#[test]
fn a_relayed_frame_names_the_session_that_produced_it() {
    let wrapped = wrap_relay("session-7", b"frame");
    assert_eq!(unwrap_relay(&wrapped), Some(("session-7", b"frame".as_slice())));
    assert_eq!(unwrap_relay(b"\x40\x00short"), None);
    assert_eq!(unwrap_relay(&[]), None);
}
//#endregion 🔖️Relay

//#region 🔖️Server
#[tokio::test]
async fn build_collects_every_module_contribution() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() })
        .identity("hub", "0.1.0")
        .module(CountingModule)
        .app("admin", "/srv/admin")
        .admin_token(Some("secret".to_string()))
        .build()
        .await
        .expect("the test instance opens its stores");

    assert_eq!(server.definition().modules.len(), 1);
    assert_eq!(server.definition().id, "hub");
    assert!(server.state().admin.is_configured());
    assert_eq!(server.state().apps.names(), vec!["admin".to_string()]);
    assert!(server.state().documents.is_none());

    let request = admission_request(&envelope());
    assert!(server.state().authorize(&request).is_err());
    server.state().policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    assert!(server.state().authorize(&request).is_ok());
    let _ = server.router();
}

#[tokio::test]
async fn an_unroutable_command_is_rejected_and_publishes_nothing() {
    let state = state().await;
    let mut live = state.fanout.subscribe(&stream_lane(&actor("c1")));
    let outcome = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), WireJson(envelope())).await.unwrap();
    assert!(matches!(outcome.0, CommandOutcome::Rejected { .. }));
    assert!(tokio::time::timeout(Duration::from_millis(20), live.recv()).await.is_err());
}

#[tokio::test]
async fn the_clock_never_goes_backwards() {
    let state = state().await;
    let first = state.now();
    let second = state.now();
    assert!(second > first);
}

#[tokio::test]
async fn the_instance_supplies_the_storage_the_builder_wires_in() {
    let state = state().await;
    state.projections.write().await.put("roster", "space-1", vec![7]).await.expect("written");
    assert_eq!(state.projections.read().await.get("roster", "space-1").await, Some(vec![7]));
    assert_eq!(state.projections.read().await.checkpoint("roster").await, 0);
    assert!(state.sessions.lock().await.get(&crate::contract::SessionId("nobody".into())).await.is_none());
    assert_eq!(state.profile.data_dir(), Some("/tmp/semio-gateway"));
}

#[tokio::test]
async fn an_instance_that_registers_no_query_handler_answers_not_found() {
    let state = state().await;
    grant(&state, PolicyPoint::QueryAccess, "read");
    assert_eq!(state.queries.len(), 0);
    let error = post_query(HeaderMap::new(), ConnectInfo(loopback()), State(state), WireJson(query())).await.expect_err("no handler is registered");
    assert_eq!(error.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_module_registers_its_deciders_on_the_bus_it_was_built_into() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).build().await.expect("built");
    let state = server.state().clone();
    state.policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    let outcome = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state), WireJson(envelope())).await.unwrap();
    match outcome.0 {
        CommandOutcome::Accepted { events, .. } => assert_eq!(events[0].kind, "counter.incremented"),
        other => panic!("expected acceptance, got {other:?}"),
    }
}

#[tokio::test]
async fn a_committed_event_reaches_the_instance_saga_runner_exactly_once() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).saga(TestSagas::Silent(SilentSaga)).build().await.expect("built");
    let state = server.state().clone();
    {
        let mut engine = state.policy.write().unwrap();
        engine.assign("anonymous".to_string(), "author".to_string());
        engine.assign(crate::policy::principal_key(&Principal::User { id: "alice".into() }), "author".to_string());
    }
    assert_eq!(state.sagas.lock().await.sagas.len(), 2, "the module's workflow and the builder's are both on the one runner");

    let committed = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), WireJson(envelope())).await.unwrap();
    assert!(matches!(committed.0, CommandOutcome::Accepted { .. }));
    let reactions = state.drain_sagas(64).await;
    assert_eq!(reactions.len(), 1, "one committed event, one follow-up turn");
    assert!(matches!(reactions[0], CommandOutcome::Accepted { .. }));

    let replayed = state.drain_sagas(64).await;
    assert!(replayed.iter().all(|outcome| matches!(outcome, CommandOutcome::Accepted { ref events, .. } if events.is_empty())), "a re-issued follow-up is deduplicated by its idempotency key rather than committing twice");
    assert!(state.store.read().await.pending_outbox(64).await.unwrap().is_empty(), "every drained row is acknowledged");
}

#[tokio::test]
async fn the_lane_has_counted_what_a_command_committed_when_it_answers() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).build().await.expect("built");
    let state = server.state().clone();
    state.policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    assert_eq!(state.committed(), Committed::default());
    let CommandOutcome::Accepted { events, .. } = state.submit(envelope()).await else { panic!("accepted") };
    let owed = state.store.read().await.pending_outbox(64).await.unwrap().len() as u64;
    assert_eq!(state.committed(), Committed { events: events.len() as u64, outbox: owed });
    assert!(owed > 0, "the counting module queues an outbox row per event");
    let refused = CommandEnvelope { kind: "counter.unknown".into(), command_id: CommandId("cmd-2".into()), ..envelope() };
    assert!(matches!(state.submit(refused).await, CommandOutcome::Rejected { .. }));
    assert_eq!(state.committed().events, events.len() as u64, "a refused command commits nothing");
}

fn query() -> QueryEnvelope {
    QueryEnvelope { query_id: QueryId("q-1".into()), kind: "counter.value".into(), version: 1, scope: Scope("space-1".into()), principal: Principal::Anonymous, arguments: Vec::new(), consistency: QueryConsistency::Local, cursor: None }
}

pub(super) fn envelope() -> CommandEnvelope {
    CommandEnvelope {
        command_id: CommandId("cmd-1".into()),
        kind: "counter.increment".into(),
        version: 1,
        target: actor("c1"),
        scope: Scope("space-1".into()),
        principal: Principal::Anonymous,
        session: None,
        device: None,
        payload: vec![1],
        causal_frontier: None,
        client_hlc: HybridLogicalClock::default(),
        expected_revision: None,
        idempotency_key: None,
        capability_proof: None,
        trace: TraceContext::default(),
    }
}
//#endregion 🔖️Server

#[test]
fn document_socket_identity_binds_from_resolved_actor_never_query() {
    let resolved = Resolved {
        principal: Principal::User { id: "alice".into() },
        session: Some(crate::contract::SessionId("session-1".into())),
        device: None,
        via: "test".into(),
        actor: Some("hub.v1.alice".into()),
    };
    let identity = document_socket_identity(&resolved).expect("binds");
    assert_eq!(identity.actor, "hub.v1.alice");
    assert_eq!(identity.session, "session-1");
}

#[test]
fn document_socket_identity_rejects_anonymous_without_grant() {
    let resolved = Resolved { principal: Principal::Anonymous, session: None, device: None, via: "anonymous".into(), actor: None };
    assert!(document_socket_identity(&resolved).is_err());
}
