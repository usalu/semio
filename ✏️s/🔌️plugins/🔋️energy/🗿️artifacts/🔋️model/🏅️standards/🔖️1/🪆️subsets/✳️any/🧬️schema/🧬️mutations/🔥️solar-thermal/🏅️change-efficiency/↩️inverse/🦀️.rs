//! ↩️ Inverse for `ChangeSolarThermalSystemEfficiency` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSolarThermalSystemEfficiency, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSolarThermalSystemEfficiency>,
{
    Ok((|| {
    match base.model.solar_thermal_systems.iter().find(|item| item.id == payload.id) {
        Some(item) if item.efficiency != payload.new_efficiency && !(!(payload.new_efficiency > 0.0 && payload.new_efficiency <= 1.0)) => vec![vocabulary::change_solar_thermal_system_efficiency(payload.id, item.efficiency)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
