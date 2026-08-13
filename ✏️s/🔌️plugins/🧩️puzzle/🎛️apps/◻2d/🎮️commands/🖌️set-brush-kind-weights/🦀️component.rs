//! 🖌️ `set-brush-kind-weights` command.

use crate::apps::puzzle2d::modes::edit::options::brush::{PUZZLE2D_SUGGESTION_OFFSET_MAX, PUZZLE2D_SUGGESTION_OFFSET_MIN};
use crate::apps::puzzle2d::modes::edit::tools::fill;
use crate::apps::puzzle2d::modes::edit::windows::overview;
use crate::apps::puzzle2d::{apply_brush_place_payload, apply_host_events, puzzle2d_kind_ids, puzzle2d_window_and_engagements_scope, puzzle2d_window_and_measures_scope, puzzle2d_window_only_scope, Puzzle2dActionCtx};
use semio_framework_plugin::kernel::HostEffect;
use serde_json::{json, Value};
use std::collections::BTreeMap;

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
