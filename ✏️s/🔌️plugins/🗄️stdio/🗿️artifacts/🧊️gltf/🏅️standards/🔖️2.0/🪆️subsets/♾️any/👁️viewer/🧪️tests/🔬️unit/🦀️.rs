use super::*;

#[semio_framework_async_macros::async_test]
async fn create_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_gltf_any_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, GLTF_ANY_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<GltfAnyViewer as ArtifactViewer>::DIALECT, GLTF_ANY_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_never_mutates_the_document_or_draft_store() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<GltfAnyViewer>().await;
}
