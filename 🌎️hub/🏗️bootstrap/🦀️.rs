//! OS hub backend — thin axum shell over `db::Database` (document authority + content-addressed
//! blobs) and the hub directory capability traits (identity/tenancy), speaking `protocol_wire`'s binary frames over
//! WebSocket.
//!
//! The WebSocket endpoint speaks `protocol_wire`'s binary lane-tagged `ClientFrame`/`ServerFrame`
//! frames directly (see `protocol/wire/rs/lib.rs`) — the server-side counterpart to
//! `framework/sync`'s client actors (CW5). Command-lane persistence/ordering flows through
//! `db::Database::hello`/`ArtifactHandle::submit`/`db::sync::handle_frontier_advertise`;
//! preview-lane and presence frames are ephemeral, best-effort fan-out this crate owns directly via
//! a per-document `tokio::sync::broadcast` registry (never durable, matching the preview lane's
//! contract). "Space" is a namespacing convention this crate applies on top of `db`'s flat document
//! catalog (`{space_id}:{document_id}`), not hub-internal state.

use futures::{SinkExt, StreamExt};
use semio_framework_async::ShardedMap;
use semio_framework_trace::record::{TraceOutcome, TraceRecord, Tracer};
use semio_hub_artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStorage};
use semio_hub_artifact_creation_service::ArtifactCreationServiceV1;
use semio_hub_native_openable::NativeCodecProviderSetV1;
use semio_hub_trusted_catalog_descriptor::NativeCodecProviderSourceV1;
use semio_hub_trusted_catalog_contract::schema::{TrustedCatalogGuestResidencyV1, GUEST_RESIDENCY_BYTES_ENV};
use semio_hub_artifact_checkin::DocumentCheckInJobs;
use semio_hub_artifact_authority::{AuthorityError, AuthorityLimits, AuthorityOperationControl, CheckpointPublicationOrchestrator, OperationContext};
use semio_hub_auth::rate_limit::HubRateLimiterV1;
use semio_hub_auth::CredentialSignInPolicyV1;
use semio_hub_directory_service::{DirectoryService, HubVerifiedCheckpointPublisher};
use semio_hub_directory::{AgentDelegationDirectory, ArtifactCasDirectory, ProjectionRebuildControl, SyncSessionDirectory};
use semio_hub_directory_contract::{IdentityAssertionVerifier, IdentityVerificationControl, LocalBootstrapTransport, SessionCapability};
#[cfg(feature="integration-fixtures")]
use semio_hub_inference_runtime::InferenceCheckpointTestGateV1;
use semio_hub_inference_runtime_contract::GisMapApprovalCheckpointPublisherV1;
use semio_hub_inference_runtime::HubInferenceRuntimeV1;
use semio_hub_inference_committer::RetainedGisMapApprovalCommitterV1;
use semio_hub_inference_ledger::InferenceJobLedgerV1;
use semio_hub_inference_catalog::verified_gis_map_binding;
use semio_hub_inference_binding::MapInferenceProposalBindingV1;
use semio_hub_gis::inference::GisMapInferenceBindingV1;
use semio_hub_lag_rebootstrap::{RebootstrapTransferControl, VerifiedRebootstrapSource};
use semio_hub_local_bootstrap::{serve_local_bootstrap, InheritedLocalBootstrapTransport, LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS};
use std::net::SocketAddr;
use std::sync::Arc;
use db::storage::PayloadStorage as _;
use server::storage::SessionStore as _;
use semio_hub_route_models::admin::{ADMIN_OPERATION_SHUTDOWN_DEADLINE, AdminOperationTaskOwner};
use semio_hub_runtime::readiness::{AGENT_DELEGATION_READINESS_PROBE_SCOPE, BootReadinessServerV1, CHECKPOINT_POLICY_BYTES_ENV, CHECKPOINT_POLICY_EDITS_ENV, HubCheckpointPolicyV1, HubMode, declare_public_session_issuance, hub_readiness, hub_starting_readiness, readiness_trace_detail, startup_readiness_line, with_checkpoint_policy};
use semio_hub_runtime::startup::{ArtifactCasMaintenanceSupervisor, DocumentOpenCatalogAuthorityV1, HubBootstrapControl, HubError, SAGA_DRAIN_INTERVAL, SagaDrainSupervisor, StartupArtifactAuthority, StartupCancellationV1, StartupCatalogControl, StartupProgressCellV1, TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS, artifact_authority_closed_reason, artifact_cas_sweep_execute_from_env, compose_hub_server, configured_artifact_authority, merge_policy_from_env, verify_pending_catalog_guests};
use semio_hub_route_models::artifact_creation::{ArtifactCreationHttpTaskOwnerV1, HubArtifactCreationCommitAuthorityV1};
use semio_hub_runtime::transport::{CrossOriginPolicyV1, ForwardedTlsTrustV1};
use semio_hub_runtime::lifecycle::{DATABASE_SHUTDOWN_DEADLINE, HUB_RUNTIME_THREAD_STACK_BYTES, HubSocketDrainV1, SOCKET_DRAIN_DEADLINE, close_hub_database, connect_artifact_cas, connect_db, connect_directory};
use semio_hub_ledgers::open_plan::DocumentOpenPlanLedgerV1;
use semio_hub_route_models::check_in::GisMapApprovalCheckpointPublisherV1Impl;
use semio_hub_state::HubState;
use semio_hub_runtime::presence::PresenceDirectoryProjections;
use semio_hub_ledgers::socket_grants::{SocketBindingGatesV1, SocketGrantLedgerV1};
use semio_hub_ledgers::guard::{configured_admin_subjects, validate_auth_startup};
use semio_hub_runtime::support::now_ms;
use semio_hub_routes::router;
#[cfg(unix)]
use semio_hub_runtime::lifecycle::termination_signal;

//#region 🔖️Main
fn main() -> Result<(), HubError> {
    tokio::runtime::Builder::new_multi_thread().enable_all().thread_stack_size(HUB_RUNTIME_THREAD_STACK_BYTES).build().map_err(HubError::Io)?.block_on(serve())
}

/// 🚪️ The hub process: CLI verbs first, then the configured server until a termination signal.
///
/// 🔐️ Read before the startup gate rather than next to its first use: whether the hub's own
/// credential authority is enabled is *the* thing that decides whether production may boot, and
/// `CredentialSignInPolicyV1::from_env` reads nothing but the environment.
///
/// 🪪️ The external-IdP seam. No adapter ships in this repository; production reaches its identity
/// requirement through `credential_sign_in` instead — see `validate_auth_startup`.
///
/// 📝️ Observability is configured before the first subsystem that reports, so the trusted-catalog
/// load, the artifact-CAS sweep and the creation-recovery loop all report onto the same tracer the
/// routes later use — a tracer built after them would have silently lost every boot record.
///
/// 🧹️ Contract §C0: clear crash residue before any real connection lands — a session that never
/// got its `disconnected_at` because a previous process was killed mid-connection.
///
/// ⏳️ The artifact-CAS coordinator handshake is startup work no client is waiting on, so it takes
/// the same no-progress bound as the catalog load above rather than a second wall-clock budget.
///
/// 🤖️ One bounded read of the real method, never a declared capability flag beside it: a backend
/// that carries the family's erroring default answers `Err(Backend)` here and a backend that
/// implements it answers an empty page, so `features.mcpWorkspace` cannot drift from what
/// `POST /auth/agent-sessions` would actually do on the next request.
///
/// 🗄️ Hub as instance #1 of the server product. Its four roles open under `{OS_HUB_DATA}/instance`
/// — inside the same root the directory, the artifact CAS and the extension mirror live in, so one
/// backup or one `rm -rf` covers the whole hub and no second data location has to be documented.
/// Auth + directory modules are registered on the framework gateway here; hub's own router, which
/// owns the document socket, merges with that router below.
///
/// 🗣️ The startup banner is NOT a log line and is deliberately not routed through the tracer: it
/// is this process's announcement of its own readiness, its exact text is pinned by
/// `the_startup_line_names_the_bound_address_when_ready` and read by the collaboration harness,
/// and it must survive `SEMIO_TRACE_SINK=none`. The same fact is additionally reported as a
/// structured `server.readiness` record, so a collector sees it without parsing prose.
///
/// 🚰️ Last, and after the router is already refusing connections: a final pass over whatever the
/// in-flight requests committed on their way out, so nothing is left acknowledged-but-undelivered
/// in the outbox across the restart.
async fn serve() -> Result<(), HubError> {
    let arguments = std::env::args_os().skip(1).collect::<Vec<_>>();
    if credential_command::dispatch(&arguments).await? {
        return Ok(());
    }
    if trusted_catalog_command::dispatch(&arguments).await? {
        return Ok(());
    }
    let port: u16 = std::env::var("OS_HUB_PORT").ok().and_then(|value| value.parse().ok()).unwrap_or(8787);
    let bind: std::net::IpAddr = std::env::var("OS_HUB_BIND").unwrap_or_else(|_| "0.0.0.0".into()).parse().map_err(|_| HubError::UnsafeAuthConfiguration("OS_HUB_BIND must be an IP address".into()))?;
    let mode = HubMode::from_environment(bind)?;
    let cross_origin = CrossOriginPolicyV1::from_environment(bind)?;
    let forwarded_tls = ForwardedTlsTrustV1::from_environment()?;
    let guest_residency = TrustedCatalogGuestResidencyV1::configured(std::env::var(GUEST_RESIDENCY_BYTES_ENV).ok().as_deref()).map_err(|detail| HubError::ArtifactAuthority(AuthorityError::Catalog(detail)))?;
    let checkpoint_policy = HubCheckpointPolicyV1::configured(std::env::var(CHECKPOINT_POLICY_EDITS_ENV).ok().as_deref(), std::env::var(CHECKPOINT_POLICY_BYTES_ENV).ok().as_deref()).map_err(|detail| HubError::ArtifactAuthority(AuthorityError::Catalog(detail)))?;
    let credential_sign_in = CredentialSignInPolicyV1::from_env().map_err(|error| HubError::UnsafeAuthConfiguration(error.to_string()))?;
    let identity_verifier: Option<Arc<dyn IdentityAssertionVerifier>> = None;
    let bootstrap_control = Arc::new(HubBootstrapControl::new());
    let local_bootstrap: Option<Arc<dyn LocalBootstrapTransport>> = if mode == HubMode::Development {
        let deadline_ms = now_ms().checked_add(LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS).ok_or_else(|| HubError::UnsafeAuthConfiguration("local bootstrap deadline overflow".into()))?;
        let context = semio_hub_directory_contract::IdentityVerificationContext { deadline_ms, control: bootstrap_control.as_ref() };
        Some(InheritedLocalBootstrapTransport::open_inherited(&context, bootstrap_control.clone()).await?)
    } else {
        None
    };
    let admin_subjects = configured_admin_subjects()?;
    validate_auth_startup(mode, bind, identity_verifier.as_ref(), local_bootstrap.as_ref(), &admin_subjects, credential_sign_in.is_enabled(), &cross_origin, forwarded_tls)?;
    let data_dir = std::env::var("OS_HUB_DATA").map_or_else(|_| std::path::PathBuf::from("./.🧬semio/🌐hub/"), std::path::PathBuf::from);
    let native_codec_providers = NativeCodecProviderSetV1::linked();
    let native_codec_provider: Option<&dyn NativeCodecProviderSourceV1> = Some(&native_codec_providers);
    let tracer = Tracer::from_environment();
    let startup_cancellation = StartupCancellationV1::default();
    let addr = SocketAddr::new(bind, port);
    let bound = std::net::TcpListener::bind(addr)?;
    bound.set_nonblocking(true)?;
    let bind_scope = if bind.is_loopback() { "loopback" } else { "network" };
    let run_id = local_bootstrap.as_ref().map_or_else(|| "production".to_string(), |bootstrap| bootstrap.run_id().to_string());
    let bootstrap_ready = match mode {
        HubMode::Development => local_bootstrap.as_ref().is_some_and(|bootstrap| bootstrap.is_ready()),
        HubMode::Production => identity_verifier.is_some() || credential_sign_in.is_enabled(),
    };
    let artifact_cas_sweep_execute = artifact_cas_sweep_execute_from_env()?;
    let startup_progress = StartupProgressCellV1::default();
    let boot_server = BootReadinessServerV1::start(
        tokio::net::TcpListener::from_std(bound.try_clone()?)?,
        hub_starting_readiness(mode, bind_scope, run_id.clone(), bootstrap_ready, credential_sign_in.is_enabled(), artifact_cas_sweep_execute),
        startup_progress.clone(),
        tracer.clone(),
        cross_origin.clone(),
        forwarded_tls,
    );
    let mut starting = TraceRecord::new("server.readiness", TraceOutcome::Started);
    starting.detail = Some(format!("addr={addr} scope={bind_scope} starting"));
    tracer.emit(starting);
    if let Some(transport) = local_bootstrap.clone() {
        let cancellation = startup_cancellation.clone();
        tokio::spawn(async move {
            if transport.closed().await.is_ok() {
                cancellation.cancel();
            }
        });
    }
    let startup_artifact_authority = match configured_artifact_authority(&data_dir, native_codec_provider, &tracer, &startup_cancellation, &startup_progress).await {
        Err(_) if startup_cancellation.is_cancelled() => {
            let mut record = TraceRecord::new("server.shutdown", TraceOutcome::Cancelled);
            record.detail = Some("launcher-closed-during-catalog-load database=unopened".into());
            tracer.emit(record);
            return Ok(());
        }
        loaded => loaded?,
    };
    let trusted_catalog_stalled = matches!(startup_artifact_authority, StartupArtifactAuthority::Stalled);
    let artifact_authority = startup_artifact_authority.configured();
    if let Some(configured) = artifact_authority.as_ref() {
        configured.catalog.configure_guest_residency(guest_residency);
    }
    let db = Arc::new(connect_db(&data_dir).await?);
    let served: Result<(usize, Result<(), HubError>), HubError> = async {
        let directory = connect_directory(&data_dir).await?;
        directory.close_all_sync_sessions().await?;
        let directory_service = Arc::new(DirectoryService::new(directory.clone(), 1024));
        let artifact_cas = connect_artifact_cas(&data_dir).await?;
        let startup_control = StartupCatalogControl::new(tracer.clone(), startup_cancellation.clone(), startup_progress.clone());
        let startup_context = OperationContext::stall_bounded(TRUSTED_CATALOG_STARTUP_STALL_BOUND_MS, AuthorityLimits::maximum(), &startup_control)?;
        let artifact_cas_coordinator_id = directory.artifact_cas_coordinator_id().await?;
        artifact_cas.configure_coordinator(artifact_cas_coordinator_id, &startup_context).await?;
        let artifact_publication = Arc::new(CheckpointPublicationOrchestrator::new(ArtifactChunkBlobStore::new(artifact_cas.clone()), HubVerifiedCheckpointPublisher::new(directory_service.clone(), artifact_cas.clone(), "system:artifact-authority")));
        let artifact_maintenance = ArtifactCasMaintenanceSupervisor::start(directory_service.clone(), artifact_cas.clone(), artifact_cas_sweep_execute, tracer.clone());
        let rebootstrap = Arc::new(VerifiedRebootstrapSource::new(directory.clone(), artifact_cas.clone()));
        let admin_dir = std::env::var("OS_HUB_ADMIN_DIR").map(std::path::PathBuf::from).unwrap_or_else(|_| std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🔨️modules/🛡️admin/📦️packages/🟦️typescript/📤️dist")));
        let extensions_root = std::env::var("OS_HUB_EXTENSIONS_DIR").map(std::path::PathBuf::from).unwrap_or_else(|_| data_dir.join("extension-modules"));
        std::fs::create_dir_all(&extensions_root)?;
        let artifact_authority_ready = artifact_authority.is_some();
        let open_plan_ready = artifact_authority.as_ref().is_some_and(|configured| configured.catalog.open_target_count() > 0);
        let agent_delegation_ready = directory.list_agent_delegations(AGENT_DELEGATION_READINESS_PROBE_SCOPE, AGENT_DELEGATION_READINESS_PROBE_SCOPE, 1).await.is_ok();
        let verified_catalog = artifact_authority.as_ref().map(|configured| configured.catalog.clone());
        let gis_map_inference_binding: Arc<dyn MapInferenceProposalBindingV1> = Arc::new(GisMapInferenceBindingV1);
        let gis_map_binding = match verified_catalog.as_ref() {
            Some(catalog) => verified_gis_map_binding(catalog.clone(), gis_map_inference_binding.as_ref()).map_err(|error| AuthorityError::Catalog(format!("verified GIS Map inference binding rejected: {error:?}")))?,
            None => None,
        };
        let openable_catalog = artifact_authority.as_ref().map(|configured| -> Arc<dyn DocumentOpenCatalogAuthorityV1> { configured.catalog.clone() });
        let socket_binding_gates = Arc::new(SocketBindingGatesV1::default());
        let artifact_creation_commit_authority = Arc::new(HubArtifactCreationCommitAuthorityV1 { directory: directory.clone(), gates: socket_binding_gates.clone() });
        let artifact_creation_tasks = Arc::new(ArtifactCreationHttpTaskOwnerV1::new());
        let artifact_creation = verified_catalog.as_ref().map(|catalog| Arc::new(ArtifactCreationServiceV1::new(directory_service.clone(), catalog.clone(), artifact_cas.clone())));
        if let Some(service) = artifact_creation.as_ref() {
            artifact_creation_tasks.start_recovery(service.clone(), artifact_creation_commit_authority.clone(), tracer.clone());
        }
        let fanout = Arc::new(ShardedMap::new());
        let fanout_capacity = 256;
        let inference_runtime = match gis_map_binding.as_ref() {
            Some(binding) => {
                let root = data_dir.join("inference");
                std::fs::create_dir_all(&root)?;
                let ledger = Arc::new(InferenceJobLedgerV1::open(&root.join("gis-map-jobs.sqlite3")).map_err(|error| AuthorityError::Catalog(format!("inference job ledger unavailable: {error:?}")))?);
                let configured = artifact_authority.as_ref().ok_or_else(|| AuthorityError::Catalog("GIS Map approval requires the configured canonical artifact authority".into()))?;
                let publisher: Arc<dyn GisMapApprovalCheckpointPublisherV1> = Arc::new(GisMapApprovalCheckpointPublisherV1Impl {
                    directory: directory.clone(),
                    directory_service: directory_service.clone(),
                    artifact_cas: artifact_cas.clone(),
                    authority: configured.authority.clone(),
                    fanout: fanout.clone(),
                    fanout_capacity,
                });
                let committer: Arc<RetainedGisMapApprovalCommitterV1<GisMapInferenceBindingV1>> = Arc::new(RetainedGisMapApprovalCommitterV1::new(db.clone(), db.storage().await, ledger.clone(), publisher));
                let runtime = Arc::new(HubInferenceRuntimeV1::new(binding.clone(), ledger, committer, gis_map_inference_binding.clone()));
                #[cfg(feature = "integration-fixtures")]
                if let Some(descriptor) = std::env::var_os("OS_HUB_TEST_INFERENCE_CHECKPOINT_FD") {
                    if mode != HubMode::Development || descriptor.to_str() != Some("4") {
                        return Err(HubError::UnsafeAuthConfiguration("test inference checkpoint control requires development mode and inherited descriptor 4".into()));
                    }
                    runtime
                        .install_checkpoint_test_gate(Arc::new(InferenceCheckpointTestGateV1::open_inherited().map_err(|_| HubError::UnsafeAuthConfiguration("test inference checkpoint control unavailable".into()))?))
                        .map_err(|_| HubError::UnsafeAuthConfiguration("test inference checkpoint control already installed".into()))?;
                }
                Some(runtime)
            }
            None => None,
        };
        let inference_ready = inference_runtime.is_some();
        let artifact_authority_reason =
            artifact_authority_closed_reason(true, trusted_catalog_stalled, data_dir.join("trusted-catalog/current.json").try_exists().unwrap_or(false));
        let readiness = Arc::new(declare_public_session_issuance(
            with_checkpoint_policy(
                hub_readiness(mode, bind_scope, run_id, bootstrap_ready, artifact_authority_ready, open_plan_ready, agent_delegation_ready, admin_dir.is_dir(), true, artifact_cas_sweep_execute, inference_ready, artifact_authority_reason),
                checkpoint_policy,
            ),
            credential_sign_in.is_enabled(),
        ));
        let admin_cursor_key = SessionCapability::mint()?.secret_digest();
        let space_administration_cursor_key = SessionCapability::mint()?.secret_digest();
        let merge_policy = merge_policy_from_env(&tracer);
        let (instance, framework_router) = compose_hub_server(&data_dir, directory.clone()).await?;
        let socket_drain = Arc::new(HubSocketDrainV1::default());
        tracer.emit({
            let mut record = TraceRecord::new("server.boot", TraceOutcome::Ok);
            record.detail = Some(format!("instance-storage={}", data_dir.join("instance").display()));
            record
        });
        let check_ins = Arc::new(DocumentCheckInJobs::default());
        let state: HubState = semio_hub_state::HubStateParts {
            tracer: tracer.clone(),
            route_metrics: Arc::default(),
            instance,
            db: db.clone(),
            artifact_cas,
            directory: directory.clone(),
            rebootstrap,
            artifact_authority: artifact_authority.map(|configured| configured.authority),
            verified_catalog,
            artifact_creation,
            artifact_creation_commit_authority,
            artifact_creation_tasks: artifact_creation_tasks.clone(),
            gis_map_binding,
            inference_runtime: inference_runtime.clone(),
            openable_catalog,
            artifact_publication,
            artifact_maintenance: artifact_maintenance.clone(),
            check_ins: check_ins.clone(),
            checkpoint_policy: Arc::default(),
            directory_service,
            credential_sign_in,
            rate_limits: Arc::new(HubRateLimiterV1::system()),
            asset_streams: Arc::default(),
            admin_subjects,
            admin_cursor_key,
            space_administration_cursor_key,
            admin_operations: Arc::new(ShardedMap::new()),
            admin_operation_slots: Arc::new(tokio::sync::Semaphore::new(64)),
            admin_operation_tasks: Arc::new(AdminOperationTaskOwner::new(ADMIN_OPERATION_SHUTDOWN_DEADLINE)),
            readiness,
            admin_dir,
            fanout,
            fanout_capacity,
            presence: Arc::new(ShardedMap::new()),
            presence_publication_gate: Arc::new(tokio::sync::Mutex::new(PresenceDirectoryProjections::default())),
            session_colors: Arc::new(ShardedMap::new()),
            session_kicks: Arc::new(ShardedMap::new()),
            socket_drain: socket_drain.clone(),
            socket_grants: Arc::new(SocketGrantLedgerV1::default()),
            document_open_plans: Arc::new(DocumentOpenPlanLedgerV1::default()),
            socket_binding_gates,
            extensions_root,
            merge_policy,
        }
        .into();
        let listener = tokio::net::TcpListener::from_std(bound)?;
        boot_server.hand_over().await;
        let saga_drain = SagaDrainSupervisor::start(state.instance.clone(), state.tracer.clone(), SAGA_DRAIN_INTERVAL);
        let catalog_verification = state.verified_catalog.clone().map(|catalog| tokio::spawn(verify_pending_catalog_guests(catalog, tracer.clone(), startup_cancellation.clone(), startup_progress.clone())));
        let admin_operation_tasks = state.admin_operation_tasks.clone();
        let artifact_creation_tasks = state.artifact_creation_tasks.clone();
        let bootstrap_task = local_bootstrap.clone().map(|transport| {
            let control: Arc<dyn IdentityVerificationControl> = bootstrap_control.clone();
            tokio::spawn(serve_local_bootstrap(transport, directory, control))
        });
        eprintln!("{}", startup_readiness_line(&state.readiness, &addr));
        eprintln!("[INFO] bind scope {bind_scope} ({addr}), cross-origin policy {}, trusted forwarding {}", cross_origin.label(), forwarded_tls.label());
        state.note(
            "server.readiness",
            if state.readiness.blocked_by.is_empty() { TraceOutcome::Ok } else { TraceOutcome::Refused },
            &readiness_trace_detail(&state.readiness, &addr, bind_scope),
        );
        let shutdown_tracer = state.tracer.clone();
        let signal_drain = socket_drain.clone();
        let result = {
            let server = std::future::IntoFuture::into_future(
                axum::serve(listener, framework_router.merge(router(state, cross_origin, forwarded_tls)).into_make_service_with_connect_info::<SocketAddr>()).with_graceful_shutdown(async move {
                    let signal = termination_signal(&shutdown_tracer).await;
                    let mut record = TraceRecord::new("server.readiness", TraceOutcome::Cancelled);
                    record.detail = Some(format!("{}-received-draining-in-flight-work", signal.name()));
                    shutdown_tracer.emit(record);
                    signal_drain.begin();
                }),
            );
            tokio::pin!(server);
            if let Some(mut bootstrap_task) = bootstrap_task {
                tokio::select! {
                    result = &mut server => {
                        bootstrap_control.cancel();
                        if let Some(transport) = local_bootstrap {
                            let _ = transport.shutdown().await;
                        }
                        bootstrap_task.abort();
                        let _ = bootstrap_task.await;
                        result.map_err(HubError::Io)
                    }
                    result = &mut bootstrap_task => {
                        bootstrap_control.cancel();
                        match result {
                            Ok(Ok(())) => Ok(()),
                            Ok(Err(error)) => Err(HubError::Directory(error)),
                            Err(_) => Err(HubError::UnsafeAuthConfiguration("local bootstrap service stopped".into())),
                        }
                    }
                }
            } else {
                server.await.map_err(HubError::Io)
            }
        };
        socket_drain.begin();
        let retained_sockets = socket_drain.drained(SOCKET_DRAIN_DEADLINE).await;
        admin_operation_tasks.shutdown().await;
        artifact_creation_tasks.shutdown().await;
        check_ins.cancel_all();
        let check_in_drain_started = std::time::Instant::now();
        while check_ins.live_count() != 0 && check_in_drain_started.elapsed() < SOCKET_DRAIN_DEADLINE {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        let inference_close_result = match inference_runtime {
            Some(inference_runtime) => inference_runtime.close().await.map_err(HubError::InferenceShutdown),
            None => Ok(()),
        };
        artifact_maintenance.shutdown().await;
        if let Some(verification) = catalog_verification {
            verification.abort();
            let _ = verification.await;
        }
        saga_drain.shutdown().await;
        Ok((retained_sockets, result.and(inference_close_result)))
    }
    .await;
    let database_close_result = close_hub_database(db, DATABASE_SHUTDOWN_DEADLINE).await;
    tracer.emit(hub_shutdown_record(&served, &database_close_result));
    match served {
        Ok((_, result)) => result?,
        Err(_) if startup_cancellation.is_cancelled() => {}
        Err(error) => return Err(error),
    }
    database_close_result
}

/// 🔚️ The one `server.shutdown` record every exit after the database opened ends with — a
/// drained serve and a startup refused after the open alike — naming the retained sockets (or the
/// refusal) and whether the database closed, so a restart right after it is known to reopen.
fn hub_shutdown_record(served: &Result<(usize, Result<(), HubError>), HubError>, database: &Result<(), HubError>) -> TraceRecord {
    let clean = matches!(served, Ok((0, _))) && database.is_ok();
    let mut record = TraceRecord::new("server.shutdown", if clean { TraceOutcome::Ok } else { TraceOutcome::Failed });
    let serve = match served {
        Ok((retained_sockets, _)) => format!("retained-sockets={retained_sockets}"),
        Err(error) => format!("startup-refused={error}"),
    };
    record.detail = Some(match database {
        Ok(()) => format!("{serve} database=closed"),
        Err(error) => format!("{serve} database={error}"),
    });
    record
}
//#endregion 🔖️Main

#[path = "../🗿️artifact-authority/🔏️trusted-catalog/📤️command/🦀️.rs"]
mod trusted_catalog_command;

#[path = "../🔐️auth/📤️command/🦀️.rs"]
mod credential_command;
