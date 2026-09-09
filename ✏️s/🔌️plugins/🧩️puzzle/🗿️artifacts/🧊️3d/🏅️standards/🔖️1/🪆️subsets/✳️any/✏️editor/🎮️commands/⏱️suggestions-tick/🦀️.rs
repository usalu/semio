//! 🖌️ `suggestions-tick` command.

use crate::editor::puzzle3d::modes::edit::windows::main::utilities::brush;
use crate::editor::puzzle3d::drive_precompute;
use crate::editor::puzzle3d::puzzle3d_brush_target_vortex;
use crate::editor::puzzle3d::Puzzle3dActionCtx;

/// ⏱️ The host's 120ms suggestion tick — advances the brush lane by one small chunk and refreshes
/// only the world body's suggestion-menu interaction JSON.
///
/// 🎯️ It spends its second bounded slice on the ONE target the user is actually looking at — the open
/// popup's pinned vortex, else the brush utility's live selection/hover target — instead of leaving it
/// behind the lane's whole document enumeration. Without this the placement picker and the popup stayed
/// empty for as long as the round-robin took to reach that vortex, which on a catalogued document is
/// unbounded from the user's point of view (`📓️2026-09-09-remaining-test-failures-audit.md` §3.4). The
/// refresh is skipped the moment the entry is terminal, so a resolved target costs nothing per tick.
pub fn suggestions_tick(ctx: &mut Puzzle3dActionCtx<'_>) {
    let target = ctx
        .scene
        .runtime
        .suggestion_menu
        .as_ref()
        .map(|menu| menu.vortex_full_id.clone())
        .filter(|id| !id.is_empty())
        .or_else(|| (ctx.scene.active_utility == brush::UTILITY_ID).then(|| puzzle3d_brush_target_vortex(ctx.scene, ctx.interaction)).flatten());
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    if let Some(target) = target {
        let mut precompute = ctx.app.precompute.borrow_mut();
        if precompute.brush_candidates(&target).unknown_pending {
            precompute.refresh_brush_candidates(&target);
        }
    }
}
