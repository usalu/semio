
#[semio_framework_async_macros::async_test]
async fn draw_viewer_never_mutates() {
    semio_framework_plugin::testkit::assert_viewer_never_mutates::<crate::viewer::drawing::DrawingViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn draw_editor_and_viewer_share_dialect() {
    semio_framework_plugin::testkit::assert_editor_and_viewer_share_dialect::<crate::editor::drawing::DrawingPlayApp, crate::viewer::drawing::DrawingViewer>().await;
}
