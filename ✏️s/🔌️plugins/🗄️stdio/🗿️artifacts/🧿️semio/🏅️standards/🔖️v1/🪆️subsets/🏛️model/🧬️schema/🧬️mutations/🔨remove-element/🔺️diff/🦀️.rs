//! 🔺️ Diff for `RemoveElement`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveElement, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    let super::RemoveElement { id } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { elements: Some(NamedTripleDiff { removed: vec![id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
