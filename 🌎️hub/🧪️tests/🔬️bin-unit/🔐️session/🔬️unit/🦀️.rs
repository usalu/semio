
#[cfg(test)]
use protocol::{AckStage, ActorId, ApplyOutcome, ClientFrame, Lane, MutationEnvelope, RuntimeFrontierSummary, ServerFrame};
#[cfg(test)]
use directory::os_directory::{DirectoryCommand, DirectorySessionKindV1, DirectorySpaceRole, DirectorySpaceVisibility};
#[cfg(test)]
use axum::http::{HeaderMap, StatusCode};
#[cfg(test)]
use axum::extract::{Path, State};
#[cfg(test)]
use server::contract::{Principal, SessionId};
#[cfg(test)]
use semio_framework_trace::record::SERVER_SPAN_EVENTS;
#[cfg(test)]
use server::storage::SessionRecord;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::issue_document_socket_grant_fixture;
use semio_hub_ledgers::guard::{AdminSubject, admin_provider_digest, validate_auth_startup};
use std::sync::Arc;
use semio_hub_route_models::artifact_creation::{ArtifactCreationHttpAdmissionV1, ArtifactCreationHttpControlV1, ArtifactCreationHttpTaskOwnerV1};
use semio_hub_directory_contract::model::{AuthSessionIssue, AuthSessionKind, SocketSessionBindingStatus, SpaceRole};
use protocol::{Bootstrap, ArtifactId as WireArtifactId};
use semio_hub_auth::CredentialSignInPolicyV1;
use semio_hub_runtime::transport::{CrossOriginPolicyV1, ForwardedTlsTrustV1, forwarded_external_host, forwarded_request_is_secure, is_browser_origin, is_loopback_origin};
use directory::os_directory::DocumentScope;
use semio_hub_runtime::startup::{HubError, SagaDrainSupervisor, drain_instance_sagas, instance_state};
use semio_hub_runtime::readiness::{HubMode, declare_public_session_issuance, hub_readiness, readiness_trace_detail};
use semio_hub_auth::rate_limit::{HubRateLimiterV1, RateLimitClassV1};
use semio_hub_observability::HubRouteMetricsV1;
use semio_hub_state::HubState;
use semio_hub_route_testkit::{LawHangWatchdogV1, RAW_HTTP_READ_HANG_GUARD, RawHttpResponse, STUDIO, TestIssuedSession, announce_document_for_test, authorize_test_admin, bearer_headers, bounded_law_step, client_binary, create_space_for_test, document_socket_request, issue_test_session, json_body, next_close_code, next_server_frame, next_server_frame_at, observed_test_state, presence_hex_bytes, presence_normalization_fixture, raw_http_get, raw_http_request, run_socket_test, sample_envelope, seed_author_token, socket_hello, socket_request, spawn_server, spawn_server_with_posture, tempdir, test_instance_state, test_state, upsert_member_for_test};
use semio_hub_ledgers::socket_grants::{SESSION_PROTOCOL_V1, SocketBindingKeyV1};
use semio_hub_directory_contract::{SessionCapability, identity_subject_digest};
use std::net::SocketAddr;
use semio_hub_runtime::lifecycle::{TerminationSignalV1, first_termination_signal};
use semio_framework_trace::record::{TraceLevel, TraceOutcome, Tracer};
use tokio_tungstenite::tungstenite::Message as WsMessage;
use semio_hub_session_routes::directory_commands::authorize_directory_command;
use tokio_tungstenite::connect_async;
use semio_hub_runtime::presence::db_artifact_id;
use semio_hub_route_models::document_socket::{frame_carries_frontier, project_server_frame, wire_frontier_to_db};
use semio_hub_session_routes::session_agent::get_session_me;
use semio_hub_runtime::support::now_ms;
use semio_hub_route_models::admin::observability_view;
use semio_hub_routes::router;
/// ⏱️ One request, bounded. A hub route that never answers must fail this test rather than hang the
/// whole bin suite for every other slice sharing the binary — an unbounded socket read in a shared
/// suite is a denial of service on the fleet, not a diagnostic.
async fn bounded_http_request(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> RawHttpResponse {
    match tokio::time::timeout(RAW_HTTP_READ_HANG_GUARD, raw_http_request(addr, method, path, headers, body)).await {
        Ok(response) => response,
        Err(_) => panic!("{method} {path} did not answer within {RAW_HTTP_READ_HANG_GUARD:?}"),
    }
}

//#region 🔖️CredentialSignIn
const SIGN_IN_PASSWORD: &str = "correct horse battery staple";

const SIGN_IN_ITERATIONS: u32 = 1_000;

/// 🕰️ A hand-stepped clock for the rate limiter, so a lockout law never sleeps.
struct TestRateLimitClock(std::sync::atomic::AtomicI64);

impl TestRateLimitClock {
    fn advance(&self, milliseconds: i64) {
        self.0.fetch_add(milliseconds, std::sync::atomic::Ordering::SeqCst);
    }
}

impl semio_hub_auth::rate_limit::RateLimitClockV1 for TestRateLimitClock {
    fn now_ms(&self) -> i64 {
        self.0.load(std::sync::atomic::Ordering::SeqCst)
    }
}

async fn credential_sign_in_state() -> (HubState, Arc<TestRateLimitClock>) {
    let mut state = test_state().await;
    let clock = Arc::new(TestRateLimitClock(std::sync::atomic::AtomicI64::new(1_000_000)));
    state.credential_sign_in = CredentialSignInPolicyV1::enabled(3_600, SIGN_IN_ITERATIONS).expect("credential sign-in policy");
    state.rate_limits = Arc::new(HubRateLimiterV1::new(clock.clone()));
    (state, clock)
}

async fn seed_credential_user(state: &HubState, email: &str, password: Option<&str>) -> String {
    let credential = password.map(|password| semio_hub_auth::password::PasswordCredentialV1::mint(password, SIGN_IN_ITERATIONS).expect("mint credential").encode());
    state.directory.create_user(email, email, credential.as_deref(), None, None).await.expect("credential user").id
}

fn sign_in_body(email: &str, password: &str) -> Vec<u8> {
    serde_json::json!({ "schema": "semio.hub.auth.credential-sign-in/v1", "email": email, "password": password, "deviceInstanceId": "test-device", "clientClass": "browser" }).to_string().into_bytes()
}

async fn post_sign_in(addr: SocketAddr, body: &[u8]) -> RawHttpResponse {
    raw_http_request(addr, "POST", "/auth/sessions", &[("content-type", "application/json")], body).await
}

/// 🧨️ A sign-in whose instance session store refuses the write is refused `directory-unavailable`,
/// records no instance session, and withdraws the directory issuance in the same request — the
/// failing store write reaches the caller instead of leaving two halves of one session fact apart.
#[cfg(unix)]
#[tokio::test]
async fn credential_sign_in_reports_a_failing_instance_session_store() {
    use std::os::unix::fs::PermissionsExt;
    let (mut state, _clock) = credential_sign_in_state().await;
    let dir = tempdir("instance-session-fault");
    state.instance = test_instance_state(&dir).await;
    let user_id = seed_credential_user(&state, "store-fault@example.com", Some(SIGN_IN_PASSWORD)).await;
    let sessions = dir.join("instance/sessions");
    assert!(sessions.is_dir(), "the durable profile keeps sessions under {}", sessions.display());
    std::fs::set_permissions(&sessions, std::fs::Permissions::from_mode(0o500)).expect("refuse session writes");
    let addr = spawn_server(state.clone()).await;
    let response = post_sign_in(addr, &sign_in_body("store-fault@example.com", SIGN_IN_PASSWORD)).await;
    std::fs::set_permissions(&sessions, std::fs::Permissions::from_mode(0o700)).expect("restore session writes");
    assert_eq!(response.status, 503, "{}", String::from_utf8_lossy(&response.body));
    assert_eq!(json_body(&response)["error"].as_str(), Some("directory-unavailable"));
    assert!(std::fs::read_dir(&sessions).expect("sessions dir").all(|entry| entry.expect("session entry").file_name() == "format.json"), "no instance session was recorded beside the store's own format stamp");
    assert!(state.instance.sessions.lock().await.revoke_principal(&Principal::User { id: user_id }).await.expect("instance store readable") == 0, "the instance store holds nothing for the refused principal");
}

/// 🔬️ `POST /auth/sessions` mints a real session from a password credential: the email is matched
/// case-insensitively, the response is exactly `{token, user_id}`, the token then authenticates
/// `GET /auth/sessions/me` as an `external` session, and both the attempt and the issuance are in
/// the durable authentication log.
#[tokio::test]
async fn credential_sign_in_mints_a_session_whose_token_then_authenticates() {
    let (state, _clock) = credential_sign_in_state().await;
    let user_id = seed_credential_user(&state, "signin@example.com", Some(SIGN_IN_PASSWORD)).await;
    let addr = spawn_server(state.clone()).await;
    let response = post_sign_in(addr, &sign_in_body("  SignIn@Example.COM ", SIGN_IN_PASSWORD)).await;
    assert_eq!(response.status, 200, "{}", String::from_utf8_lossy(&response.body));
    assert!(response.headers.to_lowercase().contains("cache-control: no-store"));
    let minted = json_body(&response);
    assert_eq!(minted.as_object().expect("mint object").len(), 2);
    assert_eq!(minted["user_id"].as_str(), Some(user_id.as_str()));
    let token = minted["token"].as_str().expect("minted token").to_string();
    assert!(token.starts_with("session.v1.") && token.len() == 108);

    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {token}").parse().expect("bearer"));
    let me = get_session_me(headers, State(state.clone())).await.expect("session authority");
    assert_eq!(me.0.user_id, user_id);
    assert_eq!(me.0.email, "signin@example.com");
    assert!(matches!(me.0.session_kind, DirectorySessionKindV1::External));
    assert!(me.0.expires_at > now_ms() && me.0.expires_at <= now_ms() + 3_600_000);

    let audit = state.directory.list_auth_audit(32, 0).await.expect("auth audit");
    assert!(audit.iter().any(|fact| fact.event_kind == "credential-sign-in" && fact.outcome_code == "success" && fact.target_user_id.as_deref() == Some(user_id.as_str()) && fact.peer_class == "browser"));
    assert!(audit.iter().any(|fact| fact.event_kind == "session-issued" && fact.target_user_id.as_deref() == Some(user_id.as_str())));
}

/// 🔬️ An unknown account, an account with no password credential, an unreadable stored credential
/// and a wrong password are ONE observable refusal — a caller cannot enumerate accounts — and each
/// refusal is journaled with its public reason code and no user-distinguishing detail.
#[tokio::test]
async fn credential_sign_in_refuses_every_wrong_credential_with_one_code() {
    let (state, _clock) = credential_sign_in_state().await;
    let known = seed_credential_user(&state, "known@example.com", Some(SIGN_IN_PASSWORD)).await;
    seed_credential_user(&state, "credential-free@example.com", None).await;
    let addr = spawn_server(state.clone()).await;
    for (email, password) in [("known@example.com", "a different password"), ("missing@example.com", SIGN_IN_PASSWORD), ("credential-free@example.com", SIGN_IN_PASSWORD)] {
        let response = post_sign_in(addr, &sign_in_body(email, password)).await;
        assert_eq!(response.status, 401, "{email}");
        let body = json_body(&response);
        assert_eq!(body["schema"].as_str(), Some("semio.hub.auth.error/v1"));
        assert_eq!(body["error"].as_str(), Some("invalid-credentials"), "{email}");
        assert!(body.get("retryAfterMs").is_none());
    }
    for malformed in [
        serde_json::json!({ "schema": "semio.hub.auth.credential-sign-in/v1", "email": "known@example.com", "password": SIGN_IN_PASSWORD, "deviceInstanceId": "test-device", "clientClass": "browser", "extra": 1 }),
        serde_json::json!({ "schema": "semio.hub.auth.credential-sign-in/v2", "email": "known@example.com", "password": SIGN_IN_PASSWORD, "deviceInstanceId": "test-device", "clientClass": "browser" }),
        serde_json::json!({ "schema": "semio.hub.auth.credential-sign-in/v1", "email": "known@example.com", "password": "short", "deviceInstanceId": "test-device", "clientClass": "browser" }),
        serde_json::json!({ "schema": "semio.hub.auth.credential-sign-in/v1", "email": "not-an-email", "password": SIGN_IN_PASSWORD, "deviceInstanceId": "test-device", "clientClass": "browser" }),
    ] {
        let response = post_sign_in(addr, malformed.to_string().as_bytes()).await;
        assert_eq!(response.status, 400, "{malformed}");
        assert_eq!(json_body(&response)["error"].as_str(), Some("malformed-request"));
    }
    let failures = state.directory.list_auth_audit(32, 0).await.expect("auth audit").into_iter().filter(|fact| fact.event_kind == "credential-sign-in" && fact.outcome_code == "failure").collect::<Vec<_>>();
    assert_eq!(failures.len(), 3, "one journaled fact per attempt that reached the decision");
    assert!(failures.iter().all(|fact| fact.reason_code.as_deref() == Some("invalid-credentials")));
    assert_eq!(failures.iter().filter(|fact| fact.target_user_id.as_deref() == Some(known.as_str())).count(), 1);
}

/// 🔬️ A hub that was not told to issue publicly refuses every credential — including a correct one —
/// and `/readyz` says so: `publicSessionIssuance` is the policy, not a constant.
#[tokio::test]
async fn credential_sign_in_is_refused_and_undeclared_when_the_deployment_did_not_enable_it() {
    let state = test_state().await;
    assert!(!state.credential_sign_in.is_enabled());
    seed_credential_user(&state, "disabled@example.com", Some(SIGN_IN_PASSWORD)).await;
    let addr = spawn_server(state.clone()).await;
    let response = post_sign_in(addr, &sign_in_body("disabled@example.com", SIGN_IN_PASSWORD)).await;
    assert_eq!(response.status, 403);
    assert_eq!(json_body(&response)["error"].as_str(), Some("credential-sign-in-disabled"));
    let readiness = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root");
    assert!(!declare_public_session_issuance(readiness.clone(), false).authentication.public_session_issuance);
    assert!(declare_public_session_issuance(readiness, true).authentication.public_session_issuance);
}

/// 🔬️ The token bucket in front of sign-in is real: the burst is admitted, the next attempt is
/// `429` with a `retry-after`, a CORRECT password is refused while the bucket is empty, and exactly
/// one recovered token later the same correct password mints.
#[tokio::test]
async fn credential_sign_in_locks_out_after_its_burst_and_recovers_on_the_clock() {
    let (state, clock) = credential_sign_in_state().await;
    seed_credential_user(&state, "lockout@example.com", Some(SIGN_IN_PASSWORD)).await;
    let addr = spawn_server(state.clone()).await;
    let burst = RateLimitClassV1::Auth.policy().burst;
    for attempt in 0..burst {
        assert_eq!(post_sign_in(addr, &sign_in_body("lockout@example.com", "a different password")).await.status, 401, "attempt {attempt}");
    }
    let refused = post_sign_in(addr, &sign_in_body("lockout@example.com", "a different password")).await;
    assert_eq!(refused.status, 429);
    assert!(refused.headers.to_lowercase().contains("retry-after:"), "{}", refused.headers);
    let body = json_body(&refused);
    assert_eq!(body["error"].as_str(), Some("rate-limited"));
    assert!(body["retryAfterMs"].as_u64().is_some_and(|milliseconds| milliseconds > 0));
    assert_eq!(post_sign_in(addr, &sign_in_body("lockout@example.com", SIGN_IN_PASSWORD)).await.status, 429, "a correct password is refused while the bucket is empty");
    clock.advance(i64::from(RateLimitClassV1::Auth.policy().cost_ms));
    assert_eq!(post_sign_in(addr, &sign_in_body("lockout@example.com", SIGN_IN_PASSWORD)).await.status, 200);
}

/// 🚦️ A directory-command burst beyond its bucket is refused with the typed `RateLimitRefusalV1` body (class, a positive wait,
/// the en + de notice), `retry-after` and `no-store` — never an empty 429 a client cannot read.
#[tokio::test]
async fn a_directory_command_burst_is_refused_with_a_typed_rate_limit_body() {
    let (state, _clock) = credential_sign_in_state().await;
    let addr = spawn_server(state).await;
    let burst = RateLimitClassV1::DirectoryCommand.policy().burst;
    for attempt in 0..burst {
        assert_ne!(raw_http_request(addr, "POST", "/directory/commands", &[("content-type", "application/json")], b"{}").await.status, 429, "attempt {attempt} is within the burst");
    }
    let refused = raw_http_request(addr, "POST", "/directory/commands", &[("content-type", "application/json")], b"{}").await;
    assert_eq!(refused.status, 429);
    let headers = refused.headers.to_lowercase();
    assert!(headers.contains("retry-after:") && headers.contains("no-store"), "{}", refused.headers);
    let body = json_body(&refused);
    assert_eq!(body["schema"], semio_hub_auth::rate_limit::RATE_LIMIT_REFUSAL_SCHEMA);
    assert_eq!(body["code"], "rate-limited");
    assert_eq!(body["class"], "directory-command");
    assert!(body["retryAfterMs"].as_u64().is_some_and(|milliseconds| milliseconds > 0));
    assert_eq!(body["message"], serde_json::to_value(semio_hub_auth::rate_limit::RATE_LIMIT_REFUSAL_MESSAGE).unwrap());
}

/// 🔬️ Signing out revokes the credential-minted session durably: the same token stops resolving and
/// the revocation is a fact in the log.
#[tokio::test]
async fn signing_out_a_credential_session_revokes_it_durably() {
    let (state, _clock) = credential_sign_in_state().await;
    let user_id = seed_credential_user(&state, "signout@example.com", Some(SIGN_IN_PASSWORD)).await;
    let addr = spawn_server(state.clone()).await;
    let minted = json_body(&post_sign_in(addr, &sign_in_body("signout@example.com", SIGN_IN_PASSWORD)).await);
    let authorization = format!("Bearer {}", minted["token"].as_str().expect("minted token"));
    assert_eq!(raw_http_get(addr, "/auth/sessions/me", &[("authorization", authorization.as_str())]).await.status, 200);
    assert_eq!(raw_http_request(addr, "POST", "/auth/sessions/me/sign-out", &[("authorization", authorization.as_str())], &[]).await.status, 204);
    assert_eq!(raw_http_get(addr, "/auth/sessions/me", &[("authorization", authorization.as_str())]).await.status, 401);
    assert_eq!(raw_http_request(addr, "POST", "/auth/sessions/me/sign-out", &[("authorization", authorization.as_str())], &[]).await.status, 401);
    assert_eq!(raw_http_request(addr, "DELETE", "/auth/sessions/me", &[("authorization", authorization.as_str())], &[]).await.status, 405, "sessions are revoked by command, never deleted as a resource");
    assert!(state.directory.list_auth_audit(32, 0).await.expect("auth audit").iter().any(|fact| fact.event_kind == "session-revoked" && fact.target_user_id.as_deref() == Some(user_id.as_str())));
}

/// 🔬️ A session stops resolving the moment it expires — the expiry is durable state, not a token
/// claim the hub trusts.
#[tokio::test]
async fn an_expired_session_stops_resolving() {
    let state = test_state().await;
    let user_id = seed_credential_user(&state, "expiring@example.com", Some(SIGN_IN_PASSWORD)).await;
    let issue = AuthSessionIssue {
        user_id,
        identity_provider: semio_hub_auth::CREDENTIAL_IDENTITY_PROVIDER.into(),
        identity_subject_digest: identity_subject_digest(semio_hub_auth::CREDENTIAL_IDENTITY_PROVIDER, "expiring@example.com").expect("subject digest"),
        ttl_secs: 1,
        device_instance_id: "test-device".into(),
        session_kind: AuthSessionKind::External,
        correlation_id: directory::os_identity::time_ordered_id(),
        peer_class: "browser".into(),
    };
    let session = state.directory.issue_auth_session(&issue).await.expect("short session");
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.capability.expose_once()).parse().expect("bearer"));
    assert!(get_session_me(headers.clone(), State(state.clone())).await.is_ok());
    tokio::time::sleep(std::time::Duration::from_millis(1_200)).await;
    assert_eq!(get_session_me(headers, State(state)).await.err(), Some(StatusCode::UNAUTHORIZED));
}

/// 🔬️ Authentication is not optional anywhere else either: an unauthenticated document WebSocket is
/// refused at the upgrade, a session bearer is NOT a socket grant, and the document status route
/// refuses an anonymous reader.
#[test]
fn an_unauthenticated_document_route_is_refused() {
    run_socket_test(|| async {
        let state = test_state().await;
        let session = issue_test_session(&state, "anonymous@example.com").await;
        announce_document_for_test(&state, STUDIO, "socket-unauthenticated").await;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-unauthenticated/document/ws");
        let anonymous = connect_async(url.clone().into_client_request().expect("socket request")).await.expect_err("an ungranted socket is refused");
        assert!(matches!(anonymous, tokio_tungstenite::tungstenite::Error::Http(ref response) if response.status().as_u16() == 401), "{anonymous}");
        let bearer_instead_of_grant = connect_async(socket_request(&url, &session.token)).await.expect_err("a session bearer is not a socket grant");
        assert!(matches!(bearer_instead_of_grant, tokio_tungstenite::tungstenite::Error::Http(ref response) if response.status().as_u16() == 401), "{bearer_instead_of_grant}");
        let ungranted_session = connect_async(document_socket_request(&url, &session.token)).await.expect_err("a session without a pending plan grant is refused");
        assert!(matches!(ungranted_session, tokio_tungstenite::tungstenite::Error::Http(ref response) if response.status().as_u16() == 401), "{ungranted_session}");
        let named_actor = connect_async(document_socket_request(&format!("{url}?actor=hub.v1.forged"), &session.token)).await.expect_err("a caller-named actor is refused");
        assert!(matches!(named_actor, tokio_tungstenite::tungstenite::Error::Http(ref response) if response.status().as_u16() == 400), "{named_actor}");
        assert_eq!(raw_http_get(addr, &format!("/spaces/{STUDIO}/documents/socket-unauthenticated"), &[]).await.status, 401);
    });
}

#[test]
fn a_share_holder_document_socket_is_bound_to_its_granted_actor_and_read_only() {
    run_socket_test(|| async {
        let state = test_state().await;
        announce_document_for_test(&state, STUDIO, "socket-share").await;
        let scope = DocumentScope::new(STUDIO, "socket-share");
        let share = state.directory.issue_share_token(&scope, 60, "socket-share").await.expect("share issue").capability.expose_once();
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-share".to_string())), bearer_headers(&share), State(state.clone())).await.expect("share socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-share/document/ws");
        let (mut socket, response) = connect_async(document_socket_request(&url, &share)).await.expect("share socket");
        assert_eq!(response.headers().get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL).and_then(|value| value.to_str().ok()), Some(SESSION_PROTOCOL_V1));
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("share hello");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { actor, .. } if actor == receipt.actor_id));
        let mut write = sample_envelope("share-write", &WireArtifactId("socket-share".into())).await;
        write.actor = ActorId(receipt.actor_id.clone());
        socket.send(client_binary(&ClientFrame::Commands { batch_id: 9, envelopes: vec![write] }, Lane::Command).await).await.expect("share write");
        match next_server_frame(&mut socket).await {
            ServerFrame::Ack { batch_id: 9, stages, .. } => assert!(matches!(&stages[0], AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Rejected { .. })), "a share holder is a spectator: {stages:?}"),
            other => panic!("expected share write ack, got {other:?}"),
        }
        let replay = connect_async(document_socket_request(&url, &share)).await.expect_err("a consumed share grant is not replayable");
        assert!(matches!(replay, tokio_tungstenite::tungstenite::Error::Http(ref response) if response.status().as_u16() == 401), "{replay}");
    });
}
//#endregion 🔖️CredentialSignIn

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
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/⚔️vigilant-concurrent-edit-v1/🔣️.json")).expect("vigilant fixture");
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

//#region 🔖️ProductionPosture
/// 🌐️ The cross-origin grant is a configured decision, not a reflex. Before this, every `Origin`
/// that asked got `access-control-allow-origin: <itself>` **and**
/// `access-control-allow-credentials: true`, which is the posture a naive non-loopback deploy
/// inherited unchanged. Now the default follows the bind: loopback admits loopback, a network bind
/// admits nothing, and `OS_HUB_ALLOWED_ORIGINS` names the exceptions.
#[test]
fn cross_origin_policy_admits_only_what_the_deployment_named() {
    let loopback = CrossOriginPolicyV1::LoopbackDevelopment;
    for origin in ["http://localhost:5173", "http://127.0.0.1:8787", "https://localhost", "http://[::1]:3000"] {
        assert!(loopback.admits(origin), "loopback development admits {origin}");
    }
    for origin in ["https://s.example.com", "http://evil.test", "http://localhost.evil.test", "http://10.0.0.4:5173", "null", "*"] {
        assert!(!loopback.admits(origin), "loopback development refuses {origin}");
    }

    let allowlist = CrossOriginPolicyV1::Allowlist(["https://s.example.com".to_string(), "https://admin.example.com:8443".to_string()].into());
    assert!(allowlist.admits("https://s.example.com"));
    assert!(allowlist.admits("HTTPS://S.EXAMPLE.COM"), "an origin compares ASCII-case-insensitively, whole");
    assert!(allowlist.admits("https://admin.example.com:8443"));
    for origin in ["https://s.example.com.evil.test", "http://s.example.com", "https://s.example.com:8443", "https://admin.example.com", "http://localhost:5173"] {
        assert!(!allowlist.admits(origin), "an allowlist refuses {origin}");
    }

    let closed = CrossOriginPolicyV1::Closed;
    for origin in ["http://localhost:5173", "https://s.example.com"] {
        assert!(!closed.admits(origin), "a closed policy refuses {origin}");
    }
    assert_eq!((loopback.label(), allowlist.label(), closed.label()), ("loopback-development", "allowlist", "closed"));
}

/// 🌐️ What a deployer may write in `OS_HUB_ALLOWED_ORIGINS`, and what refuses the boot instead of
/// being silently reinterpreted as a pattern.
#[test]
fn only_a_serialized_origin_is_an_allowlist_entry() {
    for origin in ["https://s.example.com", "http://localhost:5173", "https://example.com:8443", "http://[::1]:3000", "http://[2001:db8::1]"] {
        assert!(is_browser_origin(origin), "{origin} is an origin");
    }
    for rejected in ["", "s.example.com", "ftp://example.com", "https://", "https://example.com/", "https://example.com/app", "https://*.example.com", "https://example.com?x=1", "https://user@example.com", "https://example.com:port", "https://example.com:8443:1", "http://[::1", "http://[not-an-address]"] {
        assert!(!is_browser_origin(rejected), "{rejected:?} is not an origin");
    }
    assert!(is_loopback_origin("http://127.0.0.7:9999") && is_loopback_origin("https://LOCALHOST"));
    assert!(!is_loopback_origin("http://[2001:db8::1]:80") && !is_loopback_origin("http://localhost.evil.test"));
}

/// 🌐️ The same policy over a real socket: the refused origin gets a normal answer with no
/// credentialed grant, and `Vary: Origin` is present either way so a cache never hands one origin's
/// grant to the next.
#[tokio::test]
async fn a_refused_origin_receives_no_credentialed_grant_over_a_real_socket() {
    let allowed = "https://s.example.com";
    let addr = spawn_server_with_posture(test_state().await, CrossOriginPolicyV1::Allowlist([allowed.to_string()].into()), ForwardedTlsTrustV1::Untrusted).await;

    let granted = raw_http_get(addr, "/healthz", &[("Origin", allowed)]).await;
    assert_eq!(granted.status, 200);
    let granted_headers = granted.headers.to_ascii_lowercase();
    assert!(granted_headers.contains(&format!("access-control-allow-origin: {allowed}")), "{}", granted.headers);
    assert!(granted_headers.contains("access-control-allow-credentials: true"), "{}", granted.headers);
    assert!(granted_headers.contains("vary: origin"), "{}", granted.headers);

    let refused = raw_http_get(addr, "/healthz", &[("Origin", "https://evil.test")]).await;
    assert_eq!(refused.status, 200, "a refused origin still gets its answer — the browser is what enforces CORS");
    let refused_headers = refused.headers.to_ascii_lowercase();
    assert!(!refused_headers.contains("access-control-allow-origin"), "{}", refused.headers);
    assert!(!refused_headers.contains("access-control-allow-credentials"), "{}", refused.headers);
    assert!(refused_headers.contains("vary: origin"), "{}", refused.headers);

    let preflight = raw_http_request(addr, "OPTIONS", "/directory/spaces", &[("Origin", "https://evil.test"), ("Access-Control-Request-Method", "GET")], &[]).await;
    assert_eq!(preflight.status, 204);
    assert!(!preflight.headers.to_ascii_lowercase().contains("access-control-allow-origin"), "{}", preflight.headers);
}

/// 🪪️ Production mode used to be unreachable: its only identity requirement was an external
/// `IdentityAssertionVerifier`, of which this repository ships none and `main` passed a hardcoded
/// `None`, so `OS_HUB_MODE=production` died on the first check every time. The hub's *own* credential
/// authority — `os-hub credential set` seeds the first user, `POST /auth/sessions` mints the session —
/// is an identity authority, and this law pins that it now satisfies the gate on its own.
#[test]
fn a_production_hub_boots_on_its_own_credential_authority_without_any_external_idp() {
    let loopback = std::net::IpAddr::from([127, 0, 0, 1]);
    let admin = AdminSubject { provider_digest: admin_provider_digest("oidc.example"), subject_digest: identity_subject_digest("oidc.example", "admin-subject").expect("admin digest") };
    let loopback_cors = CrossOriginPolicyV1::LoopbackDevelopment;

    assert!(
        validate_auth_startup(HubMode::Production, loopback, None, None, &[admin.clone()], true, &loopback_cors, ForwardedTlsTrustV1::Untrusted).is_ok(),
        "credential sign-in is an identity authority: production boots on loopback with no verifier and no fd 3"
    );

    let refusal = validate_auth_startup(HubMode::Production, loopback, None, None, &[admin.clone()], false, &loopback_cors, ForwardedTlsTrustV1::Untrusted).expect_err("no identity authority at all");
    let HubError::UnsafeAuthConfiguration(message) = refusal else { panic!("a startup refusal is an unsafe-configuration error") };
    assert!(message.contains("OS_HUB_CREDENTIAL_SIGN_IN=true") && message.contains("IdentityAssertionVerifier"), "the refusal names both ways out: {message}");

    assert!(
        validate_auth_startup(HubMode::Production, loopback, None, None, &[], true, &loopback_cors, ForwardedTlsTrustV1::Untrusted).is_err(),
        "an identity authority does not excuse a production hub from naming its admins"
    );
}

/// 🌐️ A hub other machines can reach — outcome 2's real requirement — is now reachable, and only
/// under three explicit statements at once: production mode, a named cross-origin allowlist, and an
/// operator saying a TLS-terminating proxy is the only thing in front of the socket. Each missing
/// one is its own named refusal.
#[test]
fn a_network_bind_is_admitted_only_with_an_allowlist_and_a_declared_tls_terminating_proxy() {
    let network = std::net::IpAddr::from([10, 0, 0, 4]);
    let admin = AdminSubject { provider_digest: admin_provider_digest("oidc.example"), subject_digest: identity_subject_digest("oidc.example", "admin-subject").expect("admin digest") };
    let allowlist = CrossOriginPolicyV1::Allowlist(["https://s.example.com".to_string()].into());
    let gate = |cors: &CrossOriginPolicyV1, forwarding| validate_auth_startup(HubMode::Production, network, None, None, &[admin.clone()], true, cors, forwarding);

    let without_allowlist = gate(&CrossOriginPolicyV1::LoopbackDevelopment, ForwardedTlsTrustV1::TerminatingProxy).expect_err("a network bind with no named origins is refused");
    assert!(matches!(&without_allowlist, HubError::UnsafeAuthConfiguration(message) if message.contains("OS_HUB_ALLOWED_ORIGINS")), "{without_allowlist:?}");
    assert!(gate(&CrossOriginPolicyV1::Closed, ForwardedTlsTrustV1::TerminatingProxy).is_err(), "a closed policy admits no browser and is not an allowlist");

    let without_proxy = gate(&allowlist, ForwardedTlsTrustV1::Untrusted).expect_err("a network bind with undeclared transport is refused");
    assert!(matches!(&without_proxy, HubError::UnsafeAuthConfiguration(message) if message.contains("OS_HUB_TRUSTED_FORWARDING=proxy")), "{without_proxy:?}");

    assert!(gate(&allowlist, ForwardedTlsTrustV1::TerminatingProxy).is_ok(), "all three statements together admit a network bind");
    assert!(
        validate_auth_startup(HubMode::Development, network, None, None, &[], true, &allowlist, ForwardedTlsTrustV1::TerminatingProxy).is_err(),
        "none of this loosens development mode, which stays loopback-only"
    );
}

/// 🔐️ What the hub is entitled to believe about a request's transport. An untrusted deployment does
/// not read the forwarding headers at all — reading a header anyone can set would be strictly worse
/// than ignoring it — and a trusted one demands exactly `https`.
#[test]
fn forwarding_headers_are_read_only_where_a_proxy_was_declared() {
    let headers = |pairs: &[(&str, &str)]| {
        let mut map = HeaderMap::new();
        for (name, value) in pairs {
            map.append(axum::http::HeaderName::from_bytes(name.as_bytes()).expect("header name"), axum::http::HeaderValue::from_str(value).expect("header value"));
        }
        map
    };

    assert!(forwarded_request_is_secure(&headers(&[]), ForwardedTlsTrustV1::Untrusted));
    assert!(forwarded_request_is_secure(&headers(&[("x-forwarded-proto", "http")]), ForwardedTlsTrustV1::Untrusted), "an untrusted deployment ignores the header entirely");

    for secure in ["https", "HTTPS", " https ", "https, http"] {
        assert!(forwarded_request_is_secure(&headers(&[("x-forwarded-proto", secure)]), ForwardedTlsTrustV1::TerminatingProxy), "{secure:?}");
    }
    for insecure in ["http", "", "ws", "http, https"] {
        assert!(!forwarded_request_is_secure(&headers(&[("x-forwarded-proto", insecure)]), ForwardedTlsTrustV1::TerminatingProxy), "{insecure:?}");
    }
    assert!(!forwarded_request_is_secure(&headers(&[]), ForwardedTlsTrustV1::TerminatingProxy), "a declared proxy that says nothing has said nothing secure");

    let both = headers(&[("host", "hub.internal:8787"), ("x-forwarded-host", "s.example.com, hub.internal")]);
    assert_eq!(forwarded_external_host(&both, ForwardedTlsTrustV1::TerminatingProxy).as_deref(), Some("s.example.com"));
    assert_eq!(forwarded_external_host(&both, ForwardedTlsTrustV1::Untrusted).as_deref(), Some("hub.internal:8787"), "an untrusted deployment answers with its own Host");
    assert_eq!(forwarded_external_host(&headers(&[("host", "hub.internal")]), ForwardedTlsTrustV1::TerminatingProxy).as_deref(), Some("hub.internal"));
    assert_eq!(forwarded_external_host(&headers(&[]), ForwardedTlsTrustV1::TerminatingProxy), None);
}

/// 🛡️ The same rule over a real socket: with a proxy declared, a request the proxy reports as
/// cleartext is refused before any handler runs, and the refusal names itself.
#[tokio::test]
async fn a_proxy_reported_cleartext_request_is_refused_before_any_handler() {
    let addr = spawn_server_with_posture(test_state().await, CrossOriginPolicyV1::LoopbackDevelopment, ForwardedTlsTrustV1::TerminatingProxy).await;
    let refused = raw_http_get(addr, "/healthz", &[("X-Forwarded-Proto", "http")]).await;
    assert_eq!(refused.status, 403);
    assert!(refused.headers.to_ascii_lowercase().contains("x-semio-refusal: insecure-transport"), "{}", refused.headers);
    assert_eq!(raw_http_get(addr, "/healthz", &[]).await.status, 403, "a declared proxy that reports nothing is not a secure transport");
    assert_eq!(raw_http_get(addr, "/healthz", &[("X-Forwarded-Proto", "https")]).await.status, 200);

    let untrusted = spawn_server_with_posture(test_state().await, CrossOriginPolicyV1::LoopbackDevelopment, ForwardedTlsTrustV1::Untrusted).await;
    assert_eq!(raw_http_get(untrusted, "/healthz", &[("X-Forwarded-Proto", "http")]).await.status, 200, "a loopback hub never reads the header");
}

/// 🪪️ The whole production posture at once, over a real socket: a hub configured the way a network
/// deployment must be configured — named cross-origin allowlist, declared TLS-terminating proxy —
/// mints a real session from a password credential for a browser on an allowed origin, refuses the
/// identical request when the proxy reports cleartext, and gives an unnamed origin no grant.
#[tokio::test]
async fn a_production_posture_hub_signs_a_browser_in_over_its_declared_proxy() {
    let origin = "https://s.example.com";
    let (state, _clock) = credential_sign_in_state().await;
    let user_id = seed_credential_user(&state, "operator@example.com", Some(SIGN_IN_PASSWORD)).await;
    let addr = spawn_server_with_posture(state, CrossOriginPolicyV1::Allowlist([origin.to_string()].into()), ForwardedTlsTrustV1::TerminatingProxy).await;
    let body = sign_in_body("operator@example.com", SIGN_IN_PASSWORD);

    let minted = raw_http_request(addr, "POST", "/auth/sessions", &[("content-type", "application/json"), ("Origin", origin), ("X-Forwarded-Proto", "https"), ("X-Forwarded-Host", "s.example.com")], &body).await;
    assert_eq!(minted.status, 200, "{}", String::from_utf8_lossy(&minted.body));
    assert_eq!(json_body(&minted)["user_id"].as_str(), Some(user_id.as_str()));
    let granted = minted.headers.to_ascii_lowercase();
    assert!(granted.contains(&format!("access-control-allow-origin: {origin}")) && granted.contains("access-control-allow-credentials: true"), "{}", minted.headers);

    let cleartext = raw_http_request(addr, "POST", "/auth/sessions", &[("content-type", "application/json"), ("Origin", origin), ("X-Forwarded-Proto", "http")], &body).await;
    assert_eq!(cleartext.status, 403, "a session is never minted onto a connection the proxy says was cleartext");
    assert!(cleartext.headers.to_ascii_lowercase().contains("x-semio-refusal: insecure-transport"), "{}", cleartext.headers);

    let unnamed = raw_http_request(addr, "POST", "/auth/sessions", &[("content-type", "application/json"), ("Origin", "https://evil.test"), ("X-Forwarded-Proto", "https")], &body).await;
    assert!(!unnamed.headers.to_ascii_lowercase().contains("access-control-allow-origin"), "{}", unnamed.headers);
}

/// 🛑️ `SIGTERM` outranks `SIGINT` when both are already pending, and whichever arrives alone wins.
/// The real handler is the same composition over `tokio::signal`, so this pins the choice without
/// raising a signal at a test binary shared with every other suite in this file.
#[tokio::test]
async fn a_termination_signal_prefers_the_orchestrators_verdict() {
    assert_eq!(first_termination_signal(std::future::pending(), std::future::ready(())).await, TerminationSignalV1::Terminate);
    assert_eq!(first_termination_signal(std::future::ready(()), std::future::pending()).await, TerminationSignalV1::Interrupt);
    assert_eq!(first_termination_signal(std::future::ready(()), std::future::ready(())).await, TerminationSignalV1::Terminate);
    assert_eq!((TerminationSignalV1::Terminate.name(), TerminationSignalV1::Interrupt.name()), ("SIGTERM", "SIGINT"));
}

/// 🛑️ The whole point of item #6: a termination signal stops the listener and then the drains run,
/// instead of the process being killed where it stands. This drives the exact composition `main`
/// installs — `axum::serve(…).with_graceful_shutdown(termination future)` followed by
/// `ArtifactCreationHttpTaskOwnerV1::shutdown_with_deadline` — and asserts both halves: the socket
/// stops answering, and an in-flight artifact-creation task is drained rather than abandoned.
#[tokio::test]
async fn a_termination_signal_stops_the_listener_and_then_drains_in_flight_work() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.expect("test listener");
    let addr = listener.local_addr().expect("listener address");
    let app = router(test_state().await, CrossOriginPolicyV1::LoopbackDevelopment, ForwardedTlsTrustV1::Untrusted).into_make_service_with_connect_info::<SocketAddr>();
    let (terminate, terminated) = tokio::sync::oneshot::channel::<()>();
    let served = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let signal = first_termination_signal(std::future::pending(), async {
                    let _ = terminated.await;
                })
                .await;
                assert_eq!(signal, TerminationSignalV1::Terminate);
            })
            .await
            .expect("graceful serve");
    });
    assert_eq!(raw_http_get(addr, "/healthz", &[]).await.status, 200, "the hub answers before the signal");

    let owner = Arc::new(ArtifactCreationHttpTaskOwnerV1::new());
    let reservation = match owner.reserve("user\0space\0termination".into(), Arc::new(ArtifactCreationHttpControlV1::new())) {
        ArtifactCreationHttpAdmissionV1::Owner(reservation) => reservation,
        _ => panic!("the first exact admission owns its reservation"),
    };
    let drained = Arc::new(std::sync::atomic::AtomicBool::new(false));
    reservation.activate({
        let drained = drained.clone();
        async move {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            drained.store(true, std::sync::atomic::Ordering::Release);
        }
    });

    terminate.send(()).expect("send the termination signal");
    tokio::time::timeout(std::time::Duration::from_secs(10), served).await.expect("the listener stops within the deadline").expect("served task");
    owner.shutdown_with_deadline(std::time::Duration::from_secs(5)).await;

    assert!(drained.load(std::sync::atomic::Ordering::Acquire), "the post-serve drain runs in-flight work to completion instead of abandoning it");
    assert_eq!(owner.task_count(), 0, "the drain leaves no task behind");
    assert!(tokio::net::TcpStream::connect(addr).await.is_err(), "the stopped listener refuses new connections");
}
//#endregion 🔖️ProductionPosture

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

//#region 🗄️InstanceStores
/// ⚖️ LAW: the session hub records in the server-product instance is still there after the process
/// that wrote it is gone. This is what makes `HubInstance`'s stores a durable role rather than a
/// per-process cache, and it is the property the auth move depends on.
#[tokio::test]
async fn instance_sessions_survive_a_hub_restart() {
    let dir = tempdir("instance-restart");
    let record = SessionRecord {
        id: SessionId("session-durable".to_string()),
        principal: Principal::User { id: "user-ada".to_string() },
        device: Some(server::contract::DeviceId("device-1".to_string())),
        issued_at_millis: 1_700_000_000_000,
    };

    let first = instance_state(&dir).await.expect("open hub instance stores");
    first.sessions.lock().await.create(record.clone()).await.expect("record the session");
    assert!(first.sessions.lock().await.get(&SessionId("session-durable".to_string())).await.is_some());
    drop(first);

    let second = instance_state(&dir).await.expect("reopen hub instance stores");
    let reopened = second.sessions.lock().await.get(&SessionId("session-durable".to_string())).await.expect("the session survived the restart");
    assert_eq!(reopened, record);

    second.sessions.lock().await.delete(&SessionId("session-durable".to_string())).await.expect("forget the session");
    drop(second);
    let third = instance_state(&dir).await.expect("reopen after the deletion");
    assert!(third.sessions.lock().await.get(&SessionId("session-durable".to_string())).await.is_none(), "a deleted session leaves no replayable trace");
}

/// ⚖️ LAW: signing out everywhere on the directory clears the instance's live set for that principal
/// too, so the two halves of one session fact can never disagree after a credential change.
#[tokio::test]
async fn revoking_a_principal_clears_every_instance_session_it_holds() {
    let dir = tempdir("instance-revoke");
    let instance = instance_state(&dir).await.expect("open hub instance stores");
    for index in 0..3 {
        let record = SessionRecord {
            id: SessionId(format!("session-{index}")),
            principal: Principal::User { id: "user-ada".to_string() },
            device: None,
            issued_at_millis: 1_700_000_000_000 + index,
        };
        instance.sessions.lock().await.create(record).await.expect("record the session");
    }
    let other = SessionRecord { id: SessionId("session-other".to_string()), principal: Principal::User { id: "user-grace".to_string() }, device: None, issued_at_millis: 1_700_000_000_009 };
    instance.sessions.lock().await.create(other).await.expect("record the other principal's session");

    assert_eq!(instance.sessions.lock().await.revoke_principal(&Principal::User { id: "user-ada".to_string() }).await.expect("revoke"), 3);
    assert!(instance.sessions.lock().await.get(&SessionId("session-0".to_string())).await.is_none());
    assert!(instance.sessions.lock().await.get(&SessionId("session-other".to_string())).await.is_some(), "another principal's session is untouched");
}

/// ⚖️ LAW: the saga drain is silent when it moves nothing. A cadence that reported every empty tick
/// would emit two records a second for ever and bury the ones that matter.
#[tokio::test]
async fn a_drain_over_an_empty_outbox_moves_nothing_and_says_nothing() {
    let dir = tempdir("saga-drain");
    let instance = instance_state(&dir).await.expect("open hub instance stores");
    let (tracer, sink) = Tracer::capturing(TraceLevel::Debug);
    assert_eq!(drain_instance_sagas(&instance, &tracer).await, 0);
    assert_eq!(drain_instance_sagas(&instance, &tracer).await, 0);
    assert!(sink.records_for("server.saga.drain").is_empty(), "an empty drain is not news: {:?}", sink.records());
}

/// ⚖️ LAW: the supervisor stops on demand and takes one last pass on the way out — the requests that
/// committed after the final tick are exactly the ones an unfinished drain would strand.
#[tokio::test]
async fn the_saga_drain_supervisor_stops_and_drains_once_more_on_shutdown() {
    let dir = tempdir("saga-drain-shutdown");
    let instance = instance_state(&dir).await.expect("open hub instance stores");
    let (tracer, sink) = Tracer::capturing(TraceLevel::Debug);
    let supervisor = SagaDrainSupervisor::start(instance, tracer, std::time::Duration::from_millis(5));
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    supervisor.shutdown().await;
    assert!(sink.records_for("server.saga.drain").is_empty(), "nothing was queued, so nothing is reported");
}
//#endregion 🗄️InstanceStores

/// 🧭️ Every frontier the hub puts on the wire names the DOCUMENT the client opened, never
/// `db_artifact_id`'s internal `v1:<len>:<len>:<space><document>` key.
///
/// The db engine keys documents by that composite — the db and fanout catalogs are flat and a bare
/// document id is not unique across spaces — and stamps it into every `Frontier` it returns, which
/// the replication wire carried through unchanged into `Welcome`, `Ack` and `Commands`. It stayed
/// invisible only because the client assigned the welcome frontier without checking it; C7 §1.4
/// measured `server_frontier.document_id = v1:36:41:01a0c314-…artifact-0954e2…` against a store
/// worker (`validateArtifactBootstrapIdentity`) that refuses exactly that pair. The law pins the
/// whole round trip: both of a `Welcome`'s bootstrap frontiers as well as its own, the two frames a
/// live session repeats it on, an untouched frame with no frontier, and the ingress direction —
/// a client frontier is re-keyed onto the db id, and one naming another document is REFUSED rather
/// than quietly rewritten into the identity the hub wanted to see.
#[test]
fn every_wire_frontier_the_hub_sends_names_the_document_not_its_internal_db_key() {
    let scope = DocumentScope::new("01a0c314-e41f-780d-a980-3adda40ca9f7", "artifact-0954e2d10d8fff9605f101b0dba34f3b");
    let db_id = db_artifact_id(&scope);
    assert_ne!(db_id.0, scope.document_id, "the internal key is deliberately not the document id");
    let internal = |ordinal: u64| RuntimeFrontierSummary { document_id: db_id.clone(), head_edit_ordinal: ordinal, head_edit_id: format!("edit:{ordinal}"), last_commit_seq: ordinal, chain_hash: [ordinal.to_le_bytes()[0]; 32] };
    let named = |frontier: &RuntimeFrontierSummary| frontier.document_id.0.clone();

    let mut welcome = ServerFrame::Welcome {
        session_id: "session".into(),
        resume_token: "resume".into(),
        server_frontier: internal(9),
        bootstrap: Bootstrap::ArtifactBootstrap(Box::new(protocol::ArtifactBootstrap {
            format_version: protocol::ARTIFACT_BOOTSTRAP_FORMAT_VERSION,
            descriptor_hash: [1; 32],
            artifact_schema: "s.gis.gismap@1/*".into(),
            artifact_kind: "gis.map".into(),
            pack_schema_hash: [2; 32],
            baseline_frontier: internal(0),
            pack_hash: [3; 32],
            spr_hash: [4; 32],
            pack_length: 81_038,
            spr_length: 237,
            chunk_count: 1,
            aggregate_hash: [5; 32],
            required_tail_frontier: internal(9),
            inline: None,
        })),
    };
    project_server_frame(&mut welcome, &scope.document_id);
    let ServerFrame::Welcome { server_frontier, bootstrap, .. } = &welcome else { panic!("welcome") };
    assert_eq!(named(server_frontier), scope.document_id);
    let Bootstrap::ArtifactBootstrap(artifact) = bootstrap else { panic!("artifact bootstrap") };
    assert_eq!(named(&artifact.baseline_frontier), scope.document_id);
    assert_eq!(named(&artifact.required_tail_frontier), scope.document_id);
    assert_eq!(server_frontier.head_edit_ordinal, 9, "only the identity is projected; the position is untouched");
    assert_eq!(server_frontier.chain_hash, [9u8; 32]);

    let mut ack = ServerFrame::Ack { batch_id: 1, stages: vec![AckStage::Received], frontier: internal(10) };
    project_server_frame(&mut ack, &scope.document_id);
    let ServerFrame::Ack { frontier, .. } = &ack else { panic!("ack") };
    assert_eq!(named(frontier), scope.document_id);

    let mut commands = ServerFrame::Commands { envelopes: Vec::new(), origin: ActorId("actor".into()), frontier: internal(11) };
    project_server_frame(&mut commands, &scope.document_id);
    let ServerFrame::Commands { frontier, .. } = &commands else { panic!("commands") };
    assert_eq!(named(frontier), scope.document_id);

    assert!(frame_carries_frontier(&welcome) && frame_carries_frontier(&ack) && frame_carries_frontier(&commands));
    let mut session = ServerFrame::Session { actor: "actor".into(), color: 3 };
    let untouched = session.clone();
    assert!(!frame_carries_frontier(&session));
    project_server_frame(&mut session, &scope.document_id);
    assert_eq!(session, untouched);

    let mut advertised = RuntimeFrontierSummary { document_id: WireArtifactId(scope.document_id.clone()), head_edit_ordinal: 4, head_edit_id: "edit:4".into(), last_commit_seq: 4, chain_hash: [4; 32] };
    assert!(wire_frontier_to_db(&mut advertised, &scope.document_id, &db_id));
    assert_eq!(advertised.document_id, db_id);
    assert_eq!(advertised.head_edit_ordinal, 4);

    let mut foreign = RuntimeFrontierSummary { document_id: WireArtifactId("artifact-somebody-elses".into()), head_edit_ordinal: 4, head_edit_id: "edit:4".into(), last_commit_seq: 4, chain_hash: [4; 32] };
    assert!(!wire_frontier_to_db(&mut foreign, &scope.document_id, &db_id));
    assert_eq!(foreign.document_id.0, "artifact-somebody-elses", "a refused frontier is left exactly as the client sent it");
}
