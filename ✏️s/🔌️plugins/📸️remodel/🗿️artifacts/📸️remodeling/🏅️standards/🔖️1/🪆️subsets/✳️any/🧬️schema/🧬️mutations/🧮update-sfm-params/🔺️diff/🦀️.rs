//! 🔺️ Sparse diff builder for `UpdateSfmParams` — always present, no target-missing check possible
//! (a struct field, not an id-keyed collection). The invariant is checked BEFORE the
//! identical-resubmission warning, the one guard order the whole vocabulary follows — non-finite
//! thresholds ⇒ Fatal.
use crate::diff::RemodelingDiff;
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateSfmParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !payload.params.ransac_threshold_px.is_finite() || !payload.params.huber_delta_px.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "SfM params have non-finite thresholds.".to_string(), [base.id.clone()]);
    }
    if payload.params == base.params.sfm {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "SfM params are already up to date.".to_string());
    }
    let mut params = base.params.clone();
    params.sfm = payload.params.clone();
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(params), ..Default::default() })
}
//#endregion 🔖️Diff
