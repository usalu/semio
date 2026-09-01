//! 🔺️ Diff for `CreateVortexKind`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVortexKindsDelta};

//#region 🔖️Diff
pub async fn diff(payload: &super::CreateVortexKind, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if crate::artifacts::block3d::vortex_kinds_of(base).iter().any(|item| item.id == payload.vortex_kind.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "vortex-kind", payload.vortex_kind.id), vec![payload.vortex_kind.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { vortex_kinds: Some(Block3dVortexKindsDelta { added: vec![payload.vortex_kind.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
