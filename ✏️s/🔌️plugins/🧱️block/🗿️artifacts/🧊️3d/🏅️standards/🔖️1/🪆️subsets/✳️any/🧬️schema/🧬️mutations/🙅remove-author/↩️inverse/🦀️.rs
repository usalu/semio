//! ↩️ Inverse for `RemoveAuthor`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveAuthor, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.authors.iter().find(|author| author.id == payload.id) {
        Some((position, existing)) => vec![super::super::add_author::add_author_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
