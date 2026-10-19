//! 🔺️ Sparse diff builder for `RenamePuzzle5d` — patches the document `label`.
use crate::standards::v1::subsets::any::schema::diff::Puzzle5dDiff;
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::RenamePuzzle5d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if payload.new_label == base.label {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Label is unchanged.");
    }
    protocol::MutationOutcome::new(Puzzle5dDiff { label: Some(payload.new_label.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
