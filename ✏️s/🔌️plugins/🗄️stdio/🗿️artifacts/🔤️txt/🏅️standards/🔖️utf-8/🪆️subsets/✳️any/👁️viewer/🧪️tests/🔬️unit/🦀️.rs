use super::*;

#[semio_framework_async_macros::async_test]
async fn create_txt_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_txt_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, TXT_VIEWER_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<TxtViewer as ArtifactViewer>::DIALECT, TXT_VIEWER_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_text_window() {
    let def = create_txt_viewer();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}
