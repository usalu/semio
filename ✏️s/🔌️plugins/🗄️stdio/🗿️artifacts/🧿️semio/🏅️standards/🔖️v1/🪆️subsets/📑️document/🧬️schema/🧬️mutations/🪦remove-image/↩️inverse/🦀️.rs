//! ↩️ Inverse for `RemoveImage`.

#[allow(unused_imports)]
use crate::schema::snapshot::SemioDocumentSnapshot;
#[allow(unused_imports)]
use crate::schema::mutations::support::image_at;
#[allow(unused_imports)]
use crate::schema::mutations::insert_image;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::RemoveImage, base: &SemioDocumentSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::insert_image::InsertImage>,
{
    let super::RemoveImage { id } = payload;
    Ok(match image_at(base, id) {
        Some(image) => vec![Op::from(insert_image::InsertImage { image: image.clone(), at: base.images.iter().position(|i| i.id == *id) })],
        None => Vec::new(),
    })
}
//#endregion 🔖️Inverse
