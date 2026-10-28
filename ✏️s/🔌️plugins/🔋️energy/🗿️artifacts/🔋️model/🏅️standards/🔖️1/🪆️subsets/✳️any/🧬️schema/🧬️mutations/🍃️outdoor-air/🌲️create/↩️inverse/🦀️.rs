//! ↩️ Inverse for `CreateOutdoorAirSystem` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::CreateOutdoorAirSystem, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::CreateOutdoorAirSystem> + From<crate::DeleteOutdoorAirSystem>,
{
    Ok((|| {
    if (base.model.outdoor_air_systems.iter().any(|item| item.id == payload.id)) || (!base.model.air_loops.iter().any(|item| item.id == payload.air_loop_id)) || (!payload.min_oa_flow_m3_s.is_finite() || payload.min_oa_flow_m3_s < 0.0) {
        return Vec::new();
    }
    if payload.index.is_some_and(|index| index as usize > base.model.outdoor_air_systems.len()) {
        return Vec::new();
    }
    vec![vocabulary::delete_outdoor_air_system(payload.id)]

    })())
}
//#endregion 🔖️Inverse
