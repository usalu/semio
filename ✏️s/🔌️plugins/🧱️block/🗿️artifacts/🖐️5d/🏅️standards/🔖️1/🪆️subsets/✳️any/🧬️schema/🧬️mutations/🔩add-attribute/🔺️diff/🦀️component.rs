//! 🔺️ Sparse diff builder for `AddAttribute` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dAttributesDelta;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::AddAttribute, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.attributes.iter().any(|item| item.key == payload.attribute.key) {
        return protocol::MutationOutcome::new(Block5dDiff::default())
            .absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "attribute", payload.attribute.key)).at(vec![payload.attribute.key.clone()])]);
    }
    protocol::MutationOutcome::new(Block5dDiff { attributes: Some(Block5dAttributesDelta { added: vec![payload.attribute.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
