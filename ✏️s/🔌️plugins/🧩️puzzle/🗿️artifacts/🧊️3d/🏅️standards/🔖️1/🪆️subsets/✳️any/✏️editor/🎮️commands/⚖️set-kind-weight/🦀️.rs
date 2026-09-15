//! 🪣️ `set-kind-weight` command.

use crate::editor::puzzle3d::puzzle3d_apply_distribution_weight;
use crate::editor::puzzle3d::puzzle3d_kind_ids;
use crate::editor::puzzle3d::sync_precompute_weights;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

/// 🎲️ `setObjectKindWeight`/`setVortexKindWeight` — joint probabilities across the nested tree sum to 1.
pub fn set_kind_weight(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0).clamp(0.0, 1.0);
    let object_kind_id = args.and_then(|v| v.get("objectKindId")).and_then(|v| v.as_str());
    let object_ids = puzzle3d_kind_ids(&ctx.scene.fixture, "objects");
    let vortex_ids = puzzle3d_kind_ids(&ctx.scene.fixture, "vortices");
    puzzle3d_apply_distribution_weight(&mut ctx.scene.runtime.object_kind_weights, &mut ctx.scene.runtime.vortex_kind_weights, &object_ids, &vortex_ids, action, kind_id, object_kind_id, value);
    sync_precompute_weights(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
}
