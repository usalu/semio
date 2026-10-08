//! 🪦️ `delete-target-volume` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use semio_framework_pack_json::Value;

/// 🪦️ Drops one target volume by id with one `delete-target-volume`, when the base holds it. A locked volume is still
/// deletable — `locked` gates the gumball, not the outliner's destructive row action, exactly as puzzle 3d reads it.
pub fn delete_target_volume(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(id) = args.and_then(|value| value.get("id")).and_then(Value::as_str).filter(|id| ctx.snapshot.typed().target_volumes.iter().any(|volume| volume.id == *id)) {
        ctx.artifact_mutations.push(crate::standards::v1::subsets::any::schema::mutations::delete_target_volume(id.to_string()));
    }
}
