
use super::*;

#[test]
fn probe_pack_schema_hash_matches_the_cross_process_descriptor_contract() {
    let actual = store::os_pack::schema_hash(&probe_record_spec()).iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(actual, PROBE_PACK_SCHEMA_HASH);
}

#[test]
fn mcp_probe_document_transport_binds_full_scope_and_exact_surface_authority() {
    let origin = WorkspaceOrigin::Hub { base_url: "https://hub.example".into(), space_id: "space-a".into() };
    assert_eq!(origin.artifact_document_key("shared-document"), store::sync::ArtifactDocumentKey::hub("space-a", "shared-document"));
    assert_ne!(origin.artifact_document_key("shared-document"), store::sync::ArtifactDocumentKey::hub("space-b", "shared-document"));
    // 🪪️ The probe verifies no execution-target bytes, so it claims no lease and binds only its
    // requested surface id; a forgeable local target is no longer an admission input anywhere.
    match origin.persistence_binding() {
        store::sync::PersistenceBinding::Hub { surface, .. } => assert_eq!(surface.as_deref(), Some(PROBE_SURFACE_ID)),
        store::sync::PersistenceBinding::Folder { .. } => panic!("hub origin must bind a hub persistence binding"),
    }
    assert!(!std::fs::read_to_string(Path::new(file!())).expect("probe source").contains("probe_document_socket_surface"));
}

#[test]
fn gis_map_inference_selector_requires_the_exact_kind_and_schema_pair() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔐️canonical-checkpoint-resource/🔣️.json")).unwrap();
    let selector = &fixture["selector"];
    assert!(is_gis_map_descriptor(selector["artifactKind"].as_str().unwrap(), selector["artifactSchema"].as_str().unwrap()));
    for hostile in selector["hostile"].as_array().unwrap() {
        assert!(!is_gis_map_descriptor(hostile["artifactKind"].as_str().unwrap(), hostile["artifactSchema"].as_str().unwrap()), "partial GIS identity {} must fail closed", hostile["name"].as_str().unwrap());
    }
}

fn empty_catalog() -> Arc<Catalog> {
    Arc::new(crate::compile(&crate::CatalogSource::default(), semio_framework::Locale::En, semio_framework::Terminology::Native).expect("empty catalog source compiles"))
}

fn authenticated_hub_workspace_fixture() -> HeadlessWorkspace {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🔗️remote/🧫️fixtures/🔣️authenticated-hub-descriptor-index.json")).unwrap();
    let ready = &fixture["cases"]["memberReady"];
    let page = semio_framework_os_kernel::os_directory::DirectorySpaceAdministrationPageV1::parse_canonical_json(ready["responses"][1]["canonicalBody"].as_str().unwrap()).unwrap();
    let semio_framework_os_kernel::os_directory::DirectorySpaceAdministrationPageV1::Author { space, members, documents, .. } = page else { panic!("author administration page fixture") };
    let members: Vec<semio_framework_os_kernel::os_directory::MemberView> =
        members.rows.into_iter().map(|row| semio_framework_os_kernel::os_directory::MemberView { user_id: row.user_id, email: row.email, display_name: row.display_name, role: row.role }).collect();
    let documents = documents.rows;
    let view = documents[0].clone();
    let scope = DocumentScope::new("space-a", "shared-doc");
    let digest = semio_framework_os_kernel::os_directory::descriptor_digest_v1(&view.descriptor).unwrap();
    let document = remote::AuthorizedDocumentView { scope: scope.clone(), descriptor_digest_v1: semio_framework_os_kernel::os_directory::hex_lower(digest.as_bytes()), view };
    let snapshot = AuthorizedDescriptorSnapshot { authenticated_user_id: "user-a".to_string(), session_expires_at_ms: i64::MAX, space, membership: members[0].clone(), observed_event_seq: 8, documents: HashMap::from([(scope, document)]) };
    let binding = Arc::new(HubRemoteBinding::new("https://hub.invalid", "space-a").unwrap());
    binding.install_snapshot_for_test(snapshot);
    let repo_root = find_repo_root().expect("repo root");
    let corpus: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(repo_root.join("🌎️hub/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution-target corpus")).expect("execution-target corpus json");
    let lease: semio_framework_os_kernel::os_directory::DocumentExecutionTargetLeaseFieldsV1 = semio_framework_os_kernel::os_pack::json::from_json_str(&serde_json::to_string(&corpus["manifest"]).unwrap()).expect("manifest");
    let descriptor = load_package_descriptor(&repo_root.join("✏️s/🔌️plugins/🌍️gis")).expect("installed GIS descriptor test input");
    binding.install_catalog_for_test(vec![remote::AuthorizedPackageSelection { scope: lease.scope.clone(), descriptor_digest_v1: lease.descriptor_digest_v1.clone(), lease, descriptor }]);
    let mut workspace = HeadlessWorkspace::new(WorkspaceOrigin::Hub { base_url: "https://hub.invalid".to_string(), space_id: "space-a".to_string() }, "forged-local-principal".to_string(), vec!["admin".to_string()], empty_catalog());
    workspace.hub_binding = Some(binding);
    workspace
}

#[test]
fn authenticated_hub_workspace_resources_are_snapshot_only_scoped_and_fail_closed_when_stale() {
    let workspace = authenticated_hub_workspace_fixture();
    let resources = workspace.list_resources().unwrap();
    let uris: Vec<_> = resources.iter().map(|resource| resource.uri.as_str()).collect();
    assert_eq!(uris, vec!["semio://workspace", "semio://workspace/artifacts", "semio://workspace/scopes/space-a/shared-doc/descriptor", "semio://workspace/scopes/space-a/shared-doc/checkpoint",]);
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
    for uri in ["semio://artifact/shared-doc", "semio://artifact/shared-doc/schema", "semio://artifact/shared-doc/validation"] {
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
    assert_eq!(descriptors[0].manifest.plugin_id, "gis");
    let roster = crate::inference::declared_inferences_for_workspace(&workspace).expect("selected inference roster");
    assert_eq!(roster.len(), 1);
    assert_eq!(roster[0].owner, "gis");
    let principal = AgentPrincipal::from_scope_names("agent:hub-test", "hub test", &[], None);
    let server = crate::build_server_with_workspace(principal, Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new())), workspace.clone(), Box::new(ArtifactChannels::Mock(MockArtifactChannel::new())), None);
    let listed = server.tools.list();
    let inference_list = listed.iter().find(|tool| tool.name == "inference_list").expect("inference_list tool");
    let selected = inference_list.meta.as_ref().expect("ready tools/list selection metadata");
    assert_eq!(selected["semio"]["hubSelectedPackages"][0]["package"]["pluginId"], "gis");
    assert_ne!(selected["semio"]["hubSelectedPackages"][0]["package"]["componentSha256"], "");
    assert_eq!(selected["semio"]["hubSelectedPackages"][0]["package"]["executionProtocol"]["appChannelVersion"], semio_framework_os_kernel::os_spr::CHANNEL_VERSION);
    let inference_result = server.tools.call("inference_list", serde_json::json!({})).expect("inference_list registered");
    assert!(!inference_result.is_error);
    assert_eq!(inference_result.structured_content.as_ref().expect("inference roster")["declared"][0]["owner"], "gis");
    workspace.hub_binding.as_ref().unwrap().invalidate_stream();
    let descriptor_error = workspace.discovery_descriptors().expect_err("refreshing authority must remove selected descriptors");
    assert_eq!(descriptor_error.code, GatewayErrorCode::PluginUnavailable);
    assert!(descriptor_error.retryable);
    let inference_error = crate::inference::declared_inferences_for_workspace(&workspace).expect_err("inference discovery must not fall back to installed GIS");
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

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn pending_response_close_releases_one_exact_fixed_page() {
    let mut response = PendingResponsePage::Empty;
    response.admit(semio_framework::kernel::RequestOutcome::Ok(vec![7; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]));
    assert_eq!(response.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES - 1), (false, 0));
    assert_eq!(response.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES), (true, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES));
    assert!(response.terminal_is_empty());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn pending_response_faults_oversize_and_duplicate_without_retaining_app_frame() {
    let mut oversized = PendingResponsePage::Empty;
    oversized.admit(semio_framework::kernel::RequestOutcome::Ok(vec![1; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES + 1]));
    assert_eq!(oversized.take(9).unwrap_err().code, "channel.not-wired");
    let mut duplicate = PendingResponsePage::Empty;
    duplicate.admit(semio_framework::kernel::RequestOutcome::Err(vec![2]));
    duplicate.admit(semio_framework::kernel::RequestOutcome::Ok(vec![3]));
    assert!(duplicate.take(10).unwrap_err().message.contains("more than one response"));
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn guest_fault_wire_decodes_through_the_first_party_value_codec() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️first-party-codecs.json")).expect("language-neutral codec fixture parses");
    let wire = store::DslValue::from(&fixture["guestFault"]["wire"]);
    let fault = decode_guest_fault(&store::pack_rt::encode_wire_value(&wire));
    assert_eq!(fault.code, fixture["guestFault"]["expected"]["code"].as_str().expect("fixture fault code"));
    assert_eq!(fault.message, fixture["guestFault"]["expected"]["message"].as_str().expect("fixture fault message"));
}

/// 🪲️ Regression law for the two-frame `to_value` → `to_dsl_value` → `to_value` recursion that
/// aborted every probe-committing test with `fatal runtime error: stack overflow`: reaching an
/// assertion at all proves the cycle is gone, and the fixture pins the exact wire shape so the
/// cure cannot silently change the encoding. `serde_json` is the independent third-party oracle
/// — the first-party `ToValue` tree must equal what it serializes for the same values.
#[test]
fn probe_codec_encodes_the_fixture_shape_and_agrees_with_the_third_party_serializer() {
    use store::{FromValue as _, ToValue as _};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/🔣️first-party-codecs.json")).expect("language-neutral codec fixture parses");
    let probe = &fixture["probeCodec"];
    let value = probe["value"].clone();
    let snapshot = ProbeSnapshot(value.clone());
    let diff = ProbeDiff(value.clone());
    let mutation = ProbeMutation::SetValue(value.clone());

    assert_eq!(serde_json::Value::from(snapshot.to_value()), probe["snapshotEncoding"]);
    assert_eq!(serde_json::Value::from(diff.to_value()), probe["diffEncoding"]);
    assert_eq!(serde_json::Value::from(mutation.to_value()), probe["mutationEncoding"]);
    assert_eq!(serde_json::to_value(&snapshot).expect("serde oracle"), probe["snapshotEncoding"]);
    assert_eq!(serde_json::to_value(&diff).expect("serde oracle"), probe["diffEncoding"]);
    assert_eq!(serde_json::to_value(&mutation).expect("serde oracle"), probe["mutationEncoding"]);

    assert_eq!(ProbeSnapshot::from_value(snapshot.to_value()).expect("snapshot round trip"), snapshot);
    assert_eq!(ProbeDiff::from_value(diff.to_value()).expect("diff round trip"), diff);
    assert_eq!(ProbeMutation::from_value(mutation.to_value()).expect("mutation round trip"), mutation);

    for rejected in probe["rejectedMutationEncodings"].as_array().expect("fixture rejection cases") {
        assert!(ProbeMutation::from_value(store::DslValue::from(rejected)).is_err(), "must reject {rejected}");
    }
}

#[test]
fn open_folder_creates_the_directory_if_missing() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let target = dir.path().join("nested").join("space");
    let workspace = HeadlessWorkspace::open_folder(target.clone(), "agent:test".to_string(), vec!["workspace.read".to_string()], empty_catalog()).expect("opens and creates");
    assert!(target.is_dir());
    assert_eq!(workspace.origin().describe(), format!("folder://{}", target.display()));
}

#[test]
fn a_fresh_folder_workspace_lists_zero_artifacts() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    assert_eq!(workspace.workspace_artifact_ids().expect("list"), Vec::<String>::new());
}

#[tokio::test]
async fn ensure_probe_artifact_seeds_a_real_revision_and_is_idempotent() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let first = workspace.ensure_probe_artifact("probe-a", serde_json::json!({ "text": "hello" })).await.expect("seed");
    assert_eq!(first.artifact_id, "probe-a");
    assert!(!first.head_edit_id.is_empty(), "a genuinely applied edit has a real edit id");
    let second = workspace.ensure_probe_artifact("probe-a", serde_json::json!({ "text": "should not apply" })).await.expect("idempotent re-open");
    assert_eq!(first.head_edit_id, second.head_edit_id, "re-calling ensure_probe_artifact never double-commits");
}

#[tokio::test]
async fn resolve_context_reports_the_open_probe_artifact_as_active() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), vec!["workspace.read".to_string()], empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-b", serde_json::json!({ "n": 1 })).await.expect("seed");
    let summary = workspace.resolve_context("agent:test").expect("resolve");
    assert_eq!(summary.principal, "agent:test");
    assert_eq!(summary.active_artifact_id.as_deref(), Some("probe-b"));
    assert!(!summary.session_id.is_empty());
}

#[tokio::test]
async fn read_resource_artifact_returns_real_bytes_after_a_commit() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-c", serde_json::json!({ "n": 42 })).await.expect("seed");
    let contents = workspace.read_resource("semio://artifact/probe-c").expect("read");
    let body: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().expect("text body")).expect("json body");
    assert!(body["packBytes"].as_u64().unwrap_or(0) > 0, "a real committed edit persists non-empty pack bytes: {body}");
}

#[test]
fn read_resource_on_an_unknown_artifact_is_not_found_not_fabricated() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let error = workspace.read_resource("semio://artifact/does-not-exist").expect_err("must not fabricate");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}

#[test]
fn prepare_action_on_an_unknown_capability_is_not_found_not_a_panic() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let prepare_error = workspace.prepare_action("cad.editor.translateSelection", serde_json::json!({}), None).expect_err("unknown capability in an empty catalog");
    assert_eq!(prepare_error.code, GatewayErrorCode::NotFound);
}

#[test]
fn invoke_action_on_an_unknown_handle_is_not_found_not_a_panic() {
    // 🔀️ `action_adapter()` no longer needs to resolve ANY plugin just to be BUILT — routing is
    // now per-call (`RoutingArtifactChannel`), so this reaches the real `HandleTable` and gets a
    // real, typed `NOT_FOUND` for the bogus handle — never the old blanket `PLUGIN_UNAVAILABLE`
    // every mutation call used to short-circuit with before a plugin was even looked at (the exact
    // defect `📓️w8-capability-routing.md` fixes).
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let invoke_error = workspace.invoke_action("prep_x", None).expect_err("unknown handle");
    assert_eq!(invoke_error.code, GatewayErrorCode::NotFound);
}

fn note_and_cad_catalog() -> Arc<Catalog> {
    Arc::new(crate::compile(&crate::testkit::note_and_cad_source(), semio_framework::Locale::En, semio_framework::Terminology::Native).expect("note+cad fixture source compiles"))
}

#[test]
fn resolve_plugin_for_capability_is_not_found_for_an_unknown_id() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), note_and_cad_catalog()).expect("opens");
    let error = workspace.resolve_plugin_for_capability("totally.unknown.capability").expect_err("no such capability");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}

#[test]
fn resolve_plugin_for_capability_is_plugin_unavailable_for_a_gateway_owned_id() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), note_and_cad_catalog()).expect("opens");
    let error = workspace.resolve_plugin_for_capability("capabilities.search").expect_err("gateway-owned, not a plugin");
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}

#[test]
fn resolve_plugin_for_capability_routes_note_and_cad_to_different_plugins() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), note_and_cad_catalog()).expect("opens");
    assert_eq!(workspace.resolve_plugin_for_capability("note.editor.setGridVisible").expect("note-owned"), "note");
    assert_eq!(workspace.resolve_plugin_for_capability("cad.editor.addObject").expect("cad-owned"), "cad");
}

#[test]
fn routing_artifact_channel_purecommand_unknown_capability_is_not_found_before_opening_any_channel() {
    let mut router = RoutingArtifactChannel::new(note_and_cad_catalog(), None, "agent:test#sess".to_string());
    let fault = router.exchange(0, vec![AppCommand::PureCommand { capability_id: "totally.unknown.capability".to_string(), input: serde_json::json!({}) }]).expect_err("unknown capability must not route to any plugin");
    assert_eq!(fault.code, "capability.not-found");
}

#[test]
fn routing_artifact_channel_purecommand_gateway_owned_capability_is_plugin_unavailable() {
    let mut router = RoutingArtifactChannel::new(note_and_cad_catalog(), None, "agent:test#sess".to_string());
    let fault = router.exchange(0, vec![AppCommand::PureCommand { capability_id: "capabilities.search".to_string(), input: serde_json::json!({}) }]).expect_err("a gateway-owned capability names no plugin channel");
    assert_eq!(fault.code, "plugin.unavailable");
}

#[test]
fn routing_artifact_channel_exchange_on_an_unrouted_instance_without_a_purecommand_is_plugin_unavailable() {
    let mut router = RoutingArtifactChannel::new(empty_catalog(), None, "agent:test#sess".to_string());
    let fault = router.exchange(0, vec![AppCommand::ReadHistory]).expect_err("no known plugin for this instance and no PureCommand to derive one from");
    assert_eq!(fault.code, "plugin.unavailable");
}

/// 🎫️ W8: the actual defect this ticket fixes — a multi-plugin catalog (note+cad) must route two
/// different capability ids to two different real plugin channels, and open each plugin's channel
/// at most once across repeated commands (including a `ReadHistory` for `note` that carries no
/// capability id at all, decoded via `plugin_for_instance_slot` instead). Skipped with a clear
/// message when the note/cad `.wasm` fixtures are not built (never a fabricated pass) — same
/// convention as `plugin_artifact_channel_mutation_verbs_are_real_round_trips_never_not_wired`.
#[test]
fn routing_artifact_channel_routes_two_capabilities_to_two_different_plugins_opening_each_once() {
    let repo_root = match find_repo_root() {
        Ok(root) => root,
        Err(_) => {
            eprintln!("skipped: repo root not found from this test binary's CARGO_MANIFEST_DIR");
            return;
        }
    };
    let registry = match load_plugin_registry(&repo_root) {
        Ok(registry) => registry,
        Err(error) => {
            eprintln!("skipped: plugin registry not generated: {error}");
            return;
        }
    };
    for plugin_id in ["note", "cad"] {
        let Ok(entry) = find_plugin_entry(&registry, plugin_id) else {
            eprintln!("skipped: `{plugin_id}` not in the plugin registry");
            return;
        };
        if resolve_plugin_wasm_path(&repo_root, entry).is_err() {
            eprintln!("skipped: {plugin_id}.wasm not built at target/wasm32-wasip2/{{wasm-dev,wasm-release}}");
            return;
        }
    }
    let catalog = note_and_cad_catalog();
    let note_instance = plugin_instance_slot(&catalog, "note").expect("note is in the fixture catalog");
    let cad_instance = plugin_instance_slot(&catalog, "cad").expect("cad is in the fixture catalog");
    let mut router = RoutingArtifactChannel::new(catalog, Some(repo_root), "agent:routing-test#sess".to_string());

    let note_result = router.exchange(note_instance, vec![AppCommand::PureCommand { capability_id: "note.editor.setGridVisible".to_string(), input: serde_json::json!({}) }]);
    assert_ne!(note_result.as_ref().err().map(|fault| fault.code.as_str()), Some("plugin.unavailable"), "note.editor.setGridVisible must route to a real note channel: {note_result:?}");

    let cad_result = router.exchange(cad_instance, vec![AppCommand::PureCommand { capability_id: "cad.editor.addObject".to_string(), input: serde_json::json!({}) }]);
    assert_ne!(cad_result.as_ref().err().map(|fault| fault.code.as_str()), Some("plugin.unavailable"), "cad.editor.addObject must route to a real cad channel: {cad_result:?}");

    // 🔁️ Re-exchange against `note` via `ReadHistory`, which carries no capability id at all —
    // proving `plugin_for_instance_slot` decodes the SAME plugin the earlier `PureCommand` did,
    // and that this second call reuses the cached channel rather than opening a second one.
    let _ = router.exchange(note_instance, vec![AppCommand::ReadHistory]);
    assert_eq!(router.channels.lock().expect("cache lock").len(), 2, "exactly one cached channel per distinct plugin routed to (note, cad), never one per call");
}

#[test]
fn read_artifact_resource_validation_is_plugin_unavailable_never_hardcoded_true() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let error = workspace.read_resource("semio://artifact/does-not-exist/validation").expect_err("must not fabricate `valid: true` for an artifact this workspace has never seen");
    assert_eq!(error.code, GatewayErrorCode::PluginUnavailable);
    assert!(error.retryable);
}

#[test]
fn read_artifact_resource_schema_is_real_for_an_open_probe_and_plugin_unavailable_otherwise() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let missing = workspace.read_resource("semio://artifact/does-not-exist/schema").expect_err("no schema query command exists on the real wire yet");
    assert_eq!(missing.code, GatewayErrorCode::PluginUnavailable);
}

#[tokio::test]
async fn read_artifact_resource_schema_is_real_for_an_open_probe() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-schema", serde_json::json!({})).await.expect("seed");
    let contents = workspace.read_resource("semio://artifact/probe-schema/schema").expect("real answer for an open probe artifact");
    let body: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().expect("text body")).expect("json body");
    assert_eq!(body["schema"], PROBE_SCHEMA);
}

#[tokio::test]
async fn apply_probe_mutation_commits_a_real_second_edit_beyond_the_seed() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let seeded = workspace.ensure_probe_artifact("probe-mutate", serde_json::json!({ "n": 1 })).await.expect("seed");
    let mutated = workspace.apply_probe_mutation("probe-mutate", serde_json::json!({ "n": 2 })).await.expect("real second commit");
    assert_ne!(seeded.head_edit_id, mutated.head_edit_id, "a genuine second edit gets a genuinely different edit id");
    assert_ne!(seeded, mutated, "the revision stamp a real caller would compare as `expectedRevision` differs — this is exactly the predicate REVISION_CONFLICT is built on");
    let bytes = workspace.read_artifact_bytes("probe-mutate").expect("read").expect("artifact exists");
    assert!(bytes.0.len() > 0, "the mutated pack is real, non-empty bytes");
}

#[tokio::test]
async fn undo_then_redo_round_trips_a_real_probe_mutation() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-undo-redo", serde_json::json!({ "n": 1 })).await.expect("seed");
    workspace.apply_probe_mutation("probe-undo-redo", serde_json::json!({ "n": 2 })).await.expect("second commit");
    let undone = workspace.undo_probe_mutation("probe-undo-redo").await.expect("real undo");
    assert_eq!(undone, serde_json::json!({ "n": 1 }), "undo reverts to the seeded value for real");
    let redone = workspace.redo_probe_mutation("probe-undo-redo").await.expect("real redo");
    assert_eq!(redone, serde_json::json!({ "n": 2 }), "redo restores the undone edit for real — a genuine round trip");
}

#[test]
fn base64_encode_matches_a_known_vector() {
    assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
    assert_eq!(base64_encode(b""), "");
}

#[test]
fn find_plugin_entry_reports_a_typed_not_found_for_an_unknown_plugin() {
    let error = find_plugin_entry(&[], "does-not-exist").expect_err("must not fabricate an entry");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}
