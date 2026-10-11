//! ↩️ Inverse for `RemoveAuthor`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveAuthor, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.authors.iter().enumerate().find(|(_, author)| author.id == payload.id) {
        Some((position, existing)) => vec![super::super::add_author::add_author_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
