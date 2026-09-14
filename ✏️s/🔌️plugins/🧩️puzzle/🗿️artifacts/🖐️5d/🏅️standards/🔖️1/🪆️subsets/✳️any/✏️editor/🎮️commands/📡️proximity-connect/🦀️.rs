//! 📡️ `proximity-connect` command, and the proximity search `world-relocate` shares with it.

use crate::editor::puzzle5d::precompute::geometry::distance_squared;
use crate::editor::puzzle5d::{engine_grip_kind, puzzle5d_grip_full_id, world_grip_position, Puzzle5dActionCtx, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dFreshIds, Puzzle5dGrip, Puzzle5dPart, PUZZLE5D_PROXIMITY_RADIUS};
use dsl::os_pack::json::Value;
use std::collections::HashSet;

fn arg_str<'a>(args: Option<&'a Value>, key: &str) -> Option<&'a str> {
    args.and_then(|value| value.get(key)).and_then(Value::as_str).filter(|text| !text.is_empty())
}
fn arg_f64(args: Option<&Value>, key: &str) -> Option<f64> {
    args.and_then(|value| value.get(key)).and_then(Value::as_f64)
}

/// 🧲️ One grip the proximity search found next to a part's first grip.
#[derive(Clone, Debug, PartialEq)]
pub struct Puzzle5dProximityPeer {
    pub grip: String,
    pub kind: String,
}

/// 🔎️ The proximity search: the first grip of `part_id` and, in document order, every grip of another part
/// within `radius` of it that no fastener joins to it yet and — when `gated` — whose kind the document's
/// `kindCompatibility` admits (permissive without rules, puzzle3d's attraction gate). A rotation by a quaternion `q`
/// stretches a grip's offset by exactly `|q|²`, so every grip of a part lies inside the sphere of its longest
/// stretched offset around the part origin: a part
/// whose sphere misses the query ball is rejected on its origin alone, and only the grips of parts in reach are
/// ever rotated. The fastener set and the compatibility rules are read only when a peer is in reach. `None`
/// without such a part or grip.
pub fn puzzle5d_proximity_peers(document: &Puzzle5dDocument, part_id: &str, radius: f64, gated: bool) -> Option<(String, Vec<Puzzle5dProximityPeer>)> {
    let radius = radius.max(0.0);
    let moved = document.parts.iter().find(|part| part.id == part_id)?;
    let moved_grip = moved.grips.first()?;
    let moved_id = puzzle5d_grip_full_id(&moved.id, &moved_grip.id);
    let center = world_grip_position(moved, moved_grip);
    let squared = radius * radius;
    let mut reach: Vec<(&Puzzle5dPart, &Puzzle5dGrip)> = Vec::new();
    for part in &document.parts {
        let [x, y, z, w] = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let mut longest = 0.0f64;
        for grip in &part.grips {
            let [gx, gy, gz] = grip.grip_3d.position;
            longest = longest.max(gx * gx + gy * gy + gz * gz);
        }
        let bound = radius + longest.sqrt() * (x * x + y * y + z * z + w * w);
        if distance_squared(part.part_3d.origin, center) > bound * bound || part.id == part_id {
            continue;
        }
        for grip in &part.grips {
            if distance_squared(world_grip_position(part, grip), center) <= squared {
                reach.push((part, grip));
            }
        }
    }
    if reach.is_empty() {
        return Some((moved_id, Vec::new()));
    }
    let moved_kind = engine_grip_kind(moved_grip);
    let connected: HashSet<(&str, &str)> = document.fasteners.iter().flat_map(|fastener| [(fastener.source.as_str(), fastener.target.as_str()), (fastener.target.as_str(), fastener.source.as_str())]).collect();
    let rules: Vec<(&str, &str, bool)> = if gated {
        document.kind_compatibility.as_ref().and_then(serde_json::Value::as_array).into_iter().flatten().map(|entry| (entry.get("source").and_then(serde_json::Value::as_str).unwrap_or(""), entry.get("target").and_then(serde_json::Value::as_str).unwrap_or(""), entry.get("bidirectional").and_then(serde_json::Value::as_bool).unwrap_or(false))).collect()
    } else {
        Vec::new()
    };
    let admits = |kind: &str| rules.is_empty() || kind.is_empty() || moved_kind.is_empty() || rules.iter().any(|(source, target, bidirectional)| (*source == kind && *target == moved_kind) || (*bidirectional && *source == moved_kind && *target == kind));
    let peers = reach
        .into_iter()
        .map(|(part, grip)| Puzzle5dProximityPeer { grip: puzzle5d_grip_full_id(&part.id, &grip.id), kind: engine_grip_kind(grip) })
        .filter(|peer| peer.grip != moved_id && !connected.contains(&(peer.grip.as_str(), moved_id.as_str())) && admits(&peer.kind))
        .collect();
    Some((moved_id, peers))
}

/// 🚚️ Proximity-connect (3d relocate auto-attract twin): for one part, fasten its first grip as `target` onto
/// every other grip inside `radius` (default [`PUZZLE5D_PROXIMITY_RADIUS`]) that is not already connected and
/// that passes kind compatibility. The stationary peer stays `source` so flatten keeps the pre-existing
/// structure as the resolution root.
pub fn proximity_connect(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(part_id) = arg_str(args, "partId").or_else(|| arg_str(args, "objectId")) else { return };
    let radius = arg_f64(args, "radius").unwrap_or(PUZZLE5D_PROXIMITY_RADIUS);
    let Some((moved_id, peers)) = puzzle5d_proximity_peers(&ctx.scene.document, part_id, radius, true) else { return };
    if peers.is_empty() {
        return;
    }
    let mut fresh_ids = Puzzle5dFreshIds::from_document(&ctx.scene.document);
    let fresh: Vec<Puzzle5dFastener> = peers
        .into_iter()
        .map(|peer| Puzzle5dFastener {
            id: fresh_ids.next_fastener(),
            source: peer.grip,
            target: moved_id.clone(),
            fastener_kind: arg_str(args, "fastenerKind").or_else(|| arg_str(args, "edgeKind")).map(str::to_string),
            gap: arg_f64(args, "gap").unwrap_or(0.0),
            shift: arg_f64(args, "shift").unwrap_or(0.0),
            rise: arg_f64(args, "rise").unwrap_or(0.0),
            rotation: arg_f64(args, "rotation").unwrap_or(0.0),
            turn: arg_f64(args, "turn").unwrap_or(0.0),
            tilt: arg_f64(args, "tilt").unwrap_or(0.0),
            x: arg_f64(args, "x").unwrap_or(0.0),
            y: arg_f64(args, "y").unwrap_or(0.0),
        })
        .collect();
    ctx.scene.document.fasteners.extend(fresh);
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
