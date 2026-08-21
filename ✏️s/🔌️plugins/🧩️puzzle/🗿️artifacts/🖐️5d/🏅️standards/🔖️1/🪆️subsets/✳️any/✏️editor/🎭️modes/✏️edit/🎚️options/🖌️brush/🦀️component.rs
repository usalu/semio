//! 🖌️ Edit-mode window option — the Brush utility's Utility Options group: suggestion offset, overlap
//! budget, the part/grip distribution trees, and (only when the engine has candidates for the current
//! target grip) the placement picker. Tagged `Some("brush")`.
//!
//! 🎚️ SHARED at MODE level, not per window (TEMPLATE.md §12.2): BOTH the 2D board window and the 3D
//! world window bind the `brush` utility and expose the identical group, so this measure is declared
//! once here and each window's `window_measures()` collects from it.

use crate::editor::puzzle5d::precompute::Puzzle5dPrecomputeSession;
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{
    parse_brush_candidates_free, puzzle5d_action, puzzle5d_brush_target_grip, puzzle5d_kind_ids, puzzle5d_kind_weight_sum, Puzzle5dScene, PUZZLE5D_PLAY_CONTROLLER_ID, PUZZLE5D_SUGGESTION_OFFSET_MAX, PUZZLE5D_SUGGESTION_OFFSET_MIN,
    PUZZLE5D_SUGGESTION_OFFSET_STEP,
};
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};
use serde_json::json;
use std::collections::HashMap;

//#region 🔖️Distribution
async fn kind_weight_measures(prefix: &str, action: &str, ids: &[String], weights: &HashMap<String, f64>) -> Vec<WindowMeasure> {
    ids.iter()
        .map(|kind_id| {
            let weight = weights.get(kind_id).copied().unwrap_or_else(|| if ids.is_empty() { 0.0 } else { 1.0 / ids.len() as f64 });
            WindowMeasure::Slider {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
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
                on_change: puzzle5d_action(action, Some(json!({ "kindId": kind_id }))),
            }
        })
        .collect()
}

async fn distribution_children(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    let part_ids = puzzle5d_kind_ids(&envelope.document, "parts");
    let grip_ids = puzzle5d_kind_ids(&envelope.document, "grips");
    vec![
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-parts"),
            label: format!("{} ({:.0}%)", labels.part_weights.as_str(), puzzle5d_kind_weight_sum(&envelope.runtime.object_kind_weights, &part_ids) * 100.0),
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
            children: kind_weight_measures("part-kind", "setObjectKindWeight", &part_ids, &envelope.runtime.object_kind_weights),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-grips"),
            label: format!("{} ({:.0}%)", labels.grip_weights.as_str(), puzzle5d_kind_weight_sum(&envelope.runtime.vortex_kind_weights, &grip_ids) * 100.0),
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
            children: kind_weight_measures("grip-kind", "setVortexKindWeight", &grip_ids, &envelope.runtime.vortex_kind_weights),
        },
    ]
}
//#endregion 🔖️Distribution

//#region 🔖️Measure
/// 🖌️ The Brush utility's Utility Options group, collected by both windows' `window_measures()`.
pub async fn measure(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> WindowMeasure {
    let mut children = vec![
        WindowMeasure::Slider {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-offset"),
            label: Some(labels.offset.into()),
            value: envelope.runtime.suggestion_offset,
            min: PUZZLE5D_SUGGESTION_OFFSET_MIN,
            max: PUZZLE5D_SUGGESTION_OFFSET_MAX,
            step: Some(PUZZLE5D_SUGGESTION_OFFSET_STEP),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: puzzle5d_action("setSuggestionOffset", None),
        },
        WindowMeasure::Slider {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush-overlap"),
            label: Some(labels.overlap.into()),
            value: envelope.runtime.overlap_budget,
            min: 0.0,
            max: 0.2,
            step: Some(0.005),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: puzzle5d_action("setBrushPlacementOverlapBudget", None),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush-distribution"),
            label: labels.suggestion.into(),
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
            children: distribution_children(envelope, labels),
        },
    ];
    if let Some(target) = puzzle5d_brush_target_grip(envelope) {
        let candidates = parse_brush_candidates_free(&precompute.brush_candidates(&target));
        if !candidates.is_empty() {
            let items: Vec<MeasureSelectItem> = candidates
                .iter()
                .enumerate()
                .map(|(index, candidate)| {
                    let label = candidate.get("objectKind").and_then(|value| value.as_str()).or_else(|| candidate.get("objectKindId").and_then(|value| value.as_str())).unwrap_or("kind");
                    let id = format!("puzzle5d.brush.candidate.{index}");
                    MeasureSelectItem { id: id.clone(), value: id, label: label.into() }
                })
                .collect();
            let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
            children.push(WindowMeasure::Select {
                id: "puzzle5d-brush-placement".into(),
                label: Some(labels.placement.into()),
                value: format!("puzzle5d.brush.candidate.{selected_index}"),
                items,
                on_change: puzzle5d_action("engagementControlSelect", None),
            });
        }
    }
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-utility-options-brush"),
        label: labels.brush.into(),
        default_open: Some(true),
        active_utility_id: Some("brush".into()),
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
