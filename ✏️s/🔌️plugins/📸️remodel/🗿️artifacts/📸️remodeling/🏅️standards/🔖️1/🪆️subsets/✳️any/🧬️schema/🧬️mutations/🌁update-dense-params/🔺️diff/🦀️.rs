//! 🔺️ Sparse diff builder for `UpdateDenseParams` — always present, no target-missing check possible
//! (a struct field, not an id-keyed collection). The invariant is checked BEFORE the
//! identical-resubmission warning, the one guard order the whole vocabulary follows — non-finite
//! confidence threshold ⇒ Fatal.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingParamsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::update_dense_params::component::UpdateDenseParams;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateDenseParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !payload.params.confidence_threshold.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Dense params have a non-finite confidence threshold.".to_string(), [base.id.clone()]);
    }
    if payload.params == base.params.dense {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Dense params are already up to date.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(RemodelingParamsDiff { dense: Some(payload.params.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
