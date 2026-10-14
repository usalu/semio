//! 🔺️ Sparse diff builder for `ChangeObjectKind` — patches the one addressed object in place.
use crate::standards::v1::subsets::any::schema::diff::{Puzzle3dDiff, Puzzle3dObjectPatch, Puzzle3dObjectsDelta};
use protocol::list_delta::RowPatch;
use crate::Puzzle3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ChangeObjectKind, base: &Puzzle3dSnapshot) -> protocol::MutationOutcome<Puzzle3dDiff> {
    let Some(item) = base.objects.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "object", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle3dObjectPatch {
        object_kind: (payload.new_object_kind != item.object_kind).then(|| payload.new_object_kind.clone()),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle3dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle3dDiff {
        objects: Some(Puzzle3dObjectsDelta::modification(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
