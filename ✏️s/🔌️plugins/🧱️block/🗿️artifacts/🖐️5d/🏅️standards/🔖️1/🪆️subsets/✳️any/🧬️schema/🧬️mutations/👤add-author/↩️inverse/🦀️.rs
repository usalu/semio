//! ↩️ Inverse for `AddAuthor`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAuthor, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return Vec::new();
    }
    vec![super::super::remove_author::remove_author(payload.author.id.clone())]

    })())
}
//#endregion 🔖️Inverse
