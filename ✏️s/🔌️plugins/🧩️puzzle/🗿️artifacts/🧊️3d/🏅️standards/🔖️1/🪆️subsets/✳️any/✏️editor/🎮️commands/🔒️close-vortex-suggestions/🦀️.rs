//! 🖌️ `close-vortex-suggestions` command.

use crate::editor::puzzle3d::modes::edit::windows::main::utilities::brush;
use crate::editor::puzzle3d::Puzzle3dActionCtx;

/// 🔒️ Closing the popup IS the cancel for its candidate search — which is why the progress row it
/// shows carries no cancel action of its own. The suggestions tick resolves its target from the open
/// menu first, so with the menu gone nothing warms that vortex unless the brush utility is armed on
/// it; dropping the latched live target as well means a popup closed from select mode leaves no lane
/// work behind at all. Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS wave G.
pub fn close_vortex_suggestions(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.suggestion_menu = None;
    if ctx.scene.active_utility != brush::UTILITY_ID {
        ctx.app.precompute.borrow_mut().set_brush_live_target(None);
    }
}
