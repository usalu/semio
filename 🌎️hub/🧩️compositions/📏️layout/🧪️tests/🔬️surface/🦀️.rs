//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 canonical helpers
//! (`semio_framework_plugin::artifact_app_laws::{assert_viewer_never_mutates,
//! assert_editor_and_viewer_share_dialect, new_viewer}`) — closed by lane 0-F (`📓️w0-f-report.md`
//! Gap 2), used directly here rather than local stand-ins.
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn layout_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_layout_layout_viewer::LayoutViewer>(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn layout_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_layout_layout_editor::LayoutPlayApp, semio_s_artifact_layout_layout_viewer::LayoutViewer>().await;
}
