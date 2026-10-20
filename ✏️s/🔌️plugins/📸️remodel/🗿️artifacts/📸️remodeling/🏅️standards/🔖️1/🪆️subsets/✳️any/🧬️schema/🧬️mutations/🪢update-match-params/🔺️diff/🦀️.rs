//! 🔺️ Sparse diff builder for `UpdateMatchParams` — the field is always present, so there is no
//! missing-target case. A non-finite or out-of-range `ratio_test` ⇒ Fatal `mutation.invariant`;
//! identical params ⇒ Warning `mutation.no-op`.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingParamsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::update_match_params::component::UpdateMatchParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if !payload.params.ratio_test.is_finite() || payload.params.ratio_test <= 0.0 || payload.params.ratio_test > 1.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Match ratio test {} must be finite and within (0, 1].", payload.params.ratio_test), Vec::<String>::new());
    }
    if payload.params == base.params.matching {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Matching params are unchanged.");
    }
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(RemodelingParamsDiff { matching: Some(payload.params.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
