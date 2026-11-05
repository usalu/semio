#[semio_framework_async_macros::async_test]
async fn sequence_viewer_never_mutates() {
    semio_plugin::artifact_app_laws::assert_viewer_never_mutates::<crate::viewer::sequence::SequenceViewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn sequence_editor_and_viewer_share_dialect() {
    semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<crate::editor::sequence::app::SequencePlayApp, crate::viewer::sequence::SequenceViewer>().await;
}
