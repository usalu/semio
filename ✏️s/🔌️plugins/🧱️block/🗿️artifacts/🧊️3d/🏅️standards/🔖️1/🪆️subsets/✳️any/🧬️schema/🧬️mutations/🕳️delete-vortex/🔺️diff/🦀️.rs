//! 🔺️ Diff for `DeleteVortex`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::{Block3dDiff, Block3dVorticesDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteVortex, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.vortices.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortex", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { vortices: Block3dVorticesDelta::removal(&base.vortices, base.vortices.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
