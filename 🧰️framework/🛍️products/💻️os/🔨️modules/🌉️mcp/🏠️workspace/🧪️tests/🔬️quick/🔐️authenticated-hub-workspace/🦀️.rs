/// 🔐️ Authenticated hub workspace: the empty catalog, the authenticated hub workspace fixture, its snapshot-only resources, retained discovery and the one live link per app bound.

fn empty_catalog() -> Arc<Catalog> {
    Arc::new(compile(&CatalogSource::default(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native).expect("empty catalog source compiles"))
}

fn authenticated_hub_workspace_fixture() -> HeadlessWorkspace {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text("../../🏠️workspace/🔗️remote/🧪️tests/🧫️fixtures/🔣️authenticated-hub-descriptor-index.json")).unwrap();
    let ready = &fixture["cases"]["memberReady"];
    let page = semio_framework_os_kernel::os_directory::DirectorySpaceAdministrationPageV1::parse_canonical_json(ready["responses"][1]["canonicalBody"].as_str().unwrap()).unwrap();
    let semio_framework_os_kernel::os_directory::DirectorySpaceAdministrationPageV1::Author { space, members, documents, .. } = page else { panic!("author administration page fixture") };
    let members: Vec<semio_framework_os_kernel::os_directory::MemberView> =
        members.rows.into_iter().map(|row| semio_framework_os_kernel::os_directory::MemberView { user_id: row.user_id, email: row.email, display_name: row.display_name, role: row.role }).collect();
    let documents = documents.rows;
    let view = documents[0].clone();
    let scope = DocumentScope::new("space-a", "shared-doc");
    let digest = semio_framework_os_kernel::os_directory::io::binary::descriptor_digest::descriptor_digest_v1(&view.descriptor).unwrap();
    let document = remote::AuthorizedDocumentView { scope: scope.clone(), descriptor_digest_v1: semio_framework_os_kernel::os_directory::io::binary::artifact_hash::hex_lower(digest.as_bytes()), view };
    let snapshot = AuthorizedDescriptorSnapshot { authenticated_user_id: "user-a".to_string(), session_expires_at_ms: i64::MAX, space, membership: members[0].clone(), observed_event_seq: 8, documents: HashMap::from([(scope, document)]) };
    let binding = Arc::new(HubRemoteBinding::new("https://hub.invalid", "space-a").unwrap());
    binding.install_snapshot_for_test(snapshot);
    let corpus: serde_json::Value = serde_json::from_str(fixture_text("../../../📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution-target corpus json");
    let mut lease: semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1 = semio_framework_pack_json::from_json_str(&serde_json::to_string(&corpus["manifest"]).unwrap(), semio_framework_pack_json::JsonMemberPolicy::Reject).expect("manifest");
    lease.scope = DocumentScope::new("space-a", "shared-doc");
    let (descriptor, descriptor_bytes) = catalog_contract_descriptor();
    lease.package.plugin_id = descriptor.manifest.plugin_id.clone();
    lease.package.package_id = descriptor.package_id.clone();
    lease.package.version = descriptor.manifest.version.clone();
    lease.package.component_sha256 = lease.component.sha256.clone();
    lease.package.descriptor_byte_sha256 = framework_hash::sha256_hex(&descriptor_bytes);
    lease.descriptor.sha256 = lease.package.descriptor_byte_sha256.clone();
    lease.descriptor.byte_length = descriptor_bytes.len() as u64;
    lease.package.component_receipt = semio_framework_os_kernel::os_component_receipt::ComponentReceiptV1 { plugin_id: lease.package.plugin_id.clone(), package_id: lease.package.package_id.clone(), wasm_sha256: lease.package.component_sha256.clone(), wasm_bytes: lease.component.byte_length, semantic_descriptor_sha256: lease.package.descriptor_byte_sha256.clone(), ..lease.package.component_receipt.clone() };
    binding.install_catalog_for_test(vec![remote::AuthorizedPackageSelection { scope: lease.scope.clone(), descriptor_digest_v1: lease.descriptor_digest_v1.clone(), lease, descriptor }]);
    let mut workspace = HeadlessWorkspace::new(WorkspaceOrigin::Hub { base_url: "https://hub.invalid".to_string(), space_id: "space-a".to_string() }, "forged-local-principal".to_string(), vec!["admin".to_string()], empty_catalog());
    workspace.catalog = Arc::new(WorkspaceCatalog::following(Arc::clone(&binding)).expect("the fixture's authority compiles"));
    workspace.hub_binding = Some(binding);
    workspace
}

#[test]
fn authenticated_hub_workspace_resources_are_snapshot_only_scoped_and_fail_closed_when_stale() {
    let workspace = authenticated_hub_workspace_fixture();
    let resources = workspace.list_resources().unwrap();
    let uris: Vec<_> = resources.iter().map(|resource| resource.uri.as_str()).collect();
    assert_eq!(uris, vec!["semio://workspace/scopes/space-a/shared-doc/descriptor", "semio://workspace/scopes/space-a/shared-doc/checkpoint",]);
    let workspace_body = workspace.read_resource("semio://workspace").unwrap()[0].text.clone().unwrap();
    assert!(workspace_body.contains("user-a"));
    assert!(!workspace_body.contains("forged-local-principal"));
    assert!(!workspace_body.contains("secret-never-rendered"));
    let descriptor_body = workspace.read_resource("semio://workspace/scopes/space-a/shared-doc/descriptor").unwrap()[0].text.clone().unwrap();
    assert!(descriptor_body.contains("\"spaceId\":\"space-a\""));
    assert!(descriptor_body.contains("\"documentId\":\"shared-doc\""));
    let artifacts_body = workspace.read_resource("semio://workspace/artifacts").unwrap()[0].text.clone().unwrap();
    assert!(artifacts_body.contains("\"checkpointResource\":\"semio://workspace/scopes/space-a/shared-doc/checkpoint\""));
    let checkpoint_error = workspace.read_resource("semio://workspace/scopes/space-b/shared-doc/checkpoint").unwrap_err();
    assert_eq!(checkpoint_error.code, GatewayErrorCode::NotFound);
    let schema_body = workspace.read_resource("semio://artifact/shared-doc/schema").unwrap()[0].text.clone().unwrap();
    let schema: serde_json::Value = serde_json::from_str(&schema_body).unwrap();
    assert_eq!(schema["artifactId"], "shared-doc");
    assert_eq!(schema["spaceId"], "space-a");
    assert!(schema["schema"].is_string(), "{schema_body}");
    let lease:serde_json::Value=serde_json::from_str(fixture_text("../../../📇️directory/🧪️tests/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).unwrap();
    assert_eq!(schema["artifactKind"], lease["manifest"]["parentDialect"]["artifactKind"], "the document kind follows its authenticated parent dialect: {schema_body}");
    for uri in ["semio://artifact/shared-doc", "semio://artifact/shared-doc/validation"] {
        let error = workspace.read_resource(uri).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
        assert!(error.retryable);
    }
    workspace.hub_binding.as_ref().unwrap().invalidate_stream();
    let error = workspace.list_resources().unwrap_err();
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}

#[test]
fn authenticated_hub_discovery_uses_retained_selection_and_never_installed_fallback() {
    let workspace = Arc::new(authenticated_hub_workspace_fixture());
    let descriptors = workspace.discovery_descriptors().expect("ready selected descriptors");
    assert_eq!(descriptors.len(), 1);
    assert_eq!(descriptors[0].manifest.plugin_id, "contract-test");
    let roster = declared_inferences_for_workspace(&workspace).expect("selected inference roster");
    assert_eq!(roster.len(), 1);
    assert_eq!(roster[0].owner, "contract-test");
    let principal = AgentPrincipal::from_scope_names("agent:hub-test", "hub test", &[], None);
    let server = build_server_with_workspace(principal, Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())), workspace.clone(), Box::new(ArtifactChannels::Mock(MockArtifactChannel::new())), GatewayRuntime::default());
    let listed = server.tools.list();
    let inference_list = listed.iter().find(|tool| tool.name == "inference_list").expect("inference_list tool");
    let selected = inference_list.meta.as_ref().expect("ready tools/list selection metadata");
    assert_eq!(selected["semio"]["hubSelectedPackages"][0]["package"]["pluginId"], "contract-test");
    assert_ne!(selected["semio"]["hubSelectedPackages"][0]["package"]["componentSha256"], "");
    assert_eq!(selected["semio"]["hubSelectedPackages"][0]["package"]["executionProtocol"]["appChannelVersion"], semio_framework_os_kernel::os_spr::CHANNEL_VERSION);
    let inference_result = server.tools.call("inference_list", serde_json::json!({})).expect("inference_list registered");
    assert!(!inference_result.is_error);
    assert_eq!(inference_result.structured_content.as_ref().expect("inference roster")["declared"][0]["owner"], "contract-test");
    workspace.hub_binding.as_ref().unwrap().invalidate_stream();
    let descriptor_error = workspace.discovery_descriptors().expect_err("refreshing authority must remove selected descriptors");
    assert_eq!(descriptor_error.code, GatewayErrorCode::PluginUnavailable);
    assert!(descriptor_error.retryable);
    let inference_error = declared_inferences_for_workspace(&workspace).expect_err("inference discovery must not fall back to installed packages");
    assert_eq!(inference_error.code, GatewayErrorCode::PluginUnavailable);
    assert!(inference_error.retryable);
    let revoked_inference_list = server.tools.list().into_iter().find(|tool| tool.name == "inference_list").expect("revoked inference_list tool");
    assert!(revoked_inference_list.meta.is_none(), "refreshing tools/list must expose no stale or installed package identity");
    let revoked_result = server.tools.call("inference_list", serde_json::json!({})).expect("revoked inference_list registered");
    assert!(revoked_result.is_error);
    let error = revoked_result.structured_content.expect("revoked discovery error");
    assert_eq!(error["code"], "PLUGIN_UNAVAILABLE");
    assert_eq!(error["retryable"], true);
}

/// 🔗️ LAW (fixture `🧫️fixtures/🔗️hub-live-links.json`, Python oracle `wp-g12/g12-live-links-oracle.py` of ticket
/// 26/09/23): an agent session keeps one live hub link per app and at most [`HUB_SESSION_LIVE_LINK_LIMIT`] in all —
/// opening a document supersedes its app's older link, relinks its own expired one, and closes the least recently used
/// past the bound; an app's session document is the hub document opened or edited last, a folder's the single one.
#[test]
fn a_hub_session_keeps_one_live_link_per_app_within_its_bound_and_edits_the_document_opened_last() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text("../../🏠️workspace/🧪️tests/🧫️fixtures/🔗️hub-live-links.json")).expect("hub-live-links fixture parses");
    assert_eq!(fixture["limit"].as_u64(), Some(HUB_SESSION_LIVE_LINK_LIMIT as u64), "the fixture pins the session bound");
    let text = |value: &serde_json::Value| value.as_str().expect("string").to_string();
    for case in fixture["close"].as_array().expect("close cases") {
        let live = case["live"].as_array().expect("live").iter().map(|row| (text(&row[0]), text(&row[1]), text(&row[2]), row[3].as_u64().expect("used"))).collect::<Vec<_>>();
        let limit = case["limit"].as_u64().map_or(HUB_SESSION_LIVE_LINK_LIMIT, |limit| limit as usize);
        let closes = hub_links_to_close(&live, case["opening"].as_str().expect("opening"), (case["route"][0].as_str().expect("plugin"), case["route"][1].as_str().expect("app")), limit)
            .into_iter()
            .map(|(artifact_id, close)| serde_json::json!([artifact_id, match close { HubLinkClose::Relinked => "relinked", HubLinkClose::Superseded => "superseded", HubLinkClose::Bounded => "bounded" }]))
            .collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(closes), case["expect"], "{}", case["name"]);
    }
    for case in fixture["session"].as_array().expect("session cases") {
        let bound = case["bound"].as_array().expect("bound").iter().map(|row| (row[0].as_str().expect("artifact"), row[1].as_bool().expect("hub"), row[2].as_u64().expect("used"))).collect::<Vec<_>>();
        assert_eq!(route_session_artifact(bound), case["expect"].as_str(), "{}", case["name"]);
    }
    let (first, second) = (hub_link_tick(), hub_link_tick());
    assert!(second > first, "the hub-link clock is monotonic");
}
