
use super::*;
use directory::os_directory::{DirectoryCommandOutcomeV1, DirectoryCommandResultV1};
use protocol::{ArtifactId as WireArtifactId, Bootstrap};
use semio_framework_hash::Sha256;
use semio_hub::artifact_authority::checkpoint_id_encoding_v1;

struct StartupVerifier;

impl IdentityAssertionVerifier for StartupVerifier {
    fn verify<'a>(&'a self, _assertion: &'a semio_hub::directory::IdentityAssertion, _context: &'a semio_hub::directory::IdentityVerificationContext<'a>) -> semio_hub::directory::IdentityVerificationFuture<'a> {
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

    fn accept<'a>(&'a self, _control: &'a dyn IdentityVerificationControl) -> semio_hub::directory::LocalBootstrapAcceptFuture<'a> {
        Box::pin(async { Ok(None) })
    }

    fn issue<'a>(
        &'a self,
        _request: &'a semio_hub::directory::VerifiedLocalBootstrapRequest,
        _session: &'a semio_hub::directory::model::IssuedAuthSession,
        _context: &'a semio_hub::directory::IdentityVerificationContext<'a>,
    ) -> semio_hub::directory::LocalBootstrapIssueFuture<'a> {
        Box::pin(async { Ok(()) })
    }

    fn reject<'a>(&'a self, _request_id: &'a str, _code: semio_hub::directory::LocalBootstrapRejectCode, _context: &'a semio_hub::directory::IdentityVerificationContext<'a>) -> semio_hub::directory::LocalBootstrapTerminalFuture<'a> {
        Box::pin(async { Ok(()) })
    }

    fn cancel<'a>(&'a self, _request_id: &'a str) -> semio_hub::directory::LocalBootstrapTerminalFuture<'a> {
        Box::pin(async { Ok(()) })
    }

    fn shutdown<'a>(&'a self) -> semio_hub::directory::LocalBootstrapTerminalFuture<'a> {
        Box::pin(async { Ok(()) })
    }
}

#[test]
fn startup_auth_policy_fails_closed_without_owned_adapters() {
    let loopback = std::net::IpAddr::from([127, 0, 0, 1]);
    let public = std::net::IpAddr::from([0, 0, 0, 0]);
    let verifier: Arc<dyn IdentityAssertionVerifier> = Arc::new(StartupVerifier);
    let local: Arc<dyn LocalBootstrapTransport> = Arc::new(TestLocalBootstrap);
    let admin = AdminSubject { provider_digest: admin_provider_digest("oidc.example"), subject_digest: identity_subject_digest("oidc.example", "admin-subject").expect("admin digest") };
    assert!(validate_auth_startup(HubMode::Production, loopback, None, None, &[admin.clone()]).is_err());
    assert!(validate_auth_startup(HubMode::Production, loopback, Some(&verifier), None, &[]).is_err());
    assert!(validate_auth_startup(HubMode::Production, public, Some(&verifier), None, &[admin.clone()]).is_err());
    assert!(validate_auth_startup(HubMode::Production, loopback, Some(&verifier), None, &[admin]).is_ok());
    assert!(validate_auth_startup(HubMode::Development, loopback, None, None, &[]).is_err());
    assert!(validate_auth_startup(HubMode::Development, public, None, Some(&local), &[]).is_err());
    assert!(validate_auth_startup(HubMode::Development, loopback, None, Some(&local), &[]).is_ok());
}

#[test]
fn readiness_v1_is_redacted_and_never_claims_public_session_issuance() {
    let ready = hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, false, false);
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
    let partial = hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, false, true, true, false, false);
    assert_eq!(partial.status, "not-ready");
    assert!(partial.authentication.bootstrap_ready);
    assert!(!partial.artifact_authority.ready);
    assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), false, false, false, true, true, false, false).status, "not-ready");
    assert_eq!(hub_readiness(HubMode::Development, "loopback", ready.run_id.clone(), true, false, false, false, true, false, false).status, "not-ready");
    assert_eq!(hub_readiness(HubMode::Development, "network", ready.run_id, true, true, false, true, false, false, false).status, "not-ready");
}

#[tokio::test]
async fn artifact_cas_maintenance_checkpoint_reaches_tail_after_sixteen_requests() {
    let state = test_state().await;
    for index in 0..5 {
        let space_id = create_space_for_test(&state, "seed", &format!("CAS {index}"), os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let document_id = format!("cas-maintenance-{index}");
        announce_document_for_test(&state, &space_id, &document_id).await;
        publish_checkpoint_for_test(&state, &space_id, &document_id).await;
    }
    let control = StartupCatalogControl;
    let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let mut checkpoint = ArtifactCasMaintenanceCheckpoint::default();
    let mut requests = 0usize;
    let mut examined = 0u64;
    loop {
        let result = state.directory_service.sweep_artifact_cas(state.artifact_cas.as_ref(), checkpoint.request(false, 1), &context).await.expect("bounded maintenance page");
        requests += 1;
        examined += result.examined_objects;
        if checkpoint.accept(&result) {
            break;
        }
        assert!(requests < 128, "maintenance cursor converges");
    }
    assert!(requests > 16);
    assert!(examined > 16);
}
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{Message as WsMessage, client::IntoClientRequest};

/// @emoji 🏛️ The seeded space id every test routes against (see `SqliteDirectory::seed`).
const STUDIO: &str = "default";

#[cfg(feature = "native-artifact-execution")]
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

#[cfg(all(feature = "native-artifact-execution", feature = "test-support"))]
#[tokio::test]
async fn space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed() {
    use semio_hub::artifact_authority::creation::ArtifactCreationOperationV1;
    use semio_hub::artifact_authority::trusted_catalog::test_support;

    let profile = test_support::verified_gis_map_test_profile(&test_support::unique_profile_root("artifact-creation-http")).await.expect("verified GIS Map creation profile");
    let mut state = test_state().await;
    state.verified_catalog = Some(profile.catalog().clone());
    state.artifact_creation = Some(Arc::new(ArtifactCreationServiceV1::new(state.directory_service.clone(), profile.catalog().clone(), state.artifact_cas.clone())));
    let author = issue_test_session(&state, "creation-author@example.test").await;
    let peer = issue_test_session(&state, "creation-peer@example.test").await;
    let spectator = issue_test_session(&state, "creation-spectator@example.test").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Artifact Creation", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, "creation-author@example.test", DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space_id, "creation-peer@example.test", DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space_id, "creation-spectator@example.test", DirectorySpaceRole::Spectator).await;
    let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
    let route = format!("/spaces/{space_id}/artifact-creations");
    let author_bearer = format!("Bearer {}", author.token);
    let peer_bearer = format!("Bearer {}", peer.token);
    let spectator_bearer = format!("Bearer {}", spectator.token);
    let catalog = raw_http_request(addr, "GET", &route, &[("Authorization", author_bearer.as_str())], &[]).await;
    assert_eq!(catalog.status, 200);
    let catalog_source = std::str::from_utf8(&catalog.body).expect("creation catalog UTF-8");
    let catalog = SpaceArtifactCreationCatalogV1::parse_canonical_json(catalog_source).expect("canonical selected creation catalog");
    assert_eq!(catalog.space_id, space_id);
    assert_eq!(catalog.kinds.iter().map(|kind| kind.kind_id.as_str()).collect::<Vec<_>>(), vec!["s.gis.gismap"]);
    assert_eq!(raw_http_request(addr, "GET", &route, &[], &[]).await.status, 401);
    assert_eq!(raw_http_request(addr, "GET", &route, &[("Authorization", spectator_bearer.as_str())], &[]).await.status, 403);

    let request = SpaceArtifactCreateV1 { schema: "semio.hub.space-artifact-create/v1".into(), request_id: "1234567890abcdef1234567890abcdef".into(), kind_id: "s.gis.gismap".into(), name: "Shared Map".into() };
    let body = directory::os_pack::json::to_json_string(&request);
    let malformed = body.replacen("{", "{\"documentId\":\"caller-owned\",", 1);
    assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], malformed.as_bytes()).await.status, 400);
    let unknown = SpaceArtifactCreateV1 { request_id: "2234567890abcdef1234567890abcdef".into(), kind_id: "s.gis.unknown".into(), ..request.clone() };
    assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], directory::os_pack::json::to_json_string(&unknown).as_bytes()).await.status, 409);
    let first = raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], body.as_bytes());
    let duplicate = raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], body.as_bytes());
    let (first, duplicate) = tokio::join!(first, duplicate);
    assert!([200, 202].contains(&first.status) && [200, 202].contains(&duplicate.status), "exact concurrent duplicate never reports capacity or owns a second factory");
    for response in [&first, &duplicate] {
        let source = std::str::from_utf8(&response.body).expect("creation acceptance UTF-8");
        assert!(SpaceArtifactCreationStatusV1::parse_canonical_json(source).is_some(), "creation acceptance is canonical");
    }
    let facts = state.directory.read_artifact_creation(&author.user_id, &request.request_id).await.expect("durable creation facts");
    let operation = ArtifactCreationOperationV1::fold(&facts).expect("one durable creation operation");
    assert!(operation.intent.scope.document_id.strip_prefix("artifact-").is_some_and(artifact_creation_request_id_v1));
    let created_document_id = operation.intent.scope.document_id;

    let status_route = format!("{route}/{}", request.request_id);
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(30);
    let ready = loop {
        let response = raw_http_request(addr, "GET", &status_route, &[("Authorization", author_bearer.as_str())], &[]).await;
        assert_eq!(response.status, 200);
        let status = SpaceArtifactCreationStatusV1::parse_canonical_json(std::str::from_utf8(&response.body).expect("creation status UTF-8")).expect("canonical creation status");
        if status.phase == SpaceArtifactCreationPhaseV1::Ready {
            break status;
        }
        assert!(matches!(status.phase, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing | SpaceArtifactCreationPhaseV1::Indeterminate), "creation reached an unexpected terminal phase");
        assert!(tokio::time::Instant::now() < deadline, "actual native genesis did not become Ready");
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    };
    let ready_scope = DocumentScope::new(&space_id, &ready.ready.as_ref().expect("Ready coordinates").document_id);
    assert_eq!(ready_scope.document_id, created_document_id, "both concurrent requests retain one server-minted document");
    assert!(state.directory.get_document_descriptor(&ready_scope).await.expect("created descriptor read").is_some());
    assert!(state.directory.get_active_artifact_checkpoint(&ready_scope).await.expect("created checkpoint read").is_some_and(|checkpoint| checkpoint.baseline_frontier.is_genesis_for(&ready_scope)));
    assert_eq!(raw_http_request(addr, "GET", &status_route, &[("Authorization", peer_bearer.as_str())], &[]).await.status, 404, "another Author cannot read the private request key");
    let cancelled = raw_http_request(addr, "POST", &format!("{status_route}/cancel"), &[("Authorization", author_bearer.as_str())], &[]).await;
    assert_eq!(cancelled.status, 200);
    assert_eq!(SpaceArtifactCreationStatusV1::parse_canonical_json(std::str::from_utf8(&cancelled.body).expect("cancel status UTF-8")).expect("canonical cancel status").phase, SpaceArtifactCreationPhaseV1::Ready, "cancel cannot overwrite Ready");
    assert_eq!(state.artifact_creation_tasks.task_count(), 0);
    let _ = shutdown.send(());
    server.await.expect("creation HTTP server stop");
    state.artifact_creation_tasks.shutdown().await;
}

#[cfg(feature = "native-artifact-execution")]
#[tokio::test]
async fn trusted_catalog_startup_is_selected_only_by_the_server_owned_data_root() {
    let data_root = std::fs::canonicalize(tempdir("unconfigured-trusted-catalog")).expect("canonical fixture-owned data root");
    assert!(configured_artifact_authority(&data_root, Some(&NativeCodecProviderSetV1::linked())).await.expect("unconfigured authority").is_none());
    std::fs::remove_dir_all(data_root).expect("remove unconfigured trusted catalog fixture");
}

#[tokio::test]
async fn configured_catalog_without_a_native_provider_fails_closed() {
    let unconfigured = tempdir("unconfigured-headless-catalog");
    assert!(configured_artifact_authority(&unconfigured, None).await.expect("unconfigured headless authority").is_none());
    std::fs::create_dir_all(unconfigured.join("trusted-catalog")).expect("trusted catalog directory");
    std::fs::write(unconfigured.join("trusted-catalog/current.json"), b"{}\n").expect("configured current pointer");
    let error = match configured_artifact_authority(&unconfigured, None).await {
        Ok(_) => panic!("configured trusted catalog unexpectedly admitted without its native provider"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("requires the native-artifact-execution provider"));
    std::fs::remove_dir_all(unconfigured).expect("remove headless trusted catalog fixture");
}

#[cfg(feature = "native-artifact-execution")]
fn native_openable_stdio_bundle() -> std::path::PathBuf {
    let root = tempdir("native-openable-stdio");
    let stage = root.join("generation-stage");
    std::fs::create_dir_all(stage.join("components")).expect("stdio component directory");
    std::fs::create_dir_all(stage.join("descriptors")).expect("stdio descriptor directory");
    let component = b"abc";
    let component_sha256 = os_directory::hex_lower(&Sha256::digest(component));
    let component_blake3 = blake3::hash(component).to_hex().to_string();
    let receipts = semio_s_plugin_stdio::registry::native_codec_factory_receipts().expect("artifact-owned stdio receipts");
    let viewer = semio_framework_plugin::Viewer::builder(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_plugin::StandardId("rfc8259"), subset: semio_framework_plugin::SubsetId::ANY })
        .document(["semio", "stdio", "json"])
        .mode("view", semio_framework_plugin::LocalizedLabel::native("View", "Ansicht"), "eye")
        .default_mode_id("view")
        .window_kind_def(<semio_framework_plugin::app::TreeWindowKit as semio_framework_plugin::app::WindowKit>::window_kind())
        .build_definition();
    let mut manifest = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::new("stdio", "Stdio Fixture", receipts[0].package_version).manifest;
    manifest.artifact_kinds = semio_s_plugin_stdio::registry::native_codec_artifact_kinds();
    manifest.apps.push(viewer.clone());
    manifest.topic_contributions.push(semio_s_plugin_stdio::registry::native_artifact_catalog_contribution().expect("synthetic fixture retains full catalog semantics"));
    assert_eq!(manifest.artifact_kinds.len(), receipts.len(), "every descriptor artifact kind has one executable owner receipt");
    assert_eq!(viewer.id, "s.stdio.json@rfc8259/*#viewer", "synthetic JSON fixture keeps the canonical surface coordinate");
    let viewer_id = viewer.id.clone();
    let window_id = viewer.window_kinds.iter().find(|window| window.id == "framework.window.tree").expect("descriptor-owned JSON viewer window").id.clone();
    let descriptor = semio_framework::PackageDescriptor {
        descriptor_version: 1,
        package_id: "semio:stdio".into(),
        role: semio_framework::PackageRole::Plugin,
        manifest,
        activation_events: Vec::new(),
        capability_requests: Vec::new(),
        extension_points: Vec::new(),
        execution: semio_framework::ExecutionMode::Isolated,
        execution_protocol: semio_framework::ExecutionProtocol { app_channel_version: directory::os_spr::CHANNEL_VERSION },
        quotas: semio_framework::kernel::QuotaSchema::default(),
        contributions: semio_framework::ContributionSet::default(),
        assets: Vec::new(),
        hashes: semio_framework::PackageHashes { wasm_sha256: component_sha256.clone(), core_wasm_sha256: "22".repeat(32), descriptor_sha256: "33".repeat(32) },
    };
    let descriptor_bytes = directory::os_store::pack_rt::encode_wire_value(&semio_framework::to_dsl_value(&descriptor).expect("project stdio descriptor"));
    let descriptor_sha256 = os_directory::hex_lower(&Sha256::digest(&descriptor_bytes));
    let json = receipts.iter().find(|receipt| receipt.factory_id == "stdio.native.json.v1").expect("JSON receipt");
    let native_codecs = receipts
        .iter()
        .map(|receipt| {
            serde_json::json!({
                "artifactKind": receipt.artifact_kind,
                "artifactSchema": receipt.schema,
                "packSchemaHash": os_directory::hex_lower(&receipt.pack_schema_hash)
            })
        })
        .collect::<Vec<_>>();
    let version = receipts[0].package_version;
    let target = serde_json::json!({
        "artifactKind": json.artifact_kind,
        "artifactSchema": json.schema,
        "packSchemaHash": os_directory::hex_lower(&json.pack_schema_hash),
        "surfaceId": viewer_id,
        "appId": viewer.id,
        "windowKindId": window_id,
        "role": "viewer",
        "rendererTarget": "wasm",
        "parentDialect": {
            "artifactKind": viewer.dialect.artifact_kind,
            "standard": viewer.dialect.standard,
            "subset": viewer.dialect.subset
        },
        "grant": { "read": true, "write": false, "observe": true }
    });
    let mut bundle = serde_json::json!({
        "schemaVersion": 2,
        "profiles": [{
            "id": "stdio-native-openable-v1",
            "selectedClosure": [{ "pluginId": "stdio", "packageId": "semio:stdio", "version": version }],
            "selectedClosureSha256": "11".repeat(32),
            "openTarget": {
                "package": { "pluginId": "stdio", "packageId": "semio:stdio", "version": version },
                "target": target
            },
            "generationId": "22".repeat(32)
        }],
        "packages": [{
            "pluginId": "stdio",
            "packageId": "semio:stdio",
            "version": version,
            "role": "plugin",
            "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
            "dependencies": [],
            "component": {
                "path": "components/stdio.wasm",
                "byteLength": component.len(),
                "sha256": component_sha256,
                "blake3": component_blake3
            },
            "descriptor": {
                "path": "descriptors/stdio.descriptor.semio",
                "byteLength": descriptor_bytes.len(),
                "sha256": descriptor_sha256
            },
            "nativeCodecs": native_codecs,
            "openTargets": [target]
        }]
    });
    bundle["packages"][0]["browserActor"] = serde_json::json!({
        "kind":"closed-browser-actor", "schema":"semio.os.closed-browser-actor.v1", "codegenPolicy":"semio.os.browser-jco-1.27.0-jspi.v1",
        "path":"closed-actor.mjs", "byteLength":component.len(), "sha256":component_sha256,
        "sourceComponentSha256":component_sha256, "sourceDescriptorByteSha256":descriptor_sha256, "policySha256":"41".repeat(32), "importInterfaces":[]
    });
    std::fs::write(stage.join("closed-actor.mjs"), component).expect("synthetic actor, never executed");
    let carried = serde_json::to_vec(&bundle).expect("provisional stdio bundle");
    let (selected_closure_sha256, generation_id) = semio_hub::artifact_authority::trusted_catalog::trusted_profile_digests_json(&carried, "stdio-native-openable-v1").expect("stdio profile digests");
    bundle["profiles"][0]["selectedClosureSha256"] = selected_closure_sha256.into();
    bundle["profiles"][0]["generationId"] = generation_id.into();
    std::fs::write(stage.join("components/stdio.wasm"), component).expect("write stdio component");
    std::fs::write(stage.join("descriptors/stdio.descriptor.semio"), descriptor_bytes).expect("write stdio descriptor");
    let bundle_bytes = serde_json::to_vec_pretty(&bundle).expect("stdio bundle json");
    std::fs::write(stage.join("trusted-catalog.json"), &bundle_bytes).expect("write stdio bundle");
    let trusted_root = root.join("trusted-catalog");
    let generations = trusted_root.join("generations");
    std::fs::create_dir_all(&generations).expect("trusted generation owner");
    let generation = bundle["profiles"][0]["generationId"].as_str().expect("generation id");
    std::fs::rename(stage, generations.join(generation)).expect("publish trusted generation");
    let bundle_sha256 = os_directory::hex_lower(&Sha256::digest(&bundle_bytes));
    let current_bytes = format!(
        r#"{{"profileId":"stdio-native-openable-v1","generationId":"{generation}","bundleSha256":"{bundle_sha256}","publicationRevision":"1"}}
"#
    )
    .into_bytes();
    std::fs::write(trusted_root.join("current.json"), current_bytes).expect("publish current pointer");
    std::fs::canonicalize(root).expect("canonical fixture-owned data root")
}

#[cfg(feature = "native-artifact-execution")]
#[tokio::test]
async fn native_openable_stdio_provider_is_the_only_atomic_readiness_transition() {
    let unavailable = test_state().await;
    let unavailable_addr = spawn_server(unavailable).await;
    let unavailable_readiness = raw_http_get(unavailable_addr, "/readyz", &[]).await;
    assert_eq!(unavailable_readiness.status, 503);
    let unavailable_json: serde_json::Value = serde_json::from_slice(&unavailable_readiness.body).expect("unavailable readiness JSON");
    assert_eq!(unavailable_json["artifactAuthority"]["ready"], false);
    assert_eq!(unavailable_json["features"]["openPlan"], false);

    let providers = NativeCodecProviderSetV1::linked();
    let root = native_openable_stdio_bundle();
    let configured = configured_artifact_authority(&root, Some(&providers)).await.expect("verified stdio authority").expect("configured stdio authority");
    assert_eq!(configured.catalog.codec_count(), 26);
    assert_eq!(configured.catalog.open_target_count(), 1);
    let mut ready = test_state().await;
    ready.openable_catalog = Some(configured.catalog.clone());
    ready.artifact_authority = Some(configured.authority);
    ready.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
    let ready_addr = spawn_server(ready).await;
    let readiness = raw_http_get(ready_addr, "/readyz", &[]).await;
    assert_eq!(readiness.status, 200);
    let readiness_json: serde_json::Value = serde_json::from_slice(&readiness.body).expect("ready JSON");
    assert_eq!(readiness_json["artifactAuthority"]["ready"], true);
    assert_eq!(readiness_json["features"]["openPlan"], true);
    assert_eq!(readiness_json["features"]["openPlanExchange"], true);
    let encoded = String::from_utf8(readiness.body).expect("readiness UTF-8");
    assert!(!encoded.contains("receipt"));
    assert!(!encoded.contains("factory"));
    std::fs::remove_dir_all(root).expect("remove stdio bundle fixture");
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

/// @emoji 📁️ A fresh, never-reused temp directory per call — the owned `time_ordered_id` rather than
/// `now_ms()` alone, since `cargo test` runs this whole module's `#[tokio::test]`s
/// concurrently within one process: two tests calling `test_state()` in the same millisecond
/// would otherwise collide on the identical `os-hub-test-db-<pid>-<ms>` path and open the SAME
/// `db::Database` storage root, corrupting each other's catalog/WAL state.
fn tempdir(name: &str) -> std::path::PathBuf {
    let mut dir = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(std::path::PathBuf::from).unwrap_or_else(std::env::temp_dir);
    dir.push(format!("os-hub-test-{name}-{}", directory::os_identity::time_ordered_id()));
    dir
}

fn run_socket_test<F, Fut>(test: F)
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = ()> + 'static,
{
    std::thread::Builder::new()
        .name("hub-socket-test".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(move || tokio::runtime::Builder::new_current_thread().enable_all().build().expect("test runtime").block_on(test()))
        .expect("socket test thread")
        .join()
        .expect("socket test");
}

async fn test_state() -> HubState {
    test_state_with_capacity(1024, 256).await
}

async fn test_state_with_capacity(directory_capacity: usize, fanout_capacity: usize) -> HubState {
    let dir = tempdir("db");
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
    test_state_with_directory(dir, directory, directory_capacity, fanout_capacity).await
}

async fn test_state_with_directory(dir: std::path::PathBuf, directory: SqliteDirectory, directory_capacity: usize, fanout_capacity: usize) -> HubState {
    let database = db::Database::open_at(hub_worker_pool(), &dir, db::Profile::Test).await.expect("open db");
    directory.seed().await.expect("seed");
    let directory: Arc<HubDirectories> = Arc::new(directory.into());
    let directory_service = Arc::new(DirectoryService::new(directory.clone(), directory_capacity));
    let database = Arc::new(database);
    let artifact_cas = Arc::new(ArtifactChunkCasStores::Filesystem(FsArtifactChunkCasStorage::open(&dir.join("artifact-cas/v1")).await.expect("open artifact CAS")));
    let control = StartupCatalogControl;
    let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let coordinator_id = directory.artifact_cas_coordinator_id().await.expect("artifact CAS coordinator");
    artifact_cas.configure_coordinator(coordinator_id, &context).await.expect("configure artifact CAS coordinator");
    let artifact_publication =
        Arc::new(CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(artifact_cas.clone()), HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority-test")));
    let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
    #[cfg(feature = "native-artifact-execution")]
    let socket_binding_gates = Arc::new(SocketBindingGatesV1::default());
    #[cfg(feature = "native-artifact-execution")]
    let artifact_creation_commit_authority = Arc::new(HubArtifactCreationCommitAuthorityV1 { directory: directory.clone(), gates: socket_binding_gates.clone() });
    HubState {
        db: database,
        artifact_cas,
        directory,
        rebootstrap,
        artifact_authority: None,
        verified_catalog: None,
        #[cfg(feature = "native-artifact-execution")]
        artifact_creation: None,
        #[cfg(feature = "native-artifact-execution")]
        artifact_creation_commit_authority,
        #[cfg(feature = "native-artifact-execution")]
        artifact_creation_tasks: Arc::new(ArtifactCreationHttpTaskOwnerV1::new()),
        #[cfg(feature = "native-artifact-execution")]
        gis_map_binding: None,
        #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
        inference_runtime: None,
        openable_catalog: None,
        artifact_publication,
        artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
        directory_service,
        admin_subjects: Arc::from([]),
        admin_cursor_key: [0x5a; 32],
        space_administration_cursor_key: [0xa5; 32],
        admin_operations: Arc::new(ShardedMap::new()),
        admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
        admin_operation_tasks: Arc::new(AdminOperationTaskOwner::new(ADMIN_OPERATION_SHUTDOWN_DEADLINE)),
        readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, false, true, true, false, false)),
        admin_dir: dir.join("admin-dist"),
        fanout: Arc::new(ShardedMap::new()),
        fanout_capacity,
        live_gate: None,
        canonical_pair_authorization_gate: None,
        canonical_pair_request_gate: None,
        canonical_pair_deadline_ms: None,
        document_open_plan_issue_gate: None,
        document_open_plan_deadline_ms: None,
        presence: Arc::new(ShardedMap::new()),
        presence_publication_gate: Arc::new(tokio::sync::Mutex::new(())),
        presence_clock: None,
        session_colors: Arc::new(ShardedMap::new()),
        session_kicks: Arc::new(ShardedMap::new()),
        socket_grants: Arc::new(SocketGrantLedgerV1::default()),
        document_open_plans: Arc::new(DocumentOpenPlanLedgerV1::default()),
        socket_binding_gates: {
            #[cfg(feature = "native-artifact-execution")]
            {
                socket_binding_gates
            }
            #[cfg(not(feature = "native-artifact-execution"))]
            {
                Arc::new(SocketBindingGatesV1::default())
            }
        },
        extensions_root: dir.join("extension-modules"),
        merge_policy: protocol::MergePolicy::default(),
    }
}

async fn lag_test_state(directory_capacity: usize, fanout_capacity: usize) -> HubState {
    let dir = tempdir("lag-db");
    let pool = hub_worker_pool();
    let backend = Arc::new(db::db_storage::DbBackend::Memory(db::db_storage::MemoryStorage::new(pool.clone()).await.expect("memory storage")));
    let database = Arc::new(db::Database::open(pool, db::DbConfig::for_profile(db::Profile::Test), backend).await.expect("open memory db"));
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
    directory.seed().await.expect("seed");
    let directory: Arc<HubDirectories> = Arc::new(directory.into());
    let directory_service = Arc::new(DirectoryService::new(directory.clone(), directory_capacity));
    let artifact_cas = Arc::new(ArtifactChunkCasStores::Memory(MemoryArtifactChunkCasStorage::default()));
    let control = StartupCatalogControl;
    let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let coordinator_id = directory.artifact_cas_coordinator_id().await.expect("artifact CAS coordinator");
    artifact_cas.configure_coordinator(coordinator_id, &context).await.expect("configure artifact CAS coordinator");
    let artifact_publication =
        Arc::new(CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(artifact_cas.clone()), HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority-test")));
    let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
    #[cfg(feature = "native-artifact-execution")]
    let socket_binding_gates = Arc::new(SocketBindingGatesV1::default());
    #[cfg(feature = "native-artifact-execution")]
    let artifact_creation_commit_authority = Arc::new(HubArtifactCreationCommitAuthorityV1 { directory: directory.clone(), gates: socket_binding_gates.clone() });
    HubState {
        db: database,
        artifact_cas,
        directory,
        rebootstrap,
        artifact_authority: None,
        verified_catalog: None,
        #[cfg(feature = "native-artifact-execution")]
        artifact_creation: None,
        #[cfg(feature = "native-artifact-execution")]
        artifact_creation_commit_authority,
        #[cfg(feature = "native-artifact-execution")]
        artifact_creation_tasks: Arc::new(ArtifactCreationHttpTaskOwnerV1::new()),
        #[cfg(feature = "native-artifact-execution")]
        gis_map_binding: None,
        #[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
        inference_runtime: None,
        openable_catalog: None,
        artifact_publication,
        artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
        directory_service,
        admin_subjects: Arc::from([]),
        admin_cursor_key: [0x5a; 32],
        space_administration_cursor_key: [0xa5; 32],
        admin_operations: Arc::new(ShardedMap::new()),
        admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
        admin_operation_tasks: Arc::new(AdminOperationTaskOwner::new(ADMIN_OPERATION_SHUTDOWN_DEADLINE)),
        readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, false, true, true, false, false)),
        admin_dir: dir.join("admin-dist"),
        fanout: Arc::new(ShardedMap::new()),
        fanout_capacity,
        live_gate: None,
        canonical_pair_authorization_gate: None,
        canonical_pair_request_gate: None,
        canonical_pair_deadline_ms: None,
        document_open_plan_issue_gate: None,
        document_open_plan_deadline_ms: None,
        presence: Arc::new(ShardedMap::new()),
        presence_publication_gate: Arc::new(tokio::sync::Mutex::new(())),
        presence_clock: None,
        session_colors: Arc::new(ShardedMap::new()),
        session_kicks: Arc::new(ShardedMap::new()),
        socket_grants: Arc::new(SocketGrantLedgerV1::default()),
        document_open_plans: Arc::new(DocumentOpenPlanLedgerV1::default()),
        socket_binding_gates: {
            #[cfg(feature = "native-artifact-execution")]
            {
                socket_binding_gates
            }
            #[cfg(not(feature = "native-artifact-execution"))]
            {
                Arc::new(SocketBindingGatesV1::default())
            }
        },
        extensions_root: dir.join("extension-modules"),
        merge_policy: protocol::MergePolicy::default(),
    }
}

/// @emoji 🏗️ Test-only `create-space` through `DirectoryService::execute` (the trait's own
/// `create_space` write method is gone — see `📓️w1-b-report.md`) — returns the minted space id.
/// `decide` performs zero authorization of its own, so `owner_user_id` need not be a real,
/// already-existing user for these low-level fixture setups.
async fn create_space_for_test(state: &HubState, owner_user_id: &str, name: &str, space_kind: os_directory::DirectorySpaceKind, visibility: DirectorySpaceVisibility) -> String {
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{owner_user_id}#test") };
    let (events, _) = state.directory_service.execute(actor, DirectoryCommand::CreateSpace { name: name.to_string(), space_kind, visibility }).await.expect("create space");
    events
        .into_iter()
        .find_map(|event| match event.body {
            os_directory::DirectoryEventBody::SpaceCreated { space_id, .. } => Some(space_id),
            _ => None,
        })
        .expect("space.created event")
}

/// @emoji 🏗️ Test-only `upsert-member` through `DirectoryService::execute` — `email` must match
/// an already-minted `AuthSessionRecord`'s user for the member to land on that SAME user rather
/// than a freshly-created one (`decide`'s `UpsertMember` resolves-or-creates by email).
async fn upsert_member_for_test(state: &HubState, space_id: &str, email: &str, role: DirectorySpaceRole) {
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".to_string() };
    state.directory_service.execute(actor, DirectoryCommand::UpsertMember { space_id: space_id.to_string(), email: email.to_string(), role }).await.expect("upsert member");
}

fn document_descriptor_for_test(space_id: &str, document_id: &str) -> os_directory::DocumentDescriptor {
    let bootstrap_snapshot_hash = os_directory::hex_lower(&Sha256::digest(b"document-open-genesis-pack"));
    os_directory::DocumentDescriptor {
        space_id: space_id.to_string(),
        document_id: document_id.to_string(),
        artifact_kind: "test.artifact".into(),
        artifact_schema: "test.v1".into(),
        owner: os_directory::DocumentOwner { plugin_id: "test.plugin".into(), package_id: "test.package".into(), version: "1.0.0".into(), package_hash: "22".repeat(32) },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash,
    }
}

fn artifact_document_id_for_test(label: &str) -> String {
    format!("artifact-{}", &os_directory::hex_lower(&Sha256::digest(label.as_bytes()))[..32])
}

async fn announce_document_for_test(state: &HubState, space_id: &str, document_id: &str) {
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".into() };
    state.directory_service.execute(actor, DirectoryCommand::AnnounceDocument { descriptor: document_descriptor_for_test(space_id, document_id) }).await.expect("announce document");
}

async fn publish_checkpoint_for_test(state: &HubState, space_id: &str, document_id: &str) -> os_directory::ArtifactCheckpoint {
    let pack = b"verified-pack";
    let spr = b"verified-spr";
    let pack_hash = os_directory::ArtifactHash(Sha256::digest(pack));
    let spr_hash = os_directory::ArtifactHash(Sha256::digest(spr));
    let accepted_at_ms = u64::try_from(now_ms()).expect("nonnegative genesis fixture clock");
    let mut aggregate = Sha256::new();
    aggregate.update(pack);
    aggregate.update(spr);
    let scope = DocumentScope::new(space_id, document_id);
    let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor read").expect("descriptor");
    let pack_plan = prepare_artifact_cas_manifest_v1(space_id, pack).expect("pack manifest plan");
    let spr_plan = prepare_artifact_cas_manifest_v1(space_id, spr).expect("SPR manifest plan");
    let mut checkpoint = os_directory::ArtifactCheckpoint {
        scope,
        checkpoint_id: os_directory::ArtifactHash([0; 32]),
        parent_checkpoint_id: None,
        descriptor_digest_v1: os_directory::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
        baseline_frontier: os_directory::ArtifactFrontier { document_id: document_id.to_string(), head_edit_ordinal: 1, head_edit_id: "verified-edit-1".into(), last_commit_seq: 1, chain_hash: os_directory::ArtifactHash([0x44; 32]) },
        pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: artifact_cas_manifest_locator_v1(pack_plan.manifest_id) },
        spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: artifact_cas_manifest_locator_v1(spr_plan.manifest_id) },
        aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
        published_at_ms: accepted_at_ms,
    };
    checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("checkpoint identity")));
    let ownership = prepare_artifact_cas_ownership_v1(&checkpoint, &ArtifactPair { pack: pack.to_vec(), spr: spr.to_vec() }).expect("ownership plan");
    let reservation = state.directory_service.reserve_artifact_cas(DirectoryActor { kind: DirectoryActorKind::System, id: "system:lag-rebootstrap-test".into() }, ownership, 1_000, 100).await.expect("reserve checkpoint objects");
    let cas = ArtifactChunkBlobStore::new(state.artifact_cas.clone());
    let authority_control = StartupCatalogControl;
    let authority_context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &authority_control);
    let staged_pack = cas.stage(space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &authority_context).await.expect("stage reserved pack manifest");
    let staged_spr = cas.stage(space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &authority_context).await.expect("stage reserved SPR manifest");
    assert_eq!(staged_pack.storage_key, checkpoint.pack.storage_key);
    assert_eq!(staged_spr.storage_key, checkpoint.spr.storage_key);
    state.directory_service.publish_reserved_artifact_checkpoint(DirectoryActor { kind: DirectoryActorKind::System, id: "system:lag-rebootstrap-test".into() }, checkpoint.clone(), reservation, 100).await.expect("publish verified checkpoint");
    checkpoint
}

async fn publish_genesis_checkpoint_for_test(
    state: &HubState,
    actor: ArtifactCreationActorV1,
    catalog_generation: String,
    parent_dialect: directory::os_io::ArtifactDialect,
    descriptor: DocumentDescriptor,
    pack: &[u8],
    spr: &[u8],
) -> os_directory::ArtifactCheckpoint {
    use semio_hub::artifact_authority::creation::{
        ARTIFACT_CREATION_DEADLINE_MS, ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactBodyV1, ArtifactCreationIntentV1, ArtifactCreationPreparedV1, artifact_creation_command_digest_v1,
    };
    let accepted_at_ms = 1;
    let scope = DocumentScope::new(&descriptor.space_id, &descriptor.document_id);
    let pack_hash = os_directory::ArtifactHash(Sha256::digest(pack));
    let spr_hash = os_directory::ArtifactHash(Sha256::digest(spr));
    let mut aggregate = Sha256::new();
    aggregate.update(pack);
    aggregate.update(spr);
    let pack_plan = prepare_artifact_cas_manifest_v1(&scope.space_id, pack).expect("genesis pack manifest plan");
    let spr_plan = prepare_artifact_cas_manifest_v1(&scope.space_id, spr).expect("genesis SPR manifest plan");
    let mut checkpoint = os_directory::ArtifactCheckpoint {
        scope: scope.clone(),
        checkpoint_id: os_directory::ArtifactHash([0; 32]),
        parent_checkpoint_id: None,
        descriptor_digest_v1: os_directory::descriptor_digest_v1(&descriptor).expect("genesis descriptor digest"),
        baseline_frontier: os_directory::ArtifactFrontier { document_id: scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: os_directory::ArtifactHash([0; 32]) },
        pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: artifact_cas_manifest_locator_v1(pack_plan.manifest_id) },
        spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: artifact_cas_manifest_locator_v1(spr_plan.manifest_id) },
        aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
        published_at_ms: 1,
    };
    checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("genesis checkpoint identity")));
    let request = SpaceArtifactCreateV1 {
        schema: "semio.hub.space-artifact-create/v1".into(),
        request_id: scope.document_id.strip_prefix("artifact-").expect("creation-owned document id").into(),
        kind_id: descriptor.artifact_kind.clone(),
        name: "Checkpoint publication fixture".into(),
    };
    let intent = ArtifactCreationIntentV1 {
        actor,
        scope: scope.clone(),
        command_sha256: artifact_creation_command_digest_v1(&scope.space_id, &request).expect("genesis creation digest"),
        request,
        catalog_generation,
        owner: descriptor.owner.clone(),
        artifact_schema: descriptor.artifact_schema.clone(),
        pack_schema_hash: descriptor.pack_schema_hash.clone(),
        parent_dialect,
        accepted_at_ms,
        deadline_ms: accepted_at_ms + ARTIFACT_CREATION_DEADLINE_MS,
    };
    let prepared = ArtifactCreationPreparedV1 { descriptor, checkpoint: checkpoint.clone(), pack: pack.to_vec(), spr: spr.to_vec() };
    prepared.validate(&intent).expect("exact prepared publication genesis");
    assert!(matches!(state.directory.claim_artifact_creation(&intent).await.expect("claim publication genesis"), ArtifactCreationClaimV1::Accepted(_)));
    state
        .directory
        .append_artifact_creation_fact(&ArtifactCreationFactAppendV1 {
            actor: intent.actor.clone(),
            space_id: scope.space_id.clone(),
            request_id: intent.request.request_id.clone(),
            command_sha256: intent.command_sha256.clone(),
            expected_revision: 1,
            recorded_at_ms: accepted_at_ms,
            body: ArtifactCreationFactBodyV1::Prepared { candidate: prepared.clone() },
        })
        .await
        .expect("prepare publication genesis");
    let pair = ArtifactPair { pack: pack.to_vec(), spr: spr.to_vec() };
    let ownership = prepare_artifact_cas_ownership_v1(&checkpoint, &pair).expect("genesis ownership plan");
    let reservation =
        state.directory_service.reserve_artifact_cas(DirectoryActor { kind: DirectoryActorKind::System, id: "system:checkpoint-publication-genesis-test".into() }, ownership, intent.deadline_ms, accepted_at_ms).await.expect("reserve genesis objects");
    let cas = ArtifactChunkBlobStore::new(state.artifact_cas.clone());
    let control = StartupCatalogControl;
    let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
    cas.stage(&scope.space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &context).await.expect("stage genesis pack");
    cas.stage(&scope.space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &context).await.expect("stage genesis SPR");
    state.directory_service.publish_document_genesis(intent, &prepared, checkpoint.clone(), reservation, accepted_at_ms).await.expect("publish dedicated creation genesis");
    checkpoint
}

async fn publish_openable_document_for_test(state: &HubState, token: &str, space_id: &str, document_id: &str) -> (DocumentDescriptor, os_directory::ArtifactCheckpoint) {
    let pack = b"document-open-genesis-pack";
    let spr = b"document-open-genesis-spr";
    let mut descriptor = document_descriptor_for_test(space_id, document_id);
    descriptor.bootstrap_snapshot_hash = os_directory::hex_lower(&Sha256::digest(pack));
    let session = state.directory.authenticate_session(&SessionCapability::parse(token).expect("document-open author capability")).await.expect("document-open author session read").expect("document-open author session");
    let actor = ArtifactCreationActorV1 { user_id: session.user_id, session_id: session.id, authorization_generation: session.authorization_generation };
    let parent_dialect = directory::os_io::ArtifactDialect { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() };
    let checkpoint = publish_genesis_checkpoint_for_test(state, actor, "66".repeat(32), parent_dialect, descriptor.clone(), pack, spr).await;
    (descriptor, checkpoint)
}

async fn sample_envelope(id: &str, document: &WireArtifactId) -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({ "value": id })).await.unwrap() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({})).await.unwrap() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
    }
}

#[cfg(feature = "native-artifact-execution")]
struct CheckpointPublicationFixture {
    state: HubState,
    author: TestIssuedSession,
    spectator: TestIssuedSession,
    scope: DocumentScope,
    handle: db::ArtifactHandle,
    command: CheckpointPublicationCommandV1,
    pack: Vec<u8>,
    spr: Vec<u8>,
    catalog_root: std::path::PathBuf,
}

#[cfg(feature = "native-artifact-execution")]
fn checkpoint_publication_command(correlation_id: &str, descriptor: &DocumentDescriptor, snapshot: &db::CheckpointPublicationSnapshot, expected_current: CheckpointPublicationCurrentV1, pack: &[u8], spr: &[u8]) -> CheckpointPublicationCommandV1 {
    let head_edit_id = snapshot.head_edit_id.as_ref().expect("committed checkpoint tip").0.clone();
    CheckpointPublicationCommandV1 {
        schema: "semio.hub.checkpoint-publication-command/v1".into(),
        correlation_id: correlation_id.into(),
        descriptor_digest_v1: descriptor_digest_v1(descriptor).expect("descriptor digest").hex(),
        expected_document_frontier: os_directory::DocumentFrontier { head_seq: snapshot.frontier.head_seq, commit_seq: snapshot.frontier.commit_seq, epoch: snapshot.frontier.epoch },
        expected_current,
        baseline_frontier: CheckpointPublicationFrontierV1 {
            document_id: descriptor.document_id.clone(),
            head_edit_ordinal: snapshot.frontier.head_seq,
            head_edit_id,
            last_commit_seq: snapshot.frontier.commit_seq,
            chain_sha256: os_directory::hex_lower(&snapshot.frontier.chain_hash),
        },
        pack: CheckpointPublicationBlobV1 { sha256: os_directory::hex_lower(&Sha256::digest(pack)), byte_length: pack.len() as u64 },
        spr: CheckpointPublicationBlobV1 { sha256: os_directory::hex_lower(&Sha256::digest(spr)), byte_length: spr.len() as u64 },
    }
}

#[cfg(feature = "native-artifact-execution")]
async fn checkpoint_publication_fixture(label: &str) -> CheckpointPublicationFixture {
    let catalog_root = native_openable_stdio_bundle();
    let providers = NativeCodecProviderSetV1::linked();
    let configured = configured_artifact_authority(&catalog_root, Some(&providers)).await.expect("load stdio publication catalog").expect("configured publication catalog");
    let selection = configured.catalog.selected_document_open().expect("selected stdio JSON target").clone();
    let mut state = test_state().await;
    let author = issue_test_session(&state, &format!("checkpoint-{label}-author@example.test")).await;
    let spectator = issue_test_session(&state, &format!("checkpoint-{label}-spectator@example.test")).await;
    let space_id = create_space_for_test(&state, &author.user_id, &format!("Checkpoint {label}"), os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, &format!("checkpoint-{label}-author@example.test"), DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space_id, &format!("checkpoint-{label}-spectator@example.test"), DirectorySpaceRole::Spectator).await;
    let request_id = os_directory::hex_lower(&Sha256::digest(format!("checkpoint-{label}").as_bytes()))[..32].to_string();
    let scope = DocumentScope::new(space_id, format!("artifact-{request_id}"));
    let descriptor = DocumentDescriptor {
        space_id: scope.space_id.clone(),
        document_id: scope.document_id.clone(),
        artifact_kind: selection.artifact.kind,
        artifact_schema: selection.artifact.schema,
        owner: os_directory::DocumentOwner { plugin_id: selection.package.plugin_id, package_id: selection.package.package_id, version: selection.package.version, package_hash: selection.package.component_sha256 },
        pack_schema_hash: selection.artifact.pack_schema_hash,
        bootstrap_version: 1,
        bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: String::new(),
    };
    state.artifact_authority = Some(configured.authority);
    let catalog_generation = configured.catalog.generation_id().to_string();
    let parent_dialect = selection.parent_dialect;
    state.openable_catalog = Some(configured.catalog);
    let document = db_artifact_id(&scope);
    let snapshot_value = semio_s_artifact_stdio_json::schema::snapshot::demo_json_snapshot();
    let pack = <semio_s_artifact_stdio_json::JsonSnapshot as directory::os_store::ArtifactPack>::encode_pack(&snapshot_value);
    let spr = directory::os_store::empty_document_spr(&document.0, &descriptor.artifact_schema).await;
    let mut descriptor = descriptor;
    descriptor.bootstrap_snapshot_hash = os_directory::hex_lower(&Sha256::digest(&pack));
    let session = state.directory.authenticate_session(&SessionCapability::parse(&author.token).expect("publication author capability")).await.expect("publication author session read").expect("publication author session");
    let actor = ArtifactCreationActorV1 { user_id: author.user_id.clone(), session_id: session.id, authorization_generation: session.authorization_generation };
    let genesis = publish_genesis_checkpoint_for_test(&state, actor, catalog_generation, parent_dialect, descriptor.clone(), &pack, &spr).await;
    let handle = state.ensure_document(&document).await.expect("publication document actor");
    let batch = db::document::CommandBatch::new(vec![sample_envelope(&format!("checkpoint-{label}-edit-1"), &WireArtifactId(document.0.clone())).await]).await.expect("publication command batch");
    handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy: protocol::MergePolicy::default() }).await.expect("publication actor response").expect("publication edit accepted");
    let snapshot = handle.checkpoint_publication_snapshot().await.expect("publication actor snapshot");
    let command = checkpoint_publication_command("1234567890abcdef1234567890abcdef", &descriptor, &snapshot, CheckpointPublicationCurrentV1::Genesis { checkpoint_id: genesis.checkpoint_id.hex() }, &pack, &spr);
    CheckpointPublicationFixture { state, author, spectator, scope, handle, command, pack, spr, catalog_root }
}

#[cfg(all(feature = "sqlite", feature = "test-support"))]
#[tokio::test]
async fn checkpoint_publication_process_fixture_emits_verified_gis_pair_and_catalog() {
    use semio_hub::artifact_authority::trusted_catalog::test_support;

    let artifact_root = std::path::PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("ticket-owned checkpoint process artifact root"));
    let destination = artifact_root.join("checkpoint-publication-process-fixture");
    let stage = artifact_root.join(format!(".checkpoint-publication-process-fixture-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&stage);
    let _ = std::fs::remove_dir_all(&destination);
    std::fs::create_dir_all(&stage).expect("create process fixture stage");

    let profile = test_support::verified_gis_map_test_profile(&test_support::unique_profile_root("checkpoint-process")).await.expect("verified GIS Map process profile");
    let selection = profile.binding().selection();
    assert_eq!((selection.artifact.kind.as_str(), selection.artifact.schema.as_str()), ("s.gis.gismap", "gis.map"));
    let source = profile.bundle_path().parent().expect("profile bundle parent");
    let bundle_bytes = std::fs::read(profile.bundle_path()).expect("read verified profile bundle");
    let bundle: serde_json::Value = serde_json::from_slice(&bundle_bytes).expect("decode verified profile bundle");
    let generation_id = bundle["profiles"][0]["generationId"].as_str().expect("verified generation id");
    let generation = stage.join("data/trusted-catalog/generations").join(generation_id);
    std::fs::create_dir_all(&generation).expect("create trusted generation");
    for name in ["component.wasm", "descriptor.semio", "closed-actor.mjs", "stdio-component.wasm", "stdio-descriptor.semio", "trusted-catalog.json"] {
        std::fs::copy(source.join(name), generation.join(name)).unwrap_or_else(|error| panic!("copy verified {name}: {error}"));
    }
    let bundle_sha256 = os_directory::hex_lower(&Sha256::digest(&bundle_bytes));
    std::fs::create_dir_all(stage.join("data/trusted-catalog")).expect("create trusted current owner");
    std::fs::write(
        stage.join("data/trusted-catalog/current.json"),
        format!(
            r#"{{"profileId":"{}","generationId":"{generation_id}","bundleSha256":"{bundle_sha256}","publicationRevision":"1"}}
"#,
            test_support::GIS_MAP_TEST_PROFILE_ID
        ),
    )
    .expect("write trusted current pointer");

    let pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::encode_pack(&gis_map_test_snapshot());
    let spr = directory::os_store::empty_document_spr("", &selection.artifact.schema).await;
    let diff = db::document::encode_pathmap_json(&serde_json::json!({ "checkpoint-process": "committed" })).await.expect("encode process mutation diff");
    let inverse = db::document::encode_pathmap_json(&serde_json::json!({ "checkpoint-process": null })).await.expect("encode process mutation inverse");
    let payload = stage.join("payload");
    std::fs::create_dir_all(&payload).expect("create process payload owner");
    for (name, bytes) in [("pack.bin", pack.as_slice()), ("spr.bin", spr.as_slice()), ("diff.bin", diff.as_slice()), ("inverse.bin", inverse.as_slice())] {
        std::fs::write(payload.join(name), bytes).unwrap_or_else(|error| panic!("write process {name}: {error}"));
    }
    let fixture = serde_json::json!({
        "schema": "semio.hub.checkpoint-publication-process-fixture/v1",
        "profileId": test_support::GIS_MAP_TEST_PROFILE_ID,
        "generationId": generation_id,
        "documentId": "mcp-cold-gis-map",
        "mutationId": "mcp-cold-gis-map-edit-1",
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
        "payload": {
            "pack": { "path": "payload/pack.bin", "byteLength": pack.len(), "sha256": os_directory::hex_lower(&Sha256::digest(&pack)) },
            "spr": { "path": "payload/spr.bin", "byteLength": spr.len(), "sha256": os_directory::hex_lower(&Sha256::digest(&spr)) },
            "diff": { "path": "payload/diff.bin", "schema": db::document::DB_PATHMAP_SCHEMA },
            "inverse": { "path": "payload/inverse.bin", "schema": db::document::DB_PATHMAP_SCHEMA }
        }
    });
    std::fs::write(stage.join("fixture.json"), serde_json::to_vec_pretty(&fixture).expect("encode process fixture")).expect("write process fixture receipt");
    std::fs::rename(&stage, &destination).expect("publish process fixture atomically");
    let dependency_fixture: serde_json::Value = serde_json::from_str(include_str!("../../🗿️artifact-authority/🔏️trusted-catalog/🧪️fixtures/🔗️compiled-dependencies/🔣️.json")).expect("compiled dependency fixture");
    let expected_files: std::collections::BTreeSet<_> = dependency_fixture["publicationFiles"].as_array().unwrap().iter().map(|name| name.as_str().unwrap().to_owned()).collect();
    let retained_generation = destination.join("data/trusted-catalog/generations").join(generation_id);
    let actual_files: std::collections::BTreeSet<_> = std::fs::read_dir(&retained_generation).unwrap().map(|entry| entry.unwrap().file_name().into_string().unwrap()).collect();
    assert_eq!(actual_files, expected_files);
    let control = StartupCatalogControl;
    let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let relocated = TrustedCatalogLoader::load_current(&destination.join("data"), &NativeCodecProviderSetV1::linked(), &context).await.expect("relocated process catalog dependency closure").expect("relocated current");
    assert_eq!(relocated.codec_count(), 28);
    assert_eq!(relocated.packages().len(), 2);
    assert_eq!(relocated.generation_id(), generation_id);
    eprintln!("[DEBUG] checkpoint process fixture relocated files=6 packages=2 codecs=28");
    assert!(destination.join("data/trusted-catalog/current.json").is_file());
    assert_eq!(std::fs::read(destination.join("payload/pack.bin")).expect("read retained GIS pack"), pack);
    assert_eq!(std::fs::read(destination.join("payload/spr.bin")).expect("read retained GIS SPR"), spr);
}

#[cfg(feature = "native-artifact-execution")]
async fn put_checkpoint_publication_blob(addr: SocketAddr, scope: &DocumentScope, token: &str, bytes: &[u8]) {
    let hash = os_directory::hex_lower(&Sha256::digest(bytes));
    let authorization = format!("Bearer {token}");
    let response = raw_http_request(addr, "PUT", &format!("/spaces/{}/blobs/{hash}", scope.space_id), &[("Authorization", authorization.as_str()), ("Content-Type", "application/octet-stream")], bytes).await;
    assert_eq!(response.status, 200, "checkpoint input blob lands before publication: {}", String::from_utf8_lossy(&response.body));
}

#[test]
fn mutation_message_payload_matches_language_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🚧️hub-boundaries/🔣️.json")).expect("valid hub boundary fixture");
    let messages = vec![protocol::MutationMessage::warn("mutation.clamped", "height clamped").at(["node", "height"]).at_op(2), protocol::MutationMessage::info("mutation.cascade", "dependent value updated")];
    let encoded = encode_messages(&messages);
    let parsed: serde_json::Value = serde_json::from_slice(&encoded).expect("first-party message bytes are valid JSON");
    assert_eq!(parsed, fixture["mutationMessages"]);
    assert_eq!(<Vec<protocol::MutationMessage> as FromValue>::from_value(DslValue::from(parsed)).expect("first-party message decode"), messages);
}

async fn spawn_server(state: HubState) -> SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = router(state).into_make_service_with_connect_info::<SocketAddr>();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    addr
}

async fn spawn_restartable_server(state: HubState) -> (SocketAddr, tokio::sync::oneshot::Sender<()>, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = router(state).into_make_service_with_connect_info::<SocketAddr>();
    let (shutdown, stopped) = tokio::sync::oneshot::channel();
    let task = tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                let _ = stopped.await;
            })
            .await
            .unwrap();
    });
    (addr, shutdown, task)
}

#[derive(Debug)]
struct RawHttpResponse {
    status: u16,
    headers: String,
    body: Vec<u8>,
}

async fn raw_http_request_transport(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> Option<RawHttpResponse> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = tokio::net::TcpStream::connect(addr).await.expect("HTTP connect");
    let mut request = format!("{method} {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\nContent-Length: {}\r\n", body.len());
    for (name, value) in headers {
        request.push_str(name);
        request.push_str(": ");
        request.push_str(value);
        request.push_str("\r\n");
    }
    request.push_str("\r\n");
    stream.write_all(request.as_bytes()).await.expect("HTTP write");
    stream.write_all(body).await.expect("HTTP body write");
    stream.flush().await.expect("HTTP request flush");
    let mut response = Vec::new();
    let read = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let mut chunk = [0_u8; 4096];
        loop {
            let read = stream.read(&mut chunk).await?;
            if read == 0 {
                return std::io::Result::Ok(());
            }
            response.extend_from_slice(&chunk[..read]);
            if let Some(boundary) = response.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
                let head = std::str::from_utf8(&response[..boundary]).unwrap_or_default();
                let content_length = head.lines().find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.eq_ignore_ascii_case("content-length").then(|| value.trim().parse::<usize>().ok()).flatten()
                });
                if content_length.is_some_and(|length| response.len() >= boundary + 4 + length) {
                    return std::io::Result::Ok(());
                }
            }
        }
    })
    .await
    .expect("HTTP deadline");
    if let Err(error) = read {
        assert!(matches!(error.kind(), std::io::ErrorKind::ConnectionReset | std::io::ErrorKind::BrokenPipe), "HTTP read: {error}");
    }
    if response.is_empty() {
        return None;
    }
    let boundary = response.windows(4).position(|bytes| bytes == b"\r\n\r\n").expect("HTTP header boundary");
    let head = std::str::from_utf8(&response[..boundary]).expect("HTTP headers").to_string();
    let status = head.split_whitespace().nth(1).expect("HTTP status").parse().expect("numeric HTTP status");
    Some(RawHttpResponse { status, headers: head, body: response[boundary + 4..].to_vec() })
}

async fn raw_http_request(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> RawHttpResponse {
    raw_http_request_transport(addr, method, path, headers, body).await.expect("HTTP response")
}

async fn raw_http_get(addr: SocketAddr, path: &str, headers: &[(&str, &str)]) -> RawHttpResponse {
    raw_http_request(addr, "GET", path, headers, &[]).await
}

//#region 💡️Inference
#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
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
        assert!(state.socket_binding_gates.gate(binding).try_lock_owned().is_err(), "the exact Hub ingress guard remains owned");
    }
    assert!(state.socket_binding_gates.gate(SocketBindingKeyV1::DocumentWrite(scope.clone())).try_lock_owned().is_ok(), "Hub ingress must not outer-lock the runtime document writer");
    revalidate_gis_map_approval_delivery(&state, &authority).await.expect("fresh delivery under the retained guards");
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

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
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

#[cfg(all(feature = "sqlite", feature = "native-artifact-execution"))]
#[tokio::test]
async fn gis_map_proposal_routes_fail_closed_without_a_trusted_map_binding() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture");
    let unavailable = fixture["errors"].as_array().expect("error vocabulary").iter().find(|row| row["name"] == "no-binding").expect("no-binding row");
    let state = test_state().await;
    assert!(state.inference_runtime.is_none(), "production today has no trusted GIS Map profile");
    assert!(state.gis_map_binding.is_none());
    let session = issue_test_session(&state, "inference-owner@example.test").await;
    let addr = spawn_server(state).await;
    let readiness: serde_json::Value = serde_json::from_slice(&raw_http_get(addr, "/readyz", &[]).await.body).expect("readiness body");
    assert_eq!(readiness["features"]["inference"], false, "readiness publishes inference only with a frozen binding");
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
#[cfg(all(feature = "sqlite", feature = "test-support"))]
struct GisMapInferenceFixture {
    state: HubState,
    profile: semio_hub::artifact_authority::trusted_catalog::test_support::VerifiedGisMapTestProfileV1,
    space_id: String,
    document_id: String,
    snapshot_pack: Vec<u8>,
    ledger_path: std::path::PathBuf,
}

#[cfg(all(feature = "sqlite", feature = "test-support"))]
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
#[cfg(all(feature = "sqlite", feature = "test-support"))]
async fn gis_map_inference_fixture(author_email: &str, spectator_email: &str) -> (GisMapInferenceFixture, TestIssuedSession, TestIssuedSession) {
    use semio_hub::artifact_authority::trusted_catalog::test_support;
    let mut state = test_state().await;
    let profile = test_support::verified_gis_map_test_profile(&test_support::unique_profile_root("routes")).await.expect("real GIS Map editor profile");
    let ledger_path = tempdir("inference").join("jobs.sqlite3");
    let ledger = semio_hub::inference::sqlite::InferenceJobLedgerV1::open(&ledger_path).expect("private job ledger");
    state.verified_catalog = Some(profile.catalog().clone());
    state.gis_map_binding = Some(profile.binding().clone());
    state.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(profile.binding().clone(), Arc::new(ledger), Arc::new(UnavailableGisMapApprovalCommitterV1))));
    let author = issue_test_session(&state, author_email).await;
    let spectator = issue_test_session(&state, spectator_email).await;
    let space_id = create_space_for_test(&state, &author.user_id, "GIS Map Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, author_email, DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space_id, spectator_email, DirectorySpaceRole::Spectator).await;
    let document_id = artifact_document_id_for_test("gis-map-inference");
    let selection = profile.binding().selection();
    let genesis_pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::encode_pack(&semio_s_artifact_gis_gismap::GisMapSnapshot::default());
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
        bootstrap_snapshot_hash: os_directory::hex_lower(&Sha256::digest(&genesis_pack)),
    };
    let authenticated = state.directory.authenticate_session(&SessionCapability::parse(&author.token).expect("GIS Map author capability")).await.expect("GIS Map author session read").expect("GIS Map author session");
    let actor = ArtifactCreationActorV1 { user_id: author.user_id.clone(), session_id: authenticated.id, authorization_generation: authenticated.authorization_generation };
    let genesis = publish_genesis_checkpoint_for_test(&state, actor, profile.catalog().generation_id().to_string(), selection.parent_dialect.clone(), descriptor, &genesis_pack, genesis_spr).await;
    let snapshot_pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as directory::ArtifactPack>::encode_pack(&gis_map_test_snapshot());
    publish_gis_checkpoint_for_test(&state, &space_id, &document_id, &snapshot_pack, &genesis).await;
    (GisMapInferenceFixture { state, profile, space_id, document_id, snapshot_pack, ledger_path }, author, spectator)
}

/// 🧾️ Publishes one verified active checkpoint whose pack is the literal GIS Map snapshot bytes.
#[cfg(all(feature = "sqlite", feature = "test-support"))]
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
        descriptor_digest_v1: os_directory::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
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
    let control = StartupCatalogControl;
    let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
    cas.stage(space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &context).await.expect("stage GIS Map pack");
    cas.stage(space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &context).await.expect("stage GIS Map SPR");
    state.directory_service.publish_reserved_artifact_checkpoint(system, checkpoint, reservation, 100).await.expect("publish verified GIS Map checkpoint");
}

#[cfg(all(feature = "sqlite", feature = "test-support"))]
fn inference_route(space_id: &str, document_id: &str, suffix: &str) -> String {
    format!("/spaces/{space_id}/documents/{document_id}/inference/gis-map/jobs{suffix}")
}

#[cfg(all(feature = "sqlite", feature = "test-support"))]
fn inference_intent(request_id: &str) -> String {
    serde_json::json!({ "schema": "semio.hub.inference-request/v1", "version": 1, "requestId": request_id, "serviceId": "s.gis.gismap.inference", "policyVersion": 1, "lifetimeMs": 120_000 }).to_string()
}

#[cfg(all(feature = "sqlite", feature = "test-support"))]
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

#[cfg(all(feature = "sqlite", feature = "test-support"))]
fn proposal_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧪️fixtures/🗳️gis-map-proposal-approval-v1/🔣️.json")).expect("proposal fixture")
}

#[cfg(all(feature = "sqlite", feature = "test-support"))]
#[tokio::test]
async fn gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation() {
    let fixture = proposal_fixture();
    let (bound, author, _spectator) = gis_map_inference_fixture("map-owner@example.test", "map-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
    let checkpoint = Arc::new(semio_hub::inference::runtime::InferenceCheckpointTestGateV1::new());
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
    assert!(cursors.windows(2).all(|pair| pair[1] == pair[0] + 1), "the progress cursor is monotonic and dense: {cursors:?}");
    assert!(cursors.len() as u64 <= fixture["limits"]["progressMaxCursor"].as_u64().expect("cursor bound"), "progress is bounded");
    assert_eq!(page["nextCursor"].as_u64().expect("next cursor"), cursors.last().copied().unwrap_or(0));
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

#[cfg(all(feature = "sqlite", feature = "test-support"))]
#[tokio::test]
async fn gis_map_inference_runtime_close_signals_and_joins_actual_codec_work() {
    let (bound, author, _spectator) = gis_map_inference_fixture("close-owner@example.test", "close-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
    let checkpoint = Arc::new(semio_hub::inference::runtime::InferenceCheckpointTestGateV1::new());
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

#[cfg(all(feature = "sqlite", feature = "test-support"))]
#[tokio::test]
async fn gis_map_inference_revocation_after_compute_refuses_late_publication() {
    let (bound, author, _spectator) = gis_map_inference_fixture("revoked-owner@example.test", "revoked-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
    let checkpoint = Arc::new(semio_hub::inference::runtime::InferenceCheckpointTestGateV1::new());
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
    let owner = semio_hub::inference::sqlite::InferenceReaderV1 { user_id: &session.user_id, session_id: &session.id, authorization_generation: session.authorization_generation, space_id: &space_id, document_id: &document_id };
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
    let page = runtime.ledger().events(&job_id, &semio_hub::inference::runtime::reader(&identity), 0, u64::try_from(now_ms()).expect("test clock")).expect("private terminal page");
    assert_eq!(page.state, semio_hub::inference::schema::InferenceJobStateV1::Cancelled);
    assert_eq!(page.proposal_state, semio_hub::inference::schema::InferenceProposalStateV1::None);
    assert!(page.proposal_hash.is_none(), "revocation before the final authority fence publishes no proposal");
}

#[cfg(all(feature = "sqlite", feature = "test-support"))]
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

#[cfg(all(feature = "sqlite", feature = "test-support"))]
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

#[cfg(all(feature = "sqlite", feature = "test-support"))]
#[tokio::test]
async fn gis_map_approval_is_idempotent_across_duplicate_requests_and_restart() {
    let (bound, author, _spectator) = gis_map_inference_fixture("idempotent-owner@example.test", "idempotent-watcher@example.test").await;
    let (space_id, document_id) = (bound.space_id.clone(), bound.document_id.clone());
    let runtime = bound.state.inference_runtime.as_ref().expect("GIS inference runtime").clone();
    let checkpoint = Arc::new(semio_hub::inference::runtime::InferenceCheckpointTestGateV1::new());
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
    let reopened = semio_hub::inference::sqlite::InferenceJobLedgerV1::open(&bound.ledger_path).expect("the durable ledger reopens after a restart");
    restarted.inference_runtime = Some(Arc::new(HubInferenceRuntimeV1::new(bound.profile.binding().clone(), Arc::new(reopened), Arc::new(UnavailableGisMapApprovalCommitterV1))));
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🚧️hub-boundaries/🔣️.json")).expect("hub boundary fixture");
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🚧️hub-boundaries/🔣️.json")).expect("hub boundary fixture");
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
    assert!(!canonical_pair_auth_outcome_allowed(&AuthOutcome::Denied));
    assert!(canonical_pair_auth_outcome_allowed(&AuthOutcome::ShareToken));
}

#[tokio::test]
async fn canonical_pair_route_is_exact_member_or_share_and_emits_only_verified_public_pair() {
    let mut state = lag_test_state(1024, 256).await;
    let document_id = "canonical-pair-document";
    announce_document_for_test(&state, STUDIO, document_id).await;
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
    announce_document_for_test(&state, &other_space, document_id).await;
    publish_checkpoint_for_test(&state, &other_space, document_id).await;
    let public_space = create_space_for_test(&state, &outsider.user_id, "Canonical Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
    announce_document_for_test(&state, &public_space, document_id).await;
    publish_checkpoint_for_test(&state, &public_space, document_id).await;
    let other_document = "canonical-pair-other-document";
    announce_document_for_test(&state, STUDIO, other_document).await;
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
    let document_id = "canonical-pair-lifecycle-document";
    announce_document_for_test(&state, STUDIO, document_id).await;
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

/// @emoji 🧪️ A loopback `ConnectInfo` for handlers called directly in tests. Network
/// proximity confers no authorization; every protected test also supplies a verified session.
fn loopback_peer() -> axum::extract::ConnectInfo<SocketAddr> {
    axum::extract::ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 0)))
}

async fn next_server_frame<S>(ws: &mut S) -> ServerFrame
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match tokio::time::timeout_at(deadline, ws.next()).await {
            Ok(Some(Ok(WsMessage::Binary(bytes)))) => return protocol::decode_server_frame(&bytes).await.expect("server frame").1,
            Ok(Some(Ok(_))) => continue,
            Ok(Some(other)) => panic!("expected binary frame, got {other:?}"),
            Ok(None) => panic!("stream ended before server frame"),
            Err(_) => panic!("no server frame before 5s deadline"),
        }
    }
}

async fn next_directory_message<S>(ws: &mut S) -> DirectoryStreamMessage
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match tokio::time::timeout_at(deadline, ws.next()).await {
            Ok(Some(Ok(WsMessage::Text(text)))) => return directory::os_pack::json::from_json_str(&text).expect("directory message"),
            Ok(Some(Ok(_))) => continue,
            Ok(Some(other)) => panic!("expected directory message, got {other:?}"),
            Ok(None) => panic!("stream ended before directory message"),
            Err(_) => panic!("no directory message before 5s deadline"),
        }
    }
}

async fn client_binary(frame: &ClientFrame, lane: Lane) -> WsMessage {
    WsMessage::Binary(protocol::encode_client_frame(frame, lane).await.into())
}

async fn next_close_code<S>(ws: &mut S, allow_text: bool) -> u16
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match tokio::time::timeout_at(deadline, ws.next()).await {
            Ok(Some(Ok(WsMessage::Close(Some(frame))))) => return frame.code.into(),
            Ok(Some(Ok(WsMessage::Text(text)))) if !allow_text => panic!("unauthorized rebootstrap control leaked: {text}"),
            Ok(Some(Ok(_))) => continue,
            Ok(Some(other)) => panic!("expected close frame, got {other:?}"),
            Ok(None) => panic!("stream ended before close frame"),
            Err(_) => panic!("no close frame before 5s deadline"),
        }
    }
}

async fn next_close_without_authority<S>(ws: &mut S) -> u16
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    match tokio::time::timeout(std::time::Duration::from_secs(5), ws.next()).await {
        Ok(Some(Ok(WsMessage::Close(Some(frame))))) => frame.code.into(),
        Ok(Some(Ok(WsMessage::Binary(_)))) => panic!("authority-bearing binary frame crossed revocation"),
        Ok(Some(Ok(WsMessage::Text(_)))) => panic!("authority-bearing directory frame crossed revocation"),
        Ok(Some(other)) => panic!("expected close after revocation, got {other:?}"),
        Ok(None) => panic!("stream ended before revocation close"),
        Err(_) => panic!("no revocation close before 5s deadline"),
    }
}

fn socket_hello() -> ClientFrame {
    ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: "test.v1".to_string(), pack_schema_hash: [0x11; 32], resume_token: None, frontier: None }
}

fn bearer_headers(capability: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {capability}").parse().expect("bearer header"));
    headers
}

fn socket_request(url: &str, grant: &str) -> tokio_tungstenite::tungstenite::http::Request<()> {
    let mut request = url.into_client_request().expect("socket request");
    request.headers_mut().insert(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {grant}").parse().expect("socket protocols"));
    request
}

struct TestIssuedSession {
    token: String,
    user_id: String,
}

async fn issue_test_session(state: &HubState, email: &str) -> TestIssuedSession {
    let user = match state.directory.get_user_by_email(email).await.expect("test user lookup") {
        Some(user) => user,
        None => state.directory.create_user(email, email, None, Some(email), Some("test-verifier")).await.expect("test verified user"),
    };
    let issue = AuthSessionIssue {
        user_id: user.id.clone(),
        identity_provider: "test-verifier".into(),
        identity_subject_digest: identity_subject_digest("test-verifier", email).expect("test subject digest"),
        ttl_secs: 3_600,
        device_instance_id: "test-device".into(),
        session_kind: AuthSessionKind::DevelopmentLocal,
        correlation_id: directory::os_identity::time_ordered_id(),
        peer_class: "test".into(),
    };
    let issued = state.directory.issue_auth_session(&issue).await.expect("test session issue");
    TestIssuedSession { token: issued.capability.expose_once(), user_id: user.id }
}

async fn authorize_test_admin(state: &mut HubState, email: &str) -> HeaderMap {
    let session = issue_test_session(state, email).await;
    state.admin_subjects = Arc::from([AdminSubject { provider_digest: admin_provider_digest("test-verifier"), subject_digest: identity_subject_digest("test-verifier", email).expect("test admin subject digest") }]);
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.token).parse().expect("test bearer header"));
    headers
}

async fn seed_author_token(state: &HubState) -> String {
    let issue = AuthSessionIssue {
        user_id: "seed".into(),
        identity_provider: "test-verifier".into(),
        identity_subject_digest: identity_subject_digest("test-verifier", "seed").expect("seed subject digest"),
        ttl_secs: 3_600,
        device_instance_id: "seed-device".into(),
        session_kind: AuthSessionKind::DevelopmentLocal,
        correlation_id: directory::os_identity::time_ordered_id(),
        peer_class: "test".into(),
    };
    state.directory.issue_auth_session(&issue).await.expect("seed author session").capability.expose_once()
}

#[tokio::test]
async fn socket_grant_ledger_is_bounded_single_consume_restart_scoped_and_revoke_race_safe() {
    let ledger = Arc::new(SocketGrantLedgerV1::default());
    let audience = SocketAudienceV1::Document(DocumentScope::new("space-a", "document-a"));
    let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Author), expires_at_ms: 10_000 };
    let capability = SocketGrantCapability::mint().expect("socket grant");
    ledger.issue(&capability, audience.clone(), "hub.v1.actor".into(), subject.clone(), 1, 9_000).expect("issue grant");
    assert!(ledger.pending(&capability, &SocketAudienceV1::Document(DocumentScope::new("space-a", "document-b")), 2).is_err(), "audience mismatch never consumes");
    let candidate = ledger.pending(&capability, &audience, 2).expect("pending grant");
    let barrier = Arc::new(std::sync::Barrier::new(3));
    let attempts = (0..2)
        .map(|_| {
            let ledger = ledger.clone();
            let candidate = candidate.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                ledger.consume(&candidate, 3).is_ok()
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    assert_eq!(attempts.into_iter().map(|attempt| attempt.join().expect("consume race")).filter(|won| *won).count(), 1, "exactly one concurrent upgrade consumes");
    assert!(ledger.pending(&capability, &audience, 4).is_err(), "consumed grants never replay");
    assert!(SocketGrantLedgerV1::default().pending(&capability, &audience, 4).is_err(), "grants are process-bound and disappear on restart");
    let (_, live_notify) = ledger.register_live(&candidate).expect("register consumed grant live");

    let pending = SocketGrantCapability::mint().expect("pending socket grant");
    ledger.issue(&pending, audience.clone(), "hub.v1.pending".into(), subject.clone(), 4, 9_000).expect("issue pending grant");
    let stale = ledger.pending(&pending, &audience, 5).expect("candidate before revoke");
    ledger.invalidate_binding(subject.binding());
    tokio::time::timeout(std::time::Duration::from_secs(1), live_notify.notified()).await.expect("live revoke notification");
    assert!(ledger.consume(&stale, 6).is_err(), "revoke between durable revalidation and consume fails closed");
    assert!(ledger.register_live(&candidate).is_err(), "consume then revoke then late register fails closed");

    let ttl_ledger = SocketGrantLedgerV1::default();
    let ttl_capability = SocketGrantCapability::mint().expect("TTL grant");
    ttl_ledger.issue(&ttl_capability, audience.clone(), "hub.v1.ttl".into(), subject.clone(), 1, 10).expect("issue TTL grant");
    let ttl_candidate = ttl_ledger.pending(&ttl_capability, &audience, 2).expect("pending TTL grant");
    let ttl_consumed = ttl_ledger.consume(&ttl_candidate, 3).expect("consume TTL grant");
    let (ttl_live_id, _) = ttl_ledger.register_live(&ttl_consumed).expect("register TTL grant live");
    let sweep_trigger = SocketGrantCapability::mint().expect("sweep trigger");
    ttl_ledger.issue(&sweep_trigger, audience.clone(), "hub.v1.sweep".into(), subject.clone(), 11, 100).expect("trigger grant sweep");
    assert!(ttl_ledger.is_live(&ttl_consumed, &ttl_live_id), "grant TTL applies to dial/consume, not a durably-authorized live socket");
    ttl_ledger.unregister_live(&ttl_consumed, &ttl_live_id);
    assert!(!ttl_ledger.inner.lock().expect("ledger").records.contains_key(ttl_capability.selector()), "last live lease reclaims its consumed grant record");

    let abandoned = SocketGrantLedgerV1::default();
    let mut first_abandoned = None;
    for index in 0..SOCKET_GRANT_LEDGER_CAPACITY {
        let capability = SocketGrantCapability::mint().expect("abandoned grant");
        abandoned.issue(&capability, audience.clone(), format!("hub.v1.abandoned.{index}"), subject.clone(), 1, 10).expect("fill ledger");
        let candidate = abandoned.pending(&capability, &audience, 2).expect("abandoned pending");
        abandoned.consume(&candidate, 3).expect("abandoned consume");
        first_abandoned.get_or_insert(capability);
    }
    assert!(abandoned.pending(first_abandoned.as_ref().expect("first abandoned"), &audience, 4).is_err(), "consumed failed-pre-live grant never replays");
    let recovered = SocketGrantCapability::mint().expect("recovered grant");
    abandoned.issue(&recovered, audience.clone(), "hub.v1.recovered".into(), subject.clone(), 11, 100).expect("expired pre-live tombstones reclaim full ledger capacity");

    let bounded = SocketGrantLedgerV1::default();
    for index in 0..SOCKET_GRANT_BINDING_PENDING_CAPACITY {
        let capability = SocketGrantCapability::mint().expect("bounded grant");
        bounded.issue(&capability, audience.clone(), format!("hub.v1.{index}"), subject.clone(), 1, 9_000).expect("within per-binding bound");
    }
    let overflow = SocketGrantCapability::mint().expect("overflow grant");
    assert_eq!(bounded.issue(&overflow, audience, "hub.v1.overflow".into(), subject.clone(), 1, 9_000), Err(SocketGrantLedgerErrorV1::Capacity));
    bounded.invalidate_binding(subject.binding());
    assert!(bounded.issue(&overflow, SocketAudienceV1::Document(DocumentScope::new("space-a", "document-a")), "hub.v1.after-revoke".into(), subject, 2, 9_000).is_ok());
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
struct DocumentOpenPlanLedgerFixture {
    now_ms: u64,
    descriptor: DocumentDescriptor,
    descriptor_digest_v1: String,
    valid_plan: DocumentOpenPlanV1,
}

fn document_open_plan_test_authority(fixture: &DocumentOpenPlanLedgerFixture) -> DocumentOpenPlanAuthorityV1 {
    DocumentOpenPlanAuthorityV1 {
        scope: fixture.valid_plan.scope.clone(),
        descriptor: fixture.descriptor.clone(),
        descriptor_digest_v1: fixture.descriptor_digest_v1.clone(),
        catalog: fixture.valid_plan.catalog.clone(),
        package: fixture.valid_plan.package.clone(),
        artifact: fixture.valid_plan.artifact.clone(),
        parent_dialect: semio_framework::ArtifactDialect {
            artifact_kind: fixture.valid_plan.parent_dialect.artifact_kind.clone(),
            standard: fixture.valid_plan.parent_dialect.standard.clone(),
            subset: fixture.valid_plan.parent_dialect.subset.clone(),
        },
        surface: fixture.valid_plan.surface.clone(),
        browser_actor: fixture.valid_plan.browser_actor.clone(),
        grant: fixture.valid_plan.grant,
        checkpoint: fixture.valid_plan.checkpoint.clone(),
        revalidation: fixture.valid_plan.revalidation,
        subject: SocketSubjectV1::Session {
            session_id: "open-plan-session".into(),
            user_id: "open-plan-user".into(),
            authorization_generation: fixture.valid_plan.revalidation.session_generation.expect("session generation"),
            role: Some(SpaceRole::Author),
            expires_at_ms: i64::MAX,
        },
        server_actor_id: "hub.v1.open-plan-actor".into(),
        client_instance_id_digest: [9; 32],
    }
}

const TEST_EXECUTION_TARGET_COMPONENT_BYTES: &[u8] = b"\0asm\x01\0\0\0semio-execution-target-lease-component";
const TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES: &[u8] = b"semio-execution-target-lease-descriptor";

fn document_open_catalog_for_descriptor(descriptor: &DocumentDescriptor) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
    document_open_catalog_for_descriptor_with_generation(descriptor, "66".repeat(32))
}

fn install_document_open_catalog_for_test(state: &mut HubState, descriptor: &DocumentDescriptor) {
    state.openable_catalog = Some(document_open_catalog_for_descriptor(descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
}

fn document_open_catalog_for_descriptor_with_generation(descriptor: &DocumentDescriptor, generation_id: String) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
    let package = DocumentOpenPackageV1 {
        plugin_id: descriptor.owner.plugin_id.clone(),
        package_id: descriptor.owner.package_id.clone(),
        version: descriptor.owner.version.clone(),
        component_sha256: descriptor.owner.package_hash.clone(),
        component_blake3: "44".repeat(32),
        descriptor_byte_sha256: "55".repeat(32),
        execution_protocol: os_directory::DocumentExecutionProtocolV1 { app_channel_version: directory::os_spr::CHANNEL_VERSION },
    };
    let artifact = DocumentOpenArtifactV1 { kind: descriptor.artifact_kind.clone(), schema: descriptor.artifact_schema.clone(), pack_schema_hash: descriptor.pack_schema_hash.clone() };
    Arc::new(TestDocumentOpenCatalog {
        generation_id,
        component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
        descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
        browser_actor: None,
        open_targets: vec![
            VerifiedDocumentOpenSelectionV1 {
                package: package.clone(),
                artifact: artifact.clone(),
                parent_dialect: semio_framework::ArtifactDialect { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() },
                surface: DocumentOpenSurfaceV1 {
                    surface_id: "surface.test.editor".into(),
                    app_id: "app.test".into(),
                    window_kind_id: "window.document".into(),
                    role: os_directory::DocumentOpenSurfaceRoleV1::Editor,
                    renderer_target: os_directory::DocumentOpenRendererTargetV1::React,
                },
                grant: DocumentOpenGrantV1 { read: true, write: true, observe: true },
                browser_actor: os_directory::schema::DocumentOpenBrowserActorV1::None,
            },
            VerifiedDocumentOpenSelectionV1 {
                package,
                artifact,
                parent_dialect: semio_framework::ArtifactDialect { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() },
                surface: DocumentOpenSurfaceV1 {
                    surface_id: "surface.test.viewer".into(),
                    app_id: "app.test".into(),
                    window_kind_id: "window.document".into(),
                    role: os_directory::DocumentOpenSurfaceRoleV1::Viewer,
                    renderer_target: os_directory::DocumentOpenRendererTargetV1::React,
                },
                grant: DocumentOpenGrantV1 { read: true, write: false, observe: true },
                browser_actor: os_directory::schema::DocumentOpenBrowserActorV1::None,
            },
        ]
        .into_boxed_slice(),
    })
}

fn document_open_plan_secret(index: u32) -> [u8; 32] {
    let mut secret = [0u8; 32];
    secret[28..].copy_from_slice(&index.to_be_bytes());
    secret
}

fn document_open_plan_authority_for_scope(base: &DocumentOpenPlanAuthorityV1, binding: u32, document: u32) -> DocumentOpenPlanAuthorityV1 {
    let mut authority = base.clone();
    authority.scope.document_id = format!("document-{document}");
    authority.descriptor.document_id = authority.scope.document_id.clone();
    authority.descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&authority.descriptor).expect("scoped descriptor digest").0);
    authority.checkpoint.descriptor_digest_v1 = authority.descriptor_digest_v1.clone();
    authority.checkpoint.baseline_frontier.document_id = authority.scope.document_id.clone();
    authority.subject = SocketSubjectV1::Session {
        session_id: format!("open-plan-session-{binding}"),
        user_id: format!("open-plan-user-{binding}"),
        authorization_generation: authority.revalidation.session_generation.expect("session generation"),
        role: Some(SpaceRole::Author),
        expires_at_ms: i64::MAX,
    };
    authority
}

async fn document_open_plan_authority_for_session(state: &HubState, fixture: &DocumentOpenPlanLedgerFixture, token: &str, scope: DocumentScope) -> DocumentOpenPlanAuthorityV1 {
    let capability = SessionCapability::parse(token).expect("session capability");
    let session = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
    let role = state.directory.get_role(&scope.space_id, &session.user_id).await.expect("role lookup").expect("space member");
    let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("announced descriptor");
    let mut authority = document_open_plan_test_authority(fixture);
    authority.scope = scope;
    authority.descriptor = descriptor;
    authority.descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&authority.descriptor).expect("descriptor digest").0);
    authority.package.plugin_id = authority.descriptor.owner.plugin_id.clone();
    authority.package.package_id = authority.descriptor.owner.package_id.clone();
    authority.package.version = authority.descriptor.owner.version.clone();
    authority.package.component_sha256 = authority.descriptor.owner.package_hash.clone();
    authority.artifact.kind = authority.descriptor.artifact_kind.clone();
    authority.artifact.schema = authority.descriptor.artifact_schema.clone();
    authority.artifact.pack_schema_hash = authority.descriptor.pack_schema_hash.clone();
    if let Some(catalog) = &state.openable_catalog {
        let selected = catalog.resolve_document_open(&authority.descriptor, None, matches!(role, SpaceRole::Author)).expect("test catalog selection");
        authority.catalog.generation_id = catalog.generation_id().into();
        authority.package = selected.package;
        authority.artifact = selected.artifact;
        authority.parent_dialect = selected.parent_dialect;
        authority.surface = selected.surface;
        authority.browser_actor = selected.browser_actor;
        authority.grant = selected.grant;
    }
    authority.checkpoint = document_open_checkpoint(state.directory.get_active_artifact_checkpoint(&authority.scope).await.expect("checkpoint lookup").expect("committed document checkpoint"));
    authority.grant.write = matches!(role, SpaceRole::Author);
    let directory_revision = state.directory.head_seq().await.expect("directory revision");
    authority.revalidation.directory_revision = directory_revision;
    authority.revalidation.membership_generation = directory_revision;
    authority.revalidation.session_generation = Some(session.authorization_generation);
    authority.revalidation.share_generation = None;
    authority.subject = SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: Some(role), expires_at_ms: session.expires_at };
    authority.server_actor_id = socket_actor_id(&session.secret_digest, true);
    authority.validate().expect("authenticated route authority");
    authority
}

async fn issue_and_exchange_document_open_plan_for_test(state: &HubState, token: &str, scope: &DocumentScope, client_instance_id: &str) -> (DocumentOpenPlanV1, SocketGrantReceiptV1) {
    let mut headers = bearer_headers(token);
    headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
    let intent = DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: Some("surface.test.editor".into()), client_instance_id: client_instance_id.into() };
    let DirectoryJson(plan) = match issue_document_open_plan_inner(scope.space_id.clone(), scope.document_id.clone(), headers.clone(), state.clone(), Bytes::from(directory::os_pack::json::to_json_string(&intent))).await {
        Ok(plan) => plan,
        Err((status, DirectoryJson(error))) => panic!("issue document open plan: {status} {:?}", error.code),
    };
    let exchange = DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: plan.receipt.clone() };
    let Json(grant) = match issue_document_plan_socket_grant_inner(scope.space_id.clone(), scope.document_id.clone(), headers, state.clone(), Bytes::from(directory::os_pack::json::to_json_string(&exchange))).await {
        Ok(grant) => grant,
        Err((status, DirectoryJson(error))) => panic!("exchange document open plan: {status} {:?}", error.code),
    };
    (plan, grant)
}

#[test]
fn document_open_plan_ledger_is_digest_only_bounded_single_use_revalidated_and_restart_scoped() {
    let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture");
    let authority = document_open_plan_test_authority(&fixture);
    for (field, value) in [("artifactKind", "s.foreign.document".to_owned()), ("standard", String::new()), ("subset", String::new()), ("standard", "\u{85}".to_owned()), ("subset", " * ".to_owned()), ("standard", "🌊".repeat(65))] {
        let mut hostile = authority.clone();
        match field {
            "artifactKind" => hostile.parent_dialect.artifact_kind = value,
            "standard" => hostile.parent_dialect.standard = value,
            "subset" => hostile.parent_dialect.subset = value,
            _ => unreachable!(),
        }
        assert_eq!(hostile.validate(), Err(DocumentOpenPlanErrorCodeV1::Stale), "invalid private parent {field}");
    }
    let ledger = Arc::new(DocumentOpenPlanLedgerV1::default());
    let secret = std::array::from_fn(|index| u8::try_from(index + 1).expect("fixture secret byte"));
    let public = ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + DOCUMENT_OPEN_PLAN_MAX_TTL_MS, DocumentOpenPlanCapabilityV1::from_secret(secret)).expect("issue fixture plan");
    assert_eq!(public.receipt, "open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA");
    assert_eq!(public.parent_dialect, fixture.valid_plan.parent_dialect);
    let mut public_parent_kind = public.clone();
    public_parent_kind.parent_dialect.artifact_kind.push_str(".foreign");
    assert_eq!(public_parent_kind.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let mut public_parent_control = public.clone();
    public_parent_control.parent_dialect.standard.push('\u{85}');
    assert_eq!(public_parent_control.validate(fixture.now_ms), Err(DocumentOpenPlanErrorCodeV1::Denied));
    let expected_digest = [0x5d, 0x05, 0xc0, 0xd4, 0x09, 0x43, 0xe9, 0xab, 0xd9, 0x66, 0x33, 0xca, 0x62, 0xfe, 0x36, 0x2b, 0x58, 0xc6, 0x6e, 0xa8, 0x5a, 0xe0, 0xa0, 0xba, 0x1f, 0x02, 0x48, 0x8e, 0x38, 0xc0, 0xd3, 0x24];
    {
        let inner = ledger.inner.lock().expect("open plan ledger");
        let record = inner.records.get(&expected_digest).expect("digest-only record");
        assert_eq!(record.receipt_digest, expected_digest);
        assert_eq!(record.issued_at_ms, fixture.now_ms);
        assert_eq!(record.socket_grant_selector, None);
    }

    let barrier = Arc::new(std::sync::Barrier::new(9));
    let exchange_at = fixture.now_ms + 1;
    let attempts = (0..8)
        .map(|_| {
            let ledger = ledger.clone();
            let barrier = barrier.clone();
            let receipt = public.receipt.clone();
            let authority = authority.clone();
            std::thread::spawn(move || {
                barrier.wait();
                ledger.exchange(&receipt, &authority, exchange_at, "socket-selector").is_ok()
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    assert_eq!(attempts.into_iter().map(|attempt| attempt.join().expect("exchange race")).filter(|won| *won).count(), 1);
    assert_eq!(ledger.exchange(&public.receipt, &authority, fixture.now_ms + 2, "socket-selector-2"), Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed));
    assert_eq!(DocumentOpenPlanLedgerV1::default().exchange(&public.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Denied));

    let mismatch_ledger = DocumentOpenPlanLedgerV1::default();
    let mismatch = mismatch_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(10))).expect("issue mismatch plan");
    let mut foreign = authority.clone();
    foreign.catalog.generation_id = "77".repeat(32);
    assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &foreign, fixture.now_ms + 1, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    let mut foreign_parent = authority.clone();
    foreign_parent.parent_dialect.standard = "2".into();
    assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &foreign_parent, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    assert_eq!(mismatch_ledger.exchange(&mismatch.receipt, &authority, fixture.now_ms + 100, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Expired));

    let replacement_ledger = DocumentOpenPlanLedgerV1::default();
    let first = replacement_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(11))).expect("first outstanding plan");
    let second = replacement_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 1, fixture.now_ms + 101, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(12))).expect("replacement outstanding plan");
    assert_eq!(replacement_ledger.exchange(&first.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    replacement_ledger.invalidate_receipt(&second.receipt).expect("cancel after publication");
    assert_eq!(replacement_ledger.exchange(&second.receipt, &authority, fixture.now_ms + 2, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let replacement_expiry_ledger = DocumentOpenPlanLedgerV1::default();
    let replaced_a = replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(14))).expect("replacement A");
    let replaced_b = replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 1, fixture.now_ms + 200, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(15))).expect("replacement B");
    let replaced_c =
        replacement_expiry_ledger.issue_with_capability(authority.clone(), fixture.now_ms + 101, fixture.now_ms + 201, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(16))).expect("replacement C after A expiry sweep");
    assert_eq!(replacement_expiry_ledger.exchange(&replaced_a.receipt, &authority, fixture.now_ms + 102, "socket-a"), Err(DocumentOpenPlanErrorCodeV1::Denied));
    assert_eq!(replacement_expiry_ledger.exchange(&replaced_b.receipt, &authority, fixture.now_ms + 102, "socket-b"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    assert!(replacement_expiry_ledger.exchange(&replaced_c.receipt, &authority, fixture.now_ms + 102, "socket-c").is_ok());

    let revoke_ledger = DocumentOpenPlanLedgerV1::default();
    let revoked = revoke_ledger.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(13))).expect("revocable plan");
    assert_eq!(revoke_ledger.invalidate_binding(&authority.subject.binding()), 1);
    assert_eq!(revoke_ledger.exchange(&revoked.receipt, &authority, fixture.now_ms + 1, "socket-selector"), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let share_scope = authority.scope.clone();
    let mut share = authority.clone();
    share.subject = SocketSubjectV1::Share { share_id: "share-plan".into(), selector: "share-selector".into(), scope: share_scope, expires_at_ms: i64::MAX };
    share.revalidation.session_generation = None;
    share.revalidation.share_generation = Some(1);
    share.surface.role = directory::os_directory::DocumentOpenSurfaceRoleV1::Viewer;
    share.grant.write = false;
    assert!(DocumentOpenPlanLedgerV1::default().issue(share.clone(), fixture.now_ms, fixture.now_ms + 100).is_ok());
    share.grant.write = true;
    assert_eq!(DocumentOpenPlanLedgerV1::default().issue(share, fixture.now_ms, fixture.now_ms + 100), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let mut beyond_binding = authority.clone();
    if let SocketSubjectV1::Session { expires_at_ms, .. } = &mut beyond_binding.subject {
        *expires_at_ms = i64::try_from(fixture.now_ms + 50).expect("binding expiry");
    }
    assert_eq!(DocumentOpenPlanLedgerV1::default().issue_with_capability(beyond_binding, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(22)),), Err(DocumentOpenPlanErrorCodeV1::Denied));

    let binding_bounded = DocumentOpenPlanLedgerV1::default();
    for document in 0..DOCUMENT_OPEN_PLAN_BINDING_CAPACITY {
        let scoped = document_open_plan_authority_for_scope(&authority, 1, document as u32);
        binding_bounded.issue_with_capability(scoped, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(100 + document as u32))).expect("within binding capacity");
    }
    let overflow_scope = document_open_plan_authority_for_scope(&authority, 1, DOCUMENT_OPEN_PLAN_BINDING_CAPACITY as u32);
    assert_eq!(binding_bounded.issue_with_capability(overflow_scope, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(500))), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));

    let globally_bounded = DocumentOpenPlanLedgerV1::default();
    for index in 0..DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY {
        let scoped = document_open_plan_authority_for_scope(&authority, (index / DOCUMENT_OPEN_PLAN_BINDING_CAPACITY) as u32, index as u32);
        globally_bounded.issue_with_capability(scoped, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(1_000 + index as u32))).expect("within global capacity");
    }
    let overflow = document_open_plan_authority_for_scope(&authority, 99, DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY as u32);
    assert_eq!(globally_bounded.issue_with_capability(overflow, fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(10_000))), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded));
    assert!(matches!(DocumentOpenPlanCapabilityV1::parse("open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyB"), Err(DocumentOpenPlanErrorCodeV1::Denied)));
}

#[test]
fn document_open_plan_receipt_exchange_mints_one_exact_bounded_socket_grant() {
    let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture");
    let authority = document_open_plan_test_authority(&fixture);
    let plans = Arc::new(DocumentOpenPlanLedgerV1::default());
    let sockets = Arc::new(SocketGrantLedgerV1::default());
    let public = plans.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(30))).expect("route-inaccessible plan fixture");

    let barrier = Arc::new(std::sync::Barrier::new(9));
    let attempts = (0..8)
        .map(|_| {
            let plans = plans.clone();
            let sockets = sockets.clone();
            let barrier = barrier.clone();
            let authority = authority.clone();
            let receipt = public.receipt.clone();
            std::thread::spawn(move || {
                barrier.wait();
                plans.exchange_to_socket_grant(&receipt, &authority, fixture.now_ms + 1, sockets.as_ref())
            })
        })
        .collect::<Vec<_>>();
    barrier.wait();
    let outcomes = attempts.into_iter().map(|attempt| attempt.join().expect("plan exchange race")).collect::<Vec<_>>();
    assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
    assert_eq!(outcomes.iter().filter(|outcome| matches!(outcome, Err(DocumentOpenPlanErrorCodeV1::AlreadyConsumed))).count(), 7);
    let response = outcomes.into_iter().find_map(Result::ok).expect("one socket grant response");
    assert_eq!(response.schema, "semio.hub.socket-grant/v1");
    assert_eq!(response.protocol, SOCKET_PROTOCOL_V1);
    assert_eq!(response.actor_id, authority.server_actor_id);
    assert_eq!(response.expires_at_ms, i64::try_from(fixture.now_ms + 100).expect("fixture expiry"));
    let encoded_response = serde_json::to_string(&response).expect("socket grant response encodes");
    assert!(!encoded_response.contains(&public.receipt));
    assert!(!encoded_response.contains(&authority.descriptor_digest_v1));
    assert!(!encoded_response.contains(&authority.package.component_sha256));
    assert!(!encoded_response.contains(&authority.scope.document_id));
    assert!(!encoded_response.contains("receipt"));
    let socket_capability = SocketGrantCapability::parse(&response.grant).expect("socket grant parses");
    let audience = SocketAudienceV1::Document(authority.scope.clone());
    let pending = sockets.pending(&socket_capability, &audience, i64::try_from(fixture.now_ms + 2).expect("fixture time")).expect("exact pending document grant");
    assert_eq!(pending.actor_id, authority.server_actor_id);
    assert_eq!(pending.subject, authority.subject);
    assert_eq!(pending.document_plan.as_deref(), Some(&authority));
    assert_eq!(pending.expires_at_ms, response.expires_at_ms);
    assert_eq!(sockets.inner.lock().expect("socket ledger").records.len(), 1);
    let plan_digest = DocumentOpenPlanCapabilityV1::parse(&public.receipt).expect("plan receipt parses").digest();
    let plan_record = plans.inner.lock().expect("plan ledger").records.get(&plan_digest).expect("plan record").clone();
    assert_eq!(plan_record.state, DocumentOpenPlanStateV1::Consumed);
    assert_eq!(plan_record.socket_grant_selector.as_deref(), Some(socket_capability.selector()));

    let capacity_plans = DocumentOpenPlanLedgerV1::default();
    let capacity_sockets = SocketGrantLedgerV1::default();
    let capacity_plan = capacity_plans.issue_with_capability(authority.clone(), fixture.now_ms, fixture.now_ms + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(31))).expect("capacity plan");
    for _ in 0..SOCKET_GRANT_BINDING_PENDING_CAPACITY {
        let capability = SocketGrantCapability::mint().expect("capacity socket grant");
        capacity_sockets
            .issue(
                &capability,
                SocketAudienceV1::Document(authority.scope.clone()),
                authority.server_actor_id.clone(),
                authority.subject.clone(),
                i64::try_from(fixture.now_ms).expect("fixture time"),
                i64::try_from(fixture.now_ms + 1_000).expect("fixture expiry"),
            )
            .expect("fill per-binding socket grant capacity");
    }
    assert!(matches!(capacity_plans.exchange_to_socket_grant(&capacity_plan.receipt, &authority, fixture.now_ms + 1, &capacity_sockets), Err(DocumentOpenPlanErrorCodeV1::DeadlineExceeded)));
    let capacity_digest = DocumentOpenPlanCapabilityV1::parse(&capacity_plan.receipt).expect("capacity receipt parses").digest();
    let capacity_record = capacity_plans.inner.lock().expect("capacity plan ledger").records.get(&capacity_digest).expect("capacity plan remains").clone();
    assert_eq!(capacity_record.state, DocumentOpenPlanStateV1::Issued);
    assert_eq!(capacity_record.socket_grant_selector, None);
    capacity_sockets.invalidate_binding(authority.subject.binding());
    assert!(capacity_plans.exchange_to_socket_grant(&capacity_plan.receipt, &authority, fixture.now_ms + 2, &capacity_sockets).is_ok());
}

/// 🪪️ Every execution-target asset route re-authenticates the exact scope and role, reloads the
/// durable descriptor, and resolves the current trusted selection before any body: it accepts
/// only the bounded open intent, never a package/digest/generation/path/receipt selector, serves
/// exactly the selected bytes, and denies after a catalog rotation.
#[tokio::test]
async fn execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body() {
    let mut state = test_state().await;
    let document_id = artifact_document_id_for_test("execution-target-assets");
    let token = seed_author_token(&state).await;
    let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
    let scope = DocumentScope::new(STUDIO, &document_id);
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
    let authorization = format!("Bearer {token}");
    let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
    let root = format!("/spaces/{STUDIO}/documents/{document_id}/execution-target");
    let intent_body = |surface: &str| {
        directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some(surface.into()),
            client_instance_id: "client:execution-target".into(),
        })
    };
    let addr = spawn_server(state.clone()).await;

    let manifest = raw_http_request(addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(manifest.status, 200, "{}", String::from_utf8_lossy(&manifest.body));
    let manifest_text = String::from_utf8(manifest.body).expect("manifest UTF-8");
    assert!(!manifest_text.contains(&token) && !manifest_text.contains("client:execution-target") && !manifest_text.contains("sessionId") && !manifest_text.contains("receipt"));
    let fields: DocumentExecutionTargetLeaseFieldsV1 = directory::os_pack::json::from_json_str(&manifest_text).expect("manifest JSON");
    fields.validate().expect("route manifest is a valid lease projection");
    assert_eq!(fields.scope, scope);
    assert_eq!(fields.catalog.generation_id, state.openable_catalog.as_ref().expect("catalog").generation_id());
    assert_eq!(fields.component.byte_length, TEST_EXECUTION_TARGET_COMPONENT_BYTES.len() as u64);
    assert_eq!(fields.descriptor.byte_length, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.len() as u64);

    let component = raw_http_request(addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(component.status, 200);
    assert_eq!(component.body, TEST_EXECUTION_TARGET_COMPONENT_BYTES);
    let descriptor_body = raw_http_request(addr, "POST", &format!("{root}/descriptor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(descriptor_body.status, 200);
    assert_eq!(descriptor_body.body, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES);

    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json")).expect("closed actor neutral corpus");
    let mut closed_selection = state.openable_catalog.as_ref().unwrap().resolve_document_open(&descriptor, Some("surface.test.editor"), true).unwrap();
    closed_selection.surface.renderer_target = os_directory::DocumentOpenRendererTargetV1::Wasm;
    let mut actor_json = corpus["plan"]["browserActor"].clone();
    actor_json["sha256"] = serde_json::json!(os_directory::hex_lower(&Sha256::digest(b"abc")));
    actor_json["sourceComponentSha256"] = serde_json::json!(closed_selection.package.component_sha256);
    actor_json["sourceDescriptorByteSha256"] = serde_json::json!(closed_selection.package.descriptor_byte_sha256);
    closed_selection.browser_actor = directory::os_pack::json::from_json_str(&actor_json.to_string()).expect("closed actor fixture");
    let expected_actor = closed_selection
        .browser_actor
        .to_lease(os_directory::DocumentBrowserActorSourceV1 { component_sha256: &closed_selection.package.component_sha256, descriptor_byte_sha256: &closed_selection.package.descriptor_byte_sha256 }, "wasm", Some(3))
        .expect("closed actor lease");
    let mut closed_state = state.clone();
    closed_state.openable_catalog = Some(Arc::new(TestDocumentOpenCatalog {
        generation_id: "88".repeat(32),
        open_targets: vec![closed_selection.clone()].into_boxed_slice(),
        component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
        descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
        browser_actor: Some(Arc::from(&b"abc"[..])),
    }));
    let closed_addr = spawn_server(closed_state.clone()).await;
    let closed_manifest = raw_http_request(closed_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(closed_manifest.status, 200);
    let closed_text = String::from_utf8(closed_manifest.body).expect("closed manifest UTF-8");
    let closed_fields: DocumentExecutionTargetLeaseFieldsV1 = directory::os_pack::json::from_json_str(&closed_text).expect("closed manifest");
    closed_fields.validate().expect("closed manifest bound");
    assert_eq!(closed_fields.browser_actor, expected_actor);
    for private in ["path", "moduleUrl", "actorBytes", "receipt", "sessionId", "client:execution-target"] {
        assert!(!closed_text.contains(&format!("\"{private}\"")));
    }
    assert!(!closed_text.contains(&token));
    let closed_plan = raw_http_request(closed_addr, "POST", &format!("/spaces/{STUDIO}/documents/{document_id}/open-plan"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(closed_plan.status, 200);
    let closed_plan: DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&closed_plan.body).unwrap()).expect("closed plan");
    assert_eq!(closed_plan.browser_actor, closed_selection.browser_actor);
    let actor_body = raw_http_request(closed_addr, "POST", &format!("{root}/browser-actor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(actor_body.status, 200);
    assert_eq!(actor_body.body, b"abc");
    let absent_actor = raw_http_request(addr, "POST", &format!("{root}/browser-actor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(absent_actor.status, 503);
    assert_ne!(absent_actor.body, b"abc");
    for (selection, browser_actor) in [(closed_selection, None), (state.openable_catalog.as_ref().unwrap().resolve_document_open(&descriptor, Some("surface.test.editor"), true).unwrap(), Some(Arc::from(&b"abc"[..])))] {
        let mut invalid_state = state.clone();
        invalid_state.openable_catalog = Some(Arc::new(TestDocumentOpenCatalog {
            generation_id: "99".repeat(32),
            open_targets: vec![selection].into_boxed_slice(),
            component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
            descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
            browser_actor,
        }));
        let invalid_addr = spawn_server(invalid_state).await;
        let invalid = raw_http_request(invalid_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(invalid.status, 503, "actor identity/body presence mismatch");
    }

    // 🚫 Unauthenticated, foreign-scope, foreign-surface, query-smuggled, oversized and
    // non-JSON requests never reach a byte.
    for asset in ["manifest", "component", "descriptor", "browser-actor"] {
        let route = format!("{root}/{asset}");
        let anonymous = raw_http_request(addr, "POST", &route, &[("Content-Type", "application/json")], intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(anonymous.status, 401, "{asset} served an unauthenticated caller");
        let foreign_scope = raw_http_request(addr, "POST", &format!("/spaces/{STUDIO}/documents/{document_id}-foreign/execution-target/{asset}"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert!(foreign_scope.status == 400 || foreign_scope.status == 404, "{asset} accepted a foreign scope with status {}", foreign_scope.status);
        let foreign_surface = raw_http_request(addr, "POST", &route, &headers, intent_body("surface.foreign").as_bytes()).await;
        assert_eq!(foreign_surface.status, 503, "{asset} accepted a foreign surface");
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&foreign_surface.body).expect("foreign error")["code"], "component-unavailable");
        let smuggled = raw_http_request(addr, "POST", &format!("{route}?package=semio:fixture"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(smuggled.status, 400, "{asset} accepted a query selector");
        let unknown_field =
            raw_http_request(addr, "POST", &route, &headers, br#"{"schema":"semio.hub.document-open-intent/v1","version":1,"scope":{"spaceId":"studio","documentId":"execution-target-assets"},"clientInstanceId":"c","componentSha256":"aa"}"#).await;
        assert_eq!(unknown_field.status, 400, "{asset} accepted a client-supplied digest selector");
        let oversized = raw_http_request(addr, "POST", &route, &headers, &vec![b'x'; 9 * 1024]).await;
        assert!(oversized.status == 400 || oversized.status == 413, "{asset} accepted an oversized body");
    }

    // 🔁 A rotation between reads denies every subsequent body rather than mixing generations.
    let mut rotated = state.clone();
    rotated.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
    let rotated_addr = spawn_server(rotated.clone()).await;
    let rotated_manifest = raw_http_request(rotated_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(rotated_manifest.status, 200);
    let rotated_fields: DocumentExecutionTargetLeaseFieldsV1 = directory::os_pack::json::from_json_str(&String::from_utf8(rotated_manifest.body).expect("rotated UTF-8")).expect("rotated manifest");
    assert_ne!(rotated_fields.catalog.generation_id, fields.catalog.generation_id);
    assert!(!same_lease_fields_v1(&rotated_fields, &fields));

    // 🚧 An unconfigured catalog advertises nothing and serves nothing.
    let mut unavailable = state.clone();
    unavailable.openable_catalog = None;
    let unavailable_addr = spawn_server(unavailable).await;
    let denied = raw_http_request(unavailable_addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(denied.status, 503);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&denied.body).expect("catalog error")["code"], "catalog-unavailable");
}

#[tokio::test]
async fn execution_target_selection_final_fence_matches_neutral_races() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🪪️execution-target-relay-v1/🔣️.json")).expect("execution-target relay fixture");
    for vector in fixture["fences"].as_array().expect("neutral fences") {
        let mut state = test_state().await;
        let token = seed_author_token(&state).await;
        let document_id = artifact_document_id_for_test("execution-target-fence");
        let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
        let scope = DocumentScope::new(STUDIO, document_id);
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
        let mut headers = bearer_headers(&token);
        headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let gate = Arc::new(TestDocumentOpenPlanIssueGate::default());
        state.document_open_plan_issue_gate = Some(gate.clone());
        let body = Bytes::from(directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some("surface.test.editor".into()),
            client_instance_id: "client:fence".into(),
        }));
        let task = tokio::spawn(document_execution_target_selection(scope.space_id.clone(), scope.document_id.clone(), headers, state.clone(), body));
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.admitted.acquire()).await.expect("target final fence deadline").expect("target reached final fence").forget();
        match vector["mutation"].as_str().expect("mutation") {
            "unchanged" => {}
            "directory-advanced" => announce_document_for_test(&state, STUDIO, "execution-target-racing-document").await,
            "session-revoked" => {
                let capability = SessionCapability::parse(&token).expect("session capability");
                let session = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("session");
                state.directory.revoke_auth_session(&session.id, "target-fence", None, "target-fence").await.expect("revoke session").expect("revoked row");
            }
            "cancelled" => task.abort(),
            _ => panic!("unknown target fence mutation"),
        }
        gate.release.add_permits(1);
        let outcome = match task.await {
            Ok(Ok((fields, assets))) => {
                assert_eq!(fields.scope, scope);
                assert_eq!(&*assets.component, TEST_EXECUTION_TARGET_COMPONENT_BYTES);
                "selected"
            }
            Ok(Err((_, error))) if error.0.code == DocumentOpenPlanErrorCodeV1::Stale => "stale",
            Ok(Err((_, error))) if error.0.code == DocumentOpenPlanErrorCodeV1::Denied => "denied",
            Err(error) if error.is_cancelled() => "cancelled",
            _ => panic!("unexpected execution-target fence outcome"),
        };
        assert_eq!(outcome, vector["expected"].as_str().expect("expected fence outcome"));
        assert!(state.document_open_plans.inner.lock().expect("plan ledger").records.is_empty(), "asset reads never mint or consume plan receipts");
    }
}

#[tokio::test]
async fn document_open_and_execution_target_refuse_descriptor_or_index_without_genesis() {
    for indexed in [false, true] {
        let mut state = test_state().await;
        let token = seed_author_token(&state).await;
        let document_id = artifact_document_id_for_test(if indexed { "indexed-without-genesis" } else { "descriptor-without-genesis" });
        let scope = DocumentScope::new(STUDIO, &document_id);
        announce_document_for_test(&state, STUDIO, &document_id).await;
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor-only document");
        if indexed {
            let session = state.directory.authenticate_session(&SessionCapability::parse(&token).expect("indexed author capability")).await.expect("indexed author session read").expect("indexed author session");
            let event = semio_hub::directory::NewDirectoryEvent {
                hlc: semio_hub::directory::HubClock::new().tick(),
                actor: DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#indexed-without-genesis", session.user_id) },
                space_id: Some(scope.space_id.clone()),
                user_id: Some(session.user_id),
                body: os_directory::DirectoryEventBody::DocumentIndexed {
                    scope: scope.clone(),
                    descriptor_digest_v1: os_directory::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
                    entry: os_directory::DocumentIndexEntryV1 { name: "Indexed without genesis".into(), dialect: directory::os_io::ArtifactDialect { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() } },
                },
            };
            state.directory.append_decided_events(&[event]).await.expect("corrupt indexed-only fixture");
        }
        install_document_open_catalog_for_test(&mut state, &descriptor);
        let mut headers = bearer_headers(&token);
        headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let body = Bytes::from(directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some("surface.test.editor".into()),
            client_instance_id: "client:no-genesis".into(),
        }));
        let authorization = format!("Bearer {token}");
        let raw_headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
        let addr = spawn_server(state.clone()).await;
        for path in [format!("/spaces/{}/documents/{}/open-plan", scope.space_id, scope.document_id), format!("/spaces/{}/documents/{}/execution-target/manifest", scope.space_id, scope.document_id)] {
            let denied = raw_http_request(addr, "POST", &path, &raw_headers, &body).await;
            assert_eq!(denied.status, 404, "{path} must refuse a document without committed genesis");
            let error: serde_json::Value = serde_json::from_slice(&denied.body).expect("closed checkpoint error");
            assert_eq!(error["code"], "not-found");
            assert!(!denied.body.windows(TEST_EXECUTION_TARGET_COMPONENT_BYTES.len()).any(|window| window == TEST_EXECUTION_TARGET_COMPONENT_BYTES));
        }
        assert!(state.document_open_plans.inner.lock().expect("plan ledger").records.is_empty(), "pre-genesis route refusal cannot mint a plan");
        if !indexed {
            let mut no_catalog = state.clone();
            no_catalog.openable_catalog = None;
            let no_catalog_addr = spawn_server(no_catalog).await;
            let denied = raw_http_request(no_catalog_addr, "POST", &format!("/spaces/{}/documents/{}/execution-target/manifest", scope.space_id, scope.document_id), &raw_headers, &body).await;
            assert_eq!(denied.status, 404, "checkpoint absence must be decided before catalog availability");
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&denied.body).expect("closed no-catalog checkpoint error")["code"], "not-found");
        }
        let Err((plan_status, DirectoryJson(plan_error))) = issue_document_open_plan_inner(scope.space_id.clone(), scope.document_id.clone(), headers.clone(), state.clone(), body.clone()).await else {
            panic!("descriptor/index without genesis minted an open plan");
        };
        assert_eq!((plan_status, plan_error.code), (StatusCode::NOT_FOUND, DocumentOpenPlanErrorCodeV1::NotFound));
        let Err((target_status, DirectoryJson(target_error))) = document_execution_target_selection(scope.space_id.clone(), scope.document_id.clone(), headers, state, body).await else {
            panic!("descriptor/index without genesis minted an execution target");
        };
        assert_eq!((target_status, target_error.code), (StatusCode::NOT_FOUND, DocumentOpenPlanErrorCodeV1::NotFound));
    }
    eprintln!("[DEBUG] required-checkpoint-open: descriptor-only and indexed-without-genesis refused before plan or lease");
}

#[tokio::test]
async fn document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable() {
    let mut state = test_state().await;
    let document_id = artifact_document_id_for_test("open-plan-issue");
    let token = seed_author_token(&state).await;
    let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
    let scope = DocumentScope::new(STUDIO, &document_id);
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
    let authorization = format!("Bearer {token}");
    let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
    let plan_route = format!("/spaces/{STUDIO}/documents/{document_id}/open-plan");
    let grant_route = format!("/spaces/{STUDIO}/documents/{document_id}/socket-grants");
    let intent_body = |surface: &str, client: &str| {
        directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: Some(surface.into()), client_instance_id: client.into() })
    };
    let grant_body = |receipt: &str| directory::os_pack::json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: receipt.into() });
    let addr = spawn_server(state.clone()).await;
    let readiness = raw_http_get(addr, "/readyz", &[]).await;
    let readiness_json: serde_json::Value = serde_json::from_slice(&readiness.body).expect("readiness JSON");
    assert_eq!(readiness_json["features"]["openPlan"], true);
    assert_eq!(readiness_json["features"]["openPlanExchange"], true);

    let success = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:private").as_bytes()).await;
    assert_eq!(success.status, 200, "{}", String::from_utf8_lossy(&success.body));
    let success_text = String::from_utf8(success.body).expect("plan UTF-8");
    assert!(!success_text.contains(&token));
    assert!(!success_text.contains("client:private"));
    assert!(!success_text.contains("sessionId"));
    assert!(!success_text.contains("descriptor\""));
    let plan: DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(&success_text).expect("plan JSON");
    assert_eq!(plan.scope, scope);
    assert_eq!(plan.catalog.generation_id, state.openable_catalog.as_ref().expect("catalog").generation_id());
    assert_eq!(plan.parent_dialect, DocumentOpenParentDialectV1 { artifact_kind: descriptor.artifact_kind.clone(), standard: "1".into(), subset: "*".into() });
    assert_eq!(plan.surface.surface_id, "surface.test.editor");
    assert!(plan.grant.write);
    assert!(plan.expires_at_unix_ms.saturating_sub(u64::try_from(now_ms()).expect("time")) <= DOCUMENT_OPEN_PLAN_MAX_TTL_MS);
    let exchange = raw_http_request(addr, "POST", &grant_route, &headers, grant_body(&plan.receipt).as_bytes()).await;
    assert_eq!(exchange.status, 200);
    let exchange_json: serde_json::Value = serde_json::from_slice(&exchange.body).expect("exchange JSON");
    assert_eq!(exchange_json["schema"], "semio.hub.socket-grant/v1");
    assert!(!String::from_utf8(exchange.body).expect("exchange UTF-8").contains(&plan.receipt));

    let foreign_surface = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.foreign", "client:foreign").as_bytes()).await;
    assert_eq!(foreign_surface.status, 503);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&foreign_surface.body).expect("foreign error")["code"], "component-unavailable");
    let hostile = format!(r#"{{"schema":"semio.hub.document-open-intent/v1","version":1,"scope":{{"spaceId":"{STUDIO}","documentId":"{document_id}"}},"clientInstanceId":"client","actor":"caller"}}"#);
    assert_eq!(raw_http_request(addr, "POST", &plan_route, &headers, hostile.as_bytes()).await.status, 400);
    assert_eq!(raw_http_request(addr, "POST", &format!("{plan_route}?surface=surface.test.editor"), &headers, intent_body("surface.test.editor", "client:query").as_bytes()).await.status, 400);
    let wrong_scope = directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
        schema: "semio.hub.document-open-intent/v1".into(),
        version: 1,
        scope: DocumentScope::new(STUDIO, "other"),
        requested_surface_id: Some("surface.test.editor".into()),
        client_instance_id: "client:scope".into(),
    });
    assert_eq!(raw_http_request(addr, "POST", &plan_route, &headers, wrong_scope.as_bytes()).await.status, 400);
    let mut oversized = intent_body("surface.test.editor", "client:oversized").into_bytes();
    oversized.resize(DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES + 1, b' ');
    let oversized_transport = raw_http_request_transport(addr, "POST", &plan_route, &headers, &oversized).await;
    assert!(oversized_transport.is_none_or(|response| response.status == 413));
    let oversized_request =
        axum::http::Request::builder().uri(&plan_route).header(axum::http::header::AUTHORIZATION, &authorization).header(axum::http::header::CONTENT_TYPE, "application/json").body(axum::body::Body::from(oversized)).expect("oversized request");
    assert!(matches!(issue_document_open_plan(OriginalUri(plan_route.parse().expect("plan URI")), Path((STUDIO.into(), document_id.into())), State(state.clone()), oversized_request).await, Err((StatusCode::PAYLOAD_TOO_LARGE, _))));

    let issued_share = state.directory.issue_share_token(&scope, 60, "open-plan-issue-share").await.expect("share issue");
    let share_authorization = format!("Bearer {}", issued_share.capability.expose_once());
    let share_headers = [("Authorization", share_authorization.as_str()), ("Content-Type", "application/json")];
    let share_response = raw_http_request(addr, "POST", &plan_route, &share_headers, intent_body("surface.test.viewer", "client:share").as_bytes()).await;
    assert_eq!(share_response.status, 200);
    let share_plan: DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(&String::from_utf8(share_response.body).expect("share plan UTF-8")).expect("share plan");
    assert_eq!(share_plan.parent_dialect, plan.parent_dialect);
    assert!(!share_plan.grant.write);
    assert_eq!(share_plan.surface.role, os_directory::DocumentOpenSurfaceRoleV1::Viewer);
    assert!(share_plan.revalidation.session_generation.is_none());
    assert_eq!(share_plan.revalidation.share_generation, Some(1));
    assert_eq!(raw_http_request(addr, "POST", &grant_route, &share_headers, grant_body(&share_plan.receipt).as_bytes()).await.status, 200);

    let mut unavailable = state.clone();
    unavailable.openable_catalog = None;
    unavailable.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, false, false));
    let unavailable_addr = spawn_server(unavailable).await;
    let unavailable_readiness = raw_http_get(unavailable_addr, "/readyz", &[]).await;
    let unavailable_readiness: serde_json::Value = serde_json::from_slice(&unavailable_readiness.body).expect("unavailable readiness JSON");
    assert_eq!(unavailable_readiness["features"]["openPlan"], false);
    assert_eq!(unavailable_readiness["features"]["openPlanExchange"], false);
    let unavailable_response = raw_http_request(unavailable_addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:unavailable").as_bytes()).await;
    assert_eq!(unavailable_response.status, 503);
    let unavailable_text = String::from_utf8(unavailable_response.body).expect("unavailable UTF-8");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&unavailable_text).expect("unavailable JSON")["code"], "catalog-unavailable");
    assert!(!unavailable_text.contains("fixture"));

    let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open fixture");
    let mut capacity_authority = document_open_plan_authority_for_session(&state, &fixture, &token, scope.clone()).await;
    capacity_authority.catalog.generation_id = state.openable_catalog.as_ref().expect("catalog").generation_id().into();
    capacity_authority.package = state.openable_catalog.as_ref().expect("catalog").resolve_document_open(&descriptor, Some("surface.test.editor"), true).expect("selection").package;
    capacity_authority.artifact = DocumentOpenArtifactV1 { kind: descriptor.artifact_kind.clone(), schema: descriptor.artifact_schema.clone(), pack_schema_hash: descriptor.pack_schema_hash.clone() };
    capacity_authority.surface = state.openable_catalog.as_ref().expect("catalog").resolve_document_open(&descriptor, Some("surface.test.editor"), true).expect("selection").surface;
    capacity_authority.grant = DocumentOpenGrantV1 { read: true, write: true, observe: true };
    let capacity_now = u64::try_from(now_ms()).expect("capacity time");
    for index in 0..DOCUMENT_OPEN_PLAN_BINDING_CAPACITY {
        let mut authority = capacity_authority.clone();
        authority.scope.document_id = format!("capacity-{index}");
        authority.descriptor.document_id = authority.scope.document_id.clone();
        authority.descriptor_digest_v1 = os_directory::hex_lower(&os_directory::descriptor_digest_v1(&authority.descriptor).expect("capacity digest").0);
        authority.checkpoint.descriptor_digest_v1 = authority.descriptor_digest_v1.clone();
        authority.checkpoint.baseline_frontier.document_id = authority.scope.document_id.clone();
        state.document_open_plans.issue_with_capability(authority, capacity_now, capacity_now + 10_000, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(1_000 + index as u32))).expect("fill plan capacity");
    }
    let capacity_response = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.test.editor", "client:capacity").as_bytes()).await;
    assert_eq!(capacity_response.status, 503);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&capacity_response.body).expect("capacity JSON")["code"], "deadline-exceeded");

    let mut cancelled = test_state().await;
    let cancelled_token = seed_author_token(&cancelled).await;
    let cancelled_document_id = artifact_document_id_for_test("open-plan-cancelled");
    let (cancelled_descriptor, _) = publish_openable_document_for_test(&cancelled, &cancelled_token, STUDIO, &cancelled_document_id).await;
    let cancelled_scope = DocumentScope::new(STUDIO, cancelled_document_id);
    cancelled.openable_catalog = Some(document_open_catalog_for_descriptor(&cancelled_descriptor));
    cancelled.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
    let mut cancelled_headers = bearer_headers(&cancelled_token);
    cancelled_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
    let gate = Arc::new(TestDocumentOpenPlanIssueGate::default());
    cancelled.document_open_plan_issue_gate = Some(gate.clone());
    let cancelled_body = Bytes::from(directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
        schema: "semio.hub.document-open-intent/v1".into(),
        version: 1,
        scope: cancelled_scope.clone(),
        requested_surface_id: Some("surface.test.editor".into()),
        client_instance_id: "client:cancelled".into(),
    }));
    let cancelled_state = cancelled.clone();
    let task = tokio::spawn(issue_document_open_plan_inner(STUDIO.into(), cancelled_scope.document_id, cancelled_headers, cancelled_state, cancelled_body));
    tokio::time::timeout(std::time::Duration::from_secs(2), gate.admitted.acquire()).await.expect("issuer publication fence deadline").expect("issuer reached publication fence").forget();
    task.abort();
    assert!(matches!(task.await, Err(error) if error.is_cancelled()));
    assert!(cancelled.document_open_plans.inner.lock().expect("cancelled ledger").records.is_empty());
}

#[tokio::test]
async fn document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint() {
    let mut state = test_state().await;
    let document_id = artifact_document_id_for_test("open-plan-consume");
    let token = seed_author_token(&state).await;
    let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
    let scope = DocumentScope::new(STUDIO, &document_id);
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));

    let (_, surface_grant) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:surface").await;
    let surface_capability = SocketGrantCapability::parse(&surface_grant.grant).expect("surface grant");
    let mut surface_headers = HeaderMap::new();
    surface_headers.insert(axum::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {}", surface_grant.grant).parse().expect("surface protocol"));
    assert!(matches!(consume_socket_grant(&state, &surface_headers, SocketAudienceV1::Document(scope.clone()), Some("surface.test.viewer")).await, Err(StatusCode::UNAUTHORIZED)));
    assert!(state.socket_grants.pending(&surface_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).is_err(), "surface substitution terminally rejects the pending grant");

    let (_, checkpoint_grant) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:checkpoint").await;
    let checkpoint_capability = SocketGrantCapability::parse(&checkpoint_grant.grant).expect("checkpoint grant");
    let mut checkpoint_headers = HeaderMap::new();
    checkpoint_headers.insert(axum::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {}", checkpoint_grant.grant).parse().expect("checkpoint protocol"));
    publish_checkpoint_for_test(&state, STUDIO, &document_id).await;
    assert!(matches!(consume_socket_grant(&state, &checkpoint_headers, SocketAudienceV1::Document(scope.clone()), Some("surface.test.editor")).await, Err(StatusCode::UNAUTHORIZED)));
    assert!(state.socket_grants.pending(&checkpoint_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).is_err(), "revision/checkpoint change terminally rejects the pending grant");

    let (_, catalog_grant) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:catalog").await;
    let catalog_capability = SocketGrantCapability::parse(&catalog_grant.grant).expect("catalog grant");
    let mut catalog_headers = HeaderMap::new();
    catalog_headers.insert(axum::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {}", catalog_grant.grant).parse().expect("catalog protocol"));
    state.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
    assert!(matches!(consume_socket_grant(&state, &catalog_headers, SocketAudienceV1::Document(scope.clone()), Some("surface.test.editor")).await, Err(StatusCode::UNAUTHORIZED)));
    assert!(state.socket_grants.pending(&catalog_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).is_err(), "catalog change terminally rejects the pending grant");

    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    let (_, exact_grant) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:exact").await;
    let exact_capability = SocketGrantCapability::parse(&exact_grant.grant).expect("exact grant");
    let pending = state.socket_grants.pending(&exact_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).expect("exact pending grant");
    let authority = pending.document_plan.as_ref().expect("retained plan authority");
    let mut hostile_descriptor = pending.clone();
    Arc::make_mut(hostile_descriptor.document_plan.as_mut().expect("descriptor authority")).descriptor_digest_v1 = "00".repeat(32);
    assert_eq!(document_plan_socket_validity(&state, &hostile_descriptor, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized);
    let mut hostile_revision = pending.clone();
    Arc::make_mut(hostile_revision.document_plan.as_mut().expect("revision authority")).revalidation.directory_revision += 1;
    assert_eq!(document_plan_socket_validity(&state, &hostile_revision, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized);
    let mut hostile_checkpoint = pending.clone();
    Arc::make_mut(hostile_checkpoint.document_plan.as_mut().expect("checkpoint authority")).checkpoint.descriptor_digest_v1 = "00".repeat(32);
    assert_eq!(document_plan_socket_validity(&state, &hostile_checkpoint, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized);
    for field in ["artifactKind", "standard", "subset"] {
        let mut hostile = pending.clone();
        let dialect = &mut Arc::make_mut(hostile.document_plan.as_mut().expect("dialect authority")).parent_dialect;
        match field {
            "artifactKind" => dialect.artifact_kind.push_str(".foreign"),
            "standard" => dialect.standard.push_str("-foreign"),
            "subset" => dialect.subset.push_str("-foreign"),
            _ => unreachable!(),
        }
        assert_eq!(document_plan_socket_validity(&state, &hostile, Some("surface.test.editor")).await, SocketBindingValidityV1::Unauthorized, "foreign parent {field}");
    }
    assert_eq!(authority.scope, scope);
    let mut exact_headers = HeaderMap::new();
    exact_headers.insert(axum::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {}", exact_grant.grant).parse().expect("exact protocol"));
    let admission = consume_socket_grant(&state, &exact_headers, SocketAudienceV1::Document(scope), Some("surface.test.editor")).await.expect("exact current authority consumes");
    assert_eq!(admission.record.document_plan.as_deref(), Some(authority.as_ref()));
    eprintln!("[DEBUG] open-plan socket full parent dialect:3 substitutions denied; exact sealed selection retained");
}

#[tokio::test]
async fn document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use() {
    let mut state = test_state().await;
    let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture");
    let document_id = "open-plan-route";
    let foreign_document_id = "open-plan-route-foreign";
    announce_document_for_test(&state, STUDIO, document_id).await;
    announce_document_for_test(&state, STUDIO, foreign_document_id).await;
    let route_descriptor = state.directory.get_document_descriptor(&DocumentScope::new(STUDIO, document_id)).await.expect("route descriptor").expect("route document");
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&route_descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
    let token = seed_author_token(&state).await;
    let scope = DocumentScope::new(STUDIO, document_id);
    let mut authority = document_open_plan_authority_for_session(&state, &fixture, &token, scope.clone()).await;
    let addr = spawn_server(state.clone()).await;
    let route = format!("/spaces/{STUDIO}/documents/{document_id}/socket-grants");
    let authorization = format!("Bearer {token}");
    let request_headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
    let request_body = |receipt: &str| directory::os_pack::json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: receipt.into() });
    let readiness = raw_http_get(addr, "/readyz", &[]).await;
    let readiness_json: serde_json::Value = serde_json::from_slice(&readiness.body).expect("readiness JSON");
    assert_eq!(readiness_json["features"]["openPlan"], true, "verified catalog-backed plan issuance is advertised with exchange");
    assert_eq!(readiness_json["features"]["openPlanExchange"], true);

    let now = u64::try_from(now_ms()).expect("nonnegative route time");
    let public = state.document_open_plans.issue_with_capability(authority.clone(), now, now + 10_000, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(40))).expect("private issuer fixture");
    let success = raw_http_request(addr, "POST", &route, &request_headers, request_body(&public.receipt).as_bytes()).await;
    assert_eq!(success.status, 200);
    let encoded_success = String::from_utf8(success.body.clone()).expect("success UTF-8");
    assert!(!encoded_success.contains(&public.receipt));
    assert!(!encoded_success.contains(&authority.descriptor_digest_v1));
    if let SocketSubjectV1::Session { session_id, user_id, .. } = &authority.subject {
        assert!(!encoded_success.contains(session_id));
        assert!(!encoded_success.contains(user_id));
    }
    let success_json: serde_json::Value = serde_json::from_slice(&success.body).expect("socket grant JSON");
    assert_eq!(success_json["schema"], "semio.hub.socket-grant/v1");
    assert_eq!(success_json["protocol"], SOCKET_PROTOCOL_V1);
    assert_eq!(success_json["actorId"], authority.server_actor_id);
    let socket_capability = SocketGrantCapability::parse(success_json["grant"].as_str().expect("socket grant")).expect("socket grant grammar");
    let pending = state.socket_grants.pending(&socket_capability, &SocketAudienceV1::Document(scope.clone()), now_ms()).expect("route-bound pending grant");
    assert_eq!(pending.subject, authority.subject);
    assert_eq!(pending.document_plan.as_deref(), Some(&authority));

    let replay = raw_http_request(addr, "POST", &route, &request_headers, request_body(&public.receipt).as_bytes()).await;
    assert_eq!(replay.status, 409);
    let replay_json: serde_json::Value = serde_json::from_slice(&replay.body).expect("replay error JSON");
    assert_eq!(replay_json, serde_json::json!({ "schema": "semio.hub.document-open-plan-error/v1", "code": "already-consumed" }));
    assert!(!String::from_utf8(replay.body).expect("replay UTF-8").contains(&public.receipt));

    let foreign = issue_test_session(&state, "open-plan-route-foreign@example.com").await;
    upsert_member_for_test(&state, STUDIO, "open-plan-route-foreign@example.com", DirectorySpaceRole::Author).await;
    let current_revision = state.directory.head_seq().await.expect("current directory revision");
    authority.revalidation.directory_revision = current_revision;
    authority.revalidation.membership_generation = current_revision;
    let foreign_authorization = format!("Bearer {}", foreign.token);
    let foreign_headers = [("Authorization", foreign_authorization.as_str()), ("Content-Type", "application/json")];
    let bound = state.document_open_plans.issue_with_capability(authority.clone(), now + 1, now + 10_001, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(41))).expect("binding fixture plan");
    let wrong_binding = raw_http_request(addr, "POST", &route, &foreign_headers, request_body(&bound.receipt).as_bytes()).await;
    assert_eq!(wrong_binding.status, 401);
    let wrong_binding_json: serde_json::Value = serde_json::from_slice(&wrong_binding.body).expect("binding error JSON");
    assert_eq!(wrong_binding_json["code"], "denied");
    let correct_after_foreign = raw_http_request(addr, "POST", &route, &request_headers, request_body(&bound.receipt).as_bytes()).await;
    assert_eq!(correct_after_foreign.status, 200, "foreign authentication cannot consume the exact receipt");

    let scoped = state.document_open_plans.issue_with_capability(authority.clone(), now + 2, now + 10_002, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(42))).expect("scope fixture plan");
    let foreign_route = format!("/spaces/{STUDIO}/documents/{foreign_document_id}/socket-grants");
    let wrong_scope = raw_http_request(addr, "POST", &foreign_route, &request_headers, request_body(&scoped.receipt).as_bytes()).await;
    assert_eq!(wrong_scope.status, 401);
    assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&scoped.receipt).as_bytes()).await.status, 200, "foreign path cannot consume the exact receipt");

    let strict = state.document_open_plans.issue_with_capability(authority.clone(), now + 3, now + 10_003, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(43))).expect("strict request fixture plan");
    let hostile = format!(r#"{{"schema":"semio.hub.document-plan-socket-grant-intent/v1","version":1,"planReceipt":"{}","actor":"caller-selected"}}"#, strict.receipt);
    let unknown_field = raw_http_request(addr, "POST", &route, &request_headers, hostile.as_bytes()).await;
    assert_eq!(unknown_field.status, 400);
    assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&strict.receipt).as_bytes()).await.status, 200, "rejected unknown authority field cannot consume the receipt");

    let query_plan = state.document_open_plans.issue_with_capability(authority.clone(), now + 4, now + 10_004, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(44))).expect("query fixture plan");
    assert_eq!(raw_http_request(addr, "POST", &format!("{route}?receipt=forbidden"), &request_headers, request_body(&query_plan.receipt).as_bytes()).await.status, 400);
    assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&query_plan.receipt).as_bytes()).await.status, 200, "query rejection cannot consume the body receipt");

    let bounded = state.document_open_plans.issue_with_capability(authority.clone(), now + 5, now + 10_005, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(45))).expect("bounded request fixture plan");
    let mut oversized = request_body(&bounded.receipt).into_bytes();
    oversized.resize(DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES + 1, b' ');
    let oversized_response = raw_http_request(addr, "POST", &route, &request_headers, &oversized).await;
    assert_eq!(oversized_response.status, 413);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&oversized_response.body).expect("bounded error JSON")["code"], "denied");
    assert_eq!(raw_http_request(addr, "POST", &route, &request_headers, request_body(&bounded.receipt).as_bytes()).await.status, 200, "oversized body cannot consume the receipt");

    let issued_share = state.directory.issue_share_token(&scope, 60, "open-plan-route-share").await.expect("share issue");
    let share_token = issued_share.capability.expose_once();
    let mut share_authority = authority.clone();
    share_authority.subject = SocketSubjectV1::Share { share_id: issued_share.record.id, selector: issued_share.record.selector, scope: scope.clone(), expires_at_ms: issued_share.record.expires_at };
    share_authority.revalidation.session_generation = None;
    share_authority.revalidation.share_generation = Some(1);
    let share_selection = state.openable_catalog.as_ref().expect("catalog").resolve_document_open(&share_authority.descriptor, Some("surface.test.viewer"), false).expect("share selection");
    share_authority.package = share_selection.package;
    share_authority.artifact = share_selection.artifact;
    share_authority.parent_dialect = share_selection.parent_dialect;
    share_authority.surface = share_selection.surface;
    share_authority.grant = share_selection.grant;
    share_authority.server_actor_id = socket_actor_id(&[0x44; 32], false);
    share_authority.validate().expect("share route authority");
    let share_plan = state.document_open_plans.issue_with_capability(share_authority.clone(), now + 6, now + 10_006, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(46))).expect("share fixture plan");
    let share_authorization = format!("Bearer {share_token}");
    let share_headers = [("Authorization", share_authorization.as_str()), ("Content-Type", "application/json")];
    let share_response = raw_http_request(addr, "POST", &route, &share_headers, request_body(&share_plan.receipt).as_bytes()).await;
    assert_eq!(share_response.status, 200);
    let share_json: serde_json::Value = serde_json::from_slice(&share_response.body).expect("share socket grant JSON");
    assert_eq!(share_json["actorId"], share_authority.server_actor_id);
    let share_socket = SocketGrantCapability::parse(share_json["grant"].as_str().expect("share grant")).expect("share grant grammar");
    let share_pending = state.socket_grants.pending(&share_socket, &SocketAudienceV1::Document(scope), now_ms()).expect("share pending grant");
    assert_eq!(share_pending.document_plan.as_deref(), Some(&share_authority));
    assert!(!share_pending.document_plan.as_ref().expect("share plan").grant.write);

    let invalid_receipt = request_body("open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyB");
    let invalid = raw_http_request(addr, "POST", &route, &request_headers, invalid_receipt.as_bytes()).await;
    assert_eq!(invalid.status, 400);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&invalid.body).expect("invalid error JSON")["code"], "denied");
}

#[test]
fn document_open_plan_late_invalid_receipt_wipes_exact_candidate_bytes() {
    DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS.lock().expect("wipe observations").clear();
    let mut hostile = "open.v1.AQIDBAUGBwgJCgsMDQ4PEBESExQVFhcYGRobHB0eHyA".to_string();
    hostile.pop();
    hostile.push('!');
    DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING.with(|observing| observing.set(true));
    let result = DocumentOpenPlanCapabilityV1::parse(&hostile);
    DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING.with(|observing| observing.set(false));
    assert!(matches!(result, Err(DocumentOpenPlanErrorCodeV1::Denied)));
    let observations = std::mem::take(&mut *DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS.lock().expect("wipe observations"));
    assert_eq!(observations, vec![DocumentOpenPlanDecodeWipeObservationV1 { nonzero_before: 31, after: [0; 32] }]);
}

#[tokio::test]
async fn document_open_plan_admin_revocation_invalidates_session_and_share_bindings() {
    let state = test_state().await;
    let fixture: DocumentOpenPlanLedgerFixture = directory::os_pack::json::from_json_str(include_str!("../../../🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("document open plan fixture");
    let principal = AdminPrincipalV1 {
        user_id: "open-plan-admin".into(),
        auth_session_id: "open-plan-admin-session".into(),
        authorization_generation: 1,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: i64::MAX,
        correlation_id: "open-plan-admin-revocation".into(),
        peer_class: "test",
    };

    let session = issue_test_session(&state, "open-plan-revoked-session@example.com").await;
    let session_capability = SessionCapability::parse(&session.token).expect("session capability");
    let session_record = state.directory.authenticate_session(&session_capability).await.expect("session lookup").expect("active session");
    let session_now = u64::try_from(now_ms()).expect("nonnegative session time");
    let mut session_authority = document_open_plan_test_authority(&fixture);
    session_authority.revalidation.session_generation = Some(session_record.authorization_generation);
    session_authority.subject = SocketSubjectV1::Session {
        session_id: session_record.id.clone(),
        user_id: session_record.user_id.clone(),
        authorization_generation: session_record.authorization_generation,
        role: Some(SpaceRole::Author),
        expires_at_ms: session_record.expires_at,
    };
    let session_plan = state.document_open_plans.issue_with_capability(session_authority.clone(), session_now, session_now + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(20))).expect("session plan");
    let session_intent = AdminIntentV1::RevokeUserSessions { request_id: "request:open-plan-session-revoke".into(), user_id: session_record.user_id, reason_code: "test-revoke".into() };
    let session_digest = admin_intent_digest(&session_intent);
    let mut session_revoke_authority = None;
    let session_revoke = execute_admin_intent(&state, &principal, "operation:open-plan-session-revoke", &session_digest, session_intent, None, &mut session_revoke_authority).await;
    drop(session_revoke_authority);
    assert_eq!(session_revoke.phase, "succeeded");
    assert_eq!(state.document_open_plans.exchange(&session_plan.receipt, &session_authority, session_now + 1, "socket-after-session-revoke"), Err(DocumentOpenPlanErrorCodeV1::Stale));

    let issued_share = state.directory.issue_share_token(&fixture.valid_plan.scope, 60, "open-plan-share").await.expect("share issue");
    let mut share_authority = document_open_plan_test_authority(&fixture);
    share_authority.grant.write = false;
    share_authority.surface.role = os_directory::DocumentOpenSurfaceRoleV1::Viewer;
    share_authority.revalidation.session_generation = None;
    share_authority.revalidation.share_generation = Some(1);
    share_authority.subject = SocketSubjectV1::Share { share_id: issued_share.record.id.clone(), selector: issued_share.record.selector.clone(), scope: issued_share.record.scope.clone(), expires_at_ms: issued_share.record.expires_at };
    let share_now = u64::try_from(now_ms()).expect("nonnegative share time");
    let share_plan = state.document_open_plans.issue_with_capability(share_authority.clone(), share_now, share_now + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(21))).expect("share plan");
    let share_intent = AdminIntentV1::RevokeDocumentShare { request_id: "request:open-plan-share-revoke".into(), scope: issued_share.record.scope, share_id: issued_share.record.id, reason_code: "test-revoke".into() };
    let share_digest = admin_intent_digest(&share_intent);
    let mut share_revoke_authority = None;
    let share_revoke = execute_admin_intent(&state, &principal, "operation:open-plan-share-revoke", &share_digest, share_intent, None, &mut share_revoke_authority).await;
    drop(share_revoke_authority);
    assert_eq!(share_revoke.phase, "succeeded");
    assert_eq!(state.document_open_plans.exchange(&share_plan.receipt, &share_authority, share_now + 1, "socket-after-share-revoke"), Err(DocumentOpenPlanErrorCodeV1::Stale));
}

#[test]
fn socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live() {
    run_socket_test(|| async {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-a").await;
        announce_document_for_test(&state, STUDIO, "socket-b").await;
        let nonmember = issue_test_session(&state, "socket-nonmember@example.com").await;
        let unauthorized_existing = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&nonmember.token), State(state.clone())).await.err();
        let unauthorized_missing = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-missing".to_string())), bearer_headers(&nonmember.token), State(state.clone())).await.err();
        assert_eq!(unauthorized_existing, Some(StatusCode::UNAUTHORIZED));
        assert_eq!(unauthorized_missing, unauthorized_existing, "unauthorized callers cannot enumerate descriptor existence");
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue document socket grant").0;
        assert_eq!(receipt.schema, "semio.hub.socket-grant/v1");
        assert_eq!(receipt.protocol, SOCKET_PROTOCOL_V1);
        assert_eq!(receipt.grant.len(), 107);
        assert!(receipt.grant.starts_with("socket.v1."));
        assert!(receipt.actor_id.starts_with("hub.v1."));
        assert!(!receipt.actor_id.contains(receipt.grant.rsplit('.').next().expect("secret")));

        let addr = spawn_server(state.clone()).await;
        let rejected = connect_async(socket_request(&format!("ws://{addr}/spaces/{STUDIO}/documents/socket-b/socket/v1"), &receipt.grant)).await.expect_err("cross-document grant rejected");
        assert!(matches!(rejected, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));

        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-a/socket/v1");
        let (mut socket, response) = connect_async(socket_request(&url, &receipt.grant)).await.expect("upgrade socket grant");
        assert_eq!(response.headers().get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL).and_then(|value| value.to_str().ok()), Some(SOCKET_PROTOCOL_V1));
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { actor, .. } if actor == receipt.actor_id));

        let document = db_artifact_id(&DocumentScope::new(STUDIO, "socket-a"));
        let mut forged = sample_envelope("forged-actor", &document).await;
        forged.actor = ActorId("client-selected-forgery".into());
        socket.send(client_binary(&ClientFrame::Commands { batch_id: 77, envelopes: vec![forged] }, Lane::Command).await).await.expect("forged command");
        match next_server_frame(&mut socket).await {
            ServerFrame::Ack { batch_id: 77, stages, .. } => match &stages[0] {
                AckStage::Applied { outcome } => match outcome.as_ref() {
                    ApplyOutcome::Rejected { reason, .. } => {
                        assert_eq!(reason, "socket subject actor mismatch");
                        assert!(!reason.contains(&receipt.grant));
                    }
                    other => panic!("forged actor was not rejected: {other:?}"),
                },
                other => panic!("unexpected forged actor stage: {other:?}"),
            },
            other => panic!("expected forged actor ack, got {other:?}"),
        }

        let legacy_receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue legacy-carrier rejection grant").0;
        let (mut legacy, _) = connect_async(socket_request(&url, &legacy_receipt.grant)).await.expect("legacy rejection socket");
        legacy.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("initial socket hello");
        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut legacy).await, ServerFrame::Session { .. }));
        legacy.send(WsMessage::Binary(vec![0, 0].into())).await.expect("legacy tag-zero frame");
        assert_eq!(next_close_code(&mut legacy, false).await, 4401, "v1 rejects the legacy actor/token carrier after upgrade");

        let replay = connect_async(socket_request(&url, &receipt.grant)).await.expect_err("consumed grant replay rejected");
        assert!(matches!(replay, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));
        let pending = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-a".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue pending grant").0;
        assert_eq!(pending.actor_id, receipt.actor_id, "session-derived actor is stable across grants");

        assert_eq!(delete_session_me(bearer_headers(&token), State(state.clone())).await, StatusCode::NO_CONTENT);
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "successful durable revoke immediately invalidates a live socket");
        let revoked_pending = connect_async(socket_request(&url, &pending.grant)).await.expect_err("pending grant invalidated by revoke");
        assert!(matches!(revoked_pending, tokio_tungstenite::tungstenite::Error::Http(response) if response.status().as_u16() == 401));
    });
}

#[test]
fn socket_grant_revoke_and_welcome_have_a_bounded_binding_linearization() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-linearized").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-linearized".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-linearized/socket/v1");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_before_welcome.acquire()).await.expect("pre-welcome gate deadline").expect("pre-welcome gate");
        let mut revoke = tokio::spawn({
            let state = state.clone();
            let token = token.clone();
            async move { delete_session_me(bearer_headers(&token), State(state)).await }
        });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "revoke waits while Welcome owns the binding linearization");
        gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }), "Welcome linearizes before the waiting revoke");
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_after_welcome.acquire()).await.expect("post-Welcome boundary deadline").expect("post-Welcome boundary");
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("bounded revoke completion").expect("revoke task"), StatusCode::NO_CONTENT);
        gate.socket_bootstrap_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "a revoke winning after Welcome suppresses bootstrap and Session authority");
    });
}

#[test]
fn socket_grant_revoke_before_command_admission_has_no_storage_effect() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let live_gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(live_gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-command-revoke").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-command-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-command-revoke/socket/v1");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
        live_gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
        live_gate.socket_bootstrap_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
        live_gate.document_release.add_permits(1);

        let document = db_artifact_id(&DocumentScope::new(STUDIO, "socket-command-revoke"));
        let mut accepted = sample_envelope("accepted-op", &document).await;
        accepted.actor = ActorId(receipt.actor_id.clone());
        socket.send(client_binary(&ClientFrame::Commands { batch_id: 90, envelopes: vec![accepted] }, Lane::Command).await).await.expect("control command received by server");
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("control command boundary deadline").expect("control command boundary");
        live_gate.socket_command_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Ack { batch_id: 90, .. }));
        let accepted_frontier = state.db.document(&document).await.expect("document handle").frontier().await.expect("accepted frontier");
        assert_eq!(accepted_frontier.head_seq, 1, "an actor-matching command persists while authorized");

        let mut revoked = sample_envelope("revoked-op", &document).await;
        revoked.actor = ActorId(receipt.actor_id.clone());
        socket.send(client_binary(&ClientFrame::Commands { batch_id: 91, envelopes: vec![revoked] }, Lane::Command).await).await.expect("revoked command received by server");
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("command boundary deadline").expect("command boundary");
        assert_eq!(delete_session_me(bearer_headers(&token), State(state.clone())).await, StatusCode::NO_CONTENT);
        live_gate.socket_command_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "no Ack crosses a revoke that wins before command admission");
        let frontier = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier");
        assert_eq!(frontier.head_seq, 1, "the revoked actor-matching command never reaches durable storage");
    });
}

#[test]
fn socket_grant_revoke_before_lag_authorization_reads_no_private_control() {
    run_socket_test(|| async {
        let mut state = test_state_with_capacity(1024, 1).await;
        let live_gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(live_gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-lag-revoke").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-lag-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-lag-revoke/socket/v1");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
        live_gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
        live_gate.socket_bootstrap_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
        let fanout = state.fanout_for(&document_scope_key_v1(&DocumentScope::new(STUDIO, "socket-lag-revoke")));
        fanout.send(ServerFrame::Presence { peers: vec![b"first".to_vec()] }).expect("first fanout");
        fanout.send(ServerFrame::Presence { peers: vec![b"second".to_vec()] }).expect("second fanout");
        live_gate.document_release.add_permits(1);
        tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_lag_received.acquire()).await.expect("lag boundary deadline").expect("lag boundary");
        assert_eq!(delete_session_me(bearer_headers(&token), State(state)).await, StatusCode::NO_CONTENT);
        live_gate.socket_lag_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "revoked lag path discloses no control frame");
        assert_eq!(live_gate.socket_rebootstrap_read.available_permits(), 0, "revoked lag path never enters the private checkpoint/control read");
    });
}

#[test]
fn socket_grant_revoke_before_broadcast_authorization_suppresses_frame() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let live_gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(live_gate.clone());
        let token = seed_author_token(&state).await;
        announce_document_for_test(&state, STUDIO, "socket-broadcast-revoke").await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-broadcast-revoke".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/socket-broadcast-revoke/socket/v1");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("socket upgrade");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome");
        live_gate.socket_welcome_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome");
        live_gate.socket_bootstrap_release.add_permits(1);
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription");
        let fanout = state.fanout_for(&document_scope_key_v1(&DocumentScope::new(STUDIO, "socket-broadcast-revoke")));
        fanout.send(ServerFrame::Presence { peers: vec![b"private-presence".to_vec()] }).expect("fanout");
        live_gate.document_release.add_permits(1);
        tokio::time::timeout(std::time::Duration::from_secs(2), live_gate.socket_broadcast_received.acquire()).await.expect("broadcast boundary deadline").expect("broadcast boundary");
        assert_eq!(delete_session_me(bearer_headers(&token), State(state)).await, StatusCode::NO_CONTENT);
        live_gate.socket_broadcast_release.add_permits(1);
        assert_eq!(next_close_without_authority(&mut socket).await, 4401, "a broadcast received before a winning revoke is never disclosed afterward");
    });
}

#[test]
fn socket_grant_directory_route_uses_credential_free_hello_and_revokes_live() {
    run_socket_test(|| async {
        let state = test_state().await;
        let token = seed_author_token(&state).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&token), State(state.clone())).await.expect("issue directory socket grant").0;
        let since = state.directory.head_seq().await.expect("directory head");
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("directory socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("credential-free hello");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#directory-socket-law".into() }, DirectoryCommand::RenameSpace { space_id: STUDIO.into(), name: "Socket Grant Studio".into() })
            .await
            .expect("member-visible directory event");
        assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if event.space_id.as_deref() == Some(STUDIO)));
        assert_eq!(delete_session_me(bearer_headers(&token), State(state)).await, StatusCode::NO_CONTENT);
        assert_eq!(next_close_code(&mut socket, false).await, 4401);
    });
}

#[tokio::test]
async fn scoped_directory_socket_ledger_indexes_and_invalidates_exact_membership() {
    let ledger = SocketGrantLedgerV1::default();
    let scope = DocumentScope::new("space-a", "document-a");
    let audience = SocketAudienceV1::DirectoryScoped(scope.clone());
    let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Spectator), expires_at_ms: 10_000 };
    let capability = SocketGrantCapability::mint().expect("scoped capability");
    ledger.issue(&capability, audience.clone(), "hub.v1.scoped".into(), subject.clone(), 1, 9_000).expect("scoped issue");
    let pending = ledger.pending(&capability, &audience, 2).expect("pending scoped grant");
    assert_eq!(
        pending.bindings(),
        vec![
            SocketBindingKeyV1::User("user-a".into()),
            SocketBindingKeyV1::Session("session-a".into()),
            SocketBindingKeyV1::DirectorySpaceAuthority { space_id: "space-a".into() },
            SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() },
        ]
    );
    let consumed = ledger.consume(&pending, 3).expect("consume scoped grant");
    let (live_id, notify) = ledger.register_live(&consumed).expect("register scoped live lease");
    assert!(ledger.is_live(&consumed, &live_id));
    ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() });
    assert!(!ledger.is_live(&consumed, &live_id));
    tokio::time::timeout(std::time::Duration::from_millis(50), notify.notified()).await.expect("membership invalidation notifies once");

    let later = SocketGrantCapability::mint().expect("later scoped capability");
    ledger.issue(&later, audience.clone(), "hub.v1.later".into(), subject, 4, 9_000).expect("later issue");
    ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "user-a".into(), space_id: "space-a".into() });
    assert!(ledger.pending(&later, &audience, 5).is_err(), "membership invalidation also removes pending grants");
}

#[test]
fn scoped_directory_socket_message_matching_is_body_exact_and_removal_private() {
    let scope = DocumentScope::new("space-a", "document-a");
    let event = |body: os_directory::DirectoryEventBody| DirectoryStreamMessage::Event {
        event: DirectoryEvent {
            seq: 1,
            id: "event-a".into(),
            hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:test".into() },
            space_id: Some("space-a".into()),
            user_id: None,
            body,
            recorded_at_ms: 1,
        },
    };
    assert!(directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::DocumentAnnounced { descriptor: document_descriptor_for_test("space-a", "document-a") })));
    assert!(!directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::DocumentAnnounced { descriptor: document_descriptor_for_test("space-a", "document-b") })));
    assert!(!directory_message_matches_scope(&scope, &event(os_directory::DirectoryEventBody::MemberRemoved { space_id: "space-a".into(), user_id: "user-a".into() })));
    assert!(!directory_message_matches_scope(&scope, &DirectoryStreamMessage::Heartbeat { head_seq: 99 }));
    assert!(directory_message_matches_scope(&scope, &DirectoryStreamMessage::Presence { space_id: "space-a".into(), document_id: "document-a".into(), actors: Vec::new() }));
    assert!(!directory_message_matches_scope(&scope, &DirectoryStreamMessage::Presence { space_id: "space-a".into(), document_id: "document-b".into(), actors: Vec::new() }));
}

#[test]
fn scoped_directory_socket_route_rejects_scope_substitution_and_rest_removal_closes_without_event() {
    run_socket_test(|| async {
        let state = test_state().await;
        let owner_token = seed_author_token(&state).await;
        let member = issue_test_session(&state, "scoped-member@example.com").await;
        upsert_member_for_test(&state, STUDIO, "scoped-member@example.com", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, STUDIO, "scoped-document-a").await;
        announce_document_for_test(&state, STUDIO, "scoped-document-b").await;
        let unaffected = issue_test_session(&state, "scoped-unaffected@example.com").await;
        let unaffected_space = create_space_for_test(&state, &unaffected.user_id, "Scoped Unaffected", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        announce_document_for_test(&state, &unaffected_space, "scoped-unaffected-document").await;
        let addr = spawn_server(state.clone()).await;
        let authorization = format!("Bearer {}", member.token);
        let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-document-a/socket-grants");
        let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(issued.status, 200);
        let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("scoped grant JSON");
        let grant = receipt["grant"].as_str().expect("scoped grant");
        let substituted = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-document-b/socket/v1?since=0");
        let error = connect_async(socket_request(&substituted, grant)).await.expect_err("scope substitution rejected before upgrade");
        assert!(matches!(error, tokio_tungstenite::tungstenite::Error::Http(response) if response.status() == StatusCode::UNAUTHORIZED));

        let since = state.directory.head_seq().await.expect("directory head");
        let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-document-a/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, grant)).await.expect("exact scoped socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("scoped socket hello");
        let unaffected_authorization = format!("Bearer {}", unaffected.token);
        let unaffected_issue_path = format!("/directory/spaces/{unaffected_space}/documents/scoped-unaffected-document/socket-grants");
        let unaffected_issued = raw_http_request(addr, "POST", &unaffected_issue_path, &[("Authorization", &unaffected_authorization)], &[]).await;
        assert_eq!(unaffected_issued.status, 200);
        let unaffected_receipt: serde_json::Value = serde_json::from_slice(&unaffected_issued.body).expect("unaffected scoped grant JSON");
        let unaffected_grant = unaffected_receipt["grant"].as_str().expect("unaffected scoped grant");
        let unaffected_url = format!("ws://{addr}/directory/spaces/{unaffected_space}/documents/scoped-unaffected-document/socket/v1?since={since}");
        let (mut unaffected_socket, _) = connect_async(socket_request(&unaffected_url, unaffected_grant)).await.expect("unaffected scoped socket");
        unaffected_socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("unaffected scoped hello");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        let pending = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(pending.status, 200);
        let pending_receipt: serde_json::Value = serde_json::from_slice(&pending.body).expect("pending scoped grant JSON");
        let pending_grant = pending_receipt["grant"].as_str().expect("pending scoped grant").to_string();
        announce_document_for_test(&state, STUDIO, "scoped-document-c").await;
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), socket.next()).await.is_err(), "same-space foreign document never serializes");

        let command = DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id: member.user_id.clone() };
        let body = directory::os_pack::json::to_json_string(&command);
        let owner_authorization = format!("Bearer {owner_token}");
        let removed = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &owner_authorization), ("Content-Type", "application/json")], body.as_bytes()).await;
        assert_eq!(removed.status, 202);
        let removed_body = String::from_utf8(removed.body).expect("remove response UTF-8");
        assert!(removed_body.contains("member.removed"));
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "durable removal invalidates the scoped lease without exposing its event");
        announce_document_for_test(&state, &unaffected_space, "scoped-unaffected-document").await;
        assert!(
            matches!(
                next_directory_message(&mut unaffected_socket).await,
                DirectoryStreamMessage::Event { event }
                    if matches!(event.body, os_directory::DirectoryEventBody::DocumentAnnounced { ref descriptor }
                        if descriptor.space_id == unaffected_space && descriptor.document_id == "scoped-unaffected-document")
            ),
            "another user's exact scoped subscription remains live"
        );

        for stale in [grant.to_string(), pending_grant] {
            let error = connect_async(socket_request(&url, &stale)).await.expect_err("consumed or pending pre-removal grant remains invalid");
            assert!(matches!(error, tokio_tungstenite::tungstenite::Error::Http(response) if response.status() == StatusCode::UNAUTHORIZED));
        }

        let denied = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(denied.status, 401, "removed member cannot reacquire the scoped grant");
    });
}

#[test]
fn scoped_directory_socket_admin_removal_uses_the_same_membership_fence() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let mut admin_headers = authorize_test_admin(&mut state, "scoped-admin@example.com").await;
        admin_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let member = issue_test_session(&state, "scoped-admin-target@example.com").await;
        upsert_member_for_test(&state, STUDIO, "scoped-admin-target@example.com", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, STUDIO, "scoped-admin-document").await;
        let addr = spawn_server(state.clone()).await;
        let authorization = format!("Bearer {}", member.token);
        let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-admin-document/socket-grants");
        let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
        assert_eq!(issued.status, 200);
        let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("scoped grant JSON");
        let grant = receipt["grant"].as_str().expect("scoped grant");
        let since = state.directory.head_seq().await.expect("directory head");
        let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-admin-document/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, grant)).await.expect("admin target scoped socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("scoped socket hello");
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let intent = AdminIntentV1::RemoveSpaceMember { request_id: "request:scoped-admin-removal".into(), space_id: STUDIO.into(), user_id: member.user_id };
        let body = Bytes::from(directory::os_pack::json::to_json_string(&intent));
        let (status, receipt) = admin_intents(admin_headers, loopback_peer(), State(state), body).await.expect("admin removal response");
        assert_eq!(status, StatusCode::OK);
        assert_eq!(receipt.0.state, AdminIntentStateV1::Succeeded);
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "admin removal uses the same no-event membership fence");
    });
}

async fn stop_recovery_server(state: HubState, shutdown: tokio::sync::oneshot::Sender<()>, task: tokio::task::JoinHandle<()>) {
    shutdown.send(()).expect("shutdown signal");
    tokio::time::timeout(std::time::Duration::from_secs(5), task).await.expect("server shutdown deadline").expect("server shutdown join");
    state.admin_operation_tasks.shutdown().await;
    let directory = Arc::downgrade(&state.directory);
    let database = state.db.clone();
    drop(state);
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        while directory.strong_count() != 0 || Arc::strong_count(&database) != 1 {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("all socket and directory owners retired before reopen");
    let mut database = Arc::try_unwrap(database).unwrap_or_else(|_| panic!("database owner remained shared"));
    let control = db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5));
    tokio::time::timeout(std::time::Duration::from_secs(5), database.shutdown(&control)).await.expect("database shutdown deadline").expect("database shutdown");
}

fn assert_recovery_denied(response: RawHttpResponse, expected: &serde_json::Value) {
    assert_eq!(u64::from(response.status), expected["removedStatus"].as_u64().unwrap());
    assert!(response.headers.lines().any(|line| line.eq_ignore_ascii_case("content-type: application/json")));
    let error: DocumentOpenPlanErrorV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&response.body).unwrap()).expect("strict bounded error, not selected asset bytes");
    assert_eq!(error.schema, "semio.hub.document-open-plan-error/v1");
    assert_eq!(error.code, DocumentOpenPlanErrorCodeV1::Denied);
    assert_eq!(expected["removedCode"], "denied");
}

#[test]
fn admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🛂️admin-presence-target-recovery-v1/🔣️.json")).expect("admin recovery fixture");
        let expected = &fixture["expected"];
        let scope = DocumentScope::new(fixture["scope"]["spaceId"].as_str().unwrap(), fixture["scope"]["documentId"].as_str().unwrap());
        let dir = tempdir("admin-presence-target-recovery");
        std::fs::create_dir_all(&dir).expect("recovery fixture directory");
        let path = dir.join("directory.sqlite");
        let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("file directory");
        let mut state = test_state_with_directory(dir.join("db"), directory, 1024, 256).await;
        let admin_headers = authorize_test_admin(&mut state, "recovery-admin@example.com").await;
        let removed = issue_test_session(&state, "recovery-removed@example.com").await;
        let observer = issue_test_session(&state, "recovery-observer@example.com").await;
        for member in fixture["members"].as_array().unwrap() {
            let email = format!("recovery-{}@example.com", member["id"].as_str().unwrap());
            assert_eq!(member["role"], "author");
            upsert_member_for_test(&state, &scope.space_id, &email, DirectorySpaceRole::Author).await;
        }
        announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
        let descriptor = state.directory.get_document_descriptor(&scope).await.unwrap().unwrap();
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
        let (plan_b, grant_b) = issue_and_exchange_document_open_plan_for_test(&state, &removed.token, &scope, "client:removed").await;
        let (plan_c, grant_c) = issue_and_exchange_document_open_plan_for_test(&state, &observer.token, &scope, "client:observer").await;
        assert_eq!(plan_b.surface.surface_id, fixture["surfaceId"].as_str().unwrap());
        assert_eq!(plan_b.surface.surface_id, plan_c.surface.surface_id);
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let root = format!("/spaces/{}/documents/{}", scope.space_id, scope.document_id);
        let url = format!("ws://{addr}{root}/socket/v1?surface={}", plan_b.surface.surface_id);
        let (mut b, _) = connect_async(socket_request(&url, &grant_b.grant)).await.expect("member document socket");
        let (mut c, _) = connect_async(socket_request(&url, &grant_c.grant)).await.expect("observer document socket");
        b.send(client_binary(&socket_hello(), Lane::Command).await).await.unwrap();
        c.send(client_binary(&socket_hello(), Lane::Command).await).await.unwrap();
        let welcome_b = next_server_frame(&mut b).await;
        assert!(matches!(&welcome_b, ServerFrame::Welcome { .. }), "member welcome: {welcome_b:?}");
        let ServerFrame::Session { actor: actor_b, color: color_b } = next_server_frame(&mut b).await else { panic!("member session") };
        let welcome_c = next_server_frame(&mut c).await;
        assert!(matches!(&welcome_c, ServerFrame::Welcome { .. }), "observer welcome: {welcome_c:?}");
        let ServerFrame::Session { actor: actor_c, .. } = next_server_frame(&mut c).await else { panic!("observer session") };
        let raw = presence_hex_bytes(presence_normalization_fixture()["vectors"][0]["rawPeerHex"].as_str().unwrap());
        b.send(client_binary(&ClientFrame::Presence { peer: raw.clone() }, Lane::Preview).await).await.unwrap();
        let ServerFrame::Presence { peers } = next_server_frame(&mut c).await else { panic!("observer normalized presence") };
        assert_eq!(peers.len(), 1);
        let peer = protocol::decode_presence_peer(&peers[0]).await.unwrap();
        assert_eq!(peer.actor, actor_b);
        assert_eq!(peer.user_id.as_deref(), Some(removed.user_id.as_str()));
        assert_eq!(peer.label.as_deref(), Some("recovery-removed@example.com"));
        assert_eq!(peer.role.as_deref(), Some("author"));
        assert_eq!(peer.color, Some(color_b));
        assert_eq!(peer.surface.as_deref(), Some(plan_b.surface.surface_id.as_str()));
        assert!(matches!(next_server_frame(&mut b).await, ServerFrame::Presence { .. }));
        let intent = directory::os_pack::json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some(plan_b.surface.surface_id.clone()),
            client_instance_id: "client:recovery-target".into(),
        });
        let authorization_b = format!("Bearer {}", removed.token);
        let authorization_c = format!("Bearer {}", observer.token);
        let headers_b = [("Authorization", authorization_b.as_str()), ("Content-Type", "application/json")];
        let headers_c = [("Authorization", authorization_c.as_str()), ("Content-Type", "application/json")];
        assert_eq!(raw_http_request(addr, "POST", &format!("{root}/execution-target/manifest"), &headers_b, intent.as_bytes()).await.status, 200);
        let (_, pending_b) = issue_and_exchange_document_open_plan_for_test(&state, &removed.token, &scope, "client:pending-removed").await;
        let remove = directory::os_pack::json::to_json_string(&AdminIntentV1::RemoveSpaceMember { request_id: "request:admin-presence-recovery".into(), space_id: scope.space_id.clone(), user_id: removed.user_id.clone() });
        let admin_authorization = admin_headers.get(axum::http::header::AUTHORIZATION).unwrap().to_str().unwrap();
        let receipt = raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", admin_authorization), ("Content-Type", "application/json")], remove.as_bytes()).await;
        assert_eq!(receipt.status, 200);
        let receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&receipt.body).unwrap()).unwrap();
        assert_eq!(receipt.state, AdminIntentStateV1::Succeeded);
        assert_eq!(u64::from(next_close_without_authority(&mut b).await), expected["removedCloseCode"].as_u64().unwrap());
        let ServerFrame::Presence { peers } = next_server_frame(&mut c).await else { panic!("removed member withdrawal") };
        assert_eq!(peers.len() as u64, expected["visibleAfterRemoval"].as_u64().unwrap());
        assert!(state.presence_snapshot(&document_scope_key_v1(&scope)).peers.is_empty());
        let exchange = directory::os_pack::json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: plan_b.receipt.clone() });
        let denied = raw_http_request(addr, "POST", &format!("{root}/socket-grants"), &headers_b, exchange.as_bytes()).await;
        assert_recovery_denied(denied, expected);
        assert!(connect_async(socket_request(&url, &grant_b.grant)).await.is_err(), "removed member cannot reuse its consumed grant");
        assert!(connect_async(socket_request(&url, &pending_b.grant)).await.is_err(), "removal invalidates a previously unused member grant");
        for route in ["execution-target/manifest", "execution-target/component", "execution-target/descriptor", "open-plan"] {
            let denied = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_b, intent.as_bytes()).await;
            assert_recovery_denied(denied, expected);
        }
        c.send(client_binary(&ClientFrame::Presence { peer: raw }, Lane::Preview).await).await.unwrap();
        let ServerFrame::Presence { peers } = next_server_frame(&mut c).await else { panic!("observer remains live") };
        assert_eq!(peers.len(), 1);
        let observer_peer = protocol::decode_presence_peer(&peers[0]).await.unwrap();
        assert_eq!(observer_peer.actor, actor_c);
        assert_eq!(observer_peer.user_id.as_deref(), Some(observer.user_id.as_str()));
        assert_eq!(observer_peer.role.as_deref(), Some("author"));
        assert_eq!(observer_peer.surface.as_deref(), Some(plan_c.surface.surface_id.as_str()));
        assert_eq!(u64::from(raw_http_request(addr, "POST", &format!("{root}/execution-target/manifest"), &headers_c, intent.as_bytes()).await.status), expected["observerStatus"].as_u64().unwrap());
        c.close(None).await.unwrap();
        drop(c);
        drop(b);
        stop_recovery_server(state, shutdown, server).await;
        assert_eq!(expected["sqliteReopen"], true);
        let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("reopen exact file directory");
        let mut state = test_state_with_directory(dir.join("db"), directory, 1024, 256).await;
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, false, false));
        assert!(state.presence_snapshot(&document_scope_key_v1(&scope)).peers.is_empty());
        assert!(state.socket_grants.inner.lock().unwrap().records.is_empty());
        assert!(state.document_open_plans.inner.lock().unwrap().records.is_empty());
        for session in [&removed, &observer] {
            assert!(state.directory.authenticate_session(&SessionCapability::parse(&session.token).unwrap()).await.unwrap().is_some(), "membership removal must not be mistaken for session loss");
        }
        assert_eq!(state.directory.get_role(&scope.space_id, &removed.user_id).await.unwrap(), None);
        assert_eq!(state.directory.get_role(&scope.space_id, &observer.user_id).await.unwrap(), Some(SpaceRole::Author));
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for route in ["execution-target/manifest", "execution-target/component", "execution-target/descriptor", "open-plan"] {
            let denied = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_b, intent.as_bytes()).await;
            assert_recovery_denied(denied, expected);
            let admitted = raw_http_request(addr, "POST", &format!("{root}/{route}"), &headers_c, intent.as_bytes()).await;
            assert_eq!(u64::from(admitted.status), expected["observerStatus"].as_u64().unwrap(), "reopened observer {route}");
            match route {
                "execution-target/component" => assert_eq!(admitted.body, TEST_EXECUTION_TARGET_COMPONENT_BYTES),
                "execution-target/descriptor" => assert_eq!(admitted.body, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES),
                "execution-target/manifest" => {
                    let manifest: DocumentExecutionTargetLeaseFieldsV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&admitted.body).unwrap()).unwrap();
                    manifest.validate().unwrap();
                    assert_eq!(manifest.scope, scope);
                }
                "open-plan" => {
                    let plan: DocumentOpenPlanV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&admitted.body).unwrap()).unwrap();
                    plan.validate(now_ms() as u64).unwrap();
                    assert_eq!(plan.scope, scope);
                }
                _ => unreachable!(),
            }
        }
        stop_recovery_server(state, shutdown, server).await;
        eprintln!("[DEBUG] admin removal withdrew plan-bound presence, closed only the removed member, and survived exact file-SQLite Hub reopen for all selected target routes");
    });
}

#[test]
fn scoped_directory_socket_removal_and_delivery_have_one_total_membership_order() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let member = issue_test_session(&state, "scoped-order-target@example.com").await;
        upsert_member_for_test(&state, STUDIO, "scoped-order-target@example.com", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, STUDIO, "scoped-order-document").await;
        let addr = spawn_server(state.clone()).await;
        let authorization = format!("Bearer {}", member.token);
        let issue_path = format!("/directory/spaces/{STUDIO}/documents/scoped-order-document/socket-grants");

        let open = |grant: String, since: u64| {
            let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/scoped-order-document/socket/v1?since={since}");
            async move {
                let (mut socket, _) = connect_async(socket_request(&url, &grant)).await.expect("ordered scoped socket");
                socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("ordered scoped hello");
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                socket
            }
        };
        let issue = || {
            let issue_path = issue_path.clone();
            let authorization = authorization.clone();
            async move {
                let issued = raw_http_request(addr, "POST", &issue_path, &[("Authorization", &authorization)], &[]).await;
                assert_eq!(issued.status, 200);
                let receipt: serde_json::Value = serde_json::from_slice(&issued.body).expect("ordered scoped grant JSON");
                receipt["grant"].as_str().expect("ordered scoped grant").to_string()
            }
        };

        let mut removal_wins = open(issue().await, state.directory.head_seq().await.expect("removal-wins head")).await;
        gate.socket_membership_remove_enabled.store(true, std::sync::atomic::Ordering::Release);
        let mut removal = tokio::spawn({
            let state = state.clone();
            let user_id = member.user_id.clone();
            async move { execute_directory_command_fenced(&state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:scoped-order-removal".into() }, DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id }).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_membership_remove_admitted.acquire()).await.expect("removal admission deadline").expect("removal admission");
        gate.socket_scoped_send_mode.store(1, std::sync::atomic::Ordering::Release);
        announce_document_for_test(&state, STUDIO, "scoped-order-document").await;
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_scoped_send_admitted.acquire()).await.expect("removal-wins sender deadline").expect("removal-wins sender");
        gate.socket_scoped_send_release.add_permits(1);
        gate.socket_membership_remove_release.add_permits(1);
        tokio::time::timeout(std::time::Duration::from_secs(2), &mut removal).await.expect("removal-wins completion deadline").expect("removal task").expect("fenced removal");
        assert_eq!(next_close_code(&mut removal_wins, false).await, 4401, "removal winning the membership gate exposes no scoped event");

        upsert_member_for_test(&state, STUDIO, "scoped-order-target@example.com", DirectorySpaceRole::Spectator).await;
        gate.socket_membership_remove_enabled.store(false, std::sync::atomic::Ordering::Release);
        gate.socket_scoped_send_mode.store(0, std::sync::atomic::Ordering::Release);
        let mut delivery_wins = open(issue().await, state.directory.head_seq().await.expect("delivery-wins head")).await;
        gate.socket_scoped_send_mode.store(2, std::sync::atomic::Ordering::Release);
        announce_document_for_test(&state, STUDIO, "scoped-order-document").await;
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_scoped_send_admitted.acquire()).await.expect("delivery-wins sender deadline").expect("delivery-wins sender");
        let mut removal = tokio::spawn({
            let state = state.clone();
            let user_id = member.user_id.clone();
            async move { execute_directory_command_fenced(&state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:scoped-order-delivery".into() }, DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id }).await }
        });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut removal).await.is_err(), "removal waits while an admitted scoped send owns the membership gate");
        gate.socket_scoped_send_release.add_permits(1);
        assert!(matches!(
            next_directory_message(&mut delivery_wins).await,
            DirectoryStreamMessage::Event { event }
                if matches!(event.body, os_directory::DirectoryEventBody::DocumentAnnounced { ref descriptor }
                    if descriptor.space_id == STUDIO && descriptor.document_id == "scoped-order-document")
        ));
        tokio::time::timeout(std::time::Duration::from_secs(2), removal).await.expect("delivery-wins removal deadline").expect("removal task").expect("fenced removal");
        assert_eq!(next_close_code(&mut delivery_wins, false).await, 4401, "the one admitted event precedes the terminal membership close");
    });
}

#[test]
fn admin_intent_binding_wire_matrix_is_exact_sorted_and_self_deduplicated() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).unwrap();
    let principal = AdminPrincipalV1 {
        user_id: "admin".into(),
        auth_session_id: "admin-session".into(),
        authorization_generation: 1,
        identity_provider: "fixture".into(),
        identity_subject_digest: [0; 32],
        expires_at_ms: i64::MAX,
        correlation_id: "binding-fixture".into(),
        peer_class: "test",
    };
    for row in fixture["bindings"].as_array().unwrap() {
        let intent: AdminIntentV1 = directory::os_pack::json::from_json_str(row["intentJson"].as_str().unwrap()).expect("actual closed administrator intent wire");
        let keys = admin_intent_bindings(&principal, &intent).map(|bindings| {
            bindings
                .into_iter()
                .map(|binding| match binding {
                    SocketBindingKeyV1::User(id) => format!("user:{id}"),
                    SocketBindingKeyV1::Session(id) => format!("session:{id}"),
                    SocketBindingKeyV1::DirectorySpaceAuthority { space_id } => format!("space:{space_id}"),
                    SocketBindingKeyV1::Membership { user_id, space_id } => format!("membership:{user_id}/{space_id}"),
                    SocketBindingKeyV1::Share(id) => format!("share:{id}"),
                    SocketBindingKeyV1::DocumentWrite(_) => panic!("administrator short authority cannot acquire a document writer"),
                })
                .collect::<Vec<_>>()
        });
        assert_eq!(serde_json::to_value(&keys).unwrap(), row["keys"], "exact sorted binding union: {}", row["name"]);
        eprintln!("[DEBUG] admin-intent-bindings: {} keys={keys:?}", row["name"]);
    }
}

#[test]
fn admin_short_effects_retain_principal_until_their_actual_side_effect() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).expect("admin short effect fixture");
        let root = tempdir("admin-short-authority");
        std::fs::create_dir_all(&root).expect("physical directory parent");
        let path = root.join("directory.sqlite");
        let directory = SqliteDirectory::connect(path.to_str().unwrap()).await.expect("physical directory");
        let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state_with_directory(root.join("db"), directory, 1024, 256)).await.expect("admin effect state open deadline");
        let physical = rusqlite::Connection::open(&path).expect("independent physical directory reader");
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let email = "admin-short-authority@example.test";
        let _headers = authorize_test_admin(&mut state, email).await;
        let owner = issue_test_session(&state, "admin-short-owner@example.test").await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["shortActions"].as_array().unwrap().iter().enumerate() {
            let admin = issue_test_session(&state, email).await;
            let target = issue_test_session(&state, &format!("admin-short-target-{index}@example.test")).await;
            let space = create_space_for_test(&state, &owner.user_id, "Short Authority", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let scope = DocumentScope::new(space, format!("admin-short-document-{index}"));
            announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
            let request_id = format!("request:admin-short-{index}");
            let sync_id = format!("sync:admin-short-{index}");
            let kick = Arc::new(tokio::sync::Notify::new());
            state.session_kicks.insert(sync_id.clone(), kick.clone());
            let action = row["action"].as_str().unwrap();
            let existing_share = if action == "revoke-share" { Some(state.directory.issue_share_token(&scope, 600, "fixture:existing-share").await.expect("existing target share")) } else { None };
            let intent = match action {
                "issue-share" => AdminIntentV1::IssueDocumentShare { request_id, scope: scope.clone(), ttl_secs: 600 },
                "revoke-share" => AdminIntentV1::RevokeDocumentShare { request_id, scope: scope.clone(), share_id: existing_share.as_ref().unwrap().record.id.clone(), reason_code: "test-revoke".into() },
                "revoke-user" => AdminIntentV1::RevokeUserSessions { request_id, user_id: target.user_id.clone(), reason_code: "test-revoke".into() },
                "kick" => AdminIntentV1::KickConnection { request_id, sync_session_id: sync_id.clone(), reason_code: "test-kick".into() },
                other => panic!("unknown short action {other}"),
            };
            let principal = authenticate_admin_principal(&state, &bearer_headers(&admin.token), None).await.expect("short action principal");
            let bindings = admin_intent_bindings(&principal, &intent).expect("short effect owns an authority union");
            assert_eq!(bindings.len() as u64, row["keys"].as_u64().unwrap(), "closed short action authority keys");
            let action_first = row["first"] == "action";
            *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), action_first));
            let command = tokio::spawn({
                let authorization = format!("Bearer {}", admin.token);
                let body = directory::os_pack::json::to_json_string(&intent);
                async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("short action pause deadline").expect("short action pause").forget();
            let mut revoke = tokio::spawn({
                let state = state.clone();
                let token = admin.token.clone();
                async move { delete_session_me(bearer_headers(&token), State(state)).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_session_revoke_attempted.acquire()).await.expect("short action revoke attempt deadline").expect("short action revoke attempt").forget();
            if action_first {
                assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted {} must own the principal authority", row["name"]);
                for binding in bindings {
                    assert!(state.socket_binding_gates.gate(binding).try_lock_owned().is_err(), "every short effect key stays owned through its physical side effect");
                }
            } else {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("winning short action revoke deadline").expect("winning short action revoke"), StatusCode::NO_CONTENT);
            }
            *gate.directory_command_pause_user.lock().unwrap() = None;
            gate.directory_command_release.add_permits(1);
            let response = tokio::time::timeout(std::time::Duration::from_secs(5), command).await.expect("short action completion deadline").expect("short action task");
            assert_eq!(response.status, 200);
            let receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&response.body).unwrap()).expect("short action receipt");
            assert_eq!(receipt.state, if row["effect"] == true { AdminIntentStateV1::Succeeded } else { AdminIntentStateV1::Cancelled }, "principal authority decides {}", row["name"]);
            if action_first {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing short action revoke deadline").expect("trailing short action revoke"), StatusCode::NO_CONTENT);
            }
            let effect: i64 = match action {
                "issue-share" => physical.query_row("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(),
                "revoke-share" => physical.query_row("SELECT count(*) FROM hub_share_grant WHERE id = ?1 AND revoked_at IS NOT NULL", [existing_share.as_ref().unwrap().record.id.as_str()], |row| row.get(0)).unwrap(),
                "revoke-user" => physical.query_row("SELECT count(*) FROM hub_auth_session WHERE user_id = ?1 AND revoked_at IS NOT NULL", [target.user_id.as_str()], |row| row.get(0)).unwrap(),
                "kick" => i64::from(tokio::time::timeout(std::time::Duration::from_millis(30), kick.notified()).await.is_ok()),
                _ => unreachable!(),
            };
            assert_eq!(effect, i64::from(row["effect"].as_bool().unwrap()), "exact physical effect: {}", row["name"]);
            let audit_count: i64 = physical.query_row("SELECT count(*) FROM hub_auth_audit WHERE correlation_id = ?1", [&receipt.correlation_id], |row| row.get(0)).unwrap();
            assert_eq!(audit_count, if action == "kick" { 0 } else { effect }, "no auth side-effect audit under revoked authority");
            let secret = receipt.result.as_ref().and_then(|result| result.share_token.as_ref());
            assert_eq!(secret.is_some(), row["secret"].as_bool().unwrap(), "plaintext share result follows the exact admitted effect");
            if let Some(secret) = secret {
                let capability = semio_hub::directory::ShareCapability::parse(secret).expect("minted share capability");
                assert!(state.directory.authenticate_share(&scope, &capability).await.expect("minted share authentication"));
            }
            state.session_kicks.remove(&sync_id);
            eprintln!("[DEBUG] admin-short-authority: {} state={:?} physical-effects={effect} auth-audits={audit_count} secret={}", row["name"], receipt.state, secret.is_some());
        }
        drop(physical);
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn admin_directory_commands_hold_exact_principal_without_confusing_space_role() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🏛️admin-directory-authority-v1/🔣️.json")).expect("admin authority fixture");
        let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("admin authority state open deadline");
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let email = "directory-admin-authority@example.test";
        let _headers = authorize_test_admin(&mut state, email).await;
        let owner = issue_test_session(&state, "directory-admin-target-owner@example.test").await;
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["vectors"].as_array().unwrap().iter().enumerate() {
            let admin = issue_test_session(&state, email).await;
            let headers = bearer_headers(&admin.token);
            let principal = authenticate_admin_principal(&state, &headers, None).await.expect("configured administrator");
            let space = create_space_for_test(&state, &owner.user_id, "Admin Target", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            upsert_member_for_test(&state, &space, email, DirectorySpaceRole::Author).await;
            let request_id = format!("request:admin-directory-authority-{index}");
            let name = format!("Admin Mutation {index}");
            let create = row["command"] == "create";
            let command_first = row["first"] == "command";
            let intent = if create {
                AdminIntentV1::CreateSpace { request_id: request_id.clone(), name: name.clone(), space_kind: os_directory::DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }
            } else {
                AdminIntentV1::RenameSpace { request_id: request_id.clone(), space_id: space.clone(), name: name.clone() }
            };
            let target = if create { admin_create_space_id(&request_id) } else { space.clone() };
            let before = state.directory.head_seq().await.expect("admin head before intent");
            *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), command_first));
            let mut command = tokio::spawn({
                let authorization = format!("Bearer {}", admin.token);
                let body = directory::os_pack::json::to_json_string(&intent);
                async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
            });
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("admin pause deadline").expect("admin pause").forget();
            let mut trailing_revoke = None;
            if row["transition"] == "session" {
                let mut revoke = tokio::spawn({
                    let state = state.clone();
                    let token = admin.token.clone();
                    async move { delete_session_me(bearer_headers(&token), State(state)).await }
                });
                tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_session_revoke_attempted.acquire()).await.expect("admin revoke attempt deadline").expect("admin revoke attempt").forget();
                if command_first {
                    assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted admin command retains the exact session");
                    assert!(state.socket_binding_gates.gate(SocketBindingKeyV1::User(principal.user_id.clone())).try_lock_owned().is_err(), "admitted admin command also retains user-wide revocation authority");
                    trailing_revoke = Some(revoke);
                } else {
                    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("admin revoke deadline").expect("admin revoke"), StatusCode::NO_CONTENT);
                }
            } else {
                upsert_member_for_test(&state, &space, email, DirectorySpaceRole::Spectator).await;
                assert_eq!(state.directory.get_role(&space, &admin.user_id).await.unwrap(), Some(SpaceRole::Spectator));
            }
            *gate.directory_command_pause_user.lock().unwrap() = None;
            gate.directory_command_release.add_permits(1);
            let response = tokio::time::timeout(std::time::Duration::from_secs(5), &mut command).await.expect("admin intent completion deadline").expect("admin intent task");
            assert_eq!(response.status, 200);
            let receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&response.body).unwrap()).expect("admin authority receipt");
            let expected = if row["mutated"] == true { AdminIntentStateV1::Succeeded } else { AdminIntentStateV1::Cancelled };
            assert_eq!(receipt.state, expected, "exact principal, not ordinary role, decides {}", row["name"]);
            if let Some(revoke) = trailing_revoke {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing admin revoke deadline").expect("trailing admin revoke"), StatusCode::NO_CONTENT);
            }
            let events = state.directory.events_since(before, 50).await.expect("admin authority durable events");
            let own_events: Vec<_> = events.iter().filter(|event| event.actor == principal.event_actor()).collect();
            assert_eq!(own_events.len(), if row["mutated"] == true { if create { 2 } else { 1 } } else { 0 }, "no mutation escapes a revoked principal");
            let persisted = state.directory.get_space(&target).await.expect("admin target projection");
            assert_eq!(persisted.as_ref().is_some_and(|space| space.name == name), row["mutated"].as_bool().unwrap());
            if row["mutated"] == false {
                assert!(receipt.event_seq_first.is_none() && receipt.event_seq_last.is_none());
            }
            eprintln!("[DEBUG] admin-directory-authority: {} state={:?} events={}", row["name"], receipt.state, own_events.len());
        }

        let self_session = issue_test_session(&state, email).await;
        let authorization = format!("Bearer {}", self_session.token);
        let body = directory::os_pack::json::to_json_string(&AdminIntentV1::RevokeUserSessions { request_id: "request:admin-directory-self-revoke".into(), user_id: self_session.user_id.clone(), reason_code: "test-self-revoke".into() });
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()))
            .await
            .expect("self-revocation cannot nest an administrator User fence");
        assert_eq!(response.status, 200);
        let receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&response.body).unwrap()).expect("self-revocation receipt");
        assert_eq!(receipt.state, AdminIntentStateV1::Succeeded);
        let capability = SessionCapability::parse(&self_session.token).unwrap();
        assert!(state.directory.authenticate_session(&capability).await.unwrap().is_none());
        eprintln!("[DEBUG] admin-directory-authority: self-user-revocation completed without nested User ownership");
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn directory_global_message_bindings_decode_wire_without_indexing_unrelated_memberships() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🌐️directory-message-authority-v1/🔣️.json")).expect("message authority fixture");
    let ledger = SocketGrantLedgerV1::default();
    let capability = SocketGrantCapability::mint().expect("global capability");
    let audience = SocketAudienceV1::Directory { auth_session_id: "session".into(), authorization_generation: 1 };
    let subject = SocketSubjectV1::Session { session_id: "session".into(), user_id: "recipient".into(), authorization_generation: 1, role: None, expires_at_ms: 10_000 };
    ledger.issue(&capability, audience.clone(), "hub.v1.global".into(), subject, 1, 9_000).expect("global issue");
    let pending = ledger.pending(&capability, &audience, 2).expect("pending global");
    let record = ledger.consume(&pending, 3).expect("consumed global");
    let (live_id, _) = ledger.register_live(&record).expect("global lease");
    let principal_bindings = vec![SocketBindingKeyV1::User("recipient".into()), SocketBindingKeyV1::Session("session".into())];
    for row in fixture["messages"].as_array().unwrap() {
        let message: DirectoryStreamMessage = directory::os_pack::json::from_json_str(row["messageJson"].as_str().unwrap()).unwrap_or_else(|error| panic!("wire message {}: {error:?}", row["name"]));
        assert_eq!(directory_stream_message_space(&message), row["spaceId"].as_str(), "wire-derived space");
        let bindings = directory_message_bindings(&record, &message);
        assert_eq!(bindings.len() as u64, row["keys"].as_u64().unwrap(), "complete transient union");
        let mut expected = principal_bindings.clone();
        if let Some(space_id) = row["spaceId"].as_str() {
            expected.push(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space_id.to_owned() });
            expected.push(SocketBindingKeyV1::Membership { user_id: "recipient".into(), space_id: space_id.to_owned() });
        }
        assert_eq!(bindings, expected, "sorted exact principal and recipient membership union");
        assert_eq!(record.bindings(), principal_bindings, "transient keys never become permanent global indices");
        let mut scoped = record.clone();
        scoped.audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new("scoped", "doc"));
        assert_eq!(directory_message_bindings(&scoped, &message), scoped.bindings(), "scoped audiences do not borrow unrelated scopes");
    }
    ledger.invalidate_binding(SocketBindingKeyV1::Membership { user_id: "recipient".into(), space_id: "space-a".into() });
    ledger.invalidate_binding(SocketBindingKeyV1::DirectorySpaceAuthority { space_id: "space-a".into() });
    assert!(ledger.is_live(&record, &live_id), "space A revocation cannot invalidate a global lease for B");
    ledger.invalidate_binding(SocketBindingKeyV1::Session("session".into()));
    assert!(!ledger.is_live(&record, &live_id), "principal revocation remains terminal");
    eprintln!("[DEBUG] global-directory-message-bindings: six real wire kinds, transient union, scoped isolation, global indices, principal invalidation");
}

#[test]
fn directory_global_socket_delivery_and_revocation_share_one_transient_authority_order() {
    run_socket_test(|| async {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🌐️directory-message-authority-v1/🔣️.json")).expect("message authority fixture");
        let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("global directory state open deadline");
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        for (index, row) in fixture["vectors"].as_array().unwrap().iter().enumerate() {
            let owner_email = format!("global-owner-{index}@example.test");
            let recipient_email = format!("global-recipient-{index}@example.test");
            let owner = issue_test_session(&state, &owner_email).await;
            let recipient = issue_test_session(&state, &recipient_email).await;
            let space_a = create_space_for_test(&state, &owner.user_id, "A", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let space_b = create_space_for_test(&state, &owner.user_id, "B", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            for space in [&space_a, &space_b] {
                upsert_member_for_test(&state, space, &recipient_email, DirectorySpaceRole::Spectator).await;
            }
            let receipt = issue_directory_socket_grant(bearer_headers(&recipient.token), State(state.clone())).await.expect("global grant").0;
            let since = state.directory.head_seq().await.expect("global directory head");
            let url = format!("ws://{addr}/directory/socket/v1?since={since}");
            let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("global directory socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("global socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_directory_admitted.acquire()).await.expect("global admission deadline").expect("global admission").forget();
            gate.socket_directory_release.add_permits(1);
            let delivery_first = row["first"] == "send";
            *gate.socket_global_send_pause.lock().expect("global pause lock") = Some((recipient.user_id.clone(), if delivery_first { 2 } else { 1 }));
            let event_a = state
                .directory_service
                .execute(DirectoryActor { kind: DirectoryActorKind::System, id: "system:global-message-authority".into() }, DirectoryCommand::RenameSpace { space_id: space_a.clone(), name: format!("A-{index}") })
                .await
                .expect("durable A event")
                .0
                .into_iter()
                .next()
                .expect("A event");
            tokio::time::timeout(std::time::Duration::from_secs(5), gate.socket_global_send_admitted.acquire()).await.expect("message pause deadline").expect("message pause").forget();
            let membership = row["revocation"] == "membership";
            let mut revoke = tokio::spawn({
                let state = state.clone();
                let token = if membership { owner.token.clone() } else { recipient.token.clone() };
                let command = DirectoryCommand::RemoveMember { space_id: space_a.clone(), user_id: recipient.user_id.clone() };
                async move { if membership { post_directory_command_for_test(addr, &token, "c00102030405060708090a0b0c0d0e0f", command).await.status } else { delete_session_me(bearer_headers(&token), State(state)).await.as_u16() } }
            });
            let attempted = if membership { &gate.directory_command_attempted } else { &gate.socket_session_revoke_attempted };
            tokio::time::timeout(std::time::Duration::from_secs(5), attempted.acquire()).await.expect("revocation fence attempt deadline").expect("revocation fence attempt").forget();
            if delivery_first {
                assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut revoke).await.is_err(), "admitted delivery must own the same authority keys as revocation: {}", row["name"]);
            } else {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("winning revocation deadline").expect("winning revocation"), if membership { 202 } else { 204 });
            }
            *gate.socket_global_send_pause.lock().expect("clear global pause") = None;
            gate.socket_global_send_release.add_permits(1);
            if row["a"] == true {
                assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if event == event_a), "one exact A event wins before revocation");
            }
            if delivery_first {
                assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), revoke).await.expect("trailing revocation deadline").expect("trailing revocation"), if membership { 202 } else { 204 });
            }
            if row["b"] == true {
                let event_b = state
                    .directory_service
                    .execute(DirectoryActor { kind: DirectoryActorKind::System, id: "system:global-message-authority".into() }, DirectoryCommand::RenameSpace { space_id: space_b.clone(), name: format!("B-{index}") })
                    .await
                    .expect("durable B event")
                    .0
                    .into_iter()
                    .next()
                    .expect("B event");
                assert!(matches!(next_directory_message(&mut socket).await, DirectoryStreamMessage::Event { event } if event == event_b), "removed A membership neither leaks A nor closes unrelated B");
                socket.close(None).await.expect("close unaffected global socket");
            } else {
                assert_eq!(u64::from(next_close_code(&mut socket, false).await), row["close"].as_u64().unwrap(), "revoked principal closes without a later message");
            }
            eprintln!("[DEBUG] global-directory-message-authority: {} A={} B={} close={}", row["name"], row["a"], row["b"], row["close"]);
        }
        stop_recovery_server(state, shutdown, server).await;
    });
}

#[test]
fn socket_directory_revoke_after_admission_suppresses_replay_without_deadlock() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        state.live_gate = Some(gate.clone());
        let token = seed_author_token(&state).await;
        let receipt = issue_directory_socket_grant(bearer_headers(&token), State(state.clone())).await.expect("issue directory grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/socket/v1?since=0");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("directory socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_directory_admitted.acquire()).await.expect("directory admission deadline").expect("directory admission");
        assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(2), delete_session_me(bearer_headers(&token), State(state))).await.expect("bounded revoke"), StatusCode::NO_CONTENT,);
        gate.socket_directory_release.add_permits(1);
        assert_eq!(next_close_code(&mut socket, false).await, 4401, "no replay text crosses a winning revoke");
    });
}

#[tokio::test]
async fn socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke() {
    let mut state = test_state().await;
    let gate = Arc::new(TestLiveGate::default());
    state.live_gate = Some(gate.clone());
    let mut admin_headers = authorize_test_admin(&mut state, "socket-admin@example.com").await;
    admin_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
    let target = issue_test_session(&state, "socket-target@example.com").await;
    upsert_member_for_test(&state, STUDIO, "socket-target@example.com", DirectorySpaceRole::Author).await;
    announce_document_for_test(&state, STUDIO, "socket-admin-race").await;
    let mut revoke = tokio::spawn({
        let state = state.clone();
        let user_id = target.user_id.clone();
        async move {
            let intent = AdminIntentV1::RevokeUserSessions { request_id: "request:socket-admin-revoke".into(), user_id, reason_code: "test-revoke".into() };
            let body = Bytes::from(directory::os_pack::json::to_json_string(&intent));
            admin_intents(admin_headers, loopback_peer(), State(state), body).await
        }
    });
    tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_admin_revoke_admitted.acquire()).await.expect("admin gate deadline").expect("admin gate");
    let mut issue = tokio::spawn({
        let state = state.clone();
        let token = target.token.clone();
        async move { issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-admin-race".to_string())), bearer_headers(&token), State(state)).await }
    });
    assert!(tokio::time::timeout(std::time::Duration::from_millis(100), &mut issue).await.is_err(), "same-user grant waits behind batch revoke");
    gate.socket_admin_revoke_release.add_permits(1);
    let (status, receipt) = tokio::time::timeout(std::time::Duration::from_secs(2), &mut revoke).await.expect("bounded admin revoke").expect("admin task").expect("admin response");
    assert_eq!(status, StatusCode::OK);
    assert_eq!(receipt.0.state, AdminIntentStateV1::Succeeded);
    let late_issue = tokio::time::timeout(std::time::Duration::from_secs(2), issue).await.expect("bounded late issue").expect("issue task");
    assert!(matches!(late_issue, Err(StatusCode::UNAUTHORIZED)), "revoked session cannot mint");
}

#[tokio::test]
async fn socket_directory_visibility_requires_membership_even_for_public_spaces() {
    let state = test_state().await;
    let token = seed_author_token(&state).await;
    let capability = SessionCapability::parse(&token).expect("session capability");
    let session = state.directory.authenticate_session(&capability).await.expect("authenticate session").expect("active session");
    let other = issue_test_session(&state, "public-owner@example.com").await;
    let public_space = create_space_for_test(&state, &other.user_id, "Public Other", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
    let event = state.directory.events_since(0, 100).await.expect("directory events").into_iter().find(|event| event.space_id.as_deref() == Some(public_space.as_str())).expect("public-space event");
    let audience = SocketAudienceV1::Directory { auth_session_id: session.id.clone(), authorization_generation: session.authorization_generation };
    let record = SocketGrantRecordV1 {
        selector: "visibility".into(),
        secret_digest: [0; 32],
        audience,
        actor_id: "hub.v1.visibility".into(),
        subject: SocketSubjectV1::Session { session_id: session.id, user_id: session.user_id, authorization_generation: session.authorization_generation, role: None, expires_at_ms: session.expires_at },
        document_plan: None,
        issued_at_ms: session.issued_at,
        expires_at_ms: session.expires_at,
        state: SocketGrantStateV1::Consumed,
    };
    assert_eq!(socket_directory_membership_visibility(&state, &record, &DirectoryStreamMessage::Event { event }).await, SocketBindingValidityV1::Unauthorized);
}

fn assert_public_projection_has_no_private_keys(value: &serde_json::Value) {
    const FORBIDDEN: &[&str] = &[
        "ownerUserId",
        "role",
        "activeConnections",
        "connections",
        "presence",
        "members",
        "invites",
        "email",
        "userId",
        "displayName",
        "actor",
        "hlc",
        "cursor",
        "headSeq",
        "commitSeq",
        "epoch",
        "bootstrapVersion",
        "bootstrapFrontier",
        "bootstrapSnapshotHash",
        "checkpointId",
        "storageKey",
    ];
    match value {
        serde_json::Value::Object(entries) => {
            for (key, value) in entries {
                assert!(!FORBIDDEN.contains(&key.as_str()), "public projection disclosed forbidden key {key}");
                assert_public_projection_has_no_private_keys(value);
            }
        }
        serde_json::Value::Array(entries) => entries.iter().for_each(assert_public_projection_has_no_private_keys),
        _ => {}
    }
}

//#region 🏛️SpaceAdministration
/// 🏛️ An author receives both bounded windows, a canonical receipt over the exact response
/// bytes, and the server's own capability flags — and no credential column anywhere.
#[tokio::test]
async fn space_administration_page_v1_route_returns_the_author_windows_with_a_canonical_receipt() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-author@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Administered", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "administration-guest@example.invalid", DirectorySpaceRole::Spectator).await;
    state
        .directory_service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 })
        .await
        .expect("author invite fixture");
    let addr = spawn_server(state).await;
    let authorization = format!("Bearer {}", author.token);
    let response = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await;
    assert_eq!(response.status, 200);
    let canonical = std::str::from_utf8(&response.body).expect("administration page UTF-8").to_string();
    assert!(canonical.len() <= DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES);
    let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical).expect("canonical author page");
    assert_eq!(page.space_id(), space);
    let DirectorySpaceAdministrationPageV1::Author { members, invites, capabilities, .. } = &page else { panic!("author projection") };
    assert_eq!(members.rows.len(), 2);
    assert!(members.rows.windows(2).all(|pair| pair[0].user_id < pair[1].user_id), "member rows are keyset-ordered");
    assert!(members.rows.iter().any(|row| row.owner), "the owner row is marked so removal can be disabled before dispatch");
    assert_eq!(invites.rows.len(), 1);
    assert!(capabilities.remove_member && capabilities.create_invite);
    for secret in ["selector", "secretDigest", "passwordHash", "ssoSubject", "ssoProvider", "inviteToken"] {
        assert!(!canonical.contains(secret), "administration page leaked {secret}");
    }
}

/// 🛂️ A spectator receives the member shape only: invites and capability flags are structurally
/// absent, so no renderer can mis-gate them into existence.
#[tokio::test]
async fn space_administration_page_v1_route_denies_a_spectator_the_author_windows() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-owner@example.invalid").await;
    let spectator = issue_test_session(&state, "administration-spectator@example.invalid").await;
    let outsider = issue_test_session(&state, "administration-outsider@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Spectated", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "administration-spectator@example.invalid", DirectorySpaceRole::Spectator).await;
    state
        .directory_service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-spectator-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 })
        .await
        .expect("author invite fixture");
    let addr = spawn_server(state).await;
    let spectator_authorization = format!("Bearer {}", spectator.token);
    let response = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", spectator_authorization.as_str())]).await;
    assert_eq!(response.status, 200);
    let canonical = std::str::from_utf8(&response.body).expect("spectator page UTF-8").to_string();
    let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical).expect("canonical member page");
    assert!(matches!(page, DirectorySpaceAdministrationPageV1::Member { .. }));
    assert!(page.capabilities().is_none());
    assert!(!canonical.contains("\"invites\"") && !canonical.contains("\"capabilities\""));
    let outsider_authorization = format!("Bearer {}", outsider.token);
    assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", outsider_authorization.as_str())]).await.status, 404);
    assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[]).await.status, 404);
}

/// 🧯️ A membership removal takes effect on the very next page read: the response is a denial with
/// no page bytes at all, so no member or invite row can leak past the revocation.
#[tokio::test]
async fn space_administration_page_v1_route_denies_a_removed_member_and_leaks_no_rows() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-remover@example.invalid").await;
    let removed = issue_test_session(&state, "administration-removed@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Revoked", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "administration-removed@example.invalid", DirectorySpaceRole::Spectator).await;
    let service = state.directory_service.clone();
    let addr = spawn_server(state).await;
    let authorization = format!("Bearer {}", removed.token);
    assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await.status, 200);
    service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#administration-remove-law", author.user_id) }, DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: removed.user_id.clone() })
        .await
        .expect("member removal");
    let denied = raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await;
    assert_eq!(denied.status, 404, "a removed member cannot even enumerate the private space");
    assert!(!String::from_utf8_lossy(&denied.body).contains("administration-remover@example.invalid"));
}

/// 🛡️ The query grammar is exact and every cursor is MAC-bound: a foreign, tampered, or
/// differently shaped cursor is a 400 with no page bytes, never a page for another identity.
#[tokio::test]
async fn space_administration_page_v1_route_rejects_a_noncanonical_query_and_a_foreign_cursor() {
    let state = test_state().await;
    let author = issue_test_session(&state, "administration-cursor@example.invalid").await;
    let space = create_space_for_test(&state, &author.user_id, "Cursored", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state).await;
    let authorization = format!("Bearer {}", author.token);
    for query in ["?cursor=", "?cursor=a&cursor=b", "?cursor=%6d", "?cursor=m+1", "?section=members", "?cursor=m.7573.zz"] {
        let response = raw_http_get(addr, &format!("/directory/spaces/{space}{query}"), &[("Authorization", authorization.as_str())]).await;
        assert_eq!(response.status, 400, "query {query} must be refused before any read");
        assert!(response.body.is_empty() || !String::from_utf8_lossy(&response.body).contains("receiptSha256"));
    }
    let forged = format!("m.{}.{}", os_directory::hex_lower(b"user-forged"), "ab".repeat(32));
    let response = raw_http_get(addr, &format!("/directory/spaces/{space}?cursor={forged}"), &[("Authorization", authorization.as_str())]).await;
    assert_eq!(response.status, 400, "a cursor with a forged MAC is refused");
    assert_eq!(raw_http_get(addr, &format!("/directory/spaces/{space}"), &[("Authorization", authorization.as_str())]).await.status, 200);
}
//#endregion 🏛️SpaceAdministration

#[test]
fn space_public_boundary_real_routes_emit_discriminated_public_member_author_and_private_404() {
    run_socket_test(|| async {
        let state = test_state().await;
        let author = issue_test_session(&state, "public-author@example.invalid").await;
        let spectator = issue_test_session(&state, "public-spectator@example.invalid").await;
        let outsider = issue_test_session(&state, "public-outsider@example.invalid").await;
        let public_space = create_space_for_test(&state, &author.user_id, "Discoverable", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        upsert_member_for_test(&state, &public_space, "public-spectator@example.invalid", DirectorySpaceRole::Spectator).await;
        announce_document_for_test(&state, &public_space, "catalog-document").await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#route-law", author.user_id) }, DirectoryCommand::CreateInvite { space_id: public_space.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 600 })
            .await
            .expect("author invite fixture");
        let private_space = create_space_for_test(&state, &author.user_id, "Private", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let addr = spawn_server(state).await;

        let anonymous = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[]).await;
        assert_eq!(anonymous.status, 200);
        let anonymous: serde_json::Value = serde_json::from_slice(&anonymous.body).expect("anonymous public detail JSON");
        assert_eq!(anonymous["access"], "public");
        assert_eq!(anonymous["schema"], DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA);
        assert_eq!(anonymous["space"]["visibility"], "public");
        assert_eq!(anonymous["documents"]["rows"][0]["documentId"], "catalog-document");
        assert!(anonymous["documents"]["rows"][0].get("descriptor").is_none());
        assert_public_projection_has_no_private_keys(&anonymous);

        let outsider_authorization = format!("Bearer {}", outsider.token);
        let public_nonmember = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", outsider_authorization.as_str())]).await;
        assert_eq!(public_nonmember.status, 200);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&public_nonmember.body).expect("nonmember public detail"), anonymous);
        let private_nonmember = raw_http_get(addr, &format!("/directory/spaces/{private_space}"), &[("Authorization", outsider_authorization.as_str())]).await;
        assert_eq!(private_nonmember.status, 404);

        let public_document_status = format!("/spaces/{public_space}/documents/catalog-document");
        assert_eq!(raw_http_get(addr, &public_document_status, &[]).await.status, 401, "public discovery is not document-currentness authority");
        assert_eq!(raw_http_get(addr, &public_document_status, &[("Authorization", outsider_authorization.as_str())]).await.status, 401);
        let blob = format!("/spaces/{public_space}/blobs/{}", "11".repeat(32));
        assert_eq!(raw_http_request(addr, "GET", &blob, &[], &[]).await.status, 401, "public discovery is not blob read authority");
        assert_eq!(raw_http_request(addr, "HEAD", &blob, &[], &[]).await.status, 401, "public discovery is not blob existence authority");
        assert_eq!(raw_http_request(addr, "PUT", &blob, &[], b"private").await.status, 401, "public discovery is not blob write authority");
        assert_eq!(raw_http_request(addr, "GET", &blob, &[("Authorization", outsider_authorization.as_str())], &[]).await.status, 401);
        assert_eq!(raw_http_request(addr, "HEAD", &blob, &[("Authorization", outsider_authorization.as_str())], &[]).await.status, 401);
        assert_eq!(raw_http_request(addr, "PUT", &blob, &[("Authorization", outsider_authorization.as_str())], b"private").await.status, 401);

        let spectator_authorization = format!("Bearer {}", spectator.token);
        let member = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", spectator_authorization.as_str())]).await;
        assert_eq!(member.status, 200);
        let member: serde_json::Value = serde_json::from_slice(&member.body).expect("member detail");
        assert_eq!(member["access"], "member");
        assert!(member.get("members").is_some());
        assert!(member.get("invites").is_none(), "a member page structurally omits invites");
        assert!(member.get("capabilities").is_none(), "a member page structurally omits capability flags");
        assert_eq!(member["documents"]["rows"][0]["headSeq"], 0);

        let author_authorization = format!("Bearer {}", author.token);
        let authored = raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", author_authorization.as_str())]).await;
        assert_eq!(authored.status, 200);
        let authored: serde_json::Value = serde_json::from_slice(&authored.body).expect("author detail");
        assert_eq!(authored["access"], "author");
        assert_eq!(authored["space"]["role"], "author");
        assert_eq!(authored["invites"]["rows"].as_array().map(Vec::len), Some(1));
        assert_eq!(authored["capabilities"]["removeMember"], true);
        let authored_bytes = std::str::from_utf8(&raw_http_get(addr, &format!("/directory/spaces/{public_space}"), &[("Authorization", author_authorization.as_str())]).await.body).expect("author page UTF-8").to_string();
        assert!(!authored_bytes.contains("selector") && !authored_bytes.contains("secretDigest") && !authored_bytes.contains("passwordHash"));
        assert_eq!(DirectorySpaceAdministrationPageV1::parse_canonical_json(&authored_bytes).map(|page| page.space_id().to_string()), Ok(public_space.clone()));

        let list = raw_http_get(addr, "/directory/spaces", &[]).await;
        assert_eq!(list.status, 200);
        let list: serde_json::Value = serde_json::from_slice(&list.body).expect("public list");
        let rows = list.as_array().expect("list rows");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["access"], "public");
        assert_public_projection_has_no_private_keys(&rows[0]);
    });
}

#[test]
fn space_public_boundary_public_event_route_denies_raw_directory_events() {
    run_socket_test(|| async {
        let state = test_state().await;
        let outsider = issue_test_session(&state, "event-outsider@example.invalid").await;
        let since = state.directory.head_seq().await.expect("pre-public head");
        let owner = issue_test_session(&state, "event-owner@example.invalid").await;
        let public_space = create_space_for_test(&state, &owner.user_id, "Event Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        announce_document_for_test(&state, &public_space, "event-document").await;
        let addr = spawn_server(state).await;
        let anonymous = raw_http_get(addr, &format!("/directory/events?since={since}&limit=100"), &[]).await;
        assert_eq!(anonymous.status, 200);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&anonymous.body).expect("anonymous events"), serde_json::json!([]));
        let authorization = format!("Bearer {}", outsider.token);
        let nonmember = raw_http_get(addr, &format!("/directory/events?since={since}&limit=100"), &[("Authorization", authorization.as_str())]).await;
        assert_eq!(nonmember.status, 200);
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&nonmember.body).expect("nonmember events"), serde_json::json!([]));
    });
}

#[test]
fn space_public_boundary_real_socket_denies_public_raw_events_and_member_telemetry() {
    run_socket_test(|| async {
        let state = test_state().await;
        let outsider = issue_test_session(&state, "socket-public-outsider@example.invalid").await;
        let owner = issue_test_session(&state, "socket-public-owner@example.invalid").await;
        let public_space = create_space_for_test(&state, &owner.user_id, "Socket Public", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Public).await;
        let since = state.directory.head_seq().await.expect("head");
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#socket-replay-law", owner.user_id) }, DirectoryCommand::RenameSpace { space_id: public_space.clone(), name: "Socket Public Replay".into() })
            .await
            .expect("replayed raw public event");
        let receipt = issue_directory_socket_grant(bearer_headers(&outsider.token), State(state.clone())).await.expect("outsider directory grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/directory/socket/v1?since={since}");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("public outsider directory socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("credential-free directory hello");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        state
            .directory_service
            .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#socket-live-law", owner.user_id) }, DirectoryCommand::RenameSpace { space_id: public_space.clone(), name: "Socket Public Live".into() })
            .await
            .expect("live raw public event");
        state.directory_service.publish(DirectoryStreamMessage::Connection {
            phase: DirectoryConnectionPhase::Opened,
            connection: ConnectionView {
                sync_session_id: "private-sync".into(),
                space_id: public_space,
                document_id: "private-document".into(),
                surface: "private-surface".into(),
                actor: "private-actor".into(),
                user_id: Some(owner.user_id),
                email: Some("socket-public-owner@example.invalid".into()),
                role: DirectorySpaceRole::Author,
                connected_at_ms: 1,
                presence_known: true,
            },
        });
        let head = state.directory.head_seq().await.expect("post-event head");
        assert!(head > since);
        state.directory_service.publish(DirectoryStreamMessage::Heartbeat { head_seq: head });
        assert!(tokio::time::timeout(std::time::Duration::from_millis(250), socket.next()).await.is_err(), "public nonmember received a raw event, member telemetry, or global progress cursor");
    });
}

#[test]
fn admin_intent_wire_taxonomy_rejects_generic_and_unknown_commands() {
    let valid = r#"{"kind":"create-space","requestId":"request:one","name":"Studio","spaceKind":"studio","visibility":"private"}"#;
    assert!(directory::os_pack::json::from_json_str::<AdminIntentV1>(valid).is_ok());
    let generic = r#"{"kind":"directory","requestId":"request:one","command":{"kind":"create-space","name":"Studio","spaceKind":"studio","visibility":"private"}}"#;
    let forbidden = r#"{"kind":"announce-document","requestId":"request:one","descriptor":{}}"#;
    let unknown = r#"{"kind":"create-space","requestId":"request:one","name":"Studio","spaceKind":"studio","visibility":"private","actor":"admin"}"#;
    assert!(directory::os_pack::json::from_json_str::<AdminIntentV1>(generic).is_err());
    assert!(directory::os_pack::json::from_json_str::<AdminIntentV1>(forbidden).is_err());
    assert!(directory::os_pack::json::from_json_str::<AdminIntentV1>(unknown).is_err());
}

#[test]
fn admin_document_cursor_is_principal_route_and_exact_page_bound() {
    let cursor_key = [0x5a; 32];
    let principal = AdminPrincipalV1 {
        user_id: "user:admin".into(),
        auth_session_id: "session:admin".into(),
        authorization_generation: 7,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: now_ms() + 60_000,
        correlation_id: "correlation:admin".into(),
        peer_class: "admin-rest",
    };
    let cursor = admin_cursor_encode_scoped(&cursor_key, &principal, 5, Some("space:one"), ADMIN_PAGE_MAX).expect("document cursor");
    assert_eq!(cursor.len(), 84);
    assert_eq!(admin_cursor_decode_scoped(&cursor_key, &principal, 5, Some("space:one"), Some(&cursor)), Ok(ADMIN_PAGE_MAX));
    assert_eq!(admin_cursor_decode_scoped(&cursor_key, &principal, 5, Some("space:two"), Some(&cursor)), Err(StatusCode::BAD_REQUEST));
    assert_eq!(admin_cursor_decode(&cursor_key, &principal, 2, Some(&cursor)), Err(StatusCode::BAD_REQUEST));
    let mut other_principal = principal.clone();
    other_principal.auth_session_id = "session:other".into();
    assert_eq!(admin_cursor_decode_scoped(&cursor_key, &other_principal, 5, Some("space:one"), Some(&cursor)), Err(StatusCode::BAD_REQUEST));
    assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(ADMIN_PAGE_MAX) }), Ok(ADMIN_PAGE_MAX));
    assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(0) }), Err(StatusCode::BAD_REQUEST));
    assert_eq!(admin_page_limit(&AdminPageQuery { cursor: None, limit: Some(ADMIN_PAGE_MAX + 1) }), Err(StatusCode::BAD_REQUEST));
}

#[test]
fn admin_response_pages_stop_before_exact_byte_max_and_reject_one_oversized_row() {
    let cursor_key = [0x5a; 32];
    let principal = AdminPrincipalV1 {
        user_id: "user:admin".into(),
        auth_session_id: "session:admin".into(),
        authorization_generation: 7,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: now_ms() + 60_000,
        correlation_id: "correlation:admin".into(),
        peer_class: "admin-rest",
    };
    let rows = (0..ADMIN_PAGE_MAX).map(|index| os_directory::UserView { id: format!("user:{index}:{}", "i".repeat(4_000)), email: format!("{index}@{}", "e".repeat(4_000)), display_name: "n".repeat(4_000), created_at_ms: 0 }).collect();
    let page = admin_fit_page(rows, false, 7, |rows| admin_cursor_encode(&cursor_key, &principal, 2, rows.len())).expect("byte-bounded user page");
    assert!(page.rows.len() < ADMIN_PAGE_MAX);
    assert!(page.next_cursor.is_some());
    assert!(directory::os_pack::json::to_json_string(&page).len() <= ADMIN_RESPONSE_MAX_BYTES);

    let connections = (0..ADMIN_PAGE_MAX)
        .map(|index| AdminRecordedConnectionV1 {
            sync_session_id: format!("sync:{index}:{}", "s".repeat(4_000)),
            scope: DocumentScope::new("space", format!("document:{index}:{}", "d".repeat(4_000))),
            authenticated_user_id: Some(format!("user:{index}:{}", "u".repeat(4_000))),
            email: Some("admin@example.com".into()),
            role: Some(DirectorySpaceRole::Author),
            connected_at_ms: 0,
            source: "recorded-sync-session".into(),
        })
        .collect();
    let snapshot = admin_fit_connection_snapshot(connections, false, 7, 9, &cursor_key, &principal, 0).expect("byte-bounded connection snapshot");
    assert!(snapshot.rows.len() < ADMIN_PAGE_MAX);
    assert!(snapshot.next_cursor.is_some());
    assert!(directory::os_pack::json::to_json_string(&snapshot).len() <= ADMIN_RESPONSE_MAX_BYTES);

    let view = SpaceView {
        id: "space:one".into(),
        name: "Space".into(),
        kind: os_directory::DirectorySpaceKind::Studio,
        visibility: DirectorySpaceVisibility::Private,
        owner_user_id: "user:owner".into(),
        role: None,
        member_count: ADMIN_PAGE_MAX as u32,
        document_count: 0,
        active_connections: 0,
        created_at_ms: 0,
        updated_at_ms: 0,
    };
    let members = (0..ADMIN_PAGE_MAX).map(|index| MemberView { user_id: format!("user:{index}:{}", "u".repeat(4_000)), email: format!("{index}@example.com"), display_name: "n".repeat(4_000), role: DirectorySpaceRole::Author }).collect();
    let detail = admin_fit_space_detail(view, members, false, 7, &cursor_key, &principal, "space:one", 0).expect("byte-bounded member detail");
    assert!(detail.members.rows.len() < ADMIN_PAGE_MAX);
    assert!(detail.members.next_cursor.is_some());
    assert!(directory::os_pack::json::to_json_string(&detail).len() <= ADMIN_RESPONSE_MAX_BYTES);

    let oversized = vec![os_directory::UserView { id: "i".repeat(ADMIN_RESPONSE_MAX_BYTES), email: "e@example.com".into(), display_name: "name".into(), created_at_ms: 0 }];
    assert_eq!(admin_fit_page(oversized, false, 7, |_| Ok("a".repeat(84))), Err(StatusCode::PAYLOAD_TOO_LARGE));
}

#[tokio::test]
async fn retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧪️tests/🏛️retained-short-admin/🔣️.json")).expect("retained short administrator fixture");
    assert_eq!(fixture["cases"].as_array().expect("retained cases").len(), 15);
    let root = tempdir("retained-short-admin");
    std::fs::create_dir_all(&root).expect("retained administrator root");
    let path = root.join("directory.sqlite");
    let directory = SqliteDirectory::connect(path.to_str().expect("retained directory path")).await.expect("retained directory");
    let mut state = test_state_with_directory(root.join("db"), directory, 1024, 256).await;
    let physical = rusqlite::Connection::open(&path).expect("retained administrator physical reader");
    let gate = Arc::new(TestLiveGate::default());
    state.live_gate = Some(gate.clone());
    let email = "retained-short-admin@example.test";
    let _ = authorize_test_admin(&mut state, email).await;
    let admin = issue_test_session(&state, email).await;
    let owner = issue_test_session(&state, "retained-short-owner@example.test").await;
    let space = create_space_for_test(&state, &owner.user_id, "Retained Short", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let scope = DocumentScope::new(&space, "retained-short-document");
    announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
    let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
    let request_id = "request:retained-short-drop";
    let intent = AdminIntentV1::IssueDocumentShare { request_id: request_id.into(), scope: scope.clone(), ttl_secs: 600 };
    let body = directory::os_pack::json::to_json_string(&intent);
    *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), true));
    let dropped = tokio::spawn({
        let authorization = format!("Bearer {}", admin.token);
        let body = body.clone();
        async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("dropped request admission deadline").expect("dropped request admitted").forget();
    dropped.abort();
    let _ = dropped.await;
    *gate.directory_command_pause_user.lock().unwrap() = None;
    gate.directory_command_release.add_permits(1);
    let mut rows = Vec::new();
    for _ in 0..256 {
        rows = state.directory.admin_operation_audit_for_request(request_id).await.expect("dropped request audit");
        if rows.len() == 2 {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(rows.len(), 2, "request cancellation cannot cancel its retained operation");
    assert_eq!(rows[1].fact.phase, "succeeded");
    assert_eq!(rows[1].fact.outcome_code, "share-issued");
    assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(), 1);
    let authorization = format!("Bearer {}", admin.token);
    let retry = raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], body.as_bytes()).await;
    assert_eq!(retry.status, 200);
    let retry_receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&retry.body).unwrap()).expect("retry receipt");
    assert_eq!(retry_receipt.state, AdminIntentStateV1::Succeeded);
    assert!(retry_receipt.result.is_none(), "a lost one-shot share token is never stored or replayed");
    assert_eq!(
        physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(),
        1,
        "retry cannot execute the side effect twice"
    );
    let collision = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: request_id.into(), scope: scope.clone(), ttl_secs: 601 });
    assert_eq!(raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], collision.as_bytes()).await.status, 409);

    let cancelled_scope = DocumentScope::new(&space, "retained-short-cancelled");
    announce_document_for_test(&state, &cancelled_scope.space_id, &cancelled_scope.document_id).await;
    let cancelled_request = "request:retained-short-cancelled";
    let cancelled_body = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: cancelled_request.into(), scope: cancelled_scope.clone(), ttl_secs: 600 });
    *gate.directory_command_pause_user.lock().unwrap() = Some((admin.user_id.clone(), false));
    let cancelled = tokio::spawn({
        let authorization = authorization.clone();
        async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], cancelled_body.as_bytes()).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.expect("pre-effect cancellation admission deadline").expect("pre-effect cancellation admitted").forget();
    let accepted = state.directory.admin_operation_audit_for_request(cancelled_request).await.expect("cancelled acceptance");
    let operation_id = accepted.first().expect("cancelled accepted row").fact.operation_id.clone();
    cancel_admin_operation(Path(operation_id), bearer_headers(&admin.token), loopback_peer(), State(state.clone())).await.expect("cancel retained operation");
    *gate.directory_command_pause_user.lock().unwrap() = None;
    gate.directory_command_release.add_permits(1);
    let cancelled = cancelled.await.expect("cancelled request task");
    let cancelled_receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&cancelled.body).unwrap()).expect("cancelled receipt");
    assert_eq!(cancelled_receipt.state, AdminIntentStateV1::Cancelled);
    assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![cancelled_scope.space_id, cancelled_scope.document_id], |row| row.get(0)).unwrap(), 0);

    let admitted_scope = DocumentScope::new(&space, "retained-short-admitted");
    announce_document_for_test(&state, &admitted_scope.space_id, &admitted_scope.document_id).await;
    let admitted_request = "request:retained-short-admitted";
    let admitted_body = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: admitted_request.into(), scope: admitted_scope.clone(), ttl_secs: 600 });
    gate.admin_effect_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
    let admitted = tokio::spawn({
        let authorization = authorization.clone();
        async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], admitted_body.as_bytes()).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.admin_effect_admitted.acquire()).await.expect("admitted effect deadline").expect("effect admitted").forget();
    let accepted = state.directory.admin_operation_audit_for_request(admitted_request).await.expect("admitted acceptance");
    let operation_id = accepted.first().expect("admitted accepted row").fact.operation_id.clone();
    cancel_admin_operation(Path(operation_id), bearer_headers(&admin.token), loopback_peer(), State(state.clone())).await.expect("late cancellation request");
    gate.admin_effect_pause_enabled.store(false, std::sync::atomic::Ordering::Release);
    gate.admin_effect_release.add_permits(1);
    let admitted = admitted.await.expect("admitted request task");
    let admitted_receipt: AdminIntentReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&admitted.body).unwrap()).expect("admitted receipt");
    assert_eq!(admitted_receipt.state, AdminIntentStateV1::Succeeded, "cancellation after effect admission cannot invent rollback");
    assert!(admitted_receipt.result.as_ref().and_then(|result| result.share_token.as_ref()).is_some());
    assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![admitted_scope.space_id, admitted_scope.document_id], |row| row.get(0)).unwrap(), 1);

    let fenced_scope = DocumentScope::new(&space, "retained-short-admitted-deadline");
    announce_document_for_test(&state, &fenced_scope.space_id, &fenced_scope.document_id).await;
    let first_request = "request:retained-short-admitted-deadline-first";
    let second_request = "request:retained-short-admitted-deadline-second";
    let first_body = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: first_request.into(), scope: fenced_scope.clone(), ttl_secs: 600 });
    let second_body = directory::os_pack::json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: second_request.into(), scope: fenced_scope.clone(), ttl_secs: 600 });
    gate.admin_effect_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
    let mut first = tokio::spawn({
        let authorization = authorization.clone();
        async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], first_body.as_bytes()).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.admin_effect_admitted.acquire()).await.expect("first admitted writer deadline").expect("first writer admitted").forget();
    let mut second = tokio::spawn({
        let authorization = authorization.clone();
        async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], second_body.as_bytes()).await }
    });
    for _ in 0..256 {
        if state.directory.admin_operation_audit_for_request(second_request).await.expect("competing acceptance read").len() == 1 {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert!(tokio::time::timeout(std::time::Duration::from_millis(50), &mut second).await.is_err(), "competing same-scope writer cannot pass retained authority");
    let first_response = tokio::time::timeout(ADMIN_OPERATION_DEADLINE + std::time::Duration::from_secs(2), &mut first).await.expect("first HTTP deadline response").expect("first HTTP task");
    assert_eq!(first_response.status, 503, "the HTTP waiter expires without cancelling its admitted writer");
    assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![fenced_scope.space_id, fenced_scope.document_id], |row| row.get(0)).unwrap(), 0);
    gate.admin_effect_pause_enabled.store(false, std::sync::atomic::Ordering::Release);
    gate.admin_effect_release.add_permits(1);
    let second_response = tokio::time::timeout(std::time::Duration::from_secs(5), &mut second).await.expect("competing writer completion deadline").expect("competing HTTP task");
    assert_eq!(second_response.status, 200);
    let mut first_rows = Vec::new();
    for _ in 0..256 {
        first_rows = state.directory.admin_operation_audit_for_request(first_request).await.expect("first admitted writer audit");
        if first_rows.len() == 2 {
            break;
        }
        tokio::task::yield_now().await;
    }
    let second_rows = state.directory.admin_operation_audit_for_request(second_request).await.expect("second admitted writer audit");
    assert_eq!(first_rows.len(), 2);
    assert_eq!(second_rows.len(), 2);
    assert_eq!(first_rows[1].fact.phase, "succeeded");
    assert_eq!(second_rows[1].fact.phase, "succeeded");
    assert!(first_rows[1].sequence < second_rows[1].sequence, "the retained first writer terminal precedes its blocked successor");
    assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![fenced_scope.space_id, fenced_scope.document_id], |row| row.get(0)).unwrap(), 2);
    assert_eq!(state.admin_operation_tasks.task_count(), 0);
    drop(physical);
    stop_recovery_server(state, shutdown, server).await;
}

#[tokio::test]
async fn retained_short_admin_shutdown_drains_before_bounded_abort_and_receipt_reconciliation_is_exact() {
    let cancelled = AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    };
    assert!(cancelled.request_cancel());
    assert!(!cancelled.admit_effect(), "cancellation linearized before admission refuses the effect");
    let admitted = AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    };
    assert!(admitted.admit_effect());
    assert!(!admitted.request_cancel(), "admission linearized before cancellation cannot claim rollback");
    ADMIN_SECRET_WIPE_BYTES.store(0, std::sync::atomic::Ordering::SeqCst);
    let invite = "invite.v1.selector.secret".to_string();
    let share = "share.v1.selector.secret".to_string();
    let expected_wipes = invite.len() + share.len();
    drop(AdminIntentSecretResult::Invite(invite));
    drop(AdminIntentSecretResult::Share(share));
    assert_eq!(ADMIN_SECRET_WIPE_BYTES.load(std::sync::atomic::Ordering::SeqCst), expected_wipes);

    let state = test_state().await;
    let principal = AdminPrincipalV1 {
        user_id: "user:admin".into(),
        auth_session_id: "session:admin".into(),
        authorization_generation: 1,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: now_ms() + 60_000,
        correlation_id: "correlation:retained".into(),
        peer_class: "admin-rest",
    };
    let metadata = AdminIntentMetadata { intent_kind: "delete-space", target_kind: "space", target_id: "space:one".into(), reason_code: None };
    let mut accepted = new_admin_audit_fact(&principal, "request:stale-accepted", &"11".repeat(32), "operation:stale-accepted", &metadata, "accepted", None, "accepted");
    accepted.occurred_at = now_ms() - 60_000;
    state.directory.append_admin_operation_audit(&accepted).await.expect("stale accepted audit");
    let rows = reconcile_stale_admin_acceptance(&state, state.directory.admin_operation_audit_for_request(&accepted.request_id).await.expect("stale audit read")).await.expect("stale reconciliation");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].fact.phase, "accepted", "age alone cannot prove cancellation or rollback");
    let unresolved = admin_audit_visibility(admin_audit_receipt(&rows).expect("stale accepted receipt"), false);
    assert_eq!(unresolved.state, AdminIntentStateV1::Indeterminate);
    assert_eq!(unresolved.outcome.code, "admin-effect-outcome-indeterminate");
    assert_eq!(admin_audit_visibility(admin_audit_receipt(&rows).expect("live accepted receipt"), true).state, AdminIntentStateV1::Accepted);

    let owner = issue_test_session(&state, "retained-reconcile-owner@example.test").await;
    let space = create_space_for_test(&state, &owner.user_id, "Receipt Reconciliation", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let scope = DocumentScope::new(&space, "receipt-reconciliation-document");
    announce_document_for_test(&state, &scope.space_id, &scope.document_id).await;
    let committed_digest = "22".repeat(32);
    let committed_metadata = AdminIntentMetadata { intent_kind: "issue-document-share", target_kind: "document", target_id: format!("{}/{}", scope.space_id, scope.document_id), reason_code: None };
    let mut committed = new_admin_audit_fact(&principal, "request:committed-accepted", &committed_digest, "operation:committed-accepted", &committed_metadata, "accepted", None, "accepted");
    committed.occurred_at = now_ms() - 60_000;
    state.directory.append_admin_operation_audit(&committed).await.expect("committed acceptance audit");
    let effect = NewAdminOperationEffectReceiptV1 { operation_id: committed.operation_id.clone(), intent_digest: committed.intent_digest.clone(), committed_at: now_ms(), outcome_code: "share-issued".into() };
    let AdminEffectCommitV1::Applied(issued) = state.directory.issue_share_token_as_with_admin_effect(&scope, 600, Some(&principal.user_id), &principal.correlation_id, &effect).await else {
        panic!("atomic share and effect receipt");
    };
    drop(issued);
    assert!(tokio::time::timeout(std::time::Duration::ZERO, std::future::pending::<()>()).await.is_err(), "effect commit can precede acknowledgement deadline");
    assert!(state.directory.admin_operation_effect_receipt(&committed.operation_id, &"33".repeat(32)).await.expect("mismatched receipt query").is_none());
    let rows = reconcile_stale_admin_acceptance(&state, state.directory.admin_operation_audit_for_request(&committed.request_id).await.expect("committed audit read")).await.expect("factual receipt reconciliation");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].fact.phase, "succeeded");
    assert_eq!(rows[1].fact.outcome_code, "share-issued");
    assert_eq!(rows[1].fact.event_seq_first, None);
    assert_eq!(rows[1].fact.event_seq_last, None);

    let owner = AdminOperationTaskOwner::new(std::time::Duration::from_secs(1));
    let runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    let drained = Arc::new(std::sync::atomic::AtomicBool::new(false));
    assert!(
        owner
            .spawn("operation:drain".into(), runtime.clone(), {
                let drained = drained.clone();
                let runtime = runtime.clone();
                async move {
                    while !runtime.progress().cancel_requested {
                        tokio::task::yield_now().await;
                    }
                    drained.store(true, std::sync::atomic::Ordering::Release);
                }
            })
            .is_ok()
    );
    owner.shutdown().await;
    assert!(drained.load(std::sync::atomic::Ordering::Acquire));
    assert_eq!(owner.task_count(), 0);
    assert!(owner.spawn("operation:after-close".into(), runtime.clone(), async {}).is_err(), "shutdown refuses later operation tasks");

    struct DropSignal(Option<tokio::sync::oneshot::Sender<()>>);
    impl Drop for DropSignal {
        fn drop(&mut self) {
            if let Some(signal) = self.0.take() {
                let _ = signal.send(());
            }
        }
    }
    let aborting = AdminOperationTaskOwner::new(std::time::Duration::from_millis(10));
    let abort_runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + ADMIN_OPERATION_DEADLINE,
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_ADMITTED),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    let (dropped_tx, dropped_rx) = tokio::sync::oneshot::channel();
    assert!(
        aborting
            .spawn("operation:bounded-abort".into(), abort_runtime, async move {
                let _signal = DropSignal(Some(dropped_tx));
                std::future::pending::<()>().await;
            })
            .is_ok()
    );
    aborting.shutdown().await;
    tokio::time::timeout(std::time::Duration::from_secs(1), dropped_rx).await.expect("aborted task drop deadline").expect("aborted task dropped");
    assert_eq!(aborting.task_count(), 0);
}

#[tokio::test]
async fn admin_rebuild_slots_are_atomic_and_abort_closes_once() {
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
    directory.seed().await.expect("seed directory");
    let directory = Arc::new(HubDirectories::from(directory));
    let operations = Arc::new(ShardedMap::new());
    let operation_slots = Arc::new(tokio::sync::Semaphore::new(64));
    let barrier = Arc::new(tokio::sync::Barrier::new(129));
    let release = Arc::new(tokio::sync::Notify::new());
    let acquired = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let mut contenders = Vec::new();
    for _ in 0..128 {
        let barrier = barrier.clone();
        let release = release.clone();
        let acquired = acquired.clone();
        let slots = operation_slots.clone();
        contenders.push(tokio::spawn(async move {
            barrier.wait().await;
            if let Ok(_permit) = slots.try_acquire_owned() {
                acquired.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                release.notified().await;
            }
        }));
    }
    barrier.wait().await;
    for _ in 0..128 {
        if acquired.load(std::sync::atomic::Ordering::Acquire) == 64 {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(acquired.load(std::sync::atomic::Ordering::Acquire), 64);
    assert_eq!(operation_slots.available_permits(), 0);
    release.notify_waiters();
    for contender in contenders {
        contender.await.expect("slot contender");
    }
    assert_eq!(operation_slots.available_permits(), 64);

    let principal = AdminPrincipalV1 {
        user_id: "user:admin".into(),
        auth_session_id: "session:admin".into(),
        authorization_generation: 1,
        identity_provider: "test".into(),
        identity_subject_digest: [7; 32],
        expires_at_ms: now_ms() + 60_000,
        correlation_id: "correlation:admin".into(),
        peer_class: "admin-rest",
    };
    let metadata = AdminIntentMetadata { intent_kind: "rebuild-directory-projections", target_kind: "directory", target_id: "directory".into(), reason_code: None };
    let request_id = "request:abort";
    let digest = "11".repeat(32);
    let operation_id = "operation:abort";
    let accepted = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "accepted", None, "accepted");
    directory.append_admin_operation_audit(&accepted).await.expect("accepted audit");
    let runtime = Arc::new(AdminOperationRuntime {
        deadline: std::time::Instant::now() + std::time::Duration::from_secs(10),
        completed: std::sync::atomic::AtomicU64::new(0),
        total: std::sync::atomic::AtomicU64::new(0),
        effect_state: std::sync::atomic::AtomicU8::new(ADMIN_EFFECT_PRE_EFFECT),
        cooperative_cancel_requested: std::sync::atomic::AtomicBool::new(false),
    });
    operations.insert(operation_id.into(), runtime);
    let cleanup = AdminOperationCleanup { operations: operations.clone(), operation_id: operation_id.into(), _permit: operation_slots.clone().try_acquire_owned().expect("cleanup slot") };
    let task = tokio::spawn(async move {
        let _cleanup = cleanup;
        std::future::pending::<()>().await;
    });
    tokio::task::yield_now().await;
    task.abort();
    let _ = task.await;
    let mut rows = Vec::new();
    for _ in 0..128 {
        rows = directory.admin_operation_audit_for_request(request_id).await.expect("operation audit");
        if operations.get_cloned(operation_id).is_none() {
            break;
        }
        tokio::task::yield_now().await;
    }
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].fact.phase, "accepted");
    assert_eq!(rows[0].fact.outcome_code, "accepted");
    assert!(operations.get_cloned(operation_id).is_none());
    assert_eq!(operation_slots.available_permits(), 64);
    let later = new_admin_audit_fact(&principal, request_id, &digest, operation_id, &metadata, "succeeded", None, "late-success");
    assert_eq!(directory.append_admin_operation_audit(&later).await.expect("factual late terminal").fact.phase, "succeeded", "task abortion cannot invent rollback before the factual result is known");
}

fn presence_hex_bytes(hex: &str) -> Vec<u8> {
    assert_eq!(hex.len() % 2, 0);
    (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).expect("presence fixture byte")).collect()
}

fn presence_normalization_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/🪪️presence-normalization-v1/🧪️fixture/🔣️.json")).expect("presence normalization fixture")
}

fn test_presence_slot(live: &str, user_id: Option<&str>, now: tokio::time::Instant) -> PresenceLeaseSlot {
    PresenceLeaseSlot {
        socket_live_id: live.into(),
        expires_at: now + std::time::Duration::from_millis(PRESENCE_LEASE_TTL_MS),
        user_id: user_id.map(str::to_string),
        connected_at_ms: 1000,
        label: None,
        role: None,
        document_surface: Some("surface".into()),
        color: 0,
        peer: None,
    }
}

async fn presence_test_peer(pack: &[u8]) -> Vec<u8> {
    let fixture = presence_normalization_fixture();
    let mut peer = protocol::decode_presence_peer(&presence_hex_bytes(fixture["vectors"][0]["rawPeerHex"].as_str().expect("raw peer"))).await.expect("fixture peer");
    peer.presence_pack = Some(pack.to_vec());
    protocol::encode_presence_peer(&peer).await
}

async fn presence_frame_has_pack(frame: ServerFrame, pack: &[u8]) -> bool {
    let ServerFrame::Presence { peers } = frame else { return false };
    peers.len() == 1 && protocol::decode_presence_peer(&peers[0]).await.is_ok_and(|peer| peer.presence_pack.as_deref() == Some(pack))
}

#[tokio::test]
async fn presence_normalization_matches_neutral_authority_and_no_effect_rejections() {
    let state = lag_test_state(1024, 256).await;
    let fixture = presence_normalization_fixture();
    let head = state.directory.head_seq().await.expect("directory head");
    let now = tokio::time::Instant::now();
    for vector in fixture["vectors"].as_array().expect("vectors") {
        let document_id = vector["name"].as_str().expect("vector name");
        let key = document_scope_key_v1(&DocumentScope::new(STUDIO, document_id));
        let admitted = &vector["admission"];
        let actor = admitted["actor"].as_str().expect("admitted actor");
        let mut slot = test_presence_slot("live", admitted["userId"].as_str(), now);
        slot.connected_at_ms = admitted["connectedAtMs"].as_i64().expect("admitted time");
        slot.label = admitted["label"].as_str().map(str::to_string);
        slot.role = admitted["role"].as_str().map(str::to_string);
        slot.color = u8::try_from(admitted["color"].as_u64().expect("admitted color")).expect("palette index");
        slot.document_surface = admitted["surface"].as_str().map(str::to_string);
        let deadline = slot.expires_at;
        let mut receiver = state.fanout_for(&key).subscribe();
        assert_eq!(state.install_presence_slot(&key, STUDIO, document_id, actor, slot).await, PresenceLeaseTransition::NoChange);
        let raw = presence_hex_bytes(vector["rawPeerHex"].as_str().expect("raw peer"));
        let transition = state.refresh_document_presence(&key, STUDIO, document_id, actor, "live", raw.clone(), now + std::time::Duration::from_secs(1)).await;
        let map_key = (key.clone(), actor.to_string());
        if vector["expected"]["accepted"] == true {
            let expected = presence_hex_bytes(vector["expected"]["normalizedPeerHex"].as_str().expect("normalized peer"));
            assert_eq!(transition, PresenceLeaseTransition::Published, "{document_id}");
            assert_eq!(state.presence_snapshot(&key).peers, vec![expected.clone()], "{document_id}");
            assert_eq!(state.presence_snapshot(&key).actors.len(), usize::from(admitted["surface"].is_string()), "non-plan rows cannot mint a directory surface: {document_id}");
            assert!(matches!(receiver.try_recv(), Ok(ServerFrame::Presence { peers }) if peers == vec![expected]));
            assert_eq!(state.refresh_document_presence(&key, STUDIO, document_id, actor, "live", raw, now + std::time::Duration::from_secs(2)).await, PresenceLeaseTransition::NoChange);
            assert_eq!(state.presence.with(&map_key, |slot| slot.expect("slot").expires_at), deadline + std::time::Duration::from_secs(2));
        } else {
            assert_eq!(transition, PresenceLeaseTransition::Rejected, "{document_id}");
            assert!(state.presence_snapshot(&key).peers.is_empty(), "{document_id}");
            assert_eq!(state.presence.with(&map_key, |slot| slot.expect("slot").expires_at), deadline, "{document_id}");
        }
        assert!(matches!(receiver.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "no duplicate or rejected publication: {document_id}");
        assert_eq!(vector["expected"]["durableWrites"], 0);
    }
    assert_eq!(state.directory.head_seq().await.expect("directory head"), head);
    eprintln!("[DEBUG] presence normalization: 17 exact neutral admission vectors, unchanged rejected TTL, duplicate suppression, zero directory writes");
}

#[test]
fn presence_normalization_socket_overwrites_identity_and_rejects_without_refresh() {
    run_socket_test(|| async {
        let mut state = lag_test_state(1024, 256).await;
        let clock = Arc::new(TestPresenceClock::new());
        state.presence_clock = Some(clock.clone());
        let token = seed_author_token(&state).await;
        let document_id = "presence-normalized-socket";
        announce_document_for_test(&state, STUDIO, document_id).await;
        let scope = DocumentScope::new(STUDIO, document_id);
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
        install_document_open_catalog_for_test(&mut state, &descriptor);
        let (plan, receipt) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-normalization").await;
        let session = state.directory.authenticate_session(&SessionCapability::parse(&token).expect("session capability")).await.expect("session lookup").expect("session");
        let user = state.directory.get_user(&session.user_id).await.expect("user lookup").expect("user");
        let started = now_ms();
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/{document_id}/socket/v1?surface={}", plan.surface.surface_id);
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("admitted socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("hello");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        let ServerFrame::Session { actor, color } = next_server_frame(&mut socket).await else { panic!("session frame") };
        assert_eq!(actor, receipt.actor_id);
        let fixture = presence_normalization_fixture();
        let raw = presence_hex_bytes(fixture["vectors"][0]["rawPeerHex"].as_str().expect("raw peer"));
        let input = protocol::decode_presence_peer(&raw).await.expect("raw peer decode");
        socket.send(client_binary(&ClientFrame::Presence { peer: raw.clone() }, Lane::Preview).await).await.expect("forged peer send");
        let ServerFrame::Presence { peers } = next_server_frame(&mut socket).await else { panic!("presence frame") };
        assert_eq!(peers.len(), 1);
        let normalized = protocol::decode_presence_peer(&peers[0]).await.expect("normalized peer");
        assert_eq!(normalized.actor, actor);
        assert_eq!(normalized.user_id, Some(session.user_id));
        assert_eq!(normalized.label, Some(user.display_name));
        assert_eq!(normalized.role.as_deref(), Some("author"));
        assert_eq!(normalized.color, Some(color));
        assert_eq!(normalized.surface, Some(plan.surface.surface_id));
        assert!(normalized.connected_at_ms >= started && normalized.connected_at_ms <= now_ms());
        assert_eq!(normalized.presence_pack, input.presence_pack);
        assert_eq!(normalized.drag_ghost_json, input.drag_ghost_json);
        assert_eq!(normalized.interaction, input.interaction);
        assert_eq!(normalized.views, input.views);
        assert_eq!(normalized.ui, input.ui);
        assert_eq!(protocol::encode_presence_peer(&normalized).await, peers[0]);
        let key = document_scope_key_v1(&scope);
        assert_eq!(state.presence_snapshot(&key).actors[0].surface, normalized.surface.as_deref().expect("plan surface"));
        let deadline = state.presence.with(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").expires_at);
        let head = state.directory.head_seq().await.expect("head before rejected input");
        clock.advance_to(5_000);
        for vector in fixture["vectors"]
            .as_array()
            .expect("vectors")
            .iter()
            .filter(|vector| !vector["expected"]["accepted"].as_bool().expect("accepted") && !vector["name"].as_str().expect("name").starts_with("authoritative-") && !vector["name"].as_str().expect("name").starts_with("normalized-"))
        {
            socket.send(client_binary(&ClientFrame::Presence { peer: presence_hex_bytes(vector["rawPeerHex"].as_str().expect("hostile peer")) }, Lane::Preview).await).await.expect("hostile send");
        }
        socket.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-fifo".into(), seq: 1, payload: vec![] }, Lane::Preview).await).await.expect("rejection fence");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq: 1, .. } if key == "presence-fifo"), "every rejected frame precedes the FIFO marker without an interim roster");
        assert_eq!(state.presence.with(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").expires_at), deadline);
        assert_eq!(state.presence_snapshot(&key).peers, peers);
        assert_eq!(state.directory.head_seq().await.expect("head after rejected input"), head);
        for (sequence, name, label_bytes) in [(2, "authoritative-field-over-limit", 1025), (3, "normalized-entry-over-limit", 1024)] {
            let vector = fixture["vectors"].as_array().expect("vectors").iter().find(|vector| vector["name"] == name).expect("authority expansion vector");
            let saved_label = state.presence.with_mut(&(key.clone(), actor.clone()), |slot| std::mem::replace(&mut slot.expect("visible slot").label, Some("l".repeat(label_bytes))));
            socket.send(client_binary(&ClientFrame::Presence { peer: presence_hex_bytes(vector["rawPeerHex"].as_str().expect("expansion raw peer")) }, Lane::Preview).await).await.expect("authority-expanded input");
            socket.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-fifo".into(), seq: sequence, payload: vec![] }, Lane::Preview).await).await.expect("expansion fence");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq, .. } if key == "presence-fifo" && seq == sequence), "authority-expanded output is rejected before publication: {name}");
            assert_eq!(state.presence.with(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").expires_at), deadline);
            assert_eq!(state.presence_snapshot(&key).peers, peers);
            state.presence.with_mut(&(key.clone(), actor.clone()), |slot| slot.expect("visible slot").label = saved_label);
        }
        socket.send(client_binary(&ClientFrame::Presence { peer: raw }, Lane::Preview).await).await.expect("identical refresh");
        socket.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-fifo".into(), seq: 4, payload: vec![] }, Lane::Preview).await).await.expect("duplicate fence");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Preview { key, seq: 4, .. } if key == "presence-fifo"), "identical normalized peer does not republish");
        assert_eq!(state.presence.with(&(key, actor), |slot| slot.expect("visible slot").expires_at), deadline + std::time::Duration::from_secs(5));
        socket.close(None).await.expect("close socket");
        eprintln!("[DEBUG] admitted plan-backed socket overwrote all identity fields, preserved five ephemerals, and rejected malformed input without visibility, TTL or directory changes");
    });
}

#[test]
fn presence_lease_reconnect_rejects_old_live_refresh_and_close() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        state.presence_clock = Some(Arc::new(TestPresenceClock::new()));
        let token = seed_author_token(&state).await;
        let document_id = "presence-reconnect";
        announce_document_for_test(&state, STUDIO, document_id).await;
        let scope = DocumentScope::new(STUDIO, document_id);
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
        install_document_open_catalog_for_test(&mut state, &descriptor);
        let observer_token = seed_author_token(&state).await;
        let (plan, first) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-first").await;
        let (_, second) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:presence-second").await;
        let (_, observer_grant) = issue_and_exchange_document_open_plan_for_test(&state, &observer_token, &scope, "client:presence-observer").await;
        let first_record = state.socket_grants.pending(&SocketGrantCapability::parse(&first.grant).expect("first capability"), &SocketAudienceV1::Document(scope.clone()), now_ms()).expect("first pending record");
        assert_eq!(first.actor_id, second.actor_id);
        assert_ne!(first.actor_id, observer_grant.actor_id);
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/{document_id}/socket/v1?surface={}", plan.surface.surface_id);
        let (mut observer, _) = connect_async(socket_request(&url, &observer_grant.grant)).await.expect("observer socket");
        observer.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("observer hello");
        assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Session { .. }));
        let (mut socket_a, _) = connect_async(socket_request(&url, &first.grant)).await.expect("first socket");
        socket_a.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("first hello");
        assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Session { actor, .. } if actor == first.actor_id));
        socket_a.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"old-live").await }, Lane::Preview).await).await.expect("first presence");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket_a).await, b"old-live").await);
        assert!(presence_frame_has_pack(next_server_frame(&mut observer).await, b"old-live").await);
        let key = document_scope_key_v1(&scope);
        let first_live = state.presence.with(&(key.clone(), first.actor_id.clone()), |slot| slot.expect("first slot").socket_live_id.clone());

        let (mut socket_b, _) = connect_async(socket_request(&url, &second.grant)).await.expect("replacement socket");
        socket_b.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("replacement hello");
        assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Session { actor, .. } if actor == second.actor_id));
        assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Presence { peers } if peers.is_empty()), "replacement removes the old visible row");
        assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Presence { peers } if peers.is_empty()));
        socket_a.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"stale-refresh").await }, Lane::Preview).await).await.expect("stale refresh");
        socket_a.send(client_binary(&ClientFrame::PreviewPublish { key: "presence-stale-fifo".into(), seq: 1, payload: vec![] }, Lane::Preview).await).await.expect("stale owner fence");
        assert!(matches!(next_server_frame(&mut socket_a).await, ServerFrame::Preview { seq: 1, .. }));
        assert!(matches!(next_server_frame(&mut socket_b).await, ServerFrame::Preview { seq: 1, .. }));
        assert!(matches!(next_server_frame(&mut observer).await, ServerFrame::Preview { seq: 1, .. }));
        socket_b.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"current-live").await }, Lane::Preview).await).await.expect("current refresh");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket_b).await, b"current-live").await);
        assert!(presence_frame_has_pack(next_server_frame(&mut observer).await, b"current-live").await);
        socket_a.close(None).await.expect("stale socket close");
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while state.socket_grants.is_live(&first_record, &first_live) {
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            }
        })
        .await
        .expect("old socket cleanup completed");
        socket_b.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"current-live-2").await }, Lane::Preview).await).await.expect("current refresh after stale close");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket_b).await, b"current-live-2").await);
        assert!(presence_frame_has_pack(next_server_frame(&mut observer).await, b"current-live-2").await);
        let snapshot = state.presence_snapshot(&key);
        assert_eq!(snapshot.actors.len(), 1);
        assert_eq!(snapshot.actors[0].surface, plan.surface.surface_id);
        assert_eq!(snapshot.actors[0].actor, second.actor_id);
        observer.close(None).await.expect("observer close");
        socket_b.close(None).await.expect("current socket close");
        eprintln!("[DEBUG] plan-backed reconnect retained only the new live owner across stale refresh and close, observed by an independent admitted socket");
    });
}

#[test]
fn presence_lease_expires_server_clocked_visibility_without_socket_close() {
    run_socket_test(|| async {
        let mut state = test_state().await;
        let clock = Arc::new(TestPresenceClock::new());
        state.presence_clock = Some(clock.clone());
        let token = seed_author_token(&state).await;
        let document_id = "presence-expiry";
        announce_document_for_test(&state, STUDIO, document_id).await;
        let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document_id.to_string())), bearer_headers(&token), State(state.clone())).await.expect("socket grant").0;
        let addr = spawn_server(state.clone()).await;
        let url = format!("ws://{addr}/spaces/{STUDIO}/documents/{document_id}/socket/v1");
        let (mut socket, _) = connect_async(socket_request(&url, &receipt.grant)).await.expect("presence socket");
        socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
        socket.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"visible").await }, Lane::Preview).await).await.expect("visible presence");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket).await, b"visible").await);
        let key = document_scope_key_v1(&DocumentScope::new(STUDIO, document_id));
        let mut observed = state.fanout_for(&key).subscribe();
        clock.gate_ticks.store(true, std::sync::atomic::Ordering::Release);
        clock.advance_to(PRESENCE_LEASE_TTL_MS - 1);
        clock.evaluate_tick(false).await;
        assert_eq!(state.presence_snapshot(&key).peers.len(), 1, "an evaluated server tick preserves the lease immediately before its deadline");
        assert!(matches!(observed.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "the evaluated early tick publishes no expiry");
        clock.advance_to(PRESENCE_LEASE_TTL_MS);
        clock.evaluate_tick(true).await;
        assert_eq!(clock.tick_release.available_permits(), 0, "ungating leaves no stale release permit");
        assert!(state.presence_snapshot(&key).peers.is_empty());
        assert!(matches!(observed.try_recv(), Ok(ServerFrame::Presence { peers }) if peers.is_empty()));
        assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Presence { peers } if peers.is_empty()), "the server tick publishes exact-deadline expiry");
        socket.send(client_binary(&ClientFrame::Presence { peer: presence_test_peer(b"revived").await }, Lane::Preview).await).await.expect("live socket refresh after visibility expiry");
        assert!(presence_frame_has_pack(next_server_frame(&mut socket).await, b"revived").await, "expiry does not close or unregister the authenticated socket");
        eprintln!("[DEBUG] server tick barriers preserved visibility at TTL-1, published expiry at TTL, and kept the live socket refreshable");
    });
}

#[tokio::test]
async fn presence_lease_enforces_shared_roster_bounds_and_actor_order() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📦️packages/🦀️rust/🧪️fixtures/👥️presence-lease-v1/🧪️fixture/🔣️.json")).expect("presence lease fixture");
    assert_eq!(fixture["limits"]["ttlMs"].as_u64(), Some(PRESENCE_LEASE_TTL_MS));
    assert_eq!(fixture["limits"]["maximumItems"].as_u64(), Some(PRESENCE_ROSTER_MAXIMUM_ITEMS as u64));
    assert_eq!(fixture["limits"]["maximumEntryBytes"].as_u64(), Some(PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES as u64));
    assert_eq!(fixture["limits"]["maximumBytes"].as_u64(), Some(PRESENCE_ROSTER_MAXIMUM_BYTES as u64));
    let state = test_state().await;
    let ordered_scope = DocumentScope::new(STUDIO, "presence-order");
    let ordered_key = document_scope_key_v1(&ordered_scope);
    let now = tokio::time::Instant::now();
    for (actor, live, peer) in [("actor-z", "live-z", b"aaa".to_vec()), ("actor-a", "live-a", b"zzz".to_vec())] {
        assert_eq!(state.install_presence_slot(&ordered_key, STUDIO, &ordered_scope.document_id, actor, test_presence_slot(live, None, now)).await, PresenceLeaseTransition::NoChange);
        assert_eq!(state.refresh_presence(&ordered_key, STUDIO, &ordered_scope.document_id, actor, live, peer, now).await, PresenceLeaseTransition::Published);
    }
    let ordered = state.presence_snapshot(&ordered_key);
    assert_eq!(ordered.actors.iter().map(|actor| actor.actor.as_str()).collect::<Vec<_>>(), vec!["actor-a", "actor-z"]);
    assert_eq!(ordered.peers, vec![b"zzz".to_vec(), b"aaa".to_vec()], "opaque bytes do not select roster order");

    let full_scope = DocumentScope::new(STUDIO, "presence-full");
    let full_key = document_scope_key_v1(&full_scope);
    for index in 0..PRESENCE_ROSTER_MAXIMUM_ITEMS {
        let actor = format!("actor-{index:03}");
        let live = format!("live-{index:03}");
        assert_eq!(state.install_presence_slot(&full_key, STUDIO, &full_scope.document_id, &actor, test_presence_slot(&live, None, now)).await, PresenceLeaseTransition::NoChange);
        assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, &actor, &live, vec![0; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES], now).await, PresenceLeaseTransition::Published);
    }
    assert_eq!(state.presence_snapshot(&full_key).peers.len(), PRESENCE_ROSTER_MAXIMUM_ITEMS);
    assert_eq!(state.install_presence_slot(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", test_presence_slot("live-overflow", None, now)).await, PresenceLeaseTransition::NoChange);
    let deadline = state.presence.with(&(full_key.clone(), "actor-overflow".to_string()), |slot| slot.expect("overflow slot").expires_at);
    assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", "live-overflow", vec![1], now + std::time::Duration::from_secs(1)).await, PresenceLeaseTransition::Rejected);
    assert_eq!(state.refresh_presence(&full_key, STUDIO, &full_scope.document_id, "actor-overflow", "live-overflow", vec![1; PRESENCE_ROSTER_MAXIMUM_ENTRY_BYTES + 1], now + std::time::Duration::from_secs(2)).await, PresenceLeaseTransition::Rejected);
    assert_eq!(state.presence.with(&(full_key, "actor-overflow".to_string()), |slot| slot.expect("overflow slot").expires_at), deadline, "rejection cannot refresh the lease deadline");

    let normalized_document = "presence-normalized-capacity";
    let normalized_key = document_scope_key_v1(&DocumentScope::new(STUDIO, normalized_document));
    let raw = presence_test_peer(b"canonical-capacity").await;
    let mut observed = state.fanout_for(&normalized_key).subscribe();
    for index in 0..=PRESENCE_ROSTER_MAXIMUM_ITEMS {
        let actor = format!("normalized-{index:03}");
        let live = format!("normalized-live-{index:03}");
        let slot = test_presence_slot(&live, Some("normalized-user"), now);
        let deadline = slot.expires_at;
        assert_eq!(state.install_presence_slot(&normalized_key, STUDIO, normalized_document, &actor, slot).await, PresenceLeaseTransition::NoChange);
        let before = state.presence_snapshot(&normalized_key).peers;
        let result = state.refresh_document_presence(&normalized_key, STUDIO, normalized_document, &actor, &live, raw.clone(), now + std::time::Duration::from_secs(1)).await;
        if index < PRESENCE_ROSTER_MAXIMUM_ITEMS {
            assert_eq!(result, PresenceLeaseTransition::Published);
            let ServerFrame::Presence { peers } = observed.try_recv().expect("normalized publication") else { panic!("normalized roster frame") };
            assert_eq!(peers.len(), index + 1);
            let peer = protocol::decode_presence_peer(peers.last().unwrap()).await.expect("canonical admitted row");
            assert_eq!(peer.actor, actor);
            assert_eq!(peer.user_id.as_deref(), Some("normalized-user"));
        } else {
            assert_eq!(result, PresenceLeaseTransition::Rejected);
            assert_eq!(state.presence_snapshot(&normalized_key).peers, before);
            assert_eq!(state.presence.with(&(normalized_key.clone(), actor), |slot| slot.expect("rejected normalized slot").expires_at), deadline);
            assert!(matches!(observed.try_recv(), Err(broadcast::error::TryRecvError::Empty)));
        }
    }
    eprintln!("[DEBUG] canonical presence ingress admitted 64 bounded actors and rejected the 65th without TTL, roster or fanout effects");
}

#[tokio::test]
async fn presence_lease_restart_is_empty_and_directory_presence_is_member_only() {
    let state = test_state().await;
    let key = document_scope_key_v1(&DocumentScope::new(STUDIO, "presence-restart"));
    let before = state.directory.head_seq().await.expect("directory head");
    let now = tokio::time::Instant::now();
    assert_eq!(state.install_presence_slot(&key, STUDIO, "presence-restart", "actor-a", test_presence_slot("live-a", Some("seed"), now)).await, PresenceLeaseTransition::NoChange);
    assert_eq!(state.refresh_presence(&key, STUDIO, "presence-restart", "actor-a", "live-a", b"opaque".to_vec(), now).await, PresenceLeaseTransition::Published);
    assert_eq!(state.directory.head_seq().await.expect("directory head"), before, "presence never appends a durable directory event");
    let member_token = seed_author_token(&state).await;
    let member = resolve_bearer_user(&state, Some(&member_token)).await.expect("member caller");
    let outsider_session = issue_test_session(&state, "presence-outsider@example.com").await;
    let outsider = resolve_bearer_user(&state, Some(&outsider_session.token)).await.expect("outsider caller");
    let message = DirectoryStreamMessage::Presence { space_id: STUDIO.into(), document_id: "presence-restart".into(), actors: state.presence_snapshot(&key).actors };
    assert!(directory_message_visible(&state, &message, Some(&member)).await);
    assert!(!directory_message_visible(&state, &message, Some(&outsider)).await);
    assert!(!directory_message_visible(&state, &message, None).await);
    let restarted = test_state().await;
    assert_eq!(restarted.presence.len(), 0, "a fresh hub has no server-local lease slots");
    assert!(restarted.presence_snapshot(&key).peers.is_empty());
}

async fn append_directory_page_test_events(state: &HubState, rows: &[(String, String)]) -> Vec<DirectoryEvent> {
    let events = rows
        .iter()
        .map(|(space_id, name)| semio_hub::directory::NewDirectoryEvent {
            hlc: os_directory::Hlc { physical_ms: now_ms(), logical: 0 },
            actor: DirectoryActor { kind: DirectoryActorKind::System, id: format!("system:event-page:{space_id}") },
            space_id: Some(space_id.clone()),
            user_id: None,
            body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: space_id.clone(), name: name.clone() },
        })
        .collect::<Vec<_>>();
    state.directory.append_events(&events).await.expect("append event-page fixture")
}

fn event_page_authorization(token: &str) -> String {
    format!("Bearer {token}")
}

fn directory_command_body(request_id: &str, command: DirectoryCommand) -> Vec<u8> {
    DirectoryCommandRequestV1::new(request_id, command).canonical_json().into_bytes()
}

async fn post_directory_command_for_test(addr: SocketAddr, token: &str, request_id: &str, command: DirectoryCommand) -> RawHttpResponse {
    let authorization = format!("Bearer {token}");
    raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], &directory_command_body(request_id, command)).await
}

fn parse_directory_command_receipt_for_test(response: &RawHttpResponse, request_id: &str, command: &DirectoryCommand) -> DirectoryCommandReceiptV1 {
    let canonical = std::str::from_utf8(&response.body).expect("receipt UTF-8");
    DirectoryCommandReceiptV1::parse_canonical_json(canonical, &DirectoryCommandRequestV1::new(request_id, command.clone())).expect("canonical receipt")
}

#[tokio::test]
async fn directory_command_authority_revalidates_after_durable_revocation_before_fence() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["appended"] == 0) {
        let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("authority state open deadline");
        let owner = issue_test_session(&state, "authority-owner@example.com").await;
        let author = issue_test_session(&state, "authority-author@example.com").await;
        let target = issue_test_session(&state, "authority-target@example.com").await;
        let space = create_space_for_test(&state, &owner.user_id, "Authority Race", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
        upsert_member_for_test(&state, &space, "authority-target@example.com", DirectorySpaceRole::Spectator).await;
        let gate = Arc::new(TestLiveGate::default());
        *gate.directory_command_pause_user.lock().unwrap() = Some((author.user_id.clone(), false));
        state.live_gate = Some(gate.clone());
        let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
        let command = DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: target.user_id.clone() };
        let request_id = "a00102030405060708090a0b0c0d0e0f";
        let pending = {
            let token = author.token.clone();
            let command = command.clone();
            tokio::spawn(async move { post_directory_command_for_test(addr, &token, request_id, command).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
        match row["revocation"].as_str().unwrap() {
            "demote" | "remove" => {
                let revocation = if row["revocation"] == "demote" {
                    DirectoryCommand::UpsertMember { space_id: space.clone(), email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator }
                } else {
                    DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: author.user_id.clone() }
                };
                assert_eq!(post_directory_command_for_test(addr, &owner.token, "b00102030405060708090a0b0c0d0e0f", revocation).await.status, 202);
            }
            "session" => assert_eq!(delete_session_me(bearer_headers(&author.token), State(state.clone())).await, StatusCode::NO_CONTENT),
            _ => unreachable!(),
        }
        let head = state.directory.head_seq().await.unwrap();
        gate.directory_command_release.add_permits(1);
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["id"]);
        assert!(response.body.is_empty());
        assert_eq!(state.directory.head_seq().await.unwrap(), head);
        assert_eq!(state.directory.get_role(&space, &target.user_id).await.unwrap(), Some(SpaceRole::Spectator));
        let claim = NewDirectoryCommandReceipt { actor_user_id: author.user_id.clone(), request_id: request_id.into(), command_sha256: directory_command_sha256(&command), result_kind: directory_command_result_kind(&command), claimed_at: now_ms() };
        assert!(matches!(state.directory.claim_or_read_directory_command_receipt(&claim).await.unwrap(), DirectoryCommandClaimV1::Claimed(_)), "denied command must leave no durable claim");
        state.directory.release_directory_command_receipt(&author.user_id, request_id, &claim.command_sha256).await.unwrap();
        stop_recovery_server(state, shutdown, server).await;
        eprintln!("[DEBUG] directory authority case={} status={} appended=0 receipt=0 target-retained=1", row["id"], response.status);
    }
}

#[tokio::test]
async fn directory_command_authority_holds_admitted_command_until_receipt_before_demotion() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["appended"] == 1).unwrap();
    let mut state = test_state().await;
    let owner = issue_test_session(&state, "authority-owner@example.com").await;
    let author = issue_test_session(&state, "authority-author@example.com").await;
    let target = issue_test_session(&state, "authority-target@example.com").await;
    let space = create_space_for_test(&state, &owner.user_id, "Authority Order", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space, "authority-target@example.com", DirectorySpaceRole::Spectator).await;
    let gate = Arc::new(TestLiveGate::default());
    *gate.directory_command_pause_user.lock().unwrap() = Some((author.user_id.clone(), true));
    state.live_gate = Some(gate.clone());
    let head = state.directory.head_seq().await.unwrap();
    let addr = spawn_server(state.clone()).await;
    let command = DirectoryCommand::RemoveMember { space_id: space.clone(), user_id: target.user_id.clone() };
    let pending = {
        let token = author.token.clone();
        let command = command.clone();
        tokio::spawn(async move { post_directory_command_for_test(addr, &token, "c00102030405060708090a0b0c0d0e0f", command).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
    gate.directory_command_attempted.acquire().await.unwrap().forget();
    let demotion = DirectoryCommand::UpsertMember { space_id: space.clone(), email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator };
    let revoking = {
        let token = owner.token.clone();
        tokio::spawn(async move { post_directory_command_for_test(addr, &token, "d00102030405060708090a0b0c0d0e0f", demotion).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_attempted.acquire()).await.unwrap().unwrap().forget();
    assert!(!revoking.is_finished());
    assert_eq!(state.directory.head_seq().await.unwrap(), head);
    gate.directory_command_release.add_permits(1);
    let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
    assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap());
    let receipt = parse_directory_command_receipt_for_test(&response, "c00102030405060708090a0b0c0d0e0f", &command);
    assert_eq!(receipt.outcome, DirectoryCommandOutcomeV1::Accepted);
    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(5), revoking).await.unwrap().unwrap().status, 202);
    let events = state.directory.events_since(head, 8).await.unwrap();
    assert_eq!(events.len(), 2);
    assert!(matches!(&events[0].body, os_directory::DirectoryEventBody::MemberRemoved { user_id, .. } if user_id == &target.user_id));
    assert_eq!(state.directory.get_role(&space, &target.user_id).await.unwrap(), None);
    assert_eq!(state.directory.get_role(&space, &author.user_id).await.unwrap(), Some(SpaceRole::Spectator));
    eprintln!("[DEBUG] directory authority case={} status=202 appended=1 receipt=accepted revocation-ordered=1", row["id"]);
}

#[tokio::test]
async fn directory_command_authority_invite_revocation_requires_the_exact_owned_space() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).expect("authority fixture");
    let state = test_state().await;
    let caller = issue_test_session(&state, "invite-scope-owner@example.test").await;
    let other = issue_test_session(&state, "invite-scope-other@example.test").await;
    let owned = create_space_for_test(&state, &caller.user_id, "Owned space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let foreign = create_space_for_test(&state, &other.user_id, "Foreign space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    for (index, row) in fixture["inviteScopes"].as_array().expect("invite scope rows").iter().enumerate() {
        let issued = state.directory.issue_invite(&foreign, SpaceRole::Spectator, 600, "invite-scope-law").await.expect("foreign invite");
        let invite = issued.record.clone();
        if row["accepted"].as_bool().expect("accepted fixture") {
            let recipient = issue_test_session(&state, &format!("invite-recipient-{index}@example.test")).await;
            state.directory_service.redeem_invite(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#invite-scope", recipient.user_id) }, &issued.capability, &recipient.user_id).await.expect("accepted invite fixture");
        }
        let exact = row["commandSpace"] == row["inviteSpace"];
        let (space, token) = if exact { (&foreign, &other.token) } else { (&owned, &caller.token) };
        let head = state.directory.head_seq().await.expect("before revoke head");
        let request_id = format!("{:032x}", 1000 + index);
        let response = post_directory_command_for_test(addr, token, &request_id, DirectoryCommand::RevokeInvite { space_id: space.clone(), invite_id: invite.id.clone() }).await;
        assert_eq!(u64::from(response.status), row["status"].as_u64().expect("expected status"), "{}", row["id"]);
        let current = state.directory.list_invites(&foreign).await.expect("foreign invite after request").into_iter().find(|candidate| candidate.id == invite.id).expect("retained foreign invite");
        assert_eq!(current.revoked_at.is_some(), row["revoked"].as_bool().expect("expected revocation"));
        assert_eq!(state.directory.head_seq().await.expect("after revoke head"), head, "invite revocation changes capability authority, not membership events");
        if !exact {
            assert!(response.body.is_empty(), "cross-space response carries no foreign metadata");
        }
        println!("[DEBUG] directory invite authority case={} status={} revoked={}", row["id"], response.status, current.revoked_at.is_some());
    }
}

/// 🎟️ Uses real space commands and HTTP redemption to preserve archive reads without restoring authorship.
#[tokio::test]
async fn directory_invite_redemption_obeys_current_space_state_and_readonly_replay() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).expect("invite state fixture");
    let state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("invite state open deadline");
    let owner = issue_test_session(&state, "invite-state-owner@example.test").await;
    let addr = spawn_server(state.clone()).await;
    for (index, row) in fixture["spaceStates"].as_array().expect("state rows").iter().enumerate() {
        let caller = issue_test_session(&state, &format!("invite-state-{index}@example.test")).await;
        let other = issue_test_session(&state, &format!("invite-state-other-{index}@example.test")).await;
        let space = create_space_for_test(&state, &owner.user_id, "Invite state", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let role = if row["inviteRole"] == "author" { SpaceRole::Author } else { SpaceRole::Spectator };
        let issued = state.directory.issue_invite(&space, role, 600, "invite-state-law").await.expect("pending invite");
        let accepted_user = if row["accepted"] == "other" { &other } else { &caller };
        let accepted = if row["accepted"] != "none" {
            state
                .directory_service
                .redeem_invite(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#invite-state", accepted_user.user_id) }, &issued.capability, &accepted_user.user_id)
                .await
                .map(|commit| vec![commit.into_event()])
                .expect("initial acceptance")
        } else {
            Vec::new()
        };
        let transition = match row["spaceState"].as_str().unwrap() {
            "archived" => Some(DirectoryCommand::ArchiveSpace { space_id: space.clone() }),
            "deleted" => Some(DirectoryCommand::DeleteSpace { space_id: space.clone() }),
            "writable" => None,
            _ => unreachable!(),
        };
        if let Some(command) = transition {
            assert_eq!(post_directory_command_for_test(addr, &owner.token, &format!("{:032x}", 2000 + index), command).await.status, 202);
        }
        let replay_grant = if row["accepted"] == "same" && row["spaceState"] != "deleted" {
            let session = resolve_bearer_user(&state, Some(&caller.token)).await.unwrap();
            let capability = SocketGrantCapability::mint().unwrap();
            let audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new(&space, "invite-replay"));
            let subject = SocketSubjectV1::Session { session_id: session.session_id, user_id: caller.user_id.clone(), authorization_generation: session.authorization_generation, role: Some(SpaceRole::Spectator), expires_at_ms: session.expires_at };
            state.socket_grants.issue(&capability, audience.clone(), "hub.v1.invite-replay".into(), subject, now_ms(), now_ms() + 30_000).unwrap();
            Some((capability, audience))
        } else {
            None
        };
        let before = state.directory.head_seq().await.unwrap();
        let mut stream = state.directory_service.subscribe();
        let token = issued.capability.expose_once();
        let authorization = format!("Bearer {}", caller.token);
        let response = raw_http_request(addr, "POST", &format!("/directory/invites/{token}/redeem"), &[("Authorization", &authorization)], &[]).await;
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["name"]);
        if let Some((capability, audience)) = replay_grant {
            assert!(state.socket_grants.pending(&capability, &audience, now_ms()).is_ok(), "read-only replay preserves fresh admission");
        }
        let appended = state.directory.events_since(before, 16).await.unwrap();
        assert_eq!(appended.len() as u64, row["appended"].as_u64().unwrap(), "{}", row["name"]);
        assert!(appended.iter().all(|event| matches!(event.body, os_directory::DirectoryEventBody::InviteRedeemed { .. })));
        let current = state.directory.get_role(&space, &caller.user_id).await.unwrap();
        let expected_role = match row["membershipRole"].as_str().unwrap() {
            "author" => Some(SpaceRole::Author),
            "spectator" => Some(SpaceRole::Spectator),
            "none" => None,
            _ => unreachable!(),
        };
        assert_eq!(current, expected_role, "{}", row["name"]);
        let invitations = state.directory.list_invites(&space).await.unwrap();
        if row["spaceState"] == "deleted" {
            assert!(invitations.is_empty());
        } else {
            let retained = invitations.iter().find(|invite| invite.id == issued.record.id).expect("retained invitation");
            assert_eq!(retained.accepted_at.is_some(), !accepted.is_empty() || !appended.is_empty());
            if !accepted.is_empty() {
                assert_eq!(retained.accepted_event_id.as_deref(), Some(accepted[0].id.as_str()));
            }
        }
        if row["appended"] == 0 {
            assert!(matches!(stream.try_recv(), Err(broadcast::error::TryRecvError::Empty)), "replay and denial publish no new event");
        } else {
            assert!(stream.try_recv().is_ok());
        }
        if response.status == 200 && !accepted.is_empty() {
            let expected = axum::body::to_bytes(DirectoryJson(accepted).into_response().into_body(), 1 << 20).await.unwrap();
            assert_eq!(response.body.as_slice(), expected.as_ref(), "replay returns the exact original immutable event");
        } else if response.status != 200 {
            assert!(response.body.is_empty());
        }
        eprintln!("[DEBUG] invite space state case={} status={} appended={} role={:?}", row["name"], response.status, appended.len(), current);
    }
}

/// 🧭️ Scope selection accepts only the exact stored secret and existing authenticated subject.
#[tokio::test]
async fn directory_invite_redemption_scope_hint_is_capability_bound() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("invite hint state deadline");
    let owner = issue_test_session(&state, "invite-hint-owner@example.test").await;
    let caller = issue_test_session(&state, "invite-hint-caller@example.test").await;
    let space = create_space_for_test(&state, &owner.user_id, "Invite hint", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-hint").await.unwrap();
    let other = InviteCapability::mint().unwrap();
    let other_encoded = other.expose_once();
    let wrong_secret = InviteCapability::parse(&other_encoded.replacen(other.selector(), issued.capability.selector(), 1)).unwrap();
    let head = state.directory.head_seq().await.unwrap();
    for row in fixture["vectors"].as_array().unwrap().iter().filter(|row| ["fresh-single", "wrong-selector", "wrong-secret", "actor-mismatch", "missing-user"].contains(&row["name"].as_str().unwrap())) {
        let user_id = if row["name"] == "missing-user" { "nonexistent-user" } else { &caller.user_id };
        let actor_user = if row["name"] == "actor-mismatch" { &owner.user_id } else { user_id };
        let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{actor_user}#hint") };
        let capability = match row["name"].as_str().unwrap() {
            "wrong-selector" => &other,
            "wrong-secret" => &wrong_secret,
            _ => &issued.capability,
        };
        let result = state.directory.invite_redemption_scope_hint(capability, &actor, user_id).await;
        if row["expected"]["outcomes"][0] == "newly-committed" {
            assert_eq!(result.unwrap().space_id(), space);
        } else {
            assert!(matches!(result, Err(DirectoryError::Unauthorized)), "{}", row["name"]);
        }
        assert_eq!(state.directory.head_seq().await.unwrap(), head, "a hint writes no event");
        assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), None, "a hint grants no membership");
        assert!(state.directory.list_invites(&space).await.unwrap().iter().all(|invite| invite.accepted_at.is_none()));
        eprintln!("[DEBUG] invite scope hint case={} capability-bound=1 mutation=0", row["name"]);
    }
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hint", caller.user_id) };
    state.directory_service.execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#hint-owner", owner.user_id) }, DirectoryCommand::DeleteSpace { space_id: space }).await.unwrap();
    assert!(matches!(state.directory.invite_redemption_scope_hint(&issued.capability, &actor, &caller.user_id).await, Err(DirectoryError::Unauthorized)));
}

/// 🔒️ A paused redemption cannot spend the identity captured before a durable revocation.
#[tokio::test]
async fn directory_invite_redemption_revalidates_after_hint_before_fence() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("invite race state deadline");
    let owner = issue_test_session(&state, "invite-race-owner@example.test").await;
    let gate = Arc::new(TestLiveGate::default());
    state.live_gate = Some(gate.clone());
    let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
    for (index, row) in fixture["authorityRaces"].as_array().unwrap().iter().filter(|row| row["appended"] == 0).enumerate() {
        let caller = issue_test_session(&state, &format!("invite-race-{index}@example.test")).await;
        let space = create_space_for_test(&state, &owner.user_id, "Invite race", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
        let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-race").await.unwrap();
        *gate.directory_command_pause_user.lock().unwrap() = Some((caller.user_id.clone(), false));
        let pending = {
            let token = caller.token.clone();
            let path = format!("/directory/invites/{}/redeem", issued.capability.expose_once());
            tokio::spawn(async move { raw_http_request(addr, "POST", &path, &[("Authorization", &format!("Bearer {token}"))], &[]).await })
        };
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
        if row["revocation"] == "session" {
            assert_eq!(delete_session_me(bearer_headers(&caller.token), State(state.clone())).await, StatusCode::NO_CONTENT);
        } else {
            let command = if row["revocation"] == "archive" { DirectoryCommand::ArchiveSpace { space_id: space.clone() } } else { DirectoryCommand::DeleteSpace { space_id: space.clone() } };
            assert_eq!(post_directory_command_for_test(addr, &owner.token, &format!("{:032x}", 3000 + index), command).await.status, 202);
        }
        let head = state.directory.head_seq().await.unwrap();
        gate.directory_command_release.add_permits(1);
        let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
        assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap(), "{}", row["name"]);
        assert!(response.body.is_empty());
        assert_eq!(state.directory.head_seq().await.unwrap(), head);
        assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), None);
        assert!(state.directory.list_invites(&space).await.unwrap().iter().all(|invite| invite.accepted_at.is_none()));
        eprintln!("[DEBUG] invite authority race={} status={} appended=0", row["name"], response.status);
    }
    shutdown.send(()).unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), server).await.unwrap().unwrap();
}

/// 🧱️ All session and space keys stay owned through one irreversible invitation event.
#[tokio::test]
async fn directory_invite_redemption_admitted_fence_precedes_archive() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🎟️invite-redemption-transaction-v1/🔣️.json")).unwrap();
    let row = fixture["authorityRaces"].as_array().unwrap().iter().find(|row| row["appended"] == 1).unwrap();
    let mut state = tokio::time::timeout(std::time::Duration::from_secs(5), test_state()).await.expect("invite order state deadline");
    let owner = issue_test_session(&state, "invite-order-owner@example.test").await;
    let caller = issue_test_session(&state, "invite-order-caller@example.test").await;
    let session = resolve_bearer_user(&state, Some(&caller.token)).await.unwrap();
    let space = create_space_for_test(&state, &owner.user_id, "Invite order", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let issued = state.directory.issue_invite(&space, SpaceRole::Author, 600, "invite-order").await.unwrap();
    let gate = Arc::new(TestLiveGate::default());
    *gate.directory_command_pause_user.lock().unwrap() = Some((caller.user_id.clone(), true));
    state.live_gate = Some(gate.clone());
    let (addr, shutdown, server) = spawn_restartable_server(state.clone()).await;
    let head = state.directory.head_seq().await.unwrap();
    let pending = {
        let token = caller.token.clone();
        let path = format!("/directory/invites/{}/redeem", issued.capability.expose_once());
        tokio::spawn(async move { raw_http_request(addr, "POST", &path, &[("Authorization", &format!("Bearer {token}"))], &[]).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_admitted.acquire()).await.unwrap().unwrap().forget();
    for key in [
        SocketBindingKeyV1::User(caller.user_id.clone()),
        SocketBindingKeyV1::Session(session.session_id.clone()),
        SocketBindingKeyV1::DirectorySpaceAuthority { space_id: space.clone() },
        SocketBindingKeyV1::Membership { space_id: space.clone(), user_id: caller.user_id.clone() },
    ] {
        assert!(state.socket_binding_gates.gate(key).try_lock_owned().is_err(), "the admitted redemption must own every exact authority key");
    }
    gate.directory_command_attempted.acquire().await.unwrap().forget();
    let revoking = {
        let token = owner.token.clone();
        let command = DirectoryCommand::ArchiveSpace { space_id: space.clone() };
        tokio::spawn(async move { post_directory_command_for_test(addr, &token, "f10102030405060708090a0b0c0d0e0f", command).await })
    };
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.directory_command_attempted.acquire()).await.unwrap().unwrap().forget();
    assert!(!revoking.is_finished());
    assert_eq!(state.directory.head_seq().await.unwrap(), head);
    gate.directory_command_release.add_permits(1);
    let response = tokio::time::timeout(std::time::Duration::from_secs(5), pending).await.unwrap().unwrap();
    assert_eq!(u64::from(response.status), row["status"].as_u64().unwrap());
    assert_eq!(tokio::time::timeout(std::time::Duration::from_secs(5), revoking).await.unwrap().unwrap().status, 202);
    let events = state.directory.events_since(head, 16).await.unwrap();
    assert!(matches!(&events[0].body, os_directory::DirectoryEventBody::InviteRedeemed { user_id, role: DirectorySpaceRole::Author, .. } if user_id == &caller.user_id));
    assert!(matches!(&events.last().unwrap().body, os_directory::DirectoryEventBody::SpaceArchived { .. }));
    assert_eq!(events.iter().filter(|event| matches!(event.body, os_directory::DirectoryEventBody::InviteRedeemed { .. })).count(), row["appended"].as_u64().unwrap() as usize);
    assert_eq!(state.directory.get_role(&space, &caller.user_id).await.unwrap(), Some(SpaceRole::Spectator));
    shutdown.send(()).unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(5), server).await.unwrap().unwrap();
    eprintln!("[DEBUG] invite admitted-before-archive event-first=1 final-role=spectator");
}

#[tokio::test]
async fn directory_command_authority_demotion_invalidates_only_affected_scope_once() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../📇️directory/🧫️fixtures/🛡️command-authority-v1/🔣️.json")).unwrap();
    let shared = SocketGrantLedgerV1::default();
    let capacity = &fixture["capacity"];
    assert_eq!(capacity["subjectLimit"].as_u64().unwrap() as usize, SOCKET_GRANT_BINDING_PENDING_CAPACITY);
    assert_eq!(capacity["ledgerLimit"].as_u64().unwrap() as usize, SOCKET_GRANT_LEDGER_CAPACITY);
    for index in 0..capacity["subjects"].as_u64().unwrap() {
        let subject = SocketSubjectV1::Session { session_id: format!("capacity-session-{index}"), user_id: format!("capacity-user-{index}"), authorization_generation: 1, role: Some(SpaceRole::Author), expires_at_ms: 10_000 };
        shared.issue(&SocketGrantCapability::mint().unwrap(), SocketAudienceV1::DirectoryScoped(DocumentScope::new("shared-space", "document")), format!("hub.v1.capacity-{index}"), subject, 1, 9_000).unwrap();
    }
    assert_eq!(shared.inner.lock().unwrap().records.len() as u64, capacity["accepted"].as_u64().unwrap());
    eprintln!("[DEBUG] directory authority shared-space pending-subjects=65 subject-limit=64 global-limit=4096");
    let state = test_state().await;
    let owner = issue_test_session(&state, "authority-owner@example.com").await;
    let author = issue_test_session(&state, "authority-author@example.com").await;
    let space = create_space_for_test(&state, &owner.user_id, "Authority Bindings", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let other = create_space_for_test(&state, &author.user_id, "Other Bindings", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space, "authority-author@example.com", DirectorySpaceRole::Author).await;
    let mut records = Vec::new();
    for row in fixture["bindings"].as_array().unwrap() {
        let user = if row["user"] == "author" { &author } else { &owner };
        let session = state.directory.authenticate_session(&SessionCapability::parse(&user.token).unwrap()).await.unwrap().unwrap();
        let scope = DocumentScope::new(if row["space"] == "changed" { &space } else { &other }, "authority-document");
        let subject = SocketSubjectV1::Session { session_id: session.id.clone(), user_id: user.user_id.clone(), authorization_generation: session.authorization_generation, role: Some(SpaceRole::Author), expires_at_ms: session.expires_at };
        let audience = match row["audience"].as_str().unwrap() {
            "document" => SocketAudienceV1::Document(scope),
            "scoped" => SocketAudienceV1::DirectoryScoped(scope),
            "global" => SocketAudienceV1::Directory { auth_session_id: session.id, authorization_generation: session.authorization_generation },
            _ => unreachable!(),
        };
        let pending = SocketGrantCapability::mint().unwrap();
        let live = SocketGrantCapability::mint().unwrap();
        for capability in [&pending, &live] {
            state.socket_grants.issue(capability, audience.clone(), format!("hub.v1.{}", row["id"].as_str().unwrap()), subject.clone(), now_ms(), now_ms() + 30_000).unwrap();
        }
        let record = state.socket_grants.pending(&live, &audience, now_ms()).unwrap();
        let record = state.socket_grants.consume(&record, now_ms()).unwrap();
        let (live_id, notify) = state.socket_grants.register_live(&record).unwrap();
        records.push((row, pending, record, live_id, notify));
    }
    let addr = spawn_server(state.clone()).await;
    let command = DirectoryCommand::UpsertMember { space_id: space, email: "authority-author@example.com".into(), role: DirectorySpaceRole::Spectator };
    let request_id = "e00102030405060708090a0b0c0d0e0f";
    assert_eq!(post_directory_command_for_test(addr, &owner.token, request_id, command.clone()).await.status, 202);
    for (row, pending, record, live_id, notify) in &records {
        let invalidated = row["invalidated"].as_bool().unwrap();
        assert_eq!(state.socket_grants.pending(pending, &record.audience, now_ms()).is_err(), invalidated, "pending {}", row["id"]);
        assert_eq!(!state.socket_grants.is_live(record, live_id), invalidated, "live {}", row["id"]);
        if invalidated {
            tokio::time::timeout(std::time::Duration::from_secs(1), notify.notified()).await.unwrap();
            let fresh = SocketGrantCapability::mint().unwrap();
            let mut subject = record.subject.clone();
            if let SocketSubjectV1::Session { role, .. } = &mut subject {
                *role = Some(SpaceRole::Spectator);
            }
            state.socket_grants.issue(&fresh, record.audience.clone(), record.actor_id.clone(), subject, now_ms(), now_ms() + 30_000).unwrap();
            assert_eq!(post_directory_command_for_test(addr, &owner.token, request_id, command.clone()).await.status, 202);
            assert!(state.socket_grants.pending(&fresh, &record.audience, now_ms()).is_ok(), "receipt replay must not invalidate fresh admission");
        }
        eprintln!("[DEBUG] directory authority binding={} invalidated={} replay-preserved=1", row["id"], invalidated);
    }
}

#[tokio::test]
async fn directory_command_receipt_v1_route_is_request_idempotent_for_concurrent_identical_ids() {
    let state = test_state().await;
    let author = issue_test_session(&state, "command-receipt-author@example.com").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Receipt Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    let mut live = state.directory_service.subscribe();
    let command = DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
    let request_id = "1f2e3d4c5b6a7988a1b2c3d4e5f60718";
    let (first, second) = tokio::join!(post_directory_command_for_test(addr, &author.token, request_id, command.clone()), post_directory_command_for_test(addr, &author.token, request_id, command.clone()),);
    assert_eq!((first.status, second.status), (202, 202), "an idempotent duplicate is answered, never failed");
    let receipts = [parse_directory_command_receipt_for_test(&first, request_id, &command), parse_directory_command_receipt_for_test(&second, request_id, &command)];
    let tokens: Vec<String> = receipts
        .iter()
        .filter_map(|receipt| match &receipt.result {
            os_directory::DirectoryCommandResultV1::Invite { invite_token } => Some(invite_token.clone()),
            os_directory::DirectoryCommandResultV1::None => None,
        })
        .collect();
    assert_eq!(tokens.len(), 1, "exactly one response carries the one-shot capability");
    assert_eq!(receipts.iter().filter(|receipt| receipt.outcome == DirectoryCommandOutcomeV1::Accepted).count(), 1);
    assert!(receipts.iter().any(|receipt| receipt.outcome == DirectoryCommandOutcomeV1::SecretUndeliverable && receipt.result == os_directory::DirectoryCommandResultV1::None));
    let token = tokens.into_iter().next().expect("issued capability");

    let invites = state.directory.list_invites(&space_id).await.expect("invite rows");
    assert_eq!(invites.len(), 1, "two concurrent identical request ids mint exactly one invitation");
    assert!(!format!("{invites:?}").contains(&token), "no invite row retains the capability plaintext");

    let retry = post_directory_command_for_test(addr, &author.token, request_id, command.clone()).await;
    let replayed = parse_directory_command_receipt_for_test(&retry, request_id, &command);
    assert_eq!(replayed.outcome, DirectoryCommandOutcomeV1::SecretUndeliverable);
    assert!(!std::str::from_utf8(&retry.body).expect("retry UTF-8").contains(&token), "a later resolution of the same id is redacted");
    assert_eq!(state.directory.list_invites(&space_id).await.expect("invite rows after retry").len(), 1);

    let events = state.directory.events_since(0, 1_000).await.expect("durable log");
    assert!(!format!("{events:?}").contains(&token), "the capability never enters the durable event log");
    let mut broadcast = Vec::new();
    while let Ok(message) = live.try_recv() {
        broadcast.push(format!("{message:?}"));
    }
    assert!(!broadcast.join("\n").contains(&token), "the capability never enters the live broadcast");
}

#[tokio::test]
async fn directory_command_receipt_v1_route_denies_cross_user_spectator_and_digest_substitution() {
    let state = test_state().await;
    let author = issue_test_session(&state, "command-receipt-owner@example.com").await;
    let spectator = issue_test_session(&state, "command-receipt-spectator@example.com").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Denial Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, "command-receipt-spectator@example.com", DirectorySpaceRole::Spectator).await;
    let addr = spawn_server(state.clone()).await;
    let invite = DirectoryCommand::CreateInvite { space_id: space_id.clone(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
    let request_id = "0a1b2c3d4e5f60718293a4b5c6d7e8f9";

    let accepted = post_directory_command_for_test(addr, &author.token, request_id, invite.clone()).await;
    assert_eq!(accepted.status, 202);
    let token = match parse_directory_command_receipt_for_test(&accepted, request_id, &invite).result {
        DirectoryCommandResultV1::Invite { invite_token } => invite_token,
        DirectoryCommandResultV1::None => panic!("the first accepted create-invite carries its capability"),
    };

    let denied = post_directory_command_for_test(addr, &spectator.token, request_id, invite.clone()).await;
    assert_eq!(denied.status, 403, "a spectator is denied before any stored completion is consulted");
    assert!(denied.body.is_empty(), "a denial carries no body, so no receipt or capability leaks");

    let cross_user =
        post_directory_command_for_test(addr, &spectator.token, request_id, DirectoryCommand::CreateSpace { name: "Spectator Space".into(), space_kind: os_directory::DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private }).await;
    assert_eq!(cross_user.status, 202, "the idempotency key is scoped to the authenticated user");
    assert!(!std::str::from_utf8(&cross_user.body).expect("cross-user UTF-8").contains(&token), "another user's equal request id never discovers a stored capability");

    let substituted = post_directory_command_for_test(addr, &author.token, request_id, DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "Substituted".into() }).await;
    assert_eq!(substituted.status, 409, "an equal key with an unequal command digest is a generic conflict");
    assert!(substituted.body.is_empty());
    assert_eq!(state.directory.list_invites(&space_id).await.expect("invite rows").len(), 1, "no denial or conflict executed a second time");

    let capability = SessionCapability::parse(&author.token).expect("author capability");
    let record = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
    state.directory.revoke_auth_session(&record.id, "command-receipt-test", None, "command-receipt-test").await.expect("revoke session").expect("revoked row");
    let revoked = post_directory_command_for_test(addr, &author.token, request_id, invite.clone()).await;
    assert_eq!(revoked.status, 401, "authentication re-runs before any stored completion is returned");
    assert!(revoked.body.is_empty());
}

#[tokio::test]
async fn directory_command_receipt_v1_route_bounds_request_and_receipt_bytes() {
    let state = test_state().await;
    let author = issue_test_session(&state, "command-receipt-bounds@example.com").await;
    let space_id = create_space_for_test(&state, &author.user_id, "Bounds Space", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let addr = spawn_server(state.clone()).await;
    let authorization = format!("Bearer {}", author.token);

    let fitting = DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "n".repeat(64) };
    let request = DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a0", fitting.clone());
    assert!(request.canonical_json().len() <= DIRECTORY_COMMAND_REQUEST_MAX_BYTES);
    let accepted = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], request.canonical_json().as_bytes()).await;
    assert_eq!(accepted.status, 202);
    assert!(accepted.body.len() <= os_directory::DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, "an admitted request can never produce an over-ceiling receipt");

    let padding = DIRECTORY_COMMAND_REQUEST_MAX_BYTES + 1 - DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a1", DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: String::new() }).canonical_json().len();
    let oversize = DirectoryCommandRequestV1::new("9f8e7d6c5b4a39281706f5e4d3c2b1a1", DirectoryCommand::RenameSpace { space_id: space_id.clone(), name: "x".repeat(padding) });
    assert_eq!(oversize.canonical_json().len(), DIRECTORY_COMMAND_REQUEST_MAX_BYTES + 1);
    let rejected = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], oversize.canonical_json().as_bytes()).await;
    assert!(matches!(rejected.status, 400 | 413), "one byte past the request ceiling is refused, got {}", rejected.status);

    for hostile in [
        "{\"requestId\":\"9f8e7d6c5b4a39281706f5e4d3c2b1a2\",\"schema\":\"semio.directory.command-request.v1\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
        "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"9F8E7D6C5B4A39281706F5E4D3C2B1A2\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
        "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"00000000000000000000000000000000\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"}}",
        "{\"schema\":\"semio.directory.command-request.v1\",\"requestId\":\"9f8e7d6c5b4a39281706f5e4d3c2b1a2\",\"command\":{\"kind\":\"archive-space\",\"spaceId\":\"x\"},\"extra\":1}",
        "{\"kind\":\"archive-space\",\"spaceId\":\"x\"}",
    ] {
        let response = raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], hostile.as_bytes()).await;
        assert_eq!(response.status, 400, "a noncanonical, mis-cased, zero-id, unknown-field, or bare-command body is refused: {hostile}");
        assert!(response.body.is_empty());
    }
}

#[tokio::test]
async fn directory_command_receipt_v1_store_resolves_a_lost_reply_and_survives_restart() {
    let path = tempdir("command-receipt-store");
    std::fs::create_dir_all(&path).expect("receipt store dir");
    let database = path.join("directory.sqlite");
    let database_path = database.to_str().expect("receipt store path").to_string();
    let claim = |request_id: &str, digest: &str, kind: DirectoryCommandResultKindV1, actor: &str| NewDirectoryCommandReceipt {
        actor_user_id: actor.to_string(),
        request_id: request_id.to_string(),
        command_sha256: digest.to_string(),
        result_kind: kind,
        claimed_at: 1_700_000_000_000,
    };
    let invite_digest = directory_command_sha256(&DirectoryCommand::CreateInvite { space_id: "space".into(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 });
    let rename_digest = directory_command_sha256(&DirectoryCommand::RenameSpace { space_id: "space".into(), name: "Renamed".into() });
    let request_id = "1f2e3d4c5b6a7988a1b2c3d4e5f60718";

    let directory = SqliteDirectory::connect(&database_path).await.expect("connect receipt store");
    directory.seed().await.expect("seed receipt store");
    let pending_claim = claim(request_id, &invite_digest, DirectoryCommandResultKindV1::Invite, "user-a");
    assert!(matches!(directory.claim_or_read_directory_command_receipt(&pending_claim).await.expect("first claim"), DirectoryCommandClaimV1::Claimed(_)));
    let lost = match directory.claim_or_read_directory_command_receipt(&pending_claim).await.expect("lost-reply resolution") {
        DirectoryCommandClaimV1::Existing(record) => record,
        other => panic!("a claimed key never re-executes: {other:?}"),
    };
    assert_eq!(lost.disposition, DirectoryCommandDispositionV1::Pending);
    assert_eq!(replay_directory_command_receipt(&lost).outcome, DirectoryCommandOutcomeV1::SecretUndeliverable, "a reply lost between durable command and response is honestly undeliverable");
    assert!(matches!(directory.claim_or_read_directory_command_receipt(&claim(request_id, &rename_digest, DirectoryCommandResultKindV1::None, "user-a")).await.expect("digest substitution"), DirectoryCommandClaimV1::Conflict));
    assert!(
        matches!(directory.claim_or_read_directory_command_receipt(&claim(request_id, &invite_digest, DirectoryCommandResultKindV1::Invite, "user-b")).await.expect("cross-user claim"), DirectoryCommandClaimV1::Claimed(_)),
        "the key is scoped to the authenticated user"
    );

    let replay_digest = replay_directory_command_receipt(&DirectoryCommandReceiptRecord { disposition: DirectoryCommandDispositionV1::Completed, ..lost.clone() }).receipt_sha256;
    let completed = directory
        .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
            actor_user_id: "user-a".into(),
            request_id: request_id.into(),
            event_seq_first: None,
            event_seq_last: None,
            receipt_sha256: replay_digest.clone(),
            completed_at: 1_700_000_000_001,
        })
        .await
        .expect("durable completion");
    assert_eq!(completed.disposition, DirectoryCommandDispositionV1::Completed);
    assert!(
        directory
            .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
                actor_user_id: "user-a".into(),
                request_id: request_id.into(),
                event_seq_first: None,
                event_seq_last: None,
                receipt_sha256: replay_digest.clone(),
                completed_at: 1_700_000_000_002
            })
            .await
            .is_err(),
        "one claim completes exactly once"
    );
    drop(directory);

    let restarted = SqliteDirectory::connect(&database_path).await.expect("reconnect receipt store");
    restarted.seed().await.expect("reseed receipt store");
    let resolved = match restarted.claim_or_read_directory_command_receipt(&pending_claim).await.expect("restart resolution") {
        DirectoryCommandClaimV1::Existing(record) => record,
        other => panic!("a completed key survives restart: {other:?}"),
    };
    assert_eq!(resolved.disposition, DirectoryCommandDispositionV1::Completed);
    assert_eq!(resolved.receipt_sha256.as_deref(), Some(replay_digest.as_str()), "the durable row carries the canonical redacted receipt digest");
    let replayed = replay_directory_command_receipt(&resolved);
    assert_eq!(replayed.outcome, DirectoryCommandOutcomeV1::SecretUndeliverable);
    assert_eq!(replayed.receipt_sha256, replay_digest);
    assert!(replayed.validate().is_ok() && replayed.events.is_empty() && replayed.result == DirectoryCommandResultV1::None);

    let plain = NewDirectoryCommandReceipt { result_kind: DirectoryCommandResultKindV1::None, command_sha256: rename_digest.clone(), request_id: "9f8e7d6c5b4a39281706f5e4d3c2b1a0".into(), ..pending_claim.clone() };
    assert!(matches!(restarted.claim_or_read_directory_command_receipt(&plain).await.expect("plain claim"), DirectoryCommandClaimV1::Claimed(_)));
    restarted
        .complete_directory_command_receipt(&DirectoryCommandReceiptCompletion {
            actor_user_id: plain.actor_user_id.clone(),
            request_id: plain.request_id.clone(),
            event_seq_first: Some(7),
            event_seq_last: Some(8),
            receipt_sha256: "0".repeat(64),
            completed_at: 1_700_000_000_003,
        })
        .await
        .expect("plain completion");
    let plain_record = match restarted.claim_or_read_directory_command_receipt(&plain).await.expect("plain resolution") {
        DirectoryCommandClaimV1::Existing(record) => record,
        other => panic!("a completed plain key replays: {other:?}"),
    };
    assert_eq!((plain_record.event_seq_first, plain_record.event_seq_last), (Some(7), Some(8)));
    assert_eq!(replay_directory_command_receipt(&plain_record).outcome, DirectoryCommandOutcomeV1::PreviouslyAccepted, "a completed secret-free command resolves as previously accepted");
    drop(restarted);
    let _ = std::fs::remove_dir_all(&path);
}

#[tokio::test]
async fn directory_event_page_v1_route_scans_raw_holes_bounds_canonical_receipt_and_visibility() {
    let state = test_state().await;
    let caller = issue_test_session(&state, "event-page-member@example.com").await;
    upsert_member_for_test(&state, STUDIO, "event-page-member@example.com", DirectorySpaceRole::Spectator).await;
    let outsider = issue_test_session(&state, "event-page-owner@example.com").await;
    let hidden_space = create_space_for_test(&state, &outsider.user_id, "Hidden", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    let after = state.directory.head_seq().await.expect("event-page start");
    let appended = append_directory_page_test_events(&state, &[(hidden_space.clone(), "hidden-11".into()), (STUDIO.into(), "visible-12".into()), (hidden_space.clone(), "hidden-13".into()), (STUDIO.into(), "visible-14".into())]).await;
    let addr = spawn_server(state.clone()).await;
    let authorization = event_page_authorization(&caller.token);
    let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("Authorization", &authorization)]).await;
    assert_eq!(response.status, 200);
    assert!(response.headers.to_ascii_lowercase().contains("content-type: application/json"));
    let canonical = std::str::from_utf8(&response.body).expect("event-page UTF-8");
    let page = DirectoryEventPageV1::parse_canonical_json(canonical).expect("canonical event page");
    assert_eq!(page.after_seq_exclusive, after);
    assert_eq!(page.through_seq_inclusive, appended[3].seq);
    assert_eq!(page.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[1].seq, appended[3].seq]);
    assert!(!canonical.contains(&hidden_space));
    assert!(!canonical.contains("hidden-11"));
    assert!(!canonical.contains("hidden-13"));

    let hidden_after = state.directory.head_seq().await.expect("hidden scan start");
    let hidden = (0..DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS).map(|index| (hidden_space.clone(), format!("hidden-saturated-{index}"))).collect::<Vec<_>>();
    let hidden_events = append_directory_page_test_events(&state, &hidden).await;
    let response = raw_http_get(addr, &format!("/directory/event-page/v1?after={hidden_after}"), &[("Authorization", &authorization)]).await;
    let page = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&response.body).expect("hidden page UTF-8")).expect("hidden page");
    assert!(page.events.is_empty());
    assert_eq!(page.through_seq_inclusive, hidden_events.last().expect("hidden tail").seq);
    assert!(page.has_more, "a saturated raw scan advertises the bounded follow-up even when every row is hidden");
}

#[tokio::test]
async fn directory_event_page_v1_route_revalidates_session_generation_after_read_before_response() {
    let mut state = test_state().await;
    let session = issue_test_session(&state, "event-page-revoked@example.com").await;
    upsert_member_for_test(&state, STUDIO, "event-page-revoked@example.com", DirectorySpaceRole::Spectator).await;
    append_directory_page_test_events(&state, &[(STUDIO.into(), "before-revoke".into())]).await;
    let capability = SessionCapability::parse(&session.token).expect("event-page capability");
    let record = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("active session");
    let gate = Arc::new(TestLiveGate::default());
    gate.directory_event_page_fence_enabled.store(true, std::sync::atomic::Ordering::Release);
    state.live_gate = Some(gate.clone());
    let addr = spawn_server(state.clone()).await;
    let authorization = event_page_authorization(&session.token);
    let request = tokio::spawn(async move { raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &authorization)]).await });
    let admitted = tokio::time::timeout(std::time::Duration::from_secs(2), gate.directory_event_page_read_admitted.acquire()).await.expect("event-page read fence deadline").expect("event-page read fence");
    admitted.forget();
    state.directory.revoke_auth_session(&record.id, "event-page-test", None, "event-page-test").await.expect("revoke session").expect("revoked row");
    gate.directory_event_page_read_release.add_permits(1);
    let response = tokio::time::timeout(std::time::Duration::from_secs(2), request).await.expect("revalidation response deadline").expect("revalidation request");
    assert_eq!(response.status, 401);
    assert!(response.body.is_empty());

    let session = issue_test_session(&state, "event-page-cancel@example.com").await;
    let authorization = event_page_authorization(&session.token);
    let cancelled = tokio::spawn(async move { raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &authorization)]).await });
    let admitted = tokio::time::timeout(std::time::Duration::from_secs(2), gate.directory_event_page_read_admitted.acquire()).await.expect("cancel read fence deadline").expect("cancel read fence");
    admitted.forget();
    let control = gate.directory_event_page_control.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone().expect("request-owned cancellation control");
    cancelled.abort();
    assert!(cancelled.await.expect_err("request task cancelled").is_cancelled());
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while control.active.load(std::sync::atomic::Ordering::Acquire) {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("server request cancellation deadline");
    assert!(control.cancelled.load(std::sync::atomic::Ordering::Acquire));
    gate.directory_event_page_read_release.add_permits(1);
}

#[tokio::test]
async fn directory_event_page_v1_route_stops_at_canonical_byte_prefix_without_skipping_visible_seq() {
    let state = test_state().await;
    let token = seed_author_token(&state).await;
    let after = state.directory.head_seq().await.expect("byte-prefix start");
    let appended = append_directory_page_test_events(&state, &[(STUDIO.into(), "a".repeat(32 * 1024)), (STUDIO.into(), "b".repeat(32 * 1024))]).await;
    let addr = spawn_server(state).await;
    let authorization = event_page_authorization(&token);
    let first = raw_http_get(addr, &format!("/directory/event-page/v1?after={after}"), &[("Authorization", &authorization)]).await;
    assert_eq!(first.status, 200);
    assert!(first.body.len() <= DIRECTORY_EVENT_PAGE_MAX_BYTES);
    let first = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&first.body).expect("first page UTF-8")).expect("first page");
    assert_eq!(first.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[0].seq]);
    assert_eq!(first.through_seq_inclusive, appended[0].seq);
    assert!(first.has_more);
    let second = raw_http_get(addr, &format!("/directory/event-page/v1?after={}", first.through_seq_inclusive), &[("Authorization", &authorization)]).await;
    let second = DirectoryEventPageV1::parse_canonical_json(std::str::from_utf8(&second.body).expect("second page UTF-8")).expect("second page");
    assert_eq!(second.events.iter().map(|event| event.seq).collect::<Vec<_>>(), vec![appended[1].seq]);
    assert_eq!(second.through_seq_inclusive, appended[1].seq);
}

#[tokio::test]
async fn directory_event_page_v1_append_admission_is_transactional_for_sqlite_postgres_and_neo4j() {
    let state = test_state().await;
    let mut exact = DirectoryEvent {
        seq: 1,
        id: "event-boundary".into(),
        hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:event-page-boundary".into() },
        space_id: Some(STUDIO.into()),
        user_id: None,
        body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: STUDIO.into(), name: String::new() },
        recorded_at_ms: 1,
    };
    let base = directory::os_pack::json::to_json_string(&exact).len();
    let os_directory::DirectoryEventBody::SpaceRenamed { name, .. } = &mut exact.body else { unreachable!() };
    *name = "x".repeat(os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES - base);
    assert_eq!(directory::os_pack::json::to_json_string(&exact).len(), os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES);
    assert_eq!(validate_directory_event_page_event(&exact), Ok(()));
    if let os_directory::DirectoryEventBody::SpaceRenamed { name, .. } = &mut exact.body {
        name.push('x');
    }
    assert_eq!(validate_directory_event_page_event(&exact), Err(DirectoryEventPageErrorV1::Invalid));

    let head = state.directory.head_seq().await.expect("head before rejected append");
    let rejected = semio_hub::directory::NewDirectoryEvent {
        hlc: os_directory::Hlc { physical_ms: 1, logical: 0 },
        actor: DirectoryActor { kind: DirectoryActorKind::System, id: "system:event-page-reject".into() },
        space_id: Some(STUDIO.into()),
        user_id: None,
        body: os_directory::DirectoryEventBody::SpaceRenamed { space_id: STUDIO.into(), name: "x".repeat(os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES) },
    };
    assert!(matches!(state.directory.append_events(&[rejected]).await, Err(DirectoryError::Conflict(_))));
    assert_eq!(state.directory.head_seq().await.expect("head after rejected append"), head, "SQLite rolls back both row and dense sequence");
    let space = state.directory.get_space(STUDIO).await.expect("space projection").expect("seed space");
    assert_ne!(space.name.len(), os_directory::DIRECTORY_EVENT_PAGE_MAX_EVENT_BYTES, "rejected event never reaches projection");

    let postgres = include_str!("../../📇️directory/🐘️postgres/🦀️.rs");
    let neo4j = include_str!("../../📇️directory/🌐️neo4j/🦀️.rs");
    assert_eq!(postgres.matches("validate_directory_event_page_event(&").count(), 3, "all PostgreSQL full-event append seams admit before persistence");
    assert_eq!(neo4j.matches("validate_directory_event_page_event(&").count(), 3, "all Neo4j full-event append seams admit before persistence");
}

#[tokio::test]
async fn directory_event_page_v1_route_rejects_noncanonical_query_and_stale_bearer_without_body() {
    let mut state = test_state().await;
    let token = seed_author_token(&state).await;
    let stale = issue_test_session(&state, "event-page-stale@example.com").await;
    let stale_capability = SessionCapability::parse(&stale.token).expect("stale capability");
    let stale_record = state.directory.authenticate_session(&stale_capability).await.expect("stale lookup").expect("stale session");
    state.directory.revoke_auth_session(&stale_record.id, "stale", None, "stale").await.expect("stale revoke").expect("stale row");
    let gate = Arc::new(TestLiveGate::default());
    gate.directory_event_page_fence_enabled.store(true, std::sync::atomic::Ordering::Release);
    state.live_gate = Some(gate.clone());
    let addr = spawn_server(state).await;
    let authorization = event_page_authorization(&token);
    for path in [
        "/directory/event-page/v1",
        "/directory/event-page/v1?",
        "/directory/event-page/v1?after=",
        "/directory/event-page/v1?after=00",
        "/directory/event-page/v1?after=1&after=2",
        "/directory/event-page/v1?since=0",
        "/directory/event-page/v1?after=%30",
        "/directory/event-page/v1?after=9007199254740992",
    ] {
        let response = raw_http_get(addr, path, &[("Authorization", &authorization)]).await;
        assert_eq!(response.status, 400, "query {path}");
        assert!(response.body.is_empty(), "query rejection is body-free");
    }
    let missing = raw_http_get(addr, "/directory/event-page/v1?after=0", &[]).await;
    assert_eq!(missing.status, 401);
    assert!(missing.body.is_empty());
    let stale_authorization = event_page_authorization(&stale.token);
    let stale = raw_http_get(addr, "/directory/event-page/v1?after=0", &[("Authorization", &stale_authorization)]).await;
    assert_eq!(stale.status, 401);
    assert!(stale.body.is_empty());
    assert_eq!(gate.directory_event_page_read_admitted.available_permits(), 0, "bad query and pre-read authentication failures perform no directory event scan");
}

#[cfg(feature = "native-artifact-execution")]
#[tokio::test]
async fn checkpoint_publication_route_is_author_owned_actor_fenced_idempotent_and_cancellation_safe() {
    let fixture = checkpoint_publication_fixture("idempotency").await;
    let addr = spawn_server(fixture.state.clone()).await;
    put_checkpoint_publication_blob(addr, &fixture.scope, &fixture.author.token, &fixture.pack).await;
    put_checkpoint_publication_blob(addr, &fixture.scope, &fixture.author.token, &fixture.spr).await;
    let route = format!("/spaces/{}/documents/{}/checkpoint-publications", fixture.scope.space_id, fixture.scope.document_id);
    let body = directory::os_pack::json::to_json_string(&fixture.command);
    let author = format!("Bearer {}", fixture.author.token);
    let spectator = format!("Bearer {}", fixture.spectator.token);
    assert_eq!(raw_http_request(addr, "POST", &route, &[("Content-Type", "application/json")], body.as_bytes()).await.status, 401);
    assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", spectator.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await.status, 403);

    let accepted = raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await;
    assert_eq!(accepted.status, 200, "author checkpoint publication: {}", String::from_utf8_lossy(&accepted.body));
    assert!(accepted.headers.to_ascii_lowercase().contains("cache-control: private, no-store"));
    let receipt: CheckpointPublicationReceiptV1 = directory::os_pack::json::from_json_str(std::str::from_utf8(&accepted.body).expect("publication receipt UTF-8")).expect("canonical publication receipt");
    assert_eq!(receipt.correlation_id, fixture.command.correlation_id);
    assert_eq!(receipt.checkpoint.scope, fixture.scope);
    assert_eq!(receipt.checkpoint.pack.sha256.hex(), fixture.command.pack.sha256);
    assert_eq!(receipt.checkpoint.spr.sha256.hex(), fixture.command.spr.sha256);
    assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 1);

    let replay = raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await;
    assert_eq!(replay.status, 200);
    assert_eq!(replay.body, accepted.body, "a lost-response retry returns the identical durable receipt");
    assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("replay checkpoint count"), 1, "retry emits no second checkpoint event");
    let command_sha256 = os_directory::hex_lower(&Sha256::digest(body.as_bytes()));
    let durable = fixture
        .state
        .directory
        .claim_or_read_checkpoint_publication(&NewCheckpointPublicationClaimV1 { actor_user_id: fixture.author.user_id.clone(), correlation_id: fixture.command.correlation_id.clone(), command_sha256: command_sha256.clone(), claimed_at: now_ms() })
        .await
        .expect("durable publication receipt");
    let CheckpointPublicationClaimV1::Existing(durable) = durable else { panic!("completed publication must be durable") };
    assert_eq!(durable.disposition, CheckpointPublicationDispositionV1::Completed);
    assert_eq!(durable.checkpoint_id, Some(receipt.checkpoint.checkpoint_id));

    let mut substituted = fixture.command.clone();
    substituted.spr.byte_length += 1;
    let substituted = directory::os_pack::json::to_json_string(&substituted);
    let conflict = raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], substituted.as_bytes()).await;
    assert_eq!(conflict.status, 409, "same author/correlation with a different exact command conflicts");
    assert!(conflict.body.is_empty());
    std::fs::remove_dir_all(fixture.catalog_root).expect("remove publication catalog fixture");
}

#[cfg(feature = "native-artifact-execution")]
#[tokio::test]
async fn checkpoint_publication_route_rejects_stale_or_cross_scope_inputs_before_publication() {
    let mut fixture = checkpoint_publication_fixture("fence").await;
    let gate = Arc::new(TestLiveGate::default());
    gate.checkpoint_publication_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
    fixture.state.live_gate = Some(gate.clone());
    let other_space = create_space_for_test(&fixture.state, &fixture.author.user_id, "Checkpoint other scope", os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&fixture.state, &other_space, "checkpoint-fence-author@example.test", DirectorySpaceRole::Author).await;
    let descriptor = fixture.state.directory.get_document_descriptor(&fixture.scope).await.expect("publication descriptor read").expect("publication descriptor");
    let mut other_descriptor = descriptor.clone();
    other_descriptor.space_id = other_space.clone();
    fixture
        .state
        .directory_service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#checkpoint-test", fixture.author.user_id) }, DirectoryCommand::AnnounceDocument { descriptor: other_descriptor })
        .await
        .expect("announce same-id other-space document");
    let addr = spawn_server(fixture.state.clone()).await;
    put_checkpoint_publication_blob(addr, &fixture.scope, &fixture.author.token, &fixture.pack).await;
    put_checkpoint_publication_blob(addr, &fixture.scope, &fixture.author.token, &fixture.spr).await;
    let authorization = format!("Bearer {}", fixture.author.token);
    let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];

    let mut cross_scope = fixture.command.clone();
    cross_scope.correlation_id = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into();
    let cross_scope = directory::os_pack::json::to_json_string(&cross_scope);
    let cross = raw_http_request(addr, "POST", &format!("/spaces/{other_space}/documents/{}/checkpoint-publications", fixture.scope.document_id), &headers, cross_scope.as_bytes()).await;
    assert_eq!(cross.status, 409, "route scope cannot borrow another space's selected descriptor/frontier");
    assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("source scope checkpoint count"), 0);
    assert_eq!(fixture.state.directory.artifact_checkpoint_count(&DocumentScope::new(&other_space, &fixture.scope.document_id)).await.expect("other scope checkpoint count"), 0);

    let route = format!("/spaces/{}/documents/{}/checkpoint-publications", fixture.scope.space_id, fixture.scope.document_id);
    let body = directory::os_pack::json::to_json_string(&fixture.command);
    let queued = tokio::spawn({
        let route = route.clone();
        let body = body.clone();
        let authorization = authorization.clone();
        async move { raw_http_request(addr, "POST", &route, &[("Authorization", authorization.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.checkpoint_publication_admitted.acquire()).await.expect("publication fence admission deadline").expect("publication fence admission").forget();
    let write = fixture.state.socket_binding_gates.gate(SocketBindingKeyV1::DocumentWrite(fixture.scope.clone())).lock_owned().await;
    let document = db_artifact_id(&fixture.scope);
    let batch = db::document::CommandBatch::new(vec![sample_envelope("checkpoint-fence-edit-2", &WireArtifactId(document.0)).await]).await.expect("queued write batch");
    fixture.handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy: protocol::MergePolicy::default() }).await.expect("queued write actor response").expect("queued write accepted");
    drop(write);
    gate.checkpoint_publication_release.add_permits(1);
    let queued = queued.await.expect("queued publication response");
    assert_eq!(queued.status, 409, "the final actor snapshot fence rejects a write committed during materialization");
    assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("stale publication count"), 0);
    let failed_digest = os_directory::hex_lower(&Sha256::digest(body.as_bytes()));
    let failed_claim = NewCheckpointPublicationClaimV1 { actor_user_id: fixture.author.user_id.clone(), correlation_id: fixture.command.correlation_id.clone(), command_sha256: failed_digest.clone(), claimed_at: now_ms() };
    assert!(
        matches!(fixture.state.directory.claim_or_read_checkpoint_publication(&failed_claim).await.expect("reclaim failed publication"), CheckpointPublicationClaimV1::Claimed(_)),
        "a returned failure synchronously releases its durable claim for a corrected retry"
    );
    fixture.state.directory.release_checkpoint_publication(&failed_claim.actor_user_id, &failed_claim.correlation_id, &failed_digest).await.expect("release test reclaim");

    let current = fixture.handle.checkpoint_publication_snapshot().await.expect("current publication snapshot");
    let descriptor_command = checkpoint_publication_command("cccccccccccccccccccccccccccccccc", &descriptor, &current, fixture.command.expected_current.clone(), &fixture.pack, &fixture.spr);
    let descriptor_body = directory::os_pack::json::to_json_string(&descriptor_command);
    let descriptor_swap = tokio::spawn({
        let route = route.clone();
        let authorization = authorization.clone();
        async move { raw_http_request(addr, "POST", &route, &[("Authorization", authorization.as_str()), ("Content-Type", "application/json")], descriptor_body.as_bytes()).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.checkpoint_publication_admitted.acquire()).await.expect("descriptor fence admission deadline").expect("descriptor fence admission").forget();
    let mut changed_descriptor = descriptor.clone();
    changed_descriptor.bootstrap_version = changed_descriptor.bootstrap_version.saturating_add(1);
    fixture
        .state
        .directory_service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#checkpoint-test", fixture.author.user_id) }, DirectoryCommand::AnnounceDocument { descriptor: changed_descriptor })
        .await
        .expect("replace publication descriptor");
    gate.checkpoint_publication_release.add_permits(1);
    let descriptor_swap = descriptor_swap.await.expect("descriptor-swapped publication response");
    assert_eq!(descriptor_swap.status, 409, "the final selected-descriptor fence rejects a replacement during materialization");
    assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("descriptor-swapped publication count"), 0);
    fixture
        .state
        .directory_service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#checkpoint-test", fixture.author.user_id) }, DirectoryCommand::AnnounceDocument { descriptor: descriptor.clone() })
        .await
        .expect("restore publication descriptor");

    let mut cancellation = checkpoint_publication_command("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", &descriptor, &current, fixture.command.expected_current.clone(), &fixture.pack, &fixture.spr);
    cancellation.schema = "semio.hub.checkpoint-publication-command/v1".into();
    let cancellation = directory::os_pack::json::to_json_string(&cancellation);
    let capability = SessionCapability::parse(&fixture.author.token).expect("publication session capability");
    let session = fixture.state.directory.authenticate_session(&capability).await.expect("publication session lookup").expect("publication session");
    let revoked = tokio::spawn({
        let route = route.clone();
        let authorization = authorization.clone();
        async move { raw_http_request(addr, "POST", &route, &[("Authorization", authorization.as_str()), ("Content-Type", "application/json")], cancellation.as_bytes()).await }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), gate.checkpoint_publication_admitted.acquire()).await.expect("revocation fence admission deadline").expect("revocation fence admission").forget();
    fixture.state.directory.revoke_auth_session(&session.id, "checkpoint-publication-test", None, "checkpoint-publication-test").await.expect("revoke publication session").expect("revoked publication session");
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    gate.checkpoint_publication_release.add_permits(1);
    let revoked = revoked.await.expect("revoked publication response");
    assert_eq!(revoked.status, 503, "revocation cancels the request-local authority operation");
    assert!(revoked.body.is_empty());
    assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("cancelled publication count"), 0);
    std::fs::remove_dir_all(fixture.catalog_root).expect("remove publication fence catalog fixture");
}

// 🔬️ WS duplex fan-out over the real wire-v2 protocol: A's committed command reaches B on its
// own socket as a `ServerFrame::Commands`, and B's Ack for A's own submit never round-trips
// back to A as a duplicate Commands frame (origin filtering is the caller's job — this test
// only asserts B observes it, matching `framework/sync`'s own origin check).

// 🔬️ `GET`/`DELETE /auth/sessions/me`: a live session resolves the caller's identity; revoking
// it makes the SAME token unauthorized on a subsequent call.
#[tokio::test]
async fn auth_sessions_me_roundtrip() {
    let state = test_state().await;
    let session = issue_test_session(&state, "me@example.com").await;
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.token).parse().unwrap());

    let me = get_session_me(headers.clone(), State(state.clone())).await.expect("session me");
    assert_eq!(me.0.user_id, session.user_id);
    assert_eq!(me.0.email, "me@example.com");

    assert_eq!(delete_session_me(headers.clone(), State(state.clone())).await, StatusCode::NO_CONTENT);
    assert_eq!(get_session_me(headers, State(state)).await.err(), Some(StatusCode::UNAUTHORIZED));
}
