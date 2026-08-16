//! 🔺️ Sparse diff builder for `AddCompatibilityRule` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dCompatibilityDelta};
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::AddCompatibilityRule, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return protocol::MutationOutcome::new(Block3dDiff::default()).absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "compatibility-rule", payload.rule.id)).at(vec![payload.rule.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block3dDiff { compatibility: Some(Block3dCompatibilityDelta { added: vec![payload.rule.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
