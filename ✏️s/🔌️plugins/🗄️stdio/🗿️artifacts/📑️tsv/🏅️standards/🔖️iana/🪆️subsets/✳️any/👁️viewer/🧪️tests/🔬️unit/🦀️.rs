use super::*;

#[semio_framework_async_macros::async_test]
async fn create_tsv_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_tsv_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, TSV_VIEWER_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<TsvViewer as ArtifactViewer>::DIALECT, TSV_VIEWER_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_table_window() {
    let def = create_tsv_viewer();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}
