//! 🎯️ `add-target-region` command.

use crate::editor::puzzle2d::{puzzle2d_paint_target_region, Puzzle2dActionCtx};
use serde_json::Value;

/// 🖍️ Paints one grid-snapped target region at `args.origin`, sized by the Area Brush's own
/// width/height steppers (in grid cells) unless `args.size` states an explicit extent. A dispatch
/// carrying no usable `origin` returns without touching the board — the 2d twin of puzzle3d's
/// `addTargetVolume`, whose Alt+click is otherwise indistinguishable from a dead gesture.
pub fn add_target_region(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let pair = |key: &str| {
        args.and_then(|value| value.get(key)).and_then(Value::as_array).filter(|values| values.len() >= 2).and_then(|values| Some((values[0].as_f64()?, values[1].as_f64()?))).filter(|(x, y)| x.is_finite() && y.is_finite())
    };
    let Some(origin) = pair("origin") else {
        return;
    };
    let size = pair("size").unwrap_or((ctx.scene.runtime.area_brush_width, ctx.scene.runtime.area_brush_height));
    let grid_factor = ctx.scene.runtime.grid_factor;
    puzzle2d_paint_target_region(&mut ctx.scene.fixture, origin, size, grid_factor);
}
