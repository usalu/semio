//! 🔺️ Sparse diff builder for `ChangeVortexVortexKind` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dVorticesDelta, Block3dVorticesPatch, Block3dVorticesPatchEntry};
use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::Block3dVortexTemplate;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::ChangeVortexVortexKind, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    let Some(existing) = base.vortices.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortex", payload.id), vec![payload.id.clone()]);
    };
    let replacement = Block3dVortexTemplate { vortex_kind: payload.new_vortex_kind.clone(), ..existing.clone() };
    if replacement == *existing {
        return protocol::MutationOutcome::new(Block3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block3dDiff {
        vortices: Some(Block3dVorticesDelta { patched: vec![Block3dVorticesPatchEntry { id: payload.id.clone(), patch: Block3dVorticesPatch { replacement: Some(replacement) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
