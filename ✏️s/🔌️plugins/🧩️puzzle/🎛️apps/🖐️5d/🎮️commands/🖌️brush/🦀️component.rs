//! 🖌️ Puzzle 5d play app commands — brush placement and its parameters: placing a payload (through
//! the engine first, then the paired board fallback), cycling the cached candidates, the placement
//! picker, the suggestion offset / overlap budget, the per-kind distribution weights, and the
//! renderer's real-GLB mesh registration.

use crate::apps::puzzle5d::{
    parse_brush_candidates_free, puzzle5d_brush_target_grip, puzzle5d_ensure_catalog_kind_weights, puzzle5d_kind_ids, puzzle5d_normalize_kind_weight_group, Puzzle5dActionCtx, PUZZLE5D_SUGGESTION_OFFSET_MAX, PUZZLE5D_SUGGESTION_OFFSET_MIN,
};
use serde_json::{json, Value};

/// 🧱️ `addBrushPart`/`addBrushObject`: tries the engine's collision-free placement for the explicit
/// payload first, then always runs the paired board placement so both projections land in one part.
pub fn add_brush_part(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.app.drive_precompute(ctx.scene);
    if let Some(payload_value) = args {
        let mut payload = payload_value.clone();
        if let Some(object) = payload.as_object_mut() {
            if let Some(part_kind) = object.remove("partKind") {
                object.insert("objectKindId".to_string(), part_kind);
            }
            if object.get("targetVortexFullId").is_none() {
                if let Some(grip_id) = puzzle5d_brush_target_grip(ctx.scene) {
                    object.insert("targetVortexFullId".to_string(), json!(grip_id));
                }
            }
        }
        if let Some(next) = ctx.app.apply_engine_brush_placement(ctx.scene, &payload) {
            *ctx.scene = next;
        }
    }
    let part_kind = args.and_then(|value| value.get("partKind").or_else(|| value.get("objectKindId"))).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
    let payload = json!({ "nodeKind": part_kind, "x": args.and_then(|value| value.get("x")).cloned().unwrap_or(json!(120.0)), "y": args.and_then(|value| value.get("y")).cloned().unwrap_or(json!(120.0)) });
    ctx.app.apply_board_brush_place(ctx.scene, &payload);
}

/// 🔁️ Advances the candidate index, wrapping around the engine's collision-free list for the current
/// target grip (or just incrementing when there is no target yet).
pub fn cycle_brush_candidate(ctx: &mut Puzzle5dActionCtx<'_>) {
    ctx.app.drive_precompute(ctx.scene);
    if let Some(grip_full_id) = puzzle5d_brush_target_grip(ctx.scene) {
        let free = parse_brush_candidates_free(&ctx.app.precompute.borrow().brush_candidates(&grip_full_id)).len();
        if free > 0 {
            ctx.scene.runtime.brush_candidate_index = (ctx.scene.runtime.brush_candidate_index + 1) % free;
        }
    } else {
        ctx.scene.runtime.brush_candidate_index = ctx.scene.runtime.brush_candidate_index.saturating_add(1);
    }
}

/// 🧊️ Real GLB geometry the browser round-tripped for one mesh url — installed into the collision
/// engine and remembered so `drive_precompute` never re-registers a box over it. Aborts (emitting
/// nothing at all) because the session cache is not document or config state — the pre-migration
/// `return Emit::default()`.
pub fn register_brush_mesh(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let (Some(url), Some(positions), Some(indices)) =
        (args.and_then(|v| v.get("url")).and_then(|v| v.as_str()), args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array()))
    {
        let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
        let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
        ctx.app.precompute.borrow_mut().register_mesh(url, &positions, &indices);
        ctx.app.registered_mesh_urls.borrow_mut().insert(url.to_string());
    }
    ctx.abort = true;
}

pub fn set_brush_placement_overlap_budget(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()) {
        ctx.scene.runtime.overlap_budget = value.clamp(0.0, 1.0);
        ctx.app.drive_precompute(ctx.scene);
    }
}

/// ⚖️ `setObjectKindWeight`/`setVortexKindWeight` share one arm: both re-normalize their whole group
/// so the sliders always sum to 1.
pub fn set_kind_weight(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0).clamp(0.0, 1.0);
    let part_ids = puzzle5d_kind_ids(&ctx.scene.document, "parts");
    let grip_ids = puzzle5d_kind_ids(&ctx.scene.document, "grips");
    puzzle5d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.object_kind_weights, &part_ids);
    puzzle5d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.vortex_kind_weights, &grip_ids);
    if action == "setObjectKindWeight" {
        ctx.scene.runtime.object_kind_weights = puzzle5d_normalize_kind_weight_group(&ctx.scene.runtime.object_kind_weights, &part_ids, kind_id, value);
    } else {
        ctx.scene.runtime.vortex_kind_weights = puzzle5d_normalize_kind_weight_group(&ctx.scene.runtime.vortex_kind_weights, &grip_ids, kind_id, value);
    }
    ctx.app.drive_precompute(ctx.scene);
}

/// 🎚️ The brush placement picker's select — its option values are `puzzle5d.brush.candidate.<index>`.
pub fn engagement_control_select(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
    if let Some(index) = candidate_id.strip_prefix("puzzle5d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
        ctx.scene.runtime.brush_candidate_index = index;
    }
}

pub fn set_suggestion_offset(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(distance) = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64()) {
        ctx.scene.runtime.suggestion_offset = distance.clamp(PUZZLE5D_SUGGESTION_OFFSET_MIN, PUZZLE5D_SUGGESTION_OFFSET_MAX);
    }
}
