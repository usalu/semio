//! 🔺️ Diff for `InsertBlock`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::InsertBlock, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if resolve_blocks(&base.blocks, &payload.path.segments).is_none_or(|blocks| payload.path.index > blocks.len()) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No block slot exists at index #{}.", payload.path.index), [payload.path.index.to_string()]);
    }
    let super::InsertBlock { path, block } = payload;
    protocol::MutationOutcome::new(wrap_body_diff(path, DocBlockLeaf::Inserted(block.clone())))
}
//#endregion 🔖️Diff
