
use std::sync::Arc;
use semio_hub_artifact_creation::ArtifactCreationActorV1;
use semio_hub_lag_rebootstrap::{CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE, decode_canonical_checkpoint_pair};
use directory::os_directory::{DocumentScope, EditedArtifactFrontierV1};
use semio_hub_state::HubState;
use semio_hub_directory_contract::SessionCapability;
use semio_framework_hash::Sha256;
use std::net::SocketAddr;
use semio_hub_route_testkit::{TestIssuedSession, create_space_for_test, issue_test_session, publish_genesis_checkpoint_for_test, raw_http_request, tempdir, test_state, upsert_member_for_test};
use semio_hub_artifact_authority::ValidatingCanonicalArtifactAuthority;
use protocol::ArtifactId as WireArtifactId;
use semio_hub_runtime::presence::db_artifact_id;
use semio_hub_route_models::check_in::ledger_artifact_frontier;
use semio_hub_directory::{AdminOperationDirectory, ArtifactCreationDirectory, AuthSessionDirectory, DocumentDirectory, EventLog};
#[cfg(test)]
use directory::os_directory::{DirectorySpaceRole, DirectorySpaceVisibility, DocumentCheckInStatusV1, DocumentCheckInV1, DocumentDescriptor};
#[cfg(test)]
use protocol::MutationEnvelope;
#[cfg(test)]
use directory::os_directory;
#[cfg(feature = "integration-fixtures")]
#[path = "../../🪶️count-lease/🦀️.rs"]
mod count_lease_tests;

/// 🏝️ A law that saturates a process-global capacity (the database's DB I/O credit ledger) owns a
/// whole process: inside a shared test process it re-runs itself as the only law of a child process
/// and reports that child's verdict, so it never refuses the writes of the laws running beside it; a
/// runner that already gives every law its own process (nextest) runs it in place. Returns `true`
/// where the law body must run. The database's own laws use the same protocol
/// (`db_storage::process_isolated_law`).
fn process_isolated_law(law: &str) -> bool {
    const ISOLATED_LAW: &str = "SEMIO_HUB_ISOLATED_LAW";
    const ISOLATED_LAW_WATCHDOG: std::time::Duration = std::time::Duration::from_secs(600);
    if std::env::var_os("NEXTEST").is_some() || std::env::var(ISOLATED_LAW).is_ok_and(|isolated| isolated == law) {
        return true;
    }
    let executable = std::env::current_exe().expect("process-isolated law resolves its test executable");
    let log_path = tempdir("isolated-law").join("child.log");
    let log = std::fs::File::create(&log_path).expect("process-isolated law creates its child log");
    let mut child = std::process::Command::new(executable)
        .args(["--exact", law, "--test-threads=1"])
        .env(ISOLATED_LAW, law)
        .stdout(log.try_clone().expect("process-isolated law shares its child log"))
        .stderr(log)
        .spawn()
        .expect("process-isolated law spawns its child process");
    let deadline = std::time::Instant::now() + ISOLATED_LAW_WATCHDOG;
    let status = loop {
        if let Some(status) = child.try_wait().expect("process-isolated law observes its child") {
            break Some(status);
        }
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    let output = std::fs::read_to_string(&log_path).unwrap_or_default();
    let _ = std::fs::remove_dir_all(log_path.parent().expect("child log directory"));
    assert!(status.is_some_and(|status| status.success()) && output.contains("test result: ok. 1 passed"), "process-isolated law {law} did not pass as the only law of its own process ({status:?}):\n{output}");
    false
}

/// 📌️ One GIS Map document on the verified GIS profile: a genesis checkpoint, an author, a
/// spectator, and the document's live actor.
#[cfg(feature = "integration-fixtures")]
struct CheckInFixture {
    state: HubState,
    author: TestIssuedSession,
    spectator: TestIssuedSession,
    scope: DocumentScope,
    handle: db::ArtifactHandle,
    genesis: os_directory::ArtifactCheckpoint,
    pack: Vec<u8>,
    spr: Vec<u8>,
    _profile: semio_hub_trusted_catalog_fixtures::VerifiedGisMapIntegrationProfileV1,
}

/// 🗺️ One GIS Map document on the verified GIS profile (the kind a hub opens and checks in), its
/// genesis checkpoint published, one Author, one Spectator and a separate owner.
#[cfg(feature = "integration-fixtures")]
async fn check_in_fixture(label: &str) -> CheckInFixture {
    use semio_hub_trusted_catalog_fixtures as trusted_catalog_fixture;
    let profile = trusted_catalog_fixture::verified_gis_map_integration_profile(&trusted_catalog_fixture::unique_profile_root(&format!("check-in-{label}"))).await.expect("verified GIS Map check-in profile");
    let selection = profile.binding().selection().clone();
    let mut state = test_state().await;
    let author_email = format!("check-in-{label}-author@example.test");
    let author = issue_test_session(&state, &author_email).await;
    let spectator = issue_test_session(&state, &format!("check-in-{label}-spectator@example.test")).await;
    let owner = issue_test_session(&state, &format!("check-in-{label}-owner@example.test")).await;
    let space_id = create_space_for_test(&state, &owner.user_id, &format!("Check In {label}"), os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
    upsert_member_for_test(&state, &space_id, &author_email, DirectorySpaceRole::Author).await;
    upsert_member_for_test(&state, &space_id, &format!("check-in-{label}-spectator@example.test"), DirectorySpaceRole::Spectator).await;
    let request_id = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(format!("check-in-{label}").as_bytes()))[..32].to_string();
    let scope = DocumentScope::new(space_id, format!("artifact-{request_id}"));
    let mut descriptor = DocumentDescriptor {
        space_id: scope.space_id.clone(),
        document_id: scope.document_id.clone(),
        artifact_kind: selection.artifact.kind.clone(),
        artifact_schema: selection.artifact.schema.clone(),
        owner: os_directory::DocumentOwner { plugin_id: selection.package.plugin_id.clone(), package_id: selection.package.package_id.clone(), version: selection.package.version.clone(), package_hash: selection.package.component_sha256.clone() },
        pack_schema_hash: selection.artifact.pack_schema_hash.clone(),
        bootstrap_version: 1,
        bootstrap_frontier: os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
        bootstrap_snapshot_hash: String::new(),
    };
    state.artifact_authority = Some(Arc::new(ValidatingCanonicalArtifactAuthority::new(profile.catalog().clone())));
    state.verified_catalog = Some(profile.catalog().clone());
    state.openable_catalog = Some(profile.catalog().clone());
    let document = db_artifact_id(&scope);
    let pack = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&semio_s_artifact_gis_gismap::GisMapSnapshot::default());
    let spr = directory::os_store::text_format::empty_document_spr(&document.0, &descriptor.artifact_schema).await;
    descriptor.bootstrap_snapshot_hash = os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(&pack));
    let session = state.directory.authenticate_session(&SessionCapability::parse(&author.token).expect("check-in author capability")).await.expect("check-in author session read").expect("check-in author session");
    let actor = ArtifactCreationActorV1 { user_id: author.user_id.clone(), session_id: session.id, authorization_generation: session.authorization_generation };
    let genesis = publish_genesis_checkpoint_for_test(&state, actor, profile.catalog().generation_id().to_string(), selection.parent_dialect.clone(), descriptor, &pack, &spr).await;
    let handle = state.ensure_document(&document).await.expect("check-in document actor");
    CheckInFixture { state, author, spectator, scope, handle, genesis, pack, spr, _profile: profile }
}

/// ✍️ The ledger a real GIS Map editor emits for `ids.len()` one-operation edits made on the genesis
/// pair — each places one position — as one envelope per edit, exactly its store's own event log,
/// addressed to the hub's document key.
#[cfg(feature = "integration-fixtures")]
async fn check_in_map_edits(fixture: &CheckInFixture, ids: &[&str]) -> Vec<MutationEnvelope> {
    use directory::os_store::{store_schema::ArtifactCommand, ArtifactStore, SnapshotRetirementStep};
    use semio_s_artifact_gis_gismap::mutations::create_position::CreatePosition;
    use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot, MapFeature};
    let parsed = directory::os_store::text_format::parse_document_pack::<GisMapSnapshot, GisMapMutation>(&fixture.pack, &fixture.spr).await.expect("genesis pair parses");
    let mut store = ArtifactStore::<GisMapSnapshot, GisMapMutation>::new(parsed.into_envelope(), directory::ActorId(directory::os_spr::LOCAL_ACTOR_ID.into())).await.expect("editor store");
    store.install_document_store_owners_exact(directory::os_store::document_store_owners::bounded_artifact_store_owners());
    let mut applied = Ok(());
    for (index, id) in ids.iter().enumerate() {
        let point = directory::DslValue::object([("lon".into(), directory::DslValue::float(7.0 + index as f64)), ("lat".into(), directory::DslValue::float(47.0))]);
        let mutation = GisMapMutation::CreatePosition(CreatePosition { index, item: MapFeature { id: format!("check-in-{id}"), data: point } });
        if let Err(error) = store.dispatch(ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }).await {
            applied = Err(error);
            break;
        }
    }
    let events = store.event_log();
    loop {
        match store.close_owned_step(1, directory::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).expect("editor store closes") {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { .. } => {}
            SnapshotRetirementStep::Blocked => panic!("editor store close blocked"),
        }
    }
    drop(store);
    applied.expect("editor edits apply");
    let document = WireArtifactId(db_artifact_id(&fixture.scope).0);
    events.expect("editor ledger").into_iter().map(|mut envelope| {
        envelope.document_id = document.clone();
        envelope
    }).collect()
}

/// 📥️ Commits one envelope as its own ledger transaction and names the head it produced.
#[cfg(feature = "integration-fixtures")]
async fn check_in_commit(fixture: &CheckInFixture, envelope: MutationEnvelope) -> EditedArtifactFrontierV1 {
    let batch = db::document::CommandBatch::new(vec![envelope]).await.expect("check-in command batch");
    fixture.handle.submit(batch, db::document::SubmitOptions { durability: db::DurabilityClass::Fsync, policy: protocol::MergePolicy::default() }).await.expect("check-in actor response").expect("check-in edit accepted");
    let snapshot = fixture.handle.checkpoint_publication_snapshot().await.expect("check-in actor snapshot");
    directory::os_directory::io::binary::artifact_hash::encode_edited_artifact_frontier_v1(&ledger_artifact_frontier(&fixture.scope, &snapshot).expect("an edited ledger point")).expect("edited wire frontier")
}

/// 🧮️ The pair a remote replica holds after folding `envelopes` onto the genesis pair — the oracle a
/// hub-materialized checkpoint must equal byte for byte.
#[cfg(feature = "integration-fixtures")]
async fn check_in_replica_pair(fixture: &CheckInFixture, envelopes: &[MutationEnvelope]) -> (Vec<u8>, Vec<u8>) {
    use semio_s_artifact_gis_gismap::{GisMapMutation, GisMapSnapshot};
    let files = directory::os_store::replay_envelopes_onto_pair::<GisMapSnapshot, GisMapMutation>(&fixture.pack, &fixture.spr, &directory::os_spr::encode_envelopes(envelopes), directory::os_store::document_store_owners::bounded_artifact_store_owners).await.expect("replica fold");
    (files.pack, files.spr)
}

/// 📮️ Posts one Check In and follows it to a terminal status.
#[cfg(feature = "integration-fixtures")]
async fn check_in_until_terminal(addr: SocketAddr, fixture: &CheckInFixture, token: &str, request_id: &str, head: &EditedArtifactFrontierV1) -> (u16, Option<DocumentCheckInStatusV1>) {
    let route = format!("/spaces/{}/documents/{}/check-ins", fixture.scope.space_id, fixture.scope.document_id);
    let request = DocumentCheckInV1 { schema: "semio.hub.document-check-in/v1".into(), request_id: request_id.into(), head: head.clone() }.canonical_json().expect("canonical check-in");
    let authorization = format!("Bearer {token}");
    let posted = raw_http_request(addr, "POST", &route, &[("Authorization", authorization.as_str()), ("Content-Type", "application/json")], request.as_bytes()).await;
    if posted.status != 200 && posted.status != 202 {
        return (posted.status, None);
    }
    let mut status = DocumentCheckInStatusV1::parse_canonical_json(std::str::from_utf8(&posted.body).expect("status UTF-8")).expect("canonical status");
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
    while !status.phase.is_terminal() {
        assert!(std::time::Instant::now() < deadline, "check-in {request_id} never reached a terminal status: {status:?}");
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        let polled = raw_http_request(addr, "GET", &format!("{route}/{request_id}"), &[("Authorization", authorization.as_str())], &[]).await;
        assert!(polled.status == 200 || polled.status == 202, "check-in poll answered {}", polled.status);
        let next = DocumentCheckInStatusV1::parse_canonical_json(std::str::from_utf8(&polled.body).expect("status UTF-8")).expect("canonical status");
        assert!(next.progress.completed_units >= status.progress.completed_units, "check-in progress went back: {status:?} -> {next:?}");
        status = next;
    }
    (posted.status, Some(status))
}

/// 🧾️ The hub's active checkpoint pair as a cold opener reads it.
#[cfg(feature = "integration-fixtures")]
async fn check_in_cold_pair(addr: SocketAddr, fixture: &CheckInFixture) -> semio_hub_lag_rebootstrap::VerifiedActiveCheckpointPair {
    let authorization = format!("Bearer {}", fixture.author.token);
    let response = raw_http_request(addr, "GET", &format!("/spaces/{}/documents/{}/active-checkpoint/pair", fixture.scope.space_id, fixture.scope.document_id), &[("Authorization", authorization.as_str()), ("Accept", CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE)], &[]).await;
    assert_eq!(response.status, 200, "cold open answered {}", String::from_utf8_lossy(&response.body));
    decode_canonical_checkpoint_pair(&response.body).expect("canonical cold pair")
}

#[path = "../../../🎲️hostile-generative/🦀️.rs"]
mod hostile_generative;

mod quick {
use semio_hub_route_models::admin::ADMIN_OPERATION_DEADLINE;
use std::sync::Arc;
use semio_hub_runtime::readiness::{CheckpointPolicyLedgerV1, CheckpointPolicyTallyV1, HUB_CHECKPOINT_POLICY, HubCheckpointPolicyV1, HubMode, hub_readiness, with_checkpoint_policy};
use semio_hub_artifact_checkin::DocumentCheckInKey;
use directory::os_directory::{DocumentScope, EditedArtifactFrontierV1};
use semio_hub_native_openable::NativeCodecProviderSetV1;
use semio_hub_route_testkit::{SOCKET_RENDEZVOUS_HANG_GUARD, STUDIO, announce_document_for_test, artifact_document_id_for_test, authorize_test_admin, bearer_headers, client_binary, create_space_for_test, document_socket_request, install_document_open_catalog_for_test, issue_and_exchange_document_open_plan_for_test, issue_test_session, loopback_peer, native_openable_stdio_bundle, next_close_code, next_close_without_authority, next_directory_message, next_server_frame, next_server_frame_at, publish_checkpoint_for_test, raw_http_get, raw_http_request, run_socket_test, sample_envelope, seed_author_token, seed_genesis_document_for_test, seed_genesis_for_document_for_test, socket_hello, socket_request, spawn_restartable_server, spawn_server, stop_recovery_server, tempdir, test_state, test_state_with_directory, upsert_member_for_test};
use semio_hub_directory_sqlite::SqliteDirectory;
use semio_hub_runtime::startup::{StartupCancellationV1, StartupProgressCellV1, configured_artifact_authority};
use semio_framework_trace::record::Tracer;
use protocol::ArtifactId as WireArtifactId;
use semio_hub_document_routes::admin::{admin_intents, cancel_admin_operation};
use semio_hub_document_routes::check_in::{check_in_author, run_policy_check_in};
use tokio_tungstenite::connect_async;
use semio_hub_runtime::presence::db_artifact_id;
use semio_hub_session_routes::session_agent::post_session_sign_out;
#[cfg(test)]
use protocol::{AckStage, ActorId, ApplyOutcome, ClientFrame, Lane, ServerFrame};
#[cfg(test)]
use directory::os_directory::{AdminIntentReceiptV1, AdminIntentStateV1, AdminIntentV1, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentCheckInPhaseV1, DocumentCheckInRefusalV1, DocumentCheckInStatusV1, DocumentCheckInV1};
#[cfg(test)]
use axum::body::Bytes;
#[cfg(test)]
use axum::extract::{Path, State};
#[cfg(test)]
use axum::http::StatusCode;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::state::TestLiveGate;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_runtime::presence::TestPresenceClock;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::{execute_directory_command_fenced, issue_document_socket_grant_fixture};
#[cfg(test)]
use axum::routing::post;
#[cfg(feature="integration-fixtures")]
use crate::tests::{check_in_cold_pair, check_in_commit, check_in_fixture, check_in_map_edits, check_in_replica_pair, check_in_until_terminal};

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
            let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-command-revoke/document/ws");
            let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
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
            let wire_document = WireArtifactId("socket-command-revoke".into());
            let mut accepted = sample_envelope("accepted-op", &wire_document).await;
            accepted.actor = ActorId(receipt.actor_id.clone());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 90, envelopes: vec![accepted] }, Lane::Command).await).await.expect("control command received by server");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("control command boundary deadline").expect("control command boundary").forget();
            live_gate.socket_command_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Ack { batch_id: 90, .. }));
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Commands { envelopes, .. } if envelopes.iter().all(|envelope| envelope.document_id == wire_document)), "the authorized batch's own relay is delivered before any revoke, on the wire document id");
            let accepted_frontier = state.db.document(&document).await.expect("document handle").frontier().await.expect("accepted frontier");
            assert_eq!(accepted_frontier.head_seq, 1, "an actor-matching command persists while authorized");

            let mut revoked = sample_envelope("revoked-op", &wire_document).await;
            revoked.actor = ActorId(receipt.actor_id.clone());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 91, envelopes: vec![revoked] }, Lane::Command).await).await.expect("revoked command received by server");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("command boundary deadline").expect("command boundary").forget();
            assert_eq!(post_session_sign_out(bearer_headers(&token), State(state.clone()), Bytes::new()).await, StatusCode::NO_CONTENT);
            live_gate.socket_command_release.add_permits(1);
            assert_eq!(next_close_without_authority(&mut socket).await, 4401, "no Ack crosses a revoke that wins before command admission");
            let frontier = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier");
            assert_eq!(frontier.head_seq, 1, "the revoked actor-matching command never reaches durable storage");
        });
    }

    /// 🧾️ A `Commands` batch the engine committed after its socket's frame deadline elapsed is still acknowledged on that
    /// socket, its relay still reaches the socket, and the socket stays open for the next batch: the deadline bounds a
    /// frame's admission only (ticket 26/09/23 session 14c — H13's transient probe saw batch 24 commit at head 6144 while
    /// the deadline closed its socket `1013` and its Ack never came).
    #[test]
    fn a_batch_committed_past_the_frame_deadline_is_still_acknowledged() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let live_gate = Arc::new(TestLiveGate::default());
            *live_gate.socket_frame_deadline.lock().expect("frame deadline override") = Some(std::time::Duration::from_millis(200));
            live_gate.socket_commit_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
            state.live_gate = Some(live_gate.clone());
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-commit-deadline").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-commit-deadline".to_string())), bearer_headers(&token), State(state.clone())).await.expect("issue socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-commit-deadline/document/ws");
            let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_before_welcome.acquire()).await.expect("pre-Welcome deadline").expect("pre-Welcome").forget();
            live_gate.socket_welcome_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_after_welcome.acquire()).await.expect("post-Welcome deadline").expect("post-Welcome").forget();
            live_gate.socket_bootstrap_release.add_permits(1);
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.document_subscribed.acquire()).await.expect("subscription deadline").expect("subscription").forget();
            live_gate.document_release.add_permits(1);

            let document = db_artifact_id(&DocumentScope::new(STUDIO, "socket-commit-deadline"));
            let wire_document = WireArtifactId("socket-commit-deadline".into());
            for (batch_id, id, head) in [(70u64, "late-commit-op", 1u64), (71, "next-commit-op", 2)] {
                let mut envelope = sample_envelope(id, &wire_document).await;
                envelope.actor = ActorId(receipt.actor_id.clone());
                socket.send(client_binary(&ClientFrame::Commands { batch_id, envelopes: vec![envelope] }, Lane::Command).await).await.expect("command sent");
                tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_command_received.acquire()).await.expect("command boundary deadline").expect("command boundary").forget();
                live_gate.socket_command_release.add_permits(1);
                tokio::time::timeout(std::time::Duration::from_secs(5), live_gate.socket_commit_admitted.acquire()).await.expect("commit boundary deadline").expect("commit boundary").forget();
                let committed = state.db.document(&document).await.expect("document handle").frontier().await.expect("committed frontier");
                assert_eq!(committed.head_seq, head, "the engine committed batch {batch_id} before its answer");
                tokio::time::sleep(std::time::Duration::from_millis(600)).await;
                live_gate.socket_commit_release.add_permits(1);
                match next_server_frame_at(&mut socket, "Ack of a batch committed past the frame deadline").await {
                    ServerFrame::Ack { batch_id: acked, stages, .. } => {
                        assert_eq!(acked, batch_id);
                        assert!(stages.iter().any(|stage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted))), "batch {batch_id} is acknowledged as accepted: {stages:?}");
                    }
                    other => panic!("batch {batch_id} answered {other:?} instead of its Ack"),
                }
                assert!(matches!(next_server_frame_at(&mut socket, "relay").await, ServerFrame::Commands { .. }), "the committed batch's relay follows its Ack");
            }
        });
    }

    /// 🔌️ A document socket's authority is its session, membership and sealed plan, never the checkpoint its client seeded
    /// from: a checkpoint published while the socket is open (a member's Check In, or the declared checkpoint policy's) leaves
    /// it open through ten idle minutes of authorization ticks, and its next batch commits and is acknowledged (ticket
    /// 26/09/23 session 14c: on p33 C13's probe socket and both browsers' sockets closed `4401` within one tick of every Check In).
    #[test]
    fn a_plan_socket_outlives_a_check_in_and_ten_idle_minutes_and_its_next_batch_commits() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            state.presence_clock = Some(Arc::new(TestPresenceClock::new()));
            let token = seed_author_token(&state).await;
            let document_id = artifact_document_id_for_test("plan-socket-idle");
            seed_genesis_for_document_for_test(&state, &token, STUDIO, &document_id, "plan-socket-idle").await;
            let scope = DocumentScope::new(STUDIO, &document_id);
            let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor");
            install_document_open_catalog_for_test(&mut state, &descriptor);
            let (plan, receipt) = issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:plan-socket-idle").await;
            let addr = spawn_server(state.clone()).await;
            let url = format!("ws://{addr}/scopes/{STUDIO}%2F{document_id}/document/ws?surface={}", plan.surface.surface_id);
            let (mut socket, _) = connect_async(document_socket_request(&url, &token)).await.expect("plan socket upgrade");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("socket hello");
            assert!(matches!(next_server_frame_at(&mut socket, "welcome").await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame_at(&mut socket, "session").await, ServerFrame::Session { actor, .. } if actor == receipt.actor_id));
            let published = publish_checkpoint_for_test(&state, STUDIO, &document_id).await;
            let active = state.directory.get_active_artifact_checkpoint(&scope).await.expect("active read").expect("active checkpoint");
            assert_eq!(active.baseline_frontier, published.baseline_frontier, "the socket's plan names a superseded checkpoint");
            tokio::time::pause();
            for _ in 0..600 {
                tokio::time::advance(std::time::Duration::from_secs(1)).await;
                tokio::task::yield_now().await;
            }
            tokio::time::resume();
            let document = db_artifact_id(&scope);
            let before = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier before").head_seq;
            let mut envelope = sample_envelope("after-ten-idle-minutes", &WireArtifactId(document_id.clone())).await;
            envelope.actor = ActorId(receipt.actor_id.clone());
            socket.send(client_binary(&ClientFrame::Commands { batch_id: 600, envelopes: vec![envelope] }, Lane::Command).await).await.expect("batch after ten idle minutes");
            match next_server_frame_at(&mut socket, "Ack after ten idle minutes").await {
                ServerFrame::Ack { batch_id: 600, stages, .. } => assert!(stages.iter().any(|stage| matches!(stage, AckStage::Applied { outcome } if matches!(outcome.as_ref(), ApplyOutcome::Accepted))), "the batch commits: {stages:?}"),
                other => panic!("the batch after ten idle minutes was answered {other:?}"),
            }
            let after = state.db.document(&document).await.expect("document handle").frontier().await.expect("frontier after").head_seq;
            assert_eq!(after, before + 1, "the batch is durable");
        });
    }

    #[tokio::test]
    async fn retained_short_admin_request_drop_duplicate_cancel_and_secret_lifecycle_is_exact() {
        let fixture: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("📇️directory/🧪️tests/🧫️fixtures/🏛️retained-short-admin/🔣️.json")).expect("retained short administrator fixture");
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
        let body = semio_framework_pack_json::to_json_string(&intent);
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
        let retry_receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&retry.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("retry receipt");
        assert_eq!(retry_receipt.state, AdminIntentStateV1::Succeeded);
        assert!(retry_receipt.result.is_none(), "a lost one-shot share token is never stored or replayed");
        assert_eq!(
            physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).unwrap(),
            1,
            "retry cannot execute the side effect twice"
        );
        let collision = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: request_id.into(), scope: scope.clone(), ttl_secs: 601 });
        assert_eq!(raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], collision.as_bytes()).await.status, 409);

        let cancelled_scope = DocumentScope::new(&space, "retained-short-cancelled");
        announce_document_for_test(&state, &cancelled_scope.space_id, &cancelled_scope.document_id).await;
        let cancelled_request = "request:retained-short-cancelled";
        let cancelled_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: cancelled_request.into(), scope: cancelled_scope.clone(), ttl_secs: 600 });
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
        let cancelled_receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&cancelled.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("cancelled receipt");
        assert_eq!(cancelled_receipt.state, AdminIntentStateV1::Cancelled);
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![cancelled_scope.space_id, cancelled_scope.document_id], |row| row.get(0)).unwrap(), 0);

        let admitted_scope = DocumentScope::new(&space, "retained-short-admitted");
        announce_document_for_test(&state, &admitted_scope.space_id, &admitted_scope.document_id).await;
        let admitted_request = "request:retained-short-admitted";
        let admitted_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: admitted_request.into(), scope: admitted_scope.clone(), ttl_secs: 600 });
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
        let admitted_receipt: AdminIntentReceiptV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&admitted.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("admitted receipt");
        assert_eq!(admitted_receipt.state, AdminIntentStateV1::Succeeded, "cancellation after effect admission cannot invent rollback");
        assert!(admitted_receipt.result.as_ref().and_then(|result| result.share_token.as_ref()).is_some());
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![admitted_scope.space_id, admitted_scope.document_id], |row| row.get(0)).unwrap(), 1);

        let queued_scope = DocumentScope::new(&space, "retained-short-admitted-queued");
        announce_document_for_test(&state, &queued_scope.space_id, &queued_scope.document_id).await;
        let first_request = "request:retained-short-admitted-queued-first";
        let second_request = "request:retained-short-admitted-queued-second";
        let first_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: first_request.into(), scope: queued_scope.clone(), ttl_secs: 600 });
        let second_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: second_request.into(), scope: queued_scope.clone(), ttl_secs: 600 });
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
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![queued_scope.space_id, queued_scope.document_id], |row| row.get(0)).unwrap(), 0);
        gate.admin_effect_pause_enabled.store(false, std::sync::atomic::Ordering::Release);
        gate.admin_effect_release.add_permits(1);
        let first_response = tokio::time::timeout(std::time::Duration::from_secs(5), &mut first).await.expect("first writer completion deadline").expect("first HTTP task");
        assert_eq!(first_response.status, 200, "a predecessor that releases within the binding-gate budget answers its own waiter");
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
        let mut second_rows = Vec::new();
        for _ in 0..256 {
            second_rows = state.directory.admin_operation_audit_for_request(second_request).await.expect("second admitted writer audit");
            if second_rows.len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(first_rows.len(), 2);
        assert_eq!(second_rows.len(), 2);
        assert_eq!(
            first_rows[1].fact.phase,
            "succeeded",
            "the retained first writer's terminal outcome code names which arm ended it: {} reason {:?}",
            first_rows[1].fact.outcome_code,
            first_rows[1].fact.reason_code
        );
        assert_eq!(second_rows[1].fact.phase, "succeeded", "competing writer terminal outcome {} reason {:?}", second_rows[1].fact.outcome_code, second_rows[1].fact.reason_code);
        assert!(first_rows[1].sequence < second_rows[1].sequence, "the retained first writer terminal precedes its blocked successor");
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![queued_scope.space_id, queued_scope.document_id], |row| row.get(0)).unwrap(), 2);

        let fenced_scope = DocumentScope::new(&space, "retained-short-admitted-deadline");
        announce_document_for_test(&state, &fenced_scope.space_id, &fenced_scope.document_id).await;
        let fenced_request = "request:retained-short-admitted-deadline";
        let fenced_body = semio_framework_pack_json::to_json_string(&AdminIntentV1::IssueDocumentShare { request_id: fenced_request.into(), scope: fenced_scope.clone(), ttl_secs: 600 });
        gate.admin_effect_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let mut fenced = tokio::spawn({
            let authorization = authorization.clone();
            async move { raw_http_request(addr, "POST", "/admin/api/intents", &[("Authorization", &authorization), ("Content-Type", "application/json")], fenced_body.as_bytes()).await }
        });
        tokio::time::timeout(std::time::Duration::from_secs(5), gate.admin_effect_admitted.acquire()).await.expect("fenced admitted writer deadline").expect("fenced writer admitted").forget();
        let fenced_response = tokio::time::timeout(ADMIN_OPERATION_DEADLINE + std::time::Duration::from_secs(2), &mut fenced).await.expect("fenced HTTP deadline response").expect("fenced HTTP task");
        assert_eq!(fenced_response.status, 503, "the HTTP waiter expires without cancelling its admitted writer");
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![fenced_scope.space_id, fenced_scope.document_id], |row| row.get(0)).unwrap(), 0);
        gate.admin_effect_pause_enabled.store(false, std::sync::atomic::Ordering::Release);
        gate.admin_effect_release.add_permits(1);
        let mut fenced_rows = Vec::new();
        for _ in 0..256 {
            fenced_rows = state.directory.admin_operation_audit_for_request(fenced_request).await.expect("fenced admitted writer audit");
            if fenced_rows.len() == 2 {
                break;
            }
            tokio::task::yield_now().await;
        }
        assert_eq!(fenced_rows.len(), 2);
        assert_eq!(
            fenced_rows[1].fact.phase,
            "succeeded",
            "the expired HTTP waiter never cancels its admitted writer: {} reason {:?}",
            fenced_rows[1].fact.outcome_code,
            fenced_rows[1].fact.reason_code
        );
        assert_eq!(physical.query_row::<i64, _, _>("SELECT count(*) FROM hub_share_grant WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![fenced_scope.space_id, fenced_scope.document_id], |row| row.get(0)).unwrap(), 1);
        assert_eq!(state.admin_operation_tasks.task_count(), 0);
        drop(physical);
        stop_recovery_server(state, shutdown, server).await;
    }

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
        let configured = configured_artifact_authority(&root, Some(&providers), &Tracer::disabled(), &StartupCancellationV1::default(), &StartupProgressCellV1::default()).await.expect("verified stdio authority").configured().expect("configured stdio authority");
        assert_eq!(configured.catalog.artifact_kind_count(), 29);
        let receipts = semio_hub_stdio_catalog::native_codec_factory_receipts().expect("actual stdio schema-qualified receipts");
        let pairs = receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str())).collect::<std::collections::BTreeSet<_>>();
        assert_eq!(pairs.len(), receipts.len(), "every current executable codec has one exact kind/schema identity");
        assert_eq!(pairs.len(), 30, "both original PDF standards retain their executable authority");
        assert_eq!(configured.catalog.codec_count(), pairs.len(), "all current executable kind/schema identities activate atomically");
        assert_eq!(configured.catalog.open_target_count(), 1);
        let mut ready = test_state().await;
        ready.openable_catalog = Some(configured.catalog.clone());
        ready.artifact_authority = Some(configured.authority);
        ready.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
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

    /// 📌️ The checkpoint policy is the schema's: absent bounds are the declared default the readiness body carries, each
    /// bound is a decimal within its range, anything else fails boot.
    #[test]
    fn the_checkpoint_policy_is_declared_in_readiness_and_bounded_like_its_schema() {
        assert_eq!(HubCheckpointPolicyV1::configured(None, None), Ok(HUB_CHECKPOINT_POLICY));
        assert_eq!(HubCheckpointPolicyV1::configured(Some(" 1 "), Some("4096")), Ok(HubCheckpointPolicyV1 { edits: 1, payload_bytes: 4_096 }));
        assert_eq!(HubCheckpointPolicyV1::configured(Some("1048576"), Some("1073741824")), Ok(HubCheckpointPolicyV1 { edits: 1_048_576, payload_bytes: 1_073_741_824 }));
        for (edits, bytes) in [(Some("0"), None), (Some("1048577"), None), (Some("ten"), None), (None, Some("4095")), (None, Some("1073741825")), (None, Some("-1"))] {
            assert!(HubCheckpointPolicyV1::configured(edits, bytes).is_err(), "{edits:?} {bytes:?} must fail boot");
        }
        let readiness = with_checkpoint_policy(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"), HubCheckpointPolicyV1 { edits: 7, payload_bytes: 8_192 });
        let body = serde_json::to_value(&readiness).expect("readiness JSON");
        assert_eq!(body["features"]["checkpointPolicy"], serde_json::json!({ "edits": 7, "payloadBytes": 8192 }));
        let declared = serde_json::to_value(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root")).expect("readiness JSON");
        assert_eq!(declared["features"]["checkpointPolicy"], serde_json::json!({ "edits": 1024, "payloadBytes": 524288 }));
    }

    /// 🧮️ The policy's tally starts one Check In when a document reaches either bound, keeps tallying — but starts no second
    /// one — while it runs, and tallies documents apart.
    #[test]
    fn the_checkpoint_policy_starts_one_check_in_per_bound_and_tallies_while_it_runs() {
        let ledger = CheckpointPolicyLedgerV1::default();
        let policy = HubCheckpointPolicyV1 { edits: 4, payload_bytes: 1_000 };
        let (first, second) = (DocumentScope::new("space", "first"), DocumentScope::new("space", "second"));
        assert!(!ledger.observe(&first, 3, 10, policy));
        assert!(ledger.observe(&first, 1, 10, policy), "the fourth edit reaches the edit bound");
        assert_eq!(ledger.tally(&first), Some(CheckpointPolicyTallyV1 { edits: 0, payload_bytes: 0, running: true }));
        assert!(!ledger.observe(&first, 9, 5_000, policy), "no second policy Check In while one runs");
        assert!(ledger.observe(&second, 1, 1_000, policy), "the byte bound alone starts one, for another document");
        ledger.finish(&first);
        assert_eq!(ledger.tally(&first), Some(CheckpointPolicyTallyV1 { edits: 9, payload_bytes: 5_000, running: false }), "edits committed meanwhile stay tallied");
        assert!(ledger.observe(&first, 0, 0, policy), "the tally that grew meanwhile starts the next one");
    }

    /// 📌️ Once the edits committed since the active checkpoint reach the policy the hub checks the document in by itself, on
    /// behalf of the author whose batch reached it: the active checkpoint's baseline becomes the committed head and a cold
    /// open reads the replica fold of the ledger, byte for byte; a frontier-less hello from a seeding plan then resumes there.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn the_checkpoint_policy_checks_a_document_in_at_its_committed_head() {
        let mut fixture = check_in_fixture("policy").await;
        let mut readiness = (*fixture.state.readiness).clone();
        readiness.features.checkpoint_policy = HubCheckpointPolicyV1 { edits: 2, payload_bytes: 1_073_741_824 };
        fixture.state.readiness = Arc::new(readiness);
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_map_edits(&fixture, &["61", "62"]).await;
        check_in_commit(&fixture, ledger[0].clone()).await;
        let head = check_in_commit(&fixture, ledger[1].clone()).await;
        let (subject, _) = check_in_author(&fixture.state, &fixture.scope, &bearer_headers(&fixture.author.token)).await.expect("the author checks in");
        assert!(fixture.state.checkpoint_policy.observe(&fixture.scope, 2, 0, fixture.state.readiness.features.checkpoint_policy), "two committed edits reach the policy");
        let status = run_policy_check_in(fixture.state.clone(), subject, fixture.scope.clone(), fixture.handle.clone()).await.expect("the policy Check In starts");
        assert_eq!(status.phase, DocumentCheckInPhaseV1::Ready, "policy check-in ended {status:?}");
        assert_eq!(fixture.state.checkpoint_policy.tally(&fixture.scope).map(|tally| tally.running), Some(false));
        let active = fixture.state.directory.get_active_artifact_checkpoint(&fixture.scope).await.expect("active read").expect("active checkpoint");
        assert_eq!(directory::os_directory::io::binary::artifact_hash::encode_edited_artifact_frontier_v1(&active.baseline_frontier), Some(head), "the active checkpoint is the committed head");
        let cold = check_in_cold_pair(addr, &fixture).await;
        assert_eq!(cold.selection.active_checkpoint_id, active.checkpoint_id);
        assert_eq!((cold.pair().pack.clone(), cold.pair().spr.clone()), check_in_replica_pair(&fixture, &ledger).await, "the policy checkpoint is the replica fold, byte for byte");
    }

    /// 📌️ Check In folds the hub's own ledger onto the active checkpoint and publishes it: after two
    /// edits the active checkpoint's baseline is exactly the named head, its parent is genesis, and a
    /// cold open reads the pair a remote replica holds after the same ledger, byte for byte. A third
    /// edit and a second Check In advance it again from the first checkpoint.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn check_in_advances_the_active_checkpoint_to_the_named_head_and_cold_opens_from_it() {
        let fixture = check_in_fixture("advances").await;
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_map_edits(&fixture, &["43", "44", "45"]).await;
        assert_eq!(ledger.len(), 3, "one envelope per one-operation edit");
        check_in_commit(&fixture, ledger[0].clone()).await;
        let second = check_in_commit(&fixture, ledger[1].clone()).await;

        let (code, status) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "11111111111111111111111111111111", &second).await;
        let status = status.unwrap_or_else(|| panic!("check-in refused with {code}"));
        assert_eq!(code, 202, "a fresh check-in answers accepted");
        assert_eq!(status.phase, DocumentCheckInPhaseV1::Ready, "check-in ended {status:?}");
        let ready = status.ready.clone().expect("ready names its checkpoint");
        assert_eq!(ready.baseline, second);
        assert_eq!(ready.parent_checkpoint_id, directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&fixture.genesis.checkpoint_id));
        let active = fixture.state.directory.get_active_artifact_checkpoint(&fixture.scope).await.expect("active read").expect("active checkpoint");
        assert_eq!(directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&active.checkpoint_id), ready.checkpoint_id);
        assert_eq!(directory::os_directory::io::binary::artifact_hash::encode_edited_artifact_frontier_v1(&active.baseline_frontier), Some(second.clone()), "the head advanced to the named head");
        let cold = check_in_cold_pair(addr, &fixture).await;
        assert_eq!(directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&cold.selection.active_checkpoint_id), ready.checkpoint_id, "a cold open starts from the new checkpoint");
        let expected = check_in_replica_pair(&fixture, &ledger[..2]).await;
        assert_eq!((cold.pair().pack.clone(), cold.pair().spr.clone()), expected, "the checkpoint is the replica fold of the ledger, byte for byte");

        let third = check_in_commit(&fixture, ledger[2].clone()).await;
        let (_, next) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "22222222222222222222222222222222", &third).await;
        let next = next.expect("second check-in status");
        let next_ready = next.ready.clone().unwrap_or_else(|| panic!("second check-in ended {next:?}"));
        assert_eq!(next_ready.parent_checkpoint_id, ready.checkpoint_id, "the second checkpoint extends the first");
        let cold = check_in_cold_pair(addr, &fixture).await;
        assert_eq!(directory::os_directory::io::binary::artifact_hash::artifact_hash_hex(&cold.selection.active_checkpoint_id), next_ready.checkpoint_id);
        assert_eq!((cold.pair().pack.clone(), cold.pair().spr.clone()), check_in_replica_pair(&fixture, &ledger).await);
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 3, "genesis plus exactly two check-ins");
    }

    /// ⛔️ Only an author checks in, only a committed head of this document is materialized, a head the
    /// active checkpoint already passed is stale, and checking in the active baseline again is the
    /// same checkpoint rather than a second one.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn check_in_refuses_stale_unknown_and_foreign_heads_and_is_author_owned() {
        let fixture = check_in_fixture("refusals").await;
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_map_edits(&fixture, &["43", "44"]).await;
        let first = check_in_commit(&fixture, ledger[0].clone()).await;
        let second = check_in_commit(&fixture, ledger[1].clone()).await;
        let route = format!("/spaces/{}/documents/{}/check-ins", fixture.scope.space_id, fixture.scope.document_id);
        let request = |request_id: &str, head: &EditedArtifactFrontierV1| DocumentCheckInV1 { schema: "semio.hub.document-check-in/v1".into(), request_id: request_id.into(), head: head.clone() }.canonical_json().expect("canonical check-in");
        let body = request("33333333333333333333333333333333", &second);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Content-Type", "application/json")], body.as_bytes()).await.status, 401, "no credential");
        let spectator = format!("Bearer {}", fixture.spectator.token);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", spectator.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await.status, 403, "a spectator never writes a checkpoint");
        let author = format!("Bearer {}", fixture.author.token);
        let mut foreign = second.clone();
        foreign.document_id = "artifact-other".into();
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], request("33333333333333333333333333333333", &foreign).as_bytes()).await.status, 400, "a head of another document");
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], format!(" {body}").as_bytes()).await.status, 400, "a noncanonical body");

        let mut forged = second.clone();
        forged.chain_sha256 = format!("{}{}", if forged.chain_sha256.starts_with('a') { "b" } else { "a" }, &forged.chain_sha256[1..]);
        let (_, unknown) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "44444444444444444444444444444444", &forged).await;
        assert_eq!(unknown.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::UnknownHead))), "a forged chain is not a ledger point");
        let mut beyond = second.clone();
        beyond.head_edit_ordinal += 1;
        beyond.last_commit_seq += 1;
        let (_, unknown) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "55555555555555555555555555555555", &beyond).await;
        assert_eq!(unknown.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::UnknownHead))), "a head the ledger never reached");

        let (_, ready) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "66666666666666666666666666666666", &second).await;
        let ready = ready.and_then(|status| status.ready).expect("the head checks in");
        let (_, stale) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "77777777777777777777777777777777", &first).await;
        assert_eq!(stale.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::StaleHead))), "a head the active checkpoint passed is stale");
        let (_, again) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "88888888888888888888888888888888", &second).await;
        assert_eq!(again.and_then(|status| status.ready), Some(ready), "checking in the active baseline answers the same checkpoint");
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 2, "refusals and the repeat publish nothing");
    }

    /// 🔁️ A retried request answers its own job, the same request id naming another head conflicts,
    /// a Check In the author cancels before publication publishes nothing, and an author removed
    /// from the space mid-flight ends it `authority-changed` with nothing published.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn check_in_is_idempotent_per_request_cancellable_and_revoked_with_the_author() {
        let mut fixture = check_in_fixture("lifecycle").await;
        let gate = Arc::new(TestLiveGate::default());
        fixture.state.live_gate = Some(gate.clone());
        let addr = spawn_server(fixture.state.clone()).await;
        let ledger = check_in_map_edits(&fixture, &["43", "44"]).await;
        let first = check_in_commit(&fixture, ledger[0].clone()).await;
        let second = check_in_commit(&fixture, ledger[1].clone()).await;
        let route = format!("/spaces/{}/documents/{}/check-ins", fixture.scope.space_id, fixture.scope.document_id);
        let author = format!("Bearer {}", fixture.author.token);
        let post = |request_id: &'static str, head: EditedArtifactFrontierV1| {
            let route = route.clone();
            let author = author.clone();
            async move {
                let body = DocumentCheckInV1 { schema: "semio.hub.document-check-in/v1".into(), request_id: request_id.into(), head }.canonical_json().expect("canonical check-in");
                raw_http_request(addr, "POST", &route, &[("Authorization", author.as_str()), ("Content-Type", "application/json")], body.as_bytes()).await
            }
        };

        gate.check_in_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
        let accepted = post("99999999999999999999999999999999", second.clone()).await;
        assert_eq!(accepted.status, 202);
        tokio::time::timeout(std::time::Duration::from_secs(10), gate.check_in_admitted.acquire()).await.expect("check-in reaches its publication gate").expect("gate admission").forget();
        let joined = post("99999999999999999999999999999999", second.clone()).await;
        assert_eq!(joined.status, 202, "a retry joins the running job");
        let joined = DocumentCheckInStatusV1::parse_canonical_json(std::str::from_utf8(&joined.body).unwrap()).expect("joined status");
        assert_eq!(joined.phase, DocumentCheckInPhaseV1::Materializing);
        assert_eq!(joined.progress.completed_units, 5, "materialized, not yet published");
        assert_eq!(post("99999999999999999999999999999999", first.clone()).await.status, 409, "one request id names one request");
        let cancelled = raw_http_request(addr, "POST", &format!("{route}/99999999999999999999999999999999/cancel"), &[("Authorization", author.as_str())], &[]).await;
        assert!(cancelled.status == 200 || cancelled.status == 202);
        gate.check_in_release.add_permits(1);
        let (_, status) = check_in_until_terminal(addr, &fixture, &fixture.author.token, "99999999999999999999999999999999", &second).await;
        assert_eq!(status.map(|status| (status.phase, status.refusal)), Some((DocumentCheckInPhaseV1::Cancelled, None)));
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 1, "a cancelled check-in publishes nothing");

        assert_eq!(post("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", second.clone()).await.status, 202);
        tokio::time::timeout(std::time::Duration::from_secs(10), gate.check_in_admitted.acquire()).await.expect("second check-in reaches its gate").expect("gate admission").forget();
        execute_directory_command_fenced(&fixture.state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:check-in-removal".into() }, DirectoryCommand::RemoveMember { space_id: fixture.scope.space_id.clone(), user_id: fixture.author.user_id.clone() }).await.expect("remove the author");
        gate.check_in_release.add_permits(1);
        let key = DocumentCheckInKey { user_id: fixture.author.user_id.clone(), space_id: fixture.scope.space_id.clone(), document_id: fixture.scope.document_id.clone(), request_id: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into() };
        let job = fixture.state.check_ins.get(&key).expect("the revoked job is retained");
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !job.is_terminal() {
            assert!(std::time::Instant::now() < deadline, "the revoked check-in never ended");
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert_eq!((job.status().phase, job.status().refusal), (DocumentCheckInPhaseV1::Failed, Some(DocumentCheckInRefusalV1::AuthorityChanged)));
        assert_eq!(fixture.state.directory.artifact_checkpoint_count(&fixture.scope).await.expect("checkpoint count"), 1, "a revoked author publishes nothing");
        assert_eq!(raw_http_request(addr, "GET", &format!("{route}/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), &[("Authorization", author.as_str())], &[]).await.status, 403, "the removed member lost read access to the job too");
    }

    #[test]
    fn scoped_directory_socket_removal_and_delivery_have_one_total_membership_order() {
        run_socket_test(|| async {
            let mut state = test_state().await;
            let gate = Arc::new(TestLiveGate::default());
            state.live_gate = Some(gate.clone());
            let member = issue_test_session(&state, "scoped-order-target@example.com").await;
            upsert_member_for_test(&state, STUDIO, "scoped-order-target@example.com", DirectorySpaceRole::Spectator).await;
            let (order_document, _) = seed_genesis_document_for_test(&state, "scoped-order-author@example.com", STUDIO, "scoped-order-document").await;
            let addr = spawn_server(state.clone()).await;
            let authorization = format!("Bearer {}", member.token);
            let issue_path = format!("/directory/spaces/{STUDIO}/documents/{order_document}/socket-grants");

            let open = |grant: String, since: u64| {
                let url = format!("ws://{addr}/directory/spaces/{STUDIO}/documents/{order_document}/socket/v1?since={since}");
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
            tokio::time::timeout(std::time::Duration::from_secs(2), gate.socket_membership_remove_admitted.acquire()).await.expect("removal admission deadline").expect("removal admission").forget();
            gate.socket_scoped_send_mode.store(1, std::sync::atomic::Ordering::Release);
            publish_checkpoint_for_test(&state, STUDIO, &order_document).await;
            tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, gate.socket_scoped_send_admitted.acquire()).await.expect("the removal-wins sender gate was never admitted").expect("removal-wins sender").forget();
            gate.socket_scoped_send_release.add_permits(1);
            gate.socket_membership_remove_release.add_permits(1);
            tokio::time::timeout(std::time::Duration::from_secs(2), &mut removal).await.expect("removal-wins completion deadline").expect("removal task").expect("fenced removal");
            assert_eq!(next_close_code(&mut removal_wins, false).await, 4401, "removal winning the membership gate exposes no scoped event");

            upsert_member_for_test(&state, STUDIO, "scoped-order-target@example.com", DirectorySpaceRole::Spectator).await;
            gate.socket_membership_remove_enabled.store(false, std::sync::atomic::Ordering::Release);
            gate.socket_scoped_send_mode.store(0, std::sync::atomic::Ordering::Release);
            let mut delivery_wins = open(issue().await, state.directory.head_seq().await.expect("delivery-wins head")).await;
            gate.socket_scoped_send_mode.store(2, std::sync::atomic::Ordering::Release);
            let delivered_checkpoint = publish_checkpoint_for_test(&state, STUDIO, &order_document).await;
            tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, gate.socket_scoped_send_admitted.acquire()).await.expect("the delivery-wins sender gate was never admitted").expect("delivery-wins sender").forget();
            let mut removal = tokio::spawn({
                let state = state.clone();
                let user_id = member.user_id.clone();
                async move { execute_directory_command_fenced(&state, DirectoryActor { kind: DirectoryActorKind::System, id: "system:scoped-order-delivery".into() }, DirectoryCommand::RemoveMember { space_id: STUDIO.into(), user_id }).await }
            });
            let early_removal = tokio::time::timeout(std::time::Duration::from_millis(100), &mut removal).await;
            assert!(
                early_removal.is_err(),
                "removal waits while an admitted scoped send owns the membership gate; it instead returned {:?}",
                early_removal.map(|joined| joined.map(|fenced| fenced.map(|(events, _)| events.len())))
            );
            gate.socket_scoped_send_release.add_permits(1);
            let delivered = next_directory_message(&mut delivery_wins).await;
            assert!(
                matches!(
                    &delivered,
                    DirectoryStreamMessage::Event { event }
                        if matches!(event.body, os_directory::DirectoryEventBody::ArtifactCheckpointPublished { ref checkpoint }
                            if checkpoint.scope == DocumentScope::new(STUDIO, &order_document) && checkpoint.checkpoint_id == delivered_checkpoint.checkpoint_id)
                ),
                "the admitted scoped send is the exact published checkpoint: {delivered:?}"
            );
            tokio::time::timeout(std::time::Duration::from_secs(2), removal).await.expect("delivery-wins removal deadline").expect("removal task").expect("fenced removal");
            assert_eq!(next_close_code(&mut delivery_wins, false).await, 4401, "the one admitted event precedes the terminal membership close");
        });
    }

    #[tokio::test]
    async fn socket_admin_user_gate_rejects_a_late_same_user_grant_after_batch_revoke() {
        let mut state = test_state().await;
        let gate = Arc::new(TestLiveGate::default());
        gate.socket_admin_revoke_pause_enabled.store(true, std::sync::atomic::Ordering::Release);
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
                let body = Bytes::from(semio_framework_pack_json::to_json_string(&intent));
                admin_intents(admin_headers, loopback_peer(), State(state), body).await
            }
        });
        tokio::time::timeout(SOCKET_RENDEZVOUS_HANG_GUARD, gate.socket_admin_revoke_admitted.acquire()).await.expect("the admin revoke gate was never admitted").expect("admin gate").forget();
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
}

mod long {
use semio_hub_directory_contract::{ADMIN_PAGE_MAX, ADMIN_RESPONSE_MAX_BYTES, SocketGrantCapability};
use semio_hub_ledgers::guard::AdminPrincipalV1;
use std::sync::Arc;
use semio_hub_runtime::startup::{ArtifactCasMaintenanceCheckpoint, StartupCatalogControl};
use semio_hub_artifact_creation_service::ArtifactCreationServiceV1;
use semio_hub_directory_contract::model::{AuthSessionKind, SpaceRole};
use semio_hub_artifact_authority::{AuthorityLimits, OperationContext};
use semio_hub_route_testkit::{DocumentOpenPlanLedgerFixture, STUDIO, announce_document_for_test, bearer_headers, client_binary, create_space_for_test, document_open_plan_test_authority, document_socket_request, issue_test_session, next_server_frame, next_server_frame_at, next_server_frame_within, publish_checkpoint_for_test, raw_http_request, run_socket_test, sample_envelope, seed_author_token, seed_genesis_document_for_test, socket_hello, spawn_restartable_server, spawn_server, test_state, upsert_member_for_test};
use directory::os_directory::DocumentScope;
use semio_hub_ledgers::socket_grants::{SOCKET_GRANT_BINDING_PENDING_CAPACITY, SOCKET_GRANT_LEDGER_CAPACITY, SocketAudienceV1, SocketGrantKeyV1, SocketGrantLedgerErrorV1, SocketGrantLedgerV1, SocketSubjectV1};
use protocol::ArtifactId as WireArtifactId;
use semio_hub_route_models::admin::{admin_cursor_encode, admin_fit_connection_snapshot, admin_fit_page, admin_fit_space_detail};
use semio_hub_route_models::artifact_creation::artifact_creation_request_id_v1;
use tokio_tungstenite::connect_async;
use semio_hub_runtime::presence::db_artifact_id;
use semio_hub_runtime::support::now_ms;
use crate::tests::process_isolated_law;
#[cfg(test)]
use protocol::{AckStage, ActorId, ApplyOutcome, ClientFrame, Lane, ServerFrame};
#[cfg(test)]
use directory::os_directory::{AdminRecordedConnectionV1, DirectorySpaceRole, DirectorySpaceVisibility, MemberView, SpaceView};
#[cfg(test)]
use axum::extract::{Path, State};
#[cfg(test)]
use directory::os_directory::schema::space_artifact_creation::{SpaceArtifactCreateV1, SpaceArtifactCreationPhaseV1, SpaceArtifactCreationStatusV1};
#[cfg(test)]
use axum::http::StatusCode;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::issue_document_socket_grant_fixture;

    /// 📈️ A document socket keeps acknowledging commands however large its document grows: 150
    /// chained writes over one live socket, every one `Accepted`, past each former wall of the
    /// document store (the eighth index entry, the 128th replaced value). A hub used to go silent — no
    /// Ack, no error — once a document grew past one of them.
    #[test]
    fn a_document_socket_keeps_acknowledging_commands_as_its_document_grows() {
        run_socket_test(|| async {
            const EDITS: usize = 150;
            let state = test_state().await;
            let token = seed_author_token(&state).await;
            announce_document_for_test(&state, STUDIO, "socket-growth").await;
            let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), "socket-growth".to_string())), bearer_headers(&token), State(state.clone())).await.expect("growth socket grant").0;
            let addr = spawn_server(state.clone()).await;
            let (mut socket, _) = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{STUDIO}%2Fsocket-growth/document/ws"), &token)).await.expect("growth socket");
            socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("growth hello");
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
            assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
            let mut previous: Option<protocol::MutationId> = None;
            for index in 0..EDITS {
                let mut envelope = sample_envelope(&format!("growth-{index}"), &WireArtifactId("socket-growth".into())).await;
                envelope.actor = ActorId(receipt.actor_id.clone());
                envelope.diff.payload = db::document::encode_pathmap_json(&serde_json::json!({ format!("feature-{}", index % 8): format!("{index}:{}", "g".repeat(2048)) })).await.unwrap();
                envelope.dependencies = previous.iter().cloned().collect();
                let batch = 10_000 + index as u64;
                socket.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: vec![envelope.clone()] }, Lane::Command).await).await.expect("growth command");
                let outcome = tokio::time::timeout(std::time::Duration::from_secs(60), async {
                    loop {
                        if let ServerFrame::Ack { batch_id, stages, .. } = next_server_frame_at(&mut socket, "growth ack").await {
                            if batch_id == batch {
                                return stages;
                            }
                        }
                    }
                })
                .await
                .unwrap_or_else(|_| panic!("the hub never acknowledged growth edit {index}"));
                assert!(matches!(outcome.last(), Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted)), "growth edit {index}: {outcome:?}");
                previous = Some(envelope.mutation_id);
            }
            let frontier = state.db.document(&db_artifact_id(&DocumentScope::new(STUDIO, "socket-growth"))).await.expect("growth handle").frontier().await.expect("growth frontier");
            assert_eq!(frontier.commit_seq, EDITS as u64);
        });
    }

    /// 📨️ Every command a document socket receives is answered — accepted or refused with a reason —
    /// even when the database refuses writes: twelve sockets on twelve documents write concurrently
    /// until the process's shared credit refuses some of them, and no socket is ever left without the
    /// Ack for a batch it sent. A refused write that left its sender waiting forever looked, to a
    /// client, exactly like a hub that stopped listening.
    #[test]
    fn every_command_is_answered_when_the_database_refuses_writes() {
        if !process_isolated_law("tests::long::every_command_is_answered_when_the_database_refuses_writes") {
            return;
        }
        run_socket_test(|| async {
            const SOCKETS: usize = 12;
            const EDITS: usize = 16;
            let state = test_state().await;
            let token = seed_author_token(&state).await;
            let addr = spawn_server(state.clone()).await;
            let mut sockets = Vec::with_capacity(SOCKETS);
            for index in 0..SOCKETS {
                let document = format!("refusal-{index:02}");
                announce_document_for_test(&state, STUDIO, &document).await;
                let receipt = issue_document_socket_grant_fixture(Path((STUDIO.to_string(), document.clone())), bearer_headers(&token), State(state.clone())).await.expect("refusal socket grant").0;
                let (mut socket, _) = connect_async(document_socket_request(&format!("ws://{addr}/scopes/{STUDIO}%2F{document}/document/ws"), &token)).await.expect("refusal socket");
                socket.send(client_binary(&socket_hello(), Lane::Command).await).await.expect("refusal hello");
                assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Welcome { .. }));
                assert!(matches!(next_server_frame(&mut socket).await, ServerFrame::Session { .. }));
                sockets.push((document, receipt.actor_id, socket));
            }
            let outcomes = futures::future::join_all(sockets.into_iter().map(|(document, actor, mut socket)| async move {
                let mut accepted = 0usize;
                let mut refused = Vec::new();
                let mut previous: Option<protocol::MutationId> = None;
                for edit in 0..EDITS {
                    let mut envelope = sample_envelope(&format!("{document}-{edit}"), &WireArtifactId(document.clone())).await;
                    envelope.actor = ActorId(actor.clone());
                    envelope.diff.payload = db::document::encode_pathmap_json(&serde_json::json!({ format!("path-{edit:02}"): format!("{edit}:{}", "r".repeat(1024)) })).await.unwrap();
                    envelope.dependencies = previous.iter().cloned().collect();
                    let batch = 20_000 + edit as u64;
                    socket.send(client_binary(&ClientFrame::Commands { batch_id: batch, envelopes: vec![envelope.clone()] }, Lane::Command).await).await.expect("refusal command");
                    let stages = loop {
                        if let ServerFrame::Ack { batch_id, stages, .. } = next_server_frame_within(&mut socket, &format!("{document} edit {edit} ack"), std::time::Duration::from_secs(40)).await {
                            if batch_id == batch {
                                break stages;
                            }
                        }
                    };
                    match stages.last() {
                        Some(AckStage::Applied { outcome }) if matches!(outcome.as_ref(), ApplyOutcome::Accepted) => {
                            accepted += 1;
                            previous = Some(envelope.mutation_id);
                        }
                        Some(AckStage::Applied { outcome }) => match outcome.as_ref() {
                            ApplyOutcome::Rejected { reason, .. } => refused.push(reason.clone()),
                            other => panic!("{document} edit {edit}: {other:?}"),
                        },
                        other => panic!("{document} edit {edit}: {other:?}"),
                    }
                }
                (document, accepted, refused)
            }))
            .await;
            let answered: usize = outcomes.iter().map(|(_, accepted, refused)| accepted + refused.len()).sum();
            assert_eq!(answered, SOCKETS * EDITS, "every command was answered: {outcomes:?}");
        });
    }

    /// 🌱️ Server-owned creation on the GIS plugin's real release component: the catalog is author-only,
    /// a stale generation or unknown kind claims nothing, two concurrent duplicates own one factory, and
    /// the guest's own genesis reaches `ready` with a descriptor and a genesis-backed active checkpoint.
    #[cfg(feature = "integration-fixtures")]
    #[tokio::test]
    async fn space_artifact_creation_routes_are_author_owned_idempotent_and_genesis_backed() {
        use semio_hub_artifact_creation::ArtifactCreationOperationV1;
        use semio_hub_trusted_catalog_fixtures as trusted_catalog_fixture;
        use directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationCatalogV1;

        let profile = trusted_catalog_fixture::verified_gis_map_release_profile(&trusted_catalog_fixture::unique_profile_root("artifact-creation-http")).await.expect("verified GIS Map creation profile on the real release component");
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

        let request = SpaceArtifactCreateV1 {
            schema: "semio.hub.space-artifact-create/v1".into(),
            request_id: "1234567890abcdef1234567890abcdef".into(),
            expected_catalog_generation_id: catalog.catalog_generation_id.clone(),
            kind_id: "s.gis.gismap".into(),
            name: "Shared Map".into(),
        };
        let body = semio_framework_pack_json::to_json_string(&request);
        let malformed = body.replacen("{", "{\"documentId\":\"caller-owned\",", 1);
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], malformed.as_bytes()).await.status, 400);
        let unknown = SpaceArtifactCreateV1 { request_id: "2234567890abcdef1234567890abcdef".into(), kind_id: "s.gis.unknown".into(), ..request.clone() };
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], semio_framework_pack_json::to_json_string(&unknown).as_bytes()).await.status, 409);
        let stale_generation = SpaceArtifactCreateV1 { request_id: "3234567890abcdef1234567890abcdef".into(), expected_catalog_generation_id: "9".repeat(64), ..request.clone() };
        assert_ne!(stale_generation.expected_catalog_generation_id, catalog.catalog_generation_id);
        let directory_head = state.directory.head_seq().await.expect("directory head before stale catalog creation");
        assert_eq!(raw_http_request(addr, "POST", &route, &[("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")], semio_framework_pack_json::to_json_string(&stale_generation).as_bytes()).await.status, 409);
        assert!(state.directory.read_artifact_creation(&author.user_id, &stale_generation.request_id).await.expect("stale catalog creation facts").is_empty());
        assert_eq!(state.directory.head_seq().await.expect("directory head after stale catalog creation"), directory_head, "a stale catalog generation cannot claim an operation or append directory events");
        let creation_headers = [("Authorization", author_bearer.as_str()), ("Content-Type", "application/json")];
        let first = raw_http_request(addr, "POST", &route, &creation_headers, body.as_bytes());
        let duplicate = raw_http_request(addr, "POST", &route, &creation_headers, body.as_bytes());
        let (first, duplicate) = tokio::join!(first, duplicate);
        assert!([200, 202].contains(&first.status) && [200, 202].contains(&duplicate.status), "exact concurrent duplicate never reports capacity or owns a second factory");
        for response in [&first, &duplicate] {
            let source = std::str::from_utf8(&response.body).expect("creation acceptance UTF-8");
            let status = SpaceArtifactCreationStatusV1::parse_canonical_json(source).expect("creation acceptance is canonical");
            assert_eq!(status.catalog_generation_id, request.expected_catalog_generation_id);
        }
        let facts = state.directory.read_artifact_creation(&author.user_id, &request.request_id).await.expect("durable creation facts");
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts).expect("one durable creation operation");
        assert!(operation.intent.scope.document_id.strip_prefix("artifact-").is_some_and(artifact_creation_request_id_v1));
        let created_document_id = operation.intent.scope.document_id;

        let status_route = format!("{route}/{}", request.request_id);
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(600);
        let ready = loop {
            let response = raw_http_request(addr, "GET", &status_route, &[("Authorization", author_bearer.as_str())], &[]).await;
            let status = SpaceArtifactCreationStatusV1::parse_canonical_json(std::str::from_utf8(&response.body).expect("creation status UTF-8")).expect("canonical creation status");
            let running = matches!(status.phase, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing);
            assert_eq!(response.status, if running { 202 } else { 200 }, "a running creation answers 202, a terminal one 200");
            assert_eq!(status.catalog_generation_id, request.expected_catalog_generation_id);
            if status.phase == SpaceArtifactCreationPhaseV1::Ready {
                break status;
            }
            assert!(matches!(status.phase, SpaceArtifactCreationPhaseV1::Accepted | SpaceArtifactCreationPhaseV1::Preparing | SpaceArtifactCreationPhaseV1::Indeterminate), "creation reached an unexpected terminal phase");
            assert!(tokio::time::Instant::now() < deadline, "genesis on the real GIS guest did not become Ready");
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        };
        let ready_scope = DocumentScope::new(&space_id, &ready.ready.as_ref().expect("Ready coordinates").artifact_id);
        assert_eq!(ready.catalog_generation_id, request.expected_catalog_generation_id);
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
            session_kind: AuthSessionKind::DevelopmentLocal,
            device_instance_id: "admin-device".into(),
        };
        let rows = (0..ADMIN_PAGE_MAX).map(|index| os_directory::UserView { id: format!("user:{index}:{}", "i".repeat(4_000)), email: format!("{index}@{}", "e".repeat(4_000)), display_name: "n".repeat(4_000), created_at_ms: 0 }).collect();
        let page = admin_fit_page(rows, false, 7, |rows| admin_cursor_encode(&cursor_key, &principal, 2, rows.len())).expect("byte-bounded user page");
        assert!(page.rows.len() < ADMIN_PAGE_MAX);
        assert!(page.next_cursor.is_some());
        assert!(semio_framework_pack_json::to_json_string(&page).len() <= ADMIN_RESPONSE_MAX_BYTES);

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
        assert!(semio_framework_pack_json::to_json_string(&snapshot).len() <= ADMIN_RESPONSE_MAX_BYTES);

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
        assert!(semio_framework_pack_json::to_json_string(&detail).len() <= ADMIN_RESPONSE_MAX_BYTES);

        let oversized = vec![os_directory::UserView { id: "i".repeat(ADMIN_RESPONSE_MAX_BYTES), email: "e@example.com".into(), display_name: "name".into(), created_at_ms: 0 }];
        assert_eq!(admin_fit_page(oversized, false, 7, |_| Ok("a".repeat(84))), Err(StatusCode::PAYLOAD_TOO_LARGE));
    }

    #[tokio::test]
    async fn socket_grant_ledger_is_bounded_single_consume_restart_scoped_and_revoke_race_safe() {
        let ledger = Arc::new(SocketGrantLedgerV1::default());
        let audience = SocketAudienceV1::DirectoryScoped(DocumentScope::new("space-a", "document-a"));
        let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Author), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-a".into() };
        let capability = SocketGrantCapability::mint().expect("socket grant");
        ledger.issue(&capability, audience.clone(), "hub.v1.actor".into(), subject.clone(), 1, 9_000).expect("issue grant");
        assert!(ledger.pending(&capability, &SocketAudienceV1::DirectoryScoped(DocumentScope::new("space-a", "document-b")), 2).is_err(), "audience mismatch never consumes");
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
        assert!(bounded.issue(&overflow, SocketAudienceV1::DirectoryScoped(DocumentScope::new("space-a", "document-a")), "hub.v1.after-revoke".into(), subject, 2, 9_000).is_ok());
    }

    #[tokio::test]
    async fn document_socket_grants_are_credential_bound_secret_free_oldest_first_and_single_consume() {
        let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
        let plan = Arc::new(document_open_plan_test_authority(&fixture));
        let ledger = SocketGrantLedgerV1::default();
        let audience = SocketAudienceV1::Document(DocumentScope::new("space-a", "document-a"));
        let subject = SocketSubjectV1::Session { session_id: "session-a".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Author), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-a".into() };
        let other = SocketSubjectV1::Session { session_id: "session-b".into(), user_id: "user-a".into(), authorization_generation: 7, role: Some(SpaceRole::Author), expires_at_ms: 10_000, session_kind: AuthSessionKind::External, device_instance_id: "device-b".into() };
        assert_eq!(ledger.issue(&SocketGrantCapability::mint().expect("capability"), audience.clone(), "hub.v1.capability".into(), subject.clone(), 1, 9_000), Err(SocketGrantLedgerErrorV1::Rejected), "document audiences never answer to a capability");
        assert_eq!(ledger.admit_document(SocketAudienceV1::DirectoryScoped(DocumentScope::new("space-a", "document-a")), "hub.v1.scoped".into(), subject.clone(), 1, 9_000, plan.clone()), Err(SocketGrantLedgerErrorV1::Rejected), "only document audiences are credential-bound");
        let older = ledger.admit_document(audience.clone(), "hub.v1.older".into(), subject.clone(), 1, 9_000, plan.clone()).expect("older admission");
        let newer = ledger.admit_document(audience.clone(), "hub.v1.newer".into(), subject.clone(), 2, 9_000, plan.clone()).expect("newer admission");
        assert_ne!(older, newer);
        assert!(ledger.pending_document_binding(&audience, &other.binding(), 3).is_err(), "another credential never consumes this binding's grant");
        assert!(ledger.pending_document_binding(&SocketAudienceV1::Document(DocumentScope::new("space-a", "document-b")), &subject.binding(), 3).is_err(), "audience is exact");
        let first = ledger.pending_document_binding(&audience, &subject.binding(), 3).expect("oldest pending");
        assert_eq!((first.selector.as_str(), first.actor_id.as_str(), first.key), (older.as_str(), "hub.v1.older", SocketGrantKeyV1::CredentialBinding));
        ledger.consume(&first, 4).expect("single consume");
        assert!(ledger.consume(&first, 5).is_err(), "consumed admissions never replay");
        let second = ledger.pending_document_binding(&audience, &subject.binding(), 5).expect("next pending");
        assert_eq!(second.selector, newer);
        ledger.invalidate_binding(subject.binding());
        assert!(ledger.pending_document_binding(&audience, &subject.binding(), 6).is_err(), "revoke drops pending admissions");
        assert!(ledger.admit_document(audience.clone(), "hub.v1.expired".into(), subject.clone(), 7, 8, plan).is_ok());
        assert!(ledger.pending_document_binding(&audience, &subject.binding(), 9).is_err(), "expired admissions never consume");
    }

    #[tokio::test]
    async fn artifact_cas_maintenance_checkpoint_reaches_tail_after_sixteen_requests() {
        let state = test_state().await;
        for index in 0..5 {
            let space_id = create_space_for_test(&state, "seed", &format!("CAS {index}"), os_directory::DirectorySpaceKind::Studio, DirectorySpaceVisibility::Private).await;
            let (document_id, _) = seed_genesis_document_for_test(&state, "cas-maintenance-author@example.com", &space_id, &format!("cas-maintenance-{index}")).await;
            publish_checkpoint_for_test(&state, &space_id, &document_id).await;
        }
        let control = StartupCatalogControl::silent();
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
}
