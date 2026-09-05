//! 🔺️ Diff for `ReplaceSurface`.

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::{NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{BrepFaceDiff, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepSurface, SemioBrepSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ReplaceSurface, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    let Some(face) = base.faces.iter().find(|f| f.id == payload.face_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Face \"{}\" does not exist.", payload.face_id), [payload.face_id.clone()]);
    };
    if face.surface == payload.new_surface {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Face \"{}\" already has this surface.", payload.face_id));
    }
    protocol::MutationOutcome::new(SemioBrepDiff {
        faces: Some(NamedTripleDiff {
            removed: vec![],
            modified: vec![NamedModified { key: payload.face_id.clone(), diff: BrepFaceDiff { outer_loop: None, inner_loops: None, surface: Some(payload.new_surface.clone()), orientation: None } }],
            added: vec![],
        }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
