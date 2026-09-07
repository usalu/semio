//! ↩️ Inverse for `DeleteSurface` — always computed from BASE, never by inverting the delta.

use crate::artifacts::model::mutations as vocabulary;
use crate::artifacts::model::mutations::EnergyModelMutation;
use crate::artifacts::model::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::DeleteSurface, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.surfaces.iter().any(|item| item.outside_boundary_condition.interzone_partner() == Some(payload.id)) {
        return Vec::new();
    }
    let mut steps = vec![vocabulary::create_surface(existing.id, existing.name.clone(), existing.zone_id, existing.class, existing.vertices_m.clone(), existing.construction_id, existing.outside_boundary_condition.kind(), existing.outside_boundary_condition.interzone_partner(), existing.sun_exposed, existing.wind_exposed, existing.multiplier)];
    for window in base.model.fenestrations.iter().filter(|item| item.surface_id == payload.id) {
        steps.push(vocabulary::create_fenestration(window.id, window.name.clone(), window.surface_id, window.u_value_w_m2k, window.shgc, window.vlt, window.area_m2, window.height_m, window.sill_height_m, window.frame_conductance_w_k, window.divider_conductance_w_k, window.overhang_depth_m, window.overhang_offset_m, window.fin_depth_m, window.fin_offset_m, window.glazing_construction_id));
    }
    for pair in base.model.adjacency_pairs.iter().filter(|item| item.surface_a_id == payload.id || item.surface_b_id == payload.id) {
        steps.push(vocabulary::connect_surfaces(pair.surface_a_id, pair.surface_b_id));
    }
    steps
}
//#endregion 🔖️Inverse
