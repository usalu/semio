//! 🔺️ Sparse diff builder for `ChangeGroundTemperatureDeep` — the artifact's delta is built straight from the
//! payload and BASE, never by applying and capturing.

use crate::diff::EnergyModelDiff;
use crate::EnergyModelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangeGroundTemperatureDeep, base: &EnergyModelSnapshot) -> protocol::MutationOutcome<EnergyModelDiff> {
    if !(payload.new_temperature_c.is_finite() && payload.new_temperature_c >= -273.15) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("A deep ground temperature of {} °C is not admissible.", payload.new_temperature_c), Vec::<String>::new());
    }
    if base.model.ground_temperature.deep_c == payload.new_temperature_c {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("The deep ground temperature is already {} °C.", payload.new_temperature_c));
    }
    let mut model = base.model.clone();
    model.ground_temperature.deep_c = payload.new_temperature_c;
    protocol::MutationOutcome::new(crate::schema::diff::text::diff_from_model(model))
}
//#endregion 🔖️Diff
