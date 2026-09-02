//! 🔄️ `rotate-selection` command.

use crate::editor::puzzle3d::mesh_selection_ids;
use crate::editor::puzzle3d::puzzle3d_apply_rotate;
use crate::editor::puzzle3d::puzzle3d_rederive_moved_attractions;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

fn axis_arg(args: Option<&Value>, key: &str, fallback: f64) -> f64 {
    args.and_then(|value| value.get(key)).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

pub fn rotate_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.selected_object_ids());
    let (ax, ay, az, angle) = (axis_arg(args, "ax", 0.0), axis_arg(args, "ay", 0.0), axis_arg(args, "az", 0.0), axis_arg(args, "angle", 0.0));
    let volume_ids = ctx.selected_target_volume_ids();
    let incoming = resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
    puzzle3d_apply_rotate(&mut ctx.scene.fixture, &ids, &volume_ids, ax, ay, az, angle);
    puzzle3d_rederive_moved_attractions(&mut ctx.scene.fixture, &ids, &incoming);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
