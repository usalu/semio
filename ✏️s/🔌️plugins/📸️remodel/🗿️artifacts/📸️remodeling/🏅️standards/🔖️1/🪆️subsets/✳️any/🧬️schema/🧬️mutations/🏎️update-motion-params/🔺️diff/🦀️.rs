//! 🔺️ Sparse diff builder for `UpdateMotionParams` — always present, no target-missing check
//! possible (a struct field, not an id-keyed collection). The invariant is checked BEFORE the
//! identical-resubmission warning, the one guard order the whole vocabulary follows —
//! non-finite track quality ⇒ Fatal.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingParamsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::update_motion_params::component::UpdateMotionParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !payload.params.min_track_quality.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Motion params have a non-finite minimum track quality.".to_string(), [base.id.clone()]);
    }
    if payload.params == base.params.motion {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Motion params are already up to date.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(RemodelingParamsDiff { motion: Some(payload.params.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
