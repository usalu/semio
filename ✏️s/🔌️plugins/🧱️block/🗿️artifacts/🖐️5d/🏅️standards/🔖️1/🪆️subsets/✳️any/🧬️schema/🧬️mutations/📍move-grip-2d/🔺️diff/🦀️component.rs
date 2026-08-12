//! 🔺️ Sparse diff builder for `MoveGrip2d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dGripsDelta, Block5dGripsPatch, Block5dGripsPatchEntry};
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::{Block5dGripTemplate};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::MoveGrip2d, base: &Block5dSnapshot) -> Block5dDiff {
    let Some(existing) = base.grips.iter().find(|item| item.id == payload.id) else { return Block5dDiff::default(); };
    let replacement = Block5dGripTemplate { angle: payload.new_angle, radius_2d: payload.new_radius_2d, ..existing.clone() };
    Block5dDiff { grips: Some(Block5dGripsDelta { patched: vec![Block5dGripsPatchEntry { id: payload.id.clone(), patch: Block5dGripsPatch { replacement: Some(replacement) } }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️Diff
