//! 🔺️ Sparse diff builder for `DeleteReference` — a real removal, never a whole-snapshot capture.
use crate::artifacts::puzzle3d::diff::{Puzzle3dReferencesDelta, Puzzle3dDiff};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteReference, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if !base.references.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "reference", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff { references: Some(Puzzle3dReferencesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
