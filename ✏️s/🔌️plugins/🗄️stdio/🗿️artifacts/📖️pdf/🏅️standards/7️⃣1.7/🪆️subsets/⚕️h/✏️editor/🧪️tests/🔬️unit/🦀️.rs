use super::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf17_h_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pdf17_h_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PDF17H_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf17HEditor as ArtifactEditor>::DIALECT, PDF17H_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_pdf17_h_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<Pdf17HEditor as ArtifactEditor>::command_from_action("set-page", None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn explicit_nonzero_page_payload_is_preserved() {
    let args = dsl::DslValue::Object(vec![
        ("page".into(), dsl::DslValue::float(3.0)),
        ("item".into(), dsl::DslValue::float(0.0)),
        ("revision".into(), dsl::DslValue::String("0123456789abcdef".into())),
        ("text".into(), dsl::DslValue::String("replacement".into())),
    ]);
    let command = <Pdf17HEditor as ArtifactEditor>::command_from_action("set-page", Some(&args)).expect("typed payload");
    assert!(matches!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17HEditorCommand::SetPage { page: 3, item: 0, revision, text }) if revision == "0123456789abcdef" && text == "replacement"));
}
