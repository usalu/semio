//! 🔺️ Diff for `CreateGrip`.

use crate::artifacts::block5d::{Block5dGripTemplate, Block5dSnapshot};
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripsDelta};

//#region 🔖️Diff
pub async fn diff(payload: &super::CreateGrip, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.grips.iter().any(|item| item.id == payload.grip.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "grip", payload.grip.id), vec![payload.grip.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { grips: Some(Block5dGripsDelta { added: vec![payload.grip.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
