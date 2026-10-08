//! 🔺️ Diff for `AddAttribute`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block2dDiff;
use semio_s_plugin_block::BlockAttributesDelta;

//#region 🔖️Diff
pub fn diff(payload: &super::AddAttribute, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.attributes.iter().any(|item| item.key == payload.attribute.key) {
        return protocol::MutationOutcome::new(Block2dDiff::default())
            .absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("{} \"{}\" already present", "attribute", payload.attribute.key)).at(vec![payload.attribute.key.clone()])]);
    }
    protocol::MutationOutcome::new(Block2dDiff { attributes: BlockAttributesDelta::insertion(semio_s_plugin_block::block_insert_index(base.attributes.len(), payload.index), payload.attribute.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
