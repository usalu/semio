//! 🔺️ Sparse diff builder for `DisconnectVortices` — a real removal, never a whole-snapshot capture.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dAttractionsDelta, Puzzle3dDiff};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DisconnectVortices, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(at) = base.attractions.iter().position(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortices", payload.id), vec![payload.id.clone()]);
    };
    protocol::MutationOutcome::new(Puzzle3dDiff { attractions: Some(Puzzle3dAttractionsDelta::removal_by_id(payload.id.clone(), at)), ..Default::default() })
}
//#endregion 🔖️Diff
