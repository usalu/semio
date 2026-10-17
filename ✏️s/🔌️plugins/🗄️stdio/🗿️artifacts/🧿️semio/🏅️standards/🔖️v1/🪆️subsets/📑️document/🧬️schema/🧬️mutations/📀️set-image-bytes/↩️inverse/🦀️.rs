//! ↩️ Inverse for `SetImageBytes`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::image_at;
#[allow(unused_imports)]
use crate::schema::mutations::set_image_bytes;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::SetImageBytes, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::set_image_bytes::SetImageBytes>,
{
    let super::SetImageBytes { id, .. } = payload;
    Ok(match image_at(base, id) {
        Some(image) => vec![Op::from(set_image_bytes::SetImageBytes { id: id.clone(), mime: image.mime.clone(), bytes: image.bytes.clone() })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
