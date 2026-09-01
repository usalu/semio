//! 🔺️ Sparse diff builder for `UpdateSfmParams` — always present, no target-missing check possible
//! (a struct field, not an id-keyed collection). Identical resubmission ⇒ Warning; non-finite
//! thresholds ⇒ Fatal.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateSfmParams, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.params == base.params.sfm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "SfM params are already up to date.".to_string());
    }
    if !payload.params.ransac_threshold_px.is_finite() || !payload.params.huber_delta_px.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "SfM params have non-finite thresholds.".to_string(), [base.id.clone()]);
    }
    let mut params = base.params.clone();
    params.sfm = payload.params.clone();
    protocol::MutationOutcome::new(RemodelDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
