//! ↩️ Inverse for `ChangePeopleGainRadiantFraction` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangePeopleGainRadiantFraction, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangePeopleGainRadiantFraction>,
{
    Ok((|| {
    match base.model.people.iter().find(|item| item.id == payload.id) {
        Some(item) if item.radiant_fraction != payload.new_radiant_fraction && !(!(0.0..=1.0).contains(&payload.new_radiant_fraction)) => vec![vocabulary::change_people_gain_radiant_fraction(payload.id, item.radiant_fraction)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
