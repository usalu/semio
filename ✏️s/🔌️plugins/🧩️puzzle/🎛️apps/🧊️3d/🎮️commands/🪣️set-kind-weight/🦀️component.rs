//! 🪣️ `set-kind-weight` command.

use crate::artifacts::puzzle3d::schema::PrecomputeLane;
use semio_framework::kernel::UiDirtyScope;
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::puzzle3d_ensure_catalog_kind_weights;
use crate::apps::puzzle3d::puzzle3d_fill_options_scope;
use crate::apps::puzzle3d::puzzle3d_kind_ids;
use crate::apps::puzzle3d::puzzle3d_normalize_kind_weight_group;
use crate::apps::puzzle3d::sync_precompute_weights;

/// 🎲️ `setObjectKindWeight`/`setVortexKindWeight` share one arm. Object weights live on their own
/// simplex; a vortex slider nested under an object row carries the JOINT `P(object)×P(vortex)` value
/// and is converted back to the relative `P(vortex)` on the shared vortex simplex before normalizing.
pub fn set_kind_weight(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0).clamp(0.0, 1.0);
    let object_ids = puzzle3d_kind_ids(&ctx.scene.fixture, "objects");
    let vortex_ids = puzzle3d_kind_ids(&ctx.scene.fixture, "vortices");
    puzzle3d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.object_kind_weights, &object_ids);
    puzzle3d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.vortex_kind_weights, &vortex_ids);
    if action == "setObjectKindWeight" {
        ctx.scene.runtime.object_kind_weights = puzzle3d_normalize_kind_weight_group(&ctx.scene.runtime.object_kind_weights, &object_ids, kind_id, value);
    } else if let Some(object_kind_id) = args.and_then(|v| v.get("objectKindId")).and_then(|v| v.as_str()) {
        let object_weight = ctx.scene.runtime.object_kind_weights.get(object_kind_id).copied().unwrap_or(0.0);
        if object_weight > f64::EPSILON {
            let relative = (value / object_weight).clamp(0.0, 1.0);
            ctx.scene.runtime.vortex_kind_weights = puzzle3d_normalize_kind_weight_group(&ctx.scene.runtime.vortex_kind_weights, &vortex_ids, kind_id, relative);
        }
        // 🚫️ Parent object weight is 0 — joint contribution is always 0; ignore vortex edits.
    } else {
        ctx.scene.runtime.vortex_kind_weights = puzzle3d_normalize_kind_weight_group(&ctx.scene.runtime.vortex_kind_weights, &vortex_ids, kind_id, value);
    }
    sync_precompute_weights(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    *ctx.ui_scope = puzzle3d_fill_options_scope();
}
