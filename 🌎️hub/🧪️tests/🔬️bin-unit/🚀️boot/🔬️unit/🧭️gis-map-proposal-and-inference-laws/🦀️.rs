// 🧭️ Gis map proposal and inference laws of the hub boot: the proposal owner streams and cancellation, the reconciliation route, the actual codec work close, the late publication refusal, the proposal privacy and the idempotent approval.

//#region 💡️Inference
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
