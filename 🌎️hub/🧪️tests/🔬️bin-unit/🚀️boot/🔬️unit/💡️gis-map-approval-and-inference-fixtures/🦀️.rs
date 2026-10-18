// 💡️ Gis map approval and inference fixtures of the hub boot: the approval ingress authority, the committed approval receipt, the applied checkpoint rebootstrap notification, the fail closed proposal routes and the inference fixture helpers.

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
//#endregion 💡️Inference
