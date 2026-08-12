//! 🔺️ `replace-curve` — sparse diff construction; an absent BASE `edge_id` is a no-op clone
//! (nothing at that address to replace).

use super::mutation::ReplaceCurve;
use crate::artifacts::semio::standards::v1::engine::triples::NamedModified;
use crate::artifacts::semio::standards::v1::engine::triples::NamedTripleDiff;
use crate::artifacts::semio::standards::v1::subsets::brep::schema::diff::{BrepEdgeDiff, SemioBrepDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceCurve, base: &SemioBrepSnapshot) -> SemioBrepDiff {
    if !base.edges.iter().any(|e| e.id == payload.edge_id) {
        return SemioBrepDiff::default();
    }
    SemioBrepDiff {
        edges: Some(NamedTripleDiff {
            removed: vec![],
            modified: vec![NamedModified { key: payload.edge_id.clone(), diff: BrepEdgeDiff { start_vertex: None, end_vertex: None, curve: Some(payload.new_curve.clone()) } }],
            added: vec![],
        }),
        ..Default::default()
    }
}
//#endregion 🔖️Diff
