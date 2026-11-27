//! 📂️ Folder artifact identity and reopen regression through the native tool registry.

use std::sync::Arc;
use std::sync::atomic::Ordering;
use semio_os_mcp_capabilities::core_capabilities::gateway_only_catalog;
use semio_os_mcp_foundation::errors::GatewayErrorCode;
use semio_os_mcp_workspace::headless_workspace_state::HeadlessWorkspace;

#[test]
fn folder_artifact_open_projects_kind_and_reuses_only_the_current_binding() {
    use semio_os_kernel::native_catalog::{NativeCatalogV1, NativePluginV1, install_native_catalog_v1};
    use semio_os_kernel::os_spr::{DocumentArchivePack, EncodeOptions, HistoryLog, encode_document_archive_bytes, encode_history};
    use semio_os_mcp_port::policy::AgentPrincipal;
    use semio_os_mcp_wire::protocol::{GatewayBackend, InMemoryToolRegistry, ToolRegistry};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/folder-custody.json")).expect("neutral folder reopen vectors");
    let identity = &fixture["reopenIdentity"];
    let plugin_id = identity["pluginId"].as_str().expect("editor owner");
    install_native_catalog_v1(NativeCatalogV1 { plugins: vec![NativePluginV1 {
        plugin_id: plugin_id.into(),
        crate_path: "🌎️hub/🧩️compositions/🗒️note/🧩️component/📦️packages/🦀️rust".into(),
        wasm_out: "note.wasm".into(),
        role: "plugin".into(), extends: None, capabilities: Vec::new(), activation_events: Vec::new(), host: None,
    }], ..NativeCatalogV1::default() }).expect("publish the authored note editor");
    let dir = semio_os_store_test_support::tempdir().expect("folder");
    let storage = semio_os_kernel::sync::FolderEventLogStorage::new(dir.path().to_path_buf());
    let workspace = Arc::new(HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:binding-law".into(), Vec::new(), gateway_only_catalog(), serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/host-driver-policy.json")).expect("explicit host test policy"), serde_json::from_str(include_str!("../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy")).expect("folder workspace"));
    let mut tools = InMemoryToolRegistry::new();
    let principal = AgentPrincipal::from_scope_names("agent:binding-law", "binding-law", &["workspace.read".into()], None);
    semio_os_mcp_tools_gateway::artifact::register_artifact_tools(&mut tools, Some(Arc::clone(&workspace)), principal);
    let id = "note-reopen";
    let schema = identity["schema"].as_str().expect("owner schema");
    let spr = semio_framework_async::block_on(encode_history(&HistoryLog { doc_id: id.into(), schema: schema.into(), ..HistoryLog::default() }, &EncodeOptions::default())).expect("SPR with a real owner schema");
    let export_id = "note-export-before-open";
    let export_spr = semio_framework_async::block_on(encode_history(&HistoryLog { doc_id: export_id.into(), schema: schema.into(), ..HistoryLog::default() }, &EncodeOptions::default())).unwrap();
    semio_framework_async::block_on(storage.write(export_id,"stored-alias",b"persisted",&export_spr)).unwrap();
    assert!(workspace.plugin_artifact_binding(export_id).is_none());
    let _ = tools.call("artifact_export",serde_json::json!({"artifactId":export_id,"format":"artifact:out"})).unwrap();
    let exported = workspace.plugin_artifact_binding(export_id).expect("export resolves the journal's editor before any guest exchange");
    assert_eq!(exported.plugin_id,plugin_id);
    assert_eq!(exported.schema,schema);
    for row in fixture["reopens"].as_array().expect("reopens") {
        let observed = row["observed"].as_str().expect("observed snapshot").as_bytes();
        let durable = row["durable"].as_str().expect("durable snapshot").as_bytes();
        let expected = row["expected"].as_str().expect("transition");
        let previous = workspace.plugin_artifact_binding(id);
        let archive = row["archive"].as_bool().expect("archive availability").then(|| encode_document_archive_bytes(&DocumentArchivePack {
            parent_pack: durable.to_vec(), parent_spr: spr.clone(), members: Vec::new(),
        }).expect("encode recursive archive"));
        if let Some(archive) = &archive {
            semio_framework_async::block_on(storage.write_archive(id, "stored-alias", archive)).expect("write durable archive");
        } else {
            semio_framework_async::block_on(storage.write(id, "stored-alias", durable, &spr)).expect("write durable snapshot");
        }
        if expected == "conflict" {
            let error = workspace.bind_folder_session_document(id, observed, &spr).expect_err("stale open is refused");
            assert_eq!(error.code, GatewayErrorCode::PreconditionFailed);
            assert!(Arc::ptr_eq(&previous.unwrap().relay, &workspace.plugin_artifact_binding(id).unwrap().relay));
            continue;
        }
        let open = tools.call("artifact_open", serde_json::json!({"artifactId": id})).expect("registered tool");
        assert!(!open.is_error, "{:?}", open.structured_content);
        let opened = open.structured_content.expect("open receipt");
        assert_eq!(opened["kind"], schema);
        assert_eq!(opened["artifactKind"], identity["artifactKind"]);
        assert_eq!(opened["sessionDocument"]["pluginId"], plugin_id);
        assert_eq!(opened["sessionDocument"]["appId"], identity["appId"]);
        let binding = workspace.plugin_artifact_binding(id).expect("bound owner");
        assert_eq!(binding.document.as_ref().unwrap().pack, observed);
        assert_eq!(binding.archive.as_deref(), archive.as_ref());
        if let Some(previous) = previous {
            assert_eq!(Arc::ptr_eq(previous.document.as_ref().unwrap(), binding.document.as_ref().unwrap()), expected == "reuse", "{}", row["id"]);
            assert_eq!(Arc::ptr_eq(&previous.relay, &binding.relay), expected == "reuse", "{}", row["id"]);
            assert_eq!(Arc::ptr_eq(&previous.relayed, &binding.relayed), expected == "reuse", "{}", row["id"]);
            assert_eq!(opened["sessionDocument"]["relayedBatches"], if expected == "reuse" { 4 } else { 0 });
        }
        binding.relayed.store(4, Ordering::Relaxed);
        let schema_resource = workspace.read_resource("semio://artifact/note-reopen/schema").expect("schema resource");
        let schema_body: serde_json::Value = serde_json::from_str(schema_resource[0].text.as_deref().unwrap()).unwrap();
        assert_eq!(schema_body["artifactKind"], opened["artifactKind"]);
        let resource = workspace.read_resource("semio://artifact/note-reopen").expect("body resource");
        let body: serde_json::Value = serde_json::from_str(resource[0].text.as_deref().unwrap()).unwrap();
        assert_eq!(body["untrusted"]["provenance"]["artifactKind"], opened["artifactKind"]);
    }
}
