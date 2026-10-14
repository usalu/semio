//! 🔺️ Diff for `AddCompatibilityRule`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;
use semio_s_plugin_block::BlockCompatibilityDelta;

//#region 🔖️Diff
pub fn diff(payload: &crate::add_compatibility_rule::AddCompatibilityRule, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return protocol::MutationOutcome::new(Block5dDiff::default())
            .absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("{} \"{}\" already present", "compatibility-rule", payload.rule.id)).at(vec![payload.rule.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block5dDiff { compatibility: BlockCompatibilityDelta::insertion(semio_s_plugin_block::block_insert_index(base.compatibility.len(), payload.index), payload.rule.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
