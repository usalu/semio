//! 🔧️ `edit-fastener` command.

use crate::editor::puzzle5d::{puzzle5d_resolve_number_edit, Puzzle5dActionCtx, Puzzle5dFastener};
use dsl::os_pack::json::Value;

fn arg_str<'a>(args: Option<&'a Value>, key: &str) -> Option<&'a str> {
    args.and_then(|value| value.get(key)).and_then(Value::as_str).filter(|text| !text.is_empty())
}
fn apply_fastener_number(field: &str, fastener: &mut Puzzle5dFastener, value: Option<&Value>, delta: Option<&Value>) -> bool {
    let slot = match field {
        "gap" => Some(&mut fastener.gap),
        "shift" => Some(&mut fastener.shift),
        "rise" => Some(&mut fastener.rise),
        "rotation" => Some(&mut fastener.rotation),
        "turn" => Some(&mut fastener.turn),
        "tilt" => Some(&mut fastener.tilt),
        "x" => Some(&mut fastener.x),
        "y" => Some(&mut fastener.y),
        _ => None,
    };
    let Some(slot) = slot else {
        return false;
    };
    if let Some(updated) = puzzle5d_resolve_number_edit(*slot, value, delta) {
        *slot = updated;
        true
    } else {
        false
    }
}
fn apply_fastener_batch(fastener: &mut Puzzle5dFastener, args: Option<&Value>) {
    if let Some(kind) = arg_str(args, "fastenerKind").or_else(|| arg_str(args, "edgeKind")) {
        fastener.fastener_kind = if kind.is_empty() { None } else { Some(kind.to_string()) };
    }
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y"] {
        if let Some(value) = args.and_then(|payload| payload.get(key)) {
            let _ = apply_fastener_number(key, fastener, Some(value), None);
        }
    }
}

/// 🎛 Batch-edits any subset of the eight connection parameters (and optional `fastenerKind`) on one
/// fastener. Also accepts the inspector's single-field shape (`field` + `value`/`delta`) so `x`/`y`
/// patches work even when `patchFastener` has not been extended yet.
pub fn edit_fastener(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(id) = arg_str(args, "id").or_else(|| arg_str(args, "fastenerId")).map(str::to_string) else {
        return;
    };
    let Some(fastener) = ctx.scene.document.fasteners.iter_mut().find(|fastener| fastener.id == id) else {
        return;
    };
    apply_fastener_batch(fastener, args);
    if let Some(field) = arg_str(args, "field") {
        match field {
            "fastenerKind" | "edgeKind" => {
                let text = args.and_then(|value| value.get("value")).and_then(Value::as_str).map(str::to_string);
                fastener.fastener_kind = text.filter(|text| !text.is_empty());
            }
            _ => {
                let value = args.and_then(|payload| payload.get("value"));
                let delta = args.and_then(|payload| payload.get("delta"));
                let _ = apply_fastener_number(field, fastener, value, delta);
            }
        }
    }
}
