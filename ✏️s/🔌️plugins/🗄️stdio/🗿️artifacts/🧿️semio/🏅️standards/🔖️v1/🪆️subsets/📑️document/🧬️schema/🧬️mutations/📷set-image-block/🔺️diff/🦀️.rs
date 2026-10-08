//! 🔺️ Diff for `SetImageBlock`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetImageBlock, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::SetImageBlock { path, image_id, alt, width, height } = payload;
    protocol::MutationOutcome::new(match block_at(base, path) {
        Some(old @ DocBlock::Image { .. }) => {
            let new = DocBlock::Image { image_id: image_id.clone(), alt: alt.clone(), width: *width, height: *height };
            match diff_block(old, &new) {
                Some(d) => wrap_body_diff(path, DocBlockLeaf::Modified(d)),
                None => SemioDocumentDiff::default(),
            }
        }
        _ => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
