//! 🔺️ `replace-surface` — sparse diff construction; an absent BASE `face_id` is
//! `mutation.target-missing` (Error, empty diff); a `new_surface` identical to the face's
//! current surface is `mutation.no-op` (Warning, empty diff).

use super::mutation::ReplaceSurface;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedModified;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{BrepFaceDiff, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &ReplaceSurface, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    let Some(face) = base.faces.iter().find(|f| f.id == payload.face_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Face \"{}\" does not exist.", payload.face_id), [payload.face_id.clone()]).await;
    };
    if face.surface == payload.new_surface {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Face \"{}\" already has this surface.", payload.face_id)).await;
    }
    protocol::MutationOutcome::new(SemioBrepDiff {
        faces: Some(NamedTripleDiff {
            removed: vec![],
            modified: vec![NamedModified { key: payload.face_id.clone(), diff: BrepFaceDiff { outer_loop: None, inner_loops: None, surface: Some(payload.new_surface.clone()), orientation: None } }],
            added: vec![],
        }),
        ..Default::default()
    }).await
}
//#endregion 🔖️Diff
