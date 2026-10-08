//! 🔺️ Diff for `AddCompatibilityRule`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use semio_s_plugin_block::BlockCompatibilityDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::AddCompatibilityRule, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return protocol::MutationOutcome::new(Block3dDiff::default())
            .absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("{} \"{}\" already present", "compatibility-rule", payload.rule.id)).at(vec![payload.rule.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block3dDiff { compatibility: BlockCompatibilityDelta::insertion(semio_s_plugin_block::block_insert_index(base.compatibility.len(), payload.index), payload.rule.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
