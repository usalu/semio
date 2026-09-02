//! 🔺️ Sparse diff builder for `ReplaceQc`. Clearing an already-absent report ⇒ Error; identical
//! resubmission ⇒ Warning.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceQc, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.qc.is_none() && base.results.qc.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "There is no QC report to clear.".to_string(), [base.id.clone()]);
    }
    if payload.qc == base.results.qc {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "QC report is already up to date.".to_string());
    }
    let mut results = base.results.clone();
    results.qc = payload.qc.clone();
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(results), ..Default::default() })
}
//#endregion 🔖️Diff
