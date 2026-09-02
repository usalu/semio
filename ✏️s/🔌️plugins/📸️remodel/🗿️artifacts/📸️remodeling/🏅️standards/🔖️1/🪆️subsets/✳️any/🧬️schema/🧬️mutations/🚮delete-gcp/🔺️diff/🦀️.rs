//! 🔺️ Sparse diff builder for `DeleteGcp`. Missing target ⇒ Error; a GCP carrying observations
//! reports the cascade of its own dependent observations being swept away with it.
use crate::artifacts::remodeling::diff::{RemodelingDiff, RemodelingGcpList};
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteGcp, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let cascaded_observation_count = gcp.observations.len();
    let gcps: Vec<_> = base.gcps.iter().filter(|gcp| gcp.id != payload.id).cloned().collect();
    let outcome = protocol::MutationOutcome::new(RemodelingDiff { gcps: Some(RemodelingGcpList { values: gcps }), ..Default::default() });
    if cascaded_observation_count == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting GCP \"{}\" also removed {} observation(s).", payload.id, cascaded_observation_count))
    }
}
//#endregion 🔖️Diff
