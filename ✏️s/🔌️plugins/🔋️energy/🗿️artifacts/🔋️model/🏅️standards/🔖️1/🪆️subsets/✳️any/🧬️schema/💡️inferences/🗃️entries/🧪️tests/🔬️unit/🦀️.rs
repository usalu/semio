
use super::*;

/// 🌱 `crate::model::Model` has exactly 40 top-level fields (name/version/site/zones/spaces/
/// surfaces/fenestrations/materials/constructions/people/lighting/equipment/thermostats/
/// humidistats/setpointManagers/idealLoads/zoneEquipment/airLoops/plantLoops/
/// outdoorAirSystems/infiltrations/mechanicalVentilations/shadingSurfaces/spaceLists/
/// thermalEnclosures/adjacencyPairs/airflowNetwork/electricalLoadCenters/pvSystems/
/// batteryStorage/shwSystems/solarThermalSystems/refrigerationSystems/waterSystems/faults/
/// outputVariables/sizingObjects/daylightZones/roomAirModels/groundTemperature) — counted
/// directly against `🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs`'s `struct Model`.
const MODEL_FIELD_COUNT: u32 = 40;

#[semio_framework_async_macros::async_test]
async fn default_model_yields_the_full_field_count_and_a_real_byte_size() {
    let entries = compute_energy_model_entries(&EnergyModelSnapshot::default());
    assert_eq!(entries.entry_count, MODEL_FIELD_COUNT);
    let expected_bytes = pack::json::to_json_string(&crate::model::Model::default()).len() as u32;
    assert_eq!(entries.byte_size, expected_bytes);
}

#[semio_framework_async_macros::async_test]
async fn top_level_field_count_is_stable_regardless_of_content() {
    let snapshot = crate::energy_snapshot_with_state("energy.model", &crate::model::Model { name: "demo".into(), ..crate::model::Model::default() }, None);
    let entries = compute_energy_model_entries(&snapshot);
    assert_eq!(entries.entry_count, MODEL_FIELD_COUNT);
}

/// 🌱 A cache-miss (fresh working scene, e.g. a snapshot decoded via `parse_dsl`/`decode_pack`
/// in a new process) still yields a deterministic census — `energy_model` fails soft to
/// `Model::default()`, never a panic; this is the honest staleness-gap consequence
/// `🔖️WorkingScene`'s own doc comment documents, exercised for real here.
#[semio_framework_async_macros::async_test]
async fn cache_miss_still_yields_a_deterministic_census() {
    let snapshot = EnergyModelSnapshot::default();
    let entries = compute_energy_model_entries(&snapshot);
    assert_eq!(entries, compute_energy_model_entries(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn different_bodies_yield_different_digests() {
    let a = crate::energy_snapshot_with_state("energy.model", &crate::model::Model::default(), None);
    let b = crate::energy_snapshot_with_state("energy.model", &crate::model::Model { name: "x".into(), ..crate::model::Model::default() }, None);
    assert_ne!(compute_energy_model_entries(&a).content_digest, compute_energy_model_entries(&b).content_digest);
}
