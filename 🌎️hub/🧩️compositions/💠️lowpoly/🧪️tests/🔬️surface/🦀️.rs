/// 🧪️ Contract §2.5 (closed by W0-F, `📓️w0-f-report.md` Gap 2): the real framework test context
/// functions now exist — this packet uses them directly rather than the pilot's local stand-ins.
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn lowpoly_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::lowpoly::LowpolyViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn lowpoly_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::lowpoly::LowpolyPlayApp, crate::viewer::lowpoly::LowpolyViewer>().await;
}
