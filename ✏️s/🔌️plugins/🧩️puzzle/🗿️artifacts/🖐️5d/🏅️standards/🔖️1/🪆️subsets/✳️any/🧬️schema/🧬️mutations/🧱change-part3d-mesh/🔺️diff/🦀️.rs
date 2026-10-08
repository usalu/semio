//! 🔺️ Sparse diff builder for `ChangePart3dMesh` — patches the one addressed part in place.
use crate::standards::v1::subsets::any::schema::diff::{ItemPatch, Puzzle5dDiff, Puzzle5dPart3dPatch, Puzzle5dPartPatch, Puzzle5dPartsDelta};
use crate::Puzzle5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::ChangePart3dMesh, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "part", payload.id), vec![payload.id.clone()]);
    };
    let patch = Puzzle5dPartPatch {
        part_3d: Some(Puzzle5dPart3dPatch { mesh_url: (payload.new_mesh_url != item.part_3d.mesh_url).then(|| payload.new_mesh_url.clone()), ..Default::default() }).filter(|nested| !nested.is_empty()),
        ..Default::default()
    };
    if patch.is_empty() {
        return protocol::MutationOutcome::new(Puzzle5dDiff::default()).absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Puzzle5dDiff {
        parts: Some(Puzzle5dPartsDelta::patching(payload.id.clone(), patch)),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
