//! ↩️ Inverse for `RemoveGcpObservation` — re-`add-gcp-observation`s the captured BASE observation,
//! which lands at its canonical `(stream_id, frame_index)` position, i.e. exactly the one it was
//! removed from. Missing target/index ⇒ `Vec::new()`.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveGcpObservation, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return Vec::new();
    };
    let Some(observation) = gcp.observations.get(payload.observation_index as usize) else {
        return Vec::new();
    };
    vec![crate::mutations::add_gcp_observation::add_gcp_observation(payload.id.clone(), observation.clone())]
}
//#endregion 🔖️Inverse
