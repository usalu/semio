//! 🌍️ `world-relocate` command.

use crate::editor::puzzle3d::modes::edit::windows::main::utilities::transform::puzzle3d_relocate_record;
use crate::editor::puzzle3d::{value_as_vec3, Puzzle3dActionCtx};
use dsl::os_pack::json::Value;

/// 🚚️ Drops one unlocked, visible object at the world `position` a finished Relocate drag names, as ONE
/// transform-tool transaction: the `drag-selection` from its base origin plus a `connect-vortices` from every
/// vortex within `proximity_radius` of its first vortex onto it. A locked or hidden grab answers with the lock
/// notice instead of a dead gesture.
pub fn world_relocate(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let object_id = args.and_then(|value| value.get("objectId")).and_then(Value::as_str).unwrap_or("");
    let Some(position) = args.and_then(|value| value.get("position")).and_then(value_as_vec3) else { return };
    if ctx.base.objects.iter().any(|object| object.id == object_id && (object.locked || object.hidden)) {
        ctx.refuse_when_locked();
        return;
    }
    if let Some(record) = puzzle3d_relocate_record(ctx.base, object_id, position, ctx.scene.runtime.proximity_radius) {
        ctx.commit_selection("worldRelocate", vec![record]);
    }
}
