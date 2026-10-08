//! 🔺️ Diff for `DeleteObject`.

use crate::standards::v1::subsets::base::schema::triples::{IndexedTripleDiff};
use crate::standards::v1::subsets::kit::schema::diff::{SemioKitDiff};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteObject, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.objects.iter().any(|c| c.child_id == payload.child_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Object child \"{}\" does not exist.", payload.child_id), [payload.child_id.clone()]);
    }
    let at = base.objects.iter().position(|c| c.child_id == payload.child_id).expect("checked above");
    protocol::MutationOutcome::new(SemioKitDiff { objects: Some(IndexedTripleDiff { removed: vec![at], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
