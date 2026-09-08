//! ↩️ Inverse for `DeleteFenestration` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteFenestration, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.fenestrations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if false {
        return Vec::new();
    }
    vec![vocabulary::create_fenestration(existing.id, existing.name.clone(), existing.surface_id, existing.u_value_w_m2k, existing.shgc, existing.vlt, existing.area_m2, existing.height_m, existing.sill_height_m, existing.frame_conductance_w_k, existing.divider_conductance_w_k, existing.overhang_depth_m, existing.overhang_offset_m, existing.fin_depth_m, existing.fin_offset_m, existing.glazing_construction_id)]
}
//#endregion 🔖️Inverse
