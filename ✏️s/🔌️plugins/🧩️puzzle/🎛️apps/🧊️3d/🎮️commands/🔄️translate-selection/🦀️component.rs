//! 🔄️ `translate-selection` command.

use serde_json::Value;
use std::sync::atomic::Ordering;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::mesh_selection_ids;
use crate::apps::puzzle3d::puzzle3d_apply_translate;
use crate::apps::puzzle3d::puzzle3d_rederive_moved_attractions;
use crate::apps::puzzle3d::resolve_puzzle3d_attractions;

fn axis_arg(args: Option<&Value>, key: &str, fallback: f64) -> f64 {
    args.and_then(|value| value.get(key)).and_then(|value| value.as_f64()).unwrap_or(fallback)
}

pub fn translate_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let ids = mesh_selection_ids(args, &ctx.selected_object_ids());
    let (dx, dy, dz) = (axis_arg(args, "dx", 0.0), axis_arg(args, "dy", 0.0), axis_arg(args, "dz", 0.0));
    let volume_ids = ctx.selected_target_volume_ids();
    let incoming = resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
    puzzle3d_apply_translate(&mut ctx.scene.fixture, &ids, &volume_ids, dx, dy, dz);
    puzzle3d_rederive_moved_attractions(&mut ctx.scene.fixture, &ids, &incoming);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
