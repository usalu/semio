//! ↩️ Inverse for `SetSfmParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetSfmParams { params: base.params.sfm.clone() }]
}
//#endregion 🔖️Inverse
