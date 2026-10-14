//! ↩️ Inverse for `RemoveAuthor`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::remove_author::RemoveAuthor, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_author::AddAuthor> {
    Ok((|| {
    match base.authors.iter().enumerate().find(|(_, author)| author.id == payload.id) {
        Some((position, existing)) => vec![crate::add_author::add_author_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
