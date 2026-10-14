//! 🔺️ Diff for `RemoveCompatibilityRule`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;
use semio_s_plugin_block::BlockCompatibilityDelta;

//#region 🔖️Diff
pub fn diff(payload: &crate::remove_compatibility_rule::RemoveCompatibilityRule, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.compatibility.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "compatibility-rule", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { compatibility: BlockCompatibilityDelta::removal(&base.compatibility, base.compatibility.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
