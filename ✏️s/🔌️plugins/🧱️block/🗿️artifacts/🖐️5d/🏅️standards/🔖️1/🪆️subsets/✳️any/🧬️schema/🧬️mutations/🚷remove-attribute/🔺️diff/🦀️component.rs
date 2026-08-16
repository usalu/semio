//! 🔺️ Sparse diff builder for `RemoveAttribute` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dAttributesDelta};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RemoveAttribute, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.attributes.iter().any(|item| item.key == payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "attribute", payload.key), vec![payload.key.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { attributes: Some(Block5dAttributesDelta { removed: vec![payload.key.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
