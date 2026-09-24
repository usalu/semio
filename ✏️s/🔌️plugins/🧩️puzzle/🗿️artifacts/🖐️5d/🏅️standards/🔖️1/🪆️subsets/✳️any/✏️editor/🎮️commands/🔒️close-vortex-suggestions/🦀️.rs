//! 🔒️ `close-vortex-suggestions` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;
use semio_s_artifact_puzzle_3d::editor::puzzle3d::modes::edit::windows::main::utilities::brush::run_effects;

/// 🔒️ Closing the menu releases its grip from the brush suggestions run: with no armed brush target left, the
/// abort it owes is emitted right here, through the same `run_effects` ladder the refresh poll uses. A `fullId`
/// scopes the close to the menu opened on that grip, so a context menu closing late never tears down the menu a
/// newer right-click already opened.
pub fn close_vortex_suggestions(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let scoped = args.and_then(|value| value.get("fullId")).and_then(Value::as_str).filter(|id| !id.is_empty());
    if scoped.is_some_and(|full_id| ctx.scene.runtime.suggestion_menu.as_ref().is_some_and(|menu| menu.vortex_full_id != full_id)) {
        return;
    }
    ctx.scene.runtime.suggestion_menu = None;
    let effects = ctx
        .brush_suggestions(|link| {
            link.close_menu();
            run_effects(link, ctx.tool_run)
        })
        .unwrap_or_default();
    ctx.effects.extend(effects);
}
