//! ↩️ Inverse for `AddAuthor`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAuthor, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return Vec::new();
    }
    vec![super::super::remove_author::remove_author(payload.author.id.clone())]

    })())
}
//#endregion 🔖️Inverse
