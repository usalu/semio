
use semio_framework_plugin::ViewerApp;
use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates, new_viewer};

/// 🧪️ Contract §2.5 — real teeth: dispatches `EnergyModelViewCommand::default()` through the full
/// `VcsArtifactApp<ViewerApp<EnergyModelViewer>>` runtime path and asserts the document/draft
/// stores are byte-for-byte unchanged before/after (`semio_framework_plugin::testkit`, landed by
/// W0-F — see `📓️w0-f-report.md` Gap 2; the pilot's own local stand-in is no longer needed here).
#[semio_framework_async_macros::async_test]
async fn energy_model_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::model::EnergyModelViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn energy_model_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::model::EnergyModelEditor, crate::viewer::model::EnergyModelViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn new_viewer_builds_a_runnable_energy_model_viewer_app() {
    let _app: semio_framework_plugin::app::VcsArtifactApp<ViewerApp<crate::viewer::model::EnergyModelViewer>> = new_viewer::<crate::viewer::model::EnergyModelViewer>().await;
}
