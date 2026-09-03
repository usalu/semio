//! 🔺️ Diff for `AddAttribute`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::{Block2dAttributesDelta, Block2dDiff};

//#region 🔖️Diff
pub async fn diff(payload: &super::AddAttribute, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.attributes.iter().any(|item| item.key == payload.attribute.key) {
        return protocol::MutationOutcome::new(Block2dDiff::default())
            .absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "attribute", payload.attribute.key)).at(vec![payload.attribute.key.clone()])]);
    }
    protocol::MutationOutcome::new(Block2dDiff { attributes: Some(Block2dAttributesDelta { added: vec![payload.attribute.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
