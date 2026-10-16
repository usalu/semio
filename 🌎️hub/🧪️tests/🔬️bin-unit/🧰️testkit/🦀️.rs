//! 🧰️ Shared fixtures of the hub route laws: test states, servers, sockets, sessions and document-open plan authorities.

use semio_hub_route_models::admin::{ADMIN_OPERATION_SHUTDOWN_DEADLINE, AdminOperationTaskOwner};
use protocol::{ActorId, ClientFrame, Lane, MutationEnvelope, ServerFrame, ArtifactId as WireArtifactId};
use semio_hub_ledgers::guard::{AdminSubject, admin_provider_digest};
use std::sync::Arc;
use semio_hub_artifact_authority::{ArtifactBlobIntegrity, ArtifactPair, AuthorityLimits, CheckpointPublicationOrchestrator, OperationContext, checkpoint_id_encoding_v1};
use semio_hub_runtime::startup::{ArtifactCasMaintenanceSupervisor, DocumentOpenCatalogAuthorityV1, StartupCatalogControl, instance_state};
use semio_hub_artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStores, FsArtifactChunkCasStorage, MemoryArtifactChunkCasStorage, artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1};
use semio_hub_artifact_creation::ArtifactCreationActorV1;
use semio_hub_route_models::artifact_creation::{ArtifactCreationHttpTaskOwnerV1, HubArtifactCreationCommitAuthorityV1};
use semio_hub_directory_contract::model::{AuthSessionIssue, AuthSessionKind, SpaceRole};
use axum::body::Bytes;
use semio_framework_trace::record::{CapturingSink, TraceLevel, Tracer};
use semio_hub_auth::CredentialSignInPolicyV1;
use semio_hub_runtime::transport::{CrossOriginPolicyV1, ForwardedTlsTrustV1};
use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandRequestV1, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenIntentV1, DocumentOpenPackageV1, DocumentOpenPlanV1, DocumentOpenSurfaceV1, DocumentPlanSocketGrantIntentV1, DocumentScope};
use semio_hub_runtime::support::{DirectoryJson, now_ms};
use semio_hub_directory_service::{DirectoryService, HubVerifiedCheckpointPublisher};
use semio_hub_artifact_checkin::DocumentCheckInJobs;
use semio_hub_ledgers::open_plan::{DocumentOpenPlanAuthorityV1, DocumentOpenPlanLedgerV1};
use semio_hub_ledgers::access_grants::DocumentSocketGrantReceiptV1;
use directory::{FromValue, os_directory};
use axum::http::HeaderMap;
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_stores::HubInstance;
use semio_hub_runtime::readiness::{HubMode, hub_readiness};
use semio_hub_auth::rate_limit::HubRateLimiterV1;
use semio_hub_runtime::lifecycle::{HubSocketDrainV1, hub_worker_pool};
use semio_hub_state::HubState;
use axum::Json;
use semio_hub_runtime::presence::PresenceDirectoryProjections;
use semio_hub_ledgers::socket_grants::{SESSION_PROTOCOL_V1, SOCKET_PROTOCOL_V1, SocketBindingGatesV1, SocketGrantLedgerV1, SocketSubjectV1};
use server::gateway::ServerState;
use semio_hub_directory_contract::{SessionCapability, identity_subject_digest};
use semio_framework_hash::Sha256;
use semio_framework_async::ShardedMap;
use std::net::SocketAddr;
use directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreateV1;
use semio_hub_directory_sqlite::SqliteDirectory;
use futures::StreamExt;
use semio_hub_route_models::state::{TestDocumentOpenCatalog, last_document_plan_refusal};
use semio_hub_trusted_catalog_descriptor::VerifiedDocumentOpenSelectionV1;
use semio_hub_lag_rebootstrap::VerifiedRebootstrapSource;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use semio_hub_artifact_creation::io::artifact_creation_command_digest_v1;
use semio_hub_session_routes::access_grants::{issue_document_open_plan_inner, issue_document_plan_socket_grant_inner};
use semio_hub_routes::router;
use semio_hub_directory::{ArtifactCasDirectory, ArtifactCreationDirectory, AuthSessionDirectory, DocumentDirectory, UserDirectory};

/// 🏛️ The seeded space id every test routes against (see `SqliteDirectory::seed`).
pub const STUDIO: &str = "default";

pub fn native_openable_stdio_bundle() -> std::path::PathBuf {
    let root = tempdir("native-openable-stdio");
    let stage = root.join("generation-stage");
    std::fs::create_dir_all(stage.join("components")).expect("stdio component directory");
    std::fs::create_dir_all(stage.join("descriptors")).expect("stdio descriptor directory");
    let component = b"\0asm\x0d\x00\x01\x00";
    let component_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(component));
    let component_blake3 = blake3::hash(component).to_hex().to_string();
    let receipts = semio_hub_stdio_catalog::native_codec_factory_receipts().expect("artifact-owned stdio receipts");
    let viewer = semio_framework_plugin::Viewer::builder(semio_framework_plugin::Dialect { artifact_kind: "s.stdio.json", standard: semio_framework_plugin::StandardId("rfc8259"), subset: semio_framework_plugin::SubsetId::ANY })
        .document(["semio", "stdio", "json"])
        .mode("view", semio_framework_plugin::LocalizedLabel::native("View", "Ansicht"), "eye")
        .default_mode_id("view")
        .window_kind_def(<semio_framework_plugin::app::TreeWindowKit as semio_framework_plugin::app::WindowKit>::window_kind())
        .build_definition();
    let mut viewer = viewer;
    viewer.artifact_kinds = semio_hub_stdio_catalog::native_codec_artifact_kinds().into_iter().filter(|kind| kind.id == "s.stdio.json").collect();
    let mut manifest = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::new("stdio", "Stdio Fixture", receipts[0].package_version).manifest;
    manifest.artifact_kinds = semio_hub_stdio_catalog::native_codec_artifact_kinds();
    manifest.apps.push(viewer.clone());
    manifest.topic_contributions.push(semio_hub_stdio_catalog::native_artifact_catalog_contribution().expect("synthetic fixture retains full catalog semantics"));
    let receipt_pairs = receipts.iter().map(|receipt| (receipt.artifact_kind.clone(), receipt.schema.clone())).collect::<std::collections::BTreeSet<_>>();
    let authored: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json")).expect("schema-qualified factory roster");
    let expected_pairs = authored["receipts"].as_array().unwrap().iter().map(|receipt| (receipt["artifact_kind"].as_str().unwrap().to_owned(), receipt["artifact_schema"].as_str().unwrap().to_owned())).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(receipt_pairs.len(), receipts.len(), "every executable owner receipt has one unique kind/schema pair");
    assert_eq!(receipt_pairs, expected_pairs, "every descriptor kind/schema pair has its exact executable owner receipt");
    assert_eq!(manifest.artifact_kinds.iter().map(|kind| kind.id.as_str()).collect::<std::collections::BTreeSet<_>>(), receipts.iter().map(|receipt| receipt.artifact_kind.as_str()).collect(), "every descriptor artifact kind has executable owner coverage");
    assert_eq!(manifest.artifact_kinds.len(), receipts.iter().map(|receipt| receipt.artifact_kind.as_str()).collect::<std::collections::BTreeSet<_>>().len(), "descriptor artifact kinds are unique");
    let pair_fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧩️compositions/🗄️stdio/📇️catalog/🧪️tests/🧫️fixtures/📇️native-catalog-surface/🧪️receipt-pairs.json")).expect("neutral receipt pair corpus");
    let expected = pair_fixture["pairs"].as_array().unwrap().iter().map(|pair| (pair[0].as_str().unwrap().to_owned(), pair[1].as_str().unwrap().to_owned())).collect::<std::collections::BTreeSet<_>>();
    let kinds = pair_fixture["kinds"].as_array().unwrap().iter().map(|kind| kind.as_str().unwrap().to_owned()).collect::<std::collections::BTreeSet<_>>();
    for row in pair_fixture["cases"].as_array().unwrap() {
        let mut pairs = pair_fixture["pairs"].as_array().unwrap().iter().map(|pair| (pair[0].as_str().unwrap().to_owned(), pair[1].as_str().unwrap().to_owned())).collect::<Vec<_>>();
        match row["mutation"].as_str().unwrap() {
            "none" => {}
            "reverse" => pairs.reverse(),
            "missing-schema" => { pairs.remove(1); }
            "missing-kind" => { pairs.pop(); }
            "duplicate" => pairs[1] = pairs[0].clone(),
            "foreign-schema" => pairs[1].1 = "foreign.schema".into(),
            "foreign-kind" => pairs[1].0 = "foreign.artifact".into(),
            "extra" => pairs.push(("s.stdio.pdf".into(), "foreign.schema".into())),
            mutation => panic!("unknown receipt pair mutation {mutation}"),
        }
        let actual = pairs.iter().cloned().collect::<std::collections::BTreeSet<_>>();
        let actual_kinds = pairs.iter().map(|pair| pair.0.clone()).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(actual.len() == pairs.len() && actual == expected && actual_kinds == kinds, row["accepted"].as_bool().unwrap(), "neutral receipt pair {}", row["id"]);
    }
    assert_eq!(viewer.id, "s.stdio.json@rfc8259/*#viewer", "the synthetic viewer opens the manifest-declared kind its own dialect names");
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
    };
    let descriptor_bytes = directory::os_pack::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor));
    let descriptor_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&descriptor_bytes));
    let receipt = directory::os_component_receipt::ComponentReceiptV1 {
        schema: directory::os_component_receipt::COMPONENT_RECEIPT_SCHEMA.into(),
        plugin_id: "stdio".into(),
        package_id: "semio:stdio".into(),
        profile: directory::os_component_receipt::ComponentProfileV1::Release,
        target: directory::os_component_receipt::COMPONENT_TARGET.into(),
        wasm_sha256: component_sha256.clone(),
        wasm_bytes: component.len() as u64,
        semantic_descriptor_sha256: descriptor_sha256.clone(),
        toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)".into(),
        source_tree_digest: "3".repeat(64),
        provenance_sha256: "4".repeat(64),
    };
    let receipt_bytes = receipt.encode_canonical();
    let receipt_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&receipt_bytes));
    let json = receipts.iter().find(|receipt| receipt.factory_id == "stdio.native.json.v1").expect("JSON receipt");
    let native_codecs = receipts
        .iter()
        .map(|receipt| {
            serde_json::json!({
                "artifactKind": receipt.artifact_kind,
                "artifactSchema": receipt.schema,
                "packSchemaHash": os_directory::io::binary::artifact_hash::hex_lower(&receipt.pack_schema_hash)
            })
        })
        .collect::<Vec<_>>();
    let version = receipts[0].package_version;
    let target = serde_json::json!({
        "artifactKind": json.artifact_kind,
        "artifactSchema": json.schema,
        "packSchemaHash": os_directory::io::binary::artifact_hash::hex_lower(&json.pack_schema_hash),
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
        "schemaVersion": 4,
        "profiles": [{
            "id": "stdio-native-openable-v1",
            "selectedClosure": [{ "pluginId": "stdio", "packageId": "semio:stdio", "version": version }],
            "selectedClosureSha256": "11".repeat(32),
            "openTargets": [{
                "package": { "pluginId": "stdio", "packageId": "semio:stdio", "version": version },
                "target": target
            }],
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
            "receipt": receipt,
            "nativeCodecs": native_codecs,
            "openTargets": [target]
        }]
    });
    bundle["packages"][0]["browserActor"] = serde_json::json!({
        "kind":"closed-browser-actor", "schema":"semio.os.closed-browser-actor.v1", "codegenPolicy":"semio.os.browser-jco-1.34.0-jspi.v1",
        "path":"closed-actor.mjs", "byteLength":component.len(), "sha256":component_sha256,
        "sourceComponentSha256":component_sha256, "sourceDescriptorByteSha256":descriptor_sha256, "policySha256":"41".repeat(32), "importInterfaces":[]
    });
    std::fs::write(stage.join("closed-actor.mjs"), component).expect("synthetic actor, never executed");
    let plugin_module_files = vec![("stdio/🌉️bridge.js".to_owned(), b"export {};\n".to_vec()), ("stdio/🔣️.json".to_owned(), b"{}\n".to_vec()), ("stdio/🛂️.descriptor.semio".to_owned(), descriptor_bytes.clone()), ("stdio/🧾️receipt.json".to_owned(), receipt_bytes)];
    let plugin_module_source = semio_hub_trusted_catalog_contract::plugin_module::TrustedPluginModuleSourceV1 { plugin_id: "stdio", package_id: "semio:stdio", version: &*version, component_sha256: &component_sha256, descriptor_byte_sha256: &descriptor_sha256, receipt_sha256: &receipt_sha256 };
    let plugin_module = semio_hub_trusted_catalog_contract::plugin_module::write_plugin_module_bundle(&stage, "plugin-module-stdio.json", plugin_module_source, "stdio", &plugin_module_files).expect("stdio plugin module");
    bundle["packages"][0]["pluginModule"] = serde_json::to_value(plugin_module).expect("stdio plugin module record");
    let carried = serde_json::to_vec(&bundle).expect("provisional stdio bundle");
    let (selected_closure_sha256, generation_id) = semio_hub_trusted_catalog_descriptor::trusted_profile_digests_json(&carried, "stdio-native-openable-v1").expect("stdio profile digests");
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
    let bundle_sha256 = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&bundle_bytes));
    let current_bytes = format!(
        r#"{{"profileId":"stdio-native-openable-v1","generationId":"{generation}","bundleSha256":"{bundle_sha256}","publicationRevision":"1"}}
"#
    )
    .into_bytes();
    std::fs::write(trusted_root.join("current.json"), current_bytes).expect("publish current pointer");
    std::fs::canonicalize(root).expect("canonical fixture-owned data root")
}

/// 📁️ A fresh, never-reused temp directory per call — the owned `time_ordered_id` rather than
/// `now_ms()` alone, since `cargo test` runs this whole module's `#[tokio::test]`s
/// concurrently within one process: two tests calling `test_state()` in the same millisecond
/// would otherwise collide on the identical `os-hub-test-db-<pid>-<ms>` path and open the SAME
/// `db::Database` storage root, corrupting each other's catalog/WAL state.
pub fn tempdir(name: &str) -> std::path::PathBuf {
    let mut dir = semio_hub_testing::test_artifact_root();
    dir.push(format!("os-hub-test-{name}-{}", directory::os_identity::time_ordered_id()));
    std::fs::create_dir_all(&dir).expect("owned temporary fixture directory");
    dir
}

pub fn run_socket_test<F, Fut>(test: F)
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

pub async fn test_state() -> HubState {
    test_state_with_capacity(1024, 256).await
}

/// 🗄️ The instance a test's `HubState` carries: the real durable profile, under the test's own
/// temporary root, so a law that reopens the same root observes exactly what a hub restart does.
pub async fn test_instance_state(dir: &std::path::Path) -> ServerState<HubInstance> {
    instance_state(dir).await.expect("open hub instance stores")
}

/// ⏳️ The single wall-clock bound every in-process rendezvous in this suite answers to — a socket
/// close, a live-gate permit, a server task's reply. It is a hang guard, never a measurement: a law
/// that waits on a signal the product genuinely never sends must fail here, fast and locally, instead
/// of being killed 300 s later by nextest with no site. It replaces the per-call 2 s and 5 s bounds
/// that fired on fleet scheduling latency alone.
pub const SOCKET_RENDEZVOUS_HANG_GUARD: std::time::Duration = std::time::Duration::from_secs(15);

/// ⏳️ The bound a law's whole body answers to when the runtime itself may be the thing that wedged.
/// `tokio::time::timeout` cannot fire on a starved executor — a current-thread runtime whose single
/// task blocks, or spins without yielding, never advances the timer wheel — so a law that hangs that
/// way is killed 300 s later by nextest with no site at all. `LawHangWatchdogV1` is a plain OS thread
/// that owns a wall clock nothing in the runtime can starve.
const LAW_BODY_HANG_GUARD: std::time::Duration = std::time::Duration::from_secs(25);

/// ⏱️ An out-of-runtime wall-clock watchdog: armed at the top of a law, disarmed by its own `Drop`
/// when the law returns. On expiry it names the law on stderr and aborts the process, so nextest
/// reports that law — and only that law — as failed within seconds instead of after the 300 s
/// per-test kill. It is a hang guard, never a measurement: a law that legitimately takes this long
/// does not belong in the shared bin binary.
pub struct LawHangWatchdogV1 {
    finished: Arc<std::sync::atomic::AtomicBool>,
    pub phase: Arc<std::sync::Mutex<&'static str>>,
}

impl LawHangWatchdogV1 {
    pub fn arm(law: &'static str) -> Self {
        let finished = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let phase: Arc<std::sync::Mutex<&'static str>> = Arc::new(std::sync::Mutex::new("armed, before the first step"));
        let watched = finished.clone();
        let observed = phase.clone();
        std::thread::Builder::new()
            .name("law-hang-watchdog".into())
            .spawn(move || {
                let deadline = std::time::Instant::now() + LAW_BODY_HANG_GUARD;
                while std::time::Instant::now() < deadline {
                    if watched.load(std::sync::atomic::Ordering::Acquire) {
                        return;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(25));
                }
                if !watched.load(std::sync::atomic::Ordering::Acquire) {
                    let stuck = *observed.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                    eprintln!("law hang watchdog: {law} did not finish within {LAW_BODY_HANG_GUARD:?} — the runtime is wedged, not slow; last phase entered: {stuck}");
                    std::process::abort();
                }
            })
            .expect("law hang watchdog thread");
        Self { finished, phase }
    }

    pub fn at(&self, phase: &'static str) {
        *self.phase.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = phase;
    }
}

impl Drop for LawHangWatchdogV1 {
    fn drop(&mut self) {
        self.finished.store(true, std::sync::atomic::Ordering::Release);
    }
}

/// ⏳️ One awaited fixture step, bounded and named, so a law that wedges says WHICH await never
/// returned instead of naming only itself. It complements `LawHangWatchdogV1`: this one fires when
/// the future genuinely never resolves, that one when the executor can no longer run timers at all.
pub async fn bounded_law_step<T>(step: &str, future: impl std::future::Future<Output = T>) -> T {
    match tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, future).await {
        Ok(value) => value,
        Err(_) => panic!("{step} did not complete within {SOCKET_RENDEZVOUS_HANG_GUARD:?}"),
    }
}

/// ⏳️ The single wall-clock bound every HTTP read in this suite answers to. It exists only to stop a
/// route that never answers from hanging the shared bin binary; it is never the thing a law measures.
/// The transport used to carry a second, shorter bound of its own, which silently pre-empted this one
/// and turned fleet scheduling latency into red laws that had nothing to do with the hub.
pub const RAW_HTTP_READ_HANG_GUARD: std::time::Duration = std::time::Duration::from_secs(20);

/// 📝️ A hub state whose tracer captures, handed back beside the sink its records land in.
pub async fn observed_test_state() -> (HubState, Arc<CapturingSink>) {
    let (tracer, sink) = Tracer::capturing(TraceLevel::Debug);
    let mut state = test_state().await;
    state.tracer = tracer;
    (state, sink)
}

pub async fn test_state_with_capacity(directory_capacity: usize, fanout_capacity: usize) -> HubState {
    let dir = tempdir("db");
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
    test_state_with_directory(dir, directory, directory_capacity, fanout_capacity).await
}

/// 📝️ Silent by default: a law that wants records takes `observed_test_state` and swaps in a
/// capturing tracer, so every other law pays no formatting and asserts against no log.
pub async fn test_state_with_directory(dir: std::path::PathBuf, directory: SqliteDirectory, directory_capacity: usize, fanout_capacity: usize) -> HubState {
    let database = db::Database::open_at(hub_worker_pool(), &dir, db::Profile::Test).await.expect("open db");
    directory.seed().await.expect("seed");
    let directory: Arc<HubDirectories> = Arc::new(directory.into());
    let directory_service = Arc::new(DirectoryService::new(directory.clone(), directory_capacity));
    let database = Arc::new(database);
    let artifact_cas = Arc::new(ArtifactChunkCasStores::Filesystem(FsArtifactChunkCasStorage::open(&dir.join("artifact-cas/v1")).await.expect("open artifact CAS")));
    let control = StartupCatalogControl::silent();
    let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let coordinator_id = directory.artifact_cas_coordinator_id().await.expect("artifact CAS coordinator");
    artifact_cas.configure_coordinator(coordinator_id, &context).await.expect("configure artifact CAS coordinator");
    let artifact_publication =
        Arc::new(CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(artifact_cas.clone()), HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority-test")));
    let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
    let socket_binding_gates = Arc::new(SocketBindingGatesV1::default());
    let artifact_creation_commit_authority = Arc::new(HubArtifactCreationCommitAuthorityV1 { directory: directory.clone(), gates: socket_binding_gates.clone() });
    HubState {
        tracer: Tracer::disabled(),
        route_metrics: Arc::default(),
        instance: test_instance_state(&dir).await,
        db: database,
        artifact_cas,
        directory,
        rebootstrap,
        artifact_authority: None,
        verified_catalog: None,
        artifact_creation: None,
        artifact_creation_commit_authority,
        artifact_creation_tasks: Arc::new(ArtifactCreationHttpTaskOwnerV1::new()),
        gis_map_binding: None,
        inference_runtime: None,
        openable_catalog: None,
        artifact_publication,
        artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
        check_ins: Arc::new(DocumentCheckInJobs::default()),
        directory_service,
        credential_sign_in: CredentialSignInPolicyV1::default(),
        rate_limits: Arc::new(HubRateLimiterV1::system()),
        asset_streams: Arc::default(),
        admin_subjects: Arc::from([]),
        admin_cursor_key: [0x5a; 32],
        space_administration_cursor_key: [0xa5; 32],
        admin_operations: Arc::new(ShardedMap::new()),
        admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
        admin_operation_tasks: Arc::new(AdminOperationTaskOwner::new(ADMIN_OPERATION_SHUTDOWN_DEADLINE)),
        readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, false, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root")),
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
        presence_publication_gate: Arc::new(tokio::sync::Mutex::new(PresenceDirectoryProjections::default())),
        presence_clock: None,
        session_colors: Arc::new(ShardedMap::new()),
        session_kicks: Arc::new(ShardedMap::new()),
        socket_drain: Arc::new(HubSocketDrainV1::default()),
        socket_grants: Arc::new(SocketGrantLedgerV1::default()),
        document_open_plans: Arc::new(DocumentOpenPlanLedgerV1::default()),
        socket_binding_gates: {
            {
                socket_binding_gates
            }
        },
        extensions_root: dir.join("extension-modules"),
        merge_policy: protocol::MergePolicy::default(),
        checkpoint_policy: Arc::default(),
    }
}

/// 📝️ Silent by default: a law that wants records takes `observed_test_state` and swaps in a
/// capturing tracer, so every other law pays no formatting and asserts against no log.
pub async fn lag_test_state(directory_capacity: usize, fanout_capacity: usize) -> HubState {
    let dir = tempdir("lag-db");
    let pool = hub_worker_pool();
    let backend = Arc::new(db::storage::DbBackend::Memory(db::storage::MemoryStorage::new(pool.clone()).await.expect("memory storage")));
    let database = Arc::new(db::Database::open(pool, db::DbConfig::for_profile(db::Profile::Test), backend).await.expect("open memory db"));
    let directory = SqliteDirectory::connect(":memory:").await.expect("connect directory");
    directory.seed().await.expect("seed");
    let directory: Arc<HubDirectories> = Arc::new(directory.into());
    let directory_service = Arc::new(DirectoryService::new(directory.clone(), directory_capacity));
    let artifact_cas = Arc::new(ArtifactChunkCasStores::Memory(MemoryArtifactChunkCasStorage::default()));
    let control = StartupCatalogControl::silent();
    let context = OperationContext::new(control.now_ms().saturating_add(30_000), AuthorityLimits::maximum(), &control);
    let coordinator_id = directory.artifact_cas_coordinator_id().await.expect("artifact CAS coordinator");
    artifact_cas.configure_coordinator(coordinator_id, &context).await.expect("configure artifact CAS coordinator");
    let artifact_publication =
        Arc::new(CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(artifact_cas.clone()), HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority-test")));
    let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
    let socket_binding_gates = Arc::new(SocketBindingGatesV1::default());
    let artifact_creation_commit_authority = Arc::new(HubArtifactCreationCommitAuthorityV1 { directory: directory.clone(), gates: socket_binding_gates.clone() });
    HubState {
        tracer: Tracer::disabled(),
        route_metrics: Arc::default(),
        instance: test_instance_state(&dir).await,
        db: database,
        artifact_cas,
        directory,
        rebootstrap,
        artifact_authority: None,
        verified_catalog: None,
        artifact_creation: None,
        artifact_creation_commit_authority,
        artifact_creation_tasks: Arc::new(ArtifactCreationHttpTaskOwnerV1::new()),
        gis_map_binding: None,
        inference_runtime: None,
        openable_catalog: None,
        artifact_publication,
        artifact_maintenance: ArtifactCasMaintenanceSupervisor::disabled(),
        check_ins: Arc::new(DocumentCheckInJobs::default()),
        directory_service,
        credential_sign_in: CredentialSignInPolicyV1::default(),
        rate_limits: Arc::new(HubRateLimiterV1::system()),
        asset_streams: Arc::default(),
        admin_subjects: Arc::from([]),
        admin_cursor_key: [0x5a; 32],
        space_administration_cursor_key: [0xa5; 32],
        admin_operations: Arc::new(ShardedMap::new()),
        admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
        admin_operation_tasks: Arc::new(AdminOperationTaskOwner::new(ADMIN_OPERATION_SHUTDOWN_DEADLINE)),
        readiness: Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, false, false, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root")),
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
        presence_publication_gate: Arc::new(tokio::sync::Mutex::new(PresenceDirectoryProjections::default())),
        presence_clock: None,
        session_colors: Arc::new(ShardedMap::new()),
        session_kicks: Arc::new(ShardedMap::new()),
        socket_drain: Arc::new(HubSocketDrainV1::default()),
        socket_grants: Arc::new(SocketGrantLedgerV1::default()),
        document_open_plans: Arc::new(DocumentOpenPlanLedgerV1::default()),
        socket_binding_gates: {
            {
                socket_binding_gates
            }
        },
        extensions_root: dir.join("extension-modules"),
        merge_policy: protocol::MergePolicy::default(),
        checkpoint_policy: Arc::default(),
    }
}

/// 🏗️ Test-only `create-space` through `DirectoryService::execute` (the trait's own
/// `create_space` write method is gone — see `📓️w1-b-report.md`) — returns the minted space id.
/// `decide` performs zero authorization of its own, so `owner_user_id` need not be a real,
/// already-existing user for these low-level fixture setups.
pub async fn create_space_for_test(state: &HubState, owner_user_id: &str, name: &str, space_kind: os_directory::DirectorySpaceKind, visibility: DirectorySpaceVisibility) -> String {
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

/// 🏗️ Test-only `upsert-member` through `DirectoryService::execute` — `email` must match
/// an already-minted `AuthSessionRecord`'s user for the member to land on that SAME user rather
/// than a freshly-created one (`decide`'s `UpsertMember` resolves-or-creates by email).
pub async fn upsert_member_for_test(state: &HubState, space_id: &str, email: &str, role: DirectorySpaceRole) {
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".to_string() };
    state.directory_service.execute(actor, DirectoryCommand::UpsertMember { space_id: space_id.to_string(), email: email.to_string(), role }).await.expect("upsert member");
}

/// 🪢 The synthetic document's owning app `Dialect`, in the plugin id space, deliberately spelled
/// differently from its manifest `artifact_kind` (`test.artifact`, the taxonomy space) — the shape
/// every plugin but `gis` really has. It is owned by `document_descriptor_for_test`'s
/// `owner.plugin_id`, which is what `document_index_projection_v1` binds the index entry to.
pub const TEST_ARTIFACT_PARENT_DIALECT_KIND: &str = "s.test.artifact";

pub fn document_descriptor_for_test(space_id: &str, document_id: &str) -> os_directory::DocumentDescriptor {
    let bootstrap_snapshot_hash = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(b"document-open-genesis-pack"));
    os_directory::DocumentDescriptor {
        space_id: space_id.to_string(),
        document_id: document_id.to_string(),
        artifact_kind: "test.artifact".into(),
        artifact_schema: "test.v1".into(),
        owner: os_directory::DocumentOwner { plugin_id: "test".into(), package_id: "semio:test".into(), version: "1.0.0".into(), package_hash: "22".repeat(32) },
        pack_schema_hash: "11".repeat(32),
        bootstrap_version: 1,
        bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash,
    }
}

pub fn artifact_document_id_for_test(label: &str) -> String {
    format!("artifact-{}", &os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(label.as_bytes()))[..32])
}

pub async fn announce_document_for_test(state: &HubState, space_id: &str, document_id: &str) {
    let actor = DirectoryActor { kind: DirectoryActorKind::User, id: "user:seed#test".into() };
    state.directory_service.execute(actor, DirectoryCommand::AnnounceDocument { descriptor: Box::new(document_descriptor_for_test(space_id, document_id)) }).await.expect("announce document");
}

/// 🧱️ Publishes the next ORDINARY checkpoint on a document's already-committed lineage.
/// `decide_verified_checkpoint` (`📇️directory/🦀️.rs`) refuses a parentless ordinary publication with
/// `Conflict("ordinary artifact publication requires a committed genesis parent")` — a document's first
/// checkpoint is a creation genesis and only `publish_document_genesis` mints one — so a law seeds the
/// document with `seed_genesis_document_for_test` (or `publish_openable_document_for_test`) and this
/// helper chains on whatever head that left, strictly advancing the baseline frontier.
pub async fn publish_checkpoint_for_test(state: &HubState, space_id: &str, document_id: &str) -> os_directory::ArtifactCheckpoint {
    let pack = b"verified-pack";
    let spr = b"verified-spr";
    let pack_hash = os_directory::ArtifactHash(Sha256::digest(pack));
    let spr_hash = os_directory::ArtifactHash(Sha256::digest(spr));
    let accepted_at_ms = u64::try_from(now_ms()).expect("nonnegative genesis fixture clock");
    let mut aggregate = Sha256::new();
    aggregate.update(pack);
    aggregate.update(spr);
    let scope = DocumentScope::new(space_id, document_id);
    let parent = state.directory.get_active_artifact_checkpoint(&scope).await.expect("active lineage read").expect("committed genesis parent before an ordinary publication");
    let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor read").expect("descriptor");
    let pack_plan = prepare_artifact_cas_manifest_v1(space_id, pack).expect("pack manifest plan");
    let spr_plan = prepare_artifact_cas_manifest_v1(space_id, spr).expect("SPR manifest plan");
    let mut checkpoint = os_directory::ArtifactCheckpoint {
        scope,
        checkpoint_id: os_directory::ArtifactHash([0; 32]),
        parent_checkpoint_id: Some(parent.checkpoint_id),
        descriptor_digest_v1: os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
        baseline_frontier: os_directory::ArtifactFrontier {
            document_id: document_id.to_string(),
            head_edit_ordinal: parent.baseline_frontier.head_edit_ordinal + 1,
            head_edit_id: format!("verified-edit-{}", parent.baseline_frontier.head_edit_ordinal + 1),
            last_commit_seq: parent.baseline_frontier.last_commit_seq + 1,
            chain_hash: os_directory::ArtifactHash([0x44; 32]),
        },
        pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: artifact_cas_manifest_locator_v1(pack_plan.manifest_id) },
        spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: artifact_cas_manifest_locator_v1(spr_plan.manifest_id) },
        aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
        published_at_ms: accepted_at_ms,
    };
    checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("checkpoint identity")));
    let ownership = prepare_artifact_cas_ownership_v1(&checkpoint, &ArtifactPair { pack: pack.to_vec(), spr: spr.to_vec() }).expect("ownership plan");
    let reservation = state.directory_service.reserve_artifact_cas(DirectoryActor { kind: DirectoryActorKind::System, id: "system:lag-rebootstrap-test".into() }, ownership, 1_000, 100).await.expect("reserve checkpoint objects");
    let cas = ArtifactChunkBlobStore::new(state.artifact_cas.clone());
    let authority_control = StartupCatalogControl::silent();
    let authority_context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &authority_control);
    let staged_pack = cas.stage(space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &authority_context).await.expect("stage reserved pack manifest");
    let staged_spr = cas.stage(space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &authority_context).await.expect("stage reserved SPR manifest");
    assert_eq!(staged_pack.storage_key, checkpoint.pack.storage_key);
    assert_eq!(staged_spr.storage_key, checkpoint.spr.storage_key);
    state.directory_service.publish_reserved_artifact_checkpoint(DirectoryActor { kind: DirectoryActorKind::System, id: "system:lag-rebootstrap-test".into() }, checkpoint.clone(), reservation, 100).await.expect("publish verified checkpoint");
    checkpoint
}

/// 🌱️ Publishes one creation genesis exactly the way production does, in the two forms
/// production keeps apart. The **prepared candidate** carries `sha256/<hex>` storage keys — that is
/// what `blob_reference` (`🗿️artifact-authority/🦀️.rs:516`) mints and what
/// `ArtifactCreationPreparedV1::validate` requires — and its `checkpoint_id` is computed over them.
/// Only after staging does the chunk CAS's manifest locator replace those keys, in place and without
/// recomputing the identity (`🗿️artifact-authority/🦀️.rs:503-505`). Stamping the locator before
/// validation, as this fixture used to, is what made every caller fail
/// `Conflict("artifact creation prepared pair differs from its accepted intent")`.
pub async fn publish_genesis_checkpoint_for_test(
    state: &HubState,
    actor: ArtifactCreationActorV1,
    catalog_generation: String,
    parent_dialect: semio_framework_artifact_reference::ArtifactDialect,
    descriptor: DocumentDescriptor,
    pack: &[u8],
    spr: &[u8],
) -> os_directory::ArtifactCheckpoint {
    use semio_hub_artifact_creation::{ArtifactCreationClaimV1, ArtifactCreationFactAppendV1, ArtifactCreationFactBodyV1, ArtifactCreationIntentV1, ArtifactCreationPreparedV1, ARTIFACT_CREATION_DEADLINE_MS};
    let accepted_at_ms = u64::try_from(now_ms()).expect("nonnegative genesis creation clock");
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
        descriptor_digest_v1: os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&descriptor).expect("genesis descriptor digest"),
        baseline_frontier: os_directory::ArtifactFrontier { document_id: scope.document_id.clone(), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: os_directory::ArtifactHash([0; 32]) },
        pack: os_directory::ArtifactBlobRef { sha256: pack_hash, byte_length: pack.len() as u64, storage_key: format!("sha256/{}", directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&pack_hash)) },
        spr: os_directory::ArtifactBlobRef { sha256: spr_hash, byte_length: spr.len() as u64, storage_key: format!("sha256/{}", directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&spr_hash)) },
        aggregate_sha256: os_directory::ArtifactHash(aggregate.finalize()),
        published_at_ms: accepted_at_ms,
    };
    checkpoint.checkpoint_id = os_directory::ArtifactHash(Sha256::digest(&checkpoint_id_encoding_v1(&checkpoint).expect("genesis checkpoint identity")));
    let request = SpaceArtifactCreateV1 {
        schema: "semio.hub.space-artifact-create/v1".into(),
        request_id: scope.document_id.strip_prefix("artifact-").expect("creation-owned document id").into(),
        expected_catalog_generation_id: catalog_generation.clone(),
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
    semio_hub_artifact_creation::io::validate_artifact_creation_prepared_v1(&prepared, &intent).expect("exact prepared publication genesis");
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
    let control = StartupCatalogControl::silent();
    let context = OperationContext::new(u64::MAX, AuthorityLimits::maximum(), &control);
    let staged_pack = cas.stage(&scope.space_id, ArtifactBlobIntegrity { sha256: pack_hash, byte_length: pack.len() as u64 }, pack, &context).await.expect("stage genesis pack");
    let staged_spr = cas.stage(&scope.space_id, ArtifactBlobIntegrity { sha256: spr_hash, byte_length: spr.len() as u64 }, spr, &context).await.expect("stage genesis SPR");
    assert_eq!(staged_pack.storage_key, artifact_cas_manifest_locator_v1(pack_plan.manifest_id), "staged genesis pack keeps the reserved manifest locator");
    assert_eq!(staged_spr.storage_key, artifact_cas_manifest_locator_v1(spr_plan.manifest_id), "staged genesis SPR keeps the reserved manifest locator");
    let mut published = checkpoint.clone();
    published.pack.storage_key = staged_pack.storage_key;
    published.spr.storage_key = staged_spr.storage_key;
    let committed_at_ms = u64::try_from(now_ms()).expect("nonnegative genesis commit clock");
    state.directory_service.publish_document_genesis(intent, &prepared, published.clone(), reservation, committed_at_ms).await.expect("publish dedicated creation genesis");
    published
}

/// 🌱️ Seeds one creation-owned document through the production authority: a real session minted by
/// `issue_test_session`, made an author of the space exactly as a person is, then `publish_document_genesis`
/// — the same path `AU1`/`AU3` drive. It replaces `announce_document_for_test` wherever a law then publishes
/// an ordinary checkpoint: the genesis writes the descriptor itself, and `append_document_genesis` refuses a
/// scope whose descriptor is already publicly occupied, so the two cannot be combined. The returned id is
/// creation-owned (`artifact-<hex>`), which is what `ArtifactCreationIntentV1`'s request id is derived from.
pub async fn seed_genesis_document_for_test(state: &HubState, author_email: &str, space_id: &str, label: &str) -> (String, os_directory::ArtifactCheckpoint) {
    let session = issue_test_session(state, author_email).await;
    upsert_member_for_test(state, space_id, author_email, DirectorySpaceRole::Author).await;
    let document_id = artifact_document_id_for_test(label);
    let genesis = seed_genesis_for_document_for_test(state, &session.token, space_id, &document_id, label).await;
    (document_id, genesis)
}

/// 🌱️ The same seed for a law that already holds an author's session and a document id it must keep:
/// the id has to be creation-owned (`artifact-<32 hex>`) because `ArtifactCreationIntentV1`'s request id is
/// that suffix, and `SpaceArtifactCreationStatusV1::validate` bounds it.
pub async fn seed_genesis_for_document_for_test(state: &HubState, token: &str, space_id: &str, document_id: &str, label: &str) -> os_directory::ArtifactCheckpoint {
    let pack = format!("genesis-pack-{label}").into_bytes();
    let spr = format!("genesis-spr-{label}").into_bytes();
    let mut descriptor = document_descriptor_for_test(space_id, document_id);
    descriptor.bootstrap_snapshot_hash = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&pack));
    let authenticated = state
        .directory
        .authenticate_session(&SessionCapability::parse(token).expect("genesis seed capability"))
        .await
        .expect("genesis seed session read")
        .expect("genesis seed session");
    let actor = ArtifactCreationActorV1 { user_id: authenticated.user_id, session_id: authenticated.id, authorization_generation: authenticated.authorization_generation };
    let parent_dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() };
    publish_genesis_checkpoint_for_test(state, actor, "66".repeat(32), parent_dialect, descriptor, &pack, &spr).await
}

pub async fn sample_envelope(id: &str, document: &WireArtifactId) -> MutationEnvelope {
    MutationEnvelope {
        mutation_id: protocol::MutationId(id.to_string()),
        document_id: document.clone(),
        actor: ActorId("actor-1".to_string()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({ "value": id })).await.unwrap() },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId(db::document::DB_PATHMAP_SCHEMA.to_string()), payload: db::document::encode_pathmap_json(&serde_json::json!({})).await.unwrap() },
        timestamp: protocol::HybridLogicalTimestamp::new(0, 0),
        transaction: None, verb: None, line: None,
    }
}

pub async fn spawn_server(state: HubState) -> SocketAddr {
    spawn_server_with_posture(state, CrossOriginPolicyV1::LoopbackDevelopment, ForwardedTlsTrustV1::Untrusted).await
}

pub async fn spawn_server_with_posture(state: HubState, cross_origin: CrossOriginPolicyV1, forwarded_tls: ForwardedTlsTrustV1) -> SocketAddr {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = router(state, cross_origin, forwarded_tls).into_make_service_with_connect_info::<SocketAddr>();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    addr
}

pub async fn spawn_restartable_server(state: HubState) -> (SocketAddr, tokio::sync::oneshot::Sender<()>, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = router(state, CrossOriginPolicyV1::LoopbackDevelopment, ForwardedTlsTrustV1::Untrusted).into_make_service_with_connect_info::<SocketAddr>();
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
pub struct RawHttpResponse {
    pub status: u16,
    pub headers: String,
    pub body: Vec<u8>,
}

pub async fn raw_http_request_transport(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> Option<RawHttpResponse> {
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
    let read = tokio::time::timeout(RAW_HTTP_READ_HANG_GUARD, async {
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

pub async fn raw_http_request(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> RawHttpResponse {
    raw_http_request_transport(addr, method, path, headers, body).await.expect("HTTP response")
}

pub async fn raw_http_get(addr: SocketAddr, path: &str, headers: &[(&str, &str)]) -> RawHttpResponse {
    raw_http_request(addr, "GET", path, headers, &[]).await
}

/// 🚧️ One hostile request on its own connection. Unlike [`raw_http_request`] it tolerates the hub
/// closing a request it refused early (an oversized body), and answers `Err` only when the hub never
/// answered — a dropped connection is a finding, not a transport detail.
pub async fn hostile_http_request(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &[u8]) -> Result<RawHttpResponse, String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = tokio::net::TcpStream::connect(addr).await.map_err(|error| format!("connect: {error}"))?;
    let mut request = format!("{method} {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\nContent-Length: {}\r\n", body.len());
    for (name, value) in headers {
        request.push_str(&format!("{name}: {value}\r\n"));
    }
    request.push_str("\r\n");
    let written = async {
        stream.write_all(request.as_bytes()).await?;
        stream.write_all(body).await?;
        stream.flush().await
    }
    .await;
    let mut response = Vec::new();
    let read = tokio::time::timeout(RAW_HTTP_READ_HANG_GUARD, stream.read_to_end(&mut response)).await.map_err(|_| format!("{method} {path} never answered"))?;
    if response.is_empty() {
        return Err(format!("{method} {path} dropped the connection (write {written:?}, read {read:?})"));
    }
    let boundary = response.windows(4).position(|bytes| bytes == b"\r\n\r\n").ok_or_else(|| format!("{method} {path} answered no HTTP head"))?;
    let head = String::from_utf8_lossy(&response[..boundary]).to_string();
    let status = head.split_whitespace().nth(1).and_then(|status| status.parse().ok()).ok_or_else(|| format!("{method} {path} answered no status"))?;
    let raw = response[boundary + 4..].to_vec();
    let body = if head.to_ascii_lowercase().contains("transfer-encoding: chunked") { dechunk_http_body(&raw) } else { raw };
    Ok(RawHttpResponse { status, headers: head, body })
}

fn dechunk_http_body(raw: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    let mut rest = raw;
    while let Some(line_end) = rest.windows(2).position(|bytes| bytes == b"\r\n") {
        let size = usize::from_str_radix(std::str::from_utf8(&rest[..line_end]).unwrap_or("0").trim(), 16).unwrap_or(0);
        if size == 0 || rest.len() < line_end + 2 + size {
            break;
        }
        body.extend_from_slice(&rest[line_end + 2..line_end + 2 + size]);
        rest = &rest[(line_end + 4 + size).min(rest.len())..];
    }
    body
}

/// 🧪️ A loopback `ConnectInfo` for handlers called directly in tests. Network
/// proximity confers no authorization; every protected test also supplies a verified session.
pub fn loopback_peer() -> axum::extract::ConnectInfo<SocketAddr> {
    axum::extract::ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 0)))
}

pub async fn next_server_frame<S>(ws: &mut S) -> ServerFrame
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    next_server_frame_at(ws, "unnamed").await
}

/// 🏷️ The same wait, with the caller's own phase in every refusal. One helper serves 49 call sites
/// across this suite, so an unnamed `no server frame before 5s deadline` names neither the law's
/// line nor which of its frames is missing — HT13's whole first hour went into recovering that by
/// arithmetic. A law that waits on more than one frame passes a phase here so its capture says which.
pub async fn next_server_frame_at<S>(ws: &mut S, phase: &str) -> ServerFrame
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    next_server_frame_within(ws, phase, std::time::Duration::from_secs(5)).await
}

/// ⏳️ [`next_server_frame_at`] with the law's own bound, for a frame a loaded host may take longer to answer.
pub async fn next_server_frame_within<S>(ws: &mut S, phase: &str, bound: std::time::Duration) -> ServerFrame
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + bound;
    loop {
        match tokio::time::timeout_at(deadline, ws.next()).await {
            Ok(Some(Ok(WsMessage::Binary(bytes)))) => return protocol::decode_server_frame(&bytes).await.unwrap_or_else(|error| panic!("server frame at {phase}: {error:?}")).1,
            Ok(Some(Ok(WsMessage::Close(frame)))) => panic!("the server closed before its next frame at {phase}: {frame:?} plan refusal {:?}", last_document_plan_refusal()),
            Ok(Some(Ok(_))) => continue,
            Ok(Some(other)) => panic!("expected binary frame at {phase}, got {other:?}"),
            Ok(None) => panic!("stream ended before server frame at {phase}"),
            Err(_) => panic!("no server frame within {bound:?} at {phase}"),
        }
    }
}

pub async fn next_directory_message<S>(ws: &mut S) -> DirectoryStreamMessage
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        match tokio::time::timeout_at(deadline, ws.next()).await {
            Ok(Some(Ok(WsMessage::Text(text)))) => return semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("directory message"),
            Ok(Some(Ok(WsMessage::Close(frame)))) => panic!("the server closed before its next directory message: {frame:?}"),
            Ok(Some(Ok(_))) => continue,
            Ok(Some(other)) => panic!("expected directory message, got {other:?}"),
            Ok(None) => panic!("stream ended before directory message"),
            Err(_) => panic!("no directory message before 5s deadline"),
        }
    }
}

pub async fn client_binary(frame: &ClientFrame, lane: Lane) -> WsMessage {
    WsMessage::Binary(protocol::encode_client_frame(frame, lane).await.into())
}

pub async fn next_close_code<S>(ws: &mut S, allow_text: bool) -> u16
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

pub async fn next_close_without_authority<S>(ws: &mut S) -> u16
where
    S: StreamExt<Item = Result<WsMessage, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    match tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, ws.next()).await {
        Ok(Some(Ok(WsMessage::Close(Some(frame))))) => frame.code.into(),
        Ok(Some(Ok(WsMessage::Binary(bytes)))) => panic!("authority-bearing binary frame crossed revocation: {:?}", protocol::decode_server_frame(&bytes).await.map(|decoded| decoded.1)),
        Ok(Some(Ok(WsMessage::Text(_)))) => panic!("authority-bearing directory frame crossed revocation"),
        Ok(Some(other)) => panic!("expected close after revocation, got {other:?}"),
        Ok(None) => panic!("stream ended before revocation close"),
        Err(_) => panic!("the revocation close never arrived: the server closed no socket for this revoke"),
    }
}

pub fn socket_hello() -> ClientFrame {
    ClientFrame::SocketHelloV1 { wire_version: 1, protocol_version: 1, schema: "test.v1".to_string(), pack_schema_hash: [0x11; 32], resume_token: None, frontier: None }
}

pub fn bearer_headers(capability: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {capability}").parse().expect("bearer header"));
    headers
}

pub fn socket_request(url: &str, grant: &str) -> tokio_tungstenite::tungstenite::http::Request<()> {
    let mut request = url.into_client_request().expect("socket request");
    request.headers_mut().insert(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SOCKET_PROTOCOL_V1}, {grant}").parse().expect("socket protocols"));
    request
}

pub fn document_socket_request(url: &str, session: &str) -> tokio_tungstenite::tungstenite::http::Request<()> {
    let mut request = url.into_client_request().expect("document socket request");
    request.headers_mut().insert(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL, format!("{SESSION_PROTOCOL_V1}, {session}").parse().expect("session protocol"));
    request
}

pub struct TestIssuedSession {
    pub token: String,
    pub user_id: String,
}

pub async fn issue_test_session(state: &HubState, email: &str) -> TestIssuedSession {
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

pub async fn authorize_test_admin(state: &mut HubState, email: &str) -> HeaderMap {
    let session = issue_test_session(state, email).await;
    state.admin_subjects = Arc::from([AdminSubject { provider_digest: admin_provider_digest("test-verifier"), subject_digest: identity_subject_digest("test-verifier", email).expect("test admin subject digest") }]);
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.token).parse().expect("test bearer header"));
    headers
}

pub async fn seed_author_token(state: &HubState) -> String {
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

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentOpenPlanLedgerFixture {
    pub now_ms: u64,
    pub descriptor: DocumentDescriptor,
    pub descriptor_digest_v1: String,
    pub valid_plan: DocumentOpenPlanV1,
}

pub fn document_open_plan_test_authority(fixture: &DocumentOpenPlanLedgerFixture) -> DocumentOpenPlanAuthorityV1 {
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
            session_kind: AuthSessionKind::External,
            device_instance_id: "open-plan-device".into(),
        },
        server_actor_id: "hub.v1.open-plan-actor".into(),
        client_instance_id_digest: [9; 32],
    }
}

pub const TEST_EXECUTION_TARGET_COMPONENT_BYTES: &[u8] = b"\0asm\x01\0\0\0semio-execution-target-lease-component";

pub const TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES: &[u8] = b"semio-execution-target-lease-descriptor";

pub fn document_open_catalog_for_descriptor(descriptor: &DocumentDescriptor) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
    document_open_catalog_for_descriptor_with_generation(descriptor, "66".repeat(32))
}

pub fn install_document_open_catalog_for_test(state: &mut HubState, descriptor: &DocumentDescriptor) {
    state.openable_catalog = Some(document_open_catalog_for_descriptor(descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
}

pub fn document_open_catalog_for_descriptor_with_generation(descriptor: &DocumentDescriptor, generation_id: String) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
    let package = DocumentOpenPackageV1 {
        plugin_id: descriptor.owner.plugin_id.clone(),
        package_id: descriptor.owner.package_id.clone(),
        version: descriptor.owner.version.clone(),
        component_sha256: descriptor.owner.package_hash.clone(),
        component_blake3: "44".repeat(32),
        descriptor_byte_sha256: "55".repeat(32),
        execution_protocol: os_directory::DocumentExecutionProtocolV1 { app_channel_version: directory::os_spr::CHANNEL_VERSION },
        component_receipt: directory::os_component_receipt::ComponentReceiptV1 {
            schema: directory::os_component_receipt::COMPONENT_RECEIPT_SCHEMA.into(),
            plugin_id: descriptor.owner.plugin_id.clone(),
            package_id: descriptor.owner.package_id.clone(),
            profile: directory::os_component_receipt::ComponentProfileV1::Release,
            target: directory::os_component_receipt::COMPONENT_TARGET.into(),
            wasm_sha256: descriptor.owner.package_hash.clone(),
            wasm_bytes: TEST_EXECUTION_TARGET_COMPONENT_BYTES.len() as u64,
            semantic_descriptor_sha256: "55".repeat(32),
            toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)".into(),
            source_tree_digest: "3".repeat(64),
            provenance_sha256: "4".repeat(64),
        },
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
                parent_dialect: semio_framework::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() },
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
                parent_dialect: semio_framework::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() },
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

pub async fn issue_and_exchange_document_open_plan_for_test(state: &HubState, token: &str, scope: &DocumentScope, client_instance_id: &str) -> (DocumentOpenPlanV1, DocumentSocketGrantReceiptV1) {
    let mut headers = bearer_headers(token);
    headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
    let intent = DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: Some("surface.test.editor".into()), client_instance_id: client_instance_id.into() };
    let DirectoryJson(plan) = match issue_document_open_plan_inner(scope.space_id.clone(), scope.document_id.clone(), headers.clone(), state.clone(), Bytes::from(semio_framework_pack_json::to_json_string(&intent))).await {
        Ok(plan) => plan,
        Err((status, DirectoryJson(error))) => panic!("issue document open plan: {status} {:?}", error.code),
    };
    let exchange = DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: plan.receipt.clone() };
    let Json(grant) = match issue_document_plan_socket_grant_inner(scope.space_id.clone(), scope.document_id.clone(), headers, state.clone(), Bytes::from(semio_framework_pack_json::to_json_string(&exchange))).await {
        Ok(grant) => grant,
        Err((status, DirectoryJson(error))) => panic!("exchange document open plan: {status} {:?}", error.code),
    };
    (plan, grant)
}

pub async fn stop_recovery_server(state: HubState, shutdown: tokio::sync::oneshot::Sender<()>, task: tokio::task::JoinHandle<()>) {
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

pub fn presence_hex_bytes(hex: &str) -> Vec<u8> {
    assert_eq!(hex.len() % 2, 0);
    (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).expect("presence fixture byte")).collect()
}

pub fn presence_normalization_fixture() -> serde_json::Value {
    serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json")).expect("presence normalization fixture")
}

fn directory_command_body(request_id: &str, command: DirectoryCommand) -> Vec<u8> {
    DirectoryCommandRequestV1::new(request_id, command).canonical_json().into_bytes()
}

pub async fn post_directory_command_for_test(addr: SocketAddr, token: &str, request_id: &str, command: DirectoryCommand) -> RawHttpResponse {
    let authorization = format!("Bearer {token}");
    raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], &directory_command_body(request_id, command)).await
}

//#region 🔖️CredentialSignIn
pub fn json_body(response: &RawHttpResponse) -> serde_json::Value {
    serde_json::from_slice(&response.body).expect("JSON body")
}
//#endregion 🔖️CredentialSignIn
