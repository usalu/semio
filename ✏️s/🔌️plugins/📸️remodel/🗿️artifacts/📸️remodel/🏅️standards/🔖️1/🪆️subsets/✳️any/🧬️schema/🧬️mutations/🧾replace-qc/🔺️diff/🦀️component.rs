//! 🔺️ Sparse diff builder for `ReplaceQc`. Clearing an already-absent report ⇒ Error; identical
//! resubmission ⇒ Warning.
use crate::artifacts::remodel::diff::RemodelDiff;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ReplaceQc, base: &RemodelSnapshot) -> protocol::MutationOutcome<RemodelDiff> {
    if payload.qc.is_none() && base.results.qc.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "There is no QC report to clear.".to_string(), [base.id.clone()]);
    }
    if payload.qc == base.results.qc {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "QC report is already up to date.".to_string());
    }
    let mut results = base.results.clone();
    results.qc = payload.qc.clone();
    protocol::MutationOutcome::new(RemodelDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
