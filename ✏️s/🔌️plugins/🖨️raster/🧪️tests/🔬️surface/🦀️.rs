//! 🧪️ Contract §2.5 surface laws, now the real framework functions (`📓️w0-f-report.md` Gap 2) —
//! no local stand-ins.
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn raster_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::raster::RasterViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn raster_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::raster::RasterPlayApp, crate::viewer::raster::RasterViewer>().await;
}
