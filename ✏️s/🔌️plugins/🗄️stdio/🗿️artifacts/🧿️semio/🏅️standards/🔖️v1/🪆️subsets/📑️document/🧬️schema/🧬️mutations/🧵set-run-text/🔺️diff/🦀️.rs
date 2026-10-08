//! 🔺️ Diff for `SetRunText`.

use super::super::*;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::SetRunText, base: &SemioDocumentSnapshot) -> protocol::MutationOutcome<SemioDocumentDiff> {
    if block_at(base, &payload.path).is_none() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("No block exists at {:?}.", payload.path.segments), [payload.path.index.to_string()]);
    }
    let super::SetRunText { path, run_index, text } = payload;
    protocol::MutationOutcome::new({
        let Some(block) = block_at(base, path) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
        let Some(runs) = runs_of(block) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
        let Some(run) = runs.get(*run_index) else { return protocol::MutationOutcome::new(SemioDocumentDiff::default()) };
        if &run.text == text {
            return protocol::MutationOutcome::new(SemioDocumentDiff::default());
        }
        let rd: RunsDiff = IndexedTripleDiff { modified: vec![IndexModified { index: *run_index, diff: DocRunDiff { text: Some(text.clone()), style: None } }], ..Default::default() };
        match wrap_runs_diff(block, rd) {
            Some(bd) => wrap_body_diff(path, DocBlockLeaf::Modified(bd)),
            None => SemioDocumentDiff::default(),
        }
    })
}
//#endregion 🔖️Diff
