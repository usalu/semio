//! ↩️ Inverse for `UpdateMotionParams` — the OLD `MotionParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::UpdateMotionParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::mutation::update_motion_params(base.params.motion.clone())]
}
//#endregion 🔖️Inverse
