//! 🖌️ `close-vortex-suggestions` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

/// 🔒️ Closing the menu releases its vortex from the brush suggestions run: with no armed brush target left,
/// the refresh after it aborts the run. A `fullId` scopes the close to the menu opened on that vortex, so a
/// context menu closing late never tears down the menu a newer right-click already opened.
pub fn close_vortex_suggestions(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let scoped = args.and_then(|value| value.get("fullId")).and_then(Value::as_str).filter(|id| !id.is_empty());
    if scoped.is_some_and(|full_id| ctx.scene.runtime.suggestion_menu.as_ref().is_some_and(|menu| menu.vortex_full_id != full_id)) {
        return;
    }
    ctx.scene.runtime.suggestion_menu = None;
    ctx.brush_suggestions(|link| link.close_menu());
}
