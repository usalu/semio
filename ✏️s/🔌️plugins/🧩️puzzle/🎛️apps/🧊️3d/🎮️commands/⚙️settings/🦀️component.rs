//! ⚙️ Puzzle 3d play app commands — the tuning knobs: brush overlap budget, relocate proximity
//! radius, viewport chunk size, volume-brush voxel dimensions, the transform gumball's Move/Rotate
//! flags, and the two vortex display options. All are pure view/config state.

use crate::apps::puzzle3d::{puzzle3d_absolute_or_delta, sync_precompute_session, Puzzle3dActionCtx, PUZZLE3D_VORTEX_DIRECTION_INWARDS, PUZZLE3D_VORTEX_DIRECTION_OUTWARDS, PUZZLE3D_VORTEX_SHOW_ALWAYS, PUZZLE3D_VORTEX_SHOW_SELECTED};
use serde_json::Value;

pub fn set_proximity_radius(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.proximity_radius) {
        ctx.scene.runtime.proximity_radius = value.max(0.0);
    }
}

pub fn set_chunk_size(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.chunk_size) {
        ctx.scene.runtime.chunk_size = value.max(1.0);
    }
}

/// 🖌️ The collision budget every brush/fill placement is tested against — re-syncs the precompute
/// session immediately so already-cached candidates are recomputed under the new budget.
pub fn set_brush_placement_overlap_budget(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.overlap_budget) {
        ctx.scene.runtime.overlap_budget = value.clamp(0.0, 1.0);
        sync_precompute_session(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    }
}

pub fn set_voxel_dims(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let axis = args.and_then(|value| value.get("axis")).and_then(|value| value.as_str()).unwrap_or("");
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
        let dimension = value.max(1.0).round() as u32;
        match axis {
            "w" => ctx.scene.runtime.voxel_dims[0] = dimension,
            "d" => ctx.scene.runtime.voxel_dims[1] = dimension,
            "h" => ctx.scene.runtime.voxel_dims[2] = dimension,
            _ => {}
        }
    }
}

pub fn set_transform_gumball_flag(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
    let pressed = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool);
    match flag {
        "move" => ctx.scene.runtime.transform_move = pressed.unwrap_or(!ctx.scene.runtime.transform_move),
        "rotate" => ctx.scene.runtime.transform_rotate = pressed.unwrap_or(!ctx.scene.runtime.transform_rotate),
        _ => {}
    }
}

pub fn set_vortex_show(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        if mode == PUZZLE3D_VORTEX_SHOW_ALWAYS || mode == PUZZLE3D_VORTEX_SHOW_SELECTED {
            ctx.scene.runtime.vortex_show = mode.into();
        }
    }
}

pub fn set_vortex_direction(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        if mode == PUZZLE3D_VORTEX_DIRECTION_OUTWARDS || mode == PUZZLE3D_VORTEX_DIRECTION_INWARDS {
            ctx.scene.runtime.vortex_direction = mode.into();
        }
    }
}
