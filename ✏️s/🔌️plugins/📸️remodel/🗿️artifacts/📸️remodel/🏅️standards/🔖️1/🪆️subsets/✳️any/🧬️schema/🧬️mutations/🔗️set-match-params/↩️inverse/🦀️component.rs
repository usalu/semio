//! ↩️ Inverse for `SetMatchParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetMatchParams { params: base.params.matching.clone() }]
}
//#endregion 🔖️Inverse
