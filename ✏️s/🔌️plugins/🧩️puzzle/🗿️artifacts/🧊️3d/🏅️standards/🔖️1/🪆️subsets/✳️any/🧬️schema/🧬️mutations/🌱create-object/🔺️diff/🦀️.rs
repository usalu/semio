//! 🔺️ Sparse diff builder for `CreateObject` — a real append-only insert. No-op when the id
//! already exists in `base`.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dDiff, Puzzle3dObjectsDelta};
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::CreateObject, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    if base.objects.iter().any(|entry| entry.id == payload.object.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} already exists", "object"), vec![payload.object.id.clone()]);
    }
    let reordered = payload.index.filter(|index| *index < base.objects.len()).map(|index| {
        let mut order: Vec<String> = base.objects.iter().map(|entry| entry.id.clone()).collect();
        order.insert(index, payload.object.id.clone());
        order
    });
    let delta = Puzzle3dObjectsDelta::adding(payload.object.clone(), reordered);
    protocol::MutationOutcome::new(Puzzle3dDiff { objects: Some(delta), ..Default::default() })
}
//#endregion 🔖️Diff
