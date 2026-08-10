//! 🔄️ Puzzle 5d play app commands — the 3D gumball transforms plus the relocate-and-auto-fasten drag.
//! Each translate/rotate/scale tick coalesces into one undoable edit (see `handle_action_impl`'s
//! `coalesce_key`).

use crate::apps::puzzle5d::{
    mesh_selection_ids, next_fastener_id, part_scale_json, puzzle5d_grip_full_id, quat_from_axis_angle, quat_mul, world_grip_position, Puzzle5dActionCtx, Puzzle5dFastener, PUZZLE5D_PROXIMITY_RADIUS,
};
use serde_json::{json, Value};

pub fn translate_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, ctx.scene.runtime.selection.part_ids.as_slice());
    let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    for part in &mut ctx.scene.document.parts {
        if ids.contains(&part.id) {
            part.part_3d.origin[0] += dx;
            part.part_3d.origin[1] += dy;
            part.part_3d.origin[2] += dz;
        }
    }
}

pub fn rotate_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, ctx.scene.runtime.selection.part_ids.as_slice());
    let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let delta = quat_from_axis_angle(ax, ay, az, angle);
    for part in &mut ctx.scene.document.parts {
        if ids.contains(&part.id) {
            let current = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            part.part_3d.orientation = Some(quat_mul(delta, current));
        }
    }
}

pub fn scale_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, ctx.scene.runtime.selection.part_ids.as_slice());
    let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
    let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
    let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
    for part in &mut ctx.scene.document.parts {
        if ids.contains(&part.id) {
            let current = part_scale_json(part);
            part.part_3d.scale = Some(json!([current[0] * sx, current[1] * sy, current[2] * sz]));
        }
    }
}

/// 🚚️ Drops one part at an explicit world origin, then auto-fastens its first grip to every other
/// grip that lands within [`PUZZLE5D_PROXIMITY_RADIUS`].
pub fn world_relocate(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
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
                if (dx * dx + dy * dy + dz * dz).sqrt() <= PUZZLE5D_PROXIMITY_RADIUS
                    && !ctx.scene.document.fasteners.iter().any(|entry| entry.source == source_id && entry.target == target_id || entry.source == target_id && entry.target == source_id)
                {
                    fresh.push(Puzzle5dFastener { id: next_fastener_id(), source: source_id.clone(), target: target_id, fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
                }
            }
        }
        ctx.scene.document.fasteners.extend(fresh);
    }
    ctx.app.drive_precompute(ctx.scene);
}
