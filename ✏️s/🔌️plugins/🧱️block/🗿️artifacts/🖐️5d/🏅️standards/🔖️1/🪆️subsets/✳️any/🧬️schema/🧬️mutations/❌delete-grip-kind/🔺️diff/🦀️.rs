//! 🔺️ Diff for `DeleteGripKind`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripKindsDelta};

//#region 🔖️Diff
pub async fn diff(payload: &super::DeleteGripKind, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.grip_kinds.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip-kind", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { grip_kinds: Some(Block5dGripKindsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
