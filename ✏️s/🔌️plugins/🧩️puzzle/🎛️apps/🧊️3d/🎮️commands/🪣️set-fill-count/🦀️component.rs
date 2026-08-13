//! 🪣️ `set-fill-count` command.

use crate::artifacts::puzzle3d::schema::PrecomputeLane;
use semio_framework::kernel::UiDirtyScope;
use serde_json::Value;
use crate::apps::puzzle3d::PUZZLE3D_FILL_COUNT_MAX;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::apply_puzzle3d_fill_count;
use crate::apps::puzzle3d::puzzle3d_fill_build_scope;

pub fn set_fill_count(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map_or(0, |value| value.round().max(0.0) as u32).min(PUZZLE3D_FILL_COUNT_MAX);
    apply_puzzle3d_fill_count(&mut ctx.app.precompute.borrow_mut(), ctx.scene, count);
    *ctx.ui_scope = puzzle3d_fill_build_scope();
}
