//! 🔺️ Sparse diff builder for `ReplaceQc`. Clearing an already-absent report ⇒ Error; identical
//! resubmission ⇒ Warning.
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingAssigned;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingDiff;
use semio_s_artifact_remodel_remodeling_model_diff::standards::v1::subsets::any::schema::diff::RemodelingResultsDiff;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

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
