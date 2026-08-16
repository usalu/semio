//! 🔗️ `proximity-connect` command.

use crate::editor::puzzle5d::{find_part_by_grip_full_id, next_fastener_id, puzzle5d_grip_full_id, puzzle5d_resolve_number_edit, world_grip_position, Puzzle5dActionCtx, Puzzle5dDocument, Puzzle5dFastener, PUZZLE5D_PROXIMITY_RADIUS};
use serde_json::Value;

fn arg_str<'a>(args: Option<&'a Value>, key: &str) -> Option<&'a str> {
    args.and_then(|value| value.get(key)).and_then(Value::as_str).filter(|text| !text.is_empty())
}
fn arg_f64(args: Option<&Value>, key: &str) -> Option<f64> {
    args.and_then(|value| value.get(key)).and_then(Value::as_f64)
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
    let Some(entries) = document.kind_compatibility.as_ref().and_then(Value::as_array) else {
        return true;
    };
    if entries.is_empty() {
        return true;
    }
    entries.iter().any(|entry| {
        let source = entry.get("source").and_then(Value::as_str).unwrap_or("");
        let target = entry.get("target").and_then(Value::as_str).unwrap_or("");
        let bidirectional = entry.get("bidirectional").and_then(Value::as_bool).unwrap_or(false);
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

/// 🚚️ Proximity-connect helper (3d relocate auto-attract twin): for one part, fasten its first grip as
/// `target` onto every other grip inside `radius` (default [`PUZZLE5D_PROXIMITY_RADIUS`]) that is not
/// already connected and that passes kind compatibility. The stationary peer stays `source` so flatten
/// keeps the pre-existing structure as the resolution root.
pub fn proximity_connect(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let part_id = arg_str(args, "partId").or_else(|| arg_str(args, "objectId")).unwrap_or("").to_string();
    if part_id.is_empty() {
        return;
    }
    let radius = arg_f64(args, "radius").unwrap_or(PUZZLE5D_PROXIMITY_RADIUS).max(0.0);
    let source_grip = ctx.scene.document.parts.iter().find(|part| part.id == part_id).and_then(|part| part.grips.first().map(|grip| (puzzle5d_grip_full_id(&part.id, &grip.id), world_grip_position(part, grip))));
    let Some((moved_grip_id, moved_position)) = source_grip else {
        return;
    };
    let mut fresh: Vec<Puzzle5dFastener> = Vec::new();
    for other in &ctx.scene.document.parts {
        if other.id == part_id {
            continue;
        }
        for grip in &other.grips {
            let peer_id = puzzle5d_grip_full_id(&other.id, &grip.id);
            if peer_id == moved_grip_id {
                continue;
            }
            let peer_position = world_grip_position(other, grip);
            let dx = moved_position[0] - peer_position[0];
            let dy = moved_position[1] - peer_position[1];
            let dz = moved_position[2] - peer_position[2];
            if (dx * dx + dy * dy + dz * dz).sqrt() > radius {
                continue;
            }
            if fasteners_already_connected(&ctx.scene.document, &peer_id, &moved_grip_id) {
                continue;
            }
            let compatible = match (resolve_grip_kind(&ctx.scene.document, &peer_id), resolve_grip_kind(&ctx.scene.document, &moved_grip_id)) {
                (Some(source_kind), Some(target_kind)) => puzzle5d_kinds_compatible(&ctx.scene.document, &source_kind, &target_kind),
                _ => true,
            };
            if !compatible {
                continue;
            }
            fresh.push(fastener_from_args(next_fastener_id(), peer_id, moved_grip_id.clone(), args));
        }
    }
    ctx.scene.document.fasteners.extend(fresh);
}
