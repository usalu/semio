//! 🔺️ Sparse diff builder for `UpdatePart3d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::{Block5dPart3d};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdatePart3d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    protocol::MutationOutcome::new(Block5dDiff { part_3d: Some(Block5dPart3d { orientation: payload.new_orientation, scale: payload.new_scale }), ..Default::default() })
}
//#endregion 🔖️Diff
