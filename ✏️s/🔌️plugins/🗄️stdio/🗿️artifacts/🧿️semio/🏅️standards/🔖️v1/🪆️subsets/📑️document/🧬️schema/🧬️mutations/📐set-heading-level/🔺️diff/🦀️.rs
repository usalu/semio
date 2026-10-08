//! 🔺️ Diff for `SetHeadingLevel`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetHeadingLevel, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::SetHeadingLevel { path, level } = payload;
    protocol::MutationOutcome::new(match block_at(base, path) {
        Some(DocBlock::Heading { level: old, .. }) if old != level => wrap_body_diff(path, DocBlockLeaf::Modified(DocBlockDiff::Heading(DocHeadingDiff { level: Some(*level), style_id: None, runs: None }))),
        _ => SemioDocumentDiff::default(),
    })
}
//#endregion 🔖️Diff
