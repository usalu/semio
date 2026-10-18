/// 🛰️ Execution target, admission and stream laws: the lease comparison, the URL components, the native document admission, the hostile plan and cancellation laws, the scoped streams, the space reads, the session authority, the document HTTP transport and the stream resume.

/// 🪪️ The one shared full-field lease relation, driven by the language-neutral
/// `document-execution-target-lease-v1` corpus: the positive GIS Map viewer vector's plan
/// projection equals its manifest, its exact component and descriptor bytes hash to the declared
/// SHA-256/BLAKE3 digests, and every single-field substitution in the corpus is denied by the
/// same relation the retained `DocumentSocketAuthorityV1` uses.
#[test]
fn execution_target_lease_compares_every_plan_and_verified_byte_field() {
    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution target lease corpus");
    let decode_fields =
        |value: &serde_json::Value| -> Result<DocumentExecutionTargetLeaseFieldsV1, ()> { semio_framework_pack_json::from_json_str::<DocumentExecutionTargetLeaseFieldsV1>(&serde_json::to_string(value).expect("fields json"), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| ()) };
    let manifest = decode_fields(&corpus["manifest"]).expect("corpus manifest");
    manifest.validate().expect("corpus manifest is a valid lease projection");
    let hex_bytes = |text: &str| -> Vec<u8> { (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("hex")).collect() };
    let component = hex_bytes(corpus["componentHex"].as_str().expect("component hex"));
    let descriptor_bytes = hex_bytes(corpus["descriptorHex"].as_str().expect("descriptor hex"));
    assert_eq!(semio_framework_hash::sha256_hex(&component), manifest.component.sha256);
    assert_eq!(semio_framework_hash::hash_bytes(&component), manifest.component.blake3);
    assert_eq!(semio_framework_hash::sha256_hex(&descriptor_bytes), manifest.descriptor.sha256);
    assert_eq!(component.len() as u64, manifest.component.byte_length);
    assert_eq!(descriptor_bytes.len() as u64, manifest.descriptor.byte_length);

    let mut plan_json = corpus["plan"].clone();
    plan_json["expiresAtUnixMs"] = serde_json::json!(u64::try_from(wall_now_ms()).expect("wall clock") + 20_000);
    let plan: DocumentOpenPlanV1 = semio_framework_pack_json::from_json_str(&serde_json::to_string(&plan_json).expect("plan json"), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("corpus plan");
    plan.validate(u64::try_from(wall_now_ms()).expect("wall clock")).expect("corpus plan validates");
    let projected = lease_fields_from_plan_v1(&plan, manifest.component.byte_length, manifest.descriptor.byte_length, manifest.browser_actor.byte_length()).expect("closed actor projection");
    assert!(same_lease_fields_v1(&projected, &manifest));

    let expectation = DocumentSocketExpectationV1 {
        artifact_schema: plan.artifact.schema.clone(),
        pack_schema_hash: decode_lower_hex_32(&plan.artifact.pack_schema_hash).expect("pack schema hash"),
        requested_surface_id: Some(plan.surface.surface_id.clone()),
        lease: Some(manifest.clone()),
    };
    let authority = DocumentSocketAuthorityV1::from_plan(corpus["hubOrigin"].as_str().expect("hub origin").to_string(), &plan, &expectation, u64::try_from(wall_now_ms()).expect("wall clock")).expect("validated retained authority");
    assert!(authority.matches_lease_fields(&manifest));
    let mut unbound = authority.clone();
    unbound.admitted_lease = None;
    assert!(!unbound.matches_lease_fields(&manifest));

    let mut substitutions = 0usize;
    for vector in corpus["hostile"].as_array().expect("hostile rows") {
        if vector["kind"] != "manifest-field" {
            continue;
        }
        assert_eq!(vector["expected"], "unpublished");
        let mut candidate = corpus["manifest"].clone();
        let mut cursor = &mut candidate;
        let path: Vec<&str> = vector["path"].as_str().expect("hostile path").split('.').collect();
        for segment in &path[..path.len() - 1] {
            cursor = cursor.get_mut(*segment).expect("hostile path segment");
        }
        cursor[path[path.len() - 1]] = vector["value"].clone();
        let denied = match decode_fields(&candidate) {
            Err(()) => true,
            Ok(mutated) => mutated.validate().is_err() || (!same_lease_fields_v1(&projected, &mutated) && !authority.matches_lease_fields(&mutated)),
        };
        assert!(denied, "single-field substitution {} was admitted", vector["name"]);
        substitutions += 1;
    }
    assert!(substitutions >= 30, "corpus lost single-field substitutions: {substitutions}");
}

#[test]
fn url_components_percent_encode_utf8_punctuation_without_aliases() {
    assert_eq!(encode_url_component("space /東京?#"), "space%20%2F%E6%9D%B1%E4%BA%AC%3F%23");
    assert_eq!(encode_url_component("a-z_A.9~"), "a-z_A.9~");
}

#[semio_framework_async_macros::async_test]
async fn native_document_admission_issues_validates_and_exchanges_exactly_once() {
    let transport = FakeTransport::default();
    let space_id = "space /東京?";
    let document_id = "document#ä";
    let surface_id = "surface /editor?#";
    let receipt = push_document_plan(&transport, space_id, document_id, "demo/v1", surface_id).await;
    push_document_grant(&transport).await;
    let client = authenticated_client(transport.clone(), "protected-session");
    let mut expectation = document_expectation("demo/v1", Some(surface_id));
    expectation.lease = Some(document_lease_fields(space_id, document_id, "demo/v1", surface_id));

    let admission = client.admit_document_socket(&root_ctx(), space_id, document_id, &expectation, "native-instance", 30_000).expect("document admission");
    assert_eq!(admission.authority.scope, DocumentScope::new(space_id, document_id));
    assert_eq!(admission.authority.artifact.schema, "demo/v1");
    assert_eq!(admission.authority.surface.surface_id, surface_id);
    assert_eq!(admission.authority.pack_schema_hash, [0x11; 32]);
    assert_eq!(admission.authority.package.plugin_id, "gis");
    assert_eq!(admission.authority.package.package_id, "semio:gis");
    assert_eq!(admission.authority.package.version, "1.0.0:β");
    assert_eq!(admission.authority.parent_dialect, DocumentOpenParentDialectV1 { artifact_kind: "s.gis:gismap".into(), standard: "1".into(), subset: "*".into() });
    assert_eq!(admission.authority.surface.app_id, "app.gis");
    assert_eq!(admission.authority.surface.window_kind_id, "window.document");
    assert_eq!(admission.authority.surface.renderer_target, DocumentOpenRendererTargetV1::React);
    assert_eq!((admission.socket.schema.as_str(), admission.socket.protocol.as_str(), admission.socket.actor_id.len()), ("semio.hub.document-socket-grant/v1", "semio.session.v1", 71));

    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].url, "http://hub.local/spaces/space%20%2F%E6%9D%B1%E4%BA%AC%3F/documents/document%23%C3%A4/open-plan");
    assert_eq!(requests[1].url, "http://hub.local/spaces/space%20%2F%E6%9D%B1%E4%BA%AC%3F/documents/document%23%C3%A4/socket-grants");
    let intent: serde_json::Value = serde_json::from_slice(&requests[0].body).expect("independent intent decode");
    assert_eq!(
        intent,
        serde_json::json!({
            "schema": "semio.hub.document-open-intent/v1",
            "version": 1,
            "scope": { "spaceId": space_id, "documentId": document_id },
            "requestedSurfaceId": surface_id,
            "clientInstanceId": "native-instance"
        })
    );
    let exchange: serde_json::Value = serde_json::from_slice(&requests[1].body).expect("independent exchange decode");
    assert_eq!(
        exchange,
        serde_json::json!({
            "schema": "semio.hub.document-plan-socket-grant-intent/v1",
            "version": 1,
            "planReceipt": receipt
        })
    );
    assert!(!requests[0].body.windows(receipt.len()).any(|window| window == receipt.as_bytes()));
}

#[semio_framework_async_macros::async_test]
async fn hostile_or_cancelled_plan_never_reaches_receipt_exchange() {
    let transport = FakeTransport::default();
    let receipt = push_document_plan(&transport, "space", "document", "foreign/v1", "surface").await;
    let client = authenticated_client(transport.clone(), "protected-session");
    let expectation = document_expectation("expected/v1", Some("surface"));
    let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000) {
        Err(error) => error,
        Ok(_) => panic!("schema substitution accepted"),
    };
    assert!(!error.to_string().contains(&receipt));
    assert_eq!(transport.requests.lock().unwrap().len(), 1, "invalid plan is never exchanged");

    let cancelled_transport = FakeTransport::default();
    push_document_plan(&cancelled_transport, "space", "document", "expected/v1", "surface").await;
    cancelled_transport.cancel_after_grant.store(true, Ordering::SeqCst);
    let cancelled = authenticated_client(cancelled_transport.clone(), "protected-session");
    assert!(matches!(cancelled.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000), Err(DirectoryClientError::Cancelled)));
    assert_eq!(cancelled_transport.requests.lock().unwrap().len(), 1, "cancel after plan prevents exchange");
}

#[semio_framework_async_macros::async_test]
async fn cancellation_after_receipt_exchange_never_reaches_a_document_socket() {
    let transport = FakeTransport::default();
    push_document_plan(&transport, "space", "document", "expected/v1", "surface").await;
    push_document_grant(&transport).await;
    transport.cancel_after_grant_number.store(2, Ordering::SeqCst);
    let client = authenticated_client(transport.clone(), "protected-session");
    let expectation = document_expectation("expected/v1", Some("surface"));

    assert!(matches!(client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000), Err(DirectoryClientError::Cancelled)));
    assert_eq!(transport.requests.lock().unwrap().len(), 2, "plan and receipt exchange completed once");
    assert!(transport.ws_urls.lock().unwrap().is_empty(), "cancelled exchanged grant never reaches a socket URL or protocol header");
}

#[semio_framework_async_macros::async_test]
async fn protected_document_admission_is_bounded_and_redacts_hostile_responses() {
    let oversized = FakeTransport::default();
    oversized.push_response(Ok(HttpResponse { status: 200, body: vec![b'x'; DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES + 1] })).await;
    let client = authenticated_client(oversized.clone(), "protected-session");
    let expectation = document_expectation("expected/v1", None);
    let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", u64::MAX) {
        Err(error) => error,
        Ok(_) => panic!("oversized plan accepted"),
    };
    assert_eq!(error.to_string(), "decode: protected document admission response exceeded 64 KiB");
    assert_eq!(oversized.requests.lock().unwrap().len(), 1);

    let hostile = FakeTransport::default();
    let secret = format!("plan.v1.{}.{}", "a".repeat(32), "b".repeat(43));
    hostile.push_response(Ok(HttpResponse { status: 500, body: secret.as_bytes().to_vec() })).await;
    let client = authenticated_client(hostile.clone(), "protected-session");
    let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000) {
        Err(error) => error,
        Ok(_) => panic!("hostile response accepted"),
    };
    assert!(!error.to_string().contains(&secret));
    assert_eq!(error.to_string(), "http 500: protected document admission rejected");
}

#[semio_framework_async_macros::async_test]
async fn mismatched_local_plugin_selection_never_exchanges_a_plan_receipt() {
    let transport = FakeTransport::default();
    let receipt = push_document_plan(&transport, "space", "document", "expected/v1", "surface").await;
    let client = authenticated_client(transport.clone(), "protected-session");
    let mut expectation = document_expectation("expected/v1", Some("surface"));
    let mut lease = document_lease_fields("space", "document", "expected/v1", "surface");
    lease.package.package_id = "foreign.package".into();
    expectation.lease = Some(lease);
    let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000) {
        Err(error) => error,
        Ok(_) => panic!("foreign plugin selection accepted"),
    };
    assert!(!error.to_string().contains(&receipt));
    assert_eq!(transport.requests.lock().unwrap().len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn ws_url_switches_scheme_and_encodes_query() {
    assert_eq!(directory_ws_url("http://127.0.0.1:8787", 0), "ws://127.0.0.1:8787/directory/socket/v1?since=0");
    assert_eq!(directory_ws_url("https://hub.example", 42), "wss://hub.example/directory/socket/v1?since=42");
    assert_eq!(directory_scoped_ws_url("https://hub.example", &DocumentScope::new("space /a", "document#b"), 7), "wss://hub.example/directory/spaces/space%20%2Fa/documents/document%23b/socket/v1?since=7");
}

#[semio_framework_async_macros::async_test]
async fn scoped_stream_close_4401_is_terminal_and_never_redials() {
    let client = Arc::new(authenticated_client(FakeTransport::default(), "tok"));
    let scope = DocumentScope::new("space-a", "document-a");
    let mut stream = client.stream_scoped(scope.clone(), 9);
    let DirectoryStreamTurn::DialScoped { scope: dial_scope, since, .. } = stream.turn(&root_ctx(), 0) else { panic!("scoped stream must dial exact scope") };
    assert_eq!(dial_scope, scope);
    assert_eq!(since, 9);
    assert!(matches!(stream.complete_dial(0, Ok(FakeWs::with_close_code(4401))), DirectoryStreamTurn::Idle));
    assert!(matches!(stream.turn(&root_ctx(), 1), DirectoryStreamTurn::Revoked(revoked) if revoked == scope));
    assert!(matches!(stream.turn(&root_ctx(), u64::MAX), DirectoryStreamTurn::Closed));
}

#[semio_framework_async_macros::async_test]
async fn scoped_stream_issues_and_dials_the_same_encoded_scope() {
    let transport = FakeTransport::default();
    push_grant(&transport).await;
    transport.push_ws(Ok(std::collections::VecDeque::new())).await;
    let client = authenticated_client(transport.clone(), "tok");
    let scope = DocumentScope::new("space /a", "document#b");
    let _connection = client.open_scoped_stream_ws(&root_ctx(), &scope, 7, 100).expect("scoped socket opens");
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url, "http://hub.local/directory/spaces/space%20%2Fa/documents/document%23b/socket-grants");
    assert!(requests[0].body.is_empty());
    assert_eq!(transport.ws_urls.lock().unwrap().as_slice(), ["ws://hub.local/directory/spaces/space%20%2Fa/documents/document%23b/socket/v1?since=7"]);
}

/// 🚪️ The hub admits `POST /directory/socket-grants` only with an empty body (`DefaultBodyLimit::max(0)`, hostile-input row
/// `"body": {"kind": "empty"}`); a `{}` body is refused `413` before the grant is minted, which took down every native directory
/// stream dial (measured on the 2026-09-27 B3 hub).
#[semio_framework_async_macros::async_test]
async fn the_space_wide_directory_grant_carries_the_empty_body_the_hub_route_declares() {
    let transport = FakeTransport::default();
    push_grant(&transport).await;
    transport.push_ws(Ok(std::collections::VecDeque::new())).await;
    let client = authenticated_client(transport.clone(), "tok");
    let _connection = client.open_stream_ws(&root_ctx(), 0, 100).expect("space-wide socket opens");
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url, "http://hub.local/directory/socket-grants");
    assert!(requests[0].body.is_empty(), "the grant body must be empty, got {:?}", String::from_utf8_lossy(&requests[0].body));
}

#[semio_framework_async_macros::async_test]
async fn backoff_doubles_and_caps() {
    assert_eq!(next_backoff_ms(500), 1000);
    assert_eq!(next_backoff_ms(20_000), 30_000);
    assert_eq!(next_backoff_ms(30_000), 30_000);
    assert_eq!(next_backoff_ms(0), HUB_RECONNECT_MIN_MS);
}

#[semio_framework_async_macros::async_test]
async fn spaces_decodes_and_sends_bearer() {
    let transport = FakeTransport::default();
    transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
    let client = authenticated_client(transport.clone(), "tok");

    let spaces = client.spaces(&root_ctx()).await.expect("decodes");
    assert!(spaces.is_empty());
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests[0].url, "http://hub.local/directory/spaces");
    assert_eq!(requests[0].bearer.as_deref(), Some("tok"));
}

#[semio_framework_async_macros::async_test]
async fn space_exposes_the_durable_document_descriptor() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/📇️directory/🪪️document-descriptor.json")).expect("descriptor fixture");
    let descriptor = fixture.get("valid").expect("valid descriptor").clone();
    let document: DocumentView = semio_framework_pack_json::from_json_str(&serde_json::json!({ "descriptor": descriptor, "headSeq": 7, "commitSeq": 6, "epoch": 2 }).to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("document view fixture");
    let space = MemberSpaceViewV1 {
        id: "space-a".into(),
        name: "Fixture".into(),
        kind: crate::DirectorySpaceKind::Studio,
        visibility: crate::DirectorySpaceVisibility::Private,
        owner_user_id: "user-owner".into(),
        role: DirectorySpaceRole::Author,
        member_count: 1,
        document_count: 1,
        active_connections: 0,
        created_at_ms: 1,
        updated_at_ms: 2,
    };
    let mut page = DirectorySpaceAdministrationPageV1::Author {
        schema: crate::DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
        session_binding_sha256: "a".repeat(64),
        authorization_generation: 7,
        space_id: "space-a".into(),
        space,
        members: DirectorySpaceAdministrationMemberWindowV1 { rows: Vec::new(), next_cursor: None },
        documents: DirectorySpaceAdministrationDocumentWindowV1 { rows: vec![document], next_cursor: None },
        invites: DirectorySpaceAdministrationInviteWindowV1 { rows: Vec::new(), next_cursor: None },
        capabilities: DirectorySpaceAdministrationCapabilitiesV1 { rename_space: true, set_visibility: true, delete_space: true, upsert_member: true, remove_member: true, create_invite: true, revoke_invite: true },
        receipt_sha256: String::new(),
    };
    let receipt = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    if let DirectorySpaceAdministrationPageV1::Author { receipt_sha256, .. } = &mut page {
        *receipt_sha256 = receipt;
    }
    let canonical = semio_framework_pack_json::to_json_string(&page);
    assert!(!canonical.contains("selector") && !canonical.contains("secretDigest") && !canonical.contains("passwordHash"));
    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 200, body: canonical.clone().into_bytes() })).await;
    let client = authenticated_client(transport.clone(), "member-token");

    let fetched = client.space_administration_page(&root_ctx(), "space-a", None).await.expect("administration page decodes");
    assert_eq!(fetched.canonical_json(), canonical);
    let DirectorySpaceAdministrationPageV1::Author { documents, .. } = fetched.page().clone() else { panic!("author projection") };
    assert_eq!(documents.rows[0].descriptor.document_id, "shared-document");
    assert_eq!(documents.rows[0].descriptor.owner.plugin_id, "fixture.neutral-host-fixture");
    assert_eq!(documents.rows[0].descriptor.bootstrap_frontier.head_seq, 7);
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests[0].bearer.as_deref(), Some("member-token"));
}

#[semio_framework_async_macros::async_test]
async fn unauthorized_status_maps_to_unauthorized_error() {
    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 401, body: Vec::new() })).await;
    let client = DirectoryClient::new(transport, "http://hub.local");

    let error = client.me(&root_ctx()).await.expect_err("401 is unauthorized");
    assert!(matches!(error, DirectoryClientError::Unauthorized));
}

#[semio_framework_async_macros::async_test]
async fn session_authority_client_preserves_canonical_binding_and_rejects_reordered_body() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../🧪️tests/🧫️fixtures/🪪️session-authority-v1/🔣️.json")).expect("session authority fixture");
    let canonical = fixture["raw"][0]["source"].as_str().expect("canonical");
    let reordered = fixture["raw"][1]["source"].as_str().expect("reordered");
    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
    transport.push_response(Ok(HttpResponse { status: 200, body: reordered.as_bytes().to_vec() })).await;
    let client = authenticated_client(transport, "session-authority-token");
    let authority = client.me(&root_ctx()).await.expect("canonical authority");
    assert_eq!(authority.session_binding_sha256, fixture["bindingGoldens"][0]["sessionBindingSha256"]);
    assert_eq!(authority.authorization_generation, 7);
    assert!(matches!(client.me(&root_ctx()).await, Err(DirectoryClientError::Decode(_))));
}

#[semio_framework_async_macros::async_test]
async fn document_http_transport_preserves_scope_bounds_and_owner_decode() {
    let transport = FakeTransport::default();
    transport.push_response(Ok(HttpResponse { status: 200, body: br#"{"schema":"neutral.reply/v1","value":7}"#.to_vec() })).await;
    transport.push_response(Ok(HttpResponse { status: 200, body: vec![b'x'; 65] })).await;
    transport.push_response(Ok(HttpResponse { status: 401, body: b"private detail".to_vec() })).await;
    let client = authenticated_client(transport.clone(), "port-token");
    let scope = DocumentScope::new("space/one", "document?two");
    let request = DocumentHttpRequestV1 { method: HttpMethod::Post, segments: vec!["neutral".into(), "job/one".into()], after: Some(7), body: Some(b"{}".to_vec()), request_max_bytes: 64, response_max_bytes: 64 };
    let value = client.document_http_call(&root_ctx(), &scope, &request, |bytes| {
        let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
        (value["schema"] == "neutral.reply/v1").then_some(value["value"].as_u64().unwrap()).ok_or(DocumentHttpPortCodeV1::Invalid)
    }).await.expect("owner decoder");
    assert_eq!(value, 7);
    assert!(transport.requests.lock().unwrap()[0].url.ends_with("/spaces/space%2Fone/documents/document%3Ftwo/neutral/job%2Fone?after=7"));
    assert_eq!(transport.requests.lock().unwrap()[0].bearer.as_deref(), Some("port-token"));
    assert_eq!(client.document_http_call(&root_ctx(), &scope, &request, |_| Ok(())).await, Err(DocumentHttpPortCodeV1::Bounds));
    assert_eq!(client.document_http_call(&root_ctx(), &scope, &request, |_| Ok(())).await, Err(DocumentHttpPortCodeV1::Denied));
    let ctx = root_ctx();
    ctx.cancel.cancel_now();
    assert_eq!(client.document_http_call(&ctx, &scope, &request, |_| Ok(())).await, Err(DocumentHttpPortCodeV1::Cancelled));
    assert_eq!(transport.requests.lock().unwrap().len(), 3);
}

#[semio_framework_async_macros::async_test]
async fn stream_reconnects_and_resumes_from_last_seq() {
    let transport = FakeTransport::default();
    transport.push_ws(Err(TransportError::Io("refused".to_string()))).await;
    let event =
        serde_json::json!({ "kind": "event", "event": { "seq": 7, "id": "e1", "hlc": { "physicalMs": 1, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": "sp-1" }, "recordedAtMs": 1 } })
            .to_string();
    transport.push_ws(Ok(std::collections::VecDeque::from([Ok(Some(event)), Ok(None)]))).await;
    transport.push_ws(Err(TransportError::Io("refused again".to_string()))).await;
    push_grant(&transport).await;
    push_grant(&transport).await;
    push_grant(&transport).await;

    let client = authenticated_client(transport.clone(), "tok");
    let mut stream = client.stream(0);
    let ctx = root_ctx();

    let DirectoryStreamTurn::Dial { client: dial, since } = stream.turn(&ctx, 0) else { panic!("first turn must dial") };
    assert!(matches!(stream.complete_dial(0, dial.open_stream_ws(&ctx, since, 100).map_err(|error| TransportError::Io(error.to_string()))), DirectoryStreamTurn::ReconnectAt(HUB_RECONNECT_MIN_MS)));
    assert!(matches!(stream.turn(&ctx, HUB_RECONNECT_MIN_MS), DirectoryStreamTurn::Dial { .. }));
    let result = client.open_stream_ws(&ctx, 0, 100).map_err(|error| TransportError::Io(error.to_string()));
    assert!(matches!(stream.complete_dial(HUB_RECONNECT_MIN_MS, result), DirectoryStreamTurn::Idle));
    match stream.turn(&ctx, HUB_RECONNECT_MIN_MS) {
        DirectoryStreamTurn::Message(DirectoryStreamMessage::Event { event }) => assert_eq!(event.seq, 7),
        _ => panic!("second connection must deliver the event"),
    }
    assert_eq!(stream.since(), 7);
    assert!(matches!(stream.turn(&ctx, HUB_RECONNECT_MIN_MS), DirectoryStreamTurn::ReconnectAt(1_000)));
    let DirectoryStreamTurn::Dial { client: dial, since } = stream.turn(&ctx, 1_000) else { panic!("reconnect deadline must dial") };
    assert!(matches!(stream.complete_dial(1_000, dial.open_stream_ws(&ctx, since, 100).map_err(|error| TransportError::Io(error.to_string()))), DirectoryStreamTurn::ReconnectAt(2_000)));

    let ws_urls = transport.ws_urls.lock().unwrap();
    assert_eq!(ws_urls.len(), 3);
    assert!(ws_urls[0].ends_with("since=0"));
    assert!(ws_urls[2].ends_with("since=7"), "the resumed dial carries the last-seen seq, not the original since");
}
