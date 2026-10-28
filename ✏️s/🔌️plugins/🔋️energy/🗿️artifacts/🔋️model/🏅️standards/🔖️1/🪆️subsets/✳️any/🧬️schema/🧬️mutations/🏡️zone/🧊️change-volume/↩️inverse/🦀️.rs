//! ↩️ Inverse for `ChangeZoneVolume` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeZoneVolume, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeZoneVolume>,
{
    Ok((|| {
    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.volume_m3 != payload.new_volume_m3 && payload.new_volume_m3.is_finite() && payload.new_volume_m3 > 0.0 => vec![vocabulary::change_zone_volume(payload.id, zone.volume_m3)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
