//! ↩️ Inverse for `AddAuthor`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAuthor, _base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::remove_author::remove_author(payload.author.id.clone())]

    })())
}
//#endregion 🔖️Inverse
