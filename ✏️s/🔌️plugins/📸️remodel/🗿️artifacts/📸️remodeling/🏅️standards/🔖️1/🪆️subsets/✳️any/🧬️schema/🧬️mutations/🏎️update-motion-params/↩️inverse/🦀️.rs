//! ↩️ Inverse for `UpdateMotionParams` — the OLD `MotionParams` looked up from BASE.
use crate::standards::v1::subsets::any::schema::mutations::update_motion_params::component::UpdateMotionParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateMotionParams>>(_payload: &crate::standards::v1::subsets::any::schema::mutations::update_motion_params::component::UpdateMotionParams, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_motion_params(base.params.motion.clone())]

    })())
}
//#endregion 🔖️Inverse
