//! 🔄️ `scale-selection` command.

use serde_json::Value;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::mesh_selection_ids;
use crate::editor::puzzle3d::puzzle3d_apply_scale;

async fn axis_arg(args: Option<&Value>, key: &str, fallback: f64) -> f64 {
    args.and_then(|value| value.get(key)).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

pub async fn scale_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.selected_object_ids());
    let (sx, sy, sz) = (axis_arg(args, "sx", 1.0), axis_arg(args, "sy", 1.0), axis_arg(args, "sz", 1.0));
    let volume_ids = ctx.selected_target_volume_ids();
    puzzle3d_apply_scale(&mut ctx.scene.fixture, &ids, &volume_ids, sx, sy, sz);
}
