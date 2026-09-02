//! 📦️ `set-target-volume-flag` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

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
