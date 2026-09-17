//! 🔁️ `cycle-brush-candidate` command.

use crate::editor::puzzle5d::commands::add_brush_part::puzzle5d_brush_source_grip;
use crate::editor::puzzle5d::Puzzle5dActionCtx;

/// 🔁️ Advances the candidate index, wrapping around the free candidates the brush run resolved for the current
/// target grip (or just incrementing when there is no target yet).
pub fn cycle_brush_candidate(ctx: &mut Puzzle5dActionCtx<'_>) {
    cycle(ctx, true);
}

/// 🔃️ The `shift+tab` twin: walks the same wrapped ring backwards, so a candidate stepped past is one keystroke
/// away instead of a whole lap.
pub fn cycle_brush_candidate_back(ctx: &mut Puzzle5dActionCtx<'_>) {
    cycle(ctx, false);
}

fn cycle(ctx: &mut Puzzle5dActionCtx<'_>, forwards: bool) {
    let index = ctx.scene.runtime.brush_candidate_index;
    let Some(target) = puzzle5d_brush_source_grip(ctx, None) else {
        ctx.scene.runtime.brush_candidate_index = if forwards { index.saturating_add(1) } else { index.saturating_sub(1) };
        return;
    };
    let free = ctx.brush_suggestions(|link| link.found(&target).map_or(0, |found| found.free().count())).unwrap_or(0);
    if free > 0 {
        ctx.scene.runtime.brush_candidate_index = if forwards { (index + 1) % free } else { (index % free + free - 1) % free };
    }
}
