//! ✅️ Remodel mutation — `SetQc` apply.
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Mutation
pub fn apply(next: &mut RemodelProjection, qc: &Option<crate::artifacts::remodel::QcReportSnapshot>) {
    next.results.qc = qc.clone();
}
//#endregion 🔖️Mutation
