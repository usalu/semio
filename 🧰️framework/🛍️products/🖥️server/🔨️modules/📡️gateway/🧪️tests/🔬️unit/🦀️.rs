
use super::*;
use crate::contract::{CommandId, PolicyGrant, TraceContext};
use futures::channel::mpsc::unbounded;
use std::time::Duration;

async fn state() -> ServerState {
    Server::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() }).build().await.state().clone()
}

fn actor(id: &str) -> ActorKey {
    ActorKey { tenant: TenantId("t1".into()), kind: "counter".into(), id: id.into() }
}

fn event(stream: &ActorKey, seq: u64) -> EventRecord {
    EventRecord { stream: stream.clone(), seq, hlc: HybridLogicalClock::default(), kind: "counter.incremented".into(), payload: vec![seq as u8] }
}

fn grant(state: &ServerState, point: PolicyPoint, action: &str) {
    let mut engine = state.policy.write().unwrap();
    engine.register_template(PolicyTemplate { name: format!("{point:?}-{action}"), grants: vec![PolicyGrant { point, resource: "*".into(), action: action.to_string() }] });
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
    let server = Server::builder(StorageProfile::Embedded { data_dir: "/tmp/semio-gateway".to_string() })
        .identity("hub", "0.1.0")
        .module(ServerModules::Counting(CountingModule))
        .app("admin", "/srv/admin")
        .admin_token(Some("secret".to_string()))
        .build()
        .await;

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
