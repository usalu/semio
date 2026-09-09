use super::*;

#[semio_framework_async_macros::async_test]
async fn create_block3d_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_block3d_viewer();
    assert_eq!(def.role, semio_framework::AppRole::Viewer);
    assert_eq!(def.dialect, BLOCK3D_DIALECT.into());
    assert_eq!(def.breadcrumb, vec!["semio", "block", "3d"]);
    let descriptor = serde_json::to_value(&def).expect("language-neutral app descriptor");
    assert_eq!(descriptor["document"], serde_json::json!(["semio", "block", "3d"]));
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Block3dViewer as ArtifactViewer>::DIALECT, BLOCK3D_DIALECT);
}

/// ⚖️ LAW: the viewer boots non-empty — it can never load an example itself, so an empty initial
/// snapshot would leave its `World3d` window blank forever.
#[semio_framework_async_macros::async_test]
async fn viewer_boots_with_at_least_one_representation() {
    let app = semio_framework_plugin::testkit::new_app::<semio_framework_plugin::ViewerApp<Block3dViewer>>().await;
    let snapshot = app.snapshot().expect("snapshot");
    assert!(!snapshot.representations.is_empty(), "the viewer must boot with a renderable document");
    assert!(snapshot.representations.iter().all(|representation| representation.mesh_url.is_some()));
}

#[semio_framework_async_macros::async_test]
async fn noop_command_round_trips_and_never_mutates() {
    let mut app = semio_framework_plugin::testkit::new_app::<semio_framework_plugin::ViewerApp<Block3dViewer>>().await;
    let before = app.snapshot().expect("snapshot");
    app.dispatch_typed(Block3dViewCommand::Noop, &semio_framework_plugin::testkit::meta("local")).await.expect("dispatch");
    let after = app.snapshot().expect("snapshot");
    assert_eq!(before, after, "the viewer's sole command must never change the document");
}
