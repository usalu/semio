//! 🖌️ `add-brush-part` command.

use serde_json::{json, Value};
use crate::apps::puzzle5d::Puzzle5dActionCtx;
use crate::apps::puzzle5d::puzzle5d_brush_target_grip;

/// 🧱️ `addBrushPart`/`addBrushObject`: tries the engine's collision-free placement for the explicit
/// payload first, then always runs the paired board placement so both projections land in one part.
pub fn add_brush_part(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.app.drive_precompute(ctx.scene);
    if let Some(payload_value) = args {
        let mut payload = payload_value.clone();
        if let Some(object) = payload.as_object_mut() {
            if let Some(part_kind) = object.remove("partKind") {
                object.insert("objectKindId".to_string(), part_kind);
            }
            if object.get("targetVortexFullId").is_none() {
                if let Some(grip_id) = ctx.selected_grip_ids().first().cloned().or_else(|| puzzle5d_brush_target_grip(ctx.scene)) {
                    object.insert("targetVortexFullId".to_string(), json!(grip_id));
                }
            }
        }
        if let Some(next) = ctx.app.apply_engine_brush_placement(ctx.scene, &payload) {
            *ctx.scene = next;
        }
    }
    let part_kind = args.and_then(|value| value.get("partKind").or_else(|| value.get("objectKindId"))).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
    let payload = json!({ "nodeKind": part_kind, "x": args.and_then(|value| value.get("x")).cloned().unwrap_or(json!(120.0)), "y": args.and_then(|value| value.get("y")).cloned().unwrap_or(json!(120.0)) });
    ctx.app.apply_board_brush_place(ctx.scene, &payload);
}
