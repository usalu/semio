//! ↩️ Inverse for `UpdateSfmParams` — the OLD `SfmParams` looked up from BASE.
use crate::artifacts::remodeling::mutations::RemodelingMutation;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateSfmParams, base: &RemodelingSnapshot) -> Vec<RemodelingMutation> {
    vec![super::update_sfm_params(base.params.sfm.clone())]
}
//#endregion 🔖️Inverse
