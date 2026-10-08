//! 🔺️ Sparse diff builder for `ChangeGroundDeep` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::{EnergyModelDiff, GroundTemperatureConfigPatch, ModelPatch};
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeGroundDeep, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !(payload.new_temperature_c.is_finite() && payload.new_temperature_c >= -273.15) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A deep ground temperature of {} °C is not admissible.", payload.new_temperature_c), Vec::<String>::new());
    }
    if base.model.ground_temperature.deep_c == payload.new_temperature_c {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("The deep ground temperature is already {} °C.", payload.new_temperature_c));
    }
    protocol::MutationOutcome::new(EnergyModelDiff::of(ModelPatch { ground_temperature: GroundTemperatureConfigPatch { deep_c: Some(payload.new_temperature_c), ..Default::default() }, ..Default::default() }))
}
//#endregion 🔖️Diff
