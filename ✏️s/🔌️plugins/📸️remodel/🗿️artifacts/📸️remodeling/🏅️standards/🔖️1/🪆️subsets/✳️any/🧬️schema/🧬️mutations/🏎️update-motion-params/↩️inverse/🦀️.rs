//! ↩️ Inverse for `UpdateMotionParams` — the OLD `MotionParams` looked up from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateMotionParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_motion_params(base.params.motion.clone())]
}
//#endregion 🔖️Inverse
