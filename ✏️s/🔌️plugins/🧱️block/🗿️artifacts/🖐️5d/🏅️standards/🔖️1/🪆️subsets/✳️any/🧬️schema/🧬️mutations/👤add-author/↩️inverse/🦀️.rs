//! ↩️ Inverse for `AddAuthor`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::add_author::AddAuthor, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_author::RemoveAuthor> {
    Ok((|| {
    if base.authors.iter().any(|item| item.id == payload.author.id) {
        return Vec::new();
    }
    vec![crate::remove_author::remove_author(payload.author.id.clone())]

    })())
}
//#endregion 🔖️Inverse
