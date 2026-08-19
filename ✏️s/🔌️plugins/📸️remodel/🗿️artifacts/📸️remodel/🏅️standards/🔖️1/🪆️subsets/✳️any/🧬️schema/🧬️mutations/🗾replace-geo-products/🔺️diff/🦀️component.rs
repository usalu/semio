//! 🔺️ Sparse diff builder for `ReplaceGeoProducts`. Clearing an already-absent value ⇒ Error;
//! identical resubmission ⇒ Warning.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ReplaceGeoProducts, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.geo.is_none() && base.results.geo.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "There are no geo products to clear.".to_string(), [base.id.clone()]);
    }
    if payload.geo == base.results.geo {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Geo products are already up to date.".to_string());
    }
    let mut results = base.results.clone();
    results.geo = payload.geo.clone();
    protocol::MutationOutcome::new(RemodelDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
