use super::*;

#[semio_framework_async_macros::async_test]
async fn empty_diff_is_a_no_operation() {
    let base = crate::schema::empty_energy_model_snapshot();
    let diff = EnergyModelDiff::default();
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
}

#[semio_framework_async_macros::async_test]
async fn preview_results_do_not_enter_snapshot() {
    let base = crate::schema::empty_energy_model_snapshot();
    let diff = diff_set_results_json("{\"ok\":true}");
    assert_eq!(diff.apply(&base).expect("valid mutation diff"), base);
    let artifact = EnergyModelArtifact::from_snapshot(base);
    let next = diff.apply_to_artifact(&artifact).expect("valid artifact diff");
    assert_eq!(next.results_json, "{\"ok\":true}");
}

#[semio_framework_async_macros::async_test]
async fn diff_from_model_regenerates_structure_and_zones_together() {
    let base = crate::schema::empty_energy_model_snapshot();
    let model = crate::model::Model { name: "Demo".into(), ..crate::model::Model::default() };
    let diff = diff_from_model(model);
    let applied = diff.apply(&base).expect("valid mutation diff");
    assert_eq!(applied.model.name, "Demo");
    assert_eq!(applied.structure.child_id, applied.zones.child_id, "structure/zones must share one scene id");
}
