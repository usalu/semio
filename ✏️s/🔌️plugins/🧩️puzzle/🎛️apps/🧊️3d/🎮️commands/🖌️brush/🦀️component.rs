//! 🖌️ Puzzle 3d play app commands — brush placement and its one-shot suggestion popup: placing an
//! explicit payload, cycling the cached candidates, opening/hovering/accepting/closing the popup
//! (which must never switch the host-owned active utility), the 120ms background tick, and the
//! renderer's real-GLB mesh registration.

use crate::apps::puzzle3d::{
    drive_precompute, fixture_from_engine_fixture, puzzle3d_brush_target_vortex, puzzle3d_clear_selection, puzzle3d_rederive_all_attractions, puzzle3d_suggestions_tick_scope, resolve_puzzle3d_attractions, sync_precompute_session,
    Puzzle3dActionCtx,
};
use crate::apps::puzzle3d::config::Puzzle3dSuggestionMenu;
use crate::artifacts::puzzle3d::engine::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome};
use semio_framework_plugin::SelectionSet;
use serde_json::Value;

/// 🧱️ Places an explicit `BrushPlacePayload` (the viewport's own click-to-place path).
pub fn add_brush_object(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    let Some(payload) = args.and_then(|value| serde_json::from_value::<BrushPlacePayload>(value.clone()).ok()) else {
        return;
    };
    let outcome = ctx.app.precompute.borrow_mut().dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload });
    if let Ok(Puzzle3dEngineOutcome::Fixture(fixture)) = outcome {
        if let Some(next) = fixture_from_engine_fixture(ctx.scene, &fixture) {
            *ctx.scene = next;
            puzzle3d_rederive_all_attractions(&mut ctx.scene.fixture);
            resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
        }
    }
}

/// 🔁️ `cycleBrushCandidate`/`cycleBrushCandidateBack` share one arm — the default step is the
/// direction the action id names, and an explicit `delta` overrides it.
pub fn cycle_candidate(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    let default_delta = if action == "cycleBrushCandidateBack" { -1 } else { 1 };
    let delta = args.and_then(|value| value.get("delta")).and_then(|value| value.as_i64()).unwrap_or(default_delta);
    if let Some(vortex_id) = puzzle3d_brush_target_vortex(ctx.scene) {
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

/// 💡️ One-shot suggestion popup: select the vortex and open the picker WITHOUT switching the
/// host-owned utility/tool into brush mode.
pub fn open_vortex_suggestions(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(str::to_string) else {
        return;
    };
    ctx.scene.runtime.selection.vortex_ids = SelectionSet::from(vec![full_id.clone()]);
    ctx.scene.runtime.selection.object_ids.clear();
    ctx.scene.runtime.brush_candidate_index = 0;
    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
    let window_id = args.and_then(|value| value.get("windowId")).and_then(|value| value.as_str()).filter(|id| !id.is_empty()).unwrap_or(ctx.window_id).to_string();
    ctx.scene.runtime.suggestion_menu = Some(Puzzle3dSuggestionMenu { x, y, window_id });
    // 🧊️ Drop any stale empty/pending cache for this vortex, then refresh so the popup does not open
    // on a previous "No placement" result while meshes/candidates are ready.
    ctx.app.precompute.borrow_mut().invalidate_brush_target(&full_id);
    sync_precompute_session(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    ctx.app.precompute.borrow_mut().refresh_brush_candidates(&full_id);
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
}

pub fn close_vortex_suggestions(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.suggestion_menu = None;
    ctx.scene.runtime.hovered_vortex_full_id = None;
}

pub fn hover_suggestion(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
        ctx.scene.runtime.brush_candidate_index = index as usize;
    }
}

/// ✅️ Accepts the hovered (or explicitly indexed) candidate. Always dismisses the one-shot picker
/// FIRST — a failed preview/place must not leave `suggestionMenu.open` gating every split pane's
/// regular context menu.
pub fn accept_suggestion(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(ctx.scene.runtime.brush_candidate_index as u64) as usize;
    let vortex_id = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(str::to_string).or_else(|| puzzle3d_brush_target_vortex(ctx.scene));
    ctx.scene.runtime.suggestion_menu = None;
    ctx.scene.runtime.hovered_vortex_full_id = None;
    let Some(vortex_id) = vortex_id else {
        return;
    };
    ctx.scene.runtime.selection.vortex_ids = SelectionSet::from(vec![vortex_id.clone()]);
    ctx.scene.runtime.selection.object_ids.clear();
    ctx.app.precompute.borrow_mut().refresh_brush_candidates(&vortex_id);
    let preview = ctx.app.precompute.borrow().brush_preview(&vortex_id, index);
    let Some(preview) = preview else {
        return;
    };
    let outcome = ctx.app.precompute.borrow_mut().dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload: BrushPlacePayload::from(preview) });
    if let Ok(Puzzle3dEngineOutcome::Fixture(fixture)) = outcome {
        if let Some(next) = fixture_from_engine_fixture(ctx.scene, &fixture) {
            *ctx.scene = next;
            puzzle3d_rederive_all_attractions(&mut ctx.scene.fixture);
            resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
            // ✅️ One-shot place finished — leave the scene idle (no sticky vortex/hover/menu).
            puzzle3d_clear_selection(&mut ctx.scene.runtime.selection);
            ctx.scene.runtime.suggestion_menu = None;
            ctx.scene.runtime.hovered_vortex_full_id = None;
        }
    }
}

/// ⏱️ The host's 120ms suggestion tick — advances the brush lane by one small chunk and refreshes
/// only the world body's suggestion-menu interaction JSON.
pub fn suggestions_tick(ctx: &mut Puzzle3dActionCtx<'_>) {
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    *ctx.ui_scope = puzzle3d_suggestions_tick_scope();
}

/// 🧊️ Real GLB geometry the browser round-tripped for one mesh url — installed into the collision
/// engine and remembered for the mesh exporters.
pub fn register_brush_mesh(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let (Some(url), Some(positions), Some(indices)) =
        (args.and_then(|v| v.get("url")).and_then(|v| v.as_str()), args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array()))
    else {
        return;
    };
    let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
    let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
    ctx.app.precompute.borrow_mut().register_mesh(url, &positions, &indices);
    if let Ok(mut registry) = crate::apps::puzzle3d::PUZZLE3D_MESH_REGISTRY.lock() {
        registry.insert(url.to_string(), (positions, indices));
    }
}

/// 🎚️ The brush placement picker's select — its option values are `puzzle3d.brush.candidate.<index>`.
pub fn engagement_control_select(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
    if let Some(index) = candidate_id.strip_prefix("puzzle3d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
        ctx.scene.runtime.brush_candidate_index = index;
    }
}
