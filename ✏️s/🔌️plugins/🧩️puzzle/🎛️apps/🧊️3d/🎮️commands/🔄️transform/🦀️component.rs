//! 🔄️ Puzzle 3d play app commands — moving things: the three gumball verbs (translate/rotate/scale,
//! applied here only when NO scratch drag session is live — mid-drag ticks are intercepted by
//! `Puzzle3dPlayApp::transform_drag_tick` before dispatch) and the Relocate utility's drop, which
//! also auto-attracts the moved object onto every compatible vortex inside the proximity radius.
//!
//! 🌲️ Every direct move rederives the moved objects' incoming attraction parameters from their NEW
//! poses before re-resolving, so the resolver reproduces the drop pose instead of snapping back.

use crate::apps::puzzle3d::{
    derive_attraction_params, mesh_selection_ids, puzzle3d_apply_rotate, puzzle3d_apply_scale, puzzle3d_apply_translate, puzzle3d_rederive_moved_attractions, puzzle3d_vortex_full_id, resolve_puzzle3d_attractions, sync_precompute_session,
    world_vortex_position, Puzzle3dActionCtx, Puzzle3dAttraction, PUZZLE3D_ID_COUNTER,
};
use serde_json::Value;
use std::sync::atomic::Ordering;

fn axis_arg(args: Option<&Value>, key: &str, fallback: f64) -> f64 {
    args.and_then(|value| value.get(key)).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

pub fn translate_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.scene.runtime.selection.object_ids);
    let (dx, dy, dz) = (axis_arg(args, "dx", 0.0), axis_arg(args, "dy", 0.0), axis_arg(args, "dz", 0.0));
    let volume_ids = ctx.scene.runtime.selection.target_volume_ids.to_vec();
    let incoming = resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
    puzzle3d_apply_translate(&mut ctx.scene.fixture, &ids, &volume_ids, dx, dy, dz);
    puzzle3d_rederive_moved_attractions(&mut ctx.scene.fixture, &ids, &incoming);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}

pub fn rotate_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.scene.runtime.selection.object_ids);
    let (ax, ay, az, angle) = (axis_arg(args, "ax", 0.0), axis_arg(args, "ay", 0.0), axis_arg(args, "az", 0.0), axis_arg(args, "angle", 0.0));
    let volume_ids = ctx.scene.runtime.selection.target_volume_ids.to_vec();
    let incoming = resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
    puzzle3d_apply_rotate(&mut ctx.scene.fixture, &ids, &volume_ids, ax, ay, az, angle);
    puzzle3d_rederive_moved_attractions(&mut ctx.scene.fixture, &ids, &incoming);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}

pub fn scale_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.scene.runtime.selection.object_ids);
    let (sx, sy, sz) = (axis_arg(args, "sx", 1.0), axis_arg(args, "sy", 1.0), axis_arg(args, "sz", 1.0));
    let volume_ids = ctx.scene.runtime.selection.target_volume_ids.to_vec();
    puzzle3d_apply_scale(&mut ctx.scene.fixture, &ids, &volume_ids, sx, sy, sz);
}

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
