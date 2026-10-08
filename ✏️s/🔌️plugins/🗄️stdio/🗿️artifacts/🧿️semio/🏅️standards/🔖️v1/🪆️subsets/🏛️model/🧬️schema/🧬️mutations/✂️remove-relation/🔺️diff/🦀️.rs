//! 🔺️ Diff for `RemoveRelation`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveRelation, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    let super::RemoveRelation { id } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { relations: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
