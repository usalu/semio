//! 🔺️ Sparse diff builder for `CreatePart` — a real append-only insert. No-op when the id already
//! exists in `base`.
use crate::artifacts::puzzle5d::diff::{Puzzle5dDiff, Puzzle5dPartsDelta};
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreatePart, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    if base.parts.iter().any(|entry| entry.id == payload.part.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "part"), vec![payload.part.id.clone()]);
    }
    let mut delta = Puzzle5dPartsDelta { added: vec![payload.part.clone()], ..Default::default() };
    if let Some(index) = payload.index {
        let mut order: Vec<String> = base.parts.iter().map(|entry| entry.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, payload.part.id.clone());
        delta.reordered = Some(order);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff { parts: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
