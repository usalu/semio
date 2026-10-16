
use semio_hub_directory_contract::{AUTH_TEXT_MAX_BYTES, HubCapability, IdentityAssertionVerifier, IdentityVerificationControl, LocalBootstrapTransport, SessionCapability, identity_subject_digest};
use semio_hub_ledgers::guard::{AdminSubject, AuthOutcome, admin_provider_digest, validate_auth_startup};
use std::sync::Arc;
use semio_hub_artifact_authority::{ArtifactBlobIntegrity, ArtifactPair, AuthorityError, AuthorityLimits, AuthorityProgress, AuthorityProgressStage, OperationContext, checkpoint_id_encoding_v1};
use semio_hub_artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStores, MemoryArtifactChunkCasStorage, artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1};
use semio_hub_artifact_creation::ArtifactCreationActorV1;
use semio_hub_route_models::artifact_creation::{ArtifactCreationHttpAdmissionV1, ArtifactCreationHttpControlV1, ArtifactCreationHttpTaskOwnerV1};
use semio_hub_runtime::readiness::{BootReadinessServerV1, HubMode, HubReadinessV1, HubStartupProgressV1, declare_public_session_issuance, hub_liveness, hub_readiness, hub_starting_readiness, startup_readiness_line};
use semio_hub_lag_rebootstrap::{CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE, RebootstrapProgressStage, VerifiedRebootstrapSource, decode_canonical_checkpoint_pair};
use semio_hub_ledgers::pair_blobs::{CANONICAL_PAIR_PROGRESS_STAGES, canonical_pair_progress_index, canonical_pair_request_admission};
use semio_hub_runtime::transport::{CrossOriginPolicyV1, ForwardedTlsTrustV1, refusal_middleware};
use semio_hub_route_models::document_socket::{DOCUMENT_SOCKET_COMMAND_REFILL_PER_SECOND, admit_writes, document_socket_gate, encode_messages, messages_for_error, plan_seeded_hello_frontier, transient_apply_refusal_messages, undeclared_batch_refusal, wire_rebootstrap};
use semio_hub_directory_contract::error::DirectoryError;
use semio_hub_route_models::directory_events::{DirectoryEventPageSource, load_all_directory_events};
use semio_hub_route_models::directory_commands::{DirectorySpaceAccessDecisionV1, directory_space_access_decision};
use semio_hub_route_testkit::{DocumentOpenPlanLedgerFixture, RawHttpResponse, STUDIO, TestIssuedSession, announce_document_for_test, artifact_document_id_for_test, create_space_for_test, document_open_plan_test_authority, hostile_http_request, issue_test_session, json_body, lag_test_state, native_openable_stdio_bundle, publish_checkpoint_for_test, publish_genesis_checkpoint_for_test, raw_http_get, raw_http_request, run_socket_test, sample_envelope, seed_genesis_document_for_test, spawn_server, tempdir, test_state, upsert_member_for_test};
use directory::os_directory::DocumentScope;
use semio_hub_route_models::check_in::{GisMapApprovalCheckpointPublisherV1Impl, publish_gis_map_checkpoint_change};
use semio_hub_gis::inference::GisMapInferenceBindingV1;
use semio_hub_inference_runtime::HubInferenceRuntimeV1;
use semio_hub_state::HubState;
use semio_hub_inference_runtime_contract::{InferenceRouteErrorV1, UnavailableGisMapApprovalCommitterV1};
use semio_hub_native_openable::NativeCodecProviderSetV1;
use semio_framework_hash::Sha256;
use semio_framework_async::ShardedMap;
use std::net::SocketAddr;
use semio_hub_ledgers::socket_grants::SocketBindingKeyV1;
use semio_hub_runtime::startup::{StartupArtifactAuthority, StartupCancellationV1, StartupCatalogControl, StartupProgressCellV1, artifact_authority_closed_reason, configured_artifact_authority};
use semio_framework_trace::record::{TraceLevel, TraceOutcome, Tracer};
use semio_hub_trusted_catalog::TrustedCatalogLoader;
use protocol::ArtifactId as WireArtifactId;
use semio_hub_document_routes::inference::{acquire_gis_map_approval_ingress, revalidate_gis_map_approval_authority};
use semio_hub_runtime::presence::{db_artifact_id, document_scope_key_v1};
use semio_framework_os_kernel as directory_access_permits;
use semio_hub_runtime::support::now_ms;
use semio_hub_runtime::readiness;
#[cfg(test)]
use protocol::{ActorId, ApplyOutcome, MutationEnvelope, RuntimeFrontierSummary, ServerFrame};
#[cfg(test)]
use directory::os_directory::{ArtifactFrontier, ArtifactHash, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryEvent, DirectorySpaceRole, DirectorySpaceVisibility, PublishedArtifactCheckpoint};
#[cfg(test)]
use std::collections::BTreeMap;
#[cfg(test)]
use directory::os_directory::access_policy::DirectoryAccessActionV1;
#[cfg(test)]
use directory::{DslValue, FromValue, os_directory};
#[cfg(test)]
use axum::http::{HeaderMap, StatusCode};
#[cfg(test)]
use std::sync::Mutex;
#[cfg(test)]
use axum::Router;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_ledgers::pair_blobs::TestCanonicalPairRequestGate;
#[cfg(test)]
use tokio::sync::broadcast;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::execute_directory_command_fenced;
#[cfg(test)]
use axum::routing::get;
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
    let progressed = HubReadinessV1 { startup: Some(HubStartupProgressV1 { stage: AuthorityProgressStage::GuestCodecExecuting.code(), completed_units: 25_000_000, total_units: 4_000_000_000, catalog: None }), ..starting.clone() };
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
        HubReadinessV1 { startup: Some(HubStartupProgressV1 { catalog: Some(catalog), ..cell.latest().expect("observed progress") }), ..starting.clone() }
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
    let ready_with_startup = HubReadinessV1 { startup: progressed.startup, ..bodies[0].1.clone() };
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

/// ⏳️ A trusted-catalog load that STALLS is a CLOSED READINESS GATE with a reason of its own, never
/// an abort and never a claim that the catalog is unloadable. Until ticket 26/09/18 slice M9 the
/// bound was an `Err` propagated out of the boot function, so a hub on a busy machine EXITED
/// (measured twice on a 612 MB warm catalog, M8 §2.1) and looked to an operator exactly like a
/// corrupt data root; slice HT16 then replaced the total wall budget behind it with a no-progress
/// span, so "busy" and "stopped" are no longer the same fact. The law pins both halves: the outcome
/// carries no authority, and the reason it publishes is distinguishable from every other closed
/// reason — in particular it never reads as a calendar overrun.
#[test]
fn a_stalled_trusted_catalog_load_closes_the_gate_and_never_claims_the_catalog_is_unloadable() {
    assert!(StartupArtifactAuthority::Stalled.is_none());
    assert!(StartupArtifactAuthority::Stalled.configured().is_none());
    assert!(StartupArtifactAuthority::Absent.is_none());
    assert_eq!(artifact_authority_closed_reason(true, true, true), "trusted-catalog-load-stalled-before-it-finished");
    assert_eq!(artifact_authority_closed_reason(true, true, false), "trusted-catalog-load-stalled-before-it-finished");
    assert_eq!(artifact_authority_closed_reason(true, false, true), "trusted-catalog-pointer-present-but-not-loadable");
    assert_eq!(artifact_authority_closed_reason(true, false, false), "trusted-catalog-never-published-in-this-data-root");
    assert_eq!(artifact_authority_closed_reason(false, true, true), "native-artifact-execution-feature-not-compiled");
    let reasons = [
        artifact_authority_closed_reason(true, true, true),
        artifact_authority_closed_reason(true, false, true),
        artifact_authority_closed_reason(true, false, false),
        artifact_authority_closed_reason(false, false, false),
    ];
    let mut distinct = reasons.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    assert_eq!(distinct.len(), reasons.len(), "every closed artifactAuthority reason must name a different fact");
    let blocked = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, false, true, true, true, false, false, artifact_authority_closed_reason(true, true, false));
    assert_eq!(blocked.status, "not-ready");
    assert!(blocked.blocked_by.iter().any(|gate| gate.gate == "artifactAuthority" && gate.reason == "trusted-catalog-load-stalled-before-it-finished"));
}

/// 🤖️ `features.mcpWorkspace` answers whether a `semio-os-mcp --hub … --credential-file …` gateway
/// can be served by this hub, and it is derived from both halves that gateway needs — the
/// delegation exchange and a resolvable document open target. It read a hard-coded `false` from its
/// introduction until ticket 26/09/18 slice M8, so the law pins all four corners: a constant of
/// either polarity fails at least two of them.
#[test]
fn mcp_workspace_readiness_is_derived_from_delegation_and_open_plan_never_constant() {
    let readiness = |agent_delegation_ready: bool, open_plan_ready: bool| {
        hub_readiness(
            HubMode::Development,
            "loopback",
            "00112233445566778899aabbccddeeff".into(),
            true,
            true,
            open_plan_ready,
            agent_delegation_ready,
            true,
            true,
            false,
            false,
            "trusted-catalog-never-published-in-this-data-root",
        )
    };
    assert!(readiness(true, true).features.mcp_workspace, "a hub that can exchange a delegation AND resolve an open target serves an MCP workspace");
    assert!(!readiness(true, false).features.mcp_workspace, "a delegation with no openable document is not an MCP workspace");
    assert!(!readiness(false, true).features.mcp_workspace, "an openable document an agent can never authenticate against is not an MCP workspace");
    assert!(!readiness(false, false).features.mcp_workspace);
    assert_eq!(readiness(true, true).features.mcp_workspace, readiness(true, true).features.open_plan, "with delegation available the field tracks the open-plan gate exactly");
    let encoded = serde_json::to_string(&readiness(true, true)).expect("readiness json");
    assert!(encoded.contains("\"mcpWorkspace\":true"), "the served body carries the derived value: {encoded}");
}

#[test]
fn a_not_ready_hub_names_every_closed_gate_and_its_reason_in_readyz_and_at_startup() {
    let addr: SocketAddr = "127.0.0.1:8787".parse().expect("loopback readiness address");
    let ready = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root");
    let encoded = serde_json::to_string(&ready).expect("ready readiness json");
    assert!(ready.blocked_by.is_empty());
    assert!(!encoded.contains("blockedBy"), "a ready hub publishes no closed-gate list");
    assert!(!encoded.contains("reason"), "an open gate carries no reason");
    assert_eq!(startup_readiness_line(&ready, &addr), "[INFO] os-hub ready at http://127.0.0.1:8787");

    let blocked = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), false, false, false, true, false, false, false, false, "trusted-catalog-never-published-in-this-data-root");
    assert_eq!(blocked.status, "not-ready");
    assert_eq!(
        blocked.blocked_by.iter().map(|closed| (closed.gate, closed.reason)).collect::<Vec<_>>(),
        vec![
            ("authentication.bootstrapReady", "local-bootstrap-pipe-handshake-incomplete"),
            ("artifactCasBarrier", "artifact-cas-coordinator-barrier-closed"),
            ("artifactAuthority", "trusted-catalog-never-published-in-this-data-root"),
            ("adminAssets", "admin-spa-dist-missing-run-os-hub-admin-build"),
        ]
    );
    assert_eq!(blocked.artifact_authority.reason, Some("trusted-catalog-never-published-in-this-data-root"));
    assert_eq!(blocked.artifact_cas_sweeper.reason, Some("artifact-cas-coordinator-barrier-closed"));
    assert!(blocked.directory.reason.is_none(), "an open gate never invents a reason");
    let line = startup_readiness_line(&blocked, &addr);
    assert!(line.starts_with("[WARN] os-hub listening at http://127.0.0.1:8787 but /readyz reports not-ready — closed gates: "), "{line}");
    for reason in ["local-bootstrap-pipe-handshake-incomplete", "trusted-catalog-never-published-in-this-data-root", "admin-spa-dist-missing-run-os-hub-admin-build"] {
        assert!(line.contains(reason), "{line} omits {reason}");
    }
    let production = hub_readiness(HubMode::Production, "network", "production".into(), false, true, true, true, true, true, false, false, "native-artifact-execution-feature-not-compiled");
    assert_eq!(production.blocked_by.iter().map(|closed| closed.reason).collect::<Vec<_>>(), vec!["identity-assertion-verifier-not-configured"]);
}

#[tokio::test]
async fn artifact_creation_admission_cannot_activate_after_shutdown_deadline() {
    let owner = Arc::new(ArtifactCreationHttpTaskOwnerV1::new());
    let control = Arc::new(ArtifactCreationHttpControlV1::new());
    let reservation = match owner.reserve("user\0space\0request".into(), control) {
        ArtifactCreationHttpAdmissionV1::Owner(reservation) => reservation,
        _ => panic!("first exact admission owns its reservation"),
    };
    let pending = reservation.pending.clone();
    let shutdown = tokio::spawn({
        let owner = owner.clone();
        async move { owner.shutdown_with_deadline(std::time::Duration::from_millis(10)).await }
    });
    loop {
        if owner.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).closing {
            break;
        }
        tokio::task::yield_now().await;
    }
    shutdown.await.expect("bounded owner shutdown");
    let executed = Arc::new(std::sync::atomic::AtomicBool::new(false));
    reservation.activate({
        let executed = executed.clone();
        async move { executed.store(true, std::sync::atomic::Ordering::Release) }
    });
    tokio::task::yield_now().await;
    assert!(!executed.load(std::sync::atomic::Ordering::Acquire), "a durable late acceptance cannot start its factory after close");
    assert_eq!(pending.disposition.load(std::sync::atomic::Ordering::Acquire), 2, "joiners observe the retired pre-accept owner");
    assert_eq!(owner.task_count(), 0, "shutdown clears reservations and cannot leak a post-close task");
    assert!(matches!(owner.reserve("user\0space\0request".into(), Arc::new(ArtifactCreationHttpControlV1::new())), ArtifactCreationHttpAdmissionV1::Unavailable));
}

/// 🔑️ Ticket 26/09/18 slice HC1: the one-second recovery sweep must not close a key an in-flight
/// execution owns. Recovery reads durable facts alone, where a creation that is legitimately still
/// working and one whose executor died look identical; the owner is the only thing in the process
/// that knows the difference, and `owns_live_execution` is what the sweep asks before it closes
/// anything. Ownership starts at the RESERVATION, not at the task, because the window between them
/// is exactly where the accept fact becomes durable and visible to the sweep.
#[tokio::test]
async fn artifact_creation_recovery_never_closes_a_key_a_live_execution_owns() {
    let owner = Arc::new(ArtifactCreationHttpTaskOwnerV1::new());
    let key = "user\0space\0request".to_string();
    assert!(!owner.owns_live_execution(&key), "an unknown key is recovery's to close");
    let reservation = match owner.reserve(key.clone(), Arc::new(ArtifactCreationHttpControlV1::new())) {
        ArtifactCreationHttpAdmissionV1::Owner(reservation) => reservation,
        _ => panic!("first exact admission owns its reservation"),
    };
    assert!(owner.owns_live_execution(&key), "a reserved key is owned before its execution task exists");
    let release = Arc::new(tokio::sync::Notify::new());
    let held = release.clone();
    reservation.activate(async move { held.notified().await });
    assert!(owner.owns_live_execution(&key), "an activated execution owns its key while it runs");
    release.notify_one();
    let mut settled = false;
    for _ in 0..1_000 {
        if !owner.owns_live_execution(&key) {
            settled = true;
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    }
    assert!(settled, "a finished execution releases its key back to recovery");
    owner.shutdown().await;
}

#[tokio::test]
async fn trusted_catalog_startup_is_selected_only_by_the_server_owned_data_root() {
    let data_root = std::fs::canonicalize(tempdir("unconfigured-trusted-catalog")).expect("canonical fixture-owned data root");
    assert!(configured_artifact_authority(&data_root, Some(&NativeCodecProviderSetV1::linked()), &Tracer::disabled(), &StartupCancellationV1::default(), &StartupProgressCellV1::default()).await.expect("unconfigured authority").is_none());
    std::fs::remove_dir_all(data_root).expect("remove unconfigured trusted catalog fixture");
}

/// 🚪️ A launcher that leaves while the hub is still loading its trusted catalog cancels that load:
/// it answers `Cancelled` at once instead of being retried as a stall, and the same catalog still
/// loads when nobody cancelled.
#[tokio::test]
async fn launcher_close_cancels_the_startup_catalog_load_instead_of_retrying_it() {
    let catalog_root = native_openable_stdio_bundle();
    let providers = NativeCodecProviderSetV1::linked();
    let cancellation = StartupCancellationV1::default();
    cancellation.cancel();
    let started = std::time::Instant::now();
    assert!(matches!(configured_artifact_authority(&catalog_root, Some(&providers), &Tracer::disabled(), &cancellation, &StartupProgressCellV1::default()).await, Err(AuthorityError::Cancelled)));
    assert!(started.elapsed() < std::time::Duration::from_secs(5), "a cancelled load is not retried");
    assert!(configured_artifact_authority(&catalog_root, Some(&providers), &Tracer::disabled(), &StartupCancellationV1::default(), &StartupProgressCellV1::default()).await.expect("uncancelled load").configured().is_some());
    std::fs::remove_dir_all(catalog_root).expect("remove cancelled-load catalog fixture");
}

#[tokio::test]
async fn configured_catalog_without_a_native_provider_fails_closed() {
    let unconfigured = tempdir("unconfigured-headless-catalog");
    assert!(configured_artifact_authority(&unconfigured, None, &Tracer::disabled(), &StartupCancellationV1::default(), &StartupProgressCellV1::default()).await.expect("unconfigured headless authority").is_none());
    std::fs::create_dir_all(unconfigured.join("trusted-catalog")).expect("trusted catalog directory");
    std::fs::write(unconfigured.join("trusted-catalog/current.json"), b"{}\n").expect("configured current pointer");
    let error = match configured_artifact_authority(&unconfigured, None, &Tracer::disabled(), &StartupCancellationV1::default(), &StartupProgressCellV1::default()).await {
        Ok(_) => panic!("configured trusted catalog unexpectedly admitted without its native provider"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("requires the native-artifact-execution provider"));
    std::fs::remove_dir_all(unconfigured).expect("remove headless trusted catalog fixture");
}

struct SyntheticDirectoryEventSource {
    head: u64,
    requests: std::sync::Mutex<Vec<(u64, usize)>>,
}

impl DirectoryEventPageSource for SyntheticDirectoryEventSource {
    async fn directory_event_head(&self) -> Result<u64, DirectoryError> {
        Ok(self.head)
    }

    async fn directory_event_page(&self, since: u64, limit: usize) -> Result<Vec<DirectoryEvent>, DirectoryError> {
        self.requests.lock().expect("request lock").push((since, limit));
        let limit = u64::try_from(limit).map_err(|error| DirectoryError::Backend(error.to_string()))?;
        let end = since.saturating_add(limit).min(self.head);
        Ok(((since + 1)..=end)
            .map(|seq| DirectoryEvent {
                seq,
                id: format!("event-{seq}"),
                hlc: os_directory::Hlc { physical_ms: i64::try_from(seq).expect("bounded synthetic sequence"), logical: 0 },
                actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:paged-read-law".into() },
                space_id: Some("default".into()),
                user_id: None,
                body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: "default".into(), name: format!("paged-{seq}") },
                recorded_at_ms: 0,
            })
            .collect())
    }
}

/// 📡️ A catalog load that is still working says so at the level a launcher runs the hub at: an
/// in-flight unit (a guest codec's fuel progress) is an `info` record, rate-limited per second, and
/// the terminal unit an `info` `ok`. A launcher's readiness waiter reads exactly this output, so
/// a load whose progress only reached `debug` looked wedged and was killed while it still worked.
#[test]
fn startup_catalog_progress_is_visible_at_the_launcher_level() {
    let (tracer, sink) = Tracer::capturing(TraceLevel::Info);
    let control = StartupCatalogControl::new(tracer, StartupCancellationV1::default(), StartupProgressCellV1::default());
    control.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: 25_000_000, total_units: 4_000_000_000 });
    control.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: 50_000_000, total_units: 4_000_000_000 });
    control.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: 4_000_000_000, total_units: 4_000_000_000 });
    let records = sink.records_for("server.catalog.publication");
    assert_eq!(records.iter().map(|record| (record.level, record.outcome)).collect::<Vec<_>>(), vec![(TraceLevel::Info, TraceOutcome::Started), (TraceLevel::Info, TraceOutcome::Ok)], "one in-flight record per gap, then the terminal one: {records:?}");
    assert!(records[0].detail.as_deref().is_some_and(|detail| detail.contains("GuestCodecExecuting 25000000/4000000000")), "{records:?}");
}

#[cfg(feature = "integration-fixtures")]
#[tokio::test]
async fn check_in_process_fixture_emits_verified_gis_ledger_and_catalog() {
    use semio_hub_trusted_catalog_fixtures as trusted_catalog_fixture;

    let artifact_root = semio_hub_testing::test_artifact_root();
    let destination = artifact_root.join("check-in-process-fixture");
    let stage = artifact_root.join(format!(".check-in-process-fixture-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&stage);
    let _ = std::fs::remove_dir_all(&destination);
    std::fs::create_dir_all(&stage).expect("create process fixture stage");

    let profile = trusted_catalog_fixture::verified_gis_map_integration_profile(&trusted_catalog_fixture::unique_profile_root("checkpoint-process")).await.expect("verified GIS Map process profile");
    let selection = profile.binding().selection();
    assert_eq!((selection.artifact.kind.as_str(), selection.artifact.schema.as_str()), ("s.gis.gismap", "gis.map"));
    let source = profile.bundle_path().parent().expect("profile bundle parent");
    let bundle_bytes = std::fs::read(profile.bundle_path()).expect("read verified profile bundle");
    let bundle: serde_json::Value = serde_json::from_slice(&bundle_bytes).expect("decode verified profile bundle");
    let generation_id = bundle["profiles"][0]["generationId"].as_str().expect("verified generation id");
    let generation = stage.join("data/trusted-catalog/generations").join(generation_id);
    std::fs::create_dir_all(&generation).expect("create trusted generation");
    for name in ["component.wasm", "descriptor.semio", "closed-actor.mjs", "plugin-module-gis.json", "stdio-component.wasm", "stdio-descriptor.semio", "plugin-module-stdio.json", "trusted-catalog.json"] {
        std::fs::copy(source.join(name), generation.join(name)).unwrap_or_else(|error| panic!("copy verified {name}: {error}"));
    }
    for directory in ["plugin-modules"] {
        std::fs::create_dir_all(generation.join(directory)).expect("create verified plugin module store");
        for entry in std::fs::read_dir(source.join(directory)).expect("verified plugin module store") {
            let entry = entry.expect("verified plugin module file");
            std::fs::copy(entry.path(), generation.join(directory).join(entry.file_name())).expect("copy verified plugin module file");
        }
    }
    let bundle_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&bundle_bytes));
    std::fs::create_dir_all(stage.join("data/trusted-catalog")).expect("create trusted current owner");
    std::fs::write(
        stage.join("data/trusted-catalog/current.json"),
        format!(
            r#"{{"profileId":"{}","generationId":"{generation_id}","bundleSha256":"{bundle_sha256}","publicationRevision":"1"}}
"#,
            trusted_catalog_fixture::GIS_MAP_INTEGRATION_PROFILE_ID
        ),
    )
    .expect("write trusted current pointer");

    let envelopes = check_in_fixture_gis_ledger_edit(&selection.artifact.schema).await;
    let hex = |bytes: &[u8]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let ledger: Vec<serde_json::Value> = envelopes
        .iter()
        .map(|envelope| {
            serde_json::json!({
                "mutationId": envelope.mutation_id.0,
                "dependencies": envelope.dependencies.iter().map(|dependency| dependency.0.clone()).collect::<Vec<_>>(),
                "diffSchema": envelope.diff.schema.0,
                "diff": hex(&envelope.diff.payload),
                "inverseSchema": envelope.inverse.schema.0,
                "inverse": hex(&envelope.inverse.payload),
            })
        })
        .collect();
    let ledger_bytes = serde_json::to_vec_pretty(&ledger).expect("encode process ledger");
    let payload = stage.join("payload");
    std::fs::create_dir_all(&payload).expect("create process payload owner");
    std::fs::write(payload.join("envelopes.json"), &ledger_bytes).expect("write process ledger");
    let fixture = serde_json::json!({
        "schema": "semio.hub.check-in-process-fixture/v1",
        "profileId": trusted_catalog_fixture::GIS_MAP_INTEGRATION_PROFILE_ID,
        "generationId": generation_id,
        "documentId": "mcp-cold-gis-map",
        "package": {
            "pluginId": selection.package.plugin_id.as_str(),
            "packageId": selection.package.package_id.as_str(),
            "version": selection.package.version.as_str(),
            "componentSha256": selection.package.component_sha256.as_str()
        },
        "artifact": {
            "kind": selection.artifact.kind.as_str(),
            "schema": selection.artifact.schema.as_str(),
            "packSchemaHash": selection.artifact.pack_schema_hash.as_str()
        },
        "surfaceId": selection.surface.surface_id.as_str(),
        "payload": { "envelopes": { "path": "payload/envelopes.json", "count": envelopes.len(), "sha256": os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&ledger_bytes)) } }
    });
    std::fs::write(stage.join("fixture.json"), serde_json::to_vec_pretty(&fixture).expect("encode process fixture")).expect("write process fixture receipt");
    std::fs::rename(&stage, &destination).expect("publish process fixture atomically");
    let dependency_fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🧫️fixtures/🔗️compiled-dependencies/🔣️.json")).expect("compiled dependency fixture");
    let expected_files: std::collections::BTreeSet<_> = ["publicationFiles", "publicationDirectories"].iter().flat_map(|key| dependency_fixture[*key].as_array().unwrap().iter()).map(|name| name.as_str().unwrap().to_owned()).collect();
    let retained_generation = destination.join("data/trusted-catalog/generations").join(generation_id);
    let actual_files: std::collections::BTreeSet<_> = std::fs::read_dir(&retained_generation).unwrap().map(|entry| entry.unwrap().file_name().into_string().unwrap()).collect();
    assert_eq!(actual_files, expected_files);
    let control = StartupCatalogControl::silent();
    let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let relocated = TrustedCatalogLoader::load_current(&destination.join("data"), &NativeCodecProviderSetV1::linked(), &context).await.expect("relocated process catalog dependency closure").expect("relocated current");
    assert_eq!(relocated.codec_count(), 31);
    assert_eq!(relocated.packages().len(), 2);
    assert_eq!(relocated.generation_id(), generation_id);
    assert!(destination.join("data/trusted-catalog/current.json").is_file());
    assert_eq!(std::fs::read(destination.join("payload/envelopes.json")).expect("read retained GIS ledger"), ledger_bytes);
}

/// 🗺️ The ledger a real GIS Map editor emits for one edit that places `gis_map_test_snapshot()`'s
/// position and route onto an empty map: its store's own event log, exactly what a client sends.
#[cfg(feature = "integration-fixtures")]
async fn check_in_fixture_gis_ledger_edit(schema: &str) -> Vec<MutationEnvelope> {
    use directory::os_store::{store_schema::ArtifactCommand, ArtifactStore, SnapshotRetirementStep};
    use semio_s_artifact_gis_gismap::mutations::{create_position::CreatePosition, create_route::CreateRoute};
    use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot};
    let target = gis_map_test_snapshot();
    let mut store = ArtifactStore::<GisMapSnapshot, GisMapMutation>::new(directory::os_store::document_materialize::create_document_envelope(schema, "check-in-fixture", GisMapSnapshot::default(), None), directory::ActorId(directory::os_spr::LOCAL_ACTOR_ID.into())).await.expect("fixture GIS store");
    store.install_document_store_owners_exact(directory::os_store::document_store_owners::bounded_artifact_store_owners());
    let applied = store
        .dispatch(ArtifactCommand::Apply {
            mutations: vec![GisMapMutation::CreatePosition(CreatePosition { index: 0, item: target.positions[0].clone() }), GisMapMutation::CreateRoute(CreateRoute { index: 0, item: target.routes[0].clone() })],
            transaction: None,
        })
        .await;
    let events = store.event_log();
    loop {
        match store.close_owned_step(1, directory::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).expect("fixture GIS store closes") {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { .. } => {}
            SnapshotRetirementStep::Blocked => panic!("fixture GIS store close blocked"),
        }
    }
    assert!(store.close_owned_terminal_is_empty());
    drop(store);
    applied.expect("fixture GIS edit applies");
    events.expect("fixture GIS ledger")
}

#[test]
fn mutation_message_payload_matches_language_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🚧️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
    let messages = vec![protocol::MutationMessage::warning("mutation.clamped", "height clamped").at(["node", "height"]).at_op(2), protocol::MutationMessage::info("mutation.cascade", "dependent value updated")];
    let encoded = encode_messages(&messages);
    let parsed: serde_json::Value = serde_json::from_slice(&encoded).expect("first-party message bytes are valid JSON");
    assert_eq!(parsed, fixture["mutationMessages"]);
    assert_eq!(<Vec<protocol::MutationMessage> as FromValue>::from_value(DslValue::from(parsed)).expect("first-party message decode"), messages);
}

/// 🚧️ The hostile-input fixture names every route the router registers: each `.route(` path of the
/// bootstrap source is one fixture `route`, so a new route cannot ship without its hostile vectors.
#[test]
fn the_hostile_input_fixture_covers_every_registered_route() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🚧️hostile-input-v1/🔣️.json")).unwrap();
    let covered: std::collections::BTreeSet<&str> = fixture["routes"].as_array().unwrap().iter().map(|row| row["route"].as_str().unwrap()).collect();
    let source = [semio_hub_testing::hub_source("🏗️bootstrap/🛣️routes/🦀️.rs"), semio_hub_testing::hub_source("🏗️bootstrap/🩺️readiness/🦀️.rs")].concat();
    let registered: std::collections::BTreeSet<String> = source
        .match_indices(".route(")
        .filter_map(|(at, _)| {
            let argument = source[at + ".route(".len()..].split([',', ')']).next()?.trim();
            Some(argument.trim_matches('"').rsplit("::").next()?.to_string())
        })
        .collect();
    let missing: Vec<&String> = registered.iter().filter(|route| !covered.contains(route.as_str())).collect();
    assert!(missing.is_empty(), "routes without hostile vectors: {missing:?}");
    assert!(registered.len() >= 60, "the source scan found the router: {}", registered.len());
}

/// 🚧️ Every route answers every hostile vector with a typed, signed refusal: a 4xx (or a typed 503
/// for a subsystem this hub was not configured with) carrying `x-semio-refusal` with the code its
/// status declares; the body stays the route's own, so a bare refusal discloses nothing. Never a 500,
/// never a dropped connection. A `credential-ignored` public route may succeed whatever the request
/// carries; a `credential-optional` one serves a request without a credential and refuses a forged one.
#[test]
fn every_route_answers_hostile_input_with_a_typed_signed_refusal() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🚧️hostile-input-v1/🔣️.json")).unwrap();
        let state = test_state().await;
        let session = issue_test_session(&state, "hostile-input@example.com").await;
        let addr = spawn_server(state.clone()).await;
        let bearer = format!("Bearer {}", session.token);
        let forged = fixture["bodies"]["forgedBearer"].as_str().unwrap();
        let malformed = fixture["bodies"]["malformedJson"].as_str().unwrap().as_bytes().to_vec();
        let wrong_schema = serde_json::to_vec(&fixture["bodies"]["wrongSchema"]).unwrap();
        let oversized = vec![b'x'; fixture["oversizedBytes"].as_u64().unwrap() as usize];
        let max = fixture["allowedStatuses"]["max"].as_u64().unwrap() as u16;
        let unavailable = fixture["allowedStatuses"]["typedUnavailable"].as_u64().unwrap() as u16;
        let mut findings = Vec::new();
        let mut answered = 0usize;
        let judge = |findings: &mut Vec<String>, label: String, public: bool, answer: Result<RawHttpResponse, String>| {
            let response = match answer {
                Ok(response) => response,
                Err(error) => return findings.push(format!("{label}: {error}")),
            };
            if response.status < 400 {
                if !public {
                    findings.push(format!("{label}: a hostile request succeeded with {}", response.status));
                }
                return;
            }
            if response.status > max && response.status != unavailable {
                findings.push(format!("{label}: answered {} {}", response.status, String::from_utf8_lossy(&response.body)));
            }
            let head = response.headers.to_ascii_lowercase();
            let Some(code) = head.lines().find_map(|line| line.strip_prefix(&format!("{}: ", semio_hub_refusal::HUB_REFUSAL_HEADER)).map(str::trim).map(str::to_string)) else {
                return findings.push(format!("{label}: {} without {}", response.status, semio_hub_refusal::HUB_REFUSAL_HEADER));
            };
            if semio_hub_refusal::refusal_code(response.status).is_some_and(|declared| declared != code && code != "insecure-transport") && !head.contains("content-type: application/json") {
                findings.push(format!("{label}: {} carries {code}, not its declared code", response.status));
            }
            if head.contains("content-type: application/json") && !response.body.is_empty() && serde_json::from_slice::<serde_json::Value>(&response.body).is_err() {
                findings.push(format!("{label}: a JSON refusal body that is not JSON"));
            }
        };
        for route in fixture["routes"].as_array().unwrap() {
            let method = route["method"].as_str().unwrap();
            let path = route["path"].as_str().unwrap();
            let public = route["public"].as_str();
            let body_kind = route["body"]["kind"].as_str();
            let json = [("content-type", "application/json"), ("origin", "http://127.0.0.1:6066")];
            let requests: Vec<(&str, Vec<(&str, &str)>, Vec<u8>)> = match body_kind {
                Some("json") => vec![
                    ("unauthenticated", json.to_vec(), wrong_schema.clone()),
                    ("forged-bearer", vec![json[0], json[1], ("authorization", forged)], wrong_schema.clone()),
                    ("malformed-json", vec![json[0], json[1], ("authorization", bearer.as_str())], malformed.clone()),
                    ("wrong-schema", vec![json[0], json[1], ("authorization", bearer.as_str())], wrong_schema.clone()),
                    ("wrong-content-type", vec![("content-type", "text/plain"), json[1], ("authorization", bearer.as_str())], wrong_schema.clone()),
                    ("oversized", vec![json[0], json[1], ("authorization", bearer.as_str())], oversized.clone()),
                ],
                Some("bytes") => vec![
                    ("unauthenticated", vec![("content-type", "application/octet-stream")], b"hostile".to_vec()),
                    ("forged-bearer", vec![("content-type", "application/octet-stream"), ("authorization", forged)], b"hostile".to_vec()),
                    ("oversized", vec![("content-type", "application/octet-stream"), ("authorization", bearer.as_str())], oversized.clone()),
                ],
                Some(_) => vec![
                    ("unauthenticated", vec![], Vec::new()),
                    ("forged-bearer", vec![("authorization", forged)], Vec::new()),
                    ("oversized", vec![("content-type", "application/json"), ("authorization", bearer.as_str())], oversized.clone()),
                ],
                None => vec![("unauthenticated", vec![], Vec::new()), ("forged-bearer", vec![("authorization", forged)], Vec::new())],
            };
            for (vector, headers, body) in &requests {
                let answer = hostile_http_request(addr, method, path, headers, body).await;
                answered += 1;
                judge(&mut findings, format!("{method} {path} [{vector}]"), public == Some("credential-ignored") || (public == Some("credential-optional") && *vector == "unauthenticated"), answer);
            }
            if !route["socket"].as_bool().unwrap_or(false) {
                let wrong_method = if method == "PATCH" { "TRACE" } else { "PATCH" };
                let answer = hostile_http_request(addr, wrong_method, path, &[("authorization", bearer.as_str())], &[]).await;
                answered += 1;
                judge(&mut findings, format!("{wrong_method} {path} [wrong-method]"), false, answer);
            }
            if let Some(segment_at) = path.find("space-h").filter(|at| !path[..*at].contains('?')) {
                for hostile in fixture["hostilePathSegments"].as_array().unwrap() {
                    let hostile_path = format!("{}{}{}", &path[..segment_at], hostile.as_str().unwrap(), &path[segment_at + "space-h".len()..]);
                    let answer = hostile_http_request(addr, method, &hostile_path, &[("authorization", bearer.as_str())], &[]).await;
                    answered += 1;
                    judge(&mut findings, format!("{method} {hostile_path} [hostile-path]"), public.is_some(), answer);
                }
            }
        }
        assert!(findings.is_empty(), "{} of {answered} hostile requests were not typed, signed refusals:\n{}", findings.len(), findings.join("\n"));
        assert!(answered > 400, "every route saw its vectors: {answered}");
    });
}

/// 🚪️ Every route that also answers anonymously refuses a PRESENTED credential that does not authenticate with the one
/// typed, localized answer of `🚧️refusal/🧫️fixtures/🪪️credential-refusal-v1`: `401`, `x-semio-refusal: unauthenticated`,
/// the RFC 6750 `invalid_token` challenge and the `HubCredentialRefusalV1` body (en + de) — identical for a malformed, a
/// forged (well-formed, never issued) and a revoked bearer, and never the anonymous view. An absent credential is still
/// the anonymous view, and a live session is served.
#[test]
fn credential_optional_routes_refuse_a_revoked_or_forged_session_instead_of_answering_anonymously() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🚧️refusal/🧪️tests/🧫️fixtures/🪪️credential-refusal-v1/🔣️.json")).expect("credential refusal fixture");
        let answer = &fixture["answer"];
        let state = test_state().await;
        let session = issue_test_session(&state, "optional-caller@example.com").await;
        let public_space = create_space_for_test(&state, &session.user_id, "Credential-optional public space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let addr = spawn_server(state.clone()).await;
        let live = format!("Bearer {}", session.token);
        let forged = format!("Bearer {}", SessionCapability::mint().expect("well-formed capability").expose_once());
        let paths: Vec<(String, u16)> = fixture["routes"].as_array().expect("routes").iter().map(|route| (route["path"].as_str().expect("path").replace("{publicSpace}", &public_space), route["anonymousStatus"].as_u64().expect("anonymous status") as u16)).collect();
        let assert_refused = |path: &str, vector: &str, response: &RawHttpResponse| {
            assert_eq!(u64::from(response.status), answer["status"].as_u64().unwrap(), "{path} {vector}: refused, never the anonymous view");
            let head = response.headers.to_ascii_lowercase();
            assert!(head.contains(&format!("{}: {}", semio_hub_refusal::HUB_REFUSAL_HEADER, answer["refusal"].as_str().unwrap())), "{path} {vector}: signed refusal: {head}");
            assert!(head.contains(&format!("www-authenticate: {}", answer["challenge"].as_str().unwrap().to_ascii_lowercase())), "{path} {vector}: RFC 6750 challenge: {head}");
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&response.body).expect("refusal JSON"), answer["body"], "{path} {vector}: the declared localized body");
        };
        for (path, anonymous) in &paths {
            assert_eq!(raw_http_get(addr, path, &[]).await.status, *anonymous, "{path}: no credential is the anonymous view");
            assert_eq!(raw_http_get(addr, path, &[("Authorization", live.as_str())]).await.status, 200, "{path}: a live session is served");
            for (vector, bearer) in [("malformed", "Bearer not-a-session"), ("forged", forged.as_str())] {
                assert!(fixture["presented"].as_array().unwrap().iter().any(|name| name == vector));
                assert_refused(path, vector, &raw_http_get(addr, path, &[("Authorization", bearer)]).await);
            }
        }
        let authenticated = state.directory.authenticate_session(&SessionCapability::parse(&session.token).unwrap()).await.unwrap().expect("live session");
        state.directory.revoke_auth_session(&authenticated.id, "test-revocation", Some(&session.user_id), "optional-caller").await.expect("revoke");
        for (path, anonymous) in &paths {
            assert_refused(path, "revoked", &raw_http_get(addr, path, &[("Authorization", live.as_str())]).await);
            assert_eq!(raw_http_get(addr, path, &[]).await.status, *anonymous, "{path}: the anonymous view stays open");
        }
    });
}

/// ⏳️ A document batch refused for a TRANSIENT reason carries exactly the declared `HubTransientApplyRefusalMessageV1`
/// (`🚧️refusal/🧫️fixtures/⏳️transient-apply-refusal-v1`): the db engine's `Unavailable` (DB I/O admission or capacity
/// exhausted) and a paced-out agent command answer `[{level: warning, code: hub.unavailable, message: <reason>}]`, so the
/// client resends instead of discarding the batch; a permanent refusal carries no such message.
#[test]
fn a_transiently_refused_batch_names_the_declared_resend_code_and_a_permanent_one_does_not() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🚧️refusal/🧪️tests/🧫️fixtures/⏳️transient-apply-refusal-v1/🔣️.json")).expect("transient refusal fixture");
    let answer = &fixture["answer"];
    for cause in fixture["causes"].as_array().expect("causes") {
        let reason = cause["reason"].as_str().expect("reason");
        let encoded = match cause["cause"].as_str() {
            Some("db-unavailable") => messages_for_error(&db::DbError::Unavailable(reason.to_string())),
            Some("agent-rate-limited") => transient_apply_refusal_messages(reason),
            other => panic!("unknown transient cause {other:?}"),
        };
        let messages: serde_json::Value = serde_json::from_slice(&encoded).expect("messages are JSON");
        assert_eq!(messages, serde_json::json!([{ "level": answer["level"], "code": answer["code"], "message": reason }]), "{cause}");
    }
    for cause in fixture["permanentCauses"].as_array().expect("permanent causes") {
        let reason = cause["reason"].as_str().expect("reason").to_string();
        assert!(messages_for_error(&db::DbError::InvalidArgument(reason.clone())).is_empty(), "{cause}");
        assert!(messages_for_error(&db::DbError::Conflict(reason)).is_empty(), "{cause}");
    }
}

/// ⚖️ C12 P1 (ticket 26/09/23 session 14): a document socket's gate admits, on a full budget, the largest batch the
/// document backbone declares legal (`DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES` — a link cut's whole outbox arrives as
/// one); an emptied budget refuses transiently (`hub.unavailable`, the client resends); a batch that was admitted but not
/// committed is admitted again once the budget refilled — never a replay — while a committed operation is.
#[test]
fn a_document_socket_admits_a_declared_maximal_batch_and_readmits_an_uncommitted_one() {
    run_socket_test(|| async {
        let document = WireArtifactId("p1-socket-budget".to_string());
        let gate = document_socket_gate(db::security::RoleBasedPolicy::new().with_grant(db::security::Grant::allow("editor", &["db", "document", "*", "**"], &[db::security::Action::Write])));
        let principal = db::security::Principal::new(ActorId("actor-1".to_string()), db::security::TenantId::from("space-1"), vec!["editor".to_string()]);
        let tenant = db::security::TenantId::from("space-1");
        let template = sample_envelope("p1", &document).await;
        let envelopes: Vec<MutationEnvelope> = (0..protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES).map(|index| MutationEnvelope { mutation_id: protocol::MutationId(format!("p1-{index}")), ..template.clone() }).collect();
        assert!(admit_writes(&gate, &principal, &tenant, &document, &envelopes, 0).await.is_ok(), "a full budget admits a declared-maximal batch");
        let refused = admit_writes(&gate, &principal, &tenant, &document, &envelopes[..1], 1).await.expect_err("an emptied budget refuses");
        let messages: serde_json::Value = serde_json::from_slice(&messages_for_error(&refused)).expect("messages are JSON");
        assert_eq!(messages[0]["code"], semio_hub_refusal::HUB_TRANSIENT_APPLY_REFUSAL_CODE, "an emptied budget is a transient refusal: {refused}");
        let refilled_ms = (protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES as u64 * 1_000).div_ceil(DOCUMENT_SOCKET_COMMAND_REFILL_PER_SECOND as u64) + 1;
        assert!(admit_writes(&gate, &principal, &tenant, &document, &envelopes, refilled_ms).await.is_ok(), "an admitted but uncommitted batch is readmitted, never a replay");
        gate.record_committed(&[(&envelopes[0].actor, &envelopes[0].mutation_id)], refilled_ms);
        let replayed = admit_writes(&gate, &principal, &tenant, &document, &envelopes[..1], refilled_ms + 1_000).await.expect_err("a committed operation is a replay within the window");
        assert!(matches!(replayed, db::DbError::Conflict(_)) && messages_for_error(&replayed).is_empty(), "a replay is permanent: {replayed}");
    });
}

/// 📏️ C12 P1 live (a 600-envelope outbox answered `hub.unavailable` forever): a `Commands` batch the document backbone
/// declares legal passes the socket's declaration check, one envelope more is refused PERMANENTLY with the one
/// `HubBatchLimitRefusalMessageV1` (`hub.batch-limit`, level error), and an engine `LimitExceeded` maps to the same permanent
/// message — no size refusal ever carries the transient `hub.unavailable` a client would resend without end.
#[test]
fn a_batch_beyond_its_declaration_is_refused_permanently_and_never_as_transient() {
    run_socket_test(|| async {
        let document = WireArtifactId("p1-declared-batch".to_string());
        let template = sample_envelope("declared", &document).await;
        let mut declared = Vec::new();
        let mut bytes = 0usize;
        let next = loop {
            let next = MutationEnvelope { mutation_id: protocol::MutationId(format!("declared-{:x}", declared.len())), ..template.clone() };
            let mut encoded = Vec::new();
            protocol::encode_envelope(&next, &mut encoded);
            let count_bytes = if declared.len() + 1 < 128 { 1 } else if declared.len() + 1 < 16_384 { 2 } else { 3 };
            if declared.len() == protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES || count_bytes + bytes + encoded.len() > protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES {
                break next;
            }
            bytes += encoded.len();
            declared.push(next);
        };
        let mut over = declared.clone();
        over.push(next);
        assert!(declared.len() > 256, "the declared batch ({} envelopes) exceeds the engine's former 256-envelope credit", declared.len());
        assert!(undeclared_batch_refusal(&declared).is_none(), "a declared batch passes the socket's declaration check");
        let Some(ApplyOutcome::Rejected { reason, messages }) = undeclared_batch_refusal(&over) else { panic!("one envelope beyond the declaration is refused") };
        let messages: serde_json::Value = serde_json::from_slice(&messages).expect("messages are JSON");
        assert_eq!(messages[0]["code"], semio_hub_refusal::HUB_BATCH_LIMIT_REFUSAL_CODE, "{reason}");
        assert_eq!(messages[0]["level"], "error");
        assert_eq!(messages.as_array().unwrap().len(), 1);
        let engine: serde_json::Value = serde_json::from_slice(&messages_for_error(&db::DbError::LimitExceeded("artifact submit batch item credit"))).expect("messages are JSON");
        assert_eq!(engine[0]["code"], semio_hub_refusal::HUB_BATCH_LIMIT_REFUSAL_CODE, "an engine size refusal is permanent");
        assert_ne!(engine[0]["code"], semio_hub_refusal::HUB_TRANSIENT_APPLY_REFUSAL_CODE);
    });
}

/// 🏘️ A member of many spaces reads its space list and its event pages in bounded time, exactly: the list is one
/// directory query and each event page one membership read (WG8 on 7800: 25–58 s for 82 spaces, the list folded the
/// whole event log and mounted every document). The oracle is the event-log fold (`os_directory::fold_all`): the same
/// visible spaces, discriminators, roles, member and document counts; each answer within the declared page budget.
#[test]
fn a_member_of_many_spaces_reads_its_space_list_and_event_pages_within_the_page_budget() {
    run_socket_test(|| async {
        const SPACES: usize = 120;
        const PAGE_BUDGET: std::time::Duration = std::time::Duration::from_millis(200);
        let state = test_state().await;
        let member = issue_test_session(&state, "many-spaces@example.com").await;
        let stranger = issue_test_session(&state, "many-spaces-stranger@example.com").await;
        for index in 0..SPACES {
            let owner = if index % 3 == 0 { &stranger } else { &member };
            let kind = if index % 2 == 0 { os_directory::DirectorySpaceKind::Studio } else { os_directory::DirectorySpaceKind::Atelier };
            let visibility = if index % 4 == 0 { DirectorySpaceVisibility::Public } else { DirectorySpaceVisibility::Private };
            let space_id = create_space_for_test(&state, &owner.user_id, &format!("Space {index:03}"), kind, visibility).await;
            if index % 6 == 0 {
                upsert_member_for_test(&state, &space_id, "many-spaces@example.com", DirectorySpaceRole::Spectator).await;
            }
            if index % 5 == 0 {
                announce_document_for_test(&state, &space_id, &format!("many-spaces-doc-{index}")).await;
            }
        }
        let events = load_all_directory_events(state.directory.as_ref(), 0).await.expect("directory events");
        let model = os_directory::fold_all(os_directory::DirectoryReadModel::default(), &events).await;
        let mut expected = BTreeMap::new();
        for space in model.spaces.values() {
            let role = space.members.iter().find(|row| row.user_id == member.user_id).map(|row| row.role);
            let access = match directory_space_access_decision(space.view.visibility == DirectorySpaceVisibility::Public, role) {
                DirectorySpaceAccessDecisionV1::Hidden => continue,
                DirectorySpaceAccessDecisionV1::Public => "public",
                DirectorySpaceAccessDecisionV1::Member => "member",
                DirectorySpaceAccessDecisionV1::Author => "author",
            };
            expected.insert(space.view.id.clone(), (access, space.view.name.clone(), u64::from(space.view.member_count), space.documents.len() as u64));
        }
        assert!(expected.len() > 80, "the member sees most of the {SPACES} spaces: {}", expected.len());
        let addr = spawn_server(state.clone()).await;
        let bearer = format!("Bearer {}", member.token);
        let mut fastest = std::time::Duration::MAX;
        let mut listed = serde_json::Value::Null;
        for _ in 0..3 {
            let started = std::time::Instant::now();
            let response = raw_http_get(addr, "/directory/spaces", &[("authorization", bearer.as_str())]).await;
            fastest = fastest.min(started.elapsed());
            assert_eq!(response.status, 200);
            listed = json_body(&response);
        }
        let mut observed = BTreeMap::new();
        for entry in listed.as_array().expect("space list") {
            let space = &entry["space"];
            observed.insert(space["id"].as_str().expect("id").to_string(), (entry["access"].as_str().expect("access"), space["name"].as_str().expect("name").to_string(), space["memberCount"].as_u64().expect("members"), space["documentCount"].as_u64().expect("documents")));
        }
        assert_eq!(observed, expected, "the space list equals the event-log fold");
        assert!(fastest < PAGE_BUDGET, "the list of {} spaces answered in {fastest:?}", expected.len());
        let mut after = 0u64;
        let mut pages = 0usize;
        loop {
            let started = std::time::Instant::now();
            let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("authorization", bearer.as_str())]).await;
            let elapsed = started.elapsed();
            assert_eq!(response.status, 200, "{}", String::from_utf8_lossy(&response.body));
            let page = json_body(&response);
            assert!(elapsed < PAGE_BUDGET * 2, "event page {pages} answered in {elapsed:?}");
            pages += 1;
            let through = page["throughSeqInclusive"].as_u64().expect("through");
            if !page["hasMore"].as_bool().expect("has more") || through == after {
                break;
            }
            after = through;
        }
        assert!(pages >= 2, "the event log spans several pages: {pages}");
    });
}

/// 🧯️ The refusal layer answers a panicking handler with a typed `500` instead of a dropped
/// connection, and it never detaches a handler from its request: a client that disconnects drops
/// the handler it was waiting on, exactly as without the layer.
#[tokio::test]
async fn the_refusal_layer_types_a_panic_and_keeps_every_handler_request_owned() {
    struct DropSignal(Option<tokio::sync::oneshot::Sender<()>>);
    impl Drop for DropSignal {
        fn drop(&mut self) {
            if let Some(signal) = self.0.take() {
                let _ = signal.send(());
            }
        }
    }
    async fn panicking() -> StatusCode {
        std::panic::panic_any("hostile handler")
    }
    let (started_tx, started_rx) = tokio::sync::oneshot::channel::<()>();
    let (dropped_tx, dropped_rx) = tokio::sync::oneshot::channel::<()>();
    let pending = Arc::new(Mutex::new(Some((started_tx, dropped_tx))));
    let router = axum::Router::new()
        .route("/panic", axum::routing::get(panicking))
        .route(
            "/pending",
            axum::routing::get(move || {
                let signals = pending.lock().unwrap().take();
                async move {
                    let Some((started, dropped)) = signals else { return StatusCode::CONFLICT };
                    let _dropped = DropSignal(Some(dropped));
                    let _ = started.send(());
                    std::future::pending::<StatusCode>().await
                }
            }),
        )
        .layer(axum::middleware::from_fn_with_state(Tracer::disabled(), refusal_middleware));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move { axum::serve(listener, router).await });
    let panicked = raw_http_get(addr, "/panic", &[]).await;
    assert_eq!(panicked.status, 500);
    assert!(panicked.headers.to_ascii_lowercase().contains(&format!("{}: {}", semio_hub_refusal::HUB_REFUSAL_HEADER, semio_hub_refusal::refusal_code(500).unwrap())), "{}", panicked.headers);
    let mut client = tokio::net::TcpStream::connect(addr).await.unwrap();
    tokio::io::AsyncWriteExt::write_all(&mut client, format!("GET /pending HTTP/1.1\r\nHost: {addr}\r\n\r\n").as_bytes()).await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), started_rx).await.expect("handler start deadline").expect("handler started");
    drop(client);
    tokio::time::timeout(std::time::Duration::from_secs(5), dropped_rx).await.expect("a disconnected request drops its handler").expect("handler dropped");
    assert_eq!(raw_http_get(addr, "/panic", &[]).await.status, 500, "a panic never takes the server down");
    server.abort();
}

/// 🫀️ Liveness and readiness are two different questions, and a hub that has not finished booting
/// must answer them differently: `/readyz` refuses with `503` while a required subsystem is down,
/// and `/healthz` still answers `200 live` on the very same process so an orchestrator restarts a
/// wedged hub without ever restarting a warming one.
#[tokio::test]
async fn healthz_reports_liveness_on_a_process_whose_readyz_still_refuses() {
    let state = test_state().await;
    let run_id = state.readiness.run_id.clone();
    let addr = spawn_server(state).await;

    let readiness = raw_http_get(addr, "/readyz", &[]).await;
    assert_eq!(readiness.status, 503);

    let liveness = raw_http_get(addr, "/healthz", &[]).await;
    assert_eq!(liveness.status, 200, "{}", String::from_utf8_lossy(&liveness.body));
    let body: serde_json::Value = serde_json::from_slice(&liveness.body).expect("liveness JSON");
    assert_eq!(body["schema"], "semio.hub.liveness/v1");
    assert_eq!(body["status"], "live");
    assert_eq!(body["runId"], run_id);
    assert!(body["uptimeMs"].is_u64());
    assert!(body.get("directory").is_none(), "liveness never republishes a readiness subsystem");
    assert!(body.get("artifactAuthority").is_none(), "liveness never republishes a readiness subsystem");

    let again = raw_http_get(addr, "/healthz", &[]).await;
    let repeated: serde_json::Value = serde_json::from_slice(&again.body).expect("second liveness JSON");
    assert!(repeated["uptimeMs"].as_u64().expect("uptime") >= body["uptimeMs"].as_u64().expect("uptime"));
}

//#region 💡️Inference
#[tokio::test]
async fn gis_map_approval_ingress_holds_sorted_hub_authority_without_outer_document_write() {
    let state = test_state().await;
    let email = "approval-ingress-author@example.test";
    let caller = issue_test_session(&state, email).await;
    let space_id = create_space_for_test(&state, &caller.user_id, "Approval ingress", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, email, DirectorySpaceRole::Author).await;
    let scope = DocumentScope::new(&space_id, "approval-ingress-document");
    let authority = acquire_gis_map_approval_ingress(&state, scope.clone(), Some(&caller.token)).await.expect("exact Author ingress");
    assert_eq!(authority.scope(), &scope);
    assert_eq!(authority.user_id(), caller.user_id);
    for binding in [
        SocketBindingKeyV1::User(authority.caller.user_id.clone()),
        SocketBindingKeyV1::Session(authority.caller.session_id.clone()),
        SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.clone() },
        SocketBindingKeyV1::Membership { user_id: authority.caller.user_id.clone(), space_id: space_id.clone() },
    ] {
        assert!(state.socket_binding_gates.gate(binding).try_write_owned().is_err(), "the exact Hub ingress guard remains owned");
    }
    assert!(state.socket_binding_gates.document_write(&scope).try_lock_owned().is_ok(), "Hub ingress must not outer-lock the runtime document writer");
    revalidate_gis_map_approval_authority(&state, &authority).await.expect("fresh authority under the retained guards");
    drop(authority);
    execute_directory_command_fenced(
        &state,
        DirectoryActor { kind: DirectoryActorKind::System, id: "system:approval-ingress-law".into() },
        DirectoryCommand::UpsertMember { space_id: space_id.clone(), email: email.into(), role: DirectorySpaceRole::Spectator },
    )
    .await
    .expect("demote after exact authority release");
    assert!(matches!(acquire_gis_map_approval_ingress(&state, scope, Some(&caller.token)).await, Err(InferenceRouteErrorV1::Denied)));
}

/// 🗳️ A durable approval (or undo) is answered with its receipt: once `approve_gis_map_job` /
/// `undo_gis_map_approval` returns, the route awaits nothing that could turn the committed outcome into
/// a refusal (G10: a post-commit directory re-check answered `503 inference.unavailable` while the
/// document had advanced). The ingress bindings are held across the commit instead
/// (`gis_map_approval_ingress_holds_sorted_hub_authority_without_outer_document_write`).
#[test]
fn a_committed_gis_map_approval_is_answered_with_its_receipt() {
    let source = semio_hub_testing::hub_source("🏗️bootstrap/💡️inference/🦀️.rs");
    for (route, call) in [("async fn post_inference_gis_map_job_approval(", "approve_gis_map_job(context, &job_id, &body).await"), ("async fn post_inference_gis_map_approval_undo(", "undo_gis_map_approval(context, &body).await")] {
        let body = &source[source.find(route).expect("approval route")..];
        let body = &body[..body.find("\n}\n").expect("route end")];
        let after = &body[body.find(call).expect("the route commits")..];
        assert!(after.contains("Ok(receipt) => Json(receipt).into_response(),"), "{route} answers the receipt of a durable commit");
        assert_eq!(after.matches(".await").count(), 1, "{route} awaits nothing after the commit returns");
    }
}

#[tokio::test]
async fn gis_map_applied_checkpoint_notifies_two_peers_with_one_exact_rebootstrap_pair() {
    let fanout = ShardedMap::new();
    let fanout_capacity = 8;
    let scope = DocumentScope::new("gis-map-space", "gis-map-document");
    let sender = fanout.get_or_insert_with_cloned(document_scope_key_v1(&scope), || broadcast::channel(fanout_capacity).0);
    let mut first = sender.subscribe();
    let mut second = sender.subscribe();
    assert!(matches!(first.try_recv(), Err(broadcast::error::TryRecvError::Empty)));
    assert!(matches!(second.try_recv(), Err(broadcast::error::TryRecvError::Empty)));
    let checkpoint = PublishedArtifactCheckpoint {
        scope: scope.clone(),
        checkpoint_id: ArtifactHash([0x41; 32]),
        parent_checkpoint_id: Some(ArtifactHash([0x40; 32])),
        descriptor_digest_v1: ArtifactHash([0x42; 32]),
        baseline_frontier: ArtifactFrontier { document_id: scope.document_id.clone(), head_edit_ordinal: 7, head_edit_id: "gis-map-create-region".into(), last_commit_seq: 5, chain_hash: ArtifactHash([0x43; 32]) },
        pack: os_directory::PublishedArtifactBlob { sha256: ArtifactHash([0x44; 32]), byte_length: 11 },
        spr: os_directory::PublishedArtifactBlob { sha256: ArtifactHash([0x45; 32]), byte_length: 13 },
        aggregate_sha256: ArtifactHash([0x46; 32]),
        published_at_ms: 17,
    };
    let genesis = ArtifactFrontier { document_id: scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: ArtifactHash([0; 32]) };
    assert!(!GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&scope, &genesis, None));
    for hostile in [
        ArtifactFrontier { document_id: "substituted-document".into(), ..genesis.clone() },
        ArtifactFrontier { head_edit_ordinal: 1, ..genesis.clone() },
        ArtifactFrontier { head_edit_id: "non-genesis".into(), ..genesis.clone() },
        ArtifactFrontier { last_commit_seq: 1, ..genesis.clone() },
        ArtifactFrontier { chain_hash: ArtifactHash([1; 32]), ..genesis.clone() },
    ] {
        assert!(!GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&scope, &hostile, None), "a missing Directory checkpoint is never a publication base");
    }
    assert!(GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&scope, &checkpoint.baseline_frontier, Some(&checkpoint)));
    let substituted_scope = DocumentScope::new("substituted-space", scope.document_id.clone());
    assert!(!GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&substituted_scope, &checkpoint.baseline_frontier, Some(&checkpoint)));
    for hostile in [
        ArtifactFrontier { document_id: "substituted-document".into(), ..checkpoint.baseline_frontier.clone() },
        ArtifactFrontier { head_edit_ordinal: checkpoint.baseline_frontier.head_edit_ordinal + 1, ..checkpoint.baseline_frontier.clone() },
        ArtifactFrontier { head_edit_id: "substituted-edit".into(), ..checkpoint.baseline_frontier.clone() },
        ArtifactFrontier { last_commit_seq: checkpoint.baseline_frontier.last_commit_seq + 1, ..checkpoint.baseline_frontier.clone() },
        ArtifactFrontier { chain_hash: ArtifactHash([0x47; 32]), ..checkpoint.baseline_frontier.clone() },
    ] {
        assert!(!GisMapApprovalCheckpointPublisherV1Impl::current_matches_base(&scope, &hostile, Some(&checkpoint)), "an active Directory checkpoint requires the exact scope and base frontier");
    }
    publish_gis_map_checkpoint_change(&fanout, fanout_capacity, &checkpoint);
    let expected = ServerFrame::RebootstrapRequired {
        control: wire_rebootstrap(&os_directory::RebootstrapRequired { scope, checkpoint_id: checkpoint.checkpoint_id, descriptor_digest_v1: checkpoint.descriptor_digest_v1, baseline_frontier: checkpoint.baseline_frontier.clone() }),
    };
    assert_eq!(first.recv().await.expect("first peer checkpoint change"), expected);
    assert_eq!(second.recv().await.expect("second peer checkpoint change"), expected);
    assert!(matches!(first.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "one applied checkpoint emits exactly one control per peer");
    assert!(matches!(second.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "one applied checkpoint emits exactly one control per peer");
}

#[tokio::test]
async fn gis_map_proposal_routes_fail_closed_without_a_trusted_map_binding() {
    let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture");
    let unavailable = fixture["errors"].as_array().expect("error vocabulary").iter().find(|row| row["name"] == "no-binding").expect("no-binding row");
    let state = test_state().await;
    assert!(state.inference_runtime.is_none(), "production today has no trusted GIS Map profile");
    assert!(state.gis_map_binding.is_none());
    let session = issue_test_session(&state, "inference-owner@example.test").await;
    let addr = spawn_server(state).await;
    let readiness: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, "/readyz", &[]).await.body).expect("readiness body");
    assert_eq!(readiness["features"]["inferenceServices"], serde_json::json!([]), "readiness publishes a hub-executed inference service only with a frozen binding");
    let bearer = format!("Bearer {}", session.token);
    let base = "/spaces/space-a/documents/document-a/inference/gis-map/jobs";
    let intent = serde_json::json!({ "schema": "semio.hub.inference-request/v1", "version": 1, "requestId": "11111111111111111111111111111111", "serviceId": "s.gis.gismap.inference", "policyVersion": 1, "lifetimeMs": 60_000 }).to_string();
    let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": fixture["sampleJobId"], "proposalHash": fixture["proposalHash"] }).to_string();
    let job = fixture["sampleJobId"].as_str().expect("sample job");
    let calls: [(&str, String, &str); 4] =
        [("POST", base.to_string(), intent.as_str()), ("GET", format!("{base}/{job}/events?after=0"), ""), ("POST", format!("{base}/{job}/cancel"), ""), ("POST", format!("{base}/{job}/approval"), approval.as_str())];
    for (method, path, body) in &calls {
        for headers in [vec![("Authorization", bearer.as_str()), ("Content-Type", "application/json")], vec![("Content-Type", "application/json")]] {
            let response = raw_http_request(addr, method, path, &headers, body.as_bytes()).await;
            assert_eq!(u64::from(response.status), unavailable["status"].as_u64().expect("status"), "{method} {path} must fail closed");
            let published: serde_json::Value = serde_json::from_slice(&response.body).expect("closed error body");
            assert_eq!(published["schema"], "semio.hub.inference-error/v1");
            assert_eq!(published["code"], unavailable["code"], "{method} {path}");
            assert_eq!(published.as_object().expect("closed error object").len(), 2, "the closed error body never names a private object");
        }
    }
}

/// 🗺️ Builds a real trusted GIS Map editor profile, ledger and runtime on a live `HubState`.
#[cfg(feature = "integration-fixtures")]
struct GisMapInferenceFixture {
    state: HubState,
    profile: semio_hub_trusted_catalog_fixtures::VerifiedGisMapIntegrationProfileV1,
    space_id: String,
    document_id: String,
    snapshot_pack: Vec<u8>,
    ledger_path: std::path::PathBuf,
}

#[cfg(feature = "integration-fixtures")]
fn gis_map_test_snapshot() -> semio_s_artifact_gis_gismap::GisMapSnapshot {
    use directory::DslValue;
    use semio_s_artifact_gis_gismap::{GisMapSnapshot, MapFeature};
    let point = |lon: f64, lat: f64| DslValue::object([("lon".into(), DslValue::float(lon)), ("lat".into(), DslValue::float(lat))]);
    let pair = |lon: f64, lat: f64| DslValue::Array(vec![DslValue::float(lon), DslValue::float(lat)]);
    GisMapSnapshot {
        positions: vec![MapFeature { id: "point-a".into(), data: point(7.0, 47.0) }],
        routes: vec![MapFeature { id: "route-a".into(), data: DslValue::object([("points".into(), DslValue::Array(vec![pair(8.0, 46.0), pair(9.0, 48.0)]))]) }],
        regions: Vec::new(),
        ..Default::default()
    }
}

/// 🧭️ Seeds one space, one Author, one Spectator, a GIS Map document and its verified checkpoint.
#[cfg(feature = "integration-fixtures")]
async fn gis_map_inference_fixture(author_email: &str, spectator_email: &str) -> (GisMapInferenceFixture, TestIssuedSession, TestIssuedSession) {
    use semio_hub_trusted_catalog_fixtures as trusted_catalog_fixture;
    let mut state = test_state().await;
    let profile = trusted_catalog_fixture::verified_gis_map_integration_profile(&trusted_catalog_fixture::unique_profile_root("routes")).await.expect("real GIS Map editor profile");
    let ledger_path = tempdir("inference").join("jobs.sqlite3");
    let ledger = semio_hub_inference_ledger::InferenceJobLedgerV1::open(&ledger_path).expect("private job ledger");
    state.verified_catalog = Some(profile.catalog().clone());
    state.gis_map_binding = Some(profile.binding().clone());
    state.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(profile.binding().clone(), Arc::new(ledger), Arc::new(UnavailableGisMapApprovalCommitterV1), Arc::new(GisMapInferenceBindingV1))));
    let author = issue_test_session(&state, author_email).await;
    let spectator = issue_test_session(&state, spectator_email).await;
    let space_id = create_space_for_test(&state, &author.user_id, "GIS Map Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, author_email, DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space_id, spectator_email, DirectorySpaceRole::Spectator).await;
    let document_id = artifact_document_id_for_test("gis-map-inference");
    let selection = profile.binding().selection();
    let genesis_pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&semio_s_artifact_gis_gismap::GisMapSnapshot::default());
    let genesis_spr = b"gis-map-genesis-spr";
    let descriptor = os_directory::DocumentDescriptor {
        space_id: space_id.clone(),
        document_id: document_id.clone(),
        artifact_kind: selection.artifact.kind.clone(),
        artifact_schema: selection.artifact.schema.clone(),
        owner: os_directory::DocumentOwner { plugin_id: selection.package.plugin_id.clone(), package_id: selection.package.package_id.clone(), version: selection.package.version.clone(), package_hash: selection.package.component_sha256.clone() },
        pack_schema_hash: selection.artifact.pack_schema_hash.clone(),
        bootstrap_version: 1,
        bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&genesis_pack)),
    };
    let authenticated = state.directory.authenticate_session(&SessionCapability::parse(&author.token).expect("GIS Map author capability")).await.expect("GIS Map author session read").expect("GIS Map author session");
    let actor = ArtifactCreationActorV1 { user_id: author.user_id.clone(), session_id: authenticated.id, authorization_generation: authenticated.authorization_generation };
    let genesis = publish_genesis_checkpoint_for_test(&state, actor, profile.catalog().generation_id().to_string(), selection.parent_dialect.clone(), descriptor, &genesis_pack, genesis_spr).await;
    let snapshot_pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&gis_map_test_snapshot());
    publish_gis_checkpoint_for_test(&state, &space_id, &document_id, &snapshot_pack, &genesis).await;
    (GisMapInferenceFixture { state, profile, space_id, document_id, snapshot_pack, ledger_path }, author, spectator)
}

/// 🧾️ Publishes one verified active checkpoint whose pack is the literal GIS Map snapshot bytes.
#[cfg(feature = "integration-fixtures")]
async fn publish_gis_checkpoint_for_test(state: &HubState, space_id: &str, document_id: &str, pack: &[u8], genesis: &os_directory::ArtifactCheckpoint) {
    let spr = b"gis-map-spr";
    let pack_hash = os_directory::ArtifactHash(Sha256::digest(pack));
    let spr_hash = os_directory::ArtifactHash(Sha256::digest(spr));
    let mut aggregate = Sha256::new();
    aggregate.update(pack);
    aggregate.update(spr);
    let scope = DocumentScope::new(space_id, document_id);
    let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor read").expect("announced GIS Map descriptor");
    let pack_plan = prepare_artifact_cas_manifest_v1(space_id, pack).expect("pack manifest plan");
    let spr_plan = prepare_artifact_cas_manifest_v1(space_id, spr).expect("SPR manifest plan");
    let mut checkpoint = os_directory::ArtifactCheckpoint {
        scope: scope.clone(),
        checkpoint_id: os_directory::ArtifactHash([0; 32]),
        parent_checkpoint_id: Some(genesis.checkpoint_id),
        descriptor_digest_v1: os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
        baseline_frontier: os_directory::ArtifactFrontier { document_id: document_id.to_string(), head_edit_ordinal: 1, head_edit_id: "gis-map-edit-1".into(), last_commit_seq: 1, chain_hash: os_directory::ArtifactHash([0x44; 32]) },
        pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: artifact_cas_manifest_locator_v1(pack_plan.manifest_id) },
        spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: artifact_cas_manifest_locator_v1(spr_plan.manifest_id) },
        aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
        published_at_ms: genesis.published_at_ms.saturating_add(1),
    };
    checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("checkpoint identity")));
    let ownership = prepare_artifact_cas_ownership_v1(&checkpoint, &ArtifactPair { pack: pack.to_vec(), spr: spr.to_vec() }).expect("ownership plan");
    let system = DirectoryActor { kind: DirectoryActorKind::System, id: "system:gis-map-inference-test".into() };
    let reservation = state.directory_service.reserve_artifact_cas(system.clone(), ownership, 1_000, 100).await.expect("reserve checkpoint objects");
    let cas = ArtifactChunkBlobStore::new(state.artifact_cas.clone());
    let control = StartupCatalogControl::silent();
    let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
    cas.stage(space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &context).await.expect("stage GIS Map pack");
    cas.stage(space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &context).await.expect("stage GIS Map SPR");
    state.directory_service.publish_reserved_artifact_checkpoint(system, checkpoint, reservation, 100).await.expect("publish verified GIS Map checkpoint");
}

#[cfg(feature = "integration-fixtures")]
fn inference_route(space_id: &str, document_id: &str, suffix: &str) -> String {
    format!("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs{suffix}")
}

#[cfg(feature = "integration-fixtures")]
fn inference_intent(request_id: &str) -> String {
    serde_json::json!({ "schema": "semio.hub.inference-request/v1", "version": 1, "requestId": request_id, "serviceId": "s.gis.gismap.inference", "policyVersion": 1, "lifetimeMs": 120_000 }).to_string()
}

#[cfg(feature = "integration-fixtures")]
async fn wait_for_inference_state(addr: SocketAddr, space_id: &str, document_id: &str, job_id: &str, headers: &[(&str, &str)], expected: &str) -> serde_json::Value {
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let response = raw_http_get(addr, &inference_route(space_id, document_id, &format!("/{job_id}/events?after=0")), headers).await;
            assert_eq!(response.status, 200, "inference event poll failed: {}", String::from_utf8_lossy(&response.body));
            let page: serde_json::Value = serde_json::from_slice(&response.body).expect("inference event page");
            if page["state"] == expected {
                return page;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("inference reaches its bounded terminal state")
}

#[cfg(feature = "integration-fixtures")]
fn proposal_fixture() -> serde_json::Value {
    serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture")
}

#[cfg(feature = "integration-fixtures")]
#[tokio::test]
async fn gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation() {
    let fixture = proposal_fixture();
    let (bound, author, _spectator) = gis_map_inference_fixture("map-owner@example.test", "map-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
    let checkpoint = Arc::new(semio_hub_inference_runtime::InferenceCheckpointTestGateV1::new());
    runtime.install_checkpoint_test_gate(checkpoint.clone()).expect("one actual codec checkpoint gate");
    let addr = spawn_server(bound.state.clone()).await;
    let bearer = format!("Bearer {}", author.token);
    let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
    let accepted = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("11111111111111111111111111111111").as_bytes()).await;
    assert_eq!(accepted.status, 200, "an Author with a bound Map may submit: {}", String::from_utf8_lossy(&accepted.body));
    let receipt: serde_json::Value = serde_json::from_slice(&accepted.body).expect("closed job receipt");
    assert_eq!(receipt["schema"], "semio.hub.inference-job-receipt/v1");
    assert_eq!(receipt["state"], "running", "the submit response returns before retained compute completes");
    assert_eq!(receipt["proposalState"], "none");
    assert_eq!(receipt["proposalHash"], serde_json::Value::Null);
    let job_id = receipt["jobId"].as_str().expect("server-minted job id").to_owned();
    tokio::time::timeout(std::time::Duration::from_secs(5), checkpoint.entered()).await.expect("real GIS codec reached its cancellation checkpoint");
    let page: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("owner event page");
    assert_eq!(page["schema"], "semio.hub.inference-job-events/v1");
    assert_eq!(page["stale"], false);
    assert_eq!(page["cancelRequested"], false);
    let kinds: Vec<&str> = page["events"].as_array().expect("events").iter().map(|row| row["kind"].as_str().expect("kind")).collect();
    assert_eq!(kinds, vec!["accepted", "running"], "the private stream exposes Running while actual compute is paused");
    let cursors: Vec<u64> = page["progress"].as_array().expect("progress").iter().map(|row| row["cursor"].as_u64().expect("cursor")).collect();
    assert!(!cursors.is_empty(), "the test-only pause is downstream of one durable progress heartbeat");
    assert!(cursors.windows(2).all(|pair| pair[1] == pair[0] + 1), "the progress cursor is monotonic and dense: {cursors:?}");
    assert!(cursors.len() as u64 <= fixture["limits"]["progressMaxCursor"].as_u64().expect("cursor bound"), "progress is bounded");
    assert_eq!(page["nextCursor"].as_u64().expect("next cursor"), cursors.last().copied().unwrap_or(0));
    let persisted = page["progress"].as_array().expect("progress").last().expect("persisted progress");
    assert!(persisted["completed"].as_u64().is_some_and(|value| value > 0));
    assert!(persisted["total"].as_u64().is_some_and(|total| persisted["completed"].as_u64().is_some_and(|completed| completed <= total)));
    let cancelled: serde_json::Value = serde_json::from_slice(&raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/cancel")), &headers, &[]).await.body).expect("cancel page");
    checkpoint.release();
    assert_eq!(cancelled["cancelRequested"], true, "cancellation is durably requested before any terminal effect");
    assert_eq!(cancelled["proposalState"], "none", "a cancelled running job never publishes a private proposal");
    assert_eq!(cancelled["proposalHash"], serde_json::Value::Null, "no private proposal survives cancellation");
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if runtime.retained_operation_count_for_test().await.expect("retained worker count") == 0 {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("the cancelled blocking worker is joined and released");
    let after: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("retired page");
    assert!(after["events"].as_array().expect("events").iter().any(|row| row["kind"] == "cancel-requested"));
    assert_eq!(after["events"].as_array().expect("events").iter().filter(|row| row["kind"] == "cancelled").count(), 1, "late codec completion cannot append a second terminal");
    let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": "9".repeat(64) }).to_string();
    let denied = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, approval.as_bytes()).await;
    assert_eq!(denied.status, 409, "a cancelled offer can never be approved afterwards");
}

#[cfg(feature = "integration-fixtures")]
#[tokio::test]
async fn inference_reconciliation_route_is_existing_only_reader_bound_and_body_bounded() {
    let (bound, author, spectator) = gis_map_inference_fixture("reconcile-owner@example.test", "reconcile-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
    let checkpoint = Arc::new(semio_hub_inference_runtime::InferenceCheckpointTestGateV1::new());
    runtime.install_checkpoint_test_gate(checkpoint.clone()).expect("one actual codec checkpoint gate");
    let addr = spawn_server(bound.state.clone()).await;
    let bearer = format!("Bearer {}", author.token);
    let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
    let request_id = "15151515151515151515151515151515";
    let accepted = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent(request_id).as_bytes()).await;
    assert_eq!(accepted.status, 200, "the fixture Author reaches actual acceptance");
    let receipt: serde_json::Value = serde_json::from_slice(&accepted.body).expect("closed job receipt");
    tokio::time::timeout(std::time::Duration::from_secs(5), checkpoint.entered()).await.expect("actual codec pauses after acceptance");
    let reconcile = |request_id: &str| serde_json::json!({ "schema": "semio.framework.job-reconcile/v1", "version": 1, "requestId": request_id }).to_string();

    let found = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, "/reconcile"), &headers, reconcile(request_id).as_bytes()).await;
    assert_eq!(found.status, 200, "the original live reader may reconcile");
    let found: serde_json::Value = serde_json::from_slice(&found.body).expect("closed reconciliation result");
    assert_eq!(found["schema"], "semio.framework.job-reconcile-result/v1");
    assert_eq!(found["requestId"], request_id);
    assert_eq!(found["found"], true);
    assert_eq!(found["job"]["receipt"]["jobId"], receipt["jobId"]);
    assert_eq!(found["job"]["page"]["jobId"], receipt["jobId"]);
    assert_eq!(found["job"]["approval"], serde_json::Value::Null);

    let missing = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, "/reconcile"), &headers, reconcile(&"16".repeat(16)).as_bytes()).await;
    assert_eq!(missing.status, 200, "an absent exact request is observable without creating work");
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&missing.body).expect("missing result"),
        serde_json::json!({ "schema": "semio.framework.job-reconcile-result/v1", "version": 1, "requestId": "16".repeat(16), "found": false, "job": null })
    );
    let spectator_bearer = format!("Bearer {}", spectator.token);
    let spectator_headers = [("Authorization", spectator_bearer.as_str()), ("Content-Type", "application/json")];
    let denied = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, "/reconcile"), &spectator_headers, reconcile(request_id).as_bytes()).await;
    assert_eq!(denied.status, 403, "a Spectator cannot reconcile even an absent private prefix");
    let anonymous = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, "/reconcile"), &[("Content-Type", "application/json")], reconcile(request_id).as_bytes()).await;
    assert_eq!(anonymous.status, 403, "anonymous reconciliation is denied");
    let malformed = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, "/reconcile"), &headers, br#"{"schema":"semio.framework.job-reconcile/v1"}"#).await;
    assert_eq!(malformed.status, 400, "a noncanonical request is rejected");
    let oversized = vec![b' '; semio_framework_job::reconcile::JOB_RECONCILE_REQUEST_MAX_BYTES + 1];
    let oversized = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, "/reconcile"), &headers, &oversized).await;
    assert_eq!(oversized.status, 413, "the route enforces the schema-owned byte bound before decoding");
    let other_space = create_space_for_test(&bound.state, &author.user_id, "Other Reconciliation Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let cross_scope = raw_http_request(addr, "POST", &inference_route(&other_space, &document_id, "/reconcile"), &headers, reconcile(request_id).as_bytes()).await;
    assert_eq!(cross_scope.status, 200, "a live Author learns only that its distinct exact scope has no such request");
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&cross_scope.body).expect("cross-scope result")["found"], false);

    checkpoint.release();
    let job_id = receipt["jobId"].as_str().expect("job id");
    let _ = wait_for_inference_state(addr, &space_id, &document_id, job_id, &headers, "succeeded").await;
    bound.state.directory.revoke_auth_sessions_for_user(&author.user_id, "test-revocation", None, "gis-map-reconciliation-test").await.expect("revoke original session");
    let revoked = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, "/reconcile"), &headers, reconcile(request_id).as_bytes()).await;
    assert_eq!(revoked.status, 403, "the exact original reader loses reconciliation after session revocation");
}

#[cfg(feature = "integration-fixtures")]
#[tokio::test]
async fn gis_map_inference_runtime_close_signals_and_joins_actual_codec_work() {
    let (bound, author, _spectator) = gis_map_inference_fixture("close-owner@example.test", "close-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
    let checkpoint = Arc::new(semio_hub_inference_runtime::InferenceCheckpointTestGateV1::new());
    runtime.install_checkpoint_test_gate(checkpoint.clone()).expect("one actual codec checkpoint gate");
    let addr = spawn_server(bound.state.clone()).await;
    let bearer = format!("Bearer {}", author.token);
    let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
    let submitted = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("12121212121212121212121212121212").as_bytes()).await;
    assert_eq!(submitted.status, 200);
    let receipt: serde_json::Value = serde_json::from_slice(&submitted.body).expect("running job receipt");
    assert_eq!(receipt["state"], "running");
    tokio::time::timeout(std::time::Duration::from_secs(5), checkpoint.entered()).await.expect("real GIS codec reached the shutdown checkpoint");
    assert_eq!(runtime.retained_operation_count_for_test().await.expect("retained worker count"), 1);
    tokio::time::timeout(std::time::Duration::from_secs(5), runtime.close()).await.expect("runtime close deadline").expect("runtime close");
    assert_eq!(runtime.retained_operation_count_for_test().await.expect("retained worker count"), 0, "close joins the outer and blocking worker before returning");
}

#[cfg(feature = "integration-fixtures")]
#[tokio::test]
async fn gis_map_inference_revocation_after_compute_refuses_late_publication() {
    let (bound, author, _spectator) = gis_map_inference_fixture("revoked-owner@example.test", "revoked-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
    let checkpoint = Arc::new(semio_hub_inference_runtime::InferenceCheckpointTestGateV1::new());
    runtime.install_checkpoint_test_gate(checkpoint.clone()).expect("one actual codec checkpoint gate");
    let session = match HubCapability::parse(&author.token).expect("session capability") {
        HubCapability::Session(capability) => bound.state.directory.authenticate_session(&capability).await.expect("session lookup").expect("live session"),
        _ => panic!("test issuer returned a non-session capability"),
    };
    let addr = spawn_server(bound.state.clone()).await;
    let bearer = format!("Bearer {}", author.token);
    let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
    let submitted = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("13131313131313131313131313131313").as_bytes()).await;
    assert_eq!(submitted.status, 200);
    let receipt: serde_json::Value = serde_json::from_slice(&submitted.body).expect("running job receipt");
    let job_id = receipt["jobId"].as_str().expect("job id").to_owned();
    tokio::time::timeout(std::time::Duration::from_secs(5), checkpoint.entered()).await.expect("real GIS codec reached the revocation checkpoint");
    let owner = semio_hub_inference_ledger::InferenceReaderV1 { user_id: &session.user_id, session_id: &session.id, authorization_generation: session.authorization_generation, space_id: &space_id, document_id: &document_id };
    let identity = runtime.ledger().identity_of(&job_id, &owner).expect("retained owner identity");
    bound.state.directory.revoke_auth_sessions_for_user(&author.user_id, "test-revocation", None, "gis-map-inference-test").await.expect("revoke the retained owner");
    checkpoint.release();
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            if runtime.retained_operation_count_for_test().await.expect("retained worker count") == 0 {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("revoked worker reaches terminal");
    let page = runtime.ledger().events(&job_id, &semio_hub_inference_runtime_contract::reader(&identity), 0, u64::try_from(now_ms()).expect("test clock")).expect("private terminal page");
    assert_eq!(page.state, semio_hub_inference::schema::InferenceJobStateV1::Cancelled);
    assert_eq!(page.proposal_state, semio_hub_inference::schema::InferenceProposalStateV1::None);
    assert!(page.proposal_hash.is_none(), "revocation before the final authority fence publishes no proposal");
}

#[cfg(feature = "integration-fixtures")]
#[tokio::test]
async fn gis_map_proposal_is_private_to_every_peer_spectator_and_stale_caller() {
    let fixture = proposal_fixture();
    let (bound, author, spectator) = gis_map_inference_fixture("private-owner@example.test", "private-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let peer = issue_test_session(&bound.state, "private-peer@example.test").await;
    upsert_member_for_test(&bound.state, &space_id, "private-peer@example.test", DirectorySpaceRole::Author).await;
    let other_space = create_space_for_test(&bound.state, &peer.user_id, "Other Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(bound.state.clone()).await;
    let owner_bearer = format!("Bearer {}", author.token);
    let owner_headers = [("Authorization", owner_bearer.as_str()), ("Content-Type", "application/json")];
    let accepted = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &owner_headers, inference_intent("22222222222222222222222222222222").as_bytes()).await;
    assert_eq!(accepted.status, 200, "{}", String::from_utf8_lossy(&accepted.body));
    let receipt: serde_json::Value = serde_json::from_slice(&accepted.body).expect("job receipt");
    let job_id = receipt["jobId"].as_str().expect("job id").to_owned();
    let terminal = wait_for_inference_state(addr, &space_id, &document_id, &job_id, &owner_headers, "succeeded").await;
    let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": terminal["proposalHash"] }).to_string();
    let peer_bearer = format!("Bearer {}", peer.token);
    let spectator_bearer = format!("Bearer {}", spectator.token);
    let denied_code = fixture["visibility"].as_array().expect("visibility").iter().find(|row| row["role"] == "peer-author-same-space").expect("peer row")["expectedCode"].clone();
    for (role, header) in [("peer-author-same-space", peer_bearer.as_str()), ("viewer", spectator_bearer.as_str())] {
        let headers = [("Authorization", header), ("Content-Type", "application/json")];
        for (method, suffix, body) in [("GET", format!("/{job_id}/events?after=0"), String::new()), ("POST", format!("/{job_id}/cancel"), String::new()), ("POST", format!("/{job_id}/approval"), approval.clone())] {
            let response = raw_http_request(addr, method, &inference_route(&space_id, &document_id, &suffix), &headers, body.as_bytes()).await;
            assert_eq!(response.status, 403, "{role} reached {method} {suffix}");
            let published: serde_json::Value = serde_json::from_slice(&response.body).expect("closed denial");
            assert_eq!(published["code"], denied_code, "{role} {method}");
            assert_eq!(published.as_object().expect("closed object").len(), 2, "a denial never names a private object");
        }
    }
    let cross = raw_http_request(addr, "GET", &inference_route(&other_space, &document_id, &format!("/{job_id}/events?after=0")), &[("Authorization", peer_bearer.as_str())], &[]).await;
    assert!(matches!(cross.status, 403 | 404 | 503), "a cross-space read never returns another space's job: {}", cross.status);
    assert_ne!(cross.status, 200);
    let anonymous = raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &[]).await;
    assert_eq!(anonymous.status, 403, "an unauthenticated caller is denied");
    bound.state.directory.revoke_auth_sessions_for_user(&author.user_id, "test-revocation", None, "gis-map-inference-test").await.expect("revoke the original owner sessions");
    let stale = raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &owner_headers).await;
    assert_eq!(stale.status, 403, "a revoked original session loses its own private stream");
    assert!(!bound.snapshot_pack.is_empty(), "the base Map pack the job froze is a real encoded snapshot");
}

#[cfg(feature = "integration-fixtures")]
#[tokio::test]
async fn gis_map_approval_stamps_one_create_region_and_rejects_every_frozen_drift() {
    let fixture = proposal_fixture();
    let (bound, author, _spectator) = gis_map_inference_fixture("approve-owner@example.test", "approve-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let addr = spawn_server(bound.state.clone()).await;
    let bearer = format!("Bearer {}", author.token);
    let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
    let running: serde_json::Value = serde_json::from_slice(&raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("33333333333333333333333333333333").as_bytes()).await.body).expect("job receipt");
    let job_id = running["jobId"].as_str().expect("job id").to_owned();
    assert_eq!(running["state"], "running");
    let receipt = wait_for_inference_state(addr, &space_id, &document_id, &job_id, &headers, "succeeded").await;
    let proposal_hash = receipt["proposalHash"].as_str().expect("offered hash").to_owned();
    let wrong = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": "9".repeat(64) }).to_string();
    let rejected = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, wrong.as_bytes()).await;
    assert_eq!(rejected.status, 409, "a substituted proposal hash is refused");
    let foreign_job = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": "4".repeat(32), "proposalHash": proposal_hash }).to_string();
    let mismatched = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, foreign_job.as_bytes()).await;
    assert_eq!(mismatched.status, 409, "the body's job id must equal the route's");
    let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": proposal_hash }).to_string();
    let unavailable = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, approval.as_bytes()).await;
    let expected = fixture["errors"].as_array().expect("errors").iter().find(|row| row["name"] == "no-composition-transaction").expect("commit-unavailable row");
    assert_eq!(u64::from(unavailable.status), expected["status"].as_u64().expect("status"), "{}", String::from_utf8_lossy(&unavailable.body));
    let published: serde_json::Value = serde_json::from_slice(&unavailable.body).expect("closed error");
    assert_eq!(published["code"], expected["code"], "a Map with composed children must fail closed, never auto-apply");
    let page: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("owner page");
    assert_eq!(page["proposalState"], "offered", "a refused publication never marks the proposal approved");
    let kinds: Vec<&str> = page["events"].as_array().expect("events").iter().map(|row| row["kind"].as_str().expect("kind")).collect();
    assert!(kinds.contains(&"approval-prepared"), "the outbox row is durably prepared before publication is attempted");
    assert!(!kinds.contains(&"approved"), "no committed-WAL witness, no approved event");
}

#[cfg(feature = "integration-fixtures")]
#[tokio::test]
async fn gis_map_approval_is_idempotent_across_duplicate_requests_and_restart() {
    let (bound, author, _spectator) = gis_map_inference_fixture("idempotent-owner@example.test", "idempotent-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
    let checkpoint = Arc::new(semio_hub_inference_runtime::InferenceCheckpointTestGateV1::new());
    runtime.install_checkpoint_test_gate(checkpoint.clone()).expect("one actual codec checkpoint gate");
    let addr = spawn_server(bound.state.clone()).await;
    let bearer = format!("Bearer {}", author.token);
    let headers = [("Authorization", bearer.as_str()), ("Content-Type", "application/json")];
    let first: serde_json::Value = serde_json::from_slice(&raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("55555555555555555555555555555555").as_bytes()).await.body).expect("job receipt");
    assert_eq!(first["state"], "running");
    tokio::time::timeout(std::time::Duration::from_secs(5), checkpoint.entered()).await.expect("real GIS codec reached the duplicate-submit gate");
    let repeated: serde_json::Value =
        serde_json::from_slice(&raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, ""), &headers, inference_intent("55555555555555555555555555555555").as_bytes()).await.body).expect("repeated job receipt");
    assert_eq!(first["jobId"], repeated["jobId"], "one scoped request id can only ever mint one job");
    assert_eq!(repeated["state"], "running", "the duplicate observes the installed owner instead of replacing it");
    assert_eq!(runtime.retained_operation_count_for_test().await.expect("retained worker count"), 1, "one idempotency identity owns exactly one retained worker");
    let job_id = first["jobId"].as_str().expect("job id").to_owned();
    checkpoint.release();
    let terminal = wait_for_inference_state(addr, &space_id, &document_id, &job_id, &headers, "succeeded").await;
    let approval = serde_json::json!({ "schema": "semio.hub.inference-approval/v1", "version": 1, "jobId": job_id, "proposalHash": terminal["proposalHash"] }).to_string();
    for attempt in 0..3 {
        let response = raw_http_request(addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, approval.as_bytes()).await;
        assert_eq!(response.status, 503, "attempt {attempt} must reach the same fail-closed publication boundary");
    }
    let page: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("owner page");
    let prepared = page["events"].as_array().expect("events").iter().filter(|row| row["kind"] == "approval-prepared").count();
    assert_eq!(prepared, 1, "three duplicate approvals reconcile to exactly one prepared envelope");
    assert_eq!(page["events"].as_array().expect("events").iter().filter(|row| row["kind"] == "succeeded").count(), 1, "the job succeeded exactly once");
    let mut restarted = bound.state.clone();
    let reopened = semio_hub_inference_ledger::InferenceJobLedgerV1::open(&bound.ledger_path).expect("the durable ledger reopens after a restart");
    restarted.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(bound.profile.binding().clone(), Arc::new(reopened), Arc::new(UnavailableGisMapApprovalCommitterV1), Arc::new(GisMapInferenceBindingV1))));
    let restarted_addr = spawn_server(restarted).await;
    let recovered: serde_json::Value = serde_json::from_slice(&raw_http_get(restarted_addr, &inference_route(&space_id, &document_id, &format!("/{job_id}/events?after=0")), &headers).await.body).expect("recovered owner page");
    let recovered_kinds: Vec<&str> = recovered["events"].as_array().expect("events").iter().map(|row| row["kind"].as_str().expect("kind")).collect();
    assert_eq!(recovered_kinds.iter().filter(|kind| **kind == "approval-prepared").count(), 1, "restart recovery finds exactly one prepared envelope");
    assert_eq!(recovered_kinds.iter().filter(|kind| **kind == "succeeded").count(), 1, "restart never re-executes an already-offered job");
    assert!(!recovered_kinds.contains(&"approved"), "restart never invents a committed witness");
    let after_restart = raw_http_request(restarted_addr, "POST", &inference_route(&space_id, &document_id, &format!("/{job_id}/approval")), &headers, approval.as_bytes()).await;
    assert_eq!(after_restart.status, 503, "approval after restart reaches the same fail-closed publication boundary");
}
//#endregion 💡️Inference

#[tokio::test]
async fn admin_page_routes_follow_declared_html_without_alias_files() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🚧️hub-boundaries/🔣️.json")).expect("hub boundary fixture");
    let contract = &fixture["adminPageRoutes"];
    let state = test_state().await;
    tokio::fs::create_dir_all(&state.admin_dir).await.expect("admin fixture directory");
    let html = contract["htmlUtf8"].as_str().expect("HTML bytes");
    let asset = contract["assetUtf8"].as_str().expect("asset bytes");
    tokio::fs::write(state.admin_dir.join(contract["htmlPath"].as_str().expect("HTML path")), html).await.expect("HTML fixture");
    tokio::fs::write(state.admin_dir.join(contract["assetPath"].as_str().expect("asset path")), asset).await.expect("asset fixture");
    for alias in contract["absentAliases"].as_array().expect("absent aliases") {
        assert!(!state.admin_dir.join(alias.as_str().expect("alias path")).exists());
    }
    let addr = spawn_server(state).await;
    for request in contract["requests"].as_array().expect("request cases") {
        let path = request["path"].as_str().expect("request path");
        let response = raw_http_get(addr, path, &[]).await;
        assert_eq!(u64::from(response.status), request["status"].as_u64().expect("status"), "{path}");
        match request["body"].as_str().expect("body kind") {
            "html" => {
                assert_eq!(response.body, html.as_bytes());
                assert!(response.headers.to_ascii_lowercase().contains("content-type: text/html; charset=utf-8"));
            }
            "asset" => {
                assert_eq!(response.body, asset.as_bytes());
                assert!(response.headers.to_ascii_lowercase().contains("content-type: text/javascript"));
            }
            "absent" => assert!(response.body.is_empty()),
            other => panic!("unknown fixture response {other}"),
        }
    }
    let missing = test_state().await;
    let addr = spawn_server(missing).await;
    assert_eq!(raw_http_get(addr, "/admin", &[]).await.status, 503);
}

#[tokio::test]
async fn extension_module_routes_accept_encoded_unicode_http_paths() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🚧️hub-boundaries/🔣️.json")).expect("hub boundary fixture");
    let contract = &fixture["extensionModuleRoutes"];
    let state = test_state().await;
    let extension_id = contract["extensionId"].as_str().expect("extension ID");
    let extension_dir = state.extensions_root.join(extension_id);
    tokio::fs::create_dir_all(&extension_dir).await.expect("extension fixture directory");
    let install = serde_json::json!({ "extensionId": extension_id });
    tokio::fs::write(extension_dir.join("install.json"), serde_json::to_vec(&install).expect("install metadata")).await.expect("install fixture");
    let asset = contract["assetUtf8"].as_str().expect("asset bytes");
    tokio::fs::write(extension_dir.join(contract["assetPath"].as_str().expect("asset path")), asset).await.expect("module fixture");
    let addr = spawn_server(state).await;
    for request in contract["requests"].as_array().expect("request cases") {
        let path = request["path"].as_str().expect("request path");
        let response = raw_http_get(addr, path, &[]).await;
        assert_eq!(u64::from(response.status), request["status"].as_u64().expect("status"), "{path}");
        match request["body"].as_str().expect("body kind") {
            "listing" => assert_eq!(serde_json::from_slice::<serde_json::Value>(&response.body).expect("listing JSON"), serde_json::json!({ "extensions": [install.clone()] })),
            "asset" => {
                assert_eq!(response.body, asset.as_bytes());
                assert!(response.headers.to_ascii_lowercase().contains("content-type: text/javascript"));
            }
            "absent" => assert!(response.body.is_empty()),
            other => panic!("unknown fixture response {other}"),
        }
    }
}

#[test]
fn canonical_pair_route_rejects_non_path_and_ambiguous_headers_before_work() {
    let session = format!("session.v1.{}.{}", "0".repeat(32), "1".repeat(64));
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::ACCEPT, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE.parse().expect("accept"));
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {session}").parse().expect("authorization"));
    assert!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers).is_ok());
    assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair?checkpoint=other".parse().expect("URI"), &headers), Err(StatusCode::BAD_REQUEST));
    headers.insert(axum::http::header::RANGE, "bytes=0-1".parse().expect("range"));
    assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::RANGE_NOT_SATISFIABLE));
    headers.remove(axum::http::header::RANGE);
    headers.insert(axum::http::header::ACCEPT, "application/octet-stream".parse().expect("accept"));
    assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::NOT_ACCEPTABLE));
    headers.insert(axum::http::header::ACCEPT, CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE.parse().expect("accept"));
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", "a".repeat(AUTH_TEXT_MAX_BYTES)).parse().expect("oversized authorization"));
    assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::UNAUTHORIZED));
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer invite.v1.{}.{}", "0".repeat(32), "1".repeat(64)).parse().expect("wrong capability kind"));
    assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::UNAUTHORIZED));
    headers.insert(axum::http::header::AUTHORIZATION, "Bearer session.v1.not-hex.not-hex".parse().expect("invalid capability grammar"));
    assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::UNAUTHORIZED));
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {session}").parse().expect("authorization"));
    headers.append(axum::http::header::AUTHORIZATION, "Bearer duplicate".parse().expect("duplicate"));
    assert_eq!(canonical_pair_request_admission(&"/spaces/s/documents/d/active-checkpoint/pair".parse().expect("URI"), &headers), Err(StatusCode::UNAUTHORIZED));
    assert!(!directory_access_permits(&AuthOutcome::Denied.access_roles(), DirectoryAccessActionV1::DocumentRead, Some("studio")));
    assert!(directory_access_permits(&AuthOutcome::ShareToken.access_roles(), DirectoryAccessActionV1::DocumentRead, Some("studio")));
}

#[tokio::test]
async fn canonical_pair_route_is_exact_member_or_share_and_emits_only_verified_public_pair() {
    let mut state = lag_test_state(1024, 256).await;
    let (document_id, _) = seed_genesis_document_for_test(&state, "canonical-pair-author@example.com", STUDIO, "canonical-pair-document").await;
    let document_id = document_id.as_str();
    let checkpoint = publish_checkpoint_for_test(&state, STUDIO, document_id).await;
    let member = issue_test_session(&state, "canonical-member@example.com").await;
    upsert_member_for_test(&state, STUDIO, "canonical-member@example.com", DirectorySpaceRole::Spectator).await;
    let scope = DocumentScope::new(STUDIO, document_id);
    let issued_share = state.directory.issue_share_token(&scope, 60, "canonical-pair-share").await.expect("share issue");
    let share_id = issued_share.record.id.clone();
    let share = issued_share.capability.expose_once();
    let outsider = issue_test_session(&state, "canonical-outsider@example.com").await;
    state.admin_subjects = Arc::from([AdminSubject { provider_digest: admin_provider_digest("test-verifier"), subject_digest: identity_subject_digest("test-verifier", "canonical-outsider@example.com").expect("admin subject digest") }]);
    let other_space = create_space_for_test(&state, &outsider.user_id, "Canonical Other", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    seed_genesis_document_for_test(&state, "canonical-pair-other-author@example.com", &other_space, "canonical-pair-document").await;
    publish_checkpoint_for_test(&state, &other_space, document_id).await;
    let public_space = create_space_for_test(&state, &outsider.user_id, "Canonical Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
    seed_genesis_document_for_test(&state, "canonical-pair-public-author@example.com", &public_space, "canonical-pair-document").await;
    publish_checkpoint_for_test(&state, &public_space, document_id).await;
    let (other_document, _) = seed_genesis_document_for_test(&state, "canonical-pair-author@example.com", STUDIO, "canonical-pair-other-document").await;
    let other_document = other_document.as_str();
    publish_checkpoint_for_test(&state, STUDIO, other_document).await;
    let addr = spawn_server(state.clone()).await;
    let path = format!("/spaces/{STUDIO}/documents/{document_id}/active-checkpoint/pair");
    let accept = [("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE)];
    let member_authorization = format!("Bearer {}", member.token);
    assert_eq!(raw_http_get(addr, &format!("{path}?checkpoint=other"), &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &member_authorization)]).await.status, 400);
    assert_eq!(raw_http_get(addr, &path, &[("Accept", "application/octet-stream"), ("Authorization", &member_authorization)]).await.status, 406);
    assert_eq!(raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Range", "bytes=0-1"), ("Authorization", &member_authorization)]).await.status, 416);
    assert_eq!(raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &member_authorization), ("Authorization", &member_authorization)]).await.status, 401);
    let public = raw_http_get(addr, &path, &accept).await;
    assert_eq!(public.status, 401);
    assert!(public.body.is_empty());
    let malformed = raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", "Bearer malformed")]).await;
    assert_eq!(malformed.status, 401);
    let public_fallback = raw_http_get(addr, &format!("/spaces/{public_space}/documents/{document_id}/active-checkpoint/pair"), &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", "Bearer malformed")]).await;
    assert_eq!(public_fallback.status, 401);
    let denied = raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {}", outsider.token))]).await;
    assert_eq!(denied.status, 401);
    assert!(denied.body.is_empty());
    let cross_space = raw_http_get(addr, &format!("/spaces/{other_space}/documents/{document_id}/active-checkpoint/pair"), &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {}", member.token))]).await;
    assert_eq!(cross_space.status, 401);
    let cross_document_share = raw_http_get(addr, &format!("/spaces/{STUDIO}/documents/{other_document}/active-checkpoint/pair"), &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {share}"))]).await;
    assert_eq!(cross_document_share.status, 401);

    for token in [&member.token, &share] {
        let response = raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {token}"))]).await;
        assert_eq!(response.status, 200);
        let lower_headers = response.headers.to_ascii_lowercase();
        assert!(lower_headers.contains(&format!("content-type: {CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE}")));
        assert!(lower_headers.contains("cache-control: private, no-store"));
        assert!(lower_headers.contains("vary: authorization"));
        assert!(lower_headers.contains("etag: \""));
        let verified = decode_canonical_checkpoint_pair(&response.body).expect("verified route body");
        assert_eq!(verified.selection.scope, scope);
        assert_eq!(verified.selection.active_checkpoint_id, checkpoint.checkpoint_id);
        assert_eq!(verified.pair().pack, b"verified-pack");
        assert_eq!(verified.pair().spr, b"verified-spr");
        assert!(!String::from_utf8_lossy(&response.body).contains("cas/v1"));
        assert!(!String::from_utf8_lossy(&response.body).contains("manifest"));
    }
    for authorization_checks_before_revoke in [1usize, 3, 4] {
        let checks = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let gate_checks = checks.clone();
        let mut revoked_state = state.clone();
        revoked_state.canonical_pair_authorization_gate = Some(Arc::new(move || gate_checks.fetch_add(1, std::sync::atomic::Ordering::SeqCst) < authorization_checks_before_revoke));
        let revoked_addr = spawn_server(revoked_state).await;
        let response = raw_http_get(revoked_addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &member_authorization)]).await;
        assert_eq!(response.status, 401);
        assert!(response.body.is_empty(), "revocation never publishes a partial framed body");
        assert_eq!(checks.load(std::sync::atomic::Ordering::SeqCst), authorization_checks_before_revoke + 1);
    }
    let mut missing_cas_state = state.clone();
    missing_cas_state.rebootstrap = Arc::new(VerifiedRebootstrapSource::new(state.directory.clone(), Arc::new(ArtifactChunkCasStores::Memory(MemoryArtifactChunkCasStorage::default()))));
    let missing_cas_addr = spawn_server(missing_cas_state).await;
    let missing = raw_http_get(missing_cas_addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &member_authorization)]).await;
    assert_eq!(missing.status, 409);
    assert!(missing.body.is_empty());
    state.directory.revoke_share_token(&scope, &share_id, "test-revoked", "canonical-pair-revoke").await.expect("revoke share");
    let revoked = raw_http_get(addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &format!("Bearer {share}"))]).await;
    assert_eq!(revoked.status, 401);
    assert!(revoked.body.is_empty());
}

#[tokio::test]
async fn canonical_pair_route_disconnect_deadline_and_progress_are_request_owned() {
    use tokio::io::AsyncWriteExt;

    let state = lag_test_state(1024, 256).await;
    let (document_id, _) = seed_genesis_document_for_test(&state, "canonical-lifecycle-author@example.com", STUDIO, "canonical-pair-lifecycle-document").await;
    let document_id = document_id.as_str();
    publish_checkpoint_for_test(&state, STUDIO, document_id).await;
    let member = issue_test_session(&state, "canonical-lifecycle@example.com").await;
    upsert_member_for_test(&state, STUDIO, "canonical-lifecycle@example.com", DirectorySpaceRole::Spectator).await;
    let path = format!("/spaces/{STUDIO}/documents/{document_id}/active-checkpoint/pair");
    let authorization = format!("Bearer {}", member.token);

    let progress_gate = Arc::new(TestCanonicalPairRequestGate::new(1));
    let mut progress_state = state.clone();
    progress_state.canonical_pair_request_gate = Some(progress_gate.clone());
    let progress_addr = spawn_server(progress_state).await;
    let response = raw_http_get(progress_addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &authorization)]).await;
    assert_eq!(response.status, 200);
    let progress_control = progress_gate.control();
    assert!(!progress_control.is_cancelled());
    assert!(!progress_control.is_active());
    let progress = progress_control.progress_snapshot();
    for stage in [
        RebootstrapProgressStage::Authorize,
        RebootstrapProgressStage::Metadata,
        RebootstrapProgressStage::VerifyPack,
        RebootstrapProgressStage::VerifySpr,
        RebootstrapProgressStage::StreamPack,
        RebootstrapProgressStage::StreamSpr,
        RebootstrapProgressStage::Ready,
    ] {
        assert_eq!(progress[canonical_pair_progress_index(stage)].expect("bounded route progress").stage, stage);
    }
    assert!(progress[canonical_pair_progress_index(RebootstrapProgressStage::Chunk)].is_none());

    let deadline_gate = Arc::new(TestCanonicalPairRequestGate::new(0));
    let mut deadline_state = state.clone();
    deadline_state.canonical_pair_request_gate = Some(deadline_gate.clone());
    deadline_state.canonical_pair_deadline_ms = Some(20);
    let deadline_addr = spawn_server(deadline_state).await;
    let deadline = raw_http_get(deadline_addr, &path, &[("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE), ("Authorization", &authorization)]).await;
    assert_eq!(deadline.status, 504);
    assert!(deadline.body.is_empty());
    let deadline_control = deadline_gate.control();
    assert!(deadline_control.is_cancelled());
    assert!(!deadline_control.is_active());
    assert_eq!(deadline_control.progress_snapshot(), [None; CANONICAL_PAIR_PROGRESS_STAGES]);

    let disconnect_gate = Arc::new(TestCanonicalPairRequestGate::new(0));
    let mut disconnect_state = state;
    disconnect_state.canonical_pair_request_gate = Some(disconnect_gate.clone());
    let disconnect_addr = spawn_server(disconnect_state).await;
    let mut stream = tokio::net::TcpStream::connect(disconnect_addr).await.expect("disconnect HTTP connect");
    let request = format!("GET {path} HTTP/1.1\r\nHost: {disconnect_addr}\r\nAccept: {CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE}\r\nAuthorization: {authorization}\r\nConnection: keep-alive\r\n\r\n");
    stream.write_all(request.as_bytes()).await.expect("disconnect HTTP write");
    let entered = tokio::time::timeout(std::time::Duration::from_secs(2), disconnect_gate.entered.acquire()).await.expect("disconnect admission deadline").expect("disconnect admission");
    entered.forget();
    let disconnect_control = disconnect_gate.control();
    let before_disconnect = disconnect_control.progress_snapshot();
    drop(stream);
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while disconnect_control.is_active() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("disconnect cancellation deadline");
    assert!(disconnect_control.is_cancelled());
    assert!(!disconnect_control.is_active());
    let after_disconnect = disconnect_control.progress_snapshot();
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    assert_eq!(before_disconnect, after_disconnect);
    assert_eq!(after_disconnect, disconnect_control.progress_snapshot());
}

/// 🪢️ A frontier-less hello resumes at the plan checkpoint's baseline exactly when the plan's client seeds from that
/// checkpoint's pair: a `closed-browser-actor` plan with an edited checkpoint answers the baseline on the hub's internal
/// key; the same plan without a browser actor, or with a genesis checkpoint, answers `None` (the tail from the start).
#[test]
fn a_frontierless_hello_resumes_at_the_checkpoint_its_client_seeds_from() {
    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let mut plan = document_open_plan_test_authority(&fixture);
    let db_id = db_artifact_id(&plan.scope);
    assert_eq!(plan_seeded_hello_frontier(Some(&plan), &db_id), None, "a plan without a browser actor keeps the whole tail");
    plan.browser_actor = os_directory::DocumentOpenBrowserActorV1::ClosedBrowserActor {
        schema: "semio.browser-actor/v1".into(),
        codegen_policy: "closed".into(),
        sha256: "a".repeat(64),
        source_component_sha256: "b".repeat(64),
        source_descriptor_byte_sha256: "c".repeat(64),
        policy_sha256: "d".repeat(64),
        import_interfaces: Vec::new(),
    };
    let baseline = plan.checkpoint.baseline_frontier.clone();
    assert!(baseline.head_edit_ordinal > 0);
    let resumed = plan_seeded_hello_frontier(Some(&plan), &db_id).expect("a seeding plan resumes at its checkpoint");
    assert_eq!(resumed, RuntimeFrontierSummary { document_id: db_id.clone(), head_edit_ordinal: baseline.head_edit_ordinal, head_edit_id: baseline.head_edit_id.clone(), last_commit_seq: baseline.last_commit_seq, chain_hash: baseline.chain_hash.0 });
    plan.checkpoint.baseline_frontier = directory::os_directory::ArtifactFrontier { document_id: plan.scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: directory::os_directory::ArtifactHash([0; 32]) };
    assert_eq!(plan_seeded_hello_frontier(Some(&plan), &db_id), None, "a genesis checkpoint keeps the tail from the start");
    assert_eq!(plan_seeded_hello_frontier(None, &db_id), None);
}
