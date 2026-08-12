//! ↩️ Inverse for `AddGcpObservation` — `remove-gcp-observation` targeting the index the appended
//! observation will land at (BASE's observation count). Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddGcpObservation, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    match base.gcps.iter().find(|gcp| gcp.id == payload.id) {
        Some(gcp) => vec![crate::artifacts::remodel::mutations::remove_gcp_observation::mutation::remove_gcp_observation(payload.id.clone(), gcp.observations.len() as u32)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
