//! 🔺️ Diff for `SetParagraphStyle`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetParagraphStyle, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::SetParagraphStyle { path, style_id } = payload;
    protocol::MutationOutcome::new(match block_at(base, path) {
        Some(DocBlock::Paragraph { style_id: old, .. }) if old != style_id => wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::Paragraph(DocParagraphDiff { style_id: Some(style_id.clone()), runs: None }))),
        _ => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
