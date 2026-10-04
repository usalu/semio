//! ↩️ Inverse for `RemoveAuthor`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveAuthor, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.authors.iter().find(|author| author.id == payload.id) {
        Some(existing) => vec![super::super::add_author::add_author(existing.clone())],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
