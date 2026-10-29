//! ↩️ Inverse for `DeleteSurface` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteSurface, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteSurface> + From<crate::boundary_mutations::ConnectSurfaces> + From<crate::fenestration_mutations::CreateFenestration> + From<crate::CreateSurface> + From<crate::fenestration_geometry_mutations::ReplaceFenestrationVertices>,
{
    Ok((|| {
    let Some((index, existing)) = base.model.surfaces.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    if base.model.surfaces.iter().any(|item| item.outside_boundary_condition.interzone_partner() == Some(payload.id)) {
        return Vec::new();
    }
    let mut steps = vec![vocabulary::create_surface(
        existing.id,
        existing.name.clone(),
        existing.zone_id,
        existing.class,
        existing.vertices_m.clone(),
        existing.construction_id,
        existing.outside_boundary_condition.kind(),
        existing.outside_boundary_condition.interzone_partner(),
        existing.sun_exposed,
        existing.wind_exposed,
        existing.multiplier,
        Some(index as u32),
    )];
    for (window_index, window) in base.model.fenestrations.iter().enumerate().filter(|(_, item)| item.surface_id == payload.id) {
        steps.push(crate::fenestration_mutations::create_fenestration(
            window.id,
            window.name.clone(),
            window.surface_id,
            window.u_value_w_m2k,
            window.shgc,
            window.vlt,
            window.area_m2,
            window.height_m,
            window.sill_height_m,
            window.frame_conductance_w_k,
            window.divider_conductance_w_k,
            window.overhang_depth_m,
            window.overhang_offset_m,
            window.fin_depth_m,
            window.fin_offset_m,
            window.glazing_construction_id,
            Some(window_index as u32),
        ));
        if !window.vertices_m.is_empty() {
            steps.push(crate::fenestration_geometry_mutations::replace_fenestration_vertices(window.id, window.vertices_m.clone()));
        }
    }
    for (pair_index, pair) in base.model.adjacency_pairs.iter().enumerate().filter(|(_, item)| item.surface_a_id == payload.id || item.surface_b_id == payload.id) {
        steps.push(crate::boundary_mutations::connect_surfaces(pair.surface_a_id, pair.surface_b_id, Some(pair_index as u32)));
    }
    steps.reverse();
    steps

    })())
}
//#endregion 🔖️Inverse
