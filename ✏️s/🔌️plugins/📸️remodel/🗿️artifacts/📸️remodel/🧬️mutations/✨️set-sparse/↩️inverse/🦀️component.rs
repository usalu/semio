//! ↩️ Inverse for `SetSparse`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetSparse { sparse: base.results.sparse.clone() }]
}
//#endregion 🔖️Inverse
