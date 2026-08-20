//! 🔺️ `replace-curve` — sparse diff construction; an absent BASE `edge_id` is
//! `mutation.target-missing` (Error, empty diff); a `new_curve` identical to the edge's current
//! curve is `mutation.no-op` (Warning, empty diff).

use super::mutation::ReplaceCurve;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedModified;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{BrepEdgeDiff, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &ReplaceCurve, base: &SemioBrepSnapshot) -> protocol::MutationOutcome<SemioBrepDiff> {
    let Some(edge) = base.edges.iter().find(|e| e.id == payload.edge_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Edge \"{}\" does not exist.", payload.edge_id), [payload.edge_id.clone()]);
    };
    if edge.curve == payload.new_curve {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Edge \"{}\" already has this curve.", payload.edge_id));
    }
    protocol::MutationOutcome::new(SemioBrepDiff {
        edges: Some(NamedTripleDiff { removed: vec![], modified: vec![NamedModified { key: payload.edge_id.clone(), diff: BrepEdgeDiff { start_vertex: None, end_vertex: None, curve: Some(payload.new_curve.clone()) } }], added: vec![] }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
