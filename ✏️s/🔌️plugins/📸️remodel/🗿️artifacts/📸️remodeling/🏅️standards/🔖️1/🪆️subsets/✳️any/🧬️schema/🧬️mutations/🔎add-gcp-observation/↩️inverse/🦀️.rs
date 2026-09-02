//! ↩️ Inverse for `AddGcpObservation` — `remove-gcp-observation` targeting the index the appended
//! observation will land at (BASE's observation count). Missing target ⇒ `Vec::new()`.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddGcpObservation, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    match base.gcps.iter().find(|gcp| gcp.id == payload.id) {
        Some(gcp) => vec![crate::artifacts::remodeling::mutations::remove_gcp_observation::remove_gcp_observation(payload.id.clone(), gcp.observations.len() as u32)],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
