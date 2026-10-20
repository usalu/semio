//! 🔺️ Sparse diff builder for `ReplaceGeoProducts`. Clearing an already-absent value ⇒ Error;
//! identical resubmission ⇒ Warning.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingAssigned;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingResultsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &crate::standards::v1::subsets::any::schema::mutations::replace_geo_products::component::ReplaceGeoProducts, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.geo.is_none() && base.results.geo.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "There are no geo products to clear.".to_string(), [base.id.clone()]);
    }
    if payload.geo == base.results.geo {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Geo products are already up to date.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(RemodelingResultsDiff { geo: Some(RemodelingAssigned::new(payload.geo.clone())), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
