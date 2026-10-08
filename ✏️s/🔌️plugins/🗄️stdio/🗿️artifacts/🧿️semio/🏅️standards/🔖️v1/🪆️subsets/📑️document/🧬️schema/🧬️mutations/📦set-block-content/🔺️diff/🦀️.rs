//! 🔺️ Diff for `SetBlockContent`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetBlockContent, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::SetBlockContent { path, block } = payload;
    protocol::MutationOutcome::new(match block_at(base, path) {
        Some(old) => match diff_block(old, block) {
            Some(d) => wrap_body_diff(path, DocBlockLeaf::Modified(d)),
            None => SemioDocumentDiff::default(),
        },
        None => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
