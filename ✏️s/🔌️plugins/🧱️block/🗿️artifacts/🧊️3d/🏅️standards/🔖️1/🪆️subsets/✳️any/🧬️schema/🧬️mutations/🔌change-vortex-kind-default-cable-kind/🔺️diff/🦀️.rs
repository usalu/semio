//! 🔺️ Diff for `ChangeVortexKindDefaultCableKind`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVortexKindsDelta, Block3dVortexKindsPatch, Block3dVortexKindsPatchEntry};

//#region 🔖️Diff
pub async fn diff(payload: &super::ChangeVortexKindDefaultCableKind, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    let current = crate::artifacts::block3d::vortex_kinds_of(base);
    let Some(existing) = current.iter().find(|item| item.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortex-kind", payload.id), vec![payload.id.clone()]);
    };
    let replacement = Block3dVortexKind { default_cable_kind: payload.new_default_cable_kind.clone(), ..existing.clone() };
    if replacement == *existing {
        return protocol::MutationOutcome::new(Block3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "no changes to apply").at(vec![payload.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block3dDiff {
        vortex_kinds: Some(Block3dVortexKindsDelta { patched: vec![Block3dVortexKindsPatchEntry { id: payload.id.clone(), patch: Block3dVortexKindsPatch { replacement: Some(replacement) } }], ..Default::default() }),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
