//! 🔺️ Diff for `ScaleCamera3d`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::diff::Block3dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::ScaleCamera3d, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !payload.new_zoom.is_finite() || payload.new_zoom <= 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Camera zoom {} is not a finite positive number.", payload.new_zoom), ["camera3d"]);
    }
    if payload.new_zoom == base.camera3d.zoom {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Camera zoom is already {}.", payload.new_zoom));
    }
    protocol::MutationOutcome::new(Block3dDiff { camera3d: Some(semio_s_plugin_block::BlockCamera3dPatch { zoom: Some(payload.new_zoom), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
