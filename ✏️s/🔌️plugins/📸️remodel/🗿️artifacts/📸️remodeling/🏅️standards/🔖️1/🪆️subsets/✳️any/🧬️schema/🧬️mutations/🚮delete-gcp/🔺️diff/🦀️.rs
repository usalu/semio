//! 🔺️ Sparse diff builder for `DeleteGcp`. Missing target ⇒ Error; a GCP carrying observations
//! reports the cascade of its own dependent observations being swept away with it.
use crate::diff::{RemodelingDiff, RemodelingRow};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteGcp, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let Some(gcp) = base.gcps.iter().find(|gcp| gcp.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("GCP \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let cascaded_observation_count = gcp.observations.len();
    let outcome = protocol::MutationOutcome::new(RemodelingDiff::gcp_rows(vec![RemodelingRow::Remove { key: payload.id.clone() }]));
    if cascaded_observation_count == 0 {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting GCP \"{}\" also removed {} observation(s).", payload.id, cascaded_observation_count))
    }
}
//#endregion 🔖️Diff
