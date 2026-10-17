/// 🎯️ Laws of execution-target asset routes, the selection final fence and refusals without genesis.

/// 🪪️ Every execution-target asset route re-authenticates the exact scope and role, reloads the
/// durable descriptor, and resolves the current trusted selection before any body: it accepts
/// only the bounded open intent, never a package/digest/generation/path/receipt selector, serves
/// exactly the selected bytes, and denies after a catalog rotation.
///
/// 🚫 Unauthenticated, foreign-scope, foreign-surface, query-smuggled, oversized and
/// non-JSON requests never reach a byte.
///
/// 🔁 A rotation between reads denies every subsequent body rather than mixing generations.
///
/// 🚧 An unconfigured catalog advertises nothing and serves nothing.
#[tokio::test]
async fn execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body() {
    let mut state = test_state().await;
    let document_id = artifact_document_id_for_test("execution-target-assets");
    let token = seed_author_token(&state).await;
    let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
    let scope = DocumentScope::new(STUDIO, &document_id);
    state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
    let authorization = format!("Bearer {token}");
    let headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
    let root = format!("/spaces/{STUDIO}/documents/{document_id}/execution-target");
    let intent_body = |surface: &str| {
        semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some(surface.into()),
            client_instance_id: "client:execution-target".into(),
        })
    };
    let addr = spawn_server(state.clone()).await;

    let manifest = raw_http_request(addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(manifest.status, 200, "{}", String::from_utf8_lossy(&manifest.body));
    let manifest_text = String::from_utf8(manifest.body).expect("manifest UTF-8");
    assert!(!manifest_text.contains(&token) && !manifest_text.contains("client:execution-target") && !manifest_text.contains("sessionId") && !manifest_text.contains("receipt"));
    let fields: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&manifest_text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("manifest JSON");
    fields.validate().expect("route manifest is a valid lease projection");
    assert_eq!(fields.scope, scope);
    assert_eq!(fields.catalog.generation_id, state.openable_catalog.as_ref().expect("catalog").generation_id());
    assert_eq!(fields.component.byte_length, TEST_EXECUTION_TARGET_COMPONENT_BYTES.len() as u64);
    assert_eq!(fields.descriptor.byte_length, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.len() as u64);

    let component = raw_http_request(addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(component.status, 200);
    assert_eq!(component.body, TEST_EXECUTION_TARGET_COMPONENT_BYTES);

    let asset_class = StreamLimitClassV1::ExecutionTargetAsset;
    let mut ruled = state.clone();
    ruled.asset_streams = Arc::new(HubStreamLimiterV1::with_policies([(asset_class, semio_hub_auth::rate_limit::StreamLimitPolicyV1 { hub_streams: 1, principal_streams: 1, admission_wait_ms: 50, retry_after_ms: 1_000 })]));
    let ruled_addr = spawn_server(ruled.clone()).await;
    let held = ruled.asset_streams.admit(asset_class, RateLimitSubjectV1::principal("another-principal")).await.expect("the one hub-wide asset stream");
    let busy = raw_http_request(ruled_addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(busy.status, 429, "a component whose stream rule stays taken is the typed refusal");
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&busy.body).expect("refusal JSON"), serde_json::to_value(RateLimitRefusalV1::streams(asset_class, 1_000)).unwrap());
    assert!(busy.headers.to_ascii_lowercase().contains("retry-after: 1"), "{}", busy.headers);
    assert_eq!(raw_http_request(ruled_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await.status, 200, "the lease is no asset stream");
    drop(held);
    let admitted = raw_http_request(ruled_addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!((admitted.status, admitted.body.as_slice()), (200, TEST_EXECUTION_TARGET_COMPONENT_BYTES), "a freed stream slot admits the component");
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while ruled.asset_streams.live_streams(asset_class) != 0 {
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("a finished component body frees its stream slot");
    let descriptor_body = raw_http_request(addr, "POST", &format!("{root}/descriptor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(descriptor_body.status, 200);
    assert_eq!(descriptor_body.body, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES);

    let corpus: serde_json::Value = serde_json::from_str(semio_hub_testing::hub_source("../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("closed actor neutral corpus");
    let mut closed_selection = state.openable_catalog.as_ref().unwrap().resolve_document_open(&descriptor, Some("surface.test.editor"), true).unwrap();
    closed_selection.surface.renderer_target = os_directory::DocumentOpenRendererTargetV1::Wasm;
    let mut actor_json = corpus["plan"]["browserActor"].clone();
    actor_json["sha256"] = serde_json::json!(os_directory::io::binary::artifact_hash::hex_lower(&Sha256::digest(b"abc")));
    actor_json["sourceComponentSha256"] = serde_json::json!(closed_selection.package.component_sha256);
    actor_json["sourceDescriptorByteSha256"] = serde_json::json!(closed_selection.package.descriptor_byte_sha256);
    closed_selection.browser_actor = semio_framework_pack_json::from_json_str(&actor_json.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed actor fixture");
    let expected_actor = closed_selection
        .browser_actor
        .to_lease(os_directory::DocumentBrowserActorSourceV1 { component_sha256: &closed_selection.package.component_sha256, descriptor_byte_sha256: &closed_selection.package.descriptor_byte_sha256 }, "wasm", Some(3))
        .expect("closed actor lease");
    let mut closed_state = state.clone();
    closed_state.openable_catalog = Some(Arc::new(TestDocumentOpenCatalog {
        generation_id: "88".repeat(32),
        open_targets: vec![closed_selection.clone()].into_boxed_slice(),
        component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
        descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
        browser_actor: Some(Arc::from(&b"abc"[..])),
    }));
    let closed_addr = spawn_server(closed_state.clone()).await;
    let closed_manifest = raw_http_request(closed_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(closed_manifest.status, 200);
    let closed_text = String::from_utf8(closed_manifest.body).expect("closed manifest UTF-8");
    let closed_fields: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&closed_text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed manifest");
    closed_fields.validate().expect("closed manifest bound");
    assert_eq!(closed_fields.browser_actor, expected_actor);
    for private in ["path", "moduleUrl", "actorBytes", "receipt", "sessionId", "client:execution-target"] {
        assert!(!closed_text.contains(&format!("\"{private}\"")));
    }
    assert!(!closed_text.contains(&token));
    let closed_plan = raw_http_request(closed_addr, "POST", &format!("/spaces/{STUDIO}/documents/{document_id}/open-plan"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(closed_plan.status, 200);
    let closed_plan: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(std::str::from_utf8(&closed_plan.body).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("closed plan");
    assert_eq!(closed_plan.browser_actor, closed_selection.browser_actor);
    let actor_body = raw_http_request(closed_addr, "POST", &format!("{root}/browser-actor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(actor_body.status, 200);
    assert_eq!(actor_body.body, b"abc");
    let absent_actor = raw_http_request(addr, "POST", &format!("{root}/browser-actor"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(absent_actor.status, 503);
    assert_ne!(absent_actor.body, b"abc");
    for (selection, browser_actor) in [(closed_selection, None), (state.openable_catalog.as_ref().unwrap().resolve_document_open(&descriptor, Some("surface.test.editor"), true).unwrap(), Some(Arc::from(&b"abc"[..])))] {
        let mut invalid_state = state.clone();
        invalid_state.openable_catalog = Some(Arc::new(TestDocumentOpenCatalog {
            generation_id: "99".repeat(32),
            open_targets: vec![selection].into_boxed_slice(),
            component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
            descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
            browser_actor,
        }));
        let invalid_addr = spawn_server(invalid_state).await;
        let invalid = raw_http_request(invalid_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(invalid.status, 503, "actor identity/body presence mismatch");
    }

    for asset in ["manifest", "component", "descriptor", "browser-actor"] {
        let route = format!("{root}/{asset}");
        let anonymous = raw_http_request(addr, "POST", &route, &[("Content-Type", "application/json")], intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(anonymous.status, 401, "{asset} served an unauthenticated caller");
        let foreign_scope = raw_http_request(addr, "POST", &format!("/spaces/{STUDIO}/documents/{document_id}-foreign/execution-target/{asset}"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert!(foreign_scope.status == 400 || foreign_scope.status == 404, "{asset} accepted a foreign scope with status {}", foreign_scope.status);
        let foreign_surface = raw_http_request(addr, "POST", &route, &headers, intent_body("surface.foreign").as_bytes()).await;
        assert_eq!(foreign_surface.status, 503, "{asset} accepted a foreign surface");
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&foreign_surface.body).expect("foreign error")["code"], "component-unavailable");
        let smuggled = raw_http_request(addr, "POST", &format!("{route}?package=semio:fixture"), &headers, intent_body("surface.test.editor").as_bytes()).await;
        assert_eq!(smuggled.status, 400, "{asset} accepted a query selector");
        let unknown_field =
            raw_http_request(addr, "POST", &route, &headers, br#"{"schema":"semio.hub.document-open-intent/v1","version":1,"scope":{"spaceId":"studio","documentId":"execution-target-assets"},"clientInstanceId":"c","componentSha256":"aa"}"#).await;
        assert_eq!(unknown_field.status, 400, "{asset} accepted a client-supplied digest selector");
        let oversized = raw_http_request(addr, "POST", &route, &headers, &vec![b'x'; 9 * 1024]).await;
        assert!(oversized.status == 400 || oversized.status == 413, "{asset} accepted an oversized body");
    }

    let mut rotated = state.clone();
    rotated.openable_catalog = Some(document_open_catalog_for_descriptor_with_generation(&descriptor, "77".repeat(32)));
    let rotated_addr = spawn_server(rotated.clone()).await;
    let rotated_manifest = raw_http_request(rotated_addr, "POST", &format!("{root}/manifest"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(rotated_manifest.status, 200);
    let rotated_fields: DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&String::from_utf8(rotated_manifest.body).expect("rotated UTF-8"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("rotated manifest");
    assert_ne!(rotated_fields.catalog.generation_id, fields.catalog.generation_id);
    assert!(!same_lease_fields_v1(&rotated_fields, &fields));

    let mut unavailable = state.clone();
    unavailable.openable_catalog = None;
    let unavailable_addr = spawn_server(unavailable).await;
    let denied = raw_http_request(unavailable_addr, "POST", &format!("{root}/component"), &headers, intent_body("surface.test.editor").as_bytes()).await;
    assert_eq!(denied.status, 503);
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&denied.body).expect("catalog error")["code"], "catalog-unavailable");
}

#[tokio::test]
async fn execution_target_selection_final_fence_matches_neutral_races() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧫️fixtures/🪪️execution-target-relay-v1/🔣️.json")).expect("execution-target relay fixture");
    for vector in fixture["fences"].as_array().expect("neutral fences") {
        let mut state = test_state().await;
        let token = seed_author_token(&state).await;
        let document_id = artifact_document_id_for_test("execution-target-fence");
        let (descriptor, _) = publish_openable_document_for_test(&state, &token, STUDIO, &document_id).await;
        let scope = DocumentScope::new(STUDIO, document_id);
        state.openable_catalog = Some(document_open_catalog_for_descriptor(&descriptor));
        state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
        let mut headers = bearer_headers(&token);
        headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let gate = Arc::new(TestDocumentOpenPlanIssueGate::default());
        state.document_open_plan_issue_gate = Some(gate.clone());
        let body = Bytes::from(semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some("surface.test.editor".into()),
            client_instance_id: "client:fence".into(),
        }));
        let task = tokio::spawn(document_execution_target_selection(scope.space_id.clone(), scope.document_id.clone(), headers, state.clone(), body));
        tokio::time::timeout(std::time::Duration::from_secs(2), gate.admitted.acquire()).await.expect("target final fence deadline").expect("target reached final fence").forget();
        match vector["mutation"].as_str().expect("mutation") {
            "unchanged" => {}
            "directory-advanced" => announce_document_for_test(&state, STUDIO, "execution-target-racing-document").await,
            "session-revoked" => {
                let capability = SessionCapability::parse(&token).expect("session capability");
                let session = state.directory.authenticate_session(&capability).await.expect("session lookup").expect("session");
                state.directory.revoke_auth_session(&session.id, "target-fence", None, "target-fence").await.expect("revoke session").expect("revoked row");
            }
            "cancelled" => task.abort(),
            _ => panic!("unknown target fence mutation"),
        }
        gate.release.add_permits(1);
        let outcome = match task.await {
            Ok(Ok((fields, assets, _))) => {
                assert_eq!(fields.scope, scope);
                let mut stream = assets.component.stream().expect("selected component asset streams");
                let mut read = Vec::new();
                while let Some(chunk) = stream.next_chunk().await {
                    read.extend_from_slice(&chunk.expect("selected component chunk verifies"));
                }
                assert_eq!(read.as_slice(), TEST_EXECUTION_TARGET_COMPONENT_BYTES);
                "selected"
            }
            Ok(Err((_, error))) if error.0.code == DocumentOpenPlanErrorCodeV1::Stale => "stale",
            Ok(Err((_, error))) if error.0.code == DocumentOpenPlanErrorCodeV1::Denied => "denied",
            Err(error) if error.is_cancelled() => "cancelled",
            _ => panic!("unexpected execution-target fence outcome"),
        };
        assert_eq!(outcome, vector["expected"].as_str().expect("expected fence outcome"));
        assert!(state.document_open_plans.inner.lock().expect("plan ledger").records.is_empty(), "asset reads never mint or consume plan receipts");
    }
}

#[tokio::test]
async fn document_open_and_execution_target_refuse_descriptor_or_index_without_genesis() {
    for indexed in [false, true] {
        let mut state = test_state().await;
        let token = seed_author_token(&state).await;
        let document_id = artifact_document_id_for_test(if indexed { "indexed-without-genesis" } else { "descriptor-without-genesis" });
        let scope = DocumentScope::new(STUDIO, &document_id);
        announce_document_for_test(&state, STUDIO, &document_id).await;
        let descriptor = state.directory.get_document_descriptor(&scope).await.expect("descriptor lookup").expect("descriptor-only document");
        if indexed {
            let session = state.directory.authenticate_session(&SessionCapability::parse(&token).expect("indexed author capability")).await.expect("indexed author session read").expect("indexed author session");
            let event = semio_hub_directory_contract::NewDirectoryEvent {
                hlc: semio_hub_directory_contract::HubClock::new().tick(),
                actor: DirectoryActor { kind: DirectoryActorKind::User, id: format!("user:{}#indexed-without-genesis", session.user_id) },
                space_id: Some(scope.space_id.clone()),
                user_id: Some(session.user_id),
                body: os_directory::DirectoryEventBody::DocumentIndexed {
                    scope: scope.clone(),
                    descriptor_digest_v1: os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&descriptor).expect("descriptor digest"),
                    entry: os_directory::DocumentIndexEntryV1 { name: "Indexed without genesis".into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() } },
                },
            };
            state.directory.append_decided_events(&[event]).await.expect("corrupt indexed-only fixture");
        }
        install_document_open_catalog_for_test(&mut state, &descriptor);
        let mut headers = bearer_headers(&token);
        headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
        let body = Bytes::from(semio_framework_pack_json::to_json_string(&DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: Some("surface.test.editor".into()),
            client_instance_id: "client:no-genesis".into(),
        }));
        let authorization = format!("Bearer {token}");
        let raw_headers = [("Authorization", authorization.as_str()), ("Content-Type", "application/json")];
        let addr = spawn_server(state.clone()).await;
        for path in [format!("/spaces/{}/documents/{}/open-plan", scope.space_id, scope.document_id), format!("/spaces/{}/documents/{}/execution-target/manifest", scope.space_id, scope.document_id)] {
            let denied = raw_http_request(addr, "POST", &path, &raw_headers, &body).await;
            assert_eq!(denied.status, 404, "{path} must refuse a document without committed genesis");
            let error: serde_json::Value = serde_json::from_slice(&denied.body).expect("closed checkpoint error");
            assert_eq!(error["code"], "not-found");
            assert!(!denied.body.windows(TEST_EXECUTION_TARGET_COMPONENT_BYTES.len()).any(|window| window == TEST_EXECUTION_TARGET_COMPONENT_BYTES));
        }
        assert!(state.document_open_plans.inner.lock().expect("plan ledger").records.is_empty(), "pre-genesis route refusal cannot mint a plan");
        if !indexed {
            let mut no_catalog = state.clone();
            no_catalog.openable_catalog = None;
            let no_catalog_addr = spawn_server(no_catalog).await;
            let denied = raw_http_request(no_catalog_addr, "POST", &format!("/spaces/{}/documents/{}/execution-target/manifest", scope.space_id, scope.document_id), &raw_headers, &body).await;
            assert_eq!(denied.status, 404, "checkpoint absence must be decided before catalog availability");
            assert_eq!(serde_json::from_slice::<serde_json::Value>(&denied.body).expect("closed no-catalog checkpoint error")["code"], "not-found");
        }
        let Err((plan_status, DirectoryJson(plan_error))) = issue_document_open_plan_inner(scope.space_id.clone(), scope.document_id.clone(), headers.clone(), state.clone(), body.clone()).await else {
            panic!("descriptor/index without genesis minted an open plan");
        };
        assert_eq!((plan_status, plan_error.code), (StatusCode::NOT_FOUND, DocumentOpenPlanErrorCodeV1::NotFound));
        let Err((target_status, DirectoryJson(target_error))) = document_execution_target_selection(scope.space_id.clone(), scope.document_id.clone(), headers, state, body).await else {
            panic!("descriptor/index without genesis minted an execution target");
        };
        assert_eq!((target_status, target_error.code), (StatusCode::NOT_FOUND, DocumentOpenPlanErrorCodeV1::NotFound));
    }
}
