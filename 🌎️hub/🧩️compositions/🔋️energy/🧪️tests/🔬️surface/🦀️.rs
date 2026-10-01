use semio_framework_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect;
use semio_framework_plugin::ViewerApp;

/// 🧪️ Contract §2.5 — the read-only guarantee for a viewer whose ONE verb travels the RETAINED route.
/// `artifact_app_laws::assert_viewer_never_mutates` drives `ViewerApp::handle`, the STATELESS adapter
/// seam. Since this viewer's `setCamera` became `InteractiveJobClassification::Migrated` its emission
/// is a window-config write, which `ViewEmit` structurally cannot carry, so `handle` refuses loudly
/// (`energy.model.3d.viewer.retained-route-required`) rather than dropping the orbit — and the generic
/// fixture's `expect("viewer adapter command succeeds")` can no longer be satisfied by construction.
/// The guarantee is therefore asserted over the seam that decides it: EVERY verb this viewer declares
/// is `Migrated`, so none of them can ever reach the stateless seam, and the emission itself is proved
/// window-config-only (no artifact, no config, no draft lane) by the artifact crate's
/// `a_camera_gesture_becomes_an_addressed_window_config_write_and_nothing_else` and
/// `every_viewer_publication_lane_is_a_window_config_lane`.
#[semio_framework_async_macros::async_test]
async fn energy_model_viewer_never_mutates() {
    let definition = crate::viewer::model::create_energy_model_viewer();
    let actions: Vec<_> = definition.window_kinds.iter().flat_map(|window| semio_framework::window_kind_actions(&definition, window)).collect();
    assert!(!actions.is_empty(), "the energy model viewer declares at least one verb");
    for action in &actions {
        assert_eq!(
            action.semantics.execution.interactive_job,
            semio_framework_plugin::InteractiveJobClassification::Migrated,
            "viewer action '{}' must travel the retained route — the stateless ViewEmit seam has no store lane a viewer may write",
            action.id
        );
    }
}

#[semio_framework_async_macros::async_test]
async fn energy_model_editor_and_viewer_share_dialect() {
    assert_editor_and_viewer_share_dialect::<crate::editor::model::EnergyModelEditor, crate::viewer::model::EnergyModelViewer>().await;
}

/// 🧩️ Over the real `SemioMembers` roster, not `new_viewer`'s `NoMembers`: the viewer's
/// `genesis_child_pack` mints the model's two `s.stdio.semio` children at construction, and a
/// roster that cannot name their dialects refuses the app at `seed_genesis_children`.
///
/// 🧾️ …and over the real `AppActionRegistry`, not the registry-less `VcsArtifactApp::new`:
/// `with_registry_on_bus` joins `ViewerApp<EnergyModelViewer>`'s `bounded_first_step_tool_proofs!`
/// roster against the registry's `Migrated` tool ids (`AppActionRegistry::validate_tool_job_rows`),
/// and an empty registry declares none of them — construction then panics with
/// `interactive-job.catalog-authority` … `generated_migrated=false`, `migrated={}` (tool `setCamera`).
#[semio_framework_async_macros::async_test]
async fn new_viewer_builds_a_runnable_energy_model_viewer_app() {
    let registry = semio_framework_plugin::AppActionRegistry::from_definition(&crate::viewer::model::create_energy_model_viewer());
    let mut app: semio_framework_plugin::app::VcsArtifactApp<ViewerApp<crate::viewer::model::EnergyModelViewer>, semio_s_artifact_stdio_semio::SemioMembers> =
        semio_framework_plugin::app::VcsArtifactApp::with_registry(ViewerApp::<crate::viewer::model::EnergyModelViewer>::default(), registry).await;
    // 🧹️ The two genesis members opened onto the store must be retired before Drop.
    semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app);
}
