//! 🧪️ The editor/viewer pair's own cross-surface guarantees (contract §2.5), using the landed
//! framework test context directly: `semio_framework_plugin::artifact_app_laws::{assert_viewer_never_mutates,
//! assert_editor_and_viewer_share_dialect, new_viewer}`.
use crate::editor::equation::EquationPlayApp;
use crate::viewer::equation::EquationViewer;

#[semio_framework_async_macros::async_test]
async fn equation_viewer_never_mutates() {
    semio_framework_plugin::artifact_app_laws::assert_viewer_never_mutates::<EquationViewer>().await;
}

#[semio_framework_async_macros::async_test]
async fn equation_editor_and_viewer_share_dialect() {
    semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect::<EquationPlayApp, EquationViewer>().await;
}

/// 🧩️ Over the real `SemioMembers` roster, not `new_viewer`'s `NoMembers`: the equation viewer's
/// `genesis_child_pack` mints the equation's `s.stdio.semio@v1/text` child at construction, and a
/// roster that cannot name that dialect refuses the app at `seed_genesis_children` (`derived child
/// dialect 's.stdio.semio@v1/text' is not declared by this app's member roster`) — the SAME roster
/// `MathematicalApps::EquationViewer` carries in production. Mirrors `🔋️energy`'s own surface law.
///
/// 🧹️ A registered fixture app owns an artifact store; it must reach its exact terminal-empty
/// shallow-shell witness before Drop (framework store law), so close it through the production
/// close state machine instead of letting the harness drop it mid-flight.
#[semio_framework_async_macros::async_test]
async fn equation_viewer_instantiates_through_new_viewer() {
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&crate::viewer::equation::create_equation_viewer());
    let mut app: semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::ViewerApp<EquationViewer>, semio_s_artifact_stdio_semio::SemioMembers> =
        semio_framework_plugin::app::VcsArtifactApp::with_registry(semio_framework_plugin::ViewerApp::<EquationViewer>::default(), registry).await;
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
