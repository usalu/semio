//! ↩️ Inverse for `InsertImage`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::InsertImage, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::InsertImage { image, .. } = payload;
    Ok(vec![SemioDocumentMutation::RemoveImage(remove_image::RemoveImage { id: image.id.clone() })])
}
//#endregion 🔖️Inverse
