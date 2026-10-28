//! ↩️ Inverse for `DeleteOutdoorAirSystem` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteOutdoorAirSystem, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteOutdoorAirSystem> + From<crate::CreateOutdoorAirSystem>,
{
    Ok((|| {
    match base.model.outdoor_air_systems.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((index, item)) => vec![vocabulary::create_outdoor_air_system(item.id, item.air_loop_id, item.min_oa_flow_m3_s, item.economizer_enabled, Some(index as u32))],
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
