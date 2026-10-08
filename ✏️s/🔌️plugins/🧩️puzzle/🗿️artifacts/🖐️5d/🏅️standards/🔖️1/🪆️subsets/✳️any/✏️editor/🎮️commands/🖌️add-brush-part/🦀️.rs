//! 🖌️ The brush source grip `cycle-brush-candidate` shares with the retained `addBrushPart` work.

use crate::editor::puzzle5d::Puzzle5dActionCtx;

/// 🎯️ The grip a brush placement or a candidate cycle works on: `explicit`, else the grip an open suggestion
/// menu lists candidates for, else the first selected grip, else the grip the brush suggestions run is pointed at.
pub fn puzzle5d_brush_source_grip(ctx: &Puzzle5dActionCtx<'_>, explicit: Option<&str>) -> Option<String> {
    explicit
        .map(str::to_string)
        .or_else(|| ctx.scene.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty()))
        .or_else(|| ctx.selected_grip_ids().first().cloned())
        .or_else(|| ctx.brush_suggestions(|link| link.target().map(str::to_string)).flatten())
}
