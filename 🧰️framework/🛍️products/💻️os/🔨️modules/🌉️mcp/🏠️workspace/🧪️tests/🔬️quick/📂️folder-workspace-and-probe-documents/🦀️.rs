/// 📂️ Folder workspace and probe documents: folder creation, the fresh folder listing, the probe artifact seed, context, resource reads and unknown-handle refusals.

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
    let first = workspace.ensure_probe_artifact("probe-a", serde_json::json!({ "text": "hello" }).into()).await.expect("seed");
    assert_eq!(first.artifact_id, "probe-a");
    assert!(!first.head_edit_id.is_empty(), "a genuinely applied edit has a real edit id");
    let second = workspace.ensure_probe_artifact("probe-a", serde_json::json!({ "text": "should not apply" }).into()).await.expect("idempotent re-open");
    assert_eq!(first.head_edit_id, second.head_edit_id, "re-calling ensure_probe_artifact never double-commits");
}

#[tokio::test]
async fn resolve_context_reports_the_open_probe_artifact_as_active() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), vec!["workspace.read".to_string()], empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-b", serde_json::json!({ "n": 1 }).into()).await.expect("seed");
    let summary = workspace.resolve_context("agent:test").expect("resolve");
    assert_eq!(summary.principal, "agent:test");
    assert_eq!(summary.active_artifact_id.as_deref(), Some("probe-b"));
    assert!(!summary.session_id.is_empty());
}

#[tokio::test]
async fn read_resource_artifact_returns_real_bytes_after_a_commit() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-c", serde_json::json!({ "n": 42 }).into()).await.expect("seed");
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

/// 🔀️ `action_adapter()` no longer needs to resolve ANY plugin just to be BUILT — routing is
/// now per-call (`RoutingArtifactChannel`), so this reaches the real `HandleTable` and gets a
/// real, typed `NOT_FOUND` for the bogus handle — never the old blanket `PLUGIN_UNAVAILABLE`
/// every mutation call used to short-circuit with before a plugin was even looked at (the exact
/// defect `📓️w8-capability-routing.md` fixes).
#[test]
fn invoke_action_on_an_unknown_handle_is_not_found_not_a_panic() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let invoke_error = workspace.invoke_action("prep_x", None).expect_err("unknown handle");
    assert_eq!(invoke_error.code, GatewayErrorCode::NotFound);
}

fn note_and_cad_catalog() -> Arc<Catalog> {
    Arc::new(compile(&note_and_cad_source(), semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native).expect("note+cad fixture source compiles"))
}
