use semio_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[test]
fn both_artifacts_declare_the_authored_editor_and_viewer_roster() {
    use semio_framework_value::serde_json;

    let descriptor: serde_json::Value = serde_json::from_str(include_str!("../../🧩️component/🔣️.json")).unwrap();
    let expected = descriptor["manifest"]["apps"].as_array().unwrap().iter()
        .map(|app| (app["id"].as_str().unwrap(), app["role"].as_str().unwrap()))
        .collect::<Vec<_>>();
    let declarations = [
        semio_s_artifact_fem_2d::artifact::<super::FemApps>(),
        semio_s_artifact_fem_3d::artifact::<super::FemApps>(),
    ];
    let actual = declarations.iter().flat_map(|artifact| &artifact.standards)
        .flat_map(|standard| &standard.subsets)
        .flat_map(|subset| [&subset.editor.definition, &subset.viewer.definition])
        .map(|app| (app.id.as_str(), app.role.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

#[semio_framework_async_macros::async_test]
async fn fem2d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_fem_2d_viewer::viewer::fem2d::Fem2dViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn fem2d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp, semio_s_artifact_fem_2d_viewer::viewer::fem2d::Fem2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn fem3d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_fem_3d_viewer::viewer::fem3d::Fem3dViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn fem3d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp, semio_s_artifact_fem_3d_viewer::viewer::fem3d::Fem3dViewer>().await;
}
