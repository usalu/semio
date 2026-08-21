//! 🔺️ Sparse diff builder for `AddCompatibilityRule` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dCompatibilityDelta;
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::AddCompatibilityRule, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return protocol::MutationOutcome::new(Block5dDiff::default())
            .absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "compatibility-rule", payload.rule.id)).at(vec![payload.rule.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block5dDiff { compatibility: Some(Block5dCompatibilityDelta { added: vec![payload.rule.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
