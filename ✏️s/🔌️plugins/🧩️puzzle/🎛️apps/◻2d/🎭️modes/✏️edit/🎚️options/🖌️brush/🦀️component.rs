//! 🖌️ Edit-mode window option — the Brush utility's options group: the suggestion-offset slider, the
//! per-kind placement distribution trees, and (once the host has candidates) the placement picker.
//! Tagged `active_utility_id: Some("brush")`, so the chrome only shows it while Brush is active.
//! Shared by all three canvas windows.

use crate::apps::puzzle2d::terminology::Puzzle2dLabels;
use crate::apps::puzzle2d::{puzzle2d_action, puzzle2d_kind_ids, Puzzle2dScene, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};
use serde_json::json;
use std::collections::BTreeMap;

//#region 🔖️Constants
pub const PUZZLE2D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
pub const PUZZLE2D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
const PUZZLE2D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
//#endregion 🔖️Constants

//#region 🔖️Weights
pub fn puzzle2d_kind_weight_sum(weights: &BTreeMap<String, f64>, kind_ids: &[String]) -> f64 {
    kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum()
}

fn puzzle2d_kind_weight_measures(prefix: &str, ids: &[String], weights: &BTreeMap<String, f64>, catalog_slice: &str) -> Vec<WindowMeasure> {
    ids.iter()
        .map(|kind_id| {
            let weight = weights.get(kind_id).copied().unwrap_or_else(|| if ids.is_empty() { 0.0 } else { 1.0 / ids.len() as f64 });
            WindowMeasure::Slider {
                id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                value: weight,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                ready: None,
                loading: None,
                waiting: None,
                disabled: None,
                reveal: None,
                on_change: puzzle2d_action("setBrushKindWeights", Some(json!({ "kindId": kind_id, "catalogSlice": catalog_slice }))),
            }
        })
        .collect()
}
//#endregion 🔖️Weights

//#region 🔖️Measure
/// 🖌️ Utility Options group for the brush utility.
pub fn measure(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> WindowMeasure {
    let node_ids = puzzle2d_kind_ids(&envelope.fixture, "nodes");
    let handle_ids = puzzle2d_kind_ids(&envelope.fixture, "handles");
    let mut children = vec![
        WindowMeasure::Slider {
            id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-offset"),
            label: Some(format!("{} ({})", labels.suggestion.as_str(), labels.offset.as_str())),
            value: envelope.runtime.suggestion_offset,
            min: PUZZLE2D_SUGGESTION_OFFSET_MIN,
            max: PUZZLE2D_SUGGESTION_OFFSET_MAX,
            step: Some(PUZZLE2D_SUGGESTION_OFFSET_STEP),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: puzzle2d_action("setSuggestionOffset", None),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-distribution-nodes"),
            label: format!("{} ({:.0}%)", labels.node_weights.as_str(), puzzle2d_kind_weight_sum(&envelope.runtime.node_kind_weights, &node_ids) * 100.0),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: puzzle2d_kind_weight_measures("node-kind", &node_ids, &envelope.runtime.node_kind_weights, "nodes"),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE2D_PLAY_CONTROLLER_ID}-suggestion-distribution-handles"),
            label: format!("{} ({:.0}%)", labels.handle_weights.as_str(), puzzle2d_kind_weight_sum(&envelope.runtime.handle_kind_weights, &handle_ids) * 100.0),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: puzzle2d_kind_weight_measures("handle-kind", &handle_ids, &envelope.runtime.handle_kind_weights, "handles"),
        },
    ];
    if !envelope.runtime.brush_candidates.is_empty() {
        let items: Vec<MeasureSelectItem> = envelope
            .runtime
            .brush_candidates
            .iter()
            .enumerate()
            .map(|(index, candidate)| {
                let node_kind = candidate.get("nodeKind").and_then(|value| value.as_str()).or_else(|| candidate.as_str()).unwrap_or("kind");
                let id = format!("puzzle2d.brush.candidate.{index}");
                MeasureSelectItem { id: id.clone(), value: id, label: node_kind.into() }
            })
            .collect();
        let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
        children.push(WindowMeasure::Select {
            id: "puzzle2d-brush-placement".into(),
            label: Some(labels.placement.into()),
            value: format!("puzzle2d.brush.candidate.{selected_index}"),
            items,
            on_change: puzzle2d_action("engagementControlSelect", None),
        });
    }
    WindowMeasure::Group {
        id: "puzzle2d-utility-options-brush".into(),
        label: labels.brush.into(),
        default_open: Some(true),
        active_utility_id: Some(crate::apps::puzzle2d::modes::edit::windows::overview::utilities::brush::UTILITY_ID.into()),
        children,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
    }
}
//#endregion 🔖️Measure

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::puzzle2d::modes::edit::windows::overview;
    use crate::apps::puzzle2d::terminology::puzzle2d_labels;
    use crate::apps::puzzle2d::testkit::*;
    use crate::apps::puzzle2d::{default_empty_fixture, Puzzle2dConfig};
    use crate::artifacts::puzzle2d::engine::board_host::puzzle_board_host;
    use crate::apps::puzzle2d::config::Puzzle2dPlayRuntime;
    use crate::apps::puzzle2d::modes::edit::puzzle2d_engagement;

    #[test]
    fn brush_params_are_tagged_utility_options_not_engagement_controls() {
        let labels = puzzle2d_labels(&Puzzle2dConfig::default());
        let host = puzzle_board_host();
        let group_tag = |measures: &[WindowMeasure], id: &str| {
            measures.iter().find_map(|measure| match measure {
                WindowMeasure::Group { id: gid, active_utility_id, .. } if gid == id => Some(active_utility_id.clone()),
                _ => None,
            })
        };
        // 🖌️ Brush candidate picker becomes a fill-utility-sibling tagged group, present only once the host
        // has candidates to place (empty ⇒ absent, matching the old gated-control behaviour).
        let empty_brush = scene(default_empty_fixture(), Puzzle2dPlayRuntime::default(), overview::utilities::brush::UTILITY_ID);
        assert_eq!(group_tag(&overview::window_measures(&empty_brush, labels), "puzzle2d-utility-options-brush"), Some(Some(overview::utilities::brush::UTILITY_ID.into())));
        let mut brush_runtime = Puzzle2dPlayRuntime::default();
        brush_runtime.brush_candidates = vec![json!({ "nodeKind": "node" })];
        let brush_scene = scene(default_empty_fixture(), brush_runtime, overview::utilities::brush::UTILITY_ID);
        let brush_measures = overview::window_measures(&brush_scene, labels);
        assert_eq!(group_tag(&brush_measures, "puzzle2d-utility-options-brush"), Some(Some(overview::utilities::brush::UTILITY_ID.into())));
        assert!(puzzle2d_engagement(&brush_scene, &host, overview::WINDOW_KIND_ID, labels).control.is_none(), "brush engagement HUD must no longer carry the relocated control");
    }
}
//#endregion 🧪️Tests
