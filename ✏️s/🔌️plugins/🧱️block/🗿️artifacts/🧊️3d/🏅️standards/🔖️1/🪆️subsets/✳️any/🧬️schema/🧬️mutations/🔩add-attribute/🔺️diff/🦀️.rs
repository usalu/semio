//! 🔺️ Diff for `AddAttribute`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dAttributesDelta, Block3dDiff};

//#region 🔖️Diff
pub async fn diff(payload: &super::AddAttribute, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.attributes.iter().any(|item| item.key == payload.attribute.key) {
        return protocol::MutationOutcome::new(Block3dDiff::default())
            .absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "attribute", payload.attribute.key)).at(vec![payload.attribute.key.clone()])]);
    }
    protocol::MutationOutcome::new(Block3dDiff { attributes: Some(Block3dAttributesDelta { added: vec![payload.attribute.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
