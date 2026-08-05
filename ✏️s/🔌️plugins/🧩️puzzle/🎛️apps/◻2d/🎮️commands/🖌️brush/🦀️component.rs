//! 🖌️ Puzzle 2d play app commands — the brush vocabulary: per-kind placement weights (always kept
//! normalized to sum 1 per catalog slice), the suggestion offset, candidate cycling, the open/commit/
//! cancel slot verbs and the batched fill session.

use crate::apps::puzzle2d::modes::edit::options::brush::{PUZZLE2D_SUGGESTION_OFFSET_MAX, PUZZLE2D_SUGGESTION_OFFSET_MIN};
use crate::apps::puzzle2d::modes::edit::tools::fill;
use crate::apps::puzzle2d::modes::edit::windows::overview;
use crate::apps::puzzle2d::{apply_brush_place_payload, apply_host_events, puzzle2d_kind_ids, puzzle2d_window_and_engagements_scope, puzzle2d_window_and_measures_scope, puzzle2d_window_only_scope, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::{json, Value};
use std::collections::BTreeMap;

//#region 🔖️Weights
pub fn puzzle2d_uniform_kind_weights(ids: &[String]) -> BTreeMap<String, f64> {
    if ids.is_empty() {
        return BTreeMap::new();
    }
    let weight = 1.0 / ids.len() as f64;
    ids.iter().map(|id| (id.clone(), weight)).collect()
}

/// ⚖️ Redistributes the remaining probability mass over the untouched kinds so the group still sums
/// to 1 after `changed_id` is pinned to `new_value`.
pub fn puzzle2d_normalize_kind_weight_group(weights: &BTreeMap<String, f64>, kind_ids: &[String], changed_id: &str, new_value: f64) -> BTreeMap<String, f64> {
    if kind_ids.is_empty() {
        return BTreeMap::new();
    }
    if kind_ids.len() == 1 {
        return BTreeMap::from([(kind_ids[0].clone(), 1.0)]);
    }
    let new_value = new_value.clamp(0.0, 1.0);
    let others: Vec<&String> = kind_ids.iter().filter(|id| id.as_str() != changed_id).collect();
    let remainder = (1.0 - new_value).max(0.0);
    let other_sum: f64 = others.iter().map(|id| weights.get(*id).copied().unwrap_or(0.0)).sum();
    let mut next = BTreeMap::new();
    next.insert(changed_id.to_string(), new_value);
    if remainder <= f64::EPSILON {
        for id in others {
            next.insert((*id).clone(), 0.0);
        }
        return next;
    }
    if other_sum <= f64::EPSILON {
        let each = remainder / others.len() as f64;
        for id in others {
            next.insert((*id).clone(), each);
        }
    } else {
        for id in others {
            let old = weights.get(id).copied().unwrap_or(0.0);
            next.insert((*id).clone(), old / other_sum * remainder);
        }
    }
    next
}

fn puzzle2d_ensure_catalog_kind_weights(weights: &mut BTreeMap<String, f64>, kind_ids: &[String]) {
    if kind_ids.is_empty() {
        return;
    }
    if weights.is_empty() || kind_ids.iter().any(|id| !weights.contains_key(id)) {
        *weights = puzzle2d_uniform_kind_weights(kind_ids);
        return;
    }
    let sum: f64 = kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    if (sum - 1.0).abs() > 0.001 {
        for id in kind_ids {
            if let Some(weight) = weights.get_mut(id) {
                *weight /= sum;
            }
        }
    }
}
//#endregion 🔖️Weights

//#region 🔖️Settings
pub fn set_brush_kind_weights(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let node_ids = puzzle2d_kind_ids(&ctx.scene.fixture, "nodes");
    let handle_ids = puzzle2d_kind_ids(&ctx.scene.fixture, "handles");
    puzzle2d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.node_kind_weights, &node_ids);
    puzzle2d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.handle_kind_weights, &handle_ids);
    if let Some(weights) = args.and_then(|value| value.get("weights")) {
        ctx.scene.runtime.node_kind_weights = weights.get("nodeWeights").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
        ctx.scene.runtime.handle_kind_weights = weights.get("handleWeights").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
    } else if let Some(kind_id) = args.and_then(|value| value.get("kindId")).and_then(|value| value.as_str()) {
        let weight = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()).unwrap_or(0.0).clamp(0.0, 1.0);
        let slice = args.and_then(|value| value.get("catalogSlice")).and_then(|value| value.as_str()).unwrap_or("nodes");
        if slice == "handles" {
            ctx.scene.runtime.handle_kind_weights = puzzle2d_normalize_kind_weight_group(&ctx.scene.runtime.handle_kind_weights, &handle_ids, kind_id, weight);
        } else {
            ctx.scene.runtime.node_kind_weights = puzzle2d_normalize_kind_weight_group(&ctx.scene.runtime.node_kind_weights, &node_ids, kind_id, weight);
        }
    }
    if let Ok(weights_json) = serde_json::to_string(&json!({
        "nodeWeights": ctx.scene.runtime.node_kind_weights,
        "handleWeights": ctx.scene.runtime.handle_kind_weights,
    })) {
        ctx.host.borrow_mut().set_brush_kind_weights(&weights_json);
    }
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}

pub fn set_brush_node_size(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(size) = args.and_then(|value| value.get("size")).and_then(|value| value.as_f64()) {
        ctx.host.borrow_mut().set_brush_node_size(size);
        *ctx.ui_scope = puzzle2d_window_only_scope();
    }
}

pub fn set_suggestion_offset(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let distance = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64());
    if let Some(distance) = distance {
        let clamped = distance.clamp(PUZZLE2D_SUGGESTION_OFFSET_MIN, PUZZLE2D_SUGGESTION_OFFSET_MAX);
        ctx.scene.runtime.suggestion_offset = clamped;
        ctx.host.borrow_mut().set_suggestion_offset(clamped);
        *ctx.ui_scope = puzzle2d_window_and_measures_scope();
    }
}
//#endregion 🔖️Settings

//#region 🔖️Candidates
pub fn cycle_candidate(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let forward = args.and_then(|value| value.get("forward")).and_then(|value| value.as_bool()).unwrap_or(true);
    ctx.host.borrow_mut().brush_cycle_candidate(forward);
    ctx.scene.runtime.brush_candidate_index = ctx.scene.runtime.brush_candidate_index.saturating_add(1);
    *ctx.ui_scope = puzzle2d_window_and_engagements_scope();
}

pub fn set_candidate_index(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
        ctx.host.borrow_mut().brush_set_candidate_index(index as usize);
        ctx.scene.runtime.brush_candidate_index = index as usize;
        *ctx.ui_scope = puzzle2d_window_and_engagements_scope();
    }
}
//#endregion 🔖️Candidates

//#region 🔖️Slot
pub fn open_slot(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(handle_id) = args.and_then(|value| value.get("handleId")).and_then(|value| value.as_str()) {
        ctx.host.borrow_mut().brush_open_slot(handle_id);
    }
}

pub fn commit_slot(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().brush_commit_slot();
    apply_host_events(&mut ctx.host.borrow_mut(), ctx.scene);
}

pub fn cancel_slot(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().brush_cancel_slot();
}
//#endregion 🔖️Slot

//#region 🔖️FillSession
/// 🪣️ The Fill tool's one-shot entry point: activates the tool, runs the whole session in one step
/// and splices every placement into the fixture.
pub fn set_fill_count(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let count = args
        .and_then(|value| value.get("count").or_else(|| value.get("value")))
        .and_then(|value| value.as_f64())
        .map_or(0, |value| value.round().max(0.0) as u32)
        .min(fill::PUZZLE2D_FILL_COUNT_MAX);
    ctx.scene.runtime.fill_count = count;
    ctx.effects.push(HostEffect::SetActiveTool { tool_id: fill::TOOL_ID.into() });
    ctx.host.borrow_mut().set_active_utility(overview::utilities::brush::UTILITY_ID);
    ctx.host.borrow_mut().brush_fill_session_begin(count, 1);
    let step = ctx.host.borrow_mut().brush_fill_session_step(count.max(1));
    apply_fill_placements(ctx, &step);
}

pub fn fill_session_begin(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let max_count = args.and_then(|value| value.get("maxCount")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
    let seed = args.and_then(|value| value.get("seed")).and_then(|value| value.as_u64()).unwrap_or(1) as u32;
    ctx.host.borrow_mut().brush_fill_session_begin(max_count, u64::from(seed));
}

pub fn fill_session_step(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let budget = args.and_then(|value| value.get("chunkBudget")).and_then(|value| value.as_u64()).unwrap_or(8) as u32;
    let step = ctx.host.borrow_mut().brush_fill_session_step(budget);
    apply_fill_placements(ctx, &step);
}

pub fn fill_session_clear(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().brush_fill_session_clear();
    ctx.scene.runtime.fill_count = 0;
}

fn apply_fill_placements(ctx: &mut Puzzle2dActionCtx<'_>, step_json: &str) {
    let Ok(progress) = serde_json::from_str::<Value>(step_json) else {
        return;
    };
    let Some(placements) = progress.get("placements").and_then(|value| value.as_array()) else {
        return;
    };
    for placement in placements {
        apply_brush_place_payload(&mut ctx.scene.fixture, placement);
    }
}
//#endregion 🔖️FillSession

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_weight_group_normalizes_to_sum_one() {
        let ids = vec!["a".into(), "b".into(), "c".into()];
        let initial = puzzle2d_uniform_kind_weights(&ids);
        let next = puzzle2d_normalize_kind_weight_group(&initial, &ids, "a", 0.5);
        let sum: f64 = ids.iter().map(|id| next.get(id).copied().unwrap_or(0.0)).sum();
        assert!((sum - 1.0).abs() < 0.001, "expected normalized weights to sum to 1, got {sum}");
        assert!((next.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 0.001);
    }
}
//#endregion 🧪️Tests
