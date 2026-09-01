//! 🗃 `entries` — one named inference: a census over the working-scene `Model` behind a snapshot's
//! composed `structure`/`zones` children (real top-level `Model`-field count, real JSON byte size,
//! real content digest). Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM: the old opaque
//! `model_json` field is gone — since a composed child slot can only ever hold a real, typed
//! `Model` (never arbitrary/malformed text), this leaf now census over
//! `crate::artifacts::model::energy_model(snapshot)`'s own first-party `pack::json` serialization
//! (`Model` derives `ToValue`), which is ALWAYS a full JSON object (`Model` derives `Default`,
//! every field always present) rather than treating the body as an opaque, possibly-malformed byte
//! string.

use crate::artifacts::model::EnergyModelSnapshot;
use serde::{Deserialize, Serialize};
// 🌱️ Additive `ToValue`/`FromValue` — see `🦀️component.rs`'s own docstring note on this crate's
// interim (not-yet-serde-free) state.
use semio_framework_os_kernel::{DslValue, ToValue};
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

//#region 🔖️Entries
/// 🗃️ Census of the working-scene `Model` behind a snapshot's composed children.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct EnergyModelEntries {
    pub entry_count: u32,
    pub byte_size: u32,
    pub content_digest: String,
}

/// 🗃️ `entryCount` = number of top-level `Model` fields (its own `DslValue::Object` always has
/// one key per field — `Model` derives `Default`, never a partial object); `byteSize` = real UTF-8
/// byte length of that JSON; `contentDigest` = a deterministic (within-process) fingerprint over
/// those same bytes. Std-only (`DefaultHasher`), same reasoning as `🏠️home/🆔digest`: no external
/// hash crate needed for a single scalar byte-string digest.
pub fn compute_energy_model_entries(snapshot: &EnergyModelSnapshot) -> EnergyModelEntries {
    let model = crate::artifacts::model::energy_model(snapshot);
    let json = pack::json::to_json_string(&model);
    let bytes = json.as_bytes();
    let entry_count = match model.to_value() {
        DslValue::Object(entries) => entries.len() as u32,
        _ => 0,
    };
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    EnergyModelEntries { entry_count, byte_size: bytes.len() as u32, content_digest: format!("{:016x}", hasher.finish()) }
}
//#endregion 🔖️Entries

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;

    /// 🌱 `crate::model::Model` has exactly 40 top-level fields (name/version/site/zones/spaces/
    /// surfaces/fenestrations/materials/constructions/people/lighting/equipment/thermostats/
    /// humidistats/setpointManagers/idealLoads/zoneEquipment/airLoops/plantLoops/
    /// outdoorAirSystems/infiltrations/mechanicalVentilations/shadingSurfaces/spaceLists/
    /// thermalEnclosures/adjacencyPairs/airflowNetwork/electricalLoadCenters/pvSystems/
    /// batteryStorage/shwSystems/solarThermalSystems/refrigerationSystems/waterSystems/faults/
    /// outputVariables/sizingObjects/daylightZones/roomAirModels/groundTemperature) — counted
    /// directly against `🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️component.rs`'s `struct Model`.
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
        let snapshot = crate::artifacts::model::energy_snapshot_with_state("energy.model", &crate::model::Model { name: "demo".into(), ..crate::model::Model::default() }, None);
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
        let a = crate::artifacts::model::energy_snapshot_with_state("energy.model", &crate::model::Model::default(), None);
        let b = crate::artifacts::model::energy_snapshot_with_state("energy.model", &crate::model::Model { name: "x".into(), ..crate::model::Model::default() }, None);
        assert_ne!(compute_energy_model_entries(&a).content_digest, compute_energy_model_entries(&b).content_digest);
    }
}
//#endregion 🧪️Tests
