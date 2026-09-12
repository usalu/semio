use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_ifc2x3_cv20_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, IFC2X3_CV20_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Ifc2x3Cv20Editor as ArtifactEditor>::DIALECT, IFC2X3_CV20_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<Ifc2x3Cv20Editor, crate::viewer::ifc2x3_cv20::Ifc2x3Cv20Viewer>().await;
}
