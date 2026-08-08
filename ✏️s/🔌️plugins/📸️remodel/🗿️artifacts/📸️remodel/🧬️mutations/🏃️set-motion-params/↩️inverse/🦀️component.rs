//! ↩️ Inverse for `SetMotionParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetMotionParams { params: base.params.motion.clone() }]
}
//#endregion 🔖️Inverse
