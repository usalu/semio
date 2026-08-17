//! 🔺️ Sparse diff builder for `CreateObject` — a real append-only insert. No-op when the id
//! already exists in `base`.
use crate::artifacts::puzzle3d::diff::{Puzzle3dDiff, Puzzle3dObjectsDelta};
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateObject, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.objects.iter().any(|entry| entry.id == payload.object.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "object"), vec![payload.object.id.clone()]);
    }
    let mut delta = Puzzle3dObjectsDelta { added: vec![payload.object.clone()], ..Default::default() };
    if let Some(index) = payload.index {
        let mut order: Vec<String> = base.objects.iter().map(|entry| entry.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, payload.object.id.clone());
        delta.reordered = Some(order);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff { objects: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
