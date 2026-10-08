//! 🔺️ Diff for `InsertElement`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertElement, base: &SemioModelSnapshot) -> protocol::MutationOutcome<SemioModelDiff> {
    let super::InsertElement { element } = payload;
    protocol::MutationOutcome::new(SemioModelDiff { elements: Some(NamedTripleDiff { added: vec![element.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
