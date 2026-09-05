//! 🔺️ Diff for `RemoveCompatibilityRule`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::{Block2dCompatibilityDelta, Block2dDiff};

//#region 🔖️Diff
pub fn diff(payload: &super::RemoveCompatibilityRule, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !base.compatibility.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "compatibility-rule", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block2dDiff { compatibility: Some(Block2dCompatibilityDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
