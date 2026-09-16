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
    // 🔶️ The store replays an inverse in REVERSE order (`ArtifactStore::replay_mutations`), so a
    // window that carried its own polygon lists the polygon step FIRST and the re-creation LAST —
    // reversed, the aperture exists again before its shape is put back. `create-fenestration` does
    // not carry `vertices_m` itself, which keeps its payload (and every committed vector over it)
    // exactly as landed.
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
    ));
    steps
}
//#endregion 🔖️Inverse
