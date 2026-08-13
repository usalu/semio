//! 📦️ `set-target-volume-flag` command.

use crate::apps::puzzle3d::{value_as_vec3, Puzzle3dActionCtx, Puzzle3dTargetVolume, PUZZLE3D_ID_COUNTER};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;

pub fn set_target_volume_flag(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
    if let Some(volume) = ctx.scene.fixture.target_volumes.iter_mut().find(|volume| volume.id == id) {
        match flag {
            "hidden" => volume.hidden = value,
            "locked" => volume.locked = value,
            _ => {}
        }
    }
}
