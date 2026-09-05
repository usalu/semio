//! 🔺️ Diff for `AddCompatibilityRule`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::diff::{Block3dCompatibilityDelta, Block3dDiff};

//#region 🔖️Diff
pub fn diff(payload: &super::AddCompatibilityRule, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return protocol::MutationOutcome::new(Block3dDiff::default())
            .absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "compatibility-rule", payload.rule.id)).at(vec![payload.rule.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block3dDiff { compatibility: Some(Block3dCompatibilityDelta { added: vec![payload.rule.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
