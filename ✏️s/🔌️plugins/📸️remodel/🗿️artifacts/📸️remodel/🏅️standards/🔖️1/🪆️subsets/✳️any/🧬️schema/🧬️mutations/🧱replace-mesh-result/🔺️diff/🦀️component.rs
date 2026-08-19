//! 🔺️ Sparse diff builder for `ReplaceMeshResult` — `results.mesh` is always present (defaults to a
//! placeholder box), so there is no target-missing case; identical resubmission ⇒ Warning.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ReplaceMeshResult, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if *payload.mesh == base.results.mesh {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Mesh result is already up to date.".to_string());
    }
    let mut results = base.results.clone();
    results.mesh = (*payload.mesh).clone();
    protocol::MutationOutcome::new(RemodelDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
