//! 🔺️ Sparse diff builder for `MoveCamera3d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockCamera3d};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::MoveCamera3d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_position.iter().any(|c| !c.is_finite()) || payload.new_target.iter().any(|c| !c.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Camera position {:?} / target {:?} is not finite.", payload.new_position, payload.new_target), ["camera3d"]);
    }
    if payload.new_position == base.camera3d.position && payload.new_target == base.camera3d.target {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Camera is already at {:?}.", payload.new_position));
    }
    protocol::MutationOutcome::new(Block5dDiff { camera3d: Some(BlockCamera3d { position: payload.new_position, target: payload.new_target, ..base.camera3d.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
