//! ↩️ Inverse for `ChangeFenestrationArea` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeFenestrationArea, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !payload.new_area_m2.is_finite() || payload.new_area_m2 <= 0.0 || existing.area_m2 == payload.new_area_m2 {
        return Vec::new();
    }
    vec![vocabulary::change_fenestration_area(payload.id, existing.area_m2)]
}
//#endregion 🔖️Inverse
