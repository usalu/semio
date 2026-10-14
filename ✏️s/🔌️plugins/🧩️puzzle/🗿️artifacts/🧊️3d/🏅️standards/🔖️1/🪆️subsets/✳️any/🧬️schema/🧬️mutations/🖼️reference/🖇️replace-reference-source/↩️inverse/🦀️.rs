//! ↩️ Inverse for `ReplaceReferenceSource` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use super::mutation::ReplaceReferenceSource;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceReferenceSource>>(payload: &super::mutation::ReplaceReferenceSource, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::standards::v1::subsets::any::schema::mutations::replace_reference_source::mutation::replace_reference_source(item.id.clone(), item.source.clone())]

    })())
}
//#endregion 🔖️Inverse
