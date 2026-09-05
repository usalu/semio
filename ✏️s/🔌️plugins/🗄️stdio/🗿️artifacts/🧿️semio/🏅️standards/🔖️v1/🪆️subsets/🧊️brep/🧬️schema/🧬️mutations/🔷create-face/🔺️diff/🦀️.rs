//! 🔺️ Diff for `CreateFace`.

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepFace, BrepSurface, SemioBrepSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateFace, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.faces.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A face with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff {
        faces: Some(NamedTripleDiff {
            removed: vec![],
            modified: vec![],
            added: vec![BrepFace { id: payload.id.clone(), outer_loop: payload.outer_loop.clone(), inner_loops: payload.inner_loops.clone(), surface: payload.surface.clone(), orientation: payload.orientation, tol: 0.0 }],
        }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
