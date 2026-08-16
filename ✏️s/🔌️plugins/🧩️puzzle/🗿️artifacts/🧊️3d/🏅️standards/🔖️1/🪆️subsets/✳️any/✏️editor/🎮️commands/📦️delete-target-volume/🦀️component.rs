//! 📦️ `delete-target-volume` command.

use crate::editor::puzzle3d::{value_as_vec3, Puzzle3dActionCtx, Puzzle3dTargetVolume, PUZZLE3D_ID_COUNTER};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;

pub fn delete_target_volume(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
        ctx.scene.fixture.target_volumes.retain(|volume| volume.id != id);
    }
}
