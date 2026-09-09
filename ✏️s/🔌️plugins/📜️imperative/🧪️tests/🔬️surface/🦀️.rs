use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn imperative_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::procedure::ImperativeViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn imperative_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::procedure::ImperativePlayApp, crate::viewer::procedure::ImperativeViewer>().await;
}
