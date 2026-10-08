//! 🔺️ Diff for `DeleteModel`.

use crate::standards::v1::subsets::base::schema::triples::{IndexedTripleDiff};
use crate::standards::v1::subsets::kit::schema::diff::{SemioKitDiff};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteModel, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.models.iter().any(|c| c.child_id == payload.child_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Model child \"{}\" does not exist.", payload.child_id), [payload.child_id.clone()]);
    }
    let at = base.models.iter().position(|c| c.child_id == payload.child_id).expect("checked above");
    protocol::MutationOutcome::new(SemioKitDiff { models: Some(IndexedTripleDiff { removed: vec![at], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
