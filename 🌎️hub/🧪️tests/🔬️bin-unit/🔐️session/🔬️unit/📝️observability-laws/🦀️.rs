/// 📝️ Laws of observability: session spans, the admin route, route templates, limiter counters and the readiness record.

//#region 📝️Observability
/// ⚖️ LAW: a request that reached a handler leaves exactly one structured record carrying the
/// identity the handler resolved — that is the whole point of the module, and the thing an operator
/// reading the stream depends on.
#[tokio::test]
async fn an_authenticated_session_read_reports_one_span_with_its_principal() {
    let (state, sink) = observed_test_state().await;
    let session = issue_test_session(&state, "observed@example.com").await;
    let bearer = format!("Bearer {}", session.token);
    sink.clear();
    let addr = spawn_server(state).await;
    assert_eq!(bounded_http_request(addr, "GET", "/auth/sessions/me", &[("Authorization", bearer.as_str())], &[]).await.status, 200);
    let records = sink.records_for("server.auth.session.read");
    assert_eq!(records.len(), 1, "one request, one record: {:?}", sink.records());
    assert_eq!(records[0].outcome, TraceOutcome::Ok);
    assert!(records[0].request_id.is_some(), "every hub span carries a request id");
    assert!(records[0].duration_us.is_some(), "a closed span measured itself");
}

/// ⚖️ LAW: a refusal is reported as a refusal, not as an absence. A span that simply never closed
/// would leave the operator unable to tell a rejected request from one that was never made.
#[tokio::test]
async fn a_refused_session_read_is_reported_as_refused() {
    let (state, sink) = observed_test_state().await;
    let addr = spawn_server(state).await;
    let refusal = raw_http_request(addr, "GET", "/auth/sessions/me", &[("Authorization", "Bearer not-a-capability")], &[]).await;
    assert!((400..500).contains(&refusal.status), "a malformed capability is a client refusal, got {}", refusal.status);
    let records = sink.records_for("server.auth.session.read");
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].outcome, TraceOutcome::Refused);
    assert!(records[0].detail.is_some(), "a refusal names its stable reason code");
}

/// ⚖️ LAW: `GET /admin/api/observability` is behind the admin gate. A hub bound to a network
/// interface must never hand its own internals — event names, failure counts, latency — to an
/// unauthenticated caller.
#[tokio::test]
async fn the_observability_route_refuses_a_caller_without_an_admin_capability() {
    let state = test_state().await;
    let outsider = issue_test_session(&state, "outsider@example.com").await;
    let outsider_bearer = format!("Bearer {}", outsider.token);
    let addr = spawn_server(state).await;
    assert_eq!(raw_http_request(addr, "GET", "/admin/api/observability", &[], &[]).await.status, 401);
    assert_eq!(raw_http_request(addr, "GET", "/admin/api/observability", &[("Authorization", "Bearer forged")], &[]).await.status, 401);
    assert_eq!(
        raw_http_request(addr, "GET", "/admin/api/observability", &[("Authorization", outsider_bearer.as_str())], &[]).await.status,
        401,
        "a valid session that is not an admin subject is still refused"
    );
}

/// ⚖️ LAW: an admin gets the live table, and the table reflects the requests this very hub served.
#[tokio::test]
async fn the_observability_route_answers_an_admin_with_the_events_its_own_requests_produced() {
    let (mut state, _sink) = observed_test_state().await;
    let headers = authorize_test_admin(&mut state, "ops@example.com").await;
    let bearer = headers.get(axum::http::header::AUTHORIZATION).expect("admin bearer").to_str().expect("ascii bearer").to_string();
    let addr = spawn_server(state).await;
    assert_eq!(bounded_http_request(addr, "GET", "/auth/sessions/me", &[("Authorization", bearer.as_str())], &[]).await.status, 200);

    let response = bounded_http_request(addr, "GET", "/admin/api/observability", &[("Authorization", bearer.as_str())], &[]).await;
    assert_eq!(response.status, 200);
    let body: serde_json::Value = serde_json::from_slice(&response.body).expect("observability json");
    assert_eq!(body["schema"], "semio.hub.observability/v1");
    assert_eq!(body["droppedEvents"], 0);
    let declared: Vec<String> = body["declaredEvents"].as_array().expect("declared events").iter().map(|entry| entry.as_str().expect("event name").to_string()).collect();
    assert_eq!(declared, SERVER_SPAN_EVENTS.to_vec(), "the route ships the framework's own vocabulary, not a hand-kept copy");
    let row = body["rows"].as_array().expect("rows").iter().find(|row| row["event"] == "server.auth.session.read").expect("the session read this test made");
    assert_eq!(row["ok"], 1);
    assert_eq!(row["total"], 1);
    assert_eq!(row["samples"], 1, "the closed span contributed a latency sample");
}

/// ⚖️ LAW: the route answers counters, never records. A replayed record stream would put one
/// tenant's principals, spaces and artifact ids behind another operator's admin capability.
#[tokio::test]
async fn the_observability_view_never_carries_identity_fields() {
    let (tracer, _sink) = Tracer::capturing(TraceLevel::Debug);
    tracer.span("server.document.socket").request("r-1").principal("user:ada").space("space-secret").artifact("doc-secret").ok();
    let routes = HubRouteMetricsV1::default();
    routes.observe("GET", "/spaces/{space_id}/documents/{id}", 200, 5);
    let rendered = serde_json::to_string(&observability_view(&tracer, &routes, None)).expect("observability json");
    for leaked in ["user:ada", "space-secret", "doc-secret", "r-1"] {
        assert!(!rendered.contains(leaked), "`{leaked}` reached the admin route body: {rendered}");
    }
    assert!(rendered.contains("server.document.socket"), "the event name itself is the point of the table");
    assert!(rendered.contains("/spaces/{space_id}/documents/{id}"), "a route is named by its template, never by a concrete path");
}

/// ⚖️ LAW: the admin route reports every matched route this very hub answered under its template, with its
/// answers by class and its latency, next to the DB I/O census and the residency (null without a catalog).
#[tokio::test]
async fn the_observability_route_reports_the_routes_this_hub_answered_by_template() {
    let (mut state, _sink) = observed_test_state().await;
    let headers = authorize_test_admin(&mut state, "ops-routes@example.com").await;
    let bearer = headers.get(axum::http::header::AUTHORIZATION).expect("admin bearer").to_str().expect("ascii bearer").to_string();
    let addr = spawn_server(state).await;
    for _ in 0..3 {
        assert_eq!(bounded_http_request(addr, "GET", "/auth/sessions/me", &[("Authorization", bearer.as_str())], &[]).await.status, 200);
    }
    assert_eq!(bounded_http_request(addr, "GET", "/directory/spaces/space-that-is-not-here", &[("Authorization", bearer.as_str())], &[]).await.status / 100, 4);
    let response = bounded_http_request(addr, "GET", "/admin/api/observability", &[("Authorization", bearer.as_str())], &[]).await;
    assert_eq!(response.status, 200);
    let body: serde_json::Value = serde_json::from_slice(&response.body).expect("observability json");
    let module: serde_json::Value = serde_json::from_str(semio_hub_observability::HUB_OBSERVABILITY_SCHEMA_JSON).expect("observability schema");
    let required: std::collections::BTreeSet<&str> = module["$defs"]["HubObservabilityV1"]["required"].as_array().unwrap().iter().map(|field| field.as_str().unwrap()).collect();
    assert_eq!(body.as_object().unwrap().keys().map(String::as_str).collect::<std::collections::BTreeSet<_>>(), required);
    let route = |method: &str, template: &str| body["routes"].as_array().unwrap().iter().find(|row| row["method"] == method && row["route"] == template).cloned();
    let me = route("GET", "/auth/sessions/me").expect("the session reads this test made");
    assert_eq!((me["requests"].as_u64(), me["successes"].as_u64(), me["samples"].as_u64()), (Some(3), Some(3), Some(3)));
    let space = route("GET", "/directory/spaces/{id}").expect("the refused space read, under its template");
    assert_eq!(space["clientRefusals"].as_u64(), Some(1));
    assert!(!response.body.windows(b"space-that-is-not-here".len()).any(|window| window == b"space-that-is-not-here"), "no concrete path reaches the table");
    assert!(body["residency"].is_null(), "a hub without a catalog has no residency");
    assert!(body["dbIo"]["tasks"].as_u64().is_some());
}

/// ⚖️ LAW: a request the address bucket refuses before its handler runs is still an answer of its route —
/// the route table counts it as `rateLimited` under the template, so an operator sees a lockout where it happens.
#[tokio::test]
async fn the_observability_route_counts_a_limiter_refusal_under_its_route() {
    let (mut state, _clock) = credential_sign_in_state().await;
    seed_credential_user(&state, "limited@example.com", Some(SIGN_IN_PASSWORD)).await;
    let headers = authorize_test_admin(&mut state, "ops-limits@example.com").await;
    let bearer = headers.get(axum::http::header::AUTHORIZATION).expect("admin bearer").to_str().expect("ascii bearer").to_string();
    let addr = spawn_server(state).await;
    let burst = RateLimitClassV1::Auth.policy().burst;
    for attempt in 0..burst {
        assert_eq!(post_sign_in(addr, &sign_in_body("limited@example.com", "a different password")).await.status, 401, "attempt {attempt}");
    }
    assert_eq!(post_sign_in(addr, &sign_in_body("limited@example.com", "a different password")).await.status, 429);
    let response = bounded_http_request(addr, "GET", "/admin/api/observability", &[("Authorization", bearer.as_str())], &[]).await;
    assert_eq!(response.status, 200);
    let body: serde_json::Value = serde_json::from_slice(&response.body).expect("observability json");
    let row = body["routes"].as_array().unwrap().iter().find(|row| row["method"] == "POST" && row["route"] == semio_hub_auth::SESSION_MINT_ROUTE).cloned().expect("the sign-ins this test made");
    assert_eq!((row["requests"].as_u64(), row["clientRefusals"].as_u64(), row["rateLimited"].as_u64()), (Some(u64::from(burst) + 1), Some(u64::from(burst)), Some(1)));
}

/// ⚖️ LAW: the structured readiness record carries the same gates the startup banner names, as
/// stable codes rather than prose — a collector must never have to parse the human line.
#[test]
fn the_readiness_record_names_every_closed_gate_by_its_reason_code() {
    let addr: SocketAddr = "127.0.0.1:8787".parse().expect("test address");
    let ready = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root");
    assert!(ready.blocked_by.is_empty());
    assert_eq!(readiness_trace_detail(&ready, &addr, "loopback"), "addr=127.0.0.1:8787 scope=loopback");

    let blocked = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), false, false, false, true, false, false, false, false, "trusted-catalog-never-published-in-this-data-root");
    let detail = readiness_trace_detail(&blocked, &addr, "loopback");
    assert!(detail.starts_with("addr=127.0.0.1:8787 scope=loopback"));
    for closed in &blocked.blocked_by {
        assert!(detail.contains(&format!(" blocked:{}={}", closed.gate, closed.reason)), "{detail} names {}", closed.gate);
    }
    assert!(!blocked.blocked_by.is_empty(), "the fixture must actually block a gate");
}

/// ⚖️ LAW: the bounded counter table cannot be grown without bound by a hub event name, because
/// every name hub emits is a compile-time literal from the declared vocabulary.
#[test]
fn every_event_name_the_counter_table_must_hold_fits_inside_its_capacity() {
    assert!(SERVER_SPAN_EVENTS.len() < semio_framework_trace::record::COUNTER_EVENT_CAPACITY);
}
//#endregion 📝️Observability
