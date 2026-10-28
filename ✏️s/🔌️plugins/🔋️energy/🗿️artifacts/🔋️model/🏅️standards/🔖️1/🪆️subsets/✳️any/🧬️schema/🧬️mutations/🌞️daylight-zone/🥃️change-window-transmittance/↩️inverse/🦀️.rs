//! ↩️ Inverse for `ChangeDaylightZoneWindowTransmittance` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeDaylightZoneWindowTransmittance, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeDaylightZoneWindowTransmittance>,
{
    Ok((|| {
    match base.model.daylight_zones.iter().find(|item| item.id == payload.id) {
        Some(item) if item.window_transmittance != payload.new_window_transmittance && !(!payload.new_window_transmittance.is_finite() || !(0.0..=1.0).contains(&payload.new_window_transmittance)) => {
            vec![vocabulary::change_daylight_zone_window_transmittance(payload.id, item.window_transmittance)]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
