//! ↩️ Inverse for `ChangeMetaDescription`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeMetaDescription, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::change_meta_description::change_meta_description(base.meta.description.clone())]

    })())
}
//#endregion 🔖️Inverse
