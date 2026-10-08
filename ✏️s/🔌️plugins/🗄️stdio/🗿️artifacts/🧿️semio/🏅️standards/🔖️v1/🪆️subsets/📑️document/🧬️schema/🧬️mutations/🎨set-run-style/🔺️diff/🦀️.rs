//! 🔺️ Diff for `SetRunStyle`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetRunStyle, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    let super::SetRunStyle { path, run_index, style } = payload;
    protocol::MutationOutcome::new({
        let Some(block) = block_at(base, path) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
        let Some(runs) = runs_of(block) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
        let Some(run) = runs.get(*run_index) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
        if &run.style == style {
            return protocol::MutationOutcome::new(SemioDocumentDiff::default());
        }
        let style_diff = crate::standards::v1::subsets::document::schema::diff::RunStyleDiff {
            bold: Some(style.bold),
            italic: Some(style.italic),
            underline: Some(style.underline),
            size: Some(style.size),
            font: Some(style.font.clone()),
            color: Some(style.color.clone()),
            link: Some(style.link.clone()),
        };
        let rd: RunsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *run_index, diff: DocRunDiff { text: None, style: Some(style_diff) } }], ..Default::default() };
        match wrap_runs_diff(block, rd) {
            Some(bd) => wrap_body_diff(path, DocBlockLeaf::Modified(bd)),
            None => SemioDocumentDiff::default(),
        }
    })
}
//#endregion 🔖️Diff
