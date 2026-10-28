//! ↩️ Inverse for `DeleteFenestration` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteFenestration, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteFenestration> + From<crate::CreateFenestration> + From<crate::ReplaceFenestrationVertices>,
{
    Ok((|| {
    let Some((index, existing)) = base.model.fenestrations.iter().enumerate().find(|(_, item)| item.id == payload.id) else {
        return Vec::new();
    };
    let mut steps = Vec::new();
    if !existing.vertices_m.is_empty() {
        steps.push(vocabulary::replace_fenestration_vertices(existing.id, existing.vertices_m.clone()));
    }
    steps.push(vocabulary::create_fenestration(
        existing.id,
        existing.name.clone(),
        existing.surface_id,
        existing.u_value_w_m2k,
        existing.shgc,
        existing.vlt,
        existing.area_m2,
        existing.height_m,
        existing.sill_height_m,
        existing.frame_conductance_w_k,
        existing.divider_conductance_w_k,
        existing.overhang_depth_m,
        existing.overhang_offset_m,
        existing.fin_depth_m,
        existing.fin_offset_m,
        existing.glazing_construction_id,
            Some(index as u32),
        ));
    steps

    })())
}
//#endregion 🔖️Inverse
