//! 🖌️ `set-kind-weight` command.

use crate::editor::puzzle5d::puzzle5d_ensure_catalog_kind_weights;
use crate::editor::puzzle5d::puzzle5d_kind_ids;
use crate::editor::puzzle5d::puzzle5d_normalize_kind_weight_group;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use serde_json::Value;

/// ⚖️ `setObjectKindWeight`/`setVortexKindWeight` share one arm: both re-normalize their whole group
/// so the sliders always sum to 1.
pub fn set_kind_weight(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0).clamp(0.0, 1.0);
    let part_ids = puzzle5d_kind_ids(&ctx.scene.document, "parts");
    let grip_ids = puzzle5d_kind_ids(&ctx.scene.document, "grips");
    puzzle5d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.object_kind_weights, &part_ids);
    puzzle5d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.vortex_kind_weights, &grip_ids);
    if action == "setObjectKindWeight" {
        ctx.scene.runtime.object_kind_weights = puzzle5d_normalize_kind_weight_group(&ctx.scene.runtime.object_kind_weights, &part_ids, kind_id, value);
    } else {
        ctx.scene.runtime.vortex_kind_weights = puzzle5d_normalize_kind_weight_group(&ctx.scene.runtime.vortex_kind_weights, &grip_ids, kind_id, value);
    }
    ctx.app.drive_precompute(ctx.scene);
}
