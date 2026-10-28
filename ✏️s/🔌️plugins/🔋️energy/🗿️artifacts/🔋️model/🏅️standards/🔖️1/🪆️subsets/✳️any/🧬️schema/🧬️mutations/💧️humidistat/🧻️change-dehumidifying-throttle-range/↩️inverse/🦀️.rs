//! ↩️ Inverse for `ChangeHumidistatDehumidifyingThrottleRange` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeHumidistatDehumidifyingThrottleRange, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeHumidistatDehumidifyingThrottleRange>,
{
    Ok((|| {
    match base.model.humidistats.iter().find(|item| item.id == payload.id) {
        Some(item) if item.dehumidifying_throttle_range != payload.new_dehumidifying_throttle_range && !(!payload.new_dehumidifying_throttle_range.is_finite() || payload.new_dehumidifying_throttle_range <= 0.0) => {
            vec![vocabulary::change_humidistat_dehumidifying_throttle_range(payload.id, item.dehumidifying_throttle_range)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
