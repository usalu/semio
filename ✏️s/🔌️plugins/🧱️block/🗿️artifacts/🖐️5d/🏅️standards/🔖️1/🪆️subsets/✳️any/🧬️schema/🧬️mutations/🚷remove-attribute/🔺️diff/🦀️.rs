//! 🔺️ Diff for `RemoveAttribute`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;
use semio_s_plugin_block::BlockAttributesDelta;

//#region 🔖️Diff
pub fn diff(payload: &crate::remove_attribute::RemoveAttribute, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.attributes.iter().any(|item| item.key == payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "attribute", payload.key), vec![payload.key.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { attributes: BlockAttributesDelta::removal(&base.attributes, base.attributes.iter().position(|item| item.key == payload.key).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
