//! ↩️ Inverse for `CreateZone` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateZone, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.zones.iter().any(|item| item.id == payload.id)
        || payload.name.trim().is_empty()
        || base.model.zones.iter().any(|item| item.name == payload.name)
        || !payload.volume_m3.is_finite()
        || payload.volume_m3 <= 0.0
        || payload.multiplier == 0
        || false
    {
        return Vec::new();
    }
    vec![vocabulary::delete_zone(payload.id)]
}
//#endregion 🔖️Inverse
