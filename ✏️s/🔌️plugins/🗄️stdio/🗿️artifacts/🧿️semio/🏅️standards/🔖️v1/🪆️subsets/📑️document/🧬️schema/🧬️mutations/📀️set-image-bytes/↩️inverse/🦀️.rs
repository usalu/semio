//! ↩️ Inverse for `SetImageBytes`.

use super::super::*;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::SetImageBytes, base: &SemioDocumentSnapshot) -> Result<Vec<SemioDocumentMutation>, semio_framework_value::ValueError> {
    let super::SetImageBytes { id, .. } = payload;
    Ok(match image_at(base, id) {
        Some(image) => vec![SemioDocumentMutation::SetImageBytes(set_image_bytes::SetImageBytes { id: id.clone(), mime: image.mime.clone(), bytes: image.bytes.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
