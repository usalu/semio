//! 🔺️ Sparse diff builder for `ReplaceQc`. Clearing an already-absent report ⇒ Error; identical
//! resubmission ⇒ Warning.
use crate::diff::{RemodelingAssigned, RemodelingDiff, RemodelingResultsDiff};
use crate::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ReplaceQc, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    if payload.qc.is_none() && base.results.qc.is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", "There is no QC report to clear.".to_string(), [base.id.clone()]);
    }
    if payload.qc == base.results.qc {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "QC report is already up to date.".to_string());
    }
    protocol::MutationOutcome::new(RemodelingDiff { results: Some(RemodelingResultsDiff { qc: Some(RemodelingAssigned::new(payload.qc.clone())), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
