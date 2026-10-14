//! 🔺️ Diff for `RemoveAttribute`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;
use semio_s_plugin_block::BlockAttributesDelta;

//#region 🔖️Diff
pub fn diff(payload: &crate::remove_attribute::RemoveAttribute, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.attributes.iter().any(|item| item.key == payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "attribute", payload.key), vec![payload.key.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { attributes: BlockAttributesDelta::removal(&base.attributes, base.attributes.iter().position(|item| item.key == payload.key).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
