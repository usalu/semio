//! 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.5 — w0-f (SDK gap
//! closure lane) landed the canonical `testkit::{assert_viewer_never_mutates,
//! assert_editor_and_viewer_share_dialect, new_viewer}` in `semio_framework_plugin`; used directly
//! here rather than a local stand-in (the pilot cad packet's `📓️w2-cad-report.md` had to write one
//! before this landed — see that report's "SDK gaps found" §2 for the closed gap).
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

#[semio_framework_async_macros::async_test]
async fn playbook_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::playbook::PlaybookViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn playbook_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::playbook::PlaybookPlayApp, crate::viewer::playbook::PlaybookViewer>().await;
}
