//! ↩️ Inverse for `ChangePlantLoopReturnTemperature` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangePlantLoopReturnTemperature, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangePlantLoopReturnTemperature>,
{
    Ok((|| {
    match base.model.plant_loops.iter().find(|item| item.id == payload.id) {
        Some(item) if item.return_temperature_c != payload.new_return_temperature_c && !(!payload.new_return_temperature_c.is_finite() || !(-100.0..=300.0).contains(&payload.new_return_temperature_c)) => {
            vec![vocabulary::change_plant_loop_return_temperature(payload.id, item.return_temperature_c)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
