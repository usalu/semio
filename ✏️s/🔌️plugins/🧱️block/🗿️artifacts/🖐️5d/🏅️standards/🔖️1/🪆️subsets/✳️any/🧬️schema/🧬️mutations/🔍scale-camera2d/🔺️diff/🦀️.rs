//! 🔺️ Diff for `ScaleCamera2d`.

use crate::BlockCamera2d;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ScaleCamera2d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !payload.new_zoom.is_finite() || payload.new_zoom <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Camera zoom {} is not a finite positive number.", payload.new_zoom), ["camera2d"]);
    }
    if payload.new_zoom == base.camera2d.zoom {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Camera zoom is already {}.", payload.new_zoom));
    }
    protocol::MutationOutcome::new(Block5dDiff { camera2d: Some(BlockCamera2d { zoom: payload.new_zoom, ..base.camera2d.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
