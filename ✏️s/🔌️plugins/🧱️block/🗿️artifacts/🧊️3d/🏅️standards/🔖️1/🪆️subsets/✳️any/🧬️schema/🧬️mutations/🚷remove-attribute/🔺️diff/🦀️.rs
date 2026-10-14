//! 🔺️ Diff for `RemoveAttribute`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;
use semio_s_plugin_block::BlockAttributesDelta;

//#region 🔖️Diff
pub fn diff(payload: &crate::remove_attribute::RemoveAttribute, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.attributes.iter().any(|item| item.key == payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "attribute", payload.key), vec![payload.key.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { attributes: BlockAttributesDelta::removal(&base.attributes, base.attributes.iter().position(|item| item.key == payload.key).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
