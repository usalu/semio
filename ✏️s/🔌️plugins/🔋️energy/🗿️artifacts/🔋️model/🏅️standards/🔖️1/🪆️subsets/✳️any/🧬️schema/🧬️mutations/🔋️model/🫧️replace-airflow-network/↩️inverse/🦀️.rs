//! ↩️ Inverse for `ReplaceAirflowNetwork` — always computed from BASE, never by inverting the delta.

use crate as vocabulary;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️Inverse
/// ↩️ A refused or no-op forward step has nothing to undo, so it answers with no steps at all.
pub fn inverse<Op>(payload: &super::ReplaceAirflowNetwork, base: &EnergyModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<super::ReplaceAirflowNetwork>,
{
    Ok((|| {
    if payload.zone_ids.len() != payload.node_ids.len() || (!payload.present && !(payload.zone_ids.is_empty() && payload.link_ids.is_empty())) {
        return Vec::new();
    }
    match &base.model.airflow_network {
        Some(old) => vec![vocabulary::replace_airflow_network(true, old.zone_node_ids.iter().map(|(zone, _)| zone.0).collect(), old.zone_node_ids.iter().map(|(_, node)| *node).collect(), old.outdoor_node_id, old.link_ids.clone())],
        None => vec![vocabulary::replace_airflow_network(false, Vec::new(), Vec::new(), 0, Vec::new())],
    }

    })())
}
//#endregion 🔖️Inverse
