//! 🖌️ `suggestions-tick` command.

use crate::editor::puzzle3d::config::Puzzle3dSuggestionMenu;
use crate::artifacts::puzzle3d::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
use semio_framework_plugin::SelectionSet;
use serde_json::Value;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::drive_precompute;
use crate::editor::puzzle3d::puzzle3d_suggestions_tick_scope;

/// ⏱️ The host's 120ms suggestion tick — advances the brush lane by one small chunk and refreshes
/// only the world body's suggestion-menu interaction JSON.
pub fn suggestions_tick(ctx: &mut Puzzle3dActionCtx<'_>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    *ctx.ui_scope = puzzle3d_suggestions_tick_scope();
}
