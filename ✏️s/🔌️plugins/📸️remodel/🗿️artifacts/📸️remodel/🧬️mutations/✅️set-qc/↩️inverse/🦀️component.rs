//! ↩️ Inverse for `SetQc`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetQc { qc: base.results.qc.clone() }]
}
//#endregion 🔖️Inverse
