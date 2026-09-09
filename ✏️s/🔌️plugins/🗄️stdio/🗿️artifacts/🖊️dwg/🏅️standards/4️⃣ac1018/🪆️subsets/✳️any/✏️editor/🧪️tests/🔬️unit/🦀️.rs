use super::*;

#[semio_framework_async_macros::async_test]
async fn create_editor_builds_a_definition_for_the_editor_role() {
    let def = create_dwg_ac1018_editor();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Editor);
    assert_eq!(def.dialect, DWG_AC1018_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DwgAc1018Editor as ArtifactEditor>::DIALECT, DWG_AC1018_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<DwgAc1018Editor, crate::viewer::dwg_ac1018::DwgAc1018Viewer>().await;
}
