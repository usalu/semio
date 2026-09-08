//! 🔺️ Diff for `AddCompatibilityRule`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::{Block5dCompatibilityDelta, Block5dDiff};

//#region 🔖️Diff
pub fn diff(payload: &super::AddCompatibilityRule, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return protocol::MutationOutcome::new(Block5dDiff::default())
            .absorb_messages([protocol::MutationMessage::warn("mutation.no-op", format!("{} \"{}\" already present", "compatibility-rule", payload.rule.id)).at(vec![payload.rule.id.clone()])]);
    }
    protocol::MutationOutcome::new(Block5dDiff { compatibility: Some(Block5dCompatibilityDelta { added: vec![payload.rule.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
