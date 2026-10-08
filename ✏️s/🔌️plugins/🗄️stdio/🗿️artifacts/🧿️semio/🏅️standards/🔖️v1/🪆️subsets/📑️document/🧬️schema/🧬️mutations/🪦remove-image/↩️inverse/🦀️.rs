//! ↩️ Inverse for `RemoveImage`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::RemoveImage, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::RemoveImage { id } = payload;
    Ok(match image_at(base, id) {
        Some(image) => vec![SemioDocumentMutation::InsertImage(insert_image::InsertImage { image: image.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
