use super::*;

#[semio_framework_async_macros::async_test]
async fn create_json_i_json_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_json_i_json_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, JSON_I_JSON_VIEWER_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<JsonIJsonViewer as ArtifactViewer>::DIALECT, JSON_I_JSON_VIEWER_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_the_tree_window() {
    let def = create_json_i_json_viewer();
    assert!(def.window_kinds.iter().any(|window| window.id == main::WINDOW_KIND_ID));
}
