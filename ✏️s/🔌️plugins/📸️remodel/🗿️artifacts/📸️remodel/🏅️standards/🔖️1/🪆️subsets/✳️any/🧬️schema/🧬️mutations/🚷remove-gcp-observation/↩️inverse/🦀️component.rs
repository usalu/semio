//! ↩️ Inverse for `RemoveGcpObservation` — re-`add-gcp-observation`s the captured BASE observation.
//! Missing target/index ⇒ `Vec::new()`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveGcpObservation, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return Vec::new();
    };
    let Some(observation) = gcp.observations.get(payload.observation_index as usize) else {
        return Vec::new();
    };
    vec![crate::artifacts::remodel::mutations::add_gcp_observation::mutation::add_gcp_observation(payload.id.clone(), observation.clone())]
}
//#endregion 🔖️Inverse
