//! 🔺️ Sparse diff builder for `CreatePart` — a real append-only insert. No-op when the id already
//! exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle5dDiff, Puzzle5dPartsDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::CreatePart, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if base.parts.iter().any(|entry| entry.id == payload.part.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "part"), vec![payload.part.id.clone()]);
    }
    let reordered = payload.index.filter(|index| *index < base.parts.len()).map(|index| {
        let mut order: Vec<String> = base.parts.iter().map(|entry| entry.id.clone()).collect();
        order.insert(index, payload.part.id.clone());
        order
    });
    let delta = Puzzle5dPartsDelta::adding(payload.part.clone(), reordered);
    protocol::MutationOutcome::new(Puzzle5dDiff { parts: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
