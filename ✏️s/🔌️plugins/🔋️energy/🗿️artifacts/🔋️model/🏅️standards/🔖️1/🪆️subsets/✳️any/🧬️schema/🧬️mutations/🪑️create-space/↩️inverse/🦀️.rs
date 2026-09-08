//! ↩️ Inverse for `CreateSpace` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateSpace, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.spaces.iter().any(|item| item.id == payload.id) || payload.name.trim().is_empty() || !base.model.zones.iter().any(|item| item.id == payload.zone_id) || !payload.floor_area_m2.is_finite() || payload.floor_area_m2 < 0.0 || false {
        return Vec::new();
    }
    vec![vocabulary::delete_space(payload.id)]
}
//#endregion 🔖️Inverse
