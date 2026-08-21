//! 🔺️ Sparse diff builder for `CreateGrip` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::Block5dGripsDelta;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::CreateGrip, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.grips.iter().any(|item| item.id == payload.grip.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "grip", payload.grip.id), vec![payload.grip.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { grips: Some(Block5dGripsDelta { added: vec![payload.grip.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
