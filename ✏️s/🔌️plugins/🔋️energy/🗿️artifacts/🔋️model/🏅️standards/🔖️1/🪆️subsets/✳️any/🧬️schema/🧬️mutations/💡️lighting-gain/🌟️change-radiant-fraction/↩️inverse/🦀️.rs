//! ↩️ Inverse for `ChangeLightingGainRadiantFraction` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeLightingGainRadiantFraction, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeLightingGainRadiantFraction>,
{
    Ok((|| {
    match base.model.lighting.iter().find(|item| item.id == payload.id) {
        Some(item) if item.radiant_fraction != payload.new_radiant_fraction && !(!(0.0..=1.0).contains(&payload.new_radiant_fraction)) => vec![vocabulary::change_lighting_gain_radiant_fraction(payload.id, item.radiant_fraction)],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
