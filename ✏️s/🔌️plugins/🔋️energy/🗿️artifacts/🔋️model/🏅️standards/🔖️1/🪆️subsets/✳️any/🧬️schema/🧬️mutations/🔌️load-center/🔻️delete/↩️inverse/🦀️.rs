//! ↩️ Inverse for `DeleteElectricalLoadCenter` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteElectricalLoadCenter, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteElectricalLoadCenter> + From<crate::CreateElectricalLoadCenter>,
{
    Ok((|| {
    let Some(index) = base.model.electrical_load_centers.iter().position(|item| item.id == payload.id) else {
        return Vec::new();
    };
    let existing = &base.model.electrical_load_centers[index];
    vec![vocabulary::create_electrical_load_center(index as u32, existing.id, existing.name.clone(), existing.generator_ids.clone(), existing.pv_ids.clone(), existing.battery_ids.clone())]

    })())
}
//#endregion 🔖️Inverse
