//! ↩️ Inverse for `ReplaceSurface`.

use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::ReplaceSurface, base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::replace_surface::ReplaceSurface>,
{
    Ok((|| {
    match base.faces.iter().find(|f| f.id == payload.face_id) {
        Some(face) => vec![Op::from(super::ReplaceSurface { face_id: payload.face_id.clone(), new_surface: face.surface.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
