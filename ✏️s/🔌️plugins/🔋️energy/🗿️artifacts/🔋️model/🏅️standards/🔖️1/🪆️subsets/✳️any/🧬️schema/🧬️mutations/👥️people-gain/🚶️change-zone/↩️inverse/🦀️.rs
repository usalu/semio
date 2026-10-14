//! ↩️ Inverse for `ChangePeopleGainZone` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangePeopleGainZone, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangePeopleGainZone>,
{
    Ok((|| {
    match base.model.people.iter().find(|item| item.id == payload.id) {
        Some(item) if item.zone_id != payload.new_zone_id && base.model.zones.iter().any(|zone| zone.id == payload.new_zone_id) => vec![vocabulary::change_people_gain_zone(payload.id, item.zone_id)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
