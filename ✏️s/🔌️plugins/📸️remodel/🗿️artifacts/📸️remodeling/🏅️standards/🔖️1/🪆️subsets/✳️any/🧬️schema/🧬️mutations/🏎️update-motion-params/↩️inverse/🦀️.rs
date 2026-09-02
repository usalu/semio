//! ↩️ Inverse for `UpdateMotionParams` — the OLD `MotionParams` looked up from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateMotionParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_motion_params(base.params.motion.clone())]
}
//#endregion 🔖️Inverse
