use super::*;

#[semio_framework_async_macros::async_test]
async fn create_deflate_editor_builds_a_definition_for_the_editor_role() {
    let def = create_deflate_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DEFLATE_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DeflateEditor as ArtifactEditor>::DIALECT, DEFLATE_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_deflate_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn parse_header_summary_round_trips_a_rendered_snapshot() {
    let document = DeflateSnapshot { compression_method: 8, window_bits: 9, compression_level_hint: crate::schema::snapshot::DeflateLevelHint::Maximum, dict_id: Some(7), payload: vec![9, 9], ..DeflateSnapshot::default() };
    let node = main::render(&document).expect("render");
    let scene: semio_framework_ui_scene::TextEditorScene = semio_framework_plugin::artifact_app_laws::built_surface_scene(&node).expect("decode the text scene with its lanes");
    let (method, window_bits, level_hint, dict_id) = parse_header_summary(&scene.buffer).expect("well-formed summary must parse");
    assert_eq!(method, 8);
    assert_eq!(window_bits, 9);
    assert_eq!(level_hint, crate::schema::snapshot::DeflateLevelHint::Maximum);
    assert_eq!(dict_id, Some(7));
}

#[semio_framework_async_macros::async_test]
async fn parse_header_summary_rejects_a_missing_required_field() {
    assert!(parse_header_summary("method=8\nwindowBits=7").is_none());
    assert!(parse_header_summary("method=16\nwindowBits=7\nlevelHint=default\npresetDictionary=none").is_none());
    assert!(parse_header_summary("method=8\nwindowBits=16\nlevelHint=default\npresetDictionary=none").is_none());
    assert!(parse_header_summary("method=8\nmethod=9\nwindowBits=7\nlevelHint=default\npresetDictionary=none").is_none());
    assert!(parse_header_summary("method=8\nwindowBits=7\nlevelHint=default\npresetDictionary=none\nunknown=1").is_none());
}

#[test]
fn text_edit_requires_source_text_and_rejects_out_of_schema_headers_atomically() {
    assert!(<DeflateEditor as ArtifactEditor>::command_from_action("textEdit", None).is_err());
    let text = "method=8\nwindowBits=255\nlevelHint=default\npresetDictionary=none";
    let args = dsl::DslValue::object([("text".into(), dsl::DslValue::String(text.into()))]);
    let command = <DeflateEditor as ArtifactEditor>::command_from_action("textEdit", Some(&args)).expect("complete text argument");
    assert!(deflate_text_emit(&command).is_err());
}

#[test]
fn details_reject_window_bits_outside_the_normative_schema_atomically() {
    semio_framework_schema::register_artifact_schema_descriptors(vec![crate::schema::deflate_artifact_schema_descriptor()]).expect("register deflate schema");
    let base = DeflateSnapshot::default();
    let accepted = semio_s_artifact_stdio_contract::editing::apply_snapshot_edit_for_dialect(
        &base,
        &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue { path: "/windowBits".into(), value: dsl::DslValue::uint(15) },
        DEFLATE_EDITOR_DIALECT,
        STDIO_DEFLATE_DOCUMENT_SCHEMA,
    )
    .expect("maximum boundary");
    assert_eq!(accepted.window_bits, 15);
    let error = semio_s_artifact_stdio_contract::editing::apply_snapshot_edit_for_dialect(
        &base,
        &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue { path: "/windowBits".into(), value: dsl::DslValue::uint(255) },
        DEFLATE_EDITOR_DIALECT,
        STDIO_DEFLATE_DOCUMENT_SCHEMA,
    )
    .expect_err("windowBits maximum");
    assert_eq!(error.code, "snapshot-edit.constraint-invalid");
    assert_eq!(error.path, "$.windowBits");
    assert!(error.message.contains("maximum"));
    assert_eq!(base.window_bits, DeflateSnapshot::default().window_bits);
    let identity_error = semio_s_artifact_stdio_contract::editing::apply_snapshot_edit_for_dialect(
        &base,
        &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue { path: "/schema".into(), value: dsl::DslValue::String("stdio.unknown".into()) },
        DEFLATE_EDITOR_DIALECT,
        STDIO_DEFLATE_DOCUMENT_SCHEMA,
    )
    .expect_err("schema identity");
    assert_eq!(identity_error.code, "snapshot-edit.schema-identity");
    assert_eq!(base.schema, STDIO_DEFLATE_DOCUMENT_SCHEMA);
    let mut unknown = base.clone();
    unknown.schema = "stdio.unknown".into();
    let adapter_error = semio_s_artifact_stdio_contract::editing::validate_snapshot_schema_for_dialect(
        &dsl::ToValue::to_value(&unknown),
        DEFLATE_EDITOR_DIALECT,
        STDIO_DEFLATE_DOCUMENT_SCHEMA,
    )
    .expect_err("registered adapter identity");
    assert_eq!(adapter_error.code, "snapshot-edit.schema-identity");
}
