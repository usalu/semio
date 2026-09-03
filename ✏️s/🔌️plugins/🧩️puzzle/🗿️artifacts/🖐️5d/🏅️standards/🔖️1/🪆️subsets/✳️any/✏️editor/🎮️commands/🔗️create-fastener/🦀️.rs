//! 🔗️ `create-fastener` command.

use crate::editor::puzzle5d::{find_part_by_grip_full_id, Puzzle5dActionCtx, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dFreshIds};
use dsl::os_pack::json::Value;

fn arg_str<'a>(args: Option<&'a Value>, key: &str) -> Option<&'a str> {
    args.and_then(|value| value.get(key)).and_then(Value::as_str).filter(|text| !text.is_empty())
}
fn resolve_grip_kind(document: &Puzzle5dDocument, full_id: &str) -> Option<String> {
    let (_part, grip) = find_part_by_grip_full_id(document, full_id)?;
    let kind = if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() };
    if kind.is_empty() {
        None
    } else {
        Some(kind)
    }
}
/// 🧲️ Permissive when the document declares no `kindCompatibility` rules — otherwise requires an
/// explicit (or bidirectional) entry, matching puzzle3d's attraction gate.
fn puzzle5d_kinds_compatible(document: &Puzzle5dDocument, source_kind: &str, target_kind: &str) -> bool {
    let Some(entries) = document.kind_compatibility.as_ref().and_then(serde_json::Value::as_array) else {
        return true;
    };
    if entries.is_empty() {
        return true;
    }
    entries.iter().any(|entry| {
        let source = entry.get("source").and_then(serde_json::Value::as_str).unwrap_or("");
        let target = entry.get("target").and_then(serde_json::Value::as_str).unwrap_or("");
        let bidirectional = entry.get("bidirectional").and_then(serde_json::Value::as_bool).unwrap_or(false);
        (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind)
    })
}
fn fasteners_already_connected(document: &Puzzle5dDocument, source: &str, target: &str) -> bool {
    document.fasteners.iter().any(|fastener| (fastener.source == source && fastener.target == target) || (fastener.source == target && fastener.target == source))
}
fn fastener_from_args(id: String, source: String, target: String, args: Option<&Value>) -> Puzzle5dFastener {
    Puzzle5dFastener {
        id,
        source,
        target,
        fastener_kind: arg_str(args, "fastenerKind").or_else(|| arg_str(args, "edgeKind")).map(str::to_string),
        gap: arg_f64(args, "gap").unwrap_or(0.0),
        shift: arg_f64(args, "shift").unwrap_or(0.0),
        rise: arg_f64(args, "rise").unwrap_or(0.0),
        rotation: arg_f64(args, "rotation").unwrap_or(0.0),
        turn: arg_f64(args, "turn").unwrap_or(0.0),
        tilt: arg_f64(args, "tilt").unwrap_or(0.0),
        x: arg_f64(args, "x").unwrap_or(0.0),
        y: arg_f64(args, "y").unwrap_or(0.0),
    }
}
fn arg_f64(args: Option<&Value>, key: &str) -> Option<f64> {
    args.and_then(|value| value.get(key)).and_then(Value::as_f64)
}

/// 🆕 Creates a fastener between two grip full-ids. No-ops when endpoints collide, already connect, or
/// fail the kind-compatibility gate. Optional numeric args seed the eight parameters (default `0.0`) so
/// a create can land with compose-accurate offsets without a follow-up edit.
pub fn create_fastener(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let source = arg_str(args, "source").or_else(|| arg_str(args, "attracting")).unwrap_or("").to_string();
    let target = arg_str(args, "target").or_else(|| arg_str(args, "attracted")).unwrap_or("").to_string();
    if source.is_empty() || target.is_empty() || source == target {
        return;
    }
    if find_part_by_grip_full_id(&ctx.scene.document, &source).is_none() || find_part_by_grip_full_id(&ctx.scene.document, &target).is_none() {
        return;
    }
    if fasteners_already_connected(&ctx.scene.document, &source, &target) {
        return;
    }
    let compatible = match (resolve_grip_kind(&ctx.scene.document, &source), resolve_grip_kind(&ctx.scene.document, &target)) {
        (Some(source_kind), Some(target_kind)) => puzzle5d_kinds_compatible(&ctx.scene.document, &source_kind, &target_kind),
        // 🌲️ Missing kind metadata is permissive (board `edgeCreate` never gated); only explicit pairs are checked.
        _ => true,
    };
    if !compatible {
        return;
    }
    let id = arg_str(args, "id").or_else(|| arg_str(args, "fastenerId")).map(str::to_string).unwrap_or_else(|| Puzzle5dFreshIds::from_document(&ctx.scene.document).next_fastener());
    ctx.scene.document.fasteners.push(fastener_from_args(id, source, target, args));
}
