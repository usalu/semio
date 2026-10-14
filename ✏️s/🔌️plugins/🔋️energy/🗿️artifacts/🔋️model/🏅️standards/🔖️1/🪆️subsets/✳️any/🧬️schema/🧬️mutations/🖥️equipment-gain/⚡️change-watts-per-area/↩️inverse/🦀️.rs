//! ↩️ Inverse for `ChangeEquipmentGainWattsPerArea` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeEquipmentGainWattsPerArea, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeEquipmentGainWattsPerArea>,
{
    Ok((|| {
    match base.model.equipment.iter().find(|item| item.id == payload.id) {
        Some(item) if item.watts_per_area != payload.new_watts_per_area && !(!payload.new_watts_per_area.is_finite() || payload.new_watts_per_area < 0.0) => vec![vocabulary::change_equipment_gain_watts_per_area(payload.id, item.watts_per_area)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
