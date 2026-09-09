use super::*;
use protocol::Inference;

fn populated_snapshot() -> EnergyModelSnapshot {
    crate::energy_snapshot_with_state("energy.model", &crate::model::Model { name: "demo".into(), zones: Vec::new(), ..crate::model::Model::default() }, None)
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = populated_snapshot();
    assert_eq!(EnergyModelInference::infer(&snapshot), EnergyModelInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(EnergyModelInference::infer(&EnergyModelSnapshot::default()), EnergyModelInference::default());
}

#[semio_framework_async_macros::async_test]
async fn entries_counts_top_level_model_fields_and_bytes() {
    let snapshot = populated_snapshot();
    let inferred = EnergyModelInference::infer(&snapshot);
    let expected_json = pack::json::to_json_string(&crate::energy_model(&snapshot));
    assert_eq!(inferred.entries.byte_size, expected_json.len() as u32);
    assert!(inferred.entries.entry_count > 0);
}
