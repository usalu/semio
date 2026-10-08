//! 🔺️ Diff for `AddAttribute`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;
use semio_s_plugin_block::BlockAttributesDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::AddAttribute, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.attributes.iter().any(|item| item.key == payload.attribute.key) {
        return protocol::MutationOutcome::new(Block3dDiff::default())
            .absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("{} \"{}\" already present", "attribute", payload.attribute.key)).at(vec![payload.attribute.key.clone()])]);
    }
    protocol::MutationOutcome::new(Block3dDiff { attributes: Some(BlockAttributesDelta { added: vec![payload.attribute.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
