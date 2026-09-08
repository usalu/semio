
use super::*;

#[semio_framework_async_macros::async_test]
async fn create_pdf17_ua_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_pdf17_ua_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, PDF17UA_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf17UaViewer as ArtifactViewer>::DIALECT, PDF17UA_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_main_window() {
    let def = create_pdf17_ua_viewer();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}
