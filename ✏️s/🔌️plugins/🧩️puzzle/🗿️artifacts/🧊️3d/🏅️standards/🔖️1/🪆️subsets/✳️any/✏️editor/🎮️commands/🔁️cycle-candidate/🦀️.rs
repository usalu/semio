//! 🖌️ `cycle-candidate` command.

use crate::editor::puzzle3d::drive_precompute;
use crate::editor::puzzle3d::puzzle3d_brush_target_vortex;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

/// 🔁️ `cycleBrushCandidate`/`cycleBrushCandidateBack` share one arm — the default step is the
/// direction the action id names, and an explicit `delta` overrides it.
pub fn cycle_candidate(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    let default_delta = if action == "cycleBrushCandidateBack" { -1 } else { 1 };
    let delta = args.and_then(|value| value.get("delta")).and_then(|value| value.as_i64()).unwrap_or(default_delta);
    let menu_vortex_id = ctx.scene.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty());
    if let Some(vortex_id) = menu_vortex_id.or_else(|| ctx.selected_vortex_ids().first().cloned()).or_else(|| puzzle3d_brush_target_vortex(ctx.scene)) {
        let free_count = ctx.app.precompute.borrow().brush_candidates(&vortex_id).free.len();
        if free_count > 0 {
            let current = ctx.scene.runtime.brush_candidate_index as i64;
            let next = (current + delta).rem_euclid(free_count as i64);
            ctx.scene.runtime.brush_candidate_index = next as usize;
        }
    } else {
        ctx.scene.runtime.brush_candidate_index = ctx.scene.runtime.brush_candidate_index.saturating_add_signed(delta as isize);
    }
}
