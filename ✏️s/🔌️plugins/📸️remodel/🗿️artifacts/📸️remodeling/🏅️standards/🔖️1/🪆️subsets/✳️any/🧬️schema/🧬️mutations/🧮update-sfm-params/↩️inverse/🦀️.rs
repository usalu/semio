//! ↩️ Inverse for `UpdateSfmParams` — the OLD `SfmParams` looked up from BASE.
use crate::mutations::RemodelingMutation;
use crate::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdateSfmParams, base: &RemodelingSnapshot) -> Result<Vec<RemodelingMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_sfm_params(base.params.sfm.clone())]

    })())
}
//#endregion 🔖️Inverse
