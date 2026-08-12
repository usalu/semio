//! 🔺️ `replace-surface` — sparse diff construction; an absent BASE `face_id` is a no-op clone.

use super::mutation::ReplaceSurface;
use crate::artifacts::semio::standards::v1::engine::triples::NamedModified;
use crate::artifacts::semio::standards::v1::engine::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{BrepFaceDiff, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceSurface, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if !base.faces.iter().any(|f| f.id == payload.face_id) {
        return SemioBrepDiff::default();
    }
    SemioBrepDiff {
        faces: Some(NamedTripleDiff {
            removed: vec![],
            modified: vec![NamedModified {
                key: payload.face_id.clone(),
                diff: BrepFaceDiff { outer_loop: None, inner_loops: None, surface: Some(payload.new_surface.clone()), orientation: None },
            }],
            added: vec![],
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
