//! ✏️ `patch-fastener` command.

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
