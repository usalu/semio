//! ↩️ Inverse for `ReplaceSurface`.

use crate::schema::mutations::SemioBrepMutation;
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::ReplaceSurface, base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.faces.iter().find(|f| f.id == payload.face_id) {
        Some(face) => vec![SemioBrepMutation::ReplaceSurface(super::ReplaceSurface { face_id: payload.face_id.clone(), new_surface: face.surface.clone() })],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
