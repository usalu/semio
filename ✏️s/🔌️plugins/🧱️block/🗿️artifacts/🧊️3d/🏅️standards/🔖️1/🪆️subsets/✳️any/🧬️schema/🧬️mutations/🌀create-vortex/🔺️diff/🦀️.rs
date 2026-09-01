//! 🔺️ Diff for `CreateVortex`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVorticesDelta};

//#region 🔖️Diff
pub async fn diff(payload: &super::CreateVortex, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.vortices.iter().any(|item| item.id == payload.vortex.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "vortex", payload.vortex.id), vec![payload.vortex.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { vortices: Some(Block3dVorticesDelta { added: vec![payload.vortex.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
