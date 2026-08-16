//! 🔺️ Sparse diff builder for `RenamePuzzle5d` — patches the document `label`.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RenamePuzzle5d, _base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    protocol::MutationOutcome::new(Puzzle5dDiff { label: Some(payload.new_label.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
