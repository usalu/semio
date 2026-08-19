//! ✏️ `patch-grip` command.

use crate::editor::puzzle5d::{puzzle5d_axis_index, puzzle5d_grip_full_id, puzzle5d_resolve_number_edit, Puzzle5dActionCtx};
use serde_json::Value;
use std::collections::HashSet;

async fn arg_id_set(args: Option<&Value>, plural: &str, singular: &str) -> HashSet<String> {
    let mut ids = HashSet::new();
    if let Some(array) = args.and_then(|value| value.get(plural)).and_then(Value::as_array) {
        for entry in array {
            if let Some(id) = entry.as_str().filter(|id| !id.is_empty()) {
                ids.insert(id.to_string());
            }
        }
    }
    if let Some(id) = args.and_then(|value| value.get(singular)).and_then(Value::as_str).filter(|id| !id.is_empty()) {
        ids.insert(id.to_string());
    }
    ids
}

pub async fn patch_grip(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let grip_full_ids = arg_id_set(args, "gripFullIds", "gripFullId");
    if grip_full_ids.is_empty() {
        return;
    }
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    let text = value.and_then(Value::as_str).map(str::to_string);
    for part in &mut ctx.scene.document.parts {
        let part_id = part.id.clone();
        for grip in &mut part.grips {
            if !grip_full_ids.contains(&puzzle5d_grip_full_id(&part_id, &grip.id)) {
                continue;
            }
            match field {
                "gripKind" => {
                    if let Some(text) = &text {
                        grip.grip_kind = text.clone();
                        grip.grip_2d.grip_kind = text.clone();
                    }
                }
                "angle" => {
                    if let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_2d.angle, value, delta) {
                        grip.grip_2d.angle = updated;
                    }
                }
                "radius" => {
                    if let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.radius, value, delta) {
                        grip.grip_2d.radius = updated;
                        grip.grip_3d.radius = updated;
                    }
                }
                "label" => grip.grip_3d.label = text.clone().filter(|text| !text.is_empty()),
                _ => {
                    if let Some(axis) = puzzle5d_axis_index(field, "position") {
                        if let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.position[axis], value, delta) {
                            grip.grip_3d.position[axis] = updated;
                        }
                    } else if let Some(axis) = puzzle5d_axis_index(field, "direction") {
                        let mut direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
                        if let Some(updated) = puzzle5d_resolve_number_edit(direction[axis], value, delta) {
                            direction[axis] = updated;
                            grip.grip_3d.direction = Some(direction);
                        }
                    }
                }
            }
        }
    }
}
