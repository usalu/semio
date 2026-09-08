
//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — the real
//! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect,
//! new_viewer}` (closed by w0-f, gap 2), used directly rather than local stand-ins.
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn home_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::home::HomeViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn home_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::home::HomeApp, crate::viewer::home::HomeViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn space_index_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::space_index::SpaceIndexViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn space_index_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::space_index::SpaceIndexEditor, crate::viewer::space_index::SpaceIndexViewer>().await;
}
