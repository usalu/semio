//! 🔄️ `scale-selection` command.

use serde_json::Value;
use std::sync::atomic::Ordering;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::mesh_selection_ids;
use crate::apps::puzzle3d::puzzle3d_apply_scale;

fn axis_arg(args: Option<&Value>, key: &str, fallback: f64) -> f64 {
    args.and_then(|value| value.get(key)).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

pub fn scale_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.scene.runtime.selection.object_ids);
    let (sx, sy, sz) = (axis_arg(args, "sx", 1.0), axis_arg(args, "sy", 1.0), axis_arg(args, "sz", 1.0));
    let volume_ids = ctx.scene.runtime.selection.target_volume_ids.to_vec();
    puzzle3d_apply_scale(&mut ctx.scene.fixture, &ids, &volume_ids, sx, sy, sz);
}
