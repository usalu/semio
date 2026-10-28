//! ↩️ Inverse for `ChangeHumidistatZone` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeHumidistatZone, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeHumidistatZone>,
{
    Ok((|| {
    match base.model.humidistats.iter().find(|item| item.id == payload.id) {
        Some(item) if item.zone_id != payload.new_zone_id && !(!base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id)) => vec![vocabulary::change_humidistat_zone(payload.id, item.zone_id)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
