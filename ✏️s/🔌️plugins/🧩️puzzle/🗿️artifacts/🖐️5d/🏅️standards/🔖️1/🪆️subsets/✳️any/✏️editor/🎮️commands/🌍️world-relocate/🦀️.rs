//! 🌍️ `world-relocate` command.

use crate::editor::puzzle5d::modes::edit::windows::world3d::utilities::transform::puzzle5d_relocate_record;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use semio_framework_pack_json::Value;

/// 🚚️ Drops one part at an explicit world origin as ONE transform-tool transaction: the `drag-selection3d`
/// leaf from the part's committed origin, then a `connect-grips` from its first grip to every free grip that
/// lands within the session's proximity radius.
pub fn world_relocate(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let part_id = args.and_then(|value| value.get("objectId")).and_then(Value::as_str).unwrap_or("");
    let Some(position) = args.and_then(|value| value.get("position")).and_then(Value::as_array).and_then(|values| Some([values.first()?.as_f64()?, values.get(1)?.as_f64()?, values.get(2)?.as_f64()?])) else {
        return;
    };
    let base = ctx.snapshot.typed_arc();
    let Some(record) = puzzle5d_relocate_record(&base, part_id, position, ctx.scene.runtime.proximity_radius) else { return };
    ctx.commit_selection("worldRelocate", vec![record]);
}
