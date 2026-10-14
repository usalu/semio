use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn cad_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_cad_cad_viewer::CadViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn cad_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_cad_cad_editor::CadPlayApp, semio_s_artifact_cad_cad_viewer::CadViewer>().await;
}
