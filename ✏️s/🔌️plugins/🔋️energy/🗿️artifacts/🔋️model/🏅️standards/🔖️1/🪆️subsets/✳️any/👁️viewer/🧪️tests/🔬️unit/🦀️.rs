
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
async fn viewer_declares_all_three_windows() {
    let def = create_energy_model_viewer();
    for id in [structure::WINDOW_KIND_ID, zones::WINDOW_KIND_ID, simulation::WINDOW_KIND_ID] {
        assert!(def.window_kinds.iter().any(|window| window.id == id), "missing window kind {id}");
    }
}

/// 👁️ A viewer declares no dispatchable verb at all — its two kit windows use the READ-ONLY
/// `window_kind()` variants (no `set-node`/`set-cell`) and its simulation window declares none.
#[semio_framework_async_macros::async_test]
async fn viewer_declares_no_dispatchable_action() {
    let def = create_energy_model_viewer();
    assert!(def.window_kinds.iter().all(|window| window.actions.is_empty()), "a viewer window declared an action");
}
