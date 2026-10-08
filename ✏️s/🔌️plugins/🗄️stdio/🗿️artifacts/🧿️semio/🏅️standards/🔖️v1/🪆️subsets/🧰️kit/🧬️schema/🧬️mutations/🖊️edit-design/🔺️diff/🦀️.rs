//! 🔺️ Diff for `EditDesign`.

use crate::standards::v1::subsets::base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitDesignDiff};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::EditDesign, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    let Some(existing) = base.designs.iter().find(|d| d.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Design \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.pieces == payload.pieces && existing.connections == payload.connections {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Design \"{}\" already has that content.", payload.id));
    }
    let at = base.designs.iter().position(|d| d.id == payload.id).expect("checked above");
    let diff = SemioKitDesignDiff { name: None, pieces: (existing.pieces != payload.pieces).then(|| payload.pieces.clone()), connections: (existing.connections != payload.connections).then(|| payload.connections.clone()) };
    protocol::MutationOutcome::new(SemioKitDiff { designs: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff }], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
