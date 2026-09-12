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
///
/// 🧊️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B35: the LATCHED target is the third leg, because the
/// render resolves one (`world_brush_preview_target`: menu → selection/hover → `brush_live_target`) and
/// this tick used only the first two. Whenever the render fell through to the latch — the leftover
/// `refresh-ui` shape whose hover the host skip-clears — and something then dropped that vortex's cache
/// entry (every `registerBrushMesh` and every document edit clear `brush_cache`; the browser fires
/// hundreds of the former per example), the render asked for a target no tick would ever warm again and
/// the gate printed `brushPreview.gate reason=no-free-candidate free=0 pending=true` for as long as the
/// pane stayed armed (measured live in wave B33, `suggestionsTick.enter … target=None` against renders
/// asking for `seed-left-001:v3`). Warming whatever the render asks for is the invariant; the utility
/// gate stays on the two speculative legs so plain select-mode hovering still costs no slices.
pub fn suggestions_tick(ctx: &mut Puzzle3dActionCtx<'_>) {
    let brush_armed = ctx.scene.active_utility == brush::UTILITY_ID;
    let target = ctx
        .scene
        .runtime
        .suggestion_menu
        .as_ref()
        .map(|menu| menu.vortex_full_id.clone())
        .filter(|id| !id.is_empty())
        .or_else(|| brush_armed.then(|| puzzle3d_brush_target_vortex(ctx.scene, ctx.interaction)).flatten())
        .or_else(|| brush_armed.then(|| ctx.app.precompute.borrow().brush_live_target().map(str::to_string)).flatten());
    eprintln!("[DEBUG] puzzle3d.suggestionsTick.enter utility={} menu={} target={:?}", ctx.scene.active_utility, ctx.scene.runtime.suggestion_menu.is_some(), target);
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    let mut slices = 0_u32;
    if let Some(target) = target.as_deref() {
        let mut precompute = ctx.app.precompute.borrow_mut();
        precompute.set_brush_live_target(Some(target.to_string()));
        let before = precompute.brush_candidates(target);
        eprintln!("[DEBUG] puzzle3d.brushPreview.cache tick-before vortex={target} free={} pending={}", before.free.len(), before.unknown_pending);
        if before.free.is_empty() {
            precompute.refresh_brush_candidates(target);
            slices += 1;
            for _ in 0..7 {
                let status = precompute.brush_candidates(target);
                if !status.unknown_pending || !status.free.is_empty() {
                    break;
                }
                precompute.refresh_brush_candidates(target);
                slices += 1;
            }
        }
        let after = precompute.brush_candidates(target);
        eprintln!("[DEBUG] puzzle3d.brushPreview.cache tick-after vortex={target} free={} pending={} slices={slices}", after.free.len(), after.unknown_pending);
    }
    eprintln!("[DEBUG] puzzle3d.suggestionsTick.exit target={:?} slices={slices}", target);
}
