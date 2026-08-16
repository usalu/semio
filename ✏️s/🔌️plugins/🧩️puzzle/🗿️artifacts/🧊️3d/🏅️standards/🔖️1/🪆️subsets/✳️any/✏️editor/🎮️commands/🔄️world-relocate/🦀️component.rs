//! 🔄️ `world-relocate` command.

use serde_json::Value;
use std::sync::atomic::Ordering;
use crate::editor::puzzle3d::PUZZLE3D_ID_COUNTER;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::derive_attraction_params;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;
use crate::editor::puzzle3d::sync_precompute_session;
use crate::editor::puzzle3d::world_vortex_position;
use crate::editor::puzzle3d::puzzle3d_vortex_full_id;
use crate::editor::puzzle3d::Puzzle3dAttraction;

/// 🚚️ Drops one unlocked, visible object at an absolute world position and attracts its first vortex
/// onto every other vortex inside `proximity_radius` that is not already connected to it.
pub fn world_relocate(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("").to_string();
    let position = args
        .and_then(|value| value.get("position"))
        .and_then(|value| value.as_array())
        .map(|values| [values.first().and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0)]);
    let proximity_radius = ctx.scene.runtime.proximity_radius;
    let Some(position) = position else {
        return;
    };
    let Some(object) = ctx.scene.fixture.objects.iter_mut().find(|object| object.id == object_id && !object.locked && !object.hidden) else {
        return;
    };
    object.origin = position;
    let object_orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let source_vortex = object.vortices.first().map(|vortex| (puzzle3d_vortex_full_id(&object.id, &vortex.id), world_vortex_position(object, vortex), vortex.position, vortex.direction.unwrap_or([0.0, 0.0, -1.0])));
    // 🌲️ New attractions attach the MOVED object as `attracted`: the pre-existing, stationary structure
    // it snapped onto stays the resolution root. Params are derived from the current (already-relocated)
    // poses so nothing jumps when the resolver next runs.
    if let Some((source_id, source_pos, source_local_pos, source_local_dir)) = source_vortex {
        let mut created: Vec<Puzzle3dAttraction> = Vec::new();
        for other in &ctx.scene.fixture.objects {
            if other.id == object_id {
                continue;
            }
            for vortex in &other.vortices {
                let target_id = puzzle3d_vortex_full_id(&other.id, &vortex.id);
                if target_id == source_id {
                    continue;
                }
                let target_pos = world_vortex_position(other, vortex);
                let dx = source_pos[0] - target_pos[0];
                let dy = source_pos[1] - target_pos[1];
                let dz = source_pos[2] - target_pos[2];
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                if distance > proximity_radius {
                    continue;
                }
                let already_connected = ctx.scene.fixture.attractions.iter().any(|entry| entry.attracting == source_id && entry.attracted == target_id || entry.attracting == target_id && entry.attracted == source_id);
                if already_connected {
                    continue;
                }
                let attraction_id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(
                    other.origin,
                    other.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                    vortex.position,
                    vortex.direction.unwrap_or([0.0, 0.0, -1.0]),
                    source_local_pos,
                    source_local_dir,
                    position,
                    object_orientation,
                );
                created.push(Puzzle3dAttraction { id: attraction_id, attracting: target_id, attracted: source_id.clone(), gap, shift, rise, rotation, turn, tilt });
            }
        }
        ctx.scene.fixture.attractions.extend(created);
    }
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
    sync_precompute_session(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
}
