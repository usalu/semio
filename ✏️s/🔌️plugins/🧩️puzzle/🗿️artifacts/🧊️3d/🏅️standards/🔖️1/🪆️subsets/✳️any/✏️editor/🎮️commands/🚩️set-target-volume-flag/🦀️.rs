//! 📦️ `set-target-volume-flag` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::standards::v1::subsets::any::schema::mutations::{change_target_volume_hidden, change_target_volume_locked};
use semio_framework_pack_json::Value;

/// 🚩️ One `change-target-volume-hidden` or `change-target-volume-locked` when the named volume's flag differs.
pub fn set_target_volume_flag(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
    let Some(volume) = ctx.base.target_volumes.iter().find(|volume| volume.id == id) else { return };
    let mutation = match flag {
        "hidden" if volume.hidden != value => change_target_volume_hidden(id.to_string(), value),
        "locked" if volume.locked != value => change_target_volume_locked(id.to_string(), value),
        _ => return,
    };
    ctx.artifact_mutations.push(mutation);
}
