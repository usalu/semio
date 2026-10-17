//! ↩️ Inverse for `DeleteFace`.

use crate::schema::mutations::create_face;
use crate::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(payload: &super::DeleteFace, base: &SemioBrepSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::schema::mutations::create_face::CreateFace>,
{
    Ok((|| {
    let Some(index) = base.faces.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let x = &base.faces[index];
    vec![Op::from(create_face::CreateFace { id: x.id.clone(), outer_loop: x.outer_loop.clone(), inner_loops: x.inner_loops.clone(), surface: x.surface.clone(), orientation: x.orientation, tol: x.tol, at: Some(index) })]

    })())
}
//#endregion 🔖️Inverse
