//! 🔺️ Sparse diff builder for `CreatePart` — a real append-only insert. No-op when the id already
//! exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dPartsDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreatePart, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if base.parts.iter().any(|entry| entry.id == payload.part.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "part"), vec![payload.part.id.clone()]);
    }
    let index = payload.index.map_or(base.parts.len(), |index| index.min(base.parts.len()));
    let delta = Puzzle5dPartsDelta::insertion(index, payload.part.clone());
    protocol::MutationOutcome::new(Puzzle5dDiff { parts: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
