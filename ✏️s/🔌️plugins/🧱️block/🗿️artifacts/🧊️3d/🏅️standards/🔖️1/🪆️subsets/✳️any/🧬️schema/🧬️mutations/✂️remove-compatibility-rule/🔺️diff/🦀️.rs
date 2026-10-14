//! 🔺️ Diff for `RemoveCompatibilityRule`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;
use semio_s_plugin_block::BlockCompatibilityDelta;

//#region 🔖️Diff
pub fn diff(payload: &crate::remove_compatibility_rule::RemoveCompatibilityRule, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.compatibility.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "compatibility-rule", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { compatibility: BlockCompatibilityDelta::removal(&base.compatibility, base.compatibility.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
