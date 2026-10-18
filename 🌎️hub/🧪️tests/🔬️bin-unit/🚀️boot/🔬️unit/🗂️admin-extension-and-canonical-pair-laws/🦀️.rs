/// 🗂️ Admin, extension and canonical pair laws of the hub boot: the declared admin pages, the encoded extension module routes, the canonical pair header, member and progress laws and the frontierless hello resume.

#[tokio::test]
async fn admin_page_routes_follow_declared_html_without_alias_files() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🚧️hub-boundaries/🔣️.json")).expect("hub boundary fixture");
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🚧️hub-boundaries/🔣️.json")).expect("hub boundary fixture");
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
