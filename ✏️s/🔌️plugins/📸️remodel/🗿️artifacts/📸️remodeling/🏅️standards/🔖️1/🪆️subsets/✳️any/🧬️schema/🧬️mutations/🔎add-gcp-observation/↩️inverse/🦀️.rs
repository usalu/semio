//! ↩️ Inverse for `AddGcpObservation` — `remove-gcp-observation` targeting the canonical
//! `(stream_id, frame_index)` position the observation is inserted at, computed from BASE the same way
//! the diff computes it. A step the forward verb refuses or warns off (unknown GCP, unknown stream, an
//! observation the GCP already holds) moved nothing, so its inverse is `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddGcpObservation, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return Vec::new();
    };
    if !base.streams.iter().any(|stream| stream.id == payload.observation.stream_id) || gcp.observations.contains(&payload.observation) {
        return Vec::new();
    }
    let at = crate::artifacts::remodeling::mutations::ordered_index(&gcp.observations, &(payload.observation.stream_id.clone(), payload.observation.frame_index), |observation| (observation.stream_id.clone(), observation.frame_index));
    vec![crate::artifacts::remodeling::mutations::remove_gcp_observation::remove_gcp_observation(payload.id.clone(), at as u32)]
}
//#endregion 🔖️Inverse
