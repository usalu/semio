//! 🔺️ Diff for `ReplaceCurve`.

use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{BrepEdgeDiff, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, SemioBrepSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::ReplaceCurve, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    let Some(edge) = base.edges.iter().find(|e| e.id == payload.edge_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.edge_id), [payload.edge_id.clone()]);
    };
    if edge.curve == payload.new_curve {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Edge \"{}\" already has this curve.", payload.edge_id));
    }
    protocol::MutationOutcome::new(SemioBrepDiff {
        edges: Some(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: payload.edge_id.clone(), diff: BrepEdgeDiff { start_vertex: None, end_vertex: None, curve: Some(payload.new_curve.clone()) } }], added: vec![] }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
