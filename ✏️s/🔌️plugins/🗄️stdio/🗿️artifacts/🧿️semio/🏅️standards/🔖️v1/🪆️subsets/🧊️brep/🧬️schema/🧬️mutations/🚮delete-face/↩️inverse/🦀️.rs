//! ↩️ Inverse for `DeleteFace`.

use crate::standards::v1::subsets::brep::schema::mutations::{create_face, delete_face, SemioBrepMutation};
use crate::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteFace, base: &SemioBrepSnapshot) -> Result<Vec<SemioBrepMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(index) = base.faces.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let x = &base.faces[index];
    vec![SemioBrepMutation::CreateFace(create_face::CreateFace { id: x.id.clone(), outer_loop: x.outer_loop.clone(), inner_loops: x.inner_loops.clone(), surface: x.surface.clone(), orientation: x.orientation, tol: x.tol, at: Some(index) })]

    })())
}
//#endregion 🔖️Inverse
