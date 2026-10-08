//! 🔺️ Diff for `RemoveBlock`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveBlock, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::RemoveBlock { path } = payload;
    protocol::MutationOutcome::new(wrap_body_diff(path, DocBlockLeaf::Removed))
}
//#endregion 🔖️Diff
