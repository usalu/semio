//! 🔺️ Diff for `CreateEdge`.

use crate::artifacts::semio::standards::v1::subsets::base::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepEdge, SemioBrepSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::CreateEdge, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if base.edges.iter().any(|x| x.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("An edge with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(SemioBrepDiff {
        edges: Some(NamedTripleDiff { removed: vec![], modified: vec![], added: vec![BrepEdge { id: payload.id.clone(), start_vertex: payload.start_vertex.clone(), end_vertex: payload.end_vertex.clone(), curve: payload.curve.clone(), tol: 0.0 }] }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
