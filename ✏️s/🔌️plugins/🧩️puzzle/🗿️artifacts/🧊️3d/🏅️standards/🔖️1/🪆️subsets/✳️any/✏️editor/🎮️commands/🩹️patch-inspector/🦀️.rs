//! 🧊️ `patch-inspector` command.

use crate::editor::puzzle3d::apply_puzzle3d_inspector_patch;
use crate::editor::puzzle3d::modes::edit::windows::main::utilities::transform::{Puzzle3dSelectionMotion, Puzzle3dSelectionRecord};
use crate::editor::puzzle3d::puzzle3d_inspector_origin_nudge;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::{PUZZLE3D_GRANULARITY_ATTRACTION, PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_REFERENCE, PUZZLE3D_GRANULARITY_TARGET_VOLUME, PUZZLE3D_GRANULARITY_VORTEX};
use dsl::os_pack::json::Value;

/// 🎯️ `ids` falls back to the live `vortex`-domain selection for `entity`'s matching granularity when
/// the action carries none explicitly — the inspector panel itself no longer renders per-entity
/// controls (see `panels::inspection::render`'s doc comment), so every real caller today passes `ids`
/// explicitly; the fallback stays for any future caller that does not. An origin stepper nudge is a
/// gesture: it commits the transform tool's `drag-selection` over the ids instead of absolute origins.
pub fn patch_inspector(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str()).unwrap_or("");
    let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
    let ids = args.and_then(|value| value.get("ids")).and_then(|value| <Vec<String> as dsl::FromValue>::from_value(dsl::os_pack::json::to_dsl_value(value)).ok()).filter(|ids| !ids.is_empty()).unwrap_or_else(|| {
        if entity == PUZZLE3D_GRANULARITY_OBJECT {
            ctx.selected_object_ids()
        } else if entity == PUZZLE3D_GRANULARITY_VORTEX {
            ctx.selected_vortex_ids()
        } else if entity == PUZZLE3D_GRANULARITY_ATTRACTION {
            ctx.selected_attraction_ids()
        } else if entity == PUZZLE3D_GRANULARITY_REFERENCE {
            ctx.selected_reference_ids()
        } else if entity == PUZZLE3D_GRANULARITY_TARGET_VOLUME {
            ctx.selected_target_volume_ids()
        } else {
            Vec::new()
        }
    });
    if let Some(offset) = puzzle3d_inspector_origin_nudge(args) {
        ctx.commit_selection("patchInspector", vec![Puzzle3dSelectionRecord::new(ids, Puzzle3dSelectionMotion::Drag { offset })]);
        return;
    }
    let value = args.and_then(|value| value.get("value"));
    let delta = args.and_then(|value| value.get("delta"));
    apply_puzzle3d_inspector_patch(&mut ctx.scene.fixture, entity, &ids, field, value, delta);
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
