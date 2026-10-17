//! 🪪️ Sessions and document-open plans: issued sessions, admin tokens, plan authorities, catalogs and directory commands.

use super::*;

pub struct TestIssuedSession {
    pub token: String,
    pub user_id: String,
}

pub async fn issue_test_session(state: &HubState, email: &str) -> TestIssuedSession {
    let user = match state.directory.get_user_by_email(email).await.expect("test user lookup") {
        Some(user) => user,
        None => state.directory.create_user(email, email, None, Some(email), Some("test-verifier")).await.expect("test verified user"),
    };
    let issue = AuthSessionIssue {
        user_id: user.id.clone(),
        identity_provider: "test-verifier".into(),
        identity_subject_digest: identity_subject_digest("test-verifier", email).expect("test subject digest"),
        ttl_secs: 3_600,
        device_instance_id: "test-device".into(),
        session_kind: AuthSessionKind::DevelopmentLocal,
        correlation_id: directory::os_identity::time_ordered_id(),
        peer_class: "test".into(),
    };
    let issued = state.directory.issue_auth_session(&issue).await.expect("test session issue");
    TestIssuedSession { token: issued.capability.expose_once(), user_id: user.id }
}

pub async fn authorize_test_admin(state: &mut HubState, email: &str) -> HeaderMap {
    let session = issue_test_session(state, email).await;
    state.admin_subjects = Arc::from([AdminSubject { provider_digest: admin_provider_digest("test-verifier"), subject_digest: identity_subject_digest("test-verifier", email).expect("test admin subject digest") }]);
    let mut headers = HeaderMap::new();
    headers.insert(axum::http::header::AUTHORIZATION, format!("Bearer {}", session.token).parse().expect("test bearer header"));
    headers
}

pub async fn seed_author_token(state: &HubState) -> String {
    let issue = AuthSessionIssue {
        user_id: "seed".into(),
        identity_provider: "test-verifier".into(),
        identity_subject_digest: identity_subject_digest("test-verifier", "seed").expect("seed subject digest"),
        ttl_secs: 3_600,
        device_instance_id: "seed-device".into(),
        session_kind: AuthSessionKind::DevelopmentLocal,
        correlation_id: directory::os_identity::time_ordered_id(),
        peer_class: "test".into(),
    };
    state.directory.issue_auth_session(&issue).await.expect("seed author session").capability.expose_once()
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
pub struct DocumentOpenPlanLedgerFixture {
    pub now_ms: u64,
    pub descriptor: DocumentDescriptor,
    pub descriptor_digest_v1: String,
    pub valid_plan: DocumentOpenPlanV1,
}

pub fn document_open_plan_test_authority(fixture: &DocumentOpenPlanLedgerFixture) -> DocumentOpenPlanAuthorityV1 {
    DocumentOpenPlanAuthorityV1 {
        scope: fixture.valid_plan.scope.clone(),
        descriptor: fixture.descriptor.clone(),
        descriptor_digest_v1: fixture.descriptor_digest_v1.clone(),
        catalog: fixture.valid_plan.catalog.clone(),
        package: fixture.valid_plan.package.clone(),
        artifact: fixture.valid_plan.artifact.clone(),
        parent_dialect: semio_framework::ArtifactDialect {
            artifact_kind: fixture.valid_plan.parent_dialect.artifact_kind.clone(),
            standard: fixture.valid_plan.parent_dialect.standard.clone(),
            subset: fixture.valid_plan.parent_dialect.subset.clone(),
        },
        surface: fixture.valid_plan.surface.clone(),
        browser_actor: fixture.valid_plan.browser_actor.clone(),
        grant: fixture.valid_plan.grant,
        checkpoint: fixture.valid_plan.checkpoint.clone(),
        revalidation: fixture.valid_plan.revalidation,
        subject: SocketSubjectV1::Session {
            session_id: "open-plan-session".into(),
            user_id: "open-plan-user".into(),
            authorization_generation: fixture.valid_plan.revalidation.session_generation.expect("session generation"),
            role: Some(SpaceRole::Author),
            expires_at_ms: i64::MAX,
            session_kind: AuthSessionKind::External,
            device_instance_id: "open-plan-device".into(),
        },
        server_actor_id: "hub.v1.open-plan-actor".into(),
        client_instance_id_digest: [9; 32],
    }
}

pub const TEST_EXECUTION_TARGET_COMPONENT_BYTES: &[u8] = b"\0asm\x01\0\0\0semio-execution-target-lease-component";

pub const TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES: &[u8] = b"semio-execution-target-lease-descriptor";

pub fn document_open_catalog_for_descriptor(descriptor: &DocumentDescriptor) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
    document_open_catalog_for_descriptor_with_generation(descriptor, "66".repeat(32))
}

pub fn install_document_open_catalog_for_test(state: &mut HubState, descriptor: &DocumentDescriptor) {
    state.openable_catalog = Some(document_open_catalog_for_descriptor(descriptor));
    state.readiness = Arc::new(hub_readiness(HubMode::Development, "loopback", "00112233445566778899aabbccddeeff".into(), true, true, true, true, true, true, false, false, "trusted-catalog-never-published-in-this-data-root"));
}

pub fn document_open_catalog_for_descriptor_with_generation(descriptor: &DocumentDescriptor, generation_id: String) -> Arc<dyn DocumentOpenCatalogAuthorityV1> {
    let package = DocumentOpenPackageV1 {
        plugin_id: descriptor.owner.plugin_id.clone(),
        package_id: descriptor.owner.package_id.clone(),
        version: descriptor.owner.version.clone(),
        component_sha256: descriptor.owner.package_hash.clone(),
        component_blake3: "44".repeat(32),
        descriptor_byte_sha256: "55".repeat(32),
        execution_protocol: os_directory::DocumentExecutionProtocolV1 { app_channel_version: directory::os_spr::CHANNEL_VERSION },
        component_receipt: directory::os_component_receipt::ComponentReceiptV1 {
            schema: directory::os_component_receipt::COMPONENT_RECEIPT_SCHEMA.into(),
            plugin_id: descriptor.owner.plugin_id.clone(),
            package_id: descriptor.owner.package_id.clone(),
            profile: directory::os_component_receipt::ComponentProfileV1::Release,
            target: directory::os_component_receipt::COMPONENT_TARGET.into(),
            wasm_sha256: descriptor.owner.package_hash.clone(),
            wasm_bytes: TEST_EXECUTION_TARGET_COMPONENT_BYTES.len() as u64,
            semantic_descriptor_sha256: "55".repeat(32),
            toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)".into(),
            source_tree_digest: "3".repeat(64),
            provenance_sha256: "4".repeat(64),
        },
    };
    let artifact = DocumentOpenArtifactV1 { kind: descriptor.artifact_kind.clone(), schema: descriptor.artifact_schema.clone(), pack_schema_hash: descriptor.pack_schema_hash.clone() };
    Arc::new(TestDocumentOpenCatalog {
        generation_id,
        component: TEST_EXECUTION_TARGET_COMPONENT_BYTES.into(),
        descriptor: TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES.into(),
        browser_actor: None,
        open_targets: vec![
            VerifiedDocumentOpenSelectionV1 {
                package: package.clone(),
                artifact: artifact.clone(),
                parent_dialect: semio_framework::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() },
                surface: DocumentOpenSurfaceV1 {
                    surface_id: "surface.test.editor".into(),
                    app_id: "app.test".into(),
                    window_kind_id: "window.document".into(),
                    role: os_directory::DocumentOpenSurfaceRoleV1::Editor,
                    renderer_target: os_directory::DocumentOpenRendererTargetV1::React,
                },
                grant: DocumentOpenGrantV1 { read: true, write: true, observe: true },
                browser_actor: os_directory::schema::DocumentOpenBrowserActorV1::None,
            },
            VerifiedDocumentOpenSelectionV1 {
                package,
                artifact,
                parent_dialect: semio_framework::ArtifactDialect { artifact_kind: TEST_ARTIFACT_PARENT_DIALECT_KIND.into(), standard: "1".into(), subset: "*".into() },
                surface: DocumentOpenSurfaceV1 {
                    surface_id: "surface.test.viewer".into(),
                    app_id: "app.test".into(),
                    window_kind_id: "window.document".into(),
                    role: os_directory::DocumentOpenSurfaceRoleV1::Viewer,
                    renderer_target: os_directory::DocumentOpenRendererTargetV1::React,
                },
                grant: DocumentOpenGrantV1 { read: true, write: false, observe: true },
                browser_actor: os_directory::schema::DocumentOpenBrowserActorV1::None,
            },
        ]
        .into_boxed_slice(),
    })
}

pub async fn issue_and_exchange_document_open_plan_for_test(state: &HubState, token: &str, scope: &DocumentScope, client_instance_id: &str) -> (DocumentOpenPlanV1, DocumentSocketGrantReceiptV1) {
    let mut headers = bearer_headers(token);
    headers.insert(axum::http::header::CONTENT_TYPE, "application/json".parse().expect("content type"));
    let intent = DocumentOpenIntentV1 { schema: "semio.hub.document-open-intent/v1".into(), version: 1, scope: scope.clone(), requested_surface_id: Some("surface.test.editor".into()), client_instance_id: client_instance_id.into() };
    let DirectoryJson(plan) = match issue_document_open_plan_inner(scope.space_id.clone(), scope.document_id.clone(), headers.clone(), state.clone(), Bytes::from(semio_framework_pack_json::to_json_string(&intent))).await {
        Ok(plan) => plan,
        Err((status, DirectoryJson(error))) => panic!("issue document open plan: {status} {:?}", error.code),
    };
    let exchange = DocumentPlanSocketGrantIntentV1 { schema: "semio.hub.document-plan-socket-grant-intent/v1".into(), version: 1, plan_receipt: plan.receipt.clone() };
    let Json(grant) = match issue_document_plan_socket_grant_inner(scope.space_id.clone(), scope.document_id.clone(), headers, state.clone(), Bytes::from(semio_framework_pack_json::to_json_string(&exchange))).await {
        Ok(grant) => grant,
        Err((status, DirectoryJson(error))) => panic!("exchange document open plan: {status} {:?}", error.code),
    };
    (plan, grant)
}

pub async fn stop_recovery_server(state: HubState, shutdown: tokio::sync::oneshot::Sender<()>, task: tokio::task::JoinHandle<()>) {
    shutdown.send(()).expect("shutdown signal");
    tokio::time::timeout(std::time::Duration::from_secs(5), task).await.expect("server shutdown deadline").expect("server shutdown join");
    state.admin_operation_tasks.shutdown().await;
    let directory = Arc::downgrade(&state.directory);
    let database = state.db.clone();
    drop(state);
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        while directory.strong_count() != 0 || Arc::strong_count(&database) != 1 {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        }
    })
    .await
    .expect("all socket and directory owners retired before reopen");
    let mut database = Arc::try_unwrap(database).unwrap_or_else(|_| panic!("database owner remained shared"));
    let control = db::DatabaseShutdownControl::for_timeout(std::time::Duration::from_secs(5));
    tokio::time::timeout(std::time::Duration::from_secs(5), database.shutdown(&control)).await.expect("database shutdown deadline").expect("database shutdown");
}

pub fn presence_hex_bytes(hex: &str) -> Vec<u8> {
    assert_eq!(hex.len() % 2, 0);
    (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).expect("presence fixture byte")).collect()
}

pub fn presence_normalization_fixture() -> serde_json::Value {
    serde_json::from_str(semio_hub_testing::hub_source("🧪️tests/🧫️fixtures/🪪️presence-normalization-v1/🔣️.json")).expect("presence normalization fixture")
}

fn directory_command_body(request_id: &str, command: DirectoryCommand) -> Vec<u8> {
    DirectoryCommandRequestV1::new(request_id, command).canonical_json().into_bytes()
}

pub async fn post_directory_command_for_test(addr: SocketAddr, token: &str, request_id: &str, command: DirectoryCommand) -> RawHttpResponse {
    let authorization = format!("Bearer {token}");
    raw_http_request(addr, "POST", "/directory/commands", &[("Authorization", &authorization), ("Content-Type", "application/json")], &directory_command_body(request_id, command)).await
}

//#region 🔖️CredentialSignIn
pub fn json_body(response: &RawHttpResponse) -> serde_json::Value {
    serde_json::from_slice(&response.body).expect("JSON body")
}
//#endregion 🔖️CredentialSignIn
