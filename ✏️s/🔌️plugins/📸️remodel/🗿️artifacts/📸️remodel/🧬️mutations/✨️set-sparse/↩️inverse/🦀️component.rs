//! ↩️ Inverse for `SetSparse`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetSparse { sparse: base.results.sparse.clone() }]
}
//#endregion 🔖️Inverse
