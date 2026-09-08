//! ↩️ Inverse for `DeleteSpace` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteSpace, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.spaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.space_lists.iter().any(|item| item.space_ids.contains(&payload.id)) || false {
        return Vec::new();
    }
    vec![vocabulary::create_space(existing.id, existing.name.clone(), existing.zone_id, existing.floor_area_m2)]
}
//#endregion 🔖️Inverse
