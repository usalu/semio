use super::*;

#[semio_framework_async_macros::async_test]
async fn create_drawing_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_drawing_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, DRAWING_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<DrawingViewer as ArtifactViewer>::DIALECT, DRAWING_DIALECT);
}

/// ⚖️ LAW: the viewer opens, renders and closes its document through the artifact's OWN owner catalogue
/// — the framework's generic bounded owners drop an owned snapshot root plainly, which is the guest
/// panic S15 measured on the generation2d viewer (session 11); this viewer had the same gap.
#[semio_framework_async_macros::async_test]
async fn the_viewer_opens_and_closes_its_document_through_the_artifacts_owners() {
    let mut app = semio_framework_plugin::artifact_app_laws::new_viewer::<DrawingViewer>().await;
    let tree = semio_framework_plugin::PluginApp::render(&mut app, canvas::BODY_KEY, None, &semio_framework_plugin::ViewModel::default()).await.expect("the viewer renders its default document");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("the canvas projects");
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
