//! 🧪️ Contract §2.5's two cross-surface guarantees (`assert_viewer_never_mutates`,
//! `assert_editor_and_viewer_share_dialect`), landed for real in `semio_framework_plugin::artifact_app_laws`
//! per `📓️w0-f-report.md` gap 2 — used directly, no local stand-ins.
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn remodeling_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::remodeling::RemodelingViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn remodeling_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::remodeling::RemodelingPlayApp, crate::viewer::remodeling::RemodelingViewer>().await;
}
