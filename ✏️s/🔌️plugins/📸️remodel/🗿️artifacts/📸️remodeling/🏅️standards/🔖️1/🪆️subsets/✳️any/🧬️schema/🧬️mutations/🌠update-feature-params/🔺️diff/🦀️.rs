//! 🔺️ Sparse diff builder for `UpdateFeatureParams` — the field is always present, so there is no
//! missing-target case. A zero target count or a non-finite/negative edge threshold ⇒ Fatal
//! `mutation.invariant`; identical params ⇒ Warning `mutation.no-op`.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingParamsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::update_feature_params::component::UpdateFeatureParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.params.target_count == 0 || !payload.params.edge_threshold.is_finite() || payload.params.edge_threshold < 0.0 {
        return protocol::MutationOutcome::fatal(
            "mutation.invariant",
            format!("Feature params need a positive target count and a finite non-negative edge threshold (got target_count={}, edge_threshold={}).", payload.params.target_count, payload.params.edge_threshold),
            Vec::<String>::new(),
        );
    }
    if payload.params == base.params.feature {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Feature params are unchanged.");
    }
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(RemodelingParamsDiff { feature: Some(payload.params.clone()), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
