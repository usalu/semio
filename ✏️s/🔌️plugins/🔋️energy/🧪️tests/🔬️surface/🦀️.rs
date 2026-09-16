use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};
use semio_framework_plugin::ViewerApp;

/// 🧪️ Contract §2.5 — real teeth: dispatches `EnergyModelViewCommand::default()` through the full
/// `VcsArtifactApp<ViewerApp<EnergyModelViewer>>` runtime path and asserts the document/draft
/// stores are byte-for-byte unchanged before/after (`semio_framework_plugin::artifact_app_laws`, landed by
/// W0-F — see `📓️w0-f-report.md` Gap 2; the pilot's own local stand-in is no longer needed here).
#[semio_framework_async_macros::async_test]
async fn energy_model_viewer_never_mutates() {
    assert_viewer_never_mutates::<crate::viewer::model::EnergyModelViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn energy_model_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::model::EnergyModelEditor, crate::viewer::model::EnergyModelViewer>().await;
}

/// 🧩️ Over the real `SemioMembers` roster, not `new_viewer`'s `NoMembers`: the viewer's
/// `genesis_child_pack` mints the model's two `s.stdio.semio` children at construction, and a
/// roster that cannot name their dialects refuses the app at `seed_genesis_children`.
#[semio_framework_async_macros::async_test]
async fn new_viewer_builds_a_runnable_energy_model_viewer_app() {
    let mut app: semio_framework_plugin::app::VcsArtifactApp<ViewerApp<crate::viewer::model::EnergyModelViewer>, semio_s_artifact_stdio_semio::SemioMembers> =
        semio_framework_plugin::app::VcsArtifactApp::new(ViewerApp::<crate::viewer::model::EnergyModelViewer>::default()).await;
    // 🧹️ The two genesis members opened onto the store must be retired before Drop.
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
