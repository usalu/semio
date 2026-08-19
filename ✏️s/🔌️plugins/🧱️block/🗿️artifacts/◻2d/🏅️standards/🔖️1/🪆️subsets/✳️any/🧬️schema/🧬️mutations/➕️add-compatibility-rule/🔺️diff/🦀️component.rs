//! 🔺️ Sparse diff builder for `AddCompatibilityRule` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::diff::{Block2dCompatibilityDelta};
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::AddCompatibilityRule, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return protocol::MutationOutcome::new(Block2dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "compatibility-rule", payload.rule.id)).at(vec![payload.rule.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block2dDiff { compatibility: Some(Block2dCompatibilityDelta { added: vec![payload.rule.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
