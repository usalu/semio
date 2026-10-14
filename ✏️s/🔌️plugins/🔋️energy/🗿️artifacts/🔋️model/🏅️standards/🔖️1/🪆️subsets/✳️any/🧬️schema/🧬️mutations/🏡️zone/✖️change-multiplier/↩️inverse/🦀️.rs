//! ↩️ Inverse for `ChangeZoneMultiplier` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeZoneMultiplier, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeZoneMultiplier>,
{
    Ok((|| {
    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.multiplier != payload.new_multiplier && payload.new_multiplier != 0 => vec![vocabulary::change_zone_multiplier(payload.id, zone.multiplier)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
