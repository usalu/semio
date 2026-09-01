//! 🔺️ Diff for `EditDesign`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDesignList, SemioKitDiff};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitConnection, SemioKitPiece, SemioKitSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::EditDesign, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    let Some(existing) = base.designs.iter().find(|d| d.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Design \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    if existing.pieces == payload.pieces && existing.connections == payload.connections {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Design \"{}\" already has that content.", payload.id));
    }
    let mut designs = base.designs.clone();
    if let Some(d) = designs.iter_mut().find(|d| d.id == payload.id) {
        d.pieces = payload.pieces.clone();
        d.connections = payload.connections.clone();
    }
    protocol::MutationOutcome::new(SemioKitDiff { designs: Some(SemioKitDesignList { values: designs }), ..Default::default() })
}
//#endregion 🔖️Diff
