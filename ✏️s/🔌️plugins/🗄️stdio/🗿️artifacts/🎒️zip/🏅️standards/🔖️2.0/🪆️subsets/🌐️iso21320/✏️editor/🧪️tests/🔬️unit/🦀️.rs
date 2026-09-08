
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_zip_iso21320_editor_builds_a_definition_for_the_editor_role() {
    let def = create_zip_iso21320_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, ZIP_ISO21320_EDITOR_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<ZipIso21320Editor as ArtifactEditor>::DIALECT, ZIP_ISO21320_EDITOR_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_zip_iso21320_editor();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}
