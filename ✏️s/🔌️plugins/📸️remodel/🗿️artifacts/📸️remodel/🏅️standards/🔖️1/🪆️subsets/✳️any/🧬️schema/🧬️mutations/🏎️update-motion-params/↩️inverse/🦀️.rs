//! ↩️ Inverse for `UpdateMotionParams` — the OLD `MotionParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateMotionParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::update_motion_params(base.params.motion.clone())]
}
//#endregion 🔖️Inverse
