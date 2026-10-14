//! 🔺️ Sparse diff builder for `DeleteReference` — a real removal, never a whole-snapshot capture.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dDiff, Puzzle3dReferencesDelta};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DeleteReference, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(at) = base.references.iter().position(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "reference", payload.id), vec![payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Puzzle3dDiff { references: Some(Puzzle3dReferencesDelta::removal_by_id(payload.id.clone(), at)), ..Default::default() })
}
//#endregion 🔖️Diff
