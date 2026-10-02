//! ↩️ Inverse for `ChangeGroundTemperatureDeep` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeGroundTemperatureDeep, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if !(payload.new_temperature_c.is_finite() && payload.new_temperature_c >= -273.15) || base.model.ground_temperature.deep_c == payload.new_temperature_c {
        return Vec::new();
    }
    vec![vocabulary::change_ground_temperature_deep(base.model.ground_temperature.deep_c)]
}
//#endregion 🔖️Inverse
