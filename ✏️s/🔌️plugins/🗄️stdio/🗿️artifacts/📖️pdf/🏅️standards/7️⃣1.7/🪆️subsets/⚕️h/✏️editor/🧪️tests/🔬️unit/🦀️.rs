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
