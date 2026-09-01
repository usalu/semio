//! ↩️ Inverse for `UpdateSfmParams` — the OLD `SfmParams` looked up from BASE.
use crate::artifacts::remodel::mutations::RemodelMutation;
use crate::artifacts::remodel::RemodelSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateSfmParams, base: &RemodelSnapshot) -> Vec<RemodelMutation> {
    vec![super::update_sfm_params(base.params.sfm.clone())]
}
//#endregion 🔖️Inverse
