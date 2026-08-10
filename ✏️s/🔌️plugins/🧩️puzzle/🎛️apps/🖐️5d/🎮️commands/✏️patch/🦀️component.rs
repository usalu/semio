//! ✏️ Puzzle 5d play app commands — the inspector's field patches: one arm per entity, each resolving
//! an absolute `value` or a stepper `delta` through `puzzle5d_resolve_number_edit` and the dot-path
//! axis convention `ui_inspector_vec3_group` emits. Accepts singular or plural id args from the
//! inspection panel (`partId`/`partIds`, `gripFullId`/`gripFullIds`, `fastenerId`/`fastenerIds`).

use crate::apps::puzzle5d::{puzzle5d_axis_index, puzzle5d_grip_full_id, puzzle5d_resolve_number_edit, Puzzle5dActionCtx, Puzzle5dPartAnchor};
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

pub fn patch_grip(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
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

pub fn patch_fastener(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let fastener_ids = arg_id_set(args, "fastenerIds", "fastenerId");
    if fastener_ids.is_empty() {
        return;
    }
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    let text = value.and_then(Value::as_str).map(str::to_string);
    for fastener in &mut ctx.scene.document.fasteners {
        if !fastener_ids.contains(&fastener.id) {
            continue;
        }
        match field {
            "fastenerKind" => fastener.fastener_kind = text.clone().filter(|text| !text.is_empty()),
            "gap" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(fastener.gap, value, delta) {
                    fastener.gap = updated;
                }
            }
            "shift" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(fastener.shift, value, delta) {
                    fastener.shift = updated;
                }
            }
            "rise" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(fastener.rise, value, delta) {
                    fastener.rise = updated;
                }
            }
            "rotation" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(fastener.rotation, value, delta) {
                    fastener.rotation = updated;
                }
            }
            "turn" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(fastener.turn, value, delta) {
                    fastener.turn = updated;
                }
            }
            "tilt" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(fastener.tilt, value, delta) {
                    fastener.tilt = updated;
                }
            }
            "x" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(fastener.x, value, delta) {
                    fastener.x = updated;
                }
            }
            "y" => {
                if let Some(updated) = puzzle5d_resolve_number_edit(fastener.y, value, delta) {
                    fastener.y = updated;
                }
            }
            _ => {}
        }
    }
}
