//! 🖌️ `accept-suggestion` command.

use crate::standards::v1::subsets::any::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
use crate::editor::puzzle3d::drive_precompute;
use crate::editor::puzzle3d::fixture_from_engine_fixture;
use crate::editor::puzzle3d::puzzle3d_brush_target_vortex;
use crate::editor::puzzle3d::puzzle3d_rederive_all_attractions;
use crate::editor::puzzle3d::resolve_puzzle3d_attractions;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::PUZZLE3D_GRANULARITY_OBJECT;
use dsl::os_pack::json::Value;

/// ✅️ Accepts the hovered (or explicitly indexed) candidate. Always dismisses the one-shot picker
/// FIRST — a failed preview/place must not leave `suggestionMenu.open` gating every split pane's
/// regular context menu. 🕹️ The placed object is re-selected through `Emit.interaction_writes` (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), applied by the framework once the document
/// mutations have landed.
pub fn accept_suggestion(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(ctx.scene.runtime.brush_candidate_index as u64) as usize;
    let vortex_id = args
        .and_then(|value| value.get("fullId"))
        .and_then(|value| value.as_str())
        .map(str::to_string)
        .or_else(|| ctx.scene.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty()))
        .or_else(|| ctx.selected_vortex_ids().first().cloned())
        .or_else(|| puzzle3d_brush_target_vortex(ctx.scene, ctx.interaction));
    ctx.scene.runtime.suggestion_menu = None;
    let Some(vortex_id) = vortex_id else {
        return;
    };
    let preview = ctx.app.precompute.borrow().brush_preview(&vortex_id, index);
    let Some(preview) = preview else {
        return;
    };
    let before: Vec<String> = ctx.scene.fixture.objects.iter().map(|object| object.id.clone()).collect();
    let outcome = ctx.app.precompute.borrow_mut().dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload: BrushPlacePayload::from(preview) });
    if let Ok(Puzzle3dEngineOutcome::Fixture(fixture)) = outcome {
        if let Some(next) = fixture_from_engine_fixture(ctx.scene, &fixture) {
            *ctx.scene = next;
            puzzle3d_rederive_all_attractions(&mut ctx.scene.fixture);
            resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
            // ✅️ One-shot place finished — leave the scene idle (no sticky menu).
            ctx.scene.runtime.suggestion_menu = None;
            let placed: Vec<String> = ctx.scene.fixture.objects.iter().map(|object| object.id.clone()).filter(|id| !before.contains(id)).collect();
            ctx.replace_selection(PUZZLE3D_GRANULARITY_OBJECT, placed);
        }
    }
}
