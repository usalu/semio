
use super::*;
use crate::contract::{CommandId, PolicyGrant, QueryConsistency, QueryId, TraceContext};
use crate::test_instance::{CountingModule, SilentSaga, TestInstance, TestSagas};
use futures::channel::mpsc::unbounded;
use std::time::Duration;

async fn state() -> ServerState<TestInstance> {
    server().await.state().clone()
}

async fn server() -> Server<TestInstance> {
    Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).build().await.expect("the test instance opens its stores")
}

fn actor(id: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: "counter".into(), id: id.into() }
}

fn event(stream: &ActorKey, seq: u64) -> EventRecord {
    EventRecord { stream: stream.clone(), seq, hlc: HybridLogicalClock::default(), kind: "counter.incremented".into(), payload: vec![seq as u8] }
}

fn grant(state: &ServerState<TestInstance>, point: PolicyPoint, action: &str) {
    let mut engine = state.policy.write().unwrap();
    engine.register_template(PolicyTemplate { name: format!("{point:?}-{action}"), auto_apply: false, grants: vec![PolicyGrant { point, resource: "*".into(), action: action.to_string() }] });
    engine.assign("anonymous".to_string(), format!("{point:?}-{action}"));
}

fn scratch(name: &str) -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |elapsed| elapsed.as_nanos());
    let path = std::env::temp_dir().join(format!("semio-gateway-{name}-{unique}"));
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn loopback() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 4242))
}

//#region 🔖️Presence
#[test]
fn a_colour_lease_takes_the_lowest_free_slot() {
    let presence = Presence::new();
    assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
    assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
    assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
    presence.release_colour("space-1", "alice");
    assert_eq!(presence.acquire_colour("space-1", "carol"), 0);
    assert_eq!(presence.acquire_colour("space-2", "dave"), 0);
}

#[test]
fn a_colour_is_freed_only_on_the_last_disconnect() {
    let presence = Presence::new();
    assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
    assert_eq!(presence.acquire_colour("space-1", "alice"), 0);
    presence.release_colour("space-1", "alice");
    assert_eq!(presence.colour_of("space-1", "alice"), Some(0));
    assert_eq!(presence.acquire_colour("space-1", "bob"), 1);
    presence.release_colour("space-1", "alice");
    assert_eq!(presence.colour_of("space-1", "alice"), None);
    assert_eq!(presence.acquire_colour("space-1", "carol"), 0);
}

#[test]
fn joining_and_leaving_maintains_the_roster() {
    let presence = Presence::new();
    presence.join("space-1", "bob", "canvas");
    presence.join("space-1", "alice", "canvas");
    presence.publish_peer("space-1", "alice", vec![7]);
    let roster = presence.roster("space-1");
    assert_eq!(roster.iter().map(|session| session.actor.as_str()).collect::<Vec<_>>(), vec!["alice", "bob"]);
    assert_eq!(roster[0].peer, Some(vec![7]));
    presence.leave("space-1", "alice");
    assert_eq!(presence.roster("space-1").len(), 1);
    assert_eq!(presence.colour_of("space-1", "alice"), None);
}
//#endregion 🔖️Presence

//#region 🔖️Fanout
#[tokio::test]
async fn a_lane_reaches_every_subscriber_and_is_dropped_when_empty() {
    let fanout = Fanout::new();
    assert_eq!(fanout.publish("stream:a", vec![1]), 0);
    let mut first = fanout.subscribe("stream:a");
    let mut second = fanout.subscribe("stream:a");
    assert_eq!(fanout.lanes(), 1);
    assert_eq!(fanout.publish("stream:a", vec![9]), 2);
    assert_eq!(first.recv().await, Some(vec![9]));
    assert_eq!(second.recv().await, Some(vec![9]));
    drop(first);
    assert_eq!(fanout.lanes(), 1);
    drop(second);
    assert_eq!(fanout.lanes(), 0);
}

#[test]
fn lane_keys_keep_the_durable_and_ephemeral_lanes_apart() {
    let scope = Scope("space-1".into());
    assert_eq!(document_lane(&scope), "document:space-1");
    assert_eq!(ephemeral_lane(&scope), "ephemeral:space-1");
    assert_eq!(stream_lane(&actor("c1")), "stream:t1/counter/c1");
}
//#endregion 🔖️Fanout

//#region 🔖️Kick
#[tokio::test]
async fn a_kick_fired_before_the_wait_is_still_observed() {
    let kicks = KickMap::new();
    kicks.kick("session-1");
    tokio::time::timeout(Duration::from_millis(200), kicks.kicked("session-1")).await.expect("kick must be observed");
    assert_eq!(kicks.tracked(), 1);
    kicks.forget("session-1");
    assert_eq!(kicks.tracked(), 0);
    assert!(tokio::time::timeout(Duration::from_millis(20), kicks.kicked("session-2")).await.is_err());
}
//#endregion 🔖️Kick

//#region 🔖️StaticApp
#[test]
fn a_static_host_refuses_traversal() {
    let host = StaticAppHost::new("/srv/app");
    assert!(host.resolve("../secret").is_none());
    assert!(host.resolve("nested/../../secret").is_none());
    assert!(host.resolve("nested\\secret").is_none());
    assert_eq!(host.resolve("/etc/passwd"), Some(PathBuf::from("/srv/app/etc/passwd")));
    assert_eq!(host.resolve("assets/app.js"), Some(PathBuf::from("/srv/app/assets/app.js")));
}

#[tokio::test]
async fn a_client_route_falls_back_to_index_html() {
    let root = scratch("spa");
    std::fs::write(root.join("index.html"), b"<!doctype html>shell").unwrap();
    std::fs::write(root.join("app.js"), b"export {}").unwrap();
    let host = StaticAppHost::new(&root);

    let asset = host.serve("app.js");
    assert_eq!(asset.status(), StatusCode::OK);
    assert_eq!(asset.headers().get(header::CONTENT_TYPE).unwrap(), "text/javascript");

    let route = host.serve("spaces/sp-1");
    assert_eq!(route.status(), StatusCode::OK);
    let body = axum::body::to_bytes(route.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body.as_ref(), b"<!doctype html>shell");

    assert_eq!(host.serve("../secret").status(), StatusCode::BAD_REQUEST);
    assert_eq!(StaticAppHost::new(root.join("missing")).serve("index.html").status(), StatusCode::SERVICE_UNAVAILABLE);
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn installs_are_scanned_and_ordered() {
    let root = scratch("apps");
    for id in ["beta", "alpha"] {
        std::fs::create_dir_all(root.join(id)).unwrap();
        std::fs::write(root.join(id).join("install.json"), format!("{{\"extensionId\":\"{id}\"}}")).unwrap();
    }
    std::fs::create_dir_all(root.join("empty")).unwrap();
    let registry = AppRegistry::new();
    registry.register("extensions", &root);
    let installs = registry.installs("extensions");
    assert_eq!(installs.iter().map(|install| install.id.as_str()).collect::<Vec<_>>(), vec!["alpha", "beta"]);
    assert_eq!(installs[0].manifest["extensionId"], "alpha");
    assert_eq!(registry.names(), vec!["extensions".to_string()]);
    assert!(registry.installs("nothing").is_empty());
    std::fs::remove_dir_all(&root).unwrap();
}
//#endregion 🔖️StaticApp

//#region 🔖️Cors
#[test]
fn cors_reflects_the_callers_own_origin() {
    let mut headers = HeaderMap::new();
    apply_cors_headers(&mut headers, Some(&HeaderValue::from_static("http://127.0.0.1:6072")));
    assert_eq!(headers.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(), "http://127.0.0.1:6072");
    assert_eq!(headers.get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS).unwrap(), "true");
    assert_eq!(headers.get(header::VARY).unwrap(), "origin");

    let mut anonymous = HeaderMap::new();
    apply_cors_headers(&mut anonymous, None);
    assert!(anonymous.get(header::ACCESS_CONTROL_ALLOW_ORIGIN).is_none());
    assert!(anonymous.get(header::ACCESS_CONTROL_ALLOW_METHODS).is_some());
}
//#endregion 🔖️Cors

//#region 🔖️Credential
#[test]
fn a_bearer_and_the_loopback_fact_are_read_from_the_transport() {
    let mut headers = HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_static("Bearer tok"));
    headers.insert(CAPABILITY_HEADER, HeaderValue::from_static("share-1"));
    let local = credential(&headers, Some(loopback()));
    assert_eq!(local.bearer.as_deref(), Some("tok"));
    assert_eq!(local.capability.map(|proof| proof.0), Some("share-1".to_string()));
    assert!(local.loopback);
    assert!(!credential(&headers, Some(SocketAddr::from(([10, 0, 0, 7], 80)))).loopback);
    assert!(!credential(&HeaderMap::new(), None).loopback);
    assert!(credential(&HeaderMap::new(), None).bearer.is_none());
}
//#endregion 🔖️Credential

//#region 🔖️Blob
#[tokio::test]
async fn a_blob_address_that_does_not_match_its_bytes_is_a_conflict() {
    let state = state().await;
    grant(&state, PolicyPoint::BlobWrite, "write");
    grant(&state, PolicyPoint::BlobRead, "read");

    let honest = content_hash(b"hello").to_string();
    let receipt = put_blob(Path(honest.clone()), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Bytes::from_static(b"hello")).await.expect("an honest address is accepted");
    assert_eq!(receipt.0.size, 5);

    let lie = content_hash(b"world").to_string();
    let error = put_blob(Path(lie), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Bytes::from_static(b"hello")).await.expect_err("a mismatched address is refused");
    assert_eq!(error.status(), StatusCode::CONFLICT);

    assert_eq!(head_blob(Path(honest.clone()), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone())).await, StatusCode::OK);
    let missing = content_hash(b"world").to_string();
    assert_eq!(head_blob(Path(missing), HeaderMap::new(), ConnectInfo(loopback()), State(state.clone())).await, StatusCode::NOT_FOUND);
    let short = put_blob(Path("beef".to_string()), HeaderMap::new(), ConnectInfo(loopback()), State(state), Bytes::from_static(b"hello")).await;
    assert_eq!(short.err().map(|error| error.status()), Some(StatusCode::BAD_REQUEST));
}

#[tokio::test]
async fn a_blob_write_without_a_grant_is_forbidden() {
    let state = state().await;
    let hash = content_hash(b"hello").to_string();
    let error = put_blob(Path(hash), HeaderMap::new(), ConnectInfo(loopback()), State(state), Bytes::from_static(b"hello")).await.expect_err("closed by default");
    assert_eq!(error.status(), StatusCode::FORBIDDEN);
}
//#endregion 🔖️Blob

//#region 🔖️EventStream
#[tokio::test]
async fn the_replay_live_seam_has_no_gap_and_no_duplicate() {
    let state = state().await;
    let actor = actor("c1");
    {
        let mut authority = state.authority.lock().await;
        authority.store_mut().append_events(&actor, &[event(&actor, 1), event(&actor, 2), event(&actor, 3)], &[]).await.unwrap();
    }

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
    {
        let mut authority = state.authority.lock().await;
        authority.store_mut().append_events(&actor, &[event(&actor, 1), event(&actor, 2)], &[]).await.unwrap();
    }
    let mut live = state.fanout.subscribe(&stream_lane(&actor));
    let (mut sink, stream) = unbounded::<Message>();
    let pump = pump_events(&state, &actor, 1, &mut live, &mut sink);
    assert!(tokio::time::timeout(Duration::from_millis(80), pump).await.is_err());
    drop(sink);
    assert_eq!(stream.collect::<Vec<Message>>().await.len(), 1);
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
    let outcome = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Json(envelope())).await.unwrap();
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
    state.projections.lock().await.put("roster", "space-1", vec![7]).await.expect("written");
    assert_eq!(state.projections.lock().await.get("roster", "space-1").await, Some(vec![7]));
    assert_eq!(state.projections.lock().await.checkpoint("roster").await, 0);
    assert!(state.sessions.lock().await.get(&crate::contract::SessionId("nobody".into())).await.is_none());
    assert_eq!(state.profile.data_dir(), Some("/tmp/semio-gateway"));
}

#[tokio::test]
async fn an_instance_that_registers_no_query_handler_answers_not_found() {
    let state = state().await;
    grant(&state, PolicyPoint::QueryAccess, "read");
    assert_eq!(state.queries.len(), 0);
    let error = post_query(HeaderMap::new(), ConnectInfo(loopback()), State(state), Json(query())).await.expect_err("no handler is registered");
    assert_eq!(error.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_module_registers_its_deciders_on_the_bus_it_was_built_into() {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).build().await.expect("built");
    let state = server.state().clone();
    state.policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    let outcome = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state), Json(envelope())).await.unwrap();
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

    let committed = post_command(HeaderMap::new(), ConnectInfo(loopback()), State(state.clone()), Json(envelope())).await.unwrap();
    assert!(matches!(committed.0, CommandOutcome::Accepted { .. }));
    let reactions = state.drain_sagas(64).await;
    assert_eq!(reactions.len(), 1, "one committed event, one follow-up turn");
    assert!(matches!(reactions[0], CommandOutcome::Accepted { .. }));

    let replayed = state.drain_sagas(64).await;
    assert!(replayed.iter().all(|outcome| matches!(outcome, CommandOutcome::Accepted { ref events, .. } if events.is_empty())), "a re-issued follow-up is deduplicated by its idempotency key rather than committing twice");
    assert!(state.authority.lock().await.store().pending_outbox(64).await.unwrap().is_empty(), "every drained row is acknowledged");
}

fn query() -> QueryEnvelope {
    QueryEnvelope { query_id: QueryId("q-1".into()), kind: "counter.value".into(), version: 1, scope: Scope("space-1".into()), principal: Principal::Anonymous, arguments: Vec::new(), consistency: QueryConsistency::Local, cursor: None }
}

fn envelope() -> CommandEnvelope {
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

//#region 🔖️PresenceStream
type ClientFrames = futures::channel::mpsc::UnboundedSender<Result<Message, std::convert::Infallible>>;
type ServerFrames = futures::channel::mpsc::UnboundedReceiver<Message>;

fn quick_presence(idle: Duration) -> PresenceSettings {
    PresenceSettings { tick: Duration::from_millis(20), idle, keepalive: Duration::from_secs(30), ..PresenceSettings::default() }
}

async fn presence_server(settings: PresenceSettings) -> ServerState<TestInstance> {
    let server = Server::<TestInstance>::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).module(CountingModule).presence(settings).build().await.expect("built");
    let state = server.state().clone();
    grant(&state, PolicyPoint::Subscription, PRESENCE_JOIN);
    grant(&state, PolicyPoint::Subscription, PRESENCE_PUBLISH);
    state
}

fn connect(state: &ServerState<TestInstance>, scope: &str, surface: &str) -> (ClientFrames, ServerFrames, tokio::task::JoinHandle<()>) {
    let (client, mut inbound) = unbounded::<Result<Message, std::convert::Infallible>>();
    let (mut outbound, frames) = unbounded::<Message>();
    let (state, scope, surface) = (state.clone(), Scope(scope.to_string()), surface.to_string());
    let task = tokio::spawn(async move { run_presence(&state, &scope, &surface, &PresenceGrant { principal: Principal::Anonymous, may_publish: true }, &mut outbound, &mut inbound).await });
    (client, frames, task)
}

async fn next_frame(frames: &mut ServerFrames) -> PresenceFrame {
    loop {
        match tokio::time::timeout(Duration::from_secs(5), frames.next()).await.expect("a frame arrives").expect("the socket is open") {
            Message::Text(text) => return serde_json::from_str(text.as_str()).expect("a presence frame"),
            Message::Ping(_) => continue,
            other => panic!("unexpected message {other:?}"),
        }
    }
}

fn say(client: &ClientFrames, text: &str) {
    client.unbounded_send(Ok(Message::Text(text.to_string().into()))).expect("the session is open");
}

#[test]
fn presence_rooms_coalesce_every_update_into_one_batch_per_tick() {
    let rooms = PresenceRooms::new();
    let (roster, start) = rooms.join("presence:s", "a", 0, "home");
    assert!(start);
    assert_eq!(roster.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>(), ["a"]);
    let (roster, start) = rooms.join("presence:s", "b", 1, "home");
    assert!(!start);
    assert_eq!(roster.len(), 2);
    for n in 0..5 {
        rooms.update("presence:s", "a", serde_json::json!({ "n": n }));
    }
    let Drained::Batch(PresenceFrame::Batch { entries, left }) = rooms.drain("presence:s") else { panic!("a batch") };
    assert_eq!(entries.iter().map(|entry| (entry.session.as_str(), entry.state.clone())).collect::<Vec<_>>(), [("a", serde_json::json!({ "n": 4 })), ("b", OpaqueJson::Null)]);
    assert!(left.is_empty());
    assert_eq!(rooms.drain("presence:s"), Drained::Quiet);
    rooms.update("presence:s", "b", serde_json::json!({ "x": 1 }));
    rooms.leave("presence:s", "b");
    assert_eq!(rooms.drain("presence:s"), Drained::Batch(PresenceFrame::Batch { entries: Vec::new(), left: vec!["b".to_string()] }));
    rooms.leave("presence:s", "a");
    assert!(matches!(rooms.drain("presence:s"), Drained::Batch(PresenceFrame::Batch { ref left, .. }) if left == &["a".to_string()]));
    assert_eq!(rooms.drain("presence:s"), Drained::Finished);
    assert!(rooms.roster("presence:s").is_empty());
    assert!(rooms.join("presence:s", "c", 0, "home").1, "a room whose ticker finished starts a new one");
}

#[test]
fn a_state_frame_is_bounded_by_size_rate_and_shape() {
    let settings = PresenceSettings::default();
    let now = Instant::now();
    let mut budget = StateBudget::new(settings.max_states_per_second);
    assert_eq!(admit_frame(r#"{"type":"state","state":{"x":1}}"#, &settings, &mut budget, now), Ok(Some(PresenceFrame::State { state: serde_json::json!({ "x": 1 }) })));
    let large = format!(r#"{{"type":"state","state":"{}"}}"#, "x".repeat(settings.max_state_bytes));
    assert_eq!(admit_frame(&large, &settings, &mut budget, now), Err(REFUSED_TOO_LARGE.to_string()));
    assert_eq!(admit_frame(r#"{"type":"batch","entries":[],"left":[]}"#, &settings, &mut budget, now), Err(REFUSED_INVALID.to_string()));
    assert_eq!(admit_frame("not json", &settings, &mut budget, now), Err(REFUSED_INVALID.to_string()));
    let mut spent = StateBudget::new(settings.max_states_per_second);
    let start = Instant::now();
    let admitted = (0..40).filter(|_| admit_frame(r#"{"type":"state","state":1}"#, &settings, &mut spent, start) == Ok(Some(PresenceFrame::State { state: serde_json::json!(1) }))).count();
    assert_eq!(admitted, settings.max_states_per_second as usize);
    assert_eq!(admit_frame(r#"{"type":"state","state":1}"#, &settings, &mut spent, start + Duration::from_secs(1)), Ok(Some(PresenceFrame::State { state: serde_json::json!(1) })));
}

#[test]
fn presence_session_ids_are_unique_hex() {
    let first = presence_session_id();
    let second = presence_session_id();
    assert_ne!(first, second);
    assert!(first.len() == 32 && first.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()));
}

#[tokio::test]
async fn presence_admission_needs_the_join_grant_a_short_surface_and_an_admitted_origin() {
    let state = state().await;
    let scope = Scope("space-1".into());
    let refused = admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, "home").await.expect_err("closed by default");
    assert_eq!(refused.status(), StatusCode::FORBIDDEN);
    grant(&state, PolicyPoint::Subscription, PRESENCE_JOIN);
    assert!(!admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, "home").await.expect("joins").may_publish, "join alone does not publish");
    grant(&state, PolicyPoint::Subscription, PRESENCE_PUBLISH);
    assert_eq!(admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, "home").await.expect("joins and publishes"), PresenceGrant { principal: Principal::Anonymous, may_publish: true });
    let long = "x".repeat(65);
    assert_eq!(admit_presence(&state, &HeaderMap::new(), Some(loopback()), &scope, &long).await.expect_err("too long").status(), StatusCode::BAD_REQUEST);

    let mut gated = state.clone();
    gated.origin_admission = Some(Arc::new(|origin: &str| origin == "https://quizzes.example"));
    let mut foreign = HeaderMap::new();
    foreign.insert(header::ORIGIN, HeaderValue::from_static("https://evil.example"));
    assert_eq!(admit_presence(&gated, &foreign, Some(loopback()), &scope, "home").await.expect_err("foreign origin").status(), StatusCode::FORBIDDEN);
    let mut site = HeaderMap::new();
    site.insert(header::ORIGIN, HeaderValue::from_static("https://quizzes.example"));
    assert!(admit_presence(&gated, &site, Some(loopback()), &scope, "home").await.is_ok());
    assert!(admit_presence(&gated, &HeaderMap::new(), Some(loopback()), &scope, "home").await.is_ok(), "a non-browser client presents no origin");
}

#[tokio::test]
async fn two_sessions_share_coalesced_state_and_see_each_other_leave() {
    let state = presence_server(quick_presence(Duration::from_secs(30))).await;
    let (alice, mut alice_frames, _alice_task) = connect(&state, "room-1", "home");
    let PresenceFrame::Welcome { session: alice_session, colour: alice_colour, roster } = next_frame(&mut alice_frames).await else { panic!("welcome first") };
    assert_eq!(roster.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>(), [alice_session.as_str()]);

    let (bob, mut bob_frames, bob_task) = connect(&state, "room-1", "leaderboard");
    let PresenceFrame::Welcome { session: bob_session, colour: bob_colour, roster } = next_frame(&mut bob_frames).await else { panic!("welcome first") };
    assert_ne!(alice_colour, bob_colour);
    assert_eq!(roster.len(), 2);
    assert!(roster.iter().any(|entry| entry.session == alice_session && entry.surface == "home"));

    for n in 0..5 {
        say(&alice, &format!(r#"{{"type":"state","state":{{"n":{n}}}}}"#));
    }
    let mut seen = Vec::new();
    while seen.last() != Some(&serde_json::json!({ "n": 4 })) {
        if let PresenceFrame::Batch { entries, .. } = next_frame(&mut bob_frames).await {
            seen.extend(entries.into_iter().filter(|entry| entry.session == alice_session).map(|entry| entry.state).filter(|state| !state.is_null()));
        }
    }
    assert!(seen.len() <= 2, "five states sent within one tick arrive coalesced: {seen:?}");

    say(&alice, r#"{"type":"state","state":{"refuse":"not-here"}}"#);
    let mut refusal = next_frame(&mut alice_frames).await;
    while let PresenceFrame::Batch { .. } = refusal {
        refusal = next_frame(&mut alice_frames).await;
    }
    assert_eq!(refusal, PresenceFrame::Refused { reason: "not-here".into() });
    alice.unbounded_send(Ok(Message::Binary(vec![1, 2].into()))).expect("open");
    assert_eq!(next_frame(&mut alice_frames).await, PresenceFrame::Refused { reason: REFUSED_INVALID.into() });

    drop(bob);
    tokio::time::timeout(Duration::from_secs(5), bob_task).await.expect("bob's session ends").expect("no panic");
    loop {
        if let PresenceFrame::Batch { left, .. } = next_frame(&mut alice_frames).await {
            if left.contains(&bob_session) {
                break;
            }
        }
    }
    assert_eq!(state.presence_rooms.roster(&presence_lane(&Scope("room-1".into()))).len(), 1);
    assert_eq!(state.presence.colour_of(&presence_lane(&Scope("room-1".into())), &bob_session), None);
}

#[tokio::test]
async fn an_idle_session_is_closed_and_leaves_the_room() {
    let state = presence_server(quick_presence(Duration::from_millis(150))).await;
    let (_silent, mut frames, task) = connect(&state, "room-idle", "home");
    assert!(matches!(next_frame(&mut frames).await, PresenceFrame::Welcome { .. }));
    tokio::time::timeout(Duration::from_secs(5), task).await.expect("the idle session is closed").expect("no panic");
    let rest: Vec<Message> = frames.collect().await;
    assert!(matches!(rest.last(), Some(Message::Close(_))), "{rest:?}");
    assert!(state.presence_rooms.roster(&presence_lane(&Scope("room-idle".into()))).is_empty());
}

fn entry(session: &str, state: OpaqueJson) -> PresenceEntry {
    PresenceEntry { session: session.into(), colour: 0, surface: "home".into(), state }
}

fn scopes(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|name| (*name).to_string()).collect()
}

fn watched(scope: &str, entries: Vec<PresenceEntry>, left: &[&str], snapshot: bool) -> PresenceFrame {
    PresenceFrame::Watched { scope: Scope(scope.into()), entries, left: left.iter().map(|session| (*session).to_string()).collect(), snapshot }
}

#[test]
fn watching_coalesces_per_interval_and_resynchronizes_with_a_snapshot() {
    let mut watching = Watching::default();
    assert!(watching.is_empty() && watching.flush().is_empty());
    assert_eq!(watching.watch(&scopes(&["room-b", "room-a"])), ["room-a", "room-b"]);
    assert_eq!(watching.snapshot("room-a", vec![entry("s1", OpaqueJson::Null)]), Some(watched("room-a", vec![entry("s1", OpaqueJson::Null)], &[], true)));
    for n in 0..5 {
        watching.absorb("room-a", vec![entry("s1", serde_json::json!({ "n": n })), entry("s2", OpaqueJson::Null)], Vec::new());
    }
    watching.absorb("room-b", vec![entry("s3", OpaqueJson::Null)], Vec::new());
    watching.absorb("room-b", Vec::new(), vec!["s3".into(), "s4".into()]);
    assert_eq!(watching.flush(), [watched("room-a", vec![entry("s1", serde_json::json!({ "n": 4 })), entry("s2", OpaqueJson::Null)], &[], false), watched("room-b", Vec::new(), &["s3", "s4"], false)]);
    assert!(watching.flush().is_empty(), "an interval without changes sends nothing");

    watching.absorb("room-a", Vec::new(), vec!["s2".into()]);
    watching.absorb("room-a", vec![entry("s2", serde_json::json!(1))], Vec::new());
    assert_eq!(watching.flush(), [watched("room-a", vec![entry("s2", serde_json::json!(1))], &[], false)], "a later entry supersedes a departure");
    watching.absorb("room-a", vec![entry("s1", serde_json::json!(2))], Vec::new());
    assert_eq!(watching.snapshot("room-a", vec![entry("s1", serde_json::json!(2))]), Some(watched("room-a", vec![entry("s1", serde_json::json!(2))], &[], true)));
    assert!(watching.flush().is_empty(), "a snapshot carries what was pending");

    watching.absorb("room-b", vec![entry("s5", OpaqueJson::Null)], Vec::new());
    assert_eq!(watching.watch(&scopes(&["room-b", "room-c"])), ["room-c"]);
    assert!(!watching.watches("room-a") && watching.watches("room-c"));
    watching.absorb("room-a", vec![entry("s1", OpaqueJson::Null)], Vec::new());
    assert_eq!(watching.snapshot("room-a", Vec::new()), None, "an unwatched scope is ignored");
    assert_eq!(watching.flush(), [watched("room-b", vec![entry("s5", OpaqueJson::Null)], &[], false)], "a scope that stays watched keeps its pending changes");
    assert!(watching.watch(&BTreeSet::new()).is_empty());
    assert!(watching.is_empty());
}

#[test]
fn the_watch_interval_is_clamped_between_the_tick_and_the_slowest_interval() {
    let settings = PresenceSettings::default();
    assert_eq!(watch_interval(0, &settings), settings.tick);
    assert_eq!(watch_interval(40, &settings), settings.tick);
    assert_eq!(watch_interval(250, &settings), Duration::from_millis(250));
    assert_eq!(watch_interval(u64::MAX, &settings), settings.max_watch_interval);
    let mut budget = StateBudget::new(settings.max_states_per_second);
    assert_eq!(admit_frame(r#"{"type":"watch","scopes":["a"],"intervalMs":250}"#, &settings, &mut budget, Instant::now()), Ok(Some(PresenceFrame::Watch { scopes: vec![Scope("a".into())], interval_ms: 250 })));
    assert_eq!(admit_frame(r#"{"type":"watched","scope":"a","entries":[],"left":[]}"#, &settings, &mut budget, Instant::now()), Err(REFUSED_INVALID.to_string()), "watched travels server → client only");
}

fn grant_scoped(state: &ServerState<TestInstance>, action: &str, scopes: &[&str]) {
    let mut engine = state.policy.write().unwrap();
    let name = format!("presence-{action}");
    engine.register_template(PolicyTemplate { name: name.clone(), auto_apply: false, grants: vec![PolicyGrant { point: PolicyPoint::Subscription, resource: PRESENCE_RESOURCE.into(), action: action.to_string() }] });
    for scope in scopes {
        engine.assign_scoped("anonymous".to_string(), Scope((*scope).to_string()), name.clone());
    }
}

#[tokio::test]
async fn a_watch_is_admitted_per_scope_and_bounded() {
    let state = state().await;
    let wanted = [Scope("room-a".into()), Scope("room-b".into()), Scope("room-a".into())];
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &wanted), Err("forbidden room-a".to_string()), "closed by default");
    grant_scoped(&state, PRESENCE_WATCH, &["room-a"]);
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &wanted), Err("forbidden room-b".to_string()));
    grant_scoped(&state, PRESENCE_WATCH, &["room-b"]);
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &wanted), Ok(scopes(&["room-a", "room-b"])));
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &[]), Ok(BTreeSet::new()), "an empty watch stops watching");
    let many: Vec<Scope> = (0..=state.presence_settings.max_watch_scopes).map(|_| Scope("room-a".into())).collect();
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &many), Err(REFUSED_WATCH_TOO_MANY.to_string()));
    grant_scoped(&state, PRESENCE_JOIN, &["room-c"]);
    assert_eq!(admit_watch(&state, &Principal::Anonymous, &[Scope("room-c".into())]), Err("forbidden room-c".to_string()), "joining is not watching");
}

async fn watched_frame(frames: &mut ServerFrames) -> PresenceFrame {
    loop {
        let frame = next_frame(frames).await;
        if matches!(frame, PresenceFrame::Watched { .. } | PresenceFrame::Refused { .. }) {
            return frame;
        }
    }
}

#[tokio::test]
async fn a_socket_watches_other_rooms_read_only_at_its_own_interval() {
    let state = presence_server(quick_presence(Duration::from_secs(30))).await;
    grant_scoped(&state, PRESENCE_WATCH, &["room-a", "room-b"]);
    let (alice, mut alice_frames, _alice_task) = connect(&state, "room-a", "home");
    let PresenceFrame::Welcome { session: alice_session, .. } = next_frame(&mut alice_frames).await else { panic!("welcome") };
    let (bob, mut bob_frames, bob_task) = connect(&state, "room-b", "home");
    let PresenceFrame::Welcome { session: bob_session, .. } = next_frame(&mut bob_frames).await else { panic!("welcome") };
    let (carol, mut carol_frames, carol_task) = connect(&state, "room-c", "home");
    let PresenceFrame::Welcome { session: carol_session, .. } = next_frame(&mut carol_frames).await else { panic!("welcome") };

    say(&carol, r#"{"type":"watch","scopes":["room-b","room-a"],"intervalMs":80}"#);
    let PresenceFrame::Watched { scope, entries, left, snapshot: true } = watched_frame(&mut carol_frames).await else { panic!("a snapshot") };
    assert_eq!((scope.0.as_str(), entries.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>(), left.len()), ("room-a", vec![alice_session.as_str()], 0));
    let PresenceFrame::Watched { scope, entries, snapshot: true, .. } = watched_frame(&mut carol_frames).await else { panic!("a snapshot") };
    assert_eq!((scope.0.as_str(), entries.iter().map(|entry| entry.session.as_str()).collect::<Vec<_>>()), ("room-b", vec![bob_session.as_str()]));

    let started = Instant::now();
    for n in 0..8 {
        say(&alice, &format!(r#"{{"type":"state","state":{{"n":{n}}}}}"#));
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    let intervals = started.elapsed().as_millis() / 80 + 2;
    let mut seen = Vec::new();
    while seen.last() != Some(&serde_json::json!({ "n": 7 })) {
        let PresenceFrame::Watched { scope, entries, snapshot: false, .. } = watched_frame(&mut carol_frames).await else { panic!("a coalesced change") };
        if scope.0 == "room-a" {
            seen.extend(entries.into_iter().filter(|entry| entry.session == alice_session && !entry.state.is_null()).map(|entry| entry.state));
        }
    }
    assert!(seen.len() < 8 && seen.len() as u128 <= intervals, "eight states reach an 80 ms watcher at most once per interval ({intervals}): {seen:?}");

    say(&carol, r#"{"type":"state","state":{"here":"room-c"}}"#);
    say(&carol, r#"{"type":"watch","scopes":["room-a","room-z"],"intervalMs":80}"#);
    assert_eq!(watched_frame(&mut carol_frames).await, PresenceFrame::Refused { reason: "forbidden room-z".into() });
    assert!(state.presence_rooms.roster(&presence_lane(&Scope("room-a".into()))).iter().all(|entry| entry.session != carol_session), "watching never joins");

    drop(bob);
    tokio::time::timeout(Duration::from_secs(5), bob_task).await.expect("bob's session ends").expect("no panic");
    assert_eq!(watched_frame(&mut carol_frames).await, watched("room-b", Vec::new(), &[bob_session.as_str()], false), "a refused watch kept the previous set");

    say(&carol, r#"{"type":"watch","scopes":[],"intervalMs":80}"#);
    tokio::time::sleep(Duration::from_millis(50)).await;
    say(&alice, r#"{"type":"state","state":{"n":8}}"#);
    let quiet = tokio::time::timeout(Duration::from_millis(400), watched_frame(&mut carol_frames)).await;
    assert!(quiet.is_err(), "an empty watch stops watching: {quiet:?}");

    drop((alice, carol));
    tokio::time::timeout(Duration::from_secs(5), carol_task).await.expect("carol's session ends").expect("no panic");
    let deadline = Instant::now() + Duration::from_secs(5);
    while state.fanout.lanes() > 0 && Instant::now() < deadline {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    assert_eq!(state.fanout.lanes(), 0, "closing releases every lane the socket joined or watched");
}
//#endregion 🔖️PresenceStream

