//! ↩️ Inverse for `InsertImage`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::remove_image;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::InsertImage, _base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::remove_image::RemoveImage>,
{
    let super::InsertImage { image, .. } = payload;
    Ok(vec![Op::from(remove_image::RemoveImage { id: image.id.clone() })])
}
//#endregion 🔖️Inverse
