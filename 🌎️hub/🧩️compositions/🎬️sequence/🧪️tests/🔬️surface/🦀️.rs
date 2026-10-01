#[semio_framework_async_macros::async_test]
async fn sequence_viewer_never_mutates() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<crate::viewer::sequence::SequenceViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn sequence_editor_and_viewer_share_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<crate::editor::sequence::SequencePlayApp, crate::viewer::sequence::SequenceViewer>().await;
}
