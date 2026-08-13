//! 🖌️ `open-vortex-suggestions` command.

use crate::apps::puzzle3d::config::Puzzle3dSuggestionMenu;
use crate::artifacts::puzzle3d::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
use semio_framework_plugin::SelectionSet;
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use crate::apps::puzzle3d::drive_precompute;
use crate::apps::puzzle3d::sync_precompute_session;

/// 💡️ One-shot suggestion popup: select the vortex and open the picker WITHOUT switching the
/// host-owned utility/tool into brush mode.
pub fn open_vortex_suggestions(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(str::to_string) else {
        return;
    };
    ctx.scene.runtime.selection.vortex_ids = SelectionSet::from(vec![full_id.clone()]);
    ctx.scene.runtime.selection.object_ids.clear();
    ctx.scene.runtime.brush_candidate_index = 0;
    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let window_id = args.and_then(|value| value.get("windowId")).and_then(|value| value.as_str()).filter(|id| !id.is_empty()).unwrap_or(ctx.window_id).to_string();
    ctx.scene.runtime.suggestion_menu = Some(Puzzle3dSuggestionMenu { x, y, window_id });
    // 🧊️ Drop any stale empty/pending cache for this vortex, then refresh so the popup does not open
    // on a previous "No placement" result while meshes/candidates are ready.
    ctx.app.precompute.borrow_mut().invalidate_brush_target(&full_id);
    sync_precompute_session(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    ctx.app.precompute.borrow_mut().refresh_brush_candidates(&full_id);
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
}
