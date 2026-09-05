//! 🔺️ Diff for `DeleteModel`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitModelChildList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::DeleteModel, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.models.iter().any(|c| c.child_id == payload.child_id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Model child \"{}\" does not exist.", payload.child_id), [payload.child_id.clone()]);
    }
    let models: Vec<_> = base.models.iter().filter(|c| c.child_id != payload.child_id).cloned().collect();
    protocol::MutationOutcome::new(SemioKitDiff { models: Some(SemioKitModelChildList { values: models }), ..Default::default() })
}
//#endregion 🔖️Diff
