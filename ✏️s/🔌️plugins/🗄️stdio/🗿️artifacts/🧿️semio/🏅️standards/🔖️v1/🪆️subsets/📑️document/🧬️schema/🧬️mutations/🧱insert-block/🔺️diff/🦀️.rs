//! 🔺️ Diff for `InsertBlock`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertBlock, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::InsertBlock { path, block } = payload;
    protocol::MutationOutcome::new(wrap_body_diff(path, DocBlockLeaf::Inserted(block.clone())))
}
//#endregion 🔖️Diff
