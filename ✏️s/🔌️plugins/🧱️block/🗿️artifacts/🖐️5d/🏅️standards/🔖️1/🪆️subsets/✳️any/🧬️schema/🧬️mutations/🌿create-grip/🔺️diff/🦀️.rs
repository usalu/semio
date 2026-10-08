//! 🔺️ Diff for `CreateGrip`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::{Block5dDiff, Block5dGripsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateGrip, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.grips.iter().any(|item| item.id == payload.grip.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "grip", payload.grip.id), vec![payload.grip.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { grips: Block5dGripsDelta::insertion(semio_s_plugin_block::block_insert_index(base.grips.len(), payload.index), payload.grip.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
