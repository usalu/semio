//! 🧪️ Contract §2.5 surface laws, now the real framework functions (`📓️w0-f-report.md` Gap 2) —
//! no local stand-ins.
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn raster_viewer_never_mutates() {
    assert_viewer_never_mutates::<semio_s_artifact_raster_raster_viewer::RasterViewer>(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
}

#[semio_framework_async_macros::async_test]
async fn raster_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<semio_s_artifact_raster_raster_editor::RasterPlayApp, semio_s_artifact_raster_raster_viewer::RasterViewer>().await;
}
