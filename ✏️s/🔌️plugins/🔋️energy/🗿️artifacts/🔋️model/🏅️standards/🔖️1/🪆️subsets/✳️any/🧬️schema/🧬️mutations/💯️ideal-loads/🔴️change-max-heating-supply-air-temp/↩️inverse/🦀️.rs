//! ↩️ Inverse for `ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp>,
{
    Ok((|| {
    match base.model.ideal_loads.iter().find(|item| item.id == payload.id) {
        Some(item) if item.max_heating_supply_air_temp_c != payload.new_max_heating_supply_air_temp_c && !(!payload.new_max_heating_supply_air_temp_c.is_finite() || !(-100.0..=200.0).contains(&payload.new_max_heating_supply_air_temp_c)) => {
            vec![vocabulary::change_ideal_loads_system_max_heating_supply_air_temp(payload.id, item.max_heating_supply_air_temp_c)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
