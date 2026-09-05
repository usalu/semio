//! 🔺️ Diff for `DeleteVortex`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dDiff, Block3dVorticesDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteVortex, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.vortices.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortex", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { vortices: Some(Block3dVorticesDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
