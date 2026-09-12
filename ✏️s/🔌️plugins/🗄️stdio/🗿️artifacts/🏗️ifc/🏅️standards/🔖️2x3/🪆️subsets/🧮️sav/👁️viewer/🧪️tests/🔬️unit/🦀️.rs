use super::*;

#[semio_framework_async_macros::async_test]
async fn create_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_ifc2x3_sav_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, IFC2X3_SAV_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Ifc2x3SavViewer as ArtifactViewer>::DIALECT, IFC2X3_SAV_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_never_mutates_the_document_or_draft_store() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<Ifc2x3SavViewer>().await;
}
