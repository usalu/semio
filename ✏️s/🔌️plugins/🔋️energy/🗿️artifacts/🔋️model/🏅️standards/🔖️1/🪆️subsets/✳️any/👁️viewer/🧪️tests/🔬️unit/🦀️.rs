use super::*;

#[semio_framework_async_macros::async_test]
async fn create_energy_model_viewer_builds_a_definition_for_the_viewer_role() {
    let def = create_energy_model_viewer();
    assert_eq!(def.role, semio_framework_plugin::AppRole::Viewer);
    assert_eq!(def.dialect, MODEL_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn viewer_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<EnergyModelViewer as ArtifactViewer>::DIALECT, MODEL_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn viewer_declares_every_window() {
    let def = create_energy_model_viewer();
    for id in [structure::WINDOW_KIND_ID, zones::WINDOW_KIND_ID, simulation::WINDOW_KIND_ID, model_window::WINDOW_KIND_ID] {
        assert!(def.window_kinds.iter().any(|window| window.id == id), "missing window kind {id}");
    }
    assert!(def.interactions.is_empty(), "a read-only viewer declares no interaction domain");
}

/// 👁️ A viewer declares no document-mutating verb — its two kit windows use the READ-ONLY
/// `window_kind()` variants (no `set-node`/`set-cell`) and its simulation window declares none.
/// The builder injects the framework's own history/clipboard/tool actions into EVERY window
/// (`try_build_definition`), so the law is "no `ActionKind::Mutation`", not "no actions".
#[semio_framework_async_macros::async_test]
async fn viewer_declares_no_dispatchable_action() {
    let def = create_energy_model_viewer();
    for window in &def.window_kinds {
        for action in &window.actions {
            assert!(!matches!(action.kind, semio_framework_plugin::ActionKind::Mutation), "viewer window {} declares the mutating action {}", window.id, action.id);
        }
    }
    assert!(!def.window_kinds.iter().any(|window| window.actions.iter().any(|action| action.id == "set-node" || action.id == "set-cell")), "a viewer window carries a kit edit action");
}
