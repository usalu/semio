//! 🚩️ `set-target-volume-flag` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 🚩️ The outliner's show/hide and lock/unlock toggles for one target volume. An unknown flag name
/// writes nothing, so a stale row action cannot corrupt the other flag.
pub fn set_target_volume_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let id = args.and_then(|value| value.get("id")).and_then(Value::as_str).unwrap_or("");
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(Value::as_bool).unwrap_or(false);
    if let Some(volume) = ctx.scene.document.target_volumes.iter_mut().find(|volume| volume.id == id) {
        match flag {
            "hidden" => volume.hidden = value,
            "locked" => volume.locked = value,
            _ => {}
        }
    }
}
