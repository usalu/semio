//! 🏗️ Test states: observed, capacity-bounded, directory-backed and lag-ready hub states.

use super::*;

pub async fn test_state() -> HubState {
    test_state_with_capacity(1024, 256).await
}

/// 🗄️ The instance a test's `HubState` carries: the real durable profile, under the test's own
/// temporary root, so a law that reopens the same root observes exactly what a hub restart does.
pub async fn test_instance_state(dir: &std::path::Path) -> ServerState<HubInstance> {
    instance_state(dir).await.expect("open hub instance stores")
}

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
