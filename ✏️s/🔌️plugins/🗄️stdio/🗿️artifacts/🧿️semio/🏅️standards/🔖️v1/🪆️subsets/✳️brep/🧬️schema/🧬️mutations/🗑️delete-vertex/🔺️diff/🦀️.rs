//! 🔺️ Diff for `DeleteVertex`.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::SemioBrepDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteVertex, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    if !base.vertices.iter().any(|v| v.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Vertex \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let dependent_edges: Vec<String> = base.edges.iter().filter(|e| e.start_vertex == payload.id || e.end_vertex == payload.id).map(|e| e.id.clone()).collect();
    let outcome = protocol::MutationOutcome::new(SemioBrepDiff {
        vertices: Some(NamedTripleDiff { removed: vec![payload.id.clone()], modified: vec![], added: vec![] }),
        edges: if dependent_edges.is_empty() { None } else { Some(NamedTripleDiff { removed: dependent_edges.clone(), modified: vec![], added: vec![] }) },
        ..Default::default()
    });
    if dependent_edges.is_empty() {
        outcome
    } else {
        outcome.info("mutation.cascade", format!("Deleting vertex \"{}\" also removed {} connected edge(s): {}.", payload.id, dependent_edges.len(), dependent_edges.join(", ")))
    }
}
//#endregion 🔖️Diff
