//! 🔺️ Diff for `CreateGripKind`.

use crate::artifacts::block5d::{Block5dGripKind, Block5dSnapshot};
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripKindsDelta};

//#region 🔖️Diff
pub async fn diff(payload: &super::CreateGripKind, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.grip_kinds.iter().any(|item| item.id == payload.grip_kind.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "grip-kind", payload.grip_kind.id), vec![payload.grip_kind.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { grip_kinds: Some(Block5dGripKindsDelta { added: vec![payload.grip_kind.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
