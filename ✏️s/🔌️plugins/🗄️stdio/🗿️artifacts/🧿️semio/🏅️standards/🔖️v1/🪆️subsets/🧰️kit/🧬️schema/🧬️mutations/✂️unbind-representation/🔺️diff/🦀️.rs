//! 🔺️ Diff for `UnbindRepresentation`.

use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitLinkList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::UnbindRepresentation, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if payload.index >= base.representations.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No representation binding exists at index #{}.", payload.index), [payload.index.to_string()]);
    }
    let mut representations = base.representations.clone();
    representations.remove(payload.index);
    protocol::MutationOutcome::new(SemioKitDiff { representations: Some(SemioKitLinkList { values: representations }), ..Default::default() })
}
//#endregion 🔖️Diff
