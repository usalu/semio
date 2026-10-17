//! 🛂️ The bounded edge of the gateway: client addressing, the throttle, body limits, allowances and route groups. See [`super`].

use super::*;

//#region 🔖️Edge
fn remote(last: u8) -> SocketAddr {
    SocketAddr::from(([198, 51, 100, last], 40_000))
}

fn forwarded(value: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(FORWARDED_FOR, HeaderValue::from_static(value));
    headers
}

/// 📨️ One request as the listener hands it to the router: with the peer's address attached.
pub(super) fn request(method: Method, path: &str, peer: SocketAddr, headers: &[(&'static str, &'static str)], body: Vec<u8>) -> Request {
    let mut request = Request::new(axum::body::Body::from(body));
    *request.method_mut() = method;
    *request.uri_mut() = path.parse().expect("a path");
    for (name, value) in headers {
        request.headers_mut().insert(*name, HeaderValue::from_static(value));
    }
    request.extensions_mut().insert(ConnectInfo(peer));
    request
}

async fn answer(router: &Router, request: Request) -> (StatusCode, HeaderMap, serde_json::Value) {
    let response = router.clone().oneshot(request).await.expect("the router answers");
    let (status, headers) = (response.status(), response.headers().clone());
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.expect("a body");
    (status, headers, serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null))
}

fn command_body() -> Vec<u8> {
    serde_json::to_vec(&envelope()).expect("an envelope")
}

const JSON: (&str, &str) = ("content-type", "application/json");

#[test]
fn the_client_address_is_the_peer_unless_a_trusted_proxy_names_it() {
    let proxy = SocketAddr::from(([10, 0, 0, 2], 55_000));
    let named = ClientKey::of("203.0.113.7".parse().expect("an address"));
    assert_eq!(client_key(ClientAddressing::Peer, &forwarded("203.0.113.7"), Some(proxy)), ClientKey::of(proxy.ip()), "an untrusted header is never read");
    assert_eq!(client_key(ClientAddressing::Forwarded, &forwarded("203.0.113.7"), Some(proxy)), named);
    assert_eq!(client_key(ClientAddressing::Forwarded, &forwarded("192.0.2.1, 203.0.113.7"), Some(proxy)), named, "the proxy's own entry is the last one");
    let mut twice = forwarded("192.0.2.1");
    twice.append(FORWARDED_FOR, HeaderValue::from_static("203.0.113.7"));
    assert_eq!(client_key(ClientAddressing::Forwarded, &twice, Some(proxy)), named, "of several header lines the last is the proxy's");
    assert_eq!(client_key(ClientAddressing::Forwarded, &HeaderMap::new(), Some(proxy)), ClientKey::of(proxy.ip()), "a probe beside the proxy is its own peer");
    assert_eq!(client_key(ClientAddressing::Forwarded, &forwarded("unknown"), Some(proxy)), ClientKey::of(proxy.ip()));
    assert_eq!(client_key(ClientAddressing::Peer, &HeaderMap::new(), None), ClientKey::of(IpAddr::V4(Ipv4Addr::UNSPECIFIED)));
}

#[test]
fn a_request_spends_from_the_bucket_of_its_class() {
    let plain = HeaderMap::new();
    assert_eq!(request_class(&Method::POST, "/commands", &plain), RequestClass::Write);
    assert_eq!(request_class(&Method::PUT, "/blobs/00", &plain), RequestClass::Write);
    assert_eq!(request_class(&Method::POST, QUERIES_PATH, &plain), RequestClass::Read);
    assert_eq!(request_class(&Method::GET, "/actors/t/k/i/events", &plain), RequestClass::Read);
    assert_eq!(request_class(&Method::OPTIONS, "/commands", &plain), RequestClass::Read);
    let mut upgrade = HeaderMap::new();
    upgrade.insert(header::UPGRADE, HeaderValue::from_static("WebSocket"));
    assert_eq!(request_class(&Method::GET, "/scopes/s/presence/ws", &upgrade), RequestClass::Upgrade);
}

#[tokio::test]
async fn a_refusal_names_its_wait_in_the_header_and_in_the_gateways_own_body() {
    let throttled = ServerError::from(Refusal::Throttled { retry_after: Duration::from_millis(1500) }).into_response();
    assert_eq!((throttled.status(), throttled.headers().get(header::RETRY_AFTER).map(|value| value.to_str().unwrap().to_string())), (StatusCode::TOO_MANY_REQUESTS, Some("2".to_string())));
    let body: ErrorBody = serde_json::from_slice(&axum::body::to_bytes(throttled.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body, ErrorBody { kind: "throttled".into(), message: "too many requests".into(), retry_after_ms: Some(1500), allowance: None });
    let spent = ServerError::Throttled { wait: Duration::from_secs(36), allowance: Some("sign-up") };
    assert_eq!(spent.to_string(), "the sign-up allowance of this address is spent: retry in 36000 ms");
    let spent = spent.into_response();
    assert_eq!((spent.status(), spent.headers()[header::RETRY_AFTER].to_str().unwrap()), (StatusCode::TOO_MANY_REQUESTS, "36"));
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(spent.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body, serde_json::json!({ "kind": "throttled", "message": "the sign-up allowance of this address is spent", "retryAfterMs": 36000, "allowance": "sign-up" }), "a spent named allowance is named");
    let overloaded = ServerError::from(Refusal::Overloaded { retry_after: Duration::from_secs(1) }).into_response();
    assert_eq!((overloaded.status(), overloaded.headers()[header::RETRY_AFTER].to_str().unwrap()), (StatusCode::SERVICE_UNAVAILABLE, "1"));
    let body: serde_json::Value = serde_json::from_slice(&axum::body::to_bytes(overloaded.into_body(), usize::MAX).await.unwrap()).unwrap();
    assert_eq!(body, serde_json::json!({ "kind": "overloaded", "message": "overloaded", "retryAfterMs": 1000 }));
    let missing = ServerError::NotFound("no blob".into()).into_response();
    assert!(missing.headers().get(header::RETRY_AFTER).is_none());
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&axum::body::to_bytes(missing.into_body(), usize::MAX).await.unwrap()).unwrap(), serde_json::json!({ "kind": "notFound", "message": "not found: no blob" }), "an error that does not pass names no wait");
}

#[tokio::test]
async fn a_fault_of_the_server_and_a_denial_of_its_policy_tell_the_caller_nothing() {
    for (error, status, kind, message) in [
        (ServerError::from(StorageError::Backend("disk I/O error at /srv/quiz/data/proctor.sqlite".into())), StatusCode::INTERNAL_SERVER_ERROR, "internal", "internal error"),
        (ServerError::from(StorageError::Conflict("idempotency key enroll:architecture:roster:7 is already bound to command enroll:architecture:roster:7".into())), StatusCode::CONFLICT, "conflict", "conflict: the write contradicts what is stored"),
        (ServerError::from(StorageError::SequenceGap { expected: 3, got: 5 }), StatusCode::CONFLICT, "conflict", "conflict: the write contradicts what is stored"),
        (ServerError::Forbidden("closed by default: no grant lets anonymous do 'read' on 'stream:t/quiz-roster/roster'".into()), StatusCode::FORBIDDEN, "forbidden", "forbidden"),
    ] {
        let response = error.into_response();
        assert_eq!(response.status(), status);
        let body: ErrorBody = serde_json::from_slice(&axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap()).unwrap();
        assert_eq!(body, ErrorBody { kind: kind.into(), message: message.into(), retry_after_ms: None, allowance: None });
    }
    let state = state().await;
    let denied = state.authorize(&PolicyRequest { point: PolicyPoint::EventDelivery, principal: Principal::Anonymous, scope: None, resource: "stream:t1/counter/c1".into(), action: "read".into() }).expect_err("closed by default");
    assert!(denied.to_string().contains("no grant lets anonymous"), "the operator's account keeps the reason: {denied}");
    assert_eq!(denied.message(), "forbidden");
}

async fn limited(limits: Limits, addressing: ClientAddressing, routes: RouteGroups, disclosure: InstanceDisclosure) -> (Server<TestInstance>, Router) {
    let server = Server::<TestInstance>::builder(StorageProfile::Ephemeral).module(CountingModule).limits(limits).addressing(addressing).routes(routes).disclosure(disclosure).build().await.expect("built");
    server.state().policy.write().unwrap().assign("anonymous".to_string(), "author".to_string());
    let router = server.throttled(server.router());
    (server, router)
}

#[tokio::test]
async fn a_body_past_the_limit_or_of_another_type_is_refused_in_the_gateways_own_shape() {
    let (_, router) = limited(Limits { body_bytes: Some(2048), ..Limits::default() }, ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let (status, _, body) = answer(&router, request(Method::POST, "/commands", remote(1), &[JSON], command_body())).await;
    assert_eq!((status, body["status"].as_str()), (StatusCode::OK, Some("accepted")), "a command within the limit is served: {body}");
    let mut large = envelope();
    large.payload = vec![7; 4096];
    let (status, _, body) = answer(&router, request(Method::POST, "/commands", remote(1), &[JSON], serde_json::to_vec(&large).unwrap())).await;
    assert_eq!((status, body), (StatusCode::PAYLOAD_TOO_LARGE, serde_json::json!({ "kind": "payloadTooLarge", "message": "payload too large" })));
    let (status, _, body) = answer(&router, request(Method::POST, "/commands", remote(1), &[("content-type", "text/plain")], command_body())).await;
    assert_eq!((status, body["kind"].as_str()), (StatusCode::BAD_REQUEST, Some("badRequest")), "a form or text post never reaches a handler: {body}");
    let (status, _, body) = answer(&router, request(Method::POST, "/queries", remote(1), &[JSON], b"{ not json".to_vec())).await;
    assert_eq!((status, body["kind"].as_str()), (StatusCode::BAD_REQUEST, Some("badRequest")));
}

#[tokio::test]
async fn a_body_is_taken_whole_within_its_limit_and_its_patience() {
    type Chunk = Result<Bytes, std::convert::Infallible>;
    let streamed = |chunks: Vec<&'static [u8]>| {
        let mut request = request(Method::POST, "/commands", remote(1), &[JSON], Vec::new());
        *request.body_mut() = axum::body::Body::from_stream(futures::stream::iter(chunks.into_iter().map(|chunk| Chunk::Ok(Bytes::from_static(chunk)))));
        request
    };
    let patient = Duration::from_secs(5);
    let whole = body_taken(streamed(vec![b"12345", b"67890"]), 10, patient).await.expect("ten bytes fit a limit of ten");
    assert_eq!(whole.extensions().get::<ConnectInfo<SocketAddr>>().map(|info| info.0), Some(remote(1)), "the request is the same but for its body");
    assert_eq!(axum::body::to_bytes(whole.into_body(), 64).await.expect("in hand").as_ref(), b"1234567890");
    assert!(matches!(body_taken(streamed(vec![b"12345", b"678901"]), 10, patient).await, Err(ServerError::PayloadTooLarge)), "a body that delivers more than it may is not taken");
    let declared = request(Method::POST, "/commands", remote(1), &[("content-length", "11")], Vec::new());
    assert!(matches!(body_taken(declared, 10, patient).await, Err(ServerError::PayloadTooLarge)), "a body that declares more than it may is not read at all");
    let mut silent = request(Method::POST, "/commands", remote(1), &[JSON], Vec::new());
    *silent.body_mut() = axum::body::Body::from_stream(futures::stream::once(async { Chunk::Ok(Bytes::from_static(b"{")) }).chain(futures::stream::pending()));
    let stalled = body_taken(silent, 10, Duration::from_millis(60)).await.expect_err("a body that stops arriving is given up on");
    assert_eq!((stalled.status(), stalled.kind()), (StatusCode::REQUEST_TIMEOUT, "stalled"));
}

#[tokio::test]
async fn a_refused_request_is_still_read_so_its_connection_can_go_on() {
    let rate = Rate::per_second(1, 1);
    let (_, router) = limited(Limits { writes: rate, ..Limits::default() }, ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let read = Arc::new(AtomicU64::new(0));
    let counted = |read: &Arc<AtomicU64>| {
        let read = Arc::clone(read);
        let mut request = request(Method::POST, "/commands", remote(1), &[JSON], Vec::new());
        *request.body_mut() = axum::body::Body::from_stream(futures::stream::iter(command_body().chunks(16).map(<[u8]>::to_vec).collect::<Vec<_>>()).map(move |chunk| {
            read.fetch_add(chunk.len() as u64, Ordering::Relaxed);
            Ok::<_, std::convert::Infallible>(Bytes::from(chunk))
        }));
        request
    };
    assert_eq!(answer(&router, counted(&read)).await.0, StatusCode::OK);
    let served = read.swap(0, Ordering::Relaxed);
    assert_eq!(served, command_body().len() as u64);
    assert_eq!(answer(&router, counted(&read)).await.0, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(read.load(Ordering::Relaxed), served, "the refused command's body was read to its end before the refusal was sent");
}

#[tokio::test]
async fn the_throttle_refuses_before_a_handler_runs_and_keys_on_the_address_the_proxy_names() {
    let rate = Rate::per_second(1, 2);
    let (server, router) = limited(Limits { writes: rate, ..Limits::default() }, ClientAddressing::Forwarded, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let proxy = remote(1);
    let hall = [JSON, ("x-forwarded-for", "203.0.113.7")];
    for _ in 0..2 {
        assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &hall, command_body())).await.0, StatusCode::OK);
    }
    let (status, headers, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, b"never read".to_vec())).await;
    assert_eq!((status, headers[header::RETRY_AFTER].to_str().unwrap(), body["kind"].as_str()), (StatusCode::TOO_MANY_REQUESTS, "1", Some("throttled")), "{body}");
    assert!(body["retryAfterMs"].as_u64().is_some_and(|wait| (1..=1000).contains(&wait)), "{body}");
    assert_eq!(server.state().store.read().await.last_seq(&actor("c1")).await.unwrap(), 2, "the refused command reached no turn");
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command_body())).await.0, StatusCode::OK, "another address behind the same proxy has its own bucket");
    assert_eq!(answer(&router, request(Method::GET, "/instance", proxy, &[("x-forwarded-for", "203.0.113.7")], Vec::new())).await.0, StatusCode::OK, "a read spends from another bucket");
    assert_eq!(server.state().throttle.in_flight(), 0, "an answered request is no longer in flight");

    let (_, direct) = limited(Limits { writes: rate, ..Limits::default() }, ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    for claimed in ["203.0.113.1", "203.0.113.2"] {
        assert_eq!(answer(&direct, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", claimed)], command_body())).await.0, StatusCode::OK);
    }
    assert_eq!(answer(&direct, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.3")], command_body())).await.0, StatusCode::TOO_MANY_REQUESTS, "an address a client claims buys no bucket where no proxy is trusted");
}

#[tokio::test]
async fn a_classed_command_spends_its_named_allowance_and_is_handed_back_what_kept_nothing() {
    let limits = Limits { allowances: vec![Allowance { name: CREATION, rate: Rate::per_hour(60, 2) }], ..Limits::default() };
    let (server, router) = limited(limits, ClientAddressing::Forwarded, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    let proxy = remote(1);
    let hall = [JSON, ("x-forwarded-for", "203.0.113.7")];
    let command = |target: &str, kind: &str, key: &str| {
        let mut command = envelope();
        (command.target, command.kind, command.command_id, command.idempotency_key) = (actor(target), kind.into(), CommandId(key.into()), Some(crate::contract::IdempotencyKey(key.into())));
        serde_json::to_vec(&command).expect("an envelope")
    };
    for attempt in 0..5 {
        let (status, _, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, command("new-a", "counter.forbid", &format!("refused-{attempt}")))).await;
        assert_eq!((status, body["status"].as_str()), (StatusCode::OK, Some("rejected")), "{body}");
    }
    for created in ["new-a", "new-b"] {
        let (status, _, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, command(created, "counter.increment", created))).await;
        assert_eq!((status, body["status"].as_str(), body["events"].as_array().map(Vec::len)), (StatusCode::OK, Some("accepted"), Some(1)), "five rejected creations spent nothing: {body}");
    }
    let (status, headers, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, command("new-c", "counter.increment", "new-c"))).await;
    assert_eq!((status, headers[header::RETRY_AFTER].to_str().unwrap()), (StatusCode::TOO_MANY_REQUESTS, "60"));
    assert_eq!((body["kind"].as_str(), body["allowance"].as_str(), body["message"].as_str()), (Some("throttled"), Some(CREATION), Some("the creation allowance of this address is spent")), "{body}");
    assert!(body["retryAfterMs"].as_u64().is_some_and(|wait| (59_000..=60_000).contains(&wait)), "the wait is the time the next token takes: {body}");
    assert_eq!(server.state().store.read().await.last_seq(&actor("new-c")).await.unwrap(), 0, "the refused creation reached no turn");
    for _ in 0..3 {
        let (status, _, body) = answer(&router, request(Method::POST, "/commands", proxy, &hall, command("new-a", "counter.increment", "new-a"))).await;
        assert_eq!((status, body["status"].as_str(), body["events"].as_array().map(Vec::len)), (StatusCode::TOO_MANY_REQUESTS, None, None), "with the allowance spent even a replay waits: {body}");
    }
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &hall, command("c1", "counter.increment", "plain"))).await.0, StatusCode::OK, "a command of no class spends no named allowance");
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command("new-d", "counter.increment", "new-d"))).await.0, StatusCode::OK, "another address has its own allowance");
    for _ in 0..3 {
        let (status, _, body) = answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command("new-d", "counter.increment", "new-d"))).await;
        assert_eq!((status, body["status"].as_str(), body["events"].as_array().map(Vec::len)), (StatusCode::OK, Some("accepted"), Some(0)), "a replay is answered and spends nothing: {body}");
    }
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command("new-e", "counter.increment", "new-e"))).await.0, StatusCode::OK, "three replays left the second token of that address");
    assert_eq!(answer(&router, request(Method::POST, "/commands", proxy, &[JSON, ("x-forwarded-for", "203.0.113.8")], command("new-f", "counter.increment", "new-f"))).await.0, StatusCode::TOO_MANY_REQUESTS);
}

#[test]
fn a_command_kept_something_when_it_committed_an_event_or_was_deferred() {
    let receipt = CommandReceipt { command_id: CommandId("c".into()), actor: actor("c1"), revision: Revision(1), accepted_at: HybridLogicalClock::default() };
    assert!(kept(&CommandOutcome::Accepted { receipt: receipt.clone(), events: vec![event(&actor("c1"), 1)], frontier: None }));
    assert!(!kept(&CommandOutcome::Accepted { receipt: receipt.clone(), events: Vec::new(), frontier: None }), "a replay and an effect-only turn commit no event");
    assert!(kept(&CommandOutcome::Transformed { receipt: receipt.clone(), canonical_events: vec![event(&actor("c1"), 1)], frontier: None, notices: Vec::new() }));
    assert!(kept(&CommandOutcome::Pending { receipt: receipt.clone(), process: crate::contract::ProcessId("p".into()) }));
    assert!(!kept(&CommandOutcome::Rejected { receipt, reason: Rejection::Invalid { detail: "no".into() }, notices: Vec::new() }));
}

#[tokio::test]
async fn the_instance_turns_requests_away_at_its_in_flight_cap() {
    let (server, router) = limited(Limits { in_flight: 1, ..Limits::default() }, ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let held = server.state().throttle.enter().expect("one request in flight");
    let (status, headers, body) = answer(&router, request(Method::GET, "/instance", remote(1), &[], Vec::new())).await;
    assert_eq!((status, headers[header::RETRY_AFTER].to_str().unwrap(), body["kind"].as_str()), (StatusCode::SERVICE_UNAVAILABLE, "1", Some("overloaded")));
    drop(held);
    assert_eq!(answer(&router, request(Method::GET, "/instance", remote(1), &[], Vec::new())).await.0, StatusCode::OK);
}

#[tokio::test]
async fn an_instance_mounts_only_the_route_groups_it_asked_for() {
    let (_, core) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    let (_, all) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::ALL, InstanceDisclosure::Whole).await;
    let hash = "00".repeat(32);
    let unused = [(Method::GET, "/apps".to_string()), (Method::GET, format!("/blobs/{hash}")), (Method::PUT, format!("/blobs/{hash}")), (Method::POST, "/scopes/space-1/ephemeral".to_string())];
    for (method, path) in &unused {
        assert_eq!(answer(&core, request(method.clone(), path, remote(1), &[JSON], b"{}".to_vec())).await.0, StatusCode::NOT_FOUND, "{method} {path} does not exist on a core instance");
        assert_ne!(answer(&all, request(method.clone(), path, remote(1), &[JSON], b"{}".to_vec())).await.0, StatusCode::NOT_FOUND, "{method} {path} exists where every group is mounted");
    }
    for path in ["/instance", "/counting/health"] {
        assert_eq!(answer(&core, request(Method::GET, path, remote(1), &[], Vec::new())).await.0, StatusCode::OK, "{path}");
    }
    assert_eq!(answer(&core, request(Method::POST, "/commands", remote(1), &[JSON], command_body())).await.0, StatusCode::OK);
    assert_eq!(answer(&core, request(Method::GET, "/actors/t1/counter/c1/events", remote(1), &[], Vec::new())).await.0, StatusCode::FORBIDDEN, "the durable lane is mounted and closed by default");
}

#[tokio::test]
async fn a_public_instance_document_says_what_to_address_and_not_how_it_is_authorized() {
    let (server, router) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::CORE, InstanceDisclosure::Public).await;
    let (status, _, body) = answer(&router, request(Method::GET, "/instance", remote(1), &[], Vec::new())).await;
    assert_eq!(status, StatusCode::OK);
    let disclosed: ServerInstanceDefinition = serde_json::from_value(body).expect("the definition's own shape");
    assert_eq!(disclosed, server.definition().public());
    assert_eq!((disclosed.modules[0].id.as_str(), disclosed.modules[0].policies.len()), ("counting", 0));
    assert_eq!(server.definition().modules[0].policies.len(), 1, "in process the definition is whole");
    let (_, whole) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    assert_eq!(answer(&whole, request(Method::GET, "/instance", remote(1), &[], Vec::new())).await.2["modules"][0]["policies"][0]["name"], "author");
}

#[tokio::test]
async fn a_socket_is_counted_against_its_address_and_the_instance_until_it_closes() {
    let (server, _) = limited(Limits { sockets_per_client: 1, sockets: 2, ..Limits::default() }, ClientAddressing::Forwarded, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    let state = server.state();
    let first = state.open_socket(&forwarded("203.0.113.7"), Some(remote(1))).expect("the first socket of an address");
    assert_eq!(state.open_socket(&forwarded("203.0.113.7"), Some(remote(1))).err().map(|error| error.status()), Some(StatusCode::TOO_MANY_REQUESTS));
    let _second = state.open_socket(&forwarded("203.0.113.8"), Some(remote(1))).expect("another address");
    assert_eq!(state.open_socket(&forwarded("203.0.113.9"), Some(remote(1))).err().map(|error| error.status()), Some(StatusCode::SERVICE_UNAVAILABLE));
    drop(first);
    assert!(state.open_socket(&forwarded("203.0.113.7"), Some(remote(1))).is_ok());
    assert_eq!(PresenceSettings::default().inbound_bytes(), 4096, "the presence socket reads no message past twice its largest state");
}

#[tokio::test]
async fn a_module_refuses_a_malformed_target_before_the_bus_places_anything() {
    let (server, router) = limited(Limits::default(), ClientAddressing::Peer, RouteGroups::CORE, InstanceDisclosure::Whole).await;
    let mut malformed = envelope();
    malformed.target = actor("no such id");
    malformed.idempotency_key = Some(crate::contract::IdempotencyKey("k1".into()));
    let (status, _, body) = answer(&router, request(Method::POST, "/commands", remote(1), &[JSON], serde_json::to_vec(&malformed).unwrap())).await;
    assert_eq!((status, body["status"].as_str(), &body["reason"]), (StatusCode::OK, Some("rejected"), &serde_json::json!({ "kind": "invalid", "detail": "target-malformed" })), "{body}");
    assert_eq!(server.state().authority.lock().await.directory().placed(), 0);
}
//#endregion 🔖️Edge
