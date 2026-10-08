//! 🔺️ Diff for `RemoveDesign`.

use crate::standards::v1::subsets::base::schema::triples::{IndexedTripleDiff};
use crate::standards::v1::subsets::kit::schema::diff::{SemioKitDiff};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveDesign, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.designs.iter().any(|d| d.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Design \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let at = base.designs.iter().position(|d| d.id == payload.id).expect("checked above");
    protocol::MutationOutcome::new(SemioKitDiff { designs: Some(IndexedTripleDiff { removed: vec![at], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
