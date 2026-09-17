//! 🪦️ `delete-target-volume` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 🪦️ Drops one target volume by id. A locked volume is still deletable — `locked` gates the gumball,
/// not the outliner's destructive row action, exactly as puzzle 3d reads it.
pub fn delete_target_volume(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(id) = args.and_then(|value| value.get("id")).and_then(Value::as_str) {
        ctx.scene.document.target_volumes.retain(|volume| volume.id != id);
    }
}
