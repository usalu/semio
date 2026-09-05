//! 🔺️ Diff for `RemoveCompatibilityRule`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dCompatibilityDelta, Block5dDiff};

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveCompatibilityRule, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.compatibility.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "compatibility-rule", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { compatibility: Some(Block5dCompatibilityDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
