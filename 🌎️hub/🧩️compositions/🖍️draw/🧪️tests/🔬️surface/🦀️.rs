#[semio_framework_async_macros::async_test]
async fn draw_viewer_never_mutates() {
    semio_plugin::artifact_app_laws::assert_viewer_never_mutates::<semio_s_artifact_draw_drawing_viewer::DrawingViewer>(super::DRAW_MOUNTED_OWNER_POLICY, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn draw_editor_and_viewer_share_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<semio_s_artifact_draw_drawing_editor::DrawingPlayApp, semio_s_artifact_draw_drawing_viewer::DrawingViewer>().await;
}
