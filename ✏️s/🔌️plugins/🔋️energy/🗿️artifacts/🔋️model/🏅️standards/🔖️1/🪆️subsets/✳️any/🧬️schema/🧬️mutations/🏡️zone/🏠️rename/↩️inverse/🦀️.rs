//! ↩️ Inverse for `RenameZone` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::RenameZone, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::RenameZone>,
{
    Ok((|| {
    match base.model.zones.iter().find(|zone| zone.id == payload.id) {
        Some(zone) if zone.name != payload.new_name && !payload.new_name.trim().is_empty() => vec![vocabulary::rename_zone(payload.id, zone.name.clone())],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
