/// 📖️ Artifact resources and probe edits: artifact resource validation and schemas, probe mutation commit, undo and redo, the base64 vector and the plugin entry lookup.

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
    workspace.ensure_probe_artifact("probe-schema", serde_json::json!({}).into()).await.expect("seed");
    let contents = workspace.read_resource("semio://artifact/probe-schema/schema").expect("real answer for an open probe artifact");
    let body: serde_json::Value = serde_json::from_str(contents[0].text.as_ref().expect("text body")).expect("json body");
    assert_eq!(body["schema"], PROBE_SCHEMA);
}

#[tokio::test]
async fn apply_probe_mutation_commits_a_real_second_edit_beyond_the_seed() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    let seeded = workspace.ensure_probe_artifact("probe-mutate", serde_json::json!({ "n": 1 }).into()).await.expect("seed");
    let mutated = workspace.apply_probe_mutation("probe-mutate", serde_json::json!({ "n": 2 }).into()).await.expect("real second commit");
    assert_ne!(seeded.head_edit_id, mutated.head_edit_id, "a genuine second edit gets a genuinely different edit id");
    assert_ne!(seeded, mutated, "the revision stamp a real caller would compare as `expectedRevision` differs — this is exactly the predicate REVISION_CONFLICT is built on");
    let bytes = workspace.read_artifact_bytes("probe-mutate").expect("read").expect("artifact exists");
    assert!(bytes.0.len() > 0, "the mutated pack is real, non-empty bytes");
}

#[tokio::test]
async fn undo_then_redo_round_trips_a_real_probe_mutation() {
    let dir = store::test_support::tempdir().expect("tempdir");
    let workspace = HeadlessWorkspace::open_folder(dir.path().to_path_buf(), "agent:test".to_string(), Vec::new(), empty_catalog()).expect("opens");
    workspace.ensure_probe_artifact("probe-undo-redo", serde_json::json!({ "n": 1 }).into()).await.expect("seed");
    workspace.apply_probe_mutation("probe-undo-redo", serde_json::json!({ "n": 2 }).into()).await.expect("second commit");
    let undone = workspace.undo_probe_mutation("probe-undo-redo").await.expect("real undo");
    assert_eq!(serde_json::Value::from(undone), serde_json::json!({ "n": 1 }), "undo reverts to the seeded value for real");
    let redone = workspace.redo_probe_mutation("probe-undo-redo").await.expect("real redo");
    assert_eq!(serde_json::Value::from(redone), serde_json::json!({ "n": 2 }), "redo restores the undone edit for real — a genuine round trip");
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

#[tokio::test]
async fn probe_set_value_inverse_diffs_sum_to_the_negative_diff() {
    let base = |json: serde_json::Value| ProbeSnapshot(json.into());
    let set = |json: serde_json::Value| ProbeMutation::SetValue(json.into());
    store::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&set(serde_json::json!({ "n": 2, "tags": ["a", "b"] })), &base(serde_json::json!({ "n": 1 }))).await;
    store::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&set(serde_json::json!([1, { "leaf": "x" }, 3])), &base(serde_json::json!("scalar"))).await;
    store::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&set(serde_json::json!({ "n": 1 })), &base(serde_json::json!(null))).await;
}
