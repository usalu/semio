//! ✏️ `patch-part` command.

use crate::editor::puzzle5d::{puzzle5d_axis_index, puzzle5d_grip_full_id, puzzle5d_resolve_number_edit, Puzzle5dActionCtx, Puzzle5dPartAnchor};
use serde_json::Value;
use std::collections::HashSet;

fn arg_id_set(args: Option<&Value>, plural: &str, singular: &str) -> HashSet<String> {
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

pub fn patch_part(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let part_ids = arg_id_set(args, "partIds", "partId");
    if part_ids.is_empty() {
        return;
    }
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    let text = value.and_then(Value::as_str).map(str::to_string);
    for part in &mut ctx.scene.document.parts {
        if !part_ids.contains(&part.id) {
            continue;
        }
        match field {
            "partKind" => {
                if let Some(text) = &text {
                    part.part_kind = text.clone();
                }
            }
            "anchor" => {
                if let Some(text) = &text {
                    part.anchor = match text.to_ascii_lowercase().as_str() {
                        "derived" | "connected" => Puzzle5dPartAnchor::Derived,
                        _ => Puzzle5dPartAnchor::Fixed,
                    };
                }
            }
            "text" => {
                if let Some(text) = &text {
                    part.part_2d.text = text.clone();
                }
            }
            "label" => part.part_3d.label = text.clone().filter(|text| !text.is_empty()),
            "meshUrl" => part.part_3d.mesh_url = text.clone().filter(|text| !text.is_empty()),
            "x" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(part.part_2d.x, value, delta) {
                    part.part_2d.x = updated;
                }
            }
            "y" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(part.part_2d.y, value, delta) {
                    part.part_2d.y = updated;
                }
            }
            _ => {
                if let Some(axis) = puzzle5d_axis_index(field, "origin") {
                    if let Some(updated) = puzzle5d_resolve_number_edit(part.part_3d.origin[axis], value, delta) {
                        part.part_3d.origin[axis] = updated;
                    }
                }
            }
        }
    }
}
