//! 🔺️ Sparse diff builder for `UpdateSfmParams` — always present, no target-missing check possible
//! (a struct field, not an id-keyed collection). The invariant is checked BEFORE the
//! identical-resubmission warning, the one guard order the whole vocabulary follows — non-finite
//! thresholds ⇒ Fatal.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingParamsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateSfmParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !payload.params.ransac_threshold_px.is_finite() || !payload.params.huber_delta_px.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "SfM params have non-finite thresholds.".to_string(), [base.id.clone()]);
    }
    if payload.params == base.params.sfm {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "SfM params are already up to date.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(RemodelingParamsDiff { sfm: Some(payload.params.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
