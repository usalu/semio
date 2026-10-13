//! 🔺️ Diff for `DeleteEdge`.

use semio_s_artifact_stdio_semio_base::schema::triples::IndexedTripleDiff;
use crate::schema::diff::SemioGraphDiff;
use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteEdge, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    if !base.edges.iter().any(|e| e.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.id.value), [payload.id.value.clone()]);
    }
    let at = base.edges.iter().position(|e| e.id == payload.id).expect("checked above");
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: None, edges: Some(IndexedTripleDiff { removed: vec![at], ..Default::default() }) })
}
//#endregion 🔖️Diff
