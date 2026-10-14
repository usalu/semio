//! 🔺️ Sparse diff builder for `ChangeGroundBuilding` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, GroundTemperatureConfigPatch, ModelPatch, Slots};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeGroundBuilding, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !((1..=12).contains(&payload.month) && payload.new_temperature_c.is_finite() && payload.new_temperature_c >= -273.15) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A building-surface ground temperature of {} °C for month {} is not admissible.", payload.new_temperature_c, payload.month), Vec::<String>::new());
    }
    if base.model.ground_temperature.building_surface_c[usize::from(payload.month) - 1] == payload.new_temperature_c {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The building-surface ground temperature of month {} is already {} °C.", payload.month, payload.new_temperature_c));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { ground_temperature: GroundTemperatureConfigPatch { building_surface_c: Slots::assigning(usize::from(payload.month) - 1, payload.new_temperature_c), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
