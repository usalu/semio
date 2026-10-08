//! 🔺️ Sparse diff builder for `CreateReference` — a real append-only insert. No-op when the id already
//! exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dDiff, Puzzle3dReferencesDelta};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateReference, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.references.iter().any(|entry| entry.id == payload.reference.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "reference"), vec![payload.reference.id.clone()]);
    }
    let reordered = payload.index.filter(|index| *index < base.references.len()).map(|index| {
        let mut order: Vec<String> = base.references.iter().map(|entry| entry.id.clone()).collect();
        order.insert(index, payload.reference.id.clone());
        order
    });
    let delta = Puzzle3dReferencesDelta::adding(payload.reference.clone(), reordered);
    protocol::MutationOutcome::new(Puzzle3dDiff { references: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
