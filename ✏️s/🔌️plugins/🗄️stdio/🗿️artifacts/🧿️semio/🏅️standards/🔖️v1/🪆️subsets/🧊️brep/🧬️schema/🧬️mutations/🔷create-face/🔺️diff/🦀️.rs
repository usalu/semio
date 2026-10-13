//! 🔺️ Diff for `CreateFace`.

use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use crate::schema::diff::SemioBrepDiff;
use crate::schema::snapshot::{BrepFace, SemioBrepSnapshot};

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
            added: vec![semio_s_artifact_stdio_semio_base::schema::triples::NamedAdded { index: payload.at.map_or(base.faces.len(), |at| at.min(base.faces.len())), item: BrepFace { id: payload.id.clone(), outer_loop: payload.outer_loop.clone(), inner_loops: payload.inner_loops.clone(), surface: payload.surface.clone(), orientation: payload.orientation, tol: payload.tol } }],
        }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
