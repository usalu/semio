//! 🔺️ `create-face` — sparse diff construction; if a face with this `id` already exists in
//! `base`, this is a no-op (real entity-lifecycle safety — never a duplicate id).

use super::mutation::CreateFace;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepFace, SemioBrepSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateFace, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if base.faces.iter().any(|x| x.id == payload.id) {
        return SemioBrepDiff::default();
    }
    SemioBrepDiff {
        faces: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepFace { id: payload.id.clone(), outer_loop: payload.outer_loop.clone(), inner_loops: payload.inner_loops.clone(), surface: payload.surface.clone(), orientation: payload.orientation }] }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
