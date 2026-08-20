//! 🔺️ `create-edge` — sparse diff construction; an edge with this `id` already present in
//! `base` is `mutation.duplicate-id` (Fatal, empty diff — real entity-lifecycle safety, never a
//! silent duplicate). `start_vertex`/`end_vertex` referential integrity is the subset's
//! `✅validation-report` inference's job, not this diff constructor's (see the payload leaf's
//! doc comment).

use super::mutation::CreateEdge;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepEdge, SemioBrepSnapshot};

//#region 🔖️Diff
pub async fn diff(payload: &CreateEdge, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.edges.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.id), [payload.id.clone()]).await;
    }
    protocol::MutationOutcome::new(SemioBrepDiff {
        edges: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepEdge { id: payload.id.clone(), start_vertex: payload.start_vertex.clone(), end_vertex: payload.end_vertex.clone(), curve: payload.curve.clone() }] }),
        ..Default::default()
    }).await
}
//#endregion 🔖️Diff
