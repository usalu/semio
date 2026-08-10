//! ✅️ Remodel mutation — `SetQc` apply.
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelSnapshot, qc: &Option<crate::artifacts::remodel::QcReportSnapshot>) {
    next.results.qc = qc.clone();
}
//#endregion 🔖️Mutation
