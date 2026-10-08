//! 🔺️ Diff for `SetImageBlock`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetImageBlock, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if block_at(base, &payload.path).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No block exists at {:?}.", payload.path.segments), [payload.path.index.to_string()]);
    }
    let super::SetImageBlock { path, image_id, alt, width, height } = payload;
    protocol::MutationOutcome::new(match block_at(base, path) {
        Some(DocBlock::Image { image_id: old_id, alt: old_alt, width: old_width, height: old_height }) => {
            let row = DocImageBlockDiff { image_id: (old_id != image_id).then(|| image_id.clone()), alt: (old_alt != alt).then(|| alt.clone()), width: (old_width != width).then_some(*width), height: (old_height != height).then_some(*height) };
            if row == DocImageBlockDiff::default() {
                SemioDocumentDiff::default()
            } else {
                wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::Image(row)))
            }
        }
        _ => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
