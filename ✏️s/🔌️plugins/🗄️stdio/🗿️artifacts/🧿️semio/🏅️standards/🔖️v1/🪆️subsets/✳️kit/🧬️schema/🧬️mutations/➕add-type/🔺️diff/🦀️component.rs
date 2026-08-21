//! 🔺️ `add-type` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::AddType;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDiff, SemioKitTypeList};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &AddType, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    if base.types.iter().any(|t| t.id == payload.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("A type with id \"{}\" already exists.", payload.id), [payload.id.clone()]);
    }
    let mut types = base.types.clone();
    types.push(SemioKitType { id: payload.id.clone(), name: payload.name.clone(), category: payload.category.clone() });
    protocol::MutationOutcome::new(SemioKitDiff { types: Some(SemioKitTypeList { values: types }), ..Default::default() })
}
//#endregion 🔖️Diff
