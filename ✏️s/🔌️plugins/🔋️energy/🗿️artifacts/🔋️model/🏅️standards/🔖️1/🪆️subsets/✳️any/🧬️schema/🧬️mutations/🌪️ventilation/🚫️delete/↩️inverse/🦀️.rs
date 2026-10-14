//! ↩️ Inverse for `DeleteMechanicalVentilation` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteMechanicalVentilation, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteMechanicalVentilation> + From<crate::CreateMechanicalVentilation>,
{
    Ok((|| {
    let Some(index) = base.model.mechanical_ventilations.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.mechanical_ventilations[index];
    vec![vocabulary::create_mechanical_ventilation(index as u32, existing.id, existing.zone_id, existing.schedule_id, existing.design_flow_m3_s, existing.fan_total_efficiency, existing.fan_delta_pressure_pa)]

    })())
}
//#endregion 🔖️Inverse
