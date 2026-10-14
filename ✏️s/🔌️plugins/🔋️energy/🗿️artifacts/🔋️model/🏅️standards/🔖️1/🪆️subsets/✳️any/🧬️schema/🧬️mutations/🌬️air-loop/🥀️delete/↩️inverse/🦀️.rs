//! ↩️ Inverse for `DeleteAirLoop` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::DeleteAirLoop, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::DeleteAirLoop> + From<crate::CreateAirLoop>,
{
    Ok((|| {
    match base.model.air_loops.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((index, item)) if !(base.model.outdoor_air_systems.iter().any(|system| system.air_loop_id == payload.id)) => {
            vec![vocabulary::create_air_loop(item.id, item.name.clone(), item.supply_node_id, item.return_node_id, item.design_supply_air_flow_m3_s, item.terminal_zone_ids.clone(), Some(index as u32))]
        }
        _ => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
