//! 🖌️ `accept-suggestion` command.

use crate::artifacts::puzzle3d::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
use serde_json::Value;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::drive_precompute;
use crate::editor::puzzle3d::fixture_from_engine_fixture;
use crate::editor::puzzle3d::puzzle3d_brush_target_vortex;
use crate::editor::puzzle3d::puzzle3d_rederive_all_attractions;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;

/// ✅️ Accepts the hovered (or explicitly indexed) candidate. Always dismisses the one-shot picker
/// FIRST — a failed preview/place must not leave `suggestionMenu.open` gating every split pane's
/// regular context menu. 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the
/// post-accept "select the placed vortex" step is gone — selection is framework-owned now and this
/// command has no channel to write it (see `puzzle3d_brush_target_vortex`'s doc comment); the caller
/// must still hold (or re-pick) the target via an explicit `fullId`.
pub fn accept_suggestion(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(ctx.scene.runtime.brush_candidate_index as u64) as usize;
    let vortex_id = args
        .and_then(|value| value.get("fullId"))
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .or_else(|| ctx.scene.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty()))
        .or_else(|| ctx.selected_vortex_ids().first().cloned())
        .or_else(|| puzzle3d_brush_target_vortex(ctx.scene));
    ctx.scene.runtime.suggestion_menu = None;
    let Some(vortex_id) = vortex_id else {
        return;
    };
    ctx.app.precompute.borrow_mut().refresh_brush_candidates(&vortex_id);
    let preview = ctx.app.precompute.borrow().brush_preview(&vortex_id, index);
    let Some(preview) = preview else {
        return;
    };
    let outcome = ctx.app.precompute.borrow_mut().dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload: BrushPlacePayload::from(preview) });
    if let Ok(Puzzle3dEngineOutcome::Fixture(fixture)) = outcome {
        if let Some(next) = fixture_from_engine_fixture(ctx.scene, &fixture) {
            *ctx.scene = next;
            puzzle3d_rederive_all_attractions(&mut ctx.scene.fixture);
            resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
            // ✅️ One-shot place finished — leave the scene idle (no sticky menu).
            ctx.scene.runtime.suggestion_menu = None;
        }
    }
}
