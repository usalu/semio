/// 🛣️ Laws of the plan issue and exchange routes, socket consume, late invalid receipts and admin revocation.

#[tokio::test]
async fn document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable() {
    let _watchdog = LawHangWatchdogV1::arm("document_open_plan_issue_route_is_catalog_bound_authenticated_bounded_cancel_safe_and_exchangeable");
    let mut state = test_state().await;
    let document_id = artifact_document_id_for_test("open-plan-issue");
    let token = seed_author_token(&state).await;
    let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
    let scope = DocumentScope::new(STUDIO, &document_id);
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
    let authorization = format!("Bearer {token}");
    let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
    let plan_route = format!("/spaces/{STUDIO}/documents/{document_id}/open-plan");
    let grant_route = format!("/spaces/{STUDIO}/documents/{document_id}/socket-grants");
    let intent_body = |surface: &str, client: &str| {
        semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: Some(surface.into()), client_instance_id: client.into() })
    };
    let grant_body = |receipt: &str| semio_framework_pack_json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: receipt.into() });
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
    let plan: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(&success_text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("plan JSON");
    assert_eq!(plan.scope, scope);
    assert_eq!(plan.catalog.generation_id, state.openable_catalog.as_ref().expect("catalog").generation_id());
    assert_eq!(plan.parent_dialect, DocumentOpenParentDialectV1 { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() });
    assert_eq!(plan.surface.surface_id, "surface.test.editor");
    assert!(plan.grant.write);
    assert!(plan.expires_at_unix_ms.saturating_sub(u64::try_from(now_ms()).expect("time")) <= DOCUMENT_OPEN_PLAN_MAX_TTL_MS);
    let exchange = raw_http_request(addr, "POST", &grant_route, &headers, grant_body(&plan.receipt).as_bytes()).await;
    assert_eq!(exchange.status, 200);
    let exchange_json: serde_json::Value = serde_json::from_slice(&exchange.body).expect("exchange JSON");
    assert_eq!(exchange_json["schema"], "semio.hub.document-socket-grant/v1");
    assert!(exchange_json.get("grant").is_none(), "a document socket grant carries no capability");
    assert!(!String::from_utf8(exchange.body).expect("exchange UTF-8").contains(&plan.receipt));

    let foreign_surface = raw_http_request(addr, "POST", &plan_route, &headers, intent_body("surface.foreign", "client:foreign").as_bytes()).await;
    assert_eq!(foreign_surface.status, 503);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&foreign_surface.body).expect("foreign error")["code"], "component-unavailable");
    let hostile = format!(r#"{{"schema":"semio.hub.document-open-intent/v1","version":1,"scope":{{"spaceId":"{STUDIO}","documentId":"{document_id}"}},"clientInstanceId":"client","actor":"caller"}}"#);
    assert_eq!(raw_http_request(addr, "POST", &plan_route, &headers, hostile.as_bytes()).await.status, 400);
    assert_eq!(raw_http_request(addr, "POST", &format!("{plan_route}?surface=surface.test.editor"), &headers, intent_body("surface.test.editor", "client:query").as_bytes()).await.status, 400);
    let wrong_scope = semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
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
    let share_plan: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(&String::from_utf8(share_response.body).expect("share plan UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("share plan");
    assert_eq!(share_plan.parent_dialect, plan.parent_dialect);
    assert!(!share_plan.grant.write);
    assert_eq!(share_plan.surface.role, os_directory::DocumentOpenSurfaceRoleV1::Viewer);
    assert!(share_plan.revalidation.session_generation.is_none());
    assert_eq!(share_plan.revalidation.share_generation, Some(1));
    assert_eq!(raw_http_request(addr, "POST", &grant_route, &share_headers, grant_body(&share_plan.receipt).as_bytes()).await.status, 200);

    let mut unavailable = state.clone();
    unavailable.openable_catalog = None;
    unavailable.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, false, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
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

    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open fixture");
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
        authority.descriptor_digest_v1 = os_directory::io::binary::artifact_hash::hex_lower(&os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&authority.descriptor).expect("capacity digest").0);
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
    cancelled.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
    let mut cancelled_headers = bearer_headers(&cancelled_token);
    cancelled_headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
    let gate = Arc::new(TestDocumentOpenPlanIssueGate::default());
    cancelled.document_open_plan_issue_gate = Some(gate.clone());
    let cancelled_body = Bytes::from(semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
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
    let _watchdog = LawHangWatchdogV1::arm("document_open_plan_socket_consume_revalidates_surface_descriptor_catalog_revision_and_checkpoint");
    let mut state = test_state().await;
    let document_id = artifact_document_id_for_test("open-plan-consume");
    let token = seed_author_token(&state).await;
    let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
    let scope = DocumentScope::new(STUDIO, &document_id);
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));

    let (subject, _) = authenticate_document_credential(&state, &scope, &token).await.expect("document credential");
    let audience = SocketAudienceV1::Document(scope.clone());
    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:surface").await;
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.viewer")).await, Err((StatusCode::UNAUTHORIZED, _))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "surface substitution terminally rejects the pending grant");

    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:checkpoint").await;
    let checked_in = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).expect("pending grant before the Check In");
    publish_checkpoint_for_test(&state, STUDIO, &document_id).await;
    assert_eq!(document_plan_socket_validity(&state, &checked_in, Some("surface.test.editor")).await, SocketBindingValidityV1::Active, "a Check In changes nobody's authority");
    assert_eq!(document_plan_bootstrap_current(&state, &checked_in).await, DocumentPlanBootstrapV1::Stale);
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.editor")).await, Err((StatusCode::CONFLICT, "socket-grant-stale"))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "a checkpoint published since the exchange rejects the pending grant as stale");

    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:catalog").await;
    let rotated = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).expect("pending grant before the catalog publication");
    state.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
    assert_eq!(document_plan_socket_validity(&state, &rotated, Some("surface.test.editor")).await, SocketBindingValidityV1::Active, "a catalog generation with the same selection changes nobody's authority");
    assert_eq!(document_plan_bootstrap_current(&state, &rotated).await, DocumentPlanBootstrapV1::Stale);
    assert!(matches!(consume_document_socket_grant(&state, &subject, audience.clone(), Some("surface.test.editor")).await, Err((StatusCode::CONFLICT, "socket-grant-stale"))));
    assert!(state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).is_err(), "a catalog generation published since the exchange rejects the pending grant as stale");

    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    issue_and_exchange_document_open_plan_for_test(&state, &token, &scope, "client:exact").await;
    let pending = state.socket_grants.pending_document_binding(&audience, &subject.binding(), now_ms()).expect("exact pending grant");
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
    let admission = consume_document_socket_grant(&state, &subject, audience, Some("surface.test.editor")).await.expect("exact current authority consumes");
    assert_eq!(admission.record.document_plan.as_deref(), Some(authority.as_ref()));
}

#[tokio::test]
async fn document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use() {
    let _watchdog = LawHangWatchdogV1::arm("document_open_plan_exchange_route_is_authenticated_exact_hostile_and_single_use");
    let mut state = test_state().await;
    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let token = seed_author_token(&state).await;
    let document_id = artifact_document_id_for_test("open-plan-route");
    let document_id = document_id.as_str();
    let foreign_document_id = artifact_document_id_for_test("open-plan-route-foreign");
    let foreign_document_id = foreign_document_id.as_str();
    seed_genesis_for_document_for_test(&state, &token, STUDIO, document_id, "open-plan-route").await;
    seed_genesis_for_document_for_test(&state, &token, STUDIO, foreign_document_id, "open-plan-route-foreign").await;
    let route_descriptor = state.directory.get_document_descriptor(&DocumentScope::new(STUDIO, document_id)).await.expect("route descriptor").expect("route document");
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&route_descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
    let scope = DocumentScope::new(STUDIO, document_id);
    let mut authority = document_open_plan_authority_for_session(&state, &fixture, &token, scope.clone()).await;
    let addr = spawn_server(state.clone()).await;
    let route = format!("/spaces/{STUDIO}/documents/{document_id}/socket-grants");
    let authorization = format!("Bearer {token}");
    let request_headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
    let request_body = |receipt: &str| semio_framework_pack_json::to_json_string(&DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: receipt.into() });
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
    assert_eq!(success_json["schema"], "semio.hub.document-socket-grant/v1");
    assert_eq!(success_json["protocol"], SESSION_PROTOCOL_V1);
    assert_eq!(success_json["actorId"], authority.server_actor_id);
    assert!(success_json.get("grant").is_none(), "a document socket grant carries no capability");
    let pending = state.socket_grants.pending_document_binding(&SocketAudienceV1::Document(scope.clone()), &authority.subject.binding(), now_ms()).expect("route-bound pending grant");
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
    assert!(share_json.get("grant").is_none());
    let share_pending = state.socket_grants.pending_document_binding(&SocketAudienceV1::Document(scope), &share_authority.subject.binding(), now_ms()).expect("share pending grant");
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
    let watchdog = LawHangWatchdogV1::arm("document_open_plan_admin_revocation_invalidates_session_and_share_bindings");
    watchdog.at("hub state open");
    let mut state = test_state().await;
    let fixture: DocumentOpenPlanLedgerFixture = semio_framework_pack_json::from_json_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document open plan fixture");
    let admin_headers = authorize_test_admin(&mut state, "open-plan-admin@example.com").await;
    let mut principal = authenticate_admin_principal(&state, &admin_headers, Some(loopback_peer().0)).await.expect("verified open-plan administrator principal");
    principal.correlation_id = "open-plan-admin-revocation".into();

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
        session_kind: session_record.session_kind,
        device_instance_id: session_record.device_instance_id.clone(),
    };
    let session_plan = state.document_open_plans.issue_with_capability(session_authority.clone(), session_now, session_now + 100, DocumentOpenPlanCapabilityV1::from_secret(document_open_plan_secret(20))).expect("session plan");
    let session_intent = AdminIntentV1::RevokeUserSessions { request_id: "request:open-plan-session-revoke".into(), user_id: session_record.user_id, reason_code: "test-revoke".into() };
    let session_digest = admin_intent_digest(&session_intent);
    let mut session_revoke_authority = None;
    watchdog.at("session revocation");
    let session_revoke = execute_admin_intent(&state, &principal, "operation:open-plan-session-revoke", &session_digest, session_intent, None, &mut session_revoke_authority).await;
    drop(session_revoke_authority);
    assert_eq!(session_revoke.phase, "succeeded");
    assert_eq!(state.document_open_plans.exchange(&session_plan.receipt, &session_authority, session_now + 1, "socket-after-session-revoke"), Err(DocumentOpenPlanErrorCodeV1::Stale));

    watchdog.at("share space and document announce");
    let space_effect = admin_effect_receipt_claim("operation:open-plan-share-space", &"0".repeat(64), "directory-events-appended");
    assert!(
        matches!(
            state
                .directory_service
                .execute_create_space_with_id_and_admin_effect(
                    principal.event_actor(),
                    fixture.valid_plan.scope.space_id.clone(),
                    "Open plan share".into(),
                    os_directory::DirectorySpaceKind::Studio,
                    DirectorySpaceVisibility::Private,
                    &space_effect
                )
                .await,
            AdminEffectCommitV1::Applied(_)
        ),
        "the fixture's own space carries the plan a share is revoked on"
    );
    state
        .directory_service
        .execute(DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#open-plan-test", principal.user_id) }, DirectoryCommand::AnnounceDocument { descriptor: Box::new(fixture.descriptor.clone()) })
        .await
        .expect("announce the fixture's plan document");
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
    watchdog.at("share revocation");
    let share_revoke = execute_admin_intent(&state, &principal, "operation:open-plan-share-revoke", &share_digest, share_intent, None, &mut share_revoke_authority).await;
    drop(share_revoke_authority);
    assert_eq!(share_revoke.phase, "succeeded");
    assert_eq!(state.document_open_plans.exchange(&share_plan.receipt, &share_authority, share_now + 1, "socket-after-share-revoke"), Err(DocumentOpenPlanErrorCodeV1::Stale));
    watchdog.at("hub state teardown");
    drop(state);
}
