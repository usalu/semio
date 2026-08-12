//! 🔺️ `create-edge` — sparse diff construction; if a edge with this `id` already exists in
//! `base`, this is a no-op (real entity-lifecycle safety — never a duplicate id).

use super::mutation::CreateEdge;
use crate::artifacts::semio::standards::v1::engine::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepEdge, SemioBrepSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &CreateEdge, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if base.edges.iter().any(|x| x.id == payload.id) {
        return SemioBrepDiff::default();
    }
    SemioBrepDiff {
        edges: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepEdge { id: payload.id.clone(), start_vertex: payload.start_vertex.clone(), end_vertex: payload.end_vertex.clone(), curve: payload.curve.clone() }] }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
