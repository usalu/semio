/// 🚪️ Catalog stall, gate and artifact creation laws of the hub boot: the stalled trusted catalog load, the mcp workspace readiness, the not ready gate reasons, the artifact creation admission and recovery, the server owned catalog data root, the launcher close and the missing native provider.

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
