//! ↩️ Inverse for `ChangeSpaceZone` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ChangeSpaceZone, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ChangeSpaceZone>,
{
    Ok((|| {
    let Some(existing) = base.model.spaces.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if !base.model.zones.iter().any(|item| item.id == payload.new_zone_id) || existing.zone_id == payload.new_zone_id {
        return Vec::new();
    }
    vec![vocabulary::change_space_zone(payload.id, existing.zone_id)]

    })())
}
//#endregion 🔖️Inverse
