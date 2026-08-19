//! 🔺️ Sparse diff builder for `RenamePuzzle5d` — patches the document `label`.
use crate::artifacts::puzzle5d::diff::Puzzle5dDiff;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::RenamePuzzle5d, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    // 🏷️ `label` is a document-root singleton field, the closest thing this fixture has to an
    // identity — not a catalog member addressed by id, so there is no missing-target or duplicate-id
    // case, only the no-op check applies.
    if payload.new_label == base.label {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Label is unchanged.");
    }
    protocol::MutationOutcome::new(Puzzle5dDiff { label: Some(payload.new_label.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
