//! ↩️ Inverse for `UpdateGroundTemperature` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::UpdateGroundTemperature, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if payload.building_surface_c.len() != 12 || payload.shallow_c.len() != 12 {
        return Vec::new();
    }
    let old = base.model.ground_temperature.clone();
    if old.deep_c == payload.deep_c && old.building_surface_c.as_slice() == payload.building_surface_c.as_slice() && old.shallow_c.as_slice() == payload.shallow_c.as_slice() {
        return Vec::new();
    }
    vec![vocabulary::update_ground_temperature(old.building_surface_c.to_vec(), old.shallow_c.to_vec(), old.deep_c)]
}
//#endregion 🔖️Inverse
