//! 🔺️ `create-vertex` — sparse diff construction; a vertex with this `id` already present in
//! `base` is `mutation.duplicate-id` (Fatal, empty diff — real entity-lifecycle safety, never a
//! silent duplicate).

use super::mutation::CreateVertex;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepVertex, SemioBrepSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &CreateVertex, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.vertices.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A vertex with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff { vertices: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepVertex { id: payload.id.clone(), point: payload.point }] }), ..Default::default() })
}
//#endregion 🔖️Diff
