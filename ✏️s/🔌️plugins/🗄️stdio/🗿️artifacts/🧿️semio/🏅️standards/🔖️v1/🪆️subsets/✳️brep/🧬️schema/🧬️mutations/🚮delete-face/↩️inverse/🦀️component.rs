//! ↩️ `delete-face` — reconstructs the removed face from BASE via `CreateFace`.
//! Missing target ⇒ `Vec::new()`.

use super::mutation::DeleteFace;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::{create_face, SemioBrepMutation};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &DeleteFace, base: &SemioBrepSnapshot) -> Vec<SemioBrepMutation> {
    match base.faces.iter().find(|x| x.id == payload.id) {
        Some(x) => vec![SemioBrepMutation::CreateFace(create_face::mutation::CreateFace { id: x.id.clone(), outer_loop: x.outer_loop.clone(), inner_loops: x.inner_loops.clone(), surface: x.surface.clone(), orientation: x.orientation })],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
