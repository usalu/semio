//! ↩️ Inverse for `ChangeShadingSurfaceTransmittanceSchedule` — always computed from BASE, never by inverting the delta.

use crate::mutations as vocabulary;
use crate::mutations::EnergyModelMutation;
use crate::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse(payload: &super::ChangeShadingSurfaceTransmittanceSchedule, base: &EnergyModelSnapshot) -> Vec<EnergyModelMutation> {
    let Some(existing) = base.model.shading_surfaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if payload.new_transmittance_schedule_id.is_some_and(|schedule| !base.model.schedules.contains(schedule)) || existing.transmittance_schedule_id == payload.new_transmittance_schedule_id {
        return Vec::new();
    }
    vec![vocabulary::change_shading_surface_transmittance_schedule(payload.id, existing.transmittance_schedule_id)]
}
//#endregion 🔖️Inverse
