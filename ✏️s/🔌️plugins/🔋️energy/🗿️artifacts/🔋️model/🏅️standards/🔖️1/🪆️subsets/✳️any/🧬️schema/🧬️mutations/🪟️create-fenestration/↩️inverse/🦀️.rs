//! ↩️ Inverse for `CreateFenestration` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::CreateFenestration, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    if base.model.fenestrations.iter().any(|item| item.id == payload.id)
        || payload.name.trim().is_empty()
        || !base.model.surfaces.iter().any(|item| item.id == payload.surface_id)
        || payload.glazing_construction_id.is_some_and(|glazing| !base.model.constructions.iter().any(|item| item.id == glazing))
        || !payload.u_value_w_m2k.is_finite()
        || payload.u_value_w_m2k <= 0.0
        || !payload.shgc.is_finite()
        || !(0.0..=1.0).contains(&payload.shgc)
        || !payload.vlt.is_finite()
        || !(0.0..=1.0).contains(&payload.vlt)
        || !payload.area_m2.is_finite()
        || payload.area_m2 <= 0.0
        || !payload.height_m.is_finite()
        || payload.height_m <= 0.0
        || !payload.sill_height_m.is_finite()
        || payload.sill_height_m < 0.0
        || false
    {
        return Vec::new();
    }
    vec![vocabulary::delete_fenestration(payload.id)]
}
//#endregion 🔖️Inverse
