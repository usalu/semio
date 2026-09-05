//! 🔺️ Diff for `RemoveType`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitTypeList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveType, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.types.iter().any(|t| t.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Type \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let types: Vec<_> = base.types.iter().filter(|t| t.id != payload.id).cloned().collect();
    protocol::MutationOutcome::new(SemioKitDiff { types: Some(SemioKitTypeList { values: types }), ..Default::default() })
}
//#endregion 🔖️Diff
