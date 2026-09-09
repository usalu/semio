use super::*;

#[semio_framework_async_macros::async_test]
async fn create_zip_any_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_zip_any_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, ZIP_ANY_VIEWER_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<ZipAnyViewer as ArtifactViewer>::DIALECT, ZIP_ANY_VIEWER_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_main_window() {
    let def = create_zip_any_viewer();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}
