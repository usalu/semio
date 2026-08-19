//! 🔺️ `create-face` — sparse diff construction; a face with this `id` already present in
//! `base` is `mutation.duplicate-id` (Fatal, empty diff — real entity-lifecycle safety, never a
//! silent duplicate). `outer_loop`/`inner_loops` referential integrity is the subset's
//! `✅validation-report` inference's job, not this diff constructor's (see the payload leaf's
//! doc comment).

use super::mutation::CreateFace;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepFace, SemioBrepSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &CreateFace, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.faces.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A face with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff {
        faces: Some(NamedTripleDiff {
            removed: vec![],
            modified: vec![],
            added: vec![BrepFace { id: payload.id.clone(), outer_loop: payload.outer_loop.clone(), inner_loops: payload.inner_loops.clone(), surface: payload.surface.clone(), orientation: payload.orientation }],
        }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
