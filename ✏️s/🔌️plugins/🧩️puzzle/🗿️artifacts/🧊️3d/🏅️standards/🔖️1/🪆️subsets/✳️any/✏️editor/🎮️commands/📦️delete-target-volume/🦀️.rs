//! 📦️ `delete-target-volume` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

pub fn delete_target_volume(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
        ctx.scene.fixture.target_volumes.retain(|volume| volume.id != id);
    }
}
