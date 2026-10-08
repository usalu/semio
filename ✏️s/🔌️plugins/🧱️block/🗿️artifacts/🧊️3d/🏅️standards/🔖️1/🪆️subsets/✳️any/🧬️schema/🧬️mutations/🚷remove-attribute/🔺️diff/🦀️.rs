//! 🔺️ Diff for `RemoveAttribute`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use semio_s_plugin_block::BlockAttributesDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveAttribute, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.attributes.iter().any(|item| item.key == payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "attribute", payload.key), vec![payload.key.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { attributes: BlockAttributesDelta::removal(&base.attributes, base.attributes.iter().position(|item| item.key == payload.key).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
