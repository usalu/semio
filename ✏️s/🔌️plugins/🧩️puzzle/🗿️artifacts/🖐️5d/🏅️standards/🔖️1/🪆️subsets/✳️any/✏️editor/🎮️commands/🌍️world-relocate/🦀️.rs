//! 🌍️ `world-relocate` command.

use crate::editor::puzzle5d::commands::proximity_connect::puzzle5d_proximity_peers;
use crate::editor::puzzle5d::{Puzzle5dActionCtx, Puzzle5dFastener, Puzzle5dFreshIds, PUZZLE5D_PROXIMITY_RADIUS};
use dsl::os_pack::json::Value;

/// 🚚️ Drops one part at an explicit world origin, then auto-fastens its first grip to every other grip that
/// lands within [`PUZZLE5D_PROXIMITY_RADIUS`] through the shared proximity search (no kind gate).
pub fn world_relocate(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let object_id = args.and_then(|value| value.get("objectId")).and_then(Value::as_str).unwrap_or("");
    let position = args.and_then(|value| value.get("position")).and_then(Value::as_array).map(|values| [0, 1, 2].map(|axis| values.get(axis).and_then(Value::as_f64).unwrap_or(0.0)));
    let (Some(part), Some(position)) = (ctx.scene.document.parts.iter_mut().find(|part| part.id == object_id), position) else {
        return;
    };
    part.part_3d.origin = position;
    let Some((moved_id, peers)) = puzzle5d_proximity_peers(&ctx.scene.document, object_id, PUZZLE5D_PROXIMITY_RADIUS, false) else { return };
    if peers.is_empty() {
        return;
    }
    let mut fresh_ids = Puzzle5dFreshIds::from_document(&ctx.scene.document);
    let fresh: Vec<Puzzle5dFastener> = peers.into_iter().map(|peer| Puzzle5dFastener { id: fresh_ids.next_fastener(), source: moved_id.clone(), target: peer.grip, fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 }).collect();
    ctx.scene.document.fasteners.extend(fresh);
}
