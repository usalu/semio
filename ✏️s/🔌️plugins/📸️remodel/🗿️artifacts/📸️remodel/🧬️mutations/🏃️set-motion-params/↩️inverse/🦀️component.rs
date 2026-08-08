//! ↩️ Inverse for `SetMotionParams`.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelProjection;

//#region 🔖️Inverse
pub fn inverse(base: &RemodelProjection) -> Vec<RemodelMutation> {
    vec![RemodelMutation::SetMotionParams { params: base.params.motion.clone() }]
}
//#endregion 🔖️Inverse
