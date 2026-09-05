//! 🎯️ `retarget-fastener` command.

use crate::editor::puzzle5d::{find_part_by_grip_full_id, Puzzle5dActionCtx, Puzzle5dDocument};
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

/// 🔀 Retargets `source` and/or `target` on an existing fastener. Rejects dangling grip refs, self-loops,
/// and pairs that would duplicate another fastener.
pub fn retarget_fastener(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(id) = arg_str(args, "id").or_else(|| arg_str(args, "fastenerId")).map(str::to_string) else {
        return;
    };
    let next_source = arg_str(args, "source").or_else(|| arg_str(args, "attracting")).map(str::to_string);
    let next_target = arg_str(args, "target").or_else(|| arg_str(args, "attracted")).map(str::to_string);
    if next_source.is_none() && next_target.is_none() {
        return;
    }
    let (source, target) = {
        let Some(fastener) = ctx.scene.document.fasteners.iter().find(|fastener| fastener.id == id) else {
            return;
        };
        (next_source.unwrap_or_else(|| fastener.source.clone()), next_target.unwrap_or_else(|| fastener.target.clone()))
    };
    if source.is_empty() || target.is_empty() || source == target {
        return;
    }
    if find_part_by_grip_full_id(&ctx.scene.document, &source).is_none() || find_part_by_grip_full_id(&ctx.scene.document, &target).is_none() {
        return;
    }
    if ctx.scene.document.fasteners.iter().any(|fastener| fastener.id != id && ((fastener.source == source && fastener.target == target) || (fastener.source == target && fastener.target == source))) {
        return;
    }
    let compatible = match (resolve_grip_kind(&ctx.scene.document, &source), resolve_grip_kind(&ctx.scene.document, &target)) {
        (Some(source_kind), Some(target_kind)) => puzzle5d_kinds_compatible(&ctx.scene.document, &source_kind, &target_kind),
        _ => true,
    };
    if !compatible {
        return;
    }
    if let Some(fastener) = ctx.scene.document.fasteners.iter_mut().find(|fastener| fastener.id == id) {
        fastener.source = source;
        fastener.target = target;
    }
}
