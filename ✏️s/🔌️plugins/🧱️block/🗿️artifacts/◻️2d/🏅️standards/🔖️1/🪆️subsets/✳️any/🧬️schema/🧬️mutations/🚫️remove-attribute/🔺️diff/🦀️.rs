//! 🔺️ Diff for `RemoveAttribute`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::{Block2dAttributesDelta, Block2dDiff};

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveAttribute, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.attributes.iter().any(|item| item.key == payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "attribute", payload.key), vec![payload.key.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { attributes: Some(Block2dAttributesDelta { removed: vec![payload.key.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
