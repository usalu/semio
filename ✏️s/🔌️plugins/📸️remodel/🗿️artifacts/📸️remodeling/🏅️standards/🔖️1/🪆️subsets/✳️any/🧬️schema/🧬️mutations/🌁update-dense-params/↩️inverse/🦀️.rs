//! ↩️ Inverse for `UpdateDenseParams` — the OLD `DenseParams` looked up from BASE.
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::update_dense_params::component::UpdateDenseParams;
use crate::standards::v1::subsets::any::schema::mutations::update_dense_params::component::update_dense_params;

//#region 🔖️Inverse
pub fn inverse<Op: From<UpdateDenseParams>>(_payload: &super::UpdateDenseParams, base: &RemodelingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::update_dense_params(base.params.dense.clone())]

    })())
}
//#endregion 🔖️Inverse
