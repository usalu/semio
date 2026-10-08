//! 📦️ `delete-target-volume` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use semio_framework_pack_json::Value;

/// 🪦️ One `delete-target-volume` for the named volume, when the base holds it.
pub fn delete_target_volume(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).filter(|id| ctx.base.target_volumes.iter().any(|volume| volume.id == *id)) {
        ctx.artifact_mutations.push(crate::standards::v1::subsets::any::schema::mutations::delete_target_volume(id.to_string()));
    }
}
