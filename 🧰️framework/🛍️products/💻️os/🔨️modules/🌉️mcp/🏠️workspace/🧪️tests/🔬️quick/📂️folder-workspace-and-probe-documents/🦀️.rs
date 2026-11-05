/// 📂️ Folder workspace and probe documents: folder creation, the fresh folder listing, the probe artifact seed, context, resource reads and unknown-handle refusals.

#[test]
fn open_folder_creates_the_directory_if_missing() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let target = dir.path().join("nested").join("space");
    let workspace = folder_test_workspace(target.clone(), vec!["workspace.read".to_string()]);
    assert!(target.is_dir());
    assert_eq!(workspace.origin().describe(), format!("folder://{}", target.display()));
}

#[test]
fn a_fresh_folder_workspace_lists_zero_artifacts() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = folder_test_workspace(dir.path().to_path_buf(), Vec::new());
    assert_eq!(workspace.workspace_artifact_ids().expect("list"), Vec::<String>::new());
}

#[tokio::test]
async fn ensure_probe_artifact_seeds_a_real_revision_and_is_idempotent() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = folder_test_workspace(dir.path().to_path_buf(), Vec::new());
    let first = workspace.ensure_probe_artifact("probe-a", serde_json::json!({ "text": "hello" }).into()).await.expect("seed");
    assert_eq!(first.artifact_id, "probe-a");
    assert!(!first.head_edit_id.is_empty(), "a genuinely applied edit has a real edit id");
    let second = workspace.ensure_probe_artifact("probe-a", serde_json::json!({ "text": "should not apply" }).into()).await.expect("idempotent re-open");
    assert_eq!(first.head_edit_id, second.head_edit_id, "re-calling ensure_probe_artifact never double-commits");
}

#[tokio::test]
async fn resolve_context_reports_the_open_probe_artifact_as_active() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = folder_test_workspace(dir.path().to_path_buf(), vec!["workspace.read".to_string()]);
    workspace.ensure_probe_artifact("probe-b", serde_json::json!({ "n": 1 }).into()).await.expect("seed");
    let summary = workspace.resolve_context("agent:test", "de").expect("resolve");
    assert_eq!(summary.principal, "agent:test");
    assert_eq!(summary.active_artifact_id.as_deref(), Some("probe-b"));
    assert!(!summary.session_id.is_empty());
}

#[tokio::test]
async fn read_resource_artifact_returns_real_bytes_after_a_commit() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = folder_test_workspace(dir.path().to_path_buf(), Vec::new());
    workspace.ensure_probe_artifact("probe-c", serde_json::json!({ "n": 42 }).into()).await.expect("seed");
    let contents = workspace.read_resource("semio://artifact/probe-c").expect("read");
    let body: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().expect("text body")).expect("json body");
    assert!(body["packBytes"].as_u64().unwrap_or(0) > 0, "a real committed edit persists non-empty pack bytes: {body}");
}

#[test]
fn read_resource_on_an_unknown_artifact_is_not_found_not_fabricated() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = folder_test_workspace(dir.path().to_path_buf(), Vec::new());
    let error = workspace.read_resource("semio://artifact/does-not-exist").expect_err("must not fabricate");
    assert_eq!(error.code, GatewayErrorCode::NotFound);
}

#[test]
fn prepare_action_on_an_unknown_capability_is_not_found_not_a_panic() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = folder_test_workspace(dir.path().to_path_buf(), Vec::new());
    let prepare_error = workspace.prepare_action("cad.editor.translateSelection", serde_json::json!({}), None).expect_err("unknown capability in an empty catalog");
    assert_eq!(prepare_error.code, GatewayErrorCode::NotFound);
}

/// 🔀️ `action_adapter()` no longer needs to resolve ANY plugin just to be BUILT — routing is
/// now per-call (`RoutingArtifactChannel`), so this reaches the real `HandleTable` and gets a
/// real, typed `NOT_FOUND` for the bogus handle — never the old blanket `PLUGIN_UNAVAILABLE`
/// every mutation call used to short-circuit with before a plugin was even looked at (the exact
/// defect `📓️w8-capability-routing.md` fixes).
#[test]
fn invoke_action_on_an_unknown_handle_is_not_found_not_a_panic() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = folder_test_workspace(dir.path().to_path_buf(), Vec::new());
    let invoke_error = workspace.invoke_action("prep_x", None).expect_err("unknown handle");
    assert_eq!(invoke_error.code, GatewayErrorCode::NotFound);
}

fn note_and_cad_catalog() -> Arc<Catalog> {
    Arc::new(compile(&note_and_cad_source(), semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native).expect("note+cad fixture source compiles"))
}

#[tokio::test]
async fn folder_canonical_snapshot_events_refuse_stale_writers_and_reopen_after_undo_redo() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/folder-custody.json")).expect("neutral folder vectors");
    let dir = store::test_support::tempdir().expect("tempdir");
    let storage = store::sync::FolderEventLogStorage::new(dir.path().to_path_buf());
    let mut last = None;
    for row in fixture["commits"].as_array().expect("commits") {
        let expected = row["expected"].as_array().map(|pair| (pair[0].as_str().expect("pack").as_bytes(), pair[1].as_str().expect("spr").as_bytes()));
        let pack = row["pack"].as_str().expect("pack").as_bytes();
        let spr = row["spr"].as_str().expect("spr").as_bytes();
        let accepted = storage.write_if_current("artifact-a", "schema1", expected, pack, spr).await.expect("conditional append");
        assert_eq!(accepted, row["accepted"].as_bool().expect("accepted"), "{}", row["id"]);
        if accepted { last = Some((pack.to_vec(), spr.to_vec())); }
        assert_eq!(store::sync::FolderEventLogStorage::new(dir.path().to_path_buf()).read("artifact-a").await.expect("reopen"), last);
    }
}

#[test]
fn folder_schema_binding_requires_one_declared_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/folder-custody.json")).expect("neutral owners");
    for row in fixture["owners"].as_array().expect("owners") {
        let kinds = folder_declared_kinds(row);
        let selected = HeadlessWorkspace::declared_artifact_owner(row["schema"].as_str().expect("schema"), &kinds);
        assert_eq!(matches!(selected, Ok(Some(_))), row["accepted"].as_bool().expect("accepted"), "{}", row["id"]);
    }
}

#[tokio::test]
async fn folder_snapshot_refuses_an_active_writer_and_io_failure_without_acknowledging() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let storage = store::sync::FolderEventLogStorage::new(dir.path().to_path_buf());
    storage.write("artifact", "schema", b"pack0", b"spr0").await.expect("seed");
    let file = std::fs::OpenOptions::new().read(true).write(true).open(storage.event_path()).expect("log");
    file.lock().expect("peer owns writer");
    assert!(storage.write_if_current("artifact", "schema", Some((b"pack0", b"spr0")), b"pack1", b"spr1").await.is_err());
    drop(file);
    assert_eq!(storage.read("artifact").await.expect("read"), Some((b"pack0".to_vec(), b"spr0".to_vec())));
    let failed = dir.path().join("failed");
    std::fs::create_dir(&failed).expect("folder");
    std::fs::write(failed.join(".semio"), b"not a directory").expect("blocked path");
    assert!(store::sync::FolderEventLogStorage::new(failed).write_if_current("artifact", "schema", None, b"pack1", b"spr1").await.is_err());
}

#[tokio::test]
async fn folder_create_uses_owner_spr_schema_instead_of_requested_kind_alias() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = folder_test_workspace(dir.path().to_path_buf(), Vec::new());
    workspace.ensure_probe_artifact("alias", serde_json::json!({"n":1}).into()).await.expect("actual canonical source");
    let (_, spr) = workspace.read_artifact_bytes("alias").expect("read").expect("pair");
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/folder-custody.json")).expect("alias vector");
    let row = fixture["owners"].as_array().expect("owners").iter().find(|row| row["id"] == "requested-alias-actual-schema").expect("alias case");
    let kinds = folder_declared_kinds(row);
    let owner = HeadlessWorkspace::canonical_artifact_owner(&spr, &kinds).expect("actual declared owner");
    assert_eq!(owner.schema, row["schema"].as_str().unwrap());
    assert_ne!(owner.schema, kinds[0].schema);
}

fn folder_declared_kinds(row: &serde_json::Value) -> Vec<semio_framework_os_mcp_routing::hub_relay::InstalledArtifactKind> {
    row["declarations"].as_array().expect("declarations").iter().map(|value| semio_framework_os_mcp_routing::hub_relay::InstalledArtifactKind {
        schema: value[0].as_str().unwrap().into(), plugin_id: value[1].as_str().unwrap().into(), app_id: value[2].as_str().unwrap().into(), media_out_ports: Vec::new(), export_formats: Vec::new(),
    }).collect()
}


fn folder_test_workspace(path: std::path::PathBuf, scopes: Vec<String>) -> HeadlessWorkspace {
    HeadlessWorkspace::open_folder(path, "agent:test".into(), scopes, empty_catalog(), serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/host-driver-policy.json")).expect("policy"), serde_json::from_str(include_str!("../../../../🎚️config/💡️host-operation-policy.json")).expect("explicit host funding policy")).expect("folder workspace")
}

