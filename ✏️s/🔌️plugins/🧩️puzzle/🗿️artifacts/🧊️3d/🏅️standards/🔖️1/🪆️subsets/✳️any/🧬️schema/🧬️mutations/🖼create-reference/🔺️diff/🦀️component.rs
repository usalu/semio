//! 🔺️ Sparse diff builder for `CreateReference` — a real append-only insert. No-op when the id already
//! exists in `base`.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dReferencesDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::CreateReference, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.references.iter().any(|entry| entry.id == payload.reference.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "reference"), vec![payload.reference.id.clone()]);
    }
    let mut delta = Puzzle3dReferencesDelta { added: vec![payload.reference.clone()], ..Default::default() };
    if let Some(index) = payload.index {
        let mut order: Vec<String> = base.references.iter().map(|entry| entry.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, payload.reference.id.clone());
        delta.reordered = Some(order);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff { references: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
