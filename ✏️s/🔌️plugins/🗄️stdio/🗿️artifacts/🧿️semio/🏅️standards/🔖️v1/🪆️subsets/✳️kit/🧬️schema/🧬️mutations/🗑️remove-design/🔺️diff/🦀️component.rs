//! 🔺️ `remove-design` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::RemoveDesign;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDesignList, SemioKitDiff};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &RemoveDesign, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if !base.designs.iter().any(|d| d.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Design \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    }
    let designs: Vec<_> = base.designs.iter().filter(|d| d.id != payload.id).cloned().collect();
    protocol::MutationOutcome::new(SemioKitDiff { designs: Some(SemioKitDesignList { values: designs }), ..Default::default() })
}
//#endregion 🔖️Diff
