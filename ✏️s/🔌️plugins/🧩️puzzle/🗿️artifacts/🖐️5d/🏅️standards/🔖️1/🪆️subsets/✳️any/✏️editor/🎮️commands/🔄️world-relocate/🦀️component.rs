//! 🔄️ `world-relocate` command.

use crate::editor::puzzle5d::next_fastener_id;
use crate::editor::puzzle5d::puzzle5d_grip_full_id;
use crate::editor::puzzle5d::world_grip_position;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use crate::editor::puzzle5d::Puzzle5dFastener;
use crate::editor::puzzle5d::PUZZLE5D_PROXIMITY_RADIUS;
use serde_json::Value;

/// 🚚️ Drops one part at an explicit world origin, then auto-fastens its first grip to every other
/// grip that lands within [`PUZZLE5D_PROXIMITY_RADIUS`].
pub async fn world_relocate(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
    let position = args.and_then(|value| value.get("position")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok());
    let (Some(part), Some(position)) = (ctx.scene.document.parts.iter_mut().find(|part| part.id == object_id), position) else {
        return;
    };
    part.part_3d.origin = position;
    let source_grip = part.grips.first().map(|grip| (puzzle5d_grip_full_id(&part.id, &grip.id), world_grip_position(part, grip)));
    if let Some((source_id, source_position)) = source_grip {
        let mut fresh: Vec<Puzzle5dFastener> = Vec::new();
        for other in &ctx.scene.document.parts {
            if other.id == object_id {
                continue;
            }
            for grip in &other.grips {
                let target_id = puzzle5d_grip_full_id(&other.id, &grip.id);
                if target_id == source_id {
                    continue;
                }
                let target_position = world_grip_position(other, grip);
                let dx = source_position[0] - target_position[0];
                let dy = source_position[1] - target_position[1];
                let dz = source_position[2] - target_position[2];
                if (dx * dx + dy * dy + dz * dz).sqrt() <= PUZZLE5D_PROXIMITY_RADIUS && !ctx.scene.document.fasteners.iter().any(|entry| entry.source == source_id && entry.target == target_id || entry.source == target_id && entry.target == source_id)
                {
                    fresh.push(Puzzle5dFastener { id: next_fastener_id(), source: source_id.clone(), target: target_id, fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
                }
            }
        }
        ctx.scene.document.fasteners.extend(fresh);
    }
    ctx.app.drive_precompute(ctx.scene);
}
