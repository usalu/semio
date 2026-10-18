/// 🔑️ Startup auth and readiness laws of the hub boot: the startup identity verifier and local bootstrap transport, the fail closed startup auth policy, the redacted readiness and liveness bodies, the booting hub catalog progress and the inference service readiness.

struct StartupVerifier;

impl IdentityAssertionVerifier for StartupVerifier {
    fn verify<'a>(&'a self, _assertion: &'a semio_hub_directory_contract::IdentityAssertion, _context: &'a semio_hub_directory_contract::IdentityVerificationContext<'a>) -> semio_hub_directory_contract::IdentityVerificationFuture<'a> {
        Box::pin(async { Err(DirectoryError::Unauthorized) })
    }
}

struct TestLocalBootstrap;

impl LocalBootstrapTransport for TestLocalBootstrap {
    fn run_id(&self) -> &str {
        "00112233445566778899aabbccddeeff"
    }

    fn is_ready(&self) -> bool {
        true
    }

    fn request_cancelled(&self, _request_id: &str) -> bool {
        false
    }

    fn accept<'a>(&'a self, _control: &'a dyn IdentityVerificationControl) -> semio_hub_directory_contract::LocalBootstrapAcceptFuture<'a> {
        Box::pin(async { Ok(None) })
    }

    fn issue<'a>(
        &'a self,
        _request: &'a semio_hub_directory_contract::VerifiedLocalBootstrapRequest,
        _session: &'a semio_hub_directory_contract::model::IssuedAuthSession,
        _context: &'a semio_hub_directory_contract::IdentityVerificationContext<'a>,
    ) -> semio_hub_directory_contract::LocalBootstrapIssueFuture<'a> {
        Box::pin(async { Ok(()) })
    }

    fn reject<'a>(&'a self, _request_id: &'a str, _code: semio_hub_directory_contract::LocalBootstrapRejectCode, _context: &'a semio_hub_directory_contract::IdentityVerificationContext<'a>) -> semio_hub_directory_contract::LocalBootstrapTerminalFuture<'a> {
        Box::pin(async { Ok(()) })
    }

    fn cancel<'a>(&'a self, _request_id: &'a str) -> semio_hub_directory_contract::LocalBootstrapTerminalFuture<'a> {
        Box::pin(async { Ok(()) })
    }

    fn shutdown<'a>(&'a self) -> semio_hub_directory_contract::LocalBootstrapTerminalFuture<'a> {
        Box::pin(async { Ok(()) })
    }

    fn closed<'a>(&'a self) -> semio_hub_directory_contract::LocalBootstrapTerminalFuture<'a> {
        Box::pin(std::future::pending())
    }
}

#[test]
fn startup_auth_policy_fails_closed_without_owned_adapters() {
    let loopback = std::net::IpAddr::from([127, 0, 0, 1]);
    let public = std::net::IpAddr::from([0, 0, 0, 0]);
    let verifier: Arc<dyn IdentityAssertionVerifier> = Arc::new(StartupVerifier);
    let local: Arc<dyn LocalBootstrapTransport> = Arc::new(TestLocalBootstrap);
    let admin = AdminSubject { provider_digest: admin_provider_digest("oidc.example"), subject_digest: identity_subject_digest("oidc.example", "admin-subject").expect("admin digest") };
    let loopback_cors = CrossOriginPolicyV1::LoopbackDevelopment;
    let no_forwarding = ForwardedTlsTrustV1::Untrusted;
    assert!(validate_auth_startup(HubMode::Production, loopback, None, None, &[admin.clone()], false, &loopback_cors, no_forwarding).is_err());
    assert!(validate_auth_startup(HubMode::Production, loopback, Some(&verifier), None, &[], false, &loopback_cors, no_forwarding).is_err());
    assert!(validate_auth_startup(HubMode::Production, public, Some(&verifier), None, &[admin.clone()], false, &loopback_cors, no_forwarding).is_err());
    assert!(validate_auth_startup(HubMode::Production, loopback, Some(&verifier), None, &[admin], false, &loopback_cors, no_forwarding).is_ok());
    assert!(validate_auth_startup(HubMode::Development, loopback, None, None, &[], false, &loopback_cors, no_forwarding).is_err());
    assert!(validate_auth_startup(HubMode::Development, public, None, Some(&local), &[], false, &loopback_cors, no_forwarding).is_err());
    assert!(validate_auth_startup(HubMode::Development, loopback, None, Some(&local), &[], false, &loopback_cors, no_forwarding).is_ok());
}

#[test]
fn readiness_v1_is_redacted_and_never_claims_public_session_issuance() {
    let ready = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root");
    let encoded = serde_json::to_string(&ready).expect("readiness json");
    assert_eq!(ready.status, "ready");
    assert!(!ready.authentication.public_session_issuance);
    assert!(ready.artifact_authority.ready);
    assert!(!ready.features.open_plan);
    assert!(!ready.features.open_plan_exchange);
    assert!(!encoded.contains("session.v1"));
    assert!(!encoded.contains("subject"));
    assert!(!encoded.contains("channel"));
    assert!(!encoded.contains("sessionKind"));
    assert!(!encoded.contains("authorizationGeneration"));
    let partial = hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, false, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root");
    assert_eq!(partial.status, "not-ready");
    assert!(partial.authentication.bootstrap_ready);
    assert!(!partial.artifact_authority.ready);
    assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), false, false, false, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root").status, "not-ready");
    assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, false, true, false, true, false, false, "trusted-catalog-never-published-in-this-data-root").status, "not-ready");
    assert_eq!(hub_readiness(HubMode::Development, "network", ready.run_id, true, true, false, true, true, false, false, false, "trusted-catalog-never-published-in-this-data-root").status, "not-ready");
}

/// 🧬️ Every readiness body a hub serves — ready, blocked, production on a network bind, and booting
/// with and without startup progress — is exactly the declared `LocalBootstrapReadinessV1`, and a
/// ready body can never carry startup progress.
#[test]
fn every_served_readiness_body_is_the_declared_readiness_schema() {
    let mut document: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🚀️local-bootstrap/🧬️schema/🔣️.json")).expect("local-bootstrap schema module");
    document["$ref"] = serde_json::Value::String("#/$defs/LocalBootstrapReadinessV1".into());
    let schema = semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(&document.to_string(), &[semio_hub_trusted_catalog_contract::schema::TRUSTED_CATALOG_SCHEMA_JSON]).expect("readiness schema compiles");
    let run = "00112233445566778899aabbccddeeff".to_string();
    let reason = "trusted-catalog-never-published-in-this-data-root";
    let starting = hub_starting_readiness(HubMode::Production, "network", "production".into(), true, true, true);
    let progressed = starting.clone().with_startup(Some(HubStartupProgressV1 { stage: AuthorityProgressStage::GuestCodecExecuting.code(), completed_units: 25_000_000, total_units: 4_000_000_000, catalog: None }));
    let cataloged = {
        let cell = StartupProgressCellV1::default();
        cell.observe(AuthorityProgress { stage: AuthorityProgressStage::CatalogLoading, completed_units: 3, total_units: 37 });
        let package = |plugin_id: &str, phase| semio_hub_trusted_catalog_contract::schema::TrustedCatalogPackageProgressV1 { plugin_id: plugin_id.into(), component_bytes: 14_932_128, phase, rows: 2, rows_pinned: 1, rows_verified: 0 };
        let catalog = semio_hub_trusted_catalog_contract::schema::TrustedCatalogLoadProgressV1 {
            packages: vec![package("note", semio_hub_trusted_catalog_contract::schema::TrustedCatalogPackagePhaseV1::Staged), package("draw", semio_hub_trusted_catalog_contract::schema::TrustedCatalogPackagePhaseV1::Reading)],
            packages_total: 2,
            packages_ready: 0,
            packages_refused: 0,
            component_bytes_total: 29_864_256,
            component_bytes_read: 14_932_128,
            rows_total: 4,
            rows_pinned: 2,
            rows_verified: 0,
        };
        starting.clone().with_startup(Some(HubStartupProgressV1 { catalog: Some(catalog), ..cell.latest().expect("observed progress") }))
    };
    let bodies = [
        ("ready", hub_readiness(HubMode::Development, "loopback", run.clone(), true, true, true, true, true, true, false, true, reason)),
        ("blocked", hub_readiness(HubMode::Development, "loopback", run.clone(), false, false, false, false, false, false, false, false, reason)),
        ("production", declare_public_session_issuance(hub_readiness(HubMode::Production, "network", "production".into(), true, true, true, true, true, true, true, false, reason), true)),
        ("starting", starting),
        ("starting-with-progress", progressed.clone()),
        ("starting-with-catalog-progress", cataloged),
        ("starting-development", hub_starting_readiness(HubMode::Development, "loopback", run.clone(), false, false, false)),
    ];
    for (label, body) in &bodies {
        let encoded = serde_json::to_string(body).expect("readiness json");
        assert!(schema.is_valid_json(&encoded), "{label}: {encoded}");
    }
    assert_eq!(progressed.status, "not-ready");
    assert!(progressed.blocked_by.iter().any(|closed| closed.gate == "artifactAuthority" && closed.reason == "trusted-catalog-loading"));
    let ready_with_startup = bodies[0].1.clone().with_startup(progressed.startup);
    assert!(!schema.is_valid_json(&serde_json::to_string(&ready_with_startup).expect("readiness json")), "a ready hub never reports startup progress");
}

/// 💓️ The liveness body the boot server and the full router both answer (`hub_liveness`) is exactly the declared
/// `LocalBootstrapLivenessV1` for a development run and for production's fixed run id, and it never claims readiness.
#[test]
fn the_served_liveness_body_is_the_declared_liveness_schema() {
    let mut document: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🚀️local-bootstrap/🧬️schema/🔣️.json")).expect("local-bootstrap schema module");
    document["$ref"] = serde_json::Value::String("#/$defs/LocalBootstrapLivenessV1".into());
    let schema = semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(&document.to_string(), &[semio_hub_trusted_catalog_contract::schema::TRUSTED_CATALOG_SCHEMA_JSON]).expect("liveness schema compiles");
    for run in ["00112233445566778899aabbccddeeff", "production"] {
        let encoded = serde_json::to_string(&hub_liveness(run)).expect("liveness json");
        assert!(schema.is_valid_json(&encoded), "{encoded}");
    }
    let mut claimed = serde_json::to_value(hub_liveness("production")).expect("liveness value");
    claimed["status"] = serde_json::Value::String("ready".into());
    assert!(!schema.is_valid_json(&claimed.to_string()), "a liveness body says live and nothing else");
}

/// 🌅️ A booting hub answers on its socket before its stores and catalog are up: `/healthz` live,
/// `/readyz` `503 not-ready` with the catalog load's latest progress, every other route a signed
/// `503` with `Retry-After`; then it hands the same listener to the full router, which answers on it
/// without the socket ever being rebound.
#[test]
fn a_booting_hub_answers_readiness_with_catalog_progress_then_hands_its_socket_over() {
    run_socket_test(|| async {
        let bound = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
        bound.set_nonblocking(true).expect("non-blocking");
        let addr = bound.local_addr().expect("address");
        let progress = StartupProgressCellV1::default();
        let readiness = hub_starting_readiness(HubMode::Production, "loopback", "production".into(), true, true, false);
        let boot = BootReadinessServerV1::start(tokio::net::TcpListener::from_std(bound.try_clone().expect("clone")).expect("tokio listener"), readiness, progress.clone(), Tracer::disabled(), CrossOriginPolicyV1::LoopbackDevelopment, ForwardedTlsTrustV1::Untrusted);
        let first = raw_http_get(addr, "/readyz", &[]).await;
        assert_eq!(first.status, 503);
        let body: serde_json::Value = serde_json::from_slice(&first.body).expect("readiness json");
        assert_eq!(body["status"], "not-ready");
        assert_eq!(body["artifactAuthority"]["reason"], "trusted-catalog-loading");
        assert!(body.get("startup").is_none(), "no progress reported yet");
        progress.observe(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: 50_000_000, total_units: 4_000_000_000 });
        let second: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, "/readyz", &[]).await.body).expect("readiness json");
        assert_eq!(second["startup"], serde_json::json!({ "stage": "guest-codec-executing", "completedUnits": 50_000_000u64, "totalUnits": 4_000_000_000u64 }));
        let live = raw_http_get(addr, "/healthz", &[]).await;
        assert_eq!(live.status, 200);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&live.body).expect("liveness json")["status"], "live");
        let refused = raw_http_get(addr, "/directory/spaces", &[]).await;
        assert_eq!(refused.status, 503);
        let head = refused.headers.to_ascii_lowercase();
        assert!(head.contains(&format!("{}: unavailable", semio_hub_refusal::HUB_REFUSAL_HEADER)) && head.contains("retry-after: 5"), "{head}");
        boot.hand_over().await;
        let listener = tokio::net::TcpListener::from_std(bound).expect("tokio listener");
        tokio::spawn(async move {
            let app = Router::new().route("/readyz", get(|| async { "served by the full router" }));
            let _ = axum::serve(listener, app).await;
        });
        let handed = raw_http_get(addr, "/readyz", &[]).await;
        assert_eq!((handed.status, handed.body.as_slice()), (200, b"served by the full router".as_slice()), "the full router answers on the very socket the booting hub bound");
    });
}

/// 💡️ A hub with a frozen inference binding publishes the service it executes AND the route family
/// that serves it — the route a client builds its job paths from must be one this router registers.
#[test]
fn readiness_publishes_each_hub_executed_inference_service_with_its_served_route() {
    let bound = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, true, false, true, "trusted-catalog-never-published-in-this-data-root");
    let encoded: serde_json::Value = serde_json::to_value(&bound).expect("readiness json");
    assert_eq!(encoded["features"]["inferenceServices"], serde_json::json!([{ "serviceId": "s.gis.gismap.inference", "route": "inference/gis-map" }]));
    let router = semio_hub_testing::hub_source("🏗️bootstrap/🛣️routes/🦀️.rs");
    assert!(router.contains(&format!("\"/spaces/{{space_id}}/documents/{{document_id}}/{}/jobs\"", semio_hub_inference::schema::GIS_SERVICE_ROUTE)), "the published route family is the one the router serves");
    let unbound = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root");
    assert_eq!(serde_json::to_value(&unbound).expect("readiness json")["features"]["inferenceServices"], serde_json::json!([]));
}
