//! 🖌️ `close-vortex-suggestions` command.

use crate::apps::puzzle3d::config::Puzzle3dSuggestionMenu;
use crate::artifacts::puzzle3d::schema::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
use semio_framework_plugin::SelectionSet;
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

pub fn close_vortex_suggestions(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.suggestion_menu = None;
    ctx.scene.runtime.hovered_vortex_full_id = None;
}
