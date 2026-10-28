//! ↩️ Inverse for `DeletePlantLoop` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeletePlantLoop, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeletePlantLoop> + From<crate::CreatePlantLoop>,
{
    Ok((|| {
    match base.model.plant_loops.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((index, item)) => vec![vocabulary::create_plant_loop(item.id, item.name.clone(), item.loop_type, item.supply_temperature_c, item.return_temperature_c, item.design_flow_kg_s, item.equipment_ids.clone(), Some(index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
