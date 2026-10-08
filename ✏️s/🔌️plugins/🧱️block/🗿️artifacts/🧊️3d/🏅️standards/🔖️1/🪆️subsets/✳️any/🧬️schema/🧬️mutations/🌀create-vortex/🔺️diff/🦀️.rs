//! 🔺️ Diff for `CreateVortex`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::{Block3dDiff, Block3dVorticesDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateVortex, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.vortices.iter().any(|item| item.id == payload.vortex.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "vortex", payload.vortex.id), vec![payload.vortex.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { vortices: Some(Block3dVorticesDelta { added: vec![payload.vortex.clone()], reordered: semio_s_plugin_block::block_insert_order(base.vortices.iter().map(|item| item.id.as_str()), &payload.vortex.id, payload.index), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
