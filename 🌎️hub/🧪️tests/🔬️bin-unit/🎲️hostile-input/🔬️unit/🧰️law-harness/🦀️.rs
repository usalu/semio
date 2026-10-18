/// 🧰️ Law harness: the process-isolated law protocol and the check-in fixtures, commits, replica pairs and cold pairs.

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
    use directory::os_store::{store_schema::ArtifactCommand, ArtifactStore};
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
    while !store.close_owned_terminal_is_empty() {
        let demand = store.close_owned_demands(0).expect("the store quotes its next close turn");
        let grant = directory::os_store::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        store.close_owned_step(grant).expect("editor store closes");
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
