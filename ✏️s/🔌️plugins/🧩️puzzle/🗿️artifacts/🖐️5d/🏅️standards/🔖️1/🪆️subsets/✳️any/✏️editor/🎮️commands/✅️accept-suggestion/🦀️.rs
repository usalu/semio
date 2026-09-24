//! ✅️ `accept-suggestion` command.

use crate::editor::puzzle5d::precompute::brush::puzzle5d_brush_placement;
use crate::editor::puzzle5d::{Puzzle5dActionCtx, Puzzle5dFreshIds, PUZZLE5D_GRANULARITY_PART};
use dsl::os_pack::json::Value;

/// ✅️ Places the `index`-th (else the hovered) free candidate the brush suggestions run found for the menu's
/// grip, posed exactly as the search posed it and fastened to that grip, then selects the new part. The menu
/// closes FIRST, so a failed placement never leaves it gating the context menu; every dead end is a notice.
pub fn accept_suggestion(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let index = args.and_then(|value| value.get("index")).and_then(Value::as_u64).map_or(ctx.scene.runtime.brush_candidate_index, |index| index as usize);
    let grip = args
        .and_then(|value| value.get("fullId"))
        .and_then(Value::as_str)
        .filter(|id| !id.is_empty())
        .map(str::to_string)
        .or_else(|| ctx.scene.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty()))
        .or_else(|| ctx.selected_grip_ids().first().cloned());
    ctx.scene.runtime.suggestion_menu = None;
    let Some(grip) = grip else {
        return ctx.notice(|labels| labels.placement_unavailable.as_str());
    };
    let found = ctx
        .brush_suggestions(|link| {
            let found = link.found(&grip).cloned();
            link.close_menu();
            found
        })
        .flatten();
    let Some(found) = found else {
        return ctx.notice(|labels| labels.placement_unavailable.as_str());
    };
    let mut fresh_ids = Puzzle5dFreshIds::from_document(&ctx.scene.document);
    let (part_id, fastener_id) = (fresh_ids.next_part(), fresh_ids.next_fastener());
    match puzzle5d_brush_placement(ctx.snapshot, &ctx.scene.document, &found, None, index, part_id, fastener_id) {
        Ok(Some((part, fastener))) => {
            let placed = part.id.clone();
            ctx.scene.document.parts.push(part);
            ctx.scene.document.fasteners.push(fastener);
            ctx.replace_selection(PUZZLE5D_GRANULARITY_PART, [placed]);
        }
        Ok(None) => ctx.notice(|labels| labels.placement_unavailable.as_str()),
        Err(_) => ctx.abort = true,
    }
}
