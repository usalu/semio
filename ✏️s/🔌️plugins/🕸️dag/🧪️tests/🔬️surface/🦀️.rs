
//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5's
//! `semio_framework_plugin::testkit::{assert_viewer_never_mutates, assert_editor_and_viewer_share_dialect}`
//! (W0-F gap 2) are used directly here — no local stand-ins, unlike the pilot packet which
//! predated their landing.
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn dag_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::dag::DagViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn dag_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::dag::DagPlayApp, crate::viewer::dag::DagViewer>().await;
}
