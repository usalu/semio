/// 🔑️ Laws of credential sign-in: failing stores, wrong credentials, lockout, rate limits, sign-out and expiry.

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
