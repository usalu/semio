//! 📏️ `scale-selection` command.

use crate::editor::puzzle2d::{puzzle2d_selected_target_region_ids, puzzle2d_transform_selection, puzzle2d_transform_target_regions, Puzzle2dActionCtx, Puzzle2dTransform};
use serde_json::Value;

/// 📏️ Scales the selected nodes' positions about the selection's centroid by `factor` (sizes stay —
/// a node kind's footprint is the kind's, not the layout's). Selected target regions scale with the
/// gesture too, and unlike a node they scale their EXTENT as well as their corner — a region has no
/// kind catalogue to take a footprint from, exactly as puzzle3d scales a selected target volume.
pub fn scale_selection(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let factor = args.and_then(|value| value.get("factor").or_else(|| value.get("value"))).and_then(Value::as_f64).filter(|factor| factor.is_finite() && *factor > 0.0 && *factor != 1.0);
    let Some(factor) = factor else { return };
    let selected_ids = ctx.selected_ids();
    if ctx.refuse_when_locked(&selected_ids) {
        return;
    }
    puzzle2d_transform_selection(&mut ctx.scene.fixture, &selected_ids, Puzzle2dTransform::Scale { factor });
    let region_ids = puzzle2d_selected_target_region_ids(&ctx.scene.fixture, &selected_ids);
    puzzle2d_transform_target_regions(&mut ctx.scene.fixture, &region_ids, Puzzle2dTransform::Scale { factor });
}
