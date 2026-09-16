//! 📏️ `scale-selection` command.

use crate::editor::puzzle2d::{puzzle2d_transform_selection, Puzzle2dActionCtx, Puzzle2dTransform};
use serde_json::Value;

/// 📏️ Scales the selected nodes' positions about the selection's centroid by `factor` (sizes stay —
/// a node kind's footprint is the kind's, not the layout's).
pub fn scale_selection(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let factor = args.and_then(|value| value.get("factor").or_else(|| value.get("value"))).and_then(Value::as_f64).filter(|factor| factor.is_finite() && *factor > 0.0 && *factor != 1.0);
    let Some(factor) = factor else { return };
    let selected_ids = ctx.selected_ids();
    puzzle2d_transform_selection(&mut ctx.scene.fixture, &selected_ids, Puzzle2dTransform::Scale { factor });
}
