use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn fem2d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_fem_2d_viewer::viewer::fem2d::Fem2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn fem2d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dPlayApp, semio_s_artifact_fem_2d_viewer::viewer::fem2d::Fem2dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn fem3d_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_fem_3d_viewer::viewer::fem3d::Fem3dViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn fem3d_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_fem_3d_editor::editor::fem3d::Fem3dPlayApp, semio_s_artifact_fem_3d_viewer::viewer::fem3d::Fem3dViewer>().await;
}
