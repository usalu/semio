//! ↩️ Inverse for `DeleteFace`.

use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{SemioBrepMutation, create_face, delete_face};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(payload: &super::DeleteFace, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    let Some(index) = base.faces.iter().position(|x| x.id == payload.id) else {
        return Vec::new();
    };
    let tail = &base.faces[index..];
    let mut undo: Vec<SemioBrepMutation> = tail
        .iter()
        .skip(1)
        .map(|x| SemioBrepMutation::DeleteFace(delete_face::DeleteFace { id: x.id.clone() }))
        .collect();
    undo.extend(tail.iter().map(|x| {
        SemioBrepMutation::CreateFace(create_face::CreateFace { id: x.id.clone(), outer_loop: x.outer_loop.clone(), inner_loops: x.inner_loops.clone(), surface: x.surface.clone(), orientation: x.orientation })
    }));
    undo
}
//#endregion 🔖️Inverse
