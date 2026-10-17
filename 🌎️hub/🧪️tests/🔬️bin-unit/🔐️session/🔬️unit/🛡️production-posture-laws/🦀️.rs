/// 🏭️ Laws of the production posture: origins, proxies, forwarding headers, credential authority and shutdown.

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
